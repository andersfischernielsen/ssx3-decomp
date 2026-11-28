//! `src/bx/appman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::render::particle::this_with_delta;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
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
    (
        0x0031_7328,
        app_invoke_exec_callback,
        "App_InvokeExecCallback",
    ),
    (0x0031_7348, app_vsync_tick, "App_VSyncTick"),
    (0x0031_7500, app_vsync_callback, "App_VSyncCallback"),
    (0x0031_6d28, app_dtor, "App_dtor"),
    (0x0031_6d60, app_run_main_loop, "App_Run_MainLoop"),
    (0x0031_6f00, app_run_module_loop, "App_RunModuleLoop"),
    (0x0031_7308, app_set_exec_callback, "App_SetExecCallback"),
    (
        0x0031_75a0,
        app_load_progress_update,
        "App_LoadProgressUpdate",
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

// ---------------------------------------------------------------------------
// 0x00317328 App_InvokeExecCallback

/// `PS2ExecMan_InvokeCallback(man, notify)`.
pub(crate) const PS2_EXEC_MAN_INVOKE_CALLBACK: Callee = Callee {
    addr: 0x0031_74a8,
    f: ee::s10::f_003174a8,
};

/// `void App_InvokeExecCallback(App* this)` (0x317328): run the exec
/// manager's (`this+8`) pending callback, with notification
/// (`PS2ExecMan_InvokeCallback(man, 1)`).
pub fn app_invoke_exec_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
        PS2_EXEC_MAN_INVOKE_CALLBACK.call(c, 0x0031_733c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `App_VSyncTick(App* app)` (0x317348, appman.cpp): the application
/// manager's per-vsync tick. Bumps the vsync counter (+0x18), adds the step
/// (+0x24, ticks per vsync) to the fractional accumulator (+0x28), keeps the
/// fraction and calls the app's `Tick` method (vtable +0x5c, slot 0x38) once
/// per whole tick crossed; then calls the current sub-object's (+0x8) slot
/// 0x28 (its per-vsync update).
///
/// The tick count is `trunc(acc + step) - trunc(acc)`; the loop runs while
/// the count, decremented first, is not -1.
pub fn app_vsync_tick(c: &mut Cpu) {
    const COUNTER: u32 = 0x18;
    const STEP: u32 = 0x24;
    const ACCUM: u32 = 0x28;
    const VTABLE: u32 = 0x5c;
    const SUB: u32 = 0x8;
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = u64::MAX;
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let app = Addr(r32(c, S1));
        let accum = app.off(ACCUM).read_fpu(c);
        let step = app.off(STEP).read_fpu(c);
        let whole_before = accum.to_i32();
        c.f[2] = whole_before as u32;
        c.g[A0][0] = sx(whole_before as u32);
        let count = app.off(COUNTER).lw(c);
        let count = sx((count as u32).wrapping_add(1));
        c.g[V0][0] = count;
        app.off(COUNTER).sw(c, count);
        let sum = accum + step;
        let whole_after = sum.to_i32();
        c.f[1] = whole_after as u32;
        c.g[V1][0] = sx(whole_after as u32);
        let ticks = (whole_after as u32).wrapping_sub(whole_before as u32);
        let ticks_f = FpuF32::from_i32(ticks as i32);
        c.f[1] = ticks_f.0;
        let frac = sum - ticks_f;
        abi::set_fpr(c, 0, frac);
        c.g[S0][0] = sx(ticks.wrapping_sub(1));
        app.off(ACCUM).write_fpu(c, frac);
        if c.g[S0][0] != c.g[A1][0] {
            // One `Tick` call per whole tick crossed.
            c.cycles += 2;
            c.g[S2][0] = u64::MAX;
            c.g[V0][0] = Addr(r32(c, S1)).off(VTABLE).lw(c);
            loop {
                c.cycles += 7;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                let vt = Addr(r32(c, V0));
                let delta = vt.off(0x38).lh(c);
                c.g[A0][0] = delta;
                let method = vt.off(0x3c).lw(c);
                c.g[V1][0] = method;
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(delta as u32));
                call_ptr(c, method as u32, 0x0031_73c4)?;
                if c.g[S0][0] == c.g[S2][0] {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S1)).off(VTABLE).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_73b0);
                }
            }
        }
        // The sub-object's per-vsync update (its vtable at +0x8, slot 0x28).
        c.cycles += 12;
        let sub = Addr(r32(c, S1)).off(SUB).lw(c);
        c.g[A1][0] = sub;
        let vt = Addr(sub as u32).off(0x8).lw(c);
        c.g[V0][0] = vt;
        let delta = Addr(vt as u32).off(0x28).lh(c);
        c.g[A0][0] = delta;
        let method = Addr(vt as u32).off(0x2c).lw(c);
        c.g[V1][0] = method;
        c.g[A0][0] = sx((sub as u32).wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x0031_73e4)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// App_VSyncCallback (0x317500, appman.cpp)

/// `App*` singleton (gp + 0x2a74).
pub(crate) const APP_INSTANCE_GP: u32 = 0x2a74;

/// `App_VSyncTick(App*)`.
pub(crate) const APP_VSYNC_TICK: Callee = Callee {
    addr: 0x0031_7348,
    f: ee::s10::App_VSyncTick,
};

/// `void App_VSyncCallback()` (0x317500): the VBlank handler hook; runs
/// `App_VSyncTick` on the application singleton.
pub fn app_vsync_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(APP_INSTANCE_GP)).lw(c);
        frame.save_ra(c, 0);
        APP_VSYNC_TICK.call(c, 0x0031_7514)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `App_RunModuleLoop` (0x00316f00): the application main loop, a resumable dispatcher: the whole generated
/// `match pc` is kept (every register, cycle and call), calls go through `call_ptr` / `Callee`, so the
/// function also resumes natively at any block head after an unwound callee or event.
pub fn app_run_module_loop(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00316f00 => {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S2][0] = 0x1;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00316f34);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316f34)?;
                    pc = 0x00316f34;
                    continue;
                }
                0x00316f34 => {
                    c.cycles += 1;
                    pc = 0x00316f38;
                    continue;
                }
                0x00316f38 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    pc = 0x00316f3c;
                    continue;
                }
                0x00316f3c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316f50);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316f50)?;
                    pc = 0x00316f50;
                    continue;
                }
                0x00316f50 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00316f58);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x00316f58)?;
                    pc = 0x00316f58;
                    continue;
                }
                0x00316f58 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316f70);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316f70)?;
                    pc = 0x00316f70;
                    continue;
                }
                0x00316f70 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    let t = c.g[V1][0] == c.g[S2][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        pc = 0x00316fac;
                        continue;
                    }
                    pc = 0x00316f7c;
                    continue;
                }
                0x00316f7c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x00316f94;
                        continue;
                    }
                    pc = 0x00316f84;
                    continue;
                }
                0x00316f84 => {
                    c.cycles += 2;

                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        pc = 0x00317140;
                        continue;
                    }
                    pc = 0x00316f8c;
                    continue;
                }
                0x00316f8c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00316f94;
                    continue;
                }
                0x00316f94 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x00316ff4;
                        continue;
                    }
                    pc = 0x00316f9c;
                    continue;
                }
                0x00316f9c => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                        pc = 0x003170bc;
                        continue;
                    }
                    pc = 0x00316fa4;
                    continue;
                }
                0x00316fa4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00316fac;
                    continue;
                }
                0x00316fac => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00316fc4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316fc4)?;
                    pc = 0x00316fc4;
                    continue;
                }
                0x00316fc4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2;
                    if t {
                        pc = 0x00316fe4;
                        continue;
                    }
                    pc = 0x00316fcc;
                    continue;
                }
                0x00316fcc => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00316fdc);
                    Callee {
                        addr: 0x00317600,
                        f: ee::s10::App_LoadProgressComplete,
                    }
                    .call(c, 0x00316fdc)?;
                    pc = 0x00316fdc;
                    continue;
                }
                0x00316fdc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00316fe4;
                    continue;
                }
                0x00316fe4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00316fec);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                    Callee {
                        addr: 0x003175a0,
                        f: ee::s10::App_LoadProgressUpdate,
                    }
                    .call(c, 0x00316fec)?;
                    pc = 0x00316fec;
                    continue;
                }
                0x00316fec => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00316ff4;
                    continue;
                }
                0x00316ff4 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0031700c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0031700c)?;
                    pc = 0x0031700c;
                    continue;
                }
                0x0031700c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00317014;
                    continue;
                }
                0x00317014 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0031701c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x0031701c)?;
                    pc = 0x0031701c;
                    continue;
                }
                0x0031701c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00317034);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00317034)?;
                    pc = 0x00317034;
                    continue;
                }
                0x00317034 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0031703c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x0031703c)?;
                    pc = 0x0031703c;
                    continue;
                }
                0x0031703c => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x0031705c;
                        continue;
                    }
                    pc = 0x00317048;
                    continue;
                }
                0x00317048 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0031705c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0031705c)?;
                    pc = 0x0031705c;
                    continue;
                }
                0x0031705c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x00317068);
                    c.g[A0][0] = 0x1;
                    Callee {
                        addr: 0x00319d18,
                        f: ee::s10::MEM_FrameCheckStub_ReturnZero,
                    }
                    .call(c, 0x00317068)?;
                    pc = 0x00317068;
                    continue;
                }
                0x00317068 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00317070);
                    c.g[S1][0] = 0u64;
                    Callee {
                        addr: 0x00319d10,
                        f: ee::s10::MEM_FrameBeginStub,
                    }
                    .call(c, 0x00317070)?;
                    pc = 0x00317070;
                    continue;
                }
                0x00317070 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                    c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[A1][0] as u32;
                    c.g[RA][0] = sx(0x00317090);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x00317090)?;
                    pc = 0x00317090;
                    continue;
                }
                0x00317090 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00317098);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x00317098)?;
                    pc = 0x00317098;
                    continue;
                }
                0x00317098 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003170ac);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003170ac)?;
                    pc = 0x003170ac;
                    continue;
                }
                0x003170ac => {
                    c.cycles += 4;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x003170bc;
                    continue;
                }
                0x003170bc => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003170d0);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003170d0)?;
                    pc = 0x003170d0;
                    continue;
                }
                0x003170d0 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x003170d8;
                    continue;
                }
                0x003170d8 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003170e0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x003170e0)?;
                    pc = 0x003170e0;
                    continue;
                }
                0x003170e0 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x003170f8);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003170f8)?;
                    pc = 0x003170f8;
                    continue;
                }
                0x003170f8 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00317100);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x00317100)?;
                    pc = 0x00317100;
                    continue;
                }
                0x00317100 => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x00317120;
                        continue;
                    }
                    pc = 0x0031710c;
                    continue;
                }
                0x0031710c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00317120);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00317120)?;
                    pc = 0x00317120;
                    continue;
                }
                0x00317120 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw(c.g[S0][0] as u32, 0u64);
                    if t {
                        pc = 0x003172f0;
                        continue;
                    }
                    pc = 0x00317128;
                    continue;
                }
                0x00317128 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00317138);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00317138)?;
                    pc = 0x00317138;
                    continue;
                }
                0x00317138 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x003171e0;
                        continue;
                    }
                    pc = 0x00317140;
                    continue;
                }
                0x00317140 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00317148;
                    continue;
                }
                0x00317148 => {
                    c.cycles += 4;
                    c.g[AT][0] = 0x41200000;
                    c.mtc1(1, 12);
                    c.g[RA][0] = sx(0x00317158);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                    Callee {
                        addr: 0x00317530,
                        f: ee::s10::App_LoadProgressInit,
                    }
                    .call(c, 0x00317158)?;
                    pc = 0x00317158;
                    continue;
                }
                0x00317158 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00317174);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00317174)?;
                    pc = 0x00317174;
                    continue;
                }
                0x00317174 => {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[S2][0]);
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003171c8;
                        continue;
                    }
                    pc = 0x00317180;
                    continue;
                }
                0x00317180 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00317188);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x00317188)?;
                    pc = 0x00317188;
                    continue;
                }
                0x00317188 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        if back_edge_unwinds(c, 0x00317128) {
                            return Ok(());
                        }
                        pc = 0x00317128;
                        continue;
                    }
                    pc = 0x00317198;
                    continue;
                }
                0x00317198 => {
                    c.cycles += 9;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x003171bc);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003171bc)?;
                    pc = 0x003171bc;
                    continue;
                }
                0x003171bc => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003171c4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x003171c4)?;
                    pc = 0x003171c4;
                    continue;
                }
                0x003171c4 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    pc = 0x003171c8;
                    continue;
                }
                0x003171c8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003171d8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003171d8)?;
                    pc = 0x003171d8;
                    continue;
                }
                0x003171d8 => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.g[S0][0];
                        if back_edge_unwinds(c, 0x00317180) {
                            return Ok(());
                        }
                        pc = 0x00317180;
                        continue;
                    }
                    pc = 0x003171e0;
                    continue;
                }
                0x003171e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                    let t = c.g[V0][0] != c.g[S2][0];

                    if t {
                        pc = 0x0031721c;
                        continue;
                    }
                    pc = 0x003171ec;
                    continue;
                }
                0x003171ec => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        pc = 0x0031721c;
                        continue;
                    }
                    pc = 0x003171f8;
                    continue;
                }
                0x003171f8 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00317210);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00317210)?;
                    pc = 0x00317210;
                    continue;
                }
                0x00317210 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x0031724c;
                        continue;
                    }
                    pc = 0x00317218;
                    continue;
                }
                0x00317218 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                    pc = 0x0031721c;
                    continue;
                }
                0x0031721c => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        pc = 0x003172cc;
                        continue;
                    }
                    pc = 0x00317224;
                    continue;
                }
                0x00317224 => {
                    c.cycles += 2;

                    if c.g[S1][0] == 0u64 {
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        pc = 0x003172cc;
                        continue;
                    }
                    pc = 0x0031722c;
                    continue;
                }
                0x0031722c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00317244);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00317244)?;
                    pc = 0x00317244;
                    continue;
                }
                0x00317244 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        pc = 0x003172cc;
                        continue;
                    }
                    pc = 0x0031724c;
                    continue;
                }
                0x0031724c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00317254);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x00317254)?;
                    pc = 0x00317254;
                    continue;
                }
                0x00317254 => {
                    c.cycles += 6;
                    c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x2c));
                    c.mtc1(0, 0);
                    c.c_lt_s(0, 3);
                    let t = !c.fcc();
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x24));
                    if t {
                        pc = 0x003172a8;
                        continue;
                    }
                    pc = 0x0031726c;
                    continue;
                }
                0x0031726c => {
                    c.cycles += 15;
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x30));
                    c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.cvt_s_w(5, 5);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.mtc1(17, 4);
                    c.cvt_s_w(4, 4);
                    c.mul_s(3, 2, 3);
                    c.sub_s(0, 0, 2);
                    c.mul_s(0, 0, 1);
                    c.mul_s(0, 0, 5);
                    c.div_s(0, 0, 4);
                    c.add_s(0, 0, 3);
                    let t = true;
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
                    if t {
                        pc = 0x003172c4;
                        continue;
                    }
                    pc = 0x003172a8;
                    continue;
                }
                0x003172a8 => {
                    c.cycles += 7;
                    c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
                    c.cvt_s_w(0, 0);
                    c.mtc1(17, 2);
                    c.cvt_s_w(2, 2);
                    c.mul_s(1, 1, 0);
                    c.div_s(1, 1, 2);
                    c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x2c));
                    pc = 0x003172c4;
                    continue;
                }
                0x003172c4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        if back_edge_unwinds(c, 0x00316f38) {
                            return Ok(());
                        }
                        pc = 0x00316f38;
                        continue;
                    }
                    pc = 0x003172cc;
                    continue;
                }
                0x003172cc => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x003172e0);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x003172e0)?;
                    pc = 0x003172e0;
                    continue;
                }
                0x003172e0 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003172e8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x003172e8)?;
                    pc = 0x003172e8;
                    continue;
                }
                0x003172e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x00316f3c) {
                            return Ok(());
                        }
                        pc = 0x00316f3c;
                        continue;
                    }
                    pc = 0x003172f0;
                    continue;
                }
                0x003172f0 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00316f00, 0x00317308);
                    return Ok(());
                }
            }
        }
    });
}

/// `App_LoadProgressUpdate(LoadTimer *t)` (0x3175a0): `t[0] = t[5] + t[6] *
/// expf((gp.int_1c - (int)t[4]) * gp.float_14 * t[7])`, with the offsets of
/// the generated body (the Ghidra pseudo-C agrees on the fields).
pub fn app_load_progress_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x1c));
        c.mtc1(2, 12);
        c.cvt_s_w(12, 12);
        c.mul_s(12, 12, 1);
        c.mul_s(12, 12, 0);
        call_ptr(c, 0x0040_d758, 0x0031_75dc)?;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x18));
        c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x14));
        c.mul_s(1, 1, 0);
        frame.restore_ra(c, 0);
        c.add_s(2, 2, 1);
        let progress = c.g[S0][0] as u32;
        c.swc1(2, progress);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `App_dtor` (0x316d28): clears the app's exec-callback word and, when the
/// low bit of `a1` (the delete flag) is set, frees the object with the
/// runtime's delete.
pub fn app_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x490000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2418));
        let v0 = c.g[V0][0];
        Addr(c.g[A0][0] as u32).off(0x5c).sw(c, v0);
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let t = c.g[A1][0] == 0;
        Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).sw(c, 0);
        if !t {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0031_6d50)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `App_SetExecCallback` (0x317308): passes the app's callback word (`+0x8`)
/// to the PS2 exec manager.
pub fn app_set_exec_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
        call_ptr(c, 0x0031_7478, 0x0031_7318)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int App_Run_MainLoop(App *self, Exec *exec, a2, a3)` (0x00316d60): runs the
/// whole application: sets the frame time (1/60) and the tick count (60), calls
/// the exec's setup methods, loops the exec's frame methods (with
/// `EATimer_ServiceCallbacks(0)`, `App_InvokeExecCallback`) until its done
/// method says stop, runs the next module through `App_RunModuleLoop` and shuts
/// the exec down. A `match pc` body: every resume point of the generated
/// function is an arm, so a thread that unwound inside a callee continues here.
pub fn app_run_main_loop(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00316d60 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2ef8));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = 0x3c;
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.g[A1][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A2][0];
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[S0][0] = c.g[A3][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[A1][0] = 0x3c;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[A2][0] as u32;
                    c.g[RA][0] = sx(0x00316db0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x00316db0)?;
                    pc = 0x00316db0;
                    continue;
                }
                0x00316db0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316dcc);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316dcc)?;
                    pc = 0x00316dcc;
                    continue;
                }
                0x00316dcc => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316de8);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316de8)?;
                    pc = 0x00316de8;
                    continue;
                }
                0x00316de8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x00316e48;
                        continue;
                    }
                    pc = 0x00316df0;
                    continue;
                }
                0x00316df0 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316e04);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316e04)?;
                    pc = 0x00316e04;
                    continue;
                }
                0x00316e04 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00316e0c);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x00316e0c)?;
                    pc = 0x00316e0c;
                    continue;
                }
                0x00316e0c => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316e24);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316e24)?;
                    pc = 0x00316e24;
                    continue;
                }
                0x00316e24 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316e3c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316e3c)?;
                    pc = 0x00316e3c;
                    continue;
                }
                0x00316e3c => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00316e44);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00317328,
                        f: ee::s10::App_InvokeExecCallback,
                    }
                    .call(c, 0x00316e44)?;
                    pc = 0x00316e44;
                    continue;
                }
                0x00316e44 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x00316e48;
                    continue;
                }
                0x00316e48 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00316e5c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316e5c)?;
                    pc = 0x00316e5c;
                    continue;
                }
                0x00316e5c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        if back_edge_unwinds(c, 0x00316df0) {
                            return Ok(());
                        }
                        pc = 0x00316df0;
                        continue;
                    }
                    pc = 0x00316e64;
                    continue;
                }
                0x00316e64 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                    c.sw(c.g[S2][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let t = c.g[A1][0] as u32;
                    c.g[RA][0] = sx(0x00316e84);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    call_ptr(c, t, 0x00316e84)?;
                    pc = 0x00316e84;
                    continue;
                }
                0x00316e84 => {
                    c.cycles += 3;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x58), 0u64);
                    c.g[RA][0] = sx(0x00316e90);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x00316f00,
                        f: ee::s10::App_RunModuleLoop,
                    }
                    .call(c, 0x00316e90)?;
                    pc = 0x00316e90;
                    continue;
                }
                0x00316e90 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316ea4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316ea4)?;
                    pc = 0x00316ea4;
                    continue;
                }
                0x00316ea4 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316ebc);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316ebc)?;
                    pc = 0x00316ebc;
                    continue;
                }
                0x00316ebc => {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0x3;
                    if t {
                        pc = 0x00316edc;
                        continue;
                    }
                    pc = 0x00316ec8;
                    continue;
                }
                0x00316ec8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00316edc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00316edc)?;
                    pc = 0x00316edc;
                    continue;
                }
                0x00316edc => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00316d60, 0x00316ef8);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::common;

    pub(crate) use crate::game::render::particle::tests::vtable_2 as vtable;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_2;

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

    use crate::difftest::fakes::covered;

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

    // -- 0x00317328 App_InvokeExecCallback -------------------------------------

    /// `App_InvokeExecCallback` against the generated body: the real
    /// `PS2ExecMan_InvokeCallback` with no callback, or a callback and the
    /// notification virtual (both fakes, some unwinding).
    #[test]
    fn app_invoke_exec_callback_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::difftest::DATA;
        const APP: u32 = DATA + 0x100;
        const MAN: u32 = DATA + 0x200;
        const MAN_VT: u32 = DATA + 0x300;
        covered(
            ee::s10::f_00317328__generated,
            app_invoke_exec_callback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_7328, 2);
                c.pc = 0x0031_7328;
                c.g[A0][0] = sx(APP);
                c.bus.write32(APP + 8, MAN);
                c.bus.write32(MAN, if seed % 4 == 0 { 0 } else { FAKE });
                c.bus.write32(MAN + 4, r.next_u32());
                c.bus.write32(MAN + 8, MAN_VT);
                c.bus
                    .write16(MAN_VT + 0x70, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(MAN_VT + 0x74, FAKE + 0x10);
            },
        );
    }

    /// `App_VSyncTick`: an app with an accumulator 0..1 and a step 0..3.5
    /// (0..4 ticks), counter random; its vtable's `Tick` (slot 0x38) and the
    /// sub-object's slot 0x28 are fakes (logged, sometimes unwinding). Every
    /// fourth seed has a zero step (no tick). Deltas are random small
    /// even numbers.
    #[test]
    fn app_vsync_tick_matches() {
        const APP: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x2000;
        const SUBOBJ: u32 = DATA + 0x3000;
        const SUBVT: u32 = DATA + 0x4000;
        covered(
            ssx3_ee::s10::f_00317348__generated,
            app_vsync_tick,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x317348, 6);
                c.pc = 0x0031_7348;
                c.g[A0][0] = sx(APP);
                let acc = (r.next_u32() % 1000) as f32 / 1000.0;
                let step = if seed % 4 == 0 {
                    0.0
                } else {
                    (r.next_u32() % 3500) as f32 / 1000.0
                };
                c.bus.write32(APP + 0x18, r.next_u32());
                c.bus.write32(APP + 0x24, step.to_bits());
                c.bus.write32(APP + 0x28, acc.to_bits());
                c.bus.write32(APP + 0x5c, VT);
                c.bus.write32(APP + 0x8, SUBOBJ);
                c.bus.write16(
                    VT + 0x38,
                    ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1,
                );
                c.bus.write32(VT + 0x3c, FAKE);
                c.bus.write32(SUBOBJ + 0x8, SUBVT);
                c.bus.write16(
                    SUBVT + 0x28,
                    ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1,
                );
                c.bus.write32(SUBVT + 0x2c, FAKE + 0x10);
            },
        );
    }

    /// App_VSyncCallback: the real App_VSyncTick on an App singleton (as in
    /// `app_vsync_tick_matches`: zero / non-zero frame step, fake vtable
    /// methods that sometimes unwind).
    #[test]
    fn app_vsync_callback_matches() {
        use crate::difftest::fakes::covered;
        const APP: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x2000;
        const SUBOBJ: u32 = DATA + 0x3000;
        const SUBVT: u32 = DATA + 0x4000;
        covered(
            ee::s10::f_00317500__generated,
            app_vsync_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_7500, 6);
                c.pc = 0x0031_7500;
                c.bus
                    .write32(crate::difftest::GAME_GP + APP_INSTANCE_GP, APP);
                let acc = (r.next_u32() % 1000) as f32 / 1000.0;
                let step = if seed % 4 == 0 {
                    0.0
                } else {
                    (r.next_u32() % 3500) as f32 / 1000.0
                };
                c.bus.write32(APP + 0x18, r.next_u32());
                c.bus.write32(APP + 0x24, step.to_bits());
                c.bus.write32(APP + 0x28, acc.to_bits());
                c.bus.write32(APP + 0x5c, VT);
                c.bus.write32(APP + 0x8, SUBOBJ);
                c.bus.write16(
                    VT + 0x38,
                    ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1,
                );
                c.bus.write32(VT + 0x3c, FAKE);
                c.bus.write32(SUBOBJ + 0x8, SUBVT);
                c.bus.write16(
                    SUBVT + 0x28,
                    ((r.next_u32() % 64) as u16).wrapping_sub(32) & !1,
                );
                c.bus.write32(SUBVT + 0x2c, FAKE + 0x10);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine_rt;

    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as TEST_SP;
    pub(crate) use crate::game::render::particle::tests::oc1482_app_setup;

    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc1601_install;
    pub(crate) use crate::game::seg::seg_1fbe38::tests::oc1601_machine;

    pub(crate) use crate::game::wscript::wscriptman::tests::OC1577_SEEDS;

    /// `App_RunModuleLoop` (0x316f00): the application main loop (execution
    /// manager tick, load state machine, fixed-step module updates, frame
    /// rate smoothing). A resumable dispatcher: the whole `match pc` is kept.
    #[test]
    fn app_run_module_loop_matches() {
        fakes::covered(
            ee::s10::f_00316f00__generated,
            app_run_module_loop,
            300,
            &|c, seed| oc1482_app_setup(c, seed),
        );
    }

    /// The progress value from the timer's float fields and the global int.
    #[test]
    fn app_load_progress_update_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3175_a0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            c.bus.write32(GAME_GP + 0x2a74, DATA + 0x400);
            c.bus.write32(DATA + 0x400 + 0x1c, r.next_u32());
            c.bus.write32(DATA + 0x400 + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x100 + 0x10, r.next_u32());
            c.bus.write32(DATA + 0x100 + 0x14, r.edge_f32());
            c.bus.write32(DATA + 0x100 + 0x18, r.edge_f32());
            c.bus.write32(DATA + 0x100 + 0x1c, r.edge_f32());
        };
        check(
            ee::s10::f_003175a0__generated,
            app_load_progress_update,
            64,
            setup,
        );
    }

    pub(crate) const OC1577_APP: u32 = DATA + 0x6000;

    pub(crate) const OC1577_EXEC: u32 = DATA + 0x7000;

    #[test]
    fn app_dtor_matches() {
        check(
            ee::s10::f_00316d28__generated,
            app_dtor,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                machine_rt(c, seed, 0x1577_0006, 0);
                c.g[A0][0] = sx(OC1577_APP);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn app_set_exec_callback_matches() {
        check(
            ee::s10::f_00317308__generated,
            app_set_exec_callback,
            OC1577_SEEDS,
            &|c, seed| {
                elf_image(c);
                let mut r = machine_rt(c, seed, 0x1577_0007, 0);
                c.g[A0][0] = sx(OC1577_EXEC);
                c.bus.write32(OC1577_EXEC + 0x8, r.next_u32());
            },
        );
    }

    /// A vtable slot `{delta: i16, fn}` at `vt + off` whose function is the
    /// `i`-th fake method.
    pub(crate) fn oc1601_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, i: u32) {
        c.bus
            .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i);
    }

    /// Every resume point of `App_Run_MainLoop` (the start, the return address
    /// of each call and the loop head).
    pub(crate) const MAIN_LOOP_ENTRIES: [u32; 19] = [
        0x0031_6d60,
        0x0031_6db0,
        0x0031_6dcc,
        0x0031_6de8,
        0x0031_6df0,
        0x0031_6e04,
        0x0031_6e0c,
        0x0031_6e24,
        0x0031_6e3c,
        0x0031_6e44,
        0x0031_6e48,
        0x0031_6e5c,
        0x0031_6e64,
        0x0031_6e84,
        0x0031_6e90,
        0x0031_6ea4,
        0x0031_6ebc,
        0x0031_6ec8,
        0x0031_6edc,
    ];

    /// `App_Run_MainLoop` from the start and from every resume entry: the
    /// exec's vtable calls (fake methods), the gated `EATimer_ServiceCallbacks`,
    /// `App_InvokeExecCallback` and `App_RunModuleLoop`, the done-check loop
    /// (entered with `v0 == 0`), a null exec at the final shutdown, and unwinds.
    #[test]
    fn app_run_main_loop_matches() {
        oc1601_install();
        fakes::check_reaching(
            ee::s10::f_00316d60__generated,
            app_run_main_loop,
            380,
            &[
                0x0031_6db0,
                0x0031_6dcc,
                0x0031_6de8,
                0x0031_6e04,
                0x0031_6e0c,
                0x0031_6e24,
                0x0031_6e3c,
                0x0031_6e44,
                0x0031_6e5c,
                0x0031_6e84,
                0x0031_6e90,
                0x0031_6ea4,
                0x0031_6ebc,
                0x0031_6edc,
            ],
            &|c, seed| {
                let mut r = oc1601_machine(c, seed, 0x0031_6d60, 4);
                let entry = MAIN_LOOP_ENTRIES[(seed % 19) as usize];
                let (selfp, exec, vte, vt2, next, vtn) = (
                    DATA + 0x1000,
                    DATA + 0x2000,
                    DATA + 0x2400,
                    DATA + 0x2800,
                    DATA + 0x4000,
                    DATA + 0x4400,
                );
                c.bus.write32(selfp + 4, next);
                let null_exec = entry == 0x0031_6ebc && seed % 3 == 0;
                c.bus.write32(selfp + 8, if null_exec { 0 } else { exec });
                c.bus.write32(selfp + 0x5c, vt2);
                c.bus.write32(exec + 8, vte);
                c.bus.write32(next, vtn);
                for (i, off) in [0x10u32, 0x48, 0x58, 0x60, 0x20, 0x50, 0x18, 0x08]
                    .into_iter()
                    .enumerate()
                {
                    oc1601_slot(c, &mut r, vte, off, i as u32);
                }
                oc1601_slot(c, &mut r, vt2, 0x18, 8);
                oc1601_slot(c, &mut r, vt2, 0x20, 9);
                oc1601_slot(c, &mut r, vtn, 0x28, 10);
                let fl = r.f32(1.0);
                c.bus.write32(GAME_GP.wrapping_sub(0x2ef8), fl);
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = exec as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                if entry != 0x0031_6d60 {
                    c.g[S2][0] = selfp as u64;
                    c.g[S1][0] = r.next_u32() as u64;
                    c.g[S0][0] = r.next_u32() as u64;
                    c.g[V0][0] = if seed % 2 == 0 {
                        0
                    } else {
                        r.next_u32() as u64
                    };
                    if entry == 0x0031_6ec8 {
                        c.g[A2][0] = exec as u64;
                        c.g[A1][0] = 3;
                    }
                    c.bus.write64(TEST_SP, TEST_RA as u64);
                }
                c.pc = entry;
            },
        );
    }
}
