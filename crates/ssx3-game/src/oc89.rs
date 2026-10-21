//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for small leaf functions, each with a differential test
//! against the generated body.
//!
//! - **Front-end widget handlers**: the `*_OnWidgetNotify` /
//!   `*_OnWidgetAction` / `*_OnChildEvent` filter functions, which answer
//!   0x100/0x101/0 to the widget system (and the two menu item comparators
//!   the list widgets sort with).
//! - **Load screens**: `*_TryExit`, which mark the screen done and poll the
//!   loading object behind the world.
//! - **Collision / rails**: hull diameter, sphere transform reset and the
//!   four rail impulse helpers (VU0, issued as the original's COP2 macro
//!   operations on `c.vu0()`).
//! - **PS2 chunk resolver**: the per-kind hash lookups, list re-linking and
//!   the multi-entry resolve loop.
//! - **Misc**: the pad actuator, the conveyor UV wrap, the counter event
//!   latch, the halo angle wrap, the UI colour fill and the two race
//!   settings copies.
//!
//! Every replacement reproduces the generated body's registers (including
//! `v0`/`v1` scratch and the `AT` constants the FPU compares load), its cycle
//! charges block by block, and its memory writes. Loops deliver events at the
//! same back edges with [`event_due`]/[`deliver_event_here`], keeping their
//! state in the guest registers across the check.

use crate::abi;
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_f8c0, coll_hull_get_diameter, "CollHull_GetDiameter"),
    (0x0032_fd98, coll_sphere_reset_transform, "CollSphere_ResetTransform"),
    (0x0032_7740, ps2pad_set_actuator, "PS2Pad_SetActuator"),
    (0x0034_34e8, conveyor_update, "Conveyor_Update"),
    (0x0034_3868, counter_handle_event, "Counter_HandleEvent"),
    (0x001d_4660, cfe_state_trophy_room_on_widget_notify, "cFEStateTrophyRoom_OnWidgetNotify"),
    (0x001d_4f68, cfe_state_rewards_room_on_widget_notify, "cFEStateRewardsRoom_OnWidgetNotify"),
    (0x001d_5460, mem_card_state_base_on_widget_notify, "MemCardStateBase_OnWidgetNotify"),
    (0x003a_a6e8, cps2_chunk_resolve_v16, "cPS2ChunkResolve_v16"),
    (0x003a_a700, cps2_chunk_resolve_v17, "cPS2ChunkResolve_v17"),
    (0x003a_a758, cps2_chunk_resolve_v09, "cPS2ChunkResolve_v09"),
    (0x003a_a780, cps2_chunk_resolve_v10, "cPS2ChunkResolve_v10"),
    (0x003a_a8d8, cps2_chunk_resolve_v12, "cPS2ChunkResolve_v12"),
    (0x003a_a8f0, cps2_chunk_resolve_base_v13, "cPS2ChunkResolveBase_v13"),
    (0x003a_a960, cps2_chunk_resolve_v18, "cPS2ChunkResolve_v18"),
    (0x003a_ac50, cps2_chunk_resolve_v21, "cPS2ChunkResolve_v21"),
    (0x003a_aca8, cps2_chunk_resolve_v22, "cPS2ChunkResolve_v22"),
    (0x003a_ae40, cps2_chunk_resolve_init, "cPS2ChunkResolve_Init"),
    (0x0023_2510, pre_start_screen_try_exit, "PreStartScreen_TryExit"),
    (0x0023_26d8, init_load_screen_try_exit, "InitLoadScreen_TryExit"),
    (0x0023_27b0, cpre_game_load_screen_try_exit, "cPreGameLoadScreen_TryExit"),
    (0x0023_27e8, cpre_fe_load_screen_try_exit, "cPreFELoadScreen_TryExit"),
    (0x003a_3398, ui_vector_set_color, "UIVector_SetColor"),
    (0x0031_ad00, ps2_exec_man_v10, "PS2ExecMan_v10"),
    (0x001f_4a38, cfe_state_buy_attrib_on_widget_notify, "cFEStateBuyAttrib_OnWidgetNotify"),
    (0x001f_5618, cfe_state_career_stats_on_widget_notify, "cFEStateCareerStats_OnWidgetNotify"),
    (0x0034_6258, halo_modifier_update, "HaloModifier_Update"),
    (0x0034_9798, float_rail_apply_impulse, "FloatRail_ApplyImpulse"),
    (0x0034_9840, float_rail_get_contact_velocity, "FloatRail_GetContactVelocity"),
    (0x0034_a568, spring_rail_apply_impulse, "SpringRail_ApplyImpulse"),
    (0x0024_5e30, game_mode_man_compare_by_score, "GameModeMan_CompareByScore"),
    (0x0024_5e78, game_mode_man_compare_by_time, "GameModeMan_CompareByTime"),
    (0x0024_5ec0, game_mode_man_compare_standings, "GameModeMan_CompareStandings"),
    (0x0024_6268, cfe_state_progressive_verify_ps2_on_widget_notify, "cFEStateProgressiveVerifyPS2_OnWidgetNotify"),
    (0x0035_9cf8, spline_modifier_v22, "SplineModifier_v22"),
    (0x0020_8ef8, ov_screen_session_on_widget_notify, "OVScreenSession_OnWidgetNotify"),
    (0x001f_72b8, ov_screen_enter_lodge_on_widget_notify, "OVScreenEnterLodge_OnWidgetNotify"),
    (0x001f_7590, ov_screen_bc_start_on_widget_notify, "OVScreenBCStart_OnWidgetNotify"),
    (0x001f_77f0, ov_screen_bc_fail_on_widget_notify, "OVScreenBCFail_OnWidgetNotify"),
    (0x001f_80c8, ov_screen_top_times_on_widget_notify, "OVScreenTopTimes_OnWidgetNotify"),
    (0x0019_8da0, cfe_state_char_equip_compare_items, "cFEStateCharEquip_CompareItems"),
    (0x0020_d308, ov_screen_overlay_menu_on_widget_notify, "OVScreenOverlayMenu_OnWidgetNotify"),
    (0x0020_d318, ov_screen_overlay_menu_on_widget_action, "OVScreenOverlayMenu_OnWidgetAction"),
    (0x0020_d568, ov_screen_yn_dialog_on_widget_notify, "OVScreenYNDialog_OnWidgetNotify"),
    (0x001d_2f40, cfe_state_mountain_room_on_widget_notify, "cFEStateMountainRoom_OnWidgetNotify"),
    (0x0014_4928, be_new_race_int_commit, "BENewRaceInt_Commit"),
    (0x0014_49e8, be_new_race_int_reset, "BENewRaceInt_Reset"),
    (0x0021_0dd0, ov_screen_read_private_message_on_child_event, "OVScreenReadPrivateMessage_OnChildEvent"),
    (0x0030_db90, rider_model_compare_int, "RiderModel_CompareInt"),
    (0x0019_4a48, cfe_state_title_on_widget_notify, "cFEStateTitle_OnWidgetNotify"),
];

/// `base + off` (a 32-bit address add).
#[inline(always)]
fn p(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

// ---------------------------------------------------------------------------
// Front-end widget handlers.

/// The handlers that accept only the "start" widget (`key == 6`) and answer
/// 0x101 (handled) or 0: `v0 = (a2 == 6) ? 0x101 : 0`, with the original's
/// `a2 ^= 6` in place.
fn accept_key_6(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = if c.g[A2][0] == 0 { 0x101 } else { 0 };
    abi::ret(c);
}

/// `int OVScreenBCStart_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f7590).
pub fn ov_screen_bc_start_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenBCFail_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f77f0).
pub fn ov_screen_bc_fail_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenTopTimes_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f80c8).
pub fn ov_screen_top_times_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenOverlayMenu_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x0020d308).
pub fn ov_screen_overlay_menu_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int cFEStateProgressiveVerifyPS2_OnWidgetNotify(State*, void*, int key)`
/// (0x00246268).
pub fn cfe_state_progressive_verify_ps2_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int cFEStateTitle_OnWidgetNotify(State*, void*, int key)` (0x00194a48):
/// the title accepts every key but reports 0x101 only for the start widget.
pub fn cfe_state_title_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x6;
    c.g[V1][0] = 0x101;
    c.g[V0][0] = if c.g[A2][0] == 0 { 0x101 } else { 0x100 };
    abi::ret(c);
}

/// `int OVScreenSession_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x00208ef8): 0 for the keys 8 and 9, 0x101 for the rest.
pub fn ov_screen_session_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x8));
    c.g[V0][0] = 0x101;
    c.g[A2][0] = (c.g[A2][0] < 0x2) as u64;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// The list-widget handlers that answer 0x100 (handled) for the scroll keys
/// 8 and 9 and 0x101 (ignored) otherwise, keeping the original's compare
/// chain (`key < 10`, then `key < 8`) and its cycle charges.
fn handle_list_key(c: &mut Cpu) {
    c.cycles += 3;
    let key = abi::arg64(c, 2);
    c.g[V1][0] = (key < 0xa) as u64;
    c.g[V0][0] = 0x101;
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = (key < 0x8) as u64;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = if key < 0x8 { 0x101 } else { 0x100 };
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateMountainRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d2f40).
pub fn cfe_state_mountain_room_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int cFEStateRewardsRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d4f68).
pub fn cfe_state_rewards_room_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int MemCardStateBase_OnWidgetNotify(State*, void*, int key)`
/// (0x001d5460).
pub fn mem_card_state_base_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int cFEStateBuyAttrib_OnWidgetNotify(State*, void*, int key)`
/// (0x001f4a38).
pub fn cfe_state_buy_attrib_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int OVScreenEnterLodge_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f72b8).
pub fn ov_screen_enter_lodge_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int OVScreenYNDialog_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x0020d568).
pub fn ov_screen_yn_dialog_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// The handlers that accept the keys 6, 8 and 9 (0x100) and ignore the rest
/// (0x101). The original tests 6, then `< 6`, `< 10`, `< 8`, so the cycle
/// charge depends on where the key leaves the chain.
fn handle_key_6_8_9(c: &mut Cpu) {
    let key = abi::arg64(c, 2);
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is6 = key == 6;
    c.g[V0][0] = (key < 6) as u64;
    let accept = if is6 {
        true
    } else {
        c.cycles += 2;
        let below6 = c.g[V0][0] != 0;
        c.g[V0][0] = (key < 0xa) as u64;
        if below6 {
            false
        } else {
            c.cycles += 2;
            let above9 = c.g[V0][0] == 0;
            c.g[V0][0] = (key < 8) as u64;
            if above9 {
                false
            } else {
                c.cycles += 2;
                c.g[V0][0] == 0
            }
        }
    };
    c.cycles += 2;
    c.g[V0][0] = if accept { 0x100 } else { 0x101 };
    abi::ret(c);
}

/// `int cFEStateTrophyRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d4660).
pub fn cfe_state_trophy_room_on_widget_notify(c: &mut Cpu) {
    handle_key_6_8_9(c);
}

/// `int cFEStateCareerStats_OnWidgetNotify(State*, void*, int key)`
/// (0x001f5618).
pub fn cfe_state_career_stats_on_widget_notify(c: &mut Cpu) {
    handle_key_6_8_9(c);
}

/// `void OVScreenOverlayMenu_OnWidgetAction(OVScreen*, void* item, int action)`
/// (0x0020d318): picking item action 5 sets the global overlay-menu flag
/// (`gp-0x9f4`, 0x4a26fc).
pub fn ov_screen_overlay_menu_on_widget_action(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 5;
    if abi::arg64(c, 1) != 0 {
        c.cycles += 2;
        let action = abi::arg64(c, 2);
        c.g[V0][0] = 1;
        if action == 5 {
            c.cycles += 1;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void OVScreenReadPrivateMessage_OnChildEvent(OVScreen*, int id, int ev)`
/// (0x00210dd0): the event 0x16 clears the pending message id at +0xe0.
pub fn ov_screen_read_private_message_on_child_event(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    let id = abi::arg64(c, 1);
    c.g[V0][0] = 0x16;
    if abi::arg64(c, 2) == 0x16 {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
        if id == c.g[V0][0] {
            c.sw(p(self_, 0xe0), 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateCharEquip_CompareItems(Item** a, Item** b)` (0x00198da0):
/// the list order is the signed short sort key at +0xa of each item.
pub fn cfe_state_char_equip_compare_items(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V1][0] = Addr(abi::arg(c, 1)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa)).lh(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `int RiderModel_CompareInt(int** a, int** b)` (0x0030db90):
/// `*a - *b`.
pub fn rider_model_compare_int(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V0][0] = Addr(abi::arg(c, 1)).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Load screens.

/// The three load-screen `TryExit`s: mark the screen done (`done_off`),
/// then return whether the loading object behind the world has finished
/// (its progress float at +4 compares equal to 0.0).
fn load_screen_try_exit(c: &mut Cpu, done_off: u32) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = 1;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.sw(p(self_, done_off), c.g[V0][0]);
    c.mtc1(0, 1); // f1 = 0.0 (the zero register)
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
    c.c_eq_s(0, 1);
    let done = c.fcc();
    c.g[A0][0] = 1;
    if !done {
        c.cycles += 1;
        c.g[A0][0] = 0;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `int PreStartScreen_TryExit(Screen* s)` (0x00232510): true once the
/// screen's timer (`+4`) reached 0.75 s.
pub fn pre_start_screen_try_exit(c: &mut Cpu) {
    c.cycles += 7;
    let self_ = abi::arg(c, 0);
    c.lwc1(1, p(self_, 0x4));
    c.g[AT][0] = 0x3f40_0000; // 0.75f
    c.mtc1(1, 0);
    c.c_lt_s(1, 0);
    let below = c.fcc();
    c.g[V0][0] = 0;
    if !below {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int InitLoadScreen_TryExit(InitLoadScreen* s)` (0x002326d8).
pub fn init_load_screen_try_exit(c: &mut Cpu) {
    load_screen_try_exit(c, 0x8);
}

/// `int cPreGameLoadScreen_TryExit(cPreGameLoadScreen* s)` (0x002327b0).
pub fn cpre_game_load_screen_try_exit(c: &mut Cpu) {
    load_screen_try_exit(c, 0x4);
}

/// `int cPreFELoadScreen_TryExit(cPreFELoadScreen* s)` (0x002327e8).
pub fn cpre_fe_load_screen_try_exit(c: &mut Cpu) {
    load_screen_try_exit(c, 0x4);
}

// ---------------------------------------------------------------------------
// Misc small leaves.

/// `int PS2ExecMan_v10(PS2ExecMan* m)` (0x0031ad00): true when the slot at
/// `+0x14048` is idle.
pub fn ps2_exec_man_v10(c: &mut Cpu) {
    c.cycles += 5;
    let base = abi::arg(c, 0).wrapping_add(0x1_0000);
    c.g[V0][0] = 0x1_0000;
    c.g[A0][0] = sx(base);
    c.g[V0][0] = c.lw(base.wrapping_add(0x4048));
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v16(void* self, ResolveNode* node)` (0x003aa6e8):
/// the node's `+0x20` word becomes its own address (a self-relative base).
pub fn cps2_chunk_resolve_v16(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    c.g[V1][0] = Addr(p(node, 0x20)).lw(c);
    c.g[V0][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(node));
    c.sw(p(node, 0x20), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v12(void* self, ResolveNode* node, int v)`
/// (0x003aa8d8): `node->base += v`.
pub fn cps2_chunk_resolve_v12(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let v = abi::arg(c, 2);
    c.g[V1][0] = Addr(p(node, 0x4)).lw(c);
    c.g[V0][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(v));
    c.sw(p(node, 0x4), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_Init(cPS2ChunkResolve* self, int n)` (0x003aae40):
/// remember the world and the chunk count.
pub fn cps2_chunk_resolve_init(c: &mut Cpu) {
    c.cycles += 8;
    let self_ = abi::arg(c, 0);
    let n = abi::arg(c, 1);
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8)); // the world
    c.sw(p(self_, 0xc), n as u64);
    c.sw(self_, c.g[A2][0]);
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2a));
    c.sw(p(self_, 0x10), c.g[A1][0]);
    abi::ret(c);
}

/// `float CollHull_GetDiameter(CollHull* hull)` (0x0032f8c0): -1 unless the
/// hull has a shape, else twice its bounding sphere's radius.
pub fn coll_hull_get_diameter(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 4;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 0);
    } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw(abi::arg(c, 0).wrapping_add(0x60)); // the sphere
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.add_s(0, 0, 0);
    }
    abi::ret(c);
}

/// `void CollSphere_ResetTransform(CollSphere* s)` (0x0032fd98): the
/// transform at +0x80/+0x90 becomes the current one at +0x60/+0x70.
pub fn coll_sphere_reset_transform(c: &mut Cpu) {
    c.cycles += 6;
    let s = abi::arg(c, 0);
    c.lq(2, p(s, 0x80));
    c.lwc1(0, p(s, 0x90));
    c.sq(2, p(s, 0x60));
    c.swc1(0, p(s, 0x70));
    c.sw(p(s, 0x4), 0);
    abi::ret(c);
}

/// `void PS2Pad_SetActuator(PS2Pad* pad, int mode, float strength)`
/// (0x00327740): with mode 1 store whether the actuator is on at +0x30;
/// otherwise scale `strength` to a motor value clamped to 50..255 at +0x38.
pub fn ps2pad_set_actuator(c: &mut Cpu) {
    c.cycles += 3;
    let pad = abi::arg(c, 0);
    let mode = abi::arg64(c, 1);
    c.g[V0][0] = 1;
    if mode == 1 {
        // *(int*)(pad + 0x30) = strength != 0
        c.cycles += 5;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_eq_s(12, 0);
        if c.fcc() {
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        c.sw(p(pad, 0x30), c.g[V0][0]);
        abi::ret(c);
        return;
    }
    // *(int*)(pad + 0x38) = clamp((int)(strength * 205 + 50), 50, 255)
    c.cycles += 5;
    c.mtc1(0, 0); // f0 = 0.0
    c.c_eq_s(12, 0);
    let off = c.fcc();
    c.g[A0][0] = sx(p(pad, 0x38));
    let value = if off {
        c.cycles += 1;
        c.g[V0][0] = 0;
        0u64
    } else {
        c.cycles += 11;
        c.g[AT][0] = 0x434d_0000; // 205.0f
        c.mtc1(1, 0);
        c.g[AT][0] = 0x4248_0000; // 50.0f
        c.mtc1(1, 1);
        c.mul_s(0, 12, 0);
        c.add_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(3, 1); // v1 = (int)(strength * 205 + 50)
        c.g[V0][0] = ((c.g[V1][0] as i64) < 50) as u64;
        let below = c.g[V0][0] != 0;
        c.g[A1][0] = 0xff;
        if below {
            c.cycles += 1;
            c.g[V1][0] = 0x32; // 50
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[V1][0];
        c.g[V1][0]
    };
    c.cycles += 2;
    c.sw(c.g[A0][0] as u32, value);
    abi::ret(c);
}

/// `void Conveyor_Update(Conveyor* c)` (0x003434e8): advance the UV scroll
/// offsets and wrap each back into (-1, 1].
pub fn conveyor_update(c: &mut Cpu) {
    c.cycles += 12;
    let s = abi::arg(c, 0);
    c.lwc1(3, p(s, 0x40)); // u
    c.lwc1(2, p(s, 0x38)); // du
    c.lwc1(1, p(s, 0x44)); // v
    c.add_s(3, 3, 2);
    c.lwc1(0, p(s, 0x3c)); // dv
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 4);
    c.add_s(1, 1, 0);
    c.c_lt_s(4, 3); // 1.0 < u
    c.swc1(3, p(s, 0x40));
    let over_u = c.fcc();
    c.swc1(1, p(s, 0x44));
    if over_u {
        c.cycles += 2;
        c.sub_s(0, 3, 4);
        c.cycles += 2;
        c.swc1(0, p(s, 0x40));
        c.lwc1(1, p(s, 0x44));
    } else {
        c.cycles += 6;
        c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
        c.mtc1(1, 0);
        c.c_lt_s(3, 0); // u < -1.0
        if c.fcc() {
            c.cycles += 1;
            c.add_s(0, 3, 4);
            c.cycles += 2;
            c.swc1(0, p(s, 0x40));
            c.lwc1(1, p(s, 0x44));
        } else {
            c.lwc1(1, p(s, 0x44));
        }
    }
    c.cycles += 5;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 2);
    c.c_lt_s(2, 1); // 1.0 < v
    if c.fcc() {
        c.cycles += 3;
        c.sub_s(0, 1, 2);
        c.swc1(0, p(s, 0x44));
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
    c.mtc1(1, 0);
    c.c_lt_s(1, 0); // v < -1.0
    if c.fcc() {
        c.cycles += 2;
        c.add_s(0, 1, 2);
        c.swc1(0, p(s, 0x44));
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Counter_HandleEvent(Counter* self, int msg, float level)`
/// (0x00343868): latch one bit per level 1..10 at +0x20 and, when the latch
/// fires, count the counter at +0x1c down.
pub fn counter_handle_event(c: &mut Cpu) {
    let self_ = abi::arg(c, 0);
    let msg = abi::arg64(c, 1);
    // bit = 1 << (level - 1) for the levels 1..10, 0 otherwise, from the
    // equality chain against 1.0f .. 10.0f.
    const LEVELS: [(u32, u64); 8] = [
        (0x4000_0000, 0x2),   // 2.0f
        (0x4040_0000, 0x4),   // 3.0f
        (0x4080_0000, 0x8),   // 4.0f
        (0x40a0_0000, 0x10),  // 5.0f
        (0x40c0_0000, 0x20),  // 6.0f
        (0x40e0_0000, 0x40),  // 7.0f
        (0x4100_0000, 0x80),  // 8.0f
        (0x4110_0000, 0x100), // 9.0f
    ];
    c.cycles += 7;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.g[A2][0] = 0;
    c.c_eq_s(12, 0);
    let is_one = c.fcc();
    c.g[A3][0] = 0; // fire
    let mut bit = 0u64;
    if is_one {
        c.cycles += 2;
        bit = 1;
    } else {
        for (bits, mask) in LEVELS {
            c.cycles += 5;
            c.g[AT][0] = bits as u64;
            c.mtc1(1, 0);
            c.c_eq_s(12, 0);
            if c.fcc() {
                c.cycles += 2;
                bit = mask;
                break;
            }
        }
        if bit == 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x4120_0000; // 10.0f
            c.mtc1(1, 0);
            c.c_eq_s(12, 0);
            if c.fcc() {
                bit = 0x200;
            }
        }
    }
    c.g[A2][0] = bit; // the latch bit the original keeps in a2
    // The message updates the latch (+0x20): 0x1f4 presses, 0x1f5 releases,
    // 0x1f6 is the "fire now" message.
    let mut store: Option<u64> = None;
    let mut fire = 0u64;
    c.cycles += 3;
    c.g[V0][0] = 0x1f4;
    let is_press = msg == 0x1f4;
    c.g[V0][0] = 0x1f5;
    if is_press {
        c.cycles += 4;
        let flags = Addr(p(self_, 0x20)).lw(c);
        c.g[V1][0] = flags;
        c.g[V0][0] = flags | bit;
        if flags & bit == 0 {
            c.cycles += 2;
            c.g[A3][0] = 1;
            fire = 1;
            store = Some(flags | bit);
        }
    } else {
        c.cycles += 2;
        let is_release = msg == 0x1f5;
        c.g[V0][0] = msg ^ 0x1f6;
        if !is_release {
            c.cycles += 1;
            c.g[A3][0] = (c.g[V0][0] < 1) as u64;
            fire = c.g[A3][0];
        } else {
            c.cycles += 4;
            let flags = Addr(p(self_, 0x20)).lw(c);
            c.g[V1][0] = flags;
            c.g[V0][0] = flags & bit;
            if flags & bit == 0 {
                c.cycles += 6;
                c.g[AT][0] = 0x3f80_0000; // 1.0f
                c.mtc1(1, 0);
                c.c_eq_s(12, 0);
                let level_one = c.fcc();
                c.g[V0][0] = flags | 1;
                let fires = if level_one {
                    true
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((bit as u32) as i32 >> 1) as u32);
                    c.g[V0][0] = flags & c.g[V0][0];
                    let lower_clear = c.g[V0][0] == 0;
                    c.g[V0][0] = flags | bit;
                    !lower_clear
                };
                if fires {
                    c.cycles += 1;
                    c.g[A3][0] = 1;
                    fire = 1;
                    store = Some(c.g[V0][0]);
                }
            }
        }
    }
    if let Some(v) = store {
        // The latch store and the test before it are separate blocks.
        c.cycles += 2;
        c.sw(p(self_, 0x20), v);
    }
    // The "fire" test is always executed; the counter update only when set.
    c.cycles += 2;
    if fire != 0 {
        c.cycles += 3;
        let count = Addr(p(self_, 0x1c)).lw(c);
        let expired = (count as i64) <= 0;
        c.g[V0][0] = sx((count as u32).wrapping_sub(1));
        if !expired {
            c.cycles += 1;
            c.sw(p(self_, 0x1c), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void HaloModifier_Update(HaloModifier* h)` (0x00346258): rotate by the
/// per-frame step (an int at +0x2c) and wrap the angle at +0x30 at +-360.
pub fn halo_modifier_update(c: &mut Cpu) {
    let h = abi::arg(c, 0);
    c.cycles += 10;
    c.lwc1(1, p(h, 0x2c));
    c.cvt_s_w(1, 1);
    c.lwc1(0, p(h, 0x30));
    c.g[AT][0] = 0x43b4_0000; // 360.0f
    c.mtc1(1, 2);
    c.add_s(1, 0, 1);
    c.c_le_s(2, 1); // 360 <= angle
    let wrap = c.fcc();
    c.swc1(1, p(h, 0x30));
    if wrap {
        c.cycles += 1;
        c.sw(p(h, 0x30), 0);
    } else {
        c.cycles += 5;
        c.g[AT][0] = 0xffff_ffff_c3b4_0000; // -360.0f
        c.mtc1(1, 0);
        c.c_le_s(1, 0); // angle <= -360
        if c.fcc() {
            c.cycles += 1;
            c.sw(p(h, 0x30), 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int GameModeMan_CompareByScore(GameMode** a, GameMode** b)`
/// (0x00245e30): -1 when b's score is better, else 1 for unequal scores
/// and 0 for a tie.
pub fn game_mode_man_compare_by_score(c: &mut Cpu) {
    c.cycles += 13;
    let table = 0x0053_6640u32;
    let ia = Addr(abi::arg(c, 0)).lw(c) as u32;
    let ib = Addr(abi::arg(c, 1)).lw(c) as u32;
    c.g[A2][0] = sx((ia << 2).wrapping_add(table));
    c.g[V1][0] = sx((ib << 2).wrapping_add(table));
    c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
    let better = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A2][0] ^ c.g[V1][0];
    c.cycles += 2;
    if better {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    }
    abi::ret(c);
}

/// `int GameModeMan_CompareByTime(GameMode** a, GameMode** b)` (0x00245e78):
/// the same order with the comparison reversed (a shorter time is better).
pub fn game_mode_man_compare_by_time(c: &mut Cpu) {
    c.cycles += 13;
    let table = 0x0053_6640u32;
    let ia = Addr(abi::arg(c, 0)).lw(c) as u32;
    let ib = Addr(abi::arg(c, 1)).lw(c) as u32;
    c.g[A2][0] = sx((ia << 2).wrapping_add(table));
    c.g[V1][0] = sx((ib << 2).wrapping_add(table));
    c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = (c.g[A2][0] < c.g[V1][0]) as u64;
    let better = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[A2][0] ^ c.g[V1][0];
    c.cycles += 2;
    if better {
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    }
    abi::ret(c);
}

/// `int GameModeMan_CompareStandings(GameMode** a, GameMode** b)`
/// (0x00245ec0): 1 when b's standing is better, else -1 (never 0).
pub fn game_mode_man_compare_standings(c: &mut Cpu) {
    c.cycles += 15;
    let table = 0x0053_6708u32;
    let ia = Addr(abi::arg(c, 0)).lw(c) as u32;
    let ib = Addr(abi::arg(c, 1)).lw(c) as u32;
    c.g[A2][0] = sx(ia << 2);
    c.g[V1][0] = table as u64;
    c.g[V0][0] = sx(ib << 2);
    c.g[A1][0] = 1;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(table));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(table));
    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[V0][0] = c.g[A1][0];
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// cPS2ChunkResolve: the per-kind hash lookups.

/// `int cPS2ChunkResolve_v17(void* self, ResolveNode* node)` (0x003aa700):
/// resolve the hash at `node+0x64` through the `+0x24` table, store the
/// result and clear `node+0x84`.
pub fn cps2_chunk_resolve_v17(c: &mut Cpu) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = Addr(self_).lw(c);
    c.g[A3][0] = Addr(p(node, 0x64)).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c); // *(self)
    c.g[V0][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8)); // section table
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
    let h = c.g[A3][0];
    if c.g[V1][0] != 0 {
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.cycles += 7;
        c.g[V0][0] = sx((h as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if !hit {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    } else {
        c.cycles += 2;
        c.g[V1][0] = 0;
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 4;
    c.sw(p(node, 0x64), c.g[V0][0]);
    c.g[V0][0] = 1;
    c.sw(p(node, 0x84), 0);
    abi::ret(c);
}

/// `int cPS2ChunkResolveBase_v13(void* self, ResolveNode* node)`
/// (0x003aa8f0): resolve `node+0x80` through the `+0x14` table, then seed
/// `node+0x8c`/`+0x94` from the resolved structure.
pub fn cps2_chunk_resolve_base_v13(c: &mut Cpu) {
    c.cycles += 10;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    c.g[A2][0] = Addr(self_).lw(c);
    c.g[A3][0] = Addr(p(node, 0x80)).lw(c);
    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c); // *(self)
    c.g[V0][0] = c.g[A3][0] & 0xff;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8)); // section table
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
    let h = c.g[A3][0];
    if c.g[V1][0] == 0 {
        // No section: the node keeps the null pointer (v1 = 0).
        c.cycles += 2;
        c.g[V1][0] = 0;
    } else {
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        c.cycles += 7;
        c.g[V0][0] = sx((h as u32) >> 8);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        }
        c.cycles += 1;
        c.g[V1][0] = c.g[V0][0];
    }
    c.cycles += 6;
    c.sw(p(node, 0x80), c.g[V1][0]);
    c.g[V0][0] = 1;
    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.sw(p(node, 0x8c), 0);
    c.sw(p(node, 0x94), c.g[V1][0]);
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v09(void* self, ResolveNode* node)` (0x003aa758):
/// point `node+0x10` at `node+0x14` (or 0 when it was -1).
pub fn cps2_chunk_resolve_v09(c: &mut Cpu) {
    c.cycles += 4;
    let node = abi::arg(c, 1);
    c.g[V0][0] = Addr(p(node, 0x10)).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    let unset = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = sx(p(node, 0x14));
    if unset {
        c.cycles += 2;
        c.sw(p(node, 0x10), 0);
    } else {
        c.cycles += 1;
        c.sw(p(node, 0x10), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v10(void* self, int n, u32* data)` (0x003aa780):
/// resolve every hash of `n` counted arrays in place; `data` advances by
/// the array's own length.
pub fn cps2_chunk_resolve_v10(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T3][0] = c.g[A0][0]; // self
    let n = c.g[A1][0];
    c.g[A0][0] = 0; // g
    if (n as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32); // *data
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
        let count = c.g[V0][0];
        c.g[T1][0] = 0;
        if (count as i64) > 0 {
            c.cycles += 1;
            loop {
                // h = *hp; look it up; *hp = res
                c.cycles += 10;
                c.g[V1][0] = c.lw(c.g[T3][0] as u32); // *(self)
                c.g[A3][0] = c.lw(c.g[T0][0] as u32); // h
                c.g[A0][0] = c.lw(c.g[V1][0] as u32); // *(*(self))
                c.g[V0][0] = c.g[A3][0] & 0xff;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32); // section
                let h = c.g[A3][0];
                let missing = c.g[V1][0] == 0;
                c.g[V0][0] = sx((h as u32) >> 8);
                let mut res = 0u64;
                if missing {
                    c.cycles += 1;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        res = c.g[V0][0];
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = res;
                c.sw(c.g[T0][0] as u32, c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_a7a8);
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[T2][0];
        c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_a790);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v18(void* self, List* list)` (0x003aa960): rebuild
/// the ring indices (`prev`/`next` at +0x60/+0x64, owner at +0x68) of every
/// node of the list at +0x30.
pub fn cps2_chunk_resolve_v18(c: &mut Cpu) {
    c.cycles += 7;
    let list = abi::arg(c, 1);
    c.g[V0][0] = Addr(p(list, 0x20)).lw(c);
    c.g[T5][0] = sx(p(list, 0x30)); // nodes
    c.g[T1][0] = 1; // next index
    c.sw(p(list, 0x24), c.g[T5][0]);
    c.g[T0][0] = 0; // i
    let count = c.g[V0][0];
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1)); // prev index
    if (count as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T3][0] = 0x90; // node stride
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[A2][0] = c.g[T5][0];
    loop {
        // node->prev: -1 becomes 0, else &nodes[prev]
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x60));
        let unset = c.g[V0][0] == c.g[T2][0];
        c.mult(0, 3, 7, 11); // prev * 0x90 -> v1
        if unset {
            c.cycles += 1;
            c.sw((c.g[A2][0] as u32).wrapping_add(0x60), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.sw((c.g[A2][0] as u32).wrapping_add(0x60), c.g[V0][0]);
        }
        // node->next: the same for +0x64
        c.cycles += 6;
        c.g[A0][0] = 0x90;
        c.mult(0, 2, 8, 4); // i * 0x90 -> v0
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
        c.mult(0, 12, 9, 4); // next * 0x90 -> t4
        if c.g[V0][0] == c.g[T2][0] {
            c.cycles += 1;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[T5][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
        c.sw((c.g[A2][0] as u32).wrapping_add(0x68), c.g[A1][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A3][0] = 0;
        }
        if c.g[A0][0] == 0 {
            c.g[T1][0] = 0;
        }
        c.g[V1][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x90));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_a988);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v21(void* self, ResolveNode* node)` (0x003aac50):
/// clamp the resolved object's value at +0x1c to 5000 while it is still
/// initialising (0 < state < 3).
pub fn cps2_chunk_resolve_v21(c: &mut Cpu) {
    let node = abi::arg(c, 1);
    c.cycles += 4;
    c.g[V0][0] = Addr(p(node, 0xc)).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x100;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V1][0] = Addr(p(node, 0x10)).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if (c.g[V1][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.lwc1(0, p(node, 0x1c));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x23b8)); // 5000.0f
    c.c_lt_s(1, 0); // 5000 < value
    if c.fcc() {
        c.swc1(1, p(node, 0x1c));
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `int cPS2ChunkResolve_v22(void* self, ResolveNode* node)` (0x003aaca8):
/// true while the object is initialising (state 1 or 2) and not flagged.
pub fn cps2_chunk_resolve_v22(c: &mut Cpu) {
    let node = abi::arg(c, 1);
    c.cycles += 4;
    c.g[V0][0] = Addr(p(node, 0xc)).lw(c);
    c.g[V0][0] = c.g[V0][0] & 0x100;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[A1][0] = Addr(p(node, 0x10)).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
    let below3 = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if below3 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        if (c.g[A1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void BENewRaceInt_Commit(void)` (0x00144928): copy the working race
/// settings (0x535bc8) over the live copy (0x5305b0).
pub fn be_new_race_int_commit(c: &mut Cpu) {
    copy_race_settings(c, 0x0053_5bc8, 0x0053_05b0, 0x0014_4948, 0x0014_499c);
}

/// `void BENewRaceInt_Reset(void)` (0x001449e8): copy the live race
/// settings (0x5305b0) back over the working ones (0x535bc8).
pub fn be_new_race_int_reset(c: &mut Cpu) {
    copy_race_settings(c, 0x0053_05b0, 0x0053_5bc8, 0x0014_4a08, 0x0014_4a5c);
}

/// The `sRaceSettings` block copy behind both `BENewRaceInt` entry points:
/// two 0x20 halves (unaligned `ldl`/`ldr` pairs when the addresses are not
/// 8-byte aligned, plain `ld`/`sd` otherwise) and a 12-byte tail.
fn copy_race_settings(c: &mut Cpu, src: u32, dst: u32, head_unaligned: u32, head_aligned: u32) {
    c.cycles += 8;
    c.g[V0][0] = sx(src & 0xffff_0000);
    c.g[V1][0] = sx(src);
    c.g[A0][0] = sx(dst);
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
    if aligned {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head_aligned);
            }
        }
    } else {
        loop {
            c.cycles += 19;
            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(5, c.g[V1][0] as u32);
            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
            c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.sdr(5, c.g[A0][0] as u32);
            c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
            c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head_unaligned);
            }
        }
    }
    c.cycles += 7;
    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[V1][0] as u32);
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[A0][0] as u32);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    abi::ret(c);
}

/// `void UIVector_SetColor(UIVector* v, const Color* c)` (0x003a3398):
/// copy the colour to +0x1c and fill every quad of the array at +0x7c with
/// it, scaled to bytes (count = `+0x74 >> 17`).
pub fn ui_vector_set_color(c: &mut Cpu) {
    c.cycles += 21;
    let v = abi::arg(c, 0);
    let col = abi::arg(c, 1);
    c.g[T0][0] = sx(v);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    // *c -> stack, and *c -> v+0x1c
    c.ldl(2, (col).wrapping_add(0x7));
    c.ldr(2, col);
    c.ldl(3, (col).wrapping_add(0xf));
    c.ldr(3, (col).wrapping_add(0x8));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[SP][0] as u32);
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.ldl(9, (col).wrapping_add(0x7));
    c.ldr(9, col);
    c.ldl(2, (col).wrapping_add(0xf));
    c.ldr(2, (col).wrapping_add(0x8));
    c.sdl(9, (c.g[T0][0] as u32).wrapping_add(0x23));
    c.sdr(9, (c.g[T0][0] as u32).wrapping_add(0x1c));
    c.sdl(2, (c.g[T0][0] as u32).wrapping_add(0x2b));
    c.sdr(2, (c.g[T0][0] as u32).wrapping_add(0x24));
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        abi::ret(c);
        return;
    }
    c.cycles += 34;
    c.g[AT][0] = 0x437f_0000; // 255.0f
    c.mtc1(1, 0);
    c.g[A3][0] = 0;
    c.lwc1(3, (col).wrapping_add(0x4));
    c.lwc1(1, col);
    c.lwc1(2, (col).wrapping_add(0x8));
    c.mul_s(3, 3, 0);
    c.lwc1(4, (col).wrapping_add(0xc));
    c.mul_s(1, 1, 0);
    c.mul_s(2, 2, 0);
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
    c.mul_s(4, 4, 0);
    c.cvt_w_s(0, 3);
    c.mfc1(3, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
    c.cvt_w_s(0, 1);
    c.mfc1(6, 0);
    c.cvt_w_s(0, 2);
    c.mfc1(4, 0);
    c.cvt_w_s(0, 4);
    c.mfc1(5, 0);
    c.mtc1(6, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(3, 1);
    c.cvt_s_w(1, 1);
    c.mtc1(4, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(5, 3);
    c.cvt_s_w(3, 3);
    c.swc1(0, c.g[SP][0] as u32);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
    loop {
        c.cycles += 16;
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 4);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
        c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
        if event_due(c) {
            deliver_event_here(c, 0x003a_3478);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Rails (VU0).

/// `void FloatRail_ApplyImpulse(FloatRail* self, RailNode* node, Vec4* v)`
/// (0x00349798): move the two rail points the node sits between by
/// `v * (1 - t)` and `v * t`.
pub fn float_rail_apply_impulse(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let self_ = abi::arg(c, 0);
    let v = abi::arg(c, 2);
    c.g[A3][0] = sx(node);
    c.g[T0][0] = sx(self_);
    let idx = Addr(p(node, 0x64)).lw(c);
    c.g[A0][0] = idx;
    let negative = (c.g[A0][0] as i64) < 0;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    if negative {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        abi::ret(c);
        return;
    }
    c.cycles += 35;
    c.lwc1(1, p(node, 0x68)); // t
    c.g[A1][0] = 0x50;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 0);
    c.mult(0, 4, 4, 5);
    c.sub_s(0, 0, 1); // 1 - t
    c.mfc1(3, 0);
    c.lqc2(4, v);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, c.g[A0][0] as u32);
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, c.g[A0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x64));
    c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x68));
    c.mult(0, 3, 3, 5);
    c.mfc1(2, 0);
    c.lqc2(4, v);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x50));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x50));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void FloatRail_GetContactVelocity(FloatRail* self, RailNode* node,
/// Vec4* out)` (0x00349840): blend the two rail point velocities by `t` and
/// add the contact's correction at +0x20.
pub fn float_rail_get_contact_velocity(c: &mut Cpu) {
    c.cycles += 5;
    let node = abi::arg(c, 1);
    let self_ = abi::arg(c, 0);
    c.g[A2][0] = sx(node);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x64));
    let negative = (c.g[V1][0] as i64) < 0;
    c.g[A3][0] = sx(self_);
    if negative {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        abi::ret(c);
        return;
    }
    c.cycles += 35;
    c.g[A0][0] = 0x50;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 3, 4);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x68));
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 1);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mfc1(5, 0);
    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 3, 3, 4);
    c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x68));
    c.sub_s(1, 1, 0); // 1 - t
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.mfc1(4, 1);
    c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x60));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[SP][0] as u32);
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.lqc2(4, c.g[SP][0] as u32);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A2][0] as u32).wrapping_add(0x20));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}

/// `void SpringRail_ApplyImpulse(SpringRail* self, RailNode* node, Vec4* dir)`
/// (0x0034a568): clamp the direction to length 200 and push every rail
/// point up to the node with a spring force.
pub fn spring_rail_apply_impulse(c: &mut Cpu) {
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    c.cycles += 17;
    let self_ = abi::arg(c, 0);
    let node = abi::arg(c, 1);
    let dir = abi::arg(c, 2);
    c.g[AT][0] = 0x4348_0000; // 200.0f
    c.mtc1(1, 1);
    c.lqc2(3, dir);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 0); // f0 = |dir|
    c.g[A3][0] = sx(self_);
    c.c_lt_s(1, 0); // 200 < |dir|
    let over = c.fcc();
    if over {
        c.cycles += 10;
        c.div_s(0, 1, 0); // 200 / |dir|
        c.mfc1(3, 0);
        c.lqc2(4, dir);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(2, c.g[SP][0] as u32);
        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    } else {
        c.cycles += 2;
        c.lq(2, dir);
        c.sq(2, c.g[SP][0] as u32);
    }
    // node->idx < 0: nothing to push.
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x64)).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        abi::ret(c);
        return;
    }
    // a = node->pos - elems[0].pos, c = dir x a, alen = |a|
    c.cycles += 30;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.g[T0][0] = 0;
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, c.g[V0][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, c.g[SP][0] as u32);
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(4, 22);
    c.mtc1(4, 2); // f2 = |a|
    if (c.g[V1][0] as i64) <= 0 {
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
    } else {
        c.cycles += 4;
        c.g[T1][0] = c.g[T2][0];
        c.g[A2][0] = 0;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
        loop {
            c.cycles += 69;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(4, 22);
            c.mtc1(4, 0); // f0 = |vel[i]|
            c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 1); // f1 = |a|
            c.div_s(0, 0, 1);
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x10));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0); // f0 = |vel[i]|
            c.div_s(0, 0, 2); // / alen
            c.mfc1(3, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x8)); // k[2]
            c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.mfc1(4, 0);
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
            c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x40));
            c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, (c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
            c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x64));
                break;
            }
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
            if event_due(c) {
                deliver_event_here(c, 0x0034_a670);
            }
        }
    }
    // The node's own element.
    c.cycles += 42;
    c.g[V0][0] = 0x60;
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.mult(0, 4, 4, 2); // idx * 0x60
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.lqc2(3, (c.g[A1][0] as u32));
    c.lqc2(4, c.g[V1][0] as u32);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.div_s(0, 0, 2);
    c.mfc1(3, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x8)); // k[2]
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.mfc1(5, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x70));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A0][0] as u32).wrapping_add(0x40));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    abi::ret(c);
}

/// `void SplineModifier_v22(SplineModifier* self, Body* body)` (0x00359cf8):
/// turn the spline point's tangent and curvature into a velocity and an
/// angular velocity for the body.
pub fn spline_modifier_v22(c: &mut Cpu) {
    c.cycles += 5;
    let self_ = abi::arg(c, 0);
    let body = abi::arg(c, 1);
    c.g[A2][0] = sx(self_);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x54));
    let inactive = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
    if inactive {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x50));
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        abi::ret(c);
        return;
    }
    // b = -d0/(d0+d4) * (c4/dir.x - dir.y*c0/d0)
    // a = (1/sqrt(d0+d4)) * (c8 - dir.z*(c0+c4)/(d0+d4))
    c.cycles += 8;
    c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xb0)); // dir.x
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2a30)); // 1e-5f
    c.abs_s(0, 2);
    c.mtc1(0, 5); // f5 = 0 (small-tangent case)
    c.c_lt_s(1, 0); // 1e-5 < |dir.x|
    let small = !c.fcc();
    c.mov_s(8, 5);
    if small {
        c.cycles += 69;
    } else {
        c.cycles += 24;
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc4));
        c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xc0));
        c.div_s(7, 1, 2); // d4 / dir.x
        c.lwc1(6, (c.g[A2][0] as u32).wrapping_add(0xd0));
        c.add_s(1, 0, 1); // c0 + c4
        c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0xd4));
        c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0xb4));
        c.neg_s(8, 6); // -d0
        c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0xb8));
        c.add_s(3, 6, 3); // d0 + d4
        c.mul_s(4, 4, 0); // dir.y * c0
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 5);
        c.mul_s(2, 2, 1); // dir.z * (c0+c4)
        c.sqrt_s(0, 3);
        c.div_s(5, 5, 0); // 1 / sqrt(sum)
        c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0xc8));
        c.div_s(4, 4, 6); // *c0/d0
        c.div_s(2, 2, 3); // .../sum
        c.div_s(8, 8, 3); // -d0/sum
        c.sub_s(7, 7, 4);
        c.sub_s(1, 1, 2);
        c.mul_s(8, 8, 7);
        c.mul_s(5, 5, 1);
        c.cycles += 69;
    }
    // velocity = normalize(dir) * *(sub+0x38)
    c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x38));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.mfc1(4, 0);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.qmtc2(4, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x48)); // speed
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.mul_s(0, 5, 1); // a * speed -> w.y
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0); // w.x = 0
    c.mul_s(1, 8, 1); // b * speed -> w.z
    c.sq(2, c.g[SP][0] as u32);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
    // w = mat * w
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(4, c.g[V1][0] as u32);
    c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    // d = body->pos - origin; body->vel += vel + d x w; body->rot += w
    c.lqc2(3, c.g[A1][0] as u32);
    c.lqc2(4, (c.g[A2][0] as u32).wrapping_add(0xa0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lqc2(3, c.g[SP][0] as u32);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, Rng, DATA};

    const SEEDS: u32 = 150;

    /// A float whose bits come from a small set, so every branch of a wrap or
    /// clamp function is reached.
    fn pick(r: &mut Rng, values: &[f32]) -> u32 {
        values[(r.next_u32() as usize) % values.len()].to_bits()
    }

    fn word(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    // ----------------------------------------------------------------- front end

    #[test]
    fn coll_hull_get_diameter_matches() {
        check(ssx3_ee::s11::f_0032f8c0__generated, coll_hull_get_diameter, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x32f8c0, 0);
            c.g[A0][0] = sx(DATA);
            if seed % 4 == 0 {
                word(c, DATA, 0);
            } else {
                word(c, DATA, DATA + 0x100);
                word(c, DATA + 0x160, DATA + 0x200);
                word(c, DATA + 0x220, r.edge_f32());
            }
        });
    }

    #[test]
    fn coll_sphere_reset_transform_matches() {
        check(ssx3_ee::s11::f_0032fd98__generated, coll_sphere_reset_transform, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x32fd98, 0);
            c.g[A0][0] = sx(DATA);
            for i in 0..5 {
                word(c, DATA + 0x80 + 4 * i, r.next_u32());
            }
            word(c, DATA + 4, r.next_u32());
            word(c, DATA + 0x60, r.next_u32());
        });
    }

    #[test]
    fn ps2pad_set_actuator_matches() {
        check(ssx3_ee::s10::f_00327740__generated, ps2pad_set_actuator, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x327740, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 { 1 } else { (seed % 5) as u64 };
            c.f[12] = pick(
                &mut r,
                &[0.0, -0.0, 0.244, 0.5, 0.899, 0.9, 1.0, -1.0, 1.0e9, -1.0e9, 1.0e-30],
            );
            for i in 0..0x40 {
                word(c, DATA + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn conveyor_update_matches() {
        check(ssx3_ee::s11::f_003434e8__generated, conveyor_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3434e8, 0);
            c.g[A0][0] = sx(DATA);
            let u = pick(&mut r, &[0.0, 0.5, 0.9, 1.0, -1.0, 1.5, -1.5, 2.0, -0.2]);
            let du = pick(&mut r, &[0.0, 0.1, -0.1, 0.5, 2.0, -2.0, 1.0]);
            let v = pick(&mut r, &[0.0, 0.5, 0.9, 1.0, -1.0, 1.5, -1.5, -0.7]);
            let dv = pick(&mut r, &[0.0, 0.2, -0.2, 3.0, -3.0, 0.05]);
            word(c, DATA + 0x40, u);
            word(c, DATA + 0x38, du);
            word(c, DATA + 0x44, v);
            word(c, DATA + 0x3c, dv);
        });
    }

    #[test]
    fn counter_handle_event_matches() {
        check(ssx3_ee::s11::f_00343868__generated, counter_handle_event, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x343868, 0);
            c.g[A0][0] = sx(DATA);
            c.f[12] = pick(
                &mut r,
                &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 0.5, 11.0, -2.0, 0.0],
            );
            c.g[A1][0] = [0x1f4u64, 0x1f5, 0x1f6, 0x100][(seed % 4) as usize];
            let flags = match seed % 7 {
                0 => 0,
                1 => 0x2 | 0x8,
                2 => 0x1,
                3 => 0x2,
                4 => r.next_u32(),
                5 => 1u32 << (seed % 11),
                _ => 0xffff_ffff,
            };
            word(c, DATA + 0x20, flags);
            let count = match seed % 4 {
                0 => 0,
                1 => 3,
                2 => 0xffff_ffff,
                _ => r.next_u32() & 0xff,
            };
            word(c, DATA + 0x1c, count);
        });
    }

    #[test]
    fn cfe_state_trophy_room_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001d4660__generated, cfe_state_trophy_room_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1d4660, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }

    #[test]
    fn cfe_state_career_stats_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001f5618__generated, cfe_state_career_stats_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1f5618, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }

    #[test]
    fn handle_list_key_family_matches() {
        check(ssx3_ee::s04::f_001d2f40__generated, cfe_state_mountain_room_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1d2f40, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s04::f_001d4f68__generated, cfe_state_rewards_room_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1d4f68, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s04::f_001d5460__generated, mem_card_state_base_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1d5460, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s04::f_001f4a38__generated, cfe_state_buy_attrib_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1f4a38, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s04::f_001f72b8__generated, ov_screen_enter_lodge_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1f72b8, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s05::f_0020d568__generated, ov_screen_yn_dialog_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x20d568, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }

    #[test]
    fn accept_key_6_family_matches() {
        check(ssx3_ee::s04::f_001f7590__generated, ov_screen_bc_start_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1f7590, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s04::f_001f77f0__generated, ov_screen_bc_fail_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1f77f0, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s04::f_001f80c8__generated, ov_screen_top_times_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x1f80c8, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(ssx3_ee::s05::f_0020d308__generated, ov_screen_overlay_menu_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x20d308, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
        check(
            ssx3_ee::s06::f_00246268__generated,
            cfe_state_progressive_verify_ps2_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x246268, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_title_on_widget_notify_matches() {
        check(ssx3_ee::s02::f_00194a48__generated, cfe_state_title_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x194a48, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }

    #[test]
    fn ov_screen_session_on_widget_notify_matches() {
        check(ssx3_ee::s05::f_00208ef8__generated, ov_screen_session_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x208ef8, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }

    #[test]
    fn ov_screen_overlay_menu_on_widget_action_matches() {
        check(ssx3_ee::s05::f_0020d318__generated, ov_screen_overlay_menu_on_widget_action, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x20d318, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x10) };
            c.g[A2][0] = if seed % 2 == 0 { 5 } else { (seed % 8) as u64 };
        });
    }

    #[test]
    fn ov_screen_read_private_message_on_child_event_matches() {
        check(
            ssx3_ee::s05::f_00210dd0__generated,
            ov_screen_read_private_message_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210dd0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { 0x77 } else { r.next_u32() as u64 };
                c.g[A2][0] = if seed % 3 == 0 { 0x16 } else { (seed % 0x20) as u64 };
                word(c, DATA + 0xe0, if seed % 2 == 0 { 0x77 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_compare_items_matches() {
        check(ssx3_ee::s03::f_00198da0__generated, cfe_state_char_equip_compare_items, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x198da0, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, DATA + 0x100);
            word(c, DATA + 4, DATA + 0x200);
            c.bus.write16(DATA + 0x10a, (r.next_u32() & 0xffff) as u16);
            c.bus.write16(DATA + 0x20a, (r.next_u32() & 0xffff) as u16);
        });
    }

    #[test]
    fn rider_model_compare_int_matches() {
        check(ssx3_ee::s10::f_0030db90__generated, rider_model_compare_int, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x30db90, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, r.next_u32());
            word(c, DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn pre_start_screen_try_exit_matches() {
        check(ssx3_ee::s06::f_00232510__generated, pre_start_screen_try_exit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x232510, 0);
            c.g[A0][0] = sx(DATA);
            word(c, DATA + 4, pick(&mut r, &[0.0, -0.0, 0.1, 0.7499, 0.75, 1.0, -1.0]));
        });
    }

    #[test]
    fn load_screen_try_exits_match() {
        check(ssx3_ee::s06::f_002326d8__generated, init_load_screen_try_exit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2326d8, 0);
            c.g[A0][0] = sx(DATA);
            word(c, DATA + 8, 0);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            word(c, crate::difftest::GAME_GP.wrapping_sub(0x848), DATA + 0x300);
            word(c, DATA + 0x300 + 0xa4, DATA + 0x400);
            word(c, DATA + 0x404, pick(&mut r, &[0.0, -0.0, 1.0, 1.0e-30]));
        });
        check(ssx3_ee::s06::f_002327b0__generated, cpre_game_load_screen_try_exit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2327b0, 0);
            c.g[A0][0] = sx(DATA);
            word(c, DATA + 4, 0);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            word(c, crate::difftest::GAME_GP.wrapping_sub(0x848), DATA + 0x300);
            word(c, DATA + 0x300 + 0xa4, DATA + 0x400);
            word(c, DATA + 0x404, pick(&mut r, &[0.0, -0.0, 1.0, 1.0e-30]));
        });
        check(ssx3_ee::s06::f_002327e8__generated, cpre_fe_load_screen_try_exit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2327e8, 0);
            c.g[A0][0] = sx(DATA);
            word(c, DATA + 4, 0);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            word(c, crate::difftest::GAME_GP.wrapping_sub(0x848), DATA + 0x300);
            word(c, DATA + 0x300 + 0xa4, DATA + 0x400);
            word(c, DATA + 0x404, pick(&mut r, &[0.0, -0.0, 1.0, 1.0e-30]));
        });
    }

    #[test]
    fn ps2_exec_man_v10_matches() {
        check(ssx3_ee::s10::f_0031ad00__generated, ps2_exec_man_v10, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x31ad00, 0);
            c.g[A0][0] = sx(DATA);
            word(c, DATA + 0x14048, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn cps2_chunk_resolve_v16_matches() {
        check(ssx3_ee::s13::f_003aa6e8__generated, cps2_chunk_resolve_v16, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa6e8, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            word(c, DATA + 0x120, r.next_u32());
        });
    }

    #[test]
    fn cps2_chunk_resolve_v12_matches() {
        check(ssx3_ee::s13::f_003aa8d8__generated, cps2_chunk_resolve_v12, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa8d8, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[A2][0] = r.next_u32() as u64;
            word(c, DATA + 0x104, r.next_u32());
        });
    }

    #[test]
    fn cps2_chunk_resolve_init_matches() {
        check(ssx3_ee::s13::f_003aae40__generated, cps2_chunk_resolve_init, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aae40, 0);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = r.next_u32() as u64;
            word(c, crate::difftest::GAME_GP.wrapping_add(0x16c8), DATA + 0x400);
            word(c, DATA + 0x400, DATA + 0x500);
            word(c, DATA + 0x500, DATA + 0x600);
            c.bus.write16(DATA + 0x600 + 0x2a, (r.next_u32() & 0xffff) as u16);
            word(c, DATA + 0xc, r.next_u32());
            word(c, DATA + 0x10, r.next_u32());
        });
    }

    // --------------------------------------------------------- game mode tables

    /// The sort keys behind `GameModeMan_Compare*` live in `.bss` tables.
    fn mode_tables(c: &mut Cpu, r: &mut Rng) {
        for i in 0..8 {
            word(c, 0x0053_6640 + 4 * i, r.next_u32());
            word(c, 0x0053_6708 + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn game_mode_man_compares_match() {
        check(ssx3_ee::s06::f_00245e30__generated, game_mode_man_compare_by_score, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x245e30, 0);
            mode_tables(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, (r.next_u32() % 8) as u32);
            word(c, DATA + 4, (r.next_u32() % 8) as u32);
        });
        check(ssx3_ee::s06::f_00245e78__generated, game_mode_man_compare_by_time, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x245e78, 0);
            mode_tables(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, (r.next_u32() % 8) as u32);
            word(c, DATA + 4, (r.next_u32() % 8) as u32);
        });
        check(ssx3_ee::s06::f_00245ec0__generated, game_mode_man_compare_standings, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x245ec0, 0);
            mode_tables(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, (r.next_u32() % 8) as u32);
            word(c, DATA + 4, (r.next_u32() % 8) as u32);
        });
    }

    #[test]
    fn halo_modifier_update_matches() {
        check(ssx3_ee::s11::f_00346258__generated, halo_modifier_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x346258, 0);
            c.g[A0][0] = sx(DATA);
            word(c, DATA + 0x2c, pick(&mut r, &[0.0f32, 1.0, -1.0, 100.0, -100.0, 360.0, -360.0]));
            word(c, DATA + 0x30, pick(&mut r, &[0.0, 0.5, 259.0, 260.0, 359.9, -359.9, -355.0]));
        });
    }

    // ------------------------------------------------------------ chunk resolver

    /// The hash structures the resolver walks: `self -> *(self) -> +8` section
    /// table, then each section's three inner tables (`+0x4`, `+0x14`,
    /// `+0x24`). Entry 0 of every inner table is the "missing" case.
    fn resolver(c: &mut Cpu, self_: u32) {
        let p1 = self_ + 0x100;
        let p2 = self_ + 0x200;
        let sections = self_ + 0x300;
        let sec = self_ + 0x800;
        word(c, self_, p1);
        word(c, p1, p2);
        word(c, p2 + 8, sections);
        for i in 0..256 {
            word(c, sections + 4 * i, sec);
        }
        word(c, sec + 0xc, 0x1234);
        for (off, inner) in [(0x24u32, self_ + 0x900), (0x14, self_ + 0xa00), (0x4, self_ + 0xb00)] {
            word(c, sec + off, inner);
            for i in 0..256u32 {
                let v = if i == 0 { 0 } else { (i << 8) | 0x40 };
                word(c, inner + 4 * i, v);
            }
        }
    }

    #[test]
    fn cps2_chunk_resolve_v17_matches() {
        check(ssx3_ee::s13::f_003aa700__generated, cps2_chunk_resolve_v17, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa700, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            resolver(c, DATA);
            let h = match seed % 4 {
                0 => 0,
                1 => 0x0000_0042,
                2 => 0x0000_1042,
                _ => r.next_u32() & 0xffff,
            };
            word(c, DATA + 0x1064, h);
            word(c, DATA + 0x1084, r.next_u32());
        });
    }

    #[test]
    fn cps2_chunk_resolve_base_v13_matches() {
        check(ssx3_ee::s13::f_003aa8f0__generated, cps2_chunk_resolve_base_v13, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa8f0, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            resolver(c, DATA);
            let h = match seed % 4 {
                0 => 0,
                1 => 0x0000_0042,
                2 => 0x0000_1042,
                _ => r.next_u32() & 0xffff,
            };
            word(c, DATA + 0x1080, h);
            word(c, DATA + 0x108c, r.next_u32());
            word(c, DATA + 0x1094, r.next_u32());
        });
    }

    #[test]
    fn cps2_chunk_resolve_v09_matches() {
        check(ssx3_ee::s13::f_003aa758__generated, cps2_chunk_resolve_v09, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa758, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            word(
                c,
                DATA + 0x110,
                match seed % 3 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    _ => r.next_u32(),
                },
            );
            word(c, DATA + 0x114, r.next_u32());
        });
    }

    #[test]
    fn cps2_chunk_resolve_v10_matches() {
        check(ssx3_ee::s13::f_003aa780__generated, cps2_chunk_resolve_v10, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa780, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64; // 0, 1 or 2 counted arrays
            resolver(c, DATA);
            let data = DATA + 0x1000;
            c.g[A2][0] = sx(data);
            let mut at = data;
            let n = (seed % 3).max(1);
            for _ in 0..n {
                let count = 1 + (r.next_u32() % 3);
                word(c, at, count);
                for j in 0..count {
                    let h = match r.next_u32() % 4 {
                        0 => 0,
                        1 => 0x42,
                        2 => 0x1042,
                        _ => r.next_u32() & 0xffff,
                    };
                    word(c, at + 4 + 4 * j, h);
                }
                at += 4 * (count + 1);
            }
        });
    }

    #[test]
    fn cps2_chunk_resolve_v18_matches() {
        check_lohi(ssx3_ee::s13::f_003aa960__generated, cps2_chunk_resolve_v18, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa960, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            let n = 1 + (seed % 3);
            word(c, DATA + 0x120, n);
            word(c, DATA + 0x124, r.next_u32());
            for i in 0..n {
                let node = DATA + 0x200 + 0x90 * i;
                let prev = match r.next_u32() % 4 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    2 => n - 1,
                    _ => r.next_u32() % n,
                };
                let next = match r.next_u32() % 4 {
                    0 => 0xffff_ffff,
                    1 => 1 % n,
                    _ => r.next_u32() % n,
                };
                word(c, node + 0x60, prev);
                word(c, node + 0x64, next);
                word(c, node + 0x68, r.next_u32());
            }
        });
    }

    #[test]
    fn cps2_chunk_resolve_v21_matches() {
        check(ssx3_ee::s13::f_003aac50__generated, cps2_chunk_resolve_v21, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aac50, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            word(c, crate::difftest::GAME_GP.wrapping_sub(0x23b8), 5000.0f32.to_bits());
            word(c, DATA + 0x10c, if seed % 3 == 0 { 0x100 } else { r.next_u32() });
            word(
                c,
                DATA + 0x110,
                match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => 0xffff_ffff,
                },
            );
            word(c, DATA + 0x11c, pick(&mut r, &[0.0, 4999.0, 5000.0, 5001.0, 1.0e9]));
        });
    }

    #[test]
    fn cps2_chunk_resolve_v22_matches() {
        check(ssx3_ee::s13::f_003aaca8__generated, cps2_chunk_resolve_v22, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3aaca8, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            word(c, DATA + 0x10c, if seed % 3 == 0 { 0x100 } else { r.next_u32() });
            word(
                c,
                DATA + 0x110,
                match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 4,
                    _ => 0xffff_ffff,
                },
            );
        });
    }

    // ------------------------------------------------------------------- rails

    /// A rail: `elems` at +0x70 holding `n` 0x60-byte points, plus a node
    /// (idx at +0x64, blend at +0x68).
    fn rail(c: &mut Cpu, r: &mut Rng, self_: u32, node: u32, elems: u32, n: u32) {
        word(c, self_ + 0x70, elems);
        // the spring stiffness coefficients at +0x60..+0x70
        for k in (0..0x10).step_by(4) {
            word(c, self_ + 0x60 + k, r.edge_f32());
        }
        word(c, node + 0x64, 0xffff_ffff);
        word(c, node + 0x68, r.edge_f32());
        for i in 0..n {
            for k in (0..0x60).step_by(4) {
                word(c, elems + 0x60 * i + k, r.edge_f32());
            }
        }
        // the node's own vectors
        for k in 0..8 {
            word(c, node + 4 * k, r.edge_f32());
        }
        for k in 0..8 {
            word(c, node + 0x20 + 4 * k, r.edge_f32());
        }
        for k in 0..8 {
            word(c, node + 0x40 + 4 * k, r.edge_f32());
        }
        // the trailing k[] coefficient the spring reads at elems + n*0x60 + 8
        word(c, elems + 0x60 * n + 8, r.edge_f32());
    }

    #[test]
    fn float_rail_apply_impulse_matches() {
        check_lohi(ssx3_ee::s11::f_00349798__generated, float_rail_apply_impulse, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x349798, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx(DATA + 0x1800);
            rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 3);
            if seed % 4 != 0 {
                word(c, DATA + 0x1064, (seed % 3) as u32);
            }
            for k in 0..4 {
                word(c, DATA + 0x1800 + 4 * k, r.f32(10.0));
            }
        });
    }

    #[test]
    fn float_rail_get_contact_velocity_matches() {
        check_lohi(ssx3_ee::s11::f_00349840__generated, float_rail_get_contact_velocity, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x349840, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx(DATA + 0x1800);
            rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 3);
            if seed % 4 != 0 {
                word(c, DATA + 0x1064, (seed % 3) as u32);
            }
        });
    }

    #[test]
    fn spring_rail_apply_impulse_matches() {
        check_lohi(ssx3_ee::s11::f_0034a568__generated, spring_rail_apply_impulse, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x34a568, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[A2][0] = sx(DATA + 0x1800);
            rail(c, &mut r, DATA, DATA + 0x1000, DATA + 0x2000, 4);
            let idx = match seed % 5 {
                0 => 0xffff_ffff,
                1 => 0,
                2 => 1,
                3 => 2,
                _ => 3,
            };
            word(c, DATA + 0x1064, idx);
            // directions: tiny, medium and over the 200 length clamp
            let scale: f32 = match seed % 4 {
                0 => 1.0e-6,
                1 => 1.0,
                2 => 100.0,
                _ => 1000.0,
            };
            for k in 0..4 {
                word(c, DATA + 0x1800 + 4 * k, r.f32(scale));
            }
        });
    }

    #[test]
    fn spline_modifier_v22_matches() {
        check_lohi(ssx3_ee::s11::f_00359cf8__generated, spline_modifier_v22, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x359cf8, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            word(c, crate::difftest::GAME_GP.wrapping_sub(0x2a30), 1.0e-5f32.to_bits());
            // the modifier struct: active, stopped and the spline parameters
            for k in 0..0x38 {
                word(c, DATA + 4 * k, r.edge_f32());
            }
            word(c, DATA + 0x54, if seed % 5 == 0 { 0 } else { 1 });
            word(c, DATA + 0x50, if seed % 5 == 1 { 1 } else { 0 });
            word(
                c,
                DATA + 0xb0,
                match seed % 4 {
                    0 => 0.0f32.to_bits(),
                    1 => 1.0e-9f32.to_bits(),
                    2 => 1.0f32.to_bits(),
                    _ => r.edge_f32(),
                },
            );
            for off in [0xb4u32, 0xb8, 0xc0, 0xc4, 0xc8, 0xd0, 0xd4] {
                word(c, DATA + off, pick(&mut r, &[0.0, 1.0, -1.0, 0.5, 2.0, -0.25]));
            }
            word(c, DATA + 0x48, pick(&mut r, &[0.0, 1.0, -2.5]));
            word(c, DATA + 0x38, pick(&mut r, &[0.0, 1.0, -1.0, 3.5]));
            // the body
            for k in 0..0x40 {
                word(c, DATA + 0x1000 + 4 * k, r.f32(10.0));
            }
        });
    }

    #[test]
    fn ui_vector_set_color_matches() {
        check(ssx3_ee::s13::f_003a3398__generated, ui_vector_set_color, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a3398, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            for k in 0..4 {
                word(c, DATA + 0x100 + 4 * k, r.edge_f32());
            }
            let count = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                _ => 1 + (r.next_u32() % 4),
            };
            word(c, DATA + 0x74, count << 17);
            word(c, DATA + 0x7c, if seed % 7 == 0 { 0 } else { DATA + 0x200 });
            for k in 0..16 {
                word(c, DATA + 0x200 + 4 * k, r.next_u32());
            }
            word(c, DATA + 0x1c, r.next_u32());
        });
    }

    // ------------------------------------------------------------- race settings

    #[test]
    fn be_new_race_int_copies_match() {
        check(ssx3_ee::s01::f_00144928__generated, be_new_race_int_commit, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x144928, 0);
            for k in 0..0x14 {
                word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
            }
        });
        check(ssx3_ee::s01::f_001449e8__generated, be_new_race_int_reset, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1449e8, 0);
            for k in 0..0x14 {
                word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
            }
        });
    }

    #[test]
    fn be_new_race_int_copies_deliver_events() {
        use crate::difftest::events::schedule;
        check(ssx3_ee::s01::f_00144928__generated, be_new_race_int_commit, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x244928, 0);
            for k in 0..0x14 {
                word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
            }
            schedule(c, &mut r, seed, 0x0014_4928);
        });
        check(ssx3_ee::s01::f_001449e8__generated, be_new_race_int_reset, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2449e8, 0);
            for k in 0..0x14 {
                word(c, 0x0053_05b0 + 4 * k, r.next_u32());
                word(c, 0x0053_5bc8 + 4 * k, r.next_u32());
            }
            schedule(c, &mut r, seed, 0x0014_49e8);
        });
    }

    #[test]
    fn cps2_chunk_resolve_v10_delivers_events() {
        use crate::difftest::events::schedule;
        check(ssx3_ee::s13::f_003aa780__generated, cps2_chunk_resolve_v10, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa780, 0);
            resolver(c, DATA);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = 1;
            c.g[A2][0] = sx(DATA + 0x1000);
            let count = 6;
            word(c, DATA + 0x1000, count);
            for j in 0..count {
                word(c, DATA + 0x1004 + 4 * j, r.next_u32() & 0xffff);
            }
            schedule(c, &mut r, seed, 0x003a_a780);
        });
    }

    #[test]
    fn ui_vector_set_color_delivers_events() {
        use crate::difftest::events::schedule;
        check(ssx3_ee::s13::f_003a3398__generated, ui_vector_set_color, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3a3398, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            for k in 0..4 {
                word(c, DATA + 0x100 + 4 * k, r.edge_f32());
            }
            word(c, DATA + 0x74, 6 << 17);
            word(c, DATA + 0x7c, DATA + 0x200);
            schedule(c, &mut r, seed, 0x003a_3398);
        });
    }

    #[test]
    fn cps2_chunk_resolve_v18_delivers_events() {
        use crate::difftest::events::schedule;
        check_lohi(ssx3_ee::s13::f_003aa960__generated, cps2_chunk_resolve_v18, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3aa960, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            word(c, DATA + 0x120, 6);
            for i in 0..6 {
                let node = DATA + 0x200 + 0x90 * i;
                word(c, node + 0x60, if i == 0 { 0xffff_ffff } else { i - 1 });
                word(c, node + 0x64, if i == 5 { 0xffff_ffff } else { i + 1 });
            }
            schedule(c, &mut r, seed, 0x003a_a960);
        });
    }
}
