//! `src/camera/cameraalgorithms.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::ps2f::FpuF32;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::gp_2 as gp;
pub(crate) use crate::game::fe::feasyncfile::gp_17 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw;
pub(crate) use crate::game::animation::rideranimbase::sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_dac0, game_camera_set_collision_flag4a4, "GameCamera_SetCollisionFlag4a4"),
    (0x0015_db58, game_camera_reset_all, "GameCamera_ResetAll"),
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::elf_image;


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
        check(ee::s01::f_0015dac0__generated, game_camera_set_collision_flag4a4, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x15_dac0, 0);
            c.g[A0][0] = r.next_u32() as u64;
            world_chain(c, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 4, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000 + 0x4a4, r.next_u32());
        });
    }


        #[test]
    fn game_camera_reset_all_matches() {
        check(ee::s01::f_0015db58__generated, game_camera_reset_all, 40, &|c, seed| {
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
        });
    }
}
