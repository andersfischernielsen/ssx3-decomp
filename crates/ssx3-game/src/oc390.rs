//! Staging module oc390 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc390`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x002c_55d8, cps2_mem_card_update, "cPS2MemCard_Update")];

// ---------------------------------------------------------------------------
// cPS2MemCard_Update (0x2c55d8)

/// `cPS2MemCard` fields the per-frame update reads and writes.
pub(crate) const MCU_STATE: u32 = 0x4;
pub(crate) const MCU_PORT: u32 = 0xc;
pub(crate) const MCU_PORT2: u32 = 0x10;
pub(crate) const MCU_COUNT: u32 = 0x14;
pub(crate) const MCU_RESULT: u32 = 0x18;
pub(crate) const MCU_SLOT: u32 = 0x1c;
pub(crate) const MCU_SIZE: u32 = 0x20;
pub(crate) const MCU_EXPECT: u32 = 0x24;
pub(crate) const MCU_INDEX: u32 = 0x28;
pub(crate) const MCU_ASYNC_A: u32 = 0x30;
pub(crate) const MCU_ASYNC_B: u32 = 0x34;
pub(crate) const MCU_HANDLE: u32 = 0x38;
pub(crate) const MCU_ACTIVE: u32 = 0x3c;
pub(crate) const MCU_BUSY: u32 = 0x40;
/// Request flags: go to state 0x1e / 0x18 at the next update.
pub(crate) const MCU_REQ_1E: u32 = 0x44;
pub(crate) const MCU_REQ_18: u32 = 0x48;
pub(crate) const MCU_DIR_COUNT: u32 = 0x50;
pub(crate) const MCU_LIST_RESULT: u32 = 0x54;
pub(crate) const MCU_NAME: u32 = 0x58;
pub(crate) const MCU_PATH: u32 = 0x79;
pub(crate) const MCU_DIR: u32 = 0xb9;
/// Per-port info (stride 0x14): +0 type, +4 free, +8 status,
/// +0xc formatted, +0x10 changed.
pub(crate) const MCU_PORTS: u32 = 0x17c;
/// Save-slot entries (stride 0x78): +0..+8 sizes, +0xc name, +0x4c title,
/// +0x6e icon id.
pub(crate) const MCU_ENTRIES: u32 = 0x1a4;
/// Directory listing names (stride 0x40).
pub(crate) const MCU_DIRBUF: u32 = 0x4e0;
pub(crate) const MCU_ICON_ID: u32 = 0x488;
pub(crate) const MCU_CHECK: u32 = 0x490;

/// gp-relative `sceMcGetInfo` / `sceMcSync` outputs.
pub(crate) const MCU_GP_TYPE: u32 = 0x848;
pub(crate) const MCU_GP_FREE: u32 = 0x84c;
pub(crate) const MCU_GP_RESULT: u32 = 0x850;
pub(crate) const MCU_GP_FORMAT: u32 = 0x854;

/// The two state jump tables (.rodata) and the per-state check words.
pub(crate) const MCU_SYNC_TABLE: u32 = 0x0048_6210;
pub(crate) const MCU_STATE_TABLE: u32 = 0x0048_6350;
pub(crate) const MCU_CHECK_TABLE: u32 = 0x0048_7450;
pub(crate) const MCU_PATH_FMT: u32 = 0x004a_3948;
pub(crate) const MCU_PATH_EXT: u32 = 0x0048_7460;
pub(crate) const MCU_SJIS_TABLE: u32 = 0x0053_77b8;

const MCU_MC_SYNC: u32 = 0x0040_a360;
const MCU_MEM_FILL: u32 = 0x003e_6448;
const MCU_STRLEN: u32 = 0x0041_6810;
const MCU_STRCPY: u32 = 0x0041_66f4;
const MCU_SPRINTF: u32 = 0x0041_7828;
const MCU_ASYNC_IS_DONE: u32 = 0x003d_f980;
const MCU_ASYNC_RESULT: u32 = 0x003d_f9d8;

const MCU_FRAME: Frame = Frame { size: 0xb0 };

macro_rules! mcu_callee {
    ($name:ident, $addr:expr, $f:ident) => {
        const $name: Callee = Callee {
            addr: $addr,
            f: ee::s08::$f,
        };
    };
}
mcu_callee!(MCU_IS_MC_ERROR, 0x002c_63e8, cPS2MemCard_IsMcError);
mcu_callee!(MCU_SET_STATE, 0x002c_48c0, cPS2MemCard_SetState);
mcu_callee!(MCU_CLEAR_DIR_ENTRY, 0x002c_52d8, cPS2MemCard_ClearDirEntry);
mcu_callee!(MCU_DO_LIST_DIR, 0x002c_4210, cPS2MemCard_DoListDir);
mcu_callee!(
    MCU_DO_GET_SAVE_DIR_ENTRY,
    0x002c_42b0,
    cPS2MemCard_DoGetSaveDirEntry
);
mcu_callee!(MCU_SJIS_TO_ASCII, 0x002c_2268, MemCard_SjisToAsciiString);
mcu_callee!(
    MCU_MARK_SAVE_INVALID,
    0x002c_5570,
    cPS2MemCard_MarkSaveInvalid
);
mcu_callee!(
    MCU_MARK_SLOTS_WITHOUT_SPACE,
    0x002c_6280,
    cPS2MemCard_MarkSlotsWithoutSpace
);
mcu_callee!(MCU_DO_FORMAT, 0x002c_4388, cPS2MemCard_DoFormat);
mcu_callee!(MCU_DO_DELETE_SAVE, 0x002c_4410, cPS2MemCard_DoDeleteSave);
mcu_callee!(MCU_DO_LIST_DIR_ALL, 0x002c_6848, cPS2MemCard_DoListDirAll);
mcu_callee!(MCU_DO_OPEN, 0x002c_69a0, cPS2MemCard_DoOpen);
mcu_callee!(MCU_DO_CLOSE, 0x002c_6a28, cPS2MemCard_DoClose);
mcu_callee!(MCU_DO_SEEK, 0x002c_6a88, cPS2MemCard_DoSeek);
mcu_callee!(MCU_DO_FLUSH, 0x002c_6b00, cPS2MemCard_DoFlush);
mcu_callee!(MCU_DO_READ, 0x002c_6b78, cPS2MemCard_DoRead);
mcu_callee!(MCU_DO_WRITE, 0x002c_6bf8, cPS2MemCard_DoWrite);
mcu_callee!(MCU_DO_MKDIR, 0x002c_6c88, cPS2MemCard_DoMkdir);
mcu_callee!(MCU_DO_DELETE, 0x002c_6d18, cPS2MemCard_DoDelete);
mcu_callee!(MCU_GET_PORT_ERROR, 0x002c_51d0, cPS2MemCard_GetPortError);
mcu_callee!(MCU_CLEAR_PORT_INFO, 0x002c_5250, cPS2MemCard_ClearPortInfo);
mcu_callee!(MCU_CLEAR_DIR_CACHE, 0x002c_5278, cPS2MemCard_ClearDirCache);

/// `self` (s1) field `off`.
#[inline]
fn mcu_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S1).wrapping_add(off))
}

/// gp-relative global `off`.
#[inline]
fn mcu_gp(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, GP).wrapping_add(off))
}

/// `rd = sx(rs + s1)` (`addu rd, rs, s1`).
#[inline]
fn mcu_plus_self(c: &mut Cpu, rd: usize, rs: usize) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(r32(c, S1)));
}

/// Where the update continues after its state handler.
#[derive(Clone, Copy)]
enum McuTail {
    /// `0x2c60fc`: s0 already restored, restore the rest and return.
    Epilogue,
    /// `0x2c60f8`: restore s0, then the epilogue.
    Restore,
    /// `0x2c6074`: load the port into a1, then the port checks.
    PortsLoad,
    /// `0x2c6078`: the port checks (a1 = the port).
    Ports,
}

/// `void cPS2MemCard_Update(cPS2MemCard* self)` (0x2c55d8): the memory card's
/// per-frame state machine. Collects the result of a finished `sceMc`
/// request (marking the port changed / unformatted, mapping card errors to
/// -10), polls the pending request of the waiting states, honours the two
/// request flags, then runs the current state's step: start the next
/// request (`Do*`) once the card is idle, match directory entries to save
/// slots, check save headers, finish async file reads, record results.
/// Finally drops the port's cached info / directory on a card error and
/// returns to state 0.
pub fn cps2_mem_card_update(c: &mut Cpu) {
    guest(c, |c| {
        let tail = mcu_body(c)?;
        mcu_tail(c, tail)
    })
}

fn mcu_body(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 19;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S1, 0x90);
    frame.save(c, S2, 0x80);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0xa0);
    c.g[S2][0] = 0x14;
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, S7, 0x30);
    frame.save(c, FP, 0x20);
    frame.save_ra(c, 0x10);
    c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
    c.mult(0, V1, V0, S2);
    mcu_plus_self(c, V0, V1);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0xc)).lw(c);
    let formatted = c.g[V1][0] != 0;
    c.g[S0][0] = 1;
    if !formatted {
        // No formatted card in the port: forget the directory.
        c.cycles += 7;
        mcu_field(c, MCU_DIR_COUNT).sw(c, 0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(MCU_ENTRIES));
        mcu_field(c, MCU_LIST_RESULT).sw(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x2d0;
        call_ptr(c, MCU_MEM_FILL, 0x002c_563c)?;
        c.g[S0][0] = 1;
    }
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_BUSY).lw(c);
    let busy = c.g[V0][0] != 0;
    let one = c.g[S0][0];
    mcu_field(c, MCU_ACTIVE).sw(c, one);
    if busy && mcu_collect_request(c)? {
        return Ok(McuTail::Restore);
    }

    // The states that wait for their `sceMc` request first.
    c.cycles += 5;
    c.g[V0][0] = mcu_field(c, MCU_STATE).lw(c);
    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(2));
    c.g[V1][0] = (c.g[A0][0] < 0x4d) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x48_0000;
    if in_table {
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(MCU_SYNC_TABLE);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let target = r32(c, A0);
        match target {
            0x002c_5794 => {
                if let Some(tail) = mcu_poll_request(c)? {
                    return Ok(tail);
                }
            }
            0x002c_58ac => {}
            _ => {
                c.jump_out(target, 0x002c_55d8, 0x002c_6128);
                return Err(Unwound);
            }
        }
    }

    // 0x2c58ac: the request flags.
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_REQ_1E).lw(c);
    c.g[A0][0] = c.g[S1][0];
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        mcu_field(c, MCU_REQ_1E).sw(c, 0);
        c.g[A1][0] = 0x1e;
        MCU_SET_STATE.call(c, 0x002c_58c4)?;
        MCU_FRAME.restore(c, S0, 0xa0);
        return Ok(McuTail::Epilogue);
    }
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_REQ_18).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        mcu_field(c, MCU_REQ_18).sw(c, 0);
        c.g[A1][0] = 0x18;
        MCU_SET_STATE.call(c, 0x002c_58e4)?;
        MCU_FRAME.restore(c, S0, 0xa0);
        return Ok(McuTail::Epilogue);
    }
    c.g[V0][0] = mcu_field(c, MCU_STATE).lw(c);

    // 0x2c58ec: the state's step.
    c.cycles += 4;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = (c.g[V0][0] < 0x4f) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[V0][0] = 0x48_0000;
    if !in_table {
        return Ok(McuTail::PortsLoad);
    }
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A1) << 2);
    c.g[V0][0] = sx(MCU_STATE_TABLE);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    let target = r32(c, A0);
    mcu_step(c, target)
}

/// 0x2c564c..0x2c5768: the pending request finished? Store its result in
/// the port info. True when `sceMcSync` says it is still running (return).
fn mcu_collect_request(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 8;
    mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
    c.g[A0][0] = 1;
    let sp0 = sp(c);
    Addr(sp0).sw(c, 0);
    c.g[A2][0] = sx(r32(c, GP).wrapping_add(MCU_GP_RESULT));
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, MCU_MC_SYNC, 0x002c_5664)?;
    let running = c.g[V0][0] == 0;
    c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    if running {
        return Ok(true);
    }
    // -1 / -2: the card changed.
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    let mut changed = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = sx(0xffff_fffe);
    if !changed {
        c.cycles += 2;
        changed = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x13));
    }
    if changed {
        c.cycles += 6;
        c.g[V0][0] = mcu_field(c, MCU_PORT2).lw(c);
        c.mult(0, V1, V0, S2);
        mcu_plus_self(c, V0, V1);
        let s0 = c.g[S0][0];
        Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x10)).sw(c, s0);
        c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x13));
    }
    // Results in -19..-11, -49..-40, -59..-50, -79..-70 become -10.
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 9) as u64;
    let mut card_error = c.g[V0][0] != 0;
    c.g[V0][0] = sx(0xffff_fff6);
    for bias in [0x31u32, 0x3b, 0x4f] {
        if card_error {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(bias));
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        card_error = c.g[V0][0] != 0;
        c.g[V0][0] = sx(0xffff_fff6);
    }
    if card_error {
        c.cycles += 1;
        let v0 = c.g[V0][0];
        mcu_gp(c, MCU_GP_RESULT).sw(c, v0);
    }

    c.cycles += 5;
    c.g[A1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    c.g[A0][0] = c.g[S1][0];
    MCU_IS_MC_ERROR.call(c, 0x002c_56e4)?;
    if c.g[V0][0] == 0 {
        // Not a card error: no PS2 card is -10002, unformatted -2.
        c.cycles += 4;
        c.g[V1][0] = mcu_gp(c, MCU_GP_TYPE).lw(c);
        c.g[V0][0] = 2;
        let not_ps2 = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = sx(0xffff_d8ee);
        let mut store = not_ps2;
        if !not_ps2 {
            c.cycles += 3;
            c.g[V0][0] = mcu_gp(c, MCU_GP_FORMAT).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(0xffff_fffe);
                store = true;
            }
        }
        if store {
            c.cycles += 2;
            let v0 = c.g[V0][0];
            mcu_gp(c, MCU_GP_RESULT).sw(c, v0);
        }
    }
    c.g[V1][0] = mcu_field(c, MCU_PORT2).lw(c);

    // 0x2c5714: record the request's port info.
    c.cycles += 21;
    c.g[A0][0] = 0x14;
    c.g[A1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    c.mult(0, V0, V1, A0);
    mcu_field(c, MCU_BUSY).sw(c, 0);
    c.g[A2][0] = mcu_gp(c, MCU_GP_TYPE).lw(c);
    c.g[A3][0] = mcu_gp(c, MCU_GP_FREE).lw(c);
    c.g[T0][0] = mcu_gp(c, MCU_GP_FORMAT).lw(c);
    mcu_plus_self(c, V1, V0);
    let a1 = c.g[A1][0];
    Addr(r32(c, V1).wrapping_add(MCU_PORTS + 0x8)).sw(c, a1);
    c.g[V0][0] = mcu_field(c, MCU_PORT2).lw(c);
    c.mult(0, V1, V0, A0);
    mcu_plus_self(c, V0, V1);
    let a2 = c.g[A2][0];
    Addr(r32(c, V0).wrapping_add(MCU_PORTS)).sw(c, a2);
    c.g[V1][0] = mcu_field(c, MCU_PORT2).lw(c);
    c.mult(0, V0, V1, A0);
    mcu_plus_self(c, V1, V0);
    let a3 = c.g[A3][0];
    Addr(r32(c, V1).wrapping_add(MCU_PORTS + 0x4)).sw(c, a3);
    c.g[V0][0] = mcu_field(c, MCU_PORT2).lw(c);
    c.mult(0, V1, V0, A0);
    mcu_plus_self(c, V0, V1);
    let t0 = c.g[T0][0];
    Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0xc)).sw(c, t0);
    Ok(false)
}

/// 0x2c5794..0x2c58ac: a waiting state polls its request (`sceMcSync`
/// without a command). Errors end the operation (state 1); some states
/// accept "no such file" (-4).
fn mcu_poll_request(c: &mut Cpu) -> Result<Option<McuTail>, Unwound> {
    c.cycles += 8;
    mcu_field(c, MCU_ACTIVE).sw(c, 0);
    c.g[A0][0] = 1;
    mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
    c.g[A2][0] = sx(r32(c, GP).wrapping_add(MCU_GP_RESULT));
    c.g[A1][0] = 0;
    call_ptr(c, MCU_MC_SYNC, 0x002c_57ac)?;
    let running = c.g[V0][0] == 0;
    c.g[A0][0] = 1;
    if running {
        return Ok(Some(McuTail::Restore));
    }
    c.cycles += 4;
    c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    c.g[A2][0] = u64::MAX;
    let changed = c.g[V0][0] == c.g[A2][0];
    let a0 = c.g[A0][0];
    mcu_field(c, MCU_ACTIVE).sw(c, a0);
    if changed {
        c.cycles += 5;
        c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
        c.g[V1][0] = 0x14;
        c.mult(0, A1, V0, V1);
        mcu_plus_self(c, V0, A1);
        let a0 = c.g[A0][0];
        Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x10)).sw(c, a0);
    }
    c.cycles += 3;
    c.g[A1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 4;
        c.g[V1][0] = mcu_field(c, MCU_STATE).lw(c);
        c.g[V0][0] = 0x24;
        let is_36 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x36));
        // Whether -4 counts as success (states 36 and 54..59).
        let mut allow_missing = is_36;
        if is_36 {
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_fffc);
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 6) as u64;
            allow_missing = c.g[V0][0] != 0;
            c.g[V0][0] = sx(0xffff_fffc);
        }
        if allow_missing {
            c.cycles += 2;
            if c.g[A1][0] == c.g[V0][0] {
                mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
        } else {
            // States 8..15: a card error invalidates the save being read.
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, V1).wrapping_sub(8));
            c.g[V0][0] = (c.g[V0][0] < 8) as u64;
            let reading = c.g[V0][0] != 0;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            if reading {
                c.cycles += 2;
                if c.g[A1][0] != c.g[A2][0] {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    MCU_IS_MC_ERROR.call(c, 0x002c_5820)?;
                    let card_error = c.g[V0][0] != 0;
                    c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
                    if !card_error {
                        c.cycles += 8;
                        mcu_vcall_slot(c, V0, V1, 0x140, 0x002c_5840)?;
                        c.g[A0][0] = c.g[S1][0];
                        return mcu_invalid_exit(c).map(Some);
                    }
                }
            }
        }
        // 0x2c5858
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 4;
            c.g[V1][0] = mcu_field(c, MCU_STATE).lw(c);
            c.g[V0][0] = 0x23;
            let mut clear = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 7;
            if !clear {
                c.cycles += 2;
                clear = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x13;
            }
            if clear {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                MCU_CLEAR_DIR_ENTRY.call(c, 0x002c_5880)?;
                c.g[V1][0] = mcu_field(c, MCU_STATE).lw(c);
                c.g[V0][0] = 0x13;
            }
            c.cycles += 2;
            let is_19 = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if is_19 {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                mcu_field(c, MCU_SLOT).sw(c, u64::MAX);
            }
            return mcu_fail_exit(c).map(Some);
        }
    }
    c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
    // 0x2c589c: the port's status is clear.
    c.cycles += 4;
    c.g[V1][0] = 0x14;
    c.mult(0, A0, V0, V1);
    mcu_plus_self(c, V0, A0);
    Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x8)).sw(c, 0);
    Ok(None)
}

/// A virtual call on `self` through vtable slot `off` with the index:
/// `vt` = the vtable register, `func` = the function register.
fn mcu_vcall_slot(c: &mut Cpu, vt: usize, func: usize, off: u32, ret: u32) -> Flow {
    c.g[vt][0] = mcu_field(c, 0).lw(c);
    c.g[A1][0] = mcu_field(c, MCU_INDEX).lw(c);
    c.g[A0][0] = Addr(r32(c, vt).wrapping_add(off)).lh(c);
    c.g[func][0] = Addr(r32(c, vt).wrapping_add(off + 4)).lw(c);
    let target = r32(c, func);
    mcu_plus_self(c, A0, A0);
    call_ptr(c, target, ret)
}

/// The vtable's "card busy" query (slot 0x248), 7 cycles. True when busy.
fn mcu_card_busy(c: &mut Cpu, ret: u32) -> Result<bool, Unwound> {
    c.cycles += 7;
    c.g[V1][0] = mcu_field(c, 0).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x248)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x24c)).lw(c);
    let target = r32(c, V0);
    mcu_plus_self(c, A0, A0);
    call_ptr(c, target, ret)?;
    Ok(c.g[V0][0] != 0)
}

/// Busy: load the port and go to the port checks.
fn mcu_to_ports(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
    Ok(McuTail::Ports)
}

/// `cPS2MemCard_SetState(self, state)` and return (`cycles` for the block).
fn mcu_set_state_exit(c: &mut Cpu, cycles: u64, state: u64, ret: u32) -> Result<McuTail, Unwound> {
    c.cycles += cycles;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = state;
    MCU_SET_STATE.call(c, ret)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// A `Do*` request starter on `self`, then return.
fn mcu_do_exit(c: &mut Cpu, f: Callee, ret: u32) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    f.call(c, ret)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5bb4: the save is invalid, state 0x10 (a0 = self already).
fn mcu_invalid_exit(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A1][0] = 0x10;
    MCU_SET_STATE.call(c, 0x002c_5bbc)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5db4: the operation failed, state 1 (a0 = self already).
fn mcu_fail_exit(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A1][0] = 1;
    MCU_SET_STATE.call(c, 0x002c_5dbc)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5d48: on to the next state (a1 = the state, a0 = self).
fn mcu_next_state_exit(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    MCU_SET_STATE.call(c, 0x002c_5d50)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// Busy query, then a `Do*` request once the card is idle.
fn mcu_when_idle_do(c: &mut Cpu, busy_ret: u32, f: Callee, ret: u32) -> Result<McuTail, Unwound> {
    if mcu_card_busy(c, busy_ret)? {
        return mcu_to_ports(c);
    }
    mcu_do_exit(c, f, ret)
}

/// Busy query, then (with a non-negative result) a `Do*` file request.
fn mcu_when_idle_ok_do(
    c: &mut Cpu,
    busy_ret: u32,
    f: Callee,
    ret: u32,
) -> Result<McuTail, Unwound> {
    if mcu_card_busy(c, busy_ret)? {
        return mcu_to_ports(c);
    }
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_RESULT).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        return mcu_to_ports(c);
    }
    mcu_do_exit(c, f, ret)
}

/// Busy query, then a state change once the card is idle.
fn mcu_when_idle_set(c: &mut Cpu, busy_ret: u32, state: u64, ret: u32) -> Result<McuTail, Unwound> {
    if mcu_card_busy(c, busy_ret)? {
        return mcu_to_ports(c);
    }
    mcu_set_state_exit(c, 5, state, ret)
}

/// The state's step (the second jump table's `target`; a1 = the state).
fn mcu_step(c: &mut Cpu, target: u32) -> Result<McuTail, Unwound> {
    match target {
        // 2: wait for the request, flag a changed card.
        0x002c_5914 => {
            c.cycles += 7;
            mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
            c.g[A0][0] = 1;
            c.g[A2][0] = sx(r32(c, GP).wrapping_add(MCU_GP_RESULT));
            c.g[A1][0] = 0;
            call_ptr(c, MCU_MC_SYNC, 0x002c_5928)?;
            let running = c.g[V0][0] == 0;
            c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            if running {
                return Ok(McuTail::Restore);
            }
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] != c.g[V0][0] {
                return mcu_to_ports(c);
            }
            c.cycles += 7;
            c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
            c.g[V1][0] = 0x14;
            c.g[A0][0] = 1;
            c.mult(0, A1, V0, V1);
            mcu_plus_self(c, V0, A1);
            Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x10)).sw(c, 1);
            Ok(McuTail::PortsLoad)
        }
        // 3: go to 4.
        0x002c_5958 => {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 4;
            MCU_SET_STATE.call(c, 0x002c_5964)?;
            mcu_to_ports(c)
        }
        0x002c_596c => mcu_when_idle_do(c, 0x002c_5980, MCU_DO_LIST_DIR, 0x002c_5990),
        0x002c_5998 => mcu_when_idle_do(c, 0x002c_59ac, MCU_DO_GET_SAVE_DIR_ENTRY, 0x002c_59bc),
        // 6: match the directory listing to the save slots.
        0x002c_59c4 => mcu_match_slots(c),
        // 8: keep the result, go to 10.
        0x002c_5a98 => {
            c.cycles += 7;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0xa;
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_RESULT).sw(c, v0);
            MCU_SET_STATE.call(c, 0x002c_5aac)?;
            MCU_FRAME.restore(c, S0, 0xa0);
            Ok(McuTail::Epilogue)
        }
        // 10: convert the slot's title from Shift-JIS, go to 9.
        0x002c_5ab4 => {
            c.cycles += 14;
            c.g[A1][0] = mcu_field(c, MCU_INDEX).lw(c);
            c.g[V0][0] = 0x78;
            c.g[A2][0] = 0x53_0000;
            c.g[A0][0] = c.g[S1][0];
            c.mult(0, V1, A1, V0);
            c.g[A2][0] = sx(MCU_SJIS_TABLE);
            mcu_plus_self(c, A1, V1);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(MCU_ENTRIES + 0x4c));
            MCU_SJIS_TO_ASCII.call(c, 0x002c_5ad8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 9;
            MCU_SET_STATE.call(c, 0x002c_5ae4)?;
            MCU_FRAME.restore(c, S0, 0xa0);
            Ok(McuTail::Epilogue)
        }
        // 9: go to 11.
        0x002c_5aec => mcu_set_state_exit(c, 5, 0xb, 0x002c_5af8),
        // 11: check the header, copy the icon id, go to 12.
        0x002c_5b00 => {
            c.cycles += 4;
            c.g[V1][0] = mcu_field(c, MCU_CHECK).lw(c);
            c.g[V0][0] = mcu_field(c, MCU_EXPECT).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 10;
                mcu_vcall_slot(c, V0, V1, 0x140, 0x002c_5b28)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x10;
                MCU_SET_STATE.call(c, 0x002c_5b34)?;
            }
            c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
            c.cycles += 12;
            c.g[V1][0] = 0x78;
            c.g[A0][0] = c.g[S1][0];
            c.mult(0, A2, V0, V1);
            mcu_plus_self(c, V0, A2);
            let s1 = r32(c, S1);
            c.ldl(A3, s1.wrapping_add(MCU_ICON_ID + 7));
            c.ldr(A3, s1.wrapping_add(MCU_ICON_ID));
            let dst = r32(c, V0);
            c.sdl(A3, dst.wrapping_add(MCU_ENTRIES + 0x6e + 7));
            c.sdr(A3, dst.wrapping_add(MCU_ENTRIES + 0x6e));
            c.g[A1][0] = 0xc;
            MCU_SET_STATE.call(c, 0x002c_5b60)?;
            MCU_FRAME.restore(c, S0, 0xa0);
            Ok(McuTail::Epilogue)
        }
        // 12, 13, 14: the check word must match the state's constant.
        0x002c_5b68 => {
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            let is_13 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = c.g[A1][0] ^ 0xe;
            if is_13 {
                c.cycles += 1;
                c.g[V1][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { 2 };
            }
            c.cycles += 8;
            c.g[V0][0] = 0x48_0000;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(MCU_CHECK_TABLE);
            c.g[A1][0] = mcu_field(c, MCU_CHECK).lw(c);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            if c.g[A1][0] == c.g[A0][0] {
                c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
                return mcu_next_slot(c);
            }
            c.cycles += 4;
            c.g[A1][0] = mcu_field(c, MCU_INDEX).lw(c);
            c.g[A0][0] = c.g[S1][0];
            MCU_MARK_SAVE_INVALID.call(c, 0x002c_5bb0)?;
            c.g[A0][0] = c.g[S1][0];
            mcu_invalid_exit(c)
        }
        // 15, 16: on to the next slot.
        0x002c_5be8 => {
            c.cycles += 1;
            c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
            mcu_next_slot(c)
        }
        // 7: all slots read.
        0x002c_5c00 => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            MCU_MARK_SLOTS_WITHOUT_SPACE.call(c, 0x002c_5c08)?;
            mcu_to_ports(c)
        }
        // 35: unless the result is 1, clear the directory entry.
        0x002c_5c10 => {
            c.cycles += 4;
            c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                MCU_CLEAR_DIR_ENTRY.call(c, 0x002c_5c28)?;
            }
            mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
            Ok(McuTail::PortsLoad)
        }
        // 17: format an unformatted card.
        0x002c_5c30 => {
            c.cycles += 7;
            c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
            c.g[V1][0] = 0x14;
            c.mult(0, A0, V0, V1);
            mcu_plus_self(c, V0, A0);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0xc)).lw(c);
            if c.g[A0][0] != 0 {
                return mcu_to_ports(c);
            }
            mcu_do_exit(c, MCU_DO_FORMAT, 0x002c_5c54)
        }
        0x002c_5c5c => mcu_when_idle_do(c, 0x002c_5c70, MCU_DO_DELETE_SAVE, 0x002c_5c80),
        // 40, 44, 48: wait for the async file, take its result, next state.
        0x002c_5c88 => {
            c.cycles += 5;
            c.g[A0][0] = mcu_field(c, MCU_HANDLE).lw(c);
            call_ptr(c, MCU_ASYNC_IS_DONE, 0x002c_5c90)?;
            c.g[V1][0] = 1;
            let pending = c.g[V0][0] != c.g[V1][0];
            MCU_FRAME.restore(c, S0, 0xa0);
            if pending {
                return Ok(McuTail::Epilogue);
            }
            c.cycles += 8;
            c.g[A0][0] = mcu_field(c, MCU_HANDLE).lw(c);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(MCU_ASYNC_A));
            mcu_field(c, MCU_ASYNC_A).sw(c, 0);
            c.g[A2][0] = sx(r32(c, S1).wrapping_add(MCU_ASYNC_B));
            mcu_field(c, MCU_ASYNC_B).sw(c, 0);
            call_ptr(c, MCU_ASYNC_RESULT, 0x002c_5cb4)?;
            c.g[A1][0] = mcu_field(c, MCU_STATE).lw(c);
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            mcu_next_state_exit(c)
        }
        // 22, 28, 37, 41, 45, 49: keep the result, next state.
        0x002c_5cbc => {
            c.cycles += 5;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = mcu_field(c, MCU_STATE).lw(c);
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_RESULT).sw(c, v0);
            mcu_next_state_exit(c)
        }
        0x002c_5cd0 => mcu_when_idle_set(c, 0x002c_5ce4, 0x24, 0x002c_5cf8),
        0x002c_5d00 => mcu_when_idle_set(c, 0x002c_5d14, 0x1c, 0x002c_5d28),
        0x002c_5d30 => mcu_set_state_exit(c, 5, 0x3a, 0x002c_5d3c),
        // 36, 38, 39, 42, 46, 50, 54, 57, 58: next state.
        0x002c_5d44 => {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            mcu_next_state_exit(c)
        }
        0x002c_5d58 => mcu_when_idle_set(c, 0x002c_5d6c, 0x16, 0x002c_5d80),
        0x002c_5d88 => mcu_set_state_exit(c, 5, 0x34, 0x002c_5d94),
        // 29: the written size must match.
        0x002c_5d9c => {
            c.cycles += 4;
            c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[V0][0] = mcu_field(c, MCU_SIZE).lw(c);
            let same = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx(0xffff_d8ef);
            if same {
                return mcu_set_state_exit(c, 5, 0x35, 0x002c_5dd0);
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            mcu_gp(c, MCU_GP_RESULT).sw(c, sx(0xffff_d8ef));
            mcu_fail_exit(c)
        }
        // 25: fill in the new save's slot entry.
        0x002c_5dd8 => mcu_fill_slot(c),
        0x002c_5e48 => mcu_when_idle_do(c, 0x002c_5e5c, MCU_DO_LIST_DIR_ALL, 0x002c_5e6c),
        // 61: keep the listing's result.
        0x002c_5e74 => {
            c.cycles += 3;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_LIST_RESULT).sw(c, v0);
            Ok(McuTail::PortsLoad)
        }
        0x002c_5e80 => mcu_when_idle_do(c, 0x002c_5e94, MCU_DO_OPEN, 0x002c_5ea4),
        0x002c_5eac => mcu_when_idle_ok_do(c, 0x002c_5ec0, MCU_DO_CLOSE, 0x002c_5edc),
        0x002c_5ee4 => mcu_when_idle_ok_do(c, 0x002c_5ef8, MCU_DO_SEEK, 0x002c_5f14),
        0x002c_5f1c => mcu_when_idle_ok_do(c, 0x002c_5f30, MCU_DO_FLUSH, 0x002c_5f4c),
        0x002c_5f54 => mcu_when_idle_ok_do(c, 0x002c_5f68, MCU_DO_READ, 0x002c_5f84),
        0x002c_5f8c => mcu_when_idle_ok_do(c, 0x002c_5fa0, MCU_DO_WRITE, 0x002c_5fbc),
        0x002c_5fc4 => mcu_when_idle_do(c, 0x002c_5fd8, MCU_DO_MKDIR, 0x002c_5fe8),
        0x002c_5ff0 => mcu_when_idle_do(c, 0x002c_6004, MCU_DO_DELETE, 0x002c_6014),
        // 64: done with the request's result; 74: done with -1.
        0x002c_601c | 0x002c_6024 => {
            if target == 0x002c_601c {
                c.cycles += 2;
                c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            } else {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
            }
            c.cycles += 3;
            mcu_field(c, MCU_STATE).sw(c, 0);
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_RESULT).sw(c, v0);
            Ok(McuTail::PortsLoad)
        }
        // 66, 68, 70, 72: done.
        0x002c_6070 => {
            c.cycles += 1;
            mcu_field(c, MCU_STATE).sw(c, 0);
            Ok(McuTail::PortsLoad)
        }
        // 0, 1, 4, ...: nothing to do.
        0x002c_6074 => Ok(McuTail::PortsLoad),
        // 52, 53: return at once.
        0x002c_60f8 => Ok(McuTail::Restore),
        _ => {
            c.jump_out(target, 0x002c_55d8, 0x002c_6128);
            Err(Unwound)
        }
    }
}

/// State 6 (0x2c59c4): clear the slot entries and give slot `i` the
/// directory entry whose name ends in the digit `i + 1`; then state 7.
fn mcu_match_slots(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 9;
    c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(MCU_ENTRIES));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x2d0;
    let count = c.g[V0][0];
    mcu_field(c, MCU_DIR_COUNT).sw(c, count);
    call_ptr(c, MCU_MEM_FILL, 0x002c_59dc)?;
    c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 2;
        c.g[FP][0] = 0x78;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x31));
        loop {
            // 0x2c59f0: slot a0, digit s6.
            c.cycles += 5;
            c.g[V1][0] = mcu_field(c, MCU_DIR_COUNT).lw(c);
            c.g[S6][0] = c.g[V0][0] & 0xff;
            c.g[S2][0] = 0;
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[S7][0] = sx(r32(c, A0).wrapping_add(1));
            if empty {
                c.cycles += 1;
                c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
            } else {
                c.cycles += 7;
                c.mult(0, V0, A0, FP);
                c.g[S4][0] = sx(r32(c, S1).wrapping_add(MCU_DIRBUF));
                c.g[S3][0] = u64::MAX;
                c.g[S0][0] = c.g[S4][0];
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(MCU_ENTRIES));
                mcu_plus_self(c, S5, V0);
                loop {
                    // 0x2c5a20: directory entry s2 (name at s0).
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, MCU_STRLEN, 0x002c_5a28)?;
                    let blank = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                    if blank {
                        c.cycles += 1;
                        c.g[V0][0] = mcu_field(c, MCU_DIR_COUNT).lw(c);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                        c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                        if c.g[V1][0] != c.g[S6][0] {
                            c.g[V0][0] = mcu_field(c, MCU_DIR_COUNT).lw(c);
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = sx(r32(c, S5).wrapping_add(0xc));
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, MCU_STRCPY, 0x002c_5a4c)?;
                            c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
                            break;
                        }
                    }
                    // 0x2c5a58
                    c.cycles += 5;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x40));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002c_5a20);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
                    break;
                }
            }
            // 0x2c5a70
            c.cycles += 4;
            c.g[A0][0] = c.g[S7][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x31));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_59f0);
            }
        }
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
    c.g[A1][0] = 7;
    MCU_SET_STATE.call(c, 0x002c_5a90)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5bec: advance to the next slot that holds a save (vtable slot
/// 0x218) and build its path (state 8); past the last slot, state 7's
/// free-space marking. v0 = the current index.
fn mcu_next_slot(c: &mut Cpu) -> Result<McuTail, Unwound> {
    loop {
        c.cycles += 5;
        c.g[V1][0] = mcu_field(c, MCU_COUNT).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        let index = c.g[V0][0];
        mcu_field(c, MCU_INDEX).sw(c, index);
        if !more {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            MCU_MARK_SLOTS_WITHOUT_SPACE.call(c, 0x002c_5c08)?;
            return mcu_to_ports(c);
        }
        if event_due(c) {
            deliver_event_here(c, 0x002c_5bc8);
        }
        // 0x2c5bc8
        c.cycles += 8;
        mcu_vcall_slot(c, V1, V0, 0x218, 0x002c_5be0)?;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = mcu_field(c, MCU_INDEX).lw(c);
            return mcu_build_path(c);
        }
        c.cycles += 1;
        c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
    }
}

/// 0x2c6034: `sprintf(self->path, fmt, entries[a2].name, ext)`, state 8.
fn mcu_build_path(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 15;
    c.g[V0][0] = 0x78;
    c.g[A1][0] = 0x4a_0000;
    c.g[A3][0] = 0x48_0000;
    c.mult(0, V1, A2, V0);
    c.g[A1][0] = sx(MCU_PATH_FMT);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(MCU_PATH));
    c.g[A3][0] = sx(MCU_PATH_EXT);
    mcu_plus_self(c, A2, V1);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(MCU_ENTRIES + 0xc));
    call_ptr(c, MCU_SPRINTF, 0x002c_605c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 8;
    MCU_SET_STATE.call(c, 0x002c_6068)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// State 25 (0x2c5dd8): the new save's slot entry gets zero sizes, the
/// directory name and the title.
fn mcu_fill_slot(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 28;
    c.g[V1][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.g[S0][0] = 0x78;
    mcu_field(c, MCU_ACTIVE).sw(c, 0);
    c.g[A1][0] = sx(r32(c, S1).wrapping_add(MCU_DIR));
    c.mult(0, V0, V1, S0);
    mcu_plus_self(c, V1, V0);
    Addr(r32(c, V1).wrapping_add(MCU_ENTRIES + 4)).sw(c, 0);
    c.g[V0][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.mult(0, V1, V0, S0);
    mcu_plus_self(c, V0, V1);
    Addr(r32(c, V0).wrapping_add(MCU_ENTRIES)).sw(c, 0);
    c.g[V1][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.mult(0, V0, V1, S0);
    mcu_plus_self(c, V1, V0);
    Addr(r32(c, V1).wrapping_add(MCU_ENTRIES + 8)).sw(c, 0);
    c.g[A0][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.mult(0, V0, A0, S0);
    mcu_plus_self(c, A0, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MCU_ENTRIES + 0xc));
    call_ptr(c, MCU_STRCPY, 0x002c_5e28)?;
    c.g[A0][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.g[A1][0] = sx(r32(c, S1).wrapping_add(MCU_NAME));
    c.mult(0, V0, A0, S0);
    mcu_plus_self(c, A0, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MCU_ENTRIES + 0x4c));
    call_ptr(c, MCU_STRCPY, 0x002c_5e40)?;
    mcu_to_ports(c)
}

/// The shared tails: port-error checks (drop the port's info on -10/-1,
/// its directory cache on -10/-2, back to state 0), then the epilogue.
fn mcu_tail(c: &mut Cpu, mut tail: McuTail) -> Flow {
    if let McuTail::PortsLoad = tail {
        c.cycles += 1;
        c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
        tail = McuTail::Ports;
    }
    if let McuTail::Ports = tail {
        for (first, second, wanted, clear, clear_ret) in [
            (
                0x002c_6080,
                0x002c_6098,
                u64::MAX,
                MCU_CLEAR_PORT_INFO,
                0x002c_60ac,
            ),
            (
                0x002c_60b8,
                0x002c_60d0,
                sx(0xffff_fffe),
                MCU_CLEAR_DIR_CACHE,
                0x002c_60e4,
            ),
        ] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            MCU_GET_PORT_ERROR.call(c, first)?;
            c.g[V1][0] = sx(0xffff_fff6);
            let mut hit = c.g[V0][0] == c.g[V1][0];
            if !hit {
                c.cycles += 6;
                c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
                c.g[A0][0] = c.g[S1][0];
                MCU_GET_PORT_ERROR.call(c, second)?;
                c.g[V1][0] = wanted;
                hit = c.g[V0][0] == c.g[V1][0];
            }
            if hit {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                clear.call(c, clear_ret)?;
            }
            if clear_ret == 0x002c_60ac {
                c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
            } else {
                c.g[V0][0] = mcu_field(c, MCU_STATE).lw(c);
            }
        }
        c.cycles += 2;
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !idle {
            c.cycles += 2;
            c.g[A1][0] = 0;
            MCU_SET_STATE.call(c, 0x002c_60f8)?;
        }
        tail = McuTail::Restore;
    }
    if let McuTail::Restore = tail {
        c.cycles += 1;
        MCU_FRAME.restore(c, S0, 0xa0);
    }
    c.cycles += 11;
    MCU_FRAME.restore(c, S1, 0x90);
    MCU_FRAME.restore(c, S2, 0x80);
    MCU_FRAME.restore(c, S3, 0x70);
    MCU_FRAME.restore(c, S4, 0x60);
    MCU_FRAME.restore(c, S5, 0x50);
    MCU_FRAME.restore(c, S6, 0x40);
    MCU_FRAME.restore(c, S7, 0x30);
    MCU_FRAME.restore(c, FP, 0x20);
    MCU_FRAME.restore_ra(c, 0x10);
    MCU_FRAME.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, put_words, DATA};

    /// A leaf that returns 0 (the vtable's "not busy" / "no save").
    const MCU_T_RET0: u32 = 0x0014_06c0;
    /// `cPS2MemCard_SetState`'s jump table (zeroed: its calls become fakes).
    const MCU_T_SET_STATE_TABLE: u32 = 0x0048_60c0;
    /// `sceMcSync` state: the pending command, the RPC header, the result.
    const MCU_T_MC_CMD: u32 = 0x0045_33c0;
    const MCU_T_RPC: u32 = 0x0052_71c0;
    const MCU_T_MC_RESULT: u32 = 0x0051_8780;
    /// The async-file table pointer and count.
    const MCU_T_ASYNC_TABLE: u32 = 0x0045_0c34;
    const MCU_T_ASYNC_COUNT: u32 = 0x0051_9bf0;

    fn mcu_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// A memory card object at `DATA` in state `seed % 83`, with the
    /// `sceMcSync` result, the port infos, the directory listing, the
    /// vtable's busy / has-save answers and the async file chosen per seed.
    fn mcu_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c55d8, 4);
        elf_image(c);
        put_words(c, MCU_T_SET_STATE_TABLE, &[0; 0x4e]);
        c.pc = 0x002c_55d8;
        let me = DATA;
        c.g[A0][0] = me as u64;
        let state = seed % 83;
        let v = seed / 83;
        let w = |c: &mut Cpu, off: u32, x: u32| c.bus.write32(me + off, x);

        // The vtable: 0x140 (invalidate), 0x218 (slot has a save), 0x248 (busy).
        let vt = DATA + 0x4000;
        w(c, 0, vt);
        c.bus.write16(vt + 0x140, (r.next_u32() & 0x18) as u16);
        c.bus.write32(vt + 0x144, FAKE);
        c.bus.write16(vt + 0x218, 0);
        let has_save = if r.next_u32() % 2 == 0 {
            FAKE + 0x10
        } else {
            MCU_T_RET0
        };
        c.bus.write32(vt + 0x21c, has_save);
        c.bus.write16(vt + 0x248, 0);
        let busy = if r.next_u32().wrapping_add(v) % 3 == 0 {
            FAKE + 0x20
        } else {
            MCU_T_RET0
        };
        c.bus.write32(vt + 0x24c, busy);

        w(c, MCU_STATE, state);
        w(c, MCU_PORT, r.next_u32() % 2);
        w(c, MCU_PORT2, r.next_u32() % 2);
        let count = r.next_u32() % 5;
        w(c, MCU_COUNT, count);
        let result = [0u32, 5, 0xffff_ffff][(r.next_u32() % 3) as usize];
        w(c, MCU_RESULT, result);
        w(c, MCU_SLOT, r.next_u32() % 4);
        let written = r.next_u32() % 0x2000;
        w(c, MCU_SIZE, written);
        w(c, MCU_EXPECT, 0x11db8);
        let index = (r.next_u32() % (count + 1)).wrapping_sub(1);
        w(c, MCU_INDEX, index);
        w(c, MCU_BUSY, (r.next_u32() % 2 == 0) as u32);
        w(c, MCU_REQ_1E, (r.next_u32() % 13 == 0) as u32);
        w(c, MCU_REQ_18, (r.next_u32() % 13 == 0) as u32);
        w(c, MCU_DIR_COUNT, r.next_u32() % 4);
        w(c, MCU_LIST_RESULT, r.next_u32());
        let check_word = [0x11db8u32, 0, 7][(r.next_u32() % 3) as usize];
        w(c, MCU_CHECK, check_word);
        c.bus.write32(me + MCU_ICON_ID, r.next_u32());
        c.bus.write32(me + MCU_ICON_ID + 4, r.next_u32());
        mcu_put_str(c, me + MCU_NAME, b"SSX 3 save");
        mcu_put_str(c, me + MCU_DIR, b"BASLUS-20772SSX3");
        for p in 0..2u32 {
            let port = me + MCU_PORTS + p * 0x14;
            let status =
                [0u32, 0xffff_fff6, 0xffff_ffff, 0xffff_fffe, 3][(r.next_u32() % 5) as usize];
            c.bus.write32(port, 2);
            c.bus.write32(port + 4, r.next_u32() % 0x2000);
            c.bus.write32(port + 8, status);
            c.bus.write32(port + 0xc, (r.next_u32() % 3 != 0) as u32);
            c.bus.write32(port + 0x10, 0);
        }
        // Slot entries and the directory listing (names ending in a digit).
        for i in 0..5u32 {
            let e = me + MCU_ENTRIES + i * 0x78;
            mcu_put_str(c, e + 0xc, b"BASLUS-20772SSX31");
            mcu_put_str(c, e + 0x4c, b"SSX 3");
        }
        for j in 0..5u32 {
            let name = me + MCU_DIRBUF + j * 0x40;
            match r.next_u32() % 4 {
                0 => mcu_put_str(c, name, b""),
                1 => mcu_put_str(c, name, b"BADATA-SYSTEM"),
                _ => {
                    let digit = b'1' + (r.next_u32() % 4) as u8;
                    mcu_put_str(c, name, &[b'B', b'A', b'S', b'S', b'X', digit]);
                }
            }
        }

        // gp globals: card type, free space, formatted, last result.
        let gp = crate::difftest::GAME_GP;
        c.bus
            .write32(gp + MCU_GP_TYPE, if r.next_u32() % 3 == 0 { 1 } else { 2 });
        c.bus.write32(gp + MCU_GP_FREE, r.next_u32() % 0x2000);
        c.bus.write32(gp + MCU_GP_FORMAT, r.next_u32() % 2);
        c.bus.write32(gp + MCU_GP_RESULT, r.next_u32());

        // `sceMcSync`: no command (-1), still running (0), or finished with
        // a result (1).
        let results = [
            0u32,
            1,
            5,
            written,
            0xffff_ffff,
            0xffff_fffe,
            0xffff_fffc,
            0xffff_fff6,
            0xffff_fff1,
            0xffff_ffd3,
            0xffff_ffc9,
            0xffff_ffb5,
            0xffff_ff9c,
        ];
        c.bus
            .write32(MCU_T_MC_RESULT, results[(r.next_u32() % 13) as usize]);
        c.bus.write32(MCU_T_RPC, 0);
        match r.next_u32() % 6 {
            0 => c.bus.write32(MCU_T_MC_CMD, 0),
            1 => {
                c.bus.write32(MCU_T_MC_CMD, 0x10);
                let rpc = DATA + 0x6000;
                c.bus.write32(MCU_T_RPC, rpc);
                c.bus.write32(MCU_T_RPC + 4, 0x77);
                c.bus.write32(rpc + 0x18, 0x77);
                c.bus.write32(rpc + 0x10, 1);
            }
            _ => c.bus.write32(MCU_T_MC_CMD, 0x10 + r.next_u32() % 4),
        }

        // The async file: an unknown handle, pending, or done.
        let table = DATA + 0x5000;
        c.bus.write32(MCU_T_ASYNC_TABLE, table);
        c.bus.write32(MCU_T_ASYNC_COUNT, 4);
        let handle = 0x0300 | 2;
        let entry = table + 2 * 0x30;
        c.bus.write32(entry, handle);
        c.bus.write32(entry + 0x8, 0x40);
        c.bus.write32(entry + 0x10, 1);
        c.bus.write32(entry + 0x1c, (r.next_u32() % 3 == 0) as u32);
        c.bus.write32(entry + 0x2c, 0x140);
        w(
            c,
            MCU_HANDLE,
            if r.next_u32() % 4 == 0 { 7 } else { handle },
        );
    }

    #[test]
    fn cps2_mem_card_update_matches() {
        check(
            ee::s08::f_002c55d8__generated,
            cps2_mem_card_update,
            83 * 3,
            &mcu_fixture,
        );
    }
}
