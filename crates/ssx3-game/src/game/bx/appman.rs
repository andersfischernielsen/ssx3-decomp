//! `src/bx/appman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::render::particle::this_with_delta;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_6cd0, app_ctor, "App_ctor"),
    (0x0031_6ef8, app_set_next_state, "App_SetNextState"),
    (0x0031_7400, app_v02, "App_v02"),
    (0x0031_7458, ps2_exec_man_ctor, "PS2ExecMan_ctor"),
    (0x0031_7520, app_get_exec_man, "App_GetExecMan"),
    (0x0031_7530, app_load_progress_init, "App_LoadProgressInit"),
    (
        0x0031_7550,
        app_load_progress_set_target,
        "App_LoadProgressSetTarget",
    ),
    (
        0x0031_7600,
        app_load_progress_complete,
        "App_LoadProgressComplete",
    ),
    (
        0x0031_74a8,
        ps2_exec_man_invoke_callback,
        "PS2ExecMan_InvokeCallback",
    ),
];

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

/// `void App_SetNextState(App *this, u32 next)` (0x316ef8,
/// `cAppMan_setNextModule`): `this->next = next` at `+4`.
pub fn app_set_next_state(c: &mut Cpu) {
    c.cycles += 2;
    const NEXT: u32 = 0x4;
    Addr(abi::arg(c, 0)).off(NEXT).sw(c, c.g[A1][0]);
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
// App and strings (s10).

/// `void App_v02(App* self)` (0x317400): let the app object's screen update,
/// then, if it is still there, tell it the app finished.
pub fn app_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(this).off(0x8).lw(c);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x18).lh(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0031_7428);
        c.g[A0][0] = this_with_delta(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x0031_7428)?;
        c.g[A2][0] = Addr(this).off(0x8).lw(c);
        let gone = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !gone {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, A2)).off(0x8).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0031_7448);
            c.g[A0][0] = this_with_delta(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x0031_7448)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ PS2ExecMan_InvokeCallback

/// `void PS2ExecMan_InvokeCallback(Callback* cb, int notify)` (0x3174a8,
/// `cExecutionMan::checkHalt`): call `cb->func(cb->arg)` if a function is
/// set; then, when `notify` is nonzero, tell the owner at `cb+8` through
/// its virtual method at +0x70 (`this` adjusted by the slot's delta, so
/// relative to `cb`).
pub fn ps2_exec_man_invoke_callback(c: &mut Cpu) {
    /// `Callback` fields.
    const FUNC: u32 = 0x0;
    const ARG: u32 = 0x4;
    const OWNER: u32 = 0x8;
    /// The owner's vtable slot `{delta: i16, method}` (embedded at +0x70).
    const SLOT_DELTA: u32 = 0x70;
    const SLOT_METHOD: u32 = 0x74;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let cb = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let func = Addr(cb).off(FUNC).lw(c);
        c.g[V0][0] = func;
        c.g[S1][0] = c.g[A1][0];
        if func == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(cb).off(ARG).lw(c);
            call_ptr(c, func as u32, 0x0031_74d0)?;
            // `notify` is compared as the full 64-bit register.
            if c.g[S1][0] != 0 {
                c.cycles += 5;
                let owner = Addr(cb).off(OWNER).lw(c);
                c.g[V0][0] = owner;
                c.g[A0][0] = Addr(owner as u32).off(SLOT_DELTA).lh(c);
                c.g[V1][0] = Addr(owner as u32).off(SLOT_METHOD).lw(c);
                let method = r32(c, V1);
                c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
                call_ptr(c, method, 0x0031_74ec)?;
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::vtable_2 as vtable;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;
    use ssx3_ee::s09;
    use ssx3_ee::s10;

    pub(crate) const SEEDS: u32 = 400;

    pub(crate) fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt)
    }

    /// A pointer into test RAM (16-byte aligned).
    pub(crate) fn ptr(r: &mut Rng, region: u32) -> u32 {
        DATA + region * 0x10_0000 + (r.next_u32() & 0x7fff0)
    }

    #[test]
    fn ps2_exec_man_ctor_matches() {
        check(
            s10::f_00317458__generated,
            ps2_exec_man_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 8);
                scramble(c, &mut r);
                let this = ptr(&mut r, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn app_load_progress_init_matches() {
        check(
            s10::f_00317530__generated,
            app_load_progress_init,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 9);
                scramble(c, &mut r);
                let p = ptr(&mut r, 0);
                c.g[A0][0] = sx(p);
                c.f[12] = r.edge_f32();
                for off in (0..0x20).step_by(4) {
                    c.bus.write32(p + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn app_load_progress_complete_matches() {
        check(
            s10::f_00317600__generated,
            app_load_progress_complete,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 10);
                scramble(c, &mut r);
                let p = ptr(&mut r, 0);
                c.g[A0][0] = sx(p);
                for off in (0..0x20).step_by(4) {
                    c.bus.write32(p + off, r.next_u32());
                }
            },
        );
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
    fn app_set_next_state_matches() {
        check(
            ssx3_ee::s10::f_00316ef8__generated,
            app_set_next_state,
            100,
            &|c, seed| {
                let mut r = Rng(0x2017 ^ seed as u64);
                scramble(c, &mut r);
                let this = ptr(&mut r, 0);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn app_get_exec_man_matches() {
        check(
            ssx3_ee::s10::f_00317520,
            app_get_exec_man,
            100,
            &|c, seed| {
                let mut r = Rng(0x2015 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let app = DATA + 0x100;
                c.bus.write32(GAME_GP + 0x2a74, app);
                c.bus.write32(app + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn app_load_progress_set_target_matches() {
        check(
            ssx3_ee::s10::f_00317550,
            app_load_progress_set_target,
            300,
            &|c, seed| {
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
            },
        );
    }

    // ------------------------------------------------------------------
    // App and strings.

    #[test]
    fn app_v02_matches() {
        check(
            ee::s10::f_00317400__generated,
            app_v02,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x317400, 2);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                let vt = DATA + 0x300;
                clear(c, this, 0x100);
                clear(c, obj, 0x200);
                clear(c, vt, 0xf00);
                c.bus.write32(this + 8, if seed % 3 == 0 { 0 } else { obj });
                c.bus.write32(obj + 8, vt);
                vtable(c, vt, &[(0x18, 0, FAKE), (0x8, 0, FAKE + 0x10)]);
                c.g[A0][0] = sx(this);
            },
        );
    }
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::{elf_image, fill};

    // ------------------------------------------------ PS2ExecMan_InvokeCallback

    /// `PS2ExecMan_InvokeCallback`: a callback with or without a function,
    /// `notify` zero, nonzero in the low word, or nonzero only in the upper
    /// word (64-bit compare); both calls go to fakes (some unwind).
    #[test]
    fn ps2_exec_man_invoke_callback_matches() {
        covered(
            ee::s10::f_003174a8__generated,
            ps2_exec_man_invoke_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_74a8, 2);
                c.pc = 0x0031_74a8;
                let cb = crate::difftest::fakes::ptr(&mut r);
                let owner = cb + 0x100;
                c.bus.write32(cb, if seed % 4 == 0 { 0 } else { FAKE });
                c.bus.write32(cb + 4, r.next_u32());
                c.bus.write32(cb + 8, owner);
                c.bus
                    .write16(owner + 0x70, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(owner + 0x74, FAKE + 0x10);
                c.g[A0][0] = sx(cb);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => r.next_u32() as u64 | 1,
                    _ => (r.next_u32() as u64 | 1) << 32,
                };
            },
        );
    }
}
