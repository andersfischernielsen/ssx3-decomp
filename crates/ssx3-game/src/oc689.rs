//! Staging module oc689 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc689`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_6188, ssx_mount_slus_ovf_big, "ssx_MountSlusOvfBig"),
    (0x0031_9660, ssx_get_modules_dir, "ssx_GetModulesDir"),
    (0x0031_9690, ssx_build_module_path, "ssx_BuildModulePath"),
    (0x0031_9718, ssx_load_iop_module, "ssx_LoadIopModule"),
    (0x0031_97d0, ssx_load_msifrpc, "ssx_LoadMsifrpc"),
    (
        0x0031_9930,
        ssx_install_power_off_handler,
        "ssx_InstallPowerOffHandler",
    ),
    (0x0031_a6d8, ps2_exec_man_init, "PS2ExecMan_Init"),
    (
        0x0036_96c8,
        ps2_graphics_man_get_texture_by_name,
        "PS2GraphicsMan_GetTextureByName",
    ),
    (
        0x0037_62f8,
        ps2_graphics_man_find_display_mode,
        "PS2GraphicsMan_FindDisplayMode",
    ),
    (
        0x0037_64c0,
        ps2_graphics_man_select_display_mode,
        "PS2GraphicsMan_SelectDisplayMode",
    ),
];

/// `void ssx_MountSlusOvfBig(void)` (0x00256188): mounts the
/// `data/config/slusovf.big` archive (`EAFile_MountBig(name, 0, 100,
/// &D_004A2ED4)`).
pub fn ssx_mount_slus_ovf_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3d0));
        c.g[A3][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x21c));
        c.g[A2][0] = 0x64;
        call_ptr(c, 0x003d_ed50, 0x0025_61a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_GetModulesDir(void* self, char* out)` (0x00319660): writes the
/// IOP modules directory (`sprintf(out, "%sDATA/MODULES...", "cdrom0:...")`).
pub fn ssx_get_modules_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[A0][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x49_0000;
        c.g[A2][0] = 0x49_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2628));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2638));
        call_ptr(c, 0x0041_7828, 0x0031_9680)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_BuildModulePath(void* self, char* out, const char* name)`
/// (0x00319690): a name that already has a device (contains `:`) is copied
/// as is, otherwise it is `modules dir + name + suffix`
/// (`sprintf(out, fmt, dir, name, suffix)`).
pub fn ssx_build_module_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x140);
        frame.save(c, 16, 0x130);
        frame.save(c, 17, 0x120);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 18, 0x110);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[A1][0] = 0x3a;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_6404, 0x0031_96bc)?;
        let has_device = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if has_device {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x0031_96cc)?;
            frame.restore(c, 16, 0x130);
        } else {
            c.cycles += 12;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0031_9660,
                f: ee::s10::f_00319660,
            }
            .call(c, 0x0031_96e0)?;
            c.g[A1][0] = 0x4a_0000;
            c.g[T0][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3e98));
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3ea0));
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0041_7828, 0x0031_9700)?;
            frame.restore(c, 16, 0x130);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x120);
        frame.restore(c, 18, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `int ssx_LoadIopModule(void* self, const char* module, int retry, const char* args, int arg_len)`
/// (0x00319718): builds the module path and calls `sceSifLoadModule(path,
/// args, arg_len)`. A negative result is retried when `retry` is set, or
/// while it is -204 (`0xffffff34`); any other result is returned.
pub fn ssx_load_iop_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x150);
        frame.save(c, 18, 0x120);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 16, 0x140);
        frame.save(c, 17, 0x130);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, 19, 0x110);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0x100);
        c.g[S0][0] = c.g[T0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[S3][0] = 0xffff_ffff_ffff_ff34;
        Callee {
            addr: 0x0031_9690,
            f: ee::s10::f_00319690,
        }
        .call(c, 0x0031_974c)?;
        c.g[A0][0] = c.g[SP][0];
        'retry: loop {
            c.cycles += 1;
            c.g[A1][0] = c.g[S1][0];
            'again: loop {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0042_ba68, 0x0031_975c)?;
                if (c.g[V0][0] as i64) >= 0 {
                    frame.restore(c, 16, 0x140);
                    break 'retry;
                }
                c.cycles += 2;
                let retry = c.g[S2][0] != 0;
                c.g[A0][0] = c.g[SP][0];
                if retry {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_9750);
                    }
                    continue 'retry;
                }
                c.cycles += 2;
                let busy = c.g[V0][0] == c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                if busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_9754);
                    }
                    continue 'again;
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x140);
                break 'retry;
            }
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x130);
        frame.restore(c, 18, 0x120);
        frame.restore(c, 19, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_LoadMsifrpc(void* self)` (0x003197d0): loads the `MSIFRPC.IRX`
/// IOP module (`ssx_LoadIopModule(D_004A3E94, "MSIFRPC.IRX", 1, 0, 0)`).
pub fn ssx_load_msifrpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x49_0000;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda4));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2618));
        frame.save_ra(c, 0);
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        Callee {
            addr: 0x0031_9718,
            f: ee::s10::f_00319718,
        }
        .call(c, 0x0031_97f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void ssx_InstallPowerOffHandler(void)` (0x00319930): when the power-off
/// module is present (`sceSifSearchModuleByName` >= 0), creates a semaphore
/// (initial count 0, max 1), starts a thread (stack 0x2000 bytes, priority 1)
/// and registers the power-off callback with `sceCdPOffCallback`; the
/// semaphore id is kept in `gp + 0x2a64`, the thread id in `gp + 0x2a80`.
pub fn ssx_install_power_off_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x70);
        c.g[A0][0] = 0x4a_0000;
        frame.save(c, 16, 0x60);
        frame.save_ra(c, 0x50);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3eb0));
        call_ptr(c, 0x0042_b6d0, 0x0031_9948)?;
        let missing = (c.g[V0][0] as i64) < 0;
        c.g[S0][0] = 1;
        if !missing {
            c.cycles += 28;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x30));
            c.sw(frame.at(c, 0x38), 0);
            c.sw(frame.at(c, 0x34), c.g[S0][0]);
            c.sw(frame.at(c, 0x44), 0);
            call_ptr(c, 0x0042_3da0, 0x0031_9964)?;
            c.g[A1][0] = 0x32_0000;
            c.g[A2][0] = 0x54_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6760));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x74f0));
            c.g[V1][0] = 0x2000;
            c.g[A3][0] = 0x4a_0000;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x30f0));
            c.g[A0][0] = c.g[SP][0];
            c.sw(frame.at(c, 0x4), c.g[A1][0]);
            c.sw(frame.at(c, 0xc), c.g[V1][0]);
            c.sw(frame.at(c, 0x10), c.g[A3][0]);
            c.sw(frame.at(c, 0x8), c.g[A2][0]);
            c.sw(frame.at(c, 0x14), c.g[S0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2a64), c.g[V0][0]);
            call_ptr(c, 0x0042_3ba0, 0x0031_99a0)?;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a64));
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2a80), c.g[V0][0]);
            call_ptr(c, 0x0042_3bc0, 0x0031_99b0)?;
            c.g[A0][0] = 0x32_0000;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a64));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x66f0));
            call_ptr(c, 0x0040_0d58, 0x0031_99c0)?;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    });
}

/// `void PS2ExecMan_Init(PS2ExecMan* self)` (0x0031a6d8): saves the VU0 context
/// into the manager (`self + 0x14080`), records the main thread id and
/// priority, creates the semaphores (`+0xc`, `+0x18`, `+0x4034`, `+0x4044`), starts
/// the execution thread (`+0x4040`: entry 0x31ac60, a 0x10000-byte stack at
/// `self + 0x4040`) and a second thread (`+0x4030`: entry 0x31ac08, a
/// 0x4000-byte stack at `self + 0x30`), then installs INTC handler 3
/// (`+0x4290`). Converted from
/// the generated body (every register, cycle and call kept).
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn ps2_exec_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 145;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[A0][0] = 0x10000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A0][0] = c.g[A0][0] | 0x4080;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = 0x10000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S5][0] = 0x400;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
        c.g[S6][0] = 0x10;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
        c.g[S7][0] = 0x4a0000;
        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x30f0));
        c.g[RA][0] = sx(0x0031a734);
        Callee {
            addr: 0x0031a308,
            f: ee::s10::VU0_SaveContext,
        }
        .call(c, 0x0031a734)?;
        c.g[RA][0] = sx(0x0031a73c);
        call_ptr(c, 0x00423c90, 0x0031a73c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0x30;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[RA][0] = sx(0x0031a750);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00416210, 0x0031a750)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x0031a75c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x00423ca0, 0x0031a75c)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[RA][0] = sx(0x0031a778);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00416210, 0x0031a778)?;
        c.g[RA][0] = sx(0x0031a780);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a780)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0031a798);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a798)?;
        c.g[RA][0] = sx(0x0031a7a0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a7a0)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0031a7bc);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a7bc)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[S5][0]);
        c.g[RA][0] = sx(0x0031a7c8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a7c8)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4048), 0u64);
        c.g[A2][0] = 0x30;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0031a7e4);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4044), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a7e4)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4040));
        c.g[V0][0] = 0x320000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[A0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x53a0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[S4][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S6][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S7][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
        c.g[RA][0] = sx(0x0031a824);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), 0u64);
        call_ptr(c, 0x00423ba0, 0x0031a824)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4040), c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0031a838);
        c.sw((c.g[GP][0] as u32).wrapping_add(0xddc), c.g[V0][0]);
        call_ptr(c, 0x00423bc0, 0x0031a838)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4038), 0u64);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x18;
        c.g[RA][0] = sx(0x0031a84c);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00416210, 0x0031a84c)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[S5][0]);
        c.g[RA][0] = sx(0x0031a858);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00423da0, 0x0031a858)?;
        c.g[A2][0] = 0x30;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0031a86c);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4034), c.g[V0][0]);
        call_ptr(c, 0x00416210, 0x0031a86c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = 0x320000;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = 0x4000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x53f8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S6][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x98), c.g[A2][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S7][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
        c.g[RA][0] = sx(0x0031a8b0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xb0), 0u64);
        call_ptr(c, 0x00423ba0, 0x0031a8b0)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0031a8c0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4030), c.g[V0][0]);
        call_ptr(c, 0x00423bc0, 0x0031a8c0)?;
        c.g[RA][0] = sx(0x0031a8c8);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x00424880, 0x0031a8c8)?;
        c.g[A1][0] = 0x320000;
        c.g[A3][0] = 0x320000;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x5430));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5b70));
        c.g[A0][0] = 0x3;
        c.g[RA][0] = sx(0x0031a8e4);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00423a90, 0x0031a8e4)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x4290), c.g[V0][0]);
        c.g[RA][0] = sx(0x0031a8f0);
        c.g[A0][0] = 0x3;
        call_ptr(c, 0x004248e8, 0x0031a8f0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        c.pc = t;
        return Ok(());
    });
}

/// `int PS2GraphicsMan_GetTextureByName(PS2GraphicsMan* self, const char* name)`
/// (0x003696c8): texture id of a 4-character shape name. Copies at most four
/// characters of `name` into a local buffer, looks it up in the cache
/// (`self + 0x280 + 12 * i`: handle, name; count at `+0xe80`), otherwise
/// appends an entry, searches the shape banks (`+0x278` count, `+0x27c` array)
/// with `SHAPE_locatez` and creates the texture through vtable slot 46
/// (`self->+0x10d8`), or stores -1. Returns the entry's handle.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn ps2_graphics_man_get_texture_by_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_369724: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V0][0] = c.lb((c.g[A1][0] as u32));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[A1][0];
            if t {
                break 'b_369724;
            } else {
                c.cycles += 2;
                c.g[A2][0] = c.g[SP][0];
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
                'l_369700: loop {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sb((c.g[A2][0] as u32), c.g[V0][0]);
                    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if t {
                        break 'b_369724;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
                            if event_due(c) {
                                deliver_event_here(c, 0x00369700);
                            }
                            continue 'l_369700;
                        } else {
                            break 'b_369724;
                        }
                    }
                }
            }
        }
        'b_36986c: {
            'b_369778: {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
                c.g[S3][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sb((c.g[V1][0] as u32), 0u64);
                let t = (c.g[A0][0] as i64) <= 0;
                c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
                if t {
                    break 'b_369778;
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = c.g[S4][0];
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x284));
                    'l_369748: loop {
                        'b_369764: {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x00369754);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x004165a8, 0x00369754)?;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                                break 'b_369764;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                if t {
                                    break 'b_36986c;
                                } else {
                                    break 'b_369764;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00369748);
                            }
                            continue 'l_369748;
                        } else {
                            break 'b_369778;
                        }
                    }
                }
            }
            'b_36983c: {
                c.cycles += 11;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                c.g[V0][0] = 0xc;
                c.g[A1][0] = c.g[SP][0];
                c.g[S1][0] = 0u64;
                c.mult(0, 3, 4, 2);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[RA][0] = sx(0x00369798);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
                call_ptr(c, 0x004166f4, 0x00369798)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x278));
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[S3][0] = 0xc;
                if t {
                    break 'b_36983c;
                } else {
                    c.cycles += 3;
                    c.g[S5][0] = 0x4a0000;
                    c.g[S2][0] = c.g[S4][0];
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                    'l_3697b0: loop {
                        'b_369828: {
                            c.cycles += 10;
                            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x27c));
                            c.mult(0, 4, 5, 19);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                            c.g[RA][0] = sx(0x003697d0);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x284));
                            call_ptr(c, 0x003b3900, 0x003697d0)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_369828;
                            } else {
                                c.cycles += 20;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                                c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x41a0));
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                                c.g[T0][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                                c.g[T1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003697fc);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x003697fc)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                                c.mult(0, 4, 3, 19);
                                c.g[V1][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                                c.mult(0, 2, 4, 19);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xe80), c.g[A0][0]);
                                if t {
                                    break 'b_36986c;
                                } else {
                                    break 'b_369828;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x278));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
                            if event_due(c) {
                                deliver_event_here(c, 0x003697b0);
                            }
                            continue 'l_3697b0;
                        } else {
                            break 'b_36983c;
                        }
                    }
                }
            }
            c.cycles += 12;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
            c.g[A0][0] = 0xc;
            c.g[A1][0] = 0xffffffffffffffff;
            c.mult(0, 3, 2, 4);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe80));
            c.mult(0, 2, 3, 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xe80), c.g[V1][0]);
            break 'b_36986c;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `int PS2GraphicsMan_FindDisplayMode(self, width, height, bpp, clut_bpp, fmt, id)`
/// (0x003762f8): index of the first display mode (`self + 0x59cc`, `0xa4`
/// bytes each, count at `+0x59c4`) with the width (`+0x88`), height (`+0x8c`)
/// and id (`+0xa0`) given whose pixel format (`+0x90`), CLUT format (`+0x94`)
/// and format (`+0x98`) match the bit depths asked for; -1 if none.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn ps2_graphics_man_find_display_mode(c: &mut Cpu) {
    guest(c, |c| {
        'b_376454: {
            'b_376450: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[T4][0] = 0u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[T1][0];
                c.sq(18, (c.g[SP][0] as u32));
                c.g[S1][0] = c.g[A2][0];
                c.g[S2][0] = c.g[A1][0];
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x59c4));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x59cc));
                if t {
                    break 'b_376450;
                } else {
                    c.cycles += 7;
                    c.g[T1][0] = c.g[V0][0];
                    c.g[T7][0] = 0x18;
                    c.g[A2][0] = (c.g[A3][0] < 0x19u64) as u64;
                    c.g[T9][0] = 0x10;
                    c.g[T8][0] = 0x20;
                    c.g[T5][0] = 0x1;
                    c.g[A1][0] = (c.g[T0][0] < 0x19u64) as u64;
                    'l_376340: loop {
                        'b_376444: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                            if c.g[V0][0] != c.g[S2][0] {
                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                                break 'b_376444;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                if c.g[V0][0] != c.g[S1][0] {
                                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                                    break 'b_376444;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                                    if c.g[V0][0] != c.g[T2][0] {
                                        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                                        break 'b_376444;
                                    } else {
                                        'b_3763c0: {
                                            'b_3763b8: {
                                                'b_3763a8: {
                                                    c.cycles += 4;
                                                    c.g[T3][0] = 0u64;
                                                    c.g[A0][0] = 0u64;
                                                    let t = c.g[A3][0] == c.g[T7][0];
                                                    c.g[T6][0] = 0u64;
                                                    if t {
                                                        break 'b_3763a8;
                                                    } else {
                                                        'b_37639c: {
                                                            'b_37638c: {
                                                                c.cycles += 2;
                                                                let t = c.g[A2][0] == 0u64;

                                                                if t {
                                                                    break 'b_37638c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    if c.g[A3][0] == c.g[T9][0] {
                                                                        c.g[V0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x90));
                                                                        break 'b_37639c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;

                                                                        if t {
                                                                            break 'b_3763c0;
                                                                        } else {
                                                                            break 'b_37638c;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            if c.g[A3][0] == c.g[T8][0] {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x90));
                                                                break 'b_3763b8;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;

                                                                if t {
                                                                    break 'b_3763c0;
                                                                } else {
                                                                    break 'b_37639c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[T3][0] = c.g[T5][0];
                                                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                                        if c.g[V0][0] != 0 {
                                                            c.g[T3][0] = 0u64;
                                                        }
                                                        break 'b_3763a8;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x90));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                let t = true;
                                                if c.g[V0][0] == 0 {
                                                    c.g[T3][0] = c.g[T5][0];
                                                }
                                                if t {
                                                    break 'b_3763c0;
                                                } else {
                                                    break 'b_3763b8;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[T3][0] = c.g[T5][0];
                                            if c.g[V0][0] != 0 {
                                                c.g[T3][0] = 0u64;
                                            }
                                            break 'b_3763c0;
                                        }
                                        'b_376410: {
                                            'b_37640c: {
                                                'b_376408: {
                                                    'b_3763fc: {
                                                        c.cycles += 2;
                                                        if c.g[T0][0] == c.g[T7][0] {
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x94));
                                                            break 'b_3763fc;
                                                        } else {
                                                            'b_3763f0: {
                                                                'b_3763e0: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A1][0] == 0u64;

                                                                    if t {
                                                                        break 'b_3763e0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[T0][0] == c.g[T9][0]
                                                                        {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x94,
                                                                                ));
                                                                            break 'b_3763f0;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;

                                                                            if t {
                                                                                break 'b_376410;
                                                                            } else {
                                                                                break 'b_3763e0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                if c.g[T0][0] == c.g[T8][0] {
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x94));
                                                                    break 'b_376408;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;

                                                                    if t {
                                                                        break 'b_376410;
                                                                    } else {
                                                                        break 'b_3763f0;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[A0][0] = c.g[T5][0];
                                                            let t = true;
                                                            c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                                            if t {
                                                                break 'b_37640c;
                                                            } else {
                                                                break 'b_3763fc;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c.g[T5][0];
                                                    let t = true;
                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                    if t {
                                                        break 'b_37640c;
                                                    } else {
                                                        break 'b_376408;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] = c.g[T5][0];
                                                break 'b_37640c;
                                            }
                                            c.cycles += 1;
                                            if c.g[V0][0] != 0 {
                                                c.g[A0][0] = 0u64;
                                            }
                                            break 'b_376410;
                                        }
                                        'b_376428: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != c.g[T7][0];

                                            if t {
                                                break 'b_376428;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x98));
                                                c.g[T6][0] = c.g[T5][0];
                                                c.g[V0][0] = c.g[V0][0] ^ 0x31;
                                                if c.g[V0][0] != 0 {
                                                    c.g[T6][0] = 0u64;
                                                }
                                                break 'b_376428;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[T3][0] == 0u64 {
                                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                                            break 'b_376444;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A0][0] == 0u64 {
                                                c.g[T4][0] =
                                                    sx((c.g[T4][0] as u32).wrapping_add(0x1));
                                                break 'b_376444;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[T6][0] != 0u64;
                                                c.g[V0][0] = c.g[T4][0];
                                                if t {
                                                    break 'b_376454;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[T4][0] =
                                                        sx((c.g[T4][0] as u32).wrapping_add(0x1));
                                                    break 'b_376444;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[T1][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00376340);
                            }
                            continue 'l_376340;
                        } else {
                            break 'b_376450;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0xffffffffffffffff;
            break 'b_376454;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(18, (c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `int PS2GraphicsMan_SelectDisplayMode(PS2GraphicsMan* self, int index)`
/// (0x003764c0): switches to display mode `index` (`self + 0x59cc`, `0xa4`
/// bytes each; current mode at `+0x59c8`) by calling `Renderer_SetDisplayMode`
/// with its nine fields (the sixth from `+0x9c` as flag and its inverse).
/// Returns 1 (also when the mode is already current).
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code)]
pub fn ps2_graphics_man_select_display_mode(c: &mut Cpu) {
    guest(c, |c| {
        'b_376544: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x59c8));
            let t = c.g[V0][0] == c.g[S1][0];
            c.g[V0][0] = 0xa4;
            if t {
                break 'b_376544;
            } else {
                c.cycles += 24;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x59cc));
                c.mult(0, 2, 17, 2);
                c.sw((c.g[SP][0] as u32), 0u64);
                c.g[A2][0] = 0x1;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = c.g[A2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                if c.g[A1][0] == 0 {
                    c.g[V1][0] = 0u64;
                }
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                if c.g[A1][0] != 0 {
                    c.g[A2][0] = 0u64;
                }
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                c.g[RA][0] = sx(0x00376540);
                c.g[T2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                Callee {
                    addr: 0x0037bd98,
                    f: ee::s12::Renderer_SetDisplayMode,
                }
                .call(c, 0x00376540)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x59c8), c.g[S1][0]);
                break 'b_376544;
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0x1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC689_MAGIC` (otherwise the body they replaced runs).
    const OC689_CTRL: u32 = DATA + 0x000e_4000;
    const OC689_MAGIC: u32 = 0x0689_0a11;
    /// Results (sign-extended words) of the fakes, indexed by fake call n % 16.
    const OC689_RESULTS: u32 = DATA + 0x000e_4100;
    /// The graphics manager and its tables in the graphics tests.
    const OC689_GFX: u32 = DATA + 0x0001_0000;
    const OC689_MODES: u32 = DATA + 0x0002_0000;
    const OC689_SRCS: u32 = DATA + 0x0003_0000;
    const OC689_VTABLE: u32 = DATA + 0x0004_0000;
    const OC689_SEEDS: u32 = 120;

    static OC689_PREV_MOUNT: AtomicUsize = AtomicUsize::new(0);
    static OC689_PREV_SEARCH: AtomicUsize = AtomicUsize::new(0);
    static OC689_PREV_LOAD: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake of an SCE/EA callee: logs the call like any fake and
    /// answers the seeded (sign-extended) result.
    fn oc689_fake(c: &mut Cpu, prev: &AtomicUsize, generated: fn(&mut Cpu), target: u32) {
        if c.bus.read32(OC689_CTRL) != OC689_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = c.bus.read32(OC689_RESULTS + 4 * (n % 16)) as i32 as i64 as u64;
        fakes::fake_call(c, target);
        c.g[V0][0] = result;
    }

    /// `EAFile_MountBig`.
    fn oc689_fake_mount(c: &mut Cpu) {
        oc689_fake(
            c,
            &OC689_PREV_MOUNT,
            ee::s14::f_003ded50__generated,
            0x003d_ed50,
        )
    }

    /// `sceSifSearchModuleByName`.
    fn oc689_fake_search(c: &mut Cpu) {
        oc689_fake(
            c,
            &OC689_PREV_SEARCH,
            ee::s15::f_0042b6d0__generated,
            0x0042_b6d0,
        )
    }

    /// `sceSifLoadModule`.
    fn oc689_fake_load(c: &mut Cpu) {
        oc689_fake(
            c,
            &OC689_PREV_LOAD,
            ee::s15::f_0042ba68__generated,
            0x0042_ba68,
        )
    }

    fn oc689_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DED50),
            oc689_fake_mount,
            &OC689_PREV_MOUNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042B6D0),
            oc689_fake_search,
            &OC689_PREV_SEARCH,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042BA68),
            oc689_fake_load,
            &OC689_PREV_LOAD,
        );
    }

    fn oc689_put_cstr(c: &mut Cpu, addr: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// The strings `sprintf` formats in the module-path functions: the
    /// modules directory (`0x48d9d8` with the device `0x48d9c8`), the module
    /// name `MSIFRPC.IRX` (`0x48d9e8`) and the path format and suffix.
    fn oc689_put_module_strings(c: &mut Cpu) {
        oc689_put_cstr(c, 0x0048_d9c8, b"cdrom0:");
        oc689_put_cstr(c, 0x0048_d9d8, b"%sDATA\\MODULES\\");
        oc689_put_cstr(c, 0x0048_d9e8, b"MSIFRPC.IRX");
        oc689_put_cstr(c, 0x004a_3e98, b"%s%s%s");
        oc689_put_cstr(c, 0x004a_3ea0, b";1");
    }

    /// Fill the result table of the fakes: the first `bad` calls get
    /// `first` (a failure), later ones a positive result.
    fn oc689_results(c: &mut Cpu, r: &mut Rng, bad: u32, first: &[i32]) {
        for n in 0..16u32 {
            let v: i32 = if n < bad {
                first[r.next_u32() as usize % first.len()]
            } else {
                1 + (r.next_u32() % 9) as i32
            };
            c.bus.write32(OC689_RESULTS + 4 * n, v as u32);
        }
    }

    /// `ssx_MountSlusOvfBig`: one `EAFile_MountBig` call with fixed arguments.
    #[test]
    fn ssx_mount_slus_ovf_big_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s06::f_00256188__generated,
            ssx_mount_slus_ovf_big,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x0689_0001, 1);
                c.pc = 0x0025_6188;
                c.bus.write32(OC689_CTRL, OC689_MAGIC);
            },
        );
    }

    /// `ssx_GetModulesDir`: the real `sprintf` formats the modules path.
    #[test]
    fn ssx_get_modules_dir_matches() {
        check(
            ee::s10::f_00319660__generated,
            ssx_get_modules_dir,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0002, 0);
                c.pc = 0x0031_9660;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x1000 + (r.next_u32() & 0xf0)) as u64;
                oc689_put_module_strings(c);
                if seed % 4 == 3 {
                    oc689_put_cstr(c, 0x0048_d9c8, b"host0:");
                }
            },
        );
    }

    /// `ssx_BuildModulePath`: a name with a device is copied (`strcpy`), a
    /// bare one gets the modules directory and the suffix.
    #[test]
    fn ssx_build_module_path_matches() {
        check(
            ee::s10::f_00319690__generated,
            ssx_build_module_path,
            90,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0003, 0);
                c.pc = 0x0031_9690;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x3000 + (r.next_u32() & 0xf0)) as u64;
                c.g[A2][0] = (DATA + 0x1000) as u64;
                oc689_put_module_strings(c);
                let name: &[u8] = match seed % 3 {
                    0 => b"mod.irx",
                    1 => b"host0:mod.irx",
                    _ => b"",
                };
                oc689_put_cstr(c, DATA + 0x1000, name);
            },
        );
    }

    /// The fixture of `ssx_LoadIopModule`: `long` makes the first eight
    /// loads fail (busy, -204, or any failure) so the loops iterate.
    fn oc689_load_iop_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_0004, 6);
        c.pc = 0x0031_9718;
        c.bus.write32(OC689_CTRL, OC689_MAGIC);
        oc689_put_module_strings(c);
        let retry = seed % 2;
        let first: &[i32] = if retry == 1 {
            &[-204, -1, -5]
        } else {
            &[-204, -204, -1]
        };
        oc689_results(c, &mut r, if long { 8 } else { 3 }, first);
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.g[A1][0] = (DATA + 0x1000) as u64;
        c.g[A2][0] = retry as u64;
        c.g[A3][0] = (DATA + 0x2000) as u64;
        c.g[T0][0] = (r.next_u32() % 0x40) as u64;
        let name: &[u8] = if seed % 3 == 0 {
            b"host0:a.irx"
        } else {
            b"MSIFRPC.IRX"
        };
        oc689_put_cstr(c, DATA + 0x1000, name);
        r
    }

    /// `ssx_LoadIopModule`: success, a failure that is not retried, and the
    /// retry loops (any failure with `retry`, -204 without).
    #[test]
    fn ssx_load_iop_module_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_00319718__generated,
            ssx_load_iop_module,
            OC689_SEEDS,
            &|c, seed| {
                oc689_load_iop_setup(c, seed, false);
            },
        );
        check_events(
            ee::s10::f_00319718__generated,
            ssx_load_iop_module,
            OC689_SEEDS,
            &|c, seed| {
                let mut r = oc689_load_iop_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x0031_9718)
            },
        );
    }

    /// `ssx_LoadMsifrpc`: the fixed `MSIFRPC.IRX` load with retry.
    #[test]
    fn ssx_load_msifrpc_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_003197d0__generated,
            ssx_load_msifrpc,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0005, 6);
                c.pc = 0x0031_97d0;
                c.bus.write32(OC689_CTRL, OC689_MAGIC);
                oc689_put_module_strings(c);
                oc689_results(c, &mut r, 3, &[-204, -1, -5]);
                c.bus.write32(GAME_GP_IO, DATA + 0x100);
            },
        );
    }

    /// `gp + 0xda4` (`D_004A3E94`): the IOP loader object `ssx_LoadMsifrpc` passes on.
    const GAME_GP_IO: u32 = crate::difftest::GAME_GP + 0xda4;

    /// `ssx_InstallPowerOffHandler`: with and without the power-off module.
    #[test]
    fn ssx_install_power_off_handler_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_00319930__generated,
            ssx_install_power_off_handler,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_0006, 8);
                c.pc = 0x0031_9930;
                c.bus.write32(OC689_CTRL, OC689_MAGIC);
                let miss = if seed % 3 == 0 {
                    -1
                } else {
                    5 + (r.next_u32() % 5) as i32
                };
                for n in 0..16u32 {
                    c.bus.write32(OC689_RESULTS + 4 * n, miss as u32);
                }
            },
        );
    }

    /// `PS2ExecMan_Init`: the manager object lives at DATA; every syscall is a fake.
    #[test]
    fn ps2_exec_man_init_matches() {
        oc689_install_fakes();
        fakes::covered(
            ee::s10::f_0031a6d8__generated,
            ps2_exec_man_init,
            OC689_SEEDS,
            &|c, seed| {
                fakes::machine(c, seed, 0x0689_0007, 24);
                c.pc = 0x0031_a6d8;
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// The graphics manager's texture cache (`+0x280`, 12-byte entries,
    /// count `+0xe80`) and shape banks (`+0x27c`, count `+0x278`): a bank
    /// holds `count` shapes whose names the real `SHAPE_locatez` compares;
    /// the texture is created through vtable slot 46 (a fake).
    fn oc689_texture_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_0008, 3);
        c.pc = 0x0036_96c8;
        let names: [&[u8]; 5] = [b"abc", b"abcdefgh", b"wxyz", b"", b"q"];
        let name = names[(seed % 5) as usize];
        let want: Vec<u8> = name.iter().take(4).copied().collect();
        c.g[A0][0] = OC689_GFX as u64;
        c.g[A1][0] = (DATA + 0x1000) as u64;
        oc689_put_cstr(c, DATA + 0x1000, name);
        // Cache.
        let count = if long {
            30 + r.next_u32() % 8
        } else {
            r.next_u32() % 4
        };
        c.bus.write32(OC689_GFX + 0xe80, count);
        for i in 0..count {
            let e = OC689_GFX + 0x280 + 12 * i;
            c.bus.write32(e, r.next_u32());
            let pool: [&[u8]; 4] = [b"zzzz", b"q", b"wxyz", b"abc"];
            let other = pool[(r.next_u32() % 4) as usize];
            if !long && i + 1 == count && r.next_u32() % 3 == 0 {
                oc689_put_cstr(c, e + 4, &want);
            } else if long {
                oc689_put_cstr(c, e + 4, b"zzzz");
            } else {
                oc689_put_cstr(c, e + 4, other);
            }
        }
        // Shape banks.
        let banks = r.next_u32() % 3;
        c.bus.write32(OC689_GFX + 0x278, banks);
        c.bus.write32(OC689_GFX + 0x27c, OC689_SRCS);
        for j in 0..banks {
            let src = OC689_SRCS + 0x1000 * (j + 1);
            c.bus.write32(OC689_SRCS + 4 * j, src);
            let shapes = r.next_u32() % 3;
            c.bus.write32(src + 8, shapes);
            for k in 0..shapes {
                let off = 0x100 + 0x20 * k;
                c.bus.write32(src + 0x14 + 8 * k, off);
                c.bus.write32(src + off, 0x70);
                if r.next_u32() % 2 == 0 {
                    oc689_put_cstr(c, src + off + 4, &want);
                } else {
                    oc689_put_cstr(c, src + off + 4, b"nope");
                }
            }
        }
        // Vtable slot 46: `{delta, fn}` at +0x170, a fake.
        c.bus.write32(OC689_GFX + 0x10d8, OC689_VTABLE);
        c.bus
            .write16(OC689_VTABLE + 0x170, (r.next_u32() % 0x40 * 4) as u16);
        c.bus.write32(OC689_VTABLE + 0x174, fakes::FAKE);
        r
    }

    /// `PS2GraphicsMan_GetTextureByName`: cache hit, shape found (texture
    /// created by the vtable fake) and not found (-1), names of 0 to 8 characters.
    #[test]
    fn ps2_graphics_man_get_texture_by_name_matches() {
        check(
            ee::s12::f_003696c8__generated,
            ps2_graphics_man_get_texture_by_name,
            OC689_SEEDS,
            &|c, seed| {
                oc689_texture_setup(c, seed, false);
            },
        );
        check_events(
            ee::s12::f_003696c8__generated,
            ps2_graphics_man_get_texture_by_name,
            OC689_SEEDS,
            &|c, seed| {
                let mut r = oc689_texture_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x0036_96c8)
            },
        );
    }

    /// The display mode table of `PS2GraphicsMan_FindDisplayMode`: `count`
    /// modes of 0xa4 bytes whose fields mostly agree with the query.
    fn oc689_find_mode_setup(c: &mut Cpu, seed: u32, long: bool) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_0009, 0);
        c.pc = 0x0037_62f8;
        let depth = |r: &mut Rng| [16u32, 24, 32, 8][(r.next_u32() % 4) as usize];
        let want = |bits: u32| match bits {
            16 => 2,
            24 => 1,
            32 => 0,
            _ => 9,
        };
        let (w, h, id) = (640, 448, r.next_u32() % 3);
        let (bpp, clut) = (depth(&mut r), depth(&mut r));
        let fmt = if r.next_u32() % 4 == 0 { 0x31 } else { 24 };
        c.g[A0][0] = OC689_GFX as u64;
        c.g[A1][0] = w as u64;
        c.g[A2][0] = h as u64;
        c.g[A3][0] = bpp as u64;
        c.g[T0][0] = clut as u64;
        c.g[T1][0] = fmt as u64;
        c.g[T2][0] = id as u64;
        let count = if long {
            10 + r.next_u32() % 4
        } else {
            r.next_u32() % 6
        };
        c.bus.write32(OC689_GFX + 0x59c4, count);
        c.bus.write32(OC689_GFX + 0x59cc, OC689_MODES);
        for i in 0..count {
            let m = OC689_MODES + 0xa4 * i;
            let good = |r: &mut Rng| !long && r.next_u32() % 4 != 0;
            let width = if good(&mut r) { w } else { w + 1 };
            let height = if good(&mut r) { h } else { h + 1 };
            let mid = if good(&mut r) { id } else { id + 1 };
            let psm = if good(&mut r) {
                want(bpp)
            } else {
                r.next_u32() % 4
            };
            let cpsm = if good(&mut r) {
                want(clut)
            } else {
                r.next_u32() % 4
            };
            let mfmt = if good(&mut r) { 0x31 } else { 0x30 };
            c.bus.write32(m + 0x88, width);
            c.bus.write32(m + 0x8c, height);
            c.bus.write32(m + 0xa0, mid);
            c.bus.write32(m + 0x90, psm);
            c.bus.write32(m + 0x94, cpsm);
            c.bus.write32(m + 0x98, mfmt);
        }
        r
    }

    /// `PS2GraphicsMan_FindDisplayMode`: first mode matching width, height,
    /// id and the depth checks, or -1.
    #[test]
    fn ps2_graphics_man_find_display_mode_matches() {
        check(
            ee::s12::f_003762f8__generated,
            ps2_graphics_man_find_display_mode,
            300,
            &|c, seed| {
                oc689_find_mode_setup(c, seed, false);
            },
        );
        check_events(
            ee::s12::f_003762f8__generated,
            ps2_graphics_man_find_display_mode,
            OC689_SEEDS,
            &|c, seed| {
                let mut r = oc689_find_mode_setup(c, seed, true);
                schedule(c, &mut r, seed, 0x0037_62f8)
            },
        );
    }

    /// `PS2GraphicsMan_SelectDisplayMode`: the current mode returns at once;
    /// otherwise `Renderer_SetDisplayMode` runs for real and rejects the
    /// mode's frame format (`+0x94` >= 0x3b, an invalid PSM: it returns 0
    /// before touching the hardware).
    #[test]
    fn ps2_graphics_man_select_display_mode_matches() {
        check(
            ee::s12::f_003764c0__generated,
            ps2_graphics_man_select_display_mode,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0689_000a, 0);
                c.pc = 0x0037_64c0;
                let index = r.next_u32() % 4;
                c.g[A0][0] = OC689_GFX as u64;
                c.g[A1][0] = index as u64;
                let current = if seed % 3 == 0 { index } else { index + 1 };
                c.bus.write32(OC689_GFX + 0x59c8, current);
                c.bus.write32(OC689_GFX + 0x59cc, OC689_MODES);
                for i in 0..5 {
                    let m = OC689_MODES + 0xa4 * i;
                    for f in (0x80..=0xa0).step_by(4) {
                        c.bus.write32(m + f, r.next_u32() % 4);
                    }
                    c.bus.write32(m + 0x94, 0x3b + r.next_u32() % 0x40);
                    c.bus.write32(m + 0x9c, r.next_u32() % 2);
                }
            },
        );
    }

    /// The fixture of `PS2GraphicsMan_BuildDisplayModeTable`: the built-in
    /// table at `0x44b440` (8 modes) with sizes and depth fields.
    fn oc689_build_table_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = fakes::machine(c, seed, 0x0689_000b, 16);
        c.pc = 0x0037_bb10;
        c.g[A0][0] = OC689_GFX as u64;
        oc689_put_cstr(c, 0x0049_28e8, b"%d %d %d");
        for i in 0..8u32 {
            let m = 0x0044_b440 + 0xa4 * i;
            c.bus
                .write32(m + 0x88, [640, 512, 320][(r.next_u32() % 3) as usize]);
            c.bus
                .write32(m + 0x8c, [448, 480, 240][(r.next_u32() % 3) as usize]);
            let big = r.next_u32() % 4 != 0;
            c.bus.write32(
                m + 0x90,
                if big {
                    0x3b + r.next_u32() % 8
                } else {
                    r.next_u32() % 4
                },
            );
            c.bus.write32(
                m + 0x94,
                if big {
                    0x3b + r.next_u32() % 8
                } else {
                    r.next_u32() % 3
                },
            );
            c.bus.write32(m + 0x98, r.next_u32() % 0x40);
        }
        r
    }
}
