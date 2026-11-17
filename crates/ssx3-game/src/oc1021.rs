//! Staging module oc1021 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1021`.

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
    (0x003f_75d8, slogin_step_read_acct, "SLogin_StepReadAcct"),
    (0x003f_7658, slogin_step_read_pass, "SLogin_StepReadPass"),
    (0x003f_76d8, slogin_step_close, "SLogin_StepClose"),
    (0x003f_7708, slogin_step_poll, "SLogin_StepPoll"),
    (
        0x003f_77f8,
        slogin_fetch_store_info,
        "SLogin_FetchStoreInfo",
    ),
    (0x003f_7820, slogin_on_open_done, "SLogin_OnOpenDone"),
    (
        0x003f_7840,
        slogin_on_read_acct_done,
        "SLogin_OnReadAcctDone",
    ),
    (
        0x003f_7890,
        slogin_on_read_pass_done,
        "SLogin_OnReadPassDone",
    ),
    (
        0x003f_78e0,
        slogin_on_write_acct_done,
        "SLogin_OnWriteAcctDone",
    ),
    (
        0x003f_7900,
        slogin_on_write_pass_done,
        "SLogin_OnWritePassDone",
    ),
    (
        0x003f_7920,
        slogin_free_acct_record,
        "SLogin_FreeAcctRecord",
    ),
    (
        0x003f_7958,
        slogin_free_pass_record,
        "SLogin_FreePassRecord",
    ),
    (0x003f_7990, slogin_reset_for_op, "SLogin_ResetForOp"),
    (0x003f_7a30, slogin_finish_op, "SLogin_FinishOp"),
    (0x003f_7a60, slogin_set_step, "SLogin_SetStep"),
    (0x003f_7bd0, slogin_end_sequence, "SLogin_EndSequence"),
    (0x003f_7c00, slogin_init_step_funcs, "SLogin_InitStepFuncs"),
    (
        0x003f_7d98,
        slogin_clear_step_funcs,
        "SLogin_ClearStepFuncs",
    ),
    (
        0x003f_7de0,
        slogin_verify_icon_checksum,
        "SLogin_VerifyIconChecksum",
    ),
    (0x003f_7e90, slogin_str_copy_n, "SLogin_StrCopyN"),
];

const SHARED_LOGIN_GP: u32 = 0x1ad4;

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MAP_STORE_ERROR: Callee = Callee {
        addr: 0x003f_8178,
        f: ee::s14::SLogin_MapStoreError,
    };
    pub(crate) const SLOGIN_FETCH_STORE_INFO: Callee = Callee {
        addr: 0x003f_77f8,
        f: ee::s14::SLogin_FetchStoreInfo,
    };
    pub(crate) const SLOGIN_FREE_ACCT_RECORD: Callee = Callee {
        addr: 0x003f_7920,
        f: ee::s14::SLogin_FreeAcctRecord,
    };
    pub(crate) const SLOGIN_FREE_PASS_RECORD: Callee = Callee {
        addr: 0x003f_7958,
        f: ee::s14::SLogin_FreePassRecord,
    };
    pub(crate) const SLOGIN_END_SEQUENCE: Callee = Callee {
        addr: 0x003f_7bd0,
        f: ee::s14::SLogin_EndSequence,
    };
    pub(crate) const SLOGIN_CLEAR_STEP_FUNCS: Callee = Callee {
        addr: 0x003f_7d98,
        f: ee::s14::SLogin_ClearStepFuncs,
    };
    pub(crate) const SLOGIN_INIT_STEP_FUNCS: Callee = Callee {
        addr: 0x003f_7c00,
        f: ee::s14::SLogin_InitStepFuncs,
    };
}

fn slogin_step_read_record(c: &mut Cpu, release: u32, on_done: u32, tag: u32, returns: [u32; 4]) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(release);
        c.g[V1][0] = sx(on_done);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[A2][0]);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x140));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x144));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), c.g[V1][0]);
        c.g[A2][0] = 0x11;
        c.g[A3][0] = 0;
        call_ptr(c, 0x003f_ce10, returns[0])?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, returns[1])?;
        let mapped_error = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !mapped_error {
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
            c.g[A1][0] = ((tag >> 16) as u64) << 16;
            c.g[A1][0] |= (tag & 0xffff) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x144));
            c.g[T0][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
            call_ptr(c, 0x003f_c708, returns[2])?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_MAP_STORE_ERROR.call(c, returns[3])?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Start reading the shared login account record from the card store.
pub fn slogin_step_read_acct(c: &mut Cpu) {
    slogin_step_read_record(
        c,
        0x003f_7920,
        0x003f_7840,
        0x4143_4354,
        [0x003f_7610, 0x003f_7618, 0x003f_7640, 0x003f_7648],
    );
}

/// Start reading the shared login password record from the card store.
pub fn slogin_step_read_pass(c: &mut Cpu) {
    slogin_step_read_record(
        c,
        0x003f_7958,
        0x003f_7890,
        0x5041_5353,
        [0x003f_7690, 0x003f_7698, 0x003f_76c0, 0x003f_76c8],
    );
}

/// Close the shared login's current card-store file and map its result.
pub fn slogin_step_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), 0);
        call_ptr(c, 0x003f_c4e8, 0x003f_76f4)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_76fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn slogin_poll_finish(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    c.g[V0][0] = c.g[A0][0];
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// Poll the current card-store step and run its completion or error callback.
pub fn slogin_step_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 7;
        frame.save_ra(c, 0x10);
        if c.g[S1][0] != 0 {
            c.g[A0][0] = 0;
        }
        c.sw(frame.at(c, 0), 0);

        if c.g[A0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[SP][0] | 4;
            call_ptr(c, 0x003f_ca40, 0x003f_7738)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_7740)?;
            c.g[A0][0] = c.g[V0][0];
            let mapped_error = c.g[A0][0] != 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
            if !mapped_error {
                c.cycles += 4;
                c.g[S0][0] = c.lw(frame.at(c, 0));
                c.g[V0][0] = 1;
                c.sw(c.g[S1][0] as u32, 0);
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                let no_result = c.g[S0][0] == 0;
                c.g[V0][0] = 2;
                if no_result {
                    c.cycles += 2;
                    c.g[A0][0] = 0x17;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                let finished = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
                if !finished {
                    c.cycles += 2;
                    c.g[A0][0] = 0x12;
                    c.cycles += 2;
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }

                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfc));
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    let callback = c.g[V0][0] as u32;
                    call_ptr(c, callback, 0x003f_7790)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xfc), 0);
                }
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                slogin_poll_finish(c, frame);
                return Ok(());
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x100));
        frame.restore(c, S0, 0x30);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            let callback = c.g[V0][0] as u32;
            call_ptr(c, callback, 0x003f_77cc)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        slogin_poll_finish(c, frame);
        Ok(())
    });
}

/// Fetch the current account-record information from the shared login store.
pub fn slogin_fetch_store_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
        call_ptr(c, 0x003f_c990, 0x003f_780c)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_7814)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Continue the login sequence after the card-store file opens.
pub fn slogin_on_open_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_FETCH_STORE_INFO.call(c, 0x003f_7830)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy the account record into the login state, then release its card-store block.
pub fn slogin_on_read_acct_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x144));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[A1][0] as u32);
        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(15));
        c.ldr(6, (c.g[A1][0] as u32).wrapping_add(8));
        c.g[A3][0] = c.lb((c.g[A1][0] as u32).wrapping_add(16));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(15));
        c.sdr(6, (c.g[V1][0] as u32).wrapping_add(8));
        c.sb((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
        callee::SLOGIN_FREE_ACCT_RECORD.call(c, 0x003f_7884)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy the password record into the login state, then release its card-store block.
pub fn slogin_on_read_pass_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x144));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[A1][0] as u32);
        c.ldl(6, (c.g[A1][0] as u32).wrapping_add(15));
        c.ldr(6, (c.g[A1][0] as u32).wrapping_add(8));
        c.g[A3][0] = c.lb((c.g[A1][0] as u32).wrapping_add(16));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(24));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(17));
        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(32));
        c.sdr(6, (c.g[V1][0] as u32).wrapping_add(25));
        c.sb((c.g[V1][0] as u32).wrapping_add(33), c.g[A3][0]);
        c.g[A0][0] = 0;
        callee::SLOGIN_FREE_PASS_RECORD.call(c, 0x003f_78d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Release the account record after a completed card-store write.
pub fn slogin_on_write_acct_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        callee::SLOGIN_FREE_ACCT_RECORD.call(c, 0x003f_78f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Release the password record after a completed card-store write.
pub fn slogin_on_write_pass_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        callee::SLOGIN_FREE_PASS_RECORD.call(c, 0x003f_7910)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn slogin_free_record(c: &mut Cpu, first_return: u32, second_return: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x11;
        c.g[A3][0] = 0;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x140));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x144));
        call_ptr(c, 0x003f_cf00, first_return)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, second_return)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Free the account card-store block and map the store's return code.
pub fn slogin_free_acct_record(c: &mut Cpu) {
    slogin_free_record(c, 0x003f_7940, 0x003f_7948);
}

/// Free the password card-store block and map the store's return code.
pub fn slogin_free_pass_record(c: &mut Cpu) {
    slogin_free_record(c, 0x003f_7978, 0x003f_7980);
}

/// Finish the active login operation and end its step sequence.
pub fn slogin_finish_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[V1][0] = 4;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xd4), 0);
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_7a50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Select a shared login step, preserving the previous step when requested.
pub fn slogin_set_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0xff;
        c.g[A2][0] = 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 22;
            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0xdf));
            c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0xd8));
            c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xe7));
            c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0xe0));
            c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xef));
            c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0xe8));
            c.ldl(7, (c.g[V0][0] as u32).wrapping_add(0xf7));
            c.ldr(7, (c.g[V0][0] as u32).wrapping_add(0xf0));
            c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x113));
            c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x10c));
            c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x11b));
            c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x114));
            c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x123));
            c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x11c));
            c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x12b));
            c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x124));
            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0xff));
            c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0xf8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x100));
            c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x133));
            c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x12c));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[A0][0]);
        }

        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd8));
        c.g[V0][0] = (c.g[V0][0] < 9) as u64;
        let valid_step = c.g[V0][0] != 0;
        c.g[V0][0] = 0x003f_0000;
        if valid_step {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f68));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xdc), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[A2][0] = 13;
        }

        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        if c.g[A2][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd8));
            c.g[V0][0] = (c.g[A0][0] < 9) as u64;
            c.g[A1][0] = c.g[V1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[V0][0] = 0x0049_0000;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70c0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                let callback = match target {
                    0x003f_7b2c => 0x003f_7218,
                    0x003f_7b3c => 0x003f_72c8,
                    0x003f_7b4c => 0x003f_72f8,
                    0x003f_7b5c => {
                        c.cycles += 1;
                        c.g[A2][0] = 13;
                        0
                    }
                    _ => {
                        c.jump_out(target, 0x003f_7a60, 0x003f_7bcc);
                        return Ok(());
                    }
                };
                if callback != 0 {
                    c.cycles += 4;
                    c.g[V0][0] = 0x003f_0000;
                    c.g[V0][0] = sx(callback);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                }
            } else {
                c.cycles += 1;
                c.g[A2][0] = 13;
            }

            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
            if c.g[A2][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd8));
                c.g[V0][0] = (c.g[A0][0] < 9) as u64;
                c.g[A1][0] = c.g[V1][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    c.g[V0][0] = 0x0049_0000;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70f0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let target = c.g[A0][0] as u32;
                    match target {
                        0x003f_7b94 => {
                            c.cycles += 3;
                            c.g[V0][0] = sx(0x003f_71b8);
                        }
                        0x003f_7ba0 => {
                            c.cycles += 3;
                            c.g[V0][0] = sx(0x003f_71f8);
                        }
                        0x003f_7bac => {
                            c.cycles += 2;
                            c.g[V0][0] = sx(0x003f_72f8);
                        }
                        _ => {
                            c.jump_out(target, 0x003f_7a60, 0x003f_7bcc);
                            return Ok(());
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xe4), c.g[V0][0]);
                }
            }
        }

        c.cycles += 5;
        callee::SLOGIN_INIT_STEP_FUNCS.call(c, 0x003f_7bc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Mark the login step sequence complete and clear its callbacks.
pub fn slogin_end_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = 10;
        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), 0);
        callee::SLOGIN_CLEAR_STEP_FUNCS.call(c, 0x003f_7bf4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Install the callbacks for the selected login card-store step.
pub fn slogin_init_step_funcs(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
    c.g[V1][0] = 4;
    c.g[A1][0] = 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[V1][0]);
    c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf3));
    c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0xec));
    c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xfb));
    c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0xf4));
    c.ldl(7, (c.g[V0][0] as u32).wrapping_add(0x103));
    c.ldr(7, (c.g[V0][0] as u32).wrapping_add(0xfc));
    c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x127));
    c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x120));
    c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x12f));
    c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x128));
    c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x137));
    c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x130));
    c.g[V1][0] = c.g[A1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    loop {
        c.cycles += 7;
        c.g[V0][0] = (c.g[V1][0] < 3) as u64;
        let again = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003f_7c48);
            }
            continue;
        }
        break;
    }

    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
    c.g[V1][0] = (c.g[A0][0] < 9) as u64;
    let valid_step = c.g[V1][0] != 0;
    c.g[V0][0] = 0x0049_0000;
    if valid_step {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7120));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        let step_callback = match target {
            0x003f_7c90 => 0x003f_73a8,
            0x003f_7ca4 => 0x003f_73f0,
            0x003f_7cb8 => 0x003f_7478,
            0x003f_7ccc => 0x003f_7528,
            0x003f_7ce0 => 0x003f_75d8,
            0x003f_7cf4 => 0x003f_7658,
            0x003f_7d08 => 0x003f_76d8,
            0x003f_7d1c => 0x003f_7438,
            _ => {
                c.jump_out(target, 0x003f_7c00, 0x003f_7d98);
                return;
            }
        };
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        c.g[V0][0] = 0x003f_0000;
        c.g[V0][0] = sx(step_callback);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.g[A1][0] = 13;
    }

    c.cycles += 2;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
    if c.g[A1][0] == 0 {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd8));
        c.g[V0][0] = (c.g[V0][0] < 9) as u64;
        let valid_poll = c.g[V0][0] != 0;
        c.g[V0][0] = 0x003f_0000;
        if valid_poll {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7708));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xf4), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[A1][0] = 13;
        }
        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        if c.g[A1][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd8));
            c.g[V0][0] = (c.g[V0][0] < 9) as u64;
            let valid_nop = c.g[V0][0] != 0;
            c.g[V0][0] = 0x003f_0000;
            if valid_nop {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x77f0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
            } else {
                c.cycles += 1;
                c.g[A1][0] = 13;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        }
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[A1][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
    c.sw((c.g[V1][0] as u32).wrapping_add(0xfc), 0);
    ret(c);
}

/// Reset the shared login state for a requested operation and clear its pending callbacks.
pub fn slogin_reset_for_op(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
    c.g[V0][0] = (c.g[A0][0] < 4) as u64;
    c.g[A2][0] = 0;
    c.sw((c.g[V1][0] as u32).wrapping_add(0xd0), c.g[A0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x104), c.g[A0][0]);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = sx(0x003f_6ee0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.g[A2][0] = 15;
    }

    c.cycles += 13;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
    c.g[V1][0] = sx(0x003f_7328);
    c.g[A0][0] = 9;
    c.g[A1][0] = 4;
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[A1][0]);
    c.g[A0][0] = 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[V1][0]);
    for off in [0xdc, 0xe0, 0xe4] {
        c.sw((c.g[V0][0] as u32).wrapping_add(off), 0);
    }
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    loop {
        c.cycles += 7;
        c.g[V0][0] = (c.g[A0][0] < 3) as u64;
        let again = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003f_79f0);
            }
            continue;
        }
        c.cycles += 8;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        c.g[V0][0] = c.g[A2][0];
        for off in [0x100, 0xf0, 0xf4, 0xf8, 0xfc] {
            c.sw((c.g[V1][0] as u32).wrapping_add(off), 0);
        }
        ret(c);
        return;
    }
}

/// Clear the shared login's step callbacks and pending completion handlers.
pub fn slogin_clear_step_funcs(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
    c.g[V0][0] = 5;
    c.g[A0][0] = 0;
    c.sw((c.g[V1][0] as u32).wrapping_add(0xec), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf0));
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[V0][0] = (c.g[A0][0] < 3) as u64;
        let again = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003f_7db0);
            }
            continue;
        }
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(SHARED_LOGIN_GP));
        c.g[V0][0] = 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x100), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xfc), 0);
        ret(c);
        return;
    }
}

/// Verify that the login icon data carries the card-store checksum expected by the game.
pub fn slogin_verify_icon_checksum(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S2][0] = 0;
        call_ptr(c, 0x003f_b5e8, 0x003f_7e00)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        let first = c.g[V0][0] as u32;
        call_ptr(c, first, 0x003f_7e10)?;
        c.g[A2][0] = c.g[V0][0];
        let first_failed = c.g[A2][0] != 0;
        c.g[V0][0] = 0x18;
        if !first_failed {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x296e;
            let second = c.g[V0][0] as u32;
            call_ptr(c, second, 0x003f_7e2c)?;
            c.g[A2][0] = c.g[V0][0];
            let second_failed = c.g[A2][0] != 0;
            c.g[V0][0] = 0x18;
            if !second_failed {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                let third = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, third, 0x003f_7e44)?;
                c.g[A2][0] = c.g[V0][0];
                let third_failed = c.g[A2][0] != 0;
                c.g[V0][0] = 0x18;
                if !third_failed {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = 0x0836_c71d;
                    c.g[S2][0] = 0x15;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(4));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] ^= c.g[A0][0];
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = 0;
                    }
                    c.g[V0][0] = 0x18;
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        if c.g[A2][0] == 0 {
            c.g[V0][0] = c.g[S2][0];
        }
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Copy a bounded, zero-terminated login string, returning the destination pointer.
pub fn slogin_str_copy_n(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.cycles += 2;
                c.g[V1][0] = c.g[A0][0];
                if c.g[A1][0] != c.g[A0][0] {
                    loop {
                        c.cycles += 3;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                        if c.g[A2][0] == 0 {
                            c.cycles += 2;
                            c.sb(c.g[V1][0] as u32, 0);
                            break;
                        }
                        c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                        c.cycles += 5;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        let again = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003f_7eb0);
                        }
                    }
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const LOGIN_TEST_MAGIC_AT: u32 = DATA + 0x3f0;
    const LOGIN_TEST_MAGIC: u32 = 0x53_4c_4f_47;
    static PREV_LOGIN_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static PREV_LOGIN_FREE: AtomicUsize = AtomicUsize::new(0);

    fn login_alloc_fake(c: &mut Cpu) {
        if c.bus.read32(LOGIN_TEST_MAGIC_AT) != LOGIN_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_LOGIN_ALLOC, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, 0x0026_6728);
        if c.bus.read32(DATA + 0x3e8) == 0 {
            c.g[V0][0] = 0;
        }
    }

    fn login_free_fake(c: &mut Cpu) {
        if c.bus.read32(LOGIN_TEST_MAGIC_AT) != LOGIN_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_LOGIN_FREE, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, 0x0026_6760);
    }

    fn login_mc_poll_fake(c: &mut Cpu, target: u32) {
        let out_status = c.g[A0][0] as u32;
        let out_detail = c.g[A1][0] as u32;
        fakes::fake_call(c, target);
        if target == fakes::FAKE + 0x50 {
            let mode = c.bus.read32(DATA + 0x3ec);
            c.bus.write32(out_status, mode);
            c.bus.write32(out_detail, 0);
            c.g[V0][0] = 0;
        } else if target == fakes::FAKE + 0x60 || target == fakes::FAKE + 0x70 {
            c.g[V0][0] = 0;
        }
    }

    fn login_store_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            login_alloc_fake,
            &PREV_LOGIN_ALLOC,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            login_free_fake,
            &PREV_LOGIN_FREE,
        );
        let _r = fakes::machine(c, seed, entry as u64, 5);
        c.pc = entry;
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed % 6 < 3 { seed % 6 } else { u32::MAX },
        );
        c.bus.write32(LOGIN_TEST_MAGIC_AT, LOGIN_TEST_MAGIC);
        c.bus.write32(DATA + 0x3e8, seed % 4);
        c.bus.write32(GAME_GP + SHARED_LOGIN_GP, DATA);
        c.bus.write32(DATA + 0x140, 0x11);
        c.bus.write32(DATA + 0x144, DATA + 0x800);
        c.bus.write32(DATA + 0xcc, DATA + 0xa00);
        for i in 0..17u32 {
            c.bus.write8(DATA + 0x800 + i, (0x41 + i) as u8);
        }
        c.bus.write32(DATA + 0x44, 0);
        c.bus.write32(DATA + 0x48, 0);
        c.bus.write32(DATA + 0x4c, 0);
    }

    #[test]
    fn slogin_step_read_acct_matches() {
        check(
            ee::s14::f_003f75d8__generated,
            slogin_step_read_acct,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_75d8);
            },
        );
    }

    #[test]
    fn slogin_step_read_pass_matches() {
        check(
            ee::s14::f_003f7658__generated,
            slogin_step_read_pass,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7658);
            },
        );
    }

    #[test]
    fn slogin_step_close_matches() {
        check(
            ee::s14::f_003f76d8__generated,
            slogin_step_close,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_76d8);
            },
        );
    }

    #[test]
    fn slogin_step_poll_matches() {
        check(
            ee::s14::f_003f7708__generated,
            slogin_step_poll,
            120,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7708);
                let case = seed % 12;
                c.g[A0][0] = if case < 3 { 0 } else { (DATA + 0x2200) as u64 };
                c.bus.write32(DATA + 0x2200, 0);
                c.bus.write32(
                    DATA + 0x100,
                    [0, fakes::FAKE + 0x40, 0x0014_06c0][(case % 3) as usize],
                );
                if case >= 6 {
                    c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                    c.hooks.miss = login_mc_poll_fake;
                    c.bus.write32(GAME_GP + 0x2a30, DATA + 0x4000);
                    c.bus.write32(GAME_GP + 0x1aec, DATA + 0x3000);
                    c.bus.write32(GAME_GP + 0x1ae8, DATA + 0x3400);
                    c.bus.write32(DATA + 0x3000 + 0x40, fakes::FAKE + 0x50);
                    c.bus.write32(DATA + 0x3000 + 0x4c, DATA + 0x3800);
                    c.bus.write32(DATA + 0x3000 + 0x48, DATA + 0x3900);
                    c.bus.write32(DATA + 0x3800, fakes::FAKE + 0x60);
                    c.bus.write32(DATA + 0x3900, fakes::FAKE + 0x70);
                    c.bus.write32(DATA + 0x3400, 0);
                    c.bus.write32(DATA + 0x3404, 0);
                    c.bus.write32(DATA + 0x4000 + 0x74, 0);
                    c.bus.write32(
                        DATA + 0x3ec,
                        if case == 6 {
                            0
                        } else if case == 7 {
                            1
                        } else {
                            2
                        },
                    );
                    c.bus.write32(
                        DATA + 0xfc,
                        if case == 9 {
                            fakes::FAKE + 0x80
                        } else if case == 10 {
                            0x0014_06c0
                        } else {
                            0
                        },
                    );
                }
            },
        );
    }

    #[test]
    fn slogin_fetch_store_info_matches() {
        check(
            ee::s14::f_003f77f8__generated,
            slogin_fetch_store_info,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_77f8);
            },
        );
    }

    #[test]
    fn slogin_on_open_done_matches() {
        check(
            ee::s14::f_003f7820__generated,
            slogin_on_open_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7820);
            },
        );
    }

    #[test]
    fn slogin_on_read_acct_done_matches() {
        check(
            ee::s14::f_003f7840__generated,
            slogin_on_read_acct_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7840);
            },
        );
    }

    #[test]
    fn slogin_on_read_pass_done_matches() {
        check(
            ee::s14::f_003f7890__generated,
            slogin_on_read_pass_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7890);
            },
        );
    }

    #[test]
    fn slogin_on_write_acct_done_matches() {
        check(
            ee::s14::f_003f78e0__generated,
            slogin_on_write_acct_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_78e0);
            },
        );
    }

    #[test]
    fn slogin_on_write_pass_done_matches() {
        check(
            ee::s14::f_003f7900__generated,
            slogin_on_write_pass_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7900);
            },
        );
    }

    #[test]
    fn slogin_free_acct_record_matches() {
        check(
            ee::s14::f_003f7920__generated,
            slogin_free_acct_record,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7920);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x144, 0);
                }
            },
        );
    }

    #[test]
    fn slogin_free_pass_record_matches() {
        check(
            ee::s14::f_003f7958__generated,
            slogin_free_pass_record,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7958);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x144, 0);
                }
            },
        );
    }

    #[test]
    fn slogin_finish_op_matches() {
        check(
            ee::s14::f_003f7a30__generated,
            slogin_finish_op,
            60,
            &|c, seed| {
                login_state(c, seed, 0x003f_7a30);
            },
        );
    }

    fn slogin_step_tables(c: &mut Cpu) {
        const DONE: [u32; 9] = [
            0x003f_7b2c,
            0x003f_7b2c,
            0x003f_7b2c,
            0x003f_7b2c,
            0x003f_7b2c,
            0x003f_7b2c,
            0x003f_7b3c,
            0x003f_7b4c,
            0x003f_7b3c,
        ];
        const ERROR: [u32; 9] = [
            0x003f_7b94,
            0x003f_7b94,
            0x003f_7b94,
            0x003f_7b94,
            0x003f_7b94,
            0x003f_7b94,
            0x003f_7ba0,
            0x003f_7bac,
            0x003f_7ba0,
        ];
        const STEP: [u32; 9] = [
            0x003f_7c90,
            0x003f_7ca4,
            0x003f_7cb8,
            0x003f_7ccc,
            0x003f_7ce0,
            0x003f_7cf4,
            0x003f_7d08,
            0x003f_7d08,
            0x003f_7d1c,
        ];
        for i in 0..9u32 {
            c.bus.write32(0x0049_70c0 + 4 * i, DONE[i as usize]);
            c.bus.write32(0x0049_70f0 + 4 * i, ERROR[i as usize]);
            c.bus.write32(0x0049_7120 + 4 * i, STEP[i as usize]);
        }
    }

    #[test]
    fn slogin_set_step_matches() {
        check(
            ee::s14::f_003f7a60__generated,
            slogin_set_step,
            132,
            &|c, seed| {
                login_state(c, seed, 0x003f_7a60);
                slogin_step_tables(c);
                c.g[A0][0] = (seed % 11) as u64;
                c.g[A1][0] = (seed / 11 % 2) as u64;
                for i in 0..10u32 {
                    c.bus.write32(DATA + 0xd8 + 4 * i, 0x7654_0000 + i + seed);
                }
            },
        );
    }

    #[test]
    fn slogin_init_step_funcs_matches() {
        check(
            ee::s14::f_003f7c00__generated,
            slogin_init_step_funcs,
            110,
            &|c, seed| {
                login_state(c, seed, 0x003f_7c00);
                slogin_step_tables(c);
                c.bus.write32(DATA + 0xd8, seed % 11);
                c.bus.write32(DATA + 0xf0, 0x1000 + seed);
                c.bus.write32(DATA + 0xf4, 0x2000 + seed);
                c.bus.write32(DATA + 0xf8, 0x3000 + seed);
                c.bus.write32(DATA + 0xfc, 0x4000 + seed);
                c.bus.write32(DATA + 0x100, 0x5000 + seed);
            },
        );
    }

    #[test]
    fn slogin_init_step_funcs_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f7c00__generated,
            slogin_init_step_funcs,
            90,
            &|c, seed| {
                login_state(c, seed, 0x003f_7c00);
                slogin_step_tables(c);
                c.bus.write32(DATA + 0xd8, seed % 11);
                let mut r = Rng(0x003f_7c00 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_7c00)
            },
        );
    }

    fn checksum_method_fake(c: &mut Cpu, target: u32) {
        let mode = c.bus.read32(DATA + 0x3e4);
        let output_slot = c.g[A0][0] as u32;
        fakes::fake_call(c, target);
        let status = match target {
            x if x == fakes::FAKE + 0x10 => (mode == 0) as u64,
            x if x == fakes::FAKE + 0x20 => (mode == 1) as u64,
            x if x == fakes::FAKE + 0x30 => (mode == 2) as u64,
            _ => return,
        };
        if target == fakes::FAKE + 0x30 && status == 0 {
            c.bus.write32(output_slot, DATA + 0x1600);
        }
        c.g[V0][0] = status;
    }

    #[test]
    fn slogin_verify_icon_checksum_matches() {
        check(
            ee::s14::f_003f7de0__generated,
            slogin_verify_icon_checksum,
            100,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_7de0, 3);
                c.pc = 0x003f_7de0;
                c.hooks.miss = checksum_method_fake;
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                c.bus.write32(DATA + 0x3e4, seed % 5);
                c.g[A0][0] = (DATA + 0x1200) as u64;
                c.bus.write32(0x0049_7508 + 0xc, fakes::FAKE + 0x10);
                c.bus.write32(0x0049_7508 + 0x10, fakes::FAKE + 0x20);
                c.bus.write32(0x0049_7508 + 0x14, fakes::FAKE + 0x30);
                c.bus.write32(DATA + 0x1604, DATA + 0x1800);
                c.bus.write32(
                    DATA + 0x1800,
                    if seed % 5 == 3 {
                        0x0836_c71d
                    } else {
                        0x1234_5678
                    },
                );
            },
        );
    }

    #[test]
    fn slogin_end_sequence_matches() {
        check(
            ee::s14::f_003f7bd0__generated,
            slogin_end_sequence,
            60,
            &|c, seed| {
                login_state(c, seed, 0x003f_7bd0);
            },
        );
    }

    fn login_state(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x3f75_0000 ^ seed as u64 ^ entry as u64);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + SHARED_LOGIN_GP, DATA);
    }

    #[test]
    fn slogin_reset_for_op_matches() {
        check(
            ee::s14::f_003f7990__generated,
            slogin_reset_for_op,
            60,
            &|c, seed| {
                login_state(c, seed, 0x003f_7990);
                c.g[A0][0] = (seed % 6) as u64;
            },
        );
    }

    #[test]
    fn slogin_clear_step_funcs_matches() {
        check(
            ee::s14::f_003f7d98__generated,
            slogin_clear_step_funcs,
            40,
            &|c, seed| {
                login_state(c, seed, 0x003f_7d98);
            },
        );
    }

    #[test]
    fn slogin_clear_step_funcs_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f7d98__generated,
            slogin_clear_step_funcs,
            90,
            &|c, seed| {
                login_state(c, seed, 0x003f_7d98);
                let mut r = Rng(0x003f_7d98 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_7d98)
            },
        );
    }

    #[test]
    fn slogin_str_copy_n_matches() {
        check(
            ee::s14::f_003f7e90__generated,
            slogin_str_copy_n,
            120,
            &|c, seed| {
                let mut r = Rng(0x003f_7e90 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_7e90;
                let dst = DATA + 0x400;
                let src = DATA + 0x600;
                for (i, byte) in b"account\0".iter().enumerate() {
                    c.bus.write8(src + i as u32, *byte);
                }
                match seed % 7 {
                    0 => {
                        c.g[A1][0] = 0;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 8;
                    }
                    1 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = 0;
                        c.g[A2][0] = 8;
                    }
                    2 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 0;
                    }
                    3 => {
                        c.g[A1][0] = dst as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 8;
                    }
                    4 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 1;
                    }
                    5 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 4;
                    }
                    _ => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 16;
                    }
                }
            },
        );
    }
}
