//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Work package OC4: a handful of tiny leaf getters/setters — world singleton
//! lookups, a linked-list front push/pop, two animation-track setters, the
//! PS/2 exec-manager constructor and the loading-progress-bar helpers.
//!
//! Each replacement reproduces the generated code's memory writes, the
//! registers it leaves observable (`v0`, `v1`, `a0`, `at`, FPU `f0`, and the
//! `LO`/`HI` of its multiplies) and its EE cycle cost exactly; a differential
//! test proves it.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_1c98, world_get_ai_frame_count, "World_GetAIFrameCount"),
    (0x002d_1cb0, world_get_flag_manager, "World_GetFlagManager"),
    (0x002e_dfb8, world_painter_get_view_irradiance, "WorldPainter_GetViewIrradiance"),
    (0x0030_b208, wscript_process_list_pop_front, "WScriptProcessList_PopFront"),
    (0x0030_b260, wscript_process_list_push_front, "WScriptProcessList_PushFront"),
    (0x0031_3d28, anim_sequence_set_track_weight, "AnimSequence_SetTrackWeight"),
    (0x0031_3d40, anim_sequence_set_track_loop, "AnimSequence_SetTrackLoop"),
    (0x0031_7458, ps2_exec_man_ctor, "PS2ExecMan_ctor"),
    (0x0031_7530, app_load_progress_init, "App_LoadProgressInit"),
    (0x0031_7600, app_load_progress_complete, "App_LoadProgressComplete"),
];

/// `gp-0x848`: the `World` singleton pointer (global data filled in at
/// startup).
const WORLD_GLOBAL: u32 = 0x848;

/// `World+0x84`: the World's AI manager.
const WORLD_AI_MANAGER: u32 = 0x84;
/// `World+0x84+0x70`: the World's flag manager.
const FLAG_MANAGER: u32 = 0x70;
/// `AIManager+0xc`: the AI currently being processed.
const AI_MANAGER_CURRENT: u32 = 0xc;
/// `AI+0x8`: the AI's frame counter.
const AI_FRAME_COUNT: u32 = 0x8;

// ---------------------------------------------------------------- world

/// `int World_GetAIFrameCount(void)` (0x2d1c98): the current AI's frame
/// counter. `World` at `gp-0x848`, its AI manager at `+0x84`, the current AI
/// at `+0xc`, and the count at `+0x8`.
pub fn world_get_ai_frame_count(c: &mut Cpu) {
    c.cycles += 5;
    let gp = Addr(c.g[GP][0] as u32);
    let world = gp.off(0u32.wrapping_sub(WORLD_GLOBAL)).lw(c);
    c.g[V0][0] = world;
    let ai_manager = Addr(world as u32).off(WORLD_AI_MANAGER).lw(c);
    c.g[V1][0] = ai_manager;
    let ai = Addr(ai_manager as u32).off(AI_MANAGER_CURRENT).lw(c);
    c.g[A0][0] = ai;
    c.g[V0][0] = Addr(ai as u32).off(AI_FRAME_COUNT).lw(c);
    abi::ret(c);
}

/// `FlagManager *World_GetFlagManager(void)` (0x2d1cb0): the World's flag
/// manager (`World+0x84` → `+0x70`).
pub fn world_get_flag_manager(c: &mut Cpu) {
    c.cycles += 4;
    let gp = Addr(c.g[GP][0] as u32);
    let world = gp.off(0u32.wrapping_sub(WORLD_GLOBAL)).lw(c);
    c.g[V0][0] = world;
    let manager = Addr(world as u32).off(WORLD_AI_MANAGER).lw(c);
    c.g[V1][0] = manager;
    c.g[V0][0] = Addr(manager as u32).off(FLAG_MANAGER).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------- world painter

/// `ViewIrradiance *WorldPainter_GetViewIrradiance(int view)` (0x2edfb8):
/// `&TABLE[view]`, entries of 0xf0 bytes starting at 0x4fa3c0. The index
/// multiply also sets `LO`/`HI`.
pub fn world_painter_get_view_irradiance(c: &mut Cpu) {
    c.cycles += 6;
    const VIEW_SIZE: u32 = 0xf0;
    const TABLE: u32 = 0x0050_0000 - 0x5c40; // 0x004f_a3c0
    c.g[V1][0] = VIEW_SIZE as u64;
    c.g[V0][0] = 0x0050_0000;
    let offset = abi::mult(c, abi::arg(c, 0), VIEW_SIZE);
    c.g[A0][0] = sx(offset);
    c.g[V0][0] = sx(TABLE);
    c.g[V0][0] = sx(TABLE.wrapping_add(offset));
    abi::ret(c);
}

// ---------------------------------------------------------------- wscript list

/// `WScriptProcessNode *WScriptProcessList_PopFront(WScriptProcessList *l)`
/// (0x30b208): unlink and return the head node (its `+0x18` is the next
/// pointer), or `NULL` when the list is empty.
pub fn wscript_process_list_pop_front(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let head = list.lw(c);
    c.g[V1][0] = head;
    if head != 0 {
        c.cycles += 2;
        let next = Addr(head as u32).off(0x18).lw(c);
        c.g[V0][0] = next;
        list.sw(c, next);
    }
    c.cycles += 2;
    c.g[V0][0] = head;
    abi::ret(c);
}

/// `void WScriptProcessList_PushFront(WScriptProcessList *l,
/// WScriptProcessNode *n)` (0x30b260): link `n` before the head (in
/// `n+0x18`); returns the previous head in `v0`.
pub fn wscript_process_list_push_front(c: &mut Cpu) {
    c.cycles += 4;
    let list = Addr(abi::arg(c, 0));
    let node = Addr(abi::arg(c, 1));
    let head = list.lw(c);
    c.g[V0][0] = head;
    node.off(0x18).sw(c, head);
    list.sw(c, c.g[A1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------- anim sequence

/// Size of one animation track (`AnimTrack`).
const TRACK_SIZE: u32 = 0x1c;
/// `AnimTrack+0x14`: the track's blend weight.
const TRACK_WEIGHT: u32 = 0x14;
/// `AnimTrack+0x1c`: the track's loop flag.
const TRACK_LOOP: u32 = 0x1c;

/// `void AnimSequence_SetTrackWeight(AnimSequence *seq, int track, float w)`
/// (0x313d28): set track `track`'s blend weight (`seq + track*0x1c + 0x14`)
/// to `w`. The index multiply also sets `LO`/`HI`.
pub fn anim_sequence_set_track_weight(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = TRACK_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 1), TRACK_SIZE);
    c.g[V1][0] = sx(offset);
    let seq = abi::arg(c, 0);
    let track = Addr(seq.wrapping_add(offset));
    c.g[A0][0] = sx(track.0);
    track.off(TRACK_WEIGHT).write_fpu(c, abi::farg(c, 0));
    abi::ret(c);
}

/// `void AnimSequence_SetTrackLoop(AnimSequence *seq, int track, uint loop)`
/// (0x313d40): store the loop flag at `seq + track*0x1c + 0x1c`. The index
/// multiply also sets `LO`/`HI`.
pub fn anim_sequence_set_track_loop(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = TRACK_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 1), TRACK_SIZE);
    c.g[V1][0] = sx(offset);
    let seq = abi::arg(c, 0);
    let track = Addr(seq.wrapping_add(offset));
    c.g[A0][0] = sx(track.0);
    track.off(TRACK_LOOP).sw(c, c.g[A2][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------- app / exec man

/// `PS2ExecMan *PS2ExecMan_ctor(PS2ExecMan *m)` (0x317458): install the
/// vtable at 0x48dc30 and clear the callback and its argument; returns `m`.
pub fn ps2_exec_man_ctor(c: &mut Cpu) {
    c.cycles += 7;
    const VTABLE: u32 = 0x0049_0000 - 0x23d0; // 0x0048_dc30
    c.g[V1][0] = 0x0049_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(VTABLE);
    let this = Addr(abi::arg(c, 0));
    this.sw(c, 0);
    this.off(8).sw(c, c.g[V1][0]);
    this.off(4).sw(c, 0);
    abi::ret(c);
}

/// `void App_LoadProgressInit(LoadProgress *p, float step)` (0x317530):
/// reset the progress bar: store `step` at `p+0xc` and clear the words at
/// `+0`, `+8`, `+0x14`, `+0x18` and `+0x1c`.
pub fn app_load_progress_init(c: &mut Cpu) {
    c.cycles += 7;
    let p = Addr(abi::arg(c, 0));
    p.off(0xc).write_fpu(c, abi::farg(c, 0));
    p.off(0x1c).sw(c, 0);
    p.sw(c, 0);
    p.off(8).sw(c, 0);
    p.off(0x14).sw(c, 0);
    p.off(0x18).sw(c, 0);
    abi::ret(c);
}

/// `void App_LoadProgressComplete(LoadProgress *p)` (0x317600): snap the bar
/// to 100% — f0 = 100.0, the value (`+0`) and displayed value (`+0x14`) are
/// both set to it and the fade timer at `+0x18` cleared.
pub fn app_load_progress_complete(c: &mut Cpu) {
    c.cycles += 6;
    const HUNDRED: FpuF32 = FpuF32(0x42c8_0000);
    c.g[AT][0] = HUNDRED.0 as u64;
    c.f[0] = HUNDRED.0;
    let p = Addr(abi::arg(c, 0));
    p.off(0x18).sw(c, 0);
    p.write_fpu(c, HUNDRED);
    p.off(0x14).write_fpu(c, HUNDRED);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee::{s09, s10};

    const SEEDS: u32 = 400;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt)
    }

    /// A pointer into test RAM (16-byte aligned).
    fn ptr(r: &mut Rng, region: u32) -> u32 {
        DATA + region * 0x10_0000 + (r.next_u32() & 0x7fff0)
    }

    #[test]
    fn world_get_ai_frame_count_matches() {
        check(s09::f_002d1c98__generated, world_get_ai_frame_count, SEEDS, &|c, seed| {
            let mut r = rng(seed, 1);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = ptr(&mut r, 0);
            let manager = ptr(&mut r, 1);
            let ai = ptr(&mut r, 2);
            c.bus.write32(GAME_GP.wrapping_sub(WORLD_GLOBAL), world);
            c.bus.write32(world + WORLD_AI_MANAGER, manager);
            c.bus.write32(manager + AI_MANAGER_CURRENT, ai);
            c.bus.write32(ai + AI_FRAME_COUNT, r.next_u32());
        });
    }

    #[test]
    fn world_get_flag_manager_matches() {
        check(s09::f_002d1cb0__generated, world_get_flag_manager, SEEDS, &|c, seed| {
            let mut r = rng(seed, 2);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = ptr(&mut r, 0);
            let manager = ptr(&mut r, 1);
            c.bus.write32(GAME_GP.wrapping_sub(WORLD_GLOBAL), world);
            c.bus.write32(world + WORLD_AI_MANAGER, manager);
            c.bus.write32(manager + FLAG_MANAGER, r.next_u32());
        });
    }

    #[test]
    fn world_painter_get_view_irradiance_matches() {
        check(s09::f_002edfb8__generated, world_painter_get_view_irradiance, SEEDS, &|c, seed| {
            let mut r = rng(seed, 3);
            scramble(c, &mut r);
            c.g[A0][0] = (match seed % 4 {
                0 => 0,
                1 => 1,
                2 => u32::MAX,
                _ => r.next_u32(),
            }) as u64;
        });
    }

    #[test]
    fn wscript_process_list_pop_front_matches() {
        check(s10::f_0030b208__generated, wscript_process_list_pop_front, SEEDS, &|c, seed| {
            let mut r = rng(seed, 4);
            scramble(c, &mut r);
            let list = ptr(&mut r, 0);
            let node = ptr(&mut r, 1);
            c.g[A0][0] = sx(list);
            let head = if seed % 2 == 0 { 0 } else { node };
            c.bus.write32(list, head);
            c.bus.write32(node + 0x18, r.next_u32());
        });
    }

    #[test]
    fn wscript_process_list_push_front_matches() {
        check(s10::f_0030b260__generated, wscript_process_list_push_front, SEEDS, &|c, seed| {
            let mut r = rng(seed, 5);
            scramble(c, &mut r);
            let list = ptr(&mut r, 0);
            let node = ptr(&mut r, 1);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(node);
            c.bus.write32(list, if seed % 2 == 0 { 0 } else { r.next_u32() });
        });
    }

    #[test]
    fn anim_sequence_set_track_weight_matches() {
        check(s10::f_00313d28__generated, anim_sequence_set_track_weight, SEEDS, &|c, seed| {
            let mut r = rng(seed, 6);
            scramble(c, &mut r);
            let seq = ptr(&mut r, 0);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = (r.next_u32() % 6) as u64;
            c.f[12] = r.edge_f32();
        });
    }

    #[test]
    fn anim_sequence_set_track_loop_matches() {
        check(s10::f_00313d40__generated, anim_sequence_set_track_loop, SEEDS, &|c, seed| {
            let mut r = rng(seed, 7);
            scramble(c, &mut r);
            let seq = ptr(&mut r, 0);
            c.g[A0][0] = sx(seq);
            c.g[A1][0] = (r.next_u32() % 6) as u64;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(r.next_u32()) };
        });
    }

    #[test]
    fn ps2_exec_man_ctor_matches() {
        check(s10::f_00317458__generated, ps2_exec_man_ctor, SEEDS, &|c, seed| {
            let mut r = rng(seed, 8);
            scramble(c, &mut r);
            let this = ptr(&mut r, 0);
            c.g[A0][0] = sx(this);
        });
    }

    #[test]
    fn app_load_progress_init_matches() {
        check(s10::f_00317530__generated, app_load_progress_init, SEEDS, &|c, seed| {
            let mut r = rng(seed, 9);
            scramble(c, &mut r);
            let p = ptr(&mut r, 0);
            c.g[A0][0] = sx(p);
            c.f[12] = r.edge_f32();
            for off in (0..0x20).step_by(4) {
                c.bus.write32(p + off, r.next_u32());
            }
        });
    }

    #[test]
    fn app_load_progress_complete_matches() {
        check(s10::f_00317600__generated, app_load_progress_complete, SEEDS, &|c, seed| {
            let mut r = rng(seed, 10);
            scramble(c, &mut r);
            let p = ptr(&mut r, 0);
            c.g[A0][0] = sx(p);
            for off in (0..0x20).step_by(4) {
                c.bus.write32(p + off, r.next_u32());
            }
        });
    }
}
