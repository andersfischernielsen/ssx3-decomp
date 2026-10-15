//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body.

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_1c58, world_get_current_view_index, "World_GetCurrentViewIndex"),
    (0x0038_9558, vec3_add_to, "Vec3_AddTo"),
    (0x0010_1a10, danger_man_get_rider_danger, "DangerMan_GetRiderDanger"),
    (0x0010_d9e8, computer_actor_rider_is_rival_target, "ComputerActorRider_IsRivalTarget"),
    (0x0011_2588, rider_get_ai_path_look_ahead_dist, "Rider_GetAIPathLookAheadDist"),
    (0x0011_25a8, rider_save_ai_path_look_ahead, "Rider_SaveAIPathLookAhead"),
    (0x0012_f588, rider_reset_sub_is_local_player, "RiderResetSub_IsLocalPlayer"),
    (0x0013_04d0, rider_board_press_sub_update_phase2, "RiderBoardPressSub_UpdatePhase2"),
    (0x0014_4c78, be_new_race_int_get_track_peak_by_index, "BENewRaceInt_GetTrackPeakByIndex"),
    (0x0016_d238, camera_trigger_man_push_trigger_stack, "CameraTriggerMan_PushTriggerStack"),
];

// ---------------------------------------------------------------------------
// World.

/// `int World_GetCurrentViewIndex(void)` (0x2d1c58): follows three pointers out
/// of the world singleton (at `gp - 0x848`, then +0x84, +0x84) and returns the
/// view index at +0x14.
pub fn world_get_current_view_index(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let views = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = views;
    let view = Addr(views as u32).off(0x84).lw(c);
    c.g[A0][0] = view;
    c.g[V0][0] = Addr(view as u32).off(0x14).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Vector math.

/// `void Vec3_AddTo(Vec3 *dst, const Vec3 *src)` (0x389558): `dst += src`,
/// component by component, in the original's load/store order (so overlapping
/// vectors behave identically).
pub fn vec3_add_to(c: &mut Cpu) {
    let dst = Addr(abi::arg(c, 0));
    let src = Addr(abi::arg(c, 1));
    c.cycles += 13;
    let s0 = src.read_fpu(c);
    let d0 = dst.read_fpu(c);
    let d1 = dst.off(0x4).read_fpu(c);
    let r0 = d0 + s0;
    let d2 = dst.off(0x8).read_fpu(c);
    dst.write_fpu(c, r0);
    let s1 = src.off(0x4).read_fpu(c);
    let r1 = d1 + s1;
    dst.off(0x4).write_fpu(c, r1);
    let s2 = src.off(0x8).read_fpu(c);
    let r2 = d2 + s2;
    dst.off(0x8).write_fpu(c, r2);
    // Scratch registers the original leaves behind (f0/f1 are compared).
    abi::set_fpr(c, 0, s2);
    abi::set_fpr(c, 1, s1);
    abi::set_fpr(c, 2, r1);
    abi::set_fpr(c, 3, r2);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Danger manager.

/// `int DangerMan_GetRiderDanger(DangerMan *man, int rider)` (0x101a10):
/// `man->riders[rider].danger` at `man + rider*12 + 0x710`.
pub fn danger_man_get_rider_danger(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    let scaled = abi::mult(c, abi::arg(c, 1), 0xc);
    c.g[V1][0] = sx(scaled);
    let slot = abi::arg(c, 0).wrapping_add(scaled);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(0x710).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Riders.

/// `bool ComputerActorRider_IsRivalTarget(ComputerActor *self, uint id)`
/// (0x10d9e8): false unless the actor has a rider (+0x18 -> +0xf0) and its id
/// (+0xf8) equals `id`.
pub fn computer_actor_rider_is_rival_target(c: &mut Cpu) {
    c.cycles += 4;
    let actor = Addr(abi::arg(c, 0));
    let rider = actor.off(0x18).lw(c);
    c.g[V1][0] = rider;
    let has_rider = Addr(rider as u32).off(0xf0).lw(c);
    let mut result = 0u64;
    if has_rider != 0 {
        c.cycles += 3;
        let diff = Addr(rider as u32).off(0xf8).lw(c) ^ c.g[A1][0];
        result = (diff == 0) as u64;
    }
    c.cycles += 2;
    c.g[A0][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `float Rider_GetAIPathLookAheadDist(Rider *rider, int flag)` (0x112588):
/// 796.0 when `flag` is set, 200.0 otherwise.
pub fn rider_get_ai_path_look_ahead_dist(c: &mut Cpu) {
    c.cycles += 4;
    let bits = if abi::arg64(c, 1) != 0 {
        0x4447_0000u32
    } else {
        c.cycles += 2;
        0x4348_0000u32
    };
    c.g[AT][0] = bits as u64;
    c.f[0] = bits; // mtc1 f0, at
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_SaveAIPathLookAhead(Rider *rider)` (0x1125a8): copies the
/// 16-byte look-ahead quad at +0x4a0 to +0x4b0.
pub fn rider_save_ai_path_look_ahead(c: &mut Cpu) {
    let rider = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let v = rider.off(0x4a0).read_vec4(c);
    abi::set_gpr128(c, V0, v.to_bits());
    rider.off(0x4b0).write_vec4(c, v);
    abi::ret(c);
}

/// `bool RiderResetSub_IsLocalPlayer(RiderResetSub *self)` (0x12f588): false
/// unless the object (+0x8 -> +0x87c) is present, else whether the player id at
/// +0x870 is non-negative.
pub fn rider_reset_sub_is_local_player(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let handle = obj.off(0x8).lw(c);
    c.g[V1][0] = handle;
    let present = Addr(handle as u32).off(0x87c).lw(c);
    let mut result = 0u64;
    if present != 0 {
        c.cycles += 3;
        let id = Addr(handle as u32).off(0x870).lw(c);
        result = ((id as i64) >= 0) as u64;
    }
    c.cycles += 2;
    c.g[A0][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `void RiderBoardPressSub_UpdatePhase2(RiderBoardPressSub *self)`
/// (0x1304d0): set the phase field at +0 to 1.
pub fn rider_board_press_sub_update_phase2(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    this.sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// New race.

/// `int BENewRaceInt_GetTrackPeakByIndex(BENewRaceInt *self, int index)`
/// (0x144c78): `peaks[index].peak`, the records being 100 bytes each starting
/// at 0x43d950.
pub fn be_new_race_int_get_track_peak_by_index(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x64;
    let scaled = abi::mult(c, abi::arg(c, 1), 0x64);
    c.g[A1][0] = sx(scaled);
    let base = sx(0x44_0000u32.wrapping_sub(0x26b0).wrapping_add(scaled));
    c.g[V1][0] = base;
    c.g[V0][0] = Addr(base as u32).off(0x54).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera triggers.

/// `void CameraTriggerMan_PushTriggerStack(CameraTriggerMan *man, uint value)`
/// (0x16d238): write `value` into `man->stack[man->count]` and at +0x4, then
/// bump the count.
pub fn camera_trigger_man_push_trigger_stack(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let value = c.g[A1][0];
    c.cycles += 9;
    let count = man.lw(c) as u32;
    let slot = man.0.wrapping_add(count << 2);
    c.g[V0][0] = sx(slot);
    Addr(slot).off(0x8).sw(c, value);
    man.off(0x4).sw(c, value);
    let count = man.lw(c) as u32;
    c.g[V0][0] = sx(count.wrapping_add(1));
    man.sw(c, c.g[V0][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn world_get_current_view_index_matches() {
        check(ssx3_ee::s09::f_002d1c58__generated, world_get_current_view_index, 300, &|c, seed| {
            let mut r = Rng(0x1234_5678 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let base = GAME_GP.wrapping_sub(0x848);
            let p1 = DATA + 0x400;
            let p2 = DATA + 0x500;
            let p3 = DATA + 0x600;
            c.bus.write32(base, p1);
            c.bus.write32(p1 + 0x84, p2);
            c.bus.write32(p2 + 0x84, p3);
            c.bus.write32(p3 + 0x14, r.next_u32());
        });
    }

    #[test]
    fn vec3_add_to_matches() {
        check(ssx3_ee::s12::f_00389558__generated, vec3_add_to, 300, &|c, seed| {
            let mut r = Rng(0x9e37_79b9 ^ seed as u64);
            scramble(c, &mut r);
            let dst = DATA;
            // Cover distinct, exactly-alias and partially-overlapping vectors.
            let src = match seed % 3 {
                0 => DATA,
                1 => DATA + 4,
                _ => DATA + 0x100,
            };
            c.g[A0][0] = dst as u64;
            c.g[A1][0] = src as u64;
            for i in 0..3 {
                c.bus.write32(dst + i * 4, r.edge_f32());
            }
            for i in 0..3 {
                c.bus.write32(src + i * 4, r.edge_f32());
            }
        });
    }

    #[test]
    fn danger_man_get_rider_danger_matches() {
        check(ssx3_ee::s00::f_00101a10__generated, danger_man_get_rider_danger, 200, &|c, seed| {
            let mut r = Rng(0x2718_2818 ^ seed as u64);
            scramble(c, &mut r);
            let rider = seed % 5;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = rider as u64;
            c.bus.write32(DATA + rider * 12 + 0x710, r.next_u32());
        });
    }

    #[test]
    fn computer_actor_rider_is_rival_target_matches() {
        check(ssx3_ee::s00::f_0010d9e8__generated, computer_actor_rider_is_rival_target, 300, &|c, seed| {
            let mut r = Rng(0x0f0f_0f0f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let rider = DATA + 0x100;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x18, rider);
            c.bus.write32(rider + 0xf0, if seed % 3 == 0 { 0 } else { r.next_u32() });
            let id = if seed % 4 == 0 {
                // Force the id test to succeed: lw sign-extends into 64 bits, so
                // keep a1's upper half zero and the value non-negative.
                let x = r.next_u32() & 0x7fff_ffff;
                c.g[A1][0] = x as u64;
                x
            } else {
                r.next_u32()
            };
            c.bus.write32(rider + 0xf8, id);
        });
    }

    #[test]
    fn rider_get_ai_path_look_ahead_dist_matches() {
        check(ssx3_ee::s00::f_00112588__generated, rider_get_ai_path_look_ahead_dist, 200, &|c, seed| {
            let mut r = Rng(0x5151_5151 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = if seed % 2 == 0 {
                0
            } else {
                ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1)
            };
        });
    }

    #[test]
    fn rider_save_ai_path_look_ahead_matches() {
        check(ssx3_ee::s00::f_001125a8__generated, rider_save_ai_path_look_ahead, 200, &|c, seed| {
            let mut r = Rng(0x6161_6161 ^ seed as u64);
            scramble(c, &mut r);
            // Odd seeds use a misaligned base to cover `lq`/`sq` masking.
            let base = DATA + (seed % 2) * 8;
            c.g[A0][0] = base as u64;
            for i in 0..8 {
                c.bus.write32(base + 0x4a0 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn rider_reset_sub_is_local_player_matches() {
        check(ssx3_ee::s00::f_0012f588__generated, rider_reset_sub_is_local_player, 300, &|c, seed| {
            let mut r = Rng(0x7171_7171 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let handle = DATA + 0x100;
            c.bus.write32(DATA + 0x8, handle);
            c.bus.write32(handle + 0x87c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(
                handle + 0x870,
                if seed % 5 == 0 { 0x8000_0000 } else { r.next_u32() & 0x7fff_ffff },
            );
        });
    }

    #[test]
    fn rider_board_press_sub_update_phase2_matches() {
        check(ssx3_ee::s00::f_001304d0__generated, rider_board_press_sub_update_phase2, 50, &|c, seed| {
            let mut r = Rng(0x8181_8181 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn be_new_race_int_get_track_peak_by_index_matches() {
        check(ssx3_ee::s01::f_00144c78__generated, be_new_race_int_get_track_peak_by_index, 100, &|c, seed| {
            let mut r = Rng(0x9191_9191 ^ seed as u64);
            scramble(c, &mut r);
            let index = seed % 8;
            c.g[A1][0] = index as u64;
            let base = 0x43d950u32.wrapping_add(index * 0x64);
            c.bus.write32(base + 0x54, r.next_u32());
        });
    }

    #[test]
    fn camera_trigger_man_push_trigger_stack_matches() {
        check(ssx3_ee::s02::f_0016d238__generated, camera_trigger_man_push_trigger_stack, 200, &|c, seed| {
            let mut r = Rng(0xa1a1_a1a1 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let n0 = r.next_u32() % 8;
            // Odd seeds set high bits that vanish in the 32-bit `<< 2`.
            let count = if seed % 2 == 0 { n0 } else { 0x4000_0000 | n0 };
            c.bus.write32(DATA, count);
        });
    }
}
