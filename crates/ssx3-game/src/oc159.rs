//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for rider difficulty, MIDI/audio queue pushes, a camera shake
//! and front-end popup placement, each with a differential test against the
//! generated body.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_c4f8, computer_actor_rider_apply_difficulty, "ComputerActorRider_ApplyDifficulty"),
    (0x0035_cff0, roller_modifier_set_angular_momentum, "RollerModifier_SetAngularMomentum"),
    (0x002a_8450, midi_banks_push_play_params, "MIDIBanks_PushPlayParams"),
    (0x0029_0cc0, ssx_audio_system_play_sound, "SSXAudioSystem_PlaySound"),
    (0x0016_56b0, chase_camera_start_shake, "ChaseCameraAlgorithm_StartShake"),
    (0x001c_78c0, fe_popup_confirm_position_background, "cFEPopupConfirm_PositionBackground"),
    (0x0029_9b70, ssx_audio_system_play_rail_change_sound, "SSXAudioSystem_PlayRailChangeSound"),
    (0x0029_d370, ssx_audio_system_start_loop_sfx, "SSXAudioSystem_StartLoopSfx"),
];

/// A `gp`-relative address (`lui/lw` offset form).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `lwc1 f, gp+off`, returning the bits.
#[inline(always)]
fn gpf(c: &mut Cpu, f: usize, off: i32) -> u32 {
    c.lwc1(f, gp(c, off));
    c.f[f]
}

// ---------------------------------------------------------------------------
// Rider difficulty.

/// Jump tables of `setIndividualRiderDifficulty` (`.rodata`).
const DIFFICULTY_TABLE_A: u32 = 0x0045_6a90;
const DIFFICULTY_TABLE_B: u32 = 0x0045_6ab0;
const DIFFICULTY_TABLE_C: u32 = 0x0045_6ad0;

/// One full table-A case (levels 2..5): skills A/B (`self+0xdf8`/`+0xdfc`)
/// and the tune record's acceleration (`+0xdc`), speed (`+0xe0`) and boost
/// (`+0xe4`).  `cycles` is the whole case (the original splits it in two
/// blocks; there is no event check between them).
fn difficulty_case_full(
    c: &mut Cpu,
    obj: u32,
    cycles: u64,
    skill_a: u32,
    skill_b: u32,
    accel: u32,
    speed: u32,
    boost: u64,
) {
    c.cycles += cycles;
    c.f[0] = skill_b;
    c.f[2] = skill_a;
    let tune = Addr(obj).off(0x18).lw(c);
    c.f[1] = accel;
    c.g[V0][0] = tune;
    Addr(obj).off(0xdfc).write_fpu(c, FpuF32(skill_b));
    Addr(obj).off(0xdf8).write_fpu(c, FpuF32(skill_a));
    c.f[0] = speed;
    Addr(tune as u32).off(0xdc).write_fpu(c, FpuF32(accel));
    c.g[V0][0] = tune;
    Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
    c.g[V1][0] = tune;
    Addr(tune as u32).off(0xe4).sw(c, boost);
}

/// One skill/speed case (modes 0 and 2): skills A/B and the tune speed.
fn difficulty_case_skill_speed(
    c: &mut Cpu,
    obj: u32,
    cycles: u64,
    skill_a: u32,
    skill_b: u32,
    speed: u32,
) {
    c.cycles += cycles;
    c.f[1] = skill_a;
    c.f[2] = skill_b;
    c.f[0] = speed;
    let tune = Addr(obj).off(0x18).lw(c);
    c.g[V0][0] = tune;
    Addr(obj).off(0xdf8).write_fpu(c, FpuF32(skill_a));
    Addr(obj).off(0xdfc).write_fpu(c, FpuF32(skill_b));
    Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
}

/// `skillB *= 0.01f` and return (`0x10c748`).
fn difficulty_finish(c: &mut Cpu, obj: u32) {
    c.cycles += 4;
    let b = Addr(obj).off(0xdfc).read_fpu(c);
    let scale = gpf(c, 1, -0x7d20);
    let scaled = b * FpuF32(scale);
    c.f[0] = scaled.0;
    Addr(obj).off(0xdfc).write_fpu(c, scaled);
    abi::ret(c);
}

/// `cComputer_setIndividualRiderDifficulty(cComputer *self, int level, int
/// mode)` (0x10c4f8): choose the rider's skill values and tune record for a
/// difficulty level 1..5.  Table A always applies; mode 0 and mode 2 then
/// override skills A/B and speed.  Finally skill B is scaled by 0.01.
pub fn computer_actor_rider_apply_difficulty(c: &mut Cpu) {
    c.cycles += 4;
    let obj = abi::arg(c, 0);
    let level = abi::arg(c, 1);
    let mode = abi::arg(c, 2);
    c.g[A3][0] = c.g[A0][0];
    c.g[V1][0] = sx(level.wrapping_sub(1));
    let in_range = level.wrapping_sub(1) < 5;
    c.g[V0][0] = in_range as u64;
    if in_range {
        // 0x10c508: dispatch through table A.
        c.cycles += 7;
        c.g[V1][0] = sx(level.wrapping_sub(1) << 2);
        c.g[V0][0] = sx(DIFFICULTY_TABLE_A);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        match level {
            1 => {
                // skillA = 100 via mtc1, skillB / accel / speed from the
                // table, boost 0.
                c.cycles += 14;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                c.lwc1(2, gp(c, -0x7dd4));
                let tune = Addr(obj).off(0x18).lw(c);
                c.lwc1(1, gp(c, -0x7dd0));
                let (a, b, acc) = (c.f[0], c.f[2], c.f[1]);
                Addr(obj).off(0xdf8).write_fpu(c, FpuF32(a));
                Addr(obj).off(0xdfc).write_fpu(c, FpuF32(b));
                Addr(tune as u32).off(0xdc).write_fpu(c, FpuF32(acc));
                c.lwc1(0, gp(c, -0x7dcc));
                c.g[V0][0] = tune;
                let speed = c.f[0];
                Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
                c.g[V1][0] = tune;
                Addr(tune as u32).off(0xe4).sw(c, 0);
            }
            2 => {
                let kb = gpf(c, 0, -0x7dc8);
                let ka = gpf(c, 2, -0x7dc4);
                let kacc = gpf(c, 1, -0x7dc0);
                let kspeed = gpf(c, 0, -0x7dbc);
                difficulty_case_full(c, obj, 14, ka, kb, kacc, kspeed, 1);
            }
            3 => {
                let kb = gpf(c, 0, -0x7db8);
                let ka = gpf(c, 2, -0x7db4);
                let kacc = gpf(c, 1, -0x7db0);
                let kspeed = gpf(c, 0, -0x7dac);
                difficulty_case_full(c, obj, 14, ka, kb, kacc, kspeed, 1);
            }
            4 => {
                let kb = gpf(c, 0, -0x7da8);
                let ka = gpf(c, 2, -0x7da4);
                let kacc = gpf(c, 1, -0x7da0);
                let kspeed = gpf(c, 0, -0x7d9c);
                difficulty_case_full(c, obj, 14, ka, kb, kacc, kspeed, 1);
            }
            _ => {
                let kb = gpf(c, 0, -0x7d98);
                let ka = gpf(c, 2, -0x7d94);
                let kacc = gpf(c, 1, -0x7d90);
                let kspeed = gpf(c, 0, -0x7d8c);
                difficulty_case_full(c, obj, 13, ka, kb, kacc, kspeed, 1);
            }
        }
    }
    // 0x10c608: mode dispatch.
    c.cycles += 2;
    c.g[V0][0] = 2;
    if mode != 0 {
        // 0x10c69c: only mode 2 has its own table.
        c.cycles += 2;
        if mode != 2 {
            c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
            difficulty_finish(c, obj);
            return;
        }
        // 0x10c6a4: dispatch through table C.
        c.cycles += 4;
        if !difficulty_dispatch_valid(c, level) {
            c.cycles += 1;
            c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
            difficulty_finish(c, obj);
            return;
        }
        c.cycles += 6;
        c.g[V1][0] = sx(level.wrapping_sub(1) << 2);
        c.g[V0][0] = sx(DIFFICULTY_TABLE_C);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        match level {
            1 => {
                // Both skills are 100, speed from the table.
                c.cycles += 8;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                let tune = Addr(obj).off(0x18).lw(c);
                c.g[V0][0] = tune;
                c.lwc1(1, gp(c, -0x7d54));
                let skill = c.f[0];
                Addr(obj).off(0xdfc).write_fpu(c, FpuF32(skill));
                Addr(obj).off(0xdf8).write_fpu(c, FpuF32(skill));
                let speed = c.f[1];
                Addr(tune as u32).off(0xe0).write_fpu(c, FpuF32(speed));
            }
            2 => {
                let ka = gpf(c, 1, -0x7d50);
                let kb = gpf(c, 2, -0x7d4c);
                let ks = gpf(c, 0, -0x7d48);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            3 => {
                let ka = gpf(c, 1, -0x7d44);
                let kb = gpf(c, 2, -0x7d40);
                let ks = gpf(c, 0, -0x7d3c);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            4 => {
                let ka = gpf(c, 1, -0x7d38);
                let kb = gpf(c, 2, -0x7d34);
                let ks = gpf(c, 0, -0x7d30);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            _ => {
                let ka = gpf(c, 1, -0x7d2c);
                let kb = gpf(c, 2, -0x7d28);
                let ks = gpf(c, 0, -0x7d24);
                difficulty_case_skill_speed(c, obj, 7, ka, kb, ks);
            }
        }
    } else {
        // 0x10c610: dispatch through table B.
        c.cycles += 4;
        if !difficulty_dispatch_valid(c, level) {
            c.cycles += 1;
            c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
            difficulty_finish(c, obj);
            return;
        }
        c.cycles += 6;
        c.g[V1][0] = sx(level.wrapping_sub(1) << 2);
        c.g[V0][0] = sx(DIFFICULTY_TABLE_B);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        match level {
            1 => {
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 1); // f1 = skill A = 100
                let skill_a = c.f[1];
                let kb = gpf(c, 2, -0x7d88);
                let ks = gpf(c, 0, -0x7d84);
                difficulty_case_skill_speed(c, obj, 9, skill_a, kb, ks);
            }
            2 => {
                let ka = gpf(c, 1, -0x7d80);
                let kb = gpf(c, 2, -0x7d7c);
                let ks = gpf(c, 0, -0x7d78);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            3 => {
                let ka = gpf(c, 1, -0x7d74);
                let kb = gpf(c, 2, -0x7d70);
                let ks = gpf(c, 0, -0x7d6c);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            4 => {
                let ka = gpf(c, 1, -0x7d68);
                let kb = gpf(c, 2, -0x7d64);
                let ks = gpf(c, 0, -0x7d24);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
            _ => {
                let ka = gpf(c, 1, -0x7d60);
                let kb = gpf(c, 2, -0x7d5c);
                let ks = gpf(c, 0, -0x7d58);
                difficulty_case_skill_speed(c, obj, 8, ka, kb, ks);
            }
        }
    }
    // 0x10c744.
    c.cycles += 1;
    c.f[0] = Addr(obj).off(0xdfc).read_fpu(c).0;
    difficulty_finish(c, obj);
}

/// The `level - 1 < 5` range check of the mode tables, including the
/// scratch registers the original leaves set.
fn difficulty_dispatch_valid(c: &mut Cpu, level: u32) -> bool {
    c.g[V1][0] = sx(level.wrapping_sub(1));
    c.g[V0][0] = (c.g[V1][0] < 5) as u64;
    let ok = c.g[V0][0] != 0;
    c.g[V0][0] = sx(0x0045_0000);
    ok
}

// ---------------------------------------------------------------------------
// Rail roller modifier.

/// `func_0035CFF0(RollerModifier *self, const Vec4 *v)` (0x35cff0):
/// remember the incoming position, invert the modifier's 3x3 rotation
/// matrix and transform `self->pos` by it (world -> modifier space).
///
/// The original copies the unaligned 0x20-byte rotation at `self+0x50`
/// (plus the word at `+0x70`) to a stack temporary, inverts it with the
/// cofactor/determinant expansion, then applies the inverse to
/// `self+0x90..0x9c` and stores the result at `self+0x30`.
pub fn roller_modifier_set_angular_momentum(c: &mut Cpu) {
    c.cycles += 165;
    let obj = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    let base = |c: &Cpu| c.g[A0][0] as u32;
    c.lq(2, out);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0xc0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 18); // f18 = 1.0
    c.sq(2, obj.wrapping_add(0x90));
    // Copy the unaligned rotation matrix `self+0x50..0x70` to `sp`.
    c.ldl(2, obj.wrapping_add(0x57));
    c.ldr(2, obj.wrapping_add(0x50));
    c.ldl(3, obj.wrapping_add(0x5f));
    c.ldr(3, obj.wrapping_add(0x58));
    c.ldl(5, obj.wrapping_add(0x67));
    c.ldr(5, obj.wrapping_add(0x60));
    c.ldl(6, obj.wrapping_add(0x6f));
    c.ldr(6, obj.wrapping_add(0x68));
    c.sdl(2, sp(c).wrapping_add(0x7));
    c.sdr(2, sp(c));
    c.sdl(3, sp(c).wrapping_add(0xf));
    c.sdr(3, sp(c).wrapping_add(0x8));
    c.sdl(5, sp(c).wrapping_add(0x17));
    c.sdr(5, sp(c).wrapping_add(0x10));
    c.sdl(6, sp(c).wrapping_add(0x1f));
    c.sdr(6, sp(c).wrapping_add(0x18));
    c.g[V0][0] = c.lw(base(c).wrapping_add(0x70));
    c.sw(sp(c).wrapping_add(0x20), c.g[V0][0]);
    // Invert the 3x3 part (cofactor expansion, determinant in f0).
    c.lwc1(12, sp(c).wrapping_add(0x10));
    c.lwc1(11, sp(c).wrapping_add(0x20));
    c.lwc1(4, sp(c).wrapping_add(0x14));
    c.lwc1(15, sp(c).wrapping_add(0x1c));
    c.mul_s(9, 12, 11);
    c.lwc1(10, sp(c).wrapping_add(0x8));
    c.lwc1(14, sp(c).wrapping_add(0x4));
    c.mul_s(0, 4, 15);
    c.mul_s(8, 15, 10);
    c.lwc1(3, sp(c).wrapping_add(0xc));
    c.mul_s(6, 11, 14);
    c.lwc1(1, sp(c));
    c.mul_s(7, 10, 12);
    c.lwc1(2, sp(c).wrapping_add(0x18));
    c.sub_s(9, 9, 0);
    c.mul_s(5, 14, 4);
    c.sub_s(8, 8, 6);
    c.mul_s(0, 1, 9);
    c.swc1(9, sp(c).wrapping_add(0x30));
    c.sub_s(5, 5, 7);
    c.mul_s(6, 3, 8);
    c.swc1(8, sp(c).wrapping_add(0x34));
    c.mul_s(16, 4, 1);
    c.mul_s(7, 2, 5);
    c.swc1(5, sp(c).wrapping_add(0x38));
    c.add_s(0, 0, 6);
    c.mul_s(13, 1, 11);
    c.mul_s(17, 15, 1);
    c.mul_s(6, 3, 10);
    c.add_s(0, 0, 7);
    c.mul_s(1, 1, 12);
    c.mul_s(7, 14, 3);
    c.mul_s(11, 11, 3);
    c.mul_s(10, 10, 2);
    c.mul_s(12, 12, 2);
    c.mul_s(4, 2, 4);
    c.mul_s(3, 3, 15);
    c.mul_s(2, 2, 14);
    c.div_s(0, 18, 0); // 1/det
    c.sub_s(4, 4, 11);
    c.sub_s(13, 13, 10);
    c.sub_s(6, 6, 16);
    c.sub_s(3, 3, 12);
    c.swc1(4, sp(c).wrapping_add(0x3c));
    c.sub_s(2, 2, 17);
    c.swc1(13, sp(c).wrapping_add(0x40));
    c.sub_s(1, 1, 7);
    c.swc1(6, sp(c).wrapping_add(0x44));
    c.swc1(3, sp(c).wrapping_add(0x48));
    c.mul_s(9, 9, 0);
    c.swc1(2, sp(c).wrapping_add(0x4c));
    c.mul_s(8, 8, 0);
    c.mul_s(7, 1, 0);
    c.swc1(1, sp(c).wrapping_add(0x50));
    c.mul_s(5, 5, 0);
    c.swc1(9, sp(c).wrapping_add(0x90));
    c.mul_s(4, 4, 0);
    c.swc1(8, sp(c).wrapping_add(0x94));
    c.mul_s(13, 13, 0);
    c.swc1(7, sp(c).wrapping_add(0xb0));
    c.mul_s(6, 6, 0);
    c.swc1(5, sp(c).wrapping_add(0x98));
    c.mul_s(3, 3, 0);
    c.swc1(4, sp(c).wrapping_add(0x9c));
    c.mul_s(2, 2, 0);
    c.swc1(13, sp(c).wrapping_add(0xa0));
    c.swc1(6, sp(c).wrapping_add(0xa4));
    c.swc1(3, sp(c).wrapping_add(0xa8));
    c.swc1(2, sp(c).wrapping_add(0xac));
    // Move the inverse to `sp+0x60` and its translation row to `sp+0x20`.
    c.ldl(2, sp(c).wrapping_add(0x97));
    c.ldr(2, sp(c).wrapping_add(0x90));
    c.ldl(3, sp(c).wrapping_add(0x9f));
    c.ldr(3, sp(c).wrapping_add(0x98));
    c.ldl(5, sp(c).wrapping_add(0xa7));
    c.ldr(5, sp(c).wrapping_add(0xa0));
    c.ldl(6, sp(c).wrapping_add(0xaf));
    c.ldr(6, sp(c).wrapping_add(0xa8));
    c.sdl(2, sp(c).wrapping_add(0x67));
    c.sdr(2, sp(c).wrapping_add(0x60));
    c.sdl(3, sp(c).wrapping_add(0x6f));
    c.sdr(3, sp(c).wrapping_add(0x68));
    c.sdl(5, sp(c).wrapping_add(0x77));
    c.sdr(5, sp(c).wrapping_add(0x70));
    c.sdl(6, sp(c).wrapping_add(0x7f));
    c.sdr(6, sp(c).wrapping_add(0x78));
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0xb0));
    c.sw(sp(c).wrapping_add(0x80), c.g[V0][0]);
    c.ldl(2, sp(c).wrapping_add(0x67));
    c.ldr(2, sp(c).wrapping_add(0x60));
    c.ldl(3, sp(c).wrapping_add(0x6f));
    c.ldr(3, sp(c).wrapping_add(0x68));
    c.ldl(5, sp(c).wrapping_add(0x77));
    c.ldr(5, sp(c).wrapping_add(0x70));
    c.ldl(6, sp(c).wrapping_add(0x7f));
    c.ldr(6, sp(c).wrapping_add(0x78));
    c.sdl(2, sp(c).wrapping_add(0x7));
    c.sdr(2, sp(c));
    c.sdl(3, sp(c).wrapping_add(0xf));
    c.sdr(3, sp(c).wrapping_add(0x8));
    c.sdl(5, sp(c).wrapping_add(0x17));
    c.sdr(5, sp(c).wrapping_add(0x10));
    c.sdl(6, sp(c).wrapping_add(0x1f));
    c.sdr(6, sp(c).wrapping_add(0x18));
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x80));
    c.sw(sp(c).wrapping_add(0x20), c.g[V0][0]);
    // out = inverse * self->pos, row by row.
    c.lwc1(1, base(c).wrapping_add(0x90));
    c.lwc1(7, base(c).wrapping_add(0x94));
    c.lwc1(3, sp(c));
    c.lwc1(9, sp(c).wrapping_add(0xc));
    c.lwc1(2, sp(c).wrapping_add(0x4));
    c.mul_s(3, 3, 1);
    c.lwc1(8, sp(c).wrapping_add(0x10));
    c.mul_s(9, 9, 7);
    c.lwc1(0, sp(c).wrapping_add(0x8));
    c.mul_s(2, 2, 1);
    c.lwc1(4, sp(c).wrapping_add(0x14));
    c.mul_s(8, 8, 7);
    c.mul_s(0, 0, 1);
    c.lwc1(5, base(c).wrapping_add(0x98));
    c.mul_s(4, 4, 7);
    c.lwc1(6, sp(c).wrapping_add(0x18));
    c.lwc1(7, sp(c).wrapping_add(0x1c));
    c.add_s(3, 3, 9);
    c.lwc1(1, sp(c).wrapping_add(0x20));
    c.add_s(2, 2, 8);
    c.add_s(0, 0, 4);
    c.lwc1(8, base(c).wrapping_add(0x9c));
    c.mul_s(1, 1, 5);
    c.mul_s(6, 6, 5);
    c.swc1(8, sp(c).wrapping_add(0x4c));
    c.mul_s(7, 7, 5);
    c.add_s(0, 0, 1);
    c.add_s(3, 3, 6);
    c.add_s(2, 2, 7);
    c.swc1(0, sp(c).wrapping_add(0x48));
    c.swc1(3, sp(c).wrapping_add(0x40));
    c.swc1(2, sp(c).wrapping_add(0x44));
    c.lq(2, sp(c).wrapping_add(0x40));
    c.sq(2, base(c).wrapping_add(0x30));
    c.sq(2, sp(c).wrapping_add(0x30));
    c.g[SP][0] = sx(sp(c).wrapping_add(0xc0));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end popup background placement.

/// Copy a 12-byte position from `obj+vec` to the stack temporary, add `a`
/// (f20) to x, set y from f13 and store it at `child+0x44` (the trailing word
/// is at `child+0x4c`).  `load`/`store` select the scratch registers the
/// original happens to use.
#[allow(clippy::too_many_arguments)]
fn position_child(
    c: &mut Cpu,
    obj: u32,
    vec: u32,
    child: u32,
    load: usize,
    load_z: usize,
    store: usize,
    store_z: usize,
) {
    c.ldl(load, obj.wrapping_add(vec + 0xb));
    c.ldr(load, obj.wrapping_add(vec));
    c.g[load_z][0] = c.lw(obj.wrapping_add(vec + 8));
    c.sdl(load, sp(c).wrapping_add(0x17));
    c.sdr(load, sp(c).wrapping_add(0x10));
    c.sw(sp(c).wrapping_add(0x18), c.g[load_z][0]);
    c.swc1(13, sp(c).wrapping_add(0x14));
    c.lwc1(0, sp(c).wrapping_add(0x10));
    c.add_s(0, 0, 20);
    c.swc1(0, sp(c).wrapping_add(0x10));
    c.ldl(store, sp(c).wrapping_add(0x17));
    c.ldr(store, sp(c).wrapping_add(0x10));
    c.g[store_z][0] = c.lw(sp(c).wrapping_add(0x18));
    c.sdl(store, child.wrapping_add(0x4b));
    c.sdr(store, child.wrapping_add(0x44));
    c.sw(child.wrapping_add(0x4c), c.g[store_z][0]);
}

/// `cFEPopupConfirm_PositionBackground(self, const Vec3 *v, float a, float
/// b)` (0x1c78c0): position the popup's buttons from the stored button
/// vectors.  `self+0x31c` selects the layout: 0 = one column, 1 = five
/// buttons (with a rounded y), 2 = reset the slide values, anything else =
/// leave untouched.
pub fn fe_popup_confirm_position_background(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        c.mov_s(4, 13); // b
        frame.save(c, S0, 0x30);
        c.g[V0][0] = 1;
        c.swc1(20, frame.at(c, 0x40));
        let obj = c.g[A0][0] as u32;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.mov_s(20, 12); // a
        c.mtc1(0, 13); // y = 0
        let v = c.g[A1][0] as u32;
        c.g[V1][0] = Addr(obj).off(0x31c).lw(c);
        // Copy the incoming Vec3 to the stack temporary at sp+0.
        c.ldl(4, v.wrapping_add(0x7));
        c.ldr(4, v);
        c.g[A2][0] = c.lw(v.wrapping_add(0x8));
        c.sdl(4, frame.at(c, 0x7));
        c.sdr(4, frame.at(c, 0x0));
        let state = c.g[V1][0]; // sign-extended `lw`
        let is_one = c.g[V1][0] == c.g[V0][0];
        c.sw(frame.at(c, 0x8), c.g[A2][0]);
        if !is_one {
            c.cycles += 3;
            c.g[V0][0] = ((state as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                // state >= 2: only state 2 has work.
                c.cycles += 2;
                c.g[V0][0] = 2;
                if state == 2 {
                    c.cycles += 5;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.swc1(0, Addr(obj).off(0x27c).0);
                    c.swc1(0, Addr(obj).off(0x278).0);
                } else {
                    c.cycles += 2;
                }
            } else if state == 0 {
                // Layout 0: two stacked buttons.
                c.cycles += 2;
                c.g[V0][0] = Addr(obj).off(0x48).lw(c);
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 27;
                    c.ldl(3, obj.wrapping_add(0x18f));
                    c.ldr(3, obj.wrapping_add(0x188));
                    c.g[A0][0] = c.lw(obj.wrapping_add(0x190));
                    c.sdl(3, frame.at(c, 0x17));
                    c.sdr(3, frame.at(c, 0x10));
                    c.sw(frame.at(c, 0x18), c.g[A0][0]);
                    c.g[AT][0] = 0x3f00_0000;
                    c.mtc1(1, 3); // f3 = 0.5
                    c.lwc1(1, obj.wrapping_add(0x1bc));
                    c.lwc1(2, frame.at(c, 0x4)); // v.y
                    c.add_s(1, 1, 4); // base + b
                    c.lwc1(0, frame.at(c, 0x10)); // p.x
                    c.mul_s(2, 2, 3); // v.y * 0.5
                    c.g[AT][0] = 0x40a0_0000;
                    c.mtc1(1, 4); // f4 = 5.0
                    c.add_s(0, 0, 20); // p.x += a
                    c.add_s(1, 1, 2);
                    c.swc1(0, frame.at(c, 0x10));
                    c.sub_s(13, 1, 4); // y = base + b + v.y*0.5 - 5
                    c.swc1(13, frame.at(c, 0x14));
                    c.ldl(7, frame.at(c, 0x17));
                    c.ldr(7, frame.at(c, 0x10));
                    c.g[T0][0] = c.lw(frame.at(c, 0x18));
                    let child = c.g[V0][0] as u32;
                    c.sdl(7, child.wrapping_add(0x4b));
                    c.sdr(7, child.wrapping_add(0x44));
                    c.sw(child.wrapping_add(0x4c), c.g[T0][0]);
                    c.g[V0][0] = Addr(obj).off(0x4c).lw(c);
                } else {
                    c.g[V0][0] = Addr(obj).off(0x4c).lw(c);
                }
                // Second button at 0x4c.
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 18;
                    let child = c.g[V0][0] as u32;
                    position_child(c, obj, 0x194, child, 5, A2, 3, A0);
                }
            } else {
                // Negative state: nothing to do (the two +2 charges of
                // 0x1c7930 and its fall-through).
                c.cycles += 4;
            }
        } else {
            // Layout 1: five buttons; the first one computes the shaped y.
            c.cycles += 3;
            c.g[V0][0] = Addr(obj).off(0x58).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 33;
                let child = c.g[V0][0] as u32;
                c.ldl(7, obj.wrapping_add(0x243));
                c.ldr(7, obj.wrapping_add(0x23c));
                c.g[T0][0] = c.lw(obj.wrapping_add(0x244));
                c.sdl(7, frame.at(c, 0x17));
                c.sdr(7, frame.at(c, 0x10));
                c.sw(frame.at(c, 0x18), c.g[T0][0]);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 1);
                c.lwc1(12, frame.at(c, 0x4));
                c.lwc1(0, obj.wrapping_add(0x1bc));
                c.mul_s(12, 12, 1);
                c.add_s(0, 0, 4);
                c.add_s(12, 0, 12);
                call_ptr(c, 0x0041_3af8, 0x001c_7a38)?; // __extendsfdf2
                c.g[A1][0] = 0x803c;
                c.g[A1][0] <<= 47; // 7.5
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_1f70, 0x001c_7a48)?; // __subdf3
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0041_27a8, 0x001c_7a50)?; // __truncdfsf2
                c.lwc1(1, frame.at(c, 0x10));
                c.mov_s(13, 0);
                c.g[V0][0] = Addr(obj).off(0x58).lw(c);
                c.add_s(1, 1, 20);
                c.swc1(13, frame.at(c, 0x14));
                c.swc1(1, frame.at(c, 0x10));
                c.ldl(4, frame.at(c, 0x17));
                c.ldr(4, frame.at(c, 0x10));
                c.g[A1][0] = c.lw(frame.at(c, 0x18));
                c.sdl(4, child.wrapping_add(0x4b));
                c.sdr(4, child.wrapping_add(0x44));
                c.sw(child.wrapping_add(0x4c), c.g[A1][0]);
                c.g[V0][0] = Addr(obj).off(0x5c).lw(c);
            } else {
                c.g[V0][0] = Addr(obj).off(0x5c).lw(c);
            }
            // Four more buttons, all with the y computed above (f13).
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 17;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x248, child, 8, V1, 6, A3);
            }
            c.g[V0][0] = Addr(obj).off(0x50).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 17;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x224, child, 4, A1, 8, V1);
            }
            c.g[V0][0] = Addr(obj).off(0x54).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 17;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x230, child, 6, A3, 4, A1);
            }
            c.g[V0][0] = Addr(obj).off(0x60).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 18;
                let child = c.g[V0][0] as u32;
                position_child(c, obj, 0x254, child, 8, V1, 6, A3);
            }
        }
        c.lq(16, frame.at(c, 0x30));
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.lwc1(20, frame.at(c, 0x40));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        abi::ret(c);
        Ok(())
    })
}
// ---------------------------------------------------------------------------
// MIDI play-parameter ring buffer.

/// `MIDIBanks_PushPlayParams(Bank *banks, const PlayParams *p, int channel)`
/// (0x2a8450): append one play-parameter record to the bank's ring buffers.
///
/// `banks->first` (offset 0) is the track object; `+0x1f4` is the write
/// cursor, `+0x1dc..0x1f4` the per-call parameter copies, `+0x1fc` the
/// record queue and `first+0x1d8` an array descriptor whose `+0x20` count
/// wraps the cursor.  The parallel arrays at `+0x34..0x84` hold the
/// per-record fields.
pub fn midi_banks_push_play_params(c: &mut Cpu) {
    c.cycles += 153;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[T1][0] = 0x18;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A0][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S3][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S4][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 0u64;
    c.g[S0][0] = c.lw((c.g[S2][0] as u32));
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[V1][0] as u32));
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x002a854c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210; call_indirect(c, 0x00416210);
    if c.pc != 0x002a854c { return; }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A3][0] = 0x7f;
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sw((c.g[V1][0] as u32), c.g[T0][0]);
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.g[A0][0] = c.lw((c.g[S2][0] as u32));
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S3][0]);
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio: play a sound.

/// `SSXAudioSystem_PlaySound(self, int bankId, int soundId)` (0x290cc0):
/// queue one sound.  It is the same ring-buffer push as
/// [`midi_banks_push_play_params`], then the bank's channel state gives the
/// gain slot and the sound is played.
pub fn ssx_audio_system_play_sound(c: &mut Cpu) {
    c.cycles += 166;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[T1][0] = 0x18;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A0][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S3][0] = c.g[A1][0];
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S4][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 0u64;
    c.g[S0][0] = c.lw((c.g[S2][0] as u32));
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[V1][0] as u32));
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x00290dbc);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210; call_indirect(c, 0x00416210);
    if c.pc != 0x00290dbc { return; }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A3][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[T0][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[T1][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[T2][0] = 0x7f;
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[T3][0] = 0x1;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0x5;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = 0u64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), c.g[A3][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), c.g[T2][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[T3][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S3][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S4][0]);
    c.g[RA][0] = sx(0x00290f18);
    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
    c.pc = 0x00287968; ee::s07::SSXAudioSystem_GetChannelGainPtr(c);
    if c.pc != 0x00290f18 { return; }
    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = c.g[S2][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[RA][0] = sx(0x00290f38);
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.pc = 0x002906b8; ee::s07::Audio_PlayPushedSound(c);
    if c.pc != 0x00290f38 { return; }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        abi::ret(c);
}

// ---------------------------------------------------------------------------
// Camera shake.

/// `ChaseCameraAlgorithm_StartShake(self, float ampX, float ampY, float
/// ampZ)` (0x1656b0): arm a shake on a chase camera.  The amplitudes are
/// randomised around the requested values and written to the shake state at
/// `self+0x254..0x2ec`; the slide values restart at zero.
pub fn chase_camera_start_shake(c: &mut Cpu) {
    c.cycles += 162;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = c.g[A0][0];
    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S1][0] = 0x7f0000;
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.mov_s(24, 12);
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.mov_s(23, 13);
    c.mov_s(22, 14);
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[S1][0] | 0xffff;
    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x78));
    c.g[S2][0] = 0x3f800000;
    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.neg_s(25, 22);
    c.g[RA][0] = sx(0x00165708);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165708 { return; }
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.sub_s(26, 22, 25);
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165734);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x254));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165734 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165754);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x258));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165754 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x68d4));
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.mul_s(21, 22, 21);
    c.sub_s(0, 1, 20);
    c.neg_s(27, 21);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165780);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x25c));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165780 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 26, 0);
    c.add_s(0, 25, 0);
    c.g[RA][0] = sx(0x001657a0);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x260));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x001657a0 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(26, 26, 0);
    c.add_s(25, 25, 26);
    c.g[RA][0] = sx(0x001657c0);
    c.swc1(25, (c.g[S0][0] as u32).wrapping_add(0x264));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x001657c0 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.sub_s(1, 21, 27);
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 0);
    c.mtc1(2, 3);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x26c), 0u64);
    c.mul_s(24, 24, 0);
    c.sub_s(2, 3, 20);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x270), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x274), 0u64);
    c.mul_s(23, 23, 0);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x278), 0u64);
    c.mul_s(21, 21, 0);
    c.mul_s(1, 1, 2);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x27c), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x280), 0u64);
    c.mul_s(22, 22, 0);
    c.neg_s(26, 21);
    c.add_s(27, 27, 1);
    c.neg_s(25, 22);
    c.g[RA][0] = sx(0x0016581c);
    c.swc1(27, (c.g[S0][0] as u32).wrapping_add(0x268));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x0016581c { return; }
    c.sub_s(22, 22, 25);
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165840);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x284));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165840 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 23, 0);
    c.add_s(0, 24, 0);
    c.g[RA][0] = sx(0x00165860);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x288));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165860 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(23, 23, 0);
    c.add_s(24, 24, 23);
    c.g[RA][0] = sx(0x00165880);
    c.swc1(24, (c.g[S0][0] as u32).wrapping_add(0x28c));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x00165880 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 3);
    c.sub_s(0, 3, 20);
    c.mul_s(0, 22, 0);
    c.add_s(0, 25, 0);
    c.g[RA][0] = sx(0x001658a0);
    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x290));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x001658a0 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.mtc1(2, 1);
    c.sub_s(0, 1, 20);
    c.mul_s(22, 22, 0);
    c.add_s(25, 25, 22);
    c.g[RA][0] = sx(0x001658c0);
    c.swc1(25, (c.g[S0][0] as u32).wrapping_add(0x294));
    c.pc = 0x003177f0; call_indirect(c, 0x003177f0);
    if c.pc != 0x001658c0 { return; }
    c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
    c.sub_s(21, 21, 26);
    c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2b4), 0u64);
    c.mtc1(2, 3);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x29c), 0u64);
    c.sub_s(20, 3, 20);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2a0), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2a4), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2a8), 0u64);
    c.mul_s(21, 21, 20);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2ac), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2b0), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), 0u64);
    c.add_s(26, 26, 21);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.swc1(26, (c.g[S0][0] as u32).wrapping_add(0x298));
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x78));
    c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_words, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn difficulty_matches() {
        check(
            ssx3_ee::s00::f_0010c4f8__generated,
            computer_actor_rider_apply_difficulty,
            300,
            &|c, seed| {
                let mut r = Rng(0x5177_a9c3 ^ seed as u64);
                // The generated body is a `match pc` dispatcher that also
                // jumps through the difficulty jump tables, so a caller
                // enters it with pc = entry and the tables (`.rodata`) hold
                // the case-block addresses.
                put_words(
                    c,
                    0x0045_6a90,
                    &[0x0010_c524, 0x0010_c55c, 0x0010_c584, 0x0010_c5ac, 0x0010_c5d4],
                );
                put_words(
                    c,
                    0x0045_6ab0,
                    &[0x0010_c638, 0x0010_c650, 0x0010_c664, 0x0010_c678, 0x0010_c688],
                );
                put_words(
                    c,
                    0x0045_6ad0,
                    &[0x0010_c6cc, 0x0010_c6ec, 0x0010_c700, 0x0010_c714, 0x0010_c728],
                );
                scramble(c, &mut r);
                c.pc = 0x0010_c4f8;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 7) as u64; // 0, 6 and 1..5
                c.g[A2][0] = (seed % 4) as u64; // modes 0, 1, 2, 3
                c.bus.write32(DATA + 0x18, DATA + 0x100);
                // The float constants the tables load from `gp`.
                let g = (GAME_GP as u32).wrapping_sub(0x7dd8);
                for k in 0..0x30u32 {
                    c.bus.write32(g + 4 * k, r.next_u32());
                }
                for k in 0..0x10u32 {
                    c.bus.write32(DATA + 0x100 + 4 * k, r.next_u32());
                    c.bus.write32(DATA + 0xdf8 + 4 * k, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn roller_set_angular_momentum_matches() {
        check(
            ssx3_ee::s11::f_0035cff0__generated,
            roller_modifier_set_angular_momentum,
            200,
            &|c, seed| {
                let mut r = Rng(0x35cf_f0d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                // rotation at +0x50..0x74, position at +0x90..0x9c
                for k in 0..0x30u32 {
                    c.bus.write32(DATA + 0x30 + 4 * k, r.next_u32());
                }
                for k in 0..0x10u32 {
                    c.bus.write32(DATA + 0x90 + 4 * k, r.next_u32());
                }
                for k in 0..8u32 {
                    c.bus.write32(DATA + 0x400 + 4 * k, r.next_u32());
                }
            },
        );
    }

    /// The MIDI bank structures `PushPlayParams` writes: `*banks` is the
    /// track object, whose array descriptors at `+0x1d8` point at the record
    /// arrays.  The write cursor at `+0x1f4` (descriptor `+0x1c`) stays small.
    fn midi_bank(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(DATA + 0x100, DATA + 0x200);
        c.bus.write32(DATA + 0x104, DATA + 0x2400);
        c.bus.write32(DATA + 0x108, DATA + 0x2500);
        c.bus.write32(DATA + 0x218, DATA + 0x600); // banks+0x118: channel state
        c.bus.write32(DATA + 0x600, DATA + 0x700);
        for (base, arr) in [(DATA + 0x200, DATA + 0x2000), (DATA + 0x700, DATA + 0x4000)] {
            for (i, off) in [
                0x20u32, 0x22, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c,
                0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
            ]
            .iter()
            .enumerate()
            {
                c.bus.write32(base + 0x1d8 + off, arr + 0x400 * i as u32);
            }
            c.bus.write32(base + 0x1f4, seed % 4); // write cursor
            for k in 0..0x30u32 {
                c.bus.write32(base + 0x1dc + 4 * k, r.next_u32());
            }
            for k in 0..0x100u32 {
                c.bus.write8(arr + k, (r.next_u32() & 0x7f) as u8);
            }
        }
        for k in 0..0x20u32 {
            c.bus.write32(DATA + 0x900 + 4 * k, r.next_u32());
        }
    }

    #[test]
    fn push_play_params_matches() {
        check(
            ssx3_ee::s08::f_002a8450__generated,
            midi_banks_push_play_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_8450 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.g[A2][0] = (seed % 16) as u64;
                midi_bank(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn play_sound_matches() {
        check(
            ssx3_ee::s07::f_00290cc0__generated,
            ssx_audio_system_play_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_0cc0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 16) as u64;
                midi_bank(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn start_shake_matches() {
        check(
            ssx3_ee::s01::f_001656b0__generated,
            chase_camera_start_shake,
            200,
            &|c, seed| {
                let mut r = Rng(0x0016_56b0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(4.0);
                c.f[13] = r.f32(4.0);
                c.f[14] = r.f32(4.0);
                // The per-frame random amount the shake scales by.
                c.bus.write32((GAME_GP as u32).wrapping_sub(0x68d4), r.f32(1.0));
            },
        );
    }

    #[test]
    fn play_rail_change_sound_matches() {
        check(
            ssx3_ee::s08::f_00299b70__generated,
            ssx_audio_system_play_rail_change_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_9b70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64; // rider
                midi_bank(c, &mut r, seed);
                // The rider's rail-sound state.
                c.bus.write32(DATA + 0x3000 + 0x874, (seed % 3) as u32);
                c.bus.write32(DATA + 0x3000 + 0x87c, (seed % 5) as u32);
            },
        );
    }

    #[test]
    fn start_loop_sfx_matches() {
        check(
            ssx3_ee::s08::f_0029d370__generated,
            ssx_audio_system_start_loop_sfx,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_d370 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                midi_bank(c, &mut r, seed);
                // The loop-effect handle slot: sometimes armed.
                c.bus.write32(DATA + 0x100 + 0x6c74, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn position_background_matches() {
        check(
            ssx3_ee::s03::f_001c78c0__generated,
            fe_popup_confirm_position_background,
            250,
            &|c, seed| {
                let mut r = Rng(0x001c_78c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64;
                c.f[12] = r.f32(40.0);
                c.f[13] = r.f32(40.0);
                for k in 0..3u32 {
                    c.bus.write32(DATA + 0x3000 + 4 * k, r.next_u32());
                }
                // Every layout: 0, 1, 2, >2 and negative.
                let state = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => -1i32,
                    5 => 0x7fff_ffff,
                    _ => 0,
                };
                c.bus.write32(DATA + 0x31c, state as u32);
                let null_odd = seed % 3 == 0;
                let children = [0x48u32, 0x4c, 0x58, 0x5c, 0x50, 0x54, 0x60];
                for (i, off) in children.iter().enumerate() {
                    let p = if null_odd && i % 2 == 0 {
                        0
                    } else {
                        DATA + 0x1000 + 0x100 * i as u32
                    };
                    c.bus.write32(DATA + off, p);
                }
                let sources = [0x188u32, 0x194, 0x23c, 0x248, 0x224, 0x230, 0x254];
                for off in sources {
                    for k in 0..3u32 {
                        c.bus.write32(DATA + off + 4 * k, r.next_u32());
                    }
                }
                c.bus.write32(DATA + 0x1bc, r.f32(100.0));
                c.bus.write32(DATA + 0x278, r.next_u32());
                c.bus.write32(DATA + 0x27c, r.next_u32());
            },
        );
    }
}
// ---------------------------------------------------------------------------
// Audio: rail-change sound.

/// `SSXAudioSystem_PlayRailChangeSound(self, Rider *rider)` (0x299b70):
/// while a replay is playing, or before the rider has a rail sound pending,
/// do nothing; otherwise push a play record (sound 9) and start it.
pub fn ssx_audio_system_play_rail_change_sound(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x00299b94);
    c.g[S0][0] = c.g[A1][0];
    c.pc = 0x00288ae0; call_indirect(c, 0x00288ae0);
    if c.pc != 0x00299b94 { return; }
    if c.g[V0][0] != 0 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x874));
        c.g[A1][0] = 0u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x87c));
            c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        c.g[T1][0] = 0x18;
        if c.g[A1][0] != 0 {
            c.cycles += 148;
            c.g[S0][0] = c.lw((c.g[S3][0] as u32));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[S2][0] = 0x6c;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 9);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[V1][0] as u32));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 9);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            c.g[V1][0] = c.lb((c.g[A0][0] as u32));
            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[RA][0] = sx(0x00299c8c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            c.pc = 0x00416210; call_indirect(c, 0x00416210);
            if c.pc != 0x00299c8c { return; }
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A3][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[T0][0] = 0x5a;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[T1][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[T2][0] = 0x7f;
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[T3][0] = 0x1;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x9;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), c.g[A3][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), c.g[T2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[T3][0]);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S2][0]);
            c.g[RA][0] = sx(0x00299de8);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            c.pc = 0x00287968; call_indirect(c, 0x00287968);
            if c.pc != 0x00299de8 { return; }
            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.g[RA][0] = sx(0x00299e08);
            c.sw((c.g[V1][0] as u32), c.g[V0][0]);
            c.pc = 0x002906b8; call_indirect(c, 0x002906b8);
            if c.pc != 0x00299e08 { return; }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    abi::ret(c);
}// ---------------------------------------------------------------------------
// Audio: looping effect.

/// `SSXAudioSystem_StartLoopSfx(self)` (0x29d370): if the looping-effect
/// slot at `self+0x6c74` is armed, push a play record (sound 9) and start it,
/// store the handle at `+0x5fc8`; set the active flag at `+0x5fc4`.
pub fn ssx_audio_system_start_loop_sfx(c: &mut Cpu) {
    c.cycles += 10;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6c74));
    c.g[T1][0] = 0x18;
    if c.g[V0][0] != 0 {
        c.cycles += 149;
        c.g[S0][0] = c.lw((c.g[S3][0] as u32));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
        c.g[S2][0] = 0x9;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 3, 3, 9);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[V1][0] as u32));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 9);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[RA][0] = sx(0x0029d46c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        c.pc = 0x00416210; call_indirect(c, 0x00416210);
        if c.pc != 0x0029d46c { return; }
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A3][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T0][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[T1][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[T2][0] = 0x7f;
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[T3][0] = 0x1;
        c.g[A1][0] = 0x5;
        c.g[A2][0] = 0u64;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.g[S3][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[A3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[T2][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[T3][0]);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S2][0]);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[RA][0] = sx(0x0029d5c8);
        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
        c.pc = 0x00287968; call_indirect(c, 0x00287968);
        if c.pc != 0x0029d5c8 { return; }
        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.g[S3][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[RA][0] = sx(0x0029d5e8);
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        c.pc = 0x002906b8; call_indirect(c, 0x002906b8);
        if c.pc != 0x0029d5e8 { return; }
        c.sw((c.g[S3][0] as u32).wrapping_add(0x5fc8), c.g[V0][0]);
    }
    c.cycles += 9;
    c.g[V0][0] = 0x1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x5fc4), c.g[V0][0]);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    abi::ret(c);
}