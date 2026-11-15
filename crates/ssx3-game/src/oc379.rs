//! Staging module oc379 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc379`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, Frame};
use crate::events::back_edge_unwinds;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_c808,
        mc_manager_get_instance,
        "MCManager_GetInstance",
    ),
    (0x0023_c8f0, mc_manager_begin_load, "MCManager_BeginLoad"),
    (
        0x0023_caa0,
        mc_manager_set_data_size,
        "MCManager_SetDataSize",
    ),
    (0x0023_cae8, mc_manager_begin_save, "MCManager_BeginSave"),
    (
        0x0023_cc58,
        mc_manager_set_data_size_wrapper,
        "MCManager_SetDataSize_Wrapper",
    ),
    (
        0x0023_cc78,
        mc_manager_format_save_desc,
        "MCManager_FormatSaveDesc",
    ),
    (
        0x0023_cdb0,
        mc_manager_build_save_title,
        "MCManager_BuildSaveTitle",
    ),
    (
        0x0023_d660,
        mc_manager_update_mode1,
        "MCManager_UpdateMode1",
    ),
    (0x0023_e268, mc_manager_state31, "MCManager_State31"),
    (0x0023_e2c0, mc_manager_state32, "MCManager_State32"),
    (0x0023_e4f0, mc_manager_state34, "MCManager_State34"),
    (
        0x0023_e8a0,
        mc_manager_classify_card_state,
        "MCManager_ClassifyCardState",
    ),
    (0x0023_f308, mc_manager_state0d, "MCManager_State0D"),
    (
        0x0023_f578,
        mc_manager_state11_save,
        "MCManager_State11_Save",
    ),
    (0x0023_fb18, mc_manager_set_slot, "MCManager_SetSlot"),
    (0x0023_fd40, mc_manager_state16, "MCManager_State16"),
    (0x0023_fff0, mc_manager_state1e, "MCManager_State1E"),
    (0x0024_09c8, mc_manager_state09, "MCManager_State09"),
    (0x0024_0cc0, mc_manager_state2b, "MCManager_State2B"),
    (
        0x0024_0d90,
        mc_manager_state2c_delete,
        "MCManager_State2C_Delete",
    ),
    (0x0024_0f10, mc_manager_state2f, "MCManager_State2F"),
    (
        0x0024_0fb0,
        mc_manager_write_save_by_name,
        "MCManager_WriteSaveByName",
    ),
    (
        0x0024_1050,
        mc_manager_write_save_by_slot,
        "MCManager_WriteSaveBySlot",
    ),
    (
        0x0024_10d0,
        mc_manager_clamp_file_index,
        "MCManager_ClampFileIndex",
    ),
    (0x0024_1400, mc_manager_next_slot, "MCManager_NextSlot"),
    (0x0024_1540, mc_manager_prev_slot, "MCManager_PrevSlot"),
    (0x0024_1678, mc_manager_prev_file, "MCManager_PrevFile"),
    (0x0024_16c0, mc_manager_next_file, "MCManager_NextFile"),
    (
        0x0024_1928,
        mc_manager_format_slot_free_space,
        "MCManager_FormatSlotFreeSpace",
    ),
    (
        0x0024_1fb0,
        mc_manager_read_current_dir_info,
        "MCManager_ReadCurrentDirInfo",
    ),
];

/// `MCManager *MCManager_GetInstance(void)` (0x23c808): the manager singleton
/// at `gp-0x47c`, creating it through `MCManager_ctor` on first use.
pub fn mc_manager_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let slot = Addr((c.g[GP][0] as u32).wrapping_sub(0x47c));
        c.g[V0][0] = slot.lw(c);
        let missing = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !missing {
            c.cycles += 4;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 9;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x74c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ed8));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        // EA_MemAllocNamed(size 0x74c, "memcard"-adjacent pool name).
        call_ptr(c, 0x0031_7d70, 0x0023_c830)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0024_2288, 0x0023_c838)?;
        slot.sw(c, c.g[V0][0]);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_SetSlot(MCManager *self, int slot)` (0x23fb18): record the
/// selected save slot at `self+0x428`.
pub fn mc_manager_set_slot(c: &mut Cpu) {
    c.cycles += 2;
    Addr(abi::arg(c, 0)).off(0x428).sw(c, c.g[A1][0]);
    ret(c);
}

/// `void *MCManager_SetDataSize_Wrapper(MCManager *self)` (0x23cc58): call
/// through to `MCManager_SetDataSize` and return.
pub fn mc_manager_set_data_size_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0023_caa0, 0x0023_cc68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *MCManager_State2F(MCManager *self)` (0x240f10): reset the manager
/// through `MCManager_Reset` and return.
pub fn mc_manager_state2f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0023_fb58, 0x0024_0f20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *MCManager_ReadCurrentDirInfo(MCManager *self)` (0x241fb0): reread
/// the directory info for the current slot (`self+0x42c`) through
/// `MCManager_ReadDirInfo` and return.
pub fn mc_manager_read_current_dir_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(0x42c).lw(c);
        call_ptr(c, 0x0024_20c8, 0x0024_1fc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State32(MCManager *self)` (0x23e2c0): no-op state step,
/// returns immediately.
pub fn mc_manager_state32(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MCManager_State34(MCManager *self)` (0x23e4f0): no-op state step,
/// returns immediately.
pub fn mc_manager_state34(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void *MCManager_SetDataSize(MCManager *self, int size)` (0x23caa0): store
/// `size` as the pending data size at `self+0xf0` through the backend's size
/// hook (vtable at `self+0x434`, method `+0x98/0x9c`).
pub fn mc_manager_set_data_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_cad0)?;
        Addr(c.g[S1][0] as u32).off(0xf0).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_PrevFile(MCManager *self)` (0x241678): step the file index
/// at `+0xf8` down by one, or wrap to the clamped top when it is already 0
/// and the list at `+0x100` is non-empty.
pub fn mc_manager_prev_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
        let above_zero = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if above_zero {
            c.cycles += 2;
            Addr(c.g[S0][0] as u32).off(0xf8).sw(c, c.g[V0][0]);
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x100).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0024_10d0, 0x0024_16a8)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.cycles += 2;
                Addr(c.g[S0][0] as u32).off(0xf8).sw(c, c.g[V0][0]);
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_NextFile(MCManager *self)` (0x2416c0): step the file index
/// at `+0xf8` up by one toward the clamped top, wrapping to 0 when it is
/// already there and the list at `+0x100` is non-empty.
pub fn mc_manager_next_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0024_10d0, 0x0024_16d4)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let at_top = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if at_top {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x100).lw(c);
            if c.g[V0][0] != 0 {
                Addr(c.g[S0][0] as u32).off(0xf8).sw(c, 0);
            }
        } else {
            c.cycles += 2;
            Addr(c.g[S0][0] as u32).off(0xf8).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_WriteSaveByName(MCManager *self, int a, int b, int c)`
/// (0x240fb0): flag the save pending at `gp-0x478`, run the backend's
/// by-name write hook (vtable at `self+0x434`, method `+0x120/0x124`) and
/// keep `c` at `self+0xe4`.
pub fn mc_manager_write_save_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[T0][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x478)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[T0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x120).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x124).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0fe8)?;
        Addr(c.g[S1][0] as u32).off(0xe4).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_WriteSaveBySlot(MCManager *self, int a, int b, int c)`
/// (0x241050): flag the save pending at `gp-0x478`, run the backend's
/// by-slot write hook (vtable at `self+0x434`, method `+0x130/0x134`) and
/// keep `c` at `self+0xe4`.
pub fn mc_manager_write_save_by_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[T0][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x478)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[T0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x130).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x134).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1088)?;
        Addr(c.g[S1][0] as u32).off(0xe4).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State31(MCManager *self)` (0x23e268): when the movie is
/// not finished, tell the 3D crowd manager to stop (message 0x32).
pub fn mc_manager_state31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e290)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
            c.g[A1][0] = 0x32;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e2b0)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MCManager_ClampFileIndex(MCManager *self)` (0x2410d0): the file
/// count at `+0xec`, or the backend's count when that is larger (vtable at
/// `self+0x434`, method `+0x220/0x224`).
pub fn mc_manager_clamp_file_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_24112c: {
            'b_241128: {
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x220).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x224).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_10f8)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xec).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let clamped = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[V1][0];
                if clamped {
                    break 'b_241128;
                }
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x220).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x224).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_1120)?;
                frame.restore(c, S0, 0x10);
                break 'b_24112c;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'b_24112c;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State0D(MCManager *self)` (0x23f308): poll the device at
/// the file index (vtable methods `+0x218/0x21c` then `+0x208/0x20c`); report
/// message 0xe when the first poll is done, 0xf when the second is.
pub fn mc_manager_state0d(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_23f394: {
            'b_23f37c: {
                'b_23f360: {
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x218).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x21c).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f334)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        break 'b_23f360;
                    }
                    c.cycles += 9;
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x208).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x20c).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f358)?;
                    let second_done = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    if second_done {
                        break 'b_23f37c;
                    }
                    break 'b_23f360;
                }
                c.cycles += 7;
                c.g[A1][0] = 0xe;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f374)?;
                frame.restore(c, S0, 0x10);
                break 'b_23f394;
            }
            c.cycles += 6;
            c.g[A1][0] = 0xf;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_f390)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FormatSlotFreeSpace(MCManager *self, ...)` (0x241928):
/// format one memory-card device's free space into a wide buffer. The device
/// record is at `self + a1 * 0xe0`; a missing record, or one with any of
/// three optional strings null, leaves the destination untouched.
pub fn mc_manager_format_slot_free_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x450);
        c.g[V0][0] = 0xe0;
        frame.save(c, S1, 0x430);
        c.mult(0, 17, 5, 2);
        frame.save(c, S2, 0x420);
        frame.save(c, S3, 0x410);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x440);
        frame.save_ra(c, 0x400);
        let dev = (c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32);
        c.g[V0][0] = sx(dev);
        c.g[V1][0] = Addr(dev).off(0x350).lw(c);
        let missing = c.g[V1][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        // 7 cycles for the two skipped block tails, 6 for the middle one, 5
        // for the outer one (see the generated nesting).
        let mut tail = 7u64;
        'checks: {
            if missing {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(dev).off(0x344).lw(c);
            let missing = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x440);
            if missing {
                tail = 6;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(dev).off(0x348).lw(c);
            if c.g[V1][0] == 0 {
                frame.restore(c, S1, 0x430);
                tail = 5;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(dev).off(0x34c).lw(c);
            let missing = c.g[V1][0] == 0;
            c.g[A1][0] = 0x4a0000;
            if missing {
                tail = 6;
                break 'checks;
            }
            // Two wide strings then the formatted line.
            c.cycles += 14;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c90));
            call_ptr(c, 0x002c_2540, 0x0024_198c)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c98));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_2540, 0x0024_19a0)?;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x354).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, 0x002c_26d0, 0x0024_19b8)?;
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x440);
        frame.restore(c, S1, 0x430);
        frame.restore(c, S2, 0x420);
        frame.restore(c, S3, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State1E(MCManager *self)` (0x23fff0): when the movie is
/// finished, run the state's completion hook; otherwise poll the overlay
/// (vtable `+0x1a8/0x1ac`) and either run the pending hook at `+0x78` or
/// report through the crowd manager.
pub fn mc_manager_state1e(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_2400a4: {
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0018)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'b_2400a4;
            }
            'b_240078: {
                'b_240060: {
                    'b_24004c: {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0038)?;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x78).lw(c);
                            break 'b_24004c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        c.g[A1][0] = 4;
                        break 'b_240060;
                    }
                    c.cycles += 4;
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0024_0054)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x1f;
                    if done {
                        break 'b_240078;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    break 'b_240060;
                }
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0070)?;
                frame.restore(c, S0, 0x10);
                break 'b_2400a4;
            }
            'b_2400a0: {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0024_0084)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0x1d;
                if done {
                    break 'b_2400a0;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_00a0)?;
                break 'b_2400a0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State2B(MCManager *self)` (0x240cc0): when the movie is
/// finished, run the state's completion hook; otherwise poll the overlay
/// (vtable `+0x1a8/0x1ac`) and either run the pending hook at `+0x68` or
/// report through the crowd manager.
pub fn mc_manager_state2b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_240d84: {
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0ce8)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'b_240d84;
            }
            'b_240d64: {
                'b_240d4c: {
                    'b_240d1c: {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0d08)?;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x68).lw(c);
                            break 'b_240d1c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        c.g[A1][0] = 4;
                        break 'b_240d4c;
                    }
                    'b_240d34: {
                        c.cycles += 4;
                        let target = r32(c, V0);
                        call_ptr(c, target, 0x0024_0d24)?;
                        let done = c.g[V0][0] == 0;
                        c.g[A1][0] = 0x2c;
                        if done {
                            break 'b_240d34;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        break 'b_240d4c;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0024_0d40)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = 6;
                    if done {
                        break 'b_240d64;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    break 'b_240d4c;
                }
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0d5c)?;
                frame.restore(c, S0, 0x10);
                break 'b_240d84;
            }
            c.cycles += 8;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x428).lw(c);
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x2c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0d80)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State09(MCManager *self)` (0x2409c8): when the movie is
/// finished, run the state's completion hook; otherwise poll the overlay
/// (vtable `+0x1a8/0x1ac`) and either run the pending hook at `+0x70` or
/// report through the crowd manager.
pub fn mc_manager_state09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_240aa4: {
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_09f0)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'b_240aa4;
            }
            'b_240a78: {
                'b_240a60: {
                    'b_240a24: {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0a10)?;
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            break 'b_240a24;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        c.g[A1][0] = 4;
                        break 'b_240a60;
                    }
                    'b_240a48: {
                        c.cycles += 7;
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0a38)?;
                        let done = c.g[V0][0] == 0;
                        c.g[A1][0] = 0x14;
                        if done {
                            break 'b_240a48;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        break 'b_240a60;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0024_0a54)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = 0xd;
                    if done {
                        break 'b_240a78;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    break 'b_240a60;
                }
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0a70)?;
                frame.restore(c, S0, 0x10);
                break 'b_240aa4;
            }
            'b_240aa0: {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0024_0a84)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 1;
                if done {
                    break 'b_240aa0;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0aa0)?;
                break 'b_240aa0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State11_Save(MCManager *self)` (0x23f578): count the save
/// down, update the progress fraction and either wait for the decoder
/// (vtable `+0x248/0x24c`) or finish the progress bar and kick the save.
pub fn mc_manager_state11_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
        c.cvt_s_w(0, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x108).lw(c);
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xf0));
        c.cvt_s_w(1, 1);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(c.g[S0][0] as u32).off(0x108).sw(c, c.g[V0][0]);
        c.div_s(0, 0, 1);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f5c4)?;
        'b_23f688: {
            'b_23f660: {
                'b_23f62c: {
                    'b_23f61c: {
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = c.g[S0][0];
                            break 'b_23f61c;
                        }
                        c.cycles += 9;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x150).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x154).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0023_f5e4)?;
                        c.g[V1][0] = 1;
                        if c.g[V0][0] != c.g[V1][0] {
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            break 'b_23f660;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xe4).lw(c);
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xf0).lw(c);
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let under = c.g[V0][0] == 0;
                        if under {
                            break 'b_23f62c;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        call_ptr(c, target, 0x0023_f610)?;
                        if c.g[V0][0] != 0 {
                            frame.restore(c, S0, 0x10);
                            break 'b_23f688;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        break 'b_23f61c;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0024_1180, 0x0023_f624)?;
                    frame.restore(c, S0, 0x10);
                    break 'b_23f688;
                }
                c.cycles += 13;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_cc20, 0x0023_f634)?;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_f640)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A1][0] = 0x12;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f658)?;
                frame.restore(c, S0, 0x10);
                break 'b_23f688;
            }
            'b_23f684: {
                c.cycles += 7;
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f674)?;
                let done = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if done {
                    break 'b_23f684;
                }
                c.cycles += 2;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0024_1180, 0x0023_f684)?;
                break 'b_23f684;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State2C_Delete(MCManager *self)` (0x240d90): count the
/// delete down and either wait for the decoder (vtable `+0x248/0x24c`) or
/// advance the delete slot and report through the crowd manager.
pub fn mc_manager_state2c_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x108).lw(c);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(c.g[S0][0] as u32).off(0x108).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0dc4)?;
        'b_240ea0: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'b_240ea0;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x108).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                frame.restore(c, S0, 0x10);
                break 'b_240ea0;
            }
            'b_240e14: {
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0df0)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0x2d;
                if done {
                    break 'b_240e14;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0e0c)?;
                frame.restore(c, S0, 0x10);
                break 'b_240ea0;
            }
            'b_240e9c: {
                'b_240e88: {
                    c.cycles += 10;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x68).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x6c).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0e2c)?;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        break 'b_240e88;
                    }
                    'b_240e7c: {
                        c.cycles += 8;
                        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x208).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x20c).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0e54)?;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                            break 'b_240e7c;
                        }
                        c.cycles += 8;
                        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x50).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x54).lw(c);
                        let target = r32(c, V1);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0e78)?;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                        break 'b_240e7c;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    Addr(c.g[S0][0] as u32).off(0x438).sw(c, c.g[V0][0]);
                    break 'b_240e9c;
                }
                c.cycles += 5;
                c.g[A1][0] = 0x2e;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0e9c)?;
                break 'b_240e9c;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State16(MCManager *self)` (0x23fd40): when the movie is
/// finished, run the state's completion hook; otherwise, when no load is
/// pending and the overlay is ready, end the operation (or report it done).
pub fn mc_manager_state16(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_23fe64: {
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_fd68)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                break 'b_23fe64;
            }
            'b_23fe38: {
                'b_23fe20: {
                    'b_23fda8: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x338).lw(c);
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                            break 'b_23fda8;
                        }
                        c.cycles += 8;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0023_fd94)?;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                            break 'b_23fda8;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        c.g[A1][0] = 4;
                        break 'b_23fe20;
                    }
                    'b_23fdc8: {
                        c.cycles += 4;
                        let target = r32(c, V0);
                        call_ptr(c, target, 0x0023_fdb0)?;
                        let done = c.g[V0][0] == 0;
                        c.g[A0][0] = c.g[S0][0];
                        if done {
                            break 'b_23fdc8;
                        }
                        c.cycles += 4;
                        c.g[A1][0] = 0;
                        call_ptr(c, 0x0023_ea30, 0x0023_fdc0)?;
                        frame.restore(c, S0, 0x10);
                        break 'b_23fe64;
                    }
                    'b_23fdec: {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x84).lw(c);
                        let target = r32(c, V0);
                        call_ptr(c, target, 0x0023_fdd4)?;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x68).lw(c);
                            break 'b_23fdec;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_ea90, 0x0023_fde4)?;
                        frame.restore(c, S0, 0x10);
                        break 'b_23fe64;
                    }
                    c.cycles += 4;
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_fdf4)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x234));
                    if done {
                        break 'b_23fe38;
                    }
                    c.cycles += 9;
                    c.g[A3][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
                    c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x70).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x74).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_fe18)?;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A1][0] = 0x19;
                    break 'b_23fe20;
                }
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_fe30)?;
                frame.restore(c, S0, 0x10);
                break 'b_23fe64;
            }
            'b_23fe60: {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_fe44)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0xd;
                if done {
                    break 'b_23fe60;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_fe60)?;
                break 'b_23fe60;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginSave(MCManager *self, int mode, int arg)` (0x23cae8):
/// start a save: record the mode, pick the "saving"/"loading" state word and
/// tell the movie object, then format the three 0x200-byte strings (title,
/// product code, save-type tag) into the request at `self+0x1b4`.
pub fn mc_manager_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x640);
        frame.save(c, S2, 0x610);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x630);
        frame.save(c, S1, 0x620);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0x600);
        Addr(c.g[S2][0] as u32).off(0xb4).sw(c, c.g[A1][0]);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0023_d570, 0x0023_cb10)?;
        c.g[V1][0] = 1;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        Addr(c.g[S2][0] as u32).off(0xfc).sw(c, c.g[S0][0]);
        let mode_nonzero = c.g[V0][0] != 0;
        Addr(c.g[S2][0] as u32).off(0xbc).sw(c, c.g[V1][0]);
        'b_23cbbc: {
            'b_23cb80: {
                'b_23cb64: {
                    'b_23cb38: {
                        if mode_nonzero {
                            break 'b_23cb38;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = 4;
                        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                        Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                        c.g[A1][0] = 0x80000;
                        break 'b_23cb64;
                    }
                    c.cycles += 2;
                    let mode_not_one = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 6;
                    if mode_not_one {
                        break 'b_23cb80;
                    }
                    c.cycles += 9;
                    Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                    call_ptr(c, 0x0014_dc80, 0x0023_cb48)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 5;
                    call_ptr(c, 0x0014_dd58, 0x0023_cb54)?;
                    call_ptr(c, 0x0015_2ba8, 0x0023_cb5c)?;
                    c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    break 'b_23cb64;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_cb78)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
                break 'b_23cbbc;
            }
            c.cycles += 15;
            Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V1][0]);
            call_ptr(c, 0x0014_dc80, 0x0023_cb88)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 5;
            call_ptr(c, 0x0014_dd58, 0x0023_cb94)?;
            call_ptr(c, 0x0015_2948, 0x0023_cb9c)?;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_cbb8)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        }
        c.cycles += 24;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        call_ptr(c, 0x002c_2540, 0x0023_cbcc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        call_ptr(c, 0x0024_1aa0, 0x0023_cbd8)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0x100;
        call_ptr(c, 0x0024_1d40, 0x0023_cbf0)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x002c_26d0, 0x0023_cc04)?;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore(c, S2, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginLoad(MCManager *self, int mode)` (0x23c8f0): the load
/// twin of `mc_manager_begin_save` (`-1` in the request slot, no cached
/// value, and the "loading" state word in the mode-2 path).
pub fn mc_manager_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x640);
        frame.save(c, S2, 0x610);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x630);
        frame.save(c, S1, 0x620);
        frame.save_ra(c, 0x600);
        Addr(c.g[S2][0] as u32).off(0xb4).sw(c, c.g[A1][0]);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0023_d570, 0x0023_c914)?;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        Addr(c.g[S2][0] as u32).off(0xfc).sw(c, c.g[V0][0]);
        let mode_nonzero = c.g[V1][0] != 0;
        Addr(c.g[S2][0] as u32).off(0xbc).sw(c, 0);
        'b_23c9c8: {
            'b_23c98c: {
                'b_23c970: {
                    'b_23c93c: {
                        if mode_nonzero {
                            break 'b_23c93c;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = 4;
                        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                        Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                        c.g[A1][0] = 0x80000;
                        break 'b_23c970;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    let mode_not_one = c.g[V1][0] != c.g[V0][0];
                    if mode_not_one {
                        break 'b_23c98c;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = 6;
                    Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                    call_ptr(c, 0x0014_dc80, 0x0023_c954)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 5;
                    call_ptr(c, 0x0014_dd58, 0x0023_c960)?;
                    call_ptr(c, 0x0015_2ba8, 0x0023_c968)?;
                    c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    break 'b_23c970;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_c984)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
                break 'b_23c9c8;
            }
            c.cycles += 15;
            Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0014_dc80, 0x0023_c994)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 5;
            call_ptr(c, 0x0014_dd58, 0x0023_c9a0)?;
            call_ptr(c, 0x0015_2948, 0x0023_c9a8)?;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_c9c4)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        }
        c.cycles += 24;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        call_ptr(c, 0x002c_2540, 0x0023_c9d8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        call_ptr(c, 0x0024_1aa0, 0x0023_c9e4)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0x100;
        call_ptr(c, 0x0024_1d40, 0x0023_c9fc)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x002c_26d0, 0x0023_ca10)?;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore(c, S2, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FormatSaveDesc(MCManager *self, int mode, char *dst)`
/// (0x23cc78): describe the save for `mode` into the 0x200-byte buffers: mode
/// 1 formats the "card" name from the world device, higher modes the "new
/// player" path, mode 0 the plain save-type tag.
pub fn mc_manager_format_save_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x350);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x330);
        frame.save(c, S2, 0x320);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x310);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x340);
        frame.save_ra(c, 0x300);
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xb4).lw(c);
        let mode_is_one = c.g[V1][0] == c.g[V0][0];
        c.g[S3][0] = c.g[A1][0];
        'b_23cd9c: {
            'b_23cd98: {
                'b_23cd58: {
                    if mode_is_one {
                        break 'b_23cd58;
                    }
                    'b_23cd18: {
                        'b_23ccd4: {
                            'b_23ccc4: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[V0][0] = 2;
                                    break 'b_23ccc4;
                                }
                                c.cycles += 2;
                                let mode_is_zero = c.g[V1][0] == 0;
                                frame.restore(c, S0, 0x340);
                                if mode_is_zero {
                                    break 'b_23ccd4;
                                }
                                c.cycles += 2;
                                frame.restore(c, S1, 0x330);
                                break 'b_23cd9c;
                            }
                            c.cycles += 2;
                            let mode_is_two = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                            if mode_is_two {
                                break 'b_23cd18;
                            }
                            c.cycles += 2;
                            frame.restore(c, S0, 0x340);
                            break 'b_23cd98;
                        }
                        c.cycles += 17;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                        c.g[A1][0] = 0x4a0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c88));
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002c_2540, 0x0023_cce8)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0xb4).lw(c);
                        c.g[A3][0] = 0x80;
                        call_ptr(c, 0x0024_1d40, 0x0023_ccfc)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                        c.g[A2][0] = c.g[SP][0];
                        call_ptr(c, 0x002c_26d0, 0x0023_cd10)?;
                        frame.restore(c, S0, 0x340);
                        break 'b_23cd98;
                    }
                    c.cycles += 16;
                    c.g[A0][0] = 0x480000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ec8));
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x0031_7670, 0x0023_cd34)?;
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    let target = r32(c, V1);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, target, 0x0023_cd44)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x002c_2508, 0x0023_cd50)?;
                    frame.restore(c, S0, 0x340);
                    break 'b_23cd98;
                }
                c.cycles += 16;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c88));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_2540, 0x0023_cd6c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0xb4).lw(c);
                c.g[A3][0] = 0x80;
                call_ptr(c, 0x0024_1d40, 0x0023_cd80)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x002c_26d0, 0x0023_cd94)?;
                frame.restore(c, S0, 0x340);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x330);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x320);
        frame.restore(c, S3, 0x310);
        frame.restore_ra(c, 0x300);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_UpdateMode1(MCManager *self)` (0x23d660): refresh the
/// saved-slot count when the load changed, then run the finished-movie hook
/// matching the mode word at `+0x130` (states 31, 33 and 36).
pub fn mc_manager_update_mode1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'b_23d798: {
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_d688)?;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x338).lw(c);
            'b_23d6bc: {
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x424).lw(c);
                    break 'b_23d6bc;
                }
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_d6ac)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'b_23d798;
                }
                c.cycles += 2;
                Addr(c.g[S0][0] as u32).off(0x338).sw(c, 0);
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x424).lw(c);
            }
            'b_23d6d8: {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    break 'b_23d6d8;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0024_1ac0, 0x0023_d6d0)?;
                Addr(c.g[S0][0] as u32).off(0x424).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            }
            'b_23d750: {
                c.cycles += 7;
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_d6ec)?;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
                    break 'b_23d750;
                }
                'b_23d730: {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0023_d570, 0x0023_d700)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
                    c.g[V0][0] = c.g[V1][0] ^ 0x31;
                    let is_31 = c.g[V0][0] == 0;
                    c.g[A0][0] = 0;
                    if is_31 {
                        break 'b_23d730;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] ^ 0x32;
                    let is_32 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] ^ 0x33;
                    if is_32 {
                        break 'b_23d730;
                    }
                    c.cycles += 2;
                    let is_33 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] ^ 0x34;
                    if is_33 {
                        break 'b_23d730;
                    }
                    c.cycles += 2;
                    let is_34 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] ^ 0x36;
                    if is_34 {
                        break 'b_23d730;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_23d730;
                }
                'b_23d74c: {
                    c.cycles += 2;
                    let mode_set = c.g[A0][0] == 0;
                    c.g[A1][0] = 3;
                    if mode_set {
                        break 'b_23d74c;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_d74c)?;
                    break 'b_23d74c;
                }
                c.cycles += 1;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
            }
            'b_23d76c: {
                c.cycles += 3;
                c.g[V0][0] = 0x31;
                let not_31 = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x33;
                if not_31 {
                    break 'b_23d76c;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_e268, 0x0023_d764)?;
                frame.restore(c, S0, 0x10);
                break 'b_23d798;
            }
            'b_23d784: {
                c.cycles += 2;
                let not_33 = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x36;
                if not_33 {
                    break 'b_23d784;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_e498, 0x0023_d77c)?;
                frame.restore(c, S0, 0x10);
                break 'b_23d798;
            }
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x10);
                break 'b_23d798;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023_e320, 0x0023_d794)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BuildSaveTitle(MCManager *self, ...)` (0x23cdb0): format
/// the save title into the 0x200-byte scratch buffers: mode 0 names the world
/// device, mode 1 the "new player" path, higher modes the plain save-type
/// tag.
pub fn mc_manager_build_save_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x870);
        frame.save(c, S2, 0x840);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        frame.save(c, S4, 0x820);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x830);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0x810);
        frame.save(c, S0, 0x860);
        frame.save(c, S1, 0x850);
        frame.save_ra(c, 0x800);
        c.g[S3][0] = c.g[A2][0];
        call_ptr(c, 0x0023_cc78, 0x0023_cde4)?;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[A1][0] = 0x480000;
        c.g[S5][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3eb0));
        call_ptr(c, 0x002c_2540, 0x0023_cdf8)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        let mode_is_zero = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        'b_23cef0: {
            'b_23cea8: {
                'b_23ce98: {
                    'b_23ce60: {
                        'b_23ce14: {
                            if mode_is_zero {
                                break 'b_23ce14;
                            }
                            c.cycles += 2;
                            let mode_is_one = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                            if mode_is_one {
                                break 'b_23ce60;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 0x480000;
                            break 'b_23cea8;
                        }
                        c.cycles += 19;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        c.g[A0][0] = 0x480000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ea0));
                        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0031_7670, 0x0023_ce34)?;
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                        c.g[A0][0] = c.g[S0][0];
                        let target = r32(c, V1);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, target, 0x0023_ce44)?;
                        c.g[A1][0] = c.g[S5][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x002c_26d0, 0x0023_ce54)?;
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A2][0] = c.g[S4][0];
                        break 'b_23ce98;
                    }
                    c.cycles += 14;
                    call_ptr(c, 0x0014_dc80, 0x0023_ce68)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0014_dd58, 0x0023_ce74)?;
                    c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0xfc).lw(c);
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x600));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_7170, 0x0023_ce84)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c_2540, 0x0023_ce90)?;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = c.g[S0][0];
                    break 'b_23ce98;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002c_26d0, 0x0023_cea0)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                break 'b_23cef0;
            }
            c.cycles += 18;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ec8));
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x0031_7670, 0x0023_cec0)?;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = r32(c, V1);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x0023_ced0)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x002c_2508, 0x0023_cedc)?;
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002c_26d0, 0x0023_ceec)?;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            break 'b_23cef0;
        }
        c.cycles += 18;
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x168).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x16c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_cf08)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x234));
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x002c_2508, 0x0023_cf14)?;
        frame.restore(c, S0, 0x860);
        frame.restore(c, S1, 0x850);
        frame.restore(c, S2, 0x840);
        frame.restore(c, S3, 0x830);
        frame.restore(c, S4, 0x820);
        frame.restore(c, S5, 0x810);
        frame.restore_ra(c, 0x800);
        frame.close(c);
        Ok(())
    })
}

/// `int MCManager_ClassifyCardState(MCManager *self, int arg)` (0x23e8a0):
/// walk the card-state vtable cascade and return the first matching state
/// word (`+0xbc` when a hook is done, `+0x130` when idle), reporting 6, 7,
/// 4, 5, 0x1d or 0xd through the crowd manager along the way.
pub fn mc_manager_classify_card_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        'b_23ea20: {
            'b_23ea1c: {
                'b_23ea04: {
                    'b_23e9ec: {
                        'b_23e954: {
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0023_e8d0)?;
                            let busy = c.g[V0][0] != 0;
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            if busy {
                                break 'b_23e954;
                            }
                            'b_23e940: {
                                'b_23e938: {
                                    c.cycles += 7;
                                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x188).lh(c);
                                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x18c).lw(c);
                                    let target = r32(c, V0);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, target, 0x0023_e8ec)?;
                                    if c.g[V0][0] != 0 {
                                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                                        break 'b_23e938;
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x198).lh(c);
                                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x19c).lw(c);
                                    let target = r32(c, V0);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, target, 0x0023_e90c)?;
                                    if c.g[V0][0] != 0 {
                                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                                        break 'b_23e938;
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lh(c);
                                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1dc).lw(c);
                                    let target = r32(c, V0);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, target, 0x0023_e92c)?;
                                    if c.g[V0][0] == 0 {
                                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
                                        break 'b_23e940;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                                    break 'b_23e938;
                                }
                                c.cycles += 2;
                                c.g[A1][0] = 5;
                                break 'b_23e9ec;
                            }
                            c.cycles += 3;
                            c.g[V0][0] ^= 4;
                            let is_four = c.g[V0][0] == 0;
                            c.g[A1][0] = 4;
                            if is_four {
                                break 'b_23ea1c;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                            break 'b_23e9ec;
                        }
                        'b_23e9e0: {
                            c.cycles += 7;
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1b8).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1bc).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0023_e968)?;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                                break 'b_23e9e0;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1d0).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1d4).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0023_e988)?;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                                break 'b_23e9e0;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x198).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x19c).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0023_e9a8)?;
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                                break 'b_23e9e0;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1dc).lw(c);
                            let target = r32(c, V0);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0023_e9c8)?;
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                                break 'b_23e9e0;
                            }
                            c.cycles += 2;
                            let arg_is_zero = c.g[S1][0] == 0;
                            c.g[A1][0] = 6;
                            if arg_is_zero {
                                break 'b_23ea1c;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                            break 'b_23e9ec;
                        }
                        c.cycles += 2;
                        let done = c.g[V0][0] == 0;
                        c.g[A1][0] = 7;
                        if done {
                            break 'b_23ea04;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        break 'b_23e9ec;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_e9fc)?;
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    break 'b_23ea20;
                }
                c.cycles += 6;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A1][0] = 0x1d;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_ea1c)?;
                break 'b_23ea1c;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
            frame.restore(c, S1, 0x10);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_NextSlot(MCManager *self)` (0x241400): step the chapter at
/// `+0x428` forward to the next chapter that has content (wrapping past the
/// end back to the current one), then record it and refresh the slot.
/// The back edge re-enters at 0x241430, so that address is an entry too.
pub fn mc_manager_next_slot(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    loop {
        match c.pc {
            0x0024_1400 => {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                frame.save(c, S1, 0x10);
                frame.save(c, S0, 0x20);
                c.g[S1][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
                let no_slot = c.g[V1][0] == c.g[V0][0];
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.pc = if no_slot { 0x0024_152c } else { 0x0024_1424 };
            }
            0x0024_1424 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[V1][0];
                c.pc = 0x0024_1464;
            }
            0x0024_142c => {
                c.cycles += 1;
                c.pc = 0x0024_1430;
            }
            0x0024_1430 => {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_1450;
                } else {
                    c.pc = 0x0024_1440;
                }
            }
            0x0024_1440 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                let list_done = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                c.pc = if list_done { 0x0024_1468 } else { 0x0024_144c };
            }
            0x0024_144c => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.pc = 0x0024_1450;
            }
            0x0024_1450 => {
                c.cycles += 2;
                let wrapped = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 1;
                c.pc = if wrapped { 0x0024_14a4 } else { 0x0024_1458 };
            }
            0x0024_1458 => {
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                    c.pc = 0x0024_14a8;
                } else {
                    c.pc = 0x0024_1464;
                }
            }
            0x0024_1464 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                c.pc = 0x0024_1468;
            }
            0x0024_1468 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                    c.pc = 0x0024_147c;
                } else {
                    c.pc = 0x0024_1474;
                }
            }
            0x0024_1474 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.pc = 0x0024_1480;
            }
            0x0024_147c => {
                c.cycles += 1;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = 0;
                }
                c.pc = 0x0024_1480;
            }
            0x0024_1480 => {
                c.cycles += 9;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_149c).is_err() {
                    return;
                }
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
                    if back_edge_unwinds(c, 0x0024_1430) {
                        return;
                    }
                    c.pc = 0x0024_1430;
                } else {
                    c.pc = 0x0024_14a4;
                }
            }
            0x0024_14a4 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                c.pc = 0x0024_14a8;
            }
            0x0024_14a8 => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_150c;
                } else {
                    c.pc = 0x0024_14b0;
                }
            }
            0x0024_14b0 => {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_150c;
                } else {
                    c.pc = 0x0024_14c0;
                }
            }
            0x0024_14c0 => {
                c.cycles += 9;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_14dc).is_err() {
                    return;
                }
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_150c;
                } else {
                    c.pc = 0x0024_14e4;
                }
            }
            0x0024_14e4 => {
                c.cycles += 10;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_1504).is_err() {
                    return;
                }
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 0;
                }
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.pc = 0x0024_150c;
            }
            0x0024_150c => {
                c.cycles += 2;
                if c.g[V0][0] == c.g[S0][0] {
                    Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
                    c.pc = 0x0024_152c;
                } else {
                    c.pc = 0x0024_1514;
                }
            }
            0x0024_1514 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                if call_ptr(c, 0x0023_fb18, 0x0024_1520).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S1][0];
                if call_ptr(c, 0x0023_fae0, 0x0024_1528).is_err() {
                    return;
                }
                Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
                c.pc = 0x0024_152c;
            }
            0x0024_152c => {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0024_1400, 0x0024_1540);
                return;
            }
        }
    }
}

/// `void MCManager_PrevSlot(MCManager *self)` (0x241540): step the chapter at
/// `+0x428` back to the previous chapter that has content (wrapping past the
/// start back to the current one), then record it and refresh the slot.
/// The back edge re-enters at 0x241570, so that address is an entry too.
pub fn mc_manager_prev_slot(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    loop {
        match c.pc {
            0x0024_1540 => {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                frame.save(c, S1, 0x20);
                frame.save(c, S0, 0x30);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                frame.save_ra(c, 0);
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
                let no_slot = c.g[V0][0] == c.g[V1][0];
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.pc = if no_slot { 0x0024_165c } else { 0x0024_1568 };
            }
            0x0024_1568 => {
                c.cycles += 2;
                c.g[S2][0] = 1;
                c.pc = 0x0024_1598;
            }
            0x0024_1570 => {
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_1588;
                } else {
                    c.pc = 0x0024_1578;
                }
            }
            0x0024_1578 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                let list_done = c.g[V0][0] == 0;
                c.pc = if list_done { 0x0024_1598 } else { 0x0024_1584 };
            }
            0x0024_1584 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.pc = 0x0024_1588;
            }
            0x0024_1588 => {
                c.cycles += 2;
                let wrapped = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                c.pc = if wrapped { 0x0024_15e0 } else { 0x0024_1590 };
            }
            0x0024_1590 => {
                c.cycles += 2;
                let list_one = c.g[V0][0] != c.g[S2][0];
                c.pc = if list_one { 0x0024_15e0 } else { 0x0024_1598 };
            }
            0x0024_1598 => {
                c.cycles += 2;
                if (c.g[S0][0] as i64) <= 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                    c.pc = 0x0024_15a8;
                } else {
                    c.pc = 0x0024_15a0;
                }
            }
            0x0024_15a0 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                c.pc = 0x0024_15b8;
            }
            0x0024_15a8 => {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                    c.pc = 0x0024_15bc;
                } else {
                    c.pc = 0x0024_15b0;
                }
            }
            0x0024_15b0 => {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.pc = 0x0024_15b8;
            }
            0x0024_15b8 => {
                c.cycles += 1;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.pc = 0x0024_15bc;
            }
            0x0024_15bc => {
                c.cycles += 8;
                c.g[A1][0] = c.g[S0][0];
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_15d4).is_err() {
                    return;
                }
                if c.g[V0][0] == 0 {
                    if back_edge_unwinds(c, 0x0024_1570) {
                        return;
                    }
                    c.pc = 0x0024_1570;
                } else {
                    c.pc = 0x0024_15dc;
                }
            }
            0x0024_15dc => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                c.pc = 0x0024_15e0;
            }
            0x0024_15e0 => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_163c;
                } else {
                    c.pc = 0x0024_15e8;
                }
            }
            0x0024_15e8 => {
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_163c;
                } else {
                    c.pc = 0x0024_15f0;
                }
            }
            0x0024_15f0 => {
                c.cycles += 9;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = 0;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_160c).is_err() {
                    return;
                }
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    c.pc = 0x0024_163c;
                } else {
                    c.pc = 0x0024_1614;
                }
            }
            0x0024_1614 => {
                c.cycles += 10;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_1634).is_err() {
                    return;
                }
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 0;
                }
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.pc = 0x0024_163c;
            }
            0x0024_163c => {
                c.cycles += 2;
                if c.g[V0][0] == c.g[S0][0] {
                    Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
                    c.pc = 0x0024_165c;
                } else {
                    c.pc = 0x0024_1644;
                }
            }
            0x0024_1644 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                if call_ptr(c, 0x0023_fb18, 0x0024_1650).is_err() {
                    return;
                }
                c.g[A0][0] = c.g[S1][0];
                if call_ptr(c, 0x0023_fae0, 0x0024_1658).is_err() {
                    return;
                }
                Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
                c.pc = 0x0024_165c;
            }
            0x0024_165c => {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x0024_1540, 0x0024_1674);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::{scramble, Rng, DATA, GAME_GP};
    use crate::game::dirtysock::tagsunk::tests::zero;
    use crate::game::fe::feasyncfile::tests::w32;
    use crate::game::movie::movieplayer::tests::{globals, ret_slot, RET_ZERO};

    /// Zeroed manager at `DATA+0x400` with a backend holder/vtable and a
    /// crowd-manager vtable, like the movieplayer `mc_setup_2` fixture but
    /// with the first poll result steered by `poll` (`RET_ZERO` goes on to
    /// poll, `FAKE` reports finished).
    fn mgr(c: &mut Cpu, seed: u32, salt: u64, poll: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals(c);
        let this = DATA + 0x400;
        zero(c, this, 0x800);
        let holder = DATA + 0x1000;
        let vt = DATA + 0x1100;
        w32(c, this + 0x434, holder);
        w32(c, holder, vt);
        ret_slot(c, vt, 0x58, poll);
        let crowd = DATA + 0x1200;
        w32(c, this + 0x748, crowd);
        ret_slot(c, crowd, 8, FAKE);
        ret_slot(c, crowd, 0xc, FAKE);
        c.g[A0][0] = sx(this);
        r
    }

    #[test]
    fn mc_manager_state32_matches() {
        check(
            ee::s06::f_0023e2c0__generated,
            mc_manager_state32,
            32,
            &|c, seed| {
                let mut r = Rng(0x23e2_c0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn mc_manager_state34_matches() {
        check(
            ee::s06::f_0023e4f0__generated,
            mc_manager_state34,
            32,
            &|c, seed| {
                let mut r = Rng(0x23e4_f0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn mc_manager_set_slot_matches() {
        check(
            ee::s06::f_0023fb18__generated,
            mc_manager_set_slot,
            32,
            &|c, seed| {
                let mut r = Rng(0x23fb_18 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 7,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn mc_manager_set_data_size_wrapper_matches() {
        check(
            ee::s06::f_0023cc58__generated,
            mc_manager_set_data_size_wrapper,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0xcc58, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 0x200,
                    _ => 0x8000_0000,
                };
            },
        );
    }

    #[test]
    fn mc_manager_state2f_matches() {
        check(
            ee::s06::f_00240f10__generated,
            mc_manager_state2f,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x40f10, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_read_current_dir_info_matches() {
        check(
            ee::s06::f_00241fb0__generated,
            mc_manager_read_current_dir_info,
            40,
            &|c, seed| {
                let _ = mgr(c, seed, 0x41fb0, FAKE);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x42c,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 5,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_get_instance_matches() {
        use crate::game::dirtysock::tagsunk::tests::alloc_ready;
        use crate::game::movie::movieplayer::tests::allocator_mutex;
        check(
            ee::s06::f_0023c808__generated,
            mc_manager_get_instance,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23c808, 0);
                alloc_ready(c);
                allocator_mutex(c);
                // Null slot takes the create path, set slot returns it.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x47c),
                    match seed % 3 {
                        0 => 0,
                        1 => DATA + 0x500,
                        _ => r.next_u32() & 0x00ff_ffff,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_set_data_size_matches() {
        check(
            ee::s06::f_0023caa0__generated,
            mc_manager_set_data_size,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0xcaa0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 0x200,
                    _ => 0x8000_0000,
                };
            },
        );
    }

    #[test]
    fn mc_manager_prev_file_matches() {
        check(
            ee::s06::f_00241678__generated,
            mc_manager_prev_file,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x41678, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0xf8, (seed % 5) as u32);
                w32(c, this + 0x100, if seed % 3 == 0 { 0 } else { 1 });
                w32(c, this + 0xec, (seed % 4) as u32);
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x220, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_next_file_matches() {
        check(
            ee::s06::f_002416c0__generated,
            mc_manager_next_file,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x416c0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0xf8, (seed % 5) as u32);
                w32(c, this + 0x100, if seed % 3 == 0 { 0 } else { 1 });
                w32(c, this + 0xec, (seed % 4) as u32);
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x220, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_write_save_by_name_matches() {
        check(
            ee::s06::f_00240fb0__generated,
            mc_manager_write_save_by_name,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x40fb0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x120, FAKE);
                c.g[A3][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0xabcd,
                };
            },
        );
    }

    #[test]
    fn mc_manager_write_save_by_slot_matches() {
        check(
            ee::s06::f_00241050__generated,
            mc_manager_write_save_by_slot,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x41050, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x130, FAKE);
                c.g[A3][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0xabcd,
                };
            },
        );
    }

    #[test]
    fn mc_manager_state31_matches() {
        check(
            ee::s06::f_0023e268__generated,
            mc_manager_state31,
            64,
            &|c, seed| {
                let poll = if seed % 2 == 0 { RET_ZERO } else { FAKE };
                let _ = mgr(c, seed, 0x23e268, poll);
                c.pc = 0x0023_e268;
            },
        );
    }

    #[test]
    fn mc_manager_clamp_file_index_matches() {
        check(
            ee::s06::f_002410d0__generated,
            mc_manager_clamp_file_index,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x410d0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0xec, (seed % 5) as u32);
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x220, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_state0d_matches() {
        check(
            ee::s06::f_0023f308__generated,
            mc_manager_state0d,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23f308, FAKE);
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                w32(c, this + 0xf8, (seed % 3) as u32);
                // First poll done or not, second poll done or not.
                ret_slot(c, vt, 0x218, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x208, if seed % 4 == 0 { RET_ZERO } else { FAKE });
            },
        );
    }

    #[test]
    fn mc_manager_format_slot_free_space_matches() {
        check(
            ee::s06::f_00241928__generated,
            mc_manager_format_slot_free_space,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x41928, 0);
                let idx = (seed % 3) as u32;
                let dev = DATA + idx * 0xe0;
                let dst = DATA + 0x2000;
                let variant = seed / 3 % 5;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(dst);
                c.bus.write32(dev + 0x350, if variant == 0 { 0 } else { 1 });
                c.bus.write32(dev + 0x344, if variant == 1 { 0 } else { 1 });
                c.bus.write32(dev + 0x348, if variant == 2 { 0 } else { 1 });
                c.bus.write32(dev + 0x34c, if variant == 3 { 0 } else { 1 });
                c.bus.write32(dev + 0x354, DATA + 0x3000);
                for i in (0..0x40).step_by(4) {
                    c.bus.write32(DATA + 0x3000 + i, r.next_u32());
                }
                for i in (0..0x100).step_by(4) {
                    c.bus.write32(dst + i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mc_manager_state1e_matches() {
        check(
            ee::s06::f_0023fff0__generated,
            mc_manager_state1e,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23fff0,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                w32(c, DATA + 0x400 + 0x78, FAKE);
                w32(c, DATA + 0x400 + 0x6c, FAKE);
                c.pc = 0x0023_fff0;
            },
        );
    }

    #[test]
    fn mc_manager_state2b_matches() {
        check(
            ee::s06::f_00240cc0__generated,
            mc_manager_state2b,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x40cc0,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                w32(c, DATA + 0x400 + 0x68, FAKE);
                w32(c, DATA + 0x400 + 0x6c, FAKE);
                w32(c, DATA + 0x400 + 0x428, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn mc_manager_state09_matches() {
        check(
            ee::s06::f_002409c8__generated,
            mc_manager_state09,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x409c8,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x248, FAKE);
                w32(c, DATA + 0x400 + 0x70, FAKE);
                w32(c, DATA + 0x400 + 0x6c, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_state11_save_matches() {
        check(
            ee::s06::f_0023f578__generated,
            mc_manager_state11_save,
            64,
            &|c, seed| {
                let mut r = mgr(c, seed, 0x23f578, FAKE);
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x248, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x150, if seed % 4 == 0 { RET_ZERO } else { FAKE });
                w32(c, this + 0x108, (seed % 3) as u32);
                c.bus.write32(this + 0xe4, r.next_u32());
                c.bus.write32(this + 0xf0, r.next_u32());
                w32(c, this + 0x4, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_state2c_delete_matches() {
        check(
            ee::s06::f_00240d90__generated,
            mc_manager_state2c_delete,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x40d90,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x248, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x208, FAKE);
                ret_slot(c, vt, 0x68, FAKE);
                ret_slot(c, vt, 0x50, FAKE);
                w32(c, this + 0x108, (seed % 3) as u32);
                w32(c, this + 0x438, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn mc_manager_state16_matches() {
        check(
            ee::s06::f_0023fd40__generated,
            mc_manager_state16,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23fd40,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                w32(c, this + 0x338, if seed % 3 == 0 { 0 } else { 1 });
                w32(c, this + 0x6c, FAKE);
                w32(c, this + 0x84, FAKE);
                w32(c, this + 0x68, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_begin_save_matches() {
        check(
            ee::s06::f_0023cae8__generated,
            mc_manager_begin_save,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23cae8, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0x200,
                };
            },
        );
    }

    #[test]
    fn mc_manager_prev_slot_matches() {
        check_events(
            ee::s06::f_00241540__generated,
            mc_manager_prev_slot,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x41540, 0);
                globals(c);
                let this = DATA + 0x400;
                zero(c, this, 0x800);
                let holder = DATA + 0x1000;
                let vt = DATA + 0x1100;
                w32(c, this + 0x434, holder);
                w32(c, holder, vt);
                // (idx, count, list) restricted to combos that terminate under an
                // always-zero hasChapter (verified with a stage-level simulator;
                // other combos spin forever even in the generated body).
                // hasChapter returns 0 on even seeds (loop + events) and FAKE on
                // odd seeds (immediate exit).
                let (idx, count, list) = match seed % 8 {
                    0 => (0, -1, 0),
                    1 => (0, 2, 1),
                    2 => (1, 2, 0),
                    3 => (0, 3, 1),
                    4 => (1, 4, 0),
                    5 => (2, 4, 0),
                    6 => (3, 4, 0),
                    _ => (1, 3, 1),
                };
                ret_slot(c, vt, 0x1a0, if seed % 2 == 0 { RET_ZERO } else { FAKE });
                w32(c, this + 0x428, idx as u32);
                w32(
                    c,
                    this + 0x424,
                    if count < 0 { 0xffff_ffff } else { count as u32 },
                );
                w32(c, this + 0x100, list as u32);
                c.g[A0][0] = sx(this);
                schedule(c, &mut r, seed, 0x0024_1540)
            },
        );
    }

    #[test]
    fn mc_manager_begin_load_matches() {
        check(
            ee::s06::f_0023c8f0__generated,
            mc_manager_begin_load,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23c8f0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_format_save_desc_matches() {
        check(
            ee::s06::f_0023cc78__generated,
            mc_manager_format_save_desc,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23cc78, FAKE);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x5000);
                zero(c, DATA + 0x5000, 0x200);
            },
        );
    }

    #[test]
    fn mc_manager_next_slot_matches() {
        check_events(
            ee::s06::f_00241400__generated,
            mc_manager_next_slot,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x41400, 0);
                globals(c);
                let this = DATA + 0x400;
                zero(c, this, 0x800);
                let holder = DATA + 0x1000;
                let vt = DATA + 0x1100;
                w32(c, this + 0x434, holder);
                w32(c, holder, vt);
                // (idx, count, list) restricted to combos that terminate under an
                // always-zero hasChapter (verified with a stage-level simulator;
                // other combos spin forever even in the generated body).
                // hasChapter returns 0 on even seeds (loop + events) and FAKE on
                // odd seeds (immediate exit).
                let (idx, count, list) = match seed % 8 {
                    0 => (0, -1, 0),
                    1 => (0, 2, 1),
                    2 => (1, 2, 0),
                    3 => (0, 3, 1),
                    4 => (1, 4, 0),
                    5 => (2, 4, 0),
                    6 => (3, 4, 0),
                    _ => (1, 3, 1),
                };
                ret_slot(c, vt, 0x1a0, if seed % 2 == 0 { RET_ZERO } else { FAKE });
                w32(c, this + 0x428, idx as u32);
                w32(
                    c,
                    this + 0x424,
                    if count < 0 { 0xffff_ffff } else { count as u32 },
                );
                w32(c, this + 0x100, list as u32);
                c.g[A0][0] = sx(this);
                schedule(c, &mut r, seed, 0x0024_1400)
            },
        );
    }

    #[test]
    fn mc_manager_update_mode1_matches() {
        check(
            ee::s06::f_0023d660__generated,
            mc_manager_update_mode1,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23d660,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x58, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x18, FAKE);
                w32(c, this + 0x338, if seed % 3 == 0 { 0 } else { 1 });
                w32(c, this + 0x424, (seed % 4) as u32);
                w32(
                    c,
                    this + 0x130,
                    match seed % 5 {
                        0 => 0x31,
                        1 => 0x32,
                        2 => 0x33,
                        3 => 0x36,
                        _ => 0x40,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_build_save_title_matches() {
        check(
            ee::s06::f_0023cdb0__generated,
            mc_manager_build_save_title,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23cdb0, FAKE);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_classify_card_state_matches() {
        check(
            ee::s06::f_0023e8a0__generated,
            mc_manager_classify_card_state,
            80,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23e8a0,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                // Steer the cascade: each level done or not.
                let v = seed / 2 % 8;
                ret_slot(c, vt, 0x188, if v & 1 == 0 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x198, if v & 2 == 0 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x1d8, if v & 4 == 0 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x1b8, if seed % 3 == 0 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x1d0, if seed % 3 == 1 { RET_ZERO } else { FAKE });
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
