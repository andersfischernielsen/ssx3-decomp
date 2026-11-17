//! Staging module oc1031 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1031`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003f_ca40, slogin_store_poll, "SLoginStore_Poll")];

/// `gp`-relative word holding the login store's state block.
pub(crate) const STORE_POLL_STATE_GP: u32 = 0x2a30;
/// Word at +0x60: the directory hash the store keeps.
pub(crate) const STORE_POLL_DIR_HASH: u32 = 0x60;
/// Word at +0x74: the store operation in progress (1, 4..8 handled here).
pub(crate) const STORE_POLL_OP: u32 = 0x74;
/// Word at +0x78: the memory-card step of that operation.
pub(crate) const STORE_POLL_STEP: u32 = 0x78;
/// Word at +0x7c: the record payload buffer.
pub(crate) const STORE_POLL_RECORD: u32 = 0x7c;
/// Word at +0x80: the directory buffer.
pub(crate) const STORE_POLL_DIR: u32 = 0x80;
/// Word at +0x84: the record payload size.
pub(crate) const STORE_POLL_RECORD_SIZE: u32 = 0x84;
/// Words at +0x90 / +0x94: directory entries expected / loaded so far.
pub(crate) const STORE_POLL_ENTRY_TOTAL: u32 = 0x90;
pub(crate) const STORE_POLL_ENTRY_COUNT: u32 = 0x94;
/// Byte at +0xa8: a flag set while the card poll of operation 1 succeeds.
pub(crate) const STORE_POLL_FLAG: u32 = 0xa8;

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_POLL: Callee = Callee {
        addr: 0x003f_a040,
        f: ee::s14::SLoginMc_Poll,
    };

    pub(crate) const SLOGIN_MC_GET_FILE_STATE: Callee = Callee {
        addr: 0x003f_95e8,
        f: ee::s14::SLoginMc_GetFileState,
    };

    pub(crate) const SLOGIN_STORE_IS_INITIALIZED: Callee = Callee {
        addr: 0x003f_c0d8,
        f: ee::s15::SLoginStore_IsInitialized,
    };
    pub(crate) const SLOGIN_STORE_WRITE_RECORD: Callee = Callee {
        addr: 0x003f_bed0,
        f: ee::s15::SLoginStore_WriteRecord,
    };
    pub(crate) const SLOGIN_STORE_GET_BLOCK_HEADER: Callee = Callee {
        addr: 0x003f_b910,
        f: ee::s15::SLoginStore_GetBlockHeader,
    };
    pub(crate) const SLOGIN_STORE_WRITE_SENTINEL: Callee = Callee {
        addr: 0x003f_b9a0,
        f: ee::s15::SLoginStore_WriteSentinel,
    };
    pub(crate) const SLOGIN_STORE_CALC_ALLOC_SIZE: Callee = Callee {
        addr: 0x003f_b898,
        f: ee::s15::SLoginStore_CalcAllocSize,
    };
    pub(crate) const SLOGIN_STORE_DECRYPT: Callee = Callee {
        addr: 0x003f_bcd8,
        f: ee::s15::SLoginStore_Decrypt,
    };
    pub(crate) const SLOGIN_STORE_MAP_CIPHER_ERROR: Callee = Callee {
        addr: 0x003f_b828,
        f: ee::s15::SLoginStore_MapCipherError,
    };
    pub(crate) const SLOGIN_STORE_HASH_DIRECTORY: Callee = Callee {
        addr: 0x003f_bbe0,
        f: ee::s15::SLoginStore_HashDirectory,
    };
    pub(crate) const SLOGIN_STORE_READ_RECORD: Callee = Callee {
        addr: 0x003f_bf80,
        f: ee::s15::SLoginStore_ReadRecord,
    };
    pub(crate) const SLOGIN_STORE_VERIFY_RECORD: Callee = Callee {
        addr: 0x003f_bfe0,
        f: ee::s15::SLoginStore_VerifyRecord,
    };
    pub(crate) const SLOGIN_STORE_SEEK_TO_START: Callee = Callee {
        addr: 0x003f_bdb8,
        f: ee::s15::SLoginStore_SeekToStart,
    };
    pub(crate) const SLOGIN_STORE_READ_DIR_HEADER: Callee = Callee {
        addr: 0x003f_bdf0,
        f: ee::s15::SLoginStore_ReadDirHeader,
    };
    pub(crate) const SLOGIN_STORE_LOAD_DIR_ENTRY: Callee = Callee {
        addr: 0x003f_be40,
        f: ee::s15::SLoginStore_LoadDirEntry,
    };
    pub(crate) const MEMCMP: Callee = Callee {
        addr: 0x0041_5fc8,
        f: ee::s15::memcmp,
    };
    pub(crate) const SLOGIN_STORE_MAP_MC_ERROR: Callee = Callee {
        addr: 0x003f_b810,
        f: ee::s15::SLoginStore_MapMcError,
    };
}

/// Where a step of `SLoginStore_Poll` leaves for the shared epilogue.
#[derive(Clone, Copy)]
enum StorePollExit {
    /// Translate the card result in `s0` (`SLoginStore_MapMcError`).
    MapMcError,
    /// Return `v0`, restoring `s0` first.
    RestoreS0,
    /// Return `v0`; `s0` is already restored.
    Epilogue,
}

/// The login store word at `gp+0x2a30`.
fn store_poll_state(c: &mut Cpu) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_add(STORE_POLL_STATE_GP))
}

/// `int SLoginStore_Poll(int* status, int* detail)` (0x3fca40): the login
/// store's per-frame step. It polls the memory card (`SLoginMc_Poll`); once
/// the card reports done (`*status == 2`) it finishes the card step named by
/// the store's operation (+0x74) and step (+0x78) words: writes or reads a
/// record, checks a read record's size and decrypts it, rewinds, reads the
/// directory header and loads directory entries, then compares the directory
/// hash. A step that starts another card request sets `*status = 1` and
/// returns that request's result; otherwise the card result is mapped into
/// the store's error range. Returns 2 when the store is not initialised and
/// 4 for a null argument.
pub fn slogin_store_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_ca60)?;
        let exit = store_poll_steps(c, frame)?;
        if let StorePollExit::MapMcError = exit {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_cdb0)?;
        }
        if !matches!(exit, StorePollExit::Epilogue) {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The body of `SLoginStore_Poll` after `SLoginStore_IsInitialized`
/// returned: `s1` is `status`, `s2` is `detail`, `s0` takes the card result.
fn store_poll_steps(c: &mut Cpu, frame: Frame) -> Result<StorePollExit, Unwound> {
    let not_ready = c.g[V0][0] == 0;
    c.g[V0][0] = 2;
    if not_ready {
        return Ok(StorePollExit::RestoreS0);
    }
    c.cycles += 2;
    let no_status = c.g[S1][0] == 0;
    c.g[V0][0] = 4;
    if no_status {
        return Ok(StorePollExit::RestoreS0);
    }
    c.cycles += 2;
    let has_detail = c.g[S2][0] != 0;
    c.g[A0][0] = c.g[S1][0];
    if !has_detail {
        c.cycles += 2;
        frame.restore(c, S0, 0x40);
        return Ok(StorePollExit::Epilogue);
    }

    // Poll the card; a pending card status with the store busy is error 8.
    c.cycles += 6;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, callee::SLOGIN_MC_POLL.addr, 0x003f_ca88)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
    } else {
        c.cycles += 4;
        c.g[V0][0] = store_poll_state(c);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_OP));
        let busy = c.g[V1][0] != 0;
        c.g[V0][0] = 8;
        if busy {
            return Ok(StorePollExit::RestoreS0);
        }
        c.cycles += 1;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
    }
    c.cycles += 3;
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        return Ok(StorePollExit::MapMcError);
    }

    // A card error only matters to operations 4 and 5.
    c.cycles += 2;
    let card_ok = c.g[S0][0] == 0;
    c.g[A1][0] = store_poll_state(c);
    if !card_ok {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_OP));
        c.g[V0][0] = 4;
        let op_four = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 5;
        if !op_four {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                return Ok(StorePollExit::MapMcError);
            }
        }
    }

    // Operation 1 sets the flag byte on success; every operation but 4
    // clears it otherwise.
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_OP));
    c.g[V0][0] = 1;
    let mut clear_flag = true;
    if c.g[A0][0] != c.g[V0][0] {
        c.g[A0][0] = store_poll_state(c);
    } else {
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            c.sb(
                (c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG),
                c.g[A0][0],
            );
        }
        c.cycles += 4;
        c.g[V0][0] = store_poll_state(c);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_OP));
        let still_one = c.g[V1][0] == c.g[A0][0];
        c.g[A0][0] = store_poll_state(c);
        if still_one {
            clear_flag = false;
        }
    }
    if clear_flag {
        c.cycles += 4;
        c.g[V1][0] = 4;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(STORE_POLL_OP));
        if c.g[V0][0] != c.g[V1][0] {
            c.sb((c.g[A0][0] as u32).wrapping_add(STORE_POLL_FLAG), 0);
        }
    }

    // Operation 6: write the record (step 5), or check the size of the
    // record just read and decrypt it (step 6).
    c.cycles += 5;
    c.g[V1][0] = store_poll_state(c);
    c.g[V0][0] = 6;
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_POLL_OP));
    let op_six = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = store_poll_state(c);
    if op_six {
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_POLL_STEP));
        c.g[V0][0] = 5;
        let write_step = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if write_step {
            c.cycles += 4;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::SLOGIN_STORE_WRITE_RECORD.call(c, 0x003f_cb3c)?;
            frame.restore(c, S0, 0x40);
            return Ok(StorePollExit::Epilogue);
        }
        c.cycles += 2;
        let decode_step = c.g[A0][0] == c.g[A1][0];
        c.g[V0][0] = store_poll_state(c);
        if decode_step {
            c.cycles += 12;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_POLL_RECORD));
            c.g[A1][0] = 0;
            callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_cb58)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_cb60)?;
            c.g[V0][0] = store_poll_state(c);
            c.g[A1][0] = 2;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_RECORD_SIZE));
            callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cb70)?;
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            let wrong_size = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0x6e;
            if wrong_size {
                return Ok(StorePollExit::RestoreS0);
            }
            c.cycles += 9;
            c.g[V0][0] = store_poll_state(c);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_RECORD_SIZE));
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
            c.g[V0][0] = store_poll_state(c);
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_RECORD_SIZE));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_RECORD));
            callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_cb98)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, 0x003f_cba8)?;
                frame.restore(c, S0, 0x40);
                return Ok(StorePollExit::Epilogue);
            }
            c.cycles += 2;
            let card_failed = c.g[S0][0] != 0;
            c.g[V0][0] = store_poll_state(c);
            if !card_failed {
                c.cycles += 4;
                c.g[A0][0] = store_poll_state(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(STORE_POLL_DIR_HASH));
                callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_cbc4)?;
                c.g[V0][0] = store_poll_state(c);
            }
        }
    }

    // Operation 8: read the record (step 5), or check its size and verify
    // it (step 10).
    c.cycles += 4;
    c.g[A0][0] = 8;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_OP));
    let op_eight = c.g[V1][0] == c.g[A0][0];
    c.g[A1][0] = c.g[V0][0];
    if op_eight {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_STEP));
        c.g[V0][0] = 5;
        let read_step = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0xa;
        if read_step {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::SLOGIN_STORE_READ_RECORD.call(c, 0x003f_cbf4)?;
            frame.restore(c, S0, 0x40);
            return Ok(StorePollExit::Epilogue);
        }
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_OP));
        } else {
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_RECORD));
            c.g[A1][0] = 0;
            callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_cc10)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[V0][0] = store_poll_state(c);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_RECORD_SIZE));
            callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cc24)?;
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            let wrong_size = c.g[V0][0] != c.g[V1][0];
            c.g[S0][0] = 0x6e;
            if !wrong_size {
                c.cycles += 5;
                c.g[V0][0] = store_poll_state(c);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_POLL_RECORD_SIZE));
                c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                callee::SLOGIN_STORE_VERIFY_RECORD.call(c, 0x003f_cc40)?;
                c.g[S0][0] = c.g[V0][0];
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_cc4c)?;
            c.g[V0][0] = c.g[S0][0];
            return Ok(StorePollExit::RestoreS0);
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_OP));
    }

    // Any operation but 4 and 5 ends here; a finished card status resets
    // the operation and step words.
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let directory_op = c.g[V0][0] != 0;
    c.g[V0][0] = 4;
    if !directory_op {
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.sw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_STEP), 0);
            c.sw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_OP), 0);
        }
        return Ok(StorePollExit::MapMcError);
    }

    // Operations 4 and 5, step 4: take the card's file state and rewind.
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_STEP));
    let rewind_step = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 9;
    if rewind_step {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.sb((c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG), 0);
            return Ok(StorePollExit::MapMcError);
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, callee::SLOGIN_MC_GET_FILE_STATE.addr, 0x003f_cc84)?;
        c.g[V1][0] = 1;
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        callee::SLOGIN_STORE_SEEK_TO_START.call(c, 0x003f_cc90)?;
        frame.restore(c, S0, 0x40);
        return Ok(StorePollExit::Epilogue);
    }

    // Step 9: done if the flag byte was set, else read the directory header.
    c.cycles += 2;
    let header_step = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 7;
    if header_step {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.sb((c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG), 0);
            return Ok(StorePollExit::MapMcError);
        }
        c.cycles += 3;
        c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG));
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if flagged {
            c.cycles += 4;
            c.sb((c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG), 0);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = 0;
            return Ok(StorePollExit::RestoreS0);
        }
        c.cycles += 6;
        c.sb((c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG), 0);
        c.g[V0][0] = 1;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        callee::SLOGIN_STORE_READ_DIR_HEADER.call(c, 0x003f_ccd4)?;
        frame.restore(c, S0, 0x40);
        return Ok(StorePollExit::Epilogue);
    }

    // Step 7: load the next directory entry, or (all loaded, or card
    // result 10) hash the directory and store or compare the hash.
    c.cycles += 2;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 2;
        let card_ok = c.g[S0][0] == 0;
        c.g[V0][0] = 0xa;
        let mut load_entry = false;
        if !card_ok {
            c.cycles += 2;
            let no_more = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = 2;
            if !no_more {
                c.cycles += 2;
                c.sb((c.g[A1][0] as u32).wrapping_add(STORE_POLL_FLAG), 0);
                return Ok(StorePollExit::MapMcError);
            }
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_ENTRY_TOTAL));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_ENTRY_COUNT));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if more {
                load_entry = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 2;
            }
        }
        if load_entry {
            c.cycles += 4;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::SLOGIN_STORE_LOAD_DIR_ENTRY.call(c, 0x003f_cd70)?;
            frame.restore(c, S0, 0x40);
            return Ok(StorePollExit::Epilogue);
        }
        c.cycles += 11;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = store_poll_state(c);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_POLL_DIR));
        callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_cd20)?;
        c.g[A0][0] = c.g[SP][0];
        callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_cd28)?;
        c.g[A1][0] = store_poll_state(c);
        c.g[V1][0] = 5;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_POLL_OP));
        let verify = c.g[V0][0] == c.g[V1][0];
        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
        if !verify {
            c.cycles += 3;
            c.g[V0][0] = 0;
            c.sw(
                (c.g[A1][0] as u32).wrapping_add(STORE_POLL_DIR_HASH),
                c.g[V1][0],
            );
            return Ok(StorePollExit::RestoreS0);
        }
        c.cycles += 8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(STORE_POLL_DIR_HASH));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        callee::MEMCMP.call(c, 0x003f_cd4c)?;
        c.g[V1][0] = 9;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        return Ok(StorePollExit::RestoreS0);
    }

    // Any other step: read the directory header after a good card result.
    c.cycles += 2;
    let card_failed = c.g[S0][0] != 0;
    c.g[V0][0] = 1;
    if card_failed {
        return Ok(StorePollExit::MapMcError);
    }
    c.cycles += 4;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    callee::SLOGIN_STORE_READ_DIR_HEADER.call(c, 0x003f_cd88)?;
    frame.restore(c, S0, 0x40);
    Ok(StorePollExit::Epilogue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_UNWIND_AT};
    use crate::difftest::RA as STORE_POLL_TEST_RETURN;
    use crate::difftest::{check, elf_image, fresh, run_generated, DATA, GAME_GP};

    const STORE_POLL_TEST_STATUS: u32 = DATA + 0x1000;
    const STORE_POLL_TEST_DETAIL: u32 = DATA + 0x1010;
    const STORE_POLL_TEST_MC_IFACE: u32 = DATA + 0x3000;
    const STORE_POLL_TEST_MC_STATE: u32 = DATA + 0x3400;
    const STORE_POLL_TEST_FINISH_TABLE: u32 = DATA + 0x3800;
    const STORE_POLL_TEST_STEP_TABLE: u32 = DATA + 0x3900;
    const STORE_POLL_TEST_CIPHER: u32 = DATA + 0x3c00;
    /// What the card fakes report: status word, then the results of the
    /// card poll, the finish handler and the step handler.
    const STORE_POLL_TEST_CTRL: u32 = DATA + 0x3f00;
    const STORE_POLL_TEST_STORE: u32 = DATA + 0x4000;
    const STORE_POLL_TEST_RECORD: u32 = DATA + 0x5010;
    const STORE_POLL_TEST_DIR: u32 = DATA + 0x5410;
    const STORE_POLL_TEST_RECORDS: u32 = DATA + 0x6000;
    /// Fake addresses of the card interface's poll and handlers.
    const STORE_POLL_FAKE_CARD_POLL: u32 = FAKE + 0x40;
    const STORE_POLL_FAKE_STEP: u32 = FAKE + 0x48;
    const STORE_POLL_FAKE_FINISH: u32 = FAKE + 0x4c;
    /// Fixture cases; the next seed digit picks the fake call that unwinds.
    const STORE_POLL_CASES: u32 = 36;
    const STORE_POLL_UNWIND_AT: [u32; 4] = [u32::MAX, 0, 1, 3];

    /// The `miss` hook: a logged fake call; the card poll also writes the
    /// fixture's status word, and the poll and handlers return the fixture's
    /// results (0 included, which plain fakes cannot return).
    fn store_poll_card_fake(c: &mut Cpu, target: u32) {
        let status_at = c.g[A0][0] as u32;
        fakes::fake_call(c, target);
        let result_at = match target {
            STORE_POLL_FAKE_CARD_POLL => {
                let status = c.bus.read32(STORE_POLL_TEST_CTRL);
                c.bus.write32(status_at, status);
                STORE_POLL_TEST_CTRL + 4
            }
            STORE_POLL_FAKE_FINISH => STORE_POLL_TEST_CTRL + 8,
            STORE_POLL_FAKE_STEP => STORE_POLL_TEST_CTRL + 0xc,
            _ => return,
        };
        c.g[V0][0] = sx(c.bus.read32(result_at));
    }

    /// One fixture case: the card's answers and the store's state.
    struct StorePollCase {
        initialized: bool,
        status_null: bool,
        detail_null: bool,
        /// `detail` points at `status` (a size write then changes the status).
        detail_is_status: bool,
        card_status: u32,
        first: u32,
        finish: u32,
        step_result: u32,
        op: u32,
        step: u32,
        flag: u8,
        entry_total: u32,
        dir_hash: u32,
        cipher_fails: bool,
        size_matches: bool,
        record_size: u32,
    }

    fn store_poll_case(case: u32) -> StorePollCase {
        let mut k = StorePollCase {
            initialized: true,
            status_null: false,
            detail_null: false,
            detail_is_status: false,
            card_status: 2,
            first: 0x33,
            finish: 0x21,
            step_result: 0,
            op: 0,
            step: 0,
            flag: 0x5a,
            entry_total: 0,
            dir_hash: 0x1357_9bdf,
            cipher_fails: false,
            size_matches: true,
            record_size: 0x100,
        };
        match case {
            0 => k.initialized = false,
            1 => k.status_null = true,
            2 => k.detail_null = true,
            3 => {
                k.card_status = 0;
                k.op = 3;
            }
            4 => k.card_status = 0,
            5 => {
                k.card_status = 0;
                k.first = 0;
            }
            6 => k.card_status = 1,
            7 => {
                k.step_result = 0x40;
                k.op = 3;
            }
            8 => {
                k.step_result = 0x40;
                k.op = 4;
                k.step = 4;
            }
            9 => {
                k.step_result = 0x40;
                k.op = 5;
                k.step = 9;
            }
            10 => {
                k.step_result = 0x40;
                k.op = 4;
                k.step = 7;
            }
            11 => {
                k.step_result = 0x40;
                k.op = 5;
                k.step = 2;
            }
            12 => k.op = 1,
            13 => {
                k.op = 6;
                k.step = 5;
            }
            14 => {
                k.op = 6;
                k.step = 6;
                k.size_matches = false;
            }
            15 => {
                k.op = 6;
                k.step = 6;
                k.cipher_fails = true;
            }
            16 => {
                k.op = 6;
                k.step = 6;
            }
            17 => {
                k.op = 6;
                k.step = 6;
                k.detail_is_status = true;
                k.record_size = 0xffff_fff6;
            }
            18 => {
                k.op = 6;
                k.step = 7;
            }
            19 => {
                k.op = 8;
                k.step = 5;
            }
            20 => {
                k.op = 8;
                k.step = 10;
                k.size_matches = false;
            }
            21 => {
                k.op = 8;
                k.step = 10;
            }
            22 => {
                k.op = 8;
                k.step = 3;
            }
            23 => {
                k.op = 4;
                k.step = 4;
            }
            24 => {
                k.op = 4;
                k.step = 9;
            }
            25 => {
                k.op = 5;
                k.step = 9;
            }
            26 => {
                k.op = 4;
                k.step = 7;
                k.entry_total = 3;
            }
            27 => {
                k.op = 4;
                k.step = 7;
            }
            28 => {
                k.op = 5;
                k.step = 7;
                k.dir_hash = 0;
            }
            29 => {
                k.op = 5;
                k.step = 7;
            }
            30 => {
                k.op = 4;
                k.step = 7;
                k.finish = 0xa;
            }
            31 => {
                k.op = 5;
                k.step = 7;
                k.finish = 0xa;
            }
            32 => {
                k.op = 4;
                k.step = 2;
            }
            33 => k.op = 0,
            34 => k.op = 7,
            35 => {
                k.op = 4;
                k.step = 4;
                k.finish = 0x10;
            }
            _ => unreachable!(),
        }
        k
    }

    /// `(v0, *status)` the generated body returns for a case with no unwind
    /// (`None`: the result comes from a callee path not pinned here).
    fn store_poll_expected(case: u32) -> Option<(u64, u32)> {
        let untouched = 0x5555;
        Some(match case {
            0 | 1 | 2 => (if case == 0 { 2 } else { 4 }, untouched),
            3 => (8, 0),
            4 => (0x97, 0),
            5 => (0, 0),
            6 => (0x97, 1),
            7..=11 => (0xa4, 2),
            12 => (0, 2),
            13 => (4, 1),
            14 | 20 => (0x6e, 2),
            15 | 26 => return None,
            16 | 18 | 22 | 24 | 27 | 28 | 30 | 33 | 34 => (0, 2),
            17 => (0, 0xffff_fff6),
            19 | 23 | 25 | 32 => (0, 1),
            21 => (0xb, 2),
            29 | 31 => (9, 2),
            35 => (0x74, 2),
            _ => unreachable!(),
        })
    }

    fn store_poll_fixture(c: &mut Cpu, seed: u32) {
        let case = seed % STORE_POLL_CASES;
        let k = store_poll_case(case);
        let _r = machine(c, seed, 0x003f_ca40, 4);
        elf_image(c);
        c.hooks.miss = store_poll_card_fake;
        c.bus.write32(
            LOG_UNWIND_AT,
            STORE_POLL_UNWIND_AT[(seed / STORE_POLL_CASES % 4) as usize],
        );
        c.pc = 0x003f_ca40;

        // Arguments and output words (pre-filled so every store shows).
        c.g[A0][0] = if k.status_null {
            0
        } else {
            sx(STORE_POLL_TEST_STATUS)
        };
        c.g[A1][0] = if k.detail_null {
            0
        } else if k.detail_is_status {
            sx(STORE_POLL_TEST_STATUS)
        } else {
            sx(STORE_POLL_TEST_DETAIL)
        };
        c.bus.write32(STORE_POLL_TEST_STATUS, 0x5555);
        let size_check = k.record_size.wrapping_add(0xc);
        c.bus.write32(
            STORE_POLL_TEST_DETAIL,
            if k.size_matches { size_check } else { 0x77 },
        );
        c.bus.write32(STORE_POLL_TEST_CTRL, k.card_status);
        c.bus.write32(STORE_POLL_TEST_CTRL + 4, k.first);
        c.bus.write32(STORE_POLL_TEST_CTRL + 8, k.finish);
        c.bus.write32(STORE_POLL_TEST_CTRL + 0xc, k.step_result);

        // The memory-card layer: idle state, an interface of fakes.
        c.bus.write32(GAME_GP + 0x1ae8, STORE_POLL_TEST_MC_STATE);
        c.bus.write32(GAME_GP + 0x1aec, STORE_POLL_TEST_MC_IFACE);
        for i in 0..0x20 {
            c.bus
                .write32(STORE_POLL_TEST_MC_STATE + 4 * i, 0x6100_0000 + i);
        }
        c.bus.write32(STORE_POLL_TEST_MC_STATE, 0);
        c.bus.write32(STORE_POLL_TEST_MC_STATE + 4, 0);
        for off in [0x2c, 0x30, 0x34] {
            c.bus.write32(STORE_POLL_TEST_MC_IFACE + off, FAKE + off);
        }
        c.bus
            .write32(STORE_POLL_TEST_MC_IFACE + 0x40, STORE_POLL_FAKE_CARD_POLL);
        c.bus
            .write32(STORE_POLL_TEST_MC_IFACE + 0x48, STORE_POLL_TEST_STEP_TABLE);
        c.bus.write32(
            STORE_POLL_TEST_MC_IFACE + 0x4c,
            STORE_POLL_TEST_FINISH_TABLE,
        );
        c.bus
            .write32(STORE_POLL_TEST_STEP_TABLE, STORE_POLL_FAKE_STEP);
        c.bus
            .write32(STORE_POLL_TEST_FINISH_TABLE, STORE_POLL_FAKE_FINISH);

        // The login store.
        let store = STORE_POLL_TEST_STORE;
        c.bus.write32(
            GAME_GP + STORE_POLL_STATE_GP,
            if k.initialized { store } else { 0 },
        );
        for i in 0..0x30 {
            c.bus.write32(store + 4 * i, 0x5300_0000 + i);
        }
        c.bus.write32(store + STORE_POLL_DIR_HASH, k.dir_hash);
        c.bus.write32(
            store + 0x6c,
            if k.cipher_fails {
                STORE_POLL_TEST_CIPHER
            } else {
                0
            },
        );
        for off in [0xc, 0x14, 0x18] {
            c.bus
                .write32(STORE_POLL_TEST_CIPHER + off, FAKE + 0x80 + off);
        }
        c.bus.write32(store + STORE_POLL_OP, k.op);
        c.bus.write32(store + STORE_POLL_STEP, k.step);
        c.bus
            .write32(store + STORE_POLL_RECORD, STORE_POLL_TEST_RECORD);
        c.bus.write32(store + STORE_POLL_DIR, STORE_POLL_TEST_DIR);
        c.bus.write32(store + STORE_POLL_RECORD_SIZE, k.record_size);
        c.bus.write32(store + 0x8c, 0x11);
        c.bus.write32(store + STORE_POLL_ENTRY_TOTAL, k.entry_total);
        c.bus.write32(store + STORE_POLL_ENTRY_COUNT, 0);
        c.bus.write32(store + 0x9c, STORE_POLL_TEST_RECORDS);
        c.bus.write8(store + STORE_POLL_FLAG, k.flag);
        // Record and directory blocks with headers that are not the
        // sentinel, so header writes and the header check show.
        for i in 0..0x40 {
            c.bus
                .write8(STORE_POLL_TEST_RECORD - 0xc + i, (0xa0 + i) as u8);
            c.bus
                .write8(STORE_POLL_TEST_DIR - 0xc + i, (0x30 + i) as u8);
        }
        for i in 0..8 {
            c.bus
                .write32(STORE_POLL_TEST_RECORDS + 4 * i, 0x5200_0000 + i);
        }
    }

    /// `SLoginStore_Poll`: every operation/step arm, card results 0, 10, 16
    /// and others, size match and mismatch, cipher failure, flagged and
    /// unflagged step 9, matching and differing directory hashes, and
    /// unwinds in the card poll, its handlers and a later callee.
    #[test]
    fn slogin_store_poll_matches() {
        check(
            ee::s15::f_003fca40__generated,
            slogin_store_poll,
            STORE_POLL_CASES * 4,
            &store_poll_fixture,
        );
        // The cases reach their arms: the generated body's results.
        for case in 0..STORE_POLL_CASES {
            let mut c = fresh();
            store_poll_fixture(&mut c, case);
            run_generated(&mut c, ee::s15::f_003fca40__generated);
            assert_eq!(c.pc, STORE_POLL_TEST_RETURN, "case {case}: returned");
            if let Some((v0, status)) = store_poll_expected(case) {
                assert_eq!(c.g[V0][0], v0, "case {case}: v0");
                assert_eq!(
                    c.bus.read32(STORE_POLL_TEST_STATUS),
                    status,
                    "case {case}: status"
                );
            }
        }
    }
}
