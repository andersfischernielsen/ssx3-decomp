//! Staging module oc1029 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1029`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003f_ae50, slogin_mc_delete_step, "SLoginMc_DeleteStep")];

pub(crate) const DELETE_ENTRY: u32 = 0x003f_ae50;
/// The back-edge head an unwind resumes at (restart on the next card).
pub(crate) const DELETE_RETRY_HEAD: u32 = 0x003f_af60;
const DELETE_END: u32 = 0x003f_b274;
pub(crate) const DELETE_STATE_GP: u32 = 0x1ae8;
pub(crate) const DELETE_IFACE_GP: u32 = 0x1aec;
pub(crate) const DELETE_PHASE: u32 = 4;
pub(crate) const DELETE_DEVICE: u32 = 0x10;
/// The save's path in the card state (passed to the remove callbacks).
pub(crate) const DELETE_PATH: u32 = 0x14;
/// The directory name `SLoginMc_BuildDirPath` appends.
pub(crate) const DELETE_DIR_NAME: u32 = 0x54;
/// Word at +0x6c, passed as `SLoginMc_NextDeviceInMask`'s direction.
pub(crate) const DELETE_SCAN_DIRECTION: u32 = 0x6c;
/// The phase dispatch table (phases 1..=26).
pub(crate) const DELETE_DISPATCH: u32 = 0x0049_74a0;
/// `"icon.sys"` and `"view.ico"`.
const DELETE_ICON_SYS: u32 = 0x0049_7420;
const DELETE_VIEW_ICON: u32 = 0x0049_7410;

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_SET_ERROR: Callee = Callee {
        addr: 0x003f_86f0,
        f: ee::s14::SLoginMc_SetError,
    };
    pub(crate) const SLOGIN_MC_GET_ERROR: Callee = Callee {
        addr: 0x003f_8710,
        f: ee::s14::SLoginMc_GetError,
    };
    pub(crate) const SLOGIN_MC_BUILD_DIR_PATH: Callee = Callee {
        addr: 0x003f_a7a0,
        f: ee::s14::SLoginMc_BuildDirPath,
    };
    pub(crate) const SLOGIN_MC_NEXT_DEVICE_IN_MASK: Callee = Callee {
        addr: 0x003f_85c0,
        f: ee::s14::SLoginMc_NextDeviceInMask,
    };
    pub(crate) const SLOGIN_MC_ABORT_AT_FINISH: Callee = Callee {
        addr: 0x003f_87b0,
        f: ee::s14::SLoginMc_AbortAtFinish,
    };
}

/// Where a delete step goes after its phase handler.
#[derive(Clone, Copy)]
enum DeleteExit {
    /// 0x3fb260: restore s0, then the shared epilogue.
    Return,
    /// 0x3fb1c0: abort at finish (v0 holds the card state), then return.
    AbortAtFinish,
    /// The `SLoginMc_GetError` tails: s0 comes back in the call's delay
    /// slot, so the 0x3fb260 charge is skipped.
    AfterGetError,
    /// Already left (jump out, or an unwind at the retry head).
    Left,
}

fn delete_gp_word(c: &mut Cpu, off: u32) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_add(off))
}

/// `int SLoginMc_DeleteStep(int result, int* done, int* card_reply)`
/// (0x3fae50): advance the memory card delete sequence after a card RPC.
/// `a0` is the RPC result, `a1` receives the completion status and `a2`
/// points at the RPC's returned word. The phase word at +4 of the card state
/// (`gp+0x1ae8`) picks the step: open the save directory (1), check it (2),
/// remove `icon.sys` (19), `view.ico` (20), the data file (21) and the
/// directory (18), then finish or retry on the next selected card (26).
/// Returns 0 while pending, 0x12 for bad arguments or an unknown phase.
/// Dispatcher-shaped: re-entered at [`DELETE_RETRY_HEAD`] after an unwind at
/// the retry back edge.
pub fn slogin_mc_delete_step(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        let exit = match c.pc {
            DELETE_ENTRY => {
                c.cycles += 7;
                Frame::open(c, 0x80);
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0x50);
                delete_step_phase(c)?
            }
            DELETE_RETRY_HEAD => {
                delete_retry_device(c)?;
                DeleteExit::Return
            }
            pc => {
                c.jump_out(pc, DELETE_ENTRY, DELETE_END);
                return Ok(());
            }
        };
        match exit {
            DeleteExit::Return => {
                c.cycles += 1;
                frame.restore(c, S0, 0x70);
            }
            DeleteExit::AbortAtFinish => {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(DELETE_DEVICE));
                callee::SLOGIN_MC_ABORT_AT_FINISH.call(c, 0x003f_b1cc)?;
                frame.restore(c, S0, 0x70);
            }
            DeleteExit::AfterGetError => frame.restore(c, S0, 0x70),
            DeleteExit::Left => return Ok(()),
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// Argument and mode checks, then the phase dispatch through the game's table.
fn delete_step_phase(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if c.g[S1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x12;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 2;
    c.g[V0][0] = 0x12;
    if c.g[A2][0] == 0 {
        return Ok(DeleteExit::Return);
    }
    // Only card modes 2 and 5 delete.
    c.cycles += 5;
    c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
    c.g[V1][0] = 2;
    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
    c.g[V0][0] = 5;
    if c.g[A0][0] == c.g[V1][0] {
        c.cycles += 1;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        let mode_five = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !mode_five {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
            return Ok(DeleteExit::Return);
        }
    }
    c.cycles += 7;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = delete_gp_word(c, DELETE_STATE_GP);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(DELETE_PHASE));
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = (c.g[A0][0] < 26) as u64;
    c.g[V0][0] = 0x49_0000;
    if c.g[V1][0] == 0 {
        return Ok(delete_invalid_phase(c));
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(DELETE_DISPATCH & 0xffff));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    match target {
        0x003f_aed0 => delete_open_dir(c),
        0x003f_af98 => delete_check_dir(c),
        0x003f_b00c => delete_after_icon_sys(c),
        0x003f_b080 => delete_after_view_icon(c),
        0x003f_b0f0 => delete_after_data_file(c),
        0x003f_b15c => delete_after_dir(c),
        0x003f_b1d4 => delete_finish(c),
        0x003f_b254 => Ok(delete_invalid_phase(c)),
        _ => {
            c.jump_out(target, DELETE_ENTRY, DELETE_END);
            Ok(DeleteExit::Left)
        }
    }
}

/// 0x3fb254: unknown phase; done = 2, return 0x12.
fn delete_invalid_phase(c: &mut Cpu) -> DeleteExit {
    c.cycles += 3;
    c.g[V1][0] = 2;
    c.g[V0][0] = 0x12;
    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
    DeleteExit::Return
}

/// 0x3fb248: done = v1 (2), return the RPC result.
fn delete_result_error(c: &mut Cpu) -> DeleteExit {
    c.cycles += 3;
    c.g[V0][0] = c.g[S0][0];
    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
    DeleteExit::Return
}

/// 0x3faf60, also the resume head: back to phase 1 on the next selected
/// card through the interface's +0x10 callback (`a1` = card state).
fn delete_retry_device(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 8;
    c.g[V1][0] = delete_gp_word(c, DELETE_IFACE_GP);
    c.g[V0][0] = 1;
    c.sw((c.g[A1][0] as u32).wrapping_add(DELETE_PHASE), c.g[V0][0]);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
    let callback = c.g[V0][0] as u32;
    call_ptr(c, callback, 0x003f_af78)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// 0x3fb068: remove the file named by `a1` in the save's path through the
/// interface's +0x24 callback (`a0` = card state, `v1` = interface).
fn delete_remove_file(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 6;
    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(DELETE_DEVICE));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
    let callback = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(DELETE_PATH));
    call_ptr(c, callback, 0x003f_b078)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// 0x3fb1b4: `SLoginMc_SetError(a0)`, then v0 = card state.
fn delete_set_error_reload(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 3;
    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_b1bc)?;
    c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
    Ok(())
}

/// Phase 1: clear the error, then open the save directory (result 0); after
/// result 3, in mode 2, move to the next selected card.
fn delete_open_dir(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = 0;
    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_aed8)?;
    let failed = c.g[S0][0] != 0;
    c.g[V0][0] = 3;
    if !failed {
        c.cycles += 16;
        c.g[A1][0] = delete_gp_word(c, DELETE_IFACE_GP);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = delete_gp_word(c, DELETE_STATE_GP);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(DELETE_DIR_NAME));
        callee::SLOGIN_MC_BUILD_DIR_PATH.call(c, 0x003f_aef8)?;
        c.g[V1][0] = delete_gp_word(c, DELETE_STATE_GP);
        c.g[V0][0] = 2;
        c.g[A2][0] = delete_gp_word(c, DELETE_IFACE_GP);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(DELETE_PHASE), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
        let callback = c.g[V0][0] as u32;
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(DELETE_DEVICE));
        call_ptr(c, callback, 0x003f_af18)?;
        c.g[V0][0] = 0;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 2;
    c.g[V1][0] = 2;
    if c.g[S0][0] != c.g[V0][0] {
        return Ok(delete_result_error(c));
    }
    c.cycles += 5;
    c.g[V1][0] = delete_gp_word(c, DELETE_STATE_GP);
    c.g[S0][0] = 2;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    if c.g[V0][0] != c.g[S0][0] {
        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
        c.cycles += 2;
        c.g[V0][0] = 3;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 9;
    c.g[V0][0] = delete_gp_word(c, DELETE_IFACE_GP);
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(DELETE_SCAN_DIRECTION));
    c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
    callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.call(c, 0x003f_af4c)?;
    c.g[A1][0] = delete_gp_word(c, DELETE_STATE_GP);
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    let exhausted = c.g[A0][0] == c.g[V0][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(DELETE_DEVICE), c.g[A0][0]);
    if exhausted {
        c.cycles += 4;
        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
        callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_af88)?;
        return Ok(DeleteExit::AfterGetError);
    }
    delete_retry_device(c)?;
    Ok(DeleteExit::Return)
}

/// Phase 2: the directory lookup's reply. 1: enter it and remove
/// `icon.sys` (phase 19); 0: error 4 and abort; else done with 10.
fn delete_check_dir(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 2;
    c.g[V0][0] = 1;
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_b004)?;
        c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 3;
    c.g[A2][0] = c.lw(c.g[A2][0] as u32);
    let found = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = delete_gp_word(c, DELETE_IFACE_GP);
    if found {
        c.cycles += 11;
        c.g[A0][0] = delete_gp_word(c, DELETE_STATE_GP);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let callback = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(DELETE_PATH));
        call_ptr(c, callback, 0x003f_afbc)?;
        c.g[A0][0] = delete_gp_word(c, DELETE_STATE_GP);
        c.g[V0][0] = 0x13;
        c.g[V1][0] = delete_gp_word(c, DELETE_IFACE_GP);
        c.g[A1][0] = 0x49_0000;
        c.sw((c.g[A0][0] as u32).wrapping_add(DELETE_PHASE), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(DELETE_ICON_SYS & 0xffff));
        delete_remove_file(c)?;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 2;
    c.g[V1][0] = 2;
    if c.g[A2][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        return Ok(DeleteExit::Return);
    }
    c.cycles += 4;
    c.g[A0][0] = 4;
    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_afe8)?;
    c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
    Ok(DeleteExit::AbortAtFinish)
}

/// The result check shared by phases 19-21. Result 3: abort at finish
/// (with error 3 unless one is already set); true, v0 = card state. Else:
/// unless the result is 4, record it as the error when none is set; false,
/// `state_reg` = card state. `rets`: the return addresses of the two
/// `SLoginMc_GetError` calls and of the `SLoginMc_SetError` call.
fn delete_removal_result(c: &mut Cpu, state_reg: usize, rets: [u32; 3]) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let three = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = 4;
    if three {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, rets[0])?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
        if !failed {
            c.cycles += 2;
            c.g[A0][0] = 3;
            delete_set_error_reload(c)?;
        }
        return Ok(true);
    }
    c.cycles += 2;
    let four = c.g[S0][0] == c.g[V0][0];
    c.g[state_reg][0] = delete_gp_word(c, DELETE_STATE_GP);
    if !four {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, rets[1])?;
        let failed = c.g[V0][0] != 0;
        c.g[state_reg][0] = delete_gp_word(c, DELETE_STATE_GP);
        if !failed {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::SLOGIN_MC_SET_ERROR.call(c, rets[2])?;
            c.g[state_reg][0] = delete_gp_word(c, DELETE_STATE_GP);
        }
    }
    Ok(false)
}

/// Phase 19: `icon.sys` is gone; remove `view.ico` (phase 20).
fn delete_after_icon_sys(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if delete_removal_result(c, A0, [0x003f_b020, 0x003f_b040, 0x003f_b050])? {
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 5;
    c.g[V0][0] = 0x14;
    c.g[V1][0] = delete_gp_word(c, DELETE_IFACE_GP);
    c.g[A1][0] = 0x49_0000;
    c.sw((c.g[A0][0] as u32).wrapping_add(DELETE_PHASE), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(DELETE_VIEW_ICON & 0xffff));
    delete_remove_file(c)?;
    Ok(DeleteExit::Return)
}

/// Phase 20: `view.ico` is gone; remove the data file (phase 21).
fn delete_after_view_icon(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if delete_removal_result(c, V0, [0x003f_b094, 0x003f_b0b4, 0x003f_b0c4])? {
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 10;
    c.g[V1][0] = 0x15;
    c.g[A1][0] = delete_gp_word(c, DELETE_IFACE_GP);
    c.sw((c.g[V0][0] as u32).wrapping_add(DELETE_PHASE), c.g[V1][0]);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(DELETE_PATH));
    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(DELETE_DEVICE));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
    let callback = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[A0][0];
    call_ptr(c, callback, 0x003f_b0e8)?;
    c.g[V0][0] = 0;
    Ok(DeleteExit::Return)
}

/// Phase 21: the data file is gone; remove the directory through the
/// interface's +0x28 callback (phase 18).
fn delete_after_data_file(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if delete_removal_result(c, V1, [0x003f_b104, 0x003f_b124, 0x003f_b134])? {
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 9;
    c.g[V0][0] = 0x12;
    c.g[A2][0] = delete_gp_word(c, DELETE_IFACE_GP);
    c.sw((c.g[V1][0] as u32).wrapping_add(DELETE_PHASE), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(DELETE_PATH));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x28));
    let callback = c.g[V0][0] as u32;
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(DELETE_DEVICE));
    call_ptr(c, callback, 0x003f_b154)?;
    c.g[V0][0] = 0;
    Ok(DeleteExit::Return)
}

/// Phase 18: the directory is gone. A failed removal aborts at finish
/// (recording the result unless an error is set); else clear the error
/// when none is set and finish through the interface's +0x14 callback
/// (phase 26).
fn delete_after_dir(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b1a8)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
        if !failed {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            delete_set_error_reload(c)?;
        }
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 4;
    callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b16c)?;
    let failed = c.g[V0][0] != 0;
    c.g[V1][0] = delete_gp_word(c, DELETE_STATE_GP);
    if !failed {
        c.cycles += 3;
        c.g[A0][0] = 0;
        callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_b17c)?;
        c.g[V1][0] = delete_gp_word(c, DELETE_STATE_GP);
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1a;
    c.g[A1][0] = delete_gp_word(c, DELETE_IFACE_GP);
    c.sw((c.g[V1][0] as u32).wrapping_add(DELETE_PHASE), c.g[V0][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    let callback = c.g[V0][0] as u32;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(DELETE_DEVICE));
    call_ptr(c, callback, 0x003f_b198)?;
    c.g[V0][0] = 0;
    Ok(DeleteExit::Return)
}

/// Phase 26: the transaction finished. Mode 5, or no further selected card,
/// reports done (2) and returns the error; mode 2 without an error is done
/// with 0; mode 2 with an error retries on the next selected card (the back
/// edge to [`DELETE_RETRY_HEAD`]).
fn delete_finish(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 2;
    c.g[V1][0] = 2;
    if c.g[S0][0] != 0 {
        return Ok(delete_result_error(c));
    }
    c.cycles += 5;
    c.g[V0][0] = delete_gp_word(c, DELETE_STATE_GP);
    c.g[A0][0] = 5;
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let mode_five = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = 2;
    if !mode_five {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b1f8)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = delete_gp_word(c, DELETE_IFACE_GP);
        if !failed {
            c.cycles += 4;
            c.g[V1][0] = 2;
            c.g[V0][0] = 0;
            c.sw(c.g[S1][0] as u32, c.g[V1][0]);
            return Ok(DeleteExit::Return);
        }
        c.cycles += 9;
        c.g[V1][0] = delete_gp_word(c, DELETE_STATE_GP);
        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(DELETE_SCAN_DIRECTION));
        callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.call(c, 0x003f_b220)?;
        c.g[A1][0] = delete_gp_word(c, DELETE_STATE_GP);
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        let more = c.g[A0][0] != c.g[V0][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(DELETE_DEVICE), c.g[A0][0]);
        if more {
            if back_edge_unwinds(c, DELETE_RETRY_HEAD) {
                return Ok(DeleteExit::Left);
            }
            delete_retry_device(c)?;
            return Ok(DeleteExit::Return);
        }
        c.cycles += 1;
        c.g[V0][0] = 2;
    }
    c.cycles += 4;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b240)?;
    Ok(DeleteExit::AfterGetError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, require_paths, DATA, GAME_GP};

    const DELETE_TEST_STATUS: u32 = DATA + 0x1000;
    const DELETE_TEST_REPLY: u32 = DATA + 0x1004;
    const DELETE_TEST_STATE: u32 = DATA + 0x2000;
    const DELETE_TEST_IFACE: u32 = DATA + 0x3000;
    /// The card error word `SLoginMc_GetError`/`SetError` use.
    const DELETE_TEST_ERROR: u32 = DELETE_TEST_STATE + 8;

    /// The card state (mode word, phase, error, device 0, direction 0, path
    /// and directory name), the interface (device mask halfword, fake
    /// callbacks, card name) and a non-zero completion word. The game image
    /// supplies the dispatch table and the file names.
    fn delete_step_card(c: &mut Cpu, mode: u32, phase: u32, error: u32, mask: u16) {
        elf_image(c);
        c.bus.write32(GAME_GP + DELETE_STATE_GP, DELETE_TEST_STATE);
        c.bus.write32(GAME_GP + DELETE_IFACE_GP, DELETE_TEST_IFACE);
        c.bus.write32(DELETE_TEST_STATE, mode);
        c.bus.write32(DELETE_TEST_STATE + DELETE_PHASE, phase);
        c.bus.write32(DELETE_TEST_ERROR, error);
        c.bus.write32(DELETE_TEST_STATE + DELETE_DEVICE, 0);
        c.bus.write32(DELETE_TEST_STATE + DELETE_SCAN_DIRECTION, 0);
        c.bus.write16(DELETE_TEST_IFACE, mask);
        for (i, off) in [4u32, 0x10, 0x14, 0x24, 0x28, 0x44].into_iter().enumerate() {
            c.bus
                .write32(DELETE_TEST_IFACE + off, FAKE + 0x10 * i as u32);
        }
        for (i, b) in b"CARD\0".iter().enumerate() {
            c.bus.write8(DELETE_TEST_IFACE + 0x50 + i as u32, *b);
        }
        for (i, b) in b"/SAVE/DATA\0".iter().enumerate() {
            c.bus.write8(DELETE_TEST_STATE + DELETE_PATH + i as u32, *b);
        }
        for (i, b) in b"SSX3\0".iter().enumerate() {
            c.bus
                .write8(DELETE_TEST_STATE + DELETE_DIR_NAME + i as u32, *b);
        }
        c.bus.write32(DELETE_TEST_STATUS, 0xa5a5_a5a5);
    }

    /// `seed % 14` picks the case (a phase, or an argument/mode/table edge),
    /// `seed / 14 % 4` the RPC result (0, 3, 4, 7), `seed / 56 % 4` the card:
    /// (mode, error, mask, reply) = (2, 0, found, 1), (2, 5, found, 0),
    /// (2, 5, none, -1), (5, 0, none, 2).
    fn delete_step_fixture(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3fae_5000, 2);
        c.pc = DELETE_ENTRY;
        let case = seed % 14;
        let result = [0u64, 3, 4, 7][(seed / 14 % 4) as usize];
        let card = (seed / 56 % 4) as usize;
        let (mode, error, mask) = [(2, 0, 0xff), (2, 5, 0xff), (2, 5, 0), (5, 0, 0)][card];
        let reply = [1u32, 0, u32::MAX, 2][card];
        let phase = match case {
            0 => 1,
            1 => 2,
            2 => 18,
            3 => 19,
            4 => 20,
            5 => 21,
            6 | 13 => 26,
            // Table rows that point at the unknown-phase exit.
            7 => [3, 9, 17, 22][card],
            // Outside the table's 26 rows.
            8 => [0, 27, 28, 0x8000_0000][card],
            _ => 1,
        };
        // Case 11: a card mode other than 2 and 5.
        let mode = if case == 11 {
            [0, 1, 3, 11][card]
        } else {
            mode
        };
        delete_step_card(c, mode, phase, error, mask);
        c.bus.write32(DELETE_TEST_REPLY, reply);
        c.g[A0][0] = result;
        c.g[A1][0] = if case == 9 { 0 } else { sx(DELETE_TEST_STATUS) };
        c.g[A2][0] = if case == 10 { 0 } else { sx(DELETE_TEST_REPLY) };
        if case == 12 {
            // A phase-1 row outside the function leaves through jump_out.
            c.bus.write32(DELETE_DISPATCH, FAKE + 0x100);
        }
        if case == 13 {
            // Resumed at the retry head inside the frame (a1 = card state).
            c.pc = DELETE_RETRY_HEAD;
            let frame_sp = crate::difftest::SP - 0x80;
            c.g[SP][0] = sx(frame_sp);
            c.bus.write64(frame_sp + 0x50, crate::difftest::RA as u64);
            c.g[A1][0] = sx(DELETE_TEST_STATE);
        }
    }

    #[test]
    fn slogin_mc_delete_step_matches() {
        check(
            ee::s14::f_003fae50__generated,
            slogin_mc_delete_step,
            224,
            &delete_step_fixture,
        );
        // Cycle totals of the fake-free arms: argument checks (14), bad mode
        // (23), phase outside the table (30), unknown-phase row (36), phase
        // 26 bad result (38), phase 2 odd reply (43), phase 26 mode 5 (47),
        // phase 1 bad result (50), phase 26 done (51), phase 1 mode 5 (55),
        // phase 26 no further card (138), phase 1 no further card (142).
        require_paths(
            ee::s14::f_003fae50__generated,
            224,
            &[14, 23, 30, 36, 38, 43, 47, 50, 51, 55, 138, 142],
            &delete_step_fixture,
        );
    }

    // Both sides re-enter at the retry head after an unwind, as the trampoline does.
    crate::difftest::events::driven!(
        driven_generated_mc_delete_step,
        ee::s14::f_003fae50__generated
    );
    crate::difftest::events::driven!(driven_slogin_mc_delete_step, slogin_mc_delete_step);

    #[test]
    fn slogin_mc_delete_step_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            driven_generated_mc_delete_step,
            driven_slogin_mc_delete_step,
            60,
            &|c, seed| {
                // Phase 26 in mode 2 with an error and another selected card:
                // the back edge to the retry head 0x3faf60 is the first event
                // check (the device scan finds card 1 without looping).
                let mut r = machine(c, seed, 0x3fae_50ee, 0);
                delete_step_card(c, 2, 26, 5, 0xff);
                let mode = schedule(c, &mut r, seed, DELETE_ENTRY);
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(DELETE_TEST_STATUS);
                c.g[A2][0] = sx(DELETE_TEST_REPLY);
                mode
            },
        );
    }
}
