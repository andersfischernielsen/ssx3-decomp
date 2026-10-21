//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! Three shapes dominate this batch:
//!
//! - seventeen "destructor" thunks that re-publish the class vtable into the
//!   object and then `operator delete` it when `flags & 1`
//!   ([`reset_vtable_and_delete`]);
//! - twenty `..._LoadState`/`Accept` forwarders that tail-call the third
//!   virtual slot of a sibling interface object, with a small constant or a
//!   forwarded argument ([`virtual_call_slot`]) — the `Rider*Sub_LoadState`
//!   and `Rider*State_LoadState` families differ only in that constant;
//! - the rest: small real wrappers around irradiance, world-cache, MPEG2 and
//!   AI entry points, and four straight-line table lookups/copies.
//!
//! Register-for-register copies of the generated code are used throughout,
//! including the scratch `v0`/`v1`/`a2`/`a3` values the original leaves set.

use crate::abi::r32;
use crate::call::{call_ptr, guest, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0036_18c8, position_modifier_v01, "PositionModifier_v01"),
    (0x0036_1988, parent_modifier_v01, "ParentModifier_v01"),
    (0x0036_19e0, magnet_modifier_v01, "MagnetModifier_v01"),
    (0x0036_1c78, roller_modifier_v01, "RollerModifier_v01"),
    (0x0036_23a8, intc_table_vu1_handler, "IntcTable_Vu1Handler"),
    (0x0037_0758, emitter_dtor, "Emitter_dtor"),
    (0x0038_5be0, c_ps2_grid_mesh_v07, "cPS2GridMesh_v07"),
    (0x0038_ac20, irradiance_get_entry_by_name, "Irradiance_GetEntryByName"),
    (0x0039_4b38, grid_mesh_base_base_dtor, "GridMeshBase_BaseDtor"),
    (0x0039_61a8, vfx_render_toggles_dtor, "VfxRenderToggles_dtor"),
    (0x0039_64c8, vfx_plants_dtor, "VfxPlants_dtor"),
    (0x0039_6838, vfx_tube_dtor, "VfxTube_dtor"),
    (0x0039_68e8, path_arrow_v01, "PathArrow_v01"),
    (0x0039_6928, detail_system_v01, "DetailSystem_v01"),
    (0x0039_6a00, vfx_visual_effects_test_dtor, "VfxVisualEffectsTest_dtor"),
    (0x0039_6d30, vfx_screen_tint_dtor, "VfxScreenTint_dtor"),
    (0x0039_7788, ui_list_node_accept, "UIListNode_Accept"),
    (0x0039_77b8, ui_list_node_v03, "UIListNode_v03"),
    (0x0039_bd38, ui_pair_accept, "UIPair_Accept"),
    (0x0039_e688, ui_state_v13, "UIState_v13"),
    (0x003a_47a8, ui_visitor_dtor, "UIVisitor_dtor"),
    (0x003a_4cd0, ui_input_visitor_dtor, "UIInputVisitor_dtor"),
    (0x003a_4d40, ui_focus_visitor_dtor, "UIFocusVisitor_dtor"),
    (0x003a_51a0, c_ui_group_accept, "cUIGroup_Accept"),
    (0x003a_7f90, c_world_cache_reset, "cWorldCache_Reset"),
    (0x003a_dc18, c_ps2_chunk_resolve_base_dtor, "cPS2ChunkResolve_BaseDtor"),
    (0x003b_1228, mpeg2_initialize_buffer, "MPEG2_Initialize_Buffer"),
    (0x003b_13a8, mpeg2_get_bits32, "MPEG2_Get_Bits32"),
    (0x0011_74e8, rider_metrix_load_state, "RiderMetrix_LoadState"),
    (0x0012_0e50, rider_refresh_overlaps, "Rider_RefreshOverlaps"),
    (0x0012_8a10, ai_reset_riders_and_order, "AI_ResetRidersAndOrder"),
    (0x0012_c590, rider_gate_sub_load_state, "RiderGateSub_LoadState"),
    (0x0012_c9f8, rider_finish_line_sub_load_state, "RiderFinishLineSub_LoadState"),
    (0x0012_e740, rider_wipeout_sub_load_state, "RiderWipeoutSub_LoadState"),
    (0x0012_f5e8, rider_reset_sub_load_state, "RiderResetSub_LoadState"),
    (0x0012_fc28, rider_natural_air_sub_load_state, "RiderNaturalAirSub_LoadState"),
    (0x0013_15d0, rider_board_press_sub_load_state, "RiderBoardPressSub_LoadState"),
    (0x0013_2878, rider_railslide_sub_load_state, "RiderRailslideSub_LoadState"),
    (0x0013_30f0, rider_handplant_sub_load_state, "RiderHandplantSub_LoadState"),
    (0x0013_6230, rider_spin_sub_load_state, "RiderSpinSub_LoadState"),
    (0x0013_6920, rider_uber_grind_sub_load_state, "RiderUberGrindSub_LoadState"),
    (0x0013_8b10, rider_wipeout_state_load_state, "RiderWipeoutState_LoadState"),
    (0x0013_99a8, rider_handplant_state_load_state, "RiderHandplantState_LoadState"),
    (0x0013_ace8, rider_air_state_load_state, "RiderAirState_LoadState"),
    (0x0013_c618, rider_railslide_state_load_state, "RiderRailslideState_LoadState"),
    (0x0013_f880, rider_ground_state_load_state, "RiderGroundState_LoadState"),
    (0x0014_4ce0, be_new_race_int_find_track_by_id, "BENewRaceInt_FindTrackById"),
    (0x0014_5630, be_new_race_int_get_special_event_score, "BENewRaceInt_GetSpecialEventScore"),
    (0x0014_5668, be_new_race_int_get_special_event_value, "BENewRaceInt_GetSpecialEventValue"),
    (0x0014_e9c0, be_copy_profile, "BE_CopyProfile"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls: the generated code calls the shard function itself).

mod callee {
    use super::{ee, Callee};

    /// `IntcHandler_SaveRegs` (0x362120).
    pub const INTC_SAVE_REGS: Callee = Callee { addr: 0x0036_2120, f: ee::s12::f_00362120 };
    /// `IntcHandler_RestoreRegs` (0x362230).
    pub const INTC_RESTORE_REGS: Callee = Callee { addr: 0x0036_2230, f: ee::s12::f_00362230 };
    /// `Irradiance_FindIndex` (0x38ac50).
    pub const IRRADIANCE_FIND_INDEX: Callee = Callee { addr: 0x0038_ac50, f: ee::s12::f_0038ac50 };
    /// `Irradiance_GetEntry` (0x38abf8).
    pub const IRRADIANCE_GET_ENTRY: Callee = Callee { addr: 0x0038_abf8, f: ee::s12::f_0038abf8 };
    /// `cWorldCache_Free` (0x3a7e98).
    pub const WORLD_CACHE_FREE: Callee = Callee { addr: 0x003a_7e98, f: ee::s13::f_003a7e98 };
    /// `MPEG2_Flush_Buffer` (0x3b1290).
    pub const MPEG2_FLUSH_BUFFER: Callee = Callee { addr: 0x003b_1290, f: ee::s13::f_003b1290 };
    /// `MPEG2_Show_Bits` (0x3b1258).
    pub const MPEG2_SHOW_BITS: Callee = Callee { addr: 0x003b_1258, f: ee::s13::f_003b1258 };
    /// `MPEG2_Flush_Buffer32` (0x3b1340).
    pub const MPEG2_FLUSH_BUFFER32: Callee = Callee { addr: 0x003b_1340, f: ee::s13::f_003b1340 };
    /// `AI_ResetRiders` (0x1297c8).
    pub const AI_RESET_RIDERS: Callee = Callee { addr: 0x0012_97c8, f: ee::s00::f_001297c8 };
    /// `AI_SetRiderOrderMode` (0x128a48).
    pub const AI_SET_RIDER_ORDER_MODE: Callee =
        Callee { addr: 0x0012_8a48, f: ee::s00::f_00128a48 };
    /// `Rider_FindNearbyLights` (0x122088).
    pub const RIDER_FIND_NEARBY_LIGHTS: Callee =
        Callee { addr: 0x0012_2088, f: ee::s00::f_00122088 };
    /// `BE_CopyProfileCharacters` (0x156858).
    pub const BE_COPY_PROFILE_CHARACTERS: Callee =
        Callee { addr: 0x0015_6858, f: ee::s01::f_00156858 };
}

/// `operator delete` (0x317e50): the destructors call it through the function
/// table.
const DELETE: u32 = 0x0031_7e50;
/// `OverlapList_GatherAll` (0x332db8).
const OVERLAP_LIST_GATHER_ALL: u32 = 0x0033_2db8;

// ---------------------------------------------------------------------------
// The "reset the vtable, then delete when asked" destructor shape.

/// `*(void**)((char*)self + DISP) = VTABLE; if (flags & 1) operator delete(self);`
/// — the body of the seventeen object/table destructors below. The `0x490000`
/// page is loaded into `v0` first (the literal the compiler shared across the
/// object files), then the final vtable address.
fn reset_vtable_and_delete(c: &mut Cpu, ret: u32, vtable: u32, disp: u32) -> Flow {
    c.cycles += 7;
    c.g[V0][0] = 0x0049_0000;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 0x1;
    let keep = c.g[A1][0] == 0;
    Addr(r32(c, A0).wrapping_add(disp)).sw(c, c.g[V0][0]);
    if !keep {
        c.cycles += 2;
        call_ptr(c, DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `PositionModifier` destroying destructor (0x3618c8): re-publish
/// `D_004913F8` (the shared modifier vtable) and free when asked.
pub fn position_modifier_v01(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0036_18ec, 0x0049_13f8, 0))
}

/// `ParentModifier` destroying destructor (0x361988).
pub fn parent_modifier_v01(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0036_19ac, 0x0049_13f8, 0))
}

/// `MagnetModifier` destroying destructor (0x3619e0).
pub fn magnet_modifier_v01(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0036_1a04, 0x0049_13f8, 0))
}

/// `RollerModifier` destroying destructor (0x361c78).
pub fn roller_modifier_v01(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0036_1c9c, 0x0049_13f8, 0))
}

/// `Emitter` destroying destructor (0x370758): the particle emitter's base
/// vtable lives at `self + 0x18c`.
pub fn emitter_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0037_077c, 0x0049_3160, 0x18c))
}

/// `GridMeshBase` base-object destroying destructor (0x394b38): the base
/// vtable is the second word of the object.
pub fn grid_mesh_base_base_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_4b5c, 0x0049_3938, 0x4))
}

/// `VfxRenderToggles` destroying destructor (0x3961a8).
pub fn vfx_render_toggles_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_61cc, 0x0048_8680, 0))
}

/// `VfxPlants` destroying destructor (0x3964c8).
pub fn vfx_plants_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_64ec, 0x0048_8680, 0))
}

/// `VfxTube` destroying destructor (0x396838).
pub fn vfx_tube_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_685c, 0x0048_8680, 0))
}

/// `PathArrow` destroying destructor (0x3968e8).
pub fn path_arrow_v01(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_690c, 0x0048_8680, 0))
}

/// `DetailSystem` destroying destructor (0x396928).
pub fn detail_system_v01(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_694c, 0x0048_8680, 0))
}

/// `VfxVisualEffectsTest` destroying destructor (0x396a00).
pub fn vfx_visual_effects_test_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_6a24, 0x0048_8680, 0))
}

/// `VfxScreenTint` destroying destructor (0x396d30).
pub fn vfx_screen_tint_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x0039_6d54, 0x0048_8680, 0))
}

/// `UIVisitor` destroying destructor (0x3a47a8): the visitor vtable
/// `D_00494CE8`.
pub fn ui_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x003a_47cc, 0x0049_4ce8, 0))
}

/// `UIInputVisitor` destroying destructor (0x3a4cd0).
pub fn ui_input_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x003a_4cf4, 0x0049_4ce8, 0))
}

/// `UIFocusVisitor` destroying destructor (0x3a4d40).
pub fn ui_focus_visitor_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x003a_4d64, 0x0049_4ce8, 0))
}

/// `cPS2ChunkResolve` base-object destroying destructor (0x3adc18): the base
/// vtable is the object's third word.
pub fn c_ps2_chunk_resolve_base_dtor(c: &mut Cpu) {
    guest(c, |c| reset_vtable_and_delete(c, 0x003a_dc3c, 0x0049_5150, 0x8))
}

// ---------------------------------------------------------------------------
// The "tail-call virtual slot 2 of an interface object" shape.

/// `vt = *(VTable**)obj; vt[2].fn((char*)obj + vt[2].delta, self, tag)` — the
/// shared body of the twenty interface forwarders below. `obj` is the second
/// argument (`a1`), which the compiler keeps in `a2` (or `a3` when it also has
/// to materialize the extra constant argument in `a2`); `self` (the first
/// argument) becomes the method's second argument.
fn virtual_call_slot(c: &mut Cpu, ret: u32, tag: Option<u32>) -> Flow {
    c.cycles += if tag.is_some() { 13 } else { 12 };
    let frame = Frame::open(c, 0x10);
    let obj;
    if let Some(k) = tag {
        c.g[A3][0] = c.g[A1][0];
        obj = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx(k);
    } else {
        c.g[A2][0] = c.g[A1][0];
        obj = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
    }
    c.g[V1][0] = Addr(obj as u32).lw(c);
    let slot = VSlot::read(c, c.g[V1][0], 0x10);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(obj);
    call_ptr(c, slot.func as u32, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `UIListNode::Accept(self, node)` (0x397788): `node->v02(self)`.
pub fn ui_list_node_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_77ac, None))
}

/// `UIListNode` destroying/v-table variant (0x3977b8).
pub fn ui_list_node_v03(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_77dc, None))
}

/// `UIPair::Accept(self, node)` (0x39bd38): `node->v02(self)`.
pub fn ui_pair_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0039_bd5c, None))
}

/// `cUIGroup::Accept(self, node)` (0x3a51a0): `node->v02(self)`.
pub fn c_ui_group_accept(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x003a_51c4, None))
}

/// `RiderMetrix_LoadState(self, iface)` (0x1174e8): `iface->v02(self, 0x1ac)`.
pub fn rider_metrix_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0011_7510, Some(0x1ac)))
}

/// `RiderGateSub_LoadState(self, iface)` (0x12c590): `iface->v02(self, 0x14)`.
pub fn rider_gate_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_c5b8, Some(0x14)))
}

/// `RiderFinishLineSub_LoadState(self, iface)` (0x12c9f8): `0x4`.
pub fn rider_finish_line_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_ca20, Some(0x4)))
}

/// `RiderWipeoutSub_LoadState(self, iface)` (0x12e740): `0x80`.
pub fn rider_wipeout_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_e768, Some(0x80)))
}

/// `RiderResetSub_LoadState(self, iface)` (0x12f5e8): `0x8`.
pub fn rider_reset_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_f610, Some(0x8)))
}

/// `RiderNaturalAirSub_LoadState(self, iface)` (0x12fc28): `0x14`.
pub fn rider_natural_air_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0012_fc50, Some(0x14)))
}

/// `RiderBoardPressSub_LoadState(self, iface)` (0x1315d0): `0x14`.
pub fn rider_board_press_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_15f8, Some(0x14)))
}

/// `RiderRailslideSub_LoadState(self, iface)` (0x132878): `0x8`.
pub fn rider_railslide_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_28a0, Some(0x8)))
}

/// `RiderHandplantSub_LoadState(self, iface)` (0x1330f0): `0x14`.
pub fn rider_handplant_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_3118, Some(0x14)))
}

/// `RiderSpinSub_LoadState(self, iface)` (0x136230): `0x58`.
pub fn rider_spin_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_6258, Some(0x58)))
}

/// `RiderUberGrindSub_LoadState(self, iface)` (0x136920): `0x8`.
pub fn rider_uber_grind_sub_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_6948, Some(0x8)))
}

/// `RiderWipeoutState_LoadState(self, iface)` (0x138b10): `0x40`.
pub fn rider_wipeout_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_8b38, Some(0x40)))
}

/// `RiderHandplantState_LoadState(self, iface)` (0x1399a8): `0xa0`.
pub fn rider_handplant_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_99d0, Some(0xa0)))
}

/// `RiderAirState_LoadState(self, iface)` (0x13ace8): `0x4`.
pub fn rider_air_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_ad10, Some(0x4)))
}

/// `RiderRailslideState_LoadState(self, iface)` (0x13c618): `0x50`.
pub fn rider_railslide_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_c640, Some(0x50)))
}

/// `RiderGroundState_LoadState(self, iface)` (0x13f880): `0x18`.
pub fn rider_ground_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_f8a8, Some(0x18)))
}

// ---------------------------------------------------------------------------
// IntcTable_Vu1Handler.

/// `int IntcTable_Vu1Handler(void)` (0x3623a8): the VU1 interrupt handler —
/// save the full register file to a 0x100-byte stack buffer, restore it, and
/// re-enable interrupts. Always returns 1 (the "handled" code).
pub fn intc_table_vu1_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x110);
        frame.save_ra(c, 0x100);
        c.g[A0][0] = c.g[SP][0];
        callee::INTC_SAVE_REGS.call(c, 0x0036_23b8)?;
        c.g[A0][0] = c.g[SP][0];
        callee::INTC_RESTORE_REGS.call(c, 0x0036_23c0)?;
        c.ei();
        frame.restore_ra(c, 0x100);
        c.g[V0][0] = 0x1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2GridMesh_Draw / Irradiance_GetEntryByName / cWorldCache_Reset.

/// `void cPS2GridMesh_v07(GridMesh* self, int a, void* b, int c)` (0x385be0):
/// `self->v08(a, **(short**)((char*)b + 4), c)` — the draw call, with the
/// mesh's second sub-object read out of `self + 4`.
pub fn c_ps2_grid_mesh_v07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(4)).lw(c);
        c.g[T0][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
        c.g[A2][0] = Addr(r32(c, V0)).lh(c);
        c.g[V0][0] = Addr(r32(c, T0).wrapping_add(0x40)).lh(c);
        c.g[V1][0] = Addr(r32(c, T0).wrapping_add(0x44)).lw(c);
        let method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        call_ptr(c, method, 0x0038_5c04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `IrradianceEntry* Irradiance_GetEntryByName(IrradianceSet* s, char* name)`
/// (0x38ac20): `GetEntry(s, FindIndex(s, name))`.
pub fn irradiance_get_entry_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::IRRADIANCE_FIND_INDEX.call(c, 0x0038_ac34)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        callee::IRRADIANCE_GET_ENTRY.call(c, 0x0038_ac40)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cWorldCache_Reset(cWorldCache* w)` (0x3a7f90): drop every loaded
/// section (`cWorldCache_Free`) and clear the three cache words.
pub fn c_world_cache_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::WORLD_CACHE_FREE.call(c, 0x003a_7fa4)?;
        let cache = c.g[S0][0];
        Addr(cache as u32).off(0x3e8).sw(c, 0);
        Addr(cache as u32).sw(c, 0);
        Addr(cache as u32).off(0x3ec).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MPEG2 bitstream init / read.

/// `void MPEG2_Initialize_Buffer(void* data)` (0x3b1228): install `data` as the
/// bitstream buffer (`D_0050A088`), clear the bit accumulator and refill.
pub fn mpeg2_initialize_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_sub(0x5f78)).lw(c);
        frame.save_ra(c, 0);
        Addr(r32(c, V1)).sw(c, c.g[A0][0]);
        Addr(r32(c, V1).wrapping_add(0x4)).sw(c, 0);
        c.g[A0][0] = 0;
        Addr(r32(c, V1).wrapping_add(0x8)).sw(c, 0);
        callee::MPEG2_FLUSH_BUFFER.call(c, 0x003b_124c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `unsigned int MPEG2_Get_Bits32(void)` (0x3b13a8): peek 32 bits, consume
/// them, and return the peeked value.
pub fn mpeg2_get_bits32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x20;
        callee::MPEG2_SHOW_BITS.call(c, 0x003b_13bc)?;
        c.g[S0][0] = c.g[V0][0];
        callee::MPEG2_FLUSH_BUFFER32.call(c, 0x003b_13c4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_RefreshOverlaps / AI_ResetRidersAndOrder.

/// `void Rider_RefreshOverlaps(Rider* r)` (0x120e50): re-gather the rider's
/// overlap list and then find the lights near it.
pub fn rider_refresh_overlaps(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x400));
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x860)).lw(c);
        call_ptr(c, OVERLAP_LIST_GATHER_ALL, 0x0012_0e6c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_FIND_NEARBY_LIGHTS.call(c, 0x0012_0e74)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AI_ResetRidersAndOrder(AI* ai)` (0x128a10): reset every rider and
/// then put the order mode back to 0.
pub fn ai_reset_riders_and_order(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AI_RESET_RIDERS.call(c, 0x0012_8a28)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::AI_SET_RIDER_ORDER_MODE.call(c, 0x0012_8a34)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UIState_v13.

/// `void UIState_v13(UIState* s, UIState* target)` (0x39e688): `target`'s
/// seventh virtual method called with 1 (`s`'s `+8` sub-object supplies the
/// vtable).
pub fn ui_state_v13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x8)).lw(c);
        let slot = VSlot::read(c, c.g[V0][0], 0x38);
        c.g[V1][0] = slot.delta;
        c.g[A2][0] = slot.func;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0039_e6ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BENewRaceInt track lookups and BE_CopyProfile.

/// Base of the 23-entry race-entry table (`D_0043D950`).
const RACE_TABLE: u32 = 0x0043_d950;
/// Entry stride: `D_0043D950 + 0x8fc` is the one-past-the-end address.
const RACE_ENTRY_SIZE: u32 = 0x64;
/// Entries in the table.
const RACE_COUNT: u32 = 23;

/// `sRaceEntryFull* BENewRaceInt_FindTrackById(int id)` (0x144ce0): scan the
/// race table and return the entry whose `+0x5c` word equals `id`, else
/// entry 22.
pub fn be_new_race_int_find_track_by_id(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
    let id = c.g[A1][0];
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(RACE_COUNT * RACE_ENTRY_SIZE));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != id {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RACE_ENTRY_SIZE));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0014_4cf0);
                }
                continue;
            }
            c.cycles += 2;
            let ret = c.g[RA][0] as u32;
            c.g[V0][0] = 0x16;
            c.pc = ret;
            return;
        }
        c.cycles += 2;
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        c.pc = ret;
        return;
    }
}

/// `int BENewRaceInt_GetSpecialEventScore(BENewRaceInt* r, int track, int i)`
/// (0x145630): `D_00440D18[i + (track - 6) * 3].+0x12 * 100`.
pub fn be_new_race_int_get_special_event_score(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6));
    c.g[A3][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[V1][0] = 0x0044_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd18));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = 0x64;
    c.mult(0, 6, 6, 7);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x12)).lhu(c);
    let ret = c.g[RA][0] as u32;
    c.mult(0, 2, 2, 4);
    c.pc = ret;
}

/// `int BENewRaceInt_GetSpecialEventValue(BENewRaceInt* r, int track, int i,
/// int k)` (0x145668): `D_00440D18[i + (track - 6) * 3].+0x8[k]`.
pub fn be_new_race_int_get_special_event_value(c: &mut Cpu) {
    c.cycles += 13;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6));
    c.g[A0][0] = 0x14;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[V1][0] = 0x0044_0000;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd18));
    c.mult(0, 6, 6, 4);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lhu(c);
    c.pc = ret;
}

/// `void BE_CopyProfile(int dstIndex, int srcIndex)` (0x14e9c0): copy one
/// `sBEPlayerData` record (stride 0x9b50) out of `D_004A6CA8`.
pub fn be_copy_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x004a_0000;
        c.g[V1][0] = 0x9b50;
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
        let frame = Frame::open(c, 0x10);
        c.mult(0, 2, 6, 3);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.mult(0, 2, 5, 3);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        callee::BE_COPY_PROFILE_CHARACTERS.call(c, 0x0014_e9e8)?;
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
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};

    const SEEDS: u32 = 40;

    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy so
    /// the file/allocator paths fail fast instead of waiting on the disc.
    fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// The shared destructor fixture: half the seeds free a null object (the
    /// `operator delete` path stays off the heap manager) and half keep a real
    /// pointer with the flag cleared.
    fn destructor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        if seed % 2 == 1 {
            c.g[A0][0] = 0;
            c.g[A1][0] = 1;
        } else {
            c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
            c.g[A1][0] = sx(r.next_u32() & !1);
        }
    }

    /// The shared interface-forwarder fixture: `a1` is an object whose first
    /// word is a vtable built by [`fakes::vtable`] (slot 0x10 -> a `call_ptr`
    /// target in empty RAM, so it lands in the fake hook), `a0` the receiver.
    fn virtual_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        let obj = DATA + 0x800;
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(obj);
        let vt = fakes::vtable(c, &mut r, &[slot]);
        c.bus.write32(obj, vt);
    }

    #[test]
    fn position_modifier_v01_matches() {
        check(ee::s12::f_003618c8__generated, position_modifier_v01, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3618_c8)
        });
    }

    #[test]
    fn parent_modifier_v01_matches() {
        check(ee::s12::f_00361988__generated, parent_modifier_v01, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3619_88)
        });
    }

    #[test]
    fn magnet_modifier_v01_matches() {
        check(ee::s12::f_003619e0__generated, magnet_modifier_v01, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3619_e0)
        });
    }

    #[test]
    fn roller_modifier_v01_matches() {
        check(ee::s12::f_00361c78__generated, roller_modifier_v01, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x361c_78)
        });
    }

    #[test]
    fn intc_table_vu1_handler_matches() {
        check(ee::s12::f_003623a8__generated, intc_table_vu1_handler, SEEDS, &|c, seed| {
            machine(c, seed, 0x3623_a8, 2);
            be_library(c);
        });
    }

    #[test]
    fn emitter_dtor_matches() {
        check(ee::s12::f_00370758__generated, emitter_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3707_58)
        });
    }

    #[test]
    fn c_ps2_grid_mesh_v07_matches() {
        check(ee::s12::f_00385be0__generated, c_ps2_grid_mesh_v07, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x385b_e0, 2);
            be_library(c);
            let mesh = DATA + 0x100;
            let sub = DATA + 0x200;
            c.g[A0][0] = sx(mesh);
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(DATA + 0x300);
            // `b + 4` -> a pointer whose first short is the `this` delta.
            c.bus.write32(DATA + 0x304, DATA + 0x500);
            c.bus.write16(DATA + 0x500, r.next_u32() as u16);
            // `self + 4` -> the sub-object whose `+0x40`/`+0x44` are used.
            c.bus.write32(mesh + 4, sub);
            c.bus.write16(sub + 0x40, (r.next_u32() & 0xff) as u16);
            c.bus.write32(sub + 0x44, if seed % 3 == 0 { 0 } else { fakes::FAKE });
        });
    }

    #[test]
    fn irradiance_get_entry_by_name_matches() {
        check(ee::s12::f_0038ac20__generated, irradiance_get_entry_by_name, SEEDS, &|c, seed| {
            machine(c, seed, 0x38ac_20, 2);
            be_library(c);
            let set = DATA + 0x100;
            c.g[A0][0] = sx(set);
            c.g[A1][0] = sx(DATA + 0x300);
            c.bus.write32(set + 4, 3);
            c.bus.write32(set + 8, DATA + 0x1000);
            c.bus.write32(set + 0xc, DATA + 0x200);
            let names: [&[u8]; 3] = [b"aaa\0".as_slice(), b"bbb\0", b"ccc\0"];
            for (i, name) in names.iter().enumerate() {
                for (j, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x200 + (i as u32) * 8 + j as u32, *b);
                }
            }
            let want: &[u8] = if seed % 2 == 0 { b"bbb\0" } else { b"zzz\0" };
            for (j, b) in want.iter().enumerate() {
                c.bus.write8(DATA + 0x300 + j as u32, *b);
            }
        });
    }

    #[test]
    fn grid_mesh_base_base_dtor_matches() {
        check(ee::s13::f_00394b38__generated, grid_mesh_base_base_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x394b_38)
        });
    }

    #[test]
    fn vfx_render_toggles_dtor_matches() {
        check(ee::s13::f_003961a8__generated, vfx_render_toggles_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3961_a8)
        });
    }

    #[test]
    fn vfx_plants_dtor_matches() {
        check(ee::s13::f_003964c8__generated, vfx_plants_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3964_c8)
        });
    }

    #[test]
    fn vfx_tube_dtor_matches() {
        check(ee::s13::f_00396838__generated, vfx_tube_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3968_38)
        });
    }

    #[test]
    fn path_arrow_v01_matches() {
        check(ee::s13::f_003968e8__generated, path_arrow_v01, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3968_e8)
        });
    }

    #[test]
    fn detail_system_v01_matches() {
        check(ee::s13::f_00396928__generated, detail_system_v01, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3969_28)
        });
    }

    #[test]
    fn vfx_visual_effects_test_dtor_matches() {
        check(ee::s13::f_00396a00__generated, vfx_visual_effects_test_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x396a_00)
        });
    }

    #[test]
    fn vfx_screen_tint_dtor_matches() {
        check(ee::s13::f_00396d30__generated, vfx_screen_tint_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x396d_30)
        });
    }

    #[test]
    fn ui_list_node_accept_matches() {
        check(ee::s13::f_00397788__generated, ui_list_node_accept, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x3977_88, 0x10);
        });
    }

    #[test]
    fn ui_list_node_v03_matches() {
        check(ee::s13::f_003977b8__generated, ui_list_node_v03, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x3977_b8, 0x10);
        });
    }

    #[test]
    fn ui_pair_accept_matches() {
        check(ee::s13::f_0039bd38__generated, ui_pair_accept, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x39bd_38, 0x10);
        });
    }

    #[test]
    fn ui_state_v13_matches() {
        check(ee::s13::f_0039e688__generated, ui_state_v13, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x39e6_88, 2);
            be_library(c);
            let obj = DATA + 0x800;
            let sub = DATA + 0x900;
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = sx(obj);
            c.bus.write32(obj + 8, sub);
            let vt = fakes::vtable(c, &mut r, &[0x38]);
            c.bus.write32(sub, vt);
        });
    }

    #[test]
    fn ui_visitor_dtor_matches() {
        check(ee::s13::f_003a47a8__generated, ui_visitor_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3a47_a8)
        });
    }

    #[test]
    fn ui_input_visitor_dtor_matches() {
        check(ee::s13::f_003a4cd0__generated, ui_input_visitor_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3a4c_d0)
        });
    }

    #[test]
    fn ui_focus_visitor_dtor_matches() {
        check(ee::s13::f_003a4d40__generated, ui_focus_visitor_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3a4d_40)
        });
    }

    #[test]
    fn c_ui_group_accept_matches() {
        check(ee::s13::f_003a51a0__generated, c_ui_group_accept, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x3a51_a0, 0x10);
        });
    }

    #[test]
    fn c_world_cache_reset_matches() {
        check(ee::s13::f_003a7f90__generated, c_world_cache_reset, SEEDS, &|c, seed| {
            machine(c, seed, 0x3a7f_90, 2);
            be_library(c);
            // A zeroed cache: no hull pages, no section tables, no streamer.
            c.g[A0][0] = sx(DATA + 0x1000);
        });
    }

    #[test]
    fn c_ps2_chunk_resolve_base_dtor_matches() {
        check(ee::s13::f_003adc18__generated, c_ps2_chunk_resolve_base_dtor, SEEDS, &|c, seed| {
            destructor_setup(c, seed, 0x3adc_18)
        });
    }

    #[test]
    fn mpeg2_initialize_buffer_matches() {
        check(ee::s13::f_003b1228__generated, mpeg2_initialize_buffer, SEEDS, &|c, seed| {
            machine(c, seed, 0x3b12_28, 2);
            be_library(c);
            // The global bitstream descriptor and the data block it points at.
            c.bus.write32(0x0050_a088, DATA + 0x100);
            c.g[A0][0] = sx(DATA + 0x400);
        });
    }

    #[test]
    fn mpeg2_get_bits32_matches() {
        check(ee::s13::f_003b13a8__generated, mpeg2_get_bits32, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3b13_a8, 2);
            be_library(c);
            c.bus.write32(0x0050_a088, DATA + 0x100);
            c.bus.write32(DATA + 0x104, r.next_u32());
            // A bit count that covers both the "refill" loop and its exit.
            c.bus.write32(DATA + 0x108, r.next_u32() % 0x50);
        });
    }

    #[test]
    fn rider_metrix_load_state_matches() {
        check(ee::s00::f_001174e8__generated, rider_metrix_load_state, SEEDS, &|c, seed| {
            virtual_setup(c, seed, 0x1174_e8, 0x10);
        });
    }

    #[test]
    fn rider_refresh_overlaps_matches() {
        check(ee::s00::f_00120e50__generated, rider_refresh_overlaps, SEEDS, &|c, seed| {
            machine(c, seed, 0x120e_50, 2);
            be_library(c);
            // No app / light manager: the light query sees an empty overlap
            // list and returns at once.
            c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
            let rider = DATA + 0x400;
            c.g[A0][0] = sx(rider);
            c.bus.write32(rider + 0x860, 0);
        });
    }

    #[test]
    fn ai_reset_riders_and_order_matches() {
        check(ee::s00::f_00128a10__generated, ai_reset_riders_and_order, SEEDS, &|c, seed| {
            machine(c, seed, 0x128a_10, 2);
            be_library(c);
            // No riders (+0x78) and no missions (+0x88): both reset walks skip.
            c.g[A0][0] = sx(DATA + 0x400);
        });
    }

    macro_rules! load_state_tests {
        ($(($test:ident, $func:ident, $shard:ident, $addr:ident, $salt:expr)),* $(,)?) => {
            $(
                #[test]
                fn $test() {
                    check(ee::$shard::$addr, $func, SEEDS, &|c, seed| {
                        virtual_setup(c, seed, $salt, 0x10);
                    });
                }
            )*
        };
    }

    load_state_tests! {
        (rider_gate_sub_load_state_matches, rider_gate_sub_load_state, s00, f_0012c590, 0x12c5_90),
        (rider_finish_line_sub_load_state_matches, rider_finish_line_sub_load_state, s00, f_0012c9f8, 0x12c9_f8),
        (rider_wipeout_sub_load_state_matches, rider_wipeout_sub_load_state, s00, f_0012e740, 0x12e7_40),
        (rider_reset_sub_load_state_matches, rider_reset_sub_load_state, s00, f_0012f5e8, 0x12f5_e8),
        (rider_natural_air_sub_load_state_matches, rider_natural_air_sub_load_state, s00, f_0012fc28, 0x12fc_28),
        (rider_board_press_sub_load_state_matches, rider_board_press_sub_load_state, s00, f_001315d0, 0x1315_d0),
        (rider_railslide_sub_load_state_matches, rider_railslide_sub_load_state, s01, f_00132878, 0x1328_78),
        (rider_handplant_sub_load_state_matches, rider_handplant_sub_load_state, s01, f_001330f0, 0x1330_f0),
        (rider_spin_sub_load_state_matches, rider_spin_sub_load_state, s01, f_00136230, 0x1362_30),
        (rider_uber_grind_sub_load_state_matches, rider_uber_grind_sub_load_state, s01, f_00136920, 0x1369_20),
        (rider_wipeout_state_load_state_matches, rider_wipeout_state_load_state, s01, f_00138b10, 0x138b_10),
        (rider_handplant_state_load_state_matches, rider_handplant_state_load_state, s01, f_001399a8, 0x1399_a8),
        (rider_air_state_load_state_matches, rider_air_state_load_state, s01, f_0013ace8, 0x13ac_e8),
        (rider_railslide_state_load_state_matches, rider_railslide_state_load_state, s01, f_0013c618, 0x13c6_18),
        (rider_ground_state_load_state_matches, rider_ground_state_load_state, s01, f_0013f880, 0x13f8_80),
    }

    #[test]
    fn be_new_race_int_find_track_by_id_matches() {
        check(ee::s01::f_00144ce0__generated, be_new_race_int_find_track_by_id, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x144c_e0, 1);
            be_library(c);
            const WANTED: u32 = 0x555;
            for i in 0..RACE_COUNT {
                let e = RACE_TABLE + RACE_ENTRY_SIZE * i;
                c.bus.write32(e, r.next_u32());
                // Ids in 0x1000..0x1fff can never collide with the probe.
                let id = if i == 7 { WANTED } else { 0x1000 + r.next_u32() % 0x1000 };
                c.bus.write32(e + 0x5c, id);
            }
            c.g[A1][0] = (if seed % 2 == 0 { WANTED } else { 0x777 }) as u64;
        });
    }

    #[test]
    fn be_new_race_int_get_special_event_score_matches() {
        check(ee::s01::f_00145630__generated, be_new_race_int_get_special_event_score, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1456_30, 1);
            be_library(c);
            // The `D_00440D18` records: stride 0x14, the score at +0x12.
            for i in 0..0x40u32 {
                c.bus.write32(0x0044_0d18 + 4 * i, r.next_u32());
            }
            c.g[A1][0] = if seed % 4 == 0 { 6 + (r.next_u32() % 4) as u64 } else { (r.next_u32() % 9) as u64 };
            c.g[A2][0] = (r.next_u32() % 4) as u64;
        });
    }

    #[test]
    fn be_new_race_int_get_special_event_value_matches() {
        check(ee::s01::f_00145668__generated, be_new_race_int_get_special_event_value, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x1456_68, 1);
            be_library(c);
            for i in 0..0x40u32 {
                c.bus.write32(0x0044_0d18 + 4 * i, r.next_u32());
            }
            c.g[A1][0] = if seed % 4 == 0 { 6 + (r.next_u32() % 4) as u64 } else { (r.next_u32() % 9) as u64 };
            c.g[A2][0] = (r.next_u32() % 4) as u64;
            c.g[A3][0] = (r.next_u32() % 8) as u64;
        });
    }

    #[test]
    fn be_copy_profile_matches() {
        check(ee::s01::f_0014e9c0__generated, be_copy_profile, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x14e9_c0, 2);
            be_library(c);
            // Two of the four 0x9b50-byte player records (`D_004A6CA8`).
            c.g[A0][0] = (r.next_u32() % 4) as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
        });
    }
}
