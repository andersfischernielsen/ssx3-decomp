//! Staging module oc1020 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1020`.

#![allow(unused_imports)]

use crate::abi;
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
    (0x003f_6c50, slogin_set_active, "SLogin_SetActive"),
    (0x003f_6c60, slogin_clear_active, "SLogin_ClearActive"),
    (0x003f_6c70, slogin_alloc_state, "SLogin_AllocState"),
    (0x003f_6d98, slogin_free_state, "SLogin_FreeState"),
    (0x003f_6dc8, slogin_init_store, "SLogin_InitStore"),
    (0x003f_6e70, slogin_shutdown_store, "SLogin_ShutdownStore"),
    (0x003f_6e98, slogin_start_op, "SLogin_StartOp"),
    (0x003f_6ee0, slogin_op_update, "SLogin_OpUpdate"),
    (0x003f_6f68, slogin_step_update, "SLogin_StepUpdate"),
    (0x003f_71b8, slogin_on_step_error, "SLogin_OnStepError"),
    (0x003f_71f8, slogin_on_final_error, "SLogin_OnFinalError"),
    (0x003f_7218, slogin_on_step_done, "SLogin_OnStepDone"),
    (0x003f_72c8, slogin_on_final_done, "SLogin_OnFinalDone"),
    (
        0x003f_72f8,
        slogin_on_error_close_done,
        "SLogin_OnErrorCloseDone",
    ),
    (0x003f_7328, slogin_begin_sequence, "SLogin_BeginSequence"),
    (0x003f_73a8, slogin_step_create, "SLogin_StepCreate"),
    (0x003f_73f0, slogin_step_open, "SLogin_StepOpen"),
    (0x003f_7438, slogin_step_delete, "SLogin_StepDelete"),
    (0x003f_7478, slogin_step_write_acct, "SLogin_StepWriteAcct"),
    (0x003f_7528, slogin_step_write_pass, "SLogin_StepWritePass"),
];

const ACTIVE_GP_OFFSET: u32 = 0x1ad0;
const LOGIN_STATE_GP_OFFSET: u32 = 0x1ad4;
const STORE_OPERATION_OFFSET: u32 = 0xd0;
const LOGIN_FINAL_ERROR_OFFSET: u32 = 0x148;
const STORE_STEP_OFFSET: u32 = 0xd8;
const LOGIN_STORE_TEMPLATE: u32 = 0x0049_7060;
const STEP_TABLE: u32 = 0x0049_7080;

mod callee {
    use super::*;

    pub(crate) const SLOGIN_RESET_FOR_OP: Callee = Callee {
        addr: 0x003f_7990,
        f: ee::s14::SLogin_ResetForOp,
    };
    pub(crate) const SLOGIN_END_SEQUENCE: Callee = Callee {
        addr: 0x003f_7bd0,
        f: ee::s14::SLogin_EndSequence,
    };
    pub(crate) const SLOGIN_SET_STEP: Callee = Callee {
        addr: 0x003f_7a60,
        f: ee::s14::SLogin_SetStep,
    };
    pub(crate) const SLOGIN_MAP_STORE_ERROR: Callee = Callee {
        addr: 0x003f_8178,
        f: ee::s14::SLogin_MapStoreError,
    };
    pub(crate) const SLOGIN_CLEAR_STEP_FUNCS: Callee = Callee {
        addr: 0x003f_7d98,
        f: ee::s14::SLogin_ClearStepFuncs,
    };
}

/// Mark the shared login as active and return success.
pub fn slogin_set_active(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 1;
    c.g[V0][0] = 0;
    let active = (c.g[GP][0] as u32).wrapping_add(ACTIVE_GP_OFFSET);
    let active_value = c.g[V1][0];
    Addr(active).sb(c, active_value);
    ret(c);
}

/// Clear the shared login's active flag and return success.
pub fn slogin_clear_active(c: &mut Cpu) {
    c.cycles += 3;
    let active = (c.g[GP][0] as u32).wrapping_add(ACTIVE_GP_OFFSET);
    Addr(active).sb(c, 0);
    c.g[V0][0] = 0;
    ret(c);
}

/// Allocate and initialize the shared-login state from its startup settings.
pub fn slogin_alloc_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x14c;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 4;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
        call_ptr(c, 0x0026_6728, 0x003f_6c90)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        let state = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).sw(c, state);
        c.g[A2][0] = 0x14c;
        call_ptr(c, 0x0041_6210, 0x003f_6ca4)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        let original_store = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x140)).sw(c, original_store);
        c.g[A0][0] = c.g[V1][0];
        c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = c.g[S0][0] | c.g[A0][0];
        c.g[V0][0] &= 7;
        let aligned = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        if !aligned {
            loop {
                c.cycles += 19;
                let source = c.g[S0][0] as u32;
                let destination = c.g[A0][0] as u32;
                c.ldl(3, source.wrapping_add(7));
                c.ldr(3, source);
                c.ldl(5, source.wrapping_add(15));
                c.ldr(5, source.wrapping_add(8));
                c.ldl(6, source.wrapping_add(23));
                c.ldr(6, source.wrapping_add(16));
                c.ldl(7, source.wrapping_add(31));
                c.ldr(7, source.wrapping_add(24));
                c.sdl(3, destination.wrapping_add(7));
                c.sdr(3, destination);
                c.sdl(5, destination.wrapping_add(15));
                c.sdr(5, destination.wrapping_add(8));
                c.sdl(6, destination.wrapping_add(23));
                c.sdr(6, destination.wrapping_add(16));
                c.sdl(7, destination.wrapping_add(31));
                c.sdr(7, destination.wrapping_add(24));
                c.g[S0][0] = sx(source.wrapping_add(0x20));
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = sx(destination.wrapping_add(0x20));
                if !more {
                    c.cycles += 2;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_6cc8);
                }
            }
        } else {
            loop {
                c.cycles += 11;
                let source = c.g[S0][0] as u32;
                let destination = c.g[A0][0] as u32;
                c.g[V1][0] = Addr(source).ld(c);
                c.g[A1][0] = Addr(source.wrapping_add(8)).ld(c);
                c.g[A2][0] = Addr(source.wrapping_add(16)).ld(c);
                c.g[A3][0] = Addr(source.wrapping_add(24)).ld(c);
                let v1 = c.g[V1][0];
                let a1 = c.g[A1][0];
                let a2 = c.g[A2][0];
                let a3 = c.g[A3][0];
                Addr(destination).sd(c, v1);
                Addr(destination.wrapping_add(8)).sd(c, a1);
                Addr(destination.wrapping_add(16)).sd(c, a2);
                Addr(destination.wrapping_add(24)).sd(c, a3);
                c.g[S0][0] = sx(source.wrapping_add(0x20));
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = sx(destination.wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_6d1c);
                }
            }
        }
        c.cycles += 19;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A3][0] = 5;
        let trailing = c.g[V0][0];
        Addr(c.g[A0][0] as u32).sw(c, trailing);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[A1][0] = 4;
        c.g[A2][0] = 10;
        c.g[A0][0] = u64::MAX;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        let login = c.g[V1][0] as u32;
        let a0 = c.g[A0][0];
        let a1 = c.g[A1][0];
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        Addr(login.wrapping_add(0x13c)).sw(c, a0);
        Addr(login.wrapping_add(0x104)).sw(c, a1);
        Addr(login.wrapping_add(0xd8)).sw(c, a2);
        Addr(login.wrapping_add(0xec)).sw(c, a3);
        Addr(login.wrapping_add(0xd0)).sw(c, a1);
        Addr(login.wrapping_add(0x148)).sw(c, 0);
        Addr(login.wrapping_add(0x138)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Release the shared-login state allocation and clear its global pointer.
pub fn slogin_free_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x14c;
        c.g[A3][0] = 4;
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x140)).lw(c);
        call_ptr(c, 0x0026_6760, 0x003f_6db4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Initialize the shared-login record store from its static configuration.
pub fn slogin_init_store(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = 0x0049_0000;
        c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7060));
        let template = LOGIN_STORE_TEMPLATE;
        c.ldl(7, template.wrapping_add(7));
        c.ldr(7, template);
        c.ldl(8, template.wrapping_add(15));
        c.ldr(8, template.wrapping_add(8));
        c.ldl(9, template.wrapping_add(23));
        c.ldr(9, template.wrapping_add(16));
        let config = frame.at(c, 0);
        c.sdl(7, config.wrapping_add(7));
        c.sdr(7, config);
        c.sdl(8, config.wrapping_add(15));
        c.sdr(8, config.wrapping_add(8));
        c.sdl(9, config.wrapping_add(23));
        c.sdr(9, config.wrapping_add(16));
        c.g[A3][0] = Addr(template.wrapping_add(24)).lb(c);
        c.g[T0][0] = Addr(template.wrapping_add(25)).lb(c);
        let a3 = c.g[A3][0];
        let t0 = c.g[T0][0];
        Addr(config.wrapping_add(24)).sb(c, a3);
        Addr(config.wrapping_add(25)).sb(c, t0);
        c.g[A2][0] = 0x1a;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x138));
        c.g[A0][0] = 2;
        let a0 = c.g[A0][0];
        let v0 = c.g[V0][0];
        let a1 = c.g[A1][0];
        let a2 = c.g[A2][0];
        let stack = c.g[SP][0];
        Addr(frame.at(c, 0x20)).sw(c, a0);
        Addr(frame.at(c, 0x24)).sw(c, v0);
        Addr(frame.at(c, 0x28)).sw(c, a1);
        Addr(frame.at(c, 0x30)).sw(c, a2);
        Addr(frame.at(c, 0x2c)).sw(c, stack);
        call_ptr(c, 0x003f_b800, 0x003f_6e40)?;
        let cipher_iface = c.g[V0][0];
        Addr(frame.at(c, 0x34)).sw(c, cipher_iface);
        c.g[A0][0] = sx(frame.at(c, 0x20));
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x140)).lw(c);
        let cache = c.g[V1][0];
        Addr(frame.at(c, 0x38)).sw(c, cache);
        call_ptr(c, 0x003f_c0e8, 0x003f_6e58)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_6e60)?;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// Shut down the shared-login record store and translate its final error.
pub fn slogin_shutdown_store(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003f_c260, 0x003f_6e80)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_6e88)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Start a shared-login operation when the store is in its ready state.
pub fn slogin_start_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let login = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[A1][0] = login;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 4;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((login as u32).wrapping_add(STORE_OPERATION_OFFSET)).lw(c);
        c.g[S0][0] = 0;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 4;
            callee::SLOGIN_RESET_FOR_OP.call(c, 0x003f_6ec0)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0x16;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Poll a shared-login operation, returning its current or terminal status.
pub fn slogin_op_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        let result_address = c.g[A0][0] as u32;
        Addr(result_address).sw(c, 0);
        frame.save_ra(c, 0x10);
        let has_result = c.g[A0][0] != 0;
        Addr(frame.at(c, 0)).sw(c, 0);
        if !has_result {
            c.cycles += 2;
            c.g[V0][0] = 7;
            let error = c.g[V0][0];
            Addr(frame.at(c, 0)).sw(c, error);
        }
        c.cycles += 5;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(STORE_OPERATION_OFFSET)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < 4) as u64;
        let operation_active = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        if !operation_active {
            c.cycles += 3;
            c.g[V0][0] = 15;
            let error = c.g[V0][0];
            Addr(frame.at(c, 0)).sw(c, error);
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        }
        c.cycles += 2;
        let has_error = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        if !has_error {
            c.cycles += 5;
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(STORE_STEP_OFFSET)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 9) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe8)).lw(c);
                c.cycles += 2;
                let target = c.g[V0][0] as u32;
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, target, 0x003f_6f54)?;
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xdc)).lw(c);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x003f_6f44)?;
                let result = c.g[V0][0];
                Addr(frame.at(c, 0)).sw(c, result);
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

fn slogin_copy_step_snapshot(c: &mut Cpu, login: u32, scratch: [usize; 3]) {
    for (index, register) in scratch.into_iter().enumerate() {
        let source = login.wrapping_add(0xec + 8 * index as u32);
        c.ldl(register, source.wrapping_add(7));
        c.ldr(register, source);
    }
    for (index, register) in scratch.into_iter().enumerate() {
        let destination = login.wrapping_add(0x120 + 8 * index as u32);
        c.sdl(register, destination.wrapping_add(7));
        c.sdr(register, destination);
    }
}

/// Poll the shared-login step callbacks and dispatch completion or error handling.
pub fn slogin_step_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        Addr(frame.at(c, 0x0c)).sw(c, 0);
        let has_result = c.g[S0][0] != 0;
        Addr(c.g[S0][0] as u32).sw(c, 0);
        if !has_result {
            c.cycles += 2;
            c.g[V0][0] = 7;
            Addr(frame.at(c, 0x0c)).sw(c, 7);
        }

        c.cycles += 5;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(STORE_STEP_OFFSET)).lw(c);
        c.g[V1][0] = (c.g[A0][0] < 9) as u64;
        let valid_step = c.g[V1][0] != 0;
        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
        if !valid_step {
            c.cycles += 3;
            c.g[V0][0] = 10;
            let not_terminal = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = 13;
            if !not_terminal {
                c.cycles += 1;
                c.g[V0][0] = 12;
            }
            c.cycles += 2;
            let error = c.g[V0][0];
            Addr(frame.at(c, 0x0c)).sw(c, error);
            c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
        }

        c.cycles += 2;
        let mut has_error = c.g[V0][0] != 0;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        if !has_error {
            c.cycles += 5;
            c.g[A1][0] = c.g[V1][0];
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xec)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 6) as u64;
            let valid_phase = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0049_0000;
            if !valid_phase {
                c.cycles += 1;
                c.g[V0][0] = 17;
                let error = c.g[V0][0];
                c.cycles += 1;
                Addr(frame.at(c, 0x0c)).sw(c, error);
                c.cycles += 1;
                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx(STEP_TABLE);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                match target {
                    0x003f_6fec => {
                        c.cycles += 17;
                        let login = c.g[A1][0] as u32;
                        slogin_copy_step_snapshot(c, login, [V0, V1, A0]);
                        c.g[V1][0] = 1;
                        c.g[V0][0] =
                            Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                        Addr((c.g[V0][0] as u32).wrapping_add(0xec)).sw(c, 0);
                        Addr(c.g[S0][0] as u32).sw(c, 1);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    0x003f_7030 => {
                        c.cycles += 6;
                        c.g[V1][0] =
                            Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xf0)).lw(c);
                        let target = c.g[V0][0] as u32;
                        call_ptr(c, target, 0x003f_7040)?;
                        let failed = c.g[V0][0] != 0;
                        let result = c.g[V0][0];
                        Addr(frame.at(c, 0x0c)).sw(c, result);
                        if !failed {
                            c.cycles += 18;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                            c.g[A0][0] = 1;
                            let login = c.g[V0][0] as u32;
                            slogin_copy_step_snapshot(c, login, [A3, T0, V1]);
                            c.g[V1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                            Addr((c.g[V1][0] as u32).wrapping_add(0xec)).sw(c, 1);
                            Addr(c.g[S0][0] as u32).sw(c, 1);
                            c.cycles += 1;
                            c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                            c.g[A1][0] = 2;
                            c.g[A0][0] = c.g[SP][0];
                            slogin_step_failure_cleanup(c, frame)?;
                        }
                    }
                    0x003f_70a0 => {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xf4)).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = c.g[SP][0] | 4;
                        call_ptr(c, target, 0x003f_70ac)?;
                        let failed = c.g[V0][0] != 0;
                        let result = c.g[V0][0];
                        Addr(frame.at(c, 0x0c)).sw(c, result);
                        if !failed {
                            c.cycles += 5;
                            c.g[V1][0] = 1;
                            c.g[A0][0] = Addr(frame.at(c, 4)).lw(c);
                            c.g[V0][0] = 2;
                            let incomplete = c.g[A0][0] != c.g[V0][0];
                            Addr(c.g[S0][0] as u32).sw(c, 1);
                            if incomplete {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                            } else {
                                c.cycles += 4;
                                let completed = c.g[A0][0];
                                Addr(c.g[S0][0] as u32).sw(c, completed);
                                callee::SLOGIN_CLEAR_STEP_FUNCS.call(c, 0x003f_70d0)?;
                                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                            c.g[A1][0] = 2;
                            c.g[A0][0] = c.g[SP][0] | 8;
                            slogin_step_failure_cleanup(c, frame)?;
                        }
                    }
                    0x003f_7134 => {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xf8)).lw(c);
                        let target = c.g[V0][0] as u32;
                        call_ptr(c, target, 0x003f_7140)?;
                        let result = c.g[V0][0];
                        Addr(frame.at(c, 0x0c)).sw(c, result);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    0x003f_7148 => {
                        c.cycles += 2;
                        c.g[V0][0] = 16;
                        c.cycles += 1;
                        Addr(frame.at(c, 0x0c)).sw(c, 16);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    0x003f_7150 => {
                        c.cycles += 1;
                        c.g[V0][0] = 17;
                        c.cycles += 1;
                        Addr(frame.at(c, 0x0c)).sw(c, 17);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    target => {
                        c.jump_out(target, 0x003f_6f68, 0x003f_71b4);
                        return Ok(());
                    }
                }
            }
            c.cycles += 2;
            has_error = c.g[V0][0] != 0;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
            if !has_error {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = 2;
                let complete = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                if complete {
                    c.cycles += 7;
                    c.g[V1][0] =
                        Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe0)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A1][0] = c.g[SP][0] | 0x0c;
                    call_ptr(c, target, 0x003f_7188)?;
                    c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                }
            }
        }
        if has_error {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[SP][0] | 0x0c;
            call_ptr(c, target, 0x003f_71a0)?;
            c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

fn slogin_step_failure_cleanup(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 20;
    let login = c.g[V0][0] as u32;
    slogin_copy_step_snapshot(c, login, [A2, A3, T0]);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xdc)).lw(c);
    let target = c.g[V0][0] as u32;
    let phase = c.g[A1][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0xec)).sw(c, phase);
    call_ptr(c, target, 0x003f_7124)?;
    callee::SLOGIN_CLEAR_STEP_FUNCS.call(c, 0x003f_712c)?;
    c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
    Ok(())
}

/// Save a login-step error and switch the sequence to its error step.
pub fn slogin_on_step_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        let step_result = c.g[A0][0] as u32;
        let in_progress = c.g[V0][0];
        Addr(step_result).sw(c, in_progress);
        c.g[A1][0] = 0;
        c.g[A0][0] = 7;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        let state = c.g[V1][0] as u32;
        let error = c.g[V0][0];
        Addr(state.wrapping_add(LOGIN_FINAL_ERROR_OFFSET)).sw(c, error);
        let error_address = c.g[A2][0] as u32;
        Addr(error_address).sw(c, 0);
        callee::SLOGIN_SET_STEP.call(c, 0x003f_71e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// End a shared-login sequence after its final step reports an error.
pub fn slogin_on_final_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_7208)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Advance the login sequence after one store step finishes.
pub fn slogin_on_step_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        let result_address = c.g[A2][0] as u32;
        Addr(result_address).sw(c, 1);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(STORE_STEP_OFFSET)).lw(c);
        c.g[V1][0] = (c.g[A0][0] < 6) as u64;
        c.g[V0][0] = 0x0049_0000;

        let mut next_step = None;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 12;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70a0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x003f_7258 => {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    next_step = Some(1);
                }
                0x003f_7260 => {
                    c.cycles += 5;
                    c.g[V0][0] =
                        Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
                    c.g[A0][0] =
                        Addr((c.g[V0][0] as u32).wrapping_add(STORE_OPERATION_OFFSET)).lw(c);
                    c.g[V1][0] = (c.g[A0][0] < 2) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = 2;
                        next_step = Some(2);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 2;
                        let is_other_operation = c.g[A0][0] != c.g[V0][0];
                        c.g[V0][0] = 12;
                        if !is_other_operation {
                            c.cycles += 2;
                            c.g[A0][0] = 4;
                            next_step = Some(4);
                        }
                    }
                }
                0x003f_7288 => {
                    c.cycles += 2;
                    c.g[A0][0] = 3;
                    next_step = Some(3);
                }
                0x003f_7290 => {
                    c.cycles += 2;
                    c.g[A0][0] = 5;
                    next_step = Some(5);
                }
                0x003f_7298 => {
                    c.cycles += 1;
                    c.g[A0][0] = 6;
                    next_step = Some(6);
                }
                target => {
                    c.jump_out(target, 0x003f_7218, 0x003f_72c4);
                    return Ok(());
                }
            }
        }
        if next_step.is_some() {
            c.cycles += 4;
            c.g[A1][0] = 1;
            callee::SLOGIN_SET_STEP.call(c, 0x003f_72a4)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            let error_address = c.g[A1][0] as u32;
            let error = c.g[V0][0];
            Addr(error_address).sw(c, error);
            let result_address = c.g[A2][0] as u32;
            Addr(result_address).sw(c, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Complete the final login step and record a successful result.
pub fn slogin_on_final_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_72dc)?;
        c.g[V1][0] = 2;
        frame.restore_ra(c, 0);
        let result_address = c.g[S0][0] as u32;
        let result_value = c.g[V1][0];
        Addr(result_address).sw(c, result_value);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Forward a failed close result and end the shared-login sequence.
pub fn slogin_on_error_close_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(LOGIN_FINAL_ERROR_OFFSET)).lw(c);
        let result_address = c.g[A1][0] as u32;
        let error = c.g[V1][0];
        Addr(result_address).sw(c, error);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        Addr((c.g[V0][0] as u32).wrapping_add(LOGIN_FINAL_ERROR_OFFSET)).sw(c, 0);
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_7318)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Begin the next shared-login step when the previous sequence has finished.
pub fn slogin_begin_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = 9;
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(STORE_STEP_OFFSET)).lw(c);
        let ready = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 12;
        if !ready {
            c.cycles += 1;
            let failure_address = c.g[A1][0] as u32;
            let error = c.g[V0][0];
            Addr(failure_address).sw(c, error);
        } else {
            c.cycles += 7;
            c.g[V0][0] = 1;
            let result_address = c.g[A0][0] as u32;
            let in_progress = c.g[V0][0];
            Addr(result_address).sw(c, in_progress);
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(STORE_OPERATION_OFFSET)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 3) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                let first_op = c.g[V1][0] == 0;
                c.g[A0][0] = 0;
                if !first_op {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                }
                c.cycles += 4;
                c.g[A1][0] = 1;
                callee::SLOGIN_SET_STEP.call(c, 0x003f_738c)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                let third_op = c.g[V1][0] == 3;
                c.g[V0][0] = 12;
                if third_op {
                    c.cycles += 1;
                    c.g[A0][0] = 8;
                    c.cycles += 4;
                    c.g[A1][0] = 1;
                    callee::SLOGIN_SET_STEP.call(c, 0x003f_738c)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                    let failure_address = c.g[A1][0] as u32;
                    let error = c.g[V0][0];
                    Addr(failure_address).sw(c, error);
                    c.cycles += 2;
                    let result_address = c.g[A0][0] as u32;
                    Addr(result_address).sw(c, 0);
                    frame.restore_ra(c, 0);
                }
            }
        }
        if !ready {
            c.cycles += 2;
            let result_address = c.g[A0][0] as u32;
            Addr(result_address).sw(c, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

fn slogin_file_step(
    c: &mut Cpu,
    callback: Option<u32>,
    store_call: u32,
    return_at: u32,
    map_at: u32,
) {
    guest(c, |c| {
        c.cycles += if callback.is_some() { 17 } else { 15 };
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        if let Some(callback_address) = callback {
            c.g[V1][0] = 0x003f_0000;
            frame.save_ra(c, 0);
            c.g[V1][0] = sx(callback_address);
            c.g[A0][0] = 0x004a_0000;
            let login = c.g[V0][0] as u32;
            let callback_value = c.g[V1][0];
            Addr(login.wrapping_add(0xfc)).sw(c, callback_value);
        } else {
            frame.save_ra(c, 0);
            c.g[A0][0] = 0x004a_0000;
        }
        c.g[A1][0] = u64::MAX;
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4bc8));
        let login = c.g[V0][0] as u32;
        Addr(login.wrapping_add(0x100)).sw(c, 0);
        if callback.is_none() {
            Addr(login.wrapping_add(0xfc)).sw(c, 0);
        }
        call_ptr(c, store_call, return_at)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, map_at)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Start creating the memory-card record for the shared login.
pub fn slogin_step_create(c: &mut Cpu) {
    slogin_file_step(c, Some(0x003f_77f8), 0x003f_c390, 0x003f_73d8, 0x003f_73e0);
}

/// Start opening the memory-card record for the shared login.
pub fn slogin_step_open(c: &mut Cpu) {
    slogin_file_step(c, Some(0x003f_7820), 0x003f_c4c8, 0x003f_7420, 0x003f_7428);
}

/// Start deleting the shared-login memory-card record.
pub fn slogin_step_delete(c: &mut Cpu) {
    slogin_file_step(c, None, 0x003f_c3c0, 0x003f_7460, 0x003f_7468);
}

fn slogin_write_record_step(
    c: &mut Cpu,
    poll_callback: u32,
    done_callback: u32,
    data_offset: u32,
    tag: u32,
    first_return: u32,
    first_map_return: u32,
    write_return: u32,
    final_map_return: u32,
) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x003f_0000;
        c.g[A2][0] = sx(poll_callback);
        c.g[V1][0] = 0x003f_0000;
        c.g[V1][0] = sx(done_callback);
        let login = c.g[V0][0] as u32;
        let callback_value = c.g[A2][0];
        Addr(login.wrapping_add(0x100)).sw(c, callback_value);
        c.g[A1][0] = Addr(login.wrapping_add(0x140)).lw(c);
        c.g[A0][0] = sx(login.wrapping_add(0x144));
        let done_value = c.g[V1][0];
        Addr(login.wrapping_add(0xfc)).sw(c, done_value);
        c.g[A2][0] = 0x11;
        c.g[A3][0] = 0;
        call_ptr(c, 0x003f_ce10, first_return)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, first_map_return)?;
        let failed = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !failed {
            c.cycles += 23;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
            c.g[A1][0] = (tag & 0xffff_0000) as u64;
            c.g[A1][0] |= (tag & 0xffff) as u64;
            c.g[A2][0] = 0;
            let login = c.g[V1][0] as u32;
            c.g[V0][0] = Addr(login.wrapping_add(0x144)).lw(c);
            c.g[T0][0] = 0x11;
            c.ldl(4, login.wrapping_add(data_offset + 7));
            c.ldr(4, login.wrapping_add(data_offset));
            c.ldl(7, login.wrapping_add(data_offset + 15));
            c.ldr(7, login.wrapping_add(data_offset + 8));
            c.g[T1][0] = Addr(login.wrapping_add(data_offset + 16)).lb(c);
            let block = c.g[V0][0] as u32;
            c.sdl(4, block.wrapping_add(7));
            c.sdr(4, block);
            c.sdl(7, block.wrapping_add(15));
            c.sdr(7, block.wrapping_add(8));
            let trailing_byte = c.g[T1][0];
            Addr(block.wrapping_add(16)).sb(c, trailing_byte);
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP_OFFSET)).lw(c);
            c.g[A3][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x144)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
            call_ptr(c, 0x003f_c548, write_return)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_MAP_STORE_ERROR.call(c, final_map_return)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Write the shared-login account record to the memory card.
pub fn slogin_step_write_acct(c: &mut Cpu) {
    slogin_write_record_step(
        c,
        0x003f_7920,
        0x003f_78e0,
        0xa8,
        0x4143_4354,
        0x003f_74b0,
        0x003f_74b8,
        0x003f_7510,
        0x003f_7518,
    );
}

/// Write the shared-login password record to the memory card.
pub fn slogin_step_write_pass(c: &mut Cpu) {
    slogin_write_record_step(
        c,
        0x003f_7958,
        0x003f_7900,
        0xb9,
        0x5041_5353,
        0x003f_7560,
        0x003f_7568,
        0x003f_75c0,
        0x003f_75c8,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS, LOG_UNWIND_AT};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    const ALLOC_CTRL: u32 = DATA + 0x2ff00;
    const ALLOC_MAGIC: u32 = 0x736c_6f67;

    fn slogin_fake_alloc(c: &mut Cpu) {
        let previous = fakes::prev_cell(slogin_fake_alloc);
        if c.bus.read32(ALLOC_CTRL) != ALLOC_MAGIC {
            return fakes::fall_back(c, previous, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, 0x0026_6728);
        if c.bus.read32(ALLOC_CTRL + 4) == 0 {
            c.g[V0][0] = 0;
        }
    }

    fn hook_slogin_alloc() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            slogin_fake_alloc,
            fakes::prev_cell(slogin_fake_alloc),
        );
    }

    #[test]
    fn slogin_set_active_matches() {
        check(
            ee::s14::f_003f6c50__generated,
            slogin_set_active,
            32,
            &|c, seed| {
                let mut r = Rng(0x3f6c_50 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_6c50;
            },
        );
    }

    #[test]
    fn slogin_clear_active_matches() {
        check(
            ee::s14::f_003f6c60__generated,
            slogin_clear_active,
            32,
            &|c, seed| {
                let mut r = Rng(0x3f6c_60 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_6c60;
            },
        );
    }

    #[test]
    fn slogin_alloc_state_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f6c70__generated,
            slogin_alloc_state,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6c_70, 1);
                c.pc = 0x003f_6c70;
                let config = DATA + 0x300;
                let source = DATA + 0x1000 + seed % 2;
                c.g[A0][0] = sx(config);
                c.bus.write32(config, source);
                c.bus.write32(config + 4, DATA + 0x4000);
                for index in 0..0x44 {
                    c.bus.write8(source + index, (index ^ seed) as u8);
                }
                c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
                c.bus.write32(ALLOC_CTRL + 4, 1);
                c.bus.write32(LOG_RESULTS, DATA + 0x2000);
            },
        );
    }

    #[test]
    fn slogin_start_op_matches() {
        check(
            ee::s14::f_003f6e98__generated,
            slogin_start_op,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6e_98, 0);
                c.pc = 0x003f_6e98;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = (seed % 4) as u64;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
                c.bus.write32(
                    DATA + 0x200 + STORE_OPERATION_OFFSET,
                    if seed % 2 == 0 { 4 } else { 7 },
                );
            },
        );
    }

    #[test]
    fn slogin_op_update_matches() {
        check(
            ee::s14::f_003f6ee0__generated,
            slogin_op_update,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6e_e0, 1);
                c.pc = 0x003f_6ee0;
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { sx(DATA + 0x500) };
                c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
                c.bus.write32(
                    DATA + 0x200 + STORE_OPERATION_OFFSET,
                    if seed % 4 == 3 { 5 } else { 0 },
                );
                c.bus.write32(
                    DATA + 0x200 + STORE_STEP_OFFSET,
                    if seed % 4 == 2 { 9 } else { 0 },
                );
                c.bus.write32(DATA + 0x200 + 0xdc, FAKE);
                c.bus.write32(DATA + 0x200 + 0xe8, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn slogin_step_update_matches() {
        check(
            ee::s14::f_003f6f68__generated,
            slogin_step_update,
            56,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6f_68, 4);
                c.pc = 0x003f_6f68;
                let scenario = seed % 14;
                if seed < 14 {
                    c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                }
                c.g[A0][0] = if scenario == 12 { 0 } else { sx(DATA + 0x500) };
                c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
                c.bus.write32(
                    DATA + 0x200 + STORE_STEP_OFFSET,
                    match scenario {
                        10 => 9,
                        11 => 10,
                        _ => 0,
                    },
                );
                c.bus.write32(DATA + 0x200 + STORE_OPERATION_OFFSET, 0);
                let stage = match scenario {
                    0 | 1 => 0,
                    2 | 3 | 4 => 1,
                    5 | 6 => 2,
                    7 => 3,
                    8 => 4,
                    9 => 5,
                    13 => 6,
                    _ => 0,
                };
                c.bus.write32(DATA + 0x200 + 0xec, stage);
                c.bus.write32(
                    DATA + 0x200 + 0xf0,
                    if scenario == 0 { 0x0014_06c0 } else { FAKE },
                );
                c.bus.write32(
                    DATA + 0x200 + 0xf4,
                    if scenario == 2 || scenario == 3 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x10
                    },
                );
                c.bus.write32(
                    DATA + 0x200 + 0xf8,
                    if scenario == 5 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x20
                    },
                );
                c.bus.write32(DATA + 0x200 + 0xdc, FAKE + 0x30);
                c.bus.write32(DATA + 0x200 + 0xe0, FAKE + 0x40);
                c.bus.write32(DATA + 0x200 + 0xe4, FAKE + 0x50);
                c.bus.write32(
                    crate::difftest::SP - 0x30 + 4,
                    if scenario == 3 { 2 } else { 1 },
                );
                for (index, target) in [
                    0x003f_7030,
                    0x003f_70a0,
                    0x003f_7134,
                    0x003f_7150,
                    0x003f_6fec,
                    0x003f_7148,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(STEP_TABLE + 4 * index as u32, target);
                }
            },
        );
    }

    #[test]
    fn slogin_free_state_matches() {
        check(
            ee::s14::f_003f6d98__generated,
            slogin_free_state,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6d_98, 0);
                c.pc = 0x003f_6d98;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x140, 0);
            },
        );
    }

    #[test]
    fn slogin_init_store_matches() {
        check(
            ee::s14::f_003f6dc8__generated,
            slogin_init_store,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6d_c8, 0);
                c.pc = 0x003f_6dc8;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x140, DATA + 0x500);
                c.bus.write32(GAME_GP + 0x2a30, DATA + 0x800); // Store already initialized.
                c.bus.write32(LOGIN_STORE_TEMPLATE, seed);
            },
        );
    }

    #[test]
    fn slogin_shutdown_store_matches() {
        check(
            ee::s14::f_003f6e70__generated,
            slogin_shutdown_store,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6e_70, 0);
                c.pc = 0x003f_6e70;
                c.bus.write32(GAME_GP + 0x2a30, 0);
            },
        );
    }

    fn end_sequence_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
        c.g[A0][0] = sx(DATA + 0x500);
        c.g[A1][0] = sx(DATA + 0x504);
        c.bus
            .write32(DATA + 0x200 + LOGIN_FINAL_ERROR_OFFSET, seed ^ 0x1234);
    }

    #[test]
    fn slogin_on_final_error_matches() {
        check(
            ee::s14::f_003f71f8__generated,
            slogin_on_final_error,
            24,
            &|c, seed| {
                end_sequence_fixture(c, seed, 0x003f_71f8);
            },
        );
    }

    #[test]
    fn slogin_on_step_error_matches() {
        check(
            ee::s14::f_003f71b8__generated,
            slogin_on_step_error,
            24,
            &|c, seed| {
                end_sequence_fixture(c, seed, 0x003f_71b8);
                c.bus.write32(DATA + 0x504, seed ^ 0x7777);
            },
        );
    }

    #[test]
    fn slogin_on_final_done_matches() {
        check(
            ee::s14::f_003f72c8__generated,
            slogin_on_final_done,
            24,
            &|c, seed| {
                end_sequence_fixture(c, seed, 0x003f_72c8);
            },
        );
    }

    #[test]
    fn slogin_on_step_done_matches() {
        check(
            ee::s14::f_003f7218__generated,
            slogin_on_step_done,
            32,
            &|c, seed| {
                end_sequence_fixture(c, seed, 0x003f_7218);
                let (step, target, operation) = match seed % 8 {
                    0 => (0, 0x003f_7258, 0),
                    1 => (1, 0x003f_7260, 0),
                    2 => (2, 0x003f_7260, 2),
                    3 => (3, 0x003f_7260, 3),
                    4 => (4, 0x003f_7288, 0),
                    5 => (5, 0x003f_7290, 0),
                    6 => (6, 0x003f_7258, 0),
                    _ => (0, 0x003f_7298, 0),
                };
                c.bus.write32(DATA + 0x200 + STORE_STEP_OFFSET, step);
                c.bus
                    .write32(DATA + 0x200 + STORE_OPERATION_OFFSET, operation);
                for index in 0..6 {
                    c.bus.write32(0x0049_70a0 + 4 * index, 0x003f_7258);
                }
                if step < 6 {
                    c.bus.write32(0x0049_70a0 + 4 * step, target);
                }
            },
        );
    }

    #[test]
    fn slogin_on_error_close_done_matches() {
        check(
            ee::s14::f_003f72f8__generated,
            slogin_on_error_close_done,
            24,
            &|c, seed| {
                end_sequence_fixture(c, seed, 0x003f_72f8);
            },
        );
    }

    #[test]
    fn slogin_begin_sequence_matches() {
        check(
            ee::s14::f_003f7328__generated,
            slogin_begin_sequence,
            25,
            &|c, seed| {
                end_sequence_fixture(c, seed, 0x003f_7328);
                let state = if seed % 5 == 0 { 8 } else { 9 };
                let operation = match seed % 5 {
                    1 => 0,
                    2 => 1,
                    3 => 3,
                    _ => 4,
                };
                c.bus.write32(DATA + 0x200 + STORE_STEP_OFFSET, state);
                c.bus
                    .write32(DATA + 0x200 + STORE_OPERATION_OFFSET, operation);
            },
        );
    }

    fn file_step_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
        c.bus.write32(GAME_GP + 0x2a30, DATA + 0x800);
        c.bus.write32(GAME_GP + 0x1aec, 0); // Memory-card manager not initialized.
    }

    #[test]
    fn slogin_step_create_matches() {
        check(
            ee::s14::f_003f73a8__generated,
            slogin_step_create,
            16,
            &|c, seed| {
                file_step_fixture(c, seed, 0x003f_73a8);
            },
        );
    }

    #[test]
    fn slogin_step_open_matches() {
        check(
            ee::s14::f_003f73f0__generated,
            slogin_step_open,
            16,
            &|c, seed| {
                file_step_fixture(c, seed, 0x003f_73f0);
            },
        );
    }

    #[test]
    fn slogin_step_delete_matches() {
        check(
            ee::s14::f_003f7438__generated,
            slogin_step_delete,
            16,
            &|c, seed| {
                file_step_fixture(c, seed, 0x003f_7438);
            },
        );
    }

    fn write_step_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 4);
        c.pc = entry;
        c.bus.write32(GAME_GP + LOGIN_STATE_GP_OFFSET, DATA + 0x200);
        c.bus.write32(DATA + 0x200 + 0x140, DATA + 0x500);
        c.bus.write32(GAME_GP + 0x2a30, DATA + 0x800);
        c.bus.write32(GAME_GP + 0x1aec, 0);
        for index in 0..5 {
            c.bus.write32(0x0049_7580 + 4 * index, 0x003f_b884);
        }
        for index in 0..0x73 {
            c.bus.write32(0x0049_7150 + 4 * index, 0x003f_81ec);
        }
        c.bus.write32(0x0049_7150, 0x003f_819c); // Successful store call maps to zero.
        c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
        c.bus.write32(ALLOC_CTRL + 4, seed % 2);
        c.bus.write32(LOG_RESULTS, DATA + 0x2000);
    }

    #[test]
    fn slogin_step_write_acct_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f7478__generated,
            slogin_step_write_acct,
            24,
            &|c, seed| {
                write_step_fixture(c, seed, 0x003f_7478);
            },
        );
    }

    #[test]
    fn slogin_step_write_pass_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f7528__generated,
            slogin_step_write_pass,
            24,
            &|c, seed| {
                write_step_fixture(c, seed, 0x003f_7528);
            },
        );
    }
}
