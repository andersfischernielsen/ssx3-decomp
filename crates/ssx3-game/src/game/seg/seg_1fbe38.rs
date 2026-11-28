//! `src/seg/seg_1fbe38.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::main::ssxapp::oc429_at;
pub(crate) use crate::game::object::debouncenode::OC327_EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::ui::uiengine::oc391_at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_ae38,
        world_obj_factory_attach_modifier,
        "WorldObjFactory_AttachModifier",
    ),
    (
        0x002f_c2c0,
        world_obj_factory_dead_node,
        "WorldObjFactory_DeadNode",
    ),
    (
        0x002f_b498,
        world_obj_factory_anim_teeter,
        "WorldObjFactory_AnimTeeter",
    ),
    (
        0x002f_c7d0,
        world_obj_factory_debounce,
        "WorldObjFactory_Debounce",
    ),
    (
        0x002f_cff0,
        world_obj_factory_mesh_anim,
        "WorldObjFactory_MeshAnim",
    ),
    (
        0x002f_ded0,
        wscript_fn_add_spline_modifier,
        "WScriptFn_AddSplineModifier",
    ),
    (0x002f_fb50, world_obj_factory_hide, "WorldObjFactory_Hide"),
    (
        0x0030_12f0,
        wscript_fn_mission_command,
        "WScriptFn_MissionCommand",
    ),
    (
        0x0030_3130,
        wscript_fn_object_exists,
        "WScriptFn_ObjectExists",
    ),
    (
        0x0030_61b0,
        wscript_fn_get_progress_flag_bit,
        "WScriptFn_GetProgressFlagBit",
    ),
    (
        0x002f_f1c8,
        wscript_fn_add_rail_modifier,
        "WScriptFn_AddRailModifier",
    ),
    (
        0x0030_57c0,
        wscript_fn_add_halo_modifier,
        "WScriptFn_AddHaloModifier",
    ),
    (
        0x002f_d420,
        world_obj_factory_particle_node,
        "WorldObjFactory_ParticleNode",
    ),
    (
        0x0030_1680,
        wscript_fn_get_object_slot30,
        "WScriptFn_GetObjectSlot30",
    ),
    (0x002f_c9c8, w_script_fn_create_flag, "WScriptFn_CreateFlag"),
    (
        0x0030_2778,
        w_script_fn_is_event_type,
        "WScriptFn_IsEventType",
    ),
    (
        0x0030_3df8,
        wscript_fn_event_type_matches,
        "WScriptFn_EventTypeMatches",
    ),
    (
        0x0030_5478,
        wscript_fn_add_magnet_modifier,
        "WScriptFn_AddMagnetModifier",
    ),
    (
        0x0030_6cb0,
        wscript_fn_multi_particle_control,
        "WScriptFn_MultiParticleControl",
    ),
    (
        0x002f_dc60,
        wscript_fn_add_parent_modifier_rev,
        "WScriptFn_AddParentModifierRev",
    ),
    (
        0x0030_0260,
        wscript_fn_stop_object_loop,
        "WScriptFn_StopObjectLoop",
    ),
    (0x0030_3598, wscript_fn_rand_range, "WScriptFn_RandRange"),
    (
        0x002f_f850,
        wscript_fn_rider_command,
        "WScriptFn_RiderCommand",
    ),
    (
        0x002f_f9a8,
        w_script_fn_object_slot36,
        "WScriptFn_ObjectSlot36",
    ),
    (
        0x0030_0f50,
        w_script_fn_add_collect_items,
        "WScriptFn_AddCollectItems",
    ),
    (
        0x0030_0e28,
        w_script_fn_define_collect_set,
        "WScriptFn_DefineCollectSet",
    ),
    (
        0x002f_b060,
        world_obj_factory_anim_delta,
        "WorldObjFactory_AnimDelta",
    ),
    (
        0x002f_b270,
        world_obj_factory_anim_combo,
        "WorldObjFactory_AnimCombo",
    ),
    (
        0x002f_b6b8,
        world_obj_factory_conveyor,
        "WorldObjFactory_Conveyor",
    ),
    (
        0x002f_b8d0,
        world_obj_factory_counter,
        "WorldObjFactory_Counter",
    ),
    (
        0x002f_bab0,
        world_obj_factory_floating,
        "WorldObjFactory_Floating",
    ),
    (
        0x002f_bec8,
        world_obj_factory_boost,
        "WorldObjFactory_Boost",
    ),
    (0x002f_cc20, wscript_fn_modifier9, "WScriptFn_Modifier9"),
    (
        0x002f_d758,
        wscript_fn_add_position_modifier,
        "WScriptFn_AddPositionModifier",
    ),
    (
        0x002f_d9f8,
        wscript_fn_add_parent_modifier,
        "WScriptFn_AddParentModifier",
    ),
    (
        0x002f_e4a8,
        wscript_add_tex_flip_modifier,
        "WScriptFn_AddTexFlipModifier",
    ),
    (
        0x002f_e668,
        wscript_add_boost_modifier,
        "WScriptFn_AddBoostModifier",
    ),
    (
        0x002f_f390,
        world_obj_factory_float_rail,
        "WorldObjFactory_FloatRail",
    ),
    (
        0x002f_f5e8,
        world_obj_factory_spring_rail,
        "WorldObjFactory_SpringRail",
    ),
    (0x002f_fd58, wscript_fn_unhide, "WScriptFn_Unhide"),
    (
        0x0030_0418,
        wscript_fn_kill_restorable,
        "WScriptFn_KillRestorable",
    ),
    (0x0030_05e8, wscript_fn_kill, "WScriptFn_Kill"),
    (
        0x0030_0770,
        wscript_fn_reset_rider_at_object,
        "WScriptFn_ResetRiderAtObject",
    ),
    (
        0x0030_0948,
        wscript_fn_place_rider_at_object,
        "WScriptFn_PlaceRiderAtObject",
    ),
    (
        0x0030_0b20,
        wscript_fn_set_rail_enabled,
        "WScriptFn_SetRailEnabled",
    ),
    (
        0x0030_0c78,
        wscript_fn_play_script_list,
        "WScriptFn_PlayScriptList",
    ),
    (0x0030_1440, wscript_fn_nop_1440, "WScriptFn_Nop_1440"),
    (
        0x0030_1560,
        wscript_fn_get_collected_count,
        "WScriptFn_GetCollectedCount",
    ),
    (
        0x0030_1830,
        wscript_fn_get_object_slot31,
        "WScriptFn_GetObjectSlot31",
    ),
    (0x0030_1b88, wscript_fn_jump_to_task, "WScriptFn_JumpToTask"),
    (0x0030_1c80, wscript_fn_goto_task, "WScriptFn_GotoTask"),
    (
        0x0030_1d78,
        wscript_fn_has_rider_visited,
        "WScriptFn_HasRiderVisited",
    ),
    (0x0030_1f48, wscript_fn_nop_1f48, "WScriptFn_Nop_1f48"),
    (0x0030_2680, wscript_fn_is_game_mode, "WScriptFn_IsGameMode"),
    (0x0030_2870, wscript_fn_is_event, "WScriptFn_IsEvent"),
    (
        0x0030_2ac0,
        wscript_fn_process_command,
        "WScriptFn_ProcessCommand",
    ),
    (
        0x0030_2bd8,
        wscript_fn_mission_stop_process,
        "WScriptFn_MissionStopProcess",
    ),
    (
        0x0030_2ce8,
        wscript_fn_stop_process,
        "WScriptFn_StopProcess",
    ),
    (0x0030_2df0, w_script_fn_next_task, "WScriptFn_NextTask"),
    (
        0x0030_2ef8,
        wscript_fn_is_rider_in_state,
        "WScriptFn_IsRiderInState",
    ),
    (
        0x0030_32c0,
        wscript_fn_get_rider_speed_scaled,
        "WScriptFn_GetRiderSpeedScaled",
    ),
    (
        0x0030_3380,
        wscript_fn_get_rider_speed,
        "WScriptFn_GetRiderSpeed",
    ),
    (
        0x0030_3430,
        wscript_fn_get_self_handle,
        "WScriptFn_GetSelfHandle",
    ),
    (
        0x0030_3490,
        wscript_fn_get_current_task_id,
        "WScriptFn_GetCurrentTaskId",
    ),
    (
        0x0030_34f8,
        wscript_fn_get_rider_state_info,
        "WScriptFn_GetRiderStateInfo",
    ),
    (
        0x0030_36b0,
        wscript_fn_get_event_arg,
        "WScriptFn_GetEventArg",
    ),
    (
        0x0030_37b0,
        wscript_fn_is_rider_in_state_2,
        "WScriptFn_IsRiderInState_2",
    ),
    (
        0x0030_39f0,
        wscript_fn_crowd3d_control,
        "WScriptFn_Crowd3dControl",
    ),
    (
        0x0030_3d88,
        wscript_fn_empty_helper,
        "WScriptFn_EmptyHelper",
    ),
    (
        0x0030_3d90,
        wscript_fn_game_mode_matches,
        "WScriptFn_GameModeMatches",
    ),
    (
        0x0030_3e60,
        wscript_fn_event_matches,
        "WScriptFn_EventMatches",
    ),
    (
        0x0030_3ec8,
        wscript_fn_is_human_rider,
        "WScriptFn_IsHumanRider",
    ),
    (
        0x0030_3ff0,
        wscript_fn_set_pending_request_a,
        "WScriptFn_SetPendingRequestA",
    ),
    (
        0x0030_4100,
        wscript_fn_set_pending_request_b,
        "WScriptFn_SetPendingRequestB",
    ),
    (
        0x0030_4210,
        wscript_fn_set_pending_request_c,
        "WScriptFn_SetPendingRequestC",
    ),
    (
        0x0030_4320,
        wscript_fn_set_hud_height_target,
        "WScriptFn_SetHudHeightTarget",
    ),
    (
        0x0030_44a8,
        wscript_fn_game_mode_handler_v07,
        "WScriptFn_GameModeHandlerV07",
    ),
    (
        0x0030_45b8,
        wscript_fn_set_rider_config_bits,
        "WScriptFn_SetRiderConfigBits",
    ),
    (
        0x0030_4980,
        wscript_fn_set_object_field24,
        "WScriptFn_SetObjectField24",
    ),
    (
        0x0030_4b38,
        wscript_fn_replay_highlight,
        "WScriptFn_ReplayHighlight",
    ),
    (
        0x0030_4c50,
        wscript_get_rider_height_above_object,
        "WScriptFn_GetRiderHeightAboveObject",
    ),
    (
        0x0030_4e38,
        wscript_call_object_slot41,
        "WScriptFn_CallObjectSlot41",
    ),
    (0x0030_4ff0, wscript_fn_nop_4ff0, "WScriptFn_Nop_4ff0"),
    (
        0x0030_50f0,
        wscript_fn_camera_shake,
        "WScriptFn_CameraShake",
    ),
    (0x0030_5660, wscript_screen_flash, "WScriptFn_ScreenFlash"),
    (
        0x0030_59a0,
        world_obj_factory_avalanche,
        "WorldObjFactory_Avalanche",
    ),
    (
        0x0030_5c88,
        wscript_start_avalanche,
        "WScriptFn_StartAvalanche",
    ),
    (
        0x0030_5d90,
        wscript_add_ava_spline_modifier,
        "WScriptFn_AddAvaSplineModifier",
    ),
    (
        0x0030_5f40,
        wscript_set_avalanche_node_emitter,
        "WScriptFn_SetAvalancheNodeEmitter",
    ),
    (
        0x0030_6300,
        wscript_fn_career_action,
        "WScriptFn_CareerAction",
    ),
    (0x0030_6438, wscript_fn_add_danger, "WScriptFn_AddDanger"),
    (
        0x0030_65e8,
        wscript_fn_remove_danger,
        "WScriptFn_RemoveDanger",
    ),
    (
        0x0030_6770,
        wscript_fn_acquire_danger_zone,
        "WScriptFn_AcquireDangerZone",
    ),
    (
        0x0030_6908,
        wscript_fn_release_danger_zone,
        "WScriptFn_ReleaseDangerZone",
    ),
    (
        0x0030_6e68,
        wscript_fn_check_rider_gate,
        "WScriptFn_CheckRiderGate",
    ),
    (
        0x0030_7020,
        wscript_fn_career_check,
        "WScriptFn_CareerCheck",
    ),
    (
        0x002f_c5d8,
        world_obj_factory_dead_fade,
        "WorldObjFactory_DeadFade",
    ),
    (
        0x002f_cdc8,
        world_obj_factory_flex_bridge,
        "WorldObjFactory_FlexBridge",
    ),
    (
        0x002f_d250,
        wscript_fn_add_roller_modifier,
        "WScriptFn_AddRollerModifier",
    ),
    (
        0x002f_e0c0,
        wscript_add_multi_spline_modifier,
        "WScriptFn_AddMultiSplineModifier",
    ),
    (
        0x002f_e840,
        wof_add_particle_data,
        "WorldObjFactory_AddParticleData",
    ),
    (
        0x0030_1120,
        wscript_fn_collect_item,
        "WScriptFn_CollectItem",
    ),
    (0x0030_2048, wscript_fn_start_event, "WScriptFn_StartEvent"),
    (
        0x0030_2210,
        w_script_fn_section_event_command,
        "WScriptFn_SectionEventCommand",
    ),
    (0x0030_3f80, rider_in_tunnel_func, "RiderInTunnelFunc"),
    (
        0x0030_6a90,
        wscript_spawn_multi_particle,
        "WScriptFn_SpawnMultiParticle",
    ),
];

// ---------------------------------------------------------------------------
// WorldObjFactory_DeadNode (0x2fc2c0)

/// The event's kind (+4): 0 kill (replace with a `DeadNode`), 1 hide
/// (flags only), 3 restore (replace with a `RestoreNode`).
pub(crate) const DEAD_NODE_EVENT_KIND: u32 = 0x4;

/// The world node: flags at +8, its current object at +0xc.
pub(crate) const DEAD_NODE_FLAGS: u32 = 0x8;

pub(crate) const DEAD_NODE_OBJECT: u32 = 0xc;

/// The object's vtable (+0xc) and type (`s16` at +0x10).
pub(crate) const DEAD_NODE_OBJ_VTABLE: u32 = 0xc;

pub(crate) const DEAD_NODE_OBJ_TYPE: u32 = 0x10;

/// Object types left alone: a dead node (6), a restore node (0x13).
pub(crate) const DEAD_NODE_TYPE_DEAD: u64 = 0x6;

pub(crate) const DEAD_NODE_TYPE_RESTORE: u64 = 0x13;

/// The allocation names of the two replacement nodes.
pub(crate) const DEAD_NODE_NAME: u64 = 0x0048_96b8;

pub(crate) const DEAD_NODE_RESTORE_NAME: u64 = 0x0048_96c8;

/// Delete the node's current object: `obj->vtable[8](obj + delta, 3)`.
pub(crate) fn dead_node_delete_object(c: &mut Cpu, obj: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 6;
    let vt = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_VTABLE)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 3;
    let slot = crate::call::VSlot::read(c, vt, 8);
    c.g[V1][0] = slot.func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(slot.delta as u32));
    call_ptr(c, slot.func as u32, ret)
}

/// `EA_MemAllocNamed(0x1c, name, 0x20000000, 0)`, then `ctor(block, node)`.
pub(crate) fn dead_node_replace(
    c: &mut Cpu,
    name: u64,
    alloc_ret: u32,
    ctor: u32,
    ret: u32,
) -> crate::call::Flow {
    c.g[A1][0] = name;
    c.g[A0][0] = 0x1c;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    }
    .call(c, alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, ctor, ret)
}

/// `void WorldObjFactory_DeadNode(WorldObjFactory* node, Event* ev)`
/// (0x2fc2c0): applies a dead-node script event to a world node. Kind 1
/// deletes the node's object (unless it already is a dead node) and folds
/// the flags (`f &= 0xffff0300; f |= f >> 16 (arith) | 2`); kind 0 deletes
/// it and installs a new `DeadNode`; kind 3 deletes it (unless it is a
/// dead or restore node) and installs a `RestoreNode`. Frame 0x20 (s0 =
/// node).
pub fn world_obj_factory_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let kind = Addr(r32(c, A1).wrapping_add(DEAD_NODE_EVENT_KIND)).lw(c);
        c.g[V1][0] = kind;
        c.g[S0][0] = c.g[A0][0];
        let node = r32(c, S0);
        match kind {
            1 => {
                c.cycles += 3;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c338)?;
                }
                c.cycles += 9;
                let node = r32(c, S0);
                let kept = Addr(node.wrapping_add(DEAD_NODE_FLAGS)).lw(c) & 0xffff_ffff_ffff_0300;
                c.g[V1][0] = 0xffff_ffff_ffff_0300;
                let high = sx(((kept as u32) as i32 >> 16) as u32);
                c.g[A0][0] = high;
                let flags = kept | high | 2;
                c.g[V0][0] = flags;
                Addr(node.wrapping_add(DEAD_NODE_FLAGS)).sw(c, flags);
                c.cycles += 1;
            }
            0 => {
                c.cycles += 3 + 2 + 2;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c388)?;
                }
                c.cycles += 11;
                dead_node_replace(c, DEAD_NODE_NAME, 0x002f_c3a0, 0x0035_06d8, 0x002f_c3ac)?;
            }
            3 => {
                c.cycles += 3 + 2 + 2;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    c.g[V0][0] = DEAD_NODE_TYPE_RESTORE;
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        c.cycles += 1;
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == DEAD_NODE_TYPE_RESTORE {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c3e8)?;
                }
                c.cycles += 9;
                dead_node_replace(
                    c,
                    DEAD_NODE_RESTORE_NAME,
                    0x002f_c400,
                    0x0035_0f60,
                    0x002f_c40c,
                )?;
                c.cycles += 1;
            }
            _ => {
                // Any other kind: nothing to do.
                c.cycles += 3 + 2 + 2;
                c.g[V0][0] = if (kind as i64) < 2 { 1 } else { 3 };
            }
        }
        frame.restore(c, S0, 0x10);
        dead_node_leave(c, frame)
    })
}

/// The shared epilogue (`ld ra; addiu sp, 0x20; jr ra`, 3 cycles).
pub(crate) fn dead_node_leave(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x002fae38 WorldObjFactory_AttachModifier

/// `Node.vtable` (+0xc).
pub(crate) const WOF_ATTACH_VTABLE: u32 = 0xc;

/// `Node.object` (+0x18): the world object a modifier attaches to.
pub(crate) const WOF_ATTACH_OBJECT: u32 = 0x18;

/// Vtable slots (`{i16 delta, fn}` pairs): `IsType(type)` (+0x50),
/// `Refresh()` (+0x128), the deleting destructor (+0x8).
pub(crate) const WOF_ATTACH_VT_IS_TYPE: u32 = 0x50;

pub(crate) const WOF_ATTACH_VT_REFRESH: u32 = 0x128;

pub(crate) const WOF_ATTACH_VT_DELETE: u32 = 0x8;

/// `node->vtable->slot(node + delta, a1)` with the original's registers:
/// `rv` holds the vtable, `rf` the method.
pub(crate) fn wof_attach_vcall(
    c: &mut Cpu,
    rv: usize,
    rf: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    let node = c.g[S0][0] as u32;
    c.g[rv][0] = c.lw(node.wrapping_add(WOF_ATTACH_VTABLE));
    let vt = c.g[rv][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[rf][0] = c.lw(vt.wrapping_add(slot + 4));
    let target = c.g[rf][0] as u32;
    c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `int WorldObjFactory_AttachModifier(Node* node, int type, void* data)`
/// (0x2fae38): a node that already is a type 9, 6 or 22 modifier keeps
/// going (1); one that already is `type` is refreshed (1). Otherwise types
/// 13 (particle), 11 (mesh animation), 0 (debounce) and 22 (dead fade)
/// allocate their modifier on the node's object (whose +0xc word is
/// cleared), built from `data` (-1); any other type deletes the node (0).
pub fn world_obj_factory_attach_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0]; // node
        c.g[S2][0] = c.g[A1][0]; // type
        c.g[S3][0] = c.g[A2][0]; // data
        'done: {
            // Already a type 9, 6 or 22 modifier: nothing to do.
            for (i, (kind, ret)) in [
                (0x9u64, 0x002f_ae74u32),
                (0x6, 0x002f_ae94),
                (0x16, 0x002f_aeb4),
            ]
            .into_iter()
            .enumerate()
            {
                if i > 0 {
                    c.cycles += 8;
                }
                c.g[A1][0] = kind;
                wof_attach_vcall(c, V1, V0, WOF_ATTACH_VT_IS_TYPE, ret)?;
                let is = c.g[V0][0] != 0;
                c.g[V0][0] = 0x1;
                if is {
                    break 'done;
                }
            }
            // Already `type`: refresh it.
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            wof_attach_vcall(c, V1, V0, WOF_ATTACH_VT_IS_TYPE, 0x002f_aed4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                wof_attach_vcall(c, V0, V1, WOF_ATTACH_VT_REFRESH, 0x002f_aef0)?;
                c.g[V0][0] = 0x1;
                break 'done;
            }
            c.g[V0][0] = 0xb;

            // The modifier to build: (alloc size, name, ret, ctor, ctor ret).
            c.cycles += 2;
            let ty = c.g[S2][0];
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(WOF_ATTACH_OBJECT));
            let object = c.g[S1][0] as u32;
            let kind: Option<u8> = if ty == 0xb {
                Some(11)
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((ty as i64) < 12) as u64;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if ty == 0 {
                        c.sw(object.wrapping_add(0xc), 0);
                        Some(0)
                    } else {
                        c.cycles += 2;
                        None
                    }
                } else {
                    c.g[V0][0] = 0xd;
                    c.cycles += 2;
                    if ty == 0xd {
                        c.sw(object.wrapping_add(0xc), 0);
                        Some(13)
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x16;
                        if ty == 0x16 {
                            c.sw(object.wrapping_add(0xc), 0);
                            Some(22)
                        } else {
                            c.cycles += 2;
                            None
                        }
                    }
                }
            };
            let Some(kind) = kind else {
                // Unknown type: delete the node.
                c.cycles += 2;
                c.g[A1][0] = 0x3;
                if c.g[S0][0] != 0 {
                    c.cycles += 5;
                    wof_attach_vcall(c, V0, V1, WOF_ATTACH_VT_DELETE, 0x002f_b040)?;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            };
            // (size, name, alloc ret, ctor, ctor ret)
            let (size, name, alloc_ret, ctor, ctor_ret) = match kind {
                13 => (
                    0x2c,
                    0x0049_0000 - 0x69e8,
                    0x002f_af50,
                    0x0035_78a8,
                    0x002f_af68,
                ),
                11 => (
                    0x6c0,
                    0x0049_0000 - 0x69d8,
                    0x002f_af8c,
                    0x0035_1b40,
                    0x002f_afac,
                ),
                0 => (
                    0x34,
                    0x0049_0000 - 0x69c8,
                    0x002f_afcc,
                    0x0034_2c08,
                    0x002f_afe4,
                ),
                _ => (
                    0x3c,
                    0x0049_0000 - 0x69b8,
                    0x002f_b004,
                    0x0035_0750,
                    0x002f_b01c,
                ),
            };
            if kind == 11 {
                c.cycles += 17;
                c.sw(object.wrapping_add(0xc), 0);
            } else {
                c.cycles += 14;
            }
            // EA_MemAllocNamed(size, name, 0x20000000, 0)
            c.g[A1][0] = sx(name);
            c.g[A0][0] = size;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            Callee {
                addr: 0x0031_7d70,
                f: ee::s10::EA_MemAllocNamed,
            }
            .call(c, alloc_ret)?;
            // ctor(mem, 1, object, data, [the world's +0x294 for meshes,] node)
            let node = c.g[S0][0];
            if kind == 11 {
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x294));
                c.g[T1][0] = node;
            } else {
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = node;
            }
            c.g[A1][0] = 0x1;
            call_ptr(c, ctor, ctor_ret)?;
            c.g[V0][0] = u64::MAX;
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
// WorldObjFactory_Hide (0x2ffb50)

/// gp-relative globals of the factory (`gp+0x2560` = `D_004A5650`, the
/// "static seed ready" flag).
pub(crate) const WOF_INIT_FLAG: u32 = 0x2560;

/// The 64-bit static seed word at `D_004A5648`, and its trailing byte at
/// `D_004A564C` (`gp+0x255c`).
pub(crate) const WOF_SEED_WORD: u32 = 0x2558;

pub(crate) const WOF_SEED_BYTE: u32 = 0x255c;

/// `D_004A3C10`: the per-index type of the entries (2 = float value).
pub(crate) const WOF_KINDS: u32 = 0xb20;

/// `D_004A47B8`: the world's object reference table.
pub(crate) const WOF_OBJECTS: u32 = 0x16c8;

/// `D_004A3DD8`: the object whose `+0x290` is returned for id `-1`.
pub(crate) const WOF_SPECIAL: u32 = 0xce8;

/// The tag passed to the allocator: `0x4a0000 + 0x3c18` = `D_004A3C18`.
pub(crate) const WOF_ALLOC_TAG_BASE: u32 = 0x4a_0000;

pub(crate) const WOF_ALLOC_TAG: u32 = 0x3c18;

/// `s4` holds `0x4d0000`, the epilogue's copy source minus 0x6f68.
pub(crate) const WOF_RESULT_BASE: u32 = 0x4d_0000;

pub(crate) const WOF_RESULT_OFF: u32 = 0x6f68;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const WOF_MEM_COPY: u32 = 0x003e_6574;

/// `EA_MemAllocNamed` (0x317d70).
pub(crate) const WOF_MEM_ALLOC: u32 = 0x0031_7d70;

/// `Hide_ctor` (0x350e90), called on the new 0x1c-byte block.
pub(crate) const WOF_HIDE_CTOR: u32 = 0x0035_0e90;

/// `V3 WorldObjFactory_Hide(int n, Ent* e)` (0x2ffb50): builds the 12-byte
/// world-object description `{id, value, value}` from `n` entries
/// (`{idx, val, pad, type}`), then hides the referenced object (kills it if it
/// is visible, else allocates a `Hide` node for it). `a0` is the hidden
/// return-buffer pointer; the result is copied there and returned in `v0`.
pub fn world_obj_factory_hide(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(r32(c, GP)).off(WOF_INIT_FLAG).lw(c);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !ready {
            // Guarded static init: seed word = -1, flag = 1, and the flag
            // byte goes through the stack (the copy-ctor artifact).
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SEED_BYTE).lbu(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] |= 0xffff;
            c.g[A0][0] = 0x1;
            c.sw(r32(c, GP).wrapping_add(WOF_SEED_WORD), c.g[V1][0]);
            c.sw(r32(c, GP).wrapping_add(WOF_INIT_FLAG), c.g[A0][0]);
            c.sb(r32(c, SP), c.g[V0][0]);
        }
        // buf64[0] = -1, then the 8-byte seed word is copied over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x8;
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = sx(r32(c, GP).wrapping_add(WOF_SEED_WORD));
        c.g[S4][0] = sx(WOF_RESULT_BASE);
        call_ptr(c, WOF_MEM_COPY, 0x002f_fbc0)?;
        // `a2 = s2`: the entry pointer, in the branch delay slot.
        c.g[A2][0] = c.g[S2][0];
        // For each entry: buf[idx] = val, as a float when the kind table says
        // 2 and the entry's own type says otherwise.
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(r32(c, GP).wrapping_add(WOF_KINDS));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_fbd8);
                }
            }
        }
        // Resolve the id: id -1 means the object the world points at, else
        // `table[id & 0xff] -> [id >> 8]`, a reference (byte offset) or 0.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[S1][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            c.cycles += 1;
            c.g[S1][0] = c.g[A1][0];
        }
        // Hide it: nothing to do without an object; if it is still visible,
        // kill it; otherwise build a Hide node for it.
        c.cycles += 2;
        let mut hide = false;
        if c.g[S1][0] != 0 {
            hide = true;
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, S1)).off(0xc).lw(c);
            c.g[A1][0] = sx(WOF_ALLOC_TAG_BASE);
            if c.g[S0][0] != 0 {
                // p->v10(16)?
                c.cycles += 8;
                c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                c.g[A1][0] = 0x10;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x50).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x54).lw(c);
                let query = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, query, 0x002f_fcb8)?;
                let hidden = c.g[V0][0] != 0;
                c.g[A1][0] = 0x6;
                if hidden {
                    hide = false;
                } else {
                    // p->v10(6)?
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1)).off(0x50).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(0x54).lw(c);
                    let query = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    call_ptr(c, query, 0x002f_fcd4)?;
                    let hidden = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x3;
                    if hidden {
                        hide = false;
                    } else {
                        // Still visible: p->v01(3) kills it.
                        c.cycles += 6;
                        c.g[V0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                        c.g[A0][0] = Addr(r32(c, V0)).off(0x8).lh(c);
                        c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
                        let kill = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        call_ptr(c, kill, 0x002f_fcf0)?;
                        c.g[A1][0] = sx(WOF_ALLOC_TAG_BASE);
                    }
                }
            }
        }
        if hide {
            c.cycles += 8;
            c.g[A0][0] = 0x1c;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(WOF_ALLOC_TAG));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, WOF_MEM_ALLOC, 0x002f_fd08)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, WOF_HIDE_CTOR, 0x002f_fd14)?;
        }
        // Return the 12-byte result description by value.
        c.cycles += 16;
        let src = r32(c, S4).wrapping_sub(WOF_RESULT_OFF);
        c.g[A1][0] = sx(src);
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S3);
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S3][0];
        for (reg, off) in [(S0, 0x70), (S1, 0x60), (S2, 0x50), (S3, 0x40), (S4, 0x30)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `WorldObjFactory_Debounce` globals: the static seed word at `0x4fb528`
/// (`a3 = 0x500000`, minus 0x4ad8), its ready flag at `gp+0x24fc`, and the
/// per-index kind table at `0x445fb0`.
pub(crate) const WOD_INIT_FLAG: u32 = 0x24fc;

pub(crate) const WOD_STATIC_BASE: u32 = 0x50_0000;

pub(crate) const WOD_STATIC_OFF: u32 = 0x4ad8;

pub(crate) const WOD_KINDS_BASE: u32 = 0x44_0000;

pub(crate) const WOD_KINDS_OFF: u32 = 0x5fb0;

/// The allocator tag: `0x490000 - 0x6918` = `D_004896E8`.
pub(crate) const WOD_ALLOC_TAG: u32 = 0x490000;

pub(crate) const WOD_ALLOC_TAG_OFF: u32 = 0x6918;

/// `WorldObjFactory_AttachModifier` (0x2fae38).
pub(crate) const WOD_CHECK_ACTIVE: u32 = 0x002f_ae38;

/// `Debounce_ctor_2` (0x342c08).
pub(crate) const WOD_DEBOUNCE_CTOR: u32 = 0x0034_2c08;

/// `V3 WorldObjFactory_Debounce(int n, Ent* e)` (0x2fc7d0): like
/// `WorldObjFactory_Hide`, but the node it builds is a `Debounce` (0x34
/// bytes) that ignores objects an active modifier already covers.
pub fn world_obj_factory_debounce(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP)).off(WOD_INIT_FLAG).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = sx(WOD_STATIC_BASE);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x30);
        if !ready {
            // Guarded static init of the 20-byte seed: -1, 1.0f, 1, -1 and
            // its trailing flag byte through the stack.
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WOD_STATIC_OFF));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = Addr(r32(c, V1)).off(0x10).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = !0u64;
            c.sw(r32(c, A3).wrapping_sub(WOD_STATIC_OFF), c.g[V0][0]);
            c.g[A1][0] = 0x1;
            c.sb(r32(c, V1).wrapping_add(0x10), c.g[A2][0]);
            c.swc1(0, r32(c, V1).wrapping_add(0x4));
            c.sw(r32(c, V1).wrapping_add(0xc), c.g[A0][0]);
            c.sw(r32(c, GP).wrapping_add(WOD_INIT_FLAG), c.g[A1][0]);
            c.sw(r32(c, V1).wrapping_add(0x8), c.g[A1][0]);
            c.sb(r32(c, SP), c.g[A2][0]);
        }
        // buf64[5] = -1, then the 20-byte static over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x14;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WOD_STATIC_OFF));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = sx(WOF_RESULT_BASE);
        call_ptr(c, WOF_MEM_COPY, 0x002f_c864)?;
        // `a2 = s3`: the entry pointer (the third argument), in the branch
        // delay slot.
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = sx(WOD_KINDS_BASE);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(WOD_KINDS_OFF));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c880);
                }
            }
        }
        // Resolve the id, exactly as Hide does.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[S0][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }
        // Skip objects that are already covered by an active modifier, else
        // build the Debounce node.
        c.cycles += 2;
        let mut insert = false;
        if c.g[S0][0] != 0 {
            insert = true;
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
            c.g[A1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, WOD_CHECK_ACTIVE, 0x002f_c950)?;
                if c.g[V0][0] != 0 {
                    insert = false;
                }
            }
        }
        if insert {
            c.cycles += 12;
            c.g[A1][0] = sx(WOD_ALLOC_TAG);
            c.g[A0][0] = 0x34;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(WOD_ALLOC_TAG_OFF));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, WOF_MEM_ALLOC, 0x002f_c970)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = 0;
            call_ptr(c, WOD_DEBOUNCE_CTOR, 0x002f_c988)?;
        }
        // Return the 12-byte result description by value.
        c.cycles += 16;
        let src = r32(c, S4).wrapping_sub(WOF_RESULT_OFF);
        c.g[A1][0] = sx(src);
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S2);
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        for (reg, off) in [(S0, 0x80), (S1, 0x70), (S2, 0x60), (S3, 0x50), (S4, 0x40)] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptFn_AddSplineModifier` globals: the static spline seed at 0x4fb778
/// (`0x500000 - 0x4888`), its ready flag at `gp+0x2524`, the kind table at
/// 0x4461d8, and the 12-word stack buffer at `sp+0x10`.
pub(crate) const WSA_INIT_FLAG: u32 = 0x2524;

pub(crate) const WSA_STATIC_BASE: u32 = 0x50_0000;

pub(crate) const WSA_STATIC_OFF: u32 = 0x4888;

pub(crate) const WSA_KINDS_BASE: u32 = 0x44_0000;

pub(crate) const WSA_KINDS_OFF: u32 = 0x61d8;

/// `EA_MemCopy` (0x3e6574).
pub(crate) const WSA_MEM_COPY: u32 = 0x003e_6574;

/// `cMoveNode_addSpline1` (0x355ad0).
pub(crate) const WSA_ADD_SPLINE: u32 = 0x0035_5ad0;

pub(crate) const WSA_RESULT_BASE: u32 = 0x4d_0000;

pub(crate) const WSA_RESULT_OFF: u32 = 0x6f68;

/// `V3 WScriptFn_AddSplineModifier(int n, Ent* e)` (0x2fded0): the script
/// function behind "add spline": builds a 48-byte spline description from `n`
/// entries and, if its second word names a node, adds the spline to the move
/// node the first word resolves to.
pub fn wscript_fn_add_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP)).off(WSA_INIT_FLAG).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = sx(WSA_STATIC_BASE);
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !ready {
            // Guarded static init: -1, -1, 1, 3, 20.0, 0, 0, 0, 1, 0, 0 and
            // the trailing tag byte through the stack.
            c.cycles += 23;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(WSA_STATIC_OFF));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.mtc1(0, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A2][0] = Addr(r32(c, V0)).off(0x2c).lbu(c);
            c.g[A1][0] = 0x3;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = 0x1;
            c.sw(r32(c, A3).wrapping_sub(WSA_STATIC_OFF), c.g[V1][0]);
            c.sb(r32(c, V0).wrapping_add(0x2c), c.g[A2][0]);
            c.sw(r32(c, V0).wrapping_add(0x4), c.g[V1][0]);
            c.sw(r32(c, V0).wrapping_add(0xc), c.g[A1][0]);
            c.swc1(1, r32(c, V0).wrapping_add(0x10));
            c.swc1(0, r32(c, V0).wrapping_add(0x28));
            c.sw(r32(c, GP).wrapping_add(WSA_INIT_FLAG), c.g[A0][0]);
            c.sw(r32(c, V0).wrapping_add(0x8), c.g[A0][0]);
            c.swc1(0, r32(c, V0).wrapping_add(0x14));
            c.swc1(0, r32(c, V0).wrapping_add(0x18));
            c.swc1(0, r32(c, V0).wrapping_add(0x1c));
            c.sw(r32(c, V0).wrapping_add(0x20), c.g[A0][0]);
            c.swc1(0, r32(c, V0).wrapping_add(0x24));
            c.sb(r32(c, SP), c.g[A2][0]);
        }
        // buf64[12]: -1, -1, then the 48-byte static spline over it.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x30;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(WSA_STATIC_OFF));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, WSA_MEM_COPY, 0x002f_df80)?;
        // `a2 = s2`: the entry pointer, in the branch delay slot.
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = sx(WSA_KINDS_BASE);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(WSA_KINDS_OFF));
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                let entry = r32(c, A2);
                c.g[V0][0] = Addr(entry).lw(c);
                c.g[V1][0] = Addr(entry).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, T0)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        c.cycles += 4;
                        c.lwc1(0, entry.wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        store_int = false;
                    } else {
                        c.g[V0][0] = Addr(entry).off(0x4).lw(c);
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_df98);
                }
            }
        }
        // Resolve the node id from buf[0].
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let special = c.g[V0][0] == 0;
        c.g[A1][0] = Addr(r32(c, GP)).off(WOF_OBJECTS).lw(c);
        if special {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, GP)).off(WOF_SPECIAL).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x290).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            let missing = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if !missing {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // If buf[1] names something, add the spline to the node it hangs on.
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, S1)).off(0x4).lw(c);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        if c.g[V0][0] == 0 {
            c.g[V1][0] = sx(WSA_RESULT_BASE);
        } else {
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
            let no_node = c.g[S0][0] == 0;
            c.g[V1][0] = sx(WSA_RESULT_BASE);
            if !no_node {
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0)).off(0xc).lw(c);
                c.g[A0][0] = Addr(r32(c, V1)).off(0x80).lh(c);
                c.g[V0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
                let spline = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, spline, 0x002f_e070)?;
                let unsupported = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !unsupported {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, WSA_ADD_SPLINE, 0x002f_e080)?;
                }
                c.cycles += 1;
                c.g[V1][0] = sx(WSA_RESULT_BASE);
            }
        }
        // Return the 12-byte result description by value.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(WSA_RESULT_OFF));
        let src = r32(c, A3);
        c.ldl(4, src.wrapping_add(0x7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src).off(0x8).lw(c);
        let dst = r32(c, S3);
        c.sdl(4, dst.wrapping_add(0x7));
        c.sdr(4, dst);
        c.sw(dst.wrapping_add(0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The `gp` register as an address.
#[inline]
pub(crate) fn gp_addr(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `gp + off`.
#[inline]
pub(crate) fn gp_oc322(c: &Cpu, off: u32) -> u32 {
    gp_addr(c).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// WScriptFn_ObjectExists (0x303130)
// ---------------------------------------------------------------------------

/// `D_004A57B4`: the one-time initialisation flag of the shared id buffer.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_INIT: u32 = 0x26c4;

/// `D_004A3D18`: the script value-type table (indexed by the entry's slot).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_TYPES: u32 = 0xc28;

/// `D_004A47B8`: the id table head.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_TABLE: u32 = 0x16c8;

/// `D_004A28A8`: the world object the id `-1` maps to (`+0x290`).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_WORLD: u32 = 0xce8;

/// `D_004FBC50` (`D_3130`): the shared id buffer (3 words), `0x500000-0x43b0`.
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_IDS: u32 = 0x004f_bc50;

/// The local copy of the ids (`buf`) and the 0x10-byte entry record
/// (`{int slot; int val; int pad; int type}`).
pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_BUF: u32 = 0x10;

pub(crate) const WSCRIPT_FN_OBJECT_EXISTS_ENTRY: u32 = 0x10;

/// `V3* WScriptFn_ObjectExists(V3* ret, int n, Ent* e)` (0x303130): the
/// script bridge's `ObjectExists`; copy the shared id buffer (`D_3130`),
/// overlay the `n` entries (`slot`, `value`, `type`; a float value when the
/// entry's type differs from the slot's script type and that type is 2), look
/// the first id up in the world's object table and report
/// `ret[2] = 1`, `*ret = object != 0 && object->field_0xc`, storing the flag
/// in the script object at `+8`.
pub fn wscript_fn_object_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 19, 0x30);
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        frame.save(c, 16, 0x60);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_INIT));
        c.g[S3][0] = c.g[A2][0]; // the entry array
        c.g[S1][0] = c.g[A0][0]; // the script object
        c.g[S2][0] = c.g[A1][0]; // the entry count
        c.g[A2][0] = WSCRIPT_FN_OBJECT_EXISTS_IDS.wrapping_add(0x43b0) as u64;
        if c.g[V0][0] == 0 {
            // One-time init of the shared id buffer: `D_3130[0] = -1`,
            // `D_3130[1] = 1` and the tag byte copied onto itself.
            c.cycles += 10;
            c.g[V1][0] = sx(WSCRIPT_FN_OBJECT_EXISTS_IDS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(8));
            c.g[V0][0] |= 0xffff;
            c.sw(WSCRIPT_FN_OBJECT_EXISTS_IDS, c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.sw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_INIT), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A0][0]);
            c.sb(c.g[SP][0] as u32, c.g[A1][0]);
        }
        // `buf[0] = -1; memcpy(buf, D_3130, 12)` and the entry overlay.
        'overlay: {
            c.cycles += 10;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_BUF));
            c.g[V0][0] |= 0xffff;
            c.g[A1][0] = sx(WSCRIPT_FN_OBJECT_EXISTS_IDS);
            c.g[A2][0] = 0xc;
            c.sw(
                (c.g[SP][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_BUF),
                c.g[V0][0],
            );
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x0030_31a8)?;
            let no_entries = (c.g[S2][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if no_entries {
                break 'overlay;
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_TYPES));
            c.g[A3][0] = 0x2; // the script type of a float slot
            loop {
                'store: {
                    'value: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32); // the slot
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc)); // type
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32); // the slot's type
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                            break 'value;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                            break 'value;
                        }
                        // A float value: `*(float*)d = (float)x->val`.
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        break 'store;
                    }
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(WSCRIPT_FN_OBJECT_EXISTS_ENTRY));
                if !more {
                    break 'overlay;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_31c0);
                }
            }
        }
        // `look_3130(buf[0])`: the id -> object lookup.
        'lookup: {
            {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let id_is_minus_one = c.g[V0][0] == 0;
                c.g[A1][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_TABLE));
                if id_is_minus_one {
                    // id -1: the world's own object.
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(gp_oc322(c, WSCRIPT_FN_OBJECT_EXISTS_WORLD));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    break 'lookup;
                }
                'table: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let absent = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if absent {
                        break 'table;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if found {
                        break 'lookup;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // Report: `*ret = obj && obj->field_0xc`, `ret[2] = 1`, and the flag
        // goes into the script object at `+8`.
        'report: {
            'nonnull: {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 1;
                    break 'nonnull;
                }
                'live: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let dead = c.g[V0][0] == 0;
                    c.g[S0][0] = 1;
                    if dead {
                        break 'live;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S0][0]);
                    call_ptr(c, 0x0022_6610, 0x0030_3284)?;
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    break 'report;
                }
                c.cycles += 1;
                c.g[V0][0] = 1;
                break 'nonnull;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V0][0]);
            call_ptr(c, 0x0022_6610, 0x0030_329c)?;
            c.sw(c.g[V0][0] as u32, 0);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `MEM_copy` (EA_MemCopy, 0x3e6574).
pub(crate) const OC324_MEM_COPY: u32 = 0x003e_6574;

/// `LunoVMRegister_AsInt` (0x226610): returns its argument.
pub(crate) const OC324_LUNO_REGISTER_AS_INT: u32 = 0x0022_6610;

/// The career-flag block a WScript BIND call unpacks from (`gp + 0x2790`: a
/// word, then the bind tag byte, then the "initialised" flag).
pub(crate) const OC324_CAREER_FLAGS: u32 = 0x2790;

/// The "initialised" word of that block (`gp + 0x2798`).
pub(crate) const OC324_CAREER_FLAGS_INIT: u32 = 0x2798;

/// The mission-command call's static argument description (`D_004FBEE0`).
#[cfg(test)]
pub(crate) const OC324_MISSION_DESC: u32 = 0x004f_bee0;

/// The "no change" result a script call copies into its slot
/// (`D_004C9098`, 12 bytes).
pub(crate) const OC324_NO_CHANGE: u32 = 0x004c_9098;

// ---------------------------------------------------------------------------
// 0x003061b0 WScriptFn_GetProgressFlagBit

/// `int* WScriptFn_GetProgressFlagBit(int *ret, int n, Ent *e)` (0x3061b0): a
/// Luno script call (the binder `BIND_UNPACK_ARGS` of the career flag block)
/// that answers one progress-flag bit. The first unpacked value picks the bit
/// of `D_005308d0` (0: bit 6, 1: bit 8, 2: bit 7) and the flag is its
/// complement (1 when the bit is clear); any other value answers 1. The
/// result goes to the slot `LunoVMRegister_AsInt(ret)` returns.
pub fn wscript_fn_get_progress_flag_bit(c: &mut Cpu) {
    /// The career flags word the selected bit is read from.
    const FLAGS: u32 = 0x0053_08d0;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC324_CAREER_FLAGS_INIT)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialised {
            // Guarded static-local init: zero the word, keep the tag byte.
            c.cycles += 5;
            c.g[V1][0] = Addr(gp.wrapping_add(OC324_CAREER_FLAGS + 4)).lbu(c);
            c.g[V0][0] = 1;
            Addr(gp.wrapping_add(OC324_CAREER_FLAGS_INIT)).sw(c, c.g[V0][0]);
            Addr(gp.wrapping_add(OC324_CAREER_FLAGS)).sw(c, 0);
            c.sb(frame.at(c, 0), c.g[V1][0]);
        }
        // Copy the 8-byte flag block into a stack buffer, then expand the
        // argument list into it.
        c.cycles += 7;
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A2][0] = 8;
        c.g[A1][0] = sx(gp.wrapping_add(OC324_CAREER_FLAGS));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC324_MEM_COPY, 0x0030_6204)?;
        let count = c.g[S1][0] as i64;
        c.g[A2][0] = c.g[S2][0];
        if count > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(gp.wrapping_add(0xcb8));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                // A flag whose type is 2 is stored as a float, everything
                // else as the raw word.
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_6218);
                }
            }
        }
        // The selected flag bit: the first unpacked value picks the bit.
        c.cycles += 4;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = 1;
        let key = c.g[A0][0];
        c.g[V1][0] = 0x0053_0000;
        if key == 1 {
            c.cycles += 4;
            c.g[S0][0] = 0;
            c.g[V0][0] = Addr(FLAGS).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
            c.cycles += 2;
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.g[A1][0];
            }
        } else if (key as i64) < 2 {
            c.cycles += 3;
            if key == 0 {
                c.cycles += 2;
                c.g[S0][0] = 0;
                c.cycles += 3;
                c.g[V0][0] = Addr(FLAGS).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.cycles += 2;
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A1][0];
                }
            } else {
                // A negative value: no flag, answer 1. (The value test's
                // arm, then the switch's default arm.)
                c.cycles += 4;
                c.g[S0][0] = 1;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            c.cycles += 2;
            if key == 2 {
                c.cycles += 3;
                c.g[S0][0] = 0;
                c.g[V0][0] = Addr(FLAGS).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                c.cycles += 2;
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = c.g[A1][0];
                }
            } else {
                c.cycles += 2;
                c.g[S0][0] = 1;
            }
        }
        c.cycles += 13;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        call_ptr(c, OC324_LUNO_REGISTER_AS_INT, 0x0030_62dc)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003012f0 WScriptFn_MissionCommand

/// `int* WScriptFn_MissionCommand(int *ret, int n, Ent *e)` (0x3012f0): the
/// Luno script call that issues a mission command. It expands the argument
/// block over the static description `D_004FBBE0` (three words, all -1) and,
/// unless the mission id is -1, runs `WScriptMan_MissionCommand` (forced, -1)
/// and returns its result; for id -1 it copies the "no change" result
/// `D_004C9098` (12 bytes) into `ret`.
pub fn wscript_fn_mission_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(0x25c8)).lw(c);
        frame.save(c, S3, 0x30);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            // Guarded static-local init: both words -1, tag byte kept.
            c.cycles += 11;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4420));
            c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(8).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4420), c.g[V0][0]);
            c.g[A0][0] = 0xffff_ffff_ffff_ffffu64;
            c.sb((c.g[V1][0] as u32).wrapping_add(8), c.g[A1][0]);
            c.g[V0][0] = 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(4), c.g[A0][0]);
            c.sw(gp.wrapping_add(0x25c8), c.g[V0][0]);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 7;
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4420));
        c.g[A2][0] = 0xc;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, OC324_MEM_COPY, 0x0030_1360)?;
        let count = c.g[S2][0] as i64;
        c.g[A2][0] = c.g[S3][0];
        if count > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(gp.wrapping_add(0xb80));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1378);
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] |= 0xffff;
        let no_command = c.g[A1][0] == c.g[V0][0];
        c.g[A3][0] = 0xffff_ffff_ffff_ffffu64;
        if !no_command {
            // Issue the mission command; the script manager is the callee's
            // `this`, the mission id and value the first two words.
            c.cycles += 10;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[A0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            Callee {
                addr: 0x0030_a868,
                f: ee::s10::WScriptMan_MissionCommand,
            }
            .call(c, 0x0030_13e0)?;
            c.g[V1][0] = 1;
            c.g[S0][0] = c.g[V0][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[V1][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC324_LUNO_REGISTER_AS_INT, 0x0030_13f4)?;
            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        } else {
            // No command: copy the "no change" result into the slot.
            c.cycles += 8;
            c.g[V0][0] = 0x004d_0000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f68));
            c.ldl(V1, (c.g[A2][0] as u32).wrapping_add(7));
            c.ldr(V1, c.g[A2][0] as u32);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.sdl(V1, (c.g[S1][0] as u32).wrapping_add(7));
            c.sdr(V1, c.g[S1][0] as u32);
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[A0][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002fcff0 WorldObjFactory_MeshAnim

/// The mesh-animation modifier's static description (`D_004FB5E8`, 0x40
/// bytes: id, model handle, speeds, sizes, count, colour).
#[cfg(test)]
pub(crate) const OC324_MESH_ANIM_DESC: u32 = 0x004f_b5e8;

/// The mesh-animation modifier's init flag (`D_004A55FC`).
pub(crate) const OC324_MESH_ANIM_INIT: u32 = 0x250c;

/// `void* WorldObjFactory_MeshAnim(void *ret, int n, Ent *e)` (0x2fcff0): the
/// Luno call that attaches a mesh-animation modifier to the object named by
/// the argument block: expand the block over the static description, resolve
/// the object (id -1: the world's current object), skip when the object
/// already runs one (the factory's type-11 modifier is active), else
/// allocate 0x6c0 bytes and build the anim onto it. `ret` receives the
/// "no change" result (`D_004C9098`, 12 bytes).
pub fn world_obj_factory_mesh_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xb0);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC324_MESH_ANIM_INIT)).lw(c);
        frame.save(c, S0, 0xa0);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S4, 0x60);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S1, 0x90);
        c.g[S4][0] = c.g[A2][0]; // e
        frame.save(c, S3, 0x70);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x50);
        if !initialised {
            // Guarded static-local init of the 0x40-byte description.
            c.cycles += 38;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a18));
            c.g[V1][0] = 0xffff_ffff_ffff_0000u64;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x3c).lbu(c);
            c.g[V1][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A2][0] = 0xffff_ffff_ffff_ffffu64;
            c.g[AT][0] = 0x4416_0000;
            c.mtc1(AT, 7);
            c.g[A1][0] = 1;
            c.lwc1(1, gp.wrapping_sub(0x338c));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 3);
            c.g[AT][0] = 0x4396_0000;
            c.mtc1(AT, 4);
            c.g[AT][0] = 0x43b4_0000;
            c.mtc1(AT, 5);
            c.g[AT][0] = 0xffff_ffff_bf80_0000u64;
            c.mtc1(AT, 6);
            c.sw((c.g[A3][0] as u32).wrapping_sub(0x4a18), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[A0][0]);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(8));
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x18));
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x28));
            c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x2c));
            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[A2][0]);
            c.sw(gp.wrapping_add(OC324_MESH_ANIM_INIT), c.g[A1][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
            c.sb(frame.at(c, 0), c.g[A0][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 12;
        c.g[V0][0] = 0xffff_ffff_ffff_0000u64;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x40;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a18));
        c.sw(frame.at(c, 0x44), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        call_ptr(c, OC324_MEM_COPY, 0x002f_d0e0)?;
        // `lui s3, 0x4d`: the page base `D_004C9098` is built off (kept for
        // the register state a callee's call log records).
        c.g[S3][0] = 0x004d_0000;
        let count = c.g[S0][0] as i64;
        c.g[A2][0] = c.g[S4][0];
        if count > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6060));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let mut float_store = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(4).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        float_store = true;
                    }
                }
                if !float_store {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_d100);
                }
            }
        }
        // Resolve the object the block names (-1: the current one).
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !(0u64 | c.g[A2][0]);
        let named = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if named {
            // The packed id names an instance table entry.
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1c).lw(c);
                // conv(): a table index of 0 means "no object".
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x290).lw(c);
        }
        // No object to attach to: the "no change" result only. An object
        // whose type-11 modifier node is already active: the same. Both
        // return before the allocation.
        c.cycles += 2;
        let no_object = c.g[S0][0] == 0;
        let mut build_anim = false;
        if !no_object {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
            let no_node = c.g[A0][0] == 0;
            c.g[A1][0] = 0xb;
            if no_node {
                build_anim = true;
            } else {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                Callee {
                    addr: 0x002f_ae38,
                    f: ee::s10::WorldObjFactory_AttachModifier,
                }
                .call(c, 0x002f_d1d0)?;
                // A node that answers "active" skips the allocation.
                build_anim = c.g[V0][0] == 0;
            }
        }
        if build_anim {
            c.cycles += 14;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = 0x6c0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68f8));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            Callee {
                addr: 0x0031_7d70,
                f: ee::s10::EA_MemAllocNamed,
            }
            .call(c, 0x002f_d1f0)?;
            c.g[V1][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = c.g[S0][0];
            c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x294).lw(c);
            c.g[A3][0] = c.g[S1][0];
            c.g[T1][0] = 0;
            call_ptr(c, 0x0035_1b40, 0x002f_d210)?;
        }
        // The result slot: the "no change" description D_004C9098
        // (0x4d0000 - 0x6f68).
        c.cycles += 16;
        c.g[A1][0] = sx(OC324_NO_CHANGE);
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        c.sdl(V0, (c.g[S2][0] as u32).wrapping_add(7));
        c.sdr(V0, c.g[S2][0] as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `WorldObjFactory_AttachModifier` (0x2fae38, `checkActiveNode`).
pub(crate) const OC327_CHECK_ACTIVE_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `MEM_copy` (0x3e6574; `call_indirect`).
pub(crate) const OC327_MEM_COPY: u32 = 0x003e_6574;

/// The generic animated-object constructor (0x3421a0; `call_indirect`).
pub(crate) const OC327_ANIM_OBJECT_CTOR_TEETER: u32 = 0x0034_21a0;

// ---------------------------------------------------------------------------
// World object factory.

/// `void WorldObjFactory_AnimTeeter(Vec3* out, s32 n, const AnimEntry* e)`
/// (0x2fb498): build a teeter modifier template (initialised once into
/// 0x4fb400), overlay the `n` entries (`{ s32 idx; s32 val; u8 pad[4]; s32
/// type; }`), resolve the object id in the template, skip when the object is
/// already flagged, else create the modifier through the object's ctor, and
/// return the fixed global (0x4c9098) into `out`.
pub fn world_obj_factory_anim_teeter(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b400;
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_add(0x24d8)).lw(c);
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A2][0] = 0x50_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !initialised {
            // First call: fill the global template.
            c.cycles += 26;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = Addr(TEMPLATE.wrapping_add(0x28)).lbu(c);
            c.g[V1][0] |= 0xffff;
            c.lwc1(3, gp.wrapping_sub(0x339c));
            c.g[A1][0] = 1;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 4);
            c.g[AT][0] = 0x4270_0000;
            c.mtc1(1, 0);
            c.lwc1(1, gp.wrapping_sub(0x3398));
            c.g[AT][0] = 0xffff_ffff_c120_0000;
            c.mtc1(1, 2);
            Addr(TEMPLATE).sw(c, c.g[V1][0]);
            Addr(TEMPLATE.wrapping_add(0x28)).sb(c, c.g[A0][0]);
            c.swc1(0, TEMPLATE.wrapping_add(0x0c));
            c.swc1(1, TEMPLATE.wrapping_add(0x10));
            c.swc1(2, TEMPLATE.wrapping_add(0x14));
            c.swc1(3, TEMPLATE.wrapping_add(0x18));
            c.swc1(4, TEMPLATE.wrapping_add(0x24));
            Addr(gp.wrapping_add(0x24d8)).sw(c, c.g[A1][0]);
            c.swc1(3, TEMPLATE.wrapping_add(0x04));
            Addr(TEMPLATE.wrapping_add(0x08)).sw(c, 0);
            Addr(TEMPLATE.wrapping_add(0x1c)).sw(c, 0);
            c.swc1(4, TEMPLATE.wrapping_add(0x20));
            Addr(c.g[SP][0] as u32).sb(c, c.g[A0][0]);
        }
        // Copy the template into the stack buffer (`MEM_copy`).
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c00));
        c.g[A2][0] = 0x2c;
        Addr(c.g[S1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002f_b558);
        c.g[S4][0] = 0x4d_0000;
        call_ptr(c, OC327_MEM_COPY, 0x002f_b558)?;
        let no_entries = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_entries {
            // Overlay the entries onto the buffer.
            c.cycles += 4;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x0044_5ed0);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let same = c.g[V1][0] == c.g[V0][0];
                if !same {
                    c.cycles += 2;
                }
                if same || c.g[V0][0] != c.g[A3][0] {
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                    c.cycles += 1;
                    Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, c.g[A0][0] as u32);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_b570);
                }
            }
        }
        // Resolve the template id to its object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let resolvable = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if !resolvable {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            // Charge of `s0 = obj`, after the `'b_2fb620` block: both the
            // resolved and the null-reference path pay it.
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }
        // Already active? Else create the modifier.
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            let none = c.g[A0][0] == 0;
            c.g[A1][0] = 4;
            let mut skip = false;
            if !none {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002f_b640);
                c.g[A2][0] = c.g[S1][0];
                OC327_CHECK_ACTIVE_NODE.call(c, 0x002f_b640)?;
                skip = c.g[V0][0] != 0;
            }
            if !skip {
                c.cycles += 11;
                c.g[A1][0] = 0x49_0000;
                c.g[A0][0] = 0x80;
                c.g[A1][0] = sx(0x0048_9678);
                c.g[A2][0] = 0x2000_0000;
                c.g[RA][0] = sx(0x002f_b660);
                c.g[A3][0] = 0;
                call_ptr(c, OC327_EA_MEM_ALLOC_NAMED, 0x002f_b660)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002f_b674);
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, OC327_ANIM_OBJECT_CTOR_TEETER, 0x002f_b674)?;
            }
        }
        // Return the fixed global vector into `out`.
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[A1][0] as u32);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[S2][0] as u32);
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WScriptFn_AddHaloModifier (0x3057c0)

/// The halo-modifier parameter record built on every call (`0x4fbd68`), and
/// its static-init guard (`gp+0x2764`).
pub(crate) const HALO_TEMPLATE: u32 = 0x004f_bd68;

pub(crate) const HALO_TEMPLATE_GUARD: u32 = 0x2764;

/// The script-parameter type table the modifier loop reads: `0x440000`
/// plus this (`0x4466b0`).
pub(crate) const HALO_PARAM_TYPES_OFF: u32 = 0x66b0;

/// `cLunoVar WScriptFn_AddHaloModifier(cLunoVar* out, int n, WsParam* params)`
/// (0x3057c0): copy the halo-modifier record template (`0x4fbd68`, built on
/// the first call), apply the `n` script parameters to the copy (integers
/// copied, floats converted for type-2 entries), resolve the record's id to
/// an object, and when that object is a move node with a live halo interface
/// add the halo modifier. The record (12 bytes) is returned in `*out`.
pub fn wscript_fn_add_halo_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(HALO_TEMPLATE_GUARD)).lw(c);
        frame.save(c, S0, 0x80);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !inited {
            // First call: build the template in place. The trailing byte is
            // passed through unchanged (the copy constructor's byte).
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4298));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[AT][0] = 0x3f80_0000; // 1.0
            c.mtc1(1, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A2][0] = Addr(HALO_TEMPLATE.wrapping_add(0x24)).lbu(c);
            c.g[A0][0] = u64::MAX;
            c.g[AT][0] = 0x42c8_0000; // 100.0
            c.mtc1(1, 1);
            c.g[A1][0] = 1;
            Addr(HALO_TEMPLATE).sw(c, c.g[V1][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x24)).sb(c, c.g[A2][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x8)).sw(c, c.g[A0][0]);
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x18));
            c.swc1(1, HALO_TEMPLATE.wrapping_add(0x1c));
            let guard = r32(c, GP).wrapping_add(HALO_TEMPLATE_GUARD);
            Addr(guard).sw(c, c.g[A1][0]);
            Addr(HALO_TEMPLATE.wrapping_add(0x4)).sw(c, 0);
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0xc));
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x10));
            c.swc1(0, HALO_TEMPLATE.wrapping_add(0x14));
            Addr(HALO_TEMPLATE.wrapping_add(0x20)).sw(c, 0);
            let spv = sp(c);
            c.sb(spv, c.g[A2][0]);
        }
        'b_3058c8: {
            // The stack copy of the record, id preset to -1.
            c.cycles += 10;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            let spv = sp(c);
            c.g[S1][0] = sx(spv.wrapping_add(0x10));
            c.g[V0][0] |= 0xffff;
            c.g[A2][0] = 0x28;
            c.g[A1][0] = sx(HALO_TEMPLATE);
            let buf = r32(c, S1);
            c.sw(buf, c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003e_6574, 0x0030_5868)?; // EA_MemCopy
            let no_params = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if no_params {
                break 'b_3058c8;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(HALO_PARAM_TYPES_OFF));
            c.g[A3][0] = 2;
            loop {
                // One script parameter: `idx` selects the record word, the
                // type table decides whether a value is converted to float.
                'b_3058bc: {
                    'b_3058b8: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_3058b8;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_3058b8;
                        }
                        c.cycles += 4;
                        let val = r32(c, A2).wrapping_add(4);
                        c.lwc1(0, val);
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'b_3058bc;
                    }
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    break 'b_3058bc;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_3058c8;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_5880);
                }
            }
        }
        // Resolve the record's id to an object (v0), -1 meaning "the
        // currently controlled object".
        'b_305924: {
            {
                c.cycles += 4;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let not_minus_one = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x16c8)).lw(c);
                if !not_minus_one {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0x290)).lw(c);
                    break 'b_305924;
                }
            }
            {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                let no_slot = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                if !no_slot {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if found {
                        break 'b_305924;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // A live move node: call its halo-interface test and, when it says
        // yes, add the modifier.
        'b_305964: {
            'b_305960: {
                c.cycles += 2;
                let no_obj = c.g[V0][0] == 0;
                c.g[V1][0] = 0x004d_0000;
                if no_obj {
                    break 'b_305960;
                }
                c.cycles += 3;
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = c.g[S3][0];
                if no_node {
                    break 'b_305964;
                }
                {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_594c)?;
                    let no_interface = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !no_interface {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0035_5eb0, 0x0030_595c)?; // addHalo
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
                break 'b_305960;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            break 'b_305964;
        }
        // Return the record by value: 12 bytes from the static result.
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let res = r32(c, A3);
        c.ldl(4, res.wrapping_add(7));
        c.ldr(4, res);
        c.g[A1][0] = Addr(res).off(0x8).lw(c);
        let out = r32(c, S3);
        c.sdl(4, out.wrapping_add(7));
        c.sdr(4, out);
        Addr(out).off(0x8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddRailModifier (0x2ff1c8)

/// The rail-modifier record template (`0x4fba28`), its static-init guard
/// (`gp+0x2540`) and its parameter type table (`0x446470`).
pub(crate) const RAIL_TEMPLATE: u32 = 0x004f_ba28;

pub(crate) const RAIL_TEMPLATE_GUARD: u32 = 0x2540;

pub(crate) const RAIL_PARAM_TYPES_OFF: u32 = 0x6470;

/// `cLunoVar WScriptFn_AddRailModifier(cLunoVar* out, int n, WsParam* params)`
/// (0x2ff1c8): same shape as `WScriptFn_AddHaloModifier` for the rail
/// modifier: copy the 16-byte template `0x4fba28` (built on the first call),
/// apply the parameters, resolve the object from the record's first word and
/// — only when the second word (the rail id) is not -1 — add the rail
/// modifier to the object's move node.
pub fn wscript_fn_add_rail_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(RAIL_TEMPLATE_GUARD)).lw(c);
        frame.save(c, S0, 0x60);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0];
        let inited = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !inited {
            c.cycles += 12;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x45d8));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = Addr(RAIL_TEMPLATE.wrapping_add(0xc)).lbu(c);
            c.g[V0][0] |= 0xffff;
            Addr(RAIL_TEMPLATE).sw(c, c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            Addr(RAIL_TEMPLATE.wrapping_add(0xc)).sb(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            Addr(RAIL_TEMPLATE.wrapping_add(0x4)).sw(c, c.g[V0][0]);
            Addr(RAIL_TEMPLATE.wrapping_add(0x8)).sw(c, c.g[A0][0]);
            let guard = r32(c, GP).wrapping_add(RAIL_TEMPLATE_GUARD);
            Addr(guard).sw(c, c.g[A1][0]);
            let spv = sp(c);
            c.sb(spv, c.g[A2][0]);
        }
        'b_2ff2b0: {
            c.cycles += 11;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            let spv = sp(c);
            c.g[S1][0] = sx(spv.wrapping_add(0x10));
            c.g[V0][0] |= 0xffff;
            c.g[A2][0] = 0x10;
            c.g[A1][0] = sx(RAIL_TEMPLATE);
            c.sw(spv.wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw(spv.wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x002f_f24c)?; // EA_MemCopy
            let no_params = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if no_params {
                break 'b_2ff2b0;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(RAIL_PARAM_TYPES_OFF));
            c.g[A3][0] = 2;
            loop {
                'b_2ff2a4: {
                    'b_2ff2a0: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A2)).off(0xc).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_2ff2a0;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, A2)).off(4).lw(c);
                            break 'b_2ff2a0;
                        }
                        c.cycles += 4;
                        let val = r32(c, A2).wrapping_add(4);
                        c.lwc1(0, val);
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'b_2ff2a4;
                    }
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    break 'b_2ff2a4;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_2ff2b0;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_f268);
                }
            }
        }
        // Resolve the record's id (word 0) to an object; the result is v1.
        'b_2ff310: {
            {
                c.cycles += 4;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let not_minus_one = c.g[V0][0] != 0;
                c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x16c8)).lw(c);
                if !not_minus_one {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xce8)).lw(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x290)).lw(c);
                    break 'b_2ff310;
                }
            }
            'b_2ff30c: {
                {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    let no_slot = c.g[V1][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if !no_slot {
                        c.cycles += 7;
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let found = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if found {
                            break 'b_2ff30c;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // The rail id (word 1) must be set, the node alive and its rail
        // interface present.
        'b_2ff354: {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x4).lw(c);
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0x004d_0000;
                break 'b_2ff354;
            }
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
            let no_node = c.g[S0][0] == 0;
            c.g[V1][0] = 0x004d_0000;
            if no_node {
                break 'b_2ff354;
            }
            {
                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x80)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_f340)?;
                let no_interface = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_interface {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0035_5e38, 0x002f_f350)?; // addRail
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            break 'b_2ff354;
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let res = r32(c, A3);
        c.ldl(4, res.wrapping_add(7));
        c.ldr(4, res);
        c.g[A1][0] = Addr(res).off(0x8).lw(c);
        let out = r32(c, S3);
        c.sdl(4, out.wrapping_add(7));
        c.sdr(4, out);
        Addr(out).off(0x8).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_ParticleNode (0x2fd420, worldobjfactory.cpp)

/// The particle-node defaults: the guard word (`gp + 0x2514` = 0x4a5604)
/// and the 0xe0-byte template (0x500000 - 0x49c0).
pub(crate) const WOFPN_INIT_GUARD: u32 = 0x2514;

pub(crate) const WOFPN_TEMPLATE: u32 = 0x004f_b640;

/// Parameter types by index (2: float).
pub(crate) const WOFPN_PARAM_TYPES: u32 = 0x0044_60b0;

/// The world object table pointer (`gp + 0x16c8`) and the default world
/// object holder (`gp + 0xce8`, object at +0x290).
pub(crate) const WOFPN_OBJECT_TABLE: u32 = 0x16c8;

pub(crate) const WOFPN_DEFAULT_HOLDER: u32 = 0xce8;

/// The factory's 12-byte "done" result (0x4d0000 - 0x6f68).
pub(crate) const WOFPN_RESULT: u32 = 0x004c_9098;

/// Node fields: the object id word (+0xd8, -1: the default object), the
/// lifetime (+0x1c) clamped to the max (+0x14).
pub(crate) const WOFPN_NODE_OBJECT: u32 = 0xd8;

pub(crate) const WOFPN_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

pub(crate) const WOFPN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};

pub(crate) const WOFPN_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `memcpy` and the particle-node ctor (through the table).
pub(crate) const WOFPN_MEMCPY: u32 = 0x003e_6574;

pub(crate) const WOFPN_NODE_CTOR: u32 = 0x0035_78a8;

/// The guarded static init of the particle-node template.
pub(crate) fn wofpn_init_template(c: &mut Cpu) {
    c.cycles += 72;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 1); // -1.0
    c.g[V0][0] = sx(WOFPN_TEMPLATE);
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(AT, 2); // 4.0
    c.g[A0][0] = 1;
    c.mtc1(0, 0); // 0.0
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // 1.0
    c.g[A2][0] = 0x10;
    let t = WOFPN_TEMPLATE;
    c.swc1(1, t + 0x8);
    c.g[V1][0] |= 0xffff;
    c.swc1(2, t + 0x10);
    c.sw(t, c.g[A0][0]);
    c.sw(t + 0x4, 0);
    c.swc1(3, t + 0xc);
    for off in (0x14..=0x80).step_by(4) {
        c.swc1(0, t + off);
    }
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 1); // 20.0
    c.swc1(3, t + 0x84);
    c.g[A1][0] = c.lbu(t + 0xdc);
    c.swc1(3, t + 0xb4);
    c.sw(t + 0xc4, c.g[A2][0]);
    c.swc1(0, t + 0xcc);
    c.swc1(1, t + 0xd4);
    c.sw(t + 0xd8, c.g[V1][0]);
    let guard = r32(c, GP).wrapping_add(WOFPN_INIT_GUARD);
    c.sw(guard, c.g[A0][0]);
    for off in [0x88, 0x8c, 0x90] {
        c.swc1(0, t + off);
    }
    for off in [0x94, 0x98, 0x9c, 0xa0, 0xa4] {
        c.swc1(3, t + off);
    }
    for off in [0xa8, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
        c.swc1(0, t + off);
    }
    c.sw(t + 0xc8, 0);
    c.sw(t + 0xd0, c.g[A0][0]);
    c.sb(sp(c), c.g[A1][0]);
}

/// `Result WorldObjFactory_ParticleNode(int n, Param* params)` (0x2fd420;
/// the 12-byte result through the hidden pointer in a0): allocates a
/// particle node from the default template, applies the `n` params
/// (16 bytes each: index, value, -, type; an int given for a float slot
/// is converted), looks up its world object (by id, or the default one),
/// and unless the object's modifier refuses it (AttachModifier: > 0 frees
/// the node, < 0 keeps it) clamps the lifetime and creates the particle
/// node on the object. A missing object frees the node.
pub fn world_obj_factory_particle_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let guard = r32(c, GP).wrapping_add(WOFPN_INIT_GUARD);
        c.g[V0][0] = c.lw(guard);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = 0x500000;
        frame.save(c, S4, 0x30);
        c.g[S2][0] = c.g[A0][0]; // result
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0]; // n
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A2][0]; // params
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            wofpn_init_template(c);
        }

        // node = alloc(0xe0); node->object = -1; memcpy(node, template, 0xe0)
        c.cycles += 17;
        c.g[A0][0] = 0xe0;
        c.g[A1][0] = sx(0x0049_0000 - 0x68e8);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S3][0] = 0x4d0000;
        WOFPN_ALLOC.call(c, 0x002f_d594)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = 0xe0;
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = sx(WOFPN_TEMPLATE);
        c.g[A0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(WOFPN_NODE_OBJECT), c.g[V0][0]);
        call_ptr(c, WOFPN_MEMCPY, 0x002f_d5b4)?;
        c.g[A2][0] = c.g[S5][0];
        if (c.g[S4][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S4][0];
            c.g[T0][0] = sx(WOFPN_PARAM_TYPES);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                let param = r32(c, A2);
                c.g[V0][0] = c.lw(param);
                c.g[V1][0] = c.lw(param.wrapping_add(0xc));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                c.g[V0][0] = c.lw(r32(c, V0));
                let convert = c.g[V1][0] != c.g[V0][0] && {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, param.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, r32(c, A0));
                } else {
                    c.g[V0][0] = c.lw(param.wrapping_add(4));
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(param.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_d5d0);
                }
            }
        }

        // obj = id == -1 ? default object : table[id & 0xff] entry id >> 8
        c.cycles += 4;
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(WOFPN_NODE_OBJECT));
        c.g[V0][0] = !c.g[A2][0];
        let gp = r32(c, GP);
        c.g[A1][0] = c.lw(gp.wrapping_add(WOFPN_OBJECT_TABLE));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp.wrapping_add(WOFPN_DEFAULT_HOLDER));
            c.g[S1][0] = c.lw(r32(c, V0).wrapping_add(0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(8));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, A2) >> 8);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                let some = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                if !some {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S1][0] = c.g[A2][0];
        }

        'done: {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                WOFPN_DELETE.call(c, 0x002f_d694)?;
                break 'done;
            }
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0xc));
            c.cycles += 2;
            c.g[A1][0] = 0xd;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                WOFPN_ATTACH_MODIFIER.call(c, 0x002f_d6ac)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        WOFPN_DELETE.call(c, 0x002f_d6c4)?;
                    }
                    break 'done;
                }
            } else {
                c.cycles += 1;
            }
            // lifetime = min(lifetime, max)
            let node = r32(c, S0);
            c.lwc1(0, node.wrapping_add(0x1c));
            c.cycles += 5;
            c.lwc1(1, node.wrapping_add(0x14));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, node.wrapping_add(0x1c));
            }
            c.cycles += 12;
            c.g[A0][0] = 0x2c;
            c.g[A1][0] = sx(0x0049_0000 - 0x68d0);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            WOFPN_ALLOC.call(c, 0x002f_d6fc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 0;
            call_ptr(c, WOFPN_NODE_CTOR, 0x002f_d714)?;
        }

        // *result = factory "done" result
        c.cycles += 17;
        c.g[A1][0] = sx(WOFPN_RESULT); // s3 (0x4d0000) - 0x6f68
        let (src, dst) = (r32(c, A1), r32(c, S2));
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.g[V1][0] = c.lw(src.wrapping_add(8));
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        c.sw(dst.wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_GetObjectSlot30 (0x301680)

/// `gp+0x25e0`: the shared 8-byte id buffer (`D_1680`).
pub(crate) const WSCRIPT_GOS30_IDS: u32 = 0x25e0;

/// `gp+0x25e4`: its tag byte (copied to the stack on the first call).
pub(crate) const WSCRIPT_GOS30_TAG: u32 = 0x25e4;

/// `gp+0x25e8`: the one-time initialisation flag.
pub(crate) const WSCRIPT_GOS30_INIT: u32 = 0x25e8;

/// `gp+0xb98`: the script value type of each slot.
pub(crate) const WSCRIPT_GOS30_TYPES: u32 = 0xb98;

/// `gp+0x16c8`: the id table head; `gp+0xce8`: the world (`+0x290` is the
/// object of id -1).
pub(crate) const WSCRIPT_GOS30_TABLE: u32 = 0x16c8;

pub(crate) const WSCRIPT_GOS30_WORLD: u32 = 0xce8;

/// `cLunoVar* WScriptFn_GetObjectSlot30(cLunoVar* ret, int n, Ent* e)`
/// (0x301680): the script bridge's float query: copy the shared id buffer,
/// overlay the `n` argument entries (as `WScriptFn_ObjectExists`), look the
/// first id up and return (type 2, a float) the object's `+0xc` component's
/// vtable slot 0xf0 result, or -1 when there is no object or component.
pub fn wscript_fn_get_object_slot30(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_INIT)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0]; // n
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0]; // entries
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0x70);
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_TAG)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            Addr(gp.wrapping_add(WSCRIPT_GOS30_IDS)).sw(c, c.g[V1][0]);
            Addr(gp.wrapping_add(WSCRIPT_GOS30_INIT)).sw(c, 1);
            Addr(sp(c)).sb(c, c.g[V0][0]);
        }
        // `buf[0] = -1; memcpy(buf, D_1680, 8)` and the entry overlay.
        'overlay: {
            c.cycles += 10;
            c.g[V0][0] = u64::MAX;
            c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A2][0] = 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(gp.wrapping_add(WSCRIPT_GOS30_IDS));
            Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x0030_16ec)?;
            let no_entries = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if no_entries {
                break 'overlay;
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(gp.wrapping_add(WSCRIPT_GOS30_TYPES));
            c.g[A3][0] = 2; // the script type of a float slot
            loop {
                'store: {
                    'value: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c); // the slot
                        c.g[V1][0] = Addr(oc391_at(c, A2, 0xc)).lw(c); // type
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c); // the slot's type
                        let raw = c.g[V1][0] == c.g[V0][0] || {
                            c.cycles += 2;
                            c.g[V0][0] != c.g[A3][0]
                        };
                        if raw {
                            c.g[V0][0] = Addr(oc391_at(c, A2, 4)).lw(c);
                            break 'value;
                        }
                        // A float value: `*(float*)d = (float)x->val`.
                        c.cycles += 4;
                        c.lwc1(0, oc391_at(c, A2, 4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'store;
                    }
                    c.cycles += 1;
                    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if !more {
                    break 'overlay;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1700);
                }
            }
        }
        // Look the first id up: -1 is the world's object, else the low byte
        // picks a descriptor and the next bits its entry.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_TABLE)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_WORLD)).lw(c);
            c.g[V0][0] = Addr(oc391_at(c, V0, 0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = sx(((c.g[A2][0] & 0xff) as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(oc391_at(c, V1, 8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, A2) >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(oc391_at(c, V1, 0x1c)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // The value: the component's slot 0xf0, or -1.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
            c.cycles += 4;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(1, 20);
            Addr(oc391_at(c, S2, 8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(oc391_at(c, V0, 0xc)).lw(c);
            if c.g[A1][0] == 0 {
                c.cycles += 2;
                c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
                c.mtc1(1, 20);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(oc391_at(c, A1, 0xc)).lw(c);
                c.g[A0][0] = Addr(oc391_at(c, V0, 0xf0)).lh(c);
                c.g[V1][0] = Addr(oc391_at(c, V0, 0xf4)).lw(c);
                let func = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0030_17cc)?;
                c.mov_s(20, 0);
            }
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[A0][0] = c.g[S2][0];
            Addr(oc391_at(c, S2, 8)).sw(c, 2);
        }
        c.cycles += 12;
        call_ptr(c, 0x0022_6618, 0x0030_1804)?; // LunoVMRegister_AsFloat
        c.swc1(20, r32(c, V0));
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_CreateFlag (0x002fc9c8)

/// `WorldObjFactory_AttachModifier` (0x002fae38).
pub(crate) const WORLD_OBJ_FACTORY_ATTACH_MODIFIER_CALL: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `EA_MemAllocNamed` (0x00317d70).
pub(crate) const EA_MEM_ALLOC_NAMED_CALL: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// `WScriptFn::CreateFlag(entries, n)` (0x2fc9c8): build the 108-byte flag template (static, initialised once), apply the `n` (idx, type, value) overrides (floats converted when the slot is a float), resolve the target object (the world object of the id in the template, or the default one), and unless it already has an active node of kind 10, allocate a 0x1c-byte flag node on it. Returns a constant.
pub fn w_script_fn_create_flag(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xe0 };
        'b_2fca98: {
            c.cycles += 13;
            Frame::open(c, 0xe0);
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0x2500));
            frame.save(c, S4, 0x90);
            frame.save(c, S0, 0xd0);
            c.g[S4][0] = c.g[A2][0];
            frame.save(c, S2, 0xb0);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S1, 0xc0);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S3, 0xa0);
            c.g[A2][0] = 0x500000;
            let t = c.g[V0][0] != 0u64;
            frame.save_ra(c, 0x80);
            if t {
                break 'b_2fca98;
            } else {
                c.cycles += 39;
                c.g[V0][0] = sx(r32(c, A2).wrapping_sub(0x4ac0));
                c.g[V1][0] = 0xffffffffffff0000;
                c.mtc1(0, 1);
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[A0][0] = 0x1;
                c.g[A1][0] = c.lbu(r32(c, V0).wrapping_add(0x68));
                c.g[AT][0] = 0x41c8_0000;
                c.mtc1(1, 2);
                c.sw(r32(c, A2).wrapping_sub(0x4ac0), c.g[V1][0]);
                c.sb(r32(c, V0).wrapping_add(0x68), c.g[A1][0]);
                c.swc1(2, r32(c, V0).wrapping_add(0x14));
                c.swc1(0, r32(c, V0).wrapping_add(0x3c));
                c.swc1(1, r32(c, V0).wrapping_add(0x60));
                c.sw(r32(c, GP).wrapping_add(0x2500), c.g[A0][0]);
                c.sw(r32(c, V0).wrapping_add(0x4), c.g[A0][0]);
                c.sw(r32(c, V0).wrapping_add(0x8), 0u64);
                c.sw(r32(c, V0).wrapping_add(0xc), 0u64);
                c.swc1(0, r32(c, V0).wrapping_add(0x10));
                c.swc1(0, r32(c, V0).wrapping_add(0x18));
                c.swc1(0, r32(c, V0).wrapping_add(0x1c));
                c.swc1(1, r32(c, V0).wrapping_add(0x20));
                c.swc1(0, r32(c, V0).wrapping_add(0x24));
                c.swc1(0, r32(c, V0).wrapping_add(0x28));
                c.swc1(1, r32(c, V0).wrapping_add(0x2c));
                c.swc1(0, r32(c, V0).wrapping_add(0x30));
                c.swc1(0, r32(c, V0).wrapping_add(0x34));
                c.swc1(1, r32(c, V0).wrapping_add(0x38));
                c.swc1(1, r32(c, V0).wrapping_add(0x40));
                c.swc1(1, r32(c, V0).wrapping_add(0x44));
                c.swc1(1, r32(c, V0).wrapping_add(0x48));
                c.swc1(1, r32(c, V0).wrapping_add(0x4c));
                c.swc1(1, r32(c, V0).wrapping_add(0x50));
                c.swc1(1, r32(c, V0).wrapping_add(0x54));
                c.swc1(1, r32(c, V0).wrapping_add(0x58));
                c.swc1(1, r32(c, V0).wrapping_add(0x5c));
                c.sw(r32(c, V0).wrapping_add(0x64), 0u64);
                c.sb(r32(c, SP), c.g[A1][0]);
                break 'b_2fca98;
            }
        }
        'b_2fcb20: {
            c.cycles += 11;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S1][0] = sx(r32(c, SP).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx(r32(c, A2).wrapping_sub(0x4ac0));
            c.g[A2][0] = 0x6c;
            c.sw(r32(c, SP).wrapping_add(0x10), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = 0x4d0000;
            call_ptr(c, 0x003e_6574, 0x002f_cabc)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S4][0];
            if t {
                break 'b_2fcb20;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S0][0];
                c.g[T0][0] = sx(r32(c, V0).wrapping_add(0x5fc0));
                c.g[A3][0] = 0x2;
                'l_2fcad8: loop {
                    'b_2fcb14: {
                        'b_2fcb10: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(r32(c, A2));
                            c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xc));
                            c.g[V0][0] = sx(r32(c, V0) << 2);
                            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                            c.g[V0][0] = c.lw(r32(c, V0));
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                                break 'b_2fcb10;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x4));
                                    break 'b_2fcb10;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, r32(c, A2).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, r32(c, A0));
                                    if t {
                                        break 'b_2fcb14;
                                    } else {
                                        break 'b_2fcb10;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(r32(c, A0), c.g[V0][0]);
                        break 'b_2fcb14;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f_cad8);
                        }
                        continue 'l_2fcad8;
                    } else {
                        break 'b_2fcb20;
                    }
                }
            }
        }
        'b_2fcb8c: {
            'b_2fcb3c: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(r32(c, S1));
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_add(0x16c8));
                if t {
                    break 'b_2fcb3c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(0xce8));
                    let t = true;
                    c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0x290));
                    if t {
                        break 'b_2fcb8c;
                    } else {
                        break 'b_2fcb3c;
                    }
                }
            }
            'b_2fcb88: {
                'b_2fcb64: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                        break 'b_2fcb64;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_2fcb88;
                        } else {
                            break 'b_2fcb64;
                        }
                    }
                }
                'b_2fcb84: {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A2) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if t {
                        break 'b_2fcb84;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                        break 'b_2fcb84;
                    }
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
                break 'b_2fcb88;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
            break 'b_2fcb8c;
        }
        'b_2fcbdc: {
            c.cycles += 2;
            let t = c.g[S0][0] == 0u64;

            if t {
                break 'b_2fcbdc;
            } else {
                'b_2fcbb0: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0xa;
                    if t {
                        break 'b_2fcbb0;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.g[S1][0];
                        WORLD_OBJ_FACTORY_ATTACH_MODIFIER_CALL.call(c, 0x002f_cba8)?;
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_2fcbdc;
                        } else {
                            break 'b_2fcbb0;
                        }
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x3c08));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                EA_MEM_ALLOC_NAMED_CALL.call(c, 0x002f_cbc8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0034_add8, 0x002f_cbdc)?;
                break 'b_2fcbdc;
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx(r32(c, S3).wrapping_sub(0x6f68));
        c.ldl(2, r32(c, A1).wrapping_add(0x7));
        c.ldr(2, r32(c, A1));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x8));
        c.sdl(2, r32(c, S2).wrapping_add(0x7));
        c.sdr(2, r32(c, S2));
        c.sw(r32(c, S2).wrapping_add(0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `Result* WScriptFn_IsEventType(Result* ret, int n, Arg* e)` (0x302778):
/// bind the script call's arguments into a 2-word record (defaults
/// `{-1, ...}` set once), convert the float ones, store `IsEventType(v[0])`
/// as the integer result and return `ret`.
pub fn w_script_fn_is_event_type(c: &mut Cpu) {
    const W_SCRIPT_FN_EVENT_TYPE_MATCHES: Callee = Callee {
        addr: 0x00303df8,
        f: ee::s10::f_00303df8,
    };
    guest(c, |c| {
        'b_3027bc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2660));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A2][0];
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_3027bc;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x265c));
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[A0][0] = 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2658), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2660), c.g[A0][0]);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                break 'b_3027bc;
            }
        }
        'b_302830: {
            c.cycles += 7;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = 0x8;
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2658));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x003027d0)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if t {
                break 'b_302830;
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xbe8));
                c.g[A3][0] = 0x2;
                'l_3027e8: loop {
                    'b_302824: {
                        'b_302820: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_302820;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_302820;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_302824;
                                    } else {
                                        break 'b_302820;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_302824;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003027e8);
                        }
                        continue 'l_3027e8;
                    } else {
                        break 'b_302830;
                    }
                }
            }
        }
        c.cycles += 16;
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        W_SCRIPT_FN_EVENT_TYPE_MATCHES.call(c, 0x00302838)?;
        c.g[V1][0] = 0x1;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00226610, 0x0030284c)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Small register-relative accessors (`lw off(reg)` and friends). They read the
// base register themselves, so the call never borrows `c` twice.

/// `lw dst, off(reg)` (sign-extended).
#[inline]
pub(crate) fn oc417_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lw(c)
}

/// `lh dst, off(reg)` (sign-extended).
#[inline]
pub(crate) fn oc417_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lh(c)
}

/// `lb dst, off(reg)` (sign-extended).
#[inline]
pub(crate) fn oc417_lb(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lb(c)
}

/// `lbu dst, off(reg)`.
#[inline]
pub(crate) fn oc417_lbu(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lbu(c)
}

/// `sw src, off(reg)`.
#[inline]
pub(crate) fn oc417_sw(c: &mut Cpu, reg: usize, off: u32, src: usize) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    Addr(a).sw(c, v);
}

/// `sw zero, off(reg)`.
#[inline]
pub(crate) fn oc417_sw0(c: &mut Cpu, reg: usize, off: u32) {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).sw(c, 0);
}

/// `addu` of two registers' low words, sign-extended.
#[inline]
pub(crate) fn oc417_add(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

// ---------------------------------------------------------------------------
// WScriptFn_EventTypeMatches (0x303df8)

/// `int WScriptFn_EventTypeMatches(unsigned i)` (0x303df8): is the world
/// script's event type `i` (below 8) equal to the current event byte
/// (`*0x535c10`)? Looks the BE interface up first (result unused), then
/// compares `table[i]` (`0x4465f8`) with the byte. 0 for `i >= 8`.
pub fn wscript_fn_event_type_matches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 8) as u64;
        let in_range = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if in_range {
            c.cycles += 14;
            call_ptr(c, 0x0014_dc80, 0x0030_3e20)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0030_3e2c)?;
            c.g[V1][0] = 0x44_0000;
            c.g[V0][0] = 0x53_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = oc417_add(c.g[V1][0], 0x65f8);
            c.g[A1][0] = oc417_lb(c, V0, 0x5c10);
            c.g[A0][0] = oc417_add(c.g[A0][0], c.g[V1][0]);
            c.g[V0][0] = oc417_lw(c, A0, 0);
            c.g[V0][0] ^= c.g[A1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_MultiParticleControl (0x306cb0) and WScriptFn_AddMagnetModifier
// (0x305478): the same world-script argument decoding.

/// The `MEM_copy` (0x3e6574) both functions use to copy the default record.
pub(crate) const WSCRIPT_MEM_COPY: u32 = 0x003e_6574;

/// `WScriptFn_MultiParticleControl(Rec* result, int n, Ent* entries)`
/// (0x306cb0): on first use initialise the static default record
/// (`0x4fc050`: object id -1 ...), copy it to a 16-byte stack record, apply
/// the `n` 16-byte `{index, value, type, ..}` argument entries to the
/// record's words (an int argument for a float parameter is converted with
/// `cvt.s.w`), resolve the object id (`rec[1]`, -1 = the current object at
/// `world+0x290`, else the world cache's slot table) and, when found, call
/// `MultiParticle` add (`rec[2] == 0`, `0x357ae8`) or remove (`0x357b10`)
/// with the world's particle manager (`gp-0xf38 slot`, `rec[0]`) and the
/// object's `+0x78`. Copies the 12-byte default result (`0x4c9098`) to
/// `*result` and returns `result`.
pub fn wscript_fn_multi_particle_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = oc417_lw(c, GP, 0x27c8);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 11;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fb0));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = oc417_lbu(c, V1, 0xc);
            c.g[V0][0] |= 0xffff;
            Addr((c.g[A2][0] as u32).wrapping_sub(0x3fb0)).sw(c, 0);
            c.g[A0][0] = 1;
            let a = (c.g[V1][0] as u32).wrapping_add(0xc);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
            oc417_sw(c, V1, 4, V0);
            oc417_sw(c, GP, 0x27c8, A0);
            oc417_sw0(c, V1, 8);
            let a = sp(c);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
        }
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = oc417_add(c.g[SP][0], 0x10);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fb0));
        c.g[A2][0] = 0x10;
        oc417_sw(c, SP, 0x14, V0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WSCRIPT_MEM_COPY, 0x0030_6d2c)?;
        let no_entries = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_entries {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = oc417_add(c.g[V0][0], 0x6970);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = oc417_lw(c, A2, 0);
                c.g[V1][0] = oc417_lw(c, A2, 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc417_add(c.g[V0][0], c.g[S0][0]);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[T0][0]);
                c.g[V0][0] = oc417_lw(c, V0, 0);
                // `entry.type != paramType && paramType == 2`: int -> float.
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = oc417_lw(c, A2, 4);
                    false
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = oc417_lw(c, A2, 4);
                        false
                    } else {
                        c.cycles += 4;
                        let a = (c.g[A2][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.cvt_s_w(0, 0);
                        let a = c.g[A0][0] as u32;
                        c.swc1(0, a);
                        true
                    }
                };
                if !convert {
                    c.cycles += 1;
                    oc417_sw(c, A0, 0, V0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = oc417_add(c.g[A2][0], 0x10);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_6d48);
                    }
                    continue;
                }
                break;
            }
        }
        // Resolve the object (rec[1]).
        c.cycles += 4;
        c.g[A2][0] = oc417_lw(c, S0, 4);
        c.g[V0][0] = !(c.g[A2][0]);
        let looked_up = c.g[V0][0] != 0;
        c.g[A1][0] = oc417_lw(c, GP, 0x16c8);
        if !looked_up {
            c.cycles += 3;
            c.g[V0][0] = oc417_lw(c, GP, 0xce8);
            c.g[V1][0] = oc417_lw(c, V0, 0x290);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = oc417_lw(c, A1, 0);
            c.g[A0][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = oc417_lw(c, V0, 0);
            let no_slot = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_slot {
                c.cycles += 7;
                c.g[A0][0] = oc417_lw(c, V1, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = oc417_lw(c, V0, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.g[V1][0] = 0x4d_0000;
        } else {
            let remove_instead = {
                c.cycles += 3;
                c.g[V0][0] = oc417_lw(c, S0, 8);
                let r = c.g[V0][0] != 0;
                c.g[A1][0] = oc417_lw(c, S0, 0);
                r
            };
            if !remove_instead {
                c.cycles += 5;
                c.g[A2][0] = oc417_lw(c, V1, 0x78);
                c.g[A0][0] = oc417_lw(c, GP, 0xf38);
                call_ptr(c, 0x0035_7ae8, 0x0030_6e10)?;
                c.g[V1][0] = 0x4d_0000;
            } else {
                c.cycles += 4;
                c.g[A2][0] = oc417_lw(c, V1, 0x78);
                c.g[A0][0] = oc417_lw(c, GP, 0xf38);
                call_ptr(c, 0x0035_7b10, 0x0030_6e24)?;
                c.g[V1][0] = 0x4d_0000;
            }
        }
        // Copy the default result record to *result.
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let a = (c.g[A3][0] as u32).wrapping_add(7);
        c.ldl(4, a);
        let a = c.g[A3][0] as u32;
        c.ldr(4, a);
        c.g[A1][0] = oc417_lw(c, A3, 8);
        let a = (c.g[S3][0] as u32).wrapping_add(7);
        c.sdl(4, a);
        let a = c.g[S3][0] as u32;
        c.sdr(4, a);
        oc417_sw(c, S3, 8, A1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptFn_AddMagnetModifier(Rec* result, int n, Ent* entries)`
/// (0x305478): like `WScriptFn_MultiParticleControl` but with the static
/// default record at `0x4fbd58` (object id -1, strength 400.0, radius 30.0)
/// and the parameter types at `0x4466a0`. When the resolved object has a
/// move node (`obj+0xc`) whose virtual `0x80` says it accepts magnets, adds
/// a magnet to it (`0x3559f8`: node, the node's virtual `0xc0` result,
/// strength, radius). Returns `result` with the default result record
/// (`0x4c9098`) copied in.
pub fn wscript_fn_add_magnet_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = oc417_lw(c, GP, 0x2750);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42a8));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = oc417_lbu(c, V1, 0xc);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x43c8_0000;
            c.mtc1(1, 1);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.mtc1(1, 0);
            let a = (c.g[A2][0] as u32).wrapping_sub(0x42a8);
            let v = c.g[V0][0];
            Addr(a).sw(c, v);
            let a = (c.g[V1][0] as u32).wrapping_add(0xc);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
            let a = (c.g[V1][0] as u32).wrapping_add(4);
            c.swc1(1, a);
            let a = (c.g[V1][0] as u32).wrapping_add(8);
            c.swc1(0, a);
            oc417_sw(c, GP, 0x2750, A0);
            let a = sp(c);
            let v = c.g[A1][0];
            Addr(a).sb(c, v);
        }
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = oc417_add(c.g[SP][0], 0x10);
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42a8));
        c.g[A2][0] = 0x10;
        oc417_sw(c, SP, 0x10, V0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, WSCRIPT_MEM_COPY, 0x0030_5504)?;
        let no_entries = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_entries {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = oc417_add(c.g[V0][0], 0x66a0);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = oc417_lw(c, A2, 0);
                c.g[V1][0] = oc417_lw(c, A2, 0xc);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = oc417_add(c.g[V0][0], c.g[S1][0]);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[T0][0]);
                c.g[V0][0] = oc417_lw(c, V0, 0);
                let convert = if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = oc417_lw(c, A2, 4);
                    false
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = oc417_lw(c, A2, 4);
                        false
                    } else {
                        c.cycles += 4;
                        let a = (c.g[A2][0] as u32).wrapping_add(4);
                        c.lwc1(0, a);
                        c.cvt_s_w(0, 0);
                        let a = c.g[A0][0] as u32;
                        c.swc1(0, a);
                        true
                    }
                };
                if !convert {
                    c.cycles += 1;
                    oc417_sw(c, A0, 0, V0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = oc417_add(c.g[A2][0], 0x10);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_5520);
                    }
                    continue;
                }
                break;
            }
        }
        // Resolve the object (rec[0]).
        c.cycles += 4;
        c.g[A2][0] = oc417_lw(c, S1, 0);
        c.g[V0][0] = !(c.g[A2][0]);
        let looked_up = c.g[V0][0] != 0;
        c.g[A1][0] = oc417_lw(c, GP, 0x16c8);
        if !looked_up {
            c.cycles += 3;
            c.g[V0][0] = oc417_lw(c, GP, 0xce8);
            c.g[V0][0] = oc417_lw(c, V0, 0x290);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = oc417_lw(c, A1, 0);
            c.g[A0][0] = oc417_lw(c, V1, 8);
            c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = oc417_lw(c, V0, 0);
            let no_slot = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_slot {
                c.cycles += 7;
                c.g[A0][0] = oc417_lw(c, V1, 0x1c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = oc417_add(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = oc417_lw(c, V0, 0);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // 'b_305624: the move node and the magnet.
        'magnet: {
            'result_v0: {
                c.cycles += 2;
                let no_object = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4d_0000;
                if no_object {
                    break 'result_v0;
                }
                c.cycles += 3;
                c.g[S0][0] = oc417_lw(c, V0, 0xc);
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = c.g[S3][0];
                if no_node {
                    break 'magnet;
                }
                c.cycles += 7;
                c.g[V1][0] = oc417_lw(c, S0, 0xc);
                c.g[A0][0] = oc417_lh(c, V1, 0x80);
                c.g[V0][0] = oc417_lw(c, V1, 0x84);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0030_55ec)?;
                let rejects = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4d_0000;
                if rejects {
                    break 'result_v0;
                }
                c.cycles += 11;
                c.g[V1][0] = oc417_lw(c, S0, 0xc);
                c.g[A0][0] = oc417_lh(c, V1, 0xc0);
                c.g[V0][0] = oc417_lw(c, V1, 0xc4);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = oc417_add(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, target, 0x0030_5608)?;
                let a = (c.g[S1][0] as u32).wrapping_add(8);
                c.lwc1(13, a);
                c.g[A0][0] = c.g[S0][0];
                let a = (c.g[S1][0] as u32).wrapping_add(4);
                c.lwc1(12, a);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0035_59f8, 0x0030_561c)?;
                c.g[V1][0] = 0x4d_0000;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let a = (c.g[A3][0] as u32).wrapping_add(7);
        c.ldl(4, a);
        let a = c.g[A3][0] as u32;
        c.ldr(4, a);
        c.g[A1][0] = oc417_lw(c, A3, 8);
        let a = (c.g[S3][0] as u32).wrapping_add(7);
        c.sdl(4, a);
        let a = c.g[S3][0] as u32;
        c.sdr(4, a);
        oc417_sw(c, S3, 8, A1);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared pieces of the WScriptFn_* argument decoders.

/// The argument-copy loop of the `WScriptFn_*` decoders: `a1` entries of 16
/// bytes `{index, value, _, kind}` at `a2` are written into the word buffer
/// at `buf_reg` (`buf[index] = value`, converted to float when the entry's
/// kind differs from the kind table's (`t0`) and that one is 2). `a3` is 2.
pub(crate) fn oc420_param_loop(c: &mut Cpu, buf_reg: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[buf_reg][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let mut convert = false;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
            } else {
                convert = true;
            }
        }
        if convert {
            c.cycles += 4;
            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(4));
            c.cvt_s_w(0, 0);
            c.swc1(0, c.g[A0][0] as u32);
        } else {
            c.cycles += 1;
            Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The tail the `WScriptFn_*` decoders share: copy the 12-byte default
/// result block at `0x4c9098` into the caller's return slot (`s3`, unaligned
/// `ldl/ldr/sdl/sdr` plus a word) and return it in `v0`. `v1` is 0x4d0000.
pub(crate) fn oc420_copy_result_block(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(8)).lw(c);
    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(7));
    c.sdr(4, c.g[S3][0] as u32);
    Addr((c.g[S3][0] as u32).wrapping_add(8)).sw(c, c.g[A1][0]);
}

// ---------------------------------------------------------------------------
// WScriptFn_RandRange.

/// `float Rand_FloatRange(float min, float max)` (0x317830).
pub(crate) const OC420_RAND_FLOAT_RANGE: Callee = Callee {
    addr: 0x0031_7830,
    f: ee::s10::f_00317830,
};

/// `void* WScriptFn_RandRange(Ctx* self, int n, Arg* args)` (0x303598): a
/// script function filling a 3-float parameter block (`{min, max, ..}`,
/// from a lazily initialised static at 0x4fbc60: 0, 1.0, flag byte) with
/// `n` 16-byte `{index, value, _, kind}` arguments (the value is converted
/// to float when the argument kind differs from the parameter's table kind
/// and that kind is 2), then stores a random float in `[min, max)` into the
/// result register of `self` (`self[2] = 2`, `LunoVMRegister_AsFloat`).
/// Returns `self`.
pub fn wscript_fn_rand_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x26c8)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_fpr(c, 20, 0x70);
        if !initialised {
            // First use: `static Params p = {0, 1.0f, flag}` (the guard
            // and the trailing byte copy included).
            c.cycles += 10;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x43a0));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lbu(c);
            c.g[A0][0] = 1;
            Addr(0x0050_0000u32.wrapping_sub(0x43a0)).sw(c, 0);
            Addr((c.g[V0][0] as u32).wrapping_add(8)).sb(c, c.g[V1][0]);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(4));
            Addr((c.g[GP][0] as u32).wrapping_add(0x26c8)).sw(c, c.g[A0][0]);
            Addr(sp(c)).sb(c, c.g[V1][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x43a0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3608)?;
        c.g[A2][0] = c.g[S3][0];
        if (c.g[S2][0] as i64) > 0 {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xc30));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S0, 0x0030_3620);
        }
        c.cycles += 18;
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(4));
        c.lwc1(12, c.g[S0][0] as u32);
        OC420_RAND_FLOAT_RANGE.call(c, 0x0030_3674)?;
        c.g[V0][0] = 2;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr((c.g[S1][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0022_6618, 0x0030_3688)?;
        c.swc1(20, c.g[V0][0] as u32);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_StopObjectLoop.

/// `SSXAudioSystem_GetInstance` (0x28b180) and
/// `SSXAudioSystem_StopObjectLoop` (0x297eb8).
pub(crate) const OC420_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

pub(crate) const OC420_AUDIO_STOP_OBJECT_LOOP: u32 = 0x0029_7eb8;

/// `Result WScriptFn_StopObjectLoop(Result* out, int n, Arg* args)`
/// (0x300260): decodes `n` arguments into `{object id, sound, loop}`
/// (static defaults at 0x4fbb28: `-1` objects, lazily initialised), resolves
/// the object (id `-1` is the current object `*(gp[0xce8] + 0x290)`;
/// otherwise through the world cache) and, if there is one, stops the
/// sound's loop on it. Always copies the 12-byte default result into `*out`.
pub fn wscript_fn_stop_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x257c)).lw(c);
        frame.save(c, S1, 0x50);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            // First use: `static Params p = {-1, -1, -1, flag}`.
            c.cycles += 12;
            c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44d8));
            c.g[V0][0] = u64::MAX;
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lbu(c);
            Addr(0x0050_0000u32.wrapping_sub(0x44d8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sb(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            Addr((c.g[V1][0] as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
            Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, c.g[A0][0]);
            Addr((c.g[GP][0] as u32).wrapping_add(0x257c)).sw(c, c.g[A1][0]);
            Addr(sp(c)).sb(c, c.g[A2][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x10;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x44d8));
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_02e0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S1][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(0x0044_0000u32.wrapping_add(0x6560));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S0, 0x0030_02f8);
        }
        // The object id (buf[0]): -1 means the current object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
            c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0;
            }
            c.cycles += 1;
            c.g[S2][0] = c.g[A1][0];
        }
        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[S2][0] != 0 {
            c.cycles += 9;
            c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
            c.g[S0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
            call_ptr(c, OC420_AUDIO_GET_INSTANCE, 0x0030_03c0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[S1][0];
            call_ptr(c, OC420_AUDIO_STOP_OBJECT_LOOP, 0x0030_03d4)?;
            c.g[V1][0] = 0x004d_0000;
        }
        oc420_copy_result_block(c);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptFn_AddParentModifierRev.

/// `cMoveNode_addParent` (0x355978).
pub(crate) const OC420_MOVE_NODE_ADD_PARENT: u32 = 0x0035_5978;

/// `Result WScriptFn_AddParentModifierRev(Result* out, int n, Arg* args)`
/// (0x2fdc60): decodes `n` arguments into `{child id, parent id, flags,
/// x, y, z}` (static defaults at 0x4fb758), resolves both objects (`-1` child
/// = the current object), and when the child's move node is not already of
/// kind 6 (`vtable+0x50`, `v10(6)`) and the parent's move node answers
/// `vtable+0x80`, adds the parent to its move node with the offset vector.
/// Copies the default result block into `*out`.
pub fn wscript_fn_add_parent_modifier_rev(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2520)).lw(c);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[A2][0] = 0x0050_0000;
        if c.g[V0][0] == 0 {
            // First use: `static Params p = {-1, -1, 0, 0.0, 0.0, 0.0, flag}`.
            c.cycles += 15;
            c.g[V0][0] = sx(0x0050_0000u32.wrapping_sub(0x48a8));
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lbu(c);
            c.mtc1(0, 0);
            c.g[A0][0] = 1;
            Addr(0x0050_0000u32.wrapping_sub(0x48a8)).sw(c, c.g[V1][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(0x18)).sb(c, c.g[A1][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
            Addr((c.g[GP][0] as u32).wrapping_add(0x2520)).sw(c, c.g[A0][0]);
            Addr((c.g[V0][0] as u32).wrapping_add(8)).sw(c, 0);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
            Addr(sp(c)).sb(c, c.g[A1][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x48a8));
        c.g[A2][0] = 0x1c;
        Addr(sp(c).wrapping_add(0x14)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x002f_dcf0)?;
        c.g[A2][0] = c.g[S2][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x0044_0000u32.wrapping_add(0x61c0));
            c.g[A3][0] = 2;
            oc420_param_loop(c, S1, 0x002f_dd08);
        }
        // The child object id (buf[0]): -1 means the current object.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let record = c.g[V1][0];
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if record != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
        }
        // The parent: only when buf[1] is not -1.
        c.cycles += 4;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
        c.g[V0][0] = !c.g[V0][0];
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0x004d_0000;
        } else {
            'done: {
                c.cycles += 3;
                c.g[S2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 6;
                if c.g[S2][0] != 0 {
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dde0)?;
                    c.g[V1][0] = 0x004d_0000;
                    if c.g[V0][0] != 0 {
                        break 'done;
                    }
                    // Parent object through the world cache.
                    c.cycles += 10;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x16c8)).lw(c);
                    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(4)).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let record = c.g[V1][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                    if record != 0 {
                        c.cycles += 7;
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let present = c.g[V0][0] != 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if !present {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x004d_0000;
                    if c.g[S0][0] == 0 {
                        break 'done;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_de58)?;
                    let answered = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if answered {
                        // Add the parent with the offset vector (x, y, z, 0).
                        c.cycles += 11;
                        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A3][0] = sx(sp(c).wrapping_add(0x30));
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x18)).lw(c);
                        c.swc1(2, sp(c).wrapping_add(0x30));
                        c.swc1(0, sp(c).wrapping_add(0x34));
                        c.swc1(1, sp(c).wrapping_add(0x38));
                        Addr(sp(c).wrapping_add(0x3c)).sw(c, 0);
                        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(8)).lw(c);
                        call_ptr(c, OC420_MOVE_NODE_ADD_PARENT, 0x002f_de8c)?;
                    }
                }
                // (also the exit when the child has no move node, or the
                // parent has none / does not answer)
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
        }
        oc420_copy_result_block(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptValue* WScriptFn_ObjectSlot36(ScriptValue* out, int n, Entry* e)`
/// (0x2ff9a8): script call "object slot 36". Builds an argument block of four
/// words (initialised to -1 on first use from the `0x4fbaf8` table, copied
/// with `EA_MemCopy`), fills `args[e.idx]` from the n entries (an int entry
/// whose slot type is 2 becomes a float), looks the object up by `args[0]`
/// (handle: low byte = slot, high = generation; through `gp[0x16c8]`) and
/// calls its method at vtable `+0x120` with `(args[1], args[2])`. Returns
/// `luno_gLunoNil` (copied into `out`).
#[allow(unreachable_code)]
pub fn w_script_fn_object_slot36(c: &mut Cpu) {
    guest(c, |c| {
        'b_2ffa08: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2550));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_2ffa08;
            } else {
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4508));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4508), c.g[V0][0]);
                c.g[A0][0] = 0xffffffffffffffff;
                c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                c.g[V0][0] = 0x1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2550), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_2ffa08;
            }
        }
        'b_2ffa88: {
            c.cycles += 10;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4508));
            c.g[A2][0] = 0x10;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x002ffa28)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_2ffa88;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6540));
                c.g[A3][0] = 0x2;
                'l_2ffa40: loop {
                    'b_2ffa7c: {
                        'b_2ffa78: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2ffa78;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_2ffa78;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_2ffa7c;
                                    } else {
                                        break 'b_2ffa78;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_2ffa7c;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002ffa40);
                        }
                        continue 'l_2ffa40;
                    } else {
                        break 'b_2ffa88;
                    }
                }
            }
        }
        'b_2ffae4: {
            'b_2ffaa4: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t {
                    break 'b_2ffaa4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    if t {
                        break 'b_2ffae4;
                    } else {
                        break 'b_2ffaa4;
                    }
                }
            }
            'b_2ffae0: {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                if t {
                    break 'b_2ffae0;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        break 'b_2ffae4;
                    } else {
                        break 'b_2ffae0;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_2ffae4;
        }
        'b_2ffb10: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[A2][0] == 0u64;
            c.g[V1][0] = 0x4d0000;
            if t {
                break 'b_2ffb10;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x120));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002ffb0c)?;
                c.g[V1][0] = 0x4d0000;
                break 'b_2ffb10;
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptValue* WScriptFn_AddCollectItems(ScriptValue* out, int n, Entry*
/// e)` (0x300f50): script call "add collect items". Builds a 22-word
/// argument block initialised to -1 (first call: the template at 0x4fbb78,
/// flag `gp[0x25c0]`; copied with `EA_MemCopy`), fills `args[e.idx]` from the
/// n entries (an int entry whose slot type, table 0x446570, is 2 becomes a
/// float), and, when `args[0]` is set, calls `WScriptMan_AddCollectItems(
/// gWScriptMan, args[0], args + 1, 20)`. Returns `luno_gLunoNil` in `out`.
#[allow(unreachable_code)]
pub fn w_script_fn_add_collect_items(c: &mut Cpu) {
    guest(c, |c| {
        'b_300ff4: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_300ff4;
            } else {
                c.cycles += 29;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4488));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x54));
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4488), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.sb((c.g[V1][0] as u32).wrapping_add(0x54), c.g[A1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x25c0), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_300ff4;
            }
        }
        'b_3010c0: {
            c.cycles += 29;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4488));
            c.g[A2][0] = 0x58;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
            call_ptr(c, 0x003e6574, 0x00301060)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_3010c0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6570));
                c.g[A3][0] = 0x2;
                'l_301078: loop {
                    'b_3010b4: {
                        'b_3010b0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_3010b0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_3010b0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_3010b4;
                                    } else {
                                        break 'b_3010b0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_3010b4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00301078);
                        }
                        continue 'l_301078;
                    } else {
                        break 'b_3010c0;
                    }
                }
            }
        }
        'b_3010e0: {
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            if t {
                break 'b_3010e0;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0] | 0x4;
                c.g[A3][0] = 0x14;
                Callee {
                    addr: 0x0030bac8,
                    f: ee::s10::WScriptMan_AddCollectItems,
                }
                .call(c, 0x003010e0)?;
                break 'b_3010e0;
            }
        }
        c.cycles += 16;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        Ok(())
    })
}

/// `RiderCommand* WScriptFn_RiderCommand(RiderCommand* out, int n, Ent* e)`
/// (0x2ff850): script function "rider command". On first use initialises the
/// static default command (`0x4fbae8`: id -1, two zero words, flag byte).
/// Copies it to a 16-byte stack record, overlays the `n` (index, value, ...)
/// entries of `e` (0x10 bytes each: `+0` slot, `+4` value, `+0xc` kind; a
/// value is converted to float when the slot's type in the table at
/// `0x446530` is 2 and the entry's kind differs), defaults the rider id from
/// the global at `gp + 0xce8` and calls `cRider_flagEventWorld(rider, v1,
/// v2)`; returns `out` filled from the 12-byte record at `0x4c9098`.
pub fn wscript_fn_rider_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0x254c));
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x50_0000;
        if !initialised {
            c.cycles += 10;
            c.g[V0][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)));
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = c.lbu(oc429_at(c.g[V0][0], 0xc));
            c.g[A0][0] = 1;
            c.sw(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)), c.g[V1][0]);
            c.sb(oc429_at(c.g[V0][0], 0xc), c.g[A1][0]);
            c.sw(oc429_at(c.g[GP][0], 0x254c), c.g[A0][0]);
            c.sw(oc429_at(c.g[V0][0], 0x4), 0);
            c.sw(oc429_at(c.g[V0][0], 0x8), 0);
            c.sb(sp(c), c.g[A1][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(oc429_at(c.g[SP][0], 0x10));
        c.g[A1][0] = sx(oc429_at(c.g[A2][0], 0u32.wrapping_sub(0x4518)));
        c.g[A2][0] = 0x10;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x002f_f8bc)?; // EA_MemCopy
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = 0x44_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(oc429_at(c.g[V0][0], 0x6530));
            c.g[A3][0] = 2;
            loop {
                'stored: {
                    'int_store: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        c.g[V1][0] = c.lw(oc429_at(c.g[A2][0], 0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx(oc429_at(c.g[V0][0], c.g[S0][0] as u32));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[T0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
                            break 'int_store;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(oc429_at(c.g[A2][0], 0x4));
                            break 'int_store;
                        }
                        c.cycles += 4;
                        c.lwc1(0, oc429_at(c.g[A2][0], 0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        break 'stored;
                    }
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(oc429_at(c.g[A1][0], 0u32.wrapping_sub(1)));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(oc429_at(c.g[A2][0], 0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_f8d8);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = u64::MAX;
        let have_rider = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
        if !have_rider {
            c.cycles += 5;
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0xce8));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw(oc429_at(c.g[GP][0], 0u32.wrapping_sub(0x848)));
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        }
        c.cycles += 24;
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A1][0] = c.lw(oc429_at(c.g[S0][0], 0x4));
        c.g[V0][0] = c.lw(oc429_at(c.g[A0][0], 0xc));
        c.lwc1(12, oc429_at(c.g[S0][0], 0x8));
        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x28));
        call_ptr(c, 0x0010_f1c0, 0x002f_f964)?; // cRider_flagEventWorld
        c.g[V1][0] = 0x4d_0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(oc429_at(c.g[V1][0], 0u32.wrapping_sub(0x6f68)));
        c.ldl(A0, oc429_at(c.g[A3][0], 0x7));
        c.ldr(A0, oc429_at(c.g[A3][0], 0));
        c.g[A1][0] = c.lw(oc429_at(c.g[A3][0], 0x8));
        c.sdl(A0, oc429_at(c.g[S3][0], 0x7));
        c.sdr(A0, oc429_at(c.g[S3][0], 0));
        c.sw(oc429_at(c.g[S3][0], 0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `CollectSet* WScriptFn_DefineCollectSet(CollectSet* out, int n, Entry* entries)`
/// (0x300e28): on first use initialise the static default set, copy it to the
/// stack, patch `n` (index, value, type) entries into it (type 2: store the
/// value as a float), define the collect set in the script manager when its first
/// word is not -1, and copy the 12-byte result to `out`.
#[allow(unreachable_code)]
pub fn w_script_fn_define_collect_set(c: &mut Cpu) {
    guest(c, |c| {
        'b_300e80: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25bc));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S2][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0x500000;
            if t {
                break 'b_300e80;
            } else {
                c.cycles += 10;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4498));
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x4498), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x25bc), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                break 'b_300e80;
            }
        }
        'b_300ef0: {
            c.cycles += 7;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4498));
            c.g[A2][0] = 0xc;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x00300e94)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_300ef0;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xb70));
                c.g[A3][0] = 0x2;
                'l_300ea8: loop {
                    'b_300ee4: {
                        'b_300ee0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_300ee0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_300ee0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_300ee4;
                                    } else {
                                        break 'b_300ee0;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_300ee4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00300ea8);
                        }
                        continue 'l_300ea8;
                    } else {
                        break 'b_300ef0;
                    }
                }
            }
        }
        'b_300f14: {
            c.cycles += 5;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[V1][0] = 0x4d0000;
            if t {
                break 'b_300f14;
            } else {
                c.cycles += 4;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                Callee {
                    addr: 0x0030b928,
                    f: ee::s10::WScriptMan_DefineCollectSet,
                }
                .call(c, 0x00300f10)?;
                c.g[V1][0] = 0x4d0000;
                break 'b_300f14;
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// GP-relative pointer of the counter's section table (`gp+0x16c8`, the world).
pub(crate) fn counter_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `WorldObjFactory_AttachModifier` (0x2fae38), reached by a direct call.
pub(crate) const COUNTER_ATTACH_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// Static 12-byte counter template (`D_B8D0`, `0x500000 - 0x4b98`).
pub(crate) const COUNTER_TEMPLATE: u32 = 0x004f_b468;

/// `WorldObjFactory_Counter` (0x2fb8d0), the C++ `MakeCounterFunc`. Builds a
/// counter record in a 12-byte stack buffer (the id word, then the parameter
/// values, stored as floats where the type table says so), finds the target
/// instance by id, and attaches a counter node to it. Returns the `luno` nil
/// value through the hidden pointer in `a0`. The first call copies the
/// static template; the template's own init runs once (the flag at gp+0x24e0).
pub fn world_obj_factory_counter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0x24e0)).lw(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x60);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = 0x0050_0000;
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x20);
        if flag_clear {
            // One-time init of the static template and its flag.
            c.cycles += 11;
            let tpl = COUNTER_TEMPLATE;
            c.g[V1][0] = sx(tpl);
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = Addr(tpl + 8).lbu(c);
            Addr(tpl).sw(c, u64::MAX);
            c.g[A0][0] = 3;
            let a1 = c.g[A1][0];
            Addr(tpl + 8).sb(c, a1);
            c.g[V0][0] = 1;
            let a0 = c.g[A0][0];
            Addr(tpl + 4).sw(c, a0);
            let v0 = c.g[V0][0];
            let flag = counter_gp(c).wrapping_add(0x24e0);
            Addr(flag).sw(c, v0);
            let a1 = c.g[A1][0];
            Addr(sp(c)).sb(c, a1);
        }

        // Block 2: the buffer's id word, then the template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(COUNTER_TEMPLATE);
        c.g[A2][0] = 0xc;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        // MEM_copy_B8D0(buf, template, 12).
        call_ptr(c, 0x003e_6574, 0x002f_b954)?;

        c.g[A2][0] = c.g[S3][0];
        if (c.g[S0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(counter_gp(c).wrapping_add(0xaf0));
            c.g[A3][0] = 2;
            loop {
                // One parameter: store it into buf[idx], as float if the
                // table asks for float and the type does not match.
                c.cycles += 8;
                let ent = c.g[A2][0] as u32;
                c.g[V0][0] = Addr(ent).lw(c);
                c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                let slot = c.g[A0][0] as u32;
                let mut store = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, ent.wrapping_add(4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, slot);
                        store = false;
                    }
                }
                if store {
                    c.cycles += 1;
                    let v0 = c.g[V0][0];
                    Addr(slot).sw(c, v0);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_b968);
                }
            }
        }

        // Block 3: resolve the instance from the id in buf[0].
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(counter_gp(c).wrapping_add(0x16c8)).lw(c);
        if c.g[V0][0] == 0 {
            // Id is -1: the instance is the script manager's.
            c.cycles += 3;
            c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                // Section entry: the instance's record from its id.
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }

        // Block 4: attach a counter node unless the instance is missing or
        // already has an active node.
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 7;
            let mut done = false;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                COUNTER_ATTACH_NODE.call(c, 0x002f_ba38)?;
                done = c.g[V0][0] != 0;
            }
            if !done {
                c.cycles += 11;
                c.g[A1][0] = 0x004a_0000;
                c.g[A0][0] = 0x24;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3be8));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                // EA_MemAllocNamed(36, D_004A3BE8, 0x20000000, 0).
                call_ptr(c, 0x0031_7d70, 0x002f_ba58)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                // Counter node constructor.
                call_ptr(c, 0x0034_3768, 0x002f_ba6c)?;
            }
        }

        // Epilogue: copy the nil value (12 bytes) to the return slot.
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
        let src = c.g[A1][0] as u32;
        c.ldl(2, src.wrapping_add(7));
        c.ldr(2, src);
        c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
        let dst = c.g[S2][0] as u32;
        c.sdl(2, dst.wrapping_add(7));
        c.sdr(2, dst);
        let v1 = c.g[V1][0];
        Addr(dst.wrapping_add(8)).sw(c, v1);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// Call-site constants of a factory's node allocation: the check flag for
/// `checkActiveNode`, the attach return, and the `cMemMan_alloc` size,
/// vtable, return, node constructor and its return.
pub(crate) struct NodeCall {
    flag: u64,
    attach_ret: u32,
    size: u64,
    vtable: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
}

/// The parameter loop of the counter-style factories (`MakeCounterFunc`,
/// `MakeFloatingFunc`): each parameter's value goes into `buf[idx]`, stored as
/// a float when the type table asks for one and the type does not match.
/// `pre` is the cycle charge before the loop, `table` the type table's
/// address, `head` the loop's event point. Runs only for a positive count.
pub(crate) fn wof_fill_params(c: &mut Cpu, pre: u64, table: u32, head: u32) {
    c.g[A2][0] = c.g[S3][0];
    if (c.g[S0][0] as i64) <= 0 {
        return;
    }
    c.cycles += pre;
    c.g[A1][0] = c.g[S0][0];
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// Block 3 of the counter-style factories: the instance for the id in
/// `buf[0]` (`S1`): the script manager for -1, else the section entry's
/// record; sets `S0` to it (0 when there is none).
pub(crate) fn wof_resolve_instance(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(counter_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    if c.g[V1][0] != 0 {
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = 0;
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[A2][0];
}

/// Block 4 of the counter-style factories: when there is an instance without
/// an active node, `checkActiveNode` (the attach) and, if that finds nothing,
/// a new node from `cMemMan_alloc` built by its constructor.
pub(crate) fn wof_attach_or_alloc(c: &mut Cpu, node: &NodeCall) -> Flow {
    c.cycles += 2;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A1][0] = node.flag;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[S1][0];
        COUNTER_ATTACH_NODE.call(c, node.attach_ret)?;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
    }
    c.cycles += 11;
    c.g[A0][0] = node.size;
    c.g[A1][0] = sx(node.vtable);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, node.alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S1][0];
    call_ptr(c, node.ctor, node.ctor_ret)?;
    Ok(())
}

/// Epilogue of the counter-style factories: copies the 12-byte `luno` nil
/// value (at 0x4c9098) to the hidden return slot `S2`, and leaves it in `v0`.
pub(crate) fn wof_copy_nil(c: &mut Cpu) {
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A1][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S2][0] as u32;
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    let v1 = c.g[V1][0];
    Addr(dst.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[S2][0];
}

/// `WorldObjFactory_Floating` (0x2fbab0), the C++ `MakeFloatingFunc`: the
/// counter's shape with a 32-byte buffer (the float template for speed,
/// scale, ...) and a 0x7c-byte node. Same protocol as
/// [`world_obj_factory_counter`].
pub fn world_obj_factory_floating(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b478;
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(counter_gp(c).wrapping_add(0x24e4)).lw(c);
        frame.save(c, S3, 0x50);
        frame.save(c, S0, 0x80);
        frame.save(c, S2, 0x60);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = 0x0050_0000;
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x30);
        if flag_clear {
            // One-time init of the float template and its flag.
            c.cycles += 20;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = Addr(TEMPLATE + 0x1c).lbu(c);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 0);
            c.g[V1][0] |= 0xffff;
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x4220_0000;
            c.mtc1(1, 1);
            c.lwc1(2, counter_gp(c).wrapping_sub(0x3394));
            let v1 = c.g[V1][0];
            Addr(TEMPLATE).sw(c, v1);
            let a1 = c.g[A1][0];
            Addr(TEMPLATE + 0x1c).sb(c, a1);
            c.swc1(1, TEMPLATE + 0xc);
            c.swc1(2, TEMPLATE + 0x14);
            c.swc1(0, TEMPLATE + 0x18);
            let a0 = c.g[A0][0];
            Addr(counter_gp(c).wrapping_add(0x24e4)).sw(c, a0);
            c.swc1(0, TEMPLATE + 4);
            c.swc1(0, TEMPLATE + 8);
            c.swc1(0, TEMPLATE + 0x10);
            let a1 = c.g[A1][0];
            Addr(sp(c)).sb(c, a1);
        }

        // Block 2: the id word, then the 32-byte template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A2][0] = 0x20;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        // MEM_copy_BAB0(buf, template, 32).
        call_ptr(c, 0x003e_6574, 0x002f_bb58)?;

        wof_fill_params(c, 4, 0x0044_5f28, 0x002f_bb70);
        wof_resolve_instance(c);
        wof_attach_or_alloc(
            c,
            &NodeCall {
                flag: 0xe,
                attach_ret: 0x002f_bc40,
                size: 0x7c,
                vtable: 0x0048_9698,
                alloc_ret: 0x002f_bc60,
                ctor: 0x0034_ec58,
                ctor_ret: 0x002f_bc74,
            },
        )?;
        wof_copy_nil(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptFn_EmptyHelper` (0x303d88): does nothing, returns to its caller.
pub fn wscript_fn_empty_helper(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WScriptFn_GetSelfHandle` (0x303430): stores the current task's `+0x78`
/// word into the calling script object's first word (or `-1` when the
/// script manager has no current process), marks the object as type 1 and
/// returns through `LunoVMRegister_AsInt` (0x226610).
pub fn wscript_fn_get_self_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x10));
        c.sq(16, s.wrapping_add(0x20));
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x290).lw(c);
        c.g[S1][0] = c.g[A0][0];
        // Both paths charge two cycles (the `-1` path after its block).
        c.cycles += 2;
        c.g[S0][0] = if c.g[V0][0] == 0 {
            u64::MAX
        } else {
            Addr(c.g[V0][0] as u32).off(0x78).lw(c)
        };
        // Tail: mark the object as type 1, then its int view.
        c.cycles += 11;
        let obj = c.g[S1][0] as u32;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        Addr(obj).off(0x8).sw(c, 1);
        call_ptr(c, 0x0022_6610, 0x0030_3470)?;
        let s0 = c.g[S0][0];
        let int_ptr = c.g[V0][0] as u32;
        Addr(int_ptr).sw(c, s0);
        c.g[V0][0] = c.g[S1][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x20));
        c.lq(17, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `WScriptFn_GetCurrentTaskId` (0x303490): the current task id of the
/// calling script's process (`WScriptProcess_GetCurrentTaskId`, 0x3079d8),
/// or `-1` when the manager has no process. Stores it into the calling
/// object's first word, marks the object as type 3, and returns through
/// `LunoVMRegister_AsInt`'s table slot (0x226620).
pub fn wscript_fn_get_current_task_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x10));
        c.sq(16, s.wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x2a4).lw(c);
        let mut s0 = u64::MAX;
        if c.g[A0][0] == 0 {
            c.cycles += 2;
        } else {
            c.cycles += 4;
            Callee {
                addr: 0x0030_79d8,
                f: ee::s10::f_003079d8,
            }
            .call(c, 0x0030_34bc)?;
            s0 = c.g[V0][0];
        }
        c.g[S0][0] = s0;
        // Tail: mark the object as type 3, then its int view (0x226620).
        c.cycles += 11;
        let obj = c.g[S1][0] as u32;
        c.g[V0][0] = 3;
        c.g[A0][0] = c.g[S1][0];
        Addr(obj).off(0x8).sw(c, 3);
        call_ptr(c, 0x0022_6620, 0x0030_34dc)?;
        let ptr = c.g[V0][0] as u32;
        Addr(ptr).sw(c, s0);
        c.g[V0][0] = c.g[S1][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x20));
        c.lq(17, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// The rider speed lookup both speed functions share (identical registers
/// in both, so one body): the world's rider table entry (`world+0x84`,
/// `+0xc` base, `+0x28` rider), the rider's `+0x6c0` block, whose fetch
/// function (`+0x14`, half-word offset `+0x10`) returns the vector address
/// in V0. `ret_to` is the return address of that fetch call.
pub(crate) fn rider_speed_fetch(c: &mut Cpu, ret_to: u32) -> Flow {
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    let s = sp(c);
    c.sq(16, s.wrapping_add(0x10));
    let ra = c.g[RA][0];
    c.sd(s, ra);
    c.g[S0][0] = c.g[A0][0];
    let s = sp(c);
    c.swc1(20, s.wrapping_add(0x20));
    c.g[A0][0] = Addr(c.g[GP][0] as u32).off(0xce8).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0;
    }
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
    if c.g[V0][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, ret_to)?;
    Ok(())
}

/// `WScriptFn_GetRiderSpeed` (0x303380): the rider's velocity length as a
/// VU0 `sqrt` of its squared components, stored as a float into the
/// calling object's +0x8 word (type 2) and returned through
/// `LunoVMRegister_AsFloat` (0x226618).
pub fn wscript_fn_get_rider_speed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        rider_speed_fetch(c, 0x0030_33dc)?;
        c.lqc2(3, c.g[V0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = 0x2;
        let s0 = c.g[S0][0] as u32;
        Addr(s0).off(0x8).sw(c, 2);
        call_ptr(c, 0x0022_6618, 0x0030_3414)?;
        let out = c.g[V0][0] as u32;
        c.swc1(20, out);
        c.g[V0][0] = c.g[S0][0];
        let s = sp(c);
        c.g[RA][0] = c.ld(s);
        let s = sp(c);
        c.lwc1(20, s.wrapping_add(0x20));
        c.lq(16, s.wrapping_add(0x10));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `WScriptFn_GetRiderSpeedScaled` (0x3032c0): the rider's velocity length
/// (the same VU0 maths as `GetRiderSpeed`) times the game timer period,
/// stored as a float into the calling object's +0x8 word (type 2) and
/// returned through `LunoVMRegister_AsFloat` (0x226618).
pub fn wscript_fn_get_rider_speed_scaled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        rider_speed_fetch(c, 0x0030_331c)?;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.lqc2(3, c.g[V0][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(4, 22);
        c.mtc1(4, 20);
        c.g[V0][0] = 0x2;
        c.g[A0][0] = c.g[S0][0];
        let v1 = c.g[V1][0] as u32;
        c.lwc1(0, v1.wrapping_add(0x14));
        let s0 = c.g[S0][0] as u32;
        Addr(s0).off(0x8).sw(c, 2);
        c.mul_s(20, 20, 0);
        call_ptr(c, 0x0022_6618, 0x0030_3360)?;
        let s = sp(c);
        c.g[RA][0] = c.ld(s);
        let out = c.g[V0][0] as u32;
        c.swc1(20, out);
        c.g[V0][0] = c.g[S0][0];
        let s = sp(c);
        c.lwc1(20, s.wrapping_add(0x20));
        c.lq(16, s.wrapping_add(0x10));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `gp + off` (the game's gp-relative globals, with `off` as a wrapping offset).
pub(crate) fn ws_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// Shared body of `WScriptFn_GameModeMatches` (0x303d90) and
/// `WScriptFn_EventMatches` (0x303e60): `a0 < limit` and the word at
/// `0x440000 + table` xor the byte at `0x530000 + byte` is zero. Otherwise 0.
/// Both `BE` calls return to `ret1` / `ret2`.
pub(crate) fn ws_mode_table_matches(
    c: &mut Cpu,
    limit: u64,
    table: u32,
    byte: u32,
    ret1: u32,
    ret2: u32,
) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < limit) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 14;
            call_ptr(c, 0x0014_dc80, ret1)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, ret2)?; // BELibrary_GetInterface(be, 0)
            c.g[V1][0] = 0x0044_0000;
            c.g[V0][0] = 0x0053_0000;
            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(table));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(byte)).lb(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V0][0] ^= c.g[A1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool WScriptFn_GameModeMatches(int mode)` (0x303d90): `mode < 4` and the
/// BE's table entry `0x4465e8[mode]` matches the byte `cGamePath` (0x535c11).
pub fn wscript_fn_game_mode_matches(c: &mut Cpu) {
    ws_mode_table_matches(c, 4, 0x0000_65e8, 0x0000_5c11, 0x0030_3db8, 0x0030_3dc4)
}

/// `bool WScriptFn_EventMatches(int event)` (0x303e60): as the game-mode test
/// for `event < 15`, with the table at 0x446618 and `cGameEvent` (0x535c12).
pub fn wscript_fn_event_matches(c: &mut Cpu) {
    ws_mode_table_matches(c, 0xf, 0x0000_6618, 0x0000_5c12, 0x0030_3e88, 0x0030_3e94)
}

/// `WScriptFn_IsHumanRider(luno var self)` (0x303ec8): sets the var's type to
/// 1 and its int to a flag from `WScriptMan->Being`, asking the luno var
/// helper at 0x226610 (`castInt`) for the int's address. A being of -1 gives
/// the flag 0; otherwise a function-table call on the being's record decides
/// it (its result xor 1).
pub fn wscript_fn_is_human_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(ws_gp(c, 0xce8)).lw(c);
        frame.save(c, S1, 0x10);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
        let no_being = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A0][0];
        if no_being {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 18;
            c.g[A0][0] = Addr(ws_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
            c.g[A1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            if c.g[A0][0] == 0 {
                c.g[V1][0] = 0;
            }
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
            if c.g[V1][0] == 0 {
                c.g[A0][0] = 0;
            }
            c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_3f2c)?;
            c.g[V0][0] ^= 1;
        }
        c.cycles += 2;
        let zero = c.g[V0][0] == 0;
        c.g[S0][0] = 1;
        if zero {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[A0][0] = c.g[S1][0];
            let sv = c.g[V0][0];
            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, sv);
            call_ptr(c, 0x0022_6610, 0x0030_3f64)?;
            Addr(c.g[V0][0] as u32).sw(c, 0);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            let sv = c.g[S0][0];
            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, sv);
            call_ptr(c, 0x0022_6610, 0x0030_3f4c)?;
            let sv = c.g[S0][0];
            Addr(c.g[V0][0] as u32).sw(c, sv);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The block fill shared by the `WScriptFn_SetPendingRequest*` bodies and
/// `WScriptFn_GameModeHandlerV07`: the static block at `block` gets a copy of
/// `copy_len` bytes (the copy call returns to `copy_ret`), then the loop
/// stores each parameter into it (a float when the table kind is 2 and the
/// kind differs). `s0` holds the block's stack copy, `a0` the parameter
/// count's base; the loop's event check reports `head`.
#[derive(Clone, Copy)]
pub(crate) struct FillBlock {
    /// The static block's address (`a1` of the copy call).
    block: u32,
    /// Bytes copied (`a2` of the copy call).
    copy_len: u64,
    /// Cycles charged on entering the loop (3 for the 12-byte fills, 4 for the 8-byte ones).
    enter_cycles: u64,
    /// Return address of the copy call.
    copy_ret: u32,
    /// gp offset of the parameter kind table.
    kind_table: u32,
    /// Loop head (the event check's pc).
    head: u32,
}

pub(crate) fn ws_fill_block(c: &mut Cpu, p: FillBlock) -> Flow {
    c.cycles += 7;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = p.copy_len;
    c.g[A1][0] = sx(p.block);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, p.copy_ret)?;
    let skip = (c.g[S1][0] as i64) <= 0;
    c.g[A2][0] = c.g[S2][0];
    if !skip {
        c.cycles += p.enter_cycles;
        c.g[A1][0] = c.g[S1][0];
        c.g[T0][0] = sx(ws_gp(c, p.kind_table));
        c.g[A3][0] = 2;
        loop {
            let s0 = c.g[S0][0] as u32;
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(s0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.cycles += 1;
                let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
                Addr(dst).sw(c, v);
            } else {
                c.cycles += 2;
                if c.g[V0][0] != c.g[A3][0] {
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                    c.cycles += 1;
                    let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
                    Addr(dst).sw(c, v);
                } else {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, c.g[A0][0] as u32);
                }
            }
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            let more = c.g[A1][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, p.head);
            }
        }
    }
    Ok(())
}

/// The return of the `WScriptFn_SetPendingRequest*` and `GameModeHandlerV07`
/// bodies: the 12-byte (or 8-byte) copy from the nil value at `0x4c9098` to
/// the sret pointer `s3`, then the frame's restore and return.
pub(crate) fn ws_sret_return(c: &mut Cpu, frame: Frame) {
    c.g[V1][0] = 0x004d_0000;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(A0, c.g[A3][0] as u32);
    c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
    c.sdl(A0, (c.g[S3][0] as u32).wrapping_add(0x7));
    c.sdr(A0, c.g[S3][0] as u32);
    let sret = (c.g[S3][0] as u32).wrapping_add(0x8);
    let v = c.g[A1][0];
    Addr(sret).sw(c, v);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// `WScriptFn_SetPendingRequest{A,B,C}`'s call: the block's two words go to
/// the setter with the WScriptMan pointer, then the sret return. The
/// setter's return address is `ret`.
pub(crate) fn ws_pending_call(c: &mut Cpu, frame: Frame, setter: Callee, ret: u32) -> Flow {
    c.cycles += 20;
    let s0 = c.g[S0][0] as u32;
    c.g[A2][0] = Addr(s0.wrapping_add(0x4)).lw(c);
    c.g[A1][0] = Addr(s0).lw(c);
    c.g[A0][0] = Addr(ws_gp(c, 0xce8)).lw(c);
    setter.call(c, ret)?;
    ws_sret_return(c, frame);
    Ok(())
}

/// `void WScriptFn_SetPendingRequestA(sret, int count, Param* params)`
/// (0x303ff0): on the first call clears the static request block (`0x4fbc98`:
/// its 0 word, -1 word and flag byte), then copies `count` parameters into
/// the block and passes its two words to `WScriptMan_SetPendingRequestA`.
pub fn wscript_fn_set_pending_request_a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x26f4)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x4368);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let sv = c.g[V1][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, sv);
            let sv = c.g[A0][0];
            Addr(ws_gp(c, 0x26f4)).sw(c, sv);
            let sv = c.g[A1][0];
            Addr(sp(c)).sb(c, sv);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x4368,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_4054,
                kind_table: 0xc50,
                head: 0x0030_4068,
            },
        )?;
        ws_pending_call(
            c,
            frame,
            Callee {
                addr: 0x0030_b4c0,
                f: ee::s10::f_0030b4c0,
            },
            0x0030_40c0,
        )
    })
}

/// `void WScriptFn_SetPendingRequestB(sret, int count, Param* params)`
/// (0x304100): the same as A with its own flag word (`gp+0x26f8`), block
/// (`0x4fbca8`), kind table and setter (`WScriptMan_SetPendingRequestB`).
pub fn wscript_fn_set_pending_request_b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x26f8)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A1][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A1][0] as u32).wrapping_sub(0x4358);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = 0x1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let sv = c.g[V1][0];
            Addr(ws_gp(c, 0x26f8)).sw(c, sv);
            let sv = c.g[A0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sb(c, sv);
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, 0);
            let sv = c.g[A0][0];
            Addr(sp(c)).sb(c, sv);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x4358,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_4164,
                kind_table: 0xc58,
                head: 0x0030_4178,
            },
        )?;
        ws_pending_call(
            c,
            frame,
            Callee {
                addr: 0x0030_b4d8,
                f: ee::s10::f_0030b4d8,
            },
            0x0030_41d0,
        )
    })
}

/// `void WScriptFn_SetPendingRequestC(sret, int count, Param* params)`
/// (0x304210): as A with flag `gp+0x26fc`, block `0x4fbcb8`, kind table
/// `gp+0xc60` and setter `WScriptMan_SetPendingRequestC`.
pub fn wscript_fn_set_pending_request_c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x26fc)).lw(c);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x4348);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let sv = c.g[V1][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, sv);
            let sv = c.g[A0][0];
            Addr(ws_gp(c, 0x26fc)).sw(c, sv);
            let sv = c.g[A1][0];
            Addr(sp(c)).sb(c, sv);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x4348,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_4274,
                kind_table: 0xc60,
                head: 0x0030_4288,
            },
        )?;
        ws_pending_call(
            c,
            frame,
            Callee {
                addr: 0x0030_b4f0,
                f: ee::s10::f_0030b4f0,
            },
            0x0030_42e0,
        )
    })
}

/// `void WScriptFn_GameModeHandlerV07(sret, int count, Param* params)`
/// (0x3044a8): the parameter block fill over the 8-byte static block at
/// `gp+0x2710` (first use: its -1 word, the flag `gp+0x2718`, a byte copy),
/// then the game mode manager's rider gate with the being's word (0x238550).
pub fn wscript_fn_game_mode_handler_v07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x2718)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        if !initialised {
            c.cycles += 6;
            c.g[V0][0] = Addr(ws_gp(c, 0x2714)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            let (word, flag) = (c.g[V1][0], c.g[A0][0]);
            Addr(ws_gp(c, 0x2710)).sw(c, word);
            Addr(ws_gp(c, 0x2718)).sw(c, flag);
            let byte = c.g[V0][0];
            Addr(sp(c)).sb(c, byte);
        }
        let block = ws_gp(c, 0x2710);
        ws_fill_block(
            c,
            FillBlock {
                block,
                copy_len: 8,
                enter_cycles: 4,
                copy_ret: 0x0030_4500,
                kind_table: 0xc70,
                head: 0x0030_4518,
            },
        )?;
        c.cycles += 22;
        c.g[V0][0] = Addr(ws_gp(c, 0xce8)).lw(c);
        c.g[V1][0] = Addr(ws_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc0)).lw(c);
        let s0 = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(s0).lw(c);
        call_ptr(c, 0x0023_8550, 0x0030_4578)?;
        ws_sret_return(c, frame);
        Ok(())
    })
}

/// `void WScriptFn_Nop_4ff0(sret, int count, Param* params)` (0x304ff0): the
/// parameter block fill over the static block `0x4fbd30` (first use clears
/// it and sets the flag `gp+0x2748`), then the nil return. Nothing else runs.
pub fn wscript_fn_nop_4ff0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x2748)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        c.g[A1][0] = 0x0050_0000;
        if !initialised {
            c.cycles += 8;
            let block = (c.g[A1][0] as u32).wrapping_sub(0x42d0);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = 0x1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lbu(c);
            let flag = c.g[V1][0];
            Addr(ws_gp(c, 0x2748)).sw(c, flag);
            let byte = c.g[A0][0];
            Addr((c.g[V0][0] as u32).wrapping_add(0x8)).sb(c, byte);
            Addr((c.g[V0][0] as u32).wrapping_add(0x4)).sw(c, 0);
            Addr(sp(c)).sb(c, byte);
        }
        ws_fill_block(
            c,
            FillBlock {
                block: 0x0050_0000 - 0x42d0,
                copy_len: 0xc,
                enter_cycles: 3,
                copy_ret: 0x0030_5054,
                kind_table: 0xc98,
                head: 0x0030_5068,
            },
        )?;
        c.cycles += 16;
        ws_sret_return(c, frame);
        Ok(())
    })
}

/// `void WScriptFn_ReplayHighlight(sret, int count, Param* params)`
/// (0x304b38): the parameter block fill over the 8-byte block at `gp+0x2728`
/// (first use: its -1 word, flag `gp+0x2730`, a byte copy), then one call
/// (0x2709d8) with the world's `+0x84 +0x28` record, the WScriptMan pointer's
/// word and the block's first word.
pub fn wscript_fn_replay_highlight(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(ws_gp(c, 0x2730)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let initialised = c.g[V0][0] != 0;
        if !initialised {
            c.cycles += 6;
            c.g[V0][0] = Addr(ws_gp(c, 0x272c)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            let (word, flag) = (c.g[V1][0], c.g[A0][0]);
            Addr(ws_gp(c, 0x2728)).sw(c, word);
            Addr(ws_gp(c, 0x2730)).sw(c, flag);
            let byte = c.g[V0][0];
            Addr(sp(c)).sb(c, byte);
        }
        let block = ws_gp(c, 0x2728);
        ws_fill_block(
            c,
            FillBlock {
                block,
                copy_len: 8,
                enter_cycles: 4,
                copy_ret: 0x0030_4b90,
                kind_table: 0xc80,
                head: 0x0030_4ba8,
            },
        )?;
        c.cycles += 23;
        c.g[V0][0] = Addr(ws_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[V1][0] = Addr(ws_gp(c, 0xce8)).lw(c);
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x28)).lw(c);
        let s0 = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(s0).lw(c);
        call_ptr(c, 0x0027_09d8, 0x0030_4c0c)?;
        ws_sret_return(c, frame);
        Ok(())
    })
}

/// The 12/16-byte `luno_gLunoNil` struct the WScript bodies return through
/// their struct return pointer (`sret`): the generated tail, verbatim
/// (unaligned `ldl`/`ldr` through `a0`, then a word; the same stores to `sret`).
pub(crate) fn wscript_return_nil(c: &mut Cpu, sret: u64) {
    c.g[V1][0] = 0x4d0000;
    c.g[V0][0] = sret;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (sret as u32).wrapping_add(0x7));
    c.sdr(4, sret as u32);
    c.sw((sret as u32).wrapping_add(0x8), c.g[A1][0]);
}

/// The `MEM_copy(vp, block, len)` call the WScript bodies open their param
/// block with: `vp` is `sp + 0x10` (kept in s0), `src` the block in `a1`.
pub(crate) fn wscript_copy_to_vp(
    c: &mut Cpu,
    cycles: u64,
    src: u64,
    len: u64,
    ret_addr: u32,
) -> Flow {
    c.cycles += cycles;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = len;
    c.g[A1][0] = src;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, ret_addr)
}

/// The `(params, n)` loop shared by the WScript bodies: each argument is
/// stored into the block at `vp` by its index, as a float when the kind
/// table says float and the argument's kind differs, else as an int.
/// `entry` is the cycle charge of the loop's entry block, `top` its back-edge
/// head, `table` the kind table. Leaves a0..a3, t0, v0, v1 as the loop does.
pub(crate) fn wscript_fill_block(
    c: &mut Cpu,
    vp: u64,
    count: u64,
    params: u64,
    table: u32,
    entry: u64,
    top: u32,
) {
    c.g[A2][0] = params;
    if (count as i64) <= 0 {
        return;
    }
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The zone-tag lookup of the four danger wrappers, from `vp` (the x word).
/// Returns true when the tag lookup ran (its result is in v0); false when
/// x is -1 and v0 (or v1, when `to_v1`) takes `*(gp+0xce8)->0x290`.
pub(crate) fn wscript_zone_lookup(c: &mut Cpu, vp: u64, to_v1: bool) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let t = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if !t {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        if to_v1 {
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        } else {
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        }
        return false;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if found {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return true;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    true
}

/// `WScriptFn_CareerAction` (0x306300): fills the once-initialised career
/// block at `vp` from the argument list (int or float by the kind table),
/// calls the BE interface with `SSXApp->BE` and 7, and when no network game
/// runs, `func_001E3510(x, y)`. Returns `luno_gLunoNil` through the sret.
pub fn wscript_fn_career_action(c: &mut Cpu) {
    guest(c, career_action_body)
}

pub(crate) fn career_action_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x279c));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A1][0] = 0x500000;
    if !initialised {
        c.cycles += 8;
        c.sw((c.g[A1][0] as u32).wrapping_sub(0x40c8), 0);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x40c8));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw(gp.wrapping_add(0x279c), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A0][0]);
    }
    let src = sx((c.g[A1][0] as u32).wrapping_sub(0x40c8));
    wscript_copy_to_vp(c, 7, src, 0xc, 0x0030_6364)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcc0), 3, 0x0030_6378);

    // The BE interface, then the network check.
    c.cycles += 10;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
    c.g[A1][0] = 0x7;
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
    c.g[S0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, 0x0014_dd58, 0x0030_63d8)?;
    c.g[V1][0] = 0x530000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
    let networked = c.g[V0][0] != 0;
    c.g[V1][0] = 0x4d0000;
    if !networked {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_3510, 0x0030_63f4)?;
        c.g[V1][0] = 0x4d0000;
    }

    c.cycles += 15;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_AddDanger` (0x306438): stores the argument block as above
/// (a -1 head, 16 bytes), looks up the zone tag for x, and when it is set
/// calls the danger manager's add (0x101728) on the tagged object.
pub fn wscript_fn_add_danger(c: &mut Cpu) {
    guest(c, add_danger_body)
}

pub(crate) fn add_danger_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27a0));
    c.sq(18, sp(c).wrapping_add(0x50));
    c.sq(17, sp(c).wrapping_add(0x60));
    c.g[S2][0] = c.g[A2][0];
    c.sq(19, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, sp(c).wrapping_add(0x70));
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x30), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 13;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x40b8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] |= 0xffff;
        c.g[AT][0] = 0x447a_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 0x1;
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x40b8), c.g[V0][0]);
        c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.sw(gp.wrapping_add(0x27a0), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x40b8));
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    wscript_copy_to_vp(c, 10, src, 0x10, 0x0030_64bc)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, 0x0044_6880, 5, 0x0030_64d8);

    wscript_zone_lookup(c, vp, false);
    // The tagged object's add.
    c.cycles += 2;
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
    if tagged {
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x8));
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw(sp(c).wrapping_add(0x20), c.g[V0][0]);
        c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1728, 0x0030_65a8)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x70));
    c.lq(17, sp(c).wrapping_add(0x60));
    c.lq(18, sp(c).wrapping_add(0x50));
    c.lq(19, sp(c).wrapping_add(0x40));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x30));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    ret(c);
    Ok(())
}

/// `WScriptFn_RemoveDanger` (0x3065e8): the remove twin of the add body, with
/// the 8-byte block at `gp-0x...` and the manager's remove (0x101888).
pub fn wscript_fn_remove_danger(c: &mut Cpu) {
    guest(c, remove_danger_body)
}

pub(crate) fn remove_danger_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27b0));
    c.sq(17, sp(c).wrapping_add(0x60));
    c.sq(18, sp(c).wrapping_add(0x50));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x40));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, sp(c).wrapping_add(0x70));
    c.g[S3][0] = c.g[A0][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x30), c.g[RA][0]);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x27ac));
        c.g[V1][0] = u64::MAX;
        c.g[V1][0] |= 0xffff;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x27a8), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x27b0), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    let src = sx(gp.wrapping_add(0x27a8));
    wscript_copy_to_vp(c, 10, src, 0x8, 0x0030_6650)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcc8), 4, 0x0030_6668);

    wscript_zone_lookup(c, vp, false);
    c.cycles += 2;
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = sx(sp(c).wrapping_add(0x20));
    if tagged {
        c.cycles += 7;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
        c.sw(sp(c).wrapping_add(0x20), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1888, 0x0030_6730)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x70));
    c.lq(17, sp(c).wrapping_add(0x60));
    c.lq(18, sp(c).wrapping_add(0x50));
    c.lq(19, sp(c).wrapping_add(0x40));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x30));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    ret(c);
    Ok(())
}

/// `WScriptFn_AcquireDangerZone` (0x306770): the zone-acquire wrapper. The
/// tag lookup's result goes to v1, and a set tag calls the manager's acquire
/// (0x101900) with the argument at `vp + 4`.
pub fn wscript_fn_acquire_danger_zone(c: &mut Cpu) {
    guest(c, acquire_danger_zone_body)
}

pub(crate) fn acquire_danger_zone_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27b4));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.g[S2][0] = c.g[A2][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x40a8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x40a8), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x27b4), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x40a8));
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    wscript_copy_to_vp(c, 10, src, 0xc, 0x0030_67e8)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcd0), 4, 0x0030_6800);

    if wscript_zone_lookup(c, vp, true) {
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    // The tagged object's acquire.
    c.cycles += 2;
    let tagged = c.g[V1][0] != 0;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
    if tagged {
        c.cycles += 6;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x78));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1900, 0x0030_68c8)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_ReleaseDangerZone` (0x306908): the zone-release twin of the
/// acquire body, calling the manager's release (0x101970).
pub fn wscript_fn_release_danger_zone(c: &mut Cpu) {
    guest(c, release_danger_zone_body)
}

pub(crate) fn release_danger_zone_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27c0));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x27bc));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x27b8), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x27c0), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    let src = sx(gp.wrapping_add(0x27b8));
    wscript_copy_to_vp(c, 10, src, 0x8, 0x0030_6970)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xcd8), 4, 0x0030_6988);

    if wscript_zone_lookup(c, vp, true) {
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    c.cycles += 2;
    let tagged = c.g[V1][0] != 0;
    c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
    if tagged {
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x78));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa8));
        call_ptr(c, 0x0010_1970, 0x0030_6a4c)?;
    }

    c.cycles += 16;
    let sret = c.g[S3][0];
    wscript_return_nil(c, sret);
    c.g[V0][0] = c.g[S3][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_CheckRiderGate` (0x306e68): fills the block from the argument
/// list, asks `WScriptMan_CheckRiderGate` (0x30bfc0) for the gate; when the
/// gate is open and the tagged entry has a flag word, it calls the two
/// helpers (0x28b180, 0x2974a0). Then it reports the result to 0x226610.
pub fn wscript_fn_check_rider_gate(c: &mut Cpu) {
    guest(c, check_rider_gate_body)
}

pub(crate) fn check_rider_gate_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27cc));
    c.sq(19, sp(c).wrapping_add(0x30));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.g[S3][0] = c.g[A2][0];
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S2][0] = c.g[A0][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 11;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3fa0));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x3fa0), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0xc), c.g[A1][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.sw(gp.wrapping_add(0x27cc), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sb(sp(c), c.g[A1][0]);
    }
    // The block copy: vp = sp + 0x10, 16 bytes, with the three -1 heads.
    c.g[V0][0] = u64::MAX;
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x3fa0));
    c.sw(sp(c).wrapping_add(0x18), c.g[V0][0]);
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    c.sw(sp(c).wrapping_add(0x14), c.g[V0][0]);
    wscript_copy_to_vp(c, 12, src, 0x10, 0x0030_6eec)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S3][0]);
    wscript_fill_block(c, vp, count, params, 0x0044_6980, 5, 0x0030_6f08);

    // The gate check, then the tagged entry's flag.
    c.cycles += 9;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    Callee {
        addr: 0x0030_bfc0,
        f: ee::s10::f_0030bfc0,
    }
    .call(c, 0x0030_6f64)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V0][0] = 0x1;
    let open = c.g[S1][0] == c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    if open {
        let mut to_report = false;
        c.cycles += 10;
        c.g[V1][0] = c.lw(gp.wrapping_add(0x16c8));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0;
            to_report = true;
        }
        if !to_report {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let zero = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !zero {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A1][0];
        }
        // The flag word: call the two helpers when it is set.
        c.cycles += 2;
        let flagged = c.g[S0][0] != 0;
        c.g[V0][0] = 0x1;
        if flagged {
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x0030_6fd8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x5c;
            call_ptr(c, 0x0029_74a0, 0x0030_6fe8)?;
            c.g[V0][0] = 0x1;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S2][0];
    }

    c.cycles += 11;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0022_6610, 0x0030_6ff8)?;
    c.sw(c.g[V0][0] as u32, c.g[S1][0]);
    c.g[V0][0] = c.g[S2][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `WScriptFn_CareerCheck` (0x307020): fills the career-check block (the
/// same copy and argument loop), asks `func_001E3570` for the check result,
/// stores a 1 and the (result > 0) flag into the object, and returns it.
pub fn wscript_fn_career_check(c: &mut Cpu) {
    guest(c, career_check_body)
}

pub(crate) fn career_check_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(0x27d0));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.g[S2][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S1][0] = c.g[A0][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[A2][0];
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    let initialised = c.g[V0][0] != 0;
    c.g[A1][0] = 0x500000;
    if !initialised {
        c.cycles += 8;
        c.sw((c.g[A1][0] as u32).wrapping_sub(0x3f90), 0);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3f90));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw(gp.wrapping_add(0x27d0), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A0][0]);
    }
    let src = sx((c.g[A1][0] as u32).wrapping_sub(0x3f90));
    wscript_copy_to_vp(c, 7, src, 0xc, 0x0030_7084)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S2][0], c.g[S3][0]);
    wscript_fill_block(c, vp, count, params, gp.wrapping_add(0xce0), 3, 0x0030_7098);

    c.cycles += 17;
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, 0x001e_3570, 0x0030_70ec)?;
    c.g[V1][0] = 0x1;
    c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0022_6610, 0x0030_7100)?;
    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    c.g[V0][0] = c.g[S1][0];
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
    Ok(())
}

/// `gp` of the game (the world and script-manager globals are gp-relative).
pub(crate) fn wsm_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// The `memcpy` the modifier builders copy their template with.
pub(crate) const WSM_MEMCPY: u32 = 0x003e_6574;

/// `isMoveNode`'s return and the node-add call of one `WScriptFn_Add*Modifier`:
/// the two builders share one shape and differ in these constants.
pub(crate) struct ModSite {
    /// Frame size (`addiu sp, sp, -size`).
    frame: u32,
    /// Stack offsets of the saved `s0`..`s3` (`sq`) and of `ra` (`sd`).
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    ra: u32,
    /// `gp` offset of the one-time init flag.
    flag_gp: u32,
    /// The static instance template (`0x500000 - off`, the C++ `D_E4A8`).
    tpl: u32,
    /// Template size in bytes (the stack buffer's size).
    copy_len: u64,
    /// Return address of the template copy.
    copy_ret: u32,
    /// The parameter type table (`D_00446268` and friends).
    types: u32,
    /// Cycles charged before the parameter loop.
    fill_pre: u64,
    /// Event point of the parameter loop's back edge.
    fill_head: u32,
    /// Return address of the `isMoveNode` call.
    is_move_ret: u32,
    /// The node-add function and its return address.
    move_fn: u32,
    move_ret: u32,
}

/// Constants of `WScriptFn_AddTexFlipModifier` (0x2fe4a8): a 24-byte record
/// template at `0x4fb818`, the flag at `gp+0x2530`.
pub(crate) const TEX_SITE: ModSite = ModSite {
    frame: 0x80,
    s0: 0x70,
    s1: 0x60,
    s2: 0x50,
    s3: 0x40,
    ra: 0x30,
    flag_gp: 0x2530,
    tpl: 0x004f_b818,
    copy_len: 0x18,
    copy_ret: 0x002f_e534,
    types: 0x0044_6268,
    fill_pre: 5,
    fill_head: 0x002f_e550,
    is_move_ret: 0x002f_e614,
    move_fn: 0x0035_5bf0,
    move_ret: 0x002f_e624,
};

/// `WScriptFn_AddBoostModifier` (0x2fe668): the same shape with a 36-byte
/// template at `0x4fb830`.
pub(crate) const BOOST_SITE: ModSite = ModSite {
    frame: 0x90,
    s0: 0x80,
    s1: 0x70,
    s2: 0x60,
    s3: 0x50,
    ra: 0x40,
    flag_gp: 0x2534,
    tpl: 0x004f_b830,
    copy_len: 0x24,
    copy_ret: 0x002f_e710,
    types: 0x0044_6280,
    fill_pre: 4,
    fill_head: 0x002f_e728,
    is_move_ret: 0x002f_e7ec,
    move_fn: 0x0035_5c50,
    move_ret: 0x002f_e7fc,
};

/// The one-time init of the tex-flip template (flag clear): the instance
/// record's id word, its type/value bytes and the 2.0f scale, then the flag.
pub(crate) fn wsm_tex_init(c: &mut Cpu, s: &ModSite) {
    c.cycles += 15;
    c.g[V1][0] = sx(s.tpl);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lbu(c);
    c.g[V0][0] |= 0xffff;
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = 1;
    let v0 = c.g[V0][0];
    Addr(s.tpl).sw(c, v0);
    let a1 = c.g[A1][0];
    Addr(s.tpl + 0x14).sb(c, a1);
    c.swc1(0, s.tpl + 0x10);
    let a0 = c.g[A0][0];
    Addr(wsm_gp(c).wrapping_add(s.flag_gp)).sw(c, a0);
    Addr(s.tpl + 4).sw(c, 0);
    Addr(s.tpl + 8).sw(c, 0);
    Addr(s.tpl + 0xc).sw(c, 0);
    let a1 = c.g[A1][0];
    Addr(sp(c)).sb(c, a1);
}

/// The one-time init of the boost template (flag clear): the id word, the
/// float fields (0.1f, 20.0f, 0.0f, 1.0f from the table in gp), then the flag.
pub(crate) fn wsm_boost_init(c: &mut Cpu, s: &ModSite) {
    c.cycles += 22;
    c.g[V0][0] = sx(s.tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lbu(c);
    c.g[V1][0] |= 0xffff;
    c.mtc1(0, 0);
    c.g[A0][0] = 1;
    c.lwc1(1, wsm_gp(c).wrapping_sub(0x3380));
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    let v1 = c.g[V1][0];
    Addr(s.tpl).sw(c, v1);
    let a1 = c.g[A1][0];
    Addr(s.tpl + 0x20).sb(c, a1);
    c.swc1(1, s.tpl + 0xc);
    c.swc1(2, s.tpl + 0x10);
    c.swc1(0, s.tpl + 0x18);
    c.swc1(3, s.tpl + 0x1c);
    let a0 = c.g[A0][0];
    Addr(wsm_gp(c).wrapping_add(s.flag_gp)).sw(c, a0);
    Addr(s.tpl + 4).sw(c, a0);
    c.swc1(0, s.tpl + 8);
    c.swc1(0, s.tpl + 0x14);
    let a1 = c.g[A1][0];
    Addr(sp(c)).sb(c, a1);
}

/// The parameter loop: each `(idx, val, type)` entry at `A2` (count `S0`)
/// stores its value into `buf[idx]`, as a float when the type table asks for
/// one and the entry's type does not match.
pub(crate) fn wsm_fill(c: &mut Cpu, s: &ModSite) {
    if (c.g[S0][0] as i64) <= 0 {
        return;
    }
    c.cycles += s.fill_pre;
    c.g[V0][0] = 0x0044_0000;
    c.g[A1][0] = c.g[S0][0];
    c.g[T0][0] = sx(s.types);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, s.fill_head);
        }
    }
}

/// The instance for the id in `buf[0]` (`S1`): the script manager's for -1,
/// else the section entry's record (`0` when there is none). Leaves it in V0.
pub(crate) fn wsm_resolve(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wsm_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wsm_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// The node add: when the instance (V0) has an active node that answers
/// `isMoveNode` non-zero, the node-add function runs on it with `buf` (`S1`).
pub(crate) fn wsm_move_node(c: &mut Cpu, s: &ModSite) -> Flow {
    c.cycles += 3;
    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[V1][0] = 0x004d_0000;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 7;
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    let is_move = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, is_move, s.is_move_ret)?;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, s.move_fn, s.move_ret)?;
    }
    c.cycles += 1;
    c.g[V1][0] = 0x004d_0000;
    Ok(())
}

/// Epilogue: copies the 12-byte `luno` nil value to the hidden return slot
/// (`S3`), leaves that pointer in V0, restores the saved registers and returns.
pub(crate) fn wsm_epilogue(c: &mut Cpu, s: &ModSite, frame: Frame) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S3][0] as u32;
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
    frame.restore(c, S0, s.s0);
    frame.restore(c, S1, s.s1);
    frame.restore(c, S2, s.s2);
    frame.restore(c, S3, s.s3);
    frame.restore_ra(c, s.ra);
    frame.close(c);
}

/// The shared body of the two `WScriptFn_Add*Modifier`: builds the instance
/// record from the parameter list in a stack buffer (copied from the static
/// template, with the one-time `init` when the flag is clear) and adds it to
/// the instance's node list.
pub(crate) fn wsm_add_modifier(c: &mut Cpu, s: &ModSite, init: fn(&mut Cpu, &ModSite)) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, s.frame);
    c.g[V0][0] = Addr(wsm_gp(c).wrapping_add(s.flag_gp)).lw(c);
    frame.save(c, S2, s.s2);
    frame.save(c, S0, s.s0);
    frame.save(c, S3, s.s3);
    frame.save(c, S1, s.s1);
    frame.save_ra(c, s.ra);
    c.g[S2][0] = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    c.g[S3][0] = c.g[A0][0];
    let flag_clear = c.g[V0][0] == 0;
    c.g[A2][0] = 0x0050_0000;
    if flag_clear {
        init(c, s);
    }

    // The id word of the buffer, then the template copy.
    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx(s.tpl);
    c.g[A2][0] = s.copy_len;
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, WSM_MEMCPY, s.copy_ret)?;

    c.g[A2][0] = c.g[S2][0];
    wsm_fill(c, s);
    wsm_resolve(c);
    wsm_move_node(c, s)?;
    wsm_epilogue(c, s, frame);
    Ok(())
}

/// `WScriptFn_AddTexFlipModifier` (0x2fe4a8), the C++ `AddTexFlipFunc`:
/// adds a tex-flip modifier built from the script's parameter list to the
/// instance named by the first parameter (or the script manager's, for -1).
pub fn wscript_add_tex_flip_modifier(c: &mut Cpu) {
    guest(c, |c| wsm_add_modifier(c, &TEX_SITE, wsm_tex_init));
}

/// `WScriptFn_AddBoostModifier` (0x2fe668), the C++ `AddBoostFunc`: the same
/// as the tex-flip builder with a boost record (speed, time and scale floats).
pub fn wscript_add_boost_modifier(c: &mut Cpu) {
    guest(c, |c| wsm_add_modifier(c, &BOOST_SITE, wsm_boost_init));
}

/// The 0x70-byte WScript frame and prologue shared by these eight bodies:
/// saves s0..s3/ra, moves the three arguments into s1/s2/s3 (`regs`: the
/// argument register each of s1, s2, s3 takes; the bodies differ in order),
/// loads the once-init guard at `gp+guard`, and (`set_a2`) puts 0x500000 in
/// a2 as the static-block base. Returns whether the guard was already set.
pub(crate) fn oc732_prologue(
    c: &mut Cpu,
    cycles: u64,
    guard: u32,
    regs: [usize; 3],
    set_a2: bool,
) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(guard));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[regs[0]][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[regs[1]][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[regs[2]][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    if set_a2 {
        c.g[A2][0] = 0x500000;
    }
    initialised
}

/// `MEM_copy(vp, src, len)` with `vp = sp + 0x10` (s0), then the
/// `(params, count)` fill loop of the block. `prefill` is the variant that
/// first stores -1 at `vp` (v0 = -1).
#[allow(clippy::too_many_arguments)]
pub(crate) fn oc732_copy_fill(
    c: &mut Cpu,
    cycles: u64,
    prefill: bool,
    src: u64,
    len: u64,
    ret_addr: u32,
    regs: (usize, usize),
    table: u32,
    entry: u64,
    top: u32,
) -> Flow {
    c.cycles += cycles;
    if prefill {
        c.g[V0][0] = u64::MAX;
    }
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = len;
    c.g[A1][0] = src;
    c.g[A0][0] = c.g[S0][0];
    if prefill {
        c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    }
    call_ptr(c, 0x003e_6574, ret_addr)?;
    let (vp, count, params) = (c.g[S0][0], c.g[regs.0][0], c.g[regs.1][0]);
    oc732_fill_block(c, vp, count, params, table, entry, top);
    Ok(())
}

/// The `(params, n)` loop of the WScript bodies: each argument is stored into
/// the block at `vp` by its index, as a float when the kind table says float
/// and the argument's kind differs, else as an int. `entry` is the cycle
/// charge of the loop's entry block, `top` its back-edge head.
pub(crate) fn oc732_fill_block(
    c: &mut Cpu,
    vp: u64,
    count: u64,
    params: u64,
    table: u32,
    entry: u64,
    top: u32,
) {
    c.g[A2][0] = params;
    if (count as i64) <= 0 {
        return;
    }
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The zone-tag lookup of the danger/slot wrappers, from `vp` (the x word):
/// the tag table at `gp+0x16c8` maps x (low byte, then the next bits) to a
/// small index in v0; with x = -1, v0 is `*(gp+0xce8)->0x290`. Leaves a1, a2,
/// v0, v1, a0 as the generated block does. `x_cycles` is the block charge of
/// the x = -1 path (4 in 0948, 3 in the slot getter).
pub(crate) fn oc732_zone_lookup(c: &mut Cpu, vp: u64, x_cycles: u64) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if !tagged {
        c.cycles += x_cycles;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        return false;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if found {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return true;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    true
}

/// The `luno_gLunoNil` struct return (12 bytes through `sret`), verbatim.
pub(crate) fn oc732_return_nil(c: &mut Cpu, sret: u64) {
    c.g[V1][0] = 0x4d0000;
    c.g[V0][0] = sret;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (sret as u32).wrapping_add(0x7));
    c.sdr(4, sret as u32);
    c.sw((sret as u32).wrapping_add(0x8), c.g[A1][0]);
}

/// Restore s0..s3, ra and the 0x70 frame, return.
pub(crate) fn oc732_epilogue(c: &mut Cpu) {
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
}

/// `WScriptFn_PlaceRiderAtObject` (0x300948): fills an 8-byte block (x), looks
/// up the tagged object for x (x = -1: the current one, `*(gp+0xce8)->0x290`),
/// and when the rider index at `*(gp+0xce8)` is not negative calls the
/// object's vtable method and the world's rider (`world->0x84->0xc[idx]`)
/// vtable method with it. Returns `luno_gLunoNil` through sret (a0).
pub fn wscript_fn_place_rider_at_object(c: &mut Cpu) {
    guest(c, place_rider_body)
}

pub(crate) fn place_rider_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, 0x25b0, [A1, A2, A0], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x25ac));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x25a8), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x25b0), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(0x25a8));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0x8,
        0x0030_09b0,
        (S1, S2),
        gp.wrapping_add(0xb58),
        4,
        0x0030_09c8,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc732_zone_lookup(c, vp, 4);
    if !tagged {
        // x == -1: a2 = v0 = *(gp+0xce8); v0 = a2->0x290.
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    } else {
        c.cycles += 1;
        c.g[A2][0] = c.lw(gp.wrapping_add(0xce8));
    }

    c.cycles += 3;
    c.g[S0][0] = c.lw(c.g[A2][0] as u32);
    let negative = (c.g[S0][0] as i64) < 0;
    c.g[V1][0] = 0x4d0000;
    let mut skip_move = false;
    if !negative {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.g[S3][0];
            skip_move = true;
        } else {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let no_obj = c.g[A2][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            if !no_obj {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_0aa8)?;
            }
            c.cycles += 14;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x60));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, target, 0x0030_0adc)?;
            c.g[V1][0] = 0x4d0000;
        }
    }
    if !skip_move {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
    }

    c.cycles += 14;
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_SetRailEnabled` (0x300b20): fills a 12-byte block (x, mode) and,
/// when x is not -1, `RailMan_SetRailEnabled(x, 1)` for mode 1, or
/// `(x, 0)` for mode 0 (other modes do nothing). Returns `luno_gLunoNil`.
pub fn wscript_fn_set_rail_enabled(c: &mut Cpu) {
    guest(c, set_rail_enabled_body)
}

pub(crate) fn set_rail_enabled_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 12, 0x25b4, [A1, A2, A0], true);
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44b8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw(0x0050_0000u32.wrapping_sub(0x44b8), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x25b4), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx(0x0050_0000u32.wrapping_sub(0x44b8));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0xc,
        0x0030_0b98,
        (S1, S2),
        gp.wrapping_add(0xb60),
        4,
        0x0030_0bb0,
    )?;

    c.cycles += 4;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[V0][0] = !(0u64 | c.g[A0][0]);
    let rail = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1;
    if rail {
        // x == -1: nothing to set.
        c.cycles += 1;
        c.g[V1][0] = 0x4d0000;
    } else {
        c.cycles += 3;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 4;
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0035_8b28, 0x0030_0c1c)?;
            c.g[V1][0] = 0x4d0000;
        } else {
            c.cycles += 2;
            let other = c.g[V1][0] != 0;
            c.g[V1][0] = 0x4d0000;
            if !other {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0035_8b28, 0x0030_0c34)?;
                c.cycles += 1;
                c.g[V1][0] = 0x4d0000;
            }
        }
    }

    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_PlayScriptList` (0x300c78): fills a 12-byte block (x, mode) and
/// drives the script engine's list (`gp-0x84c`): mode 0 adds a movie
/// (`ListAddMovieById`) and prepares, mode 1 adds a choice
/// (`ListAddChoice`) and prepares, mode 2 starts the list; other modes do
/// nothing. Returns `luno_gLunoNil`.
pub fn wscript_fn_play_script_list(c: &mut Cpu) {
    guest(c, play_script_list_body)
}

pub(crate) fn play_script_list_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 12, 0x25b8, [A1, A2, A0], true);
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x44a8));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw(0x0050_0000u32.wrapping_sub(0x44a8), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x25b8), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx(0x0050_0000u32.wrapping_sub(0x44a8));
    oc732_copy_fill(
        c,
        7,
        false,
        src,
        0xc,
        0x0030_0ce4,
        (S1, S2),
        gp.wrapping_add(0xb68),
        3,
        0x0030_0cf8,
    )?;

    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = 0x1;
    let mode1 = c.g[V1][0] == c.g[V0][0];
    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
    // 0: add movie + prepare; 1: add choice + prepare; 2: start; 3: nothing.
    let step;
    if mode1 {
        step = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        let ge2 = c.g[V0][0] == 0;
        c.g[V0][0] = 0x2;
        if !ge2 {
            c.cycles += 2;
            let zero = c.g[V1][0] == 0;
            c.g[V1][0] = 0x4d0000;
            if zero {
                step = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[S3][0];
                step = 3;
            }
        } else {
            c.cycles += 2;
            let two = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = 0x4d0000;
            if two {
                step = 2;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[S3][0];
                step = 3;
            }
        }
    }
    match step {
        0 | 1 => {
            c.cycles += 9;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x84c));
            c.g[A1][0] = 0;
            c.g[A3][0] = 0x3;
            if step == 0 {
                c.g[T0][0] = u64::MAX;
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = u64::MAX;
                call_ptr(c, 0x0027_8ed0, 0x0030_0d98)?;
            } else {
                c.g[T0][0] = 0;
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = u64::MAX;
                c.g[T3][0] = u64::MAX;
                call_ptr(c, 0x0027_8e50, 0x0030_0dc0)?;
            }
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x84c));
            c.cycles += 4;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0027_8f38, 0x0030_0dcc)?;
            c.g[V1][0] = 0x4d0000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        2 => {
            c.cycles += 6;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x84c));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x1;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0027_8f68, 0x0030_0de8)?;
            c.g[V1][0] = 0x4d0000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        _ => {}
    }

    c.cycles += 14;
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_Nop_1440` (0x301440): fills a 12-byte block (x, a flag) from the
/// argument list and does nothing with it (a retired script command). Returns
/// `luno_gLunoNil`.
pub fn wscript_fn_nop_1440(c: &mut Cpu) {
    guest(c, nop_1440_body)
}

pub(crate) fn nop_1440_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 12, 0x25cc, [A1, A2, A0], true);
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx(0x0050_0000u32.wrapping_sub(0x4410));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw(0x0050_0000u32.wrapping_sub(0x4410), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x25cc), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx(0x0050_0000u32.wrapping_sub(0x4410));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0xc,
        0x0030_14b8,
        (S1, S2),
        gp.wrapping_add(0xb88),
        4,
        0x0030_14d0,
    )?;

    c.cycles += 18;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[V0][0] = !(0u64 | c.g[A0][0]);
    let sret = c.g[S3][0];
    oc732_return_nil(c, sret);
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_GetCollectedCount` (0x301560): fills an 8-byte block (id) and
/// returns, as an int `LunoVMRegister` in a0, the manager's collected count
/// for the id (`WScriptMan_GetCollectedCount(gp+0xce8, id)`), or 0 for id -1.
pub fn wscript_fn_get_collected_count(c: &mut Cpu) {
    guest(c, get_collected_count_body)
}

pub(crate) fn get_collected_count_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, 0x25d8, [A0, A1, A2], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x25d4));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x25d0), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x25d8), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(0x25d0));
    oc732_copy_fill(
        c,
        7,
        false,
        src,
        0x8,
        0x0030_15bc,
        (S2, S3),
        gp.wrapping_add(0xb90),
        3,
        0x0030_15d0,
    )?;

    c.cycles += 5;
    c.g[V0][0] = u64::MAX;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    let no_id = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0x1;
    if no_id {
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_165c)?;
        c.sw(c.g[V0][0] as u32, 0);
    } else {
        c.cycles += 9;
        c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
        call_ptr(c, 0x0030_ba80, 0x0030_1634)?;
        c.g[V1][0] = 0x1;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0022_6610, 0x0030_1648)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[S1][0];
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_GetObjectSlot31` (0x301830): fills an 8-byte block (x), looks up
/// the tagged object for x (x = -1: `*(gp+0xce8)->0x290`) and returns, as an
/// int `LunoVMRegister` in a0, the result of the object's vtable method at
/// `+0xf8` (0 when the object has no sub-object), or 1 when there is no
/// object at all.
pub fn wscript_fn_get_object_slot31(c: &mut Cpu) {
    guest(c, get_object_slot31_body)
}

pub(crate) fn get_object_slot31_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, 0x25f8, [A1, A0, A2], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(0x25f4));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(0x25f0), c.g[V1][0]);
        c.sw(gp.wrapping_add(0x25f8), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(0x25f0));
    oc732_copy_fill(
        c,
        10,
        true,
        src,
        0x8,
        0x0030_1898,
        (S1, S3),
        gp.wrapping_add(0xba0),
        4,
        0x0030_18b0,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc732_zone_lookup(c, vp, 3);
    if !tagged {
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    }

    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // No object: the result is 1... (the type word stays set to v0 = 1).
        c.g[V0][0] = 0x1;
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_19a4)?;
        c.sw(c.g[V0][0] as u32, 0);
    } else {
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let no_sub = c.g[A1][0] == 0;
        c.g[S0][0] = 0;
        if !no_sub {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xf8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xfc));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_197c)?;
            c.g[S0][0] = c.g[V0][0];
        }
        c.cycles += 6;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_1990)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[S2][0];
    oc732_epilogue(c);
    Ok(())
}

/// Shared body of `JumpToTask` and `GotoTask`: an 8-byte block (task id) at
/// `gp+blk`, then `WScriptMan_<JumpTo|Goto>Task(gp+0xce8, id)` and an int
/// result register (0) in a0. All addresses are parameters.
pub(crate) struct Oc732Task {
    guard: u32,
    block: u32,
    table: u32,
    copy_ret: u32,
    top: u32,
    man_fn: u32,
    man_ret: u32,
    as_int_ret: u32,
}

pub(crate) fn oc732_task_body(c: &mut Cpu, p: Oc732Task) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc732_prologue(c, 11, p.guard, [A0, A1, A2], false);
    if !initialised {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(gp.wrapping_add(p.block + 4));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 0x1;
        c.sw(gp.wrapping_add(p.block), c.g[V1][0]);
        c.sw(gp.wrapping_add(p.guard), c.g[A0][0]);
        c.sb(sp(c), c.g[V0][0]);
    }
    let src = sx(gp.wrapping_add(p.block));
    oc732_copy_fill(
        c,
        7,
        false,
        src,
        0x8,
        p.copy_ret,
        (S2, S3),
        gp.wrapping_add(p.table),
        3,
        p.top,
    )?;

    c.cycles += 16;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    call_ptr(c, p.man_fn, p.man_ret)?;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = c.g[S1][0];
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0022_6610, p.as_int_ret)?;
    c.sw(c.g[V0][0] as u32, 0);
    c.g[V0][0] = c.g[S1][0];
    oc732_epilogue(c);
    Ok(())
}

/// `WScriptFn_JumpToTask` (0x301b88): fills an 8-byte block (task id) and
/// calls `WScriptMan_JumpToTask(gp+0xce8, id)`; returns an int register (0).
pub fn wscript_fn_jump_to_task(c: &mut Cpu) {
    guest(c, |c| {
        oc732_task_body(
            c,
            Oc732Task {
                guard: 0x2608,
                block: 0x2600,
                table: 0xbb0,
                copy_ret: 0x0030_1be4,
                top: 0x0030_1bf8,
                man_fn: 0x0030_a6d8,
                man_ret: 0x0030_1c4c,
                as_int_ret: 0x0030_1c5c,
            },
        )
    })
}

/// `WScriptFn_GotoTask` (0x301c80): the `GotoTask` twin of the above
/// (`WScriptMan_GotoTask`).
pub fn wscript_fn_goto_task(c: &mut Cpu) {
    guest(c, |c| {
        oc732_task_body(
            c,
            Oc732Task {
                guard: 0x2618,
                block: 0x2610,
                table: 0xbb8,
                copy_ret: 0x0030_1cdc,
                top: 0x0030_1cf0,
                man_fn: 0x0030_a700,
                man_ret: 0x0030_1d44,
                as_int_ret: 0x0030_1d54,
            },
        )
    })
}

/// The per-function constants of one `gp`-flagged bind block. The block is
/// a flag word at `gp+flag` (set once, on the first call), the id word
/// `-1` at `gp+flag-8` (the copy source) and the kind byte at `gp+flag-4`
/// (copied into the frame): the first call sets them, then every call copies
/// the eight bytes to the argument block at `sp+0x10` and fills it from the
/// argument list through the kind table at `gp+table`.
pub(crate) struct Ws733Block {
    flag: u32,
    init_cycles: u64,
    copy_ret: u32,
    table: u32,
    entry: u64,
    top: u32,
}

/// A bind body that answers `self` (v0 = self): the block, then one
/// `matches(id)` call, then `castInt(self)` gets the result as its int.
pub(crate) struct Ws733Match {
    block: Ws733Block,
    matches: u32,
    matches_ret: u32,
    cast_ret: u32,
}

pub(crate) const NOP_1F48: Ws733Block = Ws733Block {
    flag: 0x2638,
    init_cycles: 6,
    copy_ret: 0x0030_1fa0,
    table: 0xbc8,
    entry: 4,
    top: 0x0030_1fb8,
};

pub(crate) const MISSION_STOP_PROCESS: Ws733Block = Ws733Block {
    flag: 0x2690,
    init_cycles: 7,
    copy_ret: 0x0030_2c34,
    table: 0xc08,
    entry: 3,
    top: 0x0030_2c48,
};

pub(crate) const STOP_PROCESS: Ws733Block = Ws733Block {
    flag: 0x26a0,
    init_cycles: 7,
    copy_ret: 0x0030_2d44,
    table: 0xc10,
    entry: 3,
    top: 0x0030_2d58,
};

pub(crate) const IS_GAME_MODE: Ws733Match = Ws733Match {
    block: Ws733Block {
        flag: 0x2650,
        init_cycles: 6,
        copy_ret: 0x0030_26d8,
        table: 0xbe0,
        entry: 4,
        top: 0x0030_26f0,
    },
    matches: 0x0030_3d90,
    matches_ret: 0x0030_2740,
    cast_ret: 0x0030_2754,
};

pub(crate) const IS_EVENT: Ws733Match = Ws733Match {
    block: Ws733Block {
        flag: 0x2670,
        init_cycles: 6,
        copy_ret: 0x0030_28c8,
        table: 0xbf0,
        entry: 4,
        top: 0x0030_28e0,
    },
    matches: 0x0030_3e60,
    matches_ret: 0x0030_2930,
    cast_ret: 0x0030_2944,
};

/// Opens the frame and saves s16..s19 and ra, as the generated prologue
/// does, and loads the flag word at `gp+flag` first. `s1` gets the count
/// (`a1`); `s2`/`s3` get `a0`/`a2` (self first) or `a2`/`a0` (sret first).
/// Returns whether the flag word was already set.
pub(crate) fn ws733_prologue(c: &mut Cpu, cycles: u64, flag: u32, self_first: bool) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(gp.wrapping_add(flag));
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    let (src2, src3) = if self_first { (A0, A2) } else { (A2, A0) };
    c.g[S2][0] = c.g[src2][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[src3][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    initialised
}

/// The first-call init of a bind block: the id word `-1` and the flag word
/// `1` in the globals, the kind byte into the frame.
pub(crate) fn ws733_gp_init(c: &mut Cpu, cycles: u64, flag: u32) {
    let gp = c.g[GP][0] as u32;
    c.cycles += cycles;
    c.g[V0][0] = c.lbu(gp.wrapping_add(flag - 4));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 0x1;
    c.sw(gp.wrapping_add(flag - 8), c.g[V1][0]);
    c.sw(gp.wrapping_add(flag), c.g[A0][0]);
    c.sb(sp(c), c.g[V0][0]);
}

/// `MEM_copy(dst, src, len)` with `dst` the argument block at `sp+0x10`
/// (kept in s0).
pub(crate) fn ws733_copy_block(
    c: &mut Cpu,
    cycles: u64,
    src: u64,
    len: u64,
    ret_addr: u32,
) -> Flow {
    c.cycles += cycles;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = len;
    c.g[A1][0] = src;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, ret_addr)
}

/// The `(params, count)` loop: each argument is stored into the block at `vp`
/// by its index, as a float when the kind table says float and the argument's
/// kind differs, else as an int. `entry` is the cycle charge of the loop's
/// entry block, `top` its back-edge head. Leaves a0..a3, t0, v0, v1 as the
/// generated loop does.
pub(crate) fn ws733_fill_block(
    c: &mut Cpu,
    vp: u64,
    count: u64,
    params: u64,
    table: u32,
    entry: u64,
    top: u32,
) {
    c.g[A2][0] = params;
    if (count as i64) <= 0 {
        return;
    }
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The block every `gp`-flagged bind body opens with: the first-call init
/// (unless the flag is set), the copy of the id word, and the fill.
pub(crate) fn ws733_gp_block(
    c: &mut Cpu,
    b: &Ws733Block,
    initialised: bool,
    count: u64,
    params: u64,
) -> Flow {
    let gp = c.g[GP][0] as u32;
    if !initialised {
        ws733_gp_init(c, b.init_cycles, b.flag);
    }
    ws733_copy_block(c, 7, sx(gp.wrapping_add(b.flag - 8)), 8, b.copy_ret)?;
    let vp = c.g[S0][0];
    ws733_fill_block(
        c,
        vp,
        count,
        params,
        gp.wrapping_add(b.table),
        b.entry,
        b.top,
    );
    Ok(())
}

/// `luno_gLunoNil` through the sret pointer (the 12-byte struct tail every
/// WScript body returns through).
pub(crate) fn ws733_nil_return(c: &mut Cpu, sret: u64) {
    c.g[V1][0] = 0x4d0000;
    c.g[V0][0] = sret;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (sret as u32).wrapping_add(0x7));
    c.sdr(4, sret as u32);
    c.sw((sret as u32).wrapping_add(0x8), c.g[A1][0]);
}

/// The epilogue: restore s16..s19 and ra, pop the frame, return to ra.
pub(crate) fn ws733_leave(c: &mut Cpu) {
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
}

/// A bind body that returns `self` (v0 = self): the block, then the
/// `matches` call on the id word, then `castInt(self)` gets the result.
pub(crate) fn ws733_bind_match(c: &mut Cpu, m: &Ws733Match) -> Flow {
    let initialised = ws733_prologue(c, 11, m.block.flag, true);
    let (count, params) = (c.g[S1][0], c.g[S3][0]);
    ws733_gp_block(c, &m.block, initialised, count, params)?;
    c.cycles += 16;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, m.matches, m.matches_ret)?;
    c.g[V1][0] = 0x1;
    c.g[S0][0] = c.g[V0][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0022_6610, m.cast_ret)?;
    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    c.g[V0][0] = c.g[S2][0];
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_HasRiderVisited` (0x301d78): the `(params, n)` argument list
/// fills the block at sp+0x10 (the id word first, -1). The id names the
/// instance: -1 is the wscript's own (`*(gp+0xce8)+0x290`), any other id
/// the world cache's section table entry. When that instance is intersected
/// by the being's rider (its virtual `isIntersecting` at `+0x1c`), `self`
/// gets type 1 and its int is set to 1 through `castInt`; otherwise type 1
/// and int 0. Returns `self`.
pub fn wscript_fn_has_rider_visited(c: &mut Cpu) {
    guest(c, has_rider_visited_body)
}

pub(crate) fn has_rider_visited_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 11, 0x2628, true);
    if !initialised {
        ws733_gp_init(c, 7, 0x2628);
    }
    // The copy of the id word: -1 into the block, then MEM_copy.
    c.g[V0][0] = u64::MAX;
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    ws733_copy_block(c, 10, sx(gp.wrapping_add(0x2620)), 8, 0x0030_1de0)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S3][0]);
    ws733_fill_block(c, vp, count, params, gp.wrapping_add(0xbc0), 4, 0x0030_1df8);

    // The instance the id names.
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let has_id = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if has_id {
        c.cycles += 8;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !nonzero {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A1][0] = 0;
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    }

    // The rider test on the instance (a1).
    c.cycles += 2;
    let has_obj = c.g[A1][0] != 0;
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
    let mut hit = false;
    if has_obj {
        c.cycles += 20;
        c.g[S0][0] = 1;
        c.g[A2][0] = c.lw(gp.wrapping_add(0xce8));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
        if c.g[V0][0] == 0 {
            c.g[A0][0] = 0;
        }
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A2][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0030_1ef8)?;
        hit = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }

    if hit {
        c.cycles += 5;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0022_6610, 0x0030_1f0c)?;
        c.sw(c.g[V0][0] as u32, c.g[S0][0]);
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0022_6610, 0x0030_1f24)?;
        c.sw(c.g[V0][0] as u32, 0);
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[S2][0];
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_Nop_1f48` (0x301f48), the set-game-path body: the block, then
/// `WScriptFn_EmptyHelper(id)`, and the nil struct through the sret.
pub fn wscript_fn_nop_1f48(c: &mut Cpu) {
    guest(c, nop_1f48_body)
}

pub(crate) fn nop_1f48_body(c: &mut Cpu) -> Flow {
    let initialised = ws733_prologue(c, 11, NOP_1F48.flag, false);
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_gp_block(c, &NOP_1F48, initialised, count, params)?;
    c.cycles += 18;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    call_ptr(c, 0x0030_3d88, 0x0030_2008)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_IsGameMode` (0x302680): the block, then `isGamePath(id)` as
/// the type-and-int of `self`; returns `self`.
pub fn wscript_fn_is_game_mode(c: &mut Cpu) {
    guest(c, is_game_mode_body)
}

pub(crate) fn is_game_mode_body(c: &mut Cpu) -> Flow {
    ws733_bind_match(c, &IS_GAME_MODE)
}

/// `WScriptFn_IsEvent` (0x302870): the block, then `isGameEvent(id)` as the
/// type-and-int of `self`; returns `self`.
pub fn wscript_fn_is_event(c: &mut Cpu) {
    guest(c, is_event_body)
}

pub(crate) fn is_event_body(c: &mut Cpu) -> Flow {
    ws733_bind_match(c, &IS_EVENT)
}

/// `WScriptFn_ProcessCommand` (0x302ac0): the block (its static is the
/// 12-byte record at 0x4fbc40, initialised on the first call), then
/// `WScriptMan_ProcessCommand(manager, rec[0], rec[1])` and the nil struct.
pub fn wscript_fn_process_command(c: &mut Cpu) {
    guest(c, process_command_body)
}

pub(crate) fn process_command_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 12, 0x2684, false);
    c.g[A2][0] = 0x500000;
    if !initialised {
        c.cycles += 10;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43c0));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x43c0), c.g[V0][0]);
        c.g[A0][0] = 0x1;
        c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw(gp.wrapping_add(0x2684), c.g[A0][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0);
        c.sb(sp(c), c.g[A1][0]);
    }
    let src = sx((c.g[A2][0] as u32).wrapping_sub(0x43c0));
    ws733_copy_block(c, 7, src, 0xc, 0x0030_2b2c)?;
    let vp = c.g[S0][0];
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_fill_block(c, vp, count, params, gp.wrapping_add(0xc00), 3, 0x0030_2b40);
    c.cycles += 20;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    call_ptr(c, 0x0030_a728, 0x0030_2b98)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_MissionStopProcess` (0x302bd8): the block, then
/// `WScriptMan_MissionCommand(manager, rec[0], 0, -1)` and the nil struct.
pub fn wscript_fn_mission_stop_process(c: &mut Cpu) {
    guest(c, mission_stop_process_body)
}

pub(crate) fn mission_stop_process_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 11, MISSION_STOP_PROCESS.flag, false);
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_gp_block(c, &MISSION_STOP_PROCESS, initialised, count, params)?;
    c.cycles += 21;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    c.g[A3][0] = u64::MAX;
    call_ptr(c, 0x0030_a868, 0x0030_2ca4)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// `WScriptFn_StopProcess` (0x302ce8): the block, then
/// `WScriptMan_ProcessCommand(manager, rec[0], 0)` and the nil struct.
pub fn wscript_fn_stop_process(c: &mut Cpu) {
    guest(c, stop_process_body)
}

pub(crate) fn stop_process_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = ws733_prologue(c, 11, STOP_PROCESS.flag, false);
    let (count, params) = (c.g[S1][0], c.g[S2][0]);
    ws733_gp_block(c, &STOP_PROCESS, initialised, count, params)?;
    c.cycles += 20;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    call_ptr(c, 0x0030_a728, 0x0030_2db0)?;
    let sret = c.g[S3][0];
    ws733_nil_return(c, sret);
    ws733_leave(c);
    Ok(())
}

/// One of the two rider-query switches (`WScriptFn_IsRiderInState` and its
/// `_2` twin). Offsets are gp-relative where the generated code uses gp;
/// `table`, `copy_ret`, the return addresses and `targets` are absolute.
pub(crate) struct RiderQuery {
    /// The function's entry (the self case restarts from it).
    entry: u32,
    /// The `jump_out` end of the body.
    end: u32,
    /// The static-init flag word, its flag byte, the 8-byte block copied to
    /// `vp` and the per-index kind table.
    flag_off: u32,
    byte_off: u32,
    src_off: u32,
    kinds_off: u32,
    /// The case jump table (one word per `vp[0]`) and its 19 case targets.
    table: u32,
    targets: [u32; 19],
    /// The extra arm that charges one more cycle and lands on case 4.
    quirk: Option<u32>,
    /// Return addresses of the block copy, the call into the query table and
    /// of the int-view call.
    copy_ret: u32,
    call_ret: u32,
    tail_ret: u32,
    /// The arg loop's head and its entry charge.
    loop_top: u32,
    entry_cycles: u64,
    /// The static-init body (the only difference between the two switches).
    init: fn(&mut Cpu, &RiderQuery),
}

/// Whether a query pass restarted the body (the self case) or finished it
/// (through the epilogue or a `jump_out`).
pub(crate) enum RiderPass {
    Restart,
    Done,
}

/// The rider-query static init of `WScriptFn_IsRiderInState` (0x302ef8).
pub(crate) fn rider_query_init_a(c: &mut Cpu, q: &RiderQuery) {
    c.cycles += 5;
    let gp = c.g[GP][0] as u32;
    c.g[V1][0] = c.lbu(gp.wrapping_add(q.byte_off));
    c.g[V0][0] = 1;
    c.sw(gp.wrapping_add(q.flag_off), c.g[V0][0]);
    c.sw(gp.wrapping_add(q.src_off), 0);
    let s = sp(c);
    c.sb(s, c.g[V1][0]);
}

/// The rider-query static init of `WScriptFn_IsRiderInState_2` (0x3037b0).
pub(crate) fn rider_query_init_b(c: &mut Cpu, q: &RiderQuery) {
    c.cycles += 6;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lbu(gp.wrapping_add(q.byte_off));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 1;
    c.sw(gp.wrapping_add(q.src_off), c.g[V1][0]);
    c.sw(gp.wrapping_add(q.flag_off), c.g[A0][0]);
    let s = sp(c);
    c.sb(s, c.g[V0][0]);
}

/// The `(params, n)` loop shared by the WScript bodies: each argument is
/// stored into the block at `vp` by its index, as a float when the kind
/// table says float and the argument's kind differs, else as an int. The
/// caller has set `a2` to the parameter list and checked `count > 0`; `entry`
/// is the loop's entry charge, `top` its back-edge head, `kinds` the kind table.
pub(crate) fn rider_fill_args(c: &mut Cpu, vp: u64, count: u64, kinds: u32, entry: u64, top: u32) {
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(kinds);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The case the loaded jump-table target `target` names: the mission index
/// `n` it passes to the query table and the cycles its arm charges, or None
/// for a target outside the switch.
pub(crate) fn rider_case(q: &RiderQuery, target: u32) -> Option<(u64, u64)> {
    if q.quirk == Some(target) {
        return Some((4, 4));
    }
    let i = q.targets.iter().position(|&t| t == target)?;
    Some((i as u64, if i == 18 { 2 } else { 3 }))
}

/// The static-init-flag prologue of a rider-query switch: saves the
/// registers, and runs the init when the flag is clear.
pub(crate) fn rider_query_prologue(c: &mut Cpu, q: &RiderQuery) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_add(q.flag_off));
    let s = sp(c);
    c.sq(17, s.wrapping_add(0x50));
    c.sq(18, s.wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, s.wrapping_add(0x30));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, s.wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    let t = c.g[V0][0] != 0;
    let ra = c.g[RA][0];
    c.sd(s.wrapping_add(0x20), ra);
    if !t {
        (q.init)(c, q);
    }
}

/// The body of a rider-query switch after its prologue: the `vp` block copy
/// and arg loop, the case dispatch (the self case asks for a restart), the
/// `Rider_DispatchQueryTable` call and the int store, then the epilogue.
pub(crate) fn rider_query_rest(c: &mut Cpu, q: &RiderQuery) -> Result<RiderPass, Unwound> {
    // The 8-byte block copy into vp (sp+0x10, kept in s0).
    c.cycles += 7;
    let vp = sx(sp(c).wrapping_add(0x10));
    c.g[S0][0] = vp;
    c.g[A2][0] = 8;
    let gp = c.g[GP][0] as u32;
    c.g[A1][0] = sx(gp.wrapping_add(q.src_off));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, q.copy_ret)?;
    let count = c.g[S1][0];
    let no_args = (count as i64) <= 0;
    c.g[A2][0] = c.g[S2][0];
    if !no_args {
        let kinds = gp.wrapping_add(q.kinds_off);
        rider_fill_args(c, vp, count, kinds, q.entry_cycles, q.loop_top);
    }

    // The rider the manager's index names, and the case index (vp[0]).
    c.cycles += 16;
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A3][0] = c.lw(vp as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[A1][0] = (c.g[A3][0] < 0x13) as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0;
    }
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
    let in_range = c.g[A1][0] != 0;
    if c.g[V0][0] != 0 {
        c.g[A2][0] = c.g[V1][0];
    }

    let res = if !in_range {
        c.cycles += 1;
        0
    } else {
        // The jump table: one target per vp[0], dispatched on its value.
        c.cycles += 7;
        c.g[V0][0] = sx(q.table);
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        if target == q.entry {
            // The self case: the body runs again from its entry.
            if event_due(c) {
                deliver_event_here(c, q.entry);
            }
            return Ok(RiderPass::Restart);
        }
        match rider_case(q, target) {
            Some((n, cycles)) => {
                c.cycles += cycles;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = n;
            }
            None => {
                c.jump_out(target, q.entry, q.end);
                return Ok(RiderPass::Done);
            }
        }
        c.cycles += 4;
        call_ptr(c, 0x0012_2ee8, q.call_ret)?;
        c.g[V0][0]
    };
    c.g[S0][0] = res;

    // The tail: mark the object as type 1, store the int, return it.
    c.cycles += 13;
    c.g[V0][0] = 1;
    c.g[A0][0] = c.g[S3][0];
    let s3 = c.g[S3][0] as u32;
    c.sw(s3.wrapping_add(0x8), 1);
    call_ptr(c, 0x0022_6610, q.tail_ret)?;
    let s0 = c.g[S0][0];
    c.sw(c.g[V0][0] as u32, s0);
    c.g[V0][0] = c.g[S3][0];
    let s = sp(c);
    c.lq(16, s.wrapping_add(0x60));
    c.lq(17, s.wrapping_add(0x50));
    c.lq(18, s.wrapping_add(0x40));
    c.lq(19, s.wrapping_add(0x30));
    c.g[RA][0] = c.ld(s.wrapping_add(0x20));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    ret(c);
    Ok(RiderPass::Done)
}

/// Runs a rider-query switch: the prologue, then the body, again from the
/// prologue when the self case restarts it.
pub(crate) fn rider_query_run(c: &mut Cpu, q: &RiderQuery) -> Flow {
    loop {
        rider_query_prologue(c, q);
        match rider_query_rest(c, q)? {
            RiderPass::Restart => continue,
            RiderPass::Done => return Ok(()),
        }
    }
}

/// The `IsRiderInState` case table, indexed by `vp[0]` (0..=18).
pub(crate) const IS_RIDER_IN_STATE_TARGETS: [u32; 19] = [
    0x0030_3004,
    0x0030_3010,
    0x0030_301c,
    0x0030_3028,
    0x0030_3034,
    0x0030_3040,
    0x0030_304c,
    0x0030_3058,
    0x0030_3064,
    0x0030_3070,
    0x0030_307c,
    0x0030_3088,
    0x0030_3094,
    0x0030_30a0,
    0x0030_30ac,
    0x0030_30b8,
    0x0030_30c4,
    0x0030_30d0,
    0x0030_30dc,
];

/// The `IsRiderInState_2` case table, indexed by `vp[0]` (0..=18).
pub(crate) const IS_RIDER_IN_STATE_2_TARGETS: [u32; 19] = [
    0x0030_38c4,
    0x0030_38d0,
    0x0030_38dc,
    0x0030_38e8,
    0x0030_38f4,
    0x0030_3900,
    0x0030_390c,
    0x0030_3918,
    0x0030_3924,
    0x0030_3930,
    0x0030_393c,
    0x0030_3948,
    0x0030_3954,
    0x0030_3960,
    0x0030_396c,
    0x0030_3978,
    0x0030_3984,
    0x0030_3990,
    0x0030_399c,
];

/// The gp-relative and absolute layout of `WScriptFn_IsRiderInState` (0x302ef8).
pub(crate) const IS_RIDER_IN_STATE: RiderQuery = RiderQuery {
    entry: 0x0030_2ef8,
    end: 0x0030_312c,
    flag_off: 0x26c0,
    byte_off: 0x26bc,
    src_off: 0x26b8,
    kinds_off: 0xc20,
    table: 0x0048_97b0,
    targets: IS_RIDER_IN_STATE_TARGETS,
    quirk: Some(0x0030_3030),
    copy_ret: 0x0030_2f4c,
    call_ret: 0x0030_30ec,
    tail_ret: 0x0030_3108,
    loop_top: 0x0030_2f60,
    entry_cycles: 3,
    init: rider_query_init_a,
};

/// The gp-relative and absolute layout of `WScriptFn_IsRiderInState_2` (0x3037b0).
pub(crate) const IS_RIDER_IN_STATE_2: RiderQuery = RiderQuery {
    entry: 0x0030_37b0,
    end: 0x0030_39ec,
    flag_off: 0x26e8,
    byte_off: 0x26e4,
    src_off: 0x26e0,
    kinds_off: 0xc40,
    table: 0x0048_9800,
    targets: IS_RIDER_IN_STATE_2_TARGETS,
    quirk: None,
    copy_ret: 0x0030_3808,
    call_ret: 0x0030_39ac,
    tail_ret: 0x0030_39c8,
    loop_top: 0x0030_3820,
    entry_cycles: 4,
    init: rider_query_init_b,
};

/// `bool WScriptFn_IsRiderInState(self, params, n)` (0x302ef8): stores into
/// the calling object (type 1) whether the rider at the manager's index is in
/// the state the case index `vp[0]` names, through `Rider_DispatchQueryTable`
/// (0x122ee8). The params overwrite the block; unknown indices give 0.
pub fn wscript_fn_is_rider_in_state(c: &mut Cpu) {
    guest(c, |c| rider_query_run(c, &IS_RIDER_IN_STATE));
}

/// `WScriptFn_IsRiderInState_2` (0x3037b0): the same switch as
/// `WScriptFn_IsRiderInState` with its own static block, kind table and
/// case table.
pub fn wscript_fn_is_rider_in_state_2(c: &mut Cpu) {
    guest(c, |c| rider_query_run(c, &IS_RIDER_IN_STATE_2));
}

/// `WScriptFn_GetRiderStateInfo` (0x3034f8, `CurrentRailFunc`): the spline id
/// of the rider's railslide motion when the rider's mission int 4 is set,
/// else -1, stored into the calling object (type 1) and returned through
/// `LunoVMRegister_AsInt` (0x226610).
pub fn wscript_fn_get_rider_state_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        let s = sp(c);
        c.sq(16, s.wrapping_add(0x20));
        c.g[A1][0] = 0x4;
        c.sq(17, s.wrapping_add(0x10));
        c.g[S0][0] = 0;
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
        if c.g[V0][0] != 0 {
            c.g[S0][0] = c.g[V1][0];
        }
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0012_2ee8, 0x0030_354c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[S0][0] = u64::MAX;
        } else {
            c.cycles += 3;
            let handler = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
            c.g[V0][0] = handler;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd4));
        }
        // The tail: mark the object as type 1, store the spline id.
        c.cycles += 11;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        let s1 = c.g[S1][0] as u32;
        c.sw(s1.wrapping_add(0x8), 1);
        call_ptr(c, 0x0022_6610, 0x0030_3578)?;
        let s0 = c.g[S0][0];
        c.sw(c.g[V0][0] as u32, s0);
        c.g[V0][0] = c.g[S1][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x20));
        c.lq(17, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `WScriptFn_GetEventArg` (0x3036b0, `CurrentTrickFunc`): the current trick
/// of the manager's rider (`WScriptMan_GetEventArg`, 0x30af08) for the case
/// index `vp[0]`, stored into the calling object (type 1) and returned
/// through `LunoVMRegister_AsInt` (0x226610).
pub fn wscript_fn_get_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26d8));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x50));
        c.sq(18, s.wrapping_add(0x40));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, s.wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, s.wrapping_add(0x60));
        c.g[S3][0] = c.g[A2][0];
        let t = c.g[V0][0] != 0;
        let ra = c.g[RA][0];
        c.sd(s.wrapping_add(0x20), ra);
        if !t {
            c.cycles += 6;
            c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x26d4));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x26d0), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x26d8), c.g[A0][0]);
            let s = sp(c);
            c.sb(s, c.g[V0][0]);
        }
        // The 8-byte block copy into vp, then the arg loop.
        c.cycles += 7;
        let vp = sx(sp(c).wrapping_add(0x10));
        c.g[S0][0] = vp;
        c.g[A2][0] = 8;
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x26d0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3708)?;
        let count = c.g[S1][0];
        let no_args = (count as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_args {
            let kinds = (c.g[GP][0] as u32).wrapping_add(0xc38);
            rider_fill_args(c, vp, count, kinds, 4, 0x0030_3720);
        }
        // The trick lookup, then the int store.
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = c.lw(c.g[A0][0] as u32);
        Callee {
            addr: 0x0030_af08,
            f: ee::s10::f_0030af08,
        }
        .call(c, 0x0030_3778)?;
        c.g[V1][0] = 1;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0022_6610, 0x0030_378c)?;
        let s0 = c.g[S0][0];
        c.sw(c.g[V0][0] as u32, s0);
        c.g[V0][0] = c.g[S2][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x60));
        c.lq(17, s.wrapping_add(0x50));
        c.lq(18, s.wrapping_add(0x40));
        c.lq(19, s.wrapping_add(0x30));
        c.g[RA][0] = c.ld(s.wrapping_add(0x20));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        ret(c);
        Ok(())
    });
}

/// `gp` of the game: the world, script-manager and flag globals are gp-relative.
pub(crate) fn wsf_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// The `memcpy` the builders copy their parameter buffer with.
pub(crate) const WSF_MEMCPY: u32 = 0x003e_6574;

/// The parameter loop of a builder: its cycle charge before the loop, the
/// `V0` it sets first (if any), and the event point of its back edge.
pub(crate) struct WsfFill {
    pre: u64,
    v0: Option<u64>,
    head: u32,
}

pub(crate) const RIDER_FILL: WsfFill = WsfFill {
    pre: 3,
    v0: None,
    head: 0x0030_4cd0,
};

pub(crate) const CALL_SLOT_FILL: WsfFill = WsfFill {
    pre: 4,
    v0: None,
    head: 0x0030_4ed8,
};

pub(crate) const SCREEN_FILL: WsfFill = WsfFill {
    pre: 4,
    v0: None,
    head: 0x0030_56e0,
};

pub(crate) const AVA_FILL: WsfFill = WsfFill {
    pre: 5,
    v0: Some(0x0044_0000),
    head: 0x0030_5b38,
};

pub(crate) const START_AVA_FILL: WsfFill = WsfFill {
    pre: 3,
    v0: None,
    head: 0x0030_5cf0,
};

pub(crate) const ADD_AVA_FILL: WsfFill = WsfFill {
    pre: 4,
    v0: None,
    head: 0x0030_5e10,
};

/// The record loop of the builders: each `(idx, val, type)` record at `A2`
/// (`count` of them, `A1` counts down) stores its value into `buf[idx]` (the
/// stack buffer, a word per index), as a float when the type table `types`
/// asks for one and the record's type does not match. Leaves `A1`, `A2`, `T0`
/// and `A3` the way the generated loop does; nothing runs for `count <= 0`.
pub(crate) fn wsf_fill(c: &mut Cpu, count: u64, buf: u64, types: u64, fill: &WsfFill) {
    if (count as i64) <= 0 {
        return;
    }
    c.cycles += fill.pre;
    if let Some(v) = fill.v0 {
        c.g[V0][0] = v;
    }
    c.g[A1][0] = count;
    c.g[T0][0] = types;
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(buf as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, fill.head);
        }
    }
}

/// The epilogue's 12-byte copy of the `luno` nil value (source `V1 - 0x6f68`)
/// into the hidden return slot `dst`. Leaves `A3` and `A1` as the generated
/// code does.
pub(crate) fn wsf_nil_copy(c: &mut Cpu, dst: u32) {
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
}

/// The section lookup of the key in `A2` (the table pointer is already in
/// `A1`): record `key & 0xff` of the table, then entry `key >> 8` of the
/// record's list. Leaves the entry, scaled to a pointer-sized offset, in `V0`
/// (0 when there is none). Charges the cycles of the generated `'b_..._78`
/// block: 7 when a record is found, 1 more when the entry is zero or missing.
pub(crate) fn wsf_section_chain(c: &mut Cpu) {
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    let nonzero = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if !nonzero {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
}

/// `WScriptFn_GetRiderHeightAboveObject` (0x304c50), the C++ `RiderHeightFunc`:
/// builds the rider-height parameter record from the script's parameter list
/// (the template copy with the one-time init) and returns the rider's height
/// above the section's object, in the first argument's slot. Saves `s0`-`s3`
/// and `f20`.
pub(crate) fn rider_height_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x90);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2740)).lw(c);
    frame.save(c, S1, 0x60);
    frame.save(c, S2, 0x50);
    frame.save(c, S3, 0x40);
    frame.save(c, S0, 0x70);
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = c.g[A2][0];
    frame.save_ra(c, 0x30);
    let init = c.g[V0][0] == 0;
    frame.save_fpr(c, 20, 0x80);

    if init {
        c.cycles += 7;
        c.g[V0][0] = Addr(gp.wrapping_add(0x273c)).lbu(c);
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 1;
        let v1 = c.g[V1][0];
        Addr(gp.wrapping_add(0x2738)).sw(c, v1);
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2740)).sw(c, a0);
        let v0 = c.g[V0][0];
        Addr(sp(c)).sb(c, v0);
    }

    // The parameter buffer: the record's id word, then the template copy.
    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(gp.wrapping_add(0x2738));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_4cbc)?;

    c.g[A2][0] = c.g[S3][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xc88));
    wsf_fill(c, count, buf, types, &RIDER_FILL);

    // The key (buf[0]) and its section record; the -1 key takes the script
    // manager's section.
    c.cycles += 5;
    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V1][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    let not_neg1 = c.g[V0][0] != 0;
    c.g[A3][0] = Addr(c.g[V1][0] as u32).lw(c);
    if not_neg1 {
        c.cycles += 9;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        wsf_section_chain(c);
        c.cycles += 1;
        c.g[S0][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x290)).lw(c);
    }

    // The instance of the section: its rider list entry gives the height
    // callee's result (the `f20` the epilogue returns through the frame).
    c.cycles += 2;
    let s0_zero = c.g[S0][0] == 0;
    c.g[A0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    if s0_zero {
        c.cycles += 5;
        c.g[V0][0] = 2;
        c.g[A0][0] = c.g[S2][0];
        let v0 = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, v0);
        call_ptr(c, 0x0022_6618, 0x0030_4e10)?;
        let v0 = c.g[V0][0];
        Addr(v0 as u32).sw(c, 0);
    } else {
        c.cycles += 12;
        c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        c.g[A1][0] = 0;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
        if c.g[V1][0] != 0 {
            c.g[A1][0] = c.g[V0][0];
        }
        if c.g[A1][0] != 0 {
            c.cycles += 12;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x6c0)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_4dcc)?;
            c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
            let v0 = c.g[V0][0] as u32;
            c.lwc1(1, v0.wrapping_add(0x8));
            c.sq(3, sp(c).wrapping_add(0x20));
            c.lwc1(0, sp(c).wrapping_add(0x28));
            c.sub_s(20, 1, 0);
        } else {
            c.cycles += 1;
            c.mtc1(0, 20);
        }
        c.cycles += 6;
        c.g[V0][0] = 2;
        c.g[A0][0] = c.g[S2][0];
        let v0 = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, v0);
        call_ptr(c, 0x0022_6618, 0x0030_4df8)?;
        let v0 = c.g[V0][0] as u32;
        c.swc1(20, v0);
    }

    c.cycles += 9;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore_ra(c, 0x30);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_GetRiderHeightAboveObject` (0x304c50): see [`rider_height_body`].
pub fn wscript_get_rider_height_above_object(c: &mut Cpu) {
    guest(c, rider_height_body);
}

/// `WScriptFn_CallObjectSlot41` (0x304e38): the object's slot-41 call with the
/// script's parameters. Builds the record from the template copy (with the
/// one-time init), runs the parameter loop, looks the section up and calls its
/// object. Returns its first argument; saves `s0`-`s4`.
pub(crate) fn call_slot41_body(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x80);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2744)).lw(c);
    frame.save(c, S3, 0x40);
    frame.save(c, S0, 0x70);
    frame.save(c, S2, 0x50);
    frame.save(c, S1, 0x60);
    frame.save(c, S4, 0x30);
    c.g[S3][0] = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[A2][0] = 0x0050_0000;
    frame.save_ra(c, 0x20);
    let init = c.g[V0][0] == 0;

    if init {
        c.cycles += 12;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42e0));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lbu(c);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 1;
        let v0 = c.g[V0][0];
        Addr((c.g[A2][0] as u32).wrapping_sub(0x42e0)).sw(c, v0);
        let a1 = c.g[A1][0];
        Addr((c.g[V1][0] as u32).wrapping_add(8)).sb(c, a1);
        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(4));
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2744)).sw(c, a0);
        Addr(sp(c)).sb(c, a1);
    }

    // The parameter record: copied from the template at 0x4fbd20.
    c.cycles += 11;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42e0));
    c.g[A2][0] = 0xc;
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    c.g[A0][0] = c.g[S1][0];
    c.g[S4][0] = 0x004d_0000;
    call_ptr(c, WSF_MEMCPY, 0x0030_4ec0)?;

    c.g[A2][0] = c.g[S3][0];
    let count = c.g[S0][0];
    let buf = c.g[S1][0];
    let types = sx(gp.wrapping_add(0xc90));
    wsf_fill(c, count, buf, types, &CALL_SLOT_FILL);

    // The key (the record's id word) and its section.
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    let not_neg1 = c.g[V0][0] != 0;
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if not_neg1 {
        c.cycles += 8;
        wsf_section_chain(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
    }

    // The section's object: its call with the parameter record.
    c.cycles += 3;
    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
    if c.g[S0][0] != 0 {
        c.cycles += 7;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_4f9c)?;
        let v0_zero = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !v0_zero {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0035_5888, 0x0030_4fac)?;
        }
    }

    // The epilogue copies the nil value through the saved `S2` slot.
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A1][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S2][0] as u32;
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    let v1 = c.g[V1][0];
    Addr(dst.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_CallObjectSlot41` (0x304e38): see [`call_slot41_body`].
pub fn wscript_call_object_slot41(c: &mut Cpu) {
    guest(c, call_slot41_body);
}

/// `WScriptFn_ScreenFlash` (0x305660): the screen-flash parameter record
/// (template copy, one-time init) and the flash callee on the section's object.
pub(crate) fn screen_flash_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2760)).lw(c);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save(c, S0, 0x60);
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A2][0];
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);

    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(gp.wrapping_add(0x275c)).lbu(c);
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 1;
        let v1 = c.g[V1][0];
        Addr(gp.wrapping_add(0x2758)).sw(c, v1);
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2760)).sw(c, a0);
        let v0 = c.g[V0][0];
        Addr(sp(c)).sb(c, v0);
    }

    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(gp.wrapping_add(0x2758));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_56c8)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xca0));
    wsf_fill(c, count, buf, types, &SCREEN_FILL);

    // The flash callee of the section's object (none for the -1 section).
    c.cycles += 5;
    c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let is_neg1 = c.g[V0][0] == c.g[V1][0];
    c.g[A0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    let mut tail = true;
    if !is_neg1 {
        c.cycles += 14;
        c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
        if c.g[V1][0] == 0 {
            c.g[A0][0] = 0;
        }
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x40)).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x44)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0030_576c)?;
        let v0_zero = c.g[V0][0] == 0;
        c.g[V1][0] = 0x004d_0000;
        if v0_zero {
            tail = false;
        } else {
            c.cycles += 3;
            c.mtc1(0, 12);
            call_ptr(c, 0x0039_0ec8, 0x0030_5780)?;
        }
    }
    if tail {
        c.cycles += 1;
    }
    c.g[V1][0] = 0x004d_0000;

    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    let dst = c.g[S3][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_ScreenFlash` (0x305660): see [`screen_flash_body`].
pub fn wscript_screen_flash(c: &mut Cpu) {
    guest(c, screen_flash_body);
}

/// `WorldObjFactory_Avalanche` (0x3059a0): builds the avalanche object (the
/// one-time init of its template, the parameter copy, the parameter loop), then
/// walks the eight slots of its node list and calls each live one's update.
pub(crate) fn avalanche_factory_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0x190);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2768)).lw(c);
    frame.save(c, S1, 0x170);
    frame.save(c, S2, 0x160);
    frame.save(c, S0, 0x180);
    frame.save(c, S3, 0x150);
    frame.save(c, S4, 0x140);
    frame.save(c, S5, 0x130);
    frame.save(c, S6, 0x120);
    frame.save(c, S7, 0x110);
    frame.save(c, FP, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A2][0];
    c.g[A1][0] = 0x0050_0000;
    frame.save_ra(c, 0xf0);
    let init = c.g[V0][0] == 0;
    let a0 = c.g[A0][0];
    Addr(sp(c).wrapping_add(0xe0)).sw(c, a0);

    if init {
        avalanche_init(c);
    }

    // The parameter copy (200 bytes from the template) and its loop.
    c.cycles += 17;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0xc8;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4270));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x30)).sw(c, v0);
    c.g[A0][0] = c.g[S0][0];
    for off in [0x14u32, 0x18, 0x1c, 0x20, 0x24, 0x28] {
        Addr(sp(c).wrapping_add(off)).sw(c, v0);
    }
    Addr(sp(c).wrapping_add(0x2c)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_5b1c)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    wsf_fill(c, count, buf, sx(0x0044_66d8), &AVA_FILL);

    // The node walk: eight slots of the copied list, each with its update.
    c.cycles += 10;
    c.g[S6][0] = u64::MAX;
    c.g[FP][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[S5][0] = c.g[S0][0] | 0x4;
    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa4));
    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x44));
    c.g[S7][0] = 7;
    loop {
        c.cycles += 3;
        c.g[A2][0] = Addr(c.g[S5][0] as u32).lw(c);
        let skip = c.g[A2][0] == c.g[S6][0];
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if !skip {
            c.cycles += 8;
            c.g[A0][0] = c.g[A2][0] & 0xff;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let a0_zero = c.g[A0][0] == 0;
            c.g[A1][0] = 0;
            if !a0_zero {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                }
            }
            c.cycles += 7;
            let s0 = c.g[S0][0] as u32;
            c.lwc1(12, s0);
            c.g[A0][0] = c.g[FP][0];
            let s1 = c.g[S1][0] as u32;
            c.lwc1(13, s1);
            let s2 = c.g[S2][0] as u32;
            c.lwc1(14, s2);
            let s3 = c.g[S3][0] as u32;
            c.lwc1(15, s3);
            c.g[A2][0] = Addr(c.g[S4][0] as u32).lw(c);
            call_ptr(c, 0x002d_92d0, 0x0030_5c10)?;
        }
        c.cycles += 8;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
        let more = (c.g[S7][0] as i64) >= 0;
        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_5ba8);
        }
    }

    c.cycles += 21;
    c.g[V0][0] = Addr(sp(c).wrapping_add(0xe0)).lw(c);
    c.g[V1][0] = 0x004d_0000;
    frame.restore(c, S0, 0x180);
    let dst = c.g[V0][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S1, 0x170);
    frame.restore(c, S2, 0x160);
    frame.restore(c, S3, 0x150);
    frame.restore(c, S4, 0x140);
    frame.restore(c, S5, 0x130);
    frame.restore(c, S6, 0x120);
    frame.restore(c, S7, 0x110);
    frame.restore(c, FP, 0x100);
    frame.restore_ra(c, 0xf0);
    frame.close(c);
    Ok(())
}

/// The one-time init of the avalanche template (the flag clear): the float
/// fields (10.0f, 3.0f, 1.0f, 0.0f) and the node slots, then the flag.
pub(crate) fn avalanche_init(c: &mut Cpu) {
    let gp = wsf_gp(c);
    c.cycles += 63;
    c.g[AT][0] = 0x4120_0000;
    c.mtc1(1, 0);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.mtc1(0, 1);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4270));
    c.g[V1][0] |= 0xffff;
    let base = c.g[V0][0] as u32;
    c.swc1(0, base.wrapping_add(0x60));
    let v1 = c.g[V1][0];
    Addr(base.wrapping_add(0x20)).sw(c, v1);
    c.g[A0][0] = 1;
    c.swc1(1, base.wrapping_add(0x80));
    Addr((c.g[A1][0] as u32).wrapping_sub(0x4270)).sw(c, 0);
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(1, 2);
    for off in [0x44u32, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c] {
        c.swc1(0, base.wrapping_add(off));
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
        Addr(base.wrapping_add(off)).sw(c, v1);
    }
    for off in [0x24u32, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40] {
        Addr(base.wrapping_add(off)).sw(c, 0);
    }
    for off in [0x64u32, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c] {
        c.swc1(1, base.wrapping_add(off));
    }
    c.swc1(2, base.wrapping_add(0x84));
    c.g[V1][0] = Addr(base.wrapping_add(0xc4)).lbu(c);
    c.swc1(2, base.wrapping_add(0xa0));
    c.swc1(0, base.wrapping_add(0xc0));
    let a0 = c.g[A0][0];
    Addr(gp.wrapping_add(0x2768)).sw(c, a0);
    for off in [0x88u32, 0x8c, 0x90, 0x94, 0x98, 0x9c] {
        c.swc1(2, base.wrapping_add(off));
    }
    for off in [0xa4u32, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xbc] {
        c.swc1(0, base.wrapping_add(off));
    }
    let v1 = c.g[V1][0];
    Addr(sp(c)).sb(c, v1);
}

/// `WorldObjFactory_Avalanche` (0x3059a0): see [`avalanche_factory_body`].
pub fn world_obj_factory_avalanche(c: &mut Cpu) {
    guest(c, avalanche_factory_body);
}

/// `WScriptFn_StartAvalanche` (0x305c88): sets the avalanche's start record
/// (one-time init, the parameter copy and loop), then calls the start callee
/// of the section's object unless the global at `gp+0x9d4` is set.
pub(crate) fn start_avalanche_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2778)).lw(c);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x60);
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);

    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V1][0] = Addr(gp.wrapping_add(0x2774)).lbu(c);
        c.g[V0][0] = 1;
        let v0 = c.g[V0][0];
        Addr(gp.wrapping_add(0x2778)).sw(c, v0);
        Addr(gp.wrapping_add(0x2770)).sw(c, 0);
        let v1 = c.g[V1][0];
        Addr(sp(c)).sb(c, v1);
    }

    c.cycles += 7;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A1][0] = sx(gp.wrapping_add(0x2770));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, WSF_MEMCPY, 0x0030_5cdc)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xca8));
    wsf_fill(c, count, buf, types, &START_AVA_FILL);

    c.cycles += 3;
    c.g[V0][0] = Addr(gp.wrapping_add(0x9d4)).lw(c);
    let set = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    if !set {
        c.cycles += 2;
        call_ptr(c, 0x002d_97a8, 0x0030_5d4c)?;
    }

    c.cycles += 16;
    c.g[V1][0] = 0x004d_0000;
    c.g[V0][0] = c.g[S3][0];
    let dst = c.g[S3][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_StartAvalanche` (0x305c88): see [`start_avalanche_body`].
pub fn wscript_start_avalanche(c: &mut Cpu) {
    guest(c, start_avalanche_body);
}

/// `WScriptFn_AddAvaSplineModifier` (0x305d90): adds a spline modifier to the
/// avalanche of the section named by the first parameter: the record copy, the
/// parameter loop, the section lookup, then the node's update callee (when the
/// node is live) and the node-add callee.
pub(crate) fn add_ava_spline_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2788)).lw(c);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x60);
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);

    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2784)).lbu(c);
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 1;
        let v1 = c.g[V1][0];
        Addr(gp.wrapping_add(0x2780)).sw(c, v1);
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2788)).sw(c, a0);
        let v0 = c.g[V0][0];
        Addr(sp(c)).sb(c, v0);
    }

    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(gp.wrapping_add(0x2780));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_5df8)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xcb0));
    wsf_fill(c, count, buf, types, &ADD_AVA_FILL);

    // The section lookup of the key (buf[0]): the table pointer, then the
    // record and its entry; S1 gets the entry (0 when there is none).
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    let not_neg1 = c.g[V0][0] != 0;
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if not_neg1 {
        c.cycles += 8;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !nonzero {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A1][0] = 0;
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        // The -1 key takes the script manager's section and skips the tail.
        return add_ava_finish(c, frame);
    }
    c.cycles += 1;
    c.g[S1][0] = c.g[A1][0];
    add_ava_finish(c, frame)
}

/// The node walk and the epilogue of `WScriptFn_AddAvaSplineModifier`: the
/// node's callees when the section has a record, then the nil copy.
pub(crate) fn add_ava_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let s1_zero = c.g[S1][0] == 0;
    c.g[V1][0] = 0x004d_0000;
    if s1_zero {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
    } else {
        c.cycles += 3;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
        let s0_zero = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S3][0];
        if !s0_zero {
            ada_node_callees(c)?;
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
    }

    c.cycles += 14;
    let dst = c.g[S3][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// The node's two callees of `AddAvaSplineModifier`: the node's update (its
/// pointer at `+0x84` of the node's `+0xc` record), then the node-add on the
/// section's list (`S1`) when the update answers non-zero.
pub(crate) fn ada_node_callees(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0030_5eec)?;
    let zero = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S0][0];
    if !zero {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_5a78, 0x0030_5efc)?;
    }
    Ok(())
}

/// `WScriptFn_AddAvaSplineModifier` (0x305d90): see [`add_ava_spline_body`].
pub fn wscript_add_ava_spline_modifier(c: &mut Cpu) {
    guest(c, add_ava_spline_body);
}

/// `WorldObjFactory_AnimDelta`: builds an animation-delta modifier record (template memcpy, parameter fill,
/// `AttachModifier`, allocation).
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn world_obj_factory_anim_delta(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002fb060);
    'b_2fb0ec: {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x24d0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = 0x500000;
        let t = c.g[V0][0] != 0u64;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        if t {
            break 'b_2fb0ec;
        } else {
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c70));
            c.g[V1][0] = 0xffffffffffff0000;
            c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.g[V1][0] | 0xffff;
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 0);
            c.g[A0][0] = 0x1;
            c.g[AT][0] = 0x41f00000;
            c.mtc1(1, 1);
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4c70), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
            c.sw((c.g[GP][0] as u32).wrapping_add(0x24d0), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x24), 0u64);
            c.sb((c.g[SP][0] as u32), c.g[A1][0]);
            break 'b_2fb0ec;
        }
    }
    'b_2fb170: {
        c.cycles += 11;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4c70));
        c.g[A2][0] = 0x2c;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002fb110);
        c.g[S4][0] = 0x4d0000;
        c.pc = 0x003e6574;
        call_indirect(c, 0x003e6574);
        if c.pc != 0x002fb110 {
            return;
        }
        let t = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if t {
            break 'b_2fb170;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e70));
            c.g[A3][0] = 0x2;
            'l_2fb128: loop {
                'b_2fb164: {
                    'b_2fb160: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            break 'b_2fb160;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[A3][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2fb160;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                c.cvt_s_w(0, 0);
                                let t = true;
                                c.swc1(0, (c.g[A0][0] as u32));
                                if t {
                                    break 'b_2fb164;
                                } else {
                                    break 'b_2fb160;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                    break 'b_2fb164;
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let t = c.g[A1][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002fb128);
                    }
                    continue 'l_2fb128;
                } else {
                    break 'b_2fb170;
                }
            }
        }
    }
    'b_2fb1dc: {
        'b_2fb18c: {
            c.cycles += 4;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = !(0u64 | c.g[A2][0]);
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            if t {
                break 'b_2fb18c;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                let t = true;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                if t {
                    break 'b_2fb1dc;
                } else {
                    break 'b_2fb18c;
                }
            }
        }
        'b_2fb1d8: {
            'b_2fb1b4: {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                if c.g[V1][0] != 0u64 {
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    break 'b_2fb1b4;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = 0u64;
                    if t {
                        break 'b_2fb1d8;
                    } else {
                        break 'b_2fb1b4;
                    }
                }
            }
            'b_2fb1d4: {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t {
                    break 'b_2fb1d4;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    break 'b_2fb1d4;
                }
            }
            c.cycles += 1;
            c.g[A2][0] = c.g[V0][0];
            break 'b_2fb1d8;
        }
        c.cycles += 1;
        c.g[S0][0] = c.g[A2][0];
        break 'b_2fb1dc;
    }
    'b_2fb22c: {
        c.cycles += 2;
        let t = c.g[S0][0] == 0u64;

        if t {
            break 'b_2fb22c;
        } else {
            'b_2fb200: {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = 0x2;
                if t {
                    break 'b_2fb200;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002fb1f8);
                    c.g[A2][0] = c.g[S1][0];
                    c.pc = 0x002fae38;
                    ee::s10::f_002fae38(c);
                    if c.pc != 0x002fb1f8 {
                        return;
                    }
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_2fb22c;
                    } else {
                        break 'b_2fb200;
                    }
                }
            }
            c.cycles += 11;
            c.g[A1][0] = 0x490000;
            c.g[A0][0] = 0x70;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x69a8));
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x002fb218);
            c.g[A3][0] = 0u64;
            c.pc = 0x00317d70;
            ee::s10::f_00317d70(c);
            if c.pc != 0x002fb218 {
                return;
            }
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002fb22c);
            c.g[A3][0] = c.g[S1][0];
            c.pc = 0x00359228;
            call_indirect(c, 0x00359228);
            if c.pc != 0x002fb22c {
                return;
            }
            break 'b_2fb22c;
        }
    }
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.ldr(2, (c.g[A1][0] as u32));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[S2][0] as u32));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = c.g[S2][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
    c.pc = t;
    return;
}

/// `gp` of the game (the world and script-manager globals are gp-relative).
pub(crate) fn wf740_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// A register as a 32-bit address.
pub(crate) fn wf740_addr(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The `memcpy` the world-object factories copy their template with.
pub(crate) const WF740_MEMCPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38), reached by a direct call.
pub(crate) const WF740_ATTACH: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// The parameter loop shared by the factories: each `(idx, val, type)` entry
/// of 16 bytes at `args` (count `cnt`) stores its value into `buf[idx]`, as a
/// float when the type table asks for one and the entry's type does not match.
pub(crate) struct WfFill {
    /// Register holding the stack buffer (`buf`).
    buf: usize,
    /// Register holding the parameter count.
    cnt: usize,
    /// Register holding the parameter list.
    args: usize,
    /// Cycles charged before the loop.
    pre: u64,
    /// Address of the parameter type table.
    table: u32,
    /// Event point of the loop's back edge.
    head: u32,
}

pub(crate) fn wf740_fill(c: &mut Cpu, f: &WfFill) {
    c.g[A2][0] = c.g[f.args][0];
    if (c.g[f.cnt][0] as i64) <= 0 {
        return;
    }
    c.cycles += f.pre;
    c.g[V0][0] = 0x0044_0000;
    c.g[A1][0] = c.g[f.cnt][0];
    c.g[T0][0] = sx(f.table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = wf740_addr(c, A2);
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[f.buf][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = wf740_addr(c, A0);
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, f.head);
        }
    }
}

/// The instance lookup of the factories (`S1` = the buffer, `S0` = the result,
/// `tmp` the scratch register the result passes through): the id in `buf[0]`
/// is -1 for the script manager's instance, else a section / entry pair
/// resolved through the world's section table (0 when missing).
pub(crate) fn wf740_lookup_to_s0(c: &mut Cpu, tmp: usize) {
    c.cycles += 4;
    c.g[A2][0] = Addr(wf740_addr(c, S1)).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(wf740_addr(c, A1)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
    if c.g[V1][0] != 0 {
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[tmp][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[tmp][0] = 0;
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[tmp][0];
}

/// The section / entry half of the lookup of the later builders: `A2` is the
/// non-(-1) id and `A1` the world; leaves the instance (0 when missing) in V0.
pub(crate) fn wf740_lookup_tail_v0(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(wf740_addr(c, A1)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
    let missing = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if !missing {
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(wf740_addr(c, V0)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// The instance lookup of the later builders, with the id at `buf[0]` (`buf`
/// holds the buffer): -1 is the script manager's instance. Result in V0.
pub(crate) fn wf740_lookup_v0(c: &mut Cpu, buf: usize) {
    c.cycles += 4;
    c.g[A2][0] = Addr(wf740_addr(c, buf)).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    wf740_lookup_tail_v0(c);
}

/// Epilogue copy of the 12-byte `luno` nil value (at 0x4c9098, from `imm`,
/// the `0x4d0000` register) into the hidden return slot `dst`; the pointer is
/// left in V0.
pub(crate) fn wf740_copy_nil(c: &mut Cpu, imm: usize, dst: usize) {
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[imm][0] as u32).wrapping_sub(0x6f68));
    let src = wf740_addr(c, A1);
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let d = wf740_addr(c, dst);
    c.sdl(2, d.wrapping_add(7));
    c.sdr(2, d);
    let v1 = c.g[V1][0];
    Addr(d.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[dst][0];
}

/// The attach-or-allocate tail of a factory: with an instance (`S0`), its
/// active node (+0xc) is offered to `WorldObjFactory_AttachModifier` (`flag`
/// is the node type); when there is none or the attach finds nothing, a new
/// node is allocated (`EA_MemAllocNamed`) and built by its constructor.
pub(crate) struct WfNode {
    /// `A1` for the attach (node type).
    flag: u64,
    attach_ret: u32,
    /// Cycles charged before the allocation.
    alloc_cycles: u64,
    /// Node size and name string (vtable pointer) of the allocation.
    size: u64,
    name: u32,
    alloc_ret: u32,
    /// The node constructor and its return address.
    ctor: u32,
    ctor_ret: u32,
    /// `WorldObjFactory_Boost` hands the constructor `(this, 1, 8, inst, buf)`,
    /// the others `(this, 1, inst, buf)`.
    five_args: bool,
}

pub(crate) fn wf740_attach_or_alloc(c: &mut Cpu, n: &WfNode) -> Flow {
    c.cycles += 2;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A1][0] = n.flag;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[S1][0];
        WF740_ATTACH.call(c, n.attach_ret)?;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
    }
    c.cycles += n.alloc_cycles;
    c.g[A0][0] = n.size;
    c.g[A1][0] = sx(n.name);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    // EA_MemAllocNamed(size, name, 0x20000000, 0).
    call_ptr(c, 0x0031_7d70, n.alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    if n.five_args {
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A2][0] = 8;
    } else {
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
    }
    call_ptr(c, n.ctor, n.ctor_ret)?;
    Ok(())
}

/// What differs between the three `WorldObjFactory_*` builders that share one
/// shape (`AnimCombo`, `Conveyor`, `Boost`).
pub(crate) struct WfSite {
    /// Frame size; `S0` is saved at `frame - 0x10`, `S1` below it, ..., `ra`
    /// at `frame - 0x60`.
    frame: u32,
    /// `gp` offset of the one-time init flag of the static template.
    flag_gp: u32,
    /// The parameter-list register (`S3` or `S4`) and the `0x4d0000` register.
    args: usize,
    imm: usize,
    /// The static template and its size.
    tpl: u32,
    copy_len: u64,
    copy_ret: u32,
    /// The scratch register of the instance lookup.
    tmp: usize,
    fill: WfFill,
    node: WfNode,
}

/// The shared body: prologue, one-time template init (`init`), template copy
/// into the stack buffer, parameter fill, instance lookup, attach, nil value
/// epilogue. Returns the hidden return pointer.
pub(crate) fn wf740_factory(c: &mut Cpu, s: &WfSite, init: fn(&mut Cpu, &WfSite)) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, s.frame);
    c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(s.flag_gp)).lw(c);
    for (k, reg) in [S0, S1, S2, S3, S4].into_iter().enumerate() {
        frame.save(c, reg, s.frame - 0x10 - 0x10 * k as u32);
    }
    c.g[s.args][0] = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[A2][0] = 0x0050_0000;
    let flag_clear = c.g[V0][0] == 0;
    frame.save_ra(c, s.frame - 0x60);
    if flag_clear {
        init(c, s);
    }

    // The id word of the buffer, then the template copy.
    c.cycles += 11;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx(s.tpl);
    c.g[A2][0] = s.copy_len;
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    c.g[A0][0] = c.g[S1][0];
    c.g[s.imm][0] = 0x004d_0000;
    call_ptr(c, WF740_MEMCPY, s.copy_ret)?;

    wf740_fill(c, &s.fill);
    wf740_lookup_to_s0(c, s.tmp);
    wf740_attach_or_alloc(c, &s.node)?;
    wf740_copy_nil(c, s.imm, S2);
    for (k, reg) in [S0, S1, S2, S3, S4].into_iter().enumerate() {
        frame.restore(c, reg, s.frame - 0x10 - 0x10 * k as u32);
    }
    frame.restore_ra(c, s.frame - 0x60);
    frame.close(c);
    Ok(())
}

/// `WorldObjFactory_AnimCombo`'s constants (template at `0x4fb3c0`, 60 bytes).
pub(crate) const WF740_ANIM: WfSite = WfSite {
    frame: 0xb0,
    flag_gp: 0x24d4,
    args: S4,
    imm: S3,
    tpl: 0x004f_b3c0,
    copy_len: 0x3c,
    copy_ret: 0x002f_b338,
    tmp: A2,
    fill: WfFill {
        buf: S1,
        cnt: S0,
        args: S4,
        pre: 4,
        table: 0x0044_5e98,
        head: 0x002f_b350,
    },
    node: WfNode {
        flag: 3,
        attach_ret: 0x002f_b420,
        alloc_cycles: 11,
        size: 0x88,
        name: 0x0048_9668,
        alloc_ret: 0x002f_b440,
        ctor: 0x0035_8c80,
        ctor_ret: 0x002f_b454,
        five_args: false,
    },
};

/// `WorldObjFactory_Conveyor`'s constants (template at `0x4fb430`, 52 bytes).
pub(crate) const WF740_CONVEYOR: WfSite = WfSite {
    frame: 0xb0,
    flag_gp: 0x24dc,
    args: S3,
    imm: S4,
    tpl: 0x004f_b430,
    copy_len: 0x34,
    copy_ret: 0x002f_b76c,
    tmp: A2,
    fill: WfFill {
        buf: S1,
        cnt: S0,
        args: S3,
        pre: 5,
        table: 0x0044_5ef8,
        head: 0x002f_b788,
    },
    node: WfNode {
        flag: 0xf,
        attach_ret: 0x002f_b858,
        alloc_cycles: 11,
        size: 0x48,
        name: 0x0048_9688,
        alloc_ret: 0x002f_b878,
        ctor: 0x0034_3010,
        ctor_ret: 0x002f_b88c,
        five_args: false,
    },
};

/// `WorldObjFactory_Boost`'s constants (template at `0x4fb4c8`, 36 bytes).
pub(crate) const WF740_BOOST: WfSite = WfSite {
    frame: 0xa0,
    flag_gp: 0x24ec,
    args: S3,
    imm: S4,
    tpl: 0x004f_b4c8,
    copy_len: 0x24,
    copy_ret: 0x002f_bf6c,
    tmp: A3,
    fill: WfFill {
        buf: S1,
        cnt: S0,
        args: S3,
        pre: 5,
        table: 0x0044_5f70,
        head: 0x002f_bf88,
    },
    node: WfNode {
        flag: 8,
        attach_ret: 0x002f_c058,
        alloc_cycles: 12,
        size: 0x50,
        name: 0x004a_3bf0,
        alloc_ret: 0x002f_c078,
        ctor: 0x0034_1388,
        ctor_ret: 0x002f_c090,
        five_args: true,
    },
};

/// One-time init of the anim-combo template (flag clear): the id word, the
/// type/value fields (-1.0, 30.0, 31.0 and zeros) and the flag.
pub(crate) fn wf740_anim_init(c: &mut Cpu, s: &WfSite) {
    c.cycles += 28;
    let tpl = s.tpl;
    c.g[V0][0] = sx(tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.lbu(tpl.wrapping_add(0x38));
    c.g[V1][0] |= 0xffff;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 2);
    c.g[A0][0] = 1;
    c.g[AT][0] = 0x41f0_0000;
    c.mtc1(AT, 0);
    c.g[AT][0] = 0x41f8_0000;
    c.mtc1(AT, 1);
    let v1 = c.g[V1][0];
    c.sw(tpl, v1);
    let a1 = c.g[A1][0];
    c.sb(tpl.wrapping_add(0x38), a1);
    c.swc1(1, tpl.wrapping_add(0x28));
    c.swc1(2, tpl.wrapping_add(0x2c));
    c.swc1(0, tpl.wrapping_add(0x30));
    let a0 = c.g[A0][0];
    c.sw(wf740_gp(c).wrapping_add(s.flag_gp), a0);
    c.sw(tpl.wrapping_add(4), a0);
    c.sw(tpl.wrapping_add(8), 0);
    c.swc1(2, tpl.wrapping_add(0xc));
    c.swc1(0, tpl.wrapping_add(0x10));
    c.swc1(0, tpl.wrapping_add(0x14));
    c.sw(tpl.wrapping_add(0x18), 0);
    c.swc1(2, tpl.wrapping_add(0x1c));
    c.sw(tpl.wrapping_add(0x20), 0);
    c.sw(tpl.wrapping_add(0x24), 0);
    c.sw(tpl.wrapping_add(0x34), 0);
    c.sb(sp(c), a1);
}

/// One-time init of the conveyor template: the id word, 100.0f at +4 and zeros
/// in the other fields, and the flag.
pub(crate) fn wf740_conveyor_init(c: &mut Cpu, s: &WfSite) {
    c.cycles += 23;
    let tpl = s.tpl;
    c.g[V0][0] = sx(tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.lbu(tpl.wrapping_add(0x30));
    c.g[V1][0] |= 0xffff;
    c.mtc1(0, 1);
    c.g[A0][0] = 1;
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 0);
    let v1 = c.g[V1][0];
    c.sw(tpl, v1);
    let a1 = c.g[A1][0];
    c.sb(tpl.wrapping_add(0x30), a1);
    c.swc1(0, tpl.wrapping_add(4));
    c.swc1(1, tpl.wrapping_add(0xc));
    let a0 = c.g[A0][0];
    c.sw(wf740_gp(c).wrapping_add(s.flag_gp), a0);
    c.swc1(1, tpl.wrapping_add(8));
    for off in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
        c.sw(tpl.wrapping_add(off), 0);
    }
    c.sb(sp(c), a1);
}

/// One-time init of the boost template: the id word, 1.0f and 0.0f fields.
pub(crate) fn wf740_boost_init(c: &mut Cpu, s: &WfSite) {
    c.cycles += 19;
    let tpl = s.tpl;
    c.g[V0][0] = sx(tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = c.lbu(tpl.wrapping_add(0x20));
    c.g[V1][0] |= 0xffff;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.g[A0][0] = 1;
    c.mtc1(0, 0);
    let v1 = c.g[V1][0];
    c.sw(tpl, v1);
    let a1 = c.g[A1][0];
    c.sb(tpl.wrapping_add(0x20), a1);
    c.swc1(1, tpl.wrapping_add(0x18));
    c.swc1(0, tpl.wrapping_add(0x1c));
    let a0 = c.g[A0][0];
    c.sw(wf740_gp(c).wrapping_add(s.flag_gp), a0);
    c.sw(tpl.wrapping_add(4), 0);
    c.swc1(1, tpl.wrapping_add(8));
    c.swc1(0, tpl.wrapping_add(0xc));
    c.swc1(1, tpl.wrapping_add(0x10));
    c.swc1(0, tpl.wrapping_add(0x14));
    c.sb(sp(c), a1);
}

/// `WorldObjFactory_AnimCombo` (0x2fb270), the C++ `MakeAnimComboFunc`:
/// builds an anim-combo parameter record in a 60-byte stack buffer (the
/// static template, then the script's parameters), finds the target instance
/// by id and attaches an `AnimCombo` node to it (node type 3), allocating one
/// when the instance has no active node that takes it. Returns the `luno` nil
/// value through the hidden pointer in `a0`.
pub fn world_obj_factory_anim_combo(c: &mut Cpu) {
    guest(c, |c| wf740_factory(c, &WF740_ANIM, wf740_anim_init));
}

/// `WorldObjFactory_Conveyor` (0x2fb6b8), the C++ `MakeConveyorFunc`: the
/// anim-combo's shape with a 52-byte buffer (speed 100.0f) and a 0x48-byte
/// conveyor node (node type 15).
pub fn world_obj_factory_conveyor(c: &mut Cpu) {
    guest(c, |c| {
        wf740_factory(c, &WF740_CONVEYOR, wf740_conveyor_init)
    });
}

/// `WorldObjFactory_Boost` (0x2fbec8), the C++ `MakeBoostFunc`: the same
/// shape with a 36-byte buffer and a 0x50-byte boost node (node type 8); its
/// constructor takes the extra argument 8.
pub fn world_obj_factory_boost(c: &mut Cpu) {
    guest(c, |c| wf740_factory(c, &WF740_BOOST, wf740_boost_init));
}

/// `WScriptFn_Modifier9` (0x2fcc20), the C++ `MakeFenceFunc`: builds a 16-byte
/// parameter record (the template with 0.5f at +8, then the script's
/// parameters), finds the instance by id (-1: the script manager's) and, when
/// it has an active node, offers the record to `WorldObjFactory_AttachModifier`
/// (type 9). Returns the `luno` nil value through the hidden pointer in `a0`.
pub fn wscript_fn_modifier9(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b5b0;
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0x2504)).lw(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[A2][0] = 0x0050_0000;
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x20);
        if flag_clear {
            // One-time init of the template and its flag.
            c.cycles += 13;
            c.g[V1][0] = sx(TEMPLATE);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(TEMPLATE.wrapping_add(0xc));
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            let v0 = c.g[V0][0];
            c.sw(TEMPLATE, v0);
            let a1 = c.g[A1][0];
            c.sb(TEMPLATE.wrapping_add(0xc), a1);
            c.swc1(0, TEMPLATE.wrapping_add(8));
            let a0 = c.g[A0][0];
            c.sw(wf740_gp(c).wrapping_add(0x2504), a0);
            c.sw(TEMPLATE.wrapping_add(4), 0);
            c.sb(sp(c), a1);
        }

        // The id word of the buffer, then the template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A2][0] = 0x10;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, WF740_MEMCPY, 0x002f_ccac)?;

        wf740_fill(
            c,
            &WfFill {
                buf: S0,
                cnt: S2,
                args: S3,
                pre: 5,
                table: 0x0044_6028,
                head: 0x002f_ccc8,
            },
        );
        wf740_lookup_v0(c, S0);

        // With an instance whose active node exists: attach (type 9).
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A2][0] = c.g[S0][0];
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 9;
                WF740_ATTACH.call(c, 0x002f_cd88)?;
            }
        }
        wf740_copy_nil(c, S4, S1);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptFn_AddPositionModifier` (0x2fd758), the C++ `AddRiderPositionFunc`:
/// builds a 24-byte parameter record, finds the instance by id and, when its
/// active node answers the type query, moves the node by the position of a
/// second object: the target object's two vtable queries, the rotated offset
/// (`VU0` matrix times the record's vector) added to the position, and
/// `ModObj_AddPositionModifier`. Returns the `luno` nil value through the
/// hidden pointer in `a0`.
pub fn wscript_fn_add_position_modifier(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b720;
        c.cycles += 12;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0x2518)).lw(c);
        frame.save(c, S1, 0xa0);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x90);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        let flag_clear = c.g[V0][0] == 0;
        frame.save_ra(c, 0x70);
        if flag_clear {
            // One-time init of the template and its flag.
            c.cycles += 15;
            c.g[V1][0] = sx(TEMPLATE);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = c.lbu(TEMPLATE.wrapping_add(0x14));
            c.g[V0][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A0][0] = u64::MAX;
            let v0 = c.g[V0][0];
            c.sw(TEMPLATE, v0);
            c.g[A1][0] = 1;
            let a2 = c.g[A2][0];
            c.sb(TEMPLATE.wrapping_add(0x14), a2);
            let a0 = c.g[A0][0];
            c.sw(TEMPLATE.wrapping_add(4), a0);
            c.swc1(0, TEMPLATE.wrapping_add(0x10));
            let a1 = c.g[A1][0];
            c.sw(wf740_gp(c).wrapping_add(0x2518), a1);
            c.swc1(0, TEMPLATE.wrapping_add(8));
            c.swc1(0, TEMPLATE.wrapping_add(0xc));
            c.sb(sp(c), a2);
        }

        // The id word of the buffer, then the template copy.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x18;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x48e0));
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WF740_MEMCPY, 0x002f_d7e4)?;

        wf740_fill(
            c,
            &WfFill {
                buf: S0,
                cnt: S1,
                args: S2,
                pre: 5,
                table: 0x0044_6190,
                head: 0x002f_d800,
            },
        );
        wf740_lookup_v0(c, S0);

        // The instance's active node (S1), if any: ask it whether it is a
        // movable node, then place it relative to the target object.
        c.cycles += 3;
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = 0x004d_0000;
        if c.g[S1][0] != 0 {
            c.cycles += 7;
            c.g[V1][0] = Addr(wf740_addr(c, S1).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            let target = wf740_addr(c, V0);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002f_d8c4)?;
            let is_node = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if is_node {
                c.cycles += 3;
                c.g[A1][0] = Addr(wf740_addr(c, S0).wrapping_add(4)).lw(c);
                let same = c.g[A1][0] == c.g[V0][0];
                c.g[A0][0] = Addr(wf740_gp(c).wrapping_sub(0x848)).lw(c);
                if !same {
                    wf740_position_move(c)?;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
        }

        // Epilogue: copy the nil value to the hidden return slot (S3).
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = wf740_addr(c, A3);
        c.ldl(4, src.wrapping_add(7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
        let dst = wf740_addr(c, S3);
        c.sdl(4, dst.wrapping_add(7));
        c.sdr(4, dst);
        let a1 = c.g[A1][0];
        Addr(dst.wrapping_add(8)).sw(c, a1);
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    });
}

/// The position part of `WScriptFn_AddPositionModifier` (the world is in
/// `A0`): the target object's two world-table queries (its object, then its
/// matrix), the `VU0` product of the matrix with the record's offset vector,
/// the sum with the object's position, and `ModObj_AddPositionModifier`.
pub(crate) fn wf740_position_move(c: &mut Cpu) -> Flow {
    c.cycles += 55;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[V1][0] = Addr(wf740_addr(c, A0).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = Addr(wf740_addr(c, V1).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = Addr(wf740_addr(c, V0).wrapping_add(0x28)).lw(c);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A1][0] = Addr(wf740_addr(c, A0)).lw(c);
    c.g[V1][0] = Addr(wf740_addr(c, A1).wrapping_add(0x28)).lh(c);
    c.g[V0][0] = Addr(wf740_addr(c, A1).wrapping_add(0x2c)).lw(c);
    let target = wf740_addr(c, V0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    call_ptr(c, target, 0x002f_d908)?;
    c.lq(3, wf740_addr(c, V0));
    c.g[A0][0] = Addr(wf740_gp(c).wrapping_sub(0x848)).lw(c);
    c.sq(3, sp(c).wrapping_add(0x30));
    c.g[A1][0] = Addr(wf740_addr(c, A0).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = Addr(wf740_addr(c, S0).wrapping_add(4)).lw(c);
    c.g[V1][0] = Addr(wf740_addr(c, A1).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(wf740_addr(c, V1).wrapping_add(0x28)).lw(c);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
    if c.g[A1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[V1][0] = Addr(wf740_addr(c, A0)).lw(c);
    c.g[A1][0] = Addr(wf740_addr(c, V1).wrapping_add(0x30)).lh(c);
    c.g[V0][0] = Addr(wf740_addr(c, V1).wrapping_add(0x34)).lw(c);
    let target = wf740_addr(c, V0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    call_ptr(c, target, 0x002f_d948)?;
    let s0 = wf740_addr(c, S0);
    c.lwc1(2, s0.wrapping_add(0x10));
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(1, s0.wrapping_add(8));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
    c.lwc1(0, s0.wrapping_add(0xc));
    c.swc1(1, sp(c).wrapping_add(0x50));
    c.swc1(0, sp(c).wrapping_add(0x54));
    c.swc1(2, sp(c).wrapping_add(0x58));
    c.sw(sp(c).wrapping_add(0x5c), 0);
    c.lqc2(8, sp(c).wrapping_add(0x50));
    let v0 = wf740_addr(c, V0);
    c.lqc2(4, v0);
    c.lqc2(5, v0.wrapping_add(0x10));
    c.lqc2(6, v0.wrapping_add(0x20));
    c.lqc2(7, v0.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, sp(c).wrapping_add(0x60));
    c.lq(2, sp(c).wrapping_add(0x60));
    c.sq(2, sp(c).wrapping_add(0x40));
    c.lqc2(3, sp(c).wrapping_add(0x30));
    c.lqc2(4, sp(c).wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x30));
    // ModObj_AddPositionModifier(node, position).
    call_ptr(c, 0x0035_58b8, 0x002f_d9b4)?;
    Ok(())
}

/// `WScriptFn_AddParentModifier` (0x2fd9f8), the C++ `AddParentFunc`: builds a
/// 28-byte parameter record whose first two words are object ids, resolves
/// both (the child and the parent), asks the parent's active node whether it
/// takes the child and, when the child's node is a movable one, calls
/// `ModObj_AddParentModifier`. Returns the `luno` nil value through the
/// hidden pointer in `a0`.
pub fn wscript_fn_add_parent_modifier(c: &mut Cpu) {
    guest(c, |c| {
        const TEMPLATE: u32 = 0x004f_b738;
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0x251c)).lw(c);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        let flag_clear = c.g[V0][0] == 0;
        c.g[A2][0] = 0x0050_0000;
        if flag_clear {
            // One-time init of the template and its flag.
            c.cycles += 15;
            c.g[V0][0] = sx(TEMPLATE);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(TEMPLATE.wrapping_add(0x18));
            c.g[V1][0] |= 0xffff;
            c.mtc1(0, 0);
            c.g[A0][0] = 1;
            let v1 = c.g[V1][0];
            c.sw(TEMPLATE, v1);
            let a1 = c.g[A1][0];
            c.sb(TEMPLATE.wrapping_add(0x18), a1);
            c.sw(TEMPLATE.wrapping_add(4), v1);
            c.swc1(0, TEMPLATE.wrapping_add(0x14));
            let a0 = c.g[A0][0];
            c.sw(wf740_gp(c).wrapping_add(0x251c), a0);
            c.sw(TEMPLATE.wrapping_add(8), 0);
            c.swc1(0, TEMPLATE.wrapping_add(0xc));
            c.swc1(0, TEMPLATE.wrapping_add(0x10));
            c.sb(sp(c), a1);
        }

        // Both id words of the buffer, then the template copy.
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A2][0] = 0x1c;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x14)).sw(c, v0);
        c.g[A0][0] = c.g[S1][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        call_ptr(c, WF740_MEMCPY, 0x002f_da88)?;

        wf740_fill(
            c,
            &WfFill {
                buf: S1,
                cnt: S0,
                args: S2,
                pre: 4,
                table: 0x0044_61a8,
                head: 0x002f_daa0,
            },
        );

        // The child instance (id in buf[0]) into S0.
        c.cycles += 4;
        c.g[A2][0] = Addr(wf740_addr(c, S1)).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(wf740_gp(c).wrapping_add(0xce8)).lw(c);
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            wf740_lookup_tail_v0(c);
            c.cycles += 1;
            c.g[S0][0] = c.g[V0][0];
        }

        // The parent (id in buf[1]); with no parent the work is skipped.
        'done: {
            'exit: {
                'parent: {
                    c.cycles += 4;
                    c.g[A2][0] = Addr(wf740_addr(c, S1).wrapping_add(4)).lw(c);
                    c.g[V0][0] = !c.g[A2][0];
                    let no_parent = c.g[V0][0] == 0;
                    c.g[A1][0] = Addr(wf740_gp(c).wrapping_add(0x16c8)).lw(c);
                    if no_parent {
                        break 'parent;
                    }
                    wf740_lookup_tail_v0(c);
                    // The parent's active node (S2).
                    c.cycles += 2;
                    let missing = c.g[V0][0] == 0;
                    c.g[S2][0] = 0;
                    if !missing {
                        c.cycles += 1;
                        c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    }
                    c.cycles += 2;
                    let no_node = c.g[S2][0] == 0;
                    c.g[A1][0] = 6;
                    if no_node {
                        break 'parent;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr(wf740_addr(c, S2).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                    let target = wf740_addr(c, V0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dbc0)?;
                    let takes = c.g[V0][0] != 0;
                    c.g[V1][0] = 0x004d_0000;
                    if takes {
                        break 'exit;
                    }
                    // The child's node (S0).
                    c.cycles += 3;
                    c.g[S0][0] = Addr(wf740_addr(c, S0).wrapping_add(0xc)).lw(c);
                    let no_child = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S3][0];
                    if no_child {
                        break 'done;
                    }
                    c.cycles += 7;
                    c.g[V1][0] = Addr(wf740_addr(c, S0).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    let target = wf740_addr(c, V0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_dbe8)?;
                    let not_movable = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if not_movable {
                        break 'parent;
                    }
                    c.cycles += 11;
                    let s1 = wf740_addr(c, S1);
                    c.lwc1(2, s1.wrapping_add(0xc));
                    c.lwc1(0, s1.wrapping_add(0x10));
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x30));
                    c.lwc1(1, s1.wrapping_add(0x14));
                    c.g[A1][0] = Addr(wf740_addr(c, S2).wrapping_add(0x18)).lw(c);
                    c.swc1(2, sp(c).wrapping_add(0x30));
                    c.swc1(0, sp(c).wrapping_add(0x34));
                    c.swc1(1, sp(c).wrapping_add(0x38));
                    c.sw(sp(c).wrapping_add(0x3c), 0);
                    c.g[A2][0] = Addr(s1.wrapping_add(8)).lw(c);
                    // ModObj_AddParentModifier(child node, parent object, ..).
                    call_ptr(c, 0x0035_5978, 0x002f_dc1c)?;
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }

        // Epilogue: copy the nil value to the hidden return slot (S3).
        c.cycles += 14;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        let src = wf740_addr(c, A3);
        c.ldl(4, src.wrapping_add(7));
        c.ldr(4, src);
        c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
        let dst = wf740_addr(c, S3);
        c.sdl(4, dst.wrapping_add(7));
        c.sdr(4, dst);
        let a1 = c.g[A1][0];
        Addr(dst.wrapping_add(8)).sw(c, a1);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `gp` of the game (the world and script-manager globals are gp-relative).
pub(crate) fn wsc_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// `EA_MemCopy`, which copies the static template into the stack buffer.
pub(crate) const WSC_MEMCPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38), reached by a direct call.
pub(crate) const WSC_ATTACH: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::f_002fae38,
};

/// `EA_MemAllocNamed` (0x317d70), reached by a direct call.
pub(crate) const WSC_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::f_00317d70,
};

/// The parameter loop shared by the script functions: each `(idx, val, type)`
/// entry (16 bytes, params in `A2`, count in the `count` register) stores its
/// value into `buf[idx]` (`buf` register), as a float when the type table
/// (`types`, kept in `T0`) wants a float and the entry's type differs.
pub(crate) struct WscFill {
    /// Register holding the parameter count (also copied to `A1`).
    count: usize,
    /// Register holding the parameter list.
    params: usize,
    /// Register holding the stack buffer.
    buf: usize,
    /// Cycles charged before the loop.
    pre: u64,
    /// `V0 = 0x440000` is set before the loop (the rails' table is `0x44xxxx`).
    set_v0: bool,
    /// Value loaded in `T0` (the type table's address).
    types: u32,
    /// Event point of the loop's back edge.
    head: u32,
}

pub(crate) fn wsc_fill(c: &mut Cpu, s: &WscFill) {
    c.g[A2][0] = c.g[s.params][0];
    if (c.g[s.count][0] as i64) <= 0 {
        return;
    }
    c.cycles += s.pre;
    if s.set_v0 {
        c.g[V0][0] = 0x0044_0000;
    }
    c.g[A1][0] = c.g[s.count][0];
    c.g[T0][0] = sx(s.types);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[s.buf][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, s.head);
        }
    }
}

/// The instance for the id in `buf[0]` (`buf` register): the script
/// manager's for -1, else the section entry's record (`0` when there is
/// none). Leaves it in V0 (the shape of `Unhide` and `KillRestorable`).
pub(crate) fn wsc_resolve(c: &mut Cpu, buf: usize) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[buf][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wsc_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// The rails' instance lookup: like [`wsc_resolve`] but the result goes to
/// `S0` (and `A1`) and the instruction counts differ.
pub(crate) fn wsc_rail_resolve(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wsc_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    if c.g[V1][0] != 0 {
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A1][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[A1][0] = 0;
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[A1][0];
}

/// Per-rail constants of `WorldObjFactory_FloatRail` / `_SpringRail`: the two
/// factories share one shape (a 88-byte parameter record with a static
/// template) and differ in these addresses and in the node construction.
pub(crate) struct WscRail {
    /// `gp` offset of the one-time init flag.
    flag_gp: u32,
    /// The static record template (`0x500000 - off`).
    tpl: u32,
    /// The parameter type table.
    types: u32,
    /// Return address of the template copy and event point of the fill loop.
    copy_ret: u32,
    fill_head: u32,
    /// The `checkActiveNode` type (`A1` of the attach call) and its return.
    attach_type: u64,
    attach_ret: u32,
    /// Builds and constructs the rail node (the alloc tail).
    node: fn(&mut Cpu) -> Flow,
}

/// The floating-rail node: `cMemMan_alloc(0x80, name, 0x20000000, 0)` and its
/// constructor `(node, instance, buf)`.
pub(crate) fn wsc_float_rail_node(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A1][0] = 0x0049_0000;
    c.g[A0][0] = 0x80;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6898));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    WSC_ALLOC.call(c, 0x002f_f598)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x0034_8d98, 0x002f_f5a8)
}

/// The spring-rail node: scales the record's float at `+0x50` by a constant
/// from gp, then allocates and constructs the node.
pub(crate) fn wsc_spring_rail_node(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let buf = c.g[S1][0] as u32;
    c.lwc1(0, buf.wrapping_add(0x50));
    c.g[A1][0] = 0x0049_0000;
    c.lwc1(1, wsc_gp(c).wrapping_sub(0x336c));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6888));
    c.g[A0][0] = 0x80;
    c.g[A2][0] = 0x2000_0000;
    c.mul_s(0, 0, 1);
    c.g[A3][0] = 0;
    c.swc1(0, buf.wrapping_add(0x50));
    WSC_ALLOC.call(c, 0x002f_f800)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x0034_9b48, 0x002f_f810)
}

pub(crate) const WSC_FLOAT_RAIL: WscRail = WscRail {
    flag_gp: 0x2544,
    tpl: 0x004f_ba38,
    types: 0x0044_6480,
    copy_ret: 0x002f_f490,
    fill_head: 0x002f_f4a8,
    attach_type: 0x14,
    attach_ret: 0x002f_f578,
    node: wsc_float_rail_node,
};

pub(crate) const WSC_SPRING_RAIL: WscRail = WscRail {
    flag_gp: 0x2548,
    tpl: 0x004f_ba90,
    types: 0x0044_64d8,
    copy_ret: 0x002f_f6e8,
    fill_head: 0x002f_f700,
    attach_type: 0x15,
    attach_ret: 0x002f_f7d0,
    node: wsc_spring_rail_node,
};

/// The shared body of the two rail factories `(ret slot, n, params)`: builds
/// the 22-word parameter record on the stack (template copy plus the
/// parameter list), finds the instance for its id, and unless the instance
/// already has a matching node attaches a new rail node to it. Returns the
/// script nil value through the hidden return slot (`S2`).
pub(crate) fn wsc_rail_factory(c: &mut Cpu, s: &WscRail) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0xd0);
    c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(s.flag_gp)).lw(c);
    frame.save(c, S0, 0xc0);
    c.g[A3][0] = 0x0050_0000;
    frame.save(c, S2, 0xa0);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S4, 0x80);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S1, 0xb0);
    c.g[S4][0] = c.g[A2][0];
    frame.save(c, S3, 0x90);
    let flag_clear = c.g[V0][0] == 0;
    frame.save_ra(c, 0x70);
    if flag_clear {
        // One-time init of the template: all ids -1, 0.5f and 1.0f scale
        // floats, the flag set.
        c.cycles += 34;
        c.g[V0][0] = sx(s.tpl);
        c.g[V1][0] = u64::MAX;
        c.g[A2][0] = Addr(s.tpl.wrapping_add(0x54)).lbu(c);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 1);
        c.g[A0][0] = u64::MAX;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A1][0] = 1;
        let (v1, a0, a1, a2) = (c.g[V1][0], c.g[A0][0], c.g[A1][0], c.g[A2][0]);
        Addr(s.tpl).sw(c, v1);
        Addr(s.tpl + 0x54).sb(c, a2);
        Addr(s.tpl + 0x20).sw(c, v1);
        Addr(s.tpl + 0x40).sw(c, a0);
        c.swc1(1, s.tpl + 0x4c);
        c.swc1(0, s.tpl + 0x50);
        Addr(wsc_gp(c).wrapping_add(s.flag_gp)).sw(c, a1);
        for off in [0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
            Addr(s.tpl + off).sw(c, v1);
        }
        for off in [0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c] {
            Addr(s.tpl + off).sw(c, a0);
        }
        Addr(s.tpl + 0x44).sw(c, a1);
        c.swc1(1, s.tpl + 0x48);
        let a2 = c.g[A2][0];
        Addr(sp(c)).sb(c, a2);
    }

    // The record's first nine words are -1, then the template copy.
    c.cycles += 19;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0x58;
    c.g[A1][0] = sx(s.tpl);
    let buf = sp(c).wrapping_add(0x10);
    for i in 0..9 {
        Addr(buf.wrapping_add(4 * i)).sw(c, u64::MAX);
    }
    c.g[A0][0] = c.g[S1][0];
    c.g[S3][0] = 0x004d_0000;
    call_ptr(c, WSC_MEMCPY, s.copy_ret)?;

    wsc_fill(
        c,
        &WscFill {
            count: S0,
            params: S4,
            buf: S1,
            pre: 4,
            set_v0: true,
            types: s.types,
            head: s.fill_head,
        },
    );
    wsc_rail_resolve(c);

    // Attach a rail node unless the instance is missing or its active node
    // already took the modifier.
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A1][0] = s.attach_type;
        let mut done = false;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[A2][0] = c.g[S1][0];
            WSC_ATTACH.call(c, s.attach_ret)?;
            done = c.g[V0][0] != 0;
        }
        if !done {
            (s.node)(c)?;
        }
    }

    // Epilogue: copy the nil value (12 bytes) to the return slot.
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A1][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S2][0] as u32;
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    let v1 = c.g[V1][0];
    Addr(dst.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0xc0);
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore_ra(c, 0x70);
    frame.close(c);
    Ok(())
}

/// `WorldObjFactory_FloatRail` (0x2ff390), the C++ `MakeFloatRailFunc`: the
/// script call that gives an instance a floating-rail node, built from the
/// script's parameter list.
pub fn world_obj_factory_float_rail(c: &mut Cpu) {
    guest(c, |c| wsc_rail_factory(c, &WSC_FLOAT_RAIL));
}

/// `WorldObjFactory_SpringRail` (0x2ff5e8): the same with a spring-rail node
/// (its `+0x50` float is scaled by a gp constant before the node is built).
pub fn world_obj_factory_spring_rail(c: &mut Cpu) {
    guest(c, |c| wsc_rail_factory(c, &WSC_SPRING_RAIL));
}

/// The epilogue of `Unhide` and `KillRestorable`: copies the 12-byte nil value
/// (at `V1 - 0x6f68`) to the return slot (`S3`), leaves that pointer in V0,
/// restores the saved registers and returns.
pub(crate) fn wsc_nil_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S3][0] as u32;
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// The prologue of `Unhide` and `KillRestorable` (frame 0x70, `S1` = count,
/// `S2` = params, `S3` = return slot): returns whether the init flag at
/// `gp + flag_gp` was clear. `kill` selects the order of the two register
/// saves (the compiled prologues differ).
pub(crate) fn wsc_script_prologue(
    c: &mut Cpu,
    flag_gp: u32,
    cycles: u64,
    kill: bool,
) -> (Frame, bool) {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x70);
    c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(flag_gp)).lw(c);
    if kill {
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
    } else {
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
    }
    c.g[S3][0] = c.g[A0][0];
    let clear = c.g[V0][0] == 0;
    frame.save_ra(c, 0x20);
    (frame, clear)
}

/// `WScriptFn_Unhide` (0x2ffd58), the C++ `UnhideFunc`: unhides the instance
/// named by the first parameter: when its active node answers `isType(0x10)`
/// the node's `delete(3)` runs.
pub fn wscript_fn_unhide(c: &mut Cpu) {
    guest(c, |c| {
        let (frame, clear) = wsc_script_prologue(c, 0x2570, 11, false);
        if clear {
            c.cycles += 7;
            c.g[V0][0] = Addr(wsc_gp(c).wrapping_add(0x256c)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            let (v0, v1, a0) = (c.g[V0][0], c.g[V1][0], c.g[A0][0]);
            Addr(wsc_gp(c).wrapping_add(0x2568)).sw(c, v1);
            Addr(wsc_gp(c).wrapping_add(0x2570)).sw(c, a0);
            Addr(sp(c)).sb(c, v0);
        }
        // The two-word record (id -1, template word), then the parameters.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(wsc_gp(c).wrapping_add(0x2568));
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        call_ptr(c, WSC_MEMCPY, 0x002f_fdc0)?;
        wsc_fill(
            c,
            &WscFill {
                count: S1,
                params: S2,
                buf: S0,
                pre: 4,
                set_v0: false,
                types: wsc_gp(c).wrapping_add(0xb30),
                head: 0x002f_fdd8,
            },
        );
        wsc_resolve(c, S0);

        c.cycles += 2;
        c.g[V1][0] = 0x004d_0000;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 0x10;
            if c.g[S0][0] != 0 {
                // node->isType(0x10), then node->delete(3).
                c.cycles += 7;
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x54)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_fea4)?;
                c.g[A1][0] = 3;
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002f_fec0)?;
                }
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
            }
        }
        wsc_nil_epilogue(c, frame);
        Ok(())
    });
}

/// `WScriptFn_KillRestorable` (0x300418), the C++ `KillRestorableFunc`: when
/// the instance named by the first parameter has an active node, calls its
/// `expire(0)` or `expire(1)` as the second parameter says (nothing for other
/// values).
pub fn wscript_fn_kill_restorable(c: &mut Cpu) {
    guest(c, |c| {
        let (frame, clear) = wsc_script_prologue(c, 0x2580, 12, true);
        c.g[A2][0] = 0x0050_0000;
        if clear {
            c.cycles += 10;
            c.g[V1][0] = sx(0x004f_bb38);
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = Addr(0x004f_bb38u32.wrapping_add(8)).lbu(c);
            let (v0, a1) = (c.g[V0][0], c.g[A1][0]);
            Addr(0x004f_bb38).sw(c, v0);
            c.g[A0][0] = 1;
            Addr(0x004f_bb38u32.wrapping_add(8)).sb(c, a1);
            let a0 = c.g[A0][0];
            Addr(wsc_gp(c).wrapping_add(0x2580)).sw(c, a0);
            Addr(0x004f_bb38u32.wrapping_add(4)).sw(c, 0);
            Addr(sp(c)).sb(c, a1);
        }
        // The three-word record (id -1, template words), then the parameters.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx(0x004f_bb38);
        c.g[A2][0] = 0xc;
        let v0 = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WSC_MEMCPY, 0x0030_0490)?;
        wsc_fill(
            c,
            &WscFill {
                count: S1,
                params: S2,
                buf: S0,
                pre: 4,
                set_v0: false,
                types: wsc_gp(c).wrapping_add(0xb40),
                head: 0x0030_04a8,
            },
        );
        wsc_resolve(c, S0);

        c.cycles += 3;
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = 0x004d_0000;
        if c.g[A2][0] != 0 {
            // node->expire(buf[1]) for the values 0 and 1.
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
            c.g[V0][0] = 1;
            if c.g[A0][0] == 0 {
                c.cycles += 8;
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x110)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x114)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_057c)?;
                c.g[V1][0] = 0x004d_0000;
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0x004d_0000;
                if c.g[A0][0] == c.g[V0][0] {
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A1][0] = 1;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x110)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x114)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_05a4)?;
                    c.g[V1][0] = 0x004d_0000;
                }
            }
        }
        wsc_nil_epilogue(c, frame);
        Ok(())
    });
}

/// The 0x70-byte WScript frame and prologue: saves s0..s3/ra, moves the
/// three arguments into s1/s2/s3 (`regs`), loads the once-init guard at
/// `guard` (an address). Returns whether the guard was already set.
pub(crate) fn oc742_prologue(c: &mut Cpu, cycles: u64, guard: u32, regs: [usize; 3]) -> bool {
    c.cycles += cycles;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.g[V0][0] = c.lw(guard);
    c.sq(17, sp(c).wrapping_add(0x50));
    c.sq(18, sp(c).wrapping_add(0x40));
    c.g[S1][0] = c.g[regs[0]][0];
    c.sq(19, sp(c).wrapping_add(0x30));
    c.g[S2][0] = c.g[regs[1]][0];
    c.sq(16, sp(c).wrapping_add(0x60));
    c.g[S3][0] = c.g[regs[2]][0];
    let initialised = c.g[V0][0] != 0;
    c.sd(sp(c).wrapping_add(0x20), c.g[RA][0]);
    initialised
}

/// The one-time init of the shared 8-byte id block (`ids`, byte copy at
/// `ids + 4`, guard at `guard`): `ids = -1`, the guard set, the tag byte
/// copied to the stack.
pub(crate) fn oc742_init_block(c: &mut Cpu, ids: u32, guard: u32) {
    c.cycles += 7;
    c.g[V0][0] = c.lbu(ids.wrapping_add(4));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 0x1;
    c.sw(ids, c.g[V1][0]);
    c.sw(guard, c.g[A0][0]);
    c.sb(sp(c), c.g[V0][0]);
}

/// `MEM_copy(vp, ids, 8)` with `vp = sp + 0x10` (s0) after storing -1 at
/// `vp`, then the `(params, count)` fill loop: each argument is stored into
/// the block by its slot, as a float when the kind table says float and the
/// argument's kind differs, else as an int.
pub(crate) fn oc742_copy_fill(c: &mut Cpu, src: u32, ret_addr: u32, table: u32, top: u32) -> Flow {
    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0x8;
    c.g[A1][0] = sx(src);
    c.g[A0][0] = c.g[S0][0];
    c.sw(sp(c).wrapping_add(0x10), c.g[V0][0]);
    call_ptr(c, 0x003e_6574, ret_addr)?;
    let (vp, count, params) = (c.g[S0][0], c.g[S1][0], c.g[S2][0]);
    c.g[A2][0] = params;
    if (count as i64) <= 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
    Ok(())
}

/// The zone-tag lookup from `vp` (the x word): the tag table at `gp+0x16c8`
/// maps x (low byte, then the next bits) to a small index in v0; with x = -1
/// v0 is `*(gp+0xce8)` (the caller then loads `->0x290`). Returns whether x
/// was not -1. `x_cycles` is the block charge of the x = -1 path.
pub(crate) fn oc742_zone_lookup(c: &mut Cpu, vp: u64, x_cycles: u64) -> bool {
    let gp = c.g[GP][0] as u32;
    c.cycles += 4;
    c.g[A2][0] = c.lw(vp as u32);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let tagged = c.g[V0][0] != 0;
    c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
    if !tagged {
        c.cycles += x_cycles;
        c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
        return false;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if found {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if nonzero {
            return true;
        }
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
    true
}

/// The `luno_gLunoNil` struct return (12 bytes through `sret`).
pub(crate) fn oc742_return_nil(c: &mut Cpu, sret: u64) {
    c.g[V1][0] = 0x4d0000;
    c.g[V0][0] = sret;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (sret as u32).wrapping_add(0x7));
    c.sdr(4, sret as u32);
    c.sw((sret as u32).wrapping_add(0x8), c.g[A1][0]);
}

/// Restore s0..s3, ra and the 0x70 frame, return.
pub(crate) fn oc742_epilogue(c: &mut Cpu) {
    c.lq(16, sp(c).wrapping_add(0x60));
    c.lq(17, sp(c).wrapping_add(0x50));
    c.lq(18, sp(c).wrapping_add(0x40));
    c.lq(19, sp(c).wrapping_add(0x30));
    c.g[RA][0] = c.ld(sp(c).wrapping_add(0x20));
    c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
    ret(c);
}

/// `WScriptFn_Kill` (0x3005e8): fills an 8-byte block (x), looks up the
/// tagged object for x (x = -1: the current one, `*(gp+0xce8)->0x290`) and,
/// when the object has a script-object part (`+0xc`), calls its vtable method
/// at `+0x118/+0x11c` (the "kill"/lose-focus call). Returns `luno_gLunoNil`
/// through sret (a0).
pub fn wscript_fn_kill(c: &mut Cpu) {
    guest(c, kill_body)
}

pub(crate) fn kill_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc742_prologue(c, 11, gp.wrapping_add(0x2590), [A1, A2, A0]);
    if !initialised {
        oc742_init_block(c, gp.wrapping_add(0x2588), gp.wrapping_add(0x2590));
    }
    oc742_copy_fill(
        c,
        gp.wrapping_add(0x2588),
        0x0030_0650,
        gp.wrapping_add(0xb48),
        0x0030_0668,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc742_zone_lookup(c, vp, 3);
    if !tagged {
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    }
    c.cycles += 3;
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let no_part = c.g[A1][0] == 0;
    c.g[V1][0] = 0x4d0000;
    if !no_part {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_072c)?;
        c.g[V1][0] = 0x4d0000;
    }
    c.cycles += 15;
    let sret = c.g[S3][0];
    oc742_return_nil(c, sret);
    oc742_epilogue(c);
    Ok(())
}

/// `WScriptFn_ResetRiderAtObject` (0x300770): the twin of
/// `PlaceRiderAtObject` with its own block (`gp+0x2598`), kind table
/// (`gp+0xb50`) and rider vtable slot (`+0x50/+0x54`): fills an 8-byte block
/// (x), looks up the tagged object for x, and when the rider index at
/// `*(gp+0xce8)` is not negative calls the object's vtable method and the
/// world's rider vtable method. Returns `luno_gLunoNil` through sret (a0).
pub fn wscript_fn_reset_rider_at_object(c: &mut Cpu) {
    guest(c, reset_rider_body)
}

pub(crate) fn reset_rider_body(c: &mut Cpu) -> Flow {
    let gp = c.g[GP][0] as u32;
    let initialised = oc742_prologue(c, 11, gp.wrapping_add(0x25a0), [A1, A2, A0]);
    if !initialised {
        oc742_init_block(c, gp.wrapping_add(0x2598), gp.wrapping_add(0x25a0));
    }
    oc742_copy_fill(
        c,
        gp.wrapping_add(0x2598),
        0x0030_07d8,
        gp.wrapping_add(0xb50),
        0x0030_07f0,
    )?;

    let vp = c.g[S0][0];
    let tagged = oc742_zone_lookup(c, vp, 4);
    if !tagged {
        // x == -1: a2 = v0 = *(gp+0xce8); v0 = a2->0x290.
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
    } else {
        c.cycles += 1;
        c.g[A2][0] = c.lw(gp.wrapping_add(0xce8));
    }

    c.cycles += 3;
    c.g[S0][0] = c.lw(c.g[A2][0] as u32);
    let negative = (c.g[S0][0] as i64) < 0;
    c.g[V1][0] = 0x4d0000;
    let mut skip_move = false;
    if !negative {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.g[S3][0];
            skip_move = true;
        } else {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let no_obj = c.g[A2][0] == 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
            if !no_obj {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_08d0)?;
            }
            c.cycles += 14;
            c.g[A0][0] = c.lw(gp.wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let target = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, target, 0x0030_0904)?;
            c.g[V1][0] = 0x4d0000;
        }
    }
    if !skip_move {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
    }
    c.cycles += 14;
    let sret = c.g[S3][0];
    oc742_return_nil(c, sret);
    oc742_epilogue(c);
    Ok(())
}

/// `Result* WScriptFn_NextTask(Result* ret, int n, Arg* args)` (0x302df0):
/// the world-script command that advances to the next task. Binds the call's
/// arguments into a 2-word record (defaults `{-1, ..}` set once on first
/// use, a typed int given for a float slot is converted), then calls
/// `WScriptMan_ProcessCommand(world manager, rec[0], 2)` and returns the
/// static "no value" result through `ret`.
///
/// Resume entries (`pc` inside the body) go to the generated `__entries`
/// twin; the orchestrator keeps that fallback at install.
pub fn w_script_fn_next_task(c: &mut Cpu) {
    if c.pc != 0x0030_2df0 {
        return w_script_fn_next_task_resume(c);
    }
    guest(c, |c| {
        'b_302e38: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26b0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A2][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A0][0];
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            if t {
                break 'b_302e38;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x26ac));
                c.g[V1][0] = 0xffffffffffff0000;
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[A0][0] = 0x1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x26a8), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x26b0), c.g[A0][0]);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                break 'b_302e38;
            }
        }
        'b_302ea8: {
            c.cycles += 7;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A2][0] = 0x8;
            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x26a8));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e6574, 0x00302e4c)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_302ea8;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xc18));
                c.g[A3][0] = 0x2;
                'l_302e60: loop {
                    'b_302e9c: {
                        'b_302e98: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_302e98;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_302e98;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    let t = true;
                                    c.swc1(0, c.g[A0][0] as u32);
                                    if t {
                                        break 'b_302e9c;
                                    } else {
                                        break 'b_302e98;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_302e9c;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00302e60);
                        }
                        continue 'l_302e60;
                    } else {
                        break 'b_302ea8;
                    }
                }
            }
        }
        c.cycles += 20;
        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A2][0] = 0x2;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x0030a728, 0x00302eb8)?;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// Native resume of `w_script_fn_next_task` (converted from the `entries` body): any entry
/// other than 0x00302df0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn w_script_fn_next_task_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00302df0 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26b0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.g[A0][0];
                    let t = c.g[V0][0] != 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        pc = 0x00302e38;
                        continue;
                    }
                    pc = 0x00302e1c;
                    continue;
                }
                0x00302e1c => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x26ac));
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A0][0] = 0x1;
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x26a8), c.g[V1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x26b0), c.g[A0][0]);
                    pc = 0x00302e34;
                    continue;
                }
                0x00302e34 => {
                    c.cycles += 1;
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    pc = 0x00302e38;
                    continue;
                }
                0x00302e38 => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = 0x8;
                    c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x26a8));
                    c.g[RA][0] = sx(0x00302e4c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e6574, 0x00302e4c)?;
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x00302ea8;
                        continue;
                    }
                    pc = 0x00302e54;
                    continue;
                }
                0x00302e54 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = sx((c.g[GP][0] as u32).wrapping_add(0xc18));
                    c.g[A3][0] = 0x2;
                    pc = 0x00302e60;
                    continue;
                }
                0x00302e60 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x00302e98;
                        continue;
                    }
                    pc = 0x00302e80;
                    continue;
                }
                0x00302e80 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x00302e98;
                        continue;
                    }
                    pc = 0x00302e88;
                    continue;
                }
                0x00302e88 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    let t = true;
                    c.swc1(0, c.g[A0][0] as u32);
                    if t {
                        pc = 0x00302e9c;
                        continue;
                    }
                    pc = 0x00302e98;
                    continue;
                }
                0x00302e98 => {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    pc = 0x00302e9c;
                    continue;
                }
                0x00302e9c => {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x00302e60) {
                            return Ok(());
                        }
                        pc = 0x00302e60;
                        continue;
                    }
                    pc = 0x00302ea8;
                    continue;
                }
                0x00302ea8 => {
                    c.cycles += 20;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A2][0] = 0x2;
                    c.g[RA][0] = sx(0x00302eb8);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    Callee {
                        addr: 0x0030a728,
                        f: ee::s10::WScriptMan_ProcessCommand,
                    }
                    .call(c, 0x00302eb8)?;
                    c.g[V1][0] = 0x4d0000;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
                    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[A3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[S3][0] as u32);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00302df0, 0x00302ef8);
                    return Ok(());
                }
            }
        }
    });
}

/// The `(params, n)` loop of the WScript bodies: each argument is stored
/// into the block at `vp` by its index, as a float when the kind table says
/// float and the argument's kind differs, else as an int. `a2` holds the
/// parameter list and `count > 0` was checked; `entry` is the loop's entry
/// charge, `top` its back-edge head, `kinds` the kind table.
pub(crate) fn oc745_fill_args(c: &mut Cpu, vp: u64, count: u64, kinds: u32, entry: u64, top: u32) {
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(kinds);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The shared tail: copy the 12 bytes of `luno_gLunoNil` (0x4c9098, `v1` is
/// 0x4d0000 here) into the result `s3`, restore and return it.
pub(crate) fn oc745_return_nil(c: &mut Cpu, cycles: u64) {
    c.cycles += cycles;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(4, c.g[A3][0] as u32);
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
    c.sdr(4, c.g[S3][0] as u32);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    let s = sp(c);
    c.lq(16, s.wrapping_add(0x60));
    c.lq(17, s.wrapping_add(0x50));
    c.lq(18, s.wrapping_add(0x40));
    c.lq(19, s.wrapping_add(0x30));
    c.g[RA][0] = c.ld(s.wrapping_add(0x20));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    ret(c);
}

/// Opens the 0x70-byte frame both functions use (the register saves follow
/// in the bodies, interleaved as the generated code has them).
pub(crate) fn oc745_save_frame(c: &mut Cpu) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
}

/// `LunoVMRegister WScriptFn_Crowd3dControl(LunoVMRegister* out, int n,
/// Param* params)` (0x3039f0, `Crowd3dFunc`): a script call with a
/// (target, command) argument pair. Resolves the target (the manager's own
/// instance, or a world-cache instance record) and, for command 0 or 1,
/// adds or removes it in the crowd manager (`gp+0xf00`). Returns nil.
pub fn wscript_fn_crowd3d_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        oc745_save_frame(c);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26ec));
        let s = sp(c);
        c.sq(18, s.wrapping_add(0x40));
        c.sq(17, s.wrapping_add(0x50));
        c.g[S2][0] = c.g[A2][0];
        c.sq(19, s.wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, s.wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        let ra = c.g[RA][0];
        c.sd(s.wrapping_add(0x20), ra);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x500000;
        if !initialised {
            // Static init of the 12-byte argument block at 0x4fbc70.
            c.cycles += 11;
            c.g[V1][0] = sx(0x500000u32.wrapping_sub(0x4390));
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw(0x500000u32.wrapping_sub(0x4390), c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[V0][0] = 0x1;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x26ec), c.g[V0][0]);
            let s = sp(c);
            c.sb(s, c.g[A1][0]);
        }
        // Copy the block into the frame's 12-byte buffer, then the arg loop.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4390));
        c.g[A2][0] = 0xc;
        let s = sp(c);
        c.sw(s.wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3a6c)?;
        let no_args = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_args {
            let vp = c.g[S0][0];
            let count = c.g[S1][0];
            let kinds = (c.g[GP][0] as u32).wrapping_add(0xc48);
            oc745_fill_args(c, vp, count, kinds, 3, 0x0030_3a80);
        }
        // The target: buf[0] == -1 is the manager's instance, else a record.
        c.cycles += 4;
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
        if !by_id {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let no_section = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if no_section {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !found {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 2;
        let no_obj = c.g[V0][0] == 0;
        c.g[V1][0] = 0x4d0000;
        if !no_obj {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let cmd_zero = c.g[A0][0] == 0;
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            if cmd_zero {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xf00));
                call_ptr(c, 0x0034_3c60, 0x0030_3b40)?;
                c.g[V1][0] = 0x4d0000;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let other = c.g[A0][0] != c.g[V0][0];
                c.g[V1][0] = 0x4d0000;
                if !other {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xf00));
                    call_ptr(c, 0x0034_3f38, 0x0030_3b5c)?;
                    c.g[V1][0] = 0x4d0000;
                }
            }
        }
        oc745_return_nil(c, 15);
        Ok(())
    });
}

/// `LunoVMRegister WScriptFn_SetHudHeightTarget(LunoVMRegister* out, int n,
/// Param* params)` (0x304320, `UpdateDisplayHeightFunc`): a script call with
/// one target argument. Resolves the target like `Crowd3dControl` and passes
/// the manager's first word to `WScriptMan_SetHudHeightTarget` (0x30b508)
/// with the target as the hud. Returns nil.
pub fn wscript_fn_set_hud_height_target(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        oc745_save_frame(c);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2708));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x50));
        c.sq(18, s.wrapping_add(0x40));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, s.wrapping_add(0x30));
        c.g[S2][0] = c.g[A2][0];
        c.sq(16, s.wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        let initialised = c.g[V0][0] != 0;
        let ra = c.g[RA][0];
        c.sd(s.wrapping_add(0x20), ra);
        if !initialised {
            // Static init of the 8-byte argument block at gp+0x2700.
            c.cycles += 7;
            c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2704));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 0x1;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2700), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2708), c.g[A0][0]);
            let s = sp(c);
            c.sb(s, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2700));
        let s = sp(c);
        c.sw(s.wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x0030_4388)?;
        let no_args = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_args {
            let vp = c.g[S0][0];
            let count = c.g[S1][0];
            let kinds = (c.g[GP][0] as u32).wrapping_add(0xc68);
            oc745_fill_args(c, vp, count, kinds, 4, 0x0030_43a0);
        }
        // The manager and the target.
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[A3][0] = c.g[V0][0];
        c.g[V1][0] = !c.g[A2][0];
        let by_id = c.g[V1][0] != 0;
        c.g[T0][0] = c.lw(c.g[V0][0] as u32);
        if !by_id {
            c.cycles += 2;
            c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x290));
        } else {
            c.cycles += 9;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            let has_section = c.g[A0][0] != 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            if !has_section {
                c.cycles += 2;
                c.g[A2][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if !found {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            }
        }
        c.cycles += 2;
        let no_obj = c.g[A2][0] == 0;
        c.g[A0][0] = c.g[A3][0];
        if !no_obj {
            c.cycles += 2;
            c.g[A1][0] = c.g[T0][0];
            Callee {
                addr: 0x0030_b508,
                f: ee::s10::f_0030b508,
            }
            .call(c, 0x0030_4468)?;
        }
        c.g[V1][0] = 0x4d0000;
        oc745_return_nil(c, 16);
        Ok(())
    });
}

/// `gp + off` (the game's gp-relative globals, `off` as a wrapping offset).
pub(crate) fn w746_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// The script-parameter loop shared by both functions: `a1` entries of
/// 16 bytes at `a2` (`idx`, `val`, -, `type`); each `val` goes into word `idx`
/// of the buffer in register `buf` (a float when the kind table at `t0` says 2
/// and the entry's type differs). `head` is the loop head the event check
/// reports.
pub(crate) fn w746_param_loop(c: &mut Cpu, buf: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[buf][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
            c.cycles += 1;
            let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
            Addr(dst).sw(c, v);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
                c.cycles += 1;
                let (dst, v) = (c.g[A0][0] as u32, c.g[V0][0]);
                Addr(dst).sw(c, v);
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The fields of the rider-config record (16 words) packed into the 64-bit
/// word at `sp+0x60`: `(word offset, value mask, bit shift)`, in the order
/// the generated body merges them.
pub(crate) const W746_RIDER_CONFIG_FIELDS: [(u32, u64, u32); 16] = [
    (0x00, 0x7, 0),
    (0x04, 0xf, 3),
    (0x08, 0x7, 7),
    (0x0c, 0x3, 10),
    (0x10, 0xf, 12),
    (0x14, 0x7, 16),
    (0x18, 0x7, 19),
    (0x1c, 0x3f, 22),
    (0x20, 0xf, 28),
    (0x24, 0x7, 32),
    (0x28, 0x7f, 35),
    (0x2c, 0x1, 42),
    (0x30, 0x7f, 43),
    (0x34, 0x3, 50),
    (0x38, 0x7, 56),
    (0x3c, 0xf, 52),
];

/// `void WScriptFn_SetRiderConfigBits(sret, int count, Param* params)`
/// (0x3045b8): on the first call zeroes the static 16-word config block
/// (`0x4fbcc8`), copies it (0x44 bytes) to the stack, stores each parameter
/// into its word (a float when the kind table at `0x446650` says 2), packs the
/// words' low bits into one 64-bit word and passes it to
/// `WScriptMan_SetPendingRiderConfig`; returns the nil value in `sret`.
pub fn wscript_fn_set_rider_config_bits(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xe0);
        c.g[V0][0] = Addr(w746_gp(c, 0x271c)).lw(c);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S5, 0x80);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S3, 0xa0);
        c.g[A1][0] = 0x0050_0000;
        frame.save(c, S4, 0x90);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x70);
        if !initialised {
            c.cycles += 22;
            let block = (c.g[A1][0] as u32).wrapping_sub(0x4338);
            Addr(block).sw(c, 0);
            c.g[V0][0] = sx(block);
            c.g[V1][0] = 1;
            c.g[A0][0] = Addr(block.wrapping_add(0x40)).lbu(c);
            let flag = c.g[V1][0];
            Addr(w746_gp(c, 0x271c)).sw(c, flag);
            let byte = c.g[A0][0];
            Addr(block.wrapping_add(0x40)).sb(c, byte);
            for off in (4..=0x3cu32).step_by(4) {
                Addr(block.wrapping_add(off)).sw(c, 0);
            }
            Addr(sp(c)).sb(c, byte);
        }
        c.cycles += 8;
        c.g[S4][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x44;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4338));
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x003e_6574, 0x0030_465c)?;
        c.g[T8][0] = sx(sp(c).wrapping_add(0x60));
        let skip = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S1][0];
        if !skip {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6650));
            c.g[A3][0] = 2;
            w746_param_loop(c, S4, 0x0030_4678);
        }
        // Pack the config words (the generated body's 176 cycles).
        c.cycles += 176;
        let rec = c.g[S4][0] as u32;
        let masks = [
            A0, A1, A2, A3, T0, T1, T2, T3, T4, T5, T6, T7, S0, S1, A2, S2,
        ];
        let mut v = Addr(sp(c).wrapping_add(0x60)).ld(c);
        let mut last = 0u64;
        for (i, &(off, wm, sh)) in W746_RIDER_CONFIG_FIELDS.iter().enumerate() {
            let mask = !(wm << sh);
            c.g[masks[i]][0] = mask;
            last = (Addr(rec.wrapping_add(off)).lwu(c) & wm) << sh;
            v = (v & mask) | last;
        }
        c.g[V1][0] = last;
        c.g[S3][0] = u64::MAX >> 5;
        c.g[A0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
        c.g[A1][0] = c.g[T8][0];
        v &= c.g[S3][0];
        c.g[V0][0] = v;
        let packed = c.g[V0][0];
        Addr(sp(c).wrapping_add(0x60)).sd(c, packed);
        call_ptr(c, 0x0030_b520, 0x0030_4938)?;
        c.g[V1][0] = 0x004d_0000;
        c.g[V0][0] = c.g[S5][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(A0, c.g[A3][0] as u32);
        c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
        c.sdl(A0, (c.g[S5][0] as u32).wrapping_add(0x7));
        c.sdr(A0, c.g[S5][0] as u32);
        let sret = (c.g[S5][0] as u32).wrapping_add(0x8);
        let v = c.g[A1][0];
        Addr(sret).sw(c, v);
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptFn_SetObjectField24(sret, int count, Param* params)`
/// (0x304980): on the first call sets the static 12-byte block (`0x4fbd10`) to
/// `{-1, 1.0f, ...}`, copies it to the stack, stores each parameter into it
/// (the kind table at `gp+0xc78`), resolves the object id in word 0 (`-1`:
/// the WScriptMan's own instance at `+0x290`, else through the world cache's
/// section and data tables) and, when the object's node (`+0xc`) answers the
/// move-node virtual query, calls `0x355878(node, block)` (adds the fade).
/// Returns the nil value in `sret`.
pub fn wscript_fn_set_object_field24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(w746_gp(c, 0x2720)).lw(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialised {
            c.cycles += 12;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x42f0);
            c.g[V1][0] = sx(block);
            c.g[A1][0] = Addr(block.wrapping_add(0x8)).lbu(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = 1;
            Addr(block).sw(c, u32::MAX as u64);
            let byte = c.g[A1][0];
            Addr(block.wrapping_add(0x8)).sb(c, byte);
            c.swc1(0, block.wrapping_add(0x4));
            Addr(w746_gp(c, 0x2720)).sw(c, 1);
            Addr(sp(c)).sb(c, byte);
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 11;
        c.g[V0][0] = u64::MAX;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42f0));
        c.g[A2][0] = 0xc;
        Addr(sp(c).wrapping_add(0x10)).sw(c, u32::MAX as u64);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x0030_4a08)?;
        let skip = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !skip {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(w746_gp(c, 0xc78));
            c.g[A3][0] = 2;
            w746_param_loop(c, S1, 0x0030_4a20);
        }
        // Resolve the object id in word 0 to the object record `v0`.
        c.cycles += 4;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        let by_table = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(w746_gp(c, 0x16c8)).lw(c);
        if !by_table {
            c.cycles += 3;
            c.g[V0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let no_section = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_section {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // The object's node, if it is a move node: add the fade.
        c.cycles += 3;
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 7;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_4ae4)?;
            let no_move = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_move {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035_5878, 0x0030_4af4)?;
            }
        }
        c.cycles += 16;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lw(c);
        c.sdl(V0, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(V0, c.g[S2][0] as u32);
        let sret = (c.g[S2][0] as u32).wrapping_add(0x8);
        let v = c.g[V1][0];
        Addr(sret).sw(c, v);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The id lookup of a script block's object (`SetObjectField24`'s too): the
/// block's id word in `a2`, the world-cache pointer cell in `a1`. Returns with
/// `v0` the object record (0 when missing) and the cycles charged. Only the
/// table path: the caller handles the `-1` (instance) case.
pub(crate) fn w746_lookup_object(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let no_section = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    let mut found = false;
    if !no_section {
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        found = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
}

/// `void WScriptFn_CameraShake(sret, int count, Param* params)` (0x3050f0):
/// the block `{keyword, useObj, id, scale}` (defaults `0, 1, -1, 1.0f`, the
/// static at `0x4fbd40`, set up on the first call) is filled from the
/// parameters (kind table at `0x446690`). With keyword 0 it shakes the camera
/// of every local rider at its distance (times `scale`) from the object (the
/// object resolved from `id` when `useObj`), or, without an object, of the
/// rider whose AI index is the script's being. Returns the nil value in
/// `sret`. Has a resume twin: `c.pc` other than the entry goes to
/// `f_003050f0__entries`.
pub fn wscript_fn_camera_shake(c: &mut Cpu) {
    if c.pc != 0x0030_50f0 {
        return wscript_fn_camera_shake_resume(c);
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = Addr(w746_gp(c, 0x274c)).lw(c);
        frame.save(c, S2, 0xc0);
        frame.save(c, S1, 0xd0);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S7, 0x70);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0xe0);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S3, 0xb0);
        c.g[A2][0] = 0x0050_0000;
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x60);
        let initialised = c.g[V0][0] != 0;
        frame.save_fpr(c, 20, 0xf0);
        if !initialised {
            c.cycles += 14;
            let block = (c.g[A2][0] as u32).wrapping_sub(0x42c0);
            c.g[V1][0] = sx(block);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A1][0] = Addr(block.wrapping_add(0x10)).lbu(c);
            Addr(block).sw(c, 0);
            c.g[A0][0] = 1;
            let byte = c.g[A1][0];
            Addr(block.wrapping_add(0x10)).sb(c, byte);
            Addr(block.wrapping_add(0x8)).sw(c, u32::MAX as u64);
            c.swc1(0, block.wrapping_add(0xc));
            Addr(w746_gp(c, 0x274c)).sw(c, 1);
            Addr(block.wrapping_add(0x4)).sw(c, 1);
            Addr(sp(c)).sb(c, byte);
        }
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42c0));
        c.g[A2][0] = 0x14;
        Addr(sp(c).wrapping_add(0x18)).sw(c, u32::MAX as u64);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_518c)?;
        let skip = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !skip {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6690));
            c.g[A3][0] = 2;
            w746_param_loop(c, S0, 0x0030_51a8);
        }
        // The object (s5): none without `useObj`, the instance for id -1,
        // else through the world cache.
        c.cycles += 4;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[S5][0] = 0;
        let no_obj = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
        if !no_obj {
            c.cycles += 4;
            c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
            c.g[V0][0] = !c.g[A2][0];
            let by_table = c.g[V0][0] != 0;
            c.g[A1][0] = Addr(w746_gp(c, 0x16c8)).lw(c);
            if !by_table {
                c.cycles += 2;
                c.g[S5][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x290)).lw(c);
            } else {
                w746_lookup_object(c);
                c.cycles += 2;
                c.g[A0][0] = Addr(w746_gp(c, 0xce8)).lw(c);
                c.g[S5][0] = c.g[V0][0];
            }
        }
        c.cycles += 4;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[S3][0] = Addr(c.g[A0][0] as u32).lw(c);
        let keyword_set = c.g[A1][0] != 0;
        c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0xc));
        if keyword_set {
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(w746_gp(c, 0u32.wrapping_sub(0x848))).lw(c);
            c.g[S2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            let no_obj = c.g[S5][0] == 0;
            c.g[S4][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
            if !no_obj {
                w746_shake_by_distance(c)?;
            } else {
                w746_shake_being(c)?;
            }
        }
        c.cycles += 20;
        c.g[V0][0] = c.g[S7][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(A0, c.g[A3][0] as u32);
        c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x8)).lw(c);
        c.sdl(A0, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.sdr(A0, c.g[S7][0] as u32);
        let sret = (c.g[S7][0] as u32).wrapping_add(0x8);
        let v = c.g[A1][0];
        Addr(sret).sw(c, v);
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// Native resume of `wscript_fn_camera_shake` (converted from the `entries` body): any entry
/// other than 0x003050f0, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn wscript_fn_camera_shake_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003050f0 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x274c));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.g[S7][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A2][0] = 0x500000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    if t {
                        pc = 0x0030516c;
                        continue;
                    }
                    pc = 0x00305134;
                    continue;
                }
                0x00305134 => {
                    c.cycles += 14;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42c0));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.sw((c.g[A2][0] as u32).wrapping_sub(0x42c0), 0u64);
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A0][0] = 0x1;
                    c.sb((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x274c), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                    pc = 0x0030516c;
                    continue;
                }
                0x0030516c => {
                    c.cycles += 10;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42c0));
                    c.g[A2][0] = 0x14;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0030518c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e6574, 0x0030518c)?;
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x003051f0;
                        continue;
                    }
                    pc = 0x00305194;
                    continue;
                }
                0x00305194 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x440000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6690));
                    c.g[A3][0] = 0x2;
                    pc = 0x003051a8;
                    continue;
                }
                0x003051a8 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x003051e0;
                        continue;
                    }
                    pc = 0x003051c8;
                    continue;
                }
                0x003051c8 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x003051e0;
                        continue;
                    }
                    pc = 0x003051d0;
                    continue;
                }
                0x003051d0 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    let t = true;
                    c.swc1(0, c.g[A0][0] as u32);
                    if t {
                        pc = 0x003051e4;
                        continue;
                    }
                    pc = 0x003051e0;
                    continue;
                }
                0x003051e0 => {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    pc = 0x003051e4;
                    continue;
                }
                0x003051e4 => {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x003051a8) {
                            return Ok(());
                        }
                        pc = 0x003051a8;
                        continue;
                    }
                    pc = 0x003051f0;
                    continue;
                }
                0x003051f0 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S5][0] = 0u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    if t {
                        pc = 0x00305260;
                        continue;
                    }
                    pc = 0x00305200;
                    continue;
                }
                0x00305200 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = !(0u64 | c.g[A2][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                    if t {
                        pc = 0x00305218;
                        continue;
                    }
                    pc = 0x00305210;
                    continue;
                }
                0x00305210 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S5][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x290));
                    if t {
                        pc = 0x00305260;
                        continue;
                    }
                    pc = 0x00305218;
                    continue;
                }
                0x00305218 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if t {
                        pc = 0x00305254;
                        continue;
                    }
                    pc = 0x00305238;
                    continue;
                }
                0x00305238 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00305258;
                        continue;
                    }
                    pc = 0x00305254;
                    continue;
                }
                0x00305254 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00305258;
                    continue;
                }
                0x00305258 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[S5][0] = c.g[V0][0];
                    pc = 0x00305260;
                    continue;
                }
                0x00305260 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[S3][0] = c.lw(c.g[A0][0] as u32);
                    let t = c.g[A1][0] != 0u64;
                    c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x00305424;
                        continue;
                    }
                    pc = 0x00305270;
                    continue;
                }
                0x00305270 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    let t = c.g[S5][0] == 0u64;
                    c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    if t {
                        pc = 0x00305390;
                        continue;
                    }
                    pc = 0x00305284;
                    continue;
                }
                0x00305284 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x00305424;
                        continue;
                    }
                    pc = 0x00305290;
                    continue;
                }
                0x00305290 => {
                    c.cycles += 2;
                    c.g[S6][0] = sx((c.g[GP][0] as u32).wrapping_add(0xae8));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    pc = 0x00305298;
                    continue;
                }
                0x00305298 => {
                    c.cycles += 14;
                    c.g[A0][0] = sx((c.g[S3][0] as u32) << 2);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003052c4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x003052c4)?;
                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                    let t = c.g[A1][0] == 0u64;
                    c.g[S0][0] = c.g[V0][0];
                    if t {
                        pc = 0x003052ec;
                        continue;
                    }
                    pc = 0x003052d0;
                    continue;
                }
                0x003052d0 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003052e4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003052e4)?;
                    let t = true;
                    c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x003052f4;
                        continue;
                    }
                    pc = 0x003052ec;
                    continue;
                }
                0x003052ec => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
                    c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x30));
                    pc = 0x003052f4;
                    continue;
                }
                0x003052f4 => {
                    c.cycles += 24;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lqc2(3, c.g[S0][0] as u32);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 13);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x87c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x870));
                    if t {
                        pc = 0x00305378;
                        continue;
                    }
                    pc = 0x00305354;
                    continue;
                }
                0x00305354 => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.mul_s(13, 13, 20);
                    c.g[A1][0] = c.lw(c.g[S6][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.g[RA][0] = sx(0x00305378);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0015e360, 0x00305378)?;
                    pc = 0x00305378;
                    continue;
                }
                0x00305378 => {
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        if back_edge_unwinds(c, 0x00305298) {
                            return Ok(());
                        }
                        pc = 0x00305298;
                        continue;
                    }
                    pc = 0x00305388;
                    continue;
                }
                0x00305388 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = 0x4d0000;
                    if t {
                        pc = 0x00305428;
                        continue;
                    }
                    pc = 0x00305390;
                    continue;
                }
                0x00305390 => {
                    c.cycles += 2;
                    let t = (c.g[S4][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x00305424;
                        continue;
                    }
                    pc = 0x00305398;
                    continue;
                }
                0x00305398 => {
                    c.cycles += 2;
                    c.g[S5][0] = sx((c.g[GP][0] as u32).wrapping_add(0xae8));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    pc = 0x003053a0;
                    continue;
                }
                0x003053a0 => {
                    c.cycles += 12;
                    c.g[S1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003053c8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x003053c8)?;
                    let t = c.g[V0][0] != c.g[S3][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00305418;
                        continue;
                    }
                    pc = 0x003053d0;
                    continue;
                }
                0x003053d0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x87c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x870));
                    if t {
                        pc = 0x00305424;
                        continue;
                    }
                    pc = 0x003053ec;
                    continue;
                }
                0x003053ec => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A1][0] = c.lw(c.g[S5][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.mtc1(0, 13);
                    c.g[RA][0] = sx(0x00305410);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0015e360, 0x00305410)?;
                    let t = true;
                    c.g[V1][0] = 0x4d0000;
                    if t {
                        pc = 0x00305428;
                        continue;
                    }
                    pc = 0x00305418;
                    continue;
                }
                0x00305418 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                        if back_edge_unwinds(c, 0x003053a0) {
                            return Ok(());
                        }
                        pc = 0x003053a0;
                        continue;
                    }
                    pc = 0x00305424;
                    continue;
                }
                0x00305424 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x4d0000;
                    pc = 0x00305428;
                    continue;
                }
                0x00305428 => {
                    c.cycles += 20;
                    c.g[V0][0] = c.g[S7][0];
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
                    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[A3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.sdl(4, (c.g[S7][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[S7][0] as u32);
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003050f0, 0x00305478);
                    return Ok(());
                }
            }
        }
    });
}

/// CameraShake with an object (`s5`): for each player `s3 = 0..s4` shake the
/// local rider's camera by its distance from the object. Leaves `v1` as the
/// generated body does (`0x4d0000` at the end).
pub(crate) fn w746_shake_by_distance(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S4][0] as i64)) as u64;
    let none = c.g[V0][0] == 0;
    c.g[S3][0] = 0;
    if none {
        c.cycles += 1;
        c.g[V1][0] = 0x004d_0000;
        return Ok(());
    }
    c.cycles += 2;
    c.g[S6][0] = sx(w746_gp(c, 0xae8));
    c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
    loop {
        c.cycles += 14;
        c.g[A0][0] = sx((c.g[S3][0] as u32) << 2);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0030_52c4)?;
        c.g[A1][0] = Addr((c.g[S5][0] as u32).wrapping_add(0xc)).lw(c);
        let no_node = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if no_node {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
            c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x30));
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc0)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_52e4)?;
            c.lq(2, (c.g[V0][0] as u32).wrapping_add(0x30));
        }
        // The distance (VU0): |rider position - object translation|.
        c.cycles += 24;
        c.sq(2, sp(c).wrapping_add(0x40));
        c.lqc2(3, c.g[S0][0] as u32);
        c.lqc2(4, sp(c).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x50));
        c.lq(2, sp(c).wrapping_add(0x50));
        c.sq(2, sp(c).wrapping_add(0x30));
        c.lqc2(3, sp(c).wrapping_add(0x30));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 13);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x87c)).lw(c);
        let not_local = c.g[V0][0] == 0;
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x870)).lw(c);
        if !not_local {
            c.cycles += 9;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.mul_s(13, 13, 20);
            c.g[A1][0] = Addr(c.g[S6][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x0015_e360, 0x0030_5378)?;
        }
        c.cycles += 4;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_5298);
            }
        } else {
            c.cycles += 2;
            c.g[V1][0] = 0x004d_0000;
            return Ok(());
        }
    }
}

/// CameraShake without an object: shake the camera of the local rider whose
/// AI index (virtual call at `+0x3c`) is the script's being (`s3`), once.
pub(crate) fn w746_shake_being(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let none = (c.g[S4][0] as i64) <= 0;
    c.g[S0][0] = 0;
    if none {
        c.cycles += 1;
        c.g[V1][0] = 0x004d_0000;
        return Ok(());
    }
    c.cycles += 2;
    c.g[S5][0] = sx(w746_gp(c, 0xae8));
    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
    loop {
        c.cycles += 12;
        c.g[S1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0030_53c8)?;
        let other = c.g[V0][0] != c.g[S3][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        if !other {
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x87c)).lw(c);
            let not_local = c.g[V0][0] == 0;
            c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x870)).lw(c);
            if not_local {
                c.cycles += 1;
                c.g[V1][0] = 0x004d_0000;
                return Ok(());
            }
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[A1][0] = Addr(c.g[S5][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.mtc1(0, 13);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x0015_e360, 0x0030_5410)?;
            c.g[V1][0] = 0x004d_0000;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_53a0);
            }
        } else {
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            return Ok(());
        }
    }
}

/// `gp` offset of the one-time-init flag of the emitter record template.
pub(crate) const NODE_EMITTER_INIT_FLAG: u32 = 0x278c;

/// The emitter record template (static local) and its type table.
#[cfg(test)]
pub(crate) const NODE_EMITTER_TEMPLATE: u32 = 0x004f_be58;

pub(crate) const NODE_EMITTER_TYPES: u32 = 0x0044_67a0;

/// Word offset (bytes) of the avalanche id inside the record.
pub(crate) const NODE_EMITTER_ID: u32 = 0xd8;

/// `WScriptFn_SetAvalancheNodeEmitter` (0x305f40): script builder. One-time
/// init of the default emitter record, copy of it (0xe0 bytes) to the stack,
/// the `(idx, value, type)` parameter loop (floats converted where the type
/// table asks for it), then, when the record names an avalanche (id != -1),
/// looks up its object through the section table and calls
/// `Avalanche_SetNodeEmitter(obj, record)`. Returns the nil value through the
/// hidden slot in `a0`.
pub(crate) fn set_avalanche_node_emitter_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x140);
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = Addr(gp.wrapping_add(NODE_EMITTER_INIT_FLAG)).lw(c);
    frame.save(c, S1, 0x120);
    c.g[A3][0] = 0x0050_0000;
    frame.save(c, S2, 0x110);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x100);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x130);
    c.g[S3][0] = c.g[A0][0];
    let initialised = c.g[V0][0] != 0;
    frame.save_ra(c, 0xf0);

    if !initialised {
        node_emitter_init(c);
    }

    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0xe0;
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x41a8));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0xe8)).sw(c, v0);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, 0x0030_60b0)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    if (count as i64) > 0 {
        node_emitter_fill(c, count);
    }

    // The avalanche named by the record's id, if any.
    c.cycles += 4;
    let s0 = c.g[S0][0] as u32;
    c.g[A2][0] = Addr(s0.wrapping_add(NODE_EMITTER_ID)).lw(c);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let none = c.g[V0][0] == 0;
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if !none {
        c.cycles += 8;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let found = c.g[V1][0] != 0;
        c.g[A0][0] = 0;
        if found {
            c.cycles += 8;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if nonzero {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
            }
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x002d_9538, 0x0030_616c)?;
    }

    c.cycles += 16;
    c.g[V1][0] = 0x004d_0000;
    c.g[V0][0] = c.g[S3][0];
    let dst = c.g[S3][0] as u32;
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
    frame.restore(c, S0, 0x130);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S2, 0x110);
    frame.restore(c, S3, 0x100);
    frame.restore_ra(c, 0xf0);
    frame.close(c);
    Ok(())
}

/// The one-time init of the default emitter record (the template at
/// `0x4fbe58`): flags, floats and the id (-1), then the init flag.
pub(crate) fn node_emitter_init(c: &mut Cpu) {
    let gp = c.g[GP][0] as u32;
    c.cycles += 72;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 1);
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x41a8));
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(1, 2);
    c.g[A0][0] = 1;
    c.mtc1(0, 0);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.g[A2][0] = 0x10;
    let base = c.g[V0][0] as u32;
    c.swc1(1, base.wrapping_add(0x8));
    c.g[V1][0] |= 0xffff;
    c.swc1(2, base.wrapping_add(0x10));
    let a0 = c.g[A0][0];
    Addr((c.g[A3][0] as u32).wrapping_sub(0x41a8)).sw(c, a0);
    Addr(base.wrapping_add(0x4)).sw(c, 0);
    c.swc1(3, base.wrapping_add(0xc));
    let mut off = 0x14u32;
    while off <= 0x80 {
        c.swc1(0, base.wrapping_add(off));
        off += 4;
    }
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 1);
    c.swc1(3, base.wrapping_add(0x84));
    c.g[A1][0] = Addr(base.wrapping_add(0xdc)).lbu(c);
    c.swc1(3, base.wrapping_add(0xb4));
    let a2 = c.g[A2][0];
    Addr(base.wrapping_add(0xc4)).sw(c, a2);
    c.swc1(0, base.wrapping_add(0xcc));
    c.swc1(1, base.wrapping_add(0xd4));
    let v1 = c.g[V1][0];
    Addr(base.wrapping_add(0xd8)).sw(c, v1);
    Addr(gp.wrapping_add(NODE_EMITTER_INIT_FLAG)).sw(c, a0);
    for off in [0x88u32, 0x8c, 0x90] {
        c.swc1(0, base.wrapping_add(off));
    }
    for off in [0x94u32, 0x98, 0x9c, 0xa0, 0xa4] {
        c.swc1(3, base.wrapping_add(off));
    }
    for off in [0xa8u32, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
        c.swc1(0, base.wrapping_add(off));
    }
    Addr(base.wrapping_add(0xc8)).sw(c, 0);
    Addr(base.wrapping_add(0xd0)).sw(c, a0);
    let a1 = c.g[A1][0];
    Addr(sp(c)).sb(c, a1);
}

/// The parameter loop: `count` (> 0) records of 16 bytes at `A2`; each stores
/// its value into `record[idx]`, as a float when the type table
/// (`0x4467a0`) asks for type 2 and the record's own type differs.
pub(crate) fn node_emitter_fill(c: &mut Cpu, count: u64) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(NODE_EMITTER_TYPES);
    c.g[A3][0] = 2;
    let buf = c.g[S0][0] as u32;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(buf));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_60c8);
        }
    }
}

/// `WScriptFn_SetAvalancheNodeEmitter` (0x305f40): see
/// [`set_avalanche_node_emitter_body`].
pub fn wscript_set_avalanche_node_emitter(c: &mut Cpu) {
    guest(c, set_avalanche_node_emitter_body);
}

/// `WorldObjFactory_AddParticleData` (0x002fe840): adds one particle object of a
/// type from the entry table: copies the static record, fills the slots, looks up
/// the type through the index chain, probes the source and the viewer distances,
/// and hands the object to the modifier list / particle constructor or deletes it.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn wof_add_particle_data(c: &mut Cpu) {
    if c.pc != 0x002fe840 {
        return wof_add_particle_data_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002fe840;
        loop {
            match pc {
                0x002fe840 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2538));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S0][0] = 0x500000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    if t {
                        pc = 0x002fe9a0;
                        continue;
                    }
                    pc = 0x002fe880;
                    continue;
                }
                0x002fe880 => {
                    c.cycles += 72;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 1);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x47a8));
                    c.g[AT][0] = 0x40800000;
                    c.mtc1(1, 2);
                    c.g[A0][0] = 0x1;
                    c.mtc1(0, 0);
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[A2][0] = 0x10;
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x47a8), c.g[A0][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x28));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x44));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x50));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x54));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x58));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x60));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x68));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x6c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x70));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x78));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x80));
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 1);
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xdc));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xcc));
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2538), c.g[A0][0]);
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x88));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x90));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x94));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x98));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x9c));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa0));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa4));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xa8));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xac));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb8));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xbc));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), 0u64);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                    pc = 0x002fe9a0;
                    continue;
                }
                0x002fe9a0 => {
                    c.cycles += 17;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = 0xe0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68c0));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002fe9bc);
                    c.g[S6][0] = 0x4d0000;
                    Callee {
                        addr: 0x00317d70,
                        f: ee::s10::EA_MemAllocNamed,
                    }
                    .call(c, 0x002fe9bc)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A2][0] = 0xe0;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x47a8));
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002fe9dc);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    call_ptr(c, 0x003e6574, 0x002fe9dc)?;
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x002fea40;
                        continue;
                    }
                    pc = 0x002fe9e4;
                    continue;
                }
                0x002fe9e4 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x440000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62a0));
                    c.g[A3][0] = 0x2;
                    pc = 0x002fe9f8;
                    continue;
                }
                0x002fe9f8 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x002fea30;
                        continue;
                    }
                    pc = 0x002fea18;
                    continue;
                }
                0x002fea18 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x002fea30;
                        continue;
                    }
                    pc = 0x002fea20;
                    continue;
                }
                0x002fea20 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    let t = true;
                    c.swc1(0, c.g[A0][0] as u32);
                    if t {
                        pc = 0x002fea34;
                        continue;
                    }
                    pc = 0x002fea30;
                    continue;
                }
                0x002fea30 => {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    pc = 0x002fea34;
                    continue;
                }
                0x002fea34 => {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x002fe9f8) {
                            return Ok(());
                        }
                        pc = 0x002fe9f8;
                        continue;
                    }
                    pc = 0x002fea40;
                    continue;
                }
                0x002fea40 => {
                    c.cycles += 6;
                    c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x14));
                    c.c_lt_s(1, 0);

                    if c.fcc() {
                        c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0x1c));
                        pc = 0x002fea58;
                        continue;
                    }
                    pc = 0x002fea58;
                    continue;
                }
                0x002fea58 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd8));
                    c.g[V0][0] = !(0u64 | c.g[A2][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                    if t {
                        pc = 0x002fea74;
                        continue;
                    }
                    pc = 0x002fea68;
                    continue;
                }
                0x002fea68 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    let t = true;
                    c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    if t {
                        pc = 0x002feac4;
                        continue;
                    }
                    pc = 0x002fea74;
                    continue;
                }
                0x002fea74 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                        pc = 0x002fea9c;
                        continue;
                    }
                    pc = 0x002fea94;
                    continue;
                }
                0x002fea94 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x002feac0;
                        continue;
                    }
                    pc = 0x002fea9c;
                    continue;
                }
                0x002fea9c => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x002feabc;
                        continue;
                    }
                    pc = 0x002feab8;
                    continue;
                }
                0x002feab8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002feabc;
                    continue;
                }
                0x002feabc => {
                    c.cycles += 1;
                    c.g[A2][0] = c.g[V0][0];
                    pc = 0x002feac0;
                    continue;
                }
                0x002feac0 => {
                    c.cycles += 1;
                    c.g[S4][0] = c.g[A2][0];
                    pc = 0x002feac4;
                    continue;
                }
                0x002feac4 => {
                    c.cycles += 3;
                    c.g[S2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));

                    if c.g[S2][0] == 0u64 {
                        c.lq(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                        pc = 0x002fec90;
                        continue;
                    }
                    pc = 0x002fead0;
                    continue;
                }
                0x002fead0 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feae4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002feae4)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x002fee44;
                        continue;
                    }
                    pc = 0x002feaec;
                    continue;
                }
                0x002feaec => {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feb00);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002feb00)?;
                    c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x002feb24;
                        continue;
                    }
                    pc = 0x002feb1c;
                    continue;
                }
                0x002feb1c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002fec78;
                        continue;
                    }
                    pc = 0x002feb24;
                    continue;
                }
                0x002feb24 => {
                    c.cycles += 6;
                    c.g[S0][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x002febcc;
                        continue;
                    }
                    pc = 0x002feb3c;
                    continue;
                }
                0x002feb3c => {
                    c.cycles += 1;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x337c));
                    pc = 0x002feb40;
                    continue;
                }
                0x002feb40 => {
                    c.cycles += 32;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feb70);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002feb70)?;
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feb1c) {
                            return Ok(());
                        }
                        pc = 0x002feb1c;
                        continue;
                    }
                    pc = 0x002febc0;
                    continue;
                }
                0x002febc0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002feb40) {
                            return Ok(());
                        }
                        pc = 0x002feb40;
                        continue;
                    }
                    pc = 0x002febcc;
                    continue;
                }
                0x002febcc => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x002fec74;
                        continue;
                    }
                    pc = 0x002febe0;
                    continue;
                }
                0x002febe0 => {
                    c.cycles += 2;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3378));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x002febe8;
                    continue;
                }
                0x002febe8 => {
                    c.cycles += 32;
                    c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002fec18);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002fec18)?;
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feb1c) {
                            return Ok(());
                        }
                        pc = 0x002feb1c;
                        continue;
                    }
                    pc = 0x002fec68;
                    continue;
                }
                0x002fec68 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002febe8) {
                            return Ok(());
                        }
                        pc = 0x002febe8;
                        continue;
                    }
                    pc = 0x002fec74;
                    continue;
                }
                0x002fec74 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002fec78;
                    continue;
                }
                0x002fec78 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x002fee44;
                        continue;
                    }
                    pc = 0x002fec80;
                    continue;
                }
                0x002fec80 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002fec88);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x00355cb0, 0x002fec88)?;
                    let t = true;

                    if t {
                        pc = 0x002fee4c;
                        continue;
                    }
                    pc = 0x002fec90;
                    continue;
                }
                0x002fec90 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.sq(2, c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V1][0] = c.g[V1][0] ^ 0x2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x002fecb0;
                        continue;
                    }
                    pc = 0x002feca8;
                    continue;
                }
                0x002feca8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002fee08;
                        continue;
                    }
                    pc = 0x002fecb0;
                    continue;
                }
                0x002fecb0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S0][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x002fed5c;
                        continue;
                    }
                    pc = 0x002feccc;
                    continue;
                }
                0x002feccc => {
                    c.cycles += 1;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3374));
                    pc = 0x002fecd0;
                    continue;
                }
                0x002fecd0 => {
                    c.cycles += 32;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002fed00);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002fed00)?;
                    c.lqc2(3, c.g[S2][0] as u32);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feca8) {
                            return Ok(());
                        }
                        pc = 0x002feca8;
                        continue;
                    }
                    pc = 0x002fed50;
                    continue;
                }
                0x002fed50 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002fecd0) {
                            return Ok(());
                        }
                        pc = 0x002fecd0;
                        continue;
                    }
                    pc = 0x002fed5c;
                    continue;
                }
                0x002fed5c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x002fee04;
                        continue;
                    }
                    pc = 0x002fed70;
                    continue;
                }
                0x002fed70 => {
                    c.cycles += 2;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3370));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x002fed78;
                    continue;
                }
                0x002fed78 => {
                    c.cycles += 32;
                    c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feda8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002feda8)?;
                    c.lqc2(3, c.g[S2][0] as u32);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feca8) {
                            return Ok(());
                        }
                        pc = 0x002feca8;
                        continue;
                    }
                    pc = 0x002fedf8;
                    continue;
                }
                0x002fedf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002fed78) {
                            return Ok(());
                        }
                        pc = 0x002fed78;
                        continue;
                    }
                    pc = 0x002fee04;
                    continue;
                }
                0x002fee04 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002fee08;
                    continue;
                }
                0x002fee08 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x490000;
                    if t {
                        pc = 0x002fee44;
                        continue;
                    }
                    pc = 0x002fee10;
                    continue;
                }
                0x002fee10 => {
                    c.cycles += 13;
                    c.g[A0][0] = 0x2c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68d0));
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x002fee24);
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x00317d70,
                        f: ee::s10::EA_MemAllocNamed,
                    }
                    .call(c, 0x002fee24)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002fee3c);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x003578a8, 0x002fee3c)?;
                    let t = true;

                    if t {
                        pc = 0x002fee4c;
                        continue;
                    }
                    pc = 0x002fee44;
                    continue;
                }
                0x002fee44 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x002fee4c);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00317e50,
                        f: ee::s10::__builtin_delete,
                    }
                    .call(c, 0x002fee4c)?;
                    pc = 0x002fee4c;
                    continue;
                }
                0x002fee4c => {
                    c.cycles += 19;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6f68));
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[A1][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S5][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[S5][0] as u32);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S5][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002fe840, 0x002fee98);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `wof_add_particle_data` (converted from the `generated` body): any entry
/// other than 0x002fe840, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn wof_add_particle_data_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x002fe840 => {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2538));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S0][0] = 0x500000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    if t {
                        pc = 0x002fe9a0;
                        continue;
                    }
                    pc = 0x002fe880;
                    continue;
                }
                0x002fe880 => {
                    c.cycles += 72;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 1);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x47a8));
                    c.g[AT][0] = 0x40800000;
                    c.mtc1(1, 2);
                    c.g[A0][0] = 0x1;
                    c.mtc1(0, 0);
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[A2][0] = 0x10;
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x47a8), c.g[A0][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x28));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x44));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x50));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x54));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x58));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x60));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x68));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x6c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x70));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x78));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x80));
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 1);
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xdc));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xcc));
                    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2538), c.g[A0][0]);
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x88));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x90));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x94));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x98));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x9c));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa0));
                    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa4));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xa8));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xac));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb0));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb8));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xbc));
                    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), 0u64);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                    pc = 0x002fe9a0;
                    continue;
                }
                0x002fe9a0 => {
                    c.cycles += 17;
                    c.g[A1][0] = 0x490000;
                    c.g[A0][0] = 0xe0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68c0));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002fe9bc);
                    c.g[S6][0] = 0x4d0000;
                    Callee {
                        addr: 0x00317d70,
                        f: ee::s10::EA_MemAllocNamed,
                    }
                    .call(c, 0x002fe9bc)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A2][0] = 0xe0;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x47a8));
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002fe9dc);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                    call_ptr(c, 0x003e6574, 0x002fe9dc)?;
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x002fea40;
                        continue;
                    }
                    pc = 0x002fe9e4;
                    continue;
                }
                0x002fe9e4 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x440000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62a0));
                    c.g[A3][0] = 0x2;
                    pc = 0x002fe9f8;
                    continue;
                }
                0x002fe9f8 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x002fea30;
                        continue;
                    }
                    pc = 0x002fea18;
                    continue;
                }
                0x002fea18 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x002fea30;
                        continue;
                    }
                    pc = 0x002fea20;
                    continue;
                }
                0x002fea20 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    let t = true;
                    c.swc1(0, c.g[A0][0] as u32);
                    if t {
                        pc = 0x002fea34;
                        continue;
                    }
                    pc = 0x002fea30;
                    continue;
                }
                0x002fea30 => {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    pc = 0x002fea34;
                    continue;
                }
                0x002fea34 => {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x002fe9f8) {
                            return Ok(());
                        }
                        pc = 0x002fe9f8;
                        continue;
                    }
                    pc = 0x002fea40;
                    continue;
                }
                0x002fea40 => {
                    c.cycles += 6;
                    c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x14));
                    c.c_lt_s(1, 0);

                    if c.fcc() {
                        c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0x1c));
                        pc = 0x002fea58;
                        continue;
                    }
                    pc = 0x002fea58;
                    continue;
                }
                0x002fea58 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd8));
                    c.g[V0][0] = !(0u64 | c.g[A2][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                    if t {
                        pc = 0x002fea74;
                        continue;
                    }
                    pc = 0x002fea68;
                    continue;
                }
                0x002fea68 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    let t = true;
                    c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    if t {
                        pc = 0x002feac4;
                        continue;
                    }
                    pc = 0x002fea74;
                    continue;
                }
                0x002fea74 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                        pc = 0x002fea9c;
                        continue;
                    }
                    pc = 0x002fea94;
                    continue;
                }
                0x002fea94 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x002feac0;
                        continue;
                    }
                    pc = 0x002fea9c;
                    continue;
                }
                0x002fea9c => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x002feabc;
                        continue;
                    }
                    pc = 0x002feab8;
                    continue;
                }
                0x002feab8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002feabc;
                    continue;
                }
                0x002feabc => {
                    c.cycles += 1;
                    c.g[A2][0] = c.g[V0][0];
                    pc = 0x002feac0;
                    continue;
                }
                0x002feac0 => {
                    c.cycles += 1;
                    c.g[S4][0] = c.g[A2][0];
                    pc = 0x002feac4;
                    continue;
                }
                0x002feac4 => {
                    c.cycles += 3;
                    c.g[S2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));

                    if c.g[S2][0] == 0u64 {
                        c.lq(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                        pc = 0x002fec90;
                        continue;
                    }
                    pc = 0x002fead0;
                    continue;
                }
                0x002fead0 => {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feae4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002feae4)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x002fee44;
                        continue;
                    }
                    pc = 0x002feaec;
                    continue;
                }
                0x002feaec => {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feb00);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x002feb00)?;
                    c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x002feb24;
                        continue;
                    }
                    pc = 0x002feb1c;
                    continue;
                }
                0x002feb1c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002fec78;
                        continue;
                    }
                    pc = 0x002feb24;
                    continue;
                }
                0x002feb24 => {
                    c.cycles += 6;
                    c.g[S0][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x002febcc;
                        continue;
                    }
                    pc = 0x002feb3c;
                    continue;
                }
                0x002feb3c => {
                    c.cycles += 1;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x337c));
                    pc = 0x002feb40;
                    continue;
                }
                0x002feb40 => {
                    c.cycles += 32;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feb70);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002feb70)?;
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feb1c) {
                            return Ok(());
                        }
                        pc = 0x002feb1c;
                        continue;
                    }
                    pc = 0x002febc0;
                    continue;
                }
                0x002febc0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002feb40) {
                            return Ok(());
                        }
                        pc = 0x002feb40;
                        continue;
                    }
                    pc = 0x002febcc;
                    continue;
                }
                0x002febcc => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x002fec74;
                        continue;
                    }
                    pc = 0x002febe0;
                    continue;
                }
                0x002febe0 => {
                    c.cycles += 2;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3378));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x002febe8;
                    continue;
                }
                0x002febe8 => {
                    c.cycles += 32;
                    c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002fec18);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002fec18)?;
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feb1c) {
                            return Ok(());
                        }
                        pc = 0x002feb1c;
                        continue;
                    }
                    pc = 0x002fec68;
                    continue;
                }
                0x002fec68 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002febe8) {
                            return Ok(());
                        }
                        pc = 0x002febe8;
                        continue;
                    }
                    pc = 0x002fec74;
                    continue;
                }
                0x002fec74 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002fec78;
                    continue;
                }
                0x002fec78 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x002fee44;
                        continue;
                    }
                    pc = 0x002fec80;
                    continue;
                }
                0x002fec80 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002fec88);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x00355cb0, 0x002fec88)?;
                    let t = true;

                    if t {
                        pc = 0x002fee4c;
                        continue;
                    }
                    pc = 0x002fec90;
                    continue;
                }
                0x002fec90 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.sq(2, c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V1][0] = c.g[V1][0] ^ 0x2;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S2][0] = c.g[SP][0];
                    if t {
                        pc = 0x002fecb0;
                        continue;
                    }
                    pc = 0x002feca8;
                    continue;
                }
                0x002feca8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x002fee08;
                        continue;
                    }
                    pc = 0x002fecb0;
                    continue;
                }
                0x002fecb0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S0][0] = 0u64;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        pc = 0x002fed5c;
                        continue;
                    }
                    pc = 0x002feccc;
                    continue;
                }
                0x002feccc => {
                    c.cycles += 1;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3374));
                    pc = 0x002fecd0;
                    continue;
                }
                0x002fecd0 => {
                    c.cycles += 32;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002fed00);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002fed00)?;
                    c.lqc2(3, c.g[S2][0] as u32);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feca8) {
                            return Ok(());
                        }
                        pc = 0x002feca8;
                        continue;
                    }
                    pc = 0x002fed50;
                    continue;
                }
                0x002fed50 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002fecd0) {
                            return Ok(());
                        }
                        pc = 0x002fecd0;
                        continue;
                    }
                    pc = 0x002fed5c;
                    continue;
                }
                0x002fed5c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x002fee04;
                        continue;
                    }
                    pc = 0x002fed70;
                    continue;
                }
                0x002fed70 => {
                    c.cycles += 2;
                    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3370));
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x002fed78;
                    continue;
                }
                0x002fed78 => {
                    c.cycles += 32;
                    c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002feda8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x002feda8)?;
                    c.lqc2(3, c.g[S2][0] as u32);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 20);
                    let t = c.fcc();
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x002feca8) {
                            return Ok(());
                        }
                        pc = 0x002feca8;
                        continue;
                    }
                    pc = 0x002fedf8;
                    continue;
                }
                0x002fedf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        if back_edge_unwinds(c, 0x002fed78) {
                            return Ok(());
                        }
                        pc = 0x002fed78;
                        continue;
                    }
                    pc = 0x002fee04;
                    continue;
                }
                0x002fee04 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x002fee08;
                    continue;
                }
                0x002fee08 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x490000;
                    if t {
                        pc = 0x002fee44;
                        continue;
                    }
                    pc = 0x002fee10;
                    continue;
                }
                0x002fee10 => {
                    c.cycles += 13;
                    c.g[A0][0] = 0x2c;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68d0));
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x002fee24);
                    c.g[A3][0] = 0u64;
                    Callee {
                        addr: 0x00317d70,
                        f: ee::s10::EA_MemAllocNamed,
                    }
                    .call(c, 0x002fee24)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002fee3c);
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x003578a8, 0x002fee3c)?;
                    let t = true;

                    if t {
                        pc = 0x002fee4c;
                        continue;
                    }
                    pc = 0x002fee44;
                    continue;
                }
                0x002fee44 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x002fee4c);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00317e50,
                        f: ee::s10::__builtin_delete,
                    }
                    .call(c, 0x002fee4c)?;
                    pc = 0x002fee4c;
                    continue;
                }
                0x002fee4c => {
                    c.cycles += 19;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6f68));
                    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[A1][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S5][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[S5][0] as u32);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S5][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002fe840, 0x002fee98);
                    return Ok(());
                }
            }
        }
    });
}

/// `cLunoVar AddMultiSplineFunc(int numArgs, cLunoCArg* params)` (0x2fe0c0,
/// the WScript function `AddMultiSplineModifier`): on its first call fills
/// the static default argument block (`D_E0C0`: instance id -1, target -1,
/// mode 3, 20.0 speed, unit scale, trailing flags), then copies the 13-word
/// block to the stack, overwrites the entries the script passed
/// (`params[i] = {index, value, _, type}`; a float-typed slot given an int
/// is converted), looks the target instance up (id -1: the script manager's
/// own instance; else through the world cache's section and tag tables) and,
/// when the second argument is not -1 and the instance's active node is a
/// move node (`isMoveNode` virtual), calls `cMoveNode::addSpline(node, buf)`.
/// Returns the nil `cLunoVar` (copied through `a0`'s hidden return pointer).
pub fn wscript_add_multi_spline_modifier(c: &mut Cpu) {
    guest(c, |c| {
        'b_2fe15c: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2528));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[T0][0] = 0x500000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S2][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S3][0] = c.g[A0][0];
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
            if t {
                break 'b_2fe15c;
            } else {
                c.cycles += 27;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4858));
                c.g[V1][0] = 0xffffffffffff0000;
                c.mtc1(0, 0);
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = 0x3;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 2);
                c.g[A1][0] = 0x2;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.g[A2][0] = 0x1;
                c.sw((c.g[T0][0] as u32).wrapping_sub(0x4858), c.g[V1][0]);
                c.sb((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), c.g[A1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2528), c.g[A2][0]);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                c.sb(c.g[SP][0] as u32, c.g[A3][0]);
                break 'b_2fe15c;
            }
        }
        'b_2fe1e0: {
            c.cycles += 11;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            c.g[A2][0] = 0x34;
            c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4858));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, 0x003e6574, 0x002fe180)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = c.g[S2][0];
            if t {
                break 'b_2fe1e0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S0][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6208));
                c.g[A3][0] = 0x2;
                'l_2fe198: loop {
                    'b_2fe1d4: {
                        'b_2fe1d0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                break 'b_2fe1d0;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != c.g[A3][0] {
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                    break 'b_2fe1d0;
                                } else {
                                    c.cycles += 4;
                                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                                    c.cvt_s_w(0, 0);
                                    c.swc1(0, c.g[A0][0] as u32);
                                    break 'b_2fe1d4;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        break 'b_2fe1d4;
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002fe198);
                        }
                        continue 'l_2fe198;
                    } else {
                        break 'b_2fe1e0;
                    }
                }
            }
        }
        'b_2fe240: {
            'b_2fe1fc: {
                c.cycles += 4;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t {
                    break 'b_2fe1fc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                    break 'b_2fe240;
                }
            }
            'b_2fe23c: {
                'b_2fe238: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                    if t {
                        break 'b_2fe238;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if t {
                            break 'b_2fe23c;
                        } else {
                            break 'b_2fe238;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_2fe23c;
            }
            c.cycles += 1;
            c.g[V1][0] = c.g[V0][0];
            break 'b_2fe240;
        }
        'b_2fe284: {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = 0x4d0000;
                break 'b_2fe284;
            } else {
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[S0][0] == 0u64;
                c.g[V1][0] = 0x4d0000;
                if t {
                    break 'b_2fe284;
                } else {
                    'b_2fe280: {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002fe270)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2fe280;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00355b30, 0x002fe280)?;
                            break 'b_2fe280;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = 0x4d0000;
                    break 'b_2fe284;
                }
            }
        }
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// Offsets of `defaults` (the guarded static record at `0x4fbf68`) that hold 0.0.
pub(crate) const MPS_ZERO_F: &[u32] = &[
    0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50,
    0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x88, 0x8c, 0x90, 0xa8,
    0xac, 0xb0, 0xb8, 0xbc, 0xc0, 0xcc,
];

/// Offsets of the static record that hold 1.0.
pub(crate) const MPS_ONE_F: &[u32] = &[0xc, 0x84, 0x94, 0x98, 0x9c, 0xa0, 0xa4, 0xb4];

/// `WScriptFn_SpawnMultiParticle(Nil* result, int n, Ent* params)` (0x306a90):
/// script function that spawns a multi-particle. On its first call it fills the
/// default particle record (static at 0x4fbf68, guard word at `gp+0x27c4`); it
/// then copies that record (228 bytes) onto the stack, applies the `n`
/// 16-byte parameter entries (`idx`, `val`, `type` at +0xc; an int value whose
/// declared type in the table at 0x446890 is float (2) is converted), calls
/// `MultiParticleMan_CreateSlot` and copies the 12-byte nil result to `a0`.
pub fn wscript_spawn_multi_particle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x27c4));
        frame.save(c, S1, 0x130);
        c.g[A3][0] = 0x500000;
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x110);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x140);
        c.g[S3][0] = c.g[A2][0];
        let initialised = c.g[V0][0] != 0u64;
        frame.save_ra(c, 0x100);
        if !initialised {
            c.cycles += 72;
            let rec = 0x500000u32.wrapping_sub(0x4098);
            c.g[AT][0] = 0x40800000;
            c.mtc1(1, 1);
            c.g[V0][0] = sx(rec);
            c.g[AT][0] = 0xffffffffbf800000;
            c.mtc1(1, 2);
            c.g[V1][0] = 0x1;
            c.mtc1(0, 0);
            c.g[A1][0] = 0x10;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 3);
            c.g[A2][0] = 0x80;
            c.swc1(1, rec + 0x10);
            c.swc1(2, rec + 0x8);
            c.sw(rec, c.g[V1][0]);
            c.sw(rec + 0x4, 0u64);
            for &o in MPS_ONE_F {
                c.swc1(3, rec + o);
            }
            for &o in MPS_ZERO_F {
                c.swc1(0, rec + o);
            }
            c.g[AT][0] = 0x41a00000;
            c.mtc1(1, 1);
            c.g[A0][0] = c.lbu(rec + 0xe0);
            c.sw(rec + 0xc4, c.g[A1][0]);
            c.sw(rec + 0xdc, c.g[A2][0]);
            c.swc1(1, rec + 0xd4);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x27c4), c.g[V1][0]);
            c.sw(rec + 0xc8, 0u64);
            c.sw(rec + 0xd0, c.g[V1][0]);
            c.sw(rec + 0xd8, 0u64);
            c.sb(sp(c), c.g[A0][0]);
        }
        c.cycles += 7;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0xe4;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4098));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e6574, 0x00306bf4)?;
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !none {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6890));
            c.g[A3][0] = 0x2;
            'mps_args: loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] != c.g[V0][0] {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A3][0] {
                        // int value declared as float: convert it
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        c.cycles += 3;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        let more = c.g[A1][0] != 0u64;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x00306c10);
                            }
                            continue 'mps_args;
                        }
                        break 'mps_args;
                    }
                }
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.cycles += 1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let more = c.g[A1][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x00306c10);
                    }
                    continue 'mps_args;
                }
                break 'mps_args;
            }
        }
        c.cycles += 21;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xf38));
        c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xdc));
        call_ptr(c, 0x00357a78, 0x00306c6c)?;
        c.g[V1][0] = 0x4d0000;
        c.g[V0][0] = c.g[S2][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S2][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S2][0] as u32);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptFn_SectionEventCommand` (0x00302210): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn w_script_fn_section_event_command(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00302210 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2640));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S5][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A2][0] = 0x500000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    let t = c.g[V0][0] != 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    if t {
                        pc = 0x00302270;
                        continue;
                    }
                    pc = 0x00302248;
                    continue;
                }
                0x00302248 => {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43e0));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = 0x1;
                    c.sw((c.g[A2][0] as u32).wrapping_sub(0x43e0), c.g[V1][0]);
                    c.sb((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0x2640), c.g[A0][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.sb(c.g[SP][0] as u32, c.g[A1][0]);
                    pc = 0x00302270;
                    continue;
                }
                0x00302270 => {
                    c.cycles += 7;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43e0));
                    c.g[A2][0] = 0x10;
                    c.g[RA][0] = sx(0x00302284);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e6574, 0x00302284)?;
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x003022e8;
                        continue;
                    }
                    pc = 0x0030228c;
                    continue;
                }
                0x0030228c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x440000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x65c8));
                    c.g[A3][0] = 0x2;
                    pc = 0x003022a0;
                    continue;
                }
                0x003022a0 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x003022d8;
                        continue;
                    }
                    pc = 0x003022c0;
                    continue;
                }
                0x003022c0 => {
                    c.cycles += 2;

                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                        pc = 0x003022d8;
                        continue;
                    }
                    pc = 0x003022c8;
                    continue;
                }
                0x003022c8 => {
                    c.cycles += 4;
                    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                    c.cvt_s_w(0, 0);
                    let t = true;
                    c.swc1(0, c.g[A0][0] as u32);
                    if t {
                        pc = 0x003022dc;
                        continue;
                    }
                    pc = 0x003022d8;
                    continue;
                }
                0x003022d8 => {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    pc = 0x003022dc;
                    continue;
                }
                0x003022dc => {
                    c.cycles += 3;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[A1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x003022a0) {
                            return Ok(());
                        }
                        pc = 0x003022a0;
                        continue;
                    }
                    pc = 0x003022e8;
                    continue;
                }
                0x003022e8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V0][0] == c.g[V1][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x00302340;
                        continue;
                    }
                    pc = 0x003022fc;
                    continue;
                }
                0x003022fc => {
                    c.cycles += 17;
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    if c.g[A0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = 0u64;
                    }
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00302338);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x00302338)?;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    if t {
                        pc = 0x00302344;
                        continue;
                    }
                    pc = 0x00302340;
                    continue;
                }
                0x00302340 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00302344;
                    continue;
                }
                0x00302344 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S3][0] = 0u64;
                    if t {
                        pc = 0x00302448;
                        continue;
                    }
                    pc = 0x0030234c;
                    continue;
                }
                0x0030234c => {
                    c.cycles += 13;
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[S4][0] = c.lw(c.g[S0][0] as u32);
                    c.g[RA][0] = sx(0x0030235c);
                    c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x0014dc80, 0x0030235c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00302368);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00302368)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = (c.g[S1][0] < 0x8u64) as u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                    if t {
                        pc = 0x00302394;
                        continue;
                    }
                    pc = 0x00302380;
                    continue;
                }
                0x00302380 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e40));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S3][0] = c.lw(c.g[V1][0] as u32);
                    pc = 0x00302394;
                    continue;
                }
                0x00302394 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[RA][0] = sx(0x003023a0);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x00144ce0, 0x003023a0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = (c.g[S2][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x490000;
                    if t {
                        pc = 0x00302448;
                        continue;
                    }
                    pc = 0x003023b0;
                    continue;
                }
                0x003023b0 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6870));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x003023c8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003023d4);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0022cea8, 0x003023d4)?;
                    let t = true;
                    c.g[V1][0] = 0x4d0000;
                    if t {
                        pc = 0x0030244c;
                        continue;
                    }
                    pc = 0x003023dc;
                    continue;
                }
                0x003023dc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003023e8);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x0022d088, 0x003023e8)?;
                    let t = true;
                    c.g[V1][0] = 0x4d0000;
                    if t {
                        pc = 0x0030244c;
                        continue;
                    }
                    pc = 0x003023f0;
                    continue;
                }
                0x003023f0 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0xe;
                    c.g[A2][0] = 0x2;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00302408);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    call_ptr(c, 0x00231250, 0x00302408)?;
                    c.g[RA][0] = sx(0x00302410);
                    call_ptr(c, 0x0028b180, 0x00302410)?;
                    c.g[RA][0] = sx(0x00302418);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x002a49e8, 0x00302418)?;
                    let t = true;
                    c.g[V1][0] = 0x4d0000;
                    if t {
                        pc = 0x0030244c;
                        continue;
                    }
                    pc = 0x00302420;
                    continue;
                }
                0x00302420 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0xe;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00302438);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    call_ptr(c, 0x00231250, 0x00302438)?;
                    let t = true;
                    c.g[V1][0] = 0x4d0000;
                    if t {
                        pc = 0x0030244c;
                        continue;
                    }
                    pc = 0x00302440;
                    continue;
                }
                0x00302440 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), c.g[V0][0]);
                    pc = 0x00302448;
                    continue;
                }
                0x00302448 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x4d0000;
                    pc = 0x0030244c;
                    continue;
                }
                0x0030244c => {
                    c.cycles += 17;
                    c.g[V0][0] = c.g[S5][0];
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
                    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[A3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.sdl(4, (c.g[S5][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[S5][0] as u32);
                    c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00302210, 0x00302490);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Shared tail of the two factories below (same shape as the counter-style
// factories, but each has its own constants and constructor call).

/// `MEM_copy` (0x3e6574; `call_indirect`).
pub(crate) const OC1567_MEM_COPY: u32 = 0x003e_6574;

/// `WorldObjFactory_AttachModifier` (0x2fae38, `checkActiveNode`).
pub(crate) const OC1567_CHECK_ACTIVE_NODE: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};

/// `EA_MemAllocNamed` (0x317d70).
pub(crate) const OC1567_MEM_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};

/// Call-site constants of one factory's node build.
pub(crate) struct Oc1567Node {
    /// Modifier type given to `checkActiveNode`.
    flag: u64,
    attach_ret: u32,
    /// Cycles charged before the allocation call.
    pre_alloc: u64,
    /// `cMemMan_alloc` size and name (`0x490000 - name_sub`).
    size: u64,
    name_sub: u32,
    alloc_ret: u32,
    ctor: u32,
    ctor_ret: u32,
    /// The constructor takes a fifth argument (`t0 = 0`).
    ctor_t0: bool,
}

/// Resolve the object the id at `buf[id_off]` (`S1 + id_off`) names into `S0`:
/// the script manager's object for -1, else the section table entry's record
/// (0 when there is none).
pub(crate) fn oc1567_resolve_object(c: &mut Cpu, id_off: u32) {
    c.cycles += 4;
    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(id_off)).lw(c);
    c.g[V0][0] = !(0u64 | c.g[A2][0]);
    let named = c.g[V0][0] != 0;
    let gp = r32(c, GP);
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if !named {
        c.cycles += 3;
        c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    if c.g[V1][0] != 0 {
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !nonzero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = 0;
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[A2][0];
}

/// With an object in `S0`: `checkActiveNode` on its active node (when it has
/// one) and, if that finds nothing, a new node from `cMemMan_alloc` built by
/// the factory's constructor.
pub(crate) fn oc1567_attach_or_build(c: &mut Cpu, node: &Oc1567Node) -> Flow {
    c.cycles += 2;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A1][0] = node.flag;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A2][0] = c.g[S1][0];
        OC1567_CHECK_ACTIVE_NODE.call(c, node.attach_ret)?;
        if c.g[V0][0] != 0 {
            return Ok(());
        }
    }
    c.cycles += node.pre_alloc;
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(node.name_sub));
    c.g[A0][0] = node.size;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    OC1567_MEM_ALLOC.call(c, node.alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S1][0];
    if node.ctor_t0 {
        c.g[T0][0] = 0;
    }
    call_ptr(c, node.ctor, node.ctor_ret)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x002fc5d8 WorldObjFactory_DeadFade

/// The dead-fade factory's init flag (`D_004A55E8`, at `gp + 0x24f8`).
pub(crate) const OC1567_DEAD_FADE_INIT: u32 = 0x24f8;

/// The dead-fade static description (`D_004FB510`, 20 bytes: id, 0, 2.0f,
/// 25.0f, a trailing byte).
#[cfg(test)]
pub(crate) const OC1567_DEAD_FADE_DESC: u32 = 0x004f_b510;

/// The per-slot type table of the dead-fade arguments (5 words).
#[cfg(test)]
pub(crate) const OC1567_DEAD_FADE_TYPES: u32 = 0x0044_5fa0;

/// `cLunoVar WorldObjFactory_DeadFade(cLunoVar* ret, int n, Ent* params)`
/// (0x2fc5d8, the C++ `MakeDeadFadeFunc`): the Luno call that gives an object
/// a dead-fade node. Expands the argument block over the static description
/// (initialised on the first call), resolves the object it names (id -1: the
/// script manager's current object), and, unless its active node already
/// is a fade (`checkActiveNode` type 0x16), allocates the 0x3c-byte node and
/// builds it. The nil Luno value is copied into `ret`.
pub fn world_obj_factory_dead_fade(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC1567_DEAD_FADE_INIT)).lw(c);
        frame.save(c, S3, 0x50);
        frame.save(c, S0, 0x80);
        c.g[S3][0] = c.g[A2][0]; // params
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S1, 0x70);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S4, 0x40);
        c.g[A2][0] = 0x0050_0000;
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x30);
        if !initialised {
            // Guarded static-local init of the description.
            c.cycles += 16;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4af0));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lbu(c);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 1);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41c8_0000;
            c.mtc1(AT, 0);
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4af0), c.g[V0][0]);
            c.sb((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.swc1(1, (c.g[V1][0] as u32).wrapping_add(8));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
            c.sw(gp.wrapping_add(OC1567_DEAD_FADE_INIT), c.g[A0][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4af0));
        c.g[A2][0] = 0x14;
        c.sw(frame.at(c, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, OC1567_MEM_COPY, 0x002f_c670)?;
        wof_fill_params(c, 4, 0x0044_5fa0, 0x002f_c688);
        oc1567_resolve_object(c, 0);
        oc1567_attach_or_build(
            c,
            &Oc1567Node {
                flag: 0x16,
                attach_ret: 0x002f_c758,
                pre_alloc: 12,
                size: 0x3c,
                name_sub: 0x6928,
                alloc_ret: 0x002f_c778,
                ctor: 0x0035_0750,
                ctor_ret: 0x002f_c790,
                ctor_t0: true,
            },
        )?;
        wof_copy_nil(c);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002fcdc8 WorldObjFactory_FlexBridge

/// The flex-bridge factory's init flag (`D_004A55F8`, at `gp + 0x2508`).
pub(crate) const OC1567_FLEX_BRIDGE_INIT: u32 = 0x2508;

/// The flex-bridge static description (`D_004FB5C0`, 40 bytes: nine words
/// and a trailing byte at +0x24).
#[cfg(test)]
pub(crate) const OC1567_FLEX_BRIDGE_DESC: u32 = 0x004f_b5c0;

/// The per-slot type table of the flex-bridge arguments (10 words).
#[cfg(test)]
pub(crate) const OC1567_FLEX_BRIDGE_TYPES: u32 = 0x0044_6038;

/// `cLunoVar WorldObjFactory_FlexBridge(cLunoVar* ret, int n, Ent* params)`
/// (0x2fcdc8, the C++ `MakeFlexBridgeFunc`): the dead-fade factory's shape
/// for a flex-bridge node (type 5, 0xb0 bytes, three-argument constructor at
/// 0x346350). The object id is the description's word at +0x20.
pub fn world_obj_factory_flex_bridge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(OC1567_FLEX_BRIDGE_INIT)).lw(c);
        frame.save(c, S0, 0x90);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0]; // n
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S1, 0x80);
        c.g[S3][0] = c.g[A2][0]; // params
        frame.save(c, S4, 0x50);
        let initialised = c.g[V0][0] != 0;
        frame.save_ra(c, 0x40);
        if !initialised {
            // Guarded static-local init of the description.
            c.cycles += 27;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a40));
            c.g[AT][0] = 0x43fa_0000;
            c.mtc1(AT, 3);
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lbu(c);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.lwc1(4, gp.wrapping_sub(0x3390));
            c.g[A2][0] = 0x10;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2);
            c.g[V1][0] |= 0xffff;
            c.g[AT][0] = 0x4516_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 1);
            c.swc1(3, (c.g[A3][0] as u32).wrapping_sub(0x4a40));
            c.sb((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A1][0]);
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(8));
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.sw(gp.wrapping_add(OC1567_FLEX_BRIDGE_INIT), c.g[A0][0]);
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0);
            c.sb(frame.at(c, 0), c.g[A1][0]);
        }
        // Expand the argument block over a copy of the description.
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(frame.at(c, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x4a40));
        c.sw(frame.at(c, 0x30), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, OC1567_MEM_COPY, 0x002f_ce8c)?;
        wof_fill_params(c, 5, 0x0044_6038, 0x002f_cea8);
        oc1567_resolve_object(c, 0x20);
        oc1567_attach_or_build(
            c,
            &Oc1567Node {
                flag: 5,
                attach_ret: 0x002f_cf78,
                pre_alloc: 11,
                size: 0xb0,
                name_sub: 0x6908,
                alloc_ret: 0x002f_cf98,
                ctor: 0x0034_6350,
                ctor_ret: 0x002f_cfac,
                ctor_t0: false,
            },
        )?;
        wof_copy_nil(c);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Guest functions the replacements call directly (`jal`), with their
/// generated entries.
pub(crate) const WSMAN_COLLECT_ITEM: Callee = Callee {
    addr: 0x0030_b9a0,
    f: ee::s10::f_0030b9a0,
};

pub(crate) const WSMAN_DEACTIVATE_ALL: Callee = Callee {
    addr: 0x0030_b7f8,
    f: ee::s10::f_0030b7f8,
};

/// Indirect (`jalr`) targets of the replacements.
pub(crate) const MEM_COPY: u32 = 0x003e_6574;

pub(crate) const LUNO_CAST_INT: u32 = 0x0022_6610;

pub(crate) const SECTION_EVENTBEGIN: u32 = 0x0022_d6c8;

/// The `Nil` image the game copies from: the register pair loads it from
/// `0x4d0000 - 0x6f68` (`0x4c9098`).
pub(crate) const NIL_BASE_HI: u64 = 0x004d_0000;

pub(crate) const NIL_OFFSET: u32 = 0x6f68;

/// Part of `WScriptFn_CollectItem` and `WScriptFn_StartEvent`: the record of
/// the current being. The world at `gp-0x848` gives the per-being table at
/// `+0x84` -> `+0xc` (indexed by `being * 4`, entry at `+0x28`); the record's
/// `+0x6c0` block holds the callback whose address is at `+0x4c` and whose
/// argument offset is the halfword at `+0x48`. The callback runs with
/// `A0 = record + 0x6c0 + offset`, and V0 becomes its result inverted.
/// `charge` is the cycles the generated block spends before the call.
pub(crate) fn call_being_entry(c: &mut Cpu, being: u32, ret_addr: u32, charge: u64) -> Flow {
    c.cycles += charge;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = sx(being << 2);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    if c.g[A0][0] == 0 {
        c.g[V1][0] = 0;
    }
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x48));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    call_ptr(c, target, ret_addr)?;
    c.g[V0][0] ^= 1;
    Ok(())
}

/// The keyed-argument loop of `WScriptFn_CollectItem` and `WScriptFn_StartEvent`:
/// for each of `count` entries at `A2` (`{ idx, value, kind }`, 16 bytes),
/// store the value at `base + 4*idx`, converted to float when the table
/// `table` declares that field a float (kind 2) and the entry's kind differs.
/// `head` is the loop's own address (the back-edge event check resumes there).
pub(crate) fn copy_keyed_args(c: &mut Cpu, base: u32, table: u32, count: u64, head: u32) {
    c.g[A1][0] = count;
    c.g[T0][0] = sx(table);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(base));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.cycles += 1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `WScriptFn_CollectItem` (0x301120, `CollectFunc`): gathers the keyed
/// arguments of a script call into a 12-byte block on the stack (the
/// defaults come from a static copy, guarded by a one-time init), and when
/// the item id is set, collects that item for the current being. Returns
/// `self` with its word set to 1 (collected) or 0.
pub fn wscript_fn_collect_item(c: &mut Cpu) {
    guest(c, collect_item_body);
}

pub(crate) fn collect_item_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x80);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
    frame.save(c, S1, 0x60);
    c.g[V0][0] = u64::MAX;
    frame.save(c, S3, 0x40);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S4, 0x30);
    c.g[S3][0] = c.g[A1][0];
    frame.save(c, S0, 0x70);
    frame.save_ra(c, 0x20);
    frame.save(c, S2, 0x50);
    c.g[S2][0] = c.lw(c.g[V1][0] as u32);
    let none = c.g[S2][0] == c.g[V0][0];
    c.g[S4][0] = c.g[A2][0];
    if none {
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        let being = c.g[S2][0] as u32;
        call_being_entry(c, being, 0x0030_1198, 18)?;
    }
    collect_item_part_b(c)?;
    c.cycles += 9;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// The second half of `WScriptFn_CollectItem`: the argument block and the
/// collection. Ends with the result call that stores V0 into `self + 8`
/// (the cast) and `self`'s word; the collect path makes its own calls and
/// skips that tail.
pub(crate) fn collect_item_part_b(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let a_zero = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x25c4));
    if a_zero {
        // The part A result was 1: nothing to collect; V0 = 1 below.
        c.cycles += 1;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        let inited = c.g[V0][0] != 0;
        c.g[A2][0] = 0x500000;
        if !inited {
            c.cycles += 10;
            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4430));
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A2][0] as u32).wrapping_sub(0x4430), c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x25c4), c.g[A0][0]);
            c.sb(sp(c), c.g[A1][0]);
        }
        // The argument block at sp+0x10: the copied defaults, then the keys.
        c.cycles += 10;
        c.g[V0][0] = u64::MAX;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4430));
        c.g[A2][0] = 0xc;
        c.sw(sp(c).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, MEM_COPY, 0x0030_11fc)?;
        let no_keys = (c.g[S3][0] as i64) <= 0;
        c.g[A2][0] = c.g[S4][0];
        if !no_keys {
            c.cycles += 3;
            let base = c.g[S0][0] as u32;
            let table = (c.g[GP][0] as u32).wrapping_add(0xb78);
            let count = c.g[S3][0];
            copy_keyed_args(c, base, table, count, 0x0030_1210);
        }
        c.cycles += 5;
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        let no_item = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !no_item {
            // The item id is set: collect it for the current being.
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            }
            c.cycles += 10;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.g[S0][0] = 1;
            WSMAN_COLLECT_ITEM.call(c, 0x0030_12a4)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, LUNO_CAST_INT, 0x0030_12b0)?;
            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
            return Ok(());
        }
    }
    // The tail: `self + 8` takes V0 through the cast, and `*V0 = 0`.
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, LUNO_CAST_INT, 0x0030_12c8)?;
    c.sw(c.g[V0][0] as u32, 0);
    Ok(())
}

/// `WScriptFn_StartEvent` (0x302048): gathers the keyed arguments like
/// `WScriptFn_CollectItem`, then starts the section event the block names
/// (the mode from the table at `0x445e40`, after deactivating the missions
/// when the event is not a free ride). Returns the `Nil` result copied from
/// the game image.
pub fn wscript_fn_start_event(c: &mut Cpu) {
    guest(c, start_event_body);
}

pub(crate) fn start_event_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x70);
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x263c));
    frame.save(c, S2, 0x40);
    frame.save(c, S0, 0x60);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S3, 0x30);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S1, 0x50);
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);
    let inited = c.g[V0][0] != 0;
    c.g[A2][0] = 0x500000;
    if !inited {
        c.cycles += 9;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43f0));
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 1;
        c.sw((c.g[A2][0] as u32).wrapping_sub(0x43f0), c.g[V1][0]);
        c.sb((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x263c), c.g[A1][0]);
        c.sb(sp(c), c.g[A0][0]);
    }
    // The argument block at sp+0x10 (copied from the defaults at 0x4fc10).
    c.cycles += 7;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43f0));
    c.g[A2][0] = 0xc;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, MEM_COPY, 0x0030_20b0)?;
    let no_keys = (c.g[S0][0] as i64) <= 0;
    c.g[A2][0] = c.g[S2][0];
    if !no_keys {
        c.cycles += 4;
        let base = c.g[S1][0] as u32;
        let table = (c.g[GP][0] as u32).wrapping_add(0xbd0);
        let count = c.g[S0][0];
        copy_keyed_args(c, base, table, count, 0x0030_20c8);
    }
    // The being's record, when there is one.
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
    let no_being = c.g[V0][0] == c.g[V1][0];
    if no_being {
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.cycles += 1;
        c.g[V0][0] = 0;
    } else {
        let being = c.g[V0][0] as u32;
        call_being_entry(c, being, 0x0030_2160, 17)?;
    }
    // The section event for the block's mode, unless the block has a second
    // value (then it is left alone).
    c.cycles += 2;
    let a_zero = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    let mut exit_early = false;
    if !a_zero {
        c.cycles += 7;
        c.g[S0][0] = 0;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        let id_in_range = c.g[V0][0] != 0;
        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        if id_in_range {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e40));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = c.lw(c.g[V1][0] as u32);
        }
        c.cycles += 2;
        let second_set = c.g[A0][0] != 0;
        c.g[V1][0] = NIL_BASE_HI;
        if second_set {
            exit_early = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let free_ride = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if !free_ride {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                WSMAN_DEACTIVATE_ALL.call(c, 0x0030_21c0)?;
                c.g[A0][0] = c.g[S1][0];
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SECTION_EVENTBEGIN, 0x0030_21cc)?;
        }
    }
    if !exit_early {
        c.cycles += 1;
        c.g[V1][0] = NIL_BASE_HI;
    }
    // The epilogue: copy the Nil result to `self` and return `self`.
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(NIL_OFFSET));
    let nil = c.g[A3][0] as u32;
    c.ldl(A0, nil.wrapping_add(0x7));
    c.ldr(A0, nil);
    c.g[A1][0] = c.lw(nil.wrapping_add(0x8));
    let this = c.g[S3][0] as u32;
    c.sdl(A0, this.wrapping_add(0x7));
    c.sdr(A0, this);
    c.sw(this.wrapping_add(0x8), c.g[A1][0]);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `RiderInTunnelFunc` (0x303f80): when the current being is set, marks its
/// rider as in a tunnel (the `+0x3fc` flag of its record); returns the `Nil`
/// result copied from the game image.
pub fn rider_in_tunnel_func(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
    c.g[V0][0] = u64::MAX;
    c.g[V1][0] = c.lw(c.g[V1][0] as u32);
    let no_being = c.g[V1][0] == c.g[V0][0];
    c.g[A3][0] = c.g[A0][0];
    if !no_being {
        c.cycles += 12;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A2][0] = 1;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.sw((c.g[V0][0] as u32).wrapping_add(0x3fc), c.g[A2][0]);
    }
    c.cycles += 10;
    c.g[V1][0] = NIL_BASE_HI;
    c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(NIL_OFFSET));
    let nil = c.g[T0][0] as u32;
    c.ldl(A0, nil.wrapping_add(0x7));
    c.ldr(A0, nil);
    c.g[A1][0] = c.lw(nil.wrapping_add(0x8));
    let this = c.g[A3][0] as u32;
    c.sdl(A0, this.wrapping_add(0x7));
    c.sdr(A0, this);
    c.sw(this.wrapping_add(0x8), c.g[A1][0]);
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// `luno_t WScriptFn_AddRollerModifier(buf_t *out, int n, Ent *params)`
/// (0x002fd250): the script call `AddRollerModifier(...)`. Builds the 5-word
/// parameter block (defaults from a guarded static: id -1, 1.0, 0.7, 0.5, one
/// byte), overwrites the slots named by the `n` `{idx, val, ?, type}` entries
/// (converting an int to a float when the keyword type table says Float),
/// finds the target instance (`id == -1`: the script manager's own instance,
/// else the world cache: `section[id & 0xff]`, tag `[id >> 8]`) and, when its
/// active node is a move node (`isMoveNode()` vtable call), calls
/// `cMoveNode_addRoller(node, buf, IntersectInfo, CollideInfo)`. Returns the
/// nil constant, copied into `*out` (12 bytes).
pub fn wscript_fn_add_roller_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let fp = sp(c).wrapping_sub(0x80);
        c.g[SP][0] = sx(fp);
        c.g[V0][0] = c.lw(gp.wrapping_add(0x2510));
        c.sq(18, fp.wrapping_add(0x50));
        c.sq(16, fp.wrapping_add(0x70));
        c.g[S2][0] = c.g[A2][0];
        c.sq(19, fp.wrapping_add(0x40));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, fp.wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sd(fp.wrapping_add(0x30), c.g[RA][0]);
        let initialised = c.g[V0][0] != 0;
        c.g[A2][0] = 0x500000;
        if !initialised {
            // guarded static-local init of the default block at 0x4fb628
            c.cycles += 17;
            c.g[V1][0] = sx(0x4fb628);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A0][0] = c.lbu(0x4fb628 + 0x10);
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 2);
            c.g[A1][0] = 0x1;
            c.lwc1(0, gp.wrapping_sub(0x3388));
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 1);
            c.sw(0x4fb628, c.g[V0][0]);
            c.sb(0x4fb628 + 0x10, c.g[A0][0]);
            c.swc1(2, 0x4fb628 + 0x4);
            c.swc1(0, 0x4fb628 + 0x8);
            c.swc1(1, 0x4fb628 + 0xc);
            c.sw(gp.wrapping_add(0x2510), c.g[A1][0]);
            c.sb(fp, c.g[A0][0]);
        }

        // buf = defaults (id = -1), copied with EA_MemCopy(buf, 0x4fb628, 20)
        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[S1][0] = sx(fp.wrapping_add(0x10));
        c.g[A1][0] = sx(0x4fb628);
        c.g[A2][0] = 0x14;
        c.sw(fp.wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6574, 0x002f_d2e4)?;
        let none = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !none {
            // for each entry: buf[idx] = val (float-converted when the
            // keyword type of idx is Float (2) and the entry's type differs)
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(0x440000u32.wrapping_add(0x60a0));
            c.g[V0][0] = 0x440000;
            c.g[A3][0] = 0x2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[V0][0] = c.lw(c.g[V0][0] as u32);
                let mut store_int = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, c.g[A0][0] as u32);
                        store_int = false;
                    }
                }
                if store_int {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_d300);
                    }
                    continue;
                }
                break;
            }
        }

        // the target instance: obj in V0
        c.cycles += 4;
        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = !c.g[A2][0];
        let by_id = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(gp.wrapping_add(0x16c8));
        if by_id {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let no_section = c.g[V1][0] == 0;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            let mut found = false;
            if !no_section {
                c.cycles += 7;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
        }

        // active node: if it is a move node, add the roller
        c.cycles += 3;
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let no_node = c.g[S0][0] == 0;
        c.g[V1][0] = 0x4d0000;
        if !no_node {
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002f_d3c4)?;
            let not_move_node = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(gp.wrapping_add(0xce8));
            if !not_move_node {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x294));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x298));
                call_ptr(c, 0x0035_5db8, 0x002f_d3e0)?;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x4d0000;
        }

        // return the nil constant (12 bytes copied to *out)
        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(0x4d0000u32.wrapping_sub(0x6f68));
        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(4, (c.g[S3][0] as u32).wrapping_add(0x7));
        c.sdr(4, c.g[S3][0] as u32);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.lq(16, fp.wrapping_add(0x70));
        c.lq(17, fp.wrapping_add(0x60));
        c.lq(18, fp.wrapping_add(0x50));
        c.lq(19, fp.wrapping_add(0x40));
        c.g[RA][0] = c.ld(fp.wrapping_add(0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(fp.wrapping_add(0x80));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::game::scripter::ssxscriptengine::tests::oc428_install_fakes;

    use super::*;
    use crate::game::replay::replaycache::tests::{
        oc420_hook_add_parent, oc420_hook_audio_instance, oc420_hook_audio_stop_loop,
        oc420_hook_luno_as_float, oc420_hook_rand_range,
    };
    pub(crate) use crate::game::sound::soundsys::tests::oc739_install;

    use crate::difftest::fill;
    use crate::difftest::DATA;

    /// `WorldObjFactory_DeadNode`: every event kind (0, 1, 3, others on both
    /// sides of 2), no object or an object of type 6, 0x13 or other (its
    /// delete through a fake vtable slot), random node flags; the real
    /// `EA_MemAllocNamed` and node constructors as in
    /// `world_obj_replace_with_dead_node_matches`.
    #[test]
    fn world_obj_factory_dead_node_matches() {
        use crate::difftest::check;
        use crate::difftest::fakes::vtable;
        use crate::game::fe::ovtemplatepausemenu::tests::setup;
        const NODE: u32 = DATA + 0x800;
        const OBJ: u32 = DATA + 0xa00;
        const EVENT: u32 = DATA + 0xc00;
        check(
            ee::s10::f_002fc2c0__generated,
            world_obj_factory_dead_node,
            240,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2f_c2c0);
                c.pc = 0x002f_c2c0;
                let kind: u32 = match seed % 7 {
                    0 | 1 => 0,
                    2 | 3 => 1,
                    4 | 5 => 3,
                    _ => [2, 0xffff_ffff, 4, 0x80][(seed / 7 % 4) as usize],
                };
                c.bus.write32(EVENT + DEAD_NODE_EVENT_KIND, kind);
                fill(c, &mut r, NODE, 8);
                let obj = if seed / 7 % 4 == 0 { 0 } else { OBJ };
                c.bus.write32(NODE + DEAD_NODE_OBJECT, obj);
                fill(c, &mut r, OBJ, 8);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(OBJ + DEAD_NODE_OBJ_VTABLE, vt);
                let ty = match seed / 3 % 4 {
                    0 => 6,
                    1 => 0x13,
                    _ => r.next_u32() as u16 & 0x1f,
                };
                c.bus.write16(OBJ + DEAD_NODE_OBJ_TYPE, ty);
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(EVENT);
            },
        );
    }

    /// `WorldObjFactory_AttachModifier`: a node that already is a type 9, 6
    /// or 22 modifier, or already `type` (refresh: a fake), and for other
    /// nodes types 13 (particle) and 0 (debounce: the real allocator and
    /// constructors on a test heap; the node's "which transform" method
    /// returns the object's own matrix, so ModObj_TakeOverObject deletes the
    /// node through a fake) and unknown types (deleted through a fake, some
    /// unwinding). `IsType` is the real leaf 0x2afa58 (`*field == type`).
    /// Not reached: types 11 (mesh animation) and 22 (dead fade), whose
    /// constructors walk model data.
    #[test]
    fn world_obj_factory_attach_modifier_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{elf_image, DATA, GAME_GP};
        const NODE: u32 = DATA + 0x1000;
        const NODE_KIND: u32 = NODE + 0x40;
        const VT: u32 = DATA + 0x2000;
        const OBJECT: u32 = DATA + 0x3000;
        const BUF: u32 = DATA + 0x4000;
        const BELIB: u32 = DATA + 0x5000;
        covered(
            ee::s10::f_002fae38__generated,
            world_obj_factory_attach_modifier,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_ae38, 2);
                elf_image(c);
                crate::game::bx::bxstring::tests::heap(c);
                for a in (NODE..BELIB + 0x100).step_by(4) {
                    c.bus.write32(a, 0);
                }
                c.bus.write32(GAME_GP - 0x1f04, BELIB);
                // The node: vtable, kind field, its object.
                c.bus.write32(NODE + WOF_ATTACH_VTABLE, VT);
                c.bus.write32(NODE + WOF_ATTACH_OBJECT, OBJECT);
                c.bus.write16(VT + WOF_ATTACH_VT_IS_TYPE, 0x40);
                c.bus.write32(VT + WOF_ATTACH_VT_IS_TYPE + 4, 0x002a_fa58);
                c.bus.write32(VT + WOF_ATTACH_VT_REFRESH + 4, FAKE);
                c.bus.write32(VT + WOF_ATTACH_VT_DELETE + 4, FAKE + 0x10);
                c.bus.write32(VT + 0x84, 0x0014_06c0); // no particle modifiers
                c.bus.write32(VT + 0xc4, FAKE + 0x20); // its transform
                c.bus.write32(LOG_RESULTS, OBJECT + 0x10);
                c.bus.write32(OBJECT + 0x8, 0x100);
                c.bus.write32(OBJECT + 0xc, r.next_u32());
                let ty: u32 = match seed % 8 {
                    0 | 1 => 13,
                    2 | 3 => 0,
                    4 => [1, 5, 12, 14, 30][(seed / 8 % 5) as usize],
                    5 => (r.next_u32() % 40).wrapping_neg(),
                    _ => match r.next_u32() % 24 {
                        // Not the mesh-animation / dead-fade constructors.
                        t @ (11 | 22) => t + 1,
                        t => t,
                    },
                };
                let kind = match seed % 11 {
                    0 => 9,
                    1 => 6,
                    2 => 22,
                    3 => ty,
                    _ => 100,
                };
                c.bus.write32(NODE_KIND, kind);
                c.pc = 0x002f_ae38;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(ty);
                c.g[A2][0] = sx(BUF);
            },
        );
    }

    use crate::difftest::check;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::oc327_fake_gated;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::oc327_hook;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_CTRL;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_MAGIC;
    pub(crate) use crate::game::ealib::seg_2b4578::tests::OC327_RET;
    pub(crate) use crate::game::object::debouncenode::tests::oc327_install_alloc;
    pub(crate) use crate::game::object::debouncenode::tests::RET_ALLOC;
    pub(crate) use crate::game::object::debouncenode::tests::RET_BASE_CTOR;
    use std::sync::atomic::AtomicUsize;

    /// `WorldObjFactory_Hide`: `n` entries at DATA+0x8000 (the first one, when
    /// present, carries the id `0x123`), the world's object reference table at
    /// DATA+0x5000, the `-1` object at DATA+0x5400, and in some seeds an
    /// object to hide at DATA+0x6000 whose vtable slot 0x50 is a fake (hidden
    /// object) or the real zero leaf (killed, then a `Hide` node allocated).
    pub(crate) fn world_obj_factory_hide_setup(c: &mut Cpu, seed: u32) {
        /// `func_001406C0`, a real leaf returning 0.
        const ZERO_LEAF: u32 = 0x0014_06c0;
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;

        let mut r = machine(c, seed, 0x002f_fb50, 6);
        // Make sure the fake query/kill calls unwind in some seeds.
        if seed % 2 == 0 {
            c.bus.write32(LOG_UNWIND_AT, 0);
        }
        pool_allocator(c);
        heap_init(c);
        be_library(c);
        world(c);

        // The guarded static seed, and sometimes the uninitialized path.
        c.bus
            .write32(GAME_GP + WOF_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
        // The seed word is -1, as the guarded init leaves it.
        c.bus.write32(GAME_GP + WOF_SEED_WORD, 0xffff_ffff);
        c.bus
            .write32(GAME_GP + WOF_SEED_BYTE, 0x1122_3300 | (seed & 0xff));
        for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
            c.bus.write32(GAME_GP + WOF_KINDS + 4 * i as u32, kind);
        }

        // Variant: what the entry list holds (0-2 leave the id at -1).
        let variant = seed % 6;
        let obj_kind = (seed / 6) % 5;
        let entries: &[(u32, u32, u32)] = match variant {
            0 | 1 => &[],
            2 => &[(1, 0x42, 2), (2, 0x10, 3)],
            _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
        };
        let n: u32 = match variant {
            0 | 1 => 0,
            2 => 2,
            _ => 4,
        };
        let ents = DATA + 0x8000;
        for (i, &(idx, val, typ)) in entries.iter().enumerate() {
            let p = ents + 0x10 * i as u32;
            c.bus.write32(p, idx);
            c.bus.write32(p + 4, val);
            c.bus.write32(p + 0xc, typ);
        }

        // What the id resolves to: 0 (missing entry), 0 (null reference),
        // an object with no instance, a hidden object, or a visible one.
        let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
        if variant >= 3 {
            let world_obj = DATA + 0x5000;
            let table = DATA + 0x5100;
            let table_slot = table + 4 * 0x23; // id & 0xff
            c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
            c.bus.write32(world_obj + 8, table);
            if obj_kind == 0 {
                c.bus.write32(table_slot, 0);
            } else {
                let s = DATA + 0x5200;
                c.bus.write32(table_slot, s);
                c.bus.write32(s + 0x1c, DATA + 0x5300);
                // The reference: obj/4, read back shifted left by 8.
                let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                c.bus.write32(DATA + 0x5304, word); // [id >> 8] = [1]
            }
        } else {
            let d = DATA + 0x5400;
            c.bus.write32(GAME_GP + WOF_SPECIAL, d);
            c.bus.write32(d + 0x290, obj);
        }
        if obj != 0 {
            let p = DATA + 0x5900;
            match obj_kind {
                2 => c.bus.write32(obj + 0xc, 0),
                3 => {
                    let vt = vtable(c, &mut r, &[0x50]);
                    c.bus.write32(p + 0xc, vt);
                    c.bus.write32(obj + 0xc, p);
                }
                _ => {
                    let vt = DATA + 0x5800;
                    c.bus.write16(vt + 0x50, 0);
                    c.bus.write32(vt + 0x54, ZERO_LEAF);
                    c.bus.write32(p + 0xc, vt);
                    c.bus.write32(obj + 0xc, p);
                }
            }
        }

        c.g[A0][0] = sx(DATA + 0x700); // the 12-byte result buffer
        c.g[A1][0] = sx(n);
        c.g[A2][0] = sx(ents);
        c.pc = 0x002f_fb50;
    }

    #[test]
    fn world_obj_factory_hide_matches() {
        use crate::difftest::fakes::covered;
        covered(
            ee::s10::f_002ffb50__generated,
            world_obj_factory_hide,
            240,
            &|c, seed| world_obj_factory_hide_setup(c, seed),
        );
    }

    /// `WorldObjFactory_Debounce`: the same kind of entry list as Hide (five
    /// slots), the static seed at 0x4fb528, and an object whose +0xc node is
    /// absent (the node is allocated) or present with a fake query method
    /// (nonzero: the object counts as covered, no allocation).
    #[test]
    fn world_obj_factory_debounce_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;
        check(
            ee::s10::f_002fc7d0__generated,
            world_obj_factory_debounce,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_c7d0, 6);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
                pool_allocator(c);
                heap_init(c);
                be_library(c);
                world(c);

                c.bus
                    .write32(GAME_GP + WOD_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
                // 0x4fb528: the static's post-init contents — id -1, 1.0f, 1,
                // -1 and the flag byte.
                let seed_word = WOF_RESULT_BASE - WOD_STATIC_OFF;
                c.bus.write32(seed_word, 0xffff_ffff);
                c.bus.write32(seed_word + 4, 0x3f80_0000);
                c.bus.write32(seed_word + 8, 1);
                c.bus.write32(seed_word + 12, 0xffff_ffff);
                c.bus.write32(seed_word + 16, 0x1122_3300 | (seed & 0xff));
                for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
                    c.bus
                        .write32(WOD_KINDS_BASE + WOD_KINDS_OFF + 4 * i as u32, kind);
                }

                let variant = seed % 6;
                let obj_kind = (seed / 6) % 4;
                let entries: &[(u32, u32, u32)] = match variant {
                    0 | 1 => &[],
                    2 => &[(1, 0x42, 2), (2, 0x10, 3)],
                    _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
                };
                let n: u32 = match variant {
                    0 | 1 => 0,
                    2 => 2,
                    _ => 4,
                };
                let ents = DATA + 0x9000;
                for (i, &(idx, val, typ)) in entries.iter().enumerate() {
                    let p = ents + 0x10 * i as u32;
                    c.bus.write32(p, idx);
                    c.bus.write32(p + 4, val);
                    c.bus.write32(p + 0xc, typ);
                }

                let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
                if variant >= 3 {
                    let world_obj = DATA + 0x5000;
                    let table = DATA + 0x5100;
                    let table_slot = table + 4 * 0x23;
                    c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
                    c.bus.write32(world_obj + 8, table);
                    if obj_kind == 0 {
                        c.bus.write32(table_slot, 0);
                    } else {
                        let s = DATA + 0x5200;
                        c.bus.write32(table_slot, s);
                        c.bus.write32(s + 0x1c, DATA + 0x5300);
                        let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                        c.bus.write32(DATA + 0x5304, word);
                    }
                } else {
                    let d = DATA + 0x5400;
                    c.bus.write32(GAME_GP + WOF_SPECIAL, d);
                    c.bus.write32(d + 0x290, obj);
                }
                if obj != 0 {
                    // No node yet, or a node whose virtual query is a fake
                    // (nonzero: the object counts as covered, no allocation).
                    let p = DATA + 0x5900;
                    if obj_kind == 2 {
                        c.bus.write32(obj + 0xc, 0);
                    } else {
                        let vt = vtable(c, &mut r, &[0x50]);
                        c.bus.write32(p + 0xc, vt);
                        c.bus.write32(obj + 0xc, p);
                    }
                }

                c.g[A0][0] = sx(DATA + 0x700);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ents);
                c.pc = 0x002f_c7d0;
            },
        );
    }

    /// `WScriptFn_AddSplineModifier`: the 48-byte static seed at 0x4fb778,
    /// twelve entry slots, the id and the target-note word controlled by the
    /// entries, and a move node whose virtual "supports splines" slot 0x80 is
    /// a fake (adds the spline) or the real zero leaf (does not).
    #[test]
    fn wscript_fn_add_spline_modifier_matches() {
        /// `func_001406C0`, a real leaf returning 0.
        const ZERO_LEAF: u32 = 0x0014_06c0;
        use crate::difftest::fakes::{machine, vtable, LOG_UNWIND_AT};
        use crate::difftest::{check, DATA, GAME_GP};
        use crate::game::fe::feasyncfile::tests::{
            be_library_5 as be_library, heap_init_5 as heap_init, world_3 as world,
        };
        use crate::game::worldpainter::quadtree::tests::pool_allocator;
        check(
            ee::s10::f_002fded0__generated,
            wscript_fn_add_spline_modifier,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_ded0, 8);
                if seed % 2 == 0 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
                pool_allocator(c);
                heap_init(c);
                be_library(c);
                world(c);

                c.bus
                    .write32(GAME_GP + WSA_INIT_FLAG, if seed % 4 == 0 { 0 } else { 1 });
                // 0x4fb778: the static's post-init contents — -1, -1, 1, 3,
                // 20.0f, 0, 0, 0, 1, 0, 0 and the tag byte at +0x2c.
                let seed_word = WSA_STATIC_BASE - WSA_STATIC_OFF;
                for (off, word) in [
                    (0u32, 0xffff_ffffu32),
                    (4, 0xffff_ffff),
                    (8, 1),
                    (12, 3),
                    (16, 0x41a0_0000),
                    (20, 0),
                    (24, 0),
                    (28, 0),
                    (32, 1),
                    (36, 0),
                    (40, 0),
                ] {
                    c.bus.write32(seed_word + off, word);
                }
                c.bus.write32(seed_word + 44, 0x1122_3300 | (seed & 0xff));
                for (i, kind) in [2u32, 3, 2, 0].into_iter().enumerate() {
                    c.bus
                        .write32(WSA_KINDS_BASE + WSA_KINDS_OFF + 4 * i as u32, kind);
                }

                let variant = seed % 6;
                let obj_kind = (seed / 6) % 5;
                let entries: &[(u32, u32, u32)] = match variant {
                    0 | 1 => &[],
                    2 => &[(2, 0x10, 3), (3, 0x7, 0)],
                    4 => &[(0, 0x123, 2), (2, 0x10, 3), (3, 0x7, 0)],
                    _ => &[(0, 0x123, 2), (1, 0x42, 2), (2, 0x10, 3), (3, 0x7, 0)],
                };
                let n: u32 = match variant {
                    0 | 1 => 0,
                    2 => 2,
                    4 => 3,
                    _ => 4,
                };
                let ents = DATA + 0x8000;
                for (i, &(idx, val, typ)) in entries.iter().enumerate() {
                    let p = ents + 0x10 * i as u32;
                    c.bus.write32(p, idx);
                    c.bus.write32(p + 4, val);
                    c.bus.write32(p + 0xc, typ);
                }

                let obj = if obj_kind < 2 { 0 } else { DATA + 0x6000 };
                if variant >= 3 {
                    let world_obj = DATA + 0x5000;
                    let table = DATA + 0x5100;
                    let table_slot = table + 4 * 0x23;
                    c.bus.write32(GAME_GP + WOF_OBJECTS, world_obj);
                    c.bus.write32(world_obj + 8, table);
                    if obj_kind == 0 {
                        c.bus.write32(table_slot, 0);
                    } else {
                        let s = DATA + 0x5200;
                        c.bus.write32(table_slot, s);
                        c.bus.write32(s + 0x1c, DATA + 0x5300);
                        let word = if obj_kind == 1 { 0 } else { (obj >> 2) << 8 };
                        c.bus.write32(DATA + 0x5304, word);
                    }
                } else {
                    let d = DATA + 0x5400;
                    c.bus.write32(GAME_GP + WOF_SPECIAL, d);
                    c.bus.write32(d + 0x290, obj);
                }
                if obj != 0 {
                    let p = DATA + 0x5900;
                    match obj_kind {
                        2 => c.bus.write32(obj + 0xc, 0),
                        3 => {
                            let vt = vtable(c, &mut r, &[0x80]);
                            c.bus.write32(p + 0xc, vt);
                            c.bus.write32(obj + 0xc, p);
                        }
                        _ => {
                            let vt = DATA + 0x5800;
                            c.bus.write16(vt + 0x80, 0);
                            c.bus.write32(vt + 0x84, ZERO_LEAF);
                            c.bus.write32(p + 0xc, vt);
                            c.bus.write32(obj + 0xc, p);
                        }
                    }
                }

                c.g[A0][0] = sx(DATA + 0x700);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(ents);
                c.pc = 0x002f_ded0;
            },
        );
    }

    /// `WScriptFn_ObjectExists`: the one-time id-buffer init, the entry
    /// overlay (matching type, float slot, int slot), and every id lookup
    /// outcome (`-1`, absent low byte, absent high byte, found live/dead).
    #[test]
    fn wscript_fn_object_exists_matches() {
        check(
            ee::s10::f_00303130__generated,
            wscript_fn_object_exists,
            240,
            &|c, seed| {
                let mut r = Rng(0x3031_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let obj = DATA + 0x2000;
                let entries = DATA + 0x2100;
                let vt = DATA + 0x2200;
                let x = DATA + 0x2300;
                let base = DATA + 0x2400;
                let high = DATA + 0x2800;
                let desc = DATA + 0x2c00;
                let target = DATA + 0x3000;
                c.g[A0][0] = obj as u64;
                let n = [-1i64, 0, 1, 2, 3][(seed % 5) as usize];
                c.g[A1][0] = n as u64;
                c.g[A2][0] = entries as u64;
                // The script value types of the three slots.
                for i in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xc28 + 4 * i, i % 3);
                }
                c.bus.write32(GAME_GP + 0x26c4, seed as u32 % 2);
                // The shared id buffer: the all-ones id, or a real one whose
                // low/high bytes select the lookup outcome.
                let id = match seed % 5 {
                    0 => 0xffff_ffffu32, // the world object
                    1 => 0x0000_0101,    // lo=1, hi=1: found
                    2 => 0x0000_0103,    // lo=3: no such object
                    3 => 0x0000_0001,    // hi=0: no such entry
                    _ => 0x0000_0101,
                };
                c.bus.write32(WSCRIPT_FN_OBJECT_EXISTS_IDS, id);
                c.bus
                    .write32(WSCRIPT_FN_OBJECT_EXISTS_IDS + 4, r.next_u32());
                c.bus
                    .write32(WSCRIPT_FN_OBJECT_EXISTS_IDS + 8, r.next_u32());
                // The entries: slot, value, type (0..3).
                for i in 0..4u32 {
                    let e = entries + 16 * i;
                    c.bus.write32(e, (seed as u32 >> (2 * i)) % 3);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, (seed as u32 >> (4 + i)) % 4);
                }
                // The id -> object tables.
                c.bus.write32(GAME_GP + 0x16c8, x);
                c.bus.write32(x, x + 0x40);
                c.bus.write32(x + 0x48, base);
                c.bus.write32(GAME_GP + 0xce8, vt);
                c.bus
                    .write32(vt + 0x290, if seed % 3 == 0 { 0 } else { target });
                for lo in 0..4u32 {
                    c.bus
                        .write32(base + 4 * lo, if lo == 1 || lo == 2 { desc } else { 0 });
                }
                c.bus.write32(desc + 0x1c, high);
                for hi in 0..4u32 {
                    c.bus
                        .write32(high + 4 * hi, if hi == 1 { (target >> 2) << 8 } else { 0 });
                }
                c.bus
                    .write32(target + 0xc, if seed % 5 < 2 { 0 } else { 1 });
                c.bus.write32(target + 0x10, r.next_u32());
            },
        );
    }

    /// The career flag block (initialised or not, and a known or unknown
    /// first value), the per-flag types, the flags word and an argument list
    /// of -1..3 entries; both store forms (raw word and float).
    #[test]
    fn wscript_fn_get_progress_flag_bit_matches() {
        use crate::difftest::fakes::machine;
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        check(
            ee::s10::f_003061b0__generated,
            wscript_fn_get_progress_flag_bit,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_61b0, 0);
                c.pc = 0x0030_61b0;
                let gp = GAME_GP;
                // An uninitialised block is zeroed by the function itself.
                c.bus.write32(
                    gp + OC324_CAREER_FLAGS_INIT,
                    if seed % 3 == 0 { 0 } else { 1 },
                );
                let key = match seed % 6 {
                    0 => 0u32,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff,
                    _ => r.next_u32() % 4,
                };
                c.bus.write32(gp + OC324_CAREER_FLAGS, key);
                c.bus.write32(gp + OC324_CAREER_FLAGS + 4, r.next_u32());
                // The career flags word the bit is read from.
                c.bus.write32(0x0053_08d0, r.next_u32());
                // The per-slot types the entries are compared against.
                for i in 0..4u32 {
                    let ty = if (i + seed as u32) % 3 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(gp + 0xcb8 + 4 * i, ty);
                }
                let n = (seed % 5) as i32 - 1;
                for i in 0..4u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, (seed + i) % 4);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    /// The mission description (a -1 id or a real one), the mission id and
    /// value words, and a script manager whose command answers -1 at once
    /// (the value is never 0xf / 0x13 and the game state is never 4), so
    /// both the "no command" copy and the command path run.
    #[test]
    fn wscript_fn_mission_command_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{machine, FAKE};
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const MAN: u32 = DATA + 0x400;
        const BE_LIB: u32 = DATA + 0x1000;
        const BE_IFACE: u32 = DATA + 0x1100;
        const BE_VT: u32 = DATA + 0x1200;
        check(
            ee::s10::f_003012f0__generated,
            wscript_fn_mission_command,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_12f0, 0);
                elf_image(c);
                c.pc = 0x0030_12f0;
                let gp = GAME_GP;
                // The static description (id, value, reserved) and its flag.
                c.bus.write32(
                    OC324_MISSION_DESC,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 64
                    },
                );
                c.bus.write32(OC324_MISSION_DESC + 4, 5 + seed as u32 % 7);
                c.bus.write32(OC324_MISSION_DESC + 8, r.next_u32());
                c.bus
                    .write32(gp + 0x25c8, if seed % 3 == 0 { 0 } else { 1 });
                // The per-slot types of the argument expansion.
                for i in 0..4u32 {
                    let ty = if (i + seed as u32) % 3 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(gp + 0xb80 + 4 * i, ty);
                }
                // The script manager and the back end.
                c.bus.write32(gp + 0xce8, MAN);
                c.bus.write32(gp - 0x1f04, BE_LIB);
                c.bus.write32(gp - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VT);
                c.bus.write16(BE_VT + 8, 0);
                c.bus.write32(BE_VT + 0xc, FAKE + 0x80);
                c.bus.write8(0x0053_5c10, (r.next_u32() % 3) as u8);
                // Entries only touch slots 2 and 3: the id and value words
                // stay as described.
                let n = (seed % 4) as i32 - 1;
                for i in 0..3u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, 2 + i % 2);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    /// A description block with ids of -1 and of a table entry, an object
    /// with and without a modifier node, over the game image and a test
    /// heap (the constructor allocates).
    #[test]
    fn world_obj_factory_mesh_anim_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::machine;
        const RET: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const INST: u32 = DATA + 0x8000;
        const INST_HDR: u32 = DATA + 0x8100;
        const TABLE: u32 = DATA + 0x8200;
        const ENTRY: u32 = DATA + 0x8300;
        const SLOTS: u32 = DATA + 0x8400;
        const OBJECT: u32 = DATA + 0x8800;
        check(
            ee::s10::f_002fcff0__generated,
            world_obj_factory_mesh_anim,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_cff0, 0);
                elf_image(c);
                crate::game::bx::bxstring::tests::heap(c);
                c.pc = 0x002f_cff0;
                let gp = GAME_GP;
                // The static description and its flag.
                c.bus.write32(OC324_MESH_ANIM_DESC, r.next_u32());
                c.bus.write32(OC324_MESH_ANIM_DESC + 4, seed as u32 % 8);
                for i in 2..14u32 {
                    c.bus.write32(OC324_MESH_ANIM_DESC + 4 * i, r.f32(10.0));
                }
                c.bus
                    .write32(gp + OC324_MESH_ANIM_INIT, if seed % 3 == 0 { 0 } else { 1 });
                // The per-slot types of the argument expansion.
                for i in 0..16u32 {
                    let ty = if (i + seed as u32) % 4 == 0 {
                        2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_6060 + 4 * i, ty);
                }
                // The instance tables the id resolves through.
                c.bus.write32(gp + 0x16c8, INST);
                c.bus.write32(INST, INST_HDR);
                c.bus.write32(INST_HDR + 8, TABLE);
                for i in 0..4u32 {
                    c.bus.write32(TABLE + 4 * i, ENTRY);
                }
                c.bus.write32(ENTRY + 0x1c, SLOTS);
                for i in 0..8u32 {
                    let word = match (seed + i) % 3 {
                        0 => 0,
                        _ => (OBJECT >> 2) << 8 | (r.next_u32() & 0xff),
                    };
                    c.bus.write32(SLOTS + 4 * i, word);
                }
                // The object the modifier attaches to: no node (skip the
                // active-node test) or a node that already is a type-9
                // modifier (the test answers 1, so it is skipped).
                let node = DATA + 0x8900;
                let node_vt = DATA + 0x8a00;
                c.bus
                    .write32(OBJECT + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(OBJECT + 8, 0);
                c.bus.write32(node, 9);
                c.bus.write32(node + 0xc, node_vt);
                c.bus.write16(node_vt + 0x50, 0);
                c.bus.write32(node_vt + 0x54, 0x002a_fa58);
                c.bus.write32(gp + 0xce8, DATA + 0x8b00);
                // The argument block: id (or -1) and model handle.
                let n = (seed % 4) as i32 - 1;
                for i in 0..4u32 {
                    let e = ENTRIES + 0x10 * i;
                    c.bus.write32(e, 14 + i % 2);
                    c.bus.write32(e + 4, r.next_u32() % 100);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = sx(RET);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(ENTRIES);
            },
        );
    }

    pub(crate) const RET_ATTACH: u32 = 3;

    oc327_hook!(
        oc327_fake_attach,
        OC327_PREV_ATTACH,
        0x002f_ae38,
        RET_ATTACH,
        ee::s10::f_002fae38__generated
    );

    oc327_hook!(
        oc327_fake_base_ctor,
        OC327_PREV_BASE_CTOR,
        0x0034_d9b0,
        RET_BASE_CTOR,
        ee::s11::f_0034d9b0__generated
    );

    /// `AnimTeeter`'s create path: the allocator, `checkActiveNode` and the
    /// animated object's base constructor are faked.
    pub(crate) fn oc327_install_teeter() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc327_fake_attach,
            &OC327_PREV_ATTACH,
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034D9B0),
            oc327_fake_base_ctor,
            &OC327_PREV_BASE_CTOR,
        );
        oc327_install_alloc();
    }

    /// `WorldObjFactory_AnimTeeter`: the template init on the first call, the
    /// entry overlay (type match, type 2 as a float, plain words), both id
    /// paths, and the already-attached vs create branches (faked ctor).
    #[test]
    fn world_obj_factory_anim_teeter_matches() {
        oc327_install_teeter();
        covered(
            ee::s10::f_002fb498__generated,
            world_obj_factory_anim_teeter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002f_b498, 4);
                c.pc = 0x002f_b498;
                c.bus.write32(OC327_CTRL, OC327_MAGIC);
                c.bus.write32(OC327_RET + 4 * RET_ATTACH, (seed % 2) as u32);
                c.bus.write32(OC327_RET + 4 * RET_ALLOC, DATA + 0x8_0000);
                c.bus.write32(OC327_RET + 4 * RET_BASE_CTOR, 0);
                let gp = GAME_GP;
                c.g[GP][0] = gp as u64;
                // The one-time template flag (skipped when set) and the two
                // `.lit4` constants the init copies.
                let fresh = seed % 3 == 0;
                c.bus.write32(gp + 0x24d8, if fresh { 0 } else { 1 });
                c.bus.write32(gp.wrapping_sub(0x339c), 0x3f19_999a); // 0.6f
                c.bus.write32(gp.wrapping_sub(0x3398), 0x3ca3_d70a); // 0.02f
                                                                     // The entry type table (0x445ed0).
                for i in 0..12u32 {
                    c.bus.write32(0x0044_5ed0 + i * 4, (i * 7 + seed) % 4);
                }
                let template = 0x004f_b400;
                let id = 0x21 + (seed % 7);
                let word = match seed % 3 {
                    0 => 0,
                    1 => 0x100,
                    _ => 0x1_0000,
                };
                if !fresh {
                    c.bus.write32(template, id);
                    for i in 1..12u32 {
                        c.bus.write32(template + i * 4, r.next_u32());
                    }
                }
                // id != -1: `(*(u32**)(s + 0x1c))[id >> 8] >> 8`, a reference.
                let holder = DATA + 0x1000;
                let slots = DATA + 0x1100;
                let array = DATA + 0x1200;
                let node = DATA + 0x1400;
                c.bus.write32(gp + 0x16c8, holder);
                c.bus.write32(holder, slots);
                c.bus.write32(slots + 8, array);
                c.bus.write32(array + (id & 0xff) * 4, node);
                c.bus.write32(node + 0x1c, DATA + 0x1500);
                c.bus.write32(DATA + 0x1500 + (id >> 8) * 4, word);
                // obj = (i << 2); its +0xc word is the node the factory probes.
                let obj = (word >> 8) << 2;
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                // id == -1: the world's "current object" at +0x290.
                c.bus.write32(gp + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1290, DATA + 0x1600);
                c.bus.write32(DATA + 0x160c, node);
                c.bus.write32(DATA + 0x1600 + 0x80, 0);
                // `0x3421a0` divides by these globals.
                c.bus.write32(gp + 0x2a74, DATA + 0x1800);
                c.bus.write32(DATA + 0x1810, 0x40_0000);
                for i in 0..4u32 {
                    // The intersect info the ctor reads (+8, +0x24, +4, +0xc..).
                    c.bus.write32(node + 8 + i * 4, r.next_u32());
                }
                // The entries: idx 0..11, val, type.
                let e = DATA + 0x3000;
                let n = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    _ => (seed % 6) as u32,
                };
                for i in 0..12u32 {
                    let ent = e + i * 16;
                    c.bus.write32(ent, i % 12);
                    c.bus.write32(ent + 4, r.next_u32());
                    c.bus.write32(ent + 0xc, (seed + i) % 3);
                }
                let out = DATA + 0x7000;
                for i in 0..3u32 {
                    c.bus.write32(out + i * 4, r.next_u32());
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(e);
                c.g[A3][0] = 0;
            },
        );
    }

    // -- WScriptFn_AddHaloModifier / AddRailModifier ----------------------

    /// The record template guard, the id tables at `gp+0x16c8` / `gp+0xce8`,
    /// the object → node → vtable chain, and the script parameters both
    /// modifier ports consume. `types` is the parameter type table.
    /// Modes: 0 the record keeps id -1, 1 the id resolves to nothing, 2 the
    /// object has no node, 3 the node's halo/rail interface answers `0`.
    pub(crate) fn modifier_fixture(c: &mut Cpu, seed: u32, types: u32) {
        let params = DATA + 0x1000;
        let node = DATA + 0x3000;
        let vt = DATA + 0x3100;
        let obj = DATA + 0x3200;
        let holder = DATA + 0x3400;
        let slots = DATA + 0x3500;
        let second = DATA + 0x3600;
        let id = (2u32 << 8) | 1;
        let mode = (seed / 2) % 4;
        let inited = (seed >> 1) & 1;
        let n = if mode == 0 { (seed / 8) % 2 } else { 3 };

        c.bus.write32(GAME_GP + 0x2764, inited);
        c.bus.write32(GAME_GP + 0x2540, inited);
        // Type table: index 0 integer, index 1 float conversion, index 2
        // matching integer.
        for (i, t) in [0u32, 2, 2, 0, 0, 0, 0, 0].iter().enumerate() {
            c.bus.write32(types + 4 * i as u32, *t);
        }
        if n > 0 {
            if mode == 0 {
                // A parameter that leaves the record's id alone.
                c.bus.write32(params, 3);
                c.bus.write32(params + 4, 0x4142_0000);
                c.bus.write32(params + 0xc, 0);
            } else {
                c.bus.write32(params, 0); // idx 0: the object id
                c.bus.write32(params + 4, id);
                c.bus.write32(params + 0xc, 3);
            }
        }
        if n > 1 {
            c.bus.write32(params + 0x10, 1); // idx 1: a float
            c.bus.write32(params + 0x14, 0x4120_0000);
            c.bus.write32(params + 0x1c, 3);
        }
        if n > 2 {
            c.bus.write32(params + 0x20, 2); // idx 2: an integer
            c.bus.write32(params + 0x24, 0x1122_3344);
            c.bus.write32(params + 0x2c, 2);
        }
        // The "current object" for id -1: *(*(gp+0xce8)+0x290).
        let p = DATA + 0x3700;
        c.bus.write32(GAME_GP + 0xce8, p);
        c.bus.write32(p + 0x290, DATA + 0x3780);
        // The id table: *D_004A47B8 -> holder -> slots -> slot -> second.
        c.bus.write32(GAME_GP + 0x16c8, holder);
        c.bus.write32(holder, 0);
        c.bus.write32(holder + 8, slots);
        c.bus.write32(slots, 0);
        c.bus
            .write32(slots + 4, if mode >= 2 { slots + 0x50 } else { 0 });
        c.bus.write32(slots + 0x50 + 0x1c, second);
        c.bus.write32(second + 2 * 4, (obj >> 2) << 8);
        c.bus.write32(obj + 0xc, if mode >= 3 { node } else { 0 });
        c.bus.write32(node + 0xc, vt);
        c.bus.write16(vt + 0x80, 0);
        // A real leaf that answers 0, so the add-modifier call is skipped.
        c.bus.write32(vt + 0x84, 0x0014_06c0);
        c.g[A0][0] = sx(DATA + 0x200);
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(params);
    }

    #[test]
    fn wscript_fn_add_halo_modifier_matches() {
        check(
            ee::s10::f_003057c0__generated,
            wscript_fn_add_halo_modifier,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x30_57c0, 0);
                c.pc = 0x0030_57c0;
                modifier_fixture(c, seed, 0x0044_0000 + HALO_PARAM_TYPES_OFF);
            },
        );
    }

    #[test]
    fn wscript_fn_add_rail_modifier_matches() {
        check(
            ee::s10::f_002ff1c8__generated,
            wscript_fn_add_rail_modifier,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f_f1c8, 0);
                c.pc = 0x002f_f1c8;
                modifier_fixture(c, seed, 0x0044_0000 + RAIL_PARAM_TYPES_OFF);
            },
        );
    }

    /// WorldObjFactory_ParticleNode: template initialised or not; 0-3
    /// params (or a negative count) with matching / mismatching types
    /// (int to float conversions); the default object or one from the
    /// object table (missing slot, empty entry, found), with and without a
    /// modifier that accepts (0), frees (> 0) or keeps (< 0) the node.
    /// Gated fakes for alloc, delete, the particle-node ctor and
    /// AttachModifier (sign-extended result); real memcpy.
    #[test]
    fn world_obj_factory_particle_node_matches() {
        use crate::difftest::fakes::{fake, fall_back, hook};
        use crate::game::object::movenode::tests::{oc338_fake, OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        use std::sync::atomic::AtomicUsize;
        const BASE: u32 = DATA + 0x20_0000;
        const PARAMS: u32 = BASE;
        const RESULT: u32 = BASE + 0x100;
        const ATTACH_RESULT: u32 = BASE + 0x120;
        const HOLDER: u32 = BASE + 0x400;
        const TABLE_PTR: u32 = BASE + 0x800;
        const TABLE: u32 = BASE + 0x810;
        const SLOTS: u32 = BASE + 0x900;
        const SLOT: u32 = BASE + 0xa00;
        const ENTRIES: u32 = BASE + 0xb00;
        const OBJ: u32 = BASE + 0x1_0000;
        oc338_fake!(
            oc374_fake_wofpn_alloc,
            OC374_PREV_WOFPN_ALLOC,
            0x0031_7d70,
            ee::s10::f_00317d70__generated
        );
        oc338_fake!(
            oc374_fake_wofpn_delete,
            OC374_PREV_WOFPN_DELETE,
            0x0031_7e50,
            ee::s10::f_00317e50__generated
        );
        oc338_fake!(
            oc374_fake_wofpn_ctor,
            OC374_PREV_WOFPN_CTOR,
            0x0035_78a8,
            ee::s11::f_003578a8__generated
        );
        static OC374_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);
        fn oc374_fake_attach_modifier(c: &mut Cpu) {
            if c.bus.read32(OC338_CTRL) != OC338_MAGIC {
                return fall_back(c, &OC374_PREV_ATTACH, ee::s10::f_002fae38__generated);
            }
            let ra = c.g[RA][0] as u32;
            fake(c, 0x002f_ae38, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(ATTACH_RESULT));
            }
        }
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc374_fake_wofpn_alloc,
            &OC374_PREV_WOFPN_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc374_fake_wofpn_delete,
            &OC374_PREV_WOFPN_DELETE,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003578A8),
            oc374_fake_wofpn_ctor,
            &OC374_PREV_WOFPN_CTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc374_fake_attach_modifier,
            &OC374_PREV_ATTACH,
        );
        covered(
            ee::s10::f_002fd420__generated,
            world_obj_factory_particle_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_d420, 6);
                c.pc = 0x002f_d420;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let gp = r32(c, GP);
                c.g[A0][0] = sx(RESULT);
                for i in 0..3 {
                    c.bus.write32(WOFPN_RESULT + 4 * i, r.next_u32());
                }
                let initialised = seed % 2;
                c.bus.write32(gp + WOFPN_INIT_GUARD, initialised);
                for off in (0..0xe0).step_by(4) {
                    let f = r.f32(100.0);
                    c.bus.write32(WOFPN_TEMPLATE + off, f);
                }
                for i in 0..56 {
                    c.bus.write32(WOFPN_PARAM_TYPES + 4 * i, r.next_u32() % 4);
                }
                // The object: default (-1) or table entry (id >> 8, slot).
                let which = seed / 2 % 5;
                let id: u32 = match which {
                    0 => u32::MAX,
                    1 => 3,          // slot 3 is empty
                    2 => 2 << 8 | 1, // entry 2 is empty
                    _ => 1 << 8 | 1,
                };
                if initialised == 1 {
                    c.bus.write32(WOFPN_TEMPLATE + WOFPN_NODE_OBJECT, id);
                }
                let n = [0u32, 1, 2, 3, 0xffff_ffff][(seed / 10 % 5) as usize];
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
                for i in 0..3 {
                    let p = PARAMS + 0x10 * i;
                    let idx = if i == 0 && initialised == 0 {
                        54
                    } else {
                        r.next_u32() % 56
                    };
                    c.bus.write32(p, idx);
                    let value = if idx == 54 { id } else { r.next_u32() };
                    c.bus.write32(p + 4, value);
                    c.bus.write32(p + 8, r.next_u32());
                    let ty = if idx == 54 {
                        c.bus.read32(WOFPN_PARAM_TYPES + 4 * 54)
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(p + 0xc, ty);
                }
                let obj = if seed % 7 == 0 { 0 } else { OBJ };
                c.bus.write32(gp + WOFPN_DEFAULT_HOLDER, HOLDER);
                c.bus.write32(HOLDER + 0x290, obj);
                c.bus.write32(gp + WOFPN_OBJECT_TABLE, TABLE_PTR);
                c.bus.write32(TABLE_PTR, TABLE);
                c.bus.write32(TABLE + 8, SLOTS);
                for i in 0..4 {
                    c.bus.write32(SLOTS + 4 * i, if i == 3 { 0 } else { SLOT });
                }
                c.bus.write32(SLOT + 0x1c, ENTRIES);
                c.bus
                    .write32(ENTRIES + 4, (OBJ >> 2) << 8 | (r.next_u32() & 0xff));
                c.bus.write32(ENTRIES + 8, r.next_u32() & 0xff);
                let modifier = if seed % 3 == 0 { 0 } else { DATA + 0x30_0000 };
                c.bus.write32(OBJ + 0xc, modifier);
                let attach = [0u32, 0, 1, 0xffff_ffff][(seed / 3 % 4) as usize];
                c.bus.write32(ATTACH_RESULT, attach);
            },
        );
    }

    use crate::difftest::fakes::vtable;

    /// WScriptFn_GetObjectSlot30: initialised or not, -1..3 argument entries
    /// (raw and float stores), the world object (id -1), a found object with
    /// or without a component (fake slot 0xf0), an unknown descriptor or
    /// entry.
    #[test]
    fn wscript_fn_get_object_slot30_matches() {
        check(
            ee::s10::f_00301680__generated,
            wscript_fn_get_object_slot30,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x301680, 1);
                c.pc = 0x0030_1680;
                let gp = GAME_GP;
                let ret = DATA + 0x100;
                let entries = DATA + 0x200;
                let x = DATA + 0x2300;
                let base = DATA + 0x2400;
                let high = DATA + 0x2800;
                let desc = DATA + 0x2c00;
                let world = DATA + 0x2e00;
                let target = DATA + 0x3000;
                let comp = DATA + 0x3100;
                c.g[A0][0] = sx(ret);
                let n = [-1i64, 0, 1, 2, 3][(seed % 5) as usize];
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
                for i in 0..4u32 {
                    c.bus.write32(gp + WSCRIPT_GOS30_TYPES + 4 * i, i % 3);
                }
                c.bus.write32(gp + WSCRIPT_GOS30_INIT, (seed / 5) % 2);
                c.bus.write8(gp + WSCRIPT_GOS30_TAG, r.next_u32() as u8);
                let id = match (seed / 2) % 5 {
                    0 => 0xffff_ffffu32, // the world object
                    1 => 0x0000_0101,    // lo=1, hi=1: found
                    2 => 0x0000_0103,    // lo=3: no such descriptor
                    3 => 0x0000_0201,    // hi=2: an empty entry
                    _ => 0x0000_0101,
                };
                c.bus.write32(gp + WSCRIPT_GOS30_IDS, id);
                c.bus.write32(gp + WSCRIPT_GOS30_IDS + 4, r.next_u32());
                for i in 0..4u32 {
                    let e = entries + 16 * i;
                    c.bus.write32(e, (seed >> (2 * i)) % 2);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, (seed >> (3 + i)) % 4);
                }
                c.bus.write32(gp + WSCRIPT_GOS30_TABLE, x);
                c.bus.write32(x, x + 0x40);
                c.bus.write32(x + 0x48, base);
                c.bus.write32(gp + WSCRIPT_GOS30_WORLD, world);
                c.bus
                    .write32(world + 0x290, if seed % 7 == 0 { 0 } else { target });
                for lo in 0..4u32 {
                    c.bus
                        .write32(base + 4 * lo, if lo == 1 || lo == 2 { desc } else { 0 });
                }
                c.bus.write32(desc + 0x1c, high);
                for hi in 0..4u32 {
                    c.bus
                        .write32(high + 4 * hi, if hi == 1 { (target >> 2) << 8 } else { 0 });
                }
                c.bus
                    .write32(target + 0xc, if seed % 3 == 0 { 0 } else { comp });
                let vt = vtable(c, &mut r, &[0xf0]);
                c.bus.write32(comp + 0xc, vt);
            },
        );
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::ai::rider::tests::w32_2 as w32;
    pub(crate) use crate::game::world::worldview::tests::oc410_machine;
    pub(crate) use crate::game::world::worldview::tests::oc410_result;

    pub(crate) use crate::game::ai::ai::tests::oc414_env;

    /// `WScriptFn_CreateFlag`: static template init, the override loop (raw
    /// and float-converted), the object lookup and the node allocation.
    #[test]
    fn w_script_fn_create_flag_matches() {
        const ENTRIES: u32 = DATA + 0x1000;
        const WORLD_CELL: u32 = DATA + 0x2000;
        const WORLD_1: u32 = DATA + 0x2100;
        const WORLD_ARR: u32 = DATA + 0x2200;
        const SLOT_OBJ: u32 = DATA + 0x2300;
        const SLOT_TAB: u32 = DATA + 0x2400;
        const DEFAULT_CELL: u32 = DATA + 0x2500;
        const OBJ: u32 = DATA + 0x3000;
        const GP_FIRST_INIT: u32 = crate::difftest::GAME_GP + 0x2500;
        check(
            ee::s10::f_002fc9c8__generated,
            w_script_fn_create_flag,
            400,
            &|c, seed| {
                let mut r = oc410_machine(c, seed, 0x002f_c9c8, 5);
                c.pc = 0x002f_c9c8;
                c.g[A0][0] = (DATA + 0x500) as u64;
                let n = seed % 4;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = ENTRIES as u64;
                w32(c, GP_FIRST_INIT, (seed % 2 == 0) as u32);
                for idx in 0..9u32 {
                    w32(
                        c,
                        0x0044_5fc0 + 4 * idx,
                        if idx == 0 { 0 } else { r.next_u32() % 4 },
                    );
                }
                let (lo, hi) = (seed % 3, seed % 2);
                for i in 0..n {
                    let e = ENTRIES + 0x10 * i;
                    if i == 0 {
                        w32(c, e, 0);
                        w32(c, e + 4, (hi << 8) | lo);
                        w32(c, e + 0xc, 0);
                    } else {
                        w32(c, e, 1 + r.next_u32() % 8);
                        w32(c, e + 4, r.next_u32() % 1000);
                        w32(c, e + 0xc, r.next_u32() % 4);
                    }
                }
                w32(c, crate::difftest::GAME_GP + 0x16c8, WORLD_CELL);
                w32(c, WORLD_CELL, WORLD_1);
                w32(c, WORLD_1 + 8, WORLD_ARR);
                for i in 0..3 {
                    w32(
                        c,
                        WORLD_ARR + 4 * i,
                        if seed % 5 == 2 && i == lo {
                            0
                        } else {
                            SLOT_OBJ
                        },
                    );
                }
                w32(c, SLOT_OBJ + 0x1c, SLOT_TAB);
                for i in 0..2 {
                    let x = if seed % 9 == 4 {
                        0
                    } else {
                        ((OBJ >> 2) << 8) | (r.next_u32() & 0xff)
                    };
                    w32(c, SLOT_TAB + 4 * i, x);
                }
                w32(c, crate::difftest::GAME_GP + 0xce8, DEFAULT_CELL);
                w32(c, DEFAULT_CELL + 0x290, if seed % 6 == 5 { 0 } else { OBJ });
                w32(c, OBJ + 0xc, if seed % 4 == 3 { 0 } else { DATA + 0x3400 });
                w32(c, oc410_result(30), (seed % 3 == 0) as u32);
                w32(c, oc410_result(12), DATA + 0x3800);
                for k in 0..3 {
                    w32(c, 0x004c_9098 + 4 * k, r.next_u32());
                }
            },
        );
    }

    /// `WScriptFn_IsEventType`: first call (the record defaults are set) and
    /// later ones, 0..3 arguments whose type matches the table or not
    /// (matching: integer store; table type 2: float conversion).
    #[test]
    fn w_script_fn_is_event_type_matches() {
        check(
            ee::s10::f_00302778__generated,
            w_script_fn_is_event_type,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x302778, 0);
                c.pc = 0x0030_2778;
                oc414_env(c);
                let gp = GAME_GP;
                c.bus.write32(gp + 0x2660, (seed % 2) as u32);
                c.bus.write8(gp + 0x265c, r.next_u32() as u8);
                for i in 0..4u32 {
                    c.bus.write32(gp + 0xbe8 + 4 * i, 2 + r.next_u32() % 2);
                }
                let args = DATA + 0x200;
                let n = seed % 4;
                for i in 0..n {
                    let rec = args + 16 * i;
                    c.bus.write32(rec, r.next_u32() % 2);
                    c.bus.write32(rec + 4, r.next_u32() % 100);
                    c.bus.write32(rec + 0xc, 2 + r.next_u32() % 2);
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(args);
            },
        );
    }

    /// Control block of the gated fakes: the gate word, the call counter, the
    /// call index that unwinds, forced results (`+0x10 + 4 * id`), forced
    /// `f0` results (`+0x80 + 4 * id`) and the call log (`+0x100`).
    pub(crate) const OC417_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC417_MAGIC: u32 = 0x0c41_7a17;

    /// A gated fake callee: records its arguments, returns the forced result
    /// of its id, and unwinds at the chosen call index.
    pub(crate) fn oc417_fake(c: &mut Cpu, id: u32) {
        let n = c.bus.read32(OC417_CTRL + 4);
        c.bus.write32(OC417_CTRL + 4, n.wrapping_add(1));
        let p = OC417_CTRL + 0x100 + (n & 0x3f) * 0x40;
        c.bus.write32(p, id);
        c.bus.write32(p + 4, c.pc);
        for i in 0..4usize {
            let v = c.g[A0 + i][0];
            c.bus.write64(p + 8 + 8 * i as u32, v);
        }
        let (f12, f13) = (c.f[12], c.f[13]);
        c.bus.write32(p + 0x28, f12);
        c.bus.write32(p + 0x2c, f13);
        let (ra, spv, cyc) = (
            c.g[RA][0] as u32,
            c.g[ssx3_ee_rt::cpu::reg::SP][0] as u32,
            c.cycles,
        );
        c.bus.write32(p + 0x30, ra);
        c.bus.write32(p + 0x34, spv);
        c.bus.write64(p + 0x38, cyc);
        let forced = c.bus.read32(OC417_CTRL + 0x10 + 4 * id);
        c.g[V0][0] = sx(forced);
        c.g[V1][0] = 0x5151_0000 + n as u64;
        c.f[0] = c.bus.read32(OC417_CTRL + 0x80 + 4 * id);
        c.cycles += 5;
        if n == c.bus.read32(OC417_CTRL + 8) {
            c.pc = 0xdead_0000 | n << 4;
        } else {
            c.pc = c.g[RA][0] as u32;
        }
    }

    macro_rules! oc417_gated {
        ($name:ident, $prev:ident, $id:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC417_CTRL) == OC417_MAGIC {
                    return oc417_fake(c, $id);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    // Fake ids (index into the forced-result table).
    pub(crate) const FK_BE_INSTANCE: u32 = 0;

    pub(crate) const FK_BE_INTERFACE: u32 = 1;

    pub(crate) const FK_READ_U32: u32 = 2;

    pub(crate) const FK_NEW_NAMED: u32 = 3;

    pub(crate) const FK_BOLT_COUNT: u32 = 4;

    pub(crate) const FK_BOLTS: u32 = 5;

    pub(crate) const FK_HEAP_FREE: u32 = 6;

    pub(crate) const FK_QUEUE_SECTION: u32 = 7;

    pub(crate) const FK_ADD_MAGNET: u32 = 8;

    pub(crate) const FK_LUNO_DTOR: u32 = 9;

    pub(crate) const FK_LUNO_ASSIGN: u32 = 10;

    pub(crate) const FK_MEM_ALLOC: u32 = 11;

    pub(crate) const FK_BE_TABLE: u32 = 12;

    pub(crate) const FK_BE_DB_ENTRY: u32 = 13;

    pub(crate) const FK_GET_CHARACTER: u32 = 14;

    pub(crate) const FK_SET_SUB_STATE: u32 = 15;

    pub(crate) const FK_SET_MAIN_STATE: u32 = 16;

    pub(crate) const FK_RESET_FX: u32 = 17;

    pub(crate) const FK_BOOST_INPUT: u32 = 18;

    pub(crate) const FK_BUILD_MATRIX: u32 = 19;

    pub(crate) const FK_QUAT_EULER: u32 = 20;

    pub(crate) const FK_ANIM_LAYER_A: u32 = 21;

    pub(crate) const FK_ANIM_LAYER_B: u32 = 22;

    pub(crate) const FK_ANIM_RESET: u32 = 23;

    pub(crate) const FK_ANIM_PLAY: u32 = 24;

    pub(crate) const FK_SINCOS: u32 = 25;

    oc417_gated!(
        fake_be_instance,
        PREV_BE_INSTANCE,
        FK_BE_INSTANCE,
        ee::s01::f_0014dc80__generated
    );

    oc417_gated!(
        fake_be_interface,
        PREV_BE_INTERFACE,
        FK_BE_INTERFACE,
        ee::s01::f_0014dd58__generated
    );

    oc417_gated!(
        fake_read_u32,
        PREV_READ_U32,
        FK_READ_U32,
        ee::s09::f_002d2510__generated
    );

    oc417_gated!(
        fake_new_named,
        PREV_NEW_NAMED,
        FK_NEW_NAMED,
        ee::s10::f_00317e30__generated
    );

    oc417_gated!(
        fake_bolt_count,
        PREV_BOLT_COUNT,
        FK_BOLT_COUNT,
        ee::s01::f_0014d988__generated
    );

    oc417_gated!(
        fake_bolts,
        PREV_BOLTS,
        FK_BOLTS,
        ee::s01::f_0014d998__generated
    );

    oc417_gated!(
        fake_heap_free,
        PREV_HEAP_FREE,
        FK_HEAP_FREE,
        ee::s07::f_002723a8__generated
    );

    oc417_gated!(
        fake_queue_section,
        PREV_QUEUE_SECTION,
        FK_QUEUE_SECTION,
        ee::s05::f_0022e298__generated
    );

    oc417_gated!(
        fake_add_magnet,
        PREV_ADD_MAGNET,
        FK_ADD_MAGNET,
        ee::s11::f_003559f8__generated
    );

    oc417_gated!(
        fake_luno_dtor,
        PREV_LUNO_DTOR,
        FK_LUNO_DTOR,
        ee::s05::f_00224df0__generated
    );

    oc417_gated!(
        fake_luno_assign,
        PREV_LUNO_ASSIGN,
        FK_LUNO_ASSIGN,
        ee::s05::f_00225068__generated
    );

    oc417_gated!(
        fake_mem_alloc,
        PREV_MEM_ALLOC,
        FK_MEM_ALLOC,
        ee::s10::f_00317d70__generated
    );

    oc417_gated!(
        fake_be_table,
        PREV_BE_TABLE,
        FK_BE_TABLE,
        ee::s01::f_0014bdb8__generated
    );

    oc417_gated!(
        fake_be_db_entry,
        PREV_BE_DB_ENTRY,
        FK_BE_DB_ENTRY,
        ee::s01::f_0014ad50__generated
    );

    oc417_gated!(
        fake_get_character,
        PREV_GET_CHARACTER,
        FK_GET_CHARACTER,
        ee::s00::f_00123128__generated
    );

    oc417_gated!(
        fake_set_sub_state,
        PREV_SET_SUB_STATE,
        FK_SET_SUB_STATE,
        ee::s00::f_0011fec8__generated
    );

    oc417_gated!(
        fake_set_main_state,
        PREV_SET_MAIN_STATE,
        FK_SET_MAIN_STATE,
        ee::s00::f_0011fe78__generated
    );

    oc417_gated!(
        fake_reset_fx,
        PREV_RESET_FX,
        FK_RESET_FX,
        ee::s00::f_00111890__generated
    );

    oc417_gated!(
        fake_boost_input,
        PREV_BOOST_INPUT,
        FK_BOOST_INPUT,
        ee::s00::f_00114130__generated
    );

    oc417_gated!(
        fake_build_matrix,
        PREV_BUILD_MATRIX,
        FK_BUILD_MATRIX,
        ee::s00::f_0011e098__generated
    );

    oc417_gated!(
        fake_quat_euler,
        PREV_QUAT_EULER,
        FK_QUAT_EULER,
        ee::s00::f_001241c0__generated
    );

    oc417_gated!(
        fake_anim_layer_a,
        PREV_ANIM_LAYER_A,
        FK_ANIM_LAYER_A,
        ee::s10::f_0030e9e0__generated
    );

    oc417_gated!(
        fake_anim_layer_b,
        PREV_ANIM_LAYER_B,
        FK_ANIM_LAYER_B,
        ee::s10::f_0030ea80__generated
    );

    oc417_gated!(
        fake_anim_reset,
        PREV_ANIM_RESET,
        FK_ANIM_RESET,
        ee::s10::f_00311a50__generated
    );

    oc417_gated!(
        fake_anim_play,
        PREV_ANIM_PLAY,
        FK_ANIM_PLAY,
        ee::s10::f_003128e8__generated
    );

    oc417_gated!(
        fake_sincos,
        PREV_SINCOS,
        FK_SINCOS,
        ee::s10::f_0031be50__generated
    );

    /// Put every fake of this module in its override slot (once per
    /// process; with the gate word clear they run what the slot held).
    pub(crate) fn oc417_hook_fakes() {
        use std::ptr::addr_of_mut;
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu), &AtomicUsize); 26] = [
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                fake_be_instance,
                &PREV_BE_INSTANCE,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                fake_be_interface,
                &PREV_BE_INTERFACE,
            ),
            (
                addr_of_mut!(ee::s09::OVERRIDE_002D2510),
                fake_read_u32,
                &PREV_READ_U32,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00317E30),
                fake_new_named,
                &PREV_NEW_NAMED,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014D988),
                fake_bolt_count,
                &PREV_BOLT_COUNT,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014D998),
                fake_bolts,
                &PREV_BOLTS,
            ),
            (
                addr_of_mut!(ee::s07::OVERRIDE_002723A8),
                fake_heap_free,
                &PREV_HEAP_FREE,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_0022E298),
                fake_queue_section,
                &PREV_QUEUE_SECTION,
            ),
            (
                addr_of_mut!(ee::s11::OVERRIDE_003559F8),
                fake_add_magnet,
                &PREV_ADD_MAGNET,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_00224DF0),
                fake_luno_dtor,
                &PREV_LUNO_DTOR,
            ),
            (
                addr_of_mut!(ee::s05::OVERRIDE_00225068),
                fake_luno_assign,
                &PREV_LUNO_ASSIGN,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00317D70),
                fake_mem_alloc,
                &PREV_MEM_ALLOC,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014BDB8),
                fake_be_table,
                &PREV_BE_TABLE,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_0014AD50),
                fake_be_db_entry,
                &PREV_BE_DB_ENTRY,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00123128),
                fake_get_character,
                &PREV_GET_CHARACTER,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
                fake_set_sub_state,
                &PREV_SET_SUB_STATE,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011FE78),
                fake_set_main_state,
                &PREV_SET_MAIN_STATE,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00111890),
                fake_reset_fx,
                &PREV_RESET_FX,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00114130),
                fake_boost_input,
                &PREV_BOOST_INPUT,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                fake_build_matrix,
                &PREV_BUILD_MATRIX,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_001241C0),
                fake_quat_euler,
                &PREV_QUAT_EULER,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030E9E0),
                fake_anim_layer_a,
                &PREV_ANIM_LAYER_A,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0030EA80),
                fake_anim_layer_b,
                &PREV_ANIM_LAYER_B,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_00311A50),
                fake_anim_reset,
                &PREV_ANIM_RESET,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_003128E8),
                fake_anim_play,
                &PREV_ANIM_PLAY,
            ),
            (
                addr_of_mut!(ee::s10::OVERRIDE_0031BE50),
                fake_sincos,
                &PREV_SINCOS,
            ),
        ];
        for (slot, fake, prev) in slots {
            fakes::hook(slot, fake, prev);
        }
    }

    /// Scrambled machine with a game gp, the fakes' gate open and (one seed
    /// in three, if `calls > 0`) a gated fake call that unwinds. The
    /// generic fakes (`fakes::machine`) take the vtable calls; their own
    /// unwinding is chosen with the same `calls`.
    pub(crate) fn oc417_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc417_hook_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC417_CTRL, OC417_MAGIC);
        let unwind = if calls > 0 && seed % 3 == 1 {
            r.next_u32() % calls
        } else {
            u32::MAX
        };
        c.bus.write32(OC417_CTRL + 8, unwind);
        for id in 0..32 {
            c.bus.write32(OC417_CTRL + 0x10 + 4 * id, 0);
        }
        r
    }

    // ----------------------------------------------------------------------
    // WScriptFn_EventTypeMatches

    /// Event type below and above 8; table entry equal to the current event
    /// byte or not; the BE lookups faked (and unwinding at one of them).
    #[test]
    fn wscript_fn_event_type_matches_matches() {
        check(
            ee::s10::f_00303df8__generated,
            wscript_fn_event_type_matches,
            200,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0002, 2);
                c.pc = 0x0030_3df8;
                let current = r.next_u32() % 4;
                c.bus.write8(0x0053_5c10, current as u8);
                for i in 0..8u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        current
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_65f8 + 4 * i, v);
                }
                c.g[A0][0] = (seed % 12) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // WScriptFn_MultiParticleControl / WScriptFn_AddMagnetModifier

    pub(crate) const WS_WORLD: u32 = DATA + 0x2000;

    pub(crate) const WS_CACHE_PTR: u32 = DATA + 0x2800;

    pub(crate) const WS_CACHE: u32 = DATA + 0x2900;

    pub(crate) const WS_SLOTS: u32 = DATA + 0x2a00;

    pub(crate) const WS_SLOT: u32 = DATA + 0x2b00;

    pub(crate) const WS_ENTRIES: u32 = DATA + 0x2c00;

    pub(crate) const WS_OBJ: u32 = DATA + 0x4000;

    pub(crate) const WS_OBJ2: u32 = DATA + 0x4100;

    pub(crate) const WS_NODE: u32 = DATA + 0x4400;

    pub(crate) const WS_NODE_VT: u32 = DATA + 0x4600;

    pub(crate) const WS_MGR: u32 = DATA + 0x5000;

    pub(crate) const WS_ARGS: u32 = DATA + 0x1000;

    pub(crate) const WS_RESULT: u32 = DATA + 0x1800;

    /// The world, its object cache and two objects ids can name, written
    /// the way both functions look them up: `-1` is the current object
    /// (`world+0x290`), `slot | index << 8` goes through the cache's slot
    /// table.
    pub(crate) fn oc417_wscript_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(GAME_GP + 0xce8, WS_WORLD);
        c.bus
            .write32(WS_WORLD + 0x290, if seed % 5 == 0 { 0 } else { WS_OBJ2 });
        c.bus.write32(GAME_GP + 0x16c8, WS_CACHE_PTR);
        c.bus.write32(WS_CACHE_PTR, WS_CACHE);
        c.bus.write32(WS_CACHE + 8, WS_SLOTS);
        c.bus.write32(WS_SLOTS, WS_SLOT);
        c.bus.write32(WS_SLOTS + 5 * 4, WS_SLOT);
        c.bus.write32(WS_SLOT + 0x1c, WS_ENTRIES);
        c.bus.write32(WS_ENTRIES, (WS_OBJ >> 2) << 8);
        c.bus.write32(
            WS_ENTRIES + 3 * 4,
            if seed % 4 == 0 { 0 } else { (WS_OBJ >> 2) << 8 },
        );
        c.bus.write32(WS_OBJ + 0x78, r.next_u32());
        c.bus.write32(WS_OBJ2 + 0x78, r.next_u32());
        c.bus.write32(GAME_GP + 0xf38, WS_MGR);
        for i in 0..4u32 {
            c.bus.write32(WS_MGR + 4 * i, WS_MGR + 0x400);
        }
    }

    /// The 12-byte default result record both functions copy out.
    pub(crate) fn oc417_wscript_result(c: &mut Cpu, r: &mut Rng) {
        for i in 0..3u32 {
            c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
        }
    }

    /// `n` argument entries (`{index, value, ?, type}`): index 0-3 of the
    /// 4-word record, the id word taken from the ids above.
    pub(crate) fn oc417_wscript_args(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        id_index: u32,
        types: u32,
    ) {
        let n = (seed / 2) % 4;
        for i in 0..n {
            let e = WS_ARGS + 0x10 * i;
            let idx = r.next_u32() % 4;
            c.bus.write32(e, idx);
            let value = if idx == id_index {
                [0xffff_ffffu32, 0x0000, 0x0305, 0x0107, 0x0900][(r.next_u32() % 5) as usize]
            } else if idx == 0 {
                r.next_u32() % 4
            } else {
                r.next_u32() % 3
            };
            c.bus.write32(e + 4, value);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
    }

    /// Both initialisation states of the static record, 0-3 entries (int to
    /// float conversions included), objects found through the current object,
    /// the slot table or not at all, add and remove.
    #[test]
    fn wscript_fn_multi_particle_control_matches() {
        check(
            ee::s10::f_00306cb0__generated,
            wscript_fn_multi_particle_control,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4170_0007, 0);
                c.pc = 0x0030_6cb0;
                oc417_wscript_world(c, &mut r, seed);
                oc417_wscript_result(c, &mut r);
                oc417_wscript_args(c, &mut r, seed, 1, 0x0044_6970);
                let initialised = seed % 3 != 0;
                c.bus.write32(GAME_GP + 0x27c8, initialised as u32);
                if initialised {
                    for i in 0..4u32 {
                        let w = if i == 1 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 3
                        };
                        c.bus.write32(0x004f_c050 + 4 * i, w);
                    }
                }
                c.g[A0][0] = WS_RESULT as u64;
                c.g[A1][0] = ((seed / 2) % 4) as u64;
                c.g[A2][0] = WS_ARGS as u64;
            },
        );
    }

    /// As above for the magnet modifier: the object's move node, its
    /// "accepts magnets" virtual (`0x27c070` reads the node, so the fixture
    /// chooses the answer) and the faked magnet call.
    #[test]
    fn wscript_fn_add_magnet_modifier_matches() {
        check(
            ee::s10::f_00305478__generated,
            wscript_fn_add_magnet_modifier,
            300,
            &|c, seed| {
                let mut r = oc417_machine(c, seed, 0x4170_0008, 4);
                c.pc = 0x0030_5478;
                oc417_wscript_world(c, &mut r, seed);
                oc417_wscript_result(c, &mut r);
                oc417_wscript_args(c, &mut r, seed, 0, 0x0044_66a0);
                for obj in [WS_OBJ, WS_OBJ2] {
                    c.bus
                        .write32(obj + 0xc, if seed % 7 == 0 { 0 } else { WS_NODE });
                }
                c.bus.write32(WS_NODE + 0xc, WS_NODE_VT);
                c.bus.write32(WS_NODE + 0x14, (seed / 3) % 2);
                c.bus.write16(WS_NODE_VT + 0x80, 0);
                c.bus.write32(WS_NODE_VT + 0x84, 0x0027_c070);
                c.bus
                    .write16(WS_NODE_VT + 0xc0, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(WS_NODE_VT + 0xc4, fakes::FAKE);
                c.bus.write32(GAME_GP + 0x2750, (seed % 3 != 0) as u32);
                if seed % 3 != 0 {
                    for i in 0..4u32 {
                        let w = if i == 0 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 5
                        };
                        c.bus.write32(0x004f_bd58 + 4 * i, w);
                    }
                }
                c.g[A0][0] = WS_RESULT as u64;
                c.g[A1][0] = ((seed / 2) % 4) as u64;
                c.g[A2][0] = WS_ARGS as u64;
            },
        );
    }

    use crate::difftest::fakes::FAKE;

    pub(crate) use crate::game::ai::ai::tests::OC420_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC420_MAGIC;
    pub(crate) use crate::game::ai::ai::tests::OC420_RET;

    // ------------------------------------------------------ WScriptFn_RandRange

    /// With and without the first-use static init, 0..3 arguments of
    /// matching and differing kinds (the int->float conversion included);
    /// the random call and the register accessor are fakes, `MEM_copy` is real.
    #[test]
    fn wscript_fn_rand_range_matches() {
        oc420_hook_rand_range();
        oc420_hook_luno_as_float();
        covered(
            ee::s10::f_00303598__generated,
            wscript_fn_rand_range,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_3598, 3);
                c.pc = 0x0030_3598;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 7, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x26c8, (seed % 2) as u32);
                // The static parameter block (already initialised half the time).
                c.bus.write32(0x004f_bc60, r.next_u32());
                c.bus.write32(0x004f_bc64, 0x3f80_0000);
                c.bus.write8(0x004f_bc68, r.next_u32() as u8);
                let kinds = [0u32, 1, 2];
                for k in 0..3 {
                    c.bus
                        .write32(GAME_GP + 0xc30 + 4 * k, kinds[(r.next_u32() % 3) as usize]);
                }
                let args = DATA + 0x2000;
                let n = (seed / 2) % 4;
                for i in 0..n {
                    let e = args + 0x10 * i;
                    let k = r.next_u32() % 3;
                    c.bus.write32(e, k);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 0xc, (r.next_u32() % 3) as u32);
                }
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = if seed % 9 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
            },
        );
    }

    // ----------------------------------------- the WScriptFn_* decoder fixture

    /// The object ids a decoder resolves: `slot | index << 8` for the slots
    /// and indices of `oc420_world_objects`, and -1 (the current object).
    pub(crate) const OC420_IDS: [u32; 8] =
        [0xffff_ffff, 0x000, 0x100, 0x001, 0x101, 0x302, 0x005, 0x200];

    /// World cache, the object records and their move nodes. The cache is
    /// reached through `gp + 0x16c8`; slots 0..3 hold records (slot 2 empty),
    /// whose tables hold `value << 8` with `value << 2` an object address
    /// (zero for some entries); the current object is `OBJ0`. Each object's
    /// word at +0xc is its move node; node vtable slots +0x50/+0x54 and
    /// +0x80/+0x84 are fakes or the zero leaf.
    pub(crate) fn oc420_world_objects(c: &mut Cpu, r: &mut Rng) {
        let (cache_ptr, cache, slots) = (DATA + 0x1100, DATA + 0x1200, DATA + 0x1300);
        c.bus.write32(GAME_GP + 0x16c8, cache_ptr);
        c.bus.write32(cache_ptr, cache);
        c.bus.write32(cache + 8, slots);
        let vt = DATA + 0x2800;
        for s in 0..4u32 {
            let rec = DATA + 0x1400 + 0x40 * s;
            c.bus.write32(slots + 4 * s, if s == 2 { 0 } else { rec });
            let table = DATA + 0x1500 + 0x20 * s;
            c.bus.write32(rec + 0x1c, table);
            for k in 0..4u32 {
                let obj = DATA + 0x2000 + 0x100 * ((s + k) % 4);
                let entry = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    (obj >> 2) << 8
                };
                c.bus.write32(table + 4 * k, entry);
            }
        }
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x1600);
        c.bus.write32(DATA + 0x1600 + 0x290, DATA + 0x2000);
        for j in 0..4u32 {
            let obj = DATA + 0x2000 + 0x100 * j;
            let node = DATA + 0x2400 + 0x100 * j;
            c.bus
                .write32(obj + 0xc, if r.next_u32() % 5 == 0 { 0 } else { node });
            c.bus.write32(node + 0xc, vt);
            c.bus.write32(node + 0x18, r.next_u32());
        }
        c.bus
            .write16(vt + 0x50, [0u16, 8][(r.next_u32() % 2) as usize]);
        c.bus.write32(
            vt + 0x54,
            [FAKE + 0x10, 0x0014_06c0][(r.next_u32() % 2) as usize],
        );
        c.bus
            .write16(vt + 0x80, [0u16, 4][(r.next_u32() % 2) as usize]);
        c.bus.write32(
            vt + 0x84,
            [FAKE + 0x20, 0x0014_06c0][(r.next_u32() % 2) as usize],
        );
    }

    /// `n` argument entries `{index, value, _, kind}` at `args`; the first
    /// one often sets word 0 (the object id) to one of `OC420_IDS`.
    pub(crate) fn oc420_args(c: &mut Cpu, r: &mut Rng, args: u32, n: u32, words: u32, table: u32) {
        for k in 0..words {
            c.bus
                .write32(table + 4 * k, [0u32, 1, 2][(r.next_u32() % 3) as usize]);
        }
        for i in 0..n {
            let e = args + 0x10 * i;
            let idx = if i == 0 && r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() % words
            };
            c.bus.write32(e, idx);
            let val = if idx == 0 {
                OC420_IDS[(r.next_u32() % 8) as usize]
            } else {
                r.next_u32() % 1000
            };
            c.bus.write32(e + 4, val);
            let tv = c.bus.read32(table + 4 * idx);
            c.bus.write32(e + 0xc, tv);
            if r.next_u32() % 3 == 0 {
                c.bus.write32(e + 0xc, r.next_u32() % 3);
            }
        }
    }

    // ------------------------------------------------ WScriptFn_StopObjectLoop

    #[test]
    fn wscript_fn_stop_object_loop_matches() {
        oc420_hook_audio_instance();
        oc420_hook_audio_stop_loop();
        covered(
            ee::s10::f_00300260__generated,
            wscript_fn_stop_object_loop,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_0260, 3);
                c.pc = 0x0030_0260;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 8, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x257c, (seed % 2) as u32);
                oc420_world_objects(c, &mut r);
                // The static parameter block {-1, -1, -1, flag}.
                let d = 0x004f_bb28;
                c.bus.write32(d, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, r.next_u32());
                c.bus.write8(d + 0xc, r.next_u32() as u8);
                let args = DATA + 0x3000;
                let n = (seed / 2) % 4;
                oc420_args(c, &mut r, args, n, 4, 0x0044_6560);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = if seed % 11 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.bus.write32(0x004c_9098, r.next_u32());
                c.bus.write32(0x004c_909c, r.next_u32());
                c.bus.write32(0x004c_90a0, r.next_u32());
            },
        );
    }

    // -------------------------------------------- WScriptFn_AddParentModifierRev

    #[test]
    fn wscript_fn_add_parent_modifier_rev_matches() {
        oc420_hook_add_parent();
        covered(
            ee::s10::f_002fdc60__generated,
            wscript_fn_add_parent_modifier_rev,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_dc60, 5);
                c.pc = 0x002f_dc60;
                c.bus.write32(OC420_CTRL, OC420_MAGIC);
                c.bus.write32(OC420_RET + 4 * 10, DATA + 0x5000);
                c.bus.write32(GAME_GP + 0x2520, (seed % 2) as u32);
                oc420_world_objects(c, &mut r);
                // The static parameter block {-1, -1, 0, 0.0 x3, flag}.
                let d = 0x004f_b758;
                c.bus.write32(d, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 4, OC420_IDS[(r.next_u32() % 8) as usize]);
                c.bus.write32(d + 8, r.next_u32());
                for k in 3..6 {
                    c.bus.write32(d + 4 * k, r.f32(10.0));
                }
                c.bus.write8(d + 0x18, r.next_u32() as u8);
                let args = DATA + 0x3000;
                let n = (seed / 2) % 4;
                oc420_args(c, &mut r, args, n, 6, 0x0044_61c0);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = if seed % 13 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    n as u64
                };
                c.g[A2][0] = sx(args);
                for k in 0..3 {
                    c.bus.write32(DATA + 0x500 + 4 * k, r.next_u32());
                }
                c.bus.write32(0x004c_9098, r.next_u32());
                c.bus.write32(0x004c_909c, r.next_u32());
                c.bus.write32(0x004c_90a0, r.next_u32());
            },
        );
    }

    pub(crate) use crate::game::main::ssxapp::tests::oc429_gp;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    pub(crate) use crate::game::main::ssxapp::tests::OC429_FX;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc428_gate;

    /// `WScriptFn_ObjectSlot36` against the generated body: the first-call
    /// initialisation (flag at `gp + 0x2550`) or a table left in RAM, 0-4
    /// argument entries (int/float slot types), and the target object found
    /// by handle through the two lookup tables, through the global object,
    /// or missing (`EA_MemCopy` real; the object's method is a fake call).
    #[test]
    fn w_script_fn_object_slot36_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_002ff9a8__generated,
            w_script_fn_object_slot36,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff9a8, 1);
                oc428_gate(c);
                let (out, entries) = (DATA + 0x7000, DATA + 0x7100);
                let (ht, h, arr, ent, back) = (
                    DATA + 0x8000,
                    DATA + 0x8100,
                    DATA + 0x8200,
                    DATA + 0x8300,
                    DATA + 0x8400,
                );
                let (global, obj, p, vt) =
                    (DATA + 0x8500, DATA + 0x6000, DATA + 0x6100, DATA + 0x6200);
                let table = 0x004f_baf8;
                let flag = seed % 2;
                c.bus.write32(GAME_GP + 0x2550, flag);
                // The handle in the table: none, found, or a dead slot.
                let handle = [
                    0xffff_ffffu32,
                    0x0000_0102,
                    0x0000_0103,
                    0x0000_0001,
                    0x0000_0201,
                ][((seed / 2) % 5) as usize];
                fill(c, &mut r, table, 8);
                c.bus.write32(table, handle);
                // Slot types of the argument indices 0..3.
                for i in 0..4 {
                    c.bus.write32(0x0044_6540 + 4 * i, 1 + r.next_u32() % 3);
                }
                // Lookup tables: handle -> slot 2 -> generation 1 -> object.
                c.bus.write32(GAME_GP + 0x16c8, ht);
                c.bus.write32(ht, h);
                c.bus.write32(h + 8, arr);
                c.bus.write32(arr + 8, ent);
                c.bus.write32(arr + 4, ent);
                c.bus.write32(arr + 0xc, 0);
                c.bus.write32(ent + 0x1c, back);
                c.bus.write32(back + 4, ((obj >> 2) << 8) | 5);
                c.bus.write32(back + 8, 0x0000_0005);
                c.bus.write32(back + 12, 0);
                c.bus.write32(GAME_GP + 0xce8, global);
                c.bus.write32(global + 0x290, obj);
                c.bus.write32(obj + 0xc, if seed % 7 == 0 { 0 } else { p });
                c.bus.write32(p + 0xc, vt);
                c.bus.write16(vt + 0x120, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x124, FAKE + 0x50);
                // Entries: (index, value, _, type).
                let n = (seed / 3) % 5;
                for k in 0..4u32 {
                    let e = entries + 0x10 * k;
                    let idx = if seed % 5 == 0 && k == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    };
                    c.bus.write32(e, idx);
                    c.bus
                        .write32(e + 4, if idx == 0 { handle } else { r.next_u32() % 100 });
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, 1 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `WScriptFn_AddCollectItems` against the generated body: first-call
    /// initialisation or not, 0-4 entries of int/float slot types (indices
    /// 0-20), with `args[0]` set (the real `EA_MemCopy`d table's first word,
    /// or an entry's value) or still -1; `WScriptMan_AddCollectItems` is a
    /// fake.
    #[test]
    fn w_script_fn_add_collect_items_matches() {
        oc428_install_fakes();
        fakes::covered(
            ee::s10::f_00300f50__generated,
            w_script_fn_add_collect_items,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x300f50, 1);
                oc428_gate(c);
                let (out, entries) = (DATA + 0x7000, DATA + 0x7100);
                c.bus.write32(GAME_GP + 0x25c0, seed % 2);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x8000);
                let table = 0x004f_bb78;
                fill(c, &mut r, table, 0x16);
                if seed % 2 == 0 {
                    c.bus.write32(table, 0xffff_ffff);
                }
                for i in 0..21 {
                    c.bus.write32(0x0044_6570 + 4 * i, 1 + r.next_u32() % 3);
                }
                let n = (seed / 2) % 5;
                for k in 0..4u32 {
                    let e = entries + 0x10 * k;
                    let idx = if seed % 4 == 1 && k == 0 {
                        0
                    } else {
                        r.next_u32() % 21
                    };
                    c.bus.write32(e, idx);
                    c.bus.write32(e + 4, r.next_u32() % 1000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, 1 + r.next_u32() % 3);
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    /// `WScriptFn_RiderCommand`: first call (static defaults written) and
    /// later calls, 0..4 entries (int stores and the int-to-float conversion
    /// for slots of type 2), the rider id defaulted from the global or given.
    /// `EA_MemCopy` runs for real; the rider call is a fake.
    #[test]
    fn wscript_fn_rider_command_matches() {
        covered(
            ee::s10::f_002ff850__generated,
            wscript_fn_rider_command,
            160,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x2ff850, 1, 0x002f_f850);
                let world = OC429_FX + 0x300;
                let x = OC429_FX + 0x340;
                let riders = OC429_FX + 0x700;
                oc429_w(c, oc429_gp(-0x848), world);
                oc429_w(c, world + 0x84, x);
                oc429_w(c, x + 0xc, riders);
                // `riders[idx] + 0x28` is the rider pointer: idx 0..3 at +0x28...
                for i in 0..4u32 {
                    oc429_w(c, riders + 0x28 + 4 * i, OC429_FX + 0x1000 + 0x10 * i);
                }
                // The default record, its flag, the global rider id.
                oc429_w(c, oc429_gp(0x254c), (seed % 3 != 0) as u32);
                fill(c, &mut r, 0x004f_bae8, 4);
                oc429_w(
                    c,
                    0x004f_bae8,
                    if seed % 4 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 4
                    },
                );
                oc429_w(c, oc429_gp(0xce8), OC429_FX + 0x1200);
                oc429_w(c, OC429_FX + 0x1200, r.next_u32() % 4);
                // Slot types.
                // (Slot 0, the rider index, never holds a float: it indexes a table.)
                for k in 0..4u32 {
                    let ty = if k == 0 {
                        [0u32, 1, 3][(r.next_u32() % 3) as usize]
                    } else {
                        r.next_u32() % 4
                    };
                    oc429_w(c, 0x0044_6530 + 4 * k, ty);
                }
                fill(c, &mut r, 0x004c_9098, 3);
                let n = (seed / 3) % 5;
                let ents = OC429_FX + 0x1400;
                for i in 0..n.max(1) {
                    let e = ents + 0x10 * i;
                    oc429_w(c, e, r.next_u32() % 4);
                    oc429_w(c, e + 4, r.next_u32() % 4);
                    oc429_w(c, e + 8, r.next_u32());
                    oc429_w(c, e + 0xc, r.next_u32() % 4);
                }
                c.g[A0][0] = (OC429_FX + 0x1800) as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = ents as u64;
            },
        );
    }

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_gp;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC433_FIX;

    /// `WScriptFn_DefineCollectSet` against the generated body: first use or
    /// not, 0..3 (or a negative count of) entries of the three kinds (matching
    /// value, plain store, float store), the first slot -1 or not.
    #[test]
    fn w_script_fn_define_collect_set_matches() {
        oc433_run(
            ee::s10::f_00300e28__generated,
            w_script_fn_define_collect_set,
            0x0030_0e28,
            2,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (out, entries) = (f + 0x1000, f + 0x1100);
                oc433_gp(c, 0x25bc, seed % 2);
                oc433_gp(c, 0xce8, f + 0x1400);
                let table = [0u32, 2, 3];
                for (i, t) in table.iter().enumerate() {
                    oc433_gp(c, 0xb70 + 4 * i as i32, *t);
                }
                let n = [-1i32, 0, 1, 2, 3, 3][(seed / 2 % 6) as usize];
                for e in 0..3u32 {
                    let idx = if e == 0 {
                        (seed / 12) % 3
                    } else {
                        r.next_u32() % 3
                    };
                    let at = entries + 0x10 * e;
                    oc433_w(c, at, idx);
                    let value = if seed % 5 == 0 && e == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 50
                    };
                    oc433_w(c, at + 4, value);
                    let t = table[idx as usize];
                    oc433_w(c, at + 0xc, if r.next_u32() % 2 == 0 { t } else { t + 1 });
                }
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as i64 as u64;
                c.g[A2][0] = sx(entries);
            },
        );
    }

    pub(crate) use crate::game::sound::soundsys::tests::oc739_gate;
    pub(crate) use crate::game::sound::soundsys::tests::OC739_CTRL;

    use crate::game::ui::uistate::tests as hide;
    use std::ptr::addr_of_mut;

    /// The machine state the counter-style factories read: parameters at
    /// `DATA + 0x1000`, a type table at `table`, the flag at `flag`, the
    /// section table that resolves ids to no instance.
    pub(crate) fn counter_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, table: u32, flag: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA + 0x7000);
        let n = seed % 4;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            c.bus.write32(e, r.next_u32() % 3);
            c.bus.write32(e + 4, r.next_u32());
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..3 {
            c.bus.write32(table + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(flag, seed % 2);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        for i in 0..3 {
            c.bus.write32(DATA + 0x5000 + 4 * i, 0);
        }
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x6000);
        c.bus.write32(DATA + 0x6290, 0);
    }

    #[test]
    fn world_obj_factory_floating_matches() {
        check(
            ee::s10::f_002fbab0__generated,
            world_obj_factory_floating,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fba_b0 ^ seed as u64);
                scramble(c, &mut r);
                counter_fixture(c, &mut r, seed, 0x0044_5f28, GAME_GP + 0x24e4);
            },
        );
    }

    /// Covers the one-time init (flag clear or set), the parameter loop
    /// (int and float stores), the no-instance exits (id -1 and a missing
    /// section entry). The attach and allocation tails need fakes and are
    /// not reached here.
    #[test]
    fn world_obj_factory_counter_matches() {
        check(
            ee::s10::f_002fb8d0__generated,
            world_obj_factory_counter,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fb8_d0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA + 0x7000);
                let n = seed % 4;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(DATA + 0x1000);
                for i in 0..n {
                    let e = DATA + 0x1000 + 16 * i;
                    c.bus.write32(e, r.next_u32() % 3);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 3);
                }
                for i in 0..3 {
                    c.bus.write32(GAME_GP + 0xaf0 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(GAME_GP + 0x24e0, seed % 2);
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(DATA + 0x4008, DATA + 0x5000);
                for i in 0..3 {
                    c.bus.write32(DATA + 0x5000 + 4 * i, 0);
                }
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x6000);
                c.bus.write32(DATA + 0x6290, 0);
            },
        );
    }

    /// The script object every fixture passes as `this`.
    pub(crate) const OC703_OBJ: u32 = DATA + 0x4000;

    /// The script manager (`gp+0xce8` holds its address).
    pub(crate) const OC703_MGR: u32 = DATA + 0x1400;

    /// The current task object behind the manager's `+0x2a4` (WScriptProcess).
    pub(crate) const OC703_PROC: u32 = DATA + 0x2000;

    /// The `ProcessTask` table the process indexes.
    pub(crate) const OC703_TABLE: u32 = DATA + 0x2800;

    /// The world (`gp-0x848` holds its address) and its rider table.
    pub(crate) const OC703_WORLD: u32 = DATA + 0x1000;

    pub(crate) const OC703_RIDERS: u32 = DATA + 0x1100;

    pub(crate) const OC703_BASE: u32 = DATA + 0x1200;

    /// The rider entry block (`+0x6c0`) and the speed object behind it.
    pub(crate) const OC703_RIDER: u32 = DATA + 0x1600;

    pub(crate) const OC703_SPEED: u32 = DATA + 0x1800;

    /// The fixture of the two script-object functions: the manager with (or
    /// without) a current process.
    pub(crate) fn oc703_self_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x0303_0430, 0);
        c.pc = entry;
        c.g[A0][0] = sx(OC703_OBJ);
        c.bus.write32(GAME_GP + 0xce8, OC703_MGR);
        let cur = if seed % 4 == 0 { 0 } else { OC703_PROC };
        c.bus.write32(OC703_MGR + 0x290, cur);
        c.bus.write32(OC703_PROC + 0x78, r.next_u32());
        c.bus.write32(OC703_MGR + 0x2a4, cur);
        c.bus.write32(OC703_OBJ, r.next_u32());
    }

    /// The fixture for the two `GetRiderSpeed` functions: a rider entry
    /// whose fetch function is `LunoVMRegister_AsInt` (returns its argument),
    /// so the vector sits at the rider block plus a half-word offset.
    pub(crate) fn oc703_speed_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = fakes::machine(c, seed, 0x0303_3380, 0);
        c.pc = entry;
        c.g[A0][0] = sx(OC703_OBJ);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC703_WORLD);
        c.bus.write32(OC703_WORLD + 0x84, OC703_RIDERS);
        c.bus.write32(OC703_WORLD + 0x14, r.f32(4.0));
        c.bus.write32(OC703_RIDERS + 0xc, OC703_BASE);
        c.bus.write32(GAME_GP + 0xce8, OC703_MGR);
        c.bus.write32(OC703_MGR, 0);
        let rider = if seed % 4 == 0 { 0 } else { OC703_RIDER };
        c.bus.write32(OC703_BASE + 0x28, rider);
        c.bus.write32(rider.wrapping_add(0x6c0), OC703_SPEED);
        c.bus.write32(OC703_SPEED + 0x10, (seed % 8) * 0x10);
        c.bus.write32(OC703_SPEED + 0x14, 0x0022_6610);
        if rider != 0 {
            fill(c, &mut r, rider + 0x6c0, 0x40);
        }
    }

    #[test]
    fn wscript_fn_empty_helper_matches() {
        check(
            ee::s10::f_00303d88__generated,
            wscript_fn_empty_helper,
            4,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x0303_3d88, 0);
                c.pc = 0x0030_3d88;
            },
        );
    }

    #[test]
    fn wscript_fn_get_self_handle_matches() {
        check(
            ee::s10::f_00303430__generated,
            wscript_fn_get_self_handle,
            200,
            &|c, seed| {
                oc703_self_setup(c, seed, 0x0030_3430);
            },
        );
    }

    #[test]
    fn wscript_fn_get_current_task_id_matches() {
        check(
            ee::s10::f_00303490__generated,
            wscript_fn_get_current_task_id,
            200,
            &|c, seed| {
                let mut r = Rng(seed as u64 + 1);
                oc703_self_setup(c, seed, 0x0030_3490);
                // The process: its count (+4) and current index (+8, in or out
                // of range), with a task table behind it (+0x14).
                c.bus
                    .write32(OC703_PROC + 0x4, if seed % 3 == 0 { 1 } else { 4 });
                c.bus.write32(OC703_PROC + 0x8, (seed % 2) * 2);
                c.bus.write32(OC703_PROC + 0x14, OC703_TABLE);
                fill(c, &mut r, OC703_TABLE, 0x100);
            },
        );
    }

    #[test]
    fn wscript_fn_get_rider_speed_matches() {
        check(
            ee::s10::f_00303380__generated,
            wscript_fn_get_rider_speed,
            200,
            &|c, seed| {
                oc703_speed_setup(c, seed, 0x0030_3380);
            },
        );
    }

    #[test]
    fn wscript_fn_get_rider_speed_scaled_matches() {
        check(
            ee::s10::f_003032c0__generated,
            wscript_fn_get_rider_speed_scaled,
            200,
            &|c, seed| {
                oc703_speed_setup(c, seed, 0x0030_32c0);
            },
        );
    }

    /// The BELibrary dummy at gp-0x1f04 (the BE getters' state).
    pub(crate) fn be_dummy(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3000);
        fill(c, &mut r, DATA + 0x3100, 0x200);
        r
    }

    #[test]
    fn wscript_fn_game_mode_matches_matches() {
        check(
            ee::s10::f_00303d90__generated,
            wscript_fn_game_mode_matches,
            200,
            &|c, seed| {
                let mut r = be_dummy(c, seed, 0x3d90);
                c.g[A0][0] = (seed % 7) as u64;
                for i in 0..4u32 {
                    c.bus.write32(0x0044_65e8 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write8(0x0053_5c11, (seed % 3) as u8);
            },
        );
    }

    #[test]
    fn wscript_fn_event_matches_matches() {
        check(
            ee::s10::f_00303e60__generated,
            wscript_fn_event_matches,
            200,
            &|c, seed| {
                let mut r = be_dummy(c, seed, 0x3e60);
                c.g[A0][0] = (seed % 18) as u64;
                for i in 0..16u32 {
                    c.bus.write32(0x0044_6618 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write8(0x0053_5c12, (seed % 3) as u8);
            },
        );
    }

    /// The WScriptMan pointer (gp+0xce8) and its game state: the being index
    /// (or -1) and the chain `world->+0x84->+0xc[being]->+0x28` to a record
    /// whose `+0x6c0` points at a `{delta, method}` pair.
    pub(crate) fn is_human_rider_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x3ec8, 2);
        let wsm = DATA + 0x100;
        let being = if seed % 5 == 4 { u32::MAX } else { seed % 4 };
        c.bus.write32(GAME_GP.wrapping_add(0xce8), wsm);
        c.bus.write32(wsm, being);
        let world = DATA + 0x200;
        let arr = DATA + 0x300;
        let tbl = DATA + 0x400;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, arr);
        c.bus.write32(arr + 0xc, tbl);
        let obj = DATA + 0x600;
        let a1rec = DATA + 0x700;
        c.bus.write32(
            tbl + 4 * (being & 3) + 0x28,
            if seed % 6 == 0 { 0 } else { obj },
        );
        c.bus.write32(obj + 0x6c0, a1rec);
        c.bus.write16(a1rec + 0x48, (r.next_u32() & 0xfff0) as u16);
        c.bus
            .write32(a1rec + 0x4c, crate::difftest::fakes::FAKE + 0x20);
        r
    }

    #[test]
    fn wscript_fn_is_human_rider_matches() {
        crate::difftest::fakes::covered(
            ee::s10::f_00303ec8__generated,
            wscript_fn_is_human_rider,
            200,
            &|c, seed| {
                let mut r = is_human_rider_fixture(c, seed);
                c.g[A0][0] = DATA as u64 + 0x800;
                c.bus.write32(DATA + 0x800 + 0x8, r.next_u32());
            },
        );
    }

    /// The request block's flag word, the setter's record, and a kind table
    /// with random kinds; the parameters follow at `DATA + 0x2100`.
    pub(crate) fn pending_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_add(0xce8), DATA + 0x4000);
        fill(c, &mut r, DATA + 0x4000, 0x40);
        for off in [0xc50u32, 0xc58, 0xc60] {
            for i in 0..3u32 {
                c.bus
                    .write32(GAME_GP.wrapping_add(off + 4 * i), r.next_u32() % 3);
            }
        }
        for i in 0..3u32 {
            let p = DATA + 0x2100 + 16 * i;
            c.bus.write32(p, r.next_u32() % 3);
            c.bus.write32(p + 4, r.next_u32());
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
        r
    }

    pub(crate) fn pending_args(c: &mut Cpu, seed: u32) {
        c.g[A0][0] = DATA as u64 + 0x2000;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = DATA as u64 + 0x2100;
    }

    #[test]
    fn wscript_fn_set_pending_request_a_matches() {
        check(
            ee::s10::f_00303ff0__generated,
            wscript_fn_set_pending_request_a,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x3ff0);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x26f4),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_set_pending_request_b_matches() {
        check(
            ee::s10::f_00304100__generated,
            wscript_fn_set_pending_request_b,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x4100);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x26f8),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_set_pending_request_c_matches() {
        check(
            ee::s10::f_00304210__generated,
            wscript_fn_set_pending_request_c,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x4210);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x26fc),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_game_mode_handler_v07_matches() {
        check(
            ee::s10::f_003044a8__generated,
            wscript_fn_game_mode_handler_v07,
            200,
            &|c, seed| {
                let mut r = pending_fixture(c, seed, 0x44a8);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x2718),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write8(GAME_GP.wrapping_add(0x2714), (seed % 3) as u8);
                let wsm = DATA + 0x4000;
                c.bus.write32(wsm, r.next_u32() % 4);
                c.bus.write32(DATA + 0x5000 + 0xc0, DATA + 0x5100);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x5000);
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_nop_4ff0_matches() {
        check(
            ee::s10::f_00304ff0__generated,
            wscript_fn_nop_4ff0,
            200,
            &|c, seed| {
                let _r = pending_fixture(c, seed, 0x4ff0);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x2748),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write8(GAME_GP.wrapping_add(0x2744), (seed % 3) as u8);
                pending_args(c, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_replay_highlight_matches() {
        check(
            ee::s10::f_00304b38__generated,
            wscript_fn_replay_highlight,
            200,
            &|c, seed| {
                let mut r = pending_fixture(c, seed, 0x4b38);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x2730),
                    if seed % 2 == 0 { 0 } else { 1 },
                );
                c.bus.write8(GAME_GP.wrapping_add(0x272c), (seed % 3) as u8);
                let world = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x5100);
                c.bus.write32(DATA + 0x5100 + 0x28, DATA + 0x5200);
                c.bus.write32(DATA + 0x4000, r.next_u32() % 4);
                pending_args(c, seed);
            },
        );
    }

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    pub(crate) const FX_WORLD: u32 = DATA + 0x20_0000;

    pub(crate) const FX_BE: u32 = DATA + 0x21_0000;

    pub(crate) const FX_OBJ: u32 = DATA + 0x22_0000;

    pub(crate) const FX_VT: u32 = DATA + 0x23_0000;

    pub(crate) const FX_MAN: u32 = DATA + 0x24_0000;

    pub(crate) const FX_PARAMS: u32 = DATA + 0x25_0000;

    pub(crate) const FX_SRET: u32 = DATA + 0x26_0000;

    pub(crate) const FX_TAGS: u32 = DATA + 0x27_0000;

    pub(crate) const FX_TAGS2: u32 = DATA + 0x27_1000;

    pub(crate) const FX_TAGTBL: u32 = DATA + 0x28_0000;

    pub(crate) const FX_ENTRY: u32 = DATA + 0x29_0000;

    pub(crate) const FX_ENTRY2: u32 = DATA + 0x2a_0000;

    pub(crate) const FX_PREV: u32 = DATA + 0x2b_0000;

    /// The globals and objects the WScript bodies read, varied by `seed`:
    /// the BE singleton, the world and its object and manager chain, the
    /// zone-tag tables, the guard flag at `gp+guard`, the kind table at
    /// `table`, and four small parameter records.
    pub(crate) fn wscript_fixture(c: &mut Cpu, seed: u32, salt: u64, guard: u32, table: u32) {
        let mut r = machine(c, seed, salt, 4);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x1f04), FX_BE);
        c.bus.write32(gp.wrapping_sub(0x848), FX_WORLD);
        c.bus.write32(FX_WORLD + 0x78, FX_BE);
        c.bus.write32(FX_WORLD + 0x84, FX_OBJ);
        c.bus.write32(FX_OBJ + 0xc, FX_VT);
        c.bus.write32(FX_VT + 0xa8, FX_MAN);
        // bIsNetworkGame: odd seeds skip func_001E3510.
        c.bus.write32(0x0053_4b30, seed & 1);
        c.bus.write32(gp.wrapping_add(0x16c8), FX_TAGS);
        c.bus.write32(FX_TAGS, FX_TAGS2);
        c.bus.write32(FX_TAGS2 + 0x8, FX_TAGTBL);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 { 0 } else { FX_ENTRY };
            c.bus.write32(FX_TAGTBL + 4 * i, e);
        }
        c.bus.write32(FX_ENTRY + 0x1c, FX_ENTRY2);
        for i in 0..0x100u32 {
            c.bus.write32(FX_ENTRY2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        c.bus.write32(
            FX_PREV + 0x290,
            if r.next_u32() % 3 == 0 { 0 } else { FX_OBJ },
        );
        c.bus.write32(gp.wrapping_add(0xce8), FX_PREV);
        c.bus.write32(gp.wrapping_add(guard), r.next_u32() & 1);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = FX_PARAMS + 16 * i;
            c.bus.write32(p, r.next_u32() % 4);
            let v = if r.next_u32() % 5 == 0 {
                u32::MAX
            } else {
                r.next_u32() & 0xffff
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// Arguments of the four-argument WScript bodies: sret, count (0..4 and
    /// -1), and the parameter array.
    pub(crate) fn wscript_args(c: &mut Cpu, seed: u32, sret: u32) {
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(sret);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(FX_PARAMS);
    }

    #[test]
    fn wscript_fn_career_action_matches() {
        check(
            ee::s10::f_00306300__generated,
            wscript_fn_career_action,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6300, 0x279c, GAME_GP.wrapping_add(0xcc0));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6300;
            },
        );
    }

    #[test]
    fn wscript_fn_add_danger_matches() {
        check(
            ee::s10::f_00306438__generated,
            wscript_fn_add_danger,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6438, 0x27a0, 0x0044_6880);
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6438;
            },
        );
    }

    #[test]
    fn wscript_fn_remove_danger_matches() {
        check(
            ee::s10::f_003065e8__generated,
            wscript_fn_remove_danger,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_65e8, 0x27b0, GAME_GP.wrapping_add(0xcc8));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_65e8;
            },
        );
    }

    #[test]
    fn wscript_fn_acquire_danger_zone_matches() {
        check(
            ee::s10::f_00306770__generated,
            wscript_fn_acquire_danger_zone,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6770, 0x27b4, GAME_GP.wrapping_add(0xcd0));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6770;
            },
        );
    }

    #[test]
    fn wscript_fn_release_danger_zone_matches() {
        check(
            ee::s10::f_00306908__generated,
            wscript_fn_release_danger_zone,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6908, 0x27c0, GAME_GP.wrapping_add(0xcd8));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6908;
            },
        );
    }

    #[test]
    fn wscript_fn_check_rider_gate_matches() {
        check(
            ee::s10::f_00306e68__generated,
            wscript_fn_check_rider_gate,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_6e68, 0x27cc, 0x0044_6980);
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_6e68;
            },
        );
    }

    #[test]
    fn wscript_fn_career_check_matches() {
        check(
            ee::s10::f_00307020__generated,
            wscript_fn_career_check,
            200,
            &|c, seed| {
                wscript_fixture(c, seed, 0x0030_7020, 0x27d0, GAME_GP.wrapping_add(0xce0));
                wscript_args(c, seed, FX_SRET);
                c.pc = 0x0030_7020;
            },
        );
    }

    /// The machine state of the two modifier builders: `seed % 4` parameters
    /// (entry 0 is instance id 0 on `seed % 3 == 0`, else the id stays -1), the
    /// type table, the init flag, and the world -> section -> instance chain.
    /// The instance's active node is a vtable whose `isMoveNode` is the leaf
    /// at `0x1406c0` (returns 0), so the node-add call is not reached here.
    pub(crate) fn wsm_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, s: &ModSite) {
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA + 0x7000);
        let n = seed % 4;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            let idx = if i == 0 && seed % 3 == 0 {
                0
            } else {
                1 + r.next_u32() % 2
            };
            let val = if idx == 0 { 0 } else { r.next_u32() };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..3 {
            c.bus.write32(s.types + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(GAME_GP + s.flag_gp, seed % 2);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        let section = if (seed / 3) % 4 == 2 {
            0
        } else {
            DATA + 0x6000
        };
        c.bus.write32(DATA + 0x5000, section);
        c.bus.write32(DATA + 0x601c, DATA + 0x6100);
        let entry = if (seed / 5) % 5 == 4 {
            0
        } else {
            ((DATA + 0x8000) >> 2) << 8
        };
        c.bus.write32(DATA + 0x6100, entry);
        c.bus
            .write32(DATA + 0x800c, if seed % 2 == 0 { DATA + 0x9000 } else { 0 });
        c.bus.write32(DATA + 0x900c, DATA + 0x9100);
        c.bus.write32(DATA + 0x9184, 0x0014_06c0);
        c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
        c.bus.write32(DATA + 0xa290, DATA + 0xb000);
    }

    #[test]
    fn wscript_add_tex_flip_modifier_matches() {
        check(
            ee::s10::f_002fe4a8__generated,
            wscript_add_tex_flip_modifier,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fe4_a8 ^ seed as u64);
                scramble(c, &mut r);
                wsm_fixture(c, &mut r, seed, &TEX_SITE);
            },
        );
    }

    #[test]
    fn wscript_add_boost_modifier_matches() {
        check(
            ee::s10::f_002fe668__generated,
            wscript_add_boost_modifier,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fe6_68 ^ seed as u64);
                scramble(c, &mut r);
                wsm_fixture(c, &mut r, seed, &BOOST_SITE);
            },
        );
    }

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    pub(crate) const FX_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const FX_MAGIC: u32 = 0x4f43_3733;

    pub(crate) const FX_WORLD_2: u32 = DATA + 0x21_0000;

    pub(crate) const FX_ARR: u32 = DATA + 0x23_0000;

    pub(crate) const FX_RIDER: u32 = DATA + 0x24_0000;

    pub(crate) const FX_VT2: u32 = DATA + 0x25_0000;

    pub(crate) const FX_PARAMS_2: u32 = DATA + 0x26_0000;

    pub(crate) const FX_SRET_2: u32 = DATA + 0x27_0000;

    pub(crate) const FX_TAGS_2: u32 = DATA + 0x28_0000;

    pub(crate) const FX_TAGS2_2: u32 = DATA + 0x28_1000;

    pub(crate) const FX_TAGTBL_2: u32 = DATA + 0x29_0000;

    pub(crate) const FX_ENTRY_2: u32 = DATA + 0x2a_0000;

    pub(crate) const FX_ENTRY2_2: u32 = DATA + 0x2b_0000;

    pub(crate) const FX_PREV_2: u32 = DATA + 0x2c_0000;

    pub(crate) const FX_OBJ2: u32 = DATA + 0x2d_0000;

    pub(crate) const FX_EOBJ: u32 = DATA + 0x2e_0000;

    pub(crate) const FX_EVT: u32 = DATA + 0x2f_0000;

    pub(crate) const FX_ENGINE: u32 = DATA + 0x30_0000;

    macro_rules! oc732_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(FX_CTRL) != FX_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc732_fake!(
        oc732_fake_as_int,
        OC732_PREV_AS_INT,
        ee::s05::f_00226610__generated,
        0x0022_6610
    );

    oc732_fake!(
        oc732_fake_rail,
        OC732_PREV_RAIL,
        ee::s11::f_00358b28__generated,
        0x0035_8b28
    );

    oc732_fake!(
        oc732_fake_movie,
        OC732_PREV_MOVIE,
        ee::s07::f_00278ed0__generated,
        0x0027_8ed0
    );

    oc732_fake!(
        oc732_fake_choice,
        OC732_PREV_CHOICE,
        ee::s07::f_00278e50__generated,
        0x0027_8e50
    );

    oc732_fake!(
        oc732_fake_prepare,
        OC732_PREV_PREPARE,
        ee::s07::f_00278f38__generated,
        0x0027_8f38
    );

    oc732_fake!(
        oc732_fake_start,
        OC732_PREV_START,
        ee::s07::f_00278f68__generated,
        0x0027_8f68
    );

    oc732_fake!(
        oc732_fake_collected,
        OC732_PREV_COLLECTED,
        ee::s10::f_0030ba80__generated,
        0x0030_ba80
    );

    oc732_fake!(
        oc732_fake_jump,
        OC732_PREV_JUMP,
        ee::s10::f_0030a6d8__generated,
        0x0030_a6d8
    );

    oc732_fake!(
        oc732_fake_goto,
        OC732_PREV_GOTO,
        ee::s10::f_0030a700__generated,
        0x0030_a700
    );

    pub(crate) fn oc732_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00226610),
            oc732_fake_as_int,
            &OC732_PREV_AS_INT,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00358B28),
            oc732_fake_rail,
            &OC732_PREV_RAIL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278ED0),
            oc732_fake_movie,
            &OC732_PREV_MOVIE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278E50),
            oc732_fake_choice,
            &OC732_PREV_CHOICE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F38),
            oc732_fake_prepare,
            &OC732_PREV_PREPARE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F68),
            oc732_fake_start,
            &OC732_PREV_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030BA80),
            oc732_fake_collected,
            &OC732_PREV_COLLECTED,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A6D8),
            oc732_fake_jump,
            &OC732_PREV_JUMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A700),
            oc732_fake_goto,
            &OC732_PREV_GOTO,
        );
    }

    /// The globals and objects the WScript bodies read, varied by `seed`: the
    /// world and its rider array, the zone-tag tables, the current-object
    /// record at `gp+0xce8` (rider index, tagged object and its vtable chain),
    /// the script engine pointer, the once-init guard at `gp+guard`, the kind
    /// table at `table`, and four small argument records.
    pub(crate) fn oc732_fixture(c: &mut Cpu, seed: u32, salt: u64, guard: u32, table: u32) {
        let mut r = machine(c, seed, salt, 4);
        let gp = GAME_GP;
        c.bus.write32(FX_CTRL, FX_MAGIC);
        c.bus.write32(gp.wrapping_sub(0x848), FX_WORLD_2);
        c.bus.write32(gp.wrapping_sub(0x84c), FX_ENGINE);
        c.bus.write32(FX_WORLD_2 + 0x84, FX_OBJ);
        c.bus.write32(FX_OBJ + 0xc, FX_ARR);
        for i in 0..8u32 {
            c.bus.write32(FX_ARR + 0x28 + 4 * i, FX_RIDER);
        }
        c.bus.write32(FX_RIDER + 0x6c0, FX_VT2);
        c.bus
            .write16(FX_VT2 + 0x60, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(FX_VT2 + 0x64, fakes::FAKE + 0x10);
        // The tag tables.
        c.bus.write32(gp.wrapping_add(0x16c8), FX_TAGS_2);
        c.bus.write32(FX_TAGS_2, FX_TAGS2_2);
        c.bus.write32(FX_TAGS2_2 + 0x8, FX_TAGTBL_2);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 { 0 } else { FX_ENTRY_2 };
            c.bus.write32(FX_TAGTBL_2 + 4 * i, e);
        }
        c.bus.write32(FX_ENTRY_2 + 0x1c, FX_ENTRY2_2);
        for i in 0..0x100u32 {
            c.bus
                .write32(FX_ENTRY2_2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        // The current-object record: rider index, tagged object chain.
        c.bus.write32(gp.wrapping_add(0xce8), FX_PREV_2);
        let idx = [u32::MAX, 0, 1, 2, 3, 5][(r.next_u32() % 6) as usize];
        c.bus.write32(FX_PREV_2, idx);
        c.bus.write32(
            FX_PREV_2 + 0x290,
            if r.next_u32() % 4 == 0 { 0 } else { FX_OBJ2 },
        );
        c.bus.write32(
            FX_OBJ2 + 0xc,
            if r.next_u32() % 4 == 0 { 0 } else { FX_EOBJ },
        );
        c.bus.write32(FX_EOBJ + 0xc, FX_EVT);
        for (k, off) in [0xc0u32, 0xf8].into_iter().enumerate() {
            c.bus
                .write16(FX_EVT + off, (r.next_u32() as i16 as i32 & !3) as u16);
            c.bus
                .write32(FX_EVT + off + 4, fakes::FAKE + 0x20 + 0x10 * k as u32);
        }
        c.bus.write32(gp.wrapping_add(guard), r.next_u32() & 1);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = FX_PARAMS_2 + 16 * i;
            c.bus.write32(p, r.next_u32() % 4);
            let v = match r.next_u32() % 6 {
                0 => u32::MAX,
                1 => 0,
                2 => 1,
                3 => 2,
                4 => 3,
                _ => r.next_u32() & 0xffff,
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// Arguments of all eight bodies: a0 the struct return / result register,
    /// a1 the argument count (0..4 and -1), a2 the argument array.
    pub(crate) fn oc732_args(c: &mut Cpu, seed: u32) {
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(FX_SRET_2);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(FX_PARAMS_2);
    }

    /// `WScriptFn_PlaceRiderAtObject`: block guard `gp+0x25b0`.
    #[test]
    fn wscript_fn_place_rider_at_object_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00300948__generated,
            wscript_fn_place_rider_at_object,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_0948, 0x25b0, GAME_GP.wrapping_add(0xb58));
                oc732_args(c, seed);
                c.pc = 0x0030_0948;
            },
        );
    }

    /// `WScriptFn_SetRailEnabled`: block guard `gp+0x25b4`.
    #[test]
    fn wscript_fn_set_rail_enabled_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00300b20__generated,
            wscript_fn_set_rail_enabled,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_0b20, 0x25b4, GAME_GP.wrapping_add(0xb60));
                oc732_args(c, seed);
                c.pc = 0x0030_0b20;
            },
        );
    }

    /// `WScriptFn_PlayScriptList`: block guard `gp+0x25b8`.
    #[test]
    fn wscript_fn_play_script_list_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00300c78__generated,
            wscript_fn_play_script_list,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_0c78, 0x25b8, GAME_GP.wrapping_add(0xb68));
                oc732_args(c, seed);
                c.pc = 0x0030_0c78;
            },
        );
    }

    /// `WScriptFn_Nop_1440`: block guard `gp+0x25cc`.
    #[test]
    fn wscript_fn_nop_1440_matches() {
        oc732_install_fakes();
        check(
            ee::s10::f_00301440__generated,
            wscript_fn_nop_1440,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1440, 0x25cc, GAME_GP.wrapping_add(0xb88));
                oc732_args(c, seed);
                c.pc = 0x0030_1440;
            },
        );
    }

    /// `WScriptFn_GetCollectedCount`: block guard `gp+0x25d8`.
    #[test]
    fn wscript_fn_get_collected_count_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301560__generated,
            wscript_fn_get_collected_count,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1560, 0x25d8, GAME_GP.wrapping_add(0xb90));
                oc732_args(c, seed);
                c.pc = 0x0030_1560;
            },
        );
    }

    /// `WScriptFn_GetObjectSlot31`: block guard `gp+0x25f8`.
    #[test]
    fn wscript_fn_get_object_slot31_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301830__generated,
            wscript_fn_get_object_slot31,
            300,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1830, 0x25f8, GAME_GP.wrapping_add(0xba0));
                oc732_args(c, seed);
                c.pc = 0x0030_1830;
            },
        );
    }

    /// `WScriptFn_JumpToTask`: block guard `gp+0x2608`.
    #[test]
    fn wscript_fn_jump_to_task_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301b88__generated,
            wscript_fn_jump_to_task,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1b88, 0x2608, GAME_GP.wrapping_add(0xbb0));
                oc732_args(c, seed);
                c.pc = 0x0030_1b88;
            },
        );
    }

    /// `WScriptFn_GotoTask`: block guard `gp+0x2618`.
    #[test]
    fn wscript_fn_goto_task_matches() {
        oc732_install_fakes();
        fakes::covered(
            ee::s10::f_00301c80__generated,
            wscript_fn_goto_task,
            200,
            &|c, seed| {
                oc732_fixture(c, seed, 0x0030_1c80, 0x2618, GAME_GP.wrapping_add(0xbb8));
                oc732_args(c, seed);
                c.pc = 0x0030_1c80;
            },
        );
    }

    // Fixture objects, in DATA+0x200000 and up (clear of the fake log).
    pub(crate) const W_WORLD: u32 = DATA + 0x20_0000;

    pub(crate) const W_OBJ: u32 = DATA + 0x22_0000;

    pub(crate) const W_VT: u32 = DATA + 0x23_0000;

    pub(crate) const W_PARAMS: u32 = DATA + 0x25_0000;

    pub(crate) const W_SRET: u32 = DATA + 0x26_0000;

    pub(crate) const W_TAGS: u32 = DATA + 0x27_0000;

    pub(crate) const W_TAGS2: u32 = DATA + 0x27_1000;

    pub(crate) const W_TAGTBL: u32 = DATA + 0x28_0000;

    pub(crate) const W_ENTRY: u32 = DATA + 0x29_0000;

    pub(crate) const W_ENTRY2: u32 = DATA + 0x2a_0000;

    pub(crate) const W_PREV: u32 = DATA + 0x2b_0000;

    pub(crate) const W_RIDER: u32 = DATA + 0x2c_0000;

    pub(crate) const W_SUB: u32 = DATA + 0x2d_0000;

    /// The word the gated fakes look for (other tests' machines lack it).
    pub(crate) const W_GATE: u32 = DATA + 0x3f_0000;

    pub(crate) const W_MAGIC: u32 = 0x0733_0733;

    pub(crate) static OC733_PREV_PROCESS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC733_PREV_MISSION: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc733_gated(c: &mut Cpu) -> bool {
        c.bus.read32(W_GATE) == W_MAGIC
    }

    /// `WScriptMan_ProcessCommand` (0x30a728), gated: logs the call and returns.
    pub(crate) fn oc733_fake_process(c: &mut Cpu) {
        if !oc733_gated(c) {
            return fakes::fall_back(c, &OC733_PREV_PROCESS, ee::s10::f_0030a728__generated);
        }
        fakes::fake_call(c, 0x0030_a728);
    }

    /// `WScriptMan_MissionCommand` (0x30a868), gated: logs the call and returns.
    pub(crate) fn oc733_fake_mission(c: &mut Cpu) {
        if !oc733_gated(c) {
            return fakes::fall_back(c, &OC733_PREV_MISSION, ee::s10::f_0030a868__generated);
        }
        fakes::fake_call(c, 0x0030_a868);
    }

    pub(crate) fn oc733_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A728),
            oc733_fake_process,
            &OC733_PREV_PROCESS,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A868),
            oc733_fake_mission,
            &OC733_PREV_MISSION,
        );
    }

    /// The globals and objects the bind bodies read, varied by `seed`: the
    /// world and its object and manager chain, the zone tables, the flag word
    /// at `flag` (set or not), the id word at `block` (the copy source), the
    /// kind table at `table`, and four parameter records.
    pub(crate) fn oc733_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        flag: u32,
        block: u32,
        table: u32,
    ) {
        let mut r = machine(c, seed, salt, 2);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x848), W_WORLD);
        c.bus.write32(W_WORLD + 0x84, W_OBJ);
        c.bus.write32(W_OBJ + 0xc, W_VT);
        c.bus.write32(gp.wrapping_add(0x16c8), W_TAGS);
        c.bus.write32(W_TAGS, W_TAGS2);
        c.bus.write32(W_TAGS2 + 0x8, W_TAGTBL);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 { 0 } else { W_ENTRY };
            c.bus.write32(W_TAGTBL + 4 * i, e);
        }
        c.bus.write32(W_ENTRY + 0x1c, W_ENTRY2);
        for i in 0..0x100u32 {
            c.bus.write32(W_ENTRY2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        c.bus.write32(
            W_PREV + 0x290,
            if r.next_u32() % 3 == 0 { 0 } else { W_OBJ },
        );
        c.bus.write32(gp.wrapping_add(0xce8), W_PREV);
        c.bus.write32(flag, r.next_u32() & 1);
        // The id word: -1 (the wscript's own instance) or a small section id.
        let id = if r.next_u32() % 3 == 0 {
            u32::MAX
        } else {
            r.next_u32() & 0xffff
        };
        c.bus.write32(block, id);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = W_PARAMS + 16 * i;
            c.bus.write32(p, r.next_u32() % 4);
            let v = if r.next_u32() % 5 == 0 {
                u32::MAX
            } else {
                r.next_u32() & 0xffff
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// Arguments of the bind bodies: sret or self (`a0`), count (0..4 and
    /// -1), and the parameter array.
    pub(crate) fn oc733_args(c: &mut Cpu, seed: u32, sret: u32) {
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(sret);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(W_PARAMS);
    }

    /// The rider of the being (`gp+0xce8`'s word names the entry index): its
    /// `+0x6c0` record holds the virtual `isIntersecting` at `+0x1c`. One half
    /// of the seeds answer through the miss fake (non-zero), the other half
    /// through a real leaf that returns 0.
    pub(crate) fn oc733_rider(c: &mut Cpu, seed: u32) {
        let k = seed % 4;
        c.bus.write32(W_PREV, k);
        c.bus.write32(W_VT + 4 * k + 0x28, W_RIDER);
        c.bus.write32(W_RIDER + 0x6c0, W_SUB);
        c.bus.write16(W_SUB + 0x18, 0);
        let target = if (seed / 4) % 2 == 0 {
            fakes::FAKE
        } else {
            0x0014_06c0
        };
        c.bus.write32(W_SUB + 0x1c, target);
    }

    #[test]
    fn wscript_fn_has_rider_visited_matches() {
        check(
            ee::s10::f_00301d78__generated,
            wscript_fn_has_rider_visited,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_1d78,
                    GAME_GP.wrapping_add(0x2628),
                    GAME_GP.wrapping_add(0x2620),
                    GAME_GP.wrapping_add(0xbc0),
                );
                oc733_rider(c, seed);
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_1d78;
            },
        );
    }

    #[test]
    fn wscript_fn_nop_1f48_matches() {
        check(
            ee::s10::f_00301f48__generated,
            wscript_fn_nop_1f48,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_1f48,
                    GAME_GP.wrapping_add(0x2638),
                    GAME_GP.wrapping_add(0x2630),
                    GAME_GP.wrapping_add(0xbc8),
                );
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_1f48;
            },
        );
    }

    #[test]
    fn wscript_fn_is_game_mode_matches() {
        check(
            ee::s10::f_00302680__generated,
            wscript_fn_is_game_mode,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2680,
                    GAME_GP.wrapping_add(0x2650),
                    GAME_GP.wrapping_add(0x2648),
                    GAME_GP.wrapping_add(0xbe0),
                );
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_2680;
            },
        );
    }

    #[test]
    fn wscript_fn_is_event_matches() {
        check(
            ee::s10::f_00302870__generated,
            wscript_fn_is_event,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2870,
                    GAME_GP.wrapping_add(0x2670),
                    GAME_GP.wrapping_add(0x2668),
                    GAME_GP.wrapping_add(0xbf0),
                );
                oc733_args(c, seed, W_SRET);
                c.pc = 0x0030_2870;
            },
        );
    }

    #[test]
    fn wscript_fn_process_command_matches() {
        oc733_install_fakes();
        check(
            ee::s10::f_00302ac0__generated,
            wscript_fn_process_command,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2ac0,
                    GAME_GP.wrapping_add(0x2684),
                    0x004f_bc40,
                    GAME_GP.wrapping_add(0xc00),
                );
                oc733_args(c, seed, W_SRET);
                c.bus.write32(W_GATE, W_MAGIC);
                c.pc = 0x0030_2ac0;
            },
        );
    }

    #[test]
    fn wscript_fn_mission_stop_process_matches() {
        oc733_install_fakes();
        check(
            ee::s10::f_00302bd8__generated,
            wscript_fn_mission_stop_process,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2bd8,
                    GAME_GP.wrapping_add(0x2690),
                    GAME_GP.wrapping_add(0x2688),
                    GAME_GP.wrapping_add(0xc08),
                );
                oc733_args(c, seed, W_SRET);
                c.bus.write32(W_GATE, W_MAGIC);
                c.pc = 0x0030_2bd8;
            },
        );
    }

    #[test]
    fn wscript_fn_stop_process_matches() {
        oc733_install_fakes();
        check(
            ee::s10::f_00302ce8__generated,
            wscript_fn_stop_process,
            200,
            &|c, seed| {
                oc733_fixture(
                    c,
                    seed,
                    0x0030_2ce8,
                    GAME_GP.wrapping_add(0x26a0),
                    GAME_GP.wrapping_add(0x2698),
                    GAME_GP.wrapping_add(0xc10),
                );
                oc733_args(c, seed, W_SRET);
                c.bus.write32(W_GATE, W_MAGIC);
                c.pc = 0x0030_2ce8;
            },
        );
    }

    /// The script object every fixture passes as `this`.
    pub(crate) const OC734_OBJ: u32 = DATA + 0x4000;

    /// The script manager (`gp+0xce8` holds its address).
    pub(crate) const OC734_MGR: u32 = DATA + 0x1400;

    /// The world (`gp-0x848` holds its address) and its rider table.
    pub(crate) const OC734_WORLD: u32 = DATA + 0x1000;

    pub(crate) const OC734_RIDERS: u32 = DATA + 0x1100;

    pub(crate) const OC734_BASE: u32 = DATA + 0x1200;

    /// The rider and the handler behind its `+0x77c` word.
    pub(crate) const OC734_RIDER: u32 = DATA + 0x1600;

    pub(crate) const OC734_HANDLER: u32 = DATA + 0x1a00;

    /// The argument list the loop reads (16-byte entries).
    pub(crate) const OC734_PARAMS: u32 = DATA + 0x2000;

    /// The rider chain the fixtures build: world -> rider table (+0x84) ->
    /// base (+0xc) -> rider (+0x28), rider index 0. Returns the rider (0 for
    /// the no-rider cases).
    pub(crate) fn oc734_world(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC734_WORLD);
        c.bus.write32(OC734_WORLD + 0x84, OC734_RIDERS);
        c.bus.write32(OC734_RIDERS + 0xc, OC734_BASE);
        c.bus.write32(GAME_GP + 0xce8, OC734_MGR);
        c.bus.write32(OC734_MGR, 0);
        let rider = if seed % 4 == 0 { 0 } else { OC734_RIDER };
        c.bus.write32(OC734_BASE + 0x28, rider);
        if rider != 0 {
            fill(c, r, rider, 0x800);
            c.bus.write32(rider + 0x77c, OC734_HANDLER);
            c.bus.write32(OC734_HANDLER + 0xd4, r.next_u32());
        }
        rider
    }

    /// The static block, its flag and byte, the kind table and an argument
    /// list of up to two entries (indices 0 and 1, random values and kinds).
    pub(crate) fn oc734_args(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        flag_off: u32,
        byte_off: u32,
        src_off: u32,
        kinds_off: u32,
    ) {
        let count = seed % 3;
        c.g[A1][0] = count as u64;
        c.g[A2][0] = sx(OC734_PARAMS);
        c.bus.write32(GAME_GP + src_off, (seed / 2) % 21);
        c.bus
            .write32(GAME_GP + flag_off, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + byte_off, r.next_u32());
        for k in 0..2u32 {
            c.bus.write32(GAME_GP + kinds_off + 4 * k, r.next_u32() % 3);
        }
        for j in 0..count {
            let e = OC734_PARAMS + 16 * j;
            c.bus.write32(e, j % 2);
            c.bus.write32(e + 0x4, r.next_u32());
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
    }

    /// The fixture of the two rider-query switches: the case table of the
    /// given layout (index 4 may take the quirk arm), the static block and
    /// the arguments.
    pub(crate) fn oc734_query_setup(c: &mut Cpu, seed: u32, q: &RiderQuery) {
        let mut r = fakes::machine(c, seed, q.entry as u64, 0);
        c.pc = q.entry;
        c.g[A0][0] = sx(OC734_OBJ);
        oc734_world(c, &mut r, seed);
        oc734_args(
            c,
            &mut r,
            seed,
            q.flag_off,
            q.byte_off,
            q.src_off,
            q.kinds_off,
        );
        for (i, &target) in q.targets.iter().enumerate() {
            c.bus.write32(q.table + 4 * i as u32, target);
        }
        if let Some(quirk) = q.quirk {
            if seed % 5 == 0 {
                c.bus.write32(q.table + 16, quirk);
            }
        }
    }

    #[test]
    fn wscript_fn_is_rider_in_state_matches() {
        check(
            ee::s10::f_00302ef8__generated,
            wscript_fn_is_rider_in_state,
            300,
            &|c, seed| {
                oc734_query_setup(c, seed, &IS_RIDER_IN_STATE);
            },
        );
    }

    #[test]
    fn wscript_fn_is_rider_in_state_2_matches() {
        check(
            ee::s10::f_003037b0__generated,
            wscript_fn_is_rider_in_state_2,
            300,
            &|c, seed| {
                oc734_query_setup(c, seed, &IS_RIDER_IN_STATE_2);
            },
        );
    }

    #[test]
    fn wscript_fn_get_rider_state_info_matches() {
        check(
            ee::s10::f_003034f8__generated,
            wscript_fn_get_rider_state_info,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34f8, 0);
                c.pc = 0x0030_34f8;
                c.g[A0][0] = sx(OC734_OBJ);
                oc734_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_get_event_arg_matches() {
        check(
            ee::s10::f_003036b0__generated,
            wscript_fn_get_event_arg,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x36b0, 0);
                c.pc = 0x0030_36b0;
                c.g[A0][0] = sx(OC734_OBJ);
                oc734_world(c, &mut r, seed);
                oc734_args(c, &mut r, seed, 0x26d8, 0x26d4, 0x26d0, 0xc38);
            },
        );
    }

    /// The world the builders' lookups read: `gp+0x16c8` -> table -> records
    /// (`key & 0xff`) -> entries (`key >> 8`), the world at `gp-0x848` with its
    /// section table and the script manager's section index (`gp+0xce8`), the
    /// per-section instance and its callee (the leaf at `0x1406c0`, returns 0).
    pub(crate) fn wsf_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        for j in 0..4u32 {
            let sec = DATA + 0x6000 + 0x100 * j;
            c.bus.write32(
                DATA + 0x5000 + 4 * j,
                if (seed + j) % 5 == 0 { 0 } else { sec },
            );
            c.bus.write32(sec + 0x1c, DATA + 0x7000 + 0x100 * j);
            for m in 0..4u32 {
                let e = if (seed + m + j) % 3 == 0 {
                    0
                } else {
                    (1 + r.next_u32() % 255) << 8
                };
                c.bus.write32(DATA + 0x7000 + 0x100 * j + 4 * m, e);
            }
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x3800);
        c.bus.write32(DATA + 0x3800 + 0x84, DATA + 0x3900);
        c.bus.write32(DATA + 0x3900 + 0xc, DATA + 0x3a00);
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x3b00);
        c.bus.write32(DATA + 0x3b00, seed % 4);
        c.bus.write32(
            DATA + 0x3b00 + 0x290,
            if seed % 2 == 0 { 0 } else { DATA + 0x3c00 },
        );
        for k in 0..4u32 {
            let inst = if (seed / 4 + k) % 3 == 0 {
                0
            } else {
                DATA + 0x3c00
            };
            c.bus.write32(DATA + 0x3a00 + 4 * k + 0x28, inst);
        }
        c.bus.write32(DATA + 0x3c00 + 0x6c0, DATA + 0x3d00);
        c.bus.write32(DATA + 0x3d00 + 0x2c, 0x0014_06c0);
        c.bus.write32(DATA + 0x3d00 + 0x44, 0x0014_06c0);
    }

    /// `count` parameter records at `DATA+0x9100` and the type table at `types`.
    pub(crate) fn wsf_records(c: &mut Cpu, r: &mut Rng, seed: u32, types: u32) {
        for i in 0..4u32 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
        for i in 0..(seed % 4) {
            let e = DATA + 0x9100 + 16 * i;
            c.bus.write32(e, i % 4);
            c.bus.write32(e + 4, r.next_u32());
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
    }

    #[test]
    fn wscript_get_rider_height_above_object_matches() {
        check(
            ee::s10::f_00304c50__generated,
            wscript_get_rider_height_above_object,
            300,
            &|c, seed| {
                let mut r = Rng(0x304c_50 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2740, seed % 2);
                c.bus.write32(GAME_GP + 0x2738, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xc88);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_call_object_slot41_matches() {
        check(
            ee::s10::f_00304e38__generated,
            wscript_call_object_slot41,
            300,
            &|c, seed| {
                let mut r = Rng(0x304e_38 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2744, seed % 2);
                c.bus.write32(0x004f_bd20, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xc90);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_screen_flash_matches() {
        check(
            ee::s10::f_00305660__generated,
            wscript_screen_flash,
            300,
            &|c, seed| {
                let mut r = Rng(0x3056_60 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                c.bus.write32(
                    DATA + 0x3b00,
                    if seed % 5 == 4 { u32::MAX } else { seed % 4 },
                );
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2760, seed % 2);
                c.bus.write32(GAME_GP + 0x2758, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xca0);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn world_obj_factory_avalanche_matches() {
        check(
            ee::s10::f_003059a0__generated,
            world_obj_factory_avalanche,
            300,
            &|c, seed| {
                let mut r = Rng(0x3059_a0 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                c.bus.write32(GAME_GP + 0x2768, seed % 2);
                // Every node slot of the copied list is empty (-1), so the walk
                // never reaches the live-node callee at 0x2d92d0 (its real body
                // runs away on zeroed test data). Count 0: no parameter records.
                for i in 0..50u32 {
                    c.bus.write32(0x004f_bd90 + 4 * i, u32::MAX);
                }
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_start_avalanche_matches() {
        check(
            ee::s10::f_00305c88__generated,
            wscript_start_avalanche,
            300,
            &|c, seed| {
                let mut r = Rng(0x305c_88 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                c.bus.write32(GAME_GP + 0x2778, seed % 2);
                c.bus.write32(GAME_GP + 0x2770, r.next_u32());
                c.bus.write32(GAME_GP + 0x9d4, 1);
                wsf_records(c, &mut r, seed, GAME_GP + 0xca8);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_add_ava_spline_modifier_matches() {
        check(
            ee::s10::f_00305d90__generated,
            wscript_add_ava_spline_modifier,
            300,
            &|c, seed| {
                let mut r = Rng(0x305d_90 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2788, seed % 2);
                c.bus.write32(GAME_GP + 0x2780, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xcb0);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    /// `WorldObjFactory_AnimDelta` (the script function `MakeAnimDeltaFunc`):
    /// the init flag (`gp+0x24d0`), the parameter list (idx, val, type) with
    /// its type table at 0x445e70, the target found through the script
    /// manager (id -1) or the world's section table, and the node found,
    /// attached (fake result) or allocated (fake allocator and constructor).
    #[test]
    fn world_obj_factory_anim_delta_matches() {
        oc739_install();
        check(
            ee::s10::f_002fb060__generated,
            world_obj_factory_anim_delta,
            400,
            &|c, seed| {
                let mut r = Rng(0x002f_b060 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                oc739_gate(c);
                c.bus.write32(OC739_CTRL + 0x10, DATA + 0xb000);
                c.bus.write32(OC739_CTRL + 0x14, DATA + 0xb000);
                c.bus.write32(OC739_CTRL + 0x18, r.next_u32() % 2);
                let out = DATA + 0x7000;
                let params = DATA + 0x1000;
                let n = r.next_u32() % 4;
                c.g[A0][0] = sx(out);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(params);
                c.bus.write32(GAME_GP + 0x24d0, r.next_u32() % 2);
                for i in 0..12 {
                    c.bus.write32(0x004f_b390 + 4 * i, r.next_u32());
                    c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
                }
                for i in 0..11 {
                    // Entry 0 (the instance id) is never a float: a float id would index far outside RAM.
                    let kind = if i == 0 {
                        r.next_u32() % 2
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(0x0044_5e70 + 4 * i, kind);
                }
                for i in 0..n {
                    let e = params + 16 * i;
                    let idx = if i == 0 && r.next_u32() % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 10
                    };
                    let val = if idx == 0 {
                        (r.next_u32() % 3) << 8
                    } else {
                        r.next_u32() % 1000
                    };
                    c.bus.write32(e, idx);
                    c.bus.write32(e + 4, val);
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                // World -> section table -> instance record chain, and the script manager.
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(DATA + 0x4008, DATA + 0x5000);
                let section = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    DATA + 0x6000
                };
                c.bus.write32(DATA + 0x5000, section);
                c.bus.write32(DATA + 0x601c, DATA + 0x6100);
                for k in 0..3u32 {
                    let word = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        ((DATA + 0x8000) >> 2) << 8
                    };
                    c.bus.write32(DATA + 0x6100 + 4 * k, word);
                }
                c.bus.write32(
                    DATA + 0x800c,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x9000
                    },
                );
                c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
                c.bus.write32(
                    DATA + 0xa290,
                    if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        DATA + 0x8000
                    },
                );
            },
        );
    }

    pub(crate) const SEEDS: u32 = 240;

    /// Gate of the callee fakes: a magic word in the machine's RAM turns them
    /// on (both sides of a diff get their own machine); `CTRL + 4` is the
    /// result the attach fake returns.
    pub(crate) const OC740_MAGIC: u32 = 0x0740_0740;

    pub(crate) const OC740_CTRL: u32 = DATA + 0x20_0000;

    /// A real leaf that returns 0 (the vtable "is it a ..." queries).
    pub(crate) const OC740_LEAF_ZERO: u32 = 0x0014_06c0;

    macro_rules! oc740_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC740_CTRL) != OC740_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    pub(crate) static OC740_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `WorldObjFactory_AttachModifier`: the logged call, then
    /// the result the fixture asked for (`CTRL + 4`: 0 or 1).
    pub(crate) fn oc740_fake_attach(c: &mut Cpu) {
        if c.bus.read32(OC740_CTRL) != OC740_MAGIC {
            return fakes::fall_back(c, &OC740_PREV_ATTACH, ee::s10::f_002fae38);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002f_ae38, ra);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(OC740_CTRL + 4) as u64;
        }
    }

    oc740_fake!(
        oc740_fake_alloc,
        OC740_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70
    );

    oc740_fake!(
        oc740_fake_anim_ctor,
        OC740_PREV_ANIM_CTOR,
        0x0035_8c80,
        ee::s11::f_00358c80
    );

    oc740_fake!(
        oc740_fake_conv_ctor,
        OC740_PREV_CONV_CTOR,
        0x0034_3010,
        ee::s11::f_00343010
    );

    oc740_fake!(
        oc740_fake_boost_ctor,
        OC740_PREV_BOOST_CTOR,
        0x0034_1388,
        ee::s11::f_00341388
    );

    oc740_fake!(
        oc740_fake_position,
        OC740_PREV_POSITION,
        0x0035_58b8,
        ee::s11::f_003558b8
    );

    oc740_fake!(
        oc740_fake_parent,
        OC740_PREV_PARENT,
        0x0035_5978,
        ee::s11::f_00355978
    );

    pub(crate) fn oc740_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc740_fake_attach,
            &OC740_PREV_ATTACH,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc740_fake_alloc,
            &OC740_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00358C80),
            oc740_fake_anim_ctor,
            &OC740_PREV_ANIM_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00343010),
            oc740_fake_conv_ctor,
            &OC740_PREV_CONV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00341388),
            oc740_fake_boost_ctor,
            &OC740_PREV_BOOST_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003558B8),
            oc740_fake_position,
            &OC740_PREV_POSITION,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00355978),
            oc740_fake_parent,
            &OC740_PREV_PARENT,
        );
    }

    /// An instance record at `inst` whose active node (+0xc) is `node` (0: none).
    /// A node's vtable (+0xc of the node) answers the type queries at the
    /// offsets of `fns` (`(adjust offset, fn offset)`): each calls `FAKE` (a
    /// fake returning a pointer) or the leaf that returns 0.
    pub(crate) fn oc740_instance(
        c: &mut Cpu,
        r: &mut Rng,
        inst: u32,
        node: u32,
        fns: &[(u32, u32)],
    ) {
        c.bus.write32(inst + 0xc, node);
        if node == 0 {
            return;
        }
        let vt = node + 0x100;
        c.bus.write32(node + 0xc, vt);
        for (i, &(adj, f)) in fns.iter().enumerate() {
            c.bus.write16(vt + adj, (r.next_u32() % 8 * 4) as u16);
            let target = if r.next_u32() % 2 == 0 {
                OC740_LEAF_ZERO
            } else {
                fakes::FAKE + 0x10 * i as u32
            };
            c.bus.write32(vt + f, target);
        }
    }

    /// The world the factories resolve ids through: id 0 -> instance
    /// `DATA + 0x8000` (section 0 / entry 0), id 0x100 -> a missing entry,
    /// id 5 -> a missing section; the script manager's instance (id -1) at
    /// `DATA + 0xb000`. Both have an active node on `seed` bits.
    pub(crate) fn oc740_world(c: &mut Cpu, r: &mut Rng, seed: u32, fns: &[(u32, u32)]) {
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        c.bus.write32(DATA + 0x5000, DATA + 0x6000);
        c.bus.write32(DATA + 0x601c, DATA + 0x6100);
        c.bus.write32(DATA + 0x6100, ((DATA + 0x8000) >> 2) << 8);
        let node = if (seed / 8) % 4 == 0 {
            0
        } else {
            DATA + 0x9000
        };
        oc740_instance(c, r, DATA + 0x8000, node, fns);
        c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
        c.bus.write32(DATA + 0xa290, DATA + 0xb000);
        let node = if (seed / 32) % 4 == 0 {
            0
        } else {
            DATA + 0xb100
        };
        oc740_instance(c, r, DATA + 0xb000, node, fns);
    }

    /// The script's parameter list at `DATA + 0x1000` (16-byte entries
    /// `(idx, val, ?, type)`): first the `forced` `(idx, value)` entries
    /// (stored as integers: their type equals the table's), then `seed % 3`
    /// random ones; and the type table at `types`.
    pub(crate) fn oc740_params(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        types: u32,
        maxidx: u32,
        forced: &[(u32, u32)],
    ) {
        let table: Vec<u32> = (0..=maxidx).map(|_| r.next_u32() % 3).collect();
        for (i, &t) in table.iter().enumerate() {
            c.bus.write32(types + 4 * i as u32, t);
        }
        let n = forced.len() as u32 + seed % 3;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            let (idx, val, ty) = if (i as usize) < forced.len() {
                let (idx, val) = forced[i as usize];
                (idx, val, table[idx as usize])
            } else {
                (
                    2 + r.next_u32() % (maxidx - 1),
                    r.next_u32(),
                    r.next_u32() % 3,
                )
            };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 0xc, ty);
        }
    }

    /// The id of the target instance a seed asks for: `None` leaves the
    /// template's (-1: the script manager's), else a forced id parameter.
    pub(crate) fn oc740_id(seed: u32) -> Option<u32> {
        match seed % 5 {
            0 | 1 => None,
            2 => Some(0),
            3 => Some(0x100),
            _ => Some(5),
        }
    }

    /// The machine of the three factories: the template's id word and
    /// `flag_gp`, the parameters, the world, and the gated fakes.
    pub(crate) fn oc740_factory_machine(c: &mut Cpu, seed: u32, salt: u64, site: &WfSite) {
        oc740_hook();
        let mut r = machine(c, seed, salt, 3);
        c.bus.write32(OC740_CTRL, OC740_MAGIC);
        c.bus.write32(OC740_CTRL + 4, (seed / 16) % 2);
        c.g[A0][0] = sx(DATA + 0x7000);
        let forced: Vec<(u32, u32)> = oc740_id(seed).map(|v| (0, v)).into_iter().collect();
        oc740_params(c, &mut r, seed, site.fill.table, 5, &forced);
        c.bus
            .write32(site.tpl, if (seed / 4) % 2 == 0 { u32::MAX } else { 0 });
        c.bus.write32(GAME_GP + site.flag_gp, seed % 2);
        oc740_world(c, &mut r, seed, &[]);
    }

    /// Covers the one-time init (flag clear or set), the parameter loop (int
    /// and float stores), every instance lookup exit (-1, id 0, missing entry,
    /// missing section) and the attach / alloc / constructor tails with the
    /// fakes returning and unwinding.
    #[test]
    fn world_obj_factory_anim_combo_matches() {
        fakes::covered(
            ee::s10::f_002fb270__generated,
            world_obj_factory_anim_combo,
            SEEDS,
            &|c, seed| {
                oc740_factory_machine(c, seed, 0x2fb270, &WF740_ANIM);
            },
        );
    }

    #[test]
    fn world_obj_factory_conveyor_matches() {
        fakes::covered(
            ee::s10::f_002fb6b8__generated,
            world_obj_factory_conveyor,
            SEEDS,
            &|c, seed| {
                oc740_factory_machine(c, seed, 0x2fb6b8, &WF740_CONVEYOR);
            },
        );
    }

    #[test]
    fn world_obj_factory_boost_matches() {
        fakes::covered(
            ee::s10::f_002fbec8__generated,
            world_obj_factory_boost,
            SEEDS,
            &|c, seed| {
                oc740_factory_machine(c, seed, 0x2fbec8, &WF740_BOOST);
            },
        );
    }

    /// `WScriptFn_Modifier9`: the template, the 16-byte buffer, the lookup and
    /// the attach (type 9) when the instance has an active node.
    #[test]
    fn wscript_fn_modifier9_matches() {
        fakes::covered(
            ee::s10::f_002fcc20__generated,
            wscript_fn_modifier9,
            SEEDS,
            &|c, seed| {
                oc740_hook();
                let mut r = machine(c, seed, 0x2fcc20, 2);
                c.bus.write32(OC740_CTRL, OC740_MAGIC);
                c.bus.write32(OC740_CTRL + 4, (seed / 16) % 2);
                // a0: hidden return slot, a1: parameter count, a2: parameters.
                c.g[A0][0] = sx(DATA + 0x7000);
                let forced: Vec<(u32, u32)> = oc740_id(seed).map(|v| (0, v)).into_iter().collect();
                oc740_params(c, &mut r, seed, 0x0044_6028, 3, &forced);
                c.bus
                    .write32(0x004f_b5b0, if (seed / 4) % 2 == 0 { u32::MAX } else { 0 });
                c.bus.write32(GAME_GP + 0x2504, seed % 2);
                oc740_world(c, &mut r, seed, &[]);
            },
        );
    }

    /// `WScriptFn_AddPositionModifier`: lookup, the node's type query (a real
    /// leaf returning 0 or a fake), the target id (-1 or an object index),
    /// the object table, the two vtable queries, the `VU0` product and the
    /// position-modifier call.
    #[test]
    fn wscript_fn_add_position_modifier_matches() {
        fakes::covered(
            ee::s10::f_002fd758__generated,
            wscript_fn_add_position_modifier,
            SEEDS,
            &|c, seed| {
                oc740_hook();
                let mut r = machine(c, seed, 0x2fd758, 5);
                c.bus.write32(OC740_CTRL, OC740_MAGIC);
                c.g[A0][0] = sx(DATA + 0x7000);
                let forced: Vec<(u32, u32)> = match oc740_id(seed) {
                    Some(v) => vec![(0, v)],
                    None => vec![],
                };
                oc740_params(c, &mut r, seed, 0x0044_6190, 5, &forced);
                // The template's target id (word 1): -1 (none) or object 3.
                let tpl = 0x004f_b720;
                c.bus.write32(tpl, u32::MAX);
                c.bus
                    .write32(tpl + 4, if (seed / 4) % 3 == 0 { u32::MAX } else { 3 });
                c.bus.write32(GAME_GP + 0x2518, seed % 2);
                oc740_world(c, &mut r, seed, &[(0x80, 0x84)]);
                // The target object: gp-0x848 -> world; world+0x84 -> table whose
                // entry 3 (+0xc) has an object at +0x28 with a vtable.
                c.bus.write32(GAME_GP - 0x848, DATA + 0xc000);
                c.bus.write32(DATA + 0xc084, DATA + 0xc100);
                c.bus.write32(DATA + 0xc10c, DATA + 0xc200);
                c.bus.write32(DATA + 0xc200 + 12 + 0x28, DATA + 0xc300);
                let vt = DATA + 0xc400;
                c.bus.write32(DATA + 0xc300 + 0x6c0, vt);
                c.bus.write16(vt + 0x28, 4);
                c.bus.write32(vt + 0x2c, fakes::FAKE + 0x20);
                c.bus.write16(vt + 0x30, 8);
                c.bus.write32(vt + 0x34, fakes::FAKE + 0x30);
            },
        );
    }

    /// `WScriptFn_AddParentModifier`: both ids (child, parent), the parent's
    /// active node and its vtable query, the child's node and its query, and
    /// the parent-modifier call.
    #[test]
    fn wscript_fn_add_parent_modifier_matches() {
        fakes::covered(
            ee::s10::f_002fd9f8__generated,
            wscript_fn_add_parent_modifier,
            SEEDS,
            &|c, seed| {
                oc740_hook();
                let mut r = machine(c, seed, 0x2fd9f8, 4);
                c.bus.write32(OC740_CTRL, OC740_MAGIC);
                c.g[A0][0] = sx(DATA + 0x7000);
                // The child (id word 0) is -1 (manager) or 0; the parent (word 1)
                // is -1 (none), 0 or a missing entry.
                let mut forced = vec![];
                if seed % 2 == 0 {
                    forced.push((0, 0));
                }
                if seed % 3 != 0 {
                    forced.push((1, if seed % 7 == 0 { 0x100 } else { 0 }));
                }
                // forced entries keep their position: pad the list so that
                // (1, ..) is entry 1 when (0, ..) is absent.
                if forced.len() == 1 && forced[0].0 == 1 {
                    forced.insert(0, (0, u32::MAX));
                }
                oc740_params(c, &mut r, seed, 0x0044_61a8, 5, &forced);
                let tpl = 0x004f_b738;
                c.bus.write32(tpl, u32::MAX);
                c.bus.write32(tpl + 4, u32::MAX);
                c.bus.write32(GAME_GP + 0x251c, seed % 2);
                oc740_world(c, &mut r, seed, &[(0x50, 0x54), (0x80, 0x84)]);
            },
        );
    }

    /// Control words of the gated fakes (clear of the fake call log).
    pub(crate) const WSC_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const WSC_MAGIC: u32 = 0x4f43_3734;

    /// Non-zero makes the `AttachModifier` fake report "not attached".
    pub(crate) const WSC_ATTACH_FAILS: u32 = WSC_CTRL + 4;

    macro_rules! wsc_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(WSC_CTRL) != WSC_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    wsc_fake!(
        oc741_fake_alloc,
        OC741_PREV_ALLOC,
        ee::s10::f_00317d70__generated,
        0x0031_7d70
    );

    wsc_fake!(
        oc741_fake_float_ctor,
        OC741_PREV_FLOAT_CTOR,
        ee::s11::f_00348d98__generated,
        0x0034_8d98
    );

    wsc_fake!(
        oc741_fake_spring_ctor,
        OC741_PREV_SPRING_CTOR,
        ee::s11::f_00349b48__generated,
        0x0034_9b48
    );

    pub(crate) static OC741_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);

    /// `WorldObjFactory_AttachModifier` fake: a pointer result, or 0 when the
    /// fixture asks for "not attached".
    pub(crate) fn oc741_fake_attach(c: &mut Cpu) {
        if c.bus.read32(WSC_CTRL) != WSC_MAGIC {
            return fakes::fall_back(c, &OC741_PREV_ATTACH, ee::s10::f_002fae38__generated);
        }
        fakes::fake_call(c, 0x002f_ae38);
        if c.pc == c.g[RA][0] as u32 && c.bus.read32(WSC_ATTACH_FAILS) != 0 {
            c.g[V0][0] = 0;
        }
    }

    pub(crate) fn oc741_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc741_fake_alloc,
            &OC741_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00348D98),
            oc741_fake_float_ctor,
            &OC741_PREV_FLOAT_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00349B48),
            oc741_fake_spring_ctor,
            &OC741_PREV_SPRING_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc741_fake_attach,
            &OC741_PREV_ATTACH,
        );
    }

    /// The machine state of the four script functions `(ret slot, n, params)`:
    /// `seed % 4` parameters (entry 0 names instance id 0 on `seed % 3 == 0`,
    /// else the id stays -1), the type table, the init flag, and the
    /// world -> section -> instance chain. The instance's node (when there is
    /// one) has a vtable at `DATA + 0x9100` for the caller to fill.
    pub(crate) fn wsc_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        flag_gp: u32,
        types: u32,
        max_idx: u32,
    ) {
        c.bus.write32(WSC_CTRL, WSC_MAGIC);
        c.bus.write32(WSC_ATTACH_FAILS, (seed / 7) % 2);
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA + 0x7000);
        let n = seed % 4;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            let idx = if i == 0 && seed % 3 == 0 {
                0
            } else {
                1 + r.next_u32() % max_idx
            };
            let val = if idx == 0 { 0 } else { r.next_u32() % 3 };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..5 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(GAME_GP + flag_gp, seed % 2);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        let section = if (seed / 3) % 5 == 2 {
            0
        } else {
            DATA + 0x6000
        };
        c.bus.write32(DATA + 0x5000, section);
        c.bus.write32(DATA + 0x601c, DATA + 0x6100);
        let entry = if (seed / 5) % 6 == 4 {
            0
        } else {
            ((DATA + 0x8000) >> 2) << 8
        };
        c.bus.write32(DATA + 0x6100, entry);
        // The script manager's instance (the id -1 path) is the same record.
        c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
        let rec = if (seed / 11) % 7 == 3 {
            0
        } else {
            DATA + 0x8000
        };
        c.bus.write32(DATA + 0xa290, rec);
        c.bus
            .write32(DATA + 0x800c, if seed % 5 == 1 { 0 } else { DATA + 0x9000 });
        c.bus.write32(DATA + 0x900c, DATA + 0x9100);
    }

    /// `WorldObjFactory_FloatRail`: the one-time template init (flag clear
    /// or set), the parameter loop (int and float stores), the id -1 and
    /// section lookups (with missing section, entry and record), and the node
    /// tail: attach succeeds, attach fails (alloc + constructor), no node.
    #[test]
    fn world_obj_factory_float_rail_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_002ff390__generated,
            world_obj_factory_float_rail,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff3_90, 3);
                wsc_fixture(c, &mut r, seed, 0x2544, 0x0044_6480, 3);
                c.pc = 0x002f_f390;
            },
        );
    }

    /// `WorldObjFactory_SpringRail`: as the float rail, with the spring
    /// node's scaled float.
    #[test]
    fn world_obj_factory_spring_rail_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_002ff5e8__generated,
            world_obj_factory_spring_rail,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff5_e8, 3);
                wsc_fixture(c, &mut r, seed, 0x2548, 0x0044_64d8, 3);
                // The record's float at +0x50 gets scaled: give it a value.
                c.bus.write32(0x004f_ba90 + 0x50, 0x3f80_0000 ^ (seed << 8));
                c.pc = 0x002f_f5e8;
            },
        );
    }

    /// `WScriptFn_Unhide`: init flag clear or set, parameters, both lookups;
    /// the node's `isType(0x10)` is a leaf returning 0 or a fake (true), then
    /// the `delete(3)` fake.
    #[test]
    fn wscript_fn_unhide_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_002ffd58__generated,
            wscript_fn_unhide,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ff_fd58, 2);
                wsc_fixture(c, &mut r, seed, 0x2570, GAME_GP + 0xb30, 1);
                let vt = DATA + 0x9100;
                c.bus.write16(vt + 0x50, ((seed % 3) * 4) as u16);
                c.bus.write32(
                    vt + 0x54,
                    if seed % 4 == 3 {
                        0x0014_06c0
                    } else {
                        fakes::FAKE
                    },
                );
                c.bus.write16(vt + 8, (((seed / 3) % 3) * 4) as u16);
                c.bus.write32(vt + 0xc, fakes::FAKE + 0x10);
                c.pc = 0x002f_fd58;
            },
        );
    }

    /// `WScriptFn_KillRestorable`: init flag, parameters (the expire type is
    /// parameter index 1: 0, 1 or 2), both lookups, the `expire` fakes.
    #[test]
    fn wscript_fn_kill_restorable_matches() {
        oc741_install_fakes();
        fakes::covered(
            ee::s10::f_00300418__generated,
            wscript_fn_kill_restorable,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x300_418, 2);
                wsc_fixture(c, &mut r, seed, 0x2580, GAME_GP + 0xb40, 2);
                let vt = DATA + 0x9100;
                c.bus.write16(vt + 0x110, ((seed % 3) * 4) as u16);
                c.bus.write32(vt + 0x114, fakes::FAKE);
                c.pc = 0x0030_0418;
            },
        );
    }

    // Fixture objects, in DATA+0x200000 and up (clear of the fake log).
    pub(crate) const OC742_WORLD: u32 = DATA + 0x21_0000;

    pub(crate) const OC742_OBJ: u32 = DATA + 0x22_0000;

    pub(crate) const OC742_ARR: u32 = DATA + 0x23_0000;

    pub(crate) const OC742_RIDER: u32 = DATA + 0x24_0000;

    pub(crate) const OC742_VT2: u32 = DATA + 0x25_0000;

    pub(crate) const OC742_PARAMS: u32 = DATA + 0x26_0000;

    pub(crate) const OC742_SRET: u32 = DATA + 0x27_0000;

    pub(crate) const OC742_TAGS: u32 = DATA + 0x28_0000;

    pub(crate) const OC742_TAGS2: u32 = DATA + 0x28_1000;

    pub(crate) const OC742_TAGTBL: u32 = DATA + 0x29_0000;

    pub(crate) const OC742_ENTRY: u32 = DATA + 0x2a_0000;

    pub(crate) const OC742_ENTRY2: u32 = DATA + 0x2b_0000;

    pub(crate) const OC742_PREV: u32 = DATA + 0x2c_0000;

    pub(crate) const OC742_OBJ2: u32 = DATA + 0x2d_0000;

    pub(crate) const OC742_EOBJ: u32 = DATA + 0x2e_0000;

    pub(crate) const OC742_EVT: u32 = DATA + 0x2f_0000;

    /// The globals and objects the two bodies read, varied by `seed`: the
    /// world and its rider array, the zone-tag tables, the current-object
    /// record at `gp+0xce8` (rider index, tagged object and its vtable
    /// chain), the once-init guard at `gp+guard`, the kind table at `table`
    /// and four argument records (the first often x = -1).
    pub(crate) fn oc742_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        guard: u32,
        table: u32,
        rider_vt: u32,
    ) {
        let mut r = machine(c, seed, salt, 4);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x848), OC742_WORLD);
        c.bus.write32(OC742_WORLD + 0x84, OC742_OBJ);
        c.bus.write32(OC742_OBJ + 0xc, OC742_ARR);
        for i in 0..8u32 {
            c.bus.write32(OC742_ARR + 0x28 + 4 * i, OC742_RIDER);
        }
        c.bus.write32(OC742_RIDER + 0x6c0, OC742_VT2);
        c.bus.write16(
            OC742_VT2 + rider_vt,
            (r.next_u32() as i16 as i32 & !3) as u16,
        );
        c.bus.write32(OC742_VT2 + rider_vt + 4, fakes::FAKE + 0x10);
        // The tag tables.
        c.bus.write32(gp.wrapping_add(0x16c8), OC742_TAGS);
        c.bus.write32(OC742_TAGS, OC742_TAGS2);
        c.bus.write32(OC742_TAGS2 + 0x8, OC742_TAGTBL);
        for i in 0..256u32 {
            let e = if r.next_u32() % 4 == 0 {
                0
            } else {
                OC742_ENTRY
            };
            c.bus.write32(OC742_TAGTBL + 4 * i, e);
        }
        c.bus.write32(OC742_ENTRY + 0x1c, OC742_ENTRY2);
        for i in 0..0x100u32 {
            c.bus
                .write32(OC742_ENTRY2 + 4 * i, (r.next_u32() & 0xff) << 8);
        }
        // The current-object record: rider index, tagged object chain.
        c.bus.write32(gp.wrapping_add(0xce8), OC742_PREV);
        let idx = [u32::MAX, 0, 1, 2, 3, 5][(r.next_u32() % 6) as usize];
        c.bus.write32(OC742_PREV, idx);
        c.bus.write32(
            OC742_PREV + 0x290,
            if r.next_u32() % 4 == 0 { 0 } else { OC742_OBJ2 },
        );
        c.bus.write32(
            OC742_OBJ2 + 0xc,
            if r.next_u32() % 4 == 0 { 0 } else { OC742_EOBJ },
        );
        c.bus.write32(OC742_EOBJ + 0xc, OC742_EVT);
        // Vtable slots of the object's script part: {half offset, method}
        // at +0xc0 (Reset/Place) and +0x118 (Kill); methods are fakes.
        for (k, off) in [0xc0u32, 0x118].into_iter().enumerate() {
            c.bus
                .write16(OC742_EVT + off, (r.next_u32() as i16 as i32 & !3) as u16);
            c.bus
                .write32(OC742_EVT + off + 4, fakes::FAKE + 0x20 + 0x10 * k as u32);
        }
        c.bus.write32(gp.wrapping_add(guard), r.next_u32() & 1);
        for k in 0..4u32 {
            c.bus.write32(table + 4 * k, r.next_u32() % 3);
        }
        for i in 0..4u32 {
            let p = OC742_PARAMS + 16 * i;
            let first = i == 0;
            c.bus.write32(
                p,
                if first && r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32() % 4
                },
            );
            let v = match r.next_u32() % 6 {
                0 | 1 if first => u32::MAX,
                0 => u32::MAX,
                1 => 0,
                2 => 1,
                3 => 2,
                4 => 3,
                _ => r.next_u32() & 0xffff,
            };
            c.bus.write32(p + 0x4, v);
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
        let count = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
        c.g[A0][0] = sx(OC742_SRET);
        c.g[A1][0] = count;
        c.g[A2][0] = sx(OC742_PARAMS);
    }

    /// `WScriptFn_Kill`: block guard `gp+0x2590`, kill method at `+0x118`.
    #[test]
    fn wscript_fn_kill_matches() {
        check(
            ee::s10::f_003005e8__generated,
            wscript_fn_kill,
            400,
            &|c, seed| {
                oc742_fixture(
                    c,
                    seed,
                    0x0030_05e8,
                    0x2590,
                    GAME_GP.wrapping_add(0xb48),
                    0x60,
                );
                c.pc = 0x0030_05e8;
            },
        );
    }

    /// `WScriptFn_ResetRiderAtObject`: block guard `gp+0x25a0`, rider
    /// vtable slot at `+0x50`.
    #[test]
    fn wscript_fn_reset_rider_at_object_matches() {
        check(
            ee::s10::f_00300770__generated,
            wscript_fn_reset_rider_at_object,
            400,
            &|c, seed| {
                oc742_fixture(
                    c,
                    seed,
                    0x0030_0770,
                    0x25a0,
                    GAME_GP.wrapping_add(0xb50),
                    0x50,
                );
                c.pc = 0x0030_0770;
            },
        );
    }

    pub(crate) const OC744_PARAMS: u32 = DATA + 0x25_0000;

    pub(crate) const OC744_SRET: u32 = DATA + 0x26_0000;

    /// The word the gated fake looks for (other tests' machines lack it).
    pub(crate) const OC744_GATE: u32 = DATA + 0x3f_0000;

    pub(crate) const OC744_MAGIC: u32 = 0x0744_0744;

    pub(crate) static OC744_PREV_PROCESS: AtomicUsize = AtomicUsize::new(0);

    /// `WScriptMan_ProcessCommand` (0x30a728), gated: logs the call and returns.
    pub(crate) fn oc744_fake_process(c: &mut Cpu) {
        if c.bus.read32(OC744_GATE) != OC744_MAGIC {
            return fakes::fall_back(c, &OC744_PREV_PROCESS, ee::s10::f_0030a728__generated);
        }
        fakes::fake_call(c, 0x0030_a728);
    }

    /// `WScriptFn_NextTask`: first call (record defaults set) or later ones,
    /// 0..3 and -1 arguments whose type matches the table (integer store),
    /// differs from it (integer store) or is the float slot type (converted).
    #[test]
    fn w_script_fn_next_task_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030A728),
            oc744_fake_process,
            &OC744_PREV_PROCESS,
        );
        check(
            ee::s10::f_00302df0__generated,
            w_script_fn_next_task,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_2df0, 2);
                let gp = GAME_GP;
                c.bus.write32(gp + 0x26b0, r.next_u32() & 1);
                c.bus.write32(gp + 0x26a8, r.next_u32());
                c.bus.write8(gp + 0x26ac, r.next_u32() as u8);
                for i in 0..4u32 {
                    c.bus.write32(gp + 0xc18 + 4 * i, r.next_u32() % 4);
                }
                for i in 0..4u32 {
                    let p = OC744_PARAMS + 16 * i;
                    c.bus.write32(p, r.next_u32() % 4);
                    c.bus.write32(p + 4, r.next_u32() & 0xffff);
                    c.bus.write32(p + 0xc, r.next_u32() % 4);
                }
                c.bus.write32(gp + 0xce8, DATA + 0x27_0000);
                c.g[A0][0] = sx(OC744_SRET);
                c.g[A1][0] = [0u64, 1, 2, 3, 4, u64::MAX][(seed % 6) as usize];
                c.g[A2][0] = sx(OC744_PARAMS);
                c.bus.write32(OC744_GATE, OC744_MAGIC);
                c.pc = 0x0030_2df0;
            },
        );
    }

    /// The result slot every fixture passes as `a0`.
    pub(crate) const OC745_OUT: u32 = DATA + 0x4000;

    /// The script manager (`gp+0xce8`), the world cache (`gp+0x16c8`) and its
    /// section and tag tables.
    pub(crate) const OC745_MGR: u32 = DATA + 0x1400;

    pub(crate) const OC745_WC: u32 = DATA + 0x1800;

    pub(crate) const OC745_WC_X: u32 = DATA + 0x1810;

    pub(crate) const OC745_SECT: u32 = DATA + 0x1820;

    pub(crate) const OC745_SEC0: u32 = DATA + 0x1830;

    pub(crate) const OC745_TAGS: u32 = DATA + 0x1860;

    /// The instance records the tags point at.
    pub(crate) const OC745_OBJ: u32 = DATA + 0x2400;

    /// The argument list (16-byte entries).
    pub(crate) const OC745_PARAMS: u32 = DATA + 0x2000;

    /// The fakes' gate word in the machine's RAM.
    pub(crate) const OC745_CTRL: u32 = DATA + 0x3_0000;

    pub(crate) const OC745_MAGIC: u32 = 0x0c74_5c0d;

    /// The crowd function's static block.
    pub(crate) const OC745_STATIC: u32 = 0x4f_bc70;

    macro_rules! oc745_fakes {
        ($($name:ident $slot:ident $gen:ident $addr:literal;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC745_CTRL) != OC745_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::s11::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra)
                }
            )*
            fn oc745_install_fakes() {
                $(
                    fakes::hook(addr_of_mut!(ee::s11::$slot), $name, fakes::prev_cell($name));
                )*
            }
        };
    }

    oc745_fakes! {
        oc745_fake_add_instance OVERRIDE_00343C60 f_00343c60__generated 0x0034_3c60;
        oc745_fake_remove_instance OVERRIDE_00343F38 f_00343f38__generated 0x0034_3f38;
    }

    /// The manager, the world-cache chain and the instance records. Ids are
    /// `section | tag_index << 8`; sections 0..3, tags 0..3.
    pub(crate) fn oc745_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(GAME_GP + 0xce8, OC745_MGR);
        fill(c, r, OC745_MGR, 0x300);
        let inst = if seed % 7 == 0 { 0 } else { OC745_OBJ };
        c.bus.write32(OC745_MGR + 0x290, inst);
        fill(c, r, OC745_OBJ, 0x100);
        c.bus.write32(GAME_GP + 0x16c8, OC745_WC);
        c.bus.write32(OC745_WC, OC745_WC_X);
        c.bus.write32(OC745_WC_X + 8, OC745_SECT);
        for sec in 0..3u32 {
            let present = (seed / 3 + sec) % 4 != 0;
            c.bus.write32(
                OC745_SECT + 4 * sec,
                if present { OC745_SEC0 + 0x10 * sec } else { 0 },
            );
            c.bus
                .write32(OC745_SEC0 + 0x10 * sec + 0x1c, OC745_TAGS + 0x10 * sec);
            for t in 0..3u32 {
                let k = (seed + sec + t) % 5;
                let tag = if k == 0 {
                    r.next_u32() & 0xff
                } else {
                    (((OC745_OBJ + 4 * k) / 4) << 8) | (r.next_u32() & 0xff)
                };
                c.bus.write32(OC745_TAGS + 0x10 * sec + 4 * t, tag);
            }
        }
        c.bus.write32(GAME_GP + 0xf00, DATA + 0x5000);
    }

    /// A target id: the manager's own (-1) or `section | tag << 8`.
    pub(crate) fn oc745_id(r: &mut Rng, seed: u32) -> u32 {
        if seed % 4 == 0 {
            u32::MAX
        } else {
            (r.next_u32() % 3) | ((r.next_u32() % 3) << 8)
        }
    }

    /// The kind table (index 2 is a float) and `count` arguments: index
    /// `j % 3` with an id, a command or a random int, and a random kind.
    pub(crate) fn oc745_args(c: &mut Cpu, r: &mut Rng, seed: u32, kinds: u32, count: u32) {
        c.g[A1][0] = count as u64;
        c.g[A2][0] = sx(OC745_PARAMS);
        c.bus.write32(GAME_GP + kinds, r.next_u32() % 2);
        c.bus.write32(GAME_GP + kinds + 4, r.next_u32() % 2);
        c.bus.write32(GAME_GP + kinds + 8, 2);
        for j in 0..count {
            let e = OC745_PARAMS + 16 * j;
            let idx = (j + seed / 5) % 3;
            let val = match idx {
                0 => oc745_id(r, seed / 2 + j),
                1 => r.next_u32() % 3,
                _ => r.next_u32(),
            };
            c.bus.write32(e, idx);
            c.bus.write32(e + 0x4, val);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
    }

    /// `WScriptFn_Crowd3dControl`: the static block, the target lookup and
    /// the add/remove command (calls to the crowd manager are fakes).
    #[test]
    fn wscript_fn_crowd3d_control_matches() {
        oc745_install_fakes();
        check(
            ee::s10::f_003039f0__generated,
            wscript_fn_crowd3d_control,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x39f0, 2);
                c.pc = 0x0030_39f0;
                c.bus.write32(OC745_CTRL, OC745_MAGIC);
                c.g[A0][0] = sx(OC745_OUT);
                oc745_world(c, &mut r, seed);
                let initialised = seed % 2 == 1;
                c.bus.write32(GAME_GP + 0x26ec, initialised as u32);
                c.bus.write32(OC745_STATIC, oc745_id(&mut r, seed / 2));
                c.bus.write32(OC745_STATIC + 4, (seed / 3) % 3);
                c.bus.write32(OC745_STATIC + 8, r.next_u32());
                oc745_args(c, &mut r, seed, 0xc48, seed % 4);
            },
        );
    }

    /// `WScriptFn_SetHudHeightTarget`: the static block, the target lookup
    /// and the real `WScriptMan_SetHudHeightTarget` call.
    #[test]
    fn wscript_fn_set_hud_height_target_matches() {
        check(
            ee::s10::f_00304320__generated,
            wscript_fn_set_hud_height_target,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x4320, 0);
                c.pc = 0x0030_4320;
                c.g[A0][0] = sx(OC745_OUT);
                oc745_world(c, &mut r, seed);
                let initialised = seed % 2 == 1;
                c.bus.write32(GAME_GP + 0x2708, initialised as u32);
                c.bus.write32(GAME_GP + 0x2700, oc745_id(&mut r, seed / 2));
                c.bus.write32(GAME_GP + 0x2704, r.next_u32());
                oc745_args(c, &mut r, seed, 0xc68, seed % 4);
            },
        );
    }

    /// `count` random parameter entries at `DATA + 0x2100` (`idx` below
    /// `idx_range`, random value and type) and the kind table at `table`.
    pub(crate) fn w746_params(c: &mut Cpu, r: &mut Rng, count: u32, idx_range: u32, table: u32) {
        for i in 0..idx_range {
            c.bus.write32(table + 4 * i, r.next_u32() % 3);
        }
        for i in 0..count.max(4) {
            let p = DATA + 0x2100 + 16 * i;
            c.bus.write32(p, r.next_u32() % idx_range);
            c.bus.write32(p + 4, r.next_u32());
            c.bus.write32(p + 0xc, r.next_u32() % 3);
        }
    }

    /// SetRiderConfigBits: first-call flag, kind table at `0x446650`, random
    /// stack junk under the packed word at `sp+0x60`.
    #[test]
    fn wscript_fn_set_rider_config_bits_matches() {
        check(
            ee::s10::f_003045b8__generated,
            wscript_fn_set_rider_config_bits,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x45b8, 0);
                c.bus.write32(GAME_GP.wrapping_add(0xce8), DATA + 0x4000);
                c.bus
                    .write32(GAME_GP.wrapping_add(0x271c), (seed % 2) as u32);
                let count = seed % 5;
                w746_params(c, &mut r, count, 17, 0x0044_6650);
                c.g[A0][0] = DATA as u64 + 0x2000;
                c.g[A1][0] = count as u64;
                c.g[A2][0] = DATA as u64 + 0x2100;
            },
        );
    }

    /// SetObjectField24: the object id (word 0 of the block) either -1 (the
    /// WScriptMan's instance) or a section/slot id into the world cache
    /// tables; the object's node answers the move-node query through a
    /// vtable slot (a fake, or a real leaf returning 0).
    #[test]
    fn wscript_fn_set_object_field24_matches() {
        covered(
            ee::s10::f_00304980__generated,
            wscript_fn_set_object_field24,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x4980, 1);
                let wsm = DATA + 0x4000;
                let (cache, sections, sect, slots) =
                    (DATA + 0x5000, DATA + 0x5100, DATA + 0x5500, DATA + 0x5900);
                let (obj, node, vt) = (DATA + 0x6000, DATA + 0x6100, DATA + 0x6200);
                c.bus.write32(GAME_GP.wrapping_add(0xce8), wsm);
                c.bus
                    .write32(GAME_GP.wrapping_add(0x2720), (seed % 2) as u32);
                c.bus.write32(GAME_GP.wrapping_add(0x16c8), cache);
                c.bus.write32(cache, cache + 0x40);
                c.bus.write32(cache + 0x40 + 8, sections);
                // sections[0..4]: some missing; each has a slot table at +0x1c.
                for sec in 0..4u32 {
                    let s = if (seed >> sec) % 5 == 0 {
                        0
                    } else {
                        sect + 0x40 * sec
                    };
                    c.bus.write32(sections + 4 * sec, s);
                    c.bus.write32(sect + 0x40 * sec + 0x1c, slots + 0x10 * sec);
                    for i in 0..4u32 {
                        let o = if (seed >> (i + 3)) % 3 == 0 { 0 } else { obj };
                        c.bus.write32(
                            slots + 0x10 * sec + 4 * i,
                            ((o >> 2) << 8) | (r.next_u32() & 0xff),
                        );
                    }
                }
                // The object: the instance word at +0x290 of the WScriptMan,
                // or the slot's record. Its node (+0xc) may be null.
                c.bus.write32(wsm + 0x290, obj);
                c.bus
                    .write32(obj + 0xc, if seed % 7 == 0 { 0 } else { node });
                c.bus.write32(node + 0xc, vt);
                c.bus.write16(vt + 0x80, (r.next_u32() & 0xf0) as u16);
                let leaf = if seed % 3 == 0 {
                    0x0014_06c0
                } else {
                    FAKE + 0x20
                };
                c.bus.write32(vt + 0x84, leaf);
                // The parameters: kind table at gp+0xc78 (3 words).
                w746_params(c, &mut r, 3, 3, GAME_GP.wrapping_add(0xc78));
                // Word 0 (the id): -1, or section/slot.
                for i in 0..4u32 {
                    let p = DATA + 0x2100 + 16 * i;
                    if c.bus.read32(p) == 0 {
                        let id = match (seed / 3) % 3 {
                            0 => u32::MAX,
                            _ => ((r.next_u32() % 4) << 8) | (r.next_u32() % 4),
                        };
                        c.bus.write32(p + 4, id);
                        let kind = c.bus.read32(GAME_GP.wrapping_add(0xc78));
                        c.bus.write32(p + 0xc, kind);
                    }
                }
                c.g[A0][0] = DATA as u64 + 0x2000;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = DATA as u64 + 0x2100;
            },
        );
    }

    /// CameraShake: the block parameters (keyword, useObj, id, scale), the
    /// object (the WScriptMan's instance, or one found through the world
    /// cache) with or without a node, and 0..3 riders (local or not) whose
    /// virtual calls are a fake or a real leaf returning 0.
    #[test]
    fn wscript_fn_camera_shake_matches() {
        covered(
            ee::s10::f_003050f0__generated,
            wscript_fn_camera_shake,
            600,
            &|c, seed| {
                c.pc = 0x0030_50f0;
                let mut r = machine(c, seed, 0x50f0, 2);
                c.pc = 0x0030_50f0;
                let wsm = DATA + 0x4000;
                let (cache, sections, sect, slots) =
                    (DATA + 0x4400, DATA + 0x4500, DATA + 0x4900, DATA + 0x4d00);
                let (obj, node, nvt) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                let (viewman, game, pb, world) =
                    (DATA + 0x5300, DATA + 0x5400, DATA + 0x5500, DATA + 0x5700);
                c.bus.write32(GAME_GP.wrapping_add(0xce8), wsm);
                c.bus
                    .write32(GAME_GP.wrapping_add(0x274c), (seed % 2) as u32);
                c.bus.write32(GAME_GP.wrapping_add(0x16c8), cache);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                // The being index: 0 (matches a leaf-returning rider) or random.
                c.bus
                    .write32(wsm, if seed % 3 == 0 { 0 } else { r.next_u32() % 4 });
                c.bus.write32(wsm + 0x290, obj);
                c.bus.write32(cache, cache + 0x40);
                c.bus.write32(cache + 0x40 + 8, sections);
                for sec in 0..4u32 {
                    let sec_ptr = if (seed >> sec) % 5 == 0 {
                        0
                    } else {
                        sect + 0x40 * sec
                    };
                    c.bus.write32(sections + 4 * sec, sec_ptr);
                    c.bus.write32(sect + 0x40 * sec + 0x1c, slots + 0x10 * sec);
                    for i in 0..4u32 {
                        let o = if (seed >> (i + 3)) % 3 == 0 { 0 } else { obj };
                        c.bus
                            .write32(slots + 0x10 * sec + 4 * i, ((o >> 2) << 8) | 5);
                    }
                }
                // The object's node (maybe none) and its root-transform call.
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(node + 0xc, nvt);
                c.bus.write16(nvt + 0xc0, (r.next_u32() & 0xf0) as u16);
                c.bus.write32(nvt + 0xc4, FAKE + 0x20);
                // The game: world+0x84 -> game; game+0xc -> player block; game+0x84 -> views.
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0xc, pb);
                c.bus.write32(game + 0x84, viewman);
                c.bus.write32(pb + 0x7c, (seed / 2) % 4);
                for v in 0..4u32 {
                    c.bus.write32(viewman + 4 + 4 * v, DATA + 0x5800 + 0x10 * v);
                }
                for i in 0..3u32 {
                    let (player, rider, vt) = (
                        DATA + 0x5600 + 0x40 * i,
                        DATA + 0x6000 + 0x900 * i,
                        DATA + 0x8000 + 0x80 * i,
                    );
                    c.bus.write32(pb + 0x40 + 4 * i, player);
                    c.bus.write32(player + 0x18, rider);
                    c.bus.write32(rider + 0x6c0, vt);
                    c.bus.write32(rider + 0x870, r.next_u32() % 4);
                    c.bus
                        .write32(rider + 0x87c, ((seed >> (i + 1)) % 3 != 0) as u32);
                    c.bus.write16(vt + 0x28, (r.next_u32() & 0x70) as u16);
                    c.bus.write32(vt + 0x2c, FAKE + 0x20);
                    c.bus.write16(vt + 0x38, (r.next_u32() & 0x70) as u16);
                    let ai = if (seed + i) % 2 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x20
                    };
                    c.bus.write32(vt + 0x3c, ai);
                }
                // The shake parameter table.
                c.bus.write32(GAME_GP.wrapping_add(0xae8), r.next_u32());
                // The parameters: every field, kind table at 0x446690.
                w746_params(c, &mut r, 4, 4, 0x0044_6690);
                for i in 0..4u32 {
                    let p = DATA + 0x2100 + 16 * i;
                    c.bus.write32(p, i);
                    let kind = c.bus.read32(0x0044_6690 + 4 * i);
                    c.bus.write32(p + 0xc, kind);
                    let v = match i {
                        0 => (r.next_u32() % 4 == 0) as u32,
                        1 => (r.next_u32() % 4 != 0) as u32,
                        2 => match (seed / 5) % 3 {
                            0 => u32::MAX,
                            _ => ((r.next_u32() % 4) << 8) | (r.next_u32() % 4),
                        },
                        _ => r.next_u32(),
                    };
                    c.bus.write32(p + 4, v);
                }
                c.g[A0][0] = DATA as u64 + 0x2000;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = DATA as u64 + 0x2100;
            },
        );
    }

    /// `WScriptFn_SetAvalancheNodeEmitter`: both init paths, 0-3 parameter
    /// records (idx 0..2 and the id slot, word and float conversions), and the
    /// avalanche lookup with a missing, empty or live section entry. The
    /// avalanche list head (`gp+0x9c0`) is 0 so the real
    /// `Avalanche_SetNodeEmitter` returns at once.
    #[test]
    fn wscript_set_avalanche_node_emitter_matches() {
        check(
            ee::s10::f_00305f40__generated,
            wscript_set_avalanche_node_emitter,
            400,
            &|c, seed| {
                let mut r = Rng(0x305f_40 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_5f40;
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP + NODE_EMITTER_INIT_FLAG, seed % 2);
                c.bus.write32(GAME_GP + 0x9c0, 0);
                // Template contents (read when the init is skipped).
                for i in 0..0x38u32 {
                    c.bus.write32(NODE_EMITTER_TEMPLATE + 4 * i, r.next_u32());
                }
                // Section table: gp+0x16c8 -> table -> records -> entries.
                c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(DATA + 0x4008, DATA + 0x5000);
                for j in 0..4u32 {
                    let sec = DATA + 0x6000 + 0x100 * j;
                    c.bus.write32(
                        DATA + 0x5000 + 4 * j,
                        if (seed + j) % 5 == 0 { 0 } else { sec },
                    );
                    c.bus.write32(sec + 0x1c, DATA + 0x7000 + 0x100 * j);
                    for m in 0..4u32 {
                        let e = if (seed + m + j) % 3 == 0 {
                            0
                        } else {
                            (1 + r.next_u32() % 255) << 8
                        };
                        c.bus.write32(DATA + 0x7000 + 0x100 * j + 4 * m, e);
                    }
                }
                // Type table (idx 0..3 and the id slot 54).
                for i in 0..4u32 {
                    c.bus.write32(NODE_EMITTER_TYPES + 4 * i, r.next_u32() % 3);
                }
                c.bus.write32(
                    NODE_EMITTER_TYPES + 4 * (NODE_EMITTER_ID / 4),
                    r.next_u32() % 3,
                );
                let n = seed / 2 % 4;
                for i in 0..n {
                    let e = DATA + 0x9100 + 16 * i;
                    let id_slot = i == n - 1 && seed % 3 != 0;
                    c.bus
                        .write32(e, if id_slot { NODE_EMITTER_ID / 4 } else { i });
                    let key = (((seed / 8) % 4) << 8) | ((seed / 32) % 4);
                    c.bus
                        .write32(e + 4, if id_slot { key } else { r.next_u32() });
                    c.bus.write32(e + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    /// `UIState_HideObject(state, name)`: walks the state's object list for the
    /// first screen object whose hash name matches and calls its virtual
    /// `hide` (slot `+0x4c`, this adjusted by the `lh` at `+0x48`). Paths:
    /// empty list, a matching node, a node without a screen (list end).
    #[test]
    fn ui_state_hide_object_matches() {
        check(
            ee::s13::f_0039e8b8__generated,
            crate::game::ui::uistate::ui_state_hide_object,
            hide::SEEDS,
            &|c, seed| {
                machine(c, seed, 0x39e8, 0);
                c.pc = 0x0039_e8b8;
                hide::env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                hide::cstr(c, name, "OBJ_HELLO");
                let list = this + 0x24;
                let sentinel = DATA + 0x1000;
                let node = DATA + 0x1100;
                hide::w32(c, sentinel + 4, sentinel);
                match seed % 3 {
                    0 => {
                        hide::w32(c, list + 4, sentinel);
                    }
                    1 => {
                        let obj = DATA + 0x1200;
                        let vt = DATA + 0x1300;
                        hide::w32(c, node, sentinel);
                        hide::w32(c, node + 4, sentinel);
                        hide::w32(c, list + 4, node);
                        hide::w32(c, node + 0x38, DATA + 0x1400);
                        hide::w32(c, node + 0x3c, 1);
                        hide::w32(c, DATA + 0x1400, obj);
                        let hv = hide::hash(c, name);
                        hide::w32(c, obj + 0x38, hv);
                        hide::w32(c, obj + 8, vt);
                        hide::slot(c, vt, 0x48, hide::EMPTY);
                    }
                    _ => {
                        hide::w32(c, node, sentinel);
                        hide::w32(c, node + 4, sentinel);
                        hide::w32(c, list + 4, node);
                        hide::w32(c, node + 0x3c, 0);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::check_reaching;
    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as TEST_SP;

    pub(crate) use crate::game::ai::rider::tests::oc1490_machine;

    pub(crate) use crate::game::util::menu::tests::oc1564_install;
    pub(crate) use crate::game::util::menu::tests::oc1564_machine;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1476_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC1476_MAGIC: u32 = 0x1476_f00d;

    /// Fixture objects, clear of the allocator pools, the arena and the call log.
    pub(crate) const OC1476_BASE: u32 = DATA + 0x9_0000;

    /// The static particle-type record the function copies into its new object.
    pub(crate) const OC1476_STATIC: u32 = 0x004b_8858;

    macro_rules! oc1476_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1476_CTRL) != OC1476_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1476_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1476_fakes! {
        oc1476_fake_add_modifier, 0x0035_5cb0, ee::s11::OVERRIDE_00355CB0, ee::s11::f_00355cb0__generated;
        oc1476_fake_particle_ctor, 0x0035_78a8, ee::s11::OVERRIDE_003578A8, ee::s11::f_003578a8__generated;
    }

    /// A fixture for `WorldObjFactory_AddParticleData`: the static-init flag, the
    /// type lookup (direct or through the index chain, or an empty entry), the
    /// object with or without a source, the two virtual probes (the first one
    /// may answer 0), the viewer-distance loops over the world's two object
    /// lists, and the entry table. The seed's hash picks each choice.
    pub(crate) fn oc1476_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        oc1476_hook();
        let mut r = fakes::machine_rt(c, seed, 0x1476, 8);
        elf_image(c);
        c.bus.write32(OC1476_CTRL, OC1476_MAGIC);
        let h = fakes::mix(seed as u64 ^ 0x1476_0000);
        let bit = |n: u32| (h >> n) & 1 != 0;
        let gp = GAME_GP;
        let b = OC1476_BASE;
        c.bus.write32(gp + 0x2538, bit(0) as u32);
        // Type lookup.
        let obj4 = b + 0x1000;
        let s2obj = b + 0x2000;
        let vt = b + 0x3000;
        let (p16, root, tab, ent, subt, g) = (
            b + 0x6000,
            b + 0x6100,
            b + 0x6200,
            b + 0x6300,
            b + 0x6400,
            b + 0x6500,
        );
        let a2val: u32 = if bit(1) { 0xffff_ffff } else { 0x201 };
        c.bus.write32(OC1476_STATIC + 0xd8, a2val);
        c.bus.write32(gp + 0x16c8, p16);
        c.bus.write32(p16, root);
        c.bus.write32(root + 8, tab);
        c.bus.write32(tab + 4, if bit(2) { 0 } else { ent });
        c.bus.write32(ent + 0x1c, subt);
        c.bus.write32(subt + 8, (obj4 >> 2) << 8);
        c.bus.write32(gp + 0xce8, g);
        c.bus.write32(g + 0x290, obj4);
        // The type object and its source.
        c.bus.write32(obj4 + 0xc, if bit(3) { 0 } else { s2obj });
        for i in 0..4 {
            let q = r.f32(50.0);
            c.bus.write32(obj4 + 0x40 + 4 * i, q);
        }
        c.bus.write32(s2obj + 0xc, vt);
        c.bus.write16(vt + 0x80, 0);
        c.bus.write32(
            vt + 0x84,
            if bit(4) {
                0x0014_06c0
            } else {
                fakes::FAKE + 0x10
            },
        );
        c.bus.write16(vt + 0xc0, 0);
        c.bus.write32(vt + 0xc4, fakes::FAKE + 0x20);
        // The renderer flag object and the world's object lists.
        let o = b + 0x4000;
        c.bus.write32(gp.wrapping_sub(0x84c), o);
        c.bus.write32(o + 0x550, if bit(5) { 2 } else { 0 });
        let world = b + 0x5000;
        let (wa, wb, fobj, xobj) = (b + 0x5100, b + 0x5200, b + 0x5800, b + 0x5900);
        c.bus.write32(gp.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, wa);
        c.bus.write32(wa + 0xc, wb);
        c.bus.write32(wb + 0x7c, ((h >> 8) % 4) as u32);
        c.bus.write32(wb + 0x88, ((h >> 10) % 4) as u32);
        // Three entries per list at most: slot 3 of the second list (wb+0x7c) is the first count.
        for i in 0..3u32 {
            c.bus.write32(wb + 0x40 + 4 * i, b + 0x5400 + 0x40 * i);
            c.bus.write32(wb + 0x70 + 4 * i, b + 0x5600 + 0x40 * i);
            c.bus.write32(b + 0x5400 + 0x40 * i + 0x18, fobj);
            c.bus.write32(b + 0x5600 + 0x40 * i + 0x18, fobj);
        }
        c.bus.write32(fobj + 0x6c0, xobj);
        c.bus.write16(xobj + 0x28, 0);
        c.bus.write32(xobj + 0x2c, fakes::FAKE + 0x30);
        // The distance thresholds the loops load into f20 (a pool constant per
        // loop): huge ends the loop at once, negative lets it run through.
        let lim = |on: bool| {
            if on {
                1.0e30f32.to_bits()
            } else {
                (-1.0f32).to_bits()
            }
        };
        for (off, on) in [
            (0x337c, bit(6)),
            (0x3378, bit(7)),
            (0x3374, bit(6)),
            (0x3370, bit(7)),
        ] {
            c.bus.write32(gp.wrapping_sub(off), lim(on));
        }
        // The entry table (16-byte entries: slot, value, -, expected table word).
        let count = 1 + ((h >> 12) % 3) as u32;
        let entries = b + 0x7100;
        let tbl = 0x0044_62a0u32;
        let mut idx = [0u32; 3];
        for slot in idx.iter_mut().take(count as usize) {
            *slot = r.next_u32() % 8;
            let tv = [1u32, 2, 5][(r.next_u32() % 3) as usize];
            c.bus.write32(tbl + 4 * *slot, tv);
        }
        for i in 0..count {
            let e = entries + 16 * i;
            let tv = c.bus.read32(tbl + 4 * idx[i as usize]);
            c.bus.write32(e, idx[i as usize]);
            let v = r.next_u32();
            c.bus.write32(e + 4, v);
            c.bus
                .write32(e + 0xc, if r.next_u32() % 2 == 0 { tv } else { tv + 1 });
        }
        c.g[A0][0] = (b + 0x7000) as u64;
        c.g[A1][0] = count as u64;
        c.g[A2][0] = entries as u64;
        c.pc = 0x002f_e840;
        r
    }

    /// Static init, type lookup (direct, chained, empty), both loop pairs with
    /// early exits, the "no source" static path, and the delete / hand-off exits.
    #[test]
    fn wof_add_particle_data_matches() {
        check_reaching(
            ee::s10::f_002fe840__generated,
            wof_add_particle_data,
            240,
            &[
                0x002f_eae4,
                0x002f_eb00,
                0x002f_eb70,
                0x002f_ec18,
                0x002f_ec88,
                0x002f_ed00,
                0x002f_eda8,
                0x002f_ee3c,
            ],
            &|c, seed| {
                oc1476_setup(c, seed);
            },
        );
    }

    /// The seeds take many distinct paths.
    #[test]
    fn wof_add_particle_data_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..64 {
            let mut c = crate::difftest::fresh();
            oc1476_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s10::f_002fe840__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 8, "only {} distinct paths", seen.len());
    }

    /// Events delivered or unwinding at the loop back edges.
    #[test]
    fn wof_add_particle_data_events() {
        check_events(
            ee::s10::f_002fe840__generated,
            wof_add_particle_data,
            120,
            &|c, seed| {
                let mut r = oc1476_setup(c, seed);
                schedule(c, &mut r, seed, 0x002f_e840)
            },
        );
    }

    /// `WScriptFn_AddMultiSplineModifier`: first call (static init) and
    /// later calls, 0..6 parameter entries (float-typed slots included), the
    /// instance found by the script manager (id -1) or the world cache
    /// (section missing, tag zero, found), no/some node, `isMoveNode` false
    /// (the real leaf 0x1406c0) or true (a fake), and `addSpline` faked.
    #[test]
    fn wscript_add_multi_spline_modifier_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc1490_machine(c, seed, 0xe0c0, 2);
            let (out, params) = (DATA + 0x100, DATA + 0x200);
            let bits = r.next_u32();
            let n = [0i32, 1, 2, 4, 6, -1][(seed % 6) as usize];
            c.g[A0][0] = out as u64;
            c.g[A1][0] = n as i64 as u64;
            c.g[A2][0] = params as u64;
            // The static block's first-call flag, the block itself, and the
            // expected-argument-type table.
            c.bus.write32(GAME_GP + 0x2528, seed & 1);
            let stat = 0x004f_b7a8;
            fill(c, &mut r, stat, 13);
            if bits & 1 != 0 {
                c.bus.write32(stat, 0xffff_ffff);
            } else {
                c.bus.write32(stat, 0x507);
            }
            if bits & 2 != 0 {
                c.bus.write32(stat + 4, 0xffff_ffff);
            }
            for i in 0..13 {
                let t = r.next_u32() % 4;
                c.bus.write32(0x0044_6208 + 4 * i, t);
            }
            for i in 0..n.max(0) as u32 {
                let e = params + 16 * i;
                let sel = r.next_u32();
                let idx = match sel % 6 {
                    0 => 0,
                    1 => 1,
                    _ => (sel >> 8) % 13,
                };
                let val = if idx <= 1 && sel & 0x100 != 0 {
                    0xffff_ffff
                } else if idx == 0 {
                    0x507
                } else {
                    r.next_u32()
                };
                let junk = r.next_u32();
                c.bus.write32(e, idx);
                c.bus.write32(e + 4, val);
                c.bus.write32(e + 8, junk);
                c.bus.write32(e + 0xc, (sel >> 16) % 4);
            }
            // The world: cache -> section table -> section -> tag table.
            let (world, cache, sections, section, tags) = (
                DATA + 0x1_0000,
                DATA + 0x1_0100,
                DATA + 0x1_0200,
                DATA + 0x1_0300,
                DATA + 0x1_0400,
            );
            let (inst, node, vt) = (DATA + 0x1_1000, DATA + 0x1_2000, DATA + 0x1_2100);
            c.bus.write32(GAME_GP + 0x16c8, world);
            c.bus.write32(world, cache);
            c.bus.write32(cache + 8, sections);
            c.bus
                .write32(sections + 4 * 7, if bits & 4 != 0 { section } else { 0 });
            c.bus.write32(section + 0x1c, tags);
            let tag = if bits & 8 != 0 {
                (inst >> 2) << 8 | 0x2a
            } else {
                0x2a
            };
            c.bus.write32(tags + 4 * 5, tag);
            // The script manager's instance.
            c.bus.write32(GAME_GP + 0xce8, DATA + 0x1_3000);
            c.bus.write32(DATA + 0x1_3000 + 0x290, inst);
            c.bus
                .write32(inst + 0xc, if bits & 16 != 0 { node } else { 0 });
            c.bus.write32(node + 0xc, vt);
            c.bus
                .write16(vt + 0x80, if bits & 32 != 0 { 0x10 } else { 0 });
            let is_move_node = if bits & 64 != 0 {
                fakes::FAKE
            } else {
                0x0014_06c0
            };
            c.bus.write32(vt + 0x84, is_move_node);
            // The nil value copied to the result.
            fill(c, &mut r, 0x004c_9098, 3);
        };
        // Few seeds reach the faked callees, so `covered`'s call-count floor does
        // not apply; `check_reaching` below runs the diff and requires both calls.
        check_reaching(
            ssx3_ee::s10::f_002fe0c0__generated,
            wscript_add_multi_spline_modifier,
            200,
            &[0x002f_e270, 0x002f_e280],
            &setup,
        );
    }

    pub(crate) const OC1491_GATE: u32 = DATA + 0x2c_0000;

    pub(crate) const OC1491_MAGIC: u32 = 0x1491_0001;

    pub(crate) fn oc1491_no_prev(_c: &mut Cpu) {
        unreachable!("oc1491 fake called before it was hooked");
    }

    /// Gated fake of `MultiParticleMan_CreateSlot` (0x357a78): logs and returns.
    pub(crate) fn oc1491_fake_create_slot(c: &mut Cpu) {
        if c.bus.read32(OC1491_GATE) != OC1491_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1491_fake_create_slot), oc1491_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0035_7a78, ra);
    }

    /// `WScriptFn_SpawnMultiParticle`: both init-guard states, zero and several
    /// entries, entries whose type matches, an int entry for a float field
    /// (converted) and a mismatch for a non-float field (copied).
    #[test]
    fn wscript_spawn_multi_particle_matches() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00357A78),
            oc1491_fake_create_slot,
            fakes::prev_cell(oc1491_fake_create_slot),
        );
        fakes::covered(
            ee::s10::f_00306a90__generated,
            wscript_spawn_multi_particle,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x306a90, 2);
                c.pc = 0x0030_6a90;
                c.bus.write32(OC1491_GATE, OC1491_MAGIC);
                c.bus
                    .write32(GAME_GP + 0x27c4, if seed % 2 == 0 { 0 } else { 1 });
                // declared types of the 57 fields: 1 = int, 2 = float, 3 = other
                for i in 0..57u32 {
                    c.bus.write32(0x0044_6890 + 4 * i, 1 + r.next_u32() % 3);
                }
                let n: i32 = match seed % 7 {
                    0 => 0,
                    1 => -1,
                    _ => 1 + (r.next_u32() % 8) as i32,
                };
                let params = DATA + 0x1000;
                for i in 0..8u32 {
                    let idx = r.next_u32() % 57;
                    let decl = c.bus.read32(0x0044_6890 + 4 * idx);
                    let ty = match r.next_u32() % 3 {
                        0 => decl,
                        1 => 1 + (decl % 3),
                        _ => 2,
                    };
                    c.bus.write32(params + 16 * i, idx);
                    c.bus.write32(params + 16 * i + 4, r.next_u32() % 1000);
                    c.bus.write32(params + 16 * i + 0xc, ty);
                }
                for o in 0..4u32 {
                    c.bus.write32(0x004c_9098 + 4 * o, r.next_u32());
                }
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(n as u32);
                c.g[A2][0] = sx(params);
            },
        );
    }

    pub(crate) const WS_PARAMS: u32 = DATA + 0x3000;

    pub(crate) const WS_OUT: u32 = DATA + 0x6000;

    pub(crate) const WS_UI: u32 = DATA + 0x1300;

    /// `Luno WScriptFn_SectionEventCommand(out, n, params)`: copies the
    /// command's argument block (`n` index/value pairs applied over the
    /// defaults), and for a human rider's being runs the section command
    /// (load, activate, DJ/user select, sky swap). Entries: the start (with
    /// and without the first-time init, with and without parameters) and every
    /// resume point of the body, including the jump-table arms.
    #[test]
    fn w_script_fn_section_event_command_matches() {
        oc1564_install();
        const ENTRIES: [u32; 20] = [
            0x0030_2210,
            0x0030_2210,
            0x0030_2210,
            0x0030_2210,
            0x0030_2210,
            0x0030_2270,
            0x0030_228c,
            0x0030_22a0,
            0x0030_22dc,
            0x0030_22e8,
            0x0030_234c,
            0x0030_2394,
            0x0030_23b0,
            0x0030_23c8,
            0x0030_23dc,
            0x0030_23f0,
            0x0030_2420,
            0x0030_2440,
            0x0030_2448,
            0x0030_244c,
        ];
        fakes::covered(
            ee::s10::f_00302210__generated,
            w_script_fn_section_event_command,
            160,
            &|c, seed| {
                let mut r = oc1564_machine(c, seed, 0x0030_2210, 8, true);
                let entry = ENTRIES[(seed % 20) as usize];
                let start = entry == 0x0030_2210;
                let n: u32 = match seed / 20 % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    _ => (-1i32) as u32,
                };
                let frame = if start { TEST_SP - 0x90 } else { TEST_SP };
                let vp = frame + 0x10;
                // argument block defaults the (faked) copy leaves in place
                c.bus.write32(vp, r.next_u32());
                c.bus.write32(vp + 4, (seed / 3) % 7);
                c.bus.write32(vp + 8, (seed / 5) % 12);
                // parameters: idx, value, -, kind
                for i in 0..3 {
                    let e = WS_PARAMS + 16 * i;
                    c.bus.write32(e, r.next_u32() % 4);
                    c.bus.write32(e + 4, r.next_u32() % 20);
                    c.bus.write32(e + 12, 1 + r.next_u32() % 2);
                }
                for k in 0..4 {
                    c.bus.write32(0x0044_65c8 + 4 * k, 1 + r.next_u32() % 2);
                }
                c.bus.write32(GAME_GP + 0x2640, (seed / 2) % 2);
                // the rider being: index word (or -1: none) -> world -> AI table
                let idx = if seed % 7 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() % 4
                };
                let (world, ai, table, lst, ride, qobj) = (
                    DATA + 0x1000,
                    DATA + 0x1100,
                    DATA + 0x1200,
                    DATA + 0x7000,
                    DATA + 0x1800,
                    DATA + 0x2000,
                );
                c.bus.write32(GAME_GP + 0xce8, lst);
                c.bus.write32(lst, idx);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, ai);
                c.bus.write32(ai + 0xc, table);
                c.bus.write32(ai + 0x78, WS_UI);
                for i in 0..4 {
                    let e = DATA + 0x1400 + 0x40 * i;
                    c.bus.write32(table + 4 * i, e);
                    c.bus
                        .write32(e + 0x28, if seed % 11 == 3 { 0 } else { ride });
                }
                c.bus.write32(ride + 0x6c0, qobj);
                c.bus.write16(qobj + 0x48, 0);
                c.bus.write32(qobj + 0x4c, fakes::FAKE);
                // the "is a computer rider" vtable call returns 1 (not human)
                // for some seeds (v0 ^ 1 == 0), a pointer otherwise
                let call = if start || entry == 0x0030_2270 { 1 } else { 0 };
                if seed % 4 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * call, 1);
                }
                c.g[A0][0] = WS_OUT as u64;
                c.g[A1][0] = sx(n);
                c.g[A2][0] = WS_PARAMS as u64;
                c.g[S5][0] = WS_OUT as u64;
                if !start {
                    c.bus.write64(TEST_SP + 0x20, TEST_RA as u64);
                    c.g[S0][0] = vp as u64;
                    c.g[S1][0] = sx(n);
                    c.g[S2][0] = WS_PARAMS as u64;
                    c.g[S3][0] = r.next_u32() as u64;
                    c.g[S4][0] = r.next_u32() as u64;
                }
                match entry {
                    0x0030_2270 => c.g[A2][0] = 0x0050_0000,
                    0x0030_228c => {
                        c.g[S1][0] = if n as i32 > 0 { n as u64 } else { 2 };
                    }
                    0x0030_22a0 => {
                        c.g[A1][0] = if n as i32 > 0 { n as u64 } else { 2 };
                        c.g[T0][0] = 0x0044_65c8;
                        c.g[A3][0] = 2;
                    }
                    0x0030_22dc => {
                        c.g[A1][0] = if n as i32 > 0 { n as u64 } else { 1 };
                        c.g[T0][0] = 0x0044_65c8;
                        c.g[A3][0] = 2;
                    }
                    0x0030_2394 | 0x0030_23b0 | 0x0030_23c8 | 0x0030_23dc | 0x0030_23f0
                    | 0x0030_2420 | 0x0030_2440 => {
                        c.g[S0][0] = WS_UI as u64;
                        c.g[S2][0] = (seed % 6) as u64;
                        c.g[V0][0] = 0x0049_0000;
                    }
                    0x0030_244c => c.g[V1][0] = 0x004d_0000,
                    _ => {}
                }
                c.pc = entry;
            },
        );
    }

    // ------------------------------------------------------------ gated fakes

    /// Control word of this module's gated fakes (in the machine's RAM).
    pub(crate) const OC1567_CTRL: u32 = DATA + 0x3d_0000;

    pub(crate) const OC1567_MAGIC: u32 = 0x0c15_6700;

    /// One forced result word per fake.
    pub(crate) const OC1567_RET: u32 = DATA + 0x3d_0010;

    pub(crate) const OC1567_RET_ATTACH: u32 = 0;

    pub(crate) const OC1567_RET_ALLOC: u32 = 1;

    pub(crate) const OC1567_RET_CTOR: u32 = 2;

    /// A logging fake when the control word is set, otherwise whatever was
    /// installed before (chained).
    pub(crate) fn oc1567_gated(
        c: &mut Cpu,
        addr: u32,
        ret_idx: u32,
        prev: &AtomicUsize,
        fallback: fn(&mut Cpu),
    ) {
        if c.bus.read32(OC1567_CTRL) == OC1567_MAGIC {
            let ra = c.g[RA][0] as u32;
            fake(c, addr, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(OC1567_RET + 4 * ret_idx));
            }
            return;
        }
        fall_back(c, prev, fallback)
    }

    pub(crate) static OC1567_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1567_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1567_PREV_DEAD_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1567_PREV_FLEX_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1567_fake_attach(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x002f_ae38,
            OC1567_RET_ATTACH,
            &OC1567_PREV_ATTACH,
            ee::s10::f_002fae38__generated,
        );
    }

    pub(crate) fn oc1567_fake_alloc(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0031_7d70,
            OC1567_RET_ALLOC,
            &OC1567_PREV_ALLOC,
            ee::s10::f_00317d70__generated,
        );
    }

    pub(crate) fn oc1567_fake_dead_ctor(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0035_0750,
            OC1567_RET_CTOR,
            &OC1567_PREV_DEAD_CTOR,
            ee::s11::f_00350750__generated,
        );
    }

    pub(crate) fn oc1567_fake_flex_ctor(c: &mut Cpu) {
        oc1567_gated(
            c,
            0x0034_6350,
            OC1567_RET_CTOR,
            &OC1567_PREV_FLEX_CTOR,
            ee::s11::f_00346350__generated,
        );
    }

    pub(crate) fn oc1567_install_factory_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc1567_fake_attach,
            &OC1567_PREV_ATTACH,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc1567_fake_alloc,
            &OC1567_PREV_ALLOC,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00350750),
            oc1567_fake_dead_ctor,
            &OC1567_PREV_DEAD_CTOR,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00346350),
            oc1567_fake_flex_ctor,
            &OC1567_PREV_FLEX_CTOR,
        );
    }

    /// The shape of one factory's static data.
    pub(crate) struct Oc1567Factory {
        entry: u32,
        init: u32,
        desc: u32,
        /// Words of the description (the argument buffer).
        words: u32,
        types: u32,
        /// Index of the object id in the buffer.
        id_idx: u32,
    }

    /// A fixture for the factories: the init flag set or clear, the type
    /// table, up to three argument entries (one setting the object id), the
    /// instance tables the id resolves through (id -1: the script manager's
    /// object; a section entry; a null section entry; a slot of 0), and an
    /// object with and without an active node, with the attach, allocation
    /// and constructor calls faked.
    pub(crate) fn oc1567_factory_fixture(c: &mut Cpu, seed: u32, salt: u64, fx: &Oc1567Factory) {
        const RETBUF: u32 = DATA + 0x100;
        const ENTRIES: u32 = DATA + 0x200;
        const INST: u32 = DATA + 0x8000;
        const INST_HDR: u32 = DATA + 0x8100;
        const TABLE: u32 = DATA + 0x8200;
        const ENTRY: u32 = DATA + 0x8300;
        const SLOTS: u32 = DATA + 0x8400;
        const OBJECT: u32 = DATA + 0x8800;
        const SCRIPT_OBJECT: u32 = DATA + 0x8a00;
        const MANAGER: u32 = DATA + 0x8b00;
        const NODE: u32 = DATA + 0x8c00;
        let mut r = machine(c, seed, salt, 3);
        c.pc = fx.entry;
        let gp = GAME_GP;
        c.bus.write32(OC1567_CTRL, OC1567_MAGIC);
        c.bus
            .write32(OC1567_RET + 4 * OC1567_RET_ATTACH, (seed / 4) % 2);
        c.bus
            .write32(OC1567_RET + 4 * OC1567_RET_ALLOC, DATA + 0x8_0000);
        c.bus.write32(OC1567_RET + 4 * OC1567_RET_CTOR, 0);
        // The `.lit4` constant the flex-bridge init reads (0.1f).
        c.bus.write32(gp.wrapping_sub(0x3390), 0x3dcc_cccd);
        // The nil Luno value copied into the result.
        for i in 0..3u32 {
            c.bus.write32(0x004c_9098 + 4 * i, r.next_u32());
        }
        // Which object the id names.
        let mode = (seed / 3) % 4;
        let slot = (seed / 12) % 4;
        let table_idx = (seed / 48) % 4;
        let id = if mode == 0 {
            0xffff_ffffu32
        } else {
            (slot << 8) | table_idx
        };
        // Init flag and description.
        let inited = seed % 3 != 0;
        c.bus.write32(gp + fx.init, inited as u32);
        for i in 0..fx.words {
            c.bus.write32(fx.desc + 4 * i, r.next_u32());
        }
        c.bus.write32(fx.desc + 4 * fx.id_idx, id);
        // The per-slot types: the id slot is never a float, the others
        // mostly are or match.
        for i in 0..10u32 {
            let ty = if i == fx.id_idx {
                r.next_u32() % 2
            } else if (i + seed) % 3 == 0 {
                2
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(fx.types + 4 * i, ty);
        }
        // The argument entries.
        let n = (seed % 4) as i32 - 1;
        for i in 0..3u32 {
            let e = ENTRIES + 0x10 * i;
            let idx = if i == 0 && seed % 2 == 0 {
                fx.id_idx
            } else {
                (seed + 2 * i) % fx.words
            };
            let ety = c.bus.read32(fx.types + 4 * idx);
            let val = if idx == fx.id_idx {
                id
            } else {
                r.next_u32() % 1000
            };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 8, r.next_u32());
            c.bus.write32(
                e + 0xc,
                if (seed + i) % 3 == 0 {
                    ety
                } else {
                    r.next_u32() % 4
                },
            );
        }
        // The section tables the id resolves through.
        c.bus.write32(gp + 0x16c8, INST);
        c.bus.write32(INST, INST_HDR);
        c.bus.write32(INST_HDR + 8, TABLE);
        for i in 0..4u32 {
            c.bus
                .write32(TABLE + 4 * i, if mode == 3 { 0 } else { ENTRY });
        }
        c.bus.write32(ENTRY + 0x1c, SLOTS);
        for i in 0..4u32 {
            let word = if mode == 2 {
                0
            } else {
                ((OBJECT >> 2) << 8) | (r.next_u32() & 0xff)
            };
            c.bus.write32(SLOTS + 4 * i, word);
        }
        // The objects: with and without an active node.
        let node = if seed % 5 < 2 { 0 } else { NODE };
        c.bus.write32(OBJECT + 0xc, node);
        c.bus
            .write32(SCRIPT_OBJECT + 0xc, if seed % 7 < 3 { 0 } else { NODE });
        c.bus.write32(gp + 0xce8, MANAGER);
        c.bus.write32(
            MANAGER + 0x290,
            if seed % 11 == 0 { 0 } else { SCRIPT_OBJECT },
        );
        c.g[A0][0] = sx(RETBUF);
        c.g[A1][0] = n as i64 as u64;
        c.g[A2][0] = sx(ENTRIES);
    }

    /// `WorldObjFactory_DeadFade`: the first-call init (flag clear), the
    /// argument loop (raw and float stores), every id path, the attach check
    /// (answering 0 and 1) and the node build with its fifth constructor
    /// argument.
    #[test]
    fn world_obj_factory_dead_fade_matches() {
        oc1567_install_factory_fakes();
        let fx = Oc1567Factory {
            entry: 0x002f_c5d8,
            init: OC1567_DEAD_FADE_INIT,
            desc: OC1567_DEAD_FADE_DESC,
            words: 5,
            types: OC1567_DEAD_FADE_TYPES,
            id_idx: 0,
        };
        covered(
            ee::s10::f_002fc5d8__generated,
            world_obj_factory_dead_fade,
            240,
            &|c, seed| oc1567_factory_fixture(c, seed, 0x2f_c5d8, &fx),
        );
    }

    /// `WorldObjFactory_FlexBridge`: as the dead-fade test, for the flex
    /// bridge's 10-word buffer (object id at index 8) and its constructor.
    #[test]
    fn world_obj_factory_flex_bridge_matches() {
        oc1567_install_factory_fakes();
        let fx = Oc1567Factory {
            entry: 0x002f_cdc8,
            init: OC1567_FLEX_BRIDGE_INIT,
            desc: OC1567_FLEX_BRIDGE_DESC,
            words: 10,
            types: OC1567_FLEX_BRIDGE_TYPES,
            id_idx: 8,
        };
        covered(
            ee::s10::f_002fcdc8__generated,
            world_obj_factory_flex_bridge,
            240,
            &|c, seed| oc1567_factory_fixture(c, seed, 0x2f_cdc8, &fx),
        );
    }

    /// This module's gate word (the gated fakes below answer only when it is
    /// set in their own machine), its mailbox word (GATE + 4), and the magic.
    pub(crate) const GATE: u32 = DATA + 0x0070_1570;

    pub(crate) const MAGIC: u32 = 0x1576_0001;

    /// The WScriptMan: `+0` the being, `+0x290` the instance.
    pub(crate) const WS: u32 = DATA + 0x0070_2000;

    pub(crate) const INST: u32 = DATA + 0x0070_2400;

    /// The world (`gp-0x848`): `+0x84` the section data, `+0xc` the per-being
    /// table at `+0x28`, `+0x78` the section manager.
    pub(crate) const WORLD: u32 = DATA + 0x0070_3000;

    pub(crate) const WORLD_P: u32 = DATA + 0x0070_3400;

    pub(crate) const WORLD_Q: u32 = DATA + 0x0070_3800;

    pub(crate) const SECTION: u32 = DATA + 0x0070_3c00;

    /// Records `RECORDS + 0x100 k` and their callback blocks `TARGETS + 0x100 k`.
    pub(crate) const RECORDS: u32 = DATA + 0x0070_4000;

    pub(crate) const TARGETS: u32 = DATA + 0x0070_5000;

    pub(crate) const PARAMS: u32 = DATA + 0x0070_6000;

    pub(crate) const SELF: u32 = DATA + 0x0070_7000;

    /// The keyed-argument defaults at `0x4fbd0` (collect) and `0x4fc10`
    /// (start event), the `Nil` image at `0x4c9098`, the mode table at `0x445e40`.
    pub(crate) const COLLECT_DEFAULTS: u32 = 0x0004_fbd0;

    pub(crate) const EVENT_DEFAULTS: u32 = 0x0004_fc10;

    pub(crate) const NIL_IMAGE: u32 = 0x004c_9098;

    pub(crate) const MODE_TABLE: u32 = 0x0044_5e40;

    pub(crate) fn oc1576_no_prev(_c: &mut Cpu) {
        unreachable!("oc1576 fake called before it was hooked");
    }

    /// Gated fake of `WScriptMan_CollectItem` (0x30b9a0): logs the call.
    pub(crate) fn oc1576_fake_collect_item(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1576_fake_collect_item),
                oc1576_no_prev,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_b9a0, ra);
    }

    /// Gated fake of `luno_cLunoVar_castInt` (0x226610): logs the call.
    pub(crate) fn oc1576_fake_cast_int(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_cast_int), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_6610, ra);
    }

    /// Gated fake of `WScriptMan_DeactivateAll` (0x30b7f8).
    pub(crate) fn oc1576_fake_deactivate(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_deactivate), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0030_b7f8, ra);
    }

    /// Gated fake of `cSectionMan_eventbegin` (0x22d6c8).
    pub(crate) fn oc1576_fake_eventbegin(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1576_fake_eventbegin), oc1576_no_prev);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0022_d6c8, ra);
    }

    /// The WScriptMan with the given being (`gp+0xce8`).
    pub(crate) fn wscript_man(c: &mut Cpu, being: u32) {
        c.bus.write32(GAME_GP + 0xce8, WS);
        c.bus.write32(WS, being);
        c.bus.write32(WS + 0x290, INST);
        c.bus.write32(INST + 0x78, 0x0000_0042);
    }

    /// The world (`gp-0x848`) with a record for each being; the record is
    /// zero for the beings whose bit is set in `zero_mask`.
    pub(crate) fn world(c: &mut Cpu, r: &mut Rng, zero_mask: u32) {
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, WORLD_P);
        c.bus.write32(WORLD_P + 0xc, WORLD_Q);
        c.bus.write32(WORLD_P + 0x78, SECTION);
        c.bus.write32(0x6c0, TARGETS);
        for k in 0..4u32 {
            let rec = RECORDS + 0x100 * k;
            let target = TARGETS + 0x100 * k;
            let zero = (zero_mask >> k) & 1 == 1;
            c.bus
                .write32(WORLD_Q + 0x28 + 4 * k, if zero { 0 } else { rec });
            c.bus.write32(rec + 0x6c0, target);
            c.bus.write16(target + 0x48, (r.next_u32() % 0x40) as u16);
            c.bus.write32(target + 0x4c, fakes::FAKE);
        }
    }

    /// Three keyed-argument entries at `PARAMS`: an index below `idx_max`, a
    /// value, and a kind (1 to 3, so the float conversion is hit too).
    pub(crate) fn keyed_entries(c: &mut Cpu, r: &mut Rng, idx_max: u32) {
        for i in 0..3u32 {
            let p = PARAMS + 16 * i;
            c.bus.write32(p, r.next_u32() % idx_max);
            c.bus.write32(p + 4, r.next_u32());
            c.bus.write32(p + 0xc, 1 + r.next_u32() % 3);
        }
    }

    /// `WScriptFn_CollectItem`: no record, a record with and without the
    /// fake's forced result, the init guard on and off, item ids set and
    /// unset, the float conversion and the raw copy, and zero to three keys.
    #[test]
    fn wscript_fn_collect_item_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B9A0),
            oc1576_fake_collect_item,
            fakes::prev_cell(oc1576_fake_collect_item),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00226610),
            oc1576_fake_cast_int,
            fakes::prev_cell(oc1576_fake_cast_int),
        );
        check(
            ee::s10::f_00301120__generated,
            wscript_fn_collect_item,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_1120, 4);
                c.bus.write32(GATE, MAGIC);
                let being = [u32::MAX, 0, 1, 2, 3][(seed % 5) as usize];
                wscript_man(c, being);
                world(c, &mut r, (seed >> 2) & 0xf);
                c.bus.write32(GAME_GP + 0x25c4, seed % 2);
                for k in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xb78 + 4 * k, 1 + r.next_u32() % 3);
                }
                let first = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(COLLECT_DEFAULTS, first);
                c.bus.write32(COLLECT_DEFAULTS + 4, r.next_u32());
                c.bus.write32(COLLECT_DEFAULTS + 8, r.next_u32());
                keyed_entries(c, &mut r, 3);
                if seed % 4 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS, 1);
                }
                let n = [0u32, 1, 2, 3, u32::MAX][(seed % 5) as usize];
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `WScriptFn_StartEvent`: the being with and without a record, the second
    /// value set and unset, the modes of the table (free ride or not), and
    /// the init guard on and off.
    #[test]
    fn wscript_fn_start_event_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B7F8),
            oc1576_fake_deactivate,
            fakes::prev_cell(oc1576_fake_deactivate),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D6C8),
            oc1576_fake_eventbegin,
            fakes::prev_cell(oc1576_fake_eventbegin),
        );
        check(
            ee::s10::f_00302048__generated,
            wscript_fn_start_event,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_2048, 4);
                c.bus.write32(GATE, MAGIC);
                let being = [u32::MAX, 0, 1, 2, 3][(seed % 5) as usize];
                wscript_man(c, being);
                world(c, &mut r, (seed >> 2) & 0xf);
                c.bus.write32(GAME_GP + 0x263c, seed % 2);
                for k in 0..4u32 {
                    c.bus.write32(GAME_GP + 0xbd0 + 4 * k, 1 + r.next_u32() % 3);
                }
                for k in 0..8u32 {
                    c.bus.write32(MODE_TABLE + 4 * k, r.next_u32() % 3);
                }
                let id = if seed % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(EVENT_DEFAULTS, id);
                c.bus
                    .write32(EVENT_DEFAULTS + 4, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(EVENT_DEFAULTS + 8, r.next_u32());
                keyed_entries(c, &mut r, 2);
                if seed % 4 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS, 1);
                }
                let n = [0u32, 1, 2, 3, u32::MAX][(seed % 5) as usize];
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
            },
        );
    }

    /// `RiderInTunnelFunc`: a being with a record (zero or not), and none.
    #[test]
    fn rider_in_tunnel_func_matches() {
        check(
            ee::s10::f_00303f80__generated,
            rider_in_tunnel_func,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x0030_3f80, 0);
                let being = [u32::MAX, 0, 1, 2, 3][(seed % 5) as usize];
                wscript_man(c, being);
                world(c, &mut r, (seed >> 2) & 0xf);
                fill(c, &mut r, NIL_IMAGE, 3);
                c.g[A0][0] = sx(SELF);
            },
        );
    }

    /// Gate word of this module's callee fakes (a unique cell of the machine's
    /// RAM): the fakes answer only when a fixture sets it.
    pub(crate) const OC1601_GATE: u32 = DATA + 0x21_6010;

    pub(crate) const OC1601_MAGIC: u32 = 0x1601_f001;

    /// A gated fake for the real callee at `$addr`: with the gate off it runs
    /// whatever the slot held before, with the gate on it logs the call and
    /// returns (or unwinds) like `fakes::fake_call`.
    macro_rules! oc1601_fake {
        ($name:ident, $gen:path, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1601_GATE) != OC1601_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc1601_fake!(
        oc1601_fake_mem_copy,
        ee::s14::f_003e6574__generated,
        0x003e_6574
    );

    oc1601_fake!(
        oc1601_fake_add_roller,
        ee::s11::f_00355db8__generated,
        0x0035_5db8
    );

    oc1601_fake!(
        oc1601_fake_invoke_exec,
        ee::s10::f_00317328__generated,
        0x0031_7328
    );

    oc1601_fake!(
        oc1601_fake_module_loop,
        ee::s10::f_00316f00__generated,
        0x0031_6f00
    );

    oc1601_fake!(
        oc1601_fake_service_timers,
        ee::s14::f_003e5928__generated,
        0x003e_5928
    );

    oc1601_fake!(
        oc1601_fake_wait_sema,
        ee::s15::f_00423de0__generated,
        0x0042_3de0
    );

    oc1601_fake!(
        oc1601_fake_wakeup,
        ee::s15::f_00423cd0__generated,
        0x0042_3cd0
    );

    oc1601_fake!(
        oc1601_fake_rotate,
        ee::s15::f_00423c50__generated,
        0x0042_3c50
    );

    oc1601_fake!(
        oc1601_fake_suspend,
        ee::s15::f_00423d10__generated,
        0x0042_3d10
    );

    oc1601_fake!(
        oc1601_fake_printf,
        ee::s15::f_00425c28__generated,
        0x0042_5c28
    );

    /// Hook every fake of this module into its callee's slot (once per process).
    pub(crate) fn oc1601_install() {
        macro_rules! hook {
            ($slot:path, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        hook!(ee::s14::OVERRIDE_003E6574, oc1601_fake_mem_copy);
        hook!(ee::s11::OVERRIDE_00355DB8, oc1601_fake_add_roller);
        hook!(ee::s10::OVERRIDE_00317328, oc1601_fake_invoke_exec);
        hook!(ee::s10::OVERRIDE_00316F00, oc1601_fake_module_loop);
        hook!(ee::s14::OVERRIDE_003E5928, oc1601_fake_service_timers);
        hook!(ee::s15::OVERRIDE_00423DE0, oc1601_fake_wait_sema);
        hook!(ee::s15::OVERRIDE_00423CD0, oc1601_fake_wakeup);
        hook!(ee::s15::OVERRIDE_00423C50, oc1601_fake_rotate);
        hook!(ee::s15::OVERRIDE_00423D10, oc1601_fake_suspend);
        hook!(ee::s15::OVERRIDE_00425C28, oc1601_fake_printf);
    }

    /// `fakes::machine` with this module's fake gate switched on.
    pub(crate) fn oc1601_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1601_GATE, OC1601_MAGIC);
        r
    }

    /// `WScriptFn_AddRollerModifier`: the default block init (flag clear/set),
    /// the entry loop (equal type, different type with a Float keyword and
    /// not), the three instance lookups (script manager, world cache hit, miss
    /// by section and by tag), a missing active node, a node that is not a move
    /// node (real leaf returning 0) and one that is (`addRoller` called), with
    /// an unwind at each of the three calls.
    #[test]
    fn wscript_fn_add_roller_modifier_matches() {
        oc1601_install();
        fakes::check_reaching(
            ee::s10::f_002fd250__generated,
            wscript_fn_add_roller_modifier,
            300,
            &[0x002f_d2e4, 0x002f_d3c4, 0x002f_d3e0],
            &|c, seed| {
                let mut r = oc1601_machine(c, seed, 0x002f_d250, 3);
                let n = seed % 5;
                let params = DATA + 0x1000;
                let out = DATA + 0x1800;
                let (world, wcache, stab, sec, ctab) = (
                    DATA + 0x2000,
                    DATA + 0x2100,
                    DATA + 0x2200,
                    DATA + 0x2300,
                    DATA + 0x2400,
                );
                let (man, obj, node, vt) =
                    (DATA + 0x2800, DATA + 0x3000, DATA + 0x3400, DATA + 0x3800);
                // keyword types of the five slots
                let mut kw = [0u32; 8];
                for (i, k) in kw.iter_mut().enumerate() {
                    *k = r.next_u32() % 4;
                    c.bus.write32(0x0044_60a0 + 4 * i as u32, *k);
                }
                // the target id: -1 (script manager), a cache hit, a miss
                let kind = (seed / 5) % 3;
                let sidx = r.next_u32() & 3;
                let kidx = r.next_u32() & 3;
                let id = match kind {
                    0 => 0xffff_ffffu32,
                    _ => (kidx << 8) | sidx,
                };
                // entry 0 sets the id (a matching type: plain store; sometimes
                // a different type: float conversion when the keyword says so)
                let t0 = if seed % 7 == 0 { kw[0] ^ 2 } else { kw[0] };
                c.bus.write32(params, 0);
                c.bus.write32(params + 4, id);
                c.bus.write32(params + 0xc, t0);
                for i in 1..5u32 {
                    let e = params + 0x10 * i;
                    c.bus.write32(e, 1 + r.next_u32() % 4);
                    c.bus.write32(e + 4, r.next_u32() % 4000);
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, r.next_u32() % 4);
                }
                // world cache
                c.bus.write32(GAME_GP + 0x16c8, world);
                c.bus.write32(world, wcache);
                c.bus.write32(wcache + 8, stab);
                let no_section = kind == 2 && seed % 2 == 0;
                c.bus
                    .write32(stab + 4 * sidx, if no_section { 0 } else { sec });
                c.bus.write32(sec + 0x1c, ctab);
                let tag = if kind == 1 {
                    ((obj >> 2) << 8) | (r.next_u32() & 0xff)
                } else {
                    r.next_u32() & 0xff
                };
                c.bus.write32(ctab + 4 * kidx, tag);
                // script manager and the instance
                c.bus.write32(GAME_GP + 0xce8, man);
                c.bus.write32(man + 0x290, obj);
                c.bus.write32(man + 0x294, r.next_u32());
                c.bus.write32(man + 0x298, r.next_u32());
                c.bus
                    .write32(obj + 0xc, if seed % 4 == 0 { 0 } else { node });
                c.bus.write32(node + 0xc, vt);
                c.bus
                    .write16(vt + 0x80, (r.next_u32() as i16 as i32 & !3) as u16);
                let move_node = (seed / 3) % 2 == 0;
                // a real leaf returning 0 is "not a move node"
                c.bus
                    .write32(vt + 0x84, if move_node { fakes::FAKE } else { 0x0014_06c0 });
                // init flag and defaults
                c.bus.write32(GAME_GP + 0x2510, (seed / 2) % 2);
                let fl = r.f32(1.0);
                c.bus.write32(GAME_GP.wrapping_sub(0x3388), fl);
                for k in 0..6u32 {
                    c.bus.write32(0x004f_b628 + 4 * k, r.next_u32());
                    c.bus.write32(0x004c_9098 + 4 * k, r.next_u32());
                    c.bus.write32(out + 4 * k, r.next_u32());
                }
                c.g[A0][0] = out as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = params as u64;
            },
        );
    }
}
