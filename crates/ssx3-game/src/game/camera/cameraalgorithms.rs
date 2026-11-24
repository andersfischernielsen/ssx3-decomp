//! `src/camera/cameraalgorithms.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::fe::feasyncfile::gp_17 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0015_dac0,
        game_camera_set_collision_flag4a4,
        "GameCamera_SetCollisionFlag4a4",
    ),
    (0x0015_db58, game_camera_reset_all, "GameCamera_ResetAll"),
    (
        0x0015_d078,
        chase_cam_controller_set_algorithm,
        "ChaseCamController_SetAlgorithm",
    ),
    (
        0x0015_dae0,
        game_camera_reset_all_chase_algorithms,
        "GameCamera_ResetAllChaseAlgorithms",
    ),
];

/// `void GameCamera_SetCollisionFlag4a4(void)` (0x0015dac0): set the word at
/// `+0x4a4` of the camera control object (world → `+0x84` camera manager →
/// `+0x84` chase manager → `+0x4` controller) to 1.
pub fn game_camera_set_collision_flag4a4(c: &mut Cpu) {
    c.cycles += 7;
    let world = Addr(gp(c, -0x848)).lw(c);
    c.g[A0][0] = world;
    c.g[A1][0] = 1;
    let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
    c.g[V0][0] = cameras;
    let chase = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = chase;
    let control = Addr((chase as u32).wrapping_add(4)).lw(c);
    c.g[A0][0] = control;
    c.sw((control as u32).wrapping_add(0x4a4), 1);
    abi::ret(c);
}

/// `lh` of `a + off`.
#[inline(always)]
pub(crate) fn lh(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lh(c)
}

// ---------------------------------------------------------------------------
// `GameCamera_ResetAll` (0x0015db58).
//
// `void GameCamera_ResetAll()`: reset the chase camera state of both camera
// slots of the race manager.

pub fn game_camera_reset_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = 0x4c0000;
        frame.save(c, 17, 0x10);
        c.g[S0][0] = sx(0x4c0000u32.wrapping_add(0x5830));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_c968, 0x0015_db78)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0016_d1b8, 0x0015_db80)?;
        c.g[V0][0] = Addr(gp_2(c, -0x848)).lw(c);
        c.g[V1][0] = lw(c, c.g[V0][0] as u32, 0x84);
        let s1 = lw(c, c.g[V1][0] as u32, 0x84) as u32;
        c.g[S1][0] = sx(s1);
        c.g[V0][0] = lw(c, s1, 4);
        c.g[S0][0] = lw(c, s1, 0);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        call_ptr(c, 0x0016_2170, 0x0015_db9c)?;
        c.g[V0][0] = lw(c, s1, 4);
        c.g[A1][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        c.g[V1][0] = lw(c, c.g[A1][0] as u32, 0x14);
        c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
        c.g[V0][0] = lw(c, c.g[V1][0] as u32, 0x24);
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0015_dbb8)?;
        c.g[V1][0] = lw(c, s1, 4);
        c.g[A0][0] = 1;
        c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
        sw(c, c.g[V1][0] as u32, 0x460, 0);
        c.g[V0][0] = lw(c, s1, 4);
        sw(c, c.g[V0][0] as u32, 0x4a4, c.g[A0][0]);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            frame.restore(c, 16, 0x20);
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 12;
        c.g[V0][0] = lw(c, s1, 8);
        c.g[A0][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        call_ptr(c, 0x0016_2170, 0x0015_dbe0)?;
        c.g[V0][0] = lw(c, s1, 8);
        c.g[A2][0] = lw(c, c.g[V0][0] as u32, 0xa8);
        c.g[V1][0] = lw(c, c.g[A2][0] as u32, 0x14);
        c.g[A0][0] = lh(c, c.g[V1][0] as u32, 0x20);
        c.g[A1][0] = lw(c, c.g[V1][0] as u32, 0x24);
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0015_dbfc)?;
        c.g[V0][0] = lw(c, s1, 8);
        sw(c, c.g[V0][0] as u32, 0x460, 0);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Controller: flag word; bit 1 = new algorithm takes over at once (blend 1.0).
pub(crate) const SET_ALGO_CTRL_FLAGS: u32 = 0x30;

/// Controller: the target handed to the new algorithm's `setTarget`.
pub(crate) const SET_ALGO_CTRL_TARGET: u32 = 0x20;

/// The jump table from kind to the kind's block (98 words).
#[cfg(test)]
pub(crate) const SET_ALGO_JUMP_TABLE: u32 = 0x45_b5c0;

/// Allocation: `EA_MemAllocNamed(size, name, 0x20000000, 0)`; the name is the
/// string at `0x460000 - name_back`.
pub(crate) fn set_algo_alloc(c: &mut Cpu, size: u32, name_back: u32, ret: u32) -> Flow {
    c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(name_back));
    c.g[A0][0] = size as u64;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, ret)
}

pub(crate) fn set_algo_call(c: &mut Cpu, target: Target, ret: u32) -> Flow {
    match target {
        Target::Direct(f) => f.call(c, ret),
        Target::Table(addr) => call_ptr(c, addr, ret),
    }
}

/// The common shape: allocate, run the constructor on the block, `algo = v0`.
pub(crate) fn set_algo_simple(
    c: &mut Cpu,
    cycles: u64,
    size: u32,
    name_back: u32,
    alloc_ret: u32,
    ctor: Target,
    ctor_ret: u32,
) -> Flow {
    c.cycles += cycles;
    set_algo_alloc(c, size, name_back, alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    set_algo_call(c, ctor, ctor_ret)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// Kinds 18..25: `DirectionChaseAlgorithm(mem, direction)`.
pub(crate) fn set_algo_direction(c: &mut Cpu, dir: u32) -> Flow {
    const NAME_BACK: [u32; 8] = [
        0x4c30, 0x4c08, 0x4be0, 0x4bb8, 0x4b90, 0x4b68, 0x4b40, 0x4b18,
    ];
    const ALLOC_RET: [u32; 8] = [
        0x0015_d31c,
        0x0015_d340,
        0x0015_d364,
        0x0015_d388,
        0x0015_d3ac,
        0x0015_d3d0,
        0x0015_d3f4,
        0x0015_d418,
    ];
    c.cycles += if dir == 7 { 8 } else { 9 };
    set_algo_alloc(c, 0x3a0, NAME_BACK[dir as usize], ALLOC_RET[dir as usize])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = dir as u64;
    c.cycles += 4;
    call_ptr(c, 0x0017_74c0, 0x0015_d428)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

/// Kinds 33..40, 94..97: `RelativeChaseAlgorithm(mem, angle)`, then the kind
/// is stored at `algo + 0xc`. The block is in a0 and the angle in f12.
pub(crate) fn set_algo_relative_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    Callee {
        addr: 0x0016_0b70,
        f: ee::s01::f_00160b70,
    }
    .call(c, 0x0015_d620)?;
    c.g[S0][0] = c.g[V0][0];
    let kind = c.g[S1][0];
    Addr(c.g[S0][0] as u32).off(0xc).sw(c, kind);
    Ok(())
}

/// Kinds 94..97: relative camera at a fixed angle (`.lit4` at `gp - lit`).
pub(crate) fn set_algo_relative_fixed(c: &mut Cpu, cycles: u64, alloc_ret: u32, lit: u32) -> Flow {
    c.cycles += cycles;
    set_algo_alloc(c, 0x3c0, 0x4aa8, alloc_ret)?;
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(lit));
    c.g[A0][0] = c.g[V0][0];
    set_algo_relative_tail(c)
}

pub(crate) fn set_algo_resume_2(c: &mut Cpu) {
    ee::s01::f_0015d078__entries(c)
}

/// `void ChaseCamController_SetAlgorithm(Controller* self, int kind, Ctx* ctx,
/// float blend)` (0x15d078; the reference calls it
/// `createChaseAlgorithmBlend`): build the chase camera algorithm of `kind`
/// (a jump table at 0x45b5c0 maps the kind to its constructor), give it the
/// controller's target, push it on the controller with the blend rate (1.0 if
/// the controller's flag bit 1 is set) and activate it. Kind 91 is the
/// superpipe algorithm when the camera net is idle and the target is a line
/// bound, else the bounded one; 92 is the spline camera (four points from the
/// target and four floats from `ctx`); 33..40 and 94..97 are relative cameras.
///
/// Resume entries (`__entries`) go to the generated twin.
pub fn chase_cam_controller_set_algorithm(c: &mut Cpu) {
    if c.pc != 0x0015_d078 {
        return set_algo_resume_2(c);
    }
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        c.g[S0][0] = 0;
        frame.save(c, S5, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S6, 0x50);
        c.g[S5][0] = c.g[A2][0];
        frame.save_fpr(c, 21, 0xc8);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xc0);
        let flags = Addr(c.g[S6][0] as u32).off(SET_ALGO_CTRL_FLAGS).lw(c);
        c.g[V0][0] = flags & 0x2;
        let keep_blend = c.g[V0][0] == 0;
        c.mov_s(21, 12);
        if !keep_blend {
            c.cycles += 2;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 21);
        }

        // 0x15d0cc: kinds past 0x61 leave algo = 0.
        c.cycles += 3;
        c.g[V0][0] = (c.g[S1][0] < 0x62) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x46_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4a40));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let direct = |addr: u32, f: GuestFn| Target::Direct(Callee { addr, f });
            match target {
                0x0015_d0f0 => set_algo_superpipe_or_bounded(c)?,
                0x0015_d1a0 => set_algo_spline(c)?,
                0x0015_d23c => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4cd8,
                    0x0015_d254,
                    direct(0x0016_16a8, ee::s01::f_001616a8),
                    0x0015_d25c,
                )?,
                0x0015_d264 => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4cb8,
                    0x0015_d27c,
                    Target::Table(0x0016_89c8),
                    0x0015_d284,
                )?,
                0x0015_d28c => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4c90,
                    0x0015_d2a4,
                    Target::Table(0x0016_8cd8),
                    0x0015_d2ac,
                )?,
                0x0015_d2b4 => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4c70,
                    0x0015_d2cc,
                    direct(0x0016_0438, ee::s01::f_00160438),
                    0x0015_d2d4,
                )?,
                0x0015_d2dc => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4c50,
                    0x0015_d2f4,
                    Target::Table(0x0016_82b8),
                    0x0015_d2fc,
                )?,
                0x0015_d304 => set_algo_direction(c, 0)?,
                0x0015_d328 => set_algo_direction(c, 1)?,
                0x0015_d34c => set_algo_direction(c, 2)?,
                0x0015_d370 => set_algo_direction(c, 3)?,
                0x0015_d394 => set_algo_direction(c, 4)?,
                0x0015_d3b8 => set_algo_direction(c, 5)?,
                0x0015_d3dc => set_algo_direction(c, 6)?,
                0x0015_d400 => set_algo_direction(c, 7)?,
                0x0015_d430 => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4af0,
                    0x0015_d448,
                    Target::Table(0x0016_8650),
                    0x0015_d450,
                )?,
                0x0015_d458 => set_algo_simple(
                    c,
                    10,
                    0x3a0,
                    0x4ac8,
                    0x0015_d470,
                    direct(0x0016_0760, ee::s01::f_00160760),
                    0x0015_d478,
                )?,
                0x0015_d480 => {
                    // kinds 33..40: angle = (kind - 33) * pi/4
                    c.cycles += 14;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x21));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6cd4));
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    c.g[A1][0] = 0x46_0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4aa8));
                    c.g[A0][0] = 0x3c0;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.mul_s(20, 20, 0);
                    call_ptr(c, 0x0031_7d70, 0x0015_d4ac)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.mov_s(12, 20);
                    set_algo_relative_tail(c)?
                }
                0x0015_d4b8 => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4d28,
                    0x0015_d4d0,
                    Target::Table(0x0017_6a68),
                    0x0015_d4d8,
                )?,
                0x0015_d4e0 => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4d00,
                    0x0015_d4f8,
                    Target::Table(0x0017_7068),
                    0x0015_d500,
                )?,
                0x0015_d508 => set_algo_simple(
                    c,
                    10,
                    0x390,
                    0x4d50,
                    0x0015_d520,
                    Target::Table(0x0017_6d68),
                    0x0015_d528,
                )?,
                0x0015_d530 => set_algo_simple(
                    c,
                    10,
                    0x3a0,
                    0x4a88,
                    0x0015_d548,
                    Target::Table(0x0017_7e50),
                    0x0015_d550,
                )?,
                0x0015_d558 => {
                    // kind 67: no algorithm
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                }
                0x0015_d560 => {
                    // kind 68: post-race camera, given the controller's target
                    c.cycles += 11;
                    set_algo_alloc(c, 0x3b0, 0x4a68, 0x0015_d578)?;
                    let tgt = Addr(c.g[S6][0] as u32).off(SET_ALGO_CTRL_TARGET).lw(c);
                    c.g[A1][0] = tgt;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0017_89e8, 0x0015_d584)?;
                    c.g[S0][0] = c.g[V0][0];
                }
                0x0015_d58c => set_algo_relative_fixed(c, 9, 0x0015_d5a4, 0x6cd0)?,
                0x0015_d5b0 => set_algo_relative_fixed(c, 9, 0x0015_d5c8, 0x6ccc)?,
                0x0015_d5d4 => set_algo_relative_fixed(c, 9, 0x0015_d5ec, 0x6cc8)?,
                0x0015_d5f8 => set_algo_relative_fixed(c, 8, 0x0015_d610, 0x6cc4)?,
                0x0015_d628 => {}
                _ => {
                    c.jump_out(target, 0x0015_d078, 0x0015_d694);
                    return Ok(());
                }
            }
        }

        // 0x15d628: algo->setTarget(ctrl->target); push; algo->activate().
        c.cycles += 27;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = Addr(c.g[S6][0] as u32).off(SET_ALGO_CTRL_TARGET).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x60).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x64).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0015_d640)?;
        c.g[A0][0] = c.g[S6][0];
        c.mov_s(12, 21);
        c.g[A1][0] = c.g[S0][0];
        Callee {
            addr: 0x0016_1e58,
            f: ee::s01::f_00161e58,
        }
        .call(c, 0x0015_d650)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x24).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0015_d664)?;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0xc8);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// Kind 91 (0x15d0f0): the superpipe camera when the camera net is idle
/// (replay count at `world->0x84->0x28` below 10, or none) and the target's
/// kind (+0x28) is 2; otherwise the bounded camera, which also keeps the
/// controller context (`ctx`) at +0x80.
pub(crate) fn set_algo_superpipe_or_bounded(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[A1][0] = 0;
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
    }
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.g[A1][0] = 0x46_0000;
    } else {
        c.cycles += 5;
        c.g[S2][0] = Addr(c.g[S5][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = 0x2;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x28).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            c.g[A1][0] = 0x46_0000;
        } else {
            // superpipe: line start at sp+0, line end at sp+0x10
            c.cycles += 20;
            c.g[A1][0] = 0x46_0000;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4db0));
            c.g[A0][0] = 0x3b0;
            call_ptr(c, 0x0031_7d70, 0x0015_d140)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x0017_0240, 0x0015_d150)?;
            c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0017_07b0, 0x0015_d160)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x0017_8520, 0x0015_d170)?;
            c.g[S0][0] = c.g[V0][0];
            return Ok(());
        }
    }
    // bounded
    c.cycles += 10;
    c.g[A0][0] = 0x90;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4d88));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x0015_d18c)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0017_4190, 0x0015_d194)?;
    c.g[S0][0] = c.g[V0][0];
    let ctx = c.g[S5][0];
    Addr(c.g[S0][0] as u32).off(0x80).sw(c, ctx);
    Ok(())
}

/// Kind 92 (0x15d1a0): spline camera. Four points of the target
/// (`CameraBoundObj_GetWorldPoint(&p, target, i)`) and four floats of `ctx`.
pub(crate) fn set_algo_spline(c: &mut Cpu) -> Flow {
    c.cycles += 39;
    c.g[A1][0] = 0x46_0000;
    c.g[A3][0] = 0;
    c.g[S0][0] = Addr(c.g[S5][0] as u32).off(0x8).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4d68));
    c.g[A0][0] = 0x3d0;
    c.g[A2][0] = 0x2000_0000;
    call_ptr(c, 0x0031_7d70, 0x0015_d1bc)?;
    c.g[S4][0] = c.g[V0][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x0017_0d20, 0x0015_d1d0)?;
    c.g[S3][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 1;
    call_ptr(c, 0x0017_0d20, 0x0015_d1e4)?;
    c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = 2;
    call_ptr(c, 0x0017_0d20, 0x0015_d1f8)?;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x30));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = 3;
    call_ptr(c, 0x0017_0d20, 0x0015_d20c)?;
    let ctx = c.g[S5][0] as u32;
    c.lwc1(15, ctx.wrapping_add(0x10));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(12, ctx.wrapping_add(0x18));
    c.g[A2][0] = c.g[S3][0];
    c.lwc1(13, ctx.wrapping_add(0x14));
    c.g[A3][0] = c.g[S2][0];
    c.lwc1(14, ctx.wrapping_add(0x1c));
    c.g[T0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[SP][0];
    Callee {
        addr: 0x0016_1060,
        f: ee::s01::f_00161060,
    }
    .call(c, 0x0015_d234)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(())
}

// ---------------------------------------------------------------------------
// `GameCamera_ResetAllChaseAlgorithms` (0x15dae0), `cCamera_snapChaseControllers`.
//
// `void GameCamera_ResetAllChaseAlgorithms()`: snap the chase controllers of
// the race manager's two camera slots. Slot 0 always gets its algorithm's
// snap call; slot 1 only when the rider count is at least 2.

pub fn game_camera_reset_all_chase_algorithms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V0][0] = world;
        let cameras = Addr((world as u32).wrapping_add(0x84)).lw(c);
        c.g[V1][0] = cameras;
        c.g[S1][0] = Addr((cameras as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xa8)).lw(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0015_db1c)?;
        // Read the callee-saved registers after the call, as the body does.
        c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
        let only_slot0 = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if !only_slot0 {
            c.cycles += 8;
            let riders = c.g[S1][0] as u32;
            c.g[V0][0] = Addr(riders.wrapping_add(0x8)).lw(c);
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0015_db44)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// `world` at `gp - 0x848`, camera manager at `world + 0x84`, rider table
    /// at `camera + 0x84`.
    pub(crate) fn world_chain(c: &mut Cpu, world: u32, cameras: u32, riders: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, cameras);
        c.bus.write32(cameras + 0x84, riders);
    }

    // -----------------------------------------------------------------------

    #[test]
    fn game_camera_set_collision_flag4a4_matches() {
        check(
            ee::s01::f_0015dac0__generated,
            game_camera_set_collision_flag4a4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15_dac0, 0);
                c.g[A0][0] = r.next_u32() as u64;
                world_chain(c, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 4, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + 0x4a4, r.next_u32());
            },
        );
    }

    #[test]
    fn game_camera_reset_all_matches() {
        check(
            ee::s01::f_0015db58__generated,
            game_camera_reset_all,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15db, 8);
                let w = DATA + 0x1000;
                let x = DATA + 0x2000;
                let rm = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x848, w);
                c.bus.write32(w + 0x84, x);
                c.bus.write32(x + 0x84, rm);
                c.bus.write32(rm, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus.write32(rm + 4, DATA + 0x4000);
                c.bus.write32(rm + 8, DATA + 0x5000);
                for cam in [DATA + 0x4000, DATA + 0x5000] {
                    c.bus.write32(cam + 0xa8, DATA + 0x6000);
                    c.bus.write32(DATA + 0x6000 + 0x14, DATA + 0x7000);
                    c.bus.write16(DATA + 0x7000 + 0x20, 0);
                    c.bus.write32(DATA + 0x7000 + 0x24, FAKE);
                }
                c.g[A0][0] = sx(DATA);
                c.bus.write32(0x004e_5830, r.next_u32());
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use std::ptr::addr_of_mut;

    /// Fakes act only when the machine holds this word at `CTRL`.
    pub(crate) const OC419_MAGIC: u32 = 0x0c41_9a1c;

    pub(crate) const OC419_CTRL: u32 = DATA + 0x7_0000;

    /// The fake constructors give their object this vtable (slot 0x20:
    /// `activate`, 0x60: `setTarget`; both land in the miss fakes).
    pub(crate) const OC419_VT: u32 = DATA + 0x7_1000;

    pub(crate) const OC419_WORLD: u32 = DATA + 0x6_0000;

    pub(crate) const OC419_REPLAY: u32 = DATA + 0x6_0100;

    pub(crate) const OC419_NET: u32 = DATA + 0x6_0200;

    pub(crate) const OC419_SELF: u32 = DATA + 0x6_1000;

    pub(crate) const OC419_CTX: u32 = DATA + 0x6_2000;

    pub(crate) const OC419_TARGET: u32 = DATA + 0x6_3000;

    /// A call-logging fake for a callee's override slot, active only on
    /// machines with the gate word. `$ctor`: a constructor, which returns its
    /// block (a0) with the test vtable at +0x10.
    macro_rules! oc419_fake {
        ($name:ident, $addr:expr, $gen:path, $ctor:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC419_CTRL) != OC419_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let obj = c.g[A0][0];
                let ra = c.g[RA][0] as u32;
                fakes::fake_call(c, $addr);
                if $ctor && c.pc == ra {
                    c.g[V0][0] = obj;
                    c.bus.write32((obj as u32).wrapping_add(0x10), OC419_VT);
                }
            }
        };
    }

    oc419_fake!(
        fake_alloc,
        0x0031_7d70,
        ee::s10::f_00317d70__generated,
        false
    );

    oc419_fake!(
        fake_line_start,
        0x0017_0240,
        ee::s02::f_00170240__generated,
        false
    );

    oc419_fake!(
        fake_line_end,
        0x0017_07b0,
        ee::s02::f_001707b0__generated,
        false
    );

    oc419_fake!(
        fake_world_point,
        0x0017_0d20,
        ee::s02::f_00170d20__generated,
        false
    );

    oc419_fake!(
        fake_push,
        0x0016_1e58,
        ee::s01::f_00161e58__generated,
        false
    );

    oc419_fake!(
        fake_superpipe,
        0x0017_8520,
        ee::s02::f_00178520__generated,
        true
    );

    oc419_fake!(
        fake_bounded,
        0x0017_4190,
        ee::s02::f_00174190__generated,
        true
    );

    oc419_fake!(fake_idle, 0x0016_16a8, ee::s01::f_001616a8__generated, true);

    oc419_fake!(
        fake_midright,
        0x0016_89c8,
        ee::s02::f_001689c8__generated,
        true
    );

    oc419_fake!(
        fake_midleft,
        0x0016_8cd8,
        ee::s02::f_00168cd8__generated,
        true
    );

    oc419_fake!(
        fake_manual,
        0x0016_0438,
        ee::s01::f_00160438__generated,
        true
    );

    oc419_fake!(fake_lazy, 0x0016_82b8, ee::s02::f_001682b8__generated, true);

    oc419_fake!(
        fake_direction,
        0x0017_74c0,
        ee::s02::f_001774c0__generated,
        true
    );

    oc419_fake!(
        fake_lookback,
        0x0016_8650,
        ee::s02::f_00168650__generated,
        true
    );

    oc419_fake!(
        fake_matrix,
        0x0016_0760,
        ee::s01::f_00160760__generated,
        true
    );

    oc419_fake!(
        fake_relative,
        0x0016_0b70,
        ee::s01::f_00160b70__generated,
        true
    );

    oc419_fake!(
        fake_default2,
        0x0017_6a68,
        ee::s02::f_00176a68__generated,
        true
    );

    oc419_fake!(
        fake_default4,
        0x0017_7068,
        ee::s02::f_00177068__generated,
        true
    );

    oc419_fake!(
        fake_default3,
        0x0017_6d68,
        ee::s02::f_00176d68__generated,
        true
    );

    oc419_fake!(
        fake_spoke,
        0x0017_7e50,
        ee::s02::f_00177e50__generated,
        true
    );

    pub(crate) fn oc419_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            fake_alloc,
            fakes::prev_cell(fake_alloc),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00170240),
            fake_line_start,
            fakes::prev_cell(fake_line_start),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001707B0),
            fake_line_end,
            fakes::prev_cell(fake_line_end),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00170D20),
            fake_world_point,
            fakes::prev_cell(fake_world_point),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00161E58),
            fake_push,
            fakes::prev_cell(fake_push),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00178520),
            fake_superpipe,
            fakes::prev_cell(fake_superpipe),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00174190),
            fake_bounded,
            fakes::prev_cell(fake_bounded),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001616A8),
            fake_idle,
            fakes::prev_cell(fake_idle),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001689C8),
            fake_midright,
            fakes::prev_cell(fake_midright),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00168CD8),
            fake_midleft,
            fakes::prev_cell(fake_midleft),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00160438),
            fake_manual,
            fakes::prev_cell(fake_manual),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001682B8),
            fake_lazy,
            fakes::prev_cell(fake_lazy),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001774C0),
            fake_direction,
            fakes::prev_cell(fake_direction),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00168650),
            fake_lookback,
            fakes::prev_cell(fake_lookback),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00160760),
            fake_matrix,
            fakes::prev_cell(fake_matrix),
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00160B70),
            fake_relative,
            fakes::prev_cell(fake_relative),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00176A68),
            fake_default2,
            fakes::prev_cell(fake_default2),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00177068),
            fake_default4,
            fakes::prev_cell(fake_default4),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00176D68),
            fake_default3,
            fakes::prev_cell(fake_default3),
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00177E50),
            fake_spoke,
            fakes::prev_cell(fake_spoke),
        );
    }

    /// The jump table entry (the block address) the game stores for `kind`.
    pub(crate) fn oc419_table_entry(kind: u32) -> u32 {
        match kind {
            91 => 0x15d0f0,
            92 => 0x15d1a0,
            7 => 0x15d23c,
            9 => 0x15d264,
            10 => 0x15d28c,
            11 => 0x15d2b4,
            13 => 0x15d2dc,
            18 => 0x15d304,
            19 => 0x15d328,
            20 => 0x15d34c,
            21 => 0x15d370,
            22 => 0x15d394,
            23 => 0x15d3b8,
            24 => 0x15d3dc,
            25 => 0x15d400,
            32 => 0x15d430,
            41 => 0x15d458,
            33..=40 => 0x15d480,
            2 | 60 => 0x15d4b8,
            1 | 62 => 0x15d4e0,
            66 => 0x15d530,
            67 => 0x15d558,
            68 => 0x15d560,
            94 => 0x15d58c,
            95 => 0x15d5b0,
            96 => 0x15d5d4,
            97 => 0x15d5f8,
            0 | 3..=6 | 8 | 12 | 14..=17 | 26..=31 | 42..=59 | 61 | 63..=65 | 74 => 0x15d508,
            _ => 0x15d628,
        }
    }

    /// `ChaseCamController_SetAlgorithm`: every kind 0..97 through the jump
    /// table (seeds 98/99: kind 0x62 and -1, which build nothing), three
    /// camera-net/target setups for kind 91 (superpipe, bounded because the
    /// target is not a line, bounded because the net is busy / not), random
    /// blend and the instant-blend flag. Constructors, allocator, the point
    /// getters and `PushAlgorithm` are gated fakes that log their registers
    /// (and sometimes unwind); the spline and post-race constructors
    /// (no override slot) and the virtual calls run as they are.
    #[test]
    fn chase_cam_controller_set_algorithm_matches() {
        covered(
            ee::s01::f_0015d078__generated,
            chase_cam_controller_set_algorithm,
            300,
            &|c, seed| {
                oc419_install_fakes();
                let mut r = machine(c, seed, 0x15_d078, 12);
                c.pc = 0x0015_d078;
                c.bus.write32(OC419_CTRL, OC419_MAGIC);
                // vtable: slot 0x20 activate, 0x60 setTarget
                c.bus.write16(OC419_VT + 0x20, 4);
                c.bus.write32(OC419_VT + 0x24, FAKE + 0x10);
                c.bus.write16(OC419_VT + 0x60, 8);
                c.bus.write32(OC419_VT + 0x64, FAKE + 0x20);
                for k in 0..98 {
                    c.bus
                        .write32(SET_ALGO_JUMP_TABLE + 4 * k, oc419_table_entry(k));
                }
                // world -> replay -> net count
                c.bus.write32(GAME_GP - 0x848, OC419_WORLD);
                c.bus.write32(OC419_WORLD + 0x84, OC419_REPLAY);
                c.bus.write32(OC419_REPLAY + 0x28, OC419_NET);
                let pass = seed / 100;
                let (net, target_kind) = match pass {
                    0 => (0, 2),
                    1 => (0, 1),
                    _ if r.next_u32() % 2 == 0 => (5, 2),
                    _ => (20, 2),
                };
                c.bus.write32(OC419_NET, net);
                c.bus.write32(OC419_TARGET + 0x28, target_kind);
                // controller and context
                let flags = (r.next_u32() & !2) | if r.next_u32() % 2 == 0 { 2 } else { 0 };
                c.bus.write32(OC419_SELF + SET_ALGO_CTRL_FLAGS, flags);
                c.bus
                    .write32(OC419_SELF + SET_ALGO_CTRL_TARGET, r.next_u32());
                c.bus.write32(OC419_CTX + 8, OC419_TARGET);
                for off in [0x10, 0x14, 0x18, 0x1c] {
                    let v = r.f32(10.0);
                    c.bus.write32(OC419_CTX + off, v);
                }
                // .lit4: pi/4 and the four fixed relative angles
                for (off, v) in [
                    (0x6cd4u32, 0.7853982f32),
                    (0x6cd0, -0.3926991),
                    (0x6ccc, 0.3926991),
                    (0x6cc8, -2.3561945),
                    (0x6cc4, 2.3561945),
                ] {
                    c.bus.write32(GAME_GP - off, v.to_bits());
                }
                let kind = seed % 100;
                c.g[A0][0] = OC419_SELF as u64;
                c.g[A1][0] = if kind == 99 { u64::MAX } else { kind as u64 };
                c.g[A2][0] = OC419_CTX as u64;
                c.f[12] = r.f32(4.0);
            },
        );
    }

    #[test]
    fn game_camera_reset_all_chase_algorithms_matches() {
        covered(
            ee::s01::f_0015dae0__generated,
            game_camera_reset_all_chase_algorithms,
            200,
            &|c, seed| {
                machine(c, seed, 0x0015_dae0, 8);
                c.pc = 0x0015_dae0;
                let w = DATA + 0x1000;
                let x = DATA + 0x2000;
                let rm = DATA + 0x3000;
                c.bus.write32(GAME_GP - 0x848, w);
                c.bus.write32(w + 0x84, x);
                c.bus.write32(x + 0x84, rm);
                c.bus.write32(rm, [0u32, 1, 2][(seed % 3) as usize]);
                c.bus.write32(rm + 4, DATA + 0x4000);
                c.bus.write32(rm + 8, DATA + 0x5000);
                for cam in [DATA + 0x4000, DATA + 0x5000] {
                    c.bus.write32(cam + 0xa8, DATA + 0x6000);
                    c.bus.write32(DATA + 0x6000 + 0x14, DATA + 0x7000);
                    c.bus.write16(DATA + 0x7000 + 0x20, 0);
                    c.bus.write32(DATA + 0x7000 + 0x24, FAKE);
                }
            },
        );
    }
}
