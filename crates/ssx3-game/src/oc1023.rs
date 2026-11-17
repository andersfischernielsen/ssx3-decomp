//! Staging module oc1023 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1023`.

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
    (0x003f_8ef8, slogin_mc_io_step, "SLoginMc_IoStep"),
    (0x003f_8ff0, slogin_mc_close_step, "SLoginMc_CloseStep"),
    (0x003f_9130, slogin_mc_abort_step, "SLoginMc_AbortStep"),
    (
        0x003f_9208,
        slogin_mc_is_initialized,
        "SLoginMc_IsInitialized",
    ),
    (0x003f_9218, slogin_mc_init, "SLoginMc_Init"),
    (0x003f_9530, slogin_mc_shutdown, "SLoginMc_Shutdown"),
    (
        0x003f_95e8,
        slogin_mc_get_file_state,
        "SLoginMc_GetFileState",
    ),
    (0x003f_9698, slogin_mc_start_delete, "SLoginMc_StartDelete"),
    (0x003f_9818, slogin_mc_start_create, "SLoginMc_StartCreate"),
    (0x003f_99b8, slogin_mc_start_open, "SLoginMc_StartOpen"),
    (0x003f_9b38, slogin_mc_start_close, "SLoginMc_StartClose"),
    (0x003f_9c48, slogin_mc_start_seek, "SLoginMc_StartSeek"),
    (0x003f_9dc0, slogin_mc_start_read, "SLoginMc_StartRead"),
    (0x003f_9f00, slogin_mc_start_write, "SLoginMc_StartWrite"),
    (0x003f_a040, slogin_mc_poll, "SLoginMc_Poll"),
    (0x003f_a1d0, slogin_mc_get_port_slot, "SLoginMc_GetPortSlot"),
    (0x003f_a240, slogin_mc_set_icon_sys, "SLoginMc_SetIconSys"),
    (
        0x003f_a360,
        slogin_mc_is_available_wrapper,
        "SLoginMc_IsAvailable_Wrapper",
    ),
    (0x003f_a380, slogin_mc_ascii_to_sjis, "SLoginMc_AsciiToSjis"),
    (
        0x003f_a6a8,
        slogin_mc_calc_save_size,
        "SLoginMc_CalcSaveSize",
    ),
];

const MC_IFACE_GP: u32 = 0x1aec;
const MC_STATE_GP: u32 = 0x1ae8;

/// Install a memory-card save icon and calculate the space its assets require.
pub fn slogin_mc_set_icon_sys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let valid_icon = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        if !valid_icon {
            c.cycles += 2;
            c.g[V0][0] = 12;
        } else {
            c.cycles += 4;
            callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_a264)?;
            let initialized = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
            if !initialized {
                c.cycles += 2;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = (c.g[S0][0] | c.g[V1][0]) & 7;
                let aligned = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c0));
                if aligned {
                    loop {
                        c.cycles += 11;
                        c.g[A1][0] = c.ld(c.g[A0][0] as u32);
                        c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(8));
                        c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(16));
                        c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(24));
                        c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(8), c.g[A2][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(24), c.g[T0][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                        let again = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003f_a2e4);
                        }
                    }
                } else {
                    loop {
                        c.cycles += 19;
                        c.ldl(5, (c.g[A0][0] as u32).wrapping_add(7));
                        c.ldr(5, c.g[A0][0] as u32);
                        c.ldl(6, (c.g[A0][0] as u32).wrapping_add(15));
                        c.ldr(6, (c.g[A0][0] as u32).wrapping_add(8));
                        c.ldl(7, (c.g[A0][0] as u32).wrapping_add(23));
                        c.ldr(7, (c.g[A0][0] as u32).wrapping_add(16));
                        c.ldl(8, (c.g[A0][0] as u32).wrapping_add(31));
                        c.ldr(8, (c.g[A0][0] as u32).wrapping_add(24));
                        c.sdl(5, (c.g[V1][0] as u32).wrapping_add(7));
                        c.sdr(5, c.g[V1][0] as u32);
                        c.sdl(6, (c.g[V1][0] as u32).wrapping_add(15));
                        c.sdr(6, (c.g[V1][0] as u32).wrapping_add(8));
                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(23));
                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(16));
                        c.sdl(8, (c.g[V1][0] as u32).wrapping_add(31));
                        c.sdr(8, (c.g[V1][0] as u32).wrapping_add(24));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                        let again = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003f_a28c);
                        }
                    }
                    c.cycles += 2;
                }
                c.cycles += 15;
                c.ldl(2, (c.g[A0][0] as u32).wrapping_add(7));
                c.ldr(2, c.g[A0][0] as u32);
                c.ldl(5, (c.g[A0][0] as u32).wrapping_add(15));
                c.ldr(5, (c.g[A0][0] as u32).wrapping_add(8));
                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(7));
                c.sdr(2, c.g[V1][0] as u32);
                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(15));
                c.sdr(5, (c.g[V1][0] as u32).wrapping_add(8));
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x424));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x420));
                callee::SLOGIN_MC_CALC_SAVE_SIZE.call(c, 0x003f_a344)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003f_b308, 0x003f_a34c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Install the card interface, build its device mask, and allocate login card state.
pub fn slogin_mc_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9248)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        let mut valid = !initialized;
        let mut special_epilogue = false;
        if valid {
            c.cycles += 2;
            valid = c.g[S3][0] != 0;
            c.g[V0][0] = 12;
        }
        if valid {
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
            c.g[S4][0] = u64::MAX;
            valid = c.g[V0][0] != c.g[S4][0];
            c.g[V0][0] = 12;
        }
        if valid {
            c.cycles += 2;
            valid = c.g[S0][0] != 0;
            if !valid {
                frame.restore(c, S0, 0x50);
                special_epilogue = true;
            }
        }
        if valid {
            for slot in 0..16u32 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(slot * 4));
                valid = c.g[V0][0] != 0;
                c.g[V0][0] = 12;
                if !valid {
                    break;
                }
            }
        }
        if valid {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            valid = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
            if !valid {
                c.cycles += 2;
                c.g[V0][0] = 12;
            }
        }
        if valid {
            c.cycles += 2;
            valid = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
        }
        if valid {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
            valid = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
        }
        if valid {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x42c;
            c.g[A2][0] = 4;
            call_ptr(c, 0x0026_6728, 0x003f_9360)?;
            let allocated_iface = c.g[V0][0] != 0;
            c.sw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP), c.g[V0][0]);
            if allocated_iface {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0x74;
                c.g[A2][0] = 4;
                call_ptr(c, 0x0026_6728, 0x003f_9378)?;
                valid = c.g[V0][0] != 0;
                c.sw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP), c.g[V0][0]);
            } else {
                valid = false;
            }
            if !valid {
                c.cycles += 2;
                c.g[V0][0] = 7;
            }
        }
        if valid {
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
            c.g[A0][0] = c.g[S3][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x428), c.g[S2][0]);
            c.sh(c.g[V1][0] as u32, 0);
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            let terminated = c.g[V0][0] == c.g[S4][0];
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
            if !terminated {
                c.cycles += 5;
                c.g[T3][0] = c.g[V1][0];
                c.g[T4][0] = 9;
                c.g[T7][0] = 8;
                c.g[T6][0] = 1;
                c.g[T5][0] = u64::MAX;
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[V1][0] < 8) as u64;
                    let in_range = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[V1][0];
                    if !in_range {
                        c.cycles += 2;
                        let all_devices = c.g[A1][0] == c.g[T4][0];
                        c.g[V0][0] = 9;
                        if !all_devices {
                            valid = false;
                            break;
                        }
                    }
                    c.cycles += 2;
                    let list_full = c.g[S1][0] == c.g[T7][0];
                    c.g[V0][0] = 12;
                    if list_full {
                        valid = false;
                        break;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[T4][0] {
                        c.cycles += 8;
                        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[A0][0] = 0;
                        c.g[T2][0] = 1;
                        c.g[A2][0] = 0x100_0000;
                        c.g[T1][0] = 0x100_0000;
                        loop {
                            c.cycles += 9;
                            c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                            c.g[V1][0] = sx((c.g[T2][0] as u32) << ((c.g[A0][0] as u32) & 31));
                            c.g[A0][0] = c.g[A2][0];
                            c.g[V0][0] |= c.g[V1][0];
                            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                            c.sh(c.g[A1][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 8) as u64;
                            let again = c.g[V0][0] != 0;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T1][0] as u32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9408);
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[T0][0];
                    } else {
                        c.cycles += 7;
                        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                        c.g[V0][0] = c.lhu(c.g[T3][0] as u32);
                        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[V1][0] = sx((c.g[T6][0] as u32) << ((c.g[V1][0] as u32) & 31));
                        c.g[V0][0] |= c.g[V1][0];
                        c.sh(c.g[T3][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[T0][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    let again = c.g[V0][0] != c.g[T5][0];
                    c.g[S1][0] = c.g[A3][0] & 0xff;
                    if !again {
                        c.cycles += 1;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_93c0);
                    }
                }
            }
            if valid {
                c.cycles += 44;
                c.g[A3][0] = 0x45_0000;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A2][0] = 0x45_0000;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1468));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x14a0));
                c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
                c.g[V0][0] = 0;
                for i in 1..17u32 {
                    if i & 1 == 1 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4 * i));
                        c.sw((c.g[A1][0] as u32).wrapping_add(4 + 4 * i), c.g[V1][0]);
                    } else {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4 * i));
                        c.sw((c.g[A1][0] as u32).wrapping_add(4 + 4 * i), c.g[A0][0]);
                    }
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x48), c.g[A3][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                c.sw(c.g[V1][0] as u32, 0);
            }
        }
        if !special_epilogue {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Queue creation of a named memory-card file after validating its title and card.
pub fn slogin_mc_start_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_983c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_name = c.g[S2][0] != 0;
            c.g[V0][0] = 12;
            if valid_name {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0041_6810, 0x003f_9854)?;
                let valid_length = c.g[V0][0] < 0x18;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if !valid_length {
                    c.cycles += 2;
                    c.g[V0][0] = 12;
                } else {
                    c.cycles += 3;
                    let valid_index = c.g[V0][0] < 9;
                    c.g[V0][0] = u64::MAX;
                    if !valid_index {
                        c.cycles += 2;
                        c.g[V0][0] = 9;
                    } else {
                        c.cycles += 2;
                        let auto_device = c.g[S0][0] == c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                        let mut allowed = true;
                        if auto_device {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = 1;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S0][0] as u32) & 31));
                            c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                            c.g[V0][0] &= c.g[V1][0];
                            allowed = c.g[V0][0] != 0;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                        }
                        if !allowed {
                            c.cycles += 2;
                            c.g[V0][0] = 11;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            let busy = c.g[V0][0] != 0;
                            c.g[V0][0] = 13;
                            if !busy {
                                c.cycles += 7;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                                c.g[A1][0] = 0;
                                c.g[A2][0] = 0x60;
                                call_ptr(c, 0x0041_6210, 0x003f_98bc)?;
                                c.g[V1][0] = u64::MAX;
                                let explicit_device = c.g[S0][0] != c.g[V1][0];
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                let mut start = true;
                                if explicit_device {
                                    c.cycles += 3;
                                    c.g[V0][0] = 4;
                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                } else {
                                    c.cycles += 6;
                                    c.g[V0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                    c.g[S0][0] = 1;
                                    c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                    callee::SLOGIN_MC_COUNT_DEVICES_IN_MASK.call(c, 0x003f_98d8)?;
                                    let single = c.g[V0][0] == c.g[S0][0];
                                    c.g[V0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                    if single {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                        callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                            .call(c, 0x003f_98e8)?;
                                        c.g[A0][0] =
                                            c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                        c.g[V1][0] = 4;
                                        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S1][0] == 0 {
                                            c.cycles += 6;
                                            c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                            callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                                .call(c, 0x003f_990c)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                            c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[V0][0],
                                            );
                                        } else {
                                            c.cycles += 2;
                                            let last = c.g[S1][0] == c.g[S0][0];
                                            c.g[V0][0] = 12;
                                            if !last {
                                                start = false;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] = c
                                                    .lw((c.g[GP][0] as u32)
                                                        .wrapping_add(MC_IFACE_GP));
                                                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                                callee::SLOGIN_MC_LAST_DEVICE_IN_MASK
                                                    .call(c, 0x003f_9930)?;
                                                c.g[V1][0] = c
                                                    .lw((c.g[GP][0] as u32)
                                                        .wrapping_add(MC_STATE_GP));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[V0][0],
                                                );
                                                c.sw(c.g[V1][0] as u32, c.g[S1][0]);
                                            }
                                        }
                                    }
                                }
                                if start {
                                    c.cycles += 19;
                                    c.g[A0][0] = 0;
                                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_995c)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54));
                                    call_ptr(c, 0x0041_66f4, 0x003f_996c)?;
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.g[A0][0] = 1;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                                    call_ptr(c, 0x003f_b2f8, 0x003f_997c)?;
                                    c.g[V0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                    let target = c.g[A1][0] as u32;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                    call_ptr(c, target, 0x003f_9990)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.g[V1][0] = 1;
                                    c.g[V0][0] = 0;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V1][0]);
                                }
                            }
                        }
                    }
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

/// Queue deletion of a named memory-card file on a selected card.
pub fn slogin_mc_start_delete(c: &mut Cpu) {
    slogin_mc_start_named(c, false);
}

/// Queue opening a named memory-card file on a selected card.
pub fn slogin_mc_start_open(c: &mut Cpu) {
    slogin_mc_start_named(c, true);
}

fn slogin_mc_start_named(c: &mut Cpu, open: bool) {
    guest(c, |c| {
        let shift = if open { 0x320 } else { 0 };
        let phase_single = if open { 6 } else { 5 };
        let phase_search = if open { 3 } else { 2 };
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_96bc + shift)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_name = c.g[S2][0] != 0;
            c.g[V0][0] = 12;
            if valid_name {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let valid_index = c.g[V0][0] < 9;
                c.g[V0][0] = u64::MAX;
                if !valid_index {
                    c.cycles += 2;
                    c.g[V0][0] = 9;
                } else {
                    c.cycles += 2;
                    let auto_device = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                    let mut allowed = true;
                    if auto_device {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = 1;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S0][0] as u32) & 31));
                        c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                        c.g[V0][0] &= c.g[V1][0];
                        allowed = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    }
                    if !allowed {
                        c.cycles += 2;
                        c.g[V0][0] = 11;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let busy = c.g[V0][0] != 0;
                        c.g[V0][0] = 13;
                        if !busy {
                            c.cycles += 7;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = 0;
                            c.g[A2][0] = 0x60;
                            call_ptr(c, 0x0041_6210, 0x003f_972c + shift)?;
                            c.g[V1][0] = u64::MAX;
                            let explicit_device = c.g[S0][0] != c.g[V1][0];
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                            let mut start = true;
                            if explicit_device {
                                c.cycles += 3;
                                c.g[V0][0] = phase_single;
                                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                callee::SLOGIN_MC_COUNT_DEVICES_IN_MASK
                                    .call(c, 0x003f_9744 + shift)?;
                                c.g[V1][0] = 1;
                                let single = c.g[V0][0] == 1;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                if single {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                    callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                        .call(c, 0x003f_9758 + shift)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.g[V1][0] = phase_single;
                                } else {
                                    c.cycles += 2;
                                    if c.g[S1][0] == 0 {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                        callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                            .call(c, 0x003f_9774 + shift)?;
                                        c.g[A0][0] =
                                            c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    } else {
                                        c.cycles += 2;
                                        let last = c.g[S1][0] == c.g[V1][0];
                                        c.g[V0][0] = 12;
                                        if !last {
                                            start = false;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                            c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                            callee::SLOGIN_MC_LAST_DEVICE_IN_MASK
                                                .call(c, 0x003f_9790 + shift)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                        }
                                    }
                                    if start {
                                        c.cycles += 1;
                                        c.g[V1][0] = phase_search;
                                    }
                                }
                                if start {
                                    c.cycles += 3;
                                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                }
                            }
                            if start {
                                c.cycles += 19;
                                c.g[A0][0] = 0;
                                callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_97b8 + shift)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54));
                                call_ptr(c, 0x0041_66f4, 0x003f_97c8 + shift)?;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                c.g[A0][0] = 1;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                                call_ptr(c, 0x003f_b2f8, 0x003f_97d8 + shift)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                let target = c.g[A1][0] as u32;
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                call_ptr(c, target, 0x003f_97ec + shift)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                c.g[V1][0] = 1;
                                c.g[V0][0] = 0;
                                c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V1][0]);
                            }
                        }
                    }
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

/// Queue a seek on the open card file, applying the save-data offset for data seeks.
pub fn slogin_mc_start_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9c6c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_file = c.g[S0][0] != 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
            if !valid_file {
                c.cycles += 2;
                c.g[V0][0] = 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 13;
                if !busy {
                    c.cycles += 13;
                    c.g[V1][0] = 8;
                    c.g[V0][0] = 0x1b;
                    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
                    c.g[A0][0] = 0;
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_9ca8)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[A0][0] | c.g[V1][0]) & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    if aligned {
                        loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[A0][0] as u32);
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(8));
                            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(16));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(24));
                            c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(8), c.g[A2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(24), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9d18);
                            }
                        }
                    } else {
                        loop {
                            c.cycles += 19;
                            c.ldl(5, (c.g[A0][0] as u32).wrapping_add(7));
                            c.ldr(5, c.g[A0][0] as u32);
                            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(15));
                            c.ldr(6, (c.g[A0][0] as u32).wrapping_add(8));
                            c.ldl(7, (c.g[A0][0] as u32).wrapping_add(23));
                            c.ldr(7, (c.g[A0][0] as u32).wrapping_add(16));
                            c.ldl(8, (c.g[A0][0] as u32).wrapping_add(31));
                            c.ldr(8, (c.g[A0][0] as u32).wrapping_add(24));
                            c.sdl(5, (c.g[V1][0] as u32).wrapping_add(7));
                            c.sdr(5, c.g[V1][0] as u32);
                            c.sdl(6, (c.g[V1][0] as u32).wrapping_add(15));
                            c.sdr(6, (c.g[V1][0] as u32).wrapping_add(8));
                            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(23));
                            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(16));
                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(31));
                            c.sdr(8, (c.g[V1][0] as u32).wrapping_add(24));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9cc4);
                            }
                        }
                        c.cycles += 2;
                    }
                    c.cycles += 2;
                    let relative_to_data = c.g[S1][0] == 0;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    if relative_to_data {
                        c.cycles += 14;
                        callee::SLOGIN_MC_GET_DATA_OFFSET.call(c, 0x003f_9d54)?;
                        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                        callee::SLOGIN_MC_GET_DATA_OFFSET.call(c, 0x003f_9d68)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        let target = c.g[V1][0] as u32;
                        c.g[A2][0] = 0;
                        call_ptr(c, target, 0x003f_9d7c)?;
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                        c.g[A2][0] = c.g[S1][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[S2][0]);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x34));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, target, 0x003f_9da0)?;
                        c.g[V0][0] = 0;
                    }
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

/// Queue closure of the selected memory-card file and save its file record.
pub fn slogin_mc_start_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9b4c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_file = c.g[S0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
            if !valid_file {
                c.cycles += 2;
                c.g[V0][0] = 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 13;
                if !busy {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[S0][0] | c.g[A0][0]) & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[V1][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                    if aligned {
                        loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(8));
                            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(16));
                            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(24));
                            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(8), c.g[A2][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(16), c.g[A3][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(24), c.g[T0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            let again = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9be0);
                            }
                        }
                        c.cycles += 1;
                    } else {
                        loop {
                            c.cycles += 19;
                            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(7));
                            c.ldr(5, c.g[V1][0] as u32);
                            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(15));
                            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(8));
                            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(23));
                            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(16));
                            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(31));
                            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(24));
                            c.sdl(5, (c.g[A0][0] as u32).wrapping_add(7));
                            c.sdr(5, c.g[A0][0] as u32);
                            c.sdl(6, (c.g[A0][0] as u32).wrapping_add(15));
                            c.sdr(6, (c.g[A0][0] as u32).wrapping_add(8));
                            c.sdl(7, (c.g[A0][0] as u32).wrapping_add(23));
                            c.sdr(7, (c.g[A0][0] as u32).wrapping_add(16));
                            c.sdl(8, (c.g[A0][0] as u32).wrapping_add(31));
                            c.sdr(8, (c.g[A0][0] as u32).wrapping_add(24));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            let again = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9b88);
                            }
                        }
                        c.cycles += 2;
                    }
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.cycles += 9;
                    c.g[A1][0] = 7;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                    c.g[V0][0] = 0x18;
                    c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    call_ptr(c, target, 0x003f_9c30)?;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Poll the card interface and dispatch completed shared-login card phases.
pub fn slogin_mc_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_a064)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
        if !initialized {
            c.cycles += 2;
            c.g[V0][0] = 2;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 13;
            } else {
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    c.g[V0][0] = 12;
                } else {
                    c.cycles += 2;
                    let valid_result = c.g[S2][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if !valid_result {
                        c.cycles += 2;
                        c.g[V0][0] = 12;
                    } else {
                        c.cycles += 5;
                        let target = c.g[V0][0] as u32;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, target, 0x003f_a0a8)?;
                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                        let active_status = c.g[V1][0] != 0;
                        c.g[S0][0] = c.g[V0][0];
                        let mut dispatch = active_status;
                        if !active_status {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let busy = c.g[V1][0] != 0;
                            c.g[V0][0] = 15;
                            dispatch = !busy;
                        }
                        if dispatch {
                            c.cycles += 4;
                            c.g[S3][0] = c.lw(c.g[S1][0] as u32);
                            c.g[V0][0] = 2;
                            let completed = c.g[S3][0] == c.g[V0][0];
                            c.g[V0][0] = c.g[S0][0];
                            if completed {
                                c.cycles += 15;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                c.g[A1][0] = 2;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                let target = c.g[V0][0] as u32;
                                c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                call_ptr(c, target, 0x003f_a100)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V0][0] = 10;
                                if c.g[S0][0] == 10 {
                                    c.cycles += 4;
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                                    c.sw(c.g[V1][0] as u32, 0);
                                } else {
                                    c.g[V0][0] = 16;
                                    c.cycles += 2;
                                    let still_active = c.g[S0][0] == 16;
                                    c.g[A3][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                    if !still_active {
                                        c.cycles += 13;
                                        c.g[V0][0] =
                                            c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                        let target = c.g[V0][0] as u32;
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, target, 0x003f_a150)?;
                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                        let unchanged_status = c.g[A0][0] == c.g[S3][0];
                                        c.g[S0][0] = c.g[V0][0];
                                        if unchanged_status {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                            c.g[V1][0] = 5;
                                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                            let stage = c.g[A2][0];
                                            let clear_busy = if stage == 5 {
                                                true
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] = 7;
                                                if stage == c.g[A0][0] {
                                                    true
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[V0][0] = 11;
                                                    if stage == 7 {
                                                        true
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[V0][0] = 12;
                                                        if stage == 11 {
                                                            true
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[V0][0] = 13;
                                                            if stage == 12 {
                                                                true
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(MC_STATE_GP));
                                                                stage == 13
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            if clear_busy {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0;
                                                call_ptr(c, 0x003f_b2f8, 0x003f_a1a0)?;
                                                c.g[V0][0] = c
                                                    .lw((c.g[GP][0] as u32)
                                                        .wrapping_add(MC_STATE_GP));
                                            }
                                            c.cycles += 2;
                                            c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
                                            c.sw(c.g[V0][0] as u32, 0);
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S0][0];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Convert a card icon's ASCII title into the two-byte Shift-JIS save header.
pub fn slogin_mc_ascii_to_sjis(c: &mut Cpu) {
    c.cycles += 30;
    c.g[A2][0] = c.g[A0][0];
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
    c.g[V1][0] = 0x49_0000;
    c.g[T1][0] = c.g[A1][0];
    c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x73b0));
    c.ldl(4, (c.g[T2][0] as u32).wrapping_add(7));
    c.ldr(4, c.g[T2][0] as u32);
    c.ldl(7, (c.g[T2][0] as u32).wrapping_add(15));
    c.ldr(7, (c.g[T2][0] as u32).wrapping_add(8));
    c.ldl(8, (c.g[T2][0] as u32).wrapping_add(23));
    c.ldr(8, (c.g[T2][0] as u32).wrapping_add(16));
    c.sdl(4, sp(c).wrapping_add(7));
    c.sdr(4, sp(c));
    c.sdl(7, sp(c).wrapping_add(15));
    c.sdr(7, sp(c).wrapping_add(8));
    c.sdl(8, sp(c).wrapping_add(23));
    c.sdr(8, sp(c).wrapping_add(16));
    c.ldl(4, (c.g[T2][0] as u32).wrapping_add(31));
    c.ldr(4, (c.g[T2][0] as u32).wrapping_add(24));
    c.g[A3][0] = c.lb((c.g[T2][0] as u32).wrapping_add(32));
    c.sdl(4, sp(c).wrapping_add(31));
    c.sdr(4, sp(c).wrapping_add(24));
    c.sb(sp(c).wrapping_add(32), c.g[A3][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
    c.g[A1][0] = 0;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
    c.g[A0][0] = c.g[V0][0] & 0xffff;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A3][0] = 0x8100;
        c.g[T2][0] = 0x8220;
        c.g[T0][0] = 0x821f;
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x81));
            let double_byte = c.g[V0][0] < 0x18;
            c.g[V0][0] = double_byte as u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 8);
            if double_byte {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = c.g[V0][0] & 0xffff;
            } else {
                c.cycles += 3;
                let below_z = c.g[A0][0] < 0x7b;
                c.g[V0][0] = (c.g[A0][0] < 0x61) as u64;
                if !below_z {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x5e));
                    c.cycles += 1;
                    c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                } else {
                    c.cycles += 2;
                    let below_a = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[A0][0] < 0x5b) as u64;
                    if !below_a {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                        c.g[A0][0] = c.g[V0][0] & 0xffff;
                    } else {
                        c.cycles += 2;
                        let below_bracket = c.g[V0][0] != 0;
                        c.g[V0][0] = (c.g[A0][0] < 0x41) as u64;
                        if !below_bracket {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x44));
                            c.cycles += 1;
                            c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                        } else {
                            c.cycles += 2;
                            let below_upper = c.g[V0][0] != 0;
                            c.g[V0][0] = (c.g[A0][0] < 0x3a) as u64;
                            if !below_upper {
                                c.cycles += 3;
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.g[A0][0] = c.g[V0][0] & 0xffff;
                            } else {
                                c.cycles += 2;
                                let below_colon = c.g[V0][0] != 0;
                                c.g[V0][0] = (c.g[A0][0] < 0x30) as u64;
                                if !below_colon {
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x2a));
                                    c.cycles += 1;
                                    c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                                } else {
                                    c.cycles += 2;
                                    let below_digit = c.g[V0][0] != 0;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                                    if below_digit {
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x20));
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                        c.g[A0][0] = c.g[V0][0] & 0xffff;
                                    }
                                    if below_digit {
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 15;
            c.g[V0][0] = c.g[A0][0] & 0xff;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
            c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
            c.g[A0][0] |= c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.sh(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A1][0] &= 0xff;
            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[A0][0] = c.g[V0][0] & 0xffff;
            let again = c.g[A0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_a408);
            }
        }
    }
    c.cycles += 2;
    frame.close(c);
}

/// Begin reading from the open memory-card file into the caller's buffer.
pub fn slogin_mc_start_read(c: &mut Cpu) {
    slogin_mc_start_transfer(c, false);
}

/// Begin writing the caller's buffer into the open memory-card file.
pub fn slogin_mc_start_write(c: &mut Cpu) {
    slogin_mc_start_transfer(c, true);
}

fn slogin_mc_start_transfer(c: &mut Cpu, write: bool) {
    guest(c, |c| {
        let initialized_ret = if write { 0x003f_9f24 } else { 0x003f_9de4 };
        let error_ret = if write { 0x003f_9f60 } else { 0x003f_9e20 };
        let method_ret = if write { 0x003f_a020 } else { 0x003f_9ee0 };
        let unaligned_head = if write { 0x003f_9f7c } else { 0x003f_9e3c };
        let aligned_head = if write { 0x003f_9fd4 } else { 0x003f_9e94 };
        let phase = if write { 9 } else { 10 };
        let subphase = if write { 0x1c } else { 0x1d };
        let method_offset = if write { 0x30 } else { 0x2c };
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, initialized_ret)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let has_file = c.g[S0][0] != 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
            if !has_file {
                c.cycles += 2;
                c.g[V0][0] = 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 13;
                if !busy {
                    c.cycles += 12;
                    c.g[V1][0] = phase;
                    c.g[V0][0] = subphase;
                    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
                    c.g[A0][0] = 0;
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    callee::SLOGIN_MC_SET_ERROR.call(c, error_ret)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[S0][0] | c.g[V1][0]) & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                    if aligned {
                        loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[A0][0] as u32);
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(8));
                            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(16));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(24));
                            c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(8), c.g[A2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(24), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, aligned_head);
                            }
                        }
                        c.cycles += 1;
                    } else {
                        loop {
                            c.cycles += 19;
                            c.ldl(5, (c.g[A0][0] as u32).wrapping_add(7));
                            c.ldr(5, c.g[A0][0] as u32);
                            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(15));
                            c.ldr(6, (c.g[A0][0] as u32).wrapping_add(8));
                            c.ldl(7, (c.g[A0][0] as u32).wrapping_add(23));
                            c.ldr(7, (c.g[A0][0] as u32).wrapping_add(16));
                            c.ldl(8, (c.g[A0][0] as u32).wrapping_add(31));
                            c.ldr(8, (c.g[A0][0] as u32).wrapping_add(24));
                            c.sdl(5, (c.g[V1][0] as u32).wrapping_add(7));
                            c.sdr(5, c.g[V1][0] as u32);
                            c.sdl(6, (c.g[V1][0] as u32).wrapping_add(15));
                            c.sdr(6, (c.g[V1][0] as u32).wrapping_add(8));
                            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(23));
                            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(16));
                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(31));
                            c.sdr(8, (c.g[V1][0] as u32).wrapping_add(24));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, unaligned_head);
                            }
                        }
                        c.cycles += 2;
                    }
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                    c.g[A2][0] = c.g[S1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(method_offset));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    call_ptr(c, target, method_ret)?;
                    c.g[V0][0] = 0;
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

/// Advance memory-card close, flush, or finish callbacks after an I/O operation.
pub fn slogin_mc_close_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let valid_status = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        let mut direct_epilogue = false;
        if !valid_status {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
        } else {
            c.cycles += 2;
            let valid_data = c.g[A2][0] != 0;
            c.g[V0][0] = 0x12;
            if valid_data {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                c.g[A0][0] = 7;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let valid_phase = c.g[V1][0] == c.g[A0][0];
                c.g[V0][0] = 1;
                if !valid_phase {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x19;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                    let stage = c.g[V1][0];
                    c.g[V0][0] = (stage < 0x1a) as u64;
                    let mut unknown_stage = false;
                    if stage == 0x19 {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1a;
                        if c.g[A3][0] == 0 {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                            c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                            call_ptr(c, target, 0x003f_90d0)?;
                            c.g[V0][0] = 0;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[A3][0];
                            callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_90e0)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                            callee::SLOGIN_MC_ABORT_AT_FINISH.call(c, 0x003f_90f0)?;
                            frame.restore(c, S0, 0x10);
                            direct_epilogue = true;
                        }
                    } else {
                        c.cycles += 2;
                        let at_or_past_finish = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x18;
                        if !at_or_past_finish {
                            c.cycles += 2;
                            let at_close = stage == 0x18;
                            c.g[V1][0] = 2;
                            if at_close {
                                c.cycles += 2;
                                if c.g[A3][0] == 0 {
                                    c.cycles += 7;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[A1][0]);
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                    let target = c.g[V0][0] as u32;
                                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                    call_ptr(c, target, 0x003f_908c)?;
                                    c.g[V0][0] = 0;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[A3][0];
                                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_909c)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                                    callee::SLOGIN_MC_ABORT_AT_CLOSE.call(c, 0x003f_90ac)?;
                                    frame.restore(c, S0, 0x10);
                                    direct_epilogue = true;
                                }
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1a;
                            let at_finish = stage == 0x1a;
                            c.g[V1][0] = 2;
                            if at_finish {
                                c.cycles += 2;
                                let has_error = c.g[A3][0] != 0;
                                c.g[V0][0] = c.g[A3][0];
                                if has_error {
                                    c.cycles += 2;
                                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = 2;
                                    c.g[V0][0] = 0;
                                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                                }
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        }
                    }
                    if unknown_stage {
                        // An unknown stage marks the request finished (status word = 2).
                        c.cycles += 1;
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    }
                }
            }
        }
        if !direct_epilogue {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Complete a memory-card read or write step, checking the card phase and transfer count.
pub fn slogin_mc_io_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let valid_status = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        let mut restored_s0 = false;
        if !valid_status {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
        } else {
            c.cycles += 2;
            let valid_data = c.g[A2][0] != 0;
            c.g[V0][0] = 0x12;
            if valid_data {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                c.g[V1][0] = 8;
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                let phase = c.g[A0][0];
                c.g[V0][0] = 10;
                let valid_phase = if phase == 8 {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    true
                } else {
                    c.cycles += 2;
                    let is_ten = phase == 10;
                    c.g[V0][0] = 9;
                    if is_ten {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                        true
                    } else {
                        c.cycles += 2;
                        let is_nine = phase == 9;
                        c.g[V0][0] = 1;
                        is_nine
                    }
                };
                if !valid_phase {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = 0x1b;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                    let stage = c.g[V1][0];
                    c.g[V0][0] = (stage < 0x1b) as u64;
                    let mut has_error = false;
                    let mut unknown_stage = false;
                    if stage == 0x1b {
                        c.cycles += 2;
                        has_error = c.g[A3][0] != 0;
                        c.g[V1][0] = 2;
                        if !has_error {
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                        }
                    } else {
                        c.cycles += 2;
                        if stage < 0x1b {
                            c.g[V1][0] = 2;
                            unknown_stage = true;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (stage < 0x1e) as u64;
                            c.g[V1][0] = 2;
                            if stage >= 0x1e {
                                unknown_stage = true;
                            } else {
                                c.cycles += 4;
                                has_error = c.g[A3][0] != 0;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                                if !has_error {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                                    let matching_count = c.g[V1][0] == c.g[A0][0];
                                    c.g[V1][0] = 2;
                                    c.cycles += 2;
                                    c.g[V0][0] = if matching_count { 0 } else { 10 };
                                }
                            }
                        }
                    }
                    if has_error {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[A3][0];
                        callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_8fbc)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                        callee::SLOGIN_MC_ABORT_AT_FLUSH.call(c, 0x003f_8fcc)?;
                        frame.restore(c, S0, 0x10);
                        restored_s0 = true;
                    } else {
                        if unknown_stage {
                            c.cycles += 1;
                            c.g[V0][0] = 0x12;
                        }
                        // The step is done either way: status word = 2.
                        c.cycles += 1;
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    }
                }
            }
        }
        if !restored_s0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Advance a memory-card abort after a close or flush callback completes.
pub fn slogin_mc_abort_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let mut direct_epilogue = false;
        let valid_status = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if !valid_status {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
        } else {
            c.cycles += 2;
            let valid_data = c.g[A2][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
            if !valid_data {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(11));
                let valid_phase = c.g[V0][0] < 3;
                c.g[V0][0] = 1;
                if !valid_phase {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                } else {
                    c.cycles += 6;
                    c.g[A2][0] = 0x19;
                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                    let stage = c.g[V1][0];
                    c.g[V0][0] = 0x1a;
                    let mut unknown_stage = false;
                    if stage == 0x19 {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                        c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                        call_ptr(c, target, 0x003f_91d8)?;
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 3;
                        let below_finish = stage < 0x1a;
                        c.g[V0][0] = 0x18;
                        if below_finish {
                            c.cycles += 2;
                            c.g[V1][0] = 2;
                            if stage == 0x18 {
                                c.cycles += 7;
                                c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[A2][0]);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                let target = c.g[V0][0] as u32;
                                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                call_ptr(c, target, 0x003f_91bc)?;
                                c.g[V0][0] = 0;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1a;
                            c.g[V1][0] = 2;
                            if stage == 0x1a {
                                c.cycles += 5;
                                c.g[V0][0] = 2;
                                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_91ec)?;
                                direct_epilogue = true;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        }
                    }
                    if unknown_stage {
                        // An unknown stage marks the request finished (status word = 2).
                        c.cycles += 1;
                        c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    }
                }
            }
        }
        if !direct_epilogue {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_IS_INITIALIZED: Callee = Callee {
        addr: 0x003f_9208,
        f: ee::s14::SLoginMc_IsInitialized,
    };
    pub(crate) const SLOGIN_MC_GET_ERROR: Callee = Callee {
        addr: 0x003f_8710,
        f: ee::s14::SLoginMc_GetError,
    };
    pub(crate) const SLOGIN_MC_SET_ERROR: Callee = Callee {
        addr: 0x003f_86f0,
        f: ee::s14::SLoginMc_SetError,
    };
    pub(crate) const SLOGIN_MC_ABORT_AT_FLUSH: Callee = Callee {
        addr: 0x003f_8720,
        f: ee::s14::SLoginMc_AbortAtFlush,
    };
    pub(crate) const SLOGIN_MC_ABORT_AT_CLOSE: Callee = Callee {
        addr: 0x003f_8768,
        f: ee::s14::SLoginMc_AbortAtClose,
    };
    pub(crate) const SLOGIN_MC_ABORT_AT_FINISH: Callee = Callee {
        addr: 0x003f_87b0,
        f: ee::s14::SLoginMc_AbortAtFinish,
    };
    pub(crate) const SLOGIN_MC_GET_DATA_OFFSET: Callee = Callee {
        addr: 0x003f_a6e8,
        f: ee::s14::SLoginMc_GetDataOffset,
    };
    pub(crate) const SLOGIN_MC_CALC_SAVE_SIZE: Callee = Callee {
        addr: 0x003f_a6a8,
        f: ee::s14::SLoginMc_CalcSaveSize,
    };
    pub(crate) const SLOGIN_MC_COUNT_DEVICES_IN_MASK: Callee = Callee {
        addr: 0x003f_86b8,
        f: ee::s14::SLoginMc_CountDevicesInMask,
    };
    pub(crate) const SLOGIN_MC_FIRST_DEVICE_IN_MASK: Callee = Callee {
        addr: 0x003f_8550,
        f: ee::s14::SLoginMc_FirstDeviceInMask,
    };
    pub(crate) const SLOGIN_MC_LAST_DEVICE_IN_MASK: Callee = Callee {
        addr: 0x003f_8588,
        f: ee::s14::SLoginMc_LastDeviceInMask,
    };
    pub(crate) const SLOGIN_MC_SPLIT_DEVICE_INDEX: Callee = Callee {
        addr: 0x003f_a708,
        f: ee::s14::SLoginMc_SplitDeviceIndex,
    };
}

/// Report whether the shared-login memory-card interface has been installed.
pub fn slogin_mc_is_initialized(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    ret(c);
}

/// Shut down the shared-login memory-card state and release its two allocations.
pub fn slogin_mc_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9540)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
        if initialized {
            c.cycles += 2;
            let iface = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
            if iface {
                c.cycles += 2;
                if c.g[V1][0] != 0 {
                    c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
                    c.cycles += 31;
                    c.g[A1][0] = c.g[V1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(8), 0);
                    c.g[A2][0] = 0x74;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
                    c.g[A3][0] = 4;
                    for off in (0x10..=0x44).step_by(4) {
                        c.sw((c.g[V0][0] as u32).wrapping_add(off), 0);
                    }
                    c.sw(c.g[V1][0] as u32, 0);
                    c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x428));
                    call_ptr(c, 0x0026_6760, 0x003f_95bc)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP));
                    c.g[A2][0] = 0x42c;
                    c.sw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP), 0);
                    c.g[A3][0] = 4;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                    call_ptr(c, 0x0026_6760, 0x003f_95d4)?;
                    c.sw((c.g[GP][0] as u32).wrapping_add(MC_IFACE_GP), 0);
                    c.g[V0][0] = 0;
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy the memory-card file state into the caller's output record.
pub fn slogin_mc_get_file_state(c: &mut Cpu) {
    c.cycles += 2;
    let valid_output = c.g[A0][0] != 0;
    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(MC_STATE_GP));
    if !valid_output {
        c.cycles += 2;
        c.g[V0][0] = 12;
        ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = (c.g[V1][0] | c.g[A0][0]) & 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c));
    if aligned {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(8));
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(16));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(24));
            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(8), c.g[A2][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(16), c.g[A3][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(24), c.g[T0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
            let again = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
            if !again {
                c.cycles += 3;
                c.g[V0][0] = 0;
                ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_9660);
            }
        }
    }
    loop {
        c.cycles += 19;
        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(7));
        c.ldr(5, c.g[V1][0] as u32);
        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(15));
        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(8));
        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(23));
        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(16));
        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(31));
        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(24));
        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(7));
        c.sdr(5, c.g[A0][0] as u32);
        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(15));
        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(8));
        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(23));
        c.sdr(7, (c.g[A0][0] as u32).wrapping_add(16));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(31));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(24));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
        let again = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
        if !again {
            c.cycles += 2;
            c.g[V0][0] = 0;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003f_960c);
        }
    }
}

/// Forward the memory-card availability query to the card interface.
pub fn slogin_mc_is_available_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003f_b278, 0x003f_a370)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Compute the memory-card space needed for a shared-login save.
pub fn slogin_mc_calc_save_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003f_b280, 0x003f_a6b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Resolve a shared-login memory-card device index to its port and slot pair.
pub fn slogin_mc_get_port_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 12;
        } else {
            c.cycles += 4;
            callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_a1fc)?;
            let initialized = c.g[V0][0] != 0;
            c.g[V0][0] = (c.g[S0][0] < 8) as u64;
            if !initialized {
                c.cycles += 2;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                let valid_device = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if valid_device {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    callee::SLOGIN_MC_SPLIT_DEVICE_INDEX.call(c, 0x003f_a21c)?;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 9;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const MC_TEST_CTRL: u32 = DATA + 0x3d0;
    const MC_TEST_MAGIC: u32 = 0x4d43_534c;
    static PREV_MC_FREE: AtomicUsize = AtomicUsize::new(0);
    static PREV_MC_ALLOC: AtomicUsize = AtomicUsize::new(0);
    const MC_INIT_MAGIC: u32 = 0x4d43_494e;

    fn mc_free_fake(c: &mut Cpu) {
        if c.bus.read32(MC_TEST_CTRL) != MC_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_MC_FREE, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, 0x0026_6760);
    }

    fn mc_init_alloc_fake(c: &mut Cpu) {
        if c.bus.read32(MC_TEST_CTRL) != MC_INIT_MAGIC {
            return fakes::fall_back(c, &PREV_MC_ALLOC, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, 0x0026_6728);
        let nth = c.bus.read32(fakes::LOG_COUNT) - 1;
        c.g[V0][0] = if nth == c.bus.read32(MC_TEST_CTRL + 4) {
            0
        } else if nth == 0 {
            (DATA + 0x8000) as u64
        } else {
            (DATA + 0x9000) as u64
        };
    }

    fn mc_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x003f_a000 ^ entry as u64 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
    }

    #[test]
    fn slogin_mc_is_initialized_matches() {
        check(
            ee::s14::f_003f9208__generated,
            slogin_mc_is_initialized,
            60,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_9208);
                c.bus
                    .write32(GAME_GP + MC_IFACE_GP, if seed % 2 == 0 { 0 } else { DATA });
            },
        );
    }

    #[test]
    fn slogin_mc_init_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            mc_init_alloc_fake,
            &PREV_MC_ALLOC,
        );
        fakes::covered(
            ee::s14::f_003f9218__generated,
            slogin_mc_init,
            272,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9218, 2);
                c.pc = 0x003f_9218;
                let class = seed % 16;
                c.bus.write32(MC_TEST_CTRL, MC_INIT_MAGIC);
                c.bus.write32(
                    MC_TEST_CTRL + 4,
                    match class {
                        6 => 0,
                        7 => 1,
                        _ => u32::MAX,
                    },
                );
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 16 % 3 == 0 {
                        u32::MAX
                    } else {
                        seed / 16 % 2
                    },
                );
                c.bus.write32(
                    GAME_GP + MC_IFACE_GP,
                    if class == 0 { DATA + 0x8000 } else { 0 },
                );
                c.bus.write32(
                    GAME_GP + MC_STATE_GP,
                    if class == 5 { DATA + 0x9000 } else { 0 },
                );
                c.g[A0][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x6000) as u64
                };
                c.g[A1][0] = if class == 3 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = (DATA + 0xa000) as u64;
                for i in 0..17u32 {
                    c.bus.write32(
                        DATA + 0x7000 + 4 * i,
                        if class == 4 && i == seed / 16 % 17 {
                            0
                        } else {
                            fakes::FAKE + 0x10 * i
                        },
                    );
                }
                for i in 0..0x42cu32 {
                    c.bus.write8(DATA + 0x8000 + i, 0);
                }
                for i in 0..0x74u32 {
                    c.bus.write8(DATA + 0x9000 + i, 0);
                }
                let words: &[u32] = match class {
                    2 => &[u32::MAX],
                    10 => &[9, u32::MAX],
                    11 => &[8, u32::MAX],
                    12 => &[0, 1, 2, 3, 4, 5, 6, 7, 0, u32::MAX],
                    13 => &[0, 9, u32::MAX],
                    14 => &[9, 0, u32::MAX],
                    15 => &[0, 7, u32::MAX],
                    _ => &[0, u32::MAX],
                };
                for (i, value) in words.iter().enumerate() {
                    c.bus.write32(DATA + 0x6000 + 4 * i as u32, *value);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_init_events() {
        use crate::difftest::events::{check_events, schedule};
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            mc_init_alloc_fake,
            &PREV_MC_ALLOC,
        );
        check_events(
            ee::s14::f_003f9218__generated,
            slogin_mc_init,
            90,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9218, 2);
                c.pc = 0x003f_9218;
                c.bus.write32(MC_TEST_CTRL, MC_INIT_MAGIC);
                c.bus.write32(MC_TEST_CTRL + 4, u32::MAX);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                c.bus.write32(GAME_GP + MC_IFACE_GP, 0);
                c.bus.write32(GAME_GP + MC_STATE_GP, 0);
                c.g[A0][0] = (DATA + 0x6000) as u64;
                c.g[A1][0] = (DATA + 0x7000) as u64;
                c.g[A2][0] = (DATA + 0xa000) as u64;
                for i in 0..17u32 {
                    c.bus.write32(DATA + 0x7000 + 4 * i, fakes::FAKE + 0x10 * i);
                }
                for i in 0..0x42cu32 {
                    c.bus.write8(DATA + 0x8000 + i, 0);
                }
                for i in 0..0x74u32 {
                    c.bus.write8(DATA + 0x9000 + i, 0);
                }
                let list: &[u32] = if seed % 2 == 0 {
                    &[9, u32::MAX]
                } else {
                    &[0, 1, u32::MAX]
                };
                for (i, value) in list.iter().enumerate() {
                    c.bus.write32(DATA + 0x6000 + 4 * i as u32, *value);
                }
                let mut r = Rng(0x003f_9218 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_9218)
            },
        );
    }

    #[test]
    fn slogin_mc_abort_step_matches() {
        fakes::covered(
            ee::s14::f_003f9130__generated,
            slogin_mc_abort_step,
            128,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9130, 1);
                c.pc = 0x003f_9130;
                let class = seed % 8;
                c.g[A0][0] = (DATA + 0x6000) as u64;
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x8000) as u64
                };
                c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6000);
                c.bus.write32(GAME_GP + MC_IFACE_GP, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x14, fakes::FAKE);
                c.bus.write32(DATA + 0x5000 + 0x1c, fakes::FAKE + 0x10);
                c.bus.write32(
                    DATA + 0x6000,
                    if class == 2 { 0 } else { 11 + seed / 8 % 3 },
                );
                c.bus.write32(
                    DATA + 0x6000 + 4,
                    match class {
                        3 => 0x18,
                        4 => 0x19,
                        5 => 0x1a,
                        6 => 0x17,
                        _ => 0x1b,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 8, seed ^ 0x1234);
                c.bus.write32(DATA + 0x6000 + 0xc, DATA + 0x9000);
                c.bus.write32(DATA + 0x6000 + 0x10, DATA + 0x9100);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 8 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    #[test]
    fn slogin_mc_io_step_matches() {
        fakes::covered(
            ee::s14::f_003f8ef8__generated,
            slogin_mc_io_step,
            160,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_8ef8, 1);
                c.pc = 0x003f_8ef8;
                let class = seed % 10;
                c.g[A0][0] = if class == 5 || class == 8 { 0x44 } else { 0 };
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x8000) as u64
                };
                c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6000);
                c.bus.write32(GAME_GP + MC_IFACE_GP, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x38, fakes::FAKE);
                c.bus.write32(
                    DATA + 0x6000,
                    if class == 2 { 7 } else { 8 + seed / 10 % 3 },
                );
                c.bus.write32(
                    DATA + 0x6000 + 4,
                    match class {
                        3 => 0x1a,
                        4 | 5 => 0x1b,
                        6 | 7 => 0x1c,
                        8 => 0x1d,
                        _ => 0x1e,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 0xc, DATA + 0x9100);
                c.bus.write32(
                    DATA + 0x6000 + 0x70,
                    if class == 7 { 0x1235 } else { 0x1234 },
                );
                c.bus.write32(DATA + 0x8000, 0x1234);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 10 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    #[test]
    fn slogin_mc_close_step_matches() {
        fakes::covered(
            ee::s14::f_003f8ff0__generated,
            slogin_mc_close_step,
            160,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_8ff0, 1);
                c.pc = 0x003f_8ff0;
                let class = seed % 10;
                c.g[A0][0] = if class == 4 || class == 6 || class == 8 {
                    0x33
                } else {
                    0
                };
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x8000) as u64
                };
                c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6000);
                c.bus.write32(GAME_GP + MC_IFACE_GP, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x14, fakes::FAKE);
                c.bus.write32(DATA + 0x5000 + 0x1c, fakes::FAKE + 0x10);
                c.bus.write32(DATA + 0x6000, if class == 2 { 0 } else { 7 });
                c.bus.write32(
                    DATA + 0x6000 + 4,
                    match class {
                        3 | 4 => 0x18,
                        5 | 6 => 0x19,
                        7 | 8 => 0x1a,
                        9 if seed / 20 % 2 == 1 => 0x1b,
                        _ => 0x17,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 0xc, DATA + 0x9100);
                c.bus.write32(DATA + 0x6000 + 0x10, DATA + 0x9200);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 10 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    fn mc_transfer_fixture(c: &mut Cpu, seed: u32, write: bool) {
        let entry = if write { 0x003f_9f00 } else { 0x003f_9dc0 };
        let _r = fakes::machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let class = seed % 5;
        c.bus.write32(
            GAME_GP + MC_IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6004);
        c.bus.write32(DATA + 0x6004, if class == 2 { 1 } else { 0 });
        c.bus
            .write32(DATA + 0x5000 + if write { 0x30 } else { 0x2c }, fakes::FAKE);
        c.g[A0][0] = match class {
            1 => 0,
            4 => (DATA + 0x7001) as u64,
            _ => (DATA + 0x7000) as u64,
        };
        c.g[A1][0] = (DATA + 0x8000) as u64;
        c.g[A2][0] = (seed ^ 0x1234) as u64;
        for i in 0..0x61u32 {
            c.bus.write8(DATA + 0x7000 + i, (i ^ seed) as u8);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 5 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_read_matches() {
        fakes::covered(
            ee::s14::f_003f9dc0__generated,
            slogin_mc_start_read,
            100,
            &|c, seed| mc_transfer_fixture(c, seed, false),
        );
    }

    #[test]
    fn slogin_mc_start_write_matches() {
        fakes::covered(
            ee::s14::f_003f9f00__generated,
            slogin_mc_start_write,
            100,
            &|c, seed| mc_transfer_fixture(c, seed, true),
        );
    }

    #[test]
    fn slogin_mc_start_transfer_events() {
        use crate::difftest::events::{check_events, schedule};
        for (generated, hand, entry, write) in [
            (
                ee::s14::f_003f9dc0__generated as GuestFn,
                slogin_mc_start_read as GuestFn,
                0x003f_9dc0,
                false,
            ),
            (
                ee::s14::f_003f9f00__generated as GuestFn,
                slogin_mc_start_write as GuestFn,
                0x003f_9f00,
                true,
            ),
        ] {
            check_events(generated, hand, 60, &|c, seed| {
                mc_transfer_fixture(c, seed * 5 + 3 + seed % 2, write);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(entry as u64 ^ seed as u64);
                schedule(c, &mut r, seed, entry)
            });
        }
    }

    fn mc_sjis_fixture(c: &mut Cpu, seed: u32) {
        mc_fixture(c, seed, 0x003f_a380);
        c.g[A0][0] = (DATA + 0x7000) as u64;
        c.g[A1][0] = (DATA + 0x8000) as u64;
        for i in 0..33u32 {
            c.bus.write8(0x0049_73b0 + i, (i ^ seed) as u8);
        }
        const INPUTS: [u8; 10] = [0, 0x20, 0x30, 0x3a, 0x41, 0x5b, 0x61, 0x7b, 0x81, 0x98];
        let ch = INPUTS[(seed % 10) as usize];
        c.bus.write8(DATA + 0x7000, ch);
        c.bus
            .write8(DATA + 0x7001, if ch >= 0x81 { 0x40 } else { 0 });
        c.bus.write8(DATA + 0x7002, 0);
    }

    #[test]
    fn slogin_mc_ascii_to_sjis_matches() {
        check(
            ee::s14::f_003fa380__generated,
            slogin_mc_ascii_to_sjis,
            100,
            &|c, seed| mc_sjis_fixture(c, seed),
        );
    }

    #[test]
    fn slogin_mc_ascii_to_sjis_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003fa380__generated,
            slogin_mc_ascii_to_sjis,
            90,
            &|c, seed| {
                mc_sjis_fixture(c, seed * 10 + 8);
                let mut r = Rng(0x003f_a380 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_a380)
            },
        );
    }

    const MC_POLL_CTRL: u32 = DATA + 0x3e0;
    const MC_POLL_MAGIC: u32 = 0x504f_4c4c;

    fn mc_poll_fake(c: &mut Cpu, target: u32) {
        if c.bus.read32(MC_POLL_CTRL) == MC_POLL_MAGIC && c.bus.read32(fakes::LOG_COUNT) == 0 {
            let status = c.bus.read32(MC_POLL_CTRL + 4);
            c.bus.write32(c.g[A0][0] as u32, status);
        }
        fakes::fake_call(c, target);
    }

    fn mc_poll_jump_fake(c: &mut Cpu, target: u32, _start: u32, _end: u32) {
        mc_poll_fake(c, target);
    }

    #[test]
    fn slogin_mc_poll_matches() {
        fakes::covered(
            ee::s14::f_003fa040__generated,
            slogin_mc_poll,
            156,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_a040, 3);
                c.pc = 0x003f_a040;
                c.hooks.miss = mc_poll_fake;
                c.hooks.jump_out = mc_poll_jump_fake;
                let class = seed % 13;
                c.bus.write32(MC_POLL_CTRL, MC_POLL_MAGIC);
                c.bus.write32(
                    MC_POLL_CTRL + 4,
                    match class {
                        4 | 5 => 0,
                        6 => 1,
                        _ => 2,
                    },
                );
                c.bus.write32(
                    GAME_GP + MC_IFACE_GP,
                    if class == 0 { 0 } else { DATA + 0x5000 },
                );
                c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6000);
                c.bus.write32(
                    DATA + 0x5000 + 0x40,
                    if class == 1 { 0 } else { fakes::FAKE },
                );
                c.bus.write32(DATA + 0x5000 + 0x48, DATA + 0x8000);
                c.bus.write32(DATA + 0x5000 + 0x4c, DATA + 0x8100);
                c.g[A0][0] = if class == 2 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A1][0] = if class == 3 {
                    0
                } else {
                    (DATA + 0x7100) as u64
                };
                c.bus.write32(DATA + 0x7000, 0);
                c.bus.write32(DATA + 0x7100, seed ^ 0x44);
                c.bus.write32(
                    DATA + 0x6000,
                    match class {
                        5 => 5,
                        10 => 5,
                        11 => 0,
                        12 => [2, 7, 11, 12, 13][(seed / 13 % 5) as usize],
                        _ => 0,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 4, seed / 13 % 8);
                for i in 0..16u32 {
                    c.bus.write32(DATA + 0x8000 + 4 * i, fakes::FAKE + 0x20);
                    c.bus.write32(DATA + 0x8100 + 4 * i, fakes::FAKE + 0x30);
                }
                c.bus.write32(fakes::LOG_RESULTS, 4);
                c.bus.write32(
                    fakes::LOG_RESULTS + 4,
                    match class {
                        7 => 10,
                        8 => 16,
                        _ => 5,
                    },
                );
                c.bus.write32(fakes::LOG_RESULTS + 8, 6);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 13 % 4 == 0 {
                        u32::MAX
                    } else {
                        seed / 13 % 3
                    },
                );
            },
        );
    }

    fn mc_close_fixture(c: &mut Cpu, seed: u32) {
        let _r = fakes::machine(c, seed, 0x003f_9b38, 1);
        c.pc = 0x003f_9b38;
        let class = seed % 5;
        c.bus.write32(
            GAME_GP + MC_IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6004);
        c.bus.write32(DATA + 0x6004, if class == 2 { 1 } else { 0 });
        c.bus.write32(DATA + 0x5000 + 0x38, fakes::FAKE);
        c.g[A0][0] = match class {
            1 => 0,
            4 => (DATA + 0x7001) as u64,
            _ => (DATA + 0x7000) as u64,
        };
        for i in 0..0x61u32 {
            c.bus.write8(DATA + 0x7000 + i, (seed ^ i) as u8);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 5 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_close_matches() {
        fakes::covered(
            ee::s14::f_003f9b38__generated,
            slogin_mc_start_close,
            100,
            &|c, seed| mc_close_fixture(c, seed),
        );
    }

    #[test]
    fn slogin_mc_start_close_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f9b38__generated,
            slogin_mc_start_close,
            60,
            &|c, seed| {
                mc_close_fixture(c, seed * 5 + 3 + seed % 2);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(0x003f_9b38 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_9b38)
            },
        );
    }

    fn mc_seek_fixture(c: &mut Cpu, seed: u32) {
        let _r = fakes::machine(c, seed, 0x003f_9c48, 1);
        c.pc = 0x003f_9c48;
        let class = seed % 7;
        c.bus.write32(
            GAME_GP + MC_IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6004);
        c.bus.write32(DATA + 0x6004, if class == 2 { 1 } else { 0 });
        c.bus.write32(DATA + 0x5000 + 0x34, fakes::FAKE);
        c.g[A0][0] = match class {
            1 => 0,
            4 | 6 => (DATA + 0x7001) as u64,
            _ => (DATA + 0x7000) as u64,
        };
        c.g[A1][0] = (seed ^ 0x1020) as u64;
        c.g[A2][0] = if class < 5 { 0 } else { 1 };
        for i in 0..0x61u32 {
            c.bus.write8(DATA + 0x7000 + i, (i ^ seed) as u8);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 7 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_seek_matches() {
        fakes::covered(
            ee::s14::f_003f9c48__generated,
            slogin_mc_start_seek,
            112,
            &|c, seed| mc_seek_fixture(c, seed),
        );
    }

    #[test]
    fn slogin_mc_start_seek_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f9c48__generated,
            slogin_mc_start_seek,
            70,
            &|c, seed| {
                mc_seek_fixture(c, seed * 7 + 3 + seed % 4);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(0x003f_9c48 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_9c48)
            },
        );
    }

    fn mc_named_fixture(c: &mut Cpu, seed: u32, open: bool) {
        let entry = if open { 0x003f_99b8 } else { 0x003f_9698 };
        let _r = fakes::machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let class = seed % 11;
        c.bus.write32(
            GAME_GP + MC_IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        c.bus.write32(GAME_GP + MC_STATE_GP, DATA + 0x6000);
        c.bus.write16(
            DATA + 0x5000,
            match class {
                3 => 0,
                6 => 1,
                10 => 0x80,
                _ => 3,
            },
        );
        c.bus.write32(DATA + 0x5000 + 0x10, fakes::FAKE);
        c.bus.write32(DATA + 0x6000, if class == 4 { 1 } else { 0 });
        c.g[A0][0] = if class == 1 {
            0
        } else {
            (DATA + 0x7000) as u64
        };
        c.g[A1][0] = match class {
            2 => 8,
            6 | 7 | 8 | 9 => u64::MAX,
            10 => 7,
            _ => 0,
        };
        c.g[A2][0] = match class {
            8 => 1,
            9 => 2,
            _ => 0,
        };
        for (i, ch) in b"SLOGIN.DAT\0".iter().enumerate() {
            c.bus.write8(DATA + 0x7000 + i as u32, *ch);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 11 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_delete_matches() {
        fakes::covered(
            ee::s14::f_003f9698__generated,
            slogin_mc_start_delete,
            132,
            &|c, seed| mc_named_fixture(c, seed, false),
        );
    }

    #[test]
    fn slogin_mc_start_open_matches() {
        fakes::covered(
            ee::s14::f_003f99b8__generated,
            slogin_mc_start_open,
            132,
            &|c, seed| mc_named_fixture(c, seed, true),
        );
    }

    #[test]
    fn slogin_mc_start_create_matches() {
        fakes::covered(
            ee::s14::f_003f9818__generated,
            slogin_mc_start_create,
            144,
            &|c, seed| {
                let class = seed % 12;
                let mapped = if class == 11 { 5 } else { class };
                mc_named_fixture(c, seed / 12 * 11 + mapped, false);
                c.pc = 0x003f_9818;
                if class == 11 {
                    for i in 0..24u32 {
                        c.bus.write8(DATA + 0x7000 + i, b'X');
                    }
                    c.bus.write8(DATA + 0x7000 + 24, 0);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_shutdown_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            mc_free_fake,
            &PREV_MC_FREE,
        );
        check(
            ee::s14::f_003f9530__generated,
            slogin_mc_shutdown,
            90,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9530, 2);
                c.pc = 0x003f_9530;
                c.bus.write32(MC_TEST_CTRL, MC_TEST_MAGIC);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed % 6 < 2 { seed % 6 } else { u32::MAX },
                );
                let class = seed % 3;
                c.bus.write32(
                    GAME_GP + MC_IFACE_GP,
                    if class == 0 { 0 } else { DATA + 0x3000 },
                );
                c.bus.write32(
                    GAME_GP + MC_STATE_GP,
                    if class == 2 { DATA + 0x4000 } else { 0 },
                );
                c.bus.write32(DATA + 0x3000 + 0x428, DATA + 0x5000);
                c.bus.write32(DATA + 0x4000, 0);
            },
        );
    }

    #[test]
    fn slogin_mc_get_file_state_matches() {
        check(
            ee::s14::f_003f95e8__generated,
            slogin_mc_get_file_state,
            90,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_95e8);
                let class = seed % 3;
                let state = if class == 1 {
                    DATA + 0x2004
                } else {
                    DATA + 0x2000
                };
                c.bus.write32(GAME_GP + MC_STATE_GP, state);
                c.g[A0][0] = match class {
                    0 => 0,
                    1 => (DATA + 0x1000) as u64,
                    _ => (DATA + 0x1001) as u64,
                };
                for i in 0..0x60u32 {
                    c.bus.write8(state + 0xc + i, (i ^ seed) as u8);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_is_available_wrapper_matches() {
        check(
            ee::s14::f_003fa360__generated,
            slogin_mc_is_available_wrapper,
            40,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_a360);
            },
        );
    }

    #[test]
    fn slogin_mc_calc_save_size_matches() {
        check(
            ee::s14::f_003fa6a8__generated,
            slogin_mc_calc_save_size,
            100,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_a6a8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1004) as u64;
                c.bus
                    .write32(DATA + 0x3cc, if seed % 2 == 0 { 1024 } else { 1025 });
                c.bus
                    .write32(DATA + 4, if seed % 4 < 2 { 2048 } else { 2049 });
            },
        );
    }

    #[test]
    fn slogin_mc_get_port_slot_matches() {
        check(
            ee::s14::f_003fa1d0__generated,
            slogin_mc_get_port_slot,
            128,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_a1d0);
                let class = seed % 4;
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x1000) as u64
                };
                c.g[A0][0] = if class == 2 {
                    8 + (seed / 4 % 2) as u64
                } else {
                    (seed / 4 % 8) as u64
                };
                c.bus.write32(
                    GAME_GP + MC_IFACE_GP,
                    if class <= 1 { 0 } else { DATA + 0x2000 },
                );
                const ARMS: [u32; 8] = [
                    0x003f_a72c,
                    0x003f_a738,
                    0x003f_a748,
                    0x003f_a750,
                    0x003f_a758,
                    0x003f_a768,
                    0x003f_a778,
                    0x003f_a784,
                ];
                for i in 0..8u32 {
                    c.bus.write32(0x0049_73e0 + 4 * i, ARMS[i as usize]);
                }
            },
        );
    }
}
