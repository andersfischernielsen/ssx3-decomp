//! Staging module oc1024 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1024`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003f_a8f8, slogin_mc_create_step, "SLoginMc_CreateStep")];

const CREATE_STATE_GP: u32 = 0x1ae8;
const CREATE_IFACE_GP: u32 = 0x1aec;
const CREATE_PHASE: u32 = 4;
const CREATE_FILE_PATH: u32 = 0x14;
const CREATE_DIR_PATH: u32 = 0x34;
const CREATE_DEVICE: u32 = 0x10;
const CREATE_HANDLE: u32 = 0x0c;
const CREATE_DEVICE_MASK: u32 = 0x6c;
const CREATE_PATH_SUFFIX: u32 = 0x54;
const CREATE_CALLBACK_TABLE: u32 = 0x0049_7430;
const CREATE_VIEW_ICON: u32 = 0x0049_7410;
const CREATE_ICON_SYS: u32 = 0x0049_7420;

mod create_callee {
    use super::*;

    pub(super) const BUILD_DIR: Callee = Callee {
        addr: 0x003f_a7a0,
        f: ee::s14::SLoginMc_BuildDirPath,
    };
    pub(super) const BUILD_FILES: Callee = Callee {
        addr: 0x003f_a818,
        f: ee::s14::SLoginMc_BuildFilePaths,
    };
    pub(super) const NEXT_DEVICE: Callee = Callee {
        addr: 0x003f_85c0,
        f: ee::s14::SLoginMc_NextDeviceInMask,
    };
    pub(super) const SET_ERROR: Callee = Callee {
        addr: 0x003f_86f0,
        f: ee::s14::SLoginMc_SetError,
    };
    pub(super) const GET_ERROR: Callee = Callee {
        addr: 0x003f_8710,
        f: ee::s14::SLoginMc_GetError,
    };
    pub(super) const ABORT_FLUSH: Callee = Callee {
        addr: 0x003f_8720,
        f: ee::s14::SLoginMc_AbortAtFlush,
    };
    pub(super) const ABORT_CLOSE: Callee = Callee {
        addr: 0x003f_8768,
        f: ee::s14::SLoginMc_AbortAtClose,
    };
    pub(super) const ABORT_FINISH: Callee = Callee {
        addr: 0x003f_87b0,
        f: ee::s14::SLoginMc_AbortAtFinish,
    };
}

#[derive(Clone, Copy)]
enum CreateExit {
    Normal,
    InvalidPhase,
    InputError,
    BadResult,
    CloseCallback,
    FinishAbort,
    FlushAbort,
    CloseAbort,
    GetError,
    FinishDevice,
}

fn create_step_epilogue(c: &mut Cpu, frame: Frame, restore_s0: bool) {
    if restore_s0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
}

/// Advance the shared-login memory-card creation operation after a card RPC.
/// `a0` is the RPC result, `a1` receives completion status, and `a2` points
/// to the RPC's returned word. The state word at `gp+0x1ae8` selects the next
/// directory, capacity, file, icon, flush, close, or device-retry action.
pub fn slogin_mc_create_step(c: &mut Cpu) {
    guest(c, |c| {
        loop {
            c.cycles += 7;
            let frame = Frame::open(c, 0x80);
            c.g[A3][0] = c.g[A0][0];
            frame.save(c, S0, 0x70);
            frame.save(c, S1, 0x60);
            c.g[S0][0] = c.g[A1][0];
            frame.save_ra(c, 0x50);
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                create_step_epilogue(c, frame, true);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0x12;
            if c.g[A2][0] == 0 {
                create_step_epilogue(c, frame, true);
                return Ok(());
            }
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
            c.g[V1][0] = 1;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = 4;
            if c.g[A0][0] == c.g[V1][0] {
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                let valid = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 1;
                if !valid {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                    create_step_epilogue(c, frame, true);
                    return Ok(());
                }
            }

            c.cycles += 7;
            let store_at = c.g[S0][0] as u32;
            let store_value = c.g[V0][0];
            Addr(store_at).sw(c, store_value);
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(CREATE_PHASE)).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = (c.g[A0][0] < 26) as u64;
            c.g[V0][0] = 0x490000;
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[V1][0] = 2;
                c.g[V0][0] = 0x12;
                let store_at = c.g[S0][0] as u32;
                let store_value = c.g[V1][0];
                Addr(store_at).sw(c, store_value);
                create_step_epilogue(c, frame, true);
                return Ok(());
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CREATE_CALLBACK_TABLE & 0xffff));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let tail = match target {
                0x003f_a8f8 => {
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_a8f8);
                    }
                    continue;
                }
                // Start a directory lookup, or move to the next selected card.
                0x003f_a978 => {
                    c.cycles += 2;
                    c.g[V0][0] = 3;
                    if c.g[A3][0] == 0 {
                        c.cycles += 16;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(CREATE_PATH_SUFFIX));
                        create_callee::BUILD_DIR.call(c, 0x003f_a998)?;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                        c.g[V0][0] = 2;
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[SP][0];
                        let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_a9b8)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    } else {
                        c.cycles += 2;
                        let retry = c.g[A3][0] == c.g[V0][0];
                        c.g[V1][0] = 2;
                        if !retry {
                            CreateExit::InputError
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                            c.g[V0][0] = 1;
                            c.g[S1][0] = Addr(c.g[V1][0] as u32).lw(c);
                            if c.g[S1][0] != c.g[V0][0] {
                                c.g[V1][0] = 2;
                                c.cycles += 3;
                                c.g[V0][0] = 3;
                                let store_at = c.g[S0][0] as u32;
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                CreateExit::Normal
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                                c.g[A1][0] =
                                    Addr((c.g[V1][0] as u32).wrapping_add(CREATE_DEVICE_MASK))
                                        .lw(c);
                                c.g[A0][0] = Addr(c.g[V0][0] as u32).lhu(c);
                                create_callee::NEXT_DEVICE.call(c, 0x003f_a9ec)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[V0][0] = u64::MAX;
                                let exhausted = c.g[A0][0] == c.g[V0][0];
                                let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_DEVICE);
                                let store_value = c.g[A0][0];
                                Addr(store_at).sw(c, store_value);
                                if exhausted {
                                    c.cycles += 1;
                                    c.g[V0][0] = 2;
                                    CreateExit::GetError
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] =
                                        Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP))
                                            .lw(c);
                                    let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                                    let store_value = c.g[S1][0];
                                    Addr(store_at).sw(c, store_value);
                                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
                                    let callback = c.g[V1][0] as u32;
                                    call_ptr(c, callback, 0x003f_aa14)?;
                                    c.g[V0][0] = 0;
                                    CreateExit::Normal
                                }
                            }
                        }
                    }
                }
                // Check whether the directory exists.
                0x003f_aa28 => {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[V0][0] = 1;
                        if c.g[A1][0] == 0 {
                            c.cycles += 9;
                            c.g[V1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                            c.g[V0][0] = 3;
                            c.g[A1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                            let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                            let store_value = c.g[V0][0];
                            Addr(store_at).sw(c, store_value);
                            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
                            c.g[A0][0] =
                                Addr((c.g[V1][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                            let callback = c.g[V0][0] as u32;
                            call_ptr(c, callback, 0x003f_aa58)?;
                            c.g[V0][0] = 0;
                            CreateExit::Normal
                        } else {
                            c.cycles += 2;
                            c.g[V1][0] = 2;
                            if c.g[A1][0] != c.g[V0][0] {
                                CreateExit::BadResult
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = 8;
                                create_callee::SET_ERROR.call(c, 0x003f_aa70)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                CreateExit::CloseCallback
                            }
                        }
                    }
                }
                // Check available space for the directory.
                0x003f_aa78 => {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        if (c.g[A1][0] as i64) < 0 {
                            c.cycles += 1;
                            c.g[V1][0] = 2;
                            CreateExit::BadResult
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x420)).lw(c);
                            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                            let short = c.g[V0][0] != 0;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                            if short {
                                c.cycles += 4;
                                c.g[A0][0] = 14;
                                create_callee::SET_ERROR.call(c, 0x003f_aac0)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                CreateExit::CloseCallback
                            } else {
                                c.cycles += 7;
                                c.g[V1][0] = 4;
                                let store_at = (c.g[V0][0] as u32).wrapping_add(CREATE_PHASE);
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x0c)).lw(c);
                                c.g[A0][0] =
                                    Addr((c.g[V0][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                                let callback = c.g[V1][0] as u32;
                                call_ptr(c, callback, 0x003f_aab0)?;
                                c.g[V0][0] = 0;
                                CreateExit::Normal
                            }
                        }
                    }
                }
                // Check available space for the file, then construct paths.
                0x003f_aac8 => {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        if (c.g[A1][0] as i64) < 0 {
                            c.cycles += 1;
                            c.g[V1][0] = 2;
                            CreateExit::BadResult
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x424)).lw(c);
                            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                            let short = c.g[V0][0] != 0;
                            c.g[A1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                            if short {
                                c.cycles += 4;
                                c.g[A0][0] = 14;
                                create_callee::SET_ERROR.call(c, 0x003f_ab44)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                CreateExit::CloseCallback
                            } else {
                                c.cycles += 16;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x50));
                                c.g[A3][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(CREATE_PATH_SUFFIX));
                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(CREATE_FILE_PATH));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(CREATE_DIR_PATH));
                                create_callee::BUILD_FILES.call(c, 0x003f_ab10)?;
                                c.g[V0][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                c.g[V1][0] = 5;
                                c.g[A3][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                                let store_at = (c.g[V0][0] as u32).wrapping_add(CREATE_PHASE);
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(CREATE_FILE_PATH));
                                c.g[A2][0] =
                                    Addr((c.g[V0][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x20)).lw(c);
                                c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x420)).lw(c);
                                let callback = c.g[V0][0] as u32;
                                call_ptr(c, callback, 0x003f_ab34)?;
                                c.g[V0][0] = 0;
                                CreateExit::Normal
                            }
                        }
                    }
                }
                // Open the new data file.
                0x003f_ab4c => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = 14;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A3][0] = 0x203;
                        let store_at = (c.g[V0][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[V1][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(CREATE_FILE_PATH));
                        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
                        c.g[A1][0] = c.g[A0][0];
                        let callback = c.g[V1][0] as u32;
                        call_ptr(c, callback, 0x003f_ab78)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    }
                }
                // Read the file handle; a negative result aborts at flush.
                0x003f_ab80 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    if c.g[A3][0] != 0 {
                        CreateExit::FlushAbort
                    } else {
                        c.cycles += 3;
                        c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                        if (c.g[A2][0] as i64) < 0 {
                            c.cycles += 2;
                            c.g[A0][0] = 0;
                            CreateExit::FlushAbort
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = 16;
                            c.g[A1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                            c.g[A0][0] = c.g[A2][0];
                            let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                            let store_value = c.g[V0][0];
                            Addr(store_at).sw(c, store_value);
                            let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_HANDLE);
                            let store_value = c.g[A2][0];
                            Addr(store_at).sw(c, store_value);
                            c.cycles += 5;
                            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
                            let callback = c.g[V0][0] as u32;
                            call_ptr(c, callback, 0x003f_acd8)?;
                            c.g[V0][0] = 0;
                            CreateExit::Normal
                        }
                    }
                }
                // Close after the first file flush.
                0x003f_abb4 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::CloseAbort
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 17;
                        CreateExit::FinishDevice
                    }
                }
                // Open the icon file after closing the first one.
                0x003f_abc4 => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = 6;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A1][0] = 0x490000;
                        let store_at = (c.g[A0][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A1][0] =
                            sx((c.g[A1][0] as u32).wrapping_add(CREATE_VIEW_ICON & 0xffff));
                        CreateExit::FinishDevice
                    }
                }
                // Save the first icon handle and write its bytes.
                0x003f_abe4 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A2][0] = 7;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[A1][0];
                        let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[A2][0];
                        Addr(store_at).sw(c, store_value);
                        let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_HANDLE);
                        let store_value = c.g[A1][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x30)).lw(c);
                        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x50)).lw(c);
                        let callback = c.g[V1][0] as u32;
                        call_ptr(c, callback, 0x003f_ac14)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    }
                }
                // Flush the first icon file.
                0x003f_ac1c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    if c.g[A3][0] != 0 {
                        CreateExit::FlushAbort
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                        c.g[V0][0] = 8;
                        CreateExit::FinishDevice
                    }
                }
                // Close the first icon file.
                0x003f_ac30 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::CloseAbort
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 9;
                        CreateExit::FinishDevice
                    }
                }
                // Open the system icon file.
                0x003f_ac40 => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 10;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A1][0] = 0x490000;
                        let store_at = (c.g[A0][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(CREATE_ICON_SYS & 0xffff));
                        CreateExit::FinishDevice
                    }
                }
                // Save the system icon handle and write its bytes.
                0x003f_ac78 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A3][0] = 11;
                        c.g[A2][0] = 0x3c4;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[A1][0];
                        let store_at = (c.g[V0][0] as u32).wrapping_add(CREATE_HANDLE);
                        let store_value = c.g[A1][0];
                        Addr(store_at).sw(c, store_value);
                        let store_at = (c.g[V0][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[A3][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30)).lw(c);
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x58));
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_aca8)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    }
                }
                // Flush the system icon file.
                0x003f_acb0 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    if c.g[A3][0] != 0 {
                        CreateExit::FlushAbort
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                        c.g[V0][0] = 12;
                        CreateExit::FinishDevice
                    }
                }
                // Close the system icon file.
                0x003f_ad00 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::CloseAbort
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 13;
                        CreateExit::FinishDevice
                    }
                }
                // Finish the memory-card transaction.
                0x003f_ad48 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        CreateExit::CloseCallback
                    }
                }
                // Report completion or retry on another selected card.
                0x003f_ad90 => {
                    c.cycles += 2;
                    c.g[V1][0] = 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::InputError
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                        c.g[A0][0] = 4;
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        let mode_four = c.g[V1][0] == c.g[A0][0];
                        c.g[V0][0] = 2;
                        if mode_four {
                            CreateExit::GetError
                        } else {
                            c.cycles += 4;
                            create_callee::GET_ERROR.call(c, 0x003f_adb4)?;
                            let failed = c.g[V0][0] != 0;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                            if !failed {
                                c.cycles += 4;
                                c.g[V1][0] = 2;
                                c.g[V0][0] = 0;
                                let store_at = c.g[S0][0] as u32;
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                CreateExit::Normal
                            } else {
                                c.cycles += 9;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                c.g[A0][0] = Addr(c.g[V0][0] as u32).lhu(c);
                                c.g[A1][0] =
                                    Addr((c.g[V1][0] as u32).wrapping_add(CREATE_DEVICE_MASK))
                                        .lw(c);
                                create_callee::NEXT_DEVICE.call(c, 0x003f_addc)?;
                                c.g[A1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[V0][0] = u64::MAX;
                                let exhausted = c.g[A0][0] == c.g[V0][0];
                                let store_at = (c.g[A1][0] as u32).wrapping_add(CREATE_DEVICE);
                                let store_value = c.g[A0][0];
                                Addr(store_at).sw(c, store_value);
                                if exhausted {
                                    c.cycles += 1;
                                    c.g[V0][0] = 2;
                                    CreateExit::GetError
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] =
                                        Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP))
                                            .lw(c);
                                    c.g[V0][0] = 1;
                                    let store_at = (c.g[A1][0] as u32).wrapping_add(CREATE_PHASE);
                                    let store_value = c.g[V0][0];
                                    Addr(store_at).sw(c, store_value);
                                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lw(c);
                                    let callback = c.g[V0][0] as u32;
                                    call_ptr(c, callback, 0x003f_ae08)?;
                                    c.g[V0][0] = 0;
                                    CreateExit::Normal
                                }
                            }
                        }
                    }
                }
                0x003f_ae30 => CreateExit::InvalidPhase,
                _ => {
                    c.jump_out(target, 0x003f_a8f8, 0x003f_ae50);
                    return Ok(());
                }
            };

            let mut restore_s0 = true;
            match tail {
                CreateExit::Normal => {}
                CreateExit::InvalidPhase => {
                    c.cycles += 3;
                    c.g[V1][0] = 2;
                    c.g[V0][0] = 0x12;
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V1][0];
                    Addr(store_at).sw(c, store_value);
                }
                CreateExit::InputError => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A3][0];
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V1][0];
                    Addr(store_at).sw(c, store_value);
                }
                CreateExit::BadResult => {
                    c.cycles += 3;
                    c.g[V0][0] = 10;
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V1][0];
                    Addr(store_at).sw(c, store_value);
                }
                CreateExit::CloseCallback => {
                    c.cycles += 8;
                    c.g[V0][0] = 26;
                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                    let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                    let store_value = c.g[V0][0];
                    Addr(store_at).sw(c, store_value);
                    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                    let callback = c.g[V0][0] as u32;
                    call_ptr(c, callback, 0x003f_ad68)?;
                    c.g[V0][0] = 0;
                }
                CreateExit::FinishAbort => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A3][0];
                    create_callee::SET_ERROR.call(c, 0x003f_ad78)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                    create_callee::ABORT_FINISH.call(c, 0x003f_ad88)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::FlushAbort => {
                    c.cycles += 8;
                    create_callee::SET_ERROR.call(c, 0x003f_ace8)?;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CREATE_HANDLE));
                    create_callee::ABORT_FLUSH.call(c, 0x003f_acf8)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::CloseAbort => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A3][0];
                    create_callee::SET_ERROR.call(c, 0x003f_ad30)?;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CREATE_HANDLE));
                    create_callee::ABORT_CLOSE.call(c, 0x003f_ad40)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::GetError => {
                    c.cycles += 4;
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V0][0];
                    Addr(store_at).sw(c, store_value);
                    create_callee::GET_ERROR.call(c, 0x003f_ae1c)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::FinishDevice => {
                    // Three shared continuations: icon open, handle close, and flush.
                    if target == 0x003f_abc4 || target == 0x003f_ac40 {
                        c.cycles += 7;
                        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(CREATE_DEVICE)).lw(c);
                        c.g[A3][0] = 0x203;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CREATE_FILE_PATH));
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_ac70)?;
                    } else if target == 0x003f_abb4
                        || target == 0x003f_ac30
                        || target == 0x003f_ad00
                    {
                        c.cycles += 7;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1c)).lw(c);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(CREATE_HANDLE)).lw(c);
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_ad20)?;
                    } else {
                        // The flush callback takes the saved file handle.
                        c.cycles += 3;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP)).lw(c);
                        let store_at = (c.g[V1][0] as u32).wrapping_add(CREATE_PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(CREATE_HANDLE)).lw(c);
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_acd8)?;
                    }
                    c.g[V0][0] = 0;
                }
            }
            create_step_epilogue(c, frame, restore_s0);
            return Ok(());
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::DATA;

    const CREATE_TEST_STATE: u32 = DATA + 0x2000;
    const CREATE_TEST_IFACE: u32 = DATA + 0x3000;
    const CREATE_TEST_STATUS: u32 = DATA + 0x1000;
    const CREATE_TEST_RESULT: u32 = DATA + 0x1004;

    fn create_step_fixture(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x003f_a8f8, 2);
        c.pc = 0x003f_a8f8;
        c.g[A0][0] = 0;
        c.g[A1][0] = sx(CREATE_TEST_STATUS);
        c.g[A2][0] = sx(CREATE_TEST_RESULT);
        c.bus.write32(
            (c.g[GP][0] as u32).wrapping_add(CREATE_STATE_GP),
            CREATE_TEST_STATE,
        );
        c.bus.write32(
            (c.g[GP][0] as u32).wrapping_add(CREATE_IFACE_GP),
            CREATE_TEST_IFACE,
        );
        c.bus.write32(CREATE_TEST_STATE, 1);
        c.bus.write32(CREATE_TEST_STATE + 8, 0);
        c.bus.write32(CREATE_TEST_STATE + CREATE_DEVICE, 0);
        c.bus.write32(CREATE_TEST_STATE + CREATE_HANDLE, 0x42);
        c.bus.write32(CREATE_TEST_STATE + CREATE_DEVICE_MASK, 0);
        c.bus.write16(CREATE_TEST_IFACE, 3);
        c.bus.write32(CREATE_TEST_IFACE + 0x420, 64);
        c.bus.write32(CREATE_TEST_IFACE + 0x424, 128);
        for (i, offset) in [4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x30, 0x38]
            .into_iter()
            .enumerate()
        {
            c.bus
                .write32(CREATE_TEST_IFACE + offset, FAKE + 0x10 * i as u32);
        }
        for (i, byte) in b"CARD\0".iter().enumerate() {
            c.bus.write8(CREATE_TEST_IFACE + 0x50 + i as u32, *byte);
        }
        for (i, byte) in b"SSX3\0".iter().enumerate() {
            c.bus
                .write8(CREATE_TEST_STATE + CREATE_PATH_SUFFIX + i as u32, *byte);
        }
        let dispatch = [
            (1, 0x003f_a978),
            (2, 0x003f_aa28),
            (3, 0x003f_aa78),
            (4, 0x003f_aac8),
            (5, 0x003f_ab4c),
            (6, 0x003f_abe4),
            (7, 0x003f_ac1c),
            (8, 0x003f_ac30),
            (9, 0x003f_ac40),
            (10, 0x003f_ac78),
            (11, 0x003f_acb0),
            (12, 0x003f_ad00),
            (13, 0x003f_ad48),
            (14, 0x003f_ab80),
            (16, 0x003f_abb4),
            (17, 0x003f_abc4),
            (26, 0x003f_ad90),
        ];
        for index in 0..26 {
            c.bus
                .write32(CREATE_CALLBACK_TABLE + 4 * index, 0x003f_ae30);
        }
        for (phase, address) in dispatch {
            c.bus
                .write32(CREATE_CALLBACK_TABLE + 4 * (phase - 1), address);
        }

        // Each residue is one state and one side of its result predicates.
        let scenario = seed % 62;
        let (phase, result, data) = match scenario {
            0 => {
                c.g[A1][0] = 0;
                (1, 0, 0)
            }
            1 => {
                c.g[A2][0] = 0;
                (1, 0, 0)
            }
            2 => {
                c.bus.write32(CREATE_TEST_STATE, 2);
                (1, 0, 0)
            }
            3 => (0, 0, 0),
            4 => (1, 0, 0),
            5 => (1, 2, 0),
            6 => {
                c.bus.write32(CREATE_TEST_STATE, 4);
                (1, 3, 0)
            }
            7 => {
                c.bus.write16(CREATE_TEST_IFACE, 0);
                (1, 3, 0)
            }
            8 => (1, 3, 0),
            9 => (2, 0, 0),
            10 => (2, 0, 1),
            11 => (2, 0, 2),
            12 => (2, 2, 0),
            13 => (3, 0, u32::MAX),
            14 => (3, 0, 32),
            15 => (3, 0, 96),
            16 => (3, 2, 0),
            17 => (4, 0, u32::MAX),
            18 => (4, 0, 64),
            19 => (4, 0, 256),
            20 => (4, 2, 0),
            21 => (5, 0, 0),
            22 => (5, 2, 0),
            23 => (14, 0, 12),
            24 => (14, 0, u32::MAX),
            25 => (14, 2, 0),
            26 => (16, 0, 0),
            27 => (16, 2, 0),
            28 => (17, 0, 0),
            29 => (17, 2, 0),
            30 => (6, 0, 12),
            31 => (6, 2, 0),
            32 => (7, 0, 0),
            33 => (7, 2, 0),
            34 => (8, 0, 0),
            35 => (8, 2, 0),
            36 => (9, 0, 0),
            37 => (9, 2, 0),
            38 => (10, 0, 12),
            39 => (10, 2, 0),
            40 => (11, 0, 0),
            41 => (11, 2, 0),
            42 => (12, 0, 0),
            43 => (12, 2, 0),
            44 => (13, 0, 0),
            45 => (13, 2, 0),
            46 => (26, 2, 0),
            47 => {
                c.bus.write32(CREATE_TEST_STATE, 4);
                (26, 0, 0)
            }
            48 => (26, 0, 0),
            49 => {
                c.bus.write32(CREATE_TEST_STATE + 8, 7);
                c.bus.write16(CREATE_TEST_IFACE, 0);
                (26, 0, 0)
            }
            50 => {
                c.bus.write32(CREATE_TEST_STATE + 8, 7);
                (26, 0, 0)
            }
            51 => (15, 0, 0),
            52 => {
                c.bus.write32(CREATE_CALLBACK_TABLE, FAKE);
                (1, 0, 0)
            }
            53 => (27, 0, 0),
            54..=61 => (18 + scenario - 54, 0, 0),
            _ => unreachable!(),
        };
        c.g[A0][0] = result;
        c.bus.write32(CREATE_TEST_STATE + CREATE_PHASE, phase);
        c.bus.write32(CREATE_TEST_RESULT, data);
    }

    #[test]
    fn slogin_mc_create_step_matches() {
        check(
            ee::s14::f_003fa8f8__generated,
            slogin_mc_create_step,
            124,
            &create_step_fixture,
        );
    }
}
