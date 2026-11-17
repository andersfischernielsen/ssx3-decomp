//! Staging module oc1022 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1022`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_7ee0, slogin_init, "SLogin_Init"),
    (0x003f_7f18, slogin_shutdown, "SLogin_Shutdown"),
    (0x003f_7f48, slogin_create, "SLogin_Create"),
    (0x003f_8000, slogin_load, "SLogin_Load"),
    (0x003f_8048, slogin_update, "SLogin_Update"),
    (0x003f_80c0, slogin_delete, "SLogin_Delete"),
    (0x003f_80e0, slogin_poll, "SLogin_Poll"),
    (0x003f_8178, slogin_map_store_error, "SLogin_MapStoreError"),
    (
        0x003f_8230,
        slogin_get_record_alloc_size,
        "SLogin_GetRecordAllocSize",
    ),
    (0x003f_8258, slogin_setup_icon_sys, "SLogin_SetupIconSys"),
    (
        0x003f_8550,
        slogin_mc_first_device_in_mask,
        "SLoginMc_FirstDeviceInMask",
    ),
    (
        0x003f_8588,
        slogin_mc_last_device_in_mask,
        "SLoginMc_LastDeviceInMask",
    ),
    (
        0x003f_85c0,
        slogin_mc_next_device_in_mask,
        "SLoginMc_NextDeviceInMask",
    ),
    (
        0x003f_86b8,
        slogin_mc_count_devices_in_mask,
        "SLoginMc_CountDevicesInMask",
    ),
    (0x003f_86f0, slogin_mc_set_error, "SLoginMc_SetError"),
    (0x003f_8710, slogin_mc_get_error, "SLoginMc_GetError"),
    (
        0x003f_8720,
        slogin_mc_abort_at_flush,
        "SLoginMc_AbortAtFlush",
    ),
    (
        0x003f_8768,
        slogin_mc_abort_at_close,
        "SLoginMc_AbortAtClose",
    ),
    (
        0x003f_87b0,
        slogin_mc_abort_at_finish,
        "SLoginMc_AbortAtFinish",
    ),
    (0x003f_8c38, slogin_mc_open_step, "SLoginMc_OpenStep"),
];

const CARD_STATE_GP: u32 = 0x1ae8;
const CARD_ERROR: u32 = 0x08;
const CARD_STATUS: u32 = 0x04;
const CARD_FILE: u32 = 0x0c;
const CARD_IFACE_GP: u32 = 0x1aec;
const STORE_ERROR_TABLE: u32 = 0x0049_7150;
const LOGIN_STATE_GP: u32 = 0x1ad4;
const LOGIN_RECORD: u32 = 0xcc;

mod callee {
    use super::*;
    pub(crate) const SLOGIN_START_OP: Callee = Callee {
        addr: 0x003f_6e98,
        f: ee::s14::SLogin_StartOp,
    };
    pub(crate) const SLOGIN_STR_COPY_N: Callee = Callee {
        addr: 0x003f_7e90,
        f: ee::s14::SLogin_StrCopyN,
    };
    pub(crate) const SLOGIN_SHUTDOWN_STORE: Callee = Callee {
        addr: 0x003f_6e70,
        f: ee::s14::SLogin_ShutdownStore,
    };
    pub(crate) const SLOGIN_FREE_STATE: Callee = Callee {
        addr: 0x003f_6d98,
        f: ee::s14::SLogin_FreeState,
    };
    pub(crate) const SLOGIN_CLEAR_ACTIVE: Callee = Callee {
        addr: 0x003f_6c60,
        f: ee::s14::SLogin_ClearActive,
    };
    pub(crate) const SLOGIN_FINISH_OP: Callee = Callee {
        addr: 0x003f_7a30,
        f: ee::s14::SLogin_FinishOp,
    };
    pub(crate) const SLOGIN_VERIFY_ICON_CHECKSUM: Callee = Callee {
        addr: 0x003f_7de0,
        f: ee::s14::SLogin_VerifyIconChecksum,
    };
    pub(crate) const SLOGIN_SETUP_ICON_SYS: Callee = Callee {
        addr: 0x003f_8258,
        f: ee::s14::SLogin_SetupIconSys,
    };
    pub(crate) const SLOGIN_SET_ACTIVE: Callee = Callee {
        addr: 0x003f_6c50,
        f: ee::s14::SLogin_SetActive,
    };
    pub(crate) const SLOGIN_ALLOC_STATE: Callee = Callee {
        addr: 0x003f_6c70,
        f: ee::s14::SLogin_AllocState,
    };
    pub(crate) const SLOGIN_INIT_STORE: Callee = Callee {
        addr: 0x003f_6dc8,
        f: ee::s14::SLogin_InitStore,
    };
}

/// `void SLogin_Init(const LoginConfig* config)` (0x3f7ee0): mark shared
/// login active, allocate its state, then initialise the record store.
pub fn slogin_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_SET_ACTIVE.call(c, 0x003f_7ef4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SLOGIN_ALLOC_STATE.call(c, 0x003f_7efc)?;
        callee::SLOGIN_INIT_STORE.call(c, 0x003f_7f04)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Create(const void* icon, const char* account_and_password)`
/// (0x3f7f48): copy login fields when needed, validate and stage the save
/// icon, then start creating the account record.
pub fn slogin_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        let has_icon = c.g[S2][0] != 0;
        frame.save_ra(c, 0);
        let copy_fields = if !has_icon {
            c.cycles += 2;
            let has_fields = c.g[S0][0] != 0;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            if has_fields {
                true
            } else {
                c.cycles += 2;
                c.g[S1][0] = 7;
                false
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            true
        };
        if copy_fields {
            c.cycles += 9;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa8));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_7f90)?;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb9));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_7fa4)?;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        if c.g[S1][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            callee::SLOGIN_VERIFY_ICON_CHECKSUM.call(c, 0x003f_7fb4)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
            if c.g[S1][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                callee::SLOGIN_SETUP_ICON_SYS.call(c, 0x003f_7fc8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
                if c.g[S1][0] == 0 {
                    c.cycles += 4;
                    c.g[A0][0] = 0;
                    callee::SLOGIN_START_OP.call(c, 0x003f_7fdc)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Poll(int* out_state)` (0x3f80e0): poll the current login
/// operation, finish it when its step reports completion, and return its error.
pub fn slogin_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        let has_output = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        let mut outer_exit = false;
        if !has_output {
            c.cycles += 2;
            c.g[S2][0] = 7;
        } else {
            c.cycles += 6;
            c.sw(c.g[S0][0] as u32, 0);
            c.g[V1][0] = 4;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xd0)).lw(c);
            let finished = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = c.g[S2][0];
            if finished {
                outer_exit = true;
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xd4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, target, 0x003f_812c)?;
                c.g[S2][0] = c.g[V0][0];
                let step_error = c.g[S2][0] != 0;
                c.g[V0][0] = 2;
                if step_error {
                    c.cycles += 2;
                    callee::SLOGIN_FINISH_OP.call(c, 0x003f_815c)?;
                } else {
                    c.cycles += 3;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    let step_done = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if step_done {
                        c.cycles += 4;
                        callee::SLOGIN_FINISH_OP.call(c, 0x003f_814c)?;
                        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    } else {
                        outer_exit = true;
                    }
                }
            }
        }
        if !outer_exit {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SLogin_Shutdown()` (0x3f7f18): close the shared login store,
/// release its state, and clear the active login flag.
pub fn slogin_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_SHUTDOWN_STORE.call(c, 0x003f_7f28)?;
        callee::SLOGIN_FREE_STATE.call(c, 0x003f_7f30)?;
        callee::SLOGIN_CLEAR_ACTIVE.call(c, 0x003f_7f38)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Update(const char* account_and_password)` (0x3f8048):
/// copy the two fixed length login fields, then begin a store update.
pub fn slogin_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let has_fields = c.g[S0][0] != 0;
        c.g[S1][0] = 0;
        if has_fields {
            c.cycles += 10;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa8));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_8080)?;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb9));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_8094)?;
        } else {
            c.cycles += 2;
            c.g[S1][0] = 7;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = 1;
            callee::SLOGIN_START_OP.call(c, 0x003f_80a4)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Load(void* account)` (0x3f8000): save the account record
/// pointer and begin the login store's load operation.
pub fn slogin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0;
        let has_account = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        if has_account {
            c.cycles += 2;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_STATE_GP)).lw(c);
            c.sw((c.g[V0][0] as u32).wrapping_add(LOGIN_RECORD), c.g[A0][0]);
        } else {
            c.cycles += 2;
            c.g[V1][0] = 7;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = 2;
            callee::SLOGIN_START_OP.call(c, 0x003f_8030)?;
            c.g[V1][0] = c.g[V0][0];
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Delete()` (0x3f80c0): start deletion of the saved account
/// record in the shared login store.
pub fn slogin_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 3;
        callee::SLOGIN_START_OP.call(c, 0x003f_80d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_MapStoreError(int store_error)` (0x3f8178): translate a
/// shared login store error into the code returned to the login caller.
pub fn slogin_map_store_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 0x73) as u64;
    let outside_table = c.g[V0][0] == 0;
    c.g[V0][0] = 0x49_0000;
    if outside_table {
        c.cycles += 2;
        c.g[V0][0] = 0x18;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7150));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    let result = match target {
        0x003f_819c => 0,
        0x003f_81a4 => 1,
        0x003f_81ac => 2,
        0x003f_81b4 => 3,
        0x003f_81bc => 4,
        0x003f_81c4 => 5,
        0x003f_81cc => 6,
        0x003f_81d4 => 0x13,
        0x003f_81dc => 0x14,
        0x003f_81e4 => 7,
        0x003f_81ec => 0x0b,
        0x003f_81f4 => 0x18,
        _ => {
            c.jump_out(target, 0x003f_8178, 0x003f_81fc);
            return;
        }
    };
    c.cycles += 2;
    c.g[V0][0] = result;
    ret(c);
}

/// `int SLogin_GetRecordAllocSize(int* out)` (0x3f8230): ask the shared
/// login store for its record block size, then return success.
pub fn slogin_get_record_alloc_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x22;
        call_ptr(c, 0x003f_cf98, 0x003f_8244)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_FirstDeviceInMask(unsigned mask)` (0x3f8550): return the
/// first of the eight memory card devices selected by the low mask byte.
pub fn slogin_mc_first_device_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] &= 0xffff;
    c.g[V0][0] = c.g[A0][0] & 1;
    c.g[V1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 3;
            let in_range = (c.g[V1][0] as i64) < 8;
            c.g[V0][0] = in_range as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            if !in_range {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x003f_8568);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `int SLoginMc_LastDeviceInMask(unsigned mask)` (0x3f8588): return the
/// last selected memory card device among the eight slots.
pub fn slogin_mc_last_device_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] &= 0xffff;
    c.g[V0][0] = c.g[A0][0] & 0x80;
    c.g[V1][0] = 7;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        loop {
            c.cycles += 2;
            let below_zero = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            if below_zero {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            if event_due(c) {
                deliver_event_here(c, 0x003f_85a0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `int SLoginMc_NextDeviceInMask(unsigned mask, int direction)` (0x3f85c0):
/// scan from the current card device to the next selected device in the
/// requested direction; return -1 at either end or 18 for an invalid mode.
pub fn slogin_mc_next_device_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP)).lw(c);
    c.g[A0][0] &= 0xffff;
    c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
    if c.g[A1][0] == 0 {
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V1][0] = ((c.g[A2][0] as i64) < 8) as u64;
        c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] &= 1;
            let selected = c.g[V0][0] != 0;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
            if !selected {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 16);
                c.g[V0][0] = 0x1_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x1_0000;
                c.g[V0][0] = c.g[V1][0];
                loop {
                    c.cycles += 4;
                    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    let exhausted = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if exhausted {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
                    c.g[V0][0] &= 1;
                    let absent = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0];
                    if !absent {
                        c.cycles += 1;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_8608);
                    }
                }
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
        }
        c.cycles += 3;
        c.g[A0][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.g[A0][0] = c.g[A2][0];
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let bad_mode = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        if bad_mode {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
            ret(c);
            return;
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        let below_zero = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
        if !below_zero {
            c.cycles += 3;
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 16);
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] = c.g[V1][0];
                loop {
                    c.cycles += 3;
                    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    let exhausted = (c.g[A2][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if exhausted {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
                    c.g[V0][0] &= 1;
                    let absent = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0];
                    if !absent {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_8678);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[A2][0];
            } else {
                c.g[A0][0] = c.g[A2][0];
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[A2][0];
        }
        c.cycles += 4;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `int SLoginMc_CountDevicesInMask(unsigned mask)` (0x3f86b8): count the
/// enabled devices in the low byte of a memory card device mask.
pub fn slogin_mc_count_devices_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = 0;
    c.g[A3][0] = c.g[A0][0] & 0xffff;
    c.g[A1][0] = 0;
    c.g[T0][0] = 1;
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[T0][0] as u32) << ((c.g[A1][0] as u32) & 31));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] &= c.g[A3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < 8) as u64;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = c.g[A0][0];
        }
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0];
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003f_86c8);
        }
    }
}

/// `void SLoginMc_SetError(int error)` (0x3f86f0): store a card error only
/// when the card state is outside the three reserved states 11 through 13.
pub fn slogin_mc_set_error(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(11));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    if c.g[V0][0] == 0 {
        let error = c.g[A0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(CARD_ERROR)).sw(c, error);
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_GetError()` (0x3f8710): read the last memory card error.
pub fn slogin_mc_get_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(CARD_ERROR)).lw(c);
    ret(c);
}

/// `int SLoginMc_AbortAtFlush(int* done)` (0x3f8720): mark the pending card
/// flush complete and call the store's flush callback.
pub fn slogin_mc_abort_at_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = 11;
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = 0x18;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP)).lw(c);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP)).lw(c);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(CARD_STATUS), c.g[V1][0]);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(CARD_FILE)).lw(c);
        call_ptr(c, target, 0x003f_8754)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_AbortAtClose(int* done)` (0x3f8768): mark the pending card
/// close complete and call the store's close callback.
pub fn slogin_mc_abort_at_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = 12;
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = 0x19;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP)).lw(c);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP)).lw(c);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(CARD_STATUS), c.g[V1][0]);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(CARD_FILE)).lw(c);
        call_ptr(c, target, 0x003f_879c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_AbortAtFinish(int* done)` (0x3f87b0): mark the pending
/// operation complete and notify the card store's finish callback.
pub fn slogin_mc_abort_at_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A3][0] = 13;
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[A2][0] = 0x1a;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP)).lw(c);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP)).lw(c);
        c.sw((c.g[V1][0] as u32).wrapping_add(CARD_STATUS), c.g[A2][0]);
        c.sw(c.g[V1][0] as u32, c.g[A3][0]);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003f_87e4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_SetupIconSys(const void* icon)` (0x3f8258): build the
/// memory card icon metadata and hand it to the login store.
pub fn slogin_setup_icon_sys(c: &mut Cpu) {
    c.cycles += 85;
    let frame = Frame::open(c, 0x520);
    // Reserve the title buffer and icon.sys payload in the guest frame.
    c.g[A2][0] = 0x40;
    frame.save(c, 30, 0x490);
    c.g[A1][0] = 0u64;
    c.g[FP][0] = c.g[A0][0];
    frame.save(c, 16, 0x510);
    frame.save(c, 17, 0x500);
    frame.save(c, 18, 0x4f0);
    frame.save(c, 19, 0x4e0);
    frame.save(c, 20, 0x4d0);
    frame.save(c, 21, 0x4c0);
    frame.save(c, 22, 0x4b0);
    frame.save(c, 23, 0x4a0);
    frame.save_ra(c, 0x480);
    c.g[RA][0] = sx(0x003f8298);
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3d0));
    c.pc = 0x00416210;
    ssx3_ee_rt::funcs::call_indirect(c, 0x00416210);
    if c.pc != 0x003f8298 {
        return;
    }
    c.g[V1][0] = 0x80;
    c.g[V0][0] = 0x490000;
    // Three static icon.sys template runs are copied to the guest frame.
    c.sw((c.g[SP][0] as u32).wrapping_add(0x3e8), c.g[V1][0]);
    c.g[A0][0] = 0x490000;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x3d8), c.g[V1][0]);
    c.g[A1][0] = 0x490000;
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7320));
    c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[T0][0] as u32);
    c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
    c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x17));
    c.ldr(7, (c.g[T0][0] as u32).wrapping_add(0x10));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x417));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x410));
    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x41f));
    c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x418));
    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x427));
    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x420));
    c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x1f));
    c.ldr(3, (c.g[T0][0] as u32).wrapping_add(0x18));
    c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0x27));
    c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x20));
    c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x2f));
    c.ldr(7, (c.g[T0][0] as u32).wrapping_add(0x28));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x42f));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x428));
    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x437));
    c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x430));
    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x43f));
    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x438));
    c.g[V0][0] = 0x7;
    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7350));
    c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[T0][0] as u32);
    c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
    c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x17));
    c.ldr(7, (c.g[T0][0] as u32).wrapping_add(0x10));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x447));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x440));
    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x44f));
    c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x448));
    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x457));
    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x450));
    c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x1f));
    c.ldr(3, (c.g[T0][0] as u32).wrapping_add(0x18));
    c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0x27));
    c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x20));
    c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x2f));
    c.ldr(7, (c.g[T0][0] as u32).wrapping_add(0x28));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x45f));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x458));
    c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x467));
    c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x460));
    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x46f));
    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x468));
    if c.g[FP][0] != 0 {
        c.g[V0][0] = 0u64;
    }
    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7380));
    c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(3, c.g[A3][0] as u32);
    c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0xf));
    c.ldr(4, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x477));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x470));
    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x47f));
    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x478));
    let t = c.g[V0][0] != 0u64;
    frame.restore(c, 16, 0x510);
    // A null icon has no metadata to install.
    if !t {
        c.cycles += 94;
        // Convert the title to Shift JIS and assemble the icon.sys header.
        c.g[A0][0] = c.g[SP][0] | 0x8;
        c.g[A2][0] = 0x3c4;
        c.g[RA][0] = sx(0x003f83bc);
        c.g[A1][0] = 0u64;
        c.pc = 0x00416210;
        ssx3_ee_rt::funcs::call_indirect(c, 0x00416210);
        if c.pc != 0x003f83bc {
            return;
        }
        c.g[V1][0] = 0x4a0000;
        c.g[A0][0] = 0x490000;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4bd0));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4bd0));
        c.g[A2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7390));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[RA][0] = sx(0x003f83e4);
        c.sb((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.pc = 0x003fa380;
        ee::s14::f_003fa380(c);
        if c.pc != 0x003f83e4 {
            return;
        }
        c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[T7][0] = 0x60;
        c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x408));
        c.g[S1][0] = 0x10;
        c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[T1][0] = 0x490000;
        c.g[A2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x418));
        c.g[S7][0] = 0x296e;
        c.g[A3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x420));
        c.g[S6][0] = 0x22;
        c.g[T0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x428));
        c.g[A0][0] = c.g[SP][0];
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3d0));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3d8));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3e0));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3e8));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3f0));
        c.g[T6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3f8));
        c.g[T2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x430));
        c.g[T3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x438));
        c.g[T4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x440));
        c.g[T5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x448));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0xe), c.g[S1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T7][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S5][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[S3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[S0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T6][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T5][0]);
        c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x450));
        c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x458));
        c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x460));
        c.g[A2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x468));
        c.g[A3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x470));
        c.g[T0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x478));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x98), c.g[V0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[T0][0]);
        c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x73a0));
        c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(8, c.g[A1][0] as u32);
        c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x113));
        c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10c));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x114), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x73a0));
        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(6, c.g[V0][0] as u32);
        c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x153));
        c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x14c));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x154), c.g[A3][0]);
        c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x73a0));
        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A3][0] as u32);
        c.g[A1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x193));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18c));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x194), c.g[A1][0]);
        c.sw(c.g[SP][0] as u32, c.g[FP][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[S7][0]);
        c.g[RA][0] = sx(0x003f8518);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x3cc), c.g[S6][0]);
        // The store owns the final metadata validation and error mapping.
        c.pc = 0x003fcdc8;
        ssx3_ee_rt::funcs::call_indirect(c, 0x003fcdc8);
        if c.pc != 0x003f8518 {
            return;
        }
        c.g[RA][0] = sx(0x003f8520);
        c.g[A0][0] = c.g[V0][0];
        c.pc = 0x003f8178;
        ee::s14::f_003f8178(c);
        if c.pc != 0x003f8520 {
            return;
        }
        frame.restore(c, 16, 0x510);
    }
    c.cycles += 11;
    frame.restore(c, 17, 0x500);
    frame.restore(c, 18, 0x4f0);
    frame.restore(c, 19, 0x4e0);
    frame.restore(c, 20, 0x4d0);
    frame.restore(c, 21, 0x4c0);
    frame.restore(c, 22, 0x4b0);
    frame.restore(c, 23, 0x4a0);
    frame.restore(c, 30, 0x490);
    frame.restore_ra(c, 0x480);
    frame.close(c);
}

/// `int SLoginMc_OpenStep(int result, int* done, int* card_reply)`
/// (0x3f8c38): advance the memory card open sequence through directory,
/// file, and completion phases. The dispatcher also accepts an event resume.
pub fn slogin_mc_open_step(c: &mut Cpu) {
    // Only the start and the card-device retry head can be entered externally.
    let frame = Frame { size: 0xa0 };
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            // Validate the caller's completion and card reply pointers.
            0x003f8c38 => {
                c.cycles += 9;
                Frame::open(c, 0xa0);
                frame.save(c, 16, 0x90);
                frame.save(c, 17, 0x80);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, 18, 0x70);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, 19, 0x60);
                let t = c.g[S1][0] == 0u64;
                frame.save_ra(c, 0x50);
                if t {
                    pc = 0x003f8c80;
                    continue;
                }
                pc = 0x003f8c5c;
                continue;
            }
            0x003f8c5c => {
                c.cycles += 2;
                let t = c.g[A2][0] == 0u64;
                c.g[V0][0] = 0x12;
                if t {
                    pc = 0x003f8edc;
                    continue;
                }
                pc = 0x003f8c64;
                continue;
            }
            0x003f8c64 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[V1][0] = 0x3;
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                let t = c.g[A0][0] == c.g[V1][0];
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x003f8c88;
                    continue;
                }
                pc = 0x003f8c78;
                continue;
            }
            0x003f8c78 => {
                c.cycles += 2;
                let t = c.g[A0][0] == c.g[V0][0];
                c.g[S3][0] = 0x1;
                if t {
                    pc = 0x003f8c8c;
                    continue;
                }
                pc = 0x003f8c80;
                continue;
            }
            0x003f8c80 => {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8c88 => {
                c.cycles += 1;
                c.g[S3][0] = 0x1;
                pc = 0x003f8c8c;
                continue;
            }
            // Select the current card operation phase.
            0x003f8c8c => {
                c.cycles += 6;
                c.g[S2][0] = 0x2;
                c.sw(c.g[S1][0] as u32, c.g[S3][0]);
                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] == c.g[S2][0];
                c.g[V0][0] = (c.g[V1][0] < 0x3u64) as u64;
                if t {
                    pc = 0x003f8d78;
                    continue;
                }
                pc = 0x003f8ca4;
                continue;
            }
            0x003f8ca4 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x16;
                if t {
                    pc = 0x003f8cbc;
                    continue;
                }
                pc = 0x003f8cac;
                continue;
            }
            0x003f8cac => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[S3][0];
                c.g[V1][0] = 0x2;
                if t {
                    pc = 0x003f8cd4;
                    continue;
                }
                pc = 0x003f8cb4;
                continue;
            }
            0x003f8cb4 => {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                pc = 0x003f8ed0;
                continue;
            }
            0x003f8cbc => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x1a;
                if t {
                    pc = 0x003f8e0c;
                    continue;
                }
                pc = 0x003f8cc4;
                continue;
            }
            0x003f8cc4 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V1][0] = 0x2;
                if t {
                    pc = 0x003f8e5c;
                    continue;
                }
                pc = 0x003f8ccc;
                continue;
            }
            0x003f8ccc => {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                pc = 0x003f8ed0;
                continue;
            }
            // Begin opening the save directory or handle its response.
            0x003f8cd4 => {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;
                c.g[V0][0] = 0x3;
                if t {
                    pc = 0x003f8d14;
                    continue;
                }
                pc = 0x003f8cdc;
                continue;
            }
            0x003f8cdc => {
                c.cycles += 14;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x54));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x003f8cf0);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
                c.pc = 0x003fa7a0;
                ee::s14::f_003fa7a0(c);
                if c.pc != 0x003f8cf0 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003f8d0c);
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8d0c {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8d14 => {
                c.cycles += 2;

                if c.g[S0][0] != c.g[V0][0] {
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    pc = 0x003f8ec8;
                    continue;
                }
                pc = 0x003f8d1c;
                continue;
            }
            0x003f8d1c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                let t = c.g[V0][0] != c.g[S0][0];
                c.g[V1][0] = 0x2;
                if t {
                    pc = 0x003f8d6c;
                    continue;
                }
                pc = 0x003f8d28;
                continue;
            }
            0x003f8d28 => {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x6c));
                c.g[RA][0] = sx(0x003f8d38);
                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                c.pc = 0x003f85c0;
                ee::s14::f_003f85c0(c);
                if c.pc != 0x003f8d38 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = 0xffffffffffffffff;
                let t = c.g[A0][0] == c.g[V0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                if t {
                    pc = 0x003f8d68;
                    continue;
                }
                pc = 0x003f8d4c;
                continue;
            }
            0x003f8d4c => {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003f8d60);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8d60 {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8d68 => {
                c.cycles += 1;
                c.g[V1][0] = 0x2;
                pc = 0x003f8d6c;
                continue;
            }
            0x003f8d6c => {
                c.cycles += 3;
                c.g[V0][0] = c.g[S0][0];
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                pc = 0x003f8edc;
                continue;
            }
            // The directory is open: request the account file.
            0x003f8d78 => {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;

                if t {
                    pc = 0x003f8e3c;
                    continue;
                }
                pc = 0x003f8d80;
                continue;
            }
            0x003f8d80 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                let t = c.g[A2][0] != c.g[S3][0];
                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                if t {
                    pc = 0x003f8dd0;
                    continue;
                }
                pc = 0x003f8d8c;
                continue;
            }
            0x003f8d8c => {
                c.cycles += 17;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x54));
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x14));
                c.g[RA][0] = sx(0x003f8da0);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x50));
                c.pc = 0x003fa818;
                ee::s14::f_003fa818(c);
                if c.pc != 0x003f8da0 {
                    return;
                }
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[V0][0] = 0x16;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.g[A3][0] = 0x3;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003f8dc8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34));
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8dc8 {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8dd0 => {
                c.cycles += 2;

                if c.g[A2][0] != 0u64 {
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    pc = 0x003f8e04;
                    continue;
                }
                pc = 0x003f8dd8;
                continue;
            }
            0x003f8dd8 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x003f8de0);
                c.g[A0][0] = 0x4;
                c.pc = 0x003f86f0;
                ee::s14::f_003f86f0(c);
                if c.pc != 0x003f8de0 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[V0][0] = 0x1a;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003f8dfc);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8dfc {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8e04 => {
                c.cycles += 2;
                c.g[V0][0] = 0xa;
                pc = 0x003f8edc;
                continue;
            }
            // A pending card operation has returned its result word.
            0x003f8e0c => {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;

                if t {
                    pc = 0x003f8e3c;
                    continue;
                }
                pc = 0x003f8e14;
                continue;
            }
            0x003f8e14 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                let t = (c.g[A2][0] as i64) < 0;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x003f8e2c;
                    continue;
                }
                pc = 0x003f8e20;
                continue;
            }
            0x003f8e20 => {
                c.cycles += 3;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                pc = 0x003f8edc;
                continue;
            }
            0x003f8e2c => {
                c.cycles += 4;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                c.g[V0][0] = 0xa;
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                pc = 0x003f8edc;
                continue;
            }
            0x003f8e3c => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003f8e44);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x003f86f0;
                ee::s14::f_003f86f0(c);
                if c.pc != 0x003f8e44 {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003f8e54);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.pc = 0x003f87b0;
                ee::s14::f_003f87b0(c);
                if c.pc != 0x003f8e54 {
                    return;
                }
                frame.restore(c, 16, 0x90);
                pc = 0x003f8ee0;
                continue;
            }
            // Retry another device after a card error, or finish this open.
            0x003f8e5c => {
                c.cycles += 2;

                if c.g[S0][0] != 0u64 {
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    pc = 0x003f8ec8;
                    continue;
                }
                pc = 0x003f8e64;
                continue;
            }
            0x003f8e64 => {
                c.cycles += 4;
                c.g[A0][0] = c.lw(c.g[T0][0] as u32);
                c.g[V0][0] = 0x4;
                let t = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x003f8eb8;
                    continue;
                }
                pc = 0x003f8e74;
                continue;
            }
            0x003f8e74 => {
                c.cycles += 2;
                let t = c.g[A0][0] == c.g[V0][0];

                if t {
                    pc = 0x003f8eb8;
                    continue;
                }
                pc = 0x003f8e7c;
                continue;
            }
            0x003f8e7c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003f8e84);
                c.pc = 0x003f8710;
                ee::s14::f_003f8710(c);
                if c.pc != 0x003f8e84 {
                    return;
                }
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                if t {
                    pc = 0x003f8e94;
                    continue;
                }
                pc = 0x003f8e8c;
                continue;
            }
            0x003f8e8c => {
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                pc = 0x003f8ed8;
                continue;
            }
            0x003f8e94 => {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_IFACE_GP));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6c));
                c.g[RA][0] = sx(0x003f8ea4);
                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                c.pc = 0x003f85c0;
                ee::s14::f_003f85c0(c);
                if c.pc != 0x003f8ea4 {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(CARD_STATE_GP));
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = 0xffffffffffffffff;
                let t = c.g[A0][0] != c.g[V0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                if t {
                    if back_edge_unwinds(c, 0x003f_8d4c) {
                        return;
                    }
                    pc = 0x003f8d4c;
                    continue;
                }
                pc = 0x003f8eb8;
                continue;
            }
            0x003f8eb8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003f8ec0);
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                c.pc = 0x003f8710;
                ee::s14::f_003f8710(c);
                if c.pc != 0x003f8ec0 {
                    return;
                }
                frame.restore(c, 16, 0x90);
                pc = 0x003f8ee0;
                continue;
            }
            0x003f8ec8 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[S0][0];
                pc = 0x003f8edc;
                continue;
            }
            0x003f8ed0 => {
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                pc = 0x003f8edc;
                continue;
            }
            0x003f8ed8 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            // Shared return path, also reached after a resumed retry.
            0x003f8edc => {
                c.cycles += 1;
                frame.restore(c, 16, 0x90);
                pc = 0x003f8ee0;
                continue;
            }
            0x003f8ee0 => {
                c.cycles += 6;
                frame.restore(c, 17, 0x80);
                frame.restore(c, 18, 0x70);
                frame.restore(c, 19, 0x60);
                frame.restore_ra(c, 0x50);
                frame.close(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003f8c38, 0x003f8ef8);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const INIT_ALLOC_CTRL: u32 = DATA + 0x150;
    const INIT_ALLOC_MAGIC: u32 = 0x51_1011_0c;
    static INIT_ALLOC_PREV: AtomicUsize = AtomicUsize::new(0);

    fn init_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(INIT_ALLOC_CTRL) != INIT_ALLOC_MAGIC {
            return fakes::fall_back(c, &INIT_ALLOC_PREV, ee::s07::f_00266728__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0026_6728, ret);
    }

    #[test]
    fn slogin_init_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            init_fake_alloc,
            &INIT_ALLOC_PREV,
        );
        check(
            ee::s14::f_003f7ee0__generated,
            slogin_init,
            30,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3f7e_e000, 1);
                c.pc = 0x003f_7ee0;
                c.bus.write32(INIT_ALLOC_CTRL, INIT_ALLOC_MAGIC);
                c.g[A0][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200, DATA + 0x300);
                c.bus.write32(DATA + 0x204, 0);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
                }
                c.bus.write32(GAME_GP + 0x2a30, DATA + 0x500); // store already up
                c.bus.write32(STORE_ERROR_TABLE + 4, 0x003f_81a4);
            },
        );
    }

    #[test]
    fn slogin_create_matches() {
        check(
            ee::s14::f_003f7f48__generated,
            slogin_create,
            60,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f7f_4800, 3);
                c.pc = 0x003f_7f48;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.bus.write32(0x0049_7508 + 0x0c, FAKE);
                c.bus.write32(0x0049_7508 + 0x10, FAKE);
                c.bus.write32(0x0049_7508 + 0x14, FAKE);
                let mode = seed % 3;
                c.g[A0][0] = if mode == 2 { sx(DATA + 0xa00) } else { 0 };
                c.g[A1][0] = if mode == 0 { 0 } else { sx(DATA + 0x800) };
                for (i, b) in b"acct\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + i as u32, *b);
                }
                for (i, b) in b"pass\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x811 + i as u32, *b);
                }
            },
        );
    }

    fn create_checksum_fake(c: &mut Cpu, target: u32) {
        let mode = c.bus.read32(DATA + 0x3e4);
        let output_slot = c.g[A0][0] as u32;
        fakes::fake_call(c, target);
        let status = match target {
            x if x == FAKE + 0x10 => (mode == 1) as u64,
            x if x == FAKE + 0x20 => 0,
            x if x == FAKE + 0x30 => 0,
            _ => return,
        };
        if target == FAKE + 0x30 {
            c.bus.write32(output_slot, DATA + 0x1600);
        }
        c.g[V0][0] = status;
    }

    #[test]
    fn slogin_create_checksum_success_matches() {
        check(
            ee::s14::f_003f7f48__generated,
            slogin_create,
            12,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f7f_48ee, 0);
                let mode = seed % 3;
                c.pc = 0x003f_7f48;
                c.hooks.miss = create_checksum_fake;
                c.bus.write32(DATA + 0x3e4, mode);
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.bus.write32(GAME_GP + 0x2a30, DATA + 0x1700);
                c.bus.write32(GAME_GP + CARD_IFACE_GP, DATA + 0x1800);
                c.bus.write32(0x0049_7508 + 0x0c, FAKE + 0x10);
                c.bus.write32(0x0049_7508 + 0x10, FAKE + 0x20);
                c.bus.write32(0x0049_7508 + 0x14, FAKE + 0x30);
                c.bus.write32(DATA + 0x1604, DATA + 0x1900);
                c.bus
                    .write32(DATA + 0x1900, if mode == 2 { 0 } else { 0x0836_c71d });
                c.g[A0][0] = sx(DATA + 0xa00);
                c.g[A1][0] = sx(DATA + 0x800);
                for (i, b) in b"acct\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + i as u32, *b);
                }
                for (i, b) in b"pass\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x811 + i as u32, *b);
                }
                for (i, b) in b"SSX3\0".iter().enumerate() {
                    c.bus.write8(0x0049_7390 + i as u32, *b);
                }
            },
        );
    }

    fn poll_fake(c: &mut Cpu, target: u32) {
        if target != FAKE || c.bus.read32(DATA + 0x100) == 2 {
            return fakes::fake_call(c, target);
        }
        let mode = c.bus.read32(DATA + 0x100);
        c.bus
            .write32(c.g[A0][0] as u32, if mode == 3 { 2 } else { 1 });
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn slogin_poll_matches() {
        check(
            ee::s14::f_003f80e0__generated,
            slogin_poll,
            60,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_e000, 1);
                let mode = seed % 5;
                c.hooks.miss = poll_fake;
                c.pc = 0x003f_80e0;
                c.bus.write32(DATA + 0x100, mode);
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus
                    .write32(DATA + 0x400 + 0xd0, if mode == 1 { 4 } else { 0 });
                c.bus.write32(DATA + 0x400 + 0xd4, FAKE);
                c.g[A0][0] = if mode == 0 { 0 } else { sx(DATA + 0x200) };
            },
        );
    }

    #[test]
    fn slogin_shutdown_matches() {
        check(
            ee::s14::f_003f7f18__generated,
            slogin_shutdown,
            30,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f7f_1800, 0);
                c.pc = 0x003f_7f18;
                c.bus.write32(GAME_GP + 0x2a30, 0); // no open store
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0x140, 0); // no allocated owner block
                c.bus.write32(STORE_ERROR_TABLE + 2 * 4, 0x003f_81dc);
            },
        );
    }

    #[test]
    fn slogin_load_matches() {
        check(
            ee::s14::f_003f8000__generated,
            slogin_load,
            48,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_0000, 0);
                c.pc = 0x003f_8000;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x800) };
            },
        );
    }

    #[test]
    fn slogin_delete_matches() {
        check(
            ee::s14::f_003f80c0__generated,
            slogin_delete,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_c000, 0);
                c.pc = 0x003f_80c0;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, seed % 4);
            },
        );
    }

    #[test]
    fn slogin_update_matches() {
        check(
            ee::s14::f_003f8048__generated,
            slogin_update,
            48,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_4800, 0);
                c.pc = 0x003f_8048;
                c.bus.write32(GAME_GP + LOGIN_STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x800) };
                for (i, b) in b"acct\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + i as u32, *b);
                }
                for (i, b) in b"pass\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x811 + i as u32, *b);
                }
            },
        );
    }

    #[test]
    fn slogin_map_store_error_matches() {
        const TARGETS: [u32; 12] = [
            0x003f_819c,
            0x003f_81a4,
            0x003f_81ac,
            0x003f_81b4,
            0x003f_81bc,
            0x003f_81c4,
            0x003f_81cc,
            0x003f_81d4,
            0x003f_81dc,
            0x003f_81e4,
            0x003f_81ec,
            0x003f_81f4,
        ];
        check(
            ee::s14::f_003f8178__generated,
            slogin_map_store_error,
            192,
            &|c, seed| {
                let mut r = Rng(0x3f81_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8178;
                let error = if seed % 16 == 15 {
                    0x73 + seed % 5
                } else {
                    seed % 0x73
                };
                c.g[A0][0] = error as u64;
                for i in 0..0x73 {
                    c.bus.write32(STORE_ERROR_TABLE + i * 4, 0x003f_81f4);
                }
                if error < 0x73 {
                    c.bus
                        .write32(STORE_ERROR_TABLE + error * 4, TARGETS[(seed % 12) as usize]);
                }
            },
        );
    }

    #[test]
    fn slogin_get_record_alloc_size_matches() {
        check(
            ee::s14::f_003f8230__generated,
            slogin_get_record_alloc_size,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3f82_3000, 0);
                c.pc = 0x003f_8230;
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { DATA as u64 };
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn slogin_mc_first_device_in_mask_matches() {
        check(
            ee::s14::f_003f8550__generated,
            slogin_mc_first_device_in_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x3f85_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8550;
                c.g[A0][0] = if seed % 9 == 0 { 0 } else { 1u64 << (seed % 8) };
            },
        );
    }

    #[test]
    fn slogin_mc_last_device_in_mask_matches() {
        check(
            ee::s14::f_003f8588__generated,
            slogin_mc_last_device_in_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x3f85_8800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8588;
                c.g[A0][0] = if seed % 9 == 0 { 0 } else { 1u64 << (seed % 8) };
            },
        );
    }

    #[test]
    fn slogin_mc_count_devices_in_mask_matches() {
        check(
            ee::s14::f_003f86b8__generated,
            slogin_mc_count_devices_in_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x3f86_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_86b8;
                c.g[A0][0] = ((seed * 0x31) & 0xff) as u64;
            },
        );
    }

    #[test]
    fn slogin_mc_next_device_in_mask_matches() {
        check(
            ee::s14::f_003f85c0__generated,
            slogin_mc_next_device_in_mask,
            240,
            &|c, seed| {
                let mut r = Rng(0x3f85_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_85c0;
                c.bus.write32(GAME_GP + CARD_STATE_GP, DATA + 0x400);
                let current = (seed % 10) as i16 - 1;
                c.bus.write16(DATA + 0x400 + 0x10, current as u16);
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => 0xff,
                    2 => 1 << (seed % 8),
                    _ => (seed * 17 & 0xff) as u64,
                };
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn slogin_mc_set_error_matches() {
        check(
            ee::s14::f_003f86f0__generated,
            slogin_mc_set_error,
            50,
            &|c, seed| {
                let mut r = Rng(0x3f86_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_86f0;
                c.g[A0][0] = sx(seed + 100);
                c.bus.write32(GAME_GP + CARD_STATE_GP, DATA);
                c.bus.write32(DATA, 10 + seed % 5);
                c.bus.write32(DATA + CARD_ERROR, r.next_u32());
            },
        );
    }

    #[test]
    fn slogin_mc_get_error_matches() {
        check(
            ee::s14::f_003f8710__generated,
            slogin_mc_get_error,
            40,
            &|c, seed| {
                let mut r = Rng(0x3f87_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8710;
                c.bus.write32(GAME_GP + CARD_STATE_GP, DATA);
                c.bus.write32(DATA + CARD_ERROR, r.next_u32());
            },
        );
    }

    fn abort_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32, callback_offset: u32) {
        let mut r = fakes::machine(c, seed, salt, 1);
        c.pc = entry;
        c.g[A1][0] = sx(DATA + 0x200);
        c.bus.write32(DATA + 0x200, r.next_u32());
        c.bus.write32(GAME_GP + CARD_STATE_GP, DATA + 0x400);
        c.bus.write32(GAME_GP + CARD_IFACE_GP, DATA + 0x500);
        c.bus.write32(DATA + 0x400, r.next_u32());
        c.bus.write32(DATA + 0x400 + CARD_FILE, DATA + 0x800);
        c.bus.write32(DATA + 0x500 + callback_offset, FAKE);
    }

    #[test]
    fn slogin_mc_abort_at_flush_matches() {
        fakes::covered(
            ee::s14::f_003f8720__generated,
            slogin_mc_abort_at_flush,
            60,
            &|c, seed| {
                abort_fixture(c, seed, 0x3f87_2000, 0x003f_8720, 0x38);
            },
        );
    }

    #[test]
    fn slogin_mc_abort_at_close_matches() {
        fakes::covered(
            ee::s14::f_003f8768__generated,
            slogin_mc_abort_at_close,
            60,
            &|c, seed| {
                abort_fixture(c, seed, 0x3f87_6800, 0x003f_8768, 0x1c);
            },
        );
    }

    #[test]
    fn slogin_mc_abort_at_finish_matches() {
        fakes::covered(
            ee::s14::f_003f87b0__generated,
            slogin_mc_abort_at_finish,
            60,
            &|c, seed| {
                abort_fixture(c, seed, 0x3f87_b000, 0x003f_87b0, 0x14);
            },
        );
    }

    #[test]
    fn slogin_setup_icon_sys_matches() {
        check(
            ee::s14::f_003f8258__generated,
            slogin_setup_icon_sys,
            24,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f82_5800, 0);
                c.pc = 0x003f_8258;
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x1000) };
                c.bus.write32(GAME_GP + 0x2a30, 0); // store not initialised
                c.bus.write32(STORE_ERROR_TABLE + 2 * 4, 0x003f_81ac);
                for (i, b) in b"SSX3\0".iter().enumerate() {
                    c.bus.write8(0x0049_7390 + i as u32, *b);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_open_step_matches() {
        check(
            ee::s14::f_003f8c38__generated,
            slogin_mc_open_step,
            80,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f8c_3800, 2);
                let mode = seed % 19;
                c.pc = 0x003f_8c38;
                c.bus.write32(GAME_GP + CARD_STATE_GP, DATA + 0x400);
                c.bus.write32(GAME_GP + CARD_IFACE_GP, DATA + 0x800);
                c.bus.write32(
                    DATA + 0x400,
                    match mode {
                        2 => 0,
                        16 => 6,
                        18 => 4,
                        _ => 3,
                    },
                );
                let phase = match mode {
                    3 => 0,
                    4 | 5 | 12 | 16 => 1,
                    6 | 13 | 14 => 2,
                    7 | 8 => 0x16,
                    9 | 10 | 11 | 17 => 0x1a,
                    _ => 0xff,
                };
                c.bus.write32(DATA + 0x404, phase);
                c.bus.write16(DATA + 0x410, 0);
                c.bus.write32(
                    DATA + 0x400 + CARD_ERROR,
                    if matches!(mode, 11 | 17) { 4 } else { 0 },
                );
                c.bus.write32(DATA + 0x400 + 0x6c, 0);
                c.bus
                    .write16(DATA + 0x800, if mode == 17 { 0 } else { 0xff });
                for off in [4u32, 0x10, 0x14, 0x18] {
                    c.bus.write32(DATA + 0x800 + off, FAKE);
                }
                for (i, b) in b"SSX3\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + 0x50 + i as u32, *b);
                }
                c.g[A0][0] = if matches!(mode, 4 | 5) {
                    3
                } else if matches!(mode, 6 | 9) {
                    4
                } else {
                    0
                };
                c.g[A1][0] = if mode == 0 { 0 } else { sx(DATA + 0x1000) };
                c.g[A2][0] = if mode == 1 { 0 } else { sx(DATA + 0x1100) };
                c.bus.write32(
                    DATA + 0x1100,
                    if mode == 13 {
                        1
                    } else if mode == 8 {
                        u32::MAX
                    } else {
                        0
                    },
                );
            },
        );
    }

    // Both sides re-enter at the retry head after an unwind, as the trampoline does.
    crate::difftest::events::driven!(
        driven_generated_mc_open_step,
        ee::s14::f_003f8c38__generated
    );
    crate::difftest::events::driven!(driven_slogin_mc_open_step, slogin_mc_open_step);

    #[test]
    fn slogin_mc_open_step_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            driven_generated_mc_open_step,
            driven_slogin_mc_open_step,
            60,
            &|c, seed| {
                // The card reports an error after open, so the state machine
                // advances to another device through resume head 0x3f8d4c.
                let mut r = fakes::machine(c, seed, 0x3f8c_38ee, 0);
                c.bus.write32(GAME_GP + CARD_STATE_GP, DATA + 0x400);
                c.bus.write32(GAME_GP + CARD_IFACE_GP, DATA + 0x800);
                c.bus.write32(DATA + 0x400, 3);
                c.bus.write32(DATA + 0x404, 0x1a);
                c.bus.write32(DATA + 0x400 + CARD_ERROR, 4);
                c.bus.write32(DATA + 0x400 + 0x6c, 0);
                c.bus.write16(DATA + 0x800, 0xff);
                c.bus.write32(DATA + 0x800 + 0x10, FAKE);
                let mode = schedule(c, &mut r, seed, 0x003f_8c38);
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1100);
                mode
            },
        );
    }
}
