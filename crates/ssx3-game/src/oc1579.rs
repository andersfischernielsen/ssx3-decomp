//! Staging module oc1579 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1579`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_98a0, ssx_power_off_thread, "ssx_PowerOffThread"),
    (0x0031_9910, ssx_power_off_signal_sema, "ssx_PowerOffSignalSema"),
    (0x0031_ab30, ps2exec_run_callback_sync, "PS2ExecMan_RunCallbackSync"),
    (0x0031_adb0, ssx_system_init, "ssx_SystemInit"),
    (0x0031_b008, ssx_load_exec_ps2, "ssx_LoadExecPS2"),
    (0x0031_b9b0, quat_rotate_towards, "Quat_RotateTowards"),
    (0x0031_bcb0, quat_slerp_port, "Quat_Slerp"),
    (0x0031_e0e0, c_csi_cubic_spline_sample_range, "cCSICubicSpline_SampleRange"),
];

/// `gp + off` (the game's global pointer, read at the time of the call).
fn gp_offset(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// `gp - off`.
fn gp_below(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

// ---------------------------------------------------------------------------
// 0x003198a0 ssx_PowerOffThread
// ---------------------------------------------------------------------------

/// `void ssx_PowerOffThread(int sema)` (0x3198a0): the power-off thread.
/// Waits on `sema`, then polls `sceDevctl` (disc tray) until it answers
/// non-negative, clears the status word at `sp` and calls `sceCdPowerOff`
/// until that word stays zero, then starts over. The generated tail that
/// signals the power-off semaphore is unreachable (its condition is always
/// true), so it is not ported. Never returns.
pub fn ssx_power_off_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = 0x4a0000;
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        'disc: loop {
            // WaitSema(sema)
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0042_3de0, 0x0031_98c0)?;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3ea8));
            'devctl: loop {
                // sceDevctl(...)
                c.cycles += 8;
                c.g[A1][0] = 0x4402;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                call_ptr(c, 0x0042_a538, 0x0031_98e0)?;
                let busy = (c.g[V0][0] as i64) < 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3ea8));
                if busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_98c8);
                    }
                    continue 'devctl;
                }
                c.cycles += 2;
                let status = sp(c);
                c.sw(status, 0);
                'poweroff: loop {
                    // sceCdPowerOff(sp)
                    c.cycles += 4;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0040_2398, 0x0031_98f8)?;
                    let old_zero = c.g[V0][0] == 0;
                    let status = sp(c);
                    c.g[V0][0] = c.lw(status);
                    if old_zero {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_98f0);
                        }
                        continue 'poweroff;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_98f0);
                        }
                        continue 'poweroff;
                    }
                    c.cycles += 2;
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_98b8);
                    }
                    continue 'disc;
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// 0x00319910 ssx_PowerOffSignalSema
// ---------------------------------------------------------------------------

/// `void ssx_PowerOffSignalSema(int sema)` (0x319910): forwards to
/// `SignalSema` (0x423dd0) with the same argument and returns its result.
pub fn ssx_power_off_signal_sema(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0042_3dd0), 0x0031_9920)
}

// ---------------------------------------------------------------------------
// 0x0031ab30 PS2ExecMan_RunCallbackSync
// ---------------------------------------------------------------------------

/// `void PS2ExecMan_RunCallbackSync(PS2ExecMan* m)` (0x31ab30). Sets the
/// "callback pending" flag, waits for the current callback slot to
/// reach the callback's id (`0x423c90`/`0x423d10` poll the callback
/// semaphore), then either invokes the registered callback directly (the
/// slot is the one we are waiting for) or calls the table callback at
/// `*(m + 0x10000 ...)` and waits on the callback semaphore.
pub fn ps2exec_run_callback_sync(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        let pending = (c.g[S0][0] as u32).wrapping_add(0x4038);
        let one = c.g[V0][0];
        c.sw(pending, one);
        call_ptr(c, 0x0042_3c90, 0x0031_ab54)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10000));
        let entry_addr = (c.g[V0][0] as u32).wrapping_add(0x4040);
        let entry = c.lw(entry_addr);
        c.g[A0][0] = entry;
        if c.g[S1][0] != c.g[A0][0] {
            c.cycles += 3;
            call_ptr(c, 0x0042_3d10, 0x0031_ab74)?;
        }
        let word = (c.g[S0][0] as u32).wrapping_add(0x10);
        c.g[V0][0] = c.lw(word);
        // Both branches above end with V0 = *(s0 + 0x10).
        c.cycles += 2;
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0031_74a8, 0x0031_ab8c)?; // PS2ExecMan_InvokeCallback(m, 0)
        } else {
            let table = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = table;
            c.cycles += 7;
            let fn_off_addr = (c.g[V0][0] as u32).wrapping_add(0x28);
            c.g[A0][0] = c.lh(fn_off_addr);
            let fn_addr = (c.g[V0][0] as u32).wrapping_add(0x2c);
            c.g[V1][0] = c.lw(fn_addr);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0031_aba4)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_aaf0, 0x0031_abac)?; // PS2ExecMan_WaitSema(m)
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031adb0 ssx_SystemInit
// ---------------------------------------------------------------------------

/// `ssx_LoadIopModule(*(gp+0xda4), 0x490000 - name_off, 1, 0, 0)` returning to
/// `ret`: one of the IOP module loads of `ssx_SystemInit`.
fn ssx_system_init_load_iop(c: &mut Cpu, name_off: u32, ret: u32) -> Flow {
    let dev = gp_offset(c, 0xda4);
    c.g[A0][0] = c.lw(dev);
    c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(name_off));
    c.g[A2][0] = 1;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    call_ptr(c, 0x0031_9718, ret)
}

/// `void ssx_SystemInit(void)` (0x31adb0): boot-time system bring-up. Starts
/// the IOP, loads the IOP modules, seeds the random generator from the
/// clock, allocates and initialises the main heap (`MEM_Init`), sets up the
/// thread and display state, and returns.
pub fn ssx_system_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 115;
        let frame = Frame::open(c, 0x60);
        let dev = gp_offset(c, 0xda4);
        c.g[A0][0] = c.lw(dev);
        frame.save(c, 16, 0x50);
        frame.save_ra(c, 0x40);
        call_ptr(c, 0x0031_9610, 0x0031_adc8)?; // ssx_IopInit(dev)

        ssx_system_init_load_iop(c, 0x2578, 0x0031_ade4)?;
        ssx_system_init_load_iop(c, 0x2568, 0x0031_ae00)?;
        ssx_system_init_load_iop(c, 0x2558, 0x0031_ae1c)?;
        ssx_system_init_load_iop(c, 0x2548, 0x0031_ae38)?;
        ssx_system_init_load_iop(c, 0x2538, 0x0031_ae54)?;
        ssx_system_init_load_iop(c, 0x2528, 0x0031_ae70)?;

        call_ptr(c, 0x0040_9940, 0x0031_ae78)?;
        c.g[A0][0] = 0;
        call_ptr(c, 0x003f_f4a8, 0x0031_ae80)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0040_2520, 0x0031_ae88)?;

        // Seed: (word at sp+4 << 3) ^ (word at sp).
        let hi_addr = sp(c).wrapping_add(4);
        c.g[A0][0] = c.lw(hi_addr);
        let lo_addr = sp(c);
        c.g[V0][0] = c.lw(lo_addr);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = c.g[V0][0] ^ c.g[A0][0];
        call_ptr(c, 0x0031_77c8, 0x0031_ae9c)?; // Rand_SeedMain(seed)
        call_ptr(c, 0x003e_5018, 0x0031_aea4)?;

        c.g[A0][0] = 0x400;
        call_ptr(c, 0x0041_88c0, 0x0031_aeac)?; // heap block of 0x400 bytes
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_8910, 0x0031_aeb8)?;
        c.g[V0][0] = 0xffff_ffff_ffff_ffc0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3f));
        c.g[S0][0] &= c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x800));
        call_ptr(c, 0x0042_3d80, 0x0031_aecc)?;
        // MEM_Init(heap, size): size = V0 - S0 (the block end minus the start).
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        call_ptr(c, 0x0031_9a30, 0x0031_aed8)?;

        c.g[A0][0] = 1;
        call_ptr(c, 0x003e_6690, 0x0031_aee0)?;
        c.g[A0][0] = 0x3c;
        call_ptr(c, 0x003e_4af0, 0x0031_aee8)?;

        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[V0][0] = 0x24;
        let slot = sp(c).wrapping_add(0x10);
        let v24 = c.g[V0][0];
        c.sw(slot, v24);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003d_e928, 0x0031_aefc)?;

        c.g[V1][0] = 0x250000;
        c.g[V0][0] = 0x250000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2fa0));
        c.g[A1][0] = 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a8));
        let a1_slot = sp(c).wrapping_add(0x1c);
        let a1_val = c.g[A1][0];
        c.sw(a1_slot, a1_val);
        let v1_slot = sp(c).wrapping_add(0x14);
        let v1_val = c.g[V1][0];
        c.sw(v1_slot, v1_val);
        c.g[A0][0] = c.g[S0][0];
        let v0_slot = sp(c).wrapping_add(0x18);
        let v0_val = c.g[V0][0];
        c.sw(v0_slot, v0_val);
        call_ptr(c, 0x003d_e9b8, 0x0031_af24)?;

        c.g[A2][0] = 0x20;
        c.g[A0][0] = 0x30;
        c.g[A1][0] = 0x1f4;
        call_ptr(c, 0x003d_cbd8, 0x0031_af34)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x003d_f488, 0x0031_af40)?;
        c.g[A0][0] = 0x3e0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4000));
        call_ptr(c, 0x003e_58c8, 0x0031_af4c)?;
        c.g[A0][0] = 0x320000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x52e0));
        c.g[A2][0] = 0;
        call_ptr(c, 0x003e_57f8, 0x0031_af60)?;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ed8));
        call_ptr(c, 0x003d_de78, 0x0031_af6c)?;

        frame.restore(c, 16, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031b008 ssx_LoadExecPS2
// ---------------------------------------------------------------------------

/// `void ssx_LoadExecPS2(path, args, env)` (0x31b008): flush the CD/DVD
/// streams (through the loader table's callback), shut the command server
/// down and re-execute the ELF through `LoadExecPS2` (0x42c6a0).
pub fn ssx_load_exec_ps2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        let table_addr = gp_offset(c, 0x2a74);
        c.g[A3][0] = c.lw(table_addr);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        let entry_addr = (c.g[A3][0] as u32).wrapping_add(0x5c);
        c.g[V0][0] = c.lw(entry_addr);
        let op_addr = (c.g[V0][0] as u32).wrapping_add(0x10);
        c.g[A0][0] = c.lh(op_addr);
        let callback_addr = (c.g[V0][0] as u32).wrapping_add(0x14);
        c.g[V1][0] = c.lw(callback_addr);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0031_b040)?;
        c.g[A0][0] = 0x2;
        call_ptr(c, 0x0040_2450, 0x0031_b048)?;
        c.g[A0][0] = 0;
        call_ptr(c, 0x0040_1718, 0x0031_b050)?;
        c.g[A0][0] = 0x5;
        call_ptr(c, 0x0040_1718, 0x0031_b058)?;
        call_ptr(c, 0x0042_5fb8, 0x0031_b060)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0042_c6a0, 0x0031_b070)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031b9b0 Quat_RotateTowards
// ---------------------------------------------------------------------------

/// `void Quat_RotateTowards(Vec4* dst, const Vec4* from, const Vec4* to)`
/// (0x31b9b0): the VU0 part normalises the two directions (`from` and `to`)
/// and their cross/dot products; if they are not parallel (the dot product
/// test on `f12`) it blends them with `sin((1-t)θ)/sinθ` and `sin(tθ)/sinθ`,
/// with `θ = asin(dot)` from `Math_Asin`, and writes the result to `dst`.
/// The op sequence is the generated one, instruction for instruction.
pub fn quat_rotate_towards(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        c.mtc1(0, 1);
        frame.save(c, 16, 0x70);
        frame.save_ra(c, 0x60);
        c.g[S0][0] = c.g[A0][0];
        let from = c.g[A1][0] as u32;
        c.lqc2(4, from);
        let to = c.g[A2][0] as u32;
        c.lqc2(5, to);
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        let scratch = sp(c).wrapping_add(0x10);
        c.sqc2(6, scratch);
        c.lq(2, scratch);
        let frame_base = sp(c);
        c.sq(2, frame_base);
        let frame_base = sp(c);
        c.lqc2(3, frame_base);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 12);
        c.c_eq_s(12, 1);
        if !c.fcc() {
            // 'b_31ba4c: the clamp of the blend weight against 1.
            c.cycles += 12;
            let weight_addr = gp_below(c, 0x2ed8);
            c.lwc1(0, weight_addr);
            c.mfc1(2, 12);
            c.qmtc2(2, 3);
            c.vu0().div(0, 3, 3, 0);
            let scratch = sp(c);
            c.lqc2(4, scratch);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 4, Src::Q);
            let scratch = sp(c);
            c.sqc2(5, scratch);
            c.c_lt_s(0, 12);
            if c.fcc() {
                c.mov_s(12, 0);
            }

            // 'b_31ba98: the angle theta = asin(dot), with the nearly-parallel case.
            c.cycles += 12;
            let from = c.g[A1][0] as u32;
            c.lqc2(3, from);
            let to = c.g[A2][0] as u32;
            c.lqc2(5, to);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 5;
                call_ptr(c, 0x0031_c128, 0x0031_ba84)?; // Math_Asin(dot)
                let half_pi_addr = gp_below(c, 0x2ed4);
                c.lwc1(1, half_pi_addr);
                c.sub_s(0, 1, 0);
            } else {
                c.cycles += 2;
                call_ptr(c, 0x0031_c128, 0x0031_ba98)?;
            }

            // Blend: dst = a * f20 + b * f12, with the sin() weights.
            c.cycles += 34;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 12);
            let out_sin = sp(c).wrapping_add(0x50);
            c.g[A0][0] = sx(out_sin);
            let out_cos = sp(c).wrapping_add(0x54);
            c.g[A1][0] = sx(out_cos);
            c.mul_s(12, 0, 12);
            call_ptr(c, 0x0031_be50, 0x0031_bab0)?; // Math_SinCos(&s, &c)
            let s_addr = sp(c).wrapping_add(0x50);
            c.lwc1(0, s_addr);
            let c_addr = sp(c);
            c.lwc1(1, c_addr);
            let c_addr = sp(c).wrapping_add(4);
            c.lwc1(2, c_addr);
            let c_addr = sp(c).wrapping_add(8);
            c.lwc1(4, c_addr);
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            let c_addr = sp(c).wrapping_add(0x54);
            c.lwc1(3, c_addr);
            c.mul_s(0, 0, 4);
            let a = sp(c).wrapping_add(0x3c);
            c.swc1(3, a);
            let a = sp(c).wrapping_add(0x30);
            c.swc1(1, a);
            let a = sp(c).wrapping_add(0x34);
            c.swc1(2, a);
            let a = sp(c).wrapping_add(0x38);
            c.swc1(0, a);
            let dst = c.g[S0][0] as u32;
            c.lqc2(4, dst);
            let blend = sp(c).wrapping_add(0x30);
            c.lqc2(5, blend);
            c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().mula(0b1110, 4, Src::Bc(5, W));
            c.vu0().madda(0b1110, 5, Src::Bc(4, W));
            c.vu0().suba(0b0001, 7, Src::Bc(7, X));
            c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
            c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
            c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
            let tmp = sp(c).wrapping_add(0x40);
            c.sqc2(8, tmp);
            let tmp = sp(c).wrapping_add(0x40);
            c.lq(2, tmp);
            let dst = c.g[S0][0] as u32;
            c.sq(2, dst);
            let spill = sp(c).wrapping_add(0x20);
            c.sq(2, spill);
        }
        frame.restore(c, 16, 0x70);
        c.cycles += 3;
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031bcb0 Quat_Slerp
// ---------------------------------------------------------------------------

/// `void Quat_Slerp(Quat* dst, const Quat* a, const Quat* b, float t)`
/// (0x31bcb0): normalised quaternion slerp. Negates `b` when the dot product
/// is negative, then blends `a` and (possibly negated) `b` with
/// `sin((1-t)θ)/sinθ` and `sin(tθ)/sinθ`, falling back to a straight lerp
/// when the two are nearly parallel. `θ` comes from `Math_Asin` on the dot
/// product, with the quadrant constant at `gp-0x2ec4`. (The reference's
/// readable copy lives in `game/bx/ps2main.rs` as `quat_slerp`, unregistered.)
pub fn quat_slerp_port(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        let b = c.g[A2][0] as u32;
        c.lwc1(5, b);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_fpr(c, 22, 0x60);
        frame.save_ra(c, 0x20);
        c.mov_s(22, 12);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        c.mtc1(0, 4); // f4 = 0.0
        let a = c.g[S0][0] as u32;
        c.lwc1(6, b.wrapping_add(4));
        c.lwc1(0, a);
        c.lwc1(2, a.wrapping_add(4));
        c.mul_s(0, 0, 5);
        c.lwc1(7, b.wrapping_add(8));
        c.mul_s(2, 2, 6);
        c.lwc1(3, a.wrapping_add(8));
        c.lwc1(8, b.wrapping_add(0xc));
        c.mul_s(3, 3, 7);
        c.lwc1(1, a.wrapping_add(0xc));
        c.add_s(0, 0, 2);
        c.mul_s(1, 1, 8);
        c.add_s(0, 0, 3);
        c.add_s(12, 0, 1);
        c.c_lt_s(12, 4);
        c.g[S1][0] = c.g[A0][0];
        if c.fcc() {
            // Negative dot product: use -b.
            c.cycles += 10;
            c.neg_s(0, 5);
            c.neg_s(1, 6);
            c.neg_s(2, 7);
            c.neg_s(3, 8);
            let sp0 = sp(c);
            c.swc1(0, sp0);
            let sp4 = sp(c).wrapping_add(4);
            c.swc1(1, sp4);
            c.neg_s(12, 12);
            let sp8 = sp(c).wrapping_add(8);
            c.swc1(2, sp8);
            let spc = sp(c).wrapping_add(0xc);
            c.swc1(3, spc);
        } else {
            c.cycles += 4;
            let sp0 = sp(c);
            c.swc1(5, sp0);
            let sp4 = sp(c).wrapping_add(4);
            c.swc1(6, sp4);
            let sp8 = sp(c).wrapping_add(8);
            c.swc1(7, sp8);
            let spc = sp(c).wrapping_add(0xc);
            c.swc1(8, spc);
        }

        // The blend weights: f20 for `a`, f12 for `b`.
        'weights: {
            c.cycles += 8;
            // (checked against 'b_31bdc0 in the generated text)
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 21); // f21 = 1.0
            let half_addr = gp_below(c, 0x2ec8);
            c.lwc1(1, half_addr);
            c.sub_s(0, 21, 12);
            c.c_lt_s(1, 0);
            if !c.fcc() {
                // Nearly parallel: straight lerp.
                c.sub_s(20, 21, 22);
            } else {
                c.cycles += 17;
                call_ptr(c, 0x0031_c128, 0x0031_bd84)?; // Math_Asin(dot)
                let quadrant_addr = gp_below(c, 0x2ec4);
                c.lwc1(20, quadrant_addr);
                c.sub_s(20, 20, 0); // theta = half_pi - asin(dot)
                c.mov_s(12, 20);
                call_ptr(c, 0x0031_bf60, 0x0031_bd94)?; // sin(theta)
                c.sub_s(12, 21, 22);
                c.mov_s(21, 0);
                c.mul_s(12, 12, 20);
                call_ptr(c, 0x0031_bf60, 0x0031_bda4)?; // sin((1-t)theta)
                c.mul_s(12, 22, 20);
                c.div_s(20, 0, 21);
                call_ptr(c, 0x0031_bf60, 0x0031_bdb4)?; // sin(t*theta)
                c.div_s(12, 0, 21);
                let s_addr = (c.g[S0][0] as u32).wrapping_add(0xc);
                c.lwc1(7, s_addr);
                break 'weights;
            }
            c.cycles += 2;
            c.mov_s(12, 22);
            let s_addr = (c.g[S0][0] as u32).wrapping_add(0xc);
            c.lwc1(7, s_addr);
        }

        // dst = a * f20 + b * f12
        c.cycles += 34;
        c.g[V0][0] = c.g[S1][0];
        let a = c.g[S0][0] as u32;
        c.lwc1(5, a);
        c.lwc1(6, a.wrapping_add(4));
        c.mul_s(7, 20, 7);
        c.lwc1(4, a.wrapping_add(8));
        c.mul_s(5, 20, 5);
        let sp0 = sp(c);
        c.lwc1(3, sp0);
        c.mul_s(6, 20, 6);
        let sp4 = sp(c).wrapping_add(4);
        c.lwc1(2, sp4);
        c.mul_s(4, 20, 4);
        let sp8 = sp(c).wrapping_add(8);
        c.lwc1(1, sp8);
        c.mul_s(3, 12, 3);
        let spc = sp(c).wrapping_add(0xc);
        c.lwc1(0, spc);
        c.mul_s(2, 12, 2);
        c.mul_s(1, 12, 1);
        frame.restore(c, 16, 0x40);
        c.mul_s(0, 12, 0);
        frame.restore_ra(c, 0x20);
        c.add_s(5, 5, 3);
        frame.restore_fpr(c, 22, 0x60);
        c.add_s(6, 6, 2);
        frame.restore_fpr(c, 21, 0x58);
        c.add_s(4, 4, 1);
        frame.restore_fpr(c, 20, 0x50);
        c.add_s(7, 7, 0);
        let s10 = sp(c).wrapping_add(0x10);
        c.swc1(5, s10);
        let s14 = sp(c).wrapping_add(0x14);
        c.swc1(6, s14);
        let s18 = sp(c).wrapping_add(0x18);
        c.swc1(4, s18);
        let s1c = sp(c).wrapping_add(0x1c);
        c.swc1(7, s1c);
        frame.restore(c, 3, 0x10);
        let dst = c.g[S1][0] as u32;
        c.sq(3, dst);
        frame.restore(c, 17, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0031e0e0 cCSICubicSpline_SampleRange
// ---------------------------------------------------------------------------

/// `void cCSICubicSpline_SampleRange(Spline* s, float t0, ...)` (0x31e0e0):
/// steps the sample time from the spline's first knot up to its last one
/// (`t += step`, `step = (last - first) * gp-0x2e40`), evaluating the spline
/// with `cCSICubicSpline_Evaluate` at each sample. The first loop finds the
/// first sample at or after the start; the second evaluates until the end.
pub fn c_csi_cubic_spline_sample_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        let step_addr = gp_below(c, 0x2e40);
        c.lwc1(0, step_addr);
        frame.save(c, 16, 0x10);
        frame.save_fpr(c, 22, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        let s = c.g[S0][0] as u32;
        let knots_addr = s.wrapping_add(8);
        c.g[V0][0] = c.lw(knots_addr);
        c.lwc1(21, s.wrapping_add(4));
        let knot = c.g[V0][0] as u32;
        c.lwc1(12, knot.wrapping_add(0x10));
        c.sub_s(1, 21, 12);
        c.mov_s(2, 12);
        c.c_le_s(12, 21);
        let in_range = c.fcc();
        c.mul_s(22, 1, 0);
        if in_range {
            c.cycles += 1;
            c.add_s(2, 2, 22);
            loop {
                c.cycles += 5;
                c.c_le_s(2, 21);
                if !c.fcc() {
                    break;
                }
                c.add_s(2, 2, 22);
                if event_due(c) {
                    deliver_event_here(c, 0x0031_e128);
                }
            }
        }

        c.cycles += 5;
        c.mov_s(20, 12);
        c.c_le_s(20, 21);
        if c.fcc() {
            c.cycles += 2;
            c.mov_s(12, 20);
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.add_s(20, 20, 22);
                call_ptr(c, 0x0031_dee0, 0x0031_e164)?; // cCSICubicSpline_Evaluate(s)
                c.c_le_s(20, 21);
                let more = c.fcc();
                c.mov_s(12, 20);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_e158);
                    }
                    continue;
                }
                c.cycles += 1;
                break;
            }
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 6;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x30);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, put_vec, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// This module's fake gate and mailbox words (RAM, above the fakes' log).
    const GATE: u32 = DATA + 0x21_0000;
    const MAGIC: u32 = 0x1579_f001;
    const BUSY: u32 = DATA + 0x21_0004;
    const POWER_STATUS: u32 = DATA + 0x21_0008;
    const POWER_RET: u32 = DATA + 0x21_000c;

    /// A gated fake for the real callee at `$addr`: with the gate off it runs
    /// what the slot held before (`fakes::fall_back`), with the gate on it logs
    /// the call and returns (or unwinds) like `fakes::fake_call`.
    macro_rules! gated {
        ($name:ident, $gen:path, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(GATE) != MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    // Callees of the ports, in the shard they generate to.
    gated!(oc1579_fake_423de0, ee::s15::f_00423de0__generated, 0x0042_3de0);
    gated!(oc1579_fake_423dd0, ee::s15::f_00423dd0__generated, 0x0042_3dd0);
    gated!(oc1579_fake_423c90, ee::s15::f_00423c90__generated, 0x0042_3c90);
    gated!(oc1579_fake_423d10, ee::s15::f_00423d10__generated, 0x0042_3d10);
    gated!(oc1579_fake_423d80, ee::s15::f_00423d80__generated, 0x0042_3d80);
    gated!(oc1579_fake_42c6a0, ee::s15::f_0042c6a0__generated, 0x0042_c6a0);
    gated!(oc1579_fake_401718, ee::s15::f_00401718__generated, 0x0040_1718);
    gated!(oc1579_fake_402450, ee::s15::f_00402450__generated, 0x0040_2450);
    gated!(oc1579_fake_425fb8, ee::s15::f_00425fb8__generated, 0x0042_5fb8);
    gated!(oc1579_fake_409940, ee::s15::f_00409940__generated, 0x0040_9940);
    gated!(oc1579_fake_3ff4a8, ee::s15::f_003ff4a8__generated, 0x003f_f4a8);
    gated!(oc1579_fake_402520, ee::s15::f_00402520__generated, 0x0040_2520);
    gated!(oc1579_fake_4188c0, ee::s15::f_004188c0__generated, 0x0041_88c0);
    gated!(oc1579_fake_418910, ee::s15::f_00418910__generated, 0x0041_8910);
    gated!(oc1579_fake_3174a8, ee::s10::f_003174a8__generated, 0x0031_74a8);
    gated!(oc1579_fake_31aaf0, ee::s10::f_0031aaf0__generated, 0x0031_aaf0);
    gated!(oc1579_fake_319610, ee::s10::f_00319610__generated, 0x0031_9610);
    gated!(oc1579_fake_319718, ee::s10::f_00319718__generated, 0x0031_9718);
    gated!(oc1579_fake_3177c8, ee::s10::f_003177c8__generated, 0x0031_77c8);
    gated!(oc1579_fake_319a30, ee::s10::f_00319a30__generated, 0x0031_9a30);
    gated!(oc1579_fake_31c128, ee::s10::f_0031c128__generated, 0x0031_c128);
    gated!(oc1579_fake_31be50, ee::s10::f_0031be50__generated, 0x0031_be50);
    gated!(oc1579_fake_31bf60, ee::s10::f_0031bf60__generated, 0x0031_bf60);
    gated!(oc1579_fake_31dee0, ee::s10::f_0031dee0__generated, 0x0031_dee0);
    gated!(oc1579_fake_3e5018, ee::s14::f_003e5018__generated, 0x003e_5018);
    gated!(oc1579_fake_3e6690, ee::s14::f_003e6690__generated, 0x003e_6690);
    gated!(oc1579_fake_3e4af0, ee::s14::f_003e4af0__generated, 0x003e_4af0);
    gated!(oc1579_fake_3de928, ee::s14::f_003de928__generated, 0x003d_e928);
    gated!(oc1579_fake_3de9b8, ee::s14::f_003de9b8__generated, 0x003d_e9b8);
    gated!(oc1579_fake_3dcbd8, ee::s14::f_003dcbd8__generated, 0x003d_cbd8);
    gated!(oc1579_fake_3df488, ee::s14::f_003df488__generated, 0x003d_f488);
    gated!(oc1579_fake_3e58c8, ee::s14::f_003e58c8__generated, 0x003e_58c8);
    gated!(oc1579_fake_3e57f8, ee::s14::f_003e57f8__generated, 0x003e_57f8);
    gated!(oc1579_fake_3dde78, ee::s14::f_003dde78__generated, 0x003d_de78);

    /// `sceDevctl`: the logged fake, then a negative result while the busy
    /// mailbox word is set.
    fn oc1579_fake_42a538(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1579_fake_42a538), ee::s15::f_0042a538__generated);
        }
        fakes::fake_call(c, 0x0042_a538);
        if c.bus.read32(BUSY) != 0 {
            c.g[V0][0] = u64::MAX;
        }
    }

    /// `sceCdPowerOff(status*)`: the logged fake, writes the status word the
    /// mailbox holds through its pointer, and returns the mailbox result.
    fn oc1579_fake_402398(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1579_fake_402398), ee::s15::f_00402398__generated);
        }
        let status_ptr = c.g[A0][0] as u32;
        let status = c.bus.read32(POWER_STATUS);
        c.bus.write32(status_ptr, status);
        fakes::fake_call(c, 0x0040_2398);
        c.g[V0][0] = c.bus.read32(POWER_RET) as u64;
    }

    /// Hook every fake of this module into its callee's slot (once per process).
    fn oc1579_install() {
        macro_rules! hook {
            ($slot:path, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        hook!(ee::s15::OVERRIDE_00423DE0, oc1579_fake_423de0);
        hook!(ee::s15::OVERRIDE_00423DD0, oc1579_fake_423dd0);
        hook!(ee::s15::OVERRIDE_00423C90, oc1579_fake_423c90);
        hook!(ee::s15::OVERRIDE_00423D10, oc1579_fake_423d10);
        hook!(ee::s15::OVERRIDE_00423D80, oc1579_fake_423d80);
        hook!(ee::s15::OVERRIDE_0042C6A0, oc1579_fake_42c6a0);
        hook!(ee::s15::OVERRIDE_00401718, oc1579_fake_401718);
        hook!(ee::s15::OVERRIDE_00402450, oc1579_fake_402450);
        hook!(ee::s15::OVERRIDE_00425FB8, oc1579_fake_425fb8);
        hook!(ee::s15::OVERRIDE_00409940, oc1579_fake_409940);
        hook!(ee::s15::OVERRIDE_003FF4A8, oc1579_fake_3ff4a8);
        hook!(ee::s15::OVERRIDE_00402520, oc1579_fake_402520);
        hook!(ee::s15::OVERRIDE_004188C0, oc1579_fake_4188c0);
        hook!(ee::s15::OVERRIDE_00418910, oc1579_fake_418910);
        hook!(ee::s15::OVERRIDE_0042A538, oc1579_fake_42a538);
        hook!(ee::s15::OVERRIDE_00402398, oc1579_fake_402398);
        hook!(ee::s10::OVERRIDE_003174A8, oc1579_fake_3174a8);
        hook!(ee::s10::OVERRIDE_0031AAF0, oc1579_fake_31aaf0);
        hook!(ee::s10::OVERRIDE_00319610, oc1579_fake_319610);
        hook!(ee::s10::OVERRIDE_00319718, oc1579_fake_319718);
        hook!(ee::s10::OVERRIDE_003177C8, oc1579_fake_3177c8);
        hook!(ee::s10::OVERRIDE_00319A30, oc1579_fake_319a30);
        hook!(ee::s10::OVERRIDE_0031C128, oc1579_fake_31c128);
        hook!(ee::s10::OVERRIDE_0031BE50, oc1579_fake_31be50);
        hook!(ee::s10::OVERRIDE_0031BF60, oc1579_fake_31bf60);
        hook!(ee::s10::OVERRIDE_0031DEE0, oc1579_fake_31dee0);
        hook!(ee::s14::OVERRIDE_003E5018, oc1579_fake_3e5018);
        hook!(ee::s14::OVERRIDE_003E6690, oc1579_fake_3e6690);
        hook!(ee::s14::OVERRIDE_003E4AF0, oc1579_fake_3e4af0);
        hook!(ee::s14::OVERRIDE_003DE928, oc1579_fake_3de928);
        hook!(ee::s14::OVERRIDE_003DE9B8, oc1579_fake_3de9b8);
        hook!(ee::s14::OVERRIDE_003DCBD8, oc1579_fake_3dcbd8);
        hook!(ee::s14::OVERRIDE_003DF488, oc1579_fake_3df488);
        hook!(ee::s14::OVERRIDE_003E58C8, oc1579_fake_3e58c8);
        hook!(ee::s14::OVERRIDE_003E57F8, oc1579_fake_3e57f8);
        hook!(ee::s14::OVERRIDE_003DDE78, oc1579_fake_3dde78);
    }

    /// `fakes::machine` with this module's gate switched on.
    fn oc1579_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(GATE, MAGIC);
        r
    }

    #[test]
    fn ssx_power_off_thread_matches() {
        oc1579_install();
        check(
            ee::s10::f_003198a0__generated,
            ssx_power_off_thread,
            200,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0001, 8);
                c.g[A0][0] = sx(DATA + 0x100);
                // Unwind within the first calls: the poll loops never end otherwise.
                c.bus.write32(fakes::LOG_UNWIND_AT, seed % 8);
                c.bus.write32(BUSY, (seed / 8 % 3 == 0) as u32);
                c.bus.write32(POWER_STATUS, seed / 3 % 2);
                c.bus.write32(POWER_RET, seed / 5 % 2);
            },
        );
    }

    #[test]
    fn ssx_power_off_signal_sema_matches() {
        oc1579_install();
        check(
            ee::s10::f_00319910__generated,
            ssx_power_off_signal_sema,
            100,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0002, 2);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn ps2exec_run_callback_sync_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031ab30__generated,
            ps2exec_run_callback_sync,
            200,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0003, 0);
                let m = DATA + 0x1000;
                c.g[A0][0] = sx(m);
                let v = 0x0080_1000;
                // The first call (0x423c90) returns v.
                c.bus.write32(fakes::LOG_RESULTS, v);
                c.bus.write32(m + 0x1_4040, if seed % 2 == 0 { v } else { v + 0x1000 });
                c.bus.write32(m + 0x10, if seed % 3 == 0 { v } else { 0 });
                // The table entry the tail path calls through (not in the table: a miss).
                c.bus.write32(m + 8, DATA + 0x2000);
                c.bus.write16(DATA + 0x2028, 0x40);
                c.bus.write32(DATA + 0x202c, fakes::FAKE + 0x80);
            },
        );
    }

    #[test]
    fn ssx_system_init_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031adb0__generated,
            ssx_system_init,
            100,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0004, if seed % 2 == 0 { 0 } else { 24 });
            },
        );
    }

    #[test]
    fn ssx_load_exec_ps2_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031b008__generated,
            ssx_load_exec_ps2,
            100,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0005, 6);
                // gp+0x2a74 -> loader table -> entry (op at +0x10, callback at +0x14).
                c.bus.write32(GAME_GP + 0x2a74, DATA + 0x2000);
                c.bus.write32(DATA + 0x205c, DATA + 0x2100);
                c.bus.write16(DATA + 0x2110, 0x40);
                c.bus.write32(DATA + 0x2114, fakes::FAKE + 0x80);
            },
        );
    }

    #[test]
    fn quat_rotate_towards_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031b9b0__generated,
            quat_rotate_towards,
            200,
            &|c, seed| {
                let mut r = oc1579_machine(c, seed, 0x1579_0006, 4);
                c.g[A0][0] = sx(DATA + 0x3020);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3010);
                put_vec(c, DATA + 0x3000, [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)]);
                put_vec(c, DATA + 0x3010, [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)]);
                put_vec(c, DATA + 0x3020, [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)]);
                // f12 = 1.0 (early out), 0.0 (nearly parallel), 0.5, or random.
                c.f[12] = match seed % 4 {
                    0 => 0x3f80_0000,
                    1 => 0,
                    2 => 0x3f00_0000,
                    _ => r.f32(1.0),
                };
            },
        );
    }

    #[test]
    fn quat_slerp_port_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031bcb0__generated,
            quat_slerp_port,
            200,
            &|c, seed| {
                let mut r = oc1579_machine(c, seed, 0x1579_0007, 4);
                c.g[A0][0] = sx(DATA + 0x3100);
                c.g[A1][0] = sx(DATA + 0x3200);
                c.g[A2][0] = sx(DATA + 0x3300);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x3200 + 4 * i, r.f32(1.0));
                    c.bus.write32(DATA + 0x3300 + 4 * i, r.f32(1.0));
                }
                c.f[12] = match seed % 3 {
                    0 => 0x3f80_0000,
                    1 => 0,
                    _ => r.f32(1.0),
                };
                // The quadrant and near-parallel constants.
                c.bus.write32(GAME_GP - 0x2ec8, 0x3f7f_ffff);
                c.bus.write32(GAME_GP - 0x2ec4, 0x3fc9_0fdb);
            },
        );
    }

    #[test]
    fn c_csi_cubic_spline_sample_range_matches() {
        oc1579_install();
        check(
            ee::s10::f_0031e0e0__generated,
            c_csi_cubic_spline_sample_range,
            200,
            &|c, seed| {
                oc1579_machine(c, seed, 0x1579_0008, 4);
                let s = DATA + 0x2000;
                c.g[A0][0] = sx(s);
                // Knots: first at 0 (in range) or past the last one (skipped).
                let first = if seed % 5 == 0 { 9.0f32 } else { 0.0 };
                c.bus.write32(s + 4, (4.0f32 + (seed % 3) as f32).to_bits());
                c.bus.write32(s + 8, DATA + 0x2100);
                c.bus.write32(DATA + 0x2110, first.to_bits());
                // Step scale: a positive constant, so the loops end.
                c.bus.write32(GAME_GP - 0x2e40, 0x3f00_0000);
            },
        );
    }
}
