//! Staging module oc1027 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1027`.

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
    (
        0x003f_bfe0,
        slogin_store_verify_record,
        "SLoginStore_VerifyRecord",
    ),
    (
        0x003f_c0d8,
        slogin_store_is_initialized,
        "SLoginStore_IsInitialized",
    ),
    (0x003f_c0e8, slogin_store_init, "SLoginStore_Init"),
    (0x003f_c260, slogin_store_shutdown, "SLoginStore_Shutdown"),
    (0x003f_c390, slogin_store_create, "SLoginStore_Create"),
    (0x003f_c3c0, slogin_store_delete, "SLoginStore_Delete"),
    (0x003f_c4c8, slogin_store_open, "SLoginStore_Open"),
    (0x003f_c4e8, slogin_store_close, "SLoginStore_Close"),
    (0x003f_c548, slogin_store_write, "SLoginStore_Write"),
    (0x003f_c708, slogin_store_read, "SLoginStore_Read"),
];

const STORE_STATE_GP: u32 = 0x2a30;
const STORE_RECORD_COUNT: u32 = 0x94;
const STORE_RECORDS: u32 = 0x9c;

// Store-state words (the struct at `gp+0x2a30`), named after how the record
// and I/O functions below use them.
const STORE_CIPHER_KEY: u32 = 0x64;
const STORE_CIPHER_KEY_LEN: u32 = 0x68;
const STORE_CIPHER_IFACE: u32 = 0x6c;
const STORE_HASH_IFACE: u32 = 0x70;
const STORE_OPEN_ARG: u32 = 0x74;
const STORE_PENDING_OP: u32 = 0x78;
const STORE_RECORD_BUFFER: u32 = 0x7c;
const STORE_DIR_BUFFER: u32 = 0x80;
const STORE_RECORD_SIZE: u32 = 0x84;
const STORE_RECORD_CRC: u32 = 0x88;
const STORE_RECORD_KIND: u32 = 0x8c;
const STORE_RECORD_CAPACITY: u32 = 0x90;
const STORE_DATA_END: u32 = 0x98;

// Directory entry words (16 bytes each).
const RECORD_OFFSET: u32 = 4;
const RECORD_SIZE: u32 = 8;

const BLOCK_HEADER_BYTES: u32 = 12;

/// Word at +0x60 of the store state: the directory hash. The first 0x64
/// bytes of the state are the directory header `SLoginStore_Write`/`_Read`
/// copy in; `SLoginStore_Close` copies the hash back.
const STORE_DIR_HASH: u32 = 0x60;
/// Word at +0xa0: the heap handle from the config, passed to every
/// `TibExtMemAlloc`/`TibExtMemFree` of the store.
const STORE_HEAP: u32 = 0xa0;
/// Word at +0xa4: `SLoginStore_Read` puts the found record's offset there.
const STORE_SEEK_OFFSET: u32 = 0xa4;
/// Byte at +0xa8: cleared by `SLoginStore_Init`.
const STORE_FLAG_A8: u32 = 0xa8;
/// Size of the store state block `SLoginStore_Init` allocates.
const STORE_STATE_BYTES: u64 = 0xac;

// Words of the config `SLoginStore_Init` reads.
const STORE_CONFIG_MAX_RECORDS: u32 = 0x0;
const STORE_CONFIG_DEVICES: u32 = 0x4;
const STORE_CONFIG_MC_FUNCS: u32 = 0x8;
const STORE_CONFIG_KEY: u32 = 0xc;
const STORE_CONFIG_KEY_LEN: u32 = 0x10;
const STORE_CONFIG_CIPHER: u32 = 0x14;
const STORE_CONFIG_HEAP: u32 = 0x18;

// Memory-card layer entry points (reached through the function table).

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_START_SEEK: Callee = Callee {
        addr: 0x003f_9c48,
        f: ee::s14::SLoginMc_StartSeek,
    };

    pub(crate) const SLOGIN_MC_INIT: Callee = Callee {
        addr: 0x003f_9218,
        f: ee::s14::SLoginMc_Init,
    };

    pub(crate) const SLOGIN_MC_SHUTDOWN: Callee = Callee {
        addr: 0x003f_9530,
        f: ee::s14::SLoginMc_Shutdown,
    };

    pub(crate) const SLOGIN_MC_START_CREATE: Callee = Callee {
        addr: 0x003f_9818,
        f: ee::s14::SLoginMc_StartCreate,
    };

    pub(crate) const SLOGIN_MC_START_DELETE: Callee = Callee {
        addr: 0x003f_9698,
        f: ee::s14::SLoginMc_StartDelete,
    };

    pub(crate) const SLOGIN_MC_START_CLOSE: Callee = Callee {
        addr: 0x003f_9b38,
        f: ee::s14::SLoginMc_StartClose,
    };

    pub(crate) const SLOGIN_MC_GET_CHECKSUM_IFACE: Callee = Callee {
        addr: 0x003f_a6c8,
        f: ee::s14::SLoginMc_GetChecksumIface,
    };

    pub(crate) const MEMCPY: Callee = Callee {
        addr: 0x0041_605c,
        f: ee::s15::memcpy,
    };
    pub(crate) const TIB_EXT_MEM_ALLOC: Callee = Callee {
        addr: 0x0026_6728,
        f: ee::s07::TibExtMemAlloc,
    };
    pub(crate) const TIB_EXT_MEM_FREE: Callee = Callee {
        addr: 0x0026_6760,
        f: ee::s07::TibExtMemFree,
    };

    pub(crate) const SLOGIN_STORE_IS_INITIALIZED: Callee = Callee {
        addr: 0x003f_c0d8,
        f: ee::s15::SLoginStore_IsInitialized,
    };
    pub(crate) const SLOGIN_STORE_GET_BLOCK_HEADER: Callee = Callee {
        addr: 0x003f_b910,
        f: ee::s15::SLoginStore_GetBlockHeader,
    };
    pub(crate) const SLOGIN_STORE_WRITE_SENTINEL: Callee = Callee {
        addr: 0x003f_b9a0,
        f: ee::s15::SLoginStore_WriteSentinel,
    };
    pub(crate) const SLOGIN_STORE_GET_RECORD_HEADER: Callee = Callee {
        addr: 0x003f_b9d0,
        f: ee::s15::SLoginStore_GetRecordHeader,
    };
    pub(crate) const SLOGIN_STORE_FIND_RECORD: Callee = Callee {
        addr: 0x003f_ba20,
        f: ee::s15::SLoginStore_FindRecord,
    };
    pub(crate) const SLOGIN_STORE_SET_RECORD_CRC: Callee = Callee {
        addr: 0x003f_ba78,
        f: ee::s15::SLoginStore_SetRecordCrc,
    };
    pub(crate) const SLOGIN_STORE_ADD_RECORD: Callee = Callee {
        addr: 0x003f_bad0,
        f: ee::s15::SLoginStore_AddRecord,
    };
    pub(crate) const SLOGIN_STORE_HASH: Callee = Callee {
        addr: 0x003f_bb50,
        f: ee::s15::SLoginStore_Hash,
    };
    pub(crate) const SLOGIN_STORE_HASH_DIRECTORY: Callee = Callee {
        addr: 0x003f_bbe0,
        f: ee::s15::SLoginStore_HashDirectory,
    };
    pub(crate) const SLOGIN_STORE_DECRYPT: Callee = Callee {
        addr: 0x003f_bcd8,
        f: ee::s15::SLoginStore_Decrypt,
    };
    pub(crate) const SLOGIN_STORE_BEGIN_OPEN: Callee = Callee {
        addr: 0x003f_bd70,
        f: ee::s15::SLoginStore_BeginOpen,
    };
    pub(crate) const SLOGIN_STORE_ALLOC_BLOCK: Callee = Callee {
        addr: 0x003f_ce10,
        f: ee::s15::SLoginStore_AllocBlock,
    };
    pub(crate) const SLOGIN_STORE_FREE_BLOCK: Callee = Callee {
        addr: 0x003f_cf00,
        f: ee::s15::SLoginStore_FreeBlock,
    };
    pub(crate) const SLOGIN_STORE_MAP_MC_ERROR: Callee = Callee {
        addr: 0x003f_b810,
        f: ee::s15::SLoginStore_MapMcError,
    };
    pub(crate) const SLOGIN_STORE_MAP_CIPHER_ERROR: Callee = Callee {
        addr: 0x003f_b828,
        f: ee::s15::SLoginStore_MapCipherError,
    };
    pub(crate) const SLOGIN_STORE_MAP_HASH_ERROR: Callee = Callee {
        addr: 0x003f_b840,
        f: ee::s15::SLoginStore_MapHashError,
    };
    pub(crate) const MEMCMP: Callee = Callee {
        addr: 0x0041_5fc8,
        f: ee::s15::memcmp,
    };
}

/// The address of the login store's state pointer (`gp+0x2a30`).
fn slogin_store_state_slot(c: &Cpu) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP)
}

/// `int SLoginStore_VerifyRecord(void)` (0x3fbfe0): check the record just
/// read into the store's record buffer: decrypt its 12-byte header, compare
/// the header's kind and size words with the expected ones, decrypt the
/// payload, hash it and compare the hash with the header's CRC word. Returns
/// 0 when it matches, 0xb for a wrong kind or hash length, 0x6e for a wrong
/// size, 0xa for a CRC mismatch, or the cipher/hash error.
pub fn slogin_store_verify_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        let slot = slogin_store_state_slot(c);
        c.g[V0][0] = c.lw(slot);
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S1, 0x20);
        c.sw(sp(c).wrapping_add(8), 0);
        c.g[S1][0] = 0xa;
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
        c.sw(sp(c).wrapping_add(4), 0);
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[S1][0],
        );
        callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_c010)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = BLOCK_HEADER_BYTES as u64;
        c.g[A0][0] = c.g[S0][0];
        callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_c020)?;
        'epilogue: {
            'result: {
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'epilogue;
                }
                c.cycles += 11;
                let slot = slogin_store_state_slot(c);
                c.g[A3][0] = c.lw(slot);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = c.g[SP][0] | 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(STORE_RECORD_CRC));
                callee::SLOGIN_STORE_GET_RECORD_HEADER.call(c, 0x003f_c040)?;
                let slot = slogin_store_state_slot(c);
                c.g[A0][0] = c.lw(slot);
                c.g[V1][0] = c.lw(sp(c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(STORE_RECORD_KIND));
                let other_kind = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0xb;
                if other_kind {
                    break 'result;
                }
                c.cycles += 4;
                c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(STORE_RECORD_SIZE));
                let other_size = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x6e;
                if other_size {
                    break 'result;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
                callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_c06c)?;
                let failed = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x30);
                if failed {
                    break 'epilogue;
                }
                c.cycles += 7;
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                c.g[A2][0] = c.g[SP][0] | 8;
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_SIZE));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
                callee::SLOGIN_STORE_HASH.call(c, 0x003f_c088)?;
                let failed = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x30);
                if failed {
                    break 'epilogue;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(sp(c).wrapping_add(8));
                c.g[V1][0] = 4;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = 0xb;
                    break 'epilogue;
                }
                c.cycles += 7;
                let slot = slogin_store_state_slot(c);
                c.g[A1][0] = c.lw(slot);
                c.g[A2][0] = 4;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(STORE_RECORD_CRC));
                callee::MEMCMP.call(c, 0x003f_c0b8)?;
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = 0;
                }
                c.g[V0][0] = c.g[S1][0];
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `bool SLoginStore_IsInitialized(void)` (0x3fc0d8): whether the login
/// store's state block exists.
pub fn slogin_store_is_initialized(c: &mut Cpu) {
    c.cycles += 3;
    let slot = slogin_store_state_slot(c);
    c.g[V0][0] = c.lw(slot);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    ret(c);
}

/// `int SLoginStore_Init(const StoreConfig* config)` (0x3fc0e8): start the
/// memory-card layer, allocate the store state from the config's heap, set up
/// the checksum interface and the optional cipher (with a copy of its key),
/// allocate the record directory and the directory block. Returns 0, 1 when
/// already initialized, 6 when out of memory, or the mapped card, hash or
/// cipher error.
pub fn slogin_store_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c0fc)?;
        'epilogue: {
            'restore: {
                let initialized = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if initialized {
                    break 'restore;
                }
                c.cycles += 6;
                let config = c.g[S0][0] as u32;
                c.g[A0][0] = c.lw(config.wrapping_add(STORE_CONFIG_DEVICES));
                c.g[A1][0] = c.lw(config.wrapping_add(STORE_CONFIG_MC_FUNCS));
                c.g[A2][0] = c.lw(config.wrapping_add(STORE_CONFIG_HEAP));
                call_ptr(c, callee::SLOGIN_MC_INIT.addr, 0x003f_c114)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c124)?;
                    frame.restore(c, S0, 0x10);
                    break 'epilogue;
                }
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_HEAP));
                'no_memory: {
                    c.cycles += 5;
                    c.g[A1][0] = STORE_STATE_BYTES;
                    c.g[A2][0] = 4;
                    call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_c138)?;
                    let failed = c.g[V0][0] == 0;
                    let slot = slogin_store_state_slot(c);
                    c.sw(slot, c.g[V0][0]);
                    if failed {
                        break 'no_memory;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(STORE_HASH_IFACE));
                    call_ptr(c, callee::SLOGIN_MC_GET_CHECKSUM_IFACE.addr, 0x003f_c148)?;
                    let slot = slogin_store_state_slot(c);
                    c.g[V0][0] = c.lw(slot);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_HASH_IFACE));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let hash_init = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_HEAP));
                    call_ptr(c, hash_init, 0x003f_c15c)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V0][0];
                        callee::SLOGIN_STORE_MAP_HASH_ERROR.call(c, 0x003f_c16c)?;
                        frame.restore(c, S0, 0x10);
                        break 'epilogue;
                    }
                    let slot = slogin_store_state_slot(c);
                    c.g[V0][0] = c.lw(slot);
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_CIPHER));
                    let no_cipher = c.g[V1][0] == 0;
                    c.sw(
                        (c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_IFACE),
                        c.g[V1][0],
                    );
                    if !no_cipher {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let cipher_init = c.g[V0][0] as u32;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_HEAP));
                        call_ptr(c, cipher_init, 0x003f_c18c)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[V0][0];
                            callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, 0x003f_c19c)?;
                            frame.restore(c, S0, 0x10);
                            break 'epilogue;
                        }
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_KEY_LEN));
                        c.cycles += 9;
                        c.g[A2][0] = 4;
                        let slot = slogin_store_state_slot(c);
                        c.g[V0][0] = c.lw(slot);
                        c.g[A1][0] = c.g[V1][0];
                        c.sw(
                            (c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_KEY_LEN),
                            c.g[V1][0],
                        );
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_HEAP));
                        call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_c1bc)?;
                        let slot = slogin_store_state_slot(c);
                        c.g[V1][0] = c.lw(slot);
                        let failed = c.g[V0][0] == 0;
                        c.sw(
                            (c.g[V1][0] as u32).wrapping_add(STORE_CIPHER_KEY),
                            c.g[V0][0],
                        );
                        if failed {
                            break 'no_memory;
                        }
                        c.cycles += 4;
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_CIPHER_KEY_LEN));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_KEY));
                        callee::MEMCPY.call(c, 0x003f_c1d8)?;
                    }
                    c.cycles += 13;
                    let slot = slogin_store_state_slot(c);
                    c.g[V0][0] = c.lw(slot);
                    c.g[A2][0] = 4;
                    let config = c.g[S0][0] as u32;
                    c.g[V1][0] = c.lw(config.wrapping_add(STORE_CONFIG_MAX_RECORDS));
                    let state = c.g[V0][0] as u32;
                    c.sw(state.wrapping_add(STORE_DATA_END), 0);
                    c.sw(state.wrapping_add(STORE_RECORD_CAPACITY), c.g[V1][0]);
                    c.sw(state.wrapping_add(STORE_RECORD_COUNT), 0);
                    c.g[A1][0] = c.lw(config.wrapping_add(STORE_CONFIG_MAX_RECORDS));
                    c.g[A0][0] = c.lw(config.wrapping_add(STORE_CONFIG_HEAP));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
                    call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_c200)?;
                    let slot = slogin_store_state_slot(c);
                    c.g[A0][0] = c.lw(slot);
                    let allocated = c.g[V0][0] != 0;
                    c.sw((c.g[A0][0] as u32).wrapping_add(STORE_RECORDS), c.g[V0][0]);
                    if allocated {
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(STORE_HEAP));
                        c.g[A2][0] = 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(STORE_DIR_BUFFER));
                        c.g[A3][0] = 1;
                        callee::SLOGIN_STORE_ALLOC_BLOCK.call(c, 0x003f_c228)?;
                        if c.g[V0][0] != 0 {
                            frame.restore(c, S0, 0x10);
                            break 'epilogue;
                        }
                        c.cycles += 8;
                        let slot = slogin_store_state_slot(c);
                        c.g[V1][0] = c.lw(slot);
                        c.g[V0][0] = 0;
                        let state = c.g[V1][0] as u32;
                        c.g[A0][0] = c.lw(state.wrapping_add(STORE_DIR_BUFFER));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(BLOCK_HEADER_BYTES));
                        c.sw(state.wrapping_add(STORE_DIR_BUFFER), c.g[A0][0]);
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_CONFIG_HEAP));
                        c.sb(state.wrapping_add(STORE_FLAG_A8), 0);
                        c.sw(state.wrapping_add(STORE_HEAP), c.g[A1][0]);
                        break 'restore;
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 6;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Shutdown(void)` (0x3fc260): write the sentinel over the
/// directory block and free it, free the record directory, shut down the
/// cipher (freeing its key) and the checksum interface, shut down the
/// memory-card layer and free the store state. Returns 0, 2 when not
/// initialized, the block-free error, or the mapped cipher, hash or card
/// error.
pub fn slogin_store_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c270)?;
        'epilogue: {
            'restore_ra: {
                let initialized = c.g[V0][0] != 0;
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                if !initialized {
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    break 'restore_ra;
                }
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_DIR_BUFFER));
                callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_c288)?;
                let slot = slogin_store_state_slot(c);
                c.g[V1][0] = c.lw(slot);
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                let state = c.g[V1][0] as u32;
                c.g[V0][0] = c.lw(state.wrapping_add(STORE_DIR_BUFFER));
                c.g[A1][0] = c.lw(state.wrapping_add(STORE_HEAP));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BLOCK_HEADER_BYTES));
                c.g[A0][0] = c.g[V0][0];
                c.sw(state.wrapping_add(STORE_DIR_BUFFER), c.g[V0][0]);
                callee::SLOGIN_STORE_FREE_BLOCK.call(c, 0x003f_c2ac)?;
                let failed = c.g[V0][0] != 0;
                frame.restore_ra(c, 0);
                if failed {
                    break 'epilogue;
                }
                c.cycles += 11;
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                c.g[A3][0] = 4;
                let state = c.g[V0][0] as u32;
                c.g[A2][0] = c.lw(state.wrapping_add(STORE_RECORD_CAPACITY));
                c.g[A1][0] = c.lw(state.wrapping_add(STORE_RECORDS));
                c.g[A0][0] = c.lw(state.wrapping_add(STORE_HEAP));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
                call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_c2d0)?;
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_IFACE));
                let no_cipher = c.g[V1][0] == 0;
                c.sw((c.g[V0][0] as u32).wrapping_add(STORE_RECORDS), 0);
                if !no_cipher {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                    let cipher_shutdown = c.g[V0][0] as u32;
                    call_ptr(c, cipher_shutdown, 0x003f_c2ec)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V0][0];
                        callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, 0x003f_c2fc)?;
                        frame.restore_ra(c, 0);
                        break 'epilogue;
                    }
                    let slot = slogin_store_state_slot(c);
                    c.g[V0][0] = c.lw(slot);
                    c.cycles += 7;
                    c.g[A3][0] = 4;
                    let state = c.g[V0][0] as u32;
                    c.g[A2][0] = c.lw(state.wrapping_add(STORE_CIPHER_KEY_LEN));
                    c.g[A0][0] = c.lw(state.wrapping_add(STORE_HEAP));
                    c.g[A1][0] = c.lw(state.wrapping_add(STORE_CIPHER_KEY));
                    call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_c318)?;
                    let slot = slogin_store_state_slot(c);
                    c.g[V0][0] = c.lw(slot);
                    c.sw((c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_KEY), 0);
                }
                c.cycles += 8;
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_HASH_IFACE));
                c.sw((c.g[V0][0] as u32).wrapping_add(STORE_CIPHER_IFACE), 0);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                let hash_shutdown = c.g[V0][0] as u32;
                call_ptr(c, hash_shutdown, 0x003f_c338)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    callee::SLOGIN_STORE_MAP_HASH_ERROR.call(c, 0x003f_c348)?;
                    frame.restore_ra(c, 0);
                    break 'epilogue;
                }
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                c.cycles += 4;
                c.sw((c.g[V0][0] as u32).wrapping_add(STORE_HASH_IFACE), 0);
                call_ptr(c, callee::SLOGIN_MC_SHUTDOWN.addr, 0x003f_c358)?;
                let failed = c.g[V0][0] != 0;
                let slot = slogin_store_state_slot(c);
                c.g[A1][0] = c.lw(slot);
                if !failed {
                    c.cycles += 7;
                    c.g[A2][0] = STORE_STATE_BYTES;
                    c.g[A3][0] = 4;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(STORE_HEAP));
                    call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_c370)?;
                    let slot = slogin_store_state_slot(c);
                    c.sw(slot, 0);
                    c.g[V0][0] = 0;
                    break 'restore_ra;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c384)?;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// The shared shape of `SLoginStore_Create` and `SLoginStore_Delete`: mark
/// the store operation `op` (both the words at +0x74 and +0x78), start the
/// card operation at `start` with the caller's arguments and map its result.
fn slogin_store_start_card_op(c: &mut Cpu, op: u64, start: u32, start_ret: u32, map_ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        let slot = slogin_store_state_slot(c);
        c.g[V1][0] = c.lw(slot);
        frame.save_ra(c, 0);
        c.g[V0][0] = op;
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[V0][0],
        );
        c.sw((c.g[V1][0] as u32).wrapping_add(STORE_OPEN_ARG), c.g[V0][0]);
        call_ptr(c, start, start_ret)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, map_ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Create(a0, a1, a2)` (0x3fc390): mark store operation 1
/// and start creating the save file on the card (`SLoginMc_StartCreate` with
/// the same arguments); returns the mapped card error.
pub fn slogin_store_create(c: &mut Cpu) {
    slogin_store_start_card_op(
        c,
        1,
        callee::SLOGIN_MC_START_CREATE.addr,
        0x003f_c3ac,
        0x003f_c3b4,
    );
}

/// `int SLoginStore_Delete(a0, a1, a2)` (0x3fc3c0): mark store operation 2
/// and start deleting the save file (`SLoginMc_StartDelete` with the same
/// arguments); returns the mapped card error.
pub fn slogin_store_delete(c: &mut Cpu) {
    slogin_store_start_card_op(
        c,
        2,
        callee::SLOGIN_MC_START_DELETE.addr,
        0x003f_c3dc,
        0x003f_c3e4,
    );
}

/// `int SLoginStore_Open(a0, a1, a2)` (0x3fc4c8): `SLoginStore_BeginOpen`
/// with the same arguments and 4 as its fourth.
pub fn slogin_store_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 4;
        callee::SLOGIN_STORE_BEGIN_OPEN.call(c, 0x003f_c4d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Close(Directory* dir)` (0x3fc4e8): hash the directory
/// into its word at +0x60 and copy that hash into the store state, mark store
/// operation 3, clear the record count and data end, and start closing the
/// card file (`SLoginMc_StartClose(dir)`); returns the mapped card error.
pub fn slogin_store_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(STORE_DIR_HASH));
        callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_c500)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(STORE_DIR_HASH));
        c.g[A2][0] = 3;
        let slot = slogin_store_state_slot(c);
        c.g[A1][0] = c.lw(slot);
        c.g[A0][0] = c.g[S0][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(STORE_DIR_HASH), c.g[V1][0]);
        c.g[V0][0] = c.lw(slot);
        let state = c.g[V0][0] as u32;
        c.sw(state.wrapping_add(STORE_PENDING_OP), c.g[A2][0]);
        c.sw(state.wrapping_add(STORE_OPEN_ARG), c.g[A2][0]);
        c.sw(state.wrapping_add(STORE_RECORD_COUNT), 0);
        c.sw(state.wrapping_add(STORE_DATA_END), 0);
        call_ptr(c, callee::SLOGIN_MC_START_CLOSE.addr, 0x003f_c52c)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c534)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Write(Directory* dir, uint kind, uint occurrence,
/// void* buffer, uint size)` (0x3fc548): copy the directory header (0x64
/// bytes) into the store state, mark store operations 6/5, remember the
/// record buffer, size and kind, hash the payload, then update the CRC of the
/// existing record or add a new one, rehash the directory and start seeking
/// to the record's offset (`SLoginMc_StartSeek(dir, offset, 0)`). Returns the
/// hash error or the mapped card error.
pub fn slogin_store_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        let slot = slogin_store_state_slot(c);
        c.g[V1][0] = c.lw(slot);
        frame.save(c, S3, 0x30);
        c.g[T1][0] = 6;
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        c.g[T2][0] = c.g[V1][0];
        frame.save(c, S1, 0x50);
        c.g[A0][0] = 5;
        frame.save(c, S2, 0x40);
        c.g[V0][0] = c.g[S3][0] | c.g[T2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.sw(sp(c), 0);
        c.g[S4][0] = c.g[A2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(STORE_OPEN_ARG), c.g[T1][0]);
        c.g[V0][0] &= 7;
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[A0][0],
        );
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[S3][0];
        // Copy the directory header, 0x20 bytes per pass (src in v1, the
        // store state in t2).
        c.cycles += 2;
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(STORE_DIR_HASH));
        c.g[V0][0] = c.g[S1][0];
        if aligned {
            loop {
                c.cycles += 11;
                let (src, dst) = (c.g[V1][0] as u32, c.g[T2][0] as u32);
                c.g[A0][0] = c.ld(src);
                c.g[A1][0] = c.ld(src.wrapping_add(8));
                c.g[A2][0] = c.ld(src.wrapping_add(0x10));
                c.g[T1][0] = c.ld(src.wrapping_add(0x18));
                c.sd(dst, c.g[A0][0]);
                c.sd(dst.wrapping_add(8), c.g[A1][0]);
                c.sd(dst.wrapping_add(0x10), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c600);
                }
            }
        } else {
            loop {
                c.cycles += 19;
                let (src, dst) = (c.g[V1][0] as u32, c.g[T2][0] as u32);
                c.ldl(A0, src.wrapping_add(7));
                c.ldr(A0, src);
                c.ldl(A1, src.wrapping_add(0xf));
                c.ldr(A1, src.wrapping_add(8));
                c.ldl(A2, src.wrapping_add(0x17));
                c.ldr(A2, src.wrapping_add(0x10));
                c.ldl(T1, src.wrapping_add(0x1f));
                c.ldr(T1, src.wrapping_add(0x18));
                c.sdl(A0, dst.wrapping_add(7));
                c.sdr(A0, dst);
                c.sdl(A1, dst.wrapping_add(0xf));
                c.sdr(A1, dst.wrapping_add(8));
                c.sdl(A2, dst.wrapping_add(0x17));
                c.sdr(A2, dst.wrapping_add(0x10));
                c.sdl(T1, dst.wrapping_add(0x1f));
                c.sdr(T1, dst.wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c5a4);
                }
            }
            c.cycles += 2;
        }
        c.cycles += 17;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.sw(c.g[T2][0] as u32, c.g[V0][0]);
        let slot = slogin_store_state_slot(c);
        c.g[V0][0] = c.lw(slot);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        let state = c.g[V0][0] as u32;
        c.sw(state.wrapping_add(STORE_RECORD_SIZE), c.g[T0][0]);
        c.sw(state.wrapping_add(STORE_RECORD_BUFFER), c.g[A3][0]);
        c.sw(state.wrapping_add(STORE_RECORD_KIND), c.g[S0][0]);
        callee::SLOGIN_STORE_FIND_RECORD.call(c, 0x003f_c650)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A2][0] = c.g[SP][0];
        let slot = slogin_store_state_slot(c);
        c.g[V0][0] = c.lw(slot);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_SIZE));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(STORE_RECORD_BUFFER));
        callee::SLOGIN_STORE_HASH.call(c, 0x003f_c668)?;
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            // The hash result's CRC word (`*(result->crc)`) goes to the
            // store's record CRC.
            c.g[V0][0] = c.lw(sp(c));
            let slot = slogin_store_state_slot(c);
            c.g[A0][0] = c.lw(slot);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            c.lwl(A3, (c.g[V1][0] as u32).wrapping_add(3));
            c.lwr(A3, c.g[V1][0] as u32);
            c.swl(A3, (c.g[A0][0] as u32).wrapping_add(STORE_RECORD_CRC + 3));
            c.swr(A3, (c.g[A0][0] as u32).wrapping_add(STORE_RECORD_CRC));
            let found = c.g[S2][0] != 0;
            c.g[A2][0] = c.lw(slot);
            if !found {
                c.cycles += 8;
                c.g[V0][0] = c.lw(slot);
                c.g[A0][0] = c.g[S0][0];
                let state = c.g[V0][0] as u32;
                c.g[A1][0] = c.lw(state.wrapping_add(STORE_RECORD_SIZE));
                c.g[A2][0] = sx(state.wrapping_add(STORE_RECORD_CRC));
                c.g[S0][0] = c.lw(state.wrapping_add(STORE_DATA_END));
                callee::SLOGIN_STORE_ADD_RECORD.call(c, 0x003f_c6ac)?;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(RECORD_OFFSET));
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(STORE_RECORD_CRC));
                callee::SLOGIN_STORE_SET_RECORD_CRC.call(c, 0x003f_c6c8)?;
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[S1][0];
            callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_c6d0)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_c6e0)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c6e8)?;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Read(Directory* dir, uint kind, uint occurrence,
/// void* buffer, uint size)` (0x3fc708): copy the directory header into the
/// store state, mark store operations 8/5, remember the record buffer, size
/// and kind, find the record and, when its size matches, start seeking to
/// its offset (`SLoginMc_StartSeek(dir, offset, 0)`). Returns 7 when the
/// record is missing, 0x6e for a size mismatch, or the mapped card error.
pub fn slogin_store_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        let slot = slogin_store_state_slot(c);
        c.g[V1][0] = c.lw(slot);
        frame.save(c, S0, 0x20);
        c.g[T1][0] = 5;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 8;
        c.g[T2][0] = c.g[V1][0];
        c.g[T3][0] = c.g[A1][0];
        c.g[V0][0] = c.g[S0][0] | c.g[T2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(STORE_OPEN_ARG), c.g[A0][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] &= 7;
        let aligned = c.g[V0][0] == 0;
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(STORE_PENDING_OP),
            c.g[T1][0],
        );
        // Copy the directory header, 0x20 bytes per pass (src in a0, the
        // store state in t2).
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(STORE_DIR_HASH));
        if aligned {
            loop {
                c.cycles += 11;
                let (src, dst) = (c.g[A0][0] as u32, c.g[T2][0] as u32);
                c.g[V1][0] = c.ld(src);
                c.g[A2][0] = c.ld(src.wrapping_add(8));
                c.g[T0][0] = c.ld(src.wrapping_add(0x10));
                c.g[T1][0] = c.ld(src.wrapping_add(0x18));
                c.sd(dst, c.g[V1][0]);
                c.sd(dst.wrapping_add(8), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x10), c.g[T0][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                let more = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c7ac);
                }
            }
        } else {
            loop {
                c.cycles += 19;
                let (src, dst) = (c.g[A0][0] as u32, c.g[T2][0] as u32);
                c.ldl(V1, src.wrapping_add(7));
                c.ldr(V1, src);
                c.ldl(A2, src.wrapping_add(0xf));
                c.ldr(A2, src.wrapping_add(8));
                c.ldl(T0, src.wrapping_add(0x17));
                c.ldr(T0, src.wrapping_add(0x10));
                c.ldl(T1, src.wrapping_add(0x1f));
                c.ldr(T1, src.wrapping_add(0x18));
                c.sdl(V1, dst.wrapping_add(7));
                c.sdr(V1, dst);
                c.sdl(A2, dst.wrapping_add(0xf));
                c.sdr(A2, dst.wrapping_add(8));
                c.sdl(T0, dst.wrapping_add(0x17));
                c.sdr(T0, dst.wrapping_add(0x10));
                c.sdl(T1, dst.wrapping_add(0x1f));
                c.sdr(T1, dst.wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                let more = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c754);
                }
            }
            c.cycles += 2;
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.sw(c.g[T2][0] as u32, c.g[V0][0]);
        let slot = slogin_store_state_slot(c);
        c.g[V0][0] = c.lw(slot);
        c.g[A0][0] = c.g[T3][0];
        let state = c.g[V0][0] as u32;
        c.sw(state.wrapping_add(STORE_RECORD_BUFFER), c.g[A3][0]);
        c.sw(state.wrapping_add(STORE_RECORD_SIZE), c.g[S1][0]);
        c.sw(state.wrapping_add(STORE_RECORD_KIND), c.g[T3][0]);
        callee::SLOGIN_STORE_FIND_RECORD.call(c, 0x003f_c7f8)?;
        'epilogue: {
            'failed: {
                c.g[V1][0] = c.g[V0][0];
                let missing = c.g[V1][0] == 0;
                c.g[V0][0] = 7;
                if missing {
                    break 'failed;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(RECORD_SIZE));
                let other_size = c.g[S1][0] != c.g[V0][0];
                c.g[V0][0] = 0x6e;
                if other_size {
                    break 'failed;
                }
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(RECORD_OFFSET));
                c.g[A0][0] = c.g[S0][0];
                let slot = slogin_store_state_slot(c);
                c.g[V0][0] = c.lw(slot);
                c.g[A2][0] = 0;
                c.g[A1][0] = c.g[V1][0];
                c.sw(
                    (c.g[V0][0] as u32).wrapping_add(STORE_SEEK_OFFSET),
                    c.g[V1][0],
                );
                call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_c82c)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c834)?;
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Control block of the store fakes (in each test machine's RAM).
    const STORE_FAKE_CTRL: u32 = DATA + 0xf000;
    const STORE_FAKE_MAGIC: u32 = 0x534c_5354;
    /// Allocations made so far, and the pointer each one returns (0 fails).
    const STORE_FAKE_ALLOC_COUNT: u32 = STORE_FAKE_CTRL + 4;
    const STORE_FAKE_ALLOC_RESULTS: u32 = STORE_FAKE_CTRL + 0x10;
    /// Interface calls made so far, the index of the one that fails (with
    /// the error below), and the hash result `HASH_FINAL` hands back.
    const STORE_FAKE_IFACE_COUNT: u32 = STORE_FAKE_CTRL + 0x30;
    const STORE_FAKE_IFACE_FAIL_AT: u32 = STORE_FAKE_CTRL + 0x34;
    const STORE_FAKE_IFACE_ERROR: u32 = STORE_FAKE_CTRL + 0x38;
    const STORE_FAKE_HASH_RESULT: u32 = STORE_FAKE_CTRL + 0x3c;

    /// Fake checksum and cipher interface tables (slot i is `FAKE + base +
    /// 0x10 * i`); slot 5 of the checksum table (+0x14) is the hash's final
    /// call, which writes its result pointer.
    const STORE_FAKE_HASH_TABLE: u32 = DATA + 0xa000;
    const STORE_FAKE_CIPHER_TABLE: u32 = DATA + 0xa100;
    const STORE_FAKE_HASH_FINAL: u32 = FAKE + 0x100 + 0x50;

    const STORE_LIFECYCLE_TEST_STATE: u32 = DATA + 0x1000;
    const STORE_TEST_HASH_RESULT: u32 = DATA + 0xb000;
    const STORE_TEST_HASH_CRC: u32 = DATA + 0xb010;
    /// `gp+0x1aec` / `gp+0x1ae8`: the memory-card layer's interface and state.
    const STORE_TEST_MC_IFACE_GP: u32 = 0x1aec;
    const STORE_TEST_MC_STATE_GP: u32 = 0x1ae8;

    static STORE_ALLOC_PREV: AtomicUsize = AtomicUsize::new(0);
    static STORE_FREE_PREV: AtomicUsize = AtomicUsize::new(0);

    fn slogin_store_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(STORE_FAKE_CTRL) != STORE_FAKE_MAGIC {
            return fakes::fall_back(c, &STORE_ALLOC_PREV, ee::s07::f_00266728__generated);
        }
        let k = c.bus.read32(STORE_FAKE_ALLOC_COUNT);
        c.bus.write32(STORE_FAKE_ALLOC_COUNT, k + 1);
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0026_6728, ret);
        c.g[V0][0] = if k < 8 {
            c.bus.read32(STORE_FAKE_ALLOC_RESULTS + 4 * k) as u64
        } else {
            0
        };
    }

    fn slogin_store_fake_free(c: &mut Cpu) {
        if c.bus.read32(STORE_FAKE_CTRL) != STORE_FAKE_MAGIC {
            return fakes::fall_back(c, &STORE_FREE_PREV, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, 0x0026_6760);
    }

    fn slogin_store_hook_heap_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            slogin_store_fake_alloc,
            &STORE_ALLOC_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            slogin_store_fake_free,
            &STORE_FREE_PREV,
        );
    }

    /// The `miss` hook for the fake interface slots: log the call, return 0
    /// (or the error at the chosen call index), and for the hash's final call
    /// write the result pointer through its first argument.
    fn slogin_store_fake_iface(c: &mut Cpu, target: u32) {
        let out = c.g[A0][0] as u32;
        let n = c.bus.read32(STORE_FAKE_IFACE_COUNT);
        c.bus.write32(STORE_FAKE_IFACE_COUNT, n + 1);
        fakes::fake_call(c, target);
        c.g[V0][0] = if n == c.bus.read32(STORE_FAKE_IFACE_FAIL_AT) {
            c.bus.read32(STORE_FAKE_IFACE_ERROR) as u64
        } else {
            0
        };
        if target == STORE_FAKE_HASH_FINAL {
            let result = c.bus.read32(STORE_FAKE_HASH_RESULT);
            c.bus.write32(out, result);
        }
    }

    /// A test machine for the store: fakes armed (unwinding at fake call
    /// `unwind_at`), interface tables, the state block at
    /// `STORE_LIFECYCLE_TEST_STATE` filled with junk and pointed to by `gp+0x2a30`,
    /// and the hash result `{len, &crc}` at `STORE_TEST_HASH_RESULT`.
    fn slogin_store_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32, unwind_at: u32) -> Rng {
        let mut r = fakes::machine(c, seed, salt, 0);
        c.pc = entry;
        c.hooks.miss = slogin_store_fake_iface;
        c.bus.write32(fakes::LOG_UNWIND_AT, unwind_at);
        c.bus.write32(STORE_FAKE_CTRL, STORE_FAKE_MAGIC);
        c.bus.write32(STORE_FAKE_IFACE_FAIL_AT, u32::MAX);
        c.bus.write32(STORE_FAKE_IFACE_ERROR, 3 + seed % 5);
        c.bus
            .write32(STORE_FAKE_HASH_RESULT, STORE_TEST_HASH_RESULT);
        for i in 0..8 {
            c.bus
                .write32(STORE_FAKE_HASH_TABLE + 4 * i, FAKE + 0x100 + 0x10 * i);
            c.bus
                .write32(STORE_FAKE_CIPHER_TABLE + 4 * i, FAKE + 0x200 + 0x10 * i);
        }
        fill(c, &mut r, STORE_LIFECYCLE_TEST_STATE, 0xb0 / 4);
        c.bus
            .write32(GAME_GP + STORE_STATE_GP, STORE_LIFECYCLE_TEST_STATE);
        c.bus.write32(
            STORE_LIFECYCLE_TEST_STATE + STORE_HASH_IFACE,
            STORE_FAKE_HASH_TABLE,
        );
        c.bus
            .write32(STORE_LIFECYCLE_TEST_STATE + STORE_CIPHER_IFACE, 0);
        c.bus.write32(STORE_TEST_HASH_RESULT, 4);
        c.bus
            .write32(STORE_TEST_HASH_RESULT + 4, STORE_TEST_HASH_CRC);
        c.bus.write32(STORE_TEST_HASH_CRC, 0xc0de_0000 | seed);
        r
    }

    #[test]
    fn slogin_store_verify_record_matches() {
        const HEADER: u32 = DATA + 0x2000;
        check(
            ee::s15::f_003fbfe0__generated,
            slogin_store_verify_record,
            144,
            &|c, seed| {
                // seed % 8: the arm; seed / 8 % 2: cipher present (arms 0 and 3
                // always); seed / 16 % 3: which interface call of the failing
                // step fails; seed / 48: no unwind, or unwind in the first or
                // second group of three interface calls.
                let class = seed % 8;
                let cipher = matches!(class, 0 | 3) || seed / 8 % 2 == 1;
                let step = seed / 16 % 3;
                let unwind_at = match seed / 48 {
                    0 => u32::MAX,
                    g => 3 * (g - 1) + step,
                };
                let mut r = slogin_store_machine(c, seed, 0x3fbf_e000, 0x003f_bfe0, unwind_at);
                let state = STORE_LIFECYCLE_TEST_STATE;
                if cipher {
                    c.bus
                        .write32(state + STORE_CIPHER_IFACE, STORE_FAKE_CIPHER_TABLE);
                }
                let cipher_calls = if cipher { 3 } else { 0 };
                c.bus.write32(
                    STORE_FAKE_IFACE_FAIL_AT,
                    match class {
                        0 => step,
                        3 => 3 + step,
                        4 => 2 * cipher_calls + step,
                        _ => u32::MAX,
                    },
                );
                let kind = 0x11 + seed % 5;
                let size = 0x40 + seed % 7;
                let crc = 0xc0de_0000 | seed;
                c.bus
                    .write32(state + STORE_RECORD_BUFFER, HEADER + BLOCK_HEADER_BYTES);
                c.bus.write32(state + STORE_RECORD_KIND, kind);
                c.bus.write32(state + STORE_RECORD_SIZE, size);
                c.bus
                    .write32(HEADER, if class == 1 { kind ^ 1 } else { kind });
                c.bus
                    .write32(HEADER + 4, if class == 2 { size + 1 } else { size });
                c.bus.write32(HEADER + 8, crc);
                fill(c, &mut r, HEADER + BLOCK_HEADER_BYTES, 0x48 / 4);
                c.bus
                    .write32(STORE_TEST_HASH_RESULT, if class == 5 { 5 } else { 4 });
                c.bus.write32(
                    STORE_TEST_HASH_CRC,
                    if class == 7 { crc ^ 0x100 } else { crc },
                );
            },
        );
    }

    #[test]
    fn slogin_store_is_initialized_matches() {
        check(
            ee::s15::f_003fc0d8__generated,
            slogin_store_is_initialized,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fc0_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_c0d8;
                c.g[GP][0] = GAME_GP as u64;
                let state = match seed % 3 {
                    0 => 0,
                    1 => DATA,
                    _ => 0x8000_0000,
                };
                c.bus.write32(GAME_GP + STORE_STATE_GP, state);
            },
        );
    }

    #[test]
    fn slogin_store_init_matches() {
        const CONFIG: u32 = DATA + 0x3000;
        const DEVICES: u32 = DATA + 0x3100;
        const MC_FUNCS: u32 = DATA + 0x3200;
        const KEY: u32 = DATA + 0x3300;
        const HEAP: u32 = DATA + 0x3400;
        slogin_store_hook_heap_fakes();
        check(
            ee::s15::f_003fc0e8__generated,
            slogin_store_init,
            120,
            &|c, seed| {
                // seed % 10: the arm (0 already initialized, 1 card layer fails,
                // 2 state allocation fails, 3 checksum init fails, 4 cipher init
                // fails, 5 key allocation fails, 6 directory allocation fails,
                // 7 block allocation fails, 8 success, 9 success with a cipher);
                // seed / 10 % 2: cipher present for the arms where it is free;
                // seed / 20: unwind at fake call 1..5 (0: none).
                let class = seed % 10;
                let cipher = matches!(class, 4 | 5 | 9) || (class != 8 && seed / 10 % 2 == 1);
                let unwind_at = match seed / 20 % 6 {
                    0 => u32::MAX,
                    u => u,
                };
                crate::difftest::elf_image(c);
                let mut r = slogin_store_machine(c, seed, 0x3fc0_e800, 0x003f_c0e8, unwind_at);
                c.bus.write32(
                    GAME_GP + STORE_STATE_GP,
                    if class == 0 {
                        STORE_LIFECYCLE_TEST_STATE
                    } else {
                        0
                    },
                );
                c.bus.write32(GAME_GP + STORE_TEST_MC_IFACE_GP, 0);
                c.bus.write32(GAME_GP + STORE_TEST_MC_STATE_GP, 0);
                // Crc32_GetInterface's table: the checksum interface Init sets up.
                for i in 0..8 {
                    c.bus.write32(0x0049_7508 + 4 * i, FAKE + 0x100 + 0x10 * i);
                }
                c.bus.write32(
                    STORE_FAKE_IFACE_FAIL_AT,
                    match class {
                        3 => 0,
                        4 => 1,
                        _ => u32::MAX,
                    },
                );
                c.g[A0][0] = sx(CONFIG);
                c.bus
                    .write32(CONFIG + STORE_CONFIG_MAX_RECORDS, 2 + seed % 3);
                c.bus.write32(
                    CONFIG + STORE_CONFIG_DEVICES,
                    if class == 1 { 0 } else { DEVICES },
                );
                c.bus.write32(CONFIG + STORE_CONFIG_MC_FUNCS, MC_FUNCS);
                c.bus.write32(CONFIG + STORE_CONFIG_KEY, KEY);
                c.bus
                    .write32(CONFIG + STORE_CONFIG_KEY_LEN, 0x10 + seed % 0x20);
                c.bus.write32(
                    CONFIG + STORE_CONFIG_CIPHER,
                    if cipher { STORE_FAKE_CIPHER_TABLE } else { 0 },
                );
                c.bus.write32(CONFIG + STORE_CONFIG_HEAP, HEAP);
                c.bus.write32(DEVICES, 0);
                c.bus.write32(DEVICES + 4, u32::MAX);
                for i in 0..17 {
                    c.bus.write32(MC_FUNCS + 4 * i, FAKE + 0x300 + 0x10 * i);
                }
                fill(c, &mut r, KEY, 0x30 / 4);
                let mut results = vec![DATA + 0x4000, DATA + 0x4800];
                results.push(if class == 2 {
                    0
                } else {
                    STORE_LIFECYCLE_TEST_STATE
                });
                if cipher {
                    results.push(if class == 5 { 0 } else { DATA + 0x5000 });
                }
                results.push(if class == 6 { 0 } else { DATA + 0x5400 });
                results.push(if class == 7 { 0 } else { DATA + 0x5800 });
                for (k, p) in results.iter().enumerate() {
                    c.bus.write32(STORE_FAKE_ALLOC_RESULTS + 4 * k as u32, *p);
                }
                fill(c, &mut r, DATA + 0x5000, 0x40 / 4);
                fill(c, &mut r, DATA + 0x5800, 0x40 / 4);
            },
        );
    }

    #[test]
    fn slogin_store_shutdown_matches() {
        const BLOCK: u32 = DATA + 0x6000;
        slogin_store_hook_heap_fakes();
        check(
            ee::s15::f_003fc260__generated,
            slogin_store_shutdown,
            96,
            &|c, seed| {
                // seed % 6: the arm (0 not initialized, 1 cipher shutdown fails,
                // 2 checksum shutdown fails, 3 card layer shutdown fails,
                // 4 success, 5 success with a cipher); seed / 6 % 2: cipher
                // present for arms 2 and 3; seed / 12: unwind at fake call
                // 0..6 (0: none).
                let class = seed % 6;
                let cipher =
                    matches!(class, 1 | 5) || (matches!(class, 2 | 3) && seed / 6 % 2 == 1);
                let unwind_at = match seed / 12 % 8 {
                    0 => u32::MAX,
                    u => u - 1,
                };
                crate::difftest::elf_image(c);
                let mut r = slogin_store_machine(c, seed, 0x3fc2_6000, 0x003f_c260, unwind_at);
                let state = STORE_LIFECYCLE_TEST_STATE;
                if class == 0 {
                    c.bus.write32(GAME_GP + STORE_STATE_GP, 0);
                }
                c.bus.write32(state + STORE_DIR_BUFFER, BLOCK);
                fill(c, &mut r, BLOCK, 0x20 / 4);
                c.bus.write32(state + STORE_HEAP, DATA + 0x3400);
                c.bus.write32(state + STORE_RECORD_CAPACITY, 2 + seed % 4);
                c.bus.write32(state + STORE_RECORDS, DATA + 0x5400);
                if cipher {
                    c.bus
                        .write32(state + STORE_CIPHER_IFACE, STORE_FAKE_CIPHER_TABLE);
                }
                c.bus.write32(
                    STORE_FAKE_IFACE_FAIL_AT,
                    match class {
                        1 => 0,
                        2 => cipher as u32,
                        _ => u32::MAX,
                    },
                );
                c.bus.write32(
                    GAME_GP + STORE_TEST_MC_IFACE_GP,
                    if class == 3 { 0 } else { DATA + 0x4000 },
                );
                c.bus
                    .write32(GAME_GP + STORE_TEST_MC_STATE_GP, DATA + 0x4800);
                c.bus.write32(DATA + 0x4000 + 0x428, DATA + 0x3400);
            },
        );
    }

    /// The card layer's short paths: not initialized (`SLoginMc_*` return 2),
    /// or initialized and busy (the state word at `gp+0x1ae8` points at a
    /// non-zero word: the seek and close starts return 0xd).
    fn slogin_store_card_busy(c: &mut Cpu, busy: bool) {
        c.bus.write32(
            GAME_GP + STORE_TEST_MC_IFACE_GP,
            if busy { DATA + 0x4000 } else { 0 },
        );
        c.bus
            .write32(GAME_GP + STORE_TEST_MC_STATE_GP, DATA + 0x4800);
        c.bus.write32(DATA + 0x4800, 1);
    }

    #[test]
    fn slogin_store_create_matches() {
        check(
            ee::s15::f_003fc390__generated,
            slogin_store_create,
            48,
            &|c, seed| {
                let _r = slogin_store_machine(c, seed, 0x3fc3_9000, 0x003f_c390, u32::MAX);
                // seed % 2: card layer not initialized (2), or initialized with
                // a null first argument (0xc).
                let not_init = seed % 2 == 0;
                c.bus.write32(
                    GAME_GP + STORE_TEST_MC_IFACE_GP,
                    if not_init { 0 } else { DATA + 0x4000 },
                );
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn slogin_store_delete_matches() {
        check(
            ee::s15::f_003fc3c0__generated,
            slogin_store_delete,
            48,
            &|c, seed| {
                let _r = slogin_store_machine(c, seed, 0x3fc3_c000, 0x003f_c3c0, u32::MAX);
                let not_init = seed % 2 == 0;
                c.bus.write32(
                    GAME_GP + STORE_TEST_MC_IFACE_GP,
                    if not_init { 0 } else { DATA + 0x4000 },
                );
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn slogin_store_open_matches() {
        check(
            ee::s15::f_003fc4c8__generated,
            slogin_store_open,
            48,
            &|c, seed| {
                let _r = slogin_store_machine(c, seed, 0x3fc4_c800, 0x003f_c4c8, u32::MAX);
                let not_init = seed % 2 == 0;
                c.bus.write32(
                    GAME_GP + STORE_TEST_MC_IFACE_GP,
                    if not_init { 0 } else { DATA + 0x4000 },
                );
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn slogin_store_close_matches() {
        const DIR: u32 = DATA + 0x7000;
        check(
            ee::s15::f_003fc4e8__generated,
            slogin_store_close,
            72,
            &|c, seed| {
                // seed % 2: empty directory (no hash) or hashed; seed / 2 % 2:
                // card layer not initialized or busy; seed / 6 % 4: unwind at
                // hash call 0..2 (3: none).
                let hashed = seed % 2 == 1;
                let unwind_at = match seed / 6 % 4 {
                    3 => u32::MAX,
                    u => u,
                };
                let mut r = slogin_store_machine(c, seed, 0x3fc4_e800, 0x003f_c4e8, unwind_at);
                c.bus.write32(
                    STORE_LIFECYCLE_TEST_STATE + STORE_RECORD_COUNT,
                    if hashed { 1 + seed % 3 } else { 0 },
                );
                c.bus
                    .write32(STORE_LIFECYCLE_TEST_STATE + STORE_RECORDS, DATA + 0x5400);
                fill(c, &mut r, DIR, 0x70 / 4);
                slogin_store_card_busy(c, seed / 2 % 2 == 1);
                c.g[A0][0] = sx(DIR);
            },
        );
    }

    /// The directory (3 records: kinds 5, 7, 5), junk directory-header bytes
    /// at `DATA + 0x7000` and the card layer's short path.
    fn slogin_store_record_fixture(c: &mut Cpu, r: &mut Rng, capacity: u32, mc_busy: bool) {
        const RECORDS: u32 = DATA + 0x8000;
        let state = c.bus.read32(GAME_GP + STORE_STATE_GP);
        c.bus.write32(state + STORE_RECORD_COUNT, 3);
        c.bus.write32(state + STORE_RECORD_CAPACITY, capacity);
        c.bus.write32(state + STORE_RECORDS, RECORDS);
        c.bus.write32(state + STORE_DATA_END, 0x2000);
        fill(c, r, RECORDS, 0x50 / 4);
        for (i, kind) in [5u32, 7, 5].iter().enumerate() {
            let rec = RECORDS + 0x10 * i as u32;
            c.bus.write32(rec, *kind);
            c.bus.write32(rec + RECORD_OFFSET, 0x400 * (i as u32 + 1));
            c.bus.write32(rec + RECORD_SIZE, 0x40 + i as u32);
        }
        fill(c, r, DATA + 0x7000, 0x70 / 4);
        slogin_store_card_busy(c, mc_busy);
    }

    /// Store state and directory-header addresses for the alignment case
    /// `seed % 4`: both 8-aligned, the source 4- or 1-off, or the state 4-off.
    fn slogin_store_copy_addrs(seed: u32) -> (u32, u32) {
        match seed % 4 {
            0 => (STORE_LIFECYCLE_TEST_STATE, DATA + 0x7000),
            1 => (STORE_LIFECYCLE_TEST_STATE, DATA + 0x7004),
            2 => (STORE_LIFECYCLE_TEST_STATE, DATA + 0x7001),
            _ => (STORE_LIFECYCLE_TEST_STATE + 4, DATA + 0x7000),
        }
    }

    fn slogin_store_write_fixture(c: &mut Cpu, seed: u32, unwind_at: u32) -> Rng {
        // seed % 4: alignment; seed / 4 % 3: hash fails, record found,
        // record missing; seed / 12 % 3: which hash call fails / which
        // occurrence; seed / 24 % 2: card layer not initialized or busy;
        // seed / 48 % 2: directory full or not.
        let mut r = slogin_store_machine(c, seed, 0x3fc5_4800, 0x003f_c548, unwind_at);
        let (state, src) = slogin_store_copy_addrs(seed);
        c.bus.write32(GAME_GP + STORE_STATE_GP, state);
        c.bus
            .write32(state + STORE_HASH_IFACE, STORE_FAKE_HASH_TABLE);
        let arm = seed / 4 % 3;
        let digit = seed / 12 % 3;
        slogin_store_record_fixture(c, &mut r, 3 + seed / 48 % 2, seed / 24 % 2 == 1);
        c.bus.write32(
            STORE_FAKE_IFACE_FAIL_AT,
            if arm == 0 { digit } else { u32::MAX },
        );
        c.g[A0][0] = sx(src);
        c.g[A1][0] = match arm {
            1 => 5,
            _ => {
                if digit == 0 {
                    9
                } else {
                    5
                }
            }
        };
        c.g[A2][0] = match arm {
            1 => (digit % 2) as u64,
            _ => 2,
        };
        c.g[A3][0] = sx(DATA + 0x9000);
        c.g[T0][0] = (0x40 + seed % 9) as u64;
        r
    }

    #[test]
    fn slogin_store_write_matches() {
        check(
            ee::s15::f_003fc548__generated,
            slogin_store_write,
            96,
            &|c, seed| {
                // seed / 12 % 8: unwind at hash call 0..5 (6, 7: none).
                let unwind_at = match seed / 12 % 8 {
                    u if u < 6 => u,
                    _ => u32::MAX,
                };
                let _r = slogin_store_write_fixture(c, seed, unwind_at);
            },
        );
    }

    crate::difftest::events::driven!(driven_slogin_store_write, slogin_store_write);

    #[test]
    fn slogin_store_write_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc548__generated,
            driven_slogin_store_write,
            48,
            &|c, seed| {
                // seed % 3: the event mode, crossed with the fixture's digits.
                let mut r = slogin_store_write_fixture(c, seed, u32::MAX);
                schedule(c, &mut r, seed, 0x003f_c548)
            },
        );
    }

    fn slogin_store_read_fixture(c: &mut Cpu, seed: u32) -> Rng {
        // seed % 4: alignment; seed / 4 % 3: record missing, size mismatch,
        // found; seed / 12 % 2: card layer not initialized or busy;
        // seed / 24 % 2: which occurrence of kind 5.
        let mut r = slogin_store_machine(c, seed, 0x3fc7_0800, 0x003f_c708, u32::MAX);
        let (state, src) = slogin_store_copy_addrs(seed);
        c.bus.write32(GAME_GP + STORE_STATE_GP, state);
        let arm = seed / 4 % 3;
        let occurrence = seed / 24 % 2;
        slogin_store_record_fixture(c, &mut r, 3, seed / 12 % 2 == 1);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = if arm == 0 { 9 } else { 5 };
        c.g[A2][0] = occurrence as u64;
        c.g[A3][0] = sx(DATA + 0x9000);
        let size = 0x40 + 2 * occurrence;
        c.g[T0][0] = (if arm == 1 { size + 1 } else { size }) as u64;
        r
    }

    #[test]
    fn slogin_store_read_matches() {
        check(
            ee::s15::f_003fc708__generated,
            slogin_store_read,
            96,
            &|c, seed| {
                let _r = slogin_store_read_fixture(c, seed);
            },
        );
    }

    crate::difftest::events::driven!(driven_slogin_store_read, slogin_store_read);

    #[test]
    fn slogin_store_read_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc708__generated,
            driven_slogin_store_read,
            48,
            &|c, seed| {
                // seed % 3: the event mode, crossed with the fixture's digits.
                let mut r = slogin_store_read_fixture(c, seed);
                schedule(c, &mut r, seed, 0x003f_c708)
            },
        );
    }
}
