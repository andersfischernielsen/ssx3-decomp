//! Staging module oc1030 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1030`.

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
        0x003f_c850,
        slogin_store_count_records,
        "SLoginStore_CountRecords",
    ),
    (
        0x003f_c8f8,
        slogin_store_get_record_size,
        "SLoginStore_GetRecordSize",
    ),
    (0x003f_c990, slogin_store_get_info, "SLoginStore_GetInfo"),
    (
        0x003f_cdc8,
        slogin_store_set_icon_sys,
        "SLoginStore_SetIconSys",
    ),
    (
        0x003f_ce10,
        slogin_store_alloc_block,
        "SLoginStore_AllocBlock",
    ),
    (
        0x003f_cf00,
        slogin_store_free_block,
        "SLoginStore_FreeBlock",
    ),
    (
        0x003f_cf98,
        slogin_store_get_block_size,
        "SLoginStore_GetBlockSize",
    ),
    (0x003f_cfd8, dnas_rel_load, "DnasRel_Load"),
    (0x003f_d0d8, dnas_rel_call_init, "DnasRel_CallInit"),
];

mod callee {
    use super::*;

    pub(crate) const SLOGIN_MC_SET_ICON_SYS: Callee = Callee {
        addr: 0x003f_a240,
        f: ee::s14::SLoginMc_SetIconSys,
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
    pub(crate) const SLOGIN_STORE_FIND_RECORD: Callee = Callee {
        addr: 0x003f_ba20,
        f: ee::s15::SLoginStore_FindRecord,
    };
    pub(crate) const SLOGIN_STORE_MAP_MC_ERROR: Callee = Callee {
        addr: 0x003f_b810,
        f: ee::s15::SLoginStore_MapMcError,
    };
    pub(crate) const SLOGIN_STORE_GET_ALIGNMENT: Callee = Callee {
        addr: 0x003f_b858,
        f: ee::s15::SLoginStore_GetAlignment,
    };
    pub(crate) const SLOGIN_STORE_CALC_ALLOC_SIZE: Callee = Callee {
        addr: 0x003f_b898,
        f: ee::s15::SLoginStore_CalcAllocSize,
    };
    pub(crate) const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };
    pub(crate) const DNAS_REL_ELF_OPEN: Callee = Callee {
        addr: 0x003f_d558,
        f: ee::s15::DnasRel_ElfOpen,
    };
    pub(crate) const DNAS_REL_FIND_SYMBOL: Callee = Callee {
        addr: 0x003f_d758,
        f: ee::s15::DnasRel_FindSymbol,
    };
    pub(crate) const DNAS_REL_RELOCATE_ALL: Callee = Callee {
        addr: 0x003f_d660,
        f: ee::s15::DnasRel_RelocateAll,
    };
    pub(crate) const FLUSH_CACHE: Callee = Callee {
        addr: 0x0042_4020,
        f: ee::s15::FlushCache,
    };
}

const STORE_STATE_GP: u32 = 0x2a30;
const STORE_RECORD_COUNT: u32 = 0x94;
const STORE_RECORDS: u32 = 0x9c;
const STORE_RECORD_STRIDE: u32 = 0x10;
const RECORD_SIZE: u32 = 8;
/// `SLoginStore_GetInfo` copies the state's first 0x60 bytes in 32-byte
/// steps, then one more word.
const STORE_INFO_BLOCK_BYTES: u32 = 0x60;
const BLOCK_HEADER_BYTES: u32 = 12;

/// `int SLoginStore_CountRecords(void* store, uint kind, int* count)`
/// (0x3fc850): count the directory records of `kind` into `*count`.
/// Returns 2 when the store is not initialized, 4 when `store` or `count` is
/// null, else 0.
pub fn slogin_store_count_records(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c874)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        let mut s0_restored = false;
        'done: {
            if !initialized {
                break 'done;
            }
            c.cycles += 2;
            let no_store = c.g[S1][0] == 0;
            c.g[V0][0] = 4;
            if no_store {
                break 'done;
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                frame.restore(c, S0, 0x30);
                s0_restored = true;
                break 'done;
            }
            c.sw(c.g[S0][0] as u32, 0);
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORD_COUNT));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(STORE_RECORDS));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if empty {
                break 'done;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            loop {
                c.cycles += 2;
                let other_kind = c.g[S2][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(STORE_RECORD_STRIDE));
                if !other_kind {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'done;
                }
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c8b8);
                }
            }
        }
        if !s0_restored {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_GetRecordSize(void* store, uint kind, uint occurrence,
/// uint* size)` (0x3fc8f8): find the given occurrence of a record kind and
/// write its size word (record + 8) to `*size`, which is cleared first.
/// Returns 2 when the store is not initialized, 4 when `store` or `size` is
/// null, 7 when no such record exists, else 0.
pub fn slogin_store_get_record_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = 0;
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c92c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        let mut s0_restored = false;
        'done: {
            if !initialized {
                break 'done;
            }
            c.cycles += 2;
            let no_store = c.g[S1][0] == 0;
            c.g[V0][0] = 4;
            if no_store {
                break 'done;
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                frame.restore(c, S0, 0x50);
                s0_restored = true;
                break 'done;
            }
            c.sw(c.g[S0][0] as u32, 0);
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            callee::SLOGIN_STORE_FIND_RECORD.call(c, 0x003f_c958)?;
            if c.g[V0][0] == 0 {
                c.g[S4][0] = 7;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RECORD_SIZE));
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
        }
        if !s0_restored {
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

/// `int SLoginStore_GetInfo(void* out)` (0x3fc990): copy the first 0x64
/// bytes of the store state into `out` (doubleword copies when both
/// pointers are 8-byte aligned, unaligned `ldl`/`sdl` pairs otherwise).
/// Returns 4 when `out` is null, else 0.
pub fn slogin_store_get_info(c: &mut Cpu) {
    c.cycles += 2;
    let has_out = c.g[A0][0] != 0;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(STORE_STATE_GP));
    if !has_out {
        c.cycles += 2;
        c.g[V0][0] = 4;
        ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(STORE_INFO_BLOCK_BYTES));
    if aligned {
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            let dst = c.g[A0][0] as u32;
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_ca04);
            }
        }
    } else {
        loop {
            c.cycles += 19;
            let src = c.g[V1][0] as u32;
            c.ldl(A1, src.wrapping_add(7));
            c.ldr(A1, src);
            c.ldl(A2, src.wrapping_add(0xf));
            c.ldr(A2, src.wrapping_add(8));
            c.ldl(A3, src.wrapping_add(0x17));
            c.ldr(A3, src.wrapping_add(0x10));
            c.ldl(T0, src.wrapping_add(0x1f));
            c.ldr(T0, src.wrapping_add(0x18));
            let dst = c.g[A0][0] as u32;
            c.sdl(A1, dst.wrapping_add(7));
            c.sdr(A1, dst);
            c.sdl(A2, dst.wrapping_add(0xf));
            c.sdr(A2, dst.wrapping_add(8));
            c.sdl(A3, dst.wrapping_add(0x17));
            c.sdr(A3, dst.wrapping_add(0x10));
            c.sdl(T0, dst.wrapping_add(0x1f));
            c.sdr(T0, dst.wrapping_add(0x18));
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_c9b0);
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V0][0] = 0;
    ret(c);
}

/// `int SLoginStore_SetIconSys(const IconSys* icon)` (0x3fcdc8): hand the
/// save icon description to the memory-card layer and map its result into
/// the store's error range. Returns 2 when the store is not initialized.
pub fn slogin_store_set_icon_sys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_cddc)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SLOGIN_MC_SET_ICON_SYS.addr, 0x003f_cdec)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_cdf4)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_AllocBlock(void** out, Heap* heap, uint payload_bytes,
/// int format)` (0x3fce10): allocate a zeroed block for a record payload
/// (header included, rounded to the format's alignment), stamp the 12-byte
/// header pattern at its start and store the payload pointer (block + 12)
/// in `*out`. Returns 4 when `out` is null, 6 when the allocation fails,
/// else 0.
pub fn slogin_store_alloc_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A2][0] = c.g[SP][0] | 4;
        c.sw(sp(c).wrapping_add(4), c.g[V1][0]);
        callee::SLOGIN_STORE_GET_ALIGNMENT.call(c, 0x003f_ce58)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_ce64)?;
        let has_out = c.g[S2][0] != 0;
        c.g[S0][0] = c.g[V0][0];
        'done: {
            if !has_out {
                c.cycles += 2;
                c.g[V0][0] = 4;
                break 'done;
            }
            c.cycles += 6;
            c.g[A2][0] = c.lw(sp(c));
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_ce84)?;
            let allocated = c.g[V0][0] != 0;
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            if !allocated {
                c.cycles += 2;
                c.g[V0][0] = 6;
                break 'done;
            }
            c.cycles += 19;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::MEMSET.call(c, 0x003f_cea4)?;
            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = 0x49_0000;
            c.g[V0][0] = 0;
            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7550));
            let pattern = c.g[T0][0] as u32;
            c.ldl(V1, pattern.wrapping_add(7));
            c.ldr(V1, pattern);
            c.lwl(A2, pattern.wrapping_add(11));
            c.lwr(A2, pattern.wrapping_add(8));
            let block = c.g[A1][0] as u32;
            c.sdl(V1, block.wrapping_add(7));
            c.sdr(V1, block);
            c.swl(A2, block.wrapping_add(11));
            c.swr(A2, block.wrapping_add(8));
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BLOCK_HEADER_BYTES));
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_FreeBlock(void* payload, Heap* heap, uint payload_bytes,
/// int format)` (0x3fcf00): free a block made by `SLoginStore_AllocBlock`
/// (its start is 12 bytes before the payload), passing the same rounded
/// size and alignment. Returns 4 when `payload` is null, else 0.
pub fn slogin_store_free_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.sw(sp(c), c.g[V0][0]);
        c.sw(sp(c).wrapping_add(4), c.g[V1][0]);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0] | 4;
        callee::SLOGIN_STORE_GET_ALIGNMENT.call(c, 0x003f_cf48)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cf54)?;
        let no_payload = c.g[S2][0] == 0;
        c.g[A3][0] = c.lw(sp(c));
        if no_payload {
            c.cycles += 1;
            c.g[V0][0] = 4;
        } else {
            c.cycles += 6;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(BLOCK_HEADER_BYTES));
            call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_cf6c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_GetBlockSize(uint payload_bytes, uint* size)`
/// (0x3fcf98): write the allocation size of a payload in store format 2 to
/// `*size`. Returns 4 when `size` is null, else 0.
pub fn slogin_store_get_block_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let no_out = c.g[S0][0] == 0;
        frame.save_ra(c, 0);
        if no_out {
            c.cycles += 1;
            c.g[V0][0] = 4;
        } else {
            c.cycles += 5;
            c.g[A1][0] = 2;
            callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cfb4)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

const DNAS_RELAY_BASE: u32 = 0x0045_0000;
const DNAS_INIT_SLOT: u32 = 0x152c;
const DNAS_UPDATE_SLOT: u32 = 0x1530;
const DNAS_EXIT_SLOT: u32 = 0x1534;

/// `int DnasRel_Load(void* image, void* arg)` (0x3fcfd8): open the DNAS relay
/// ELF image, relocate it through its `_reloctable` symbol, record its
/// `DirtyDnasRelInit`/`Updt`/`Exit` entry points in the relay slots and
/// flush the caches. Returns 0, or -1 (not a relay ELF), -2 (no
/// `_reloctable`), -3/-4/-5 (init/update/exit entry missing).
pub fn dnas_rel_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A2][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[A0][0] = c.g[SP][0];
        callee::DNAS_REL_ELF_OPEN.call(c, 0x003f_cff4)?;
        let opened = c.g[V0][0] != 0;
        c.g[A1][0] = 0x49_0000;
        'done: {
            if !opened {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7598));
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d010)?;
            c.g[A0][0] = c.g[V0][0];
            let no_reloc_table = c.g[A0][0] == 0;
            c.g[V0][0] = sx(-2i32 as u32);
            if no_reloc_table {
                break 'done;
            }
            c.cycles += 18;
            c.g[V0][0] = 0x40_0000;
            c.g[V1][0] = 0x40_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x27f0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26f8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.sw(sp(c).wrapping_add(0x34), c.g[A0][0]);
            c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
            c.g[A0][0] = c.g[SP][0];
            c.sw(sp(c).wrapping_add(0x30), c.g[V0][0]);
            c.g[S0][0] = DNAS_RELAY_BASE as u64;
            c.sw(sp(c).wrapping_add(0x38), c.g[V1][0]);
            callee::DNAS_REL_RELOCATE_ALL.call(c, 0x003f_d04c)?;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75a8));
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d05c)?;
            let found = c.g[V0][0] != 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(DNAS_INIT_SLOT), c.g[V0][0]);
            if !found {
                c.cycles += 2;
                c.g[V0][0] = sx(-3i32 as u32);
                break 'done;
            }
            c.cycles += 7;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75c0));
            c.g[S0][0] = DNAS_RELAY_BASE as u64;
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d080)?;
            let found = c.g[V0][0] != 0;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(DNAS_UPDATE_SLOT),
                c.g[V0][0],
            );
            if !found {
                c.cycles += 2;
                c.g[V0][0] = sx(-4i32 as u32);
                break 'done;
            }
            c.cycles += 7;
            c.g[A1][0] = 0x49_0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75d8));
            c.g[S0][0] = DNAS_RELAY_BASE as u64;
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d0a4)?;
            let missing = c.g[V0][0] == 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(DNAS_EXIT_SLOT), c.g[V0][0]);
            if missing {
                c.cycles += 1;
                c.g[V0][0] = sx(-5i32 as u32);
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = 0;
            callee::FLUSH_CACHE.call(c, 0x003f_d0b4)?;
            c.g[A0][0] = 2;
            callee::FLUSH_CACHE.call(c, 0x003f_d0bc)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `void DnasRel_CallInit(void* arg)` (0x3fd0d8): call the loaded relay's
/// init entry (relay slot +0x152c) with `arg` and 0x1020000.
pub fn dnas_rel_call_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = DNAS_RELAY_BASE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(DNAS_INIT_SLOT));
        frame.save_ra(c, 0);
        let init = c.g[V1][0] as u32;
        c.g[A1][0] = 0x0102_0000;
        call_ptr(c, init, 0x003f_d0f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, check_paths, elf_image, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const STORE_QUERY_TEST_STATE: u32 = DATA + 0xc00;
    const STORE_QUERY_TEST_RECORDS: u32 = DATA + 0xd00;
    const STORE_TEST_OUT: u32 = DATA + 0x80;

    /// Scrambled registers, the game's gp and `pc` at `entry`.
    fn slogin_store_query_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(entry as u64 ^ (seed as u64) << 24);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// An initialized store whose directory holds one record per `kinds`
    /// entry (size word 0x100 + 0x10 * index).
    fn slogin_store_query_directory(c: &mut Cpu, kinds: &[u32]) {
        c.bus
            .write32(GAME_GP + STORE_STATE_GP, STORE_QUERY_TEST_STATE);
        c.bus.write32(
            STORE_QUERY_TEST_STATE + STORE_RECORD_COUNT,
            kinds.len() as u32,
        );
        c.bus.write32(
            STORE_QUERY_TEST_STATE + STORE_RECORDS,
            STORE_QUERY_TEST_RECORDS,
        );
        for (i, kind) in kinds.iter().enumerate() {
            let rec = STORE_QUERY_TEST_RECORDS + i as u32 * STORE_RECORD_STRIDE;
            c.bus.write32(rec, *kind);
            c.bus.write32(rec + 4, 0x1111 * (i as u32 + 1));
            c.bus.write32(rec + RECORD_SIZE, 0x100 + 0x10 * i as u32);
            c.bus.write32(rec + 0xc, 0x2222 * (i as u32 + 1));
        }
    }

    const COUNT_RECORDS_KINDS: [u32; 6] = [5, 7, 5, 9, 5, 7];

    /// Arms: seed % 5 = 0 store not initialized (20 cycles), 1 null store
    /// (22), 2 null count (25), 3 empty directory (32), 4 records; kind
    /// seed / 5 % 3 and record count 1 + seed / 15 % 4 cross the loop's
    /// match and skip arms (one record: 39 skip, 42 match).
    #[test]
    fn slogin_store_count_records_matches() {
        check_paths(
            ee::s15::f_003fc850__generated,
            slogin_store_count_records,
            120,
            &[20, 22, 25, 32, 39, 42],
            &|c, seed| {
                let _r = slogin_store_query_fixture(c, seed, 0x003f_c850);
                let arm = seed % 5;
                let records = (if arm == 3 { 0 } else { 1 + seed / 15 % 4 }) as usize;
                slogin_store_query_directory(c, &COUNT_RECORDS_KINDS[..records]);
                if arm == 0 {
                    c.bus.write32(GAME_GP + STORE_STATE_GP, 0);
                }
                c.g[A0][0] = if arm == 1 { 0 } else { sx(DATA + 0x40) };
                c.g[A1][0] = [5, 7, 11][(seed / 5 % 3) as usize];
                c.g[A2][0] = if arm == 2 { 0 } else { sx(STORE_TEST_OUT) };
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
            },
        );
    }

    crate::difftest::events::driven!(
        driven_slogin_store_count_records,
        slogin_store_count_records
    );

    /// Event seeds (seed % 3) on the one loop head (0x3fc8b8), eight records,
    /// kind seed / 3 % 2.
    #[test]
    fn slogin_store_count_records_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc850__generated,
            driven_slogin_store_count_records,
            60,
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_c850);
                slogin_store_query_directory(c, &[5, 7, 5, 9, 5, 7, 7, 5]);
                c.g[A0][0] = sx(DATA + 0x40);
                c.g[A1][0] = [5, 7][(seed / 3 % 2) as usize];
                c.g[A2][0] = sx(STORE_TEST_OUT);
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
                schedule(c, &mut r, seed, 0x003f_c850)
            },
        );
    }

    /// Arms: seed % 5 = 0 store not initialized (26 cycles), 1 null store
    /// (28), 2 null size (31), 3 empty directory: not found (46), 4 three
    /// records; kind seed / 5 % 3 and occurrence seed / 15 % 3 cross found
    /// (first record: 54) and not found.
    #[test]
    fn slogin_store_get_record_size_matches() {
        check_paths(
            ee::s15::f_003fc8f8__generated,
            slogin_store_get_record_size,
            90,
            &[26, 28, 31, 46, 54],
            &|c, seed| {
                let _r = slogin_store_query_fixture(c, seed, 0x003f_c8f8);
                let arm = seed % 5;
                let kinds: &[u32] = if arm == 3 { &[] } else { &[5, 7, 5] };
                slogin_store_query_directory(c, kinds);
                if arm == 0 {
                    c.bus.write32(GAME_GP + STORE_STATE_GP, 0);
                }
                c.g[A0][0] = if arm == 1 { 0 } else { sx(DATA + 0x40) };
                c.g[A1][0] = [5, 7, 9][(seed / 5 % 3) as usize];
                c.g[A2][0] = (seed / 15 % 3) as u64;
                c.g[A3][0] = if arm == 2 { 0 } else { sx(STORE_TEST_OUT) };
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
            },
        );
    }

    const GET_INFO_OUT: u32 = DATA + 0x2000;

    /// The store state (0x70 random bytes) at `STORE_QUERY_TEST_STATE + state_skew`
    /// and a non-zero output buffer.
    fn slogin_store_get_info_fixture(c: &mut Cpu, r: &mut Rng, state_skew: u32) {
        c.bus.write32(
            GAME_GP + STORE_STATE_GP,
            STORE_QUERY_TEST_STATE + state_skew,
        );
        for i in 0..0x70 {
            c.bus
                .write8(STORE_QUERY_TEST_STATE + i, r.next_u32() as u8 | 1);
            c.bus.write8(GET_INFO_OUT + i, 0xc3);
        }
    }

    /// Arms: seed % 4 = 0 null output (4 cycles), 1 both aligned (43),
    /// 2 output misaligned by 1 + seed / 4 % 7, 3 state misaligned by
    /// 1 + seed / 4 % 7 (both 69).
    #[test]
    fn slogin_store_get_info_matches() {
        check_paths(
            ee::s15::f_003fc990__generated,
            slogin_store_get_info,
            84,
            &[4, 43, 69],
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_c990);
                let arm = seed % 4;
                let skew = 1 + seed / 4 % 7;
                slogin_store_get_info_fixture(c, &mut r, if arm == 3 { skew } else { 0 });
                c.g[A0][0] = match arm {
                    0 => 0,
                    2 => sx(GET_INFO_OUT + skew),
                    _ => sx(GET_INFO_OUT),
                };
            },
        );
    }

    crate::difftest::events::driven!(driven_slogin_store_get_info, slogin_store_get_info);

    /// Event seeds (seed % 3) on both loop heads: seed / 3 % 2 = 0 aligned
    /// (0x3fca04), 1 misaligned (0x3fc9b0).
    #[test]
    fn slogin_store_get_info_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc990__generated,
            driven_slogin_store_get_info,
            72,
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_c990);
                slogin_store_get_info_fixture(c, &mut r, 0);
                let skew = if seed / 3 % 2 == 0 {
                    0
                } else {
                    1 + seed / 6 % 7
                };
                c.g[A0][0] = sx(GET_INFO_OUT + skew);
                schedule(c, &mut r, seed, 0x003f_c990)
            },
        );
    }

    const SET_ICON_SYS_ICON: u32 = DATA + 0x1000;
    const SET_ICON_SYS_MC_STATE: u32 = DATA + 0x4000;
    const SET_ICON_SYS_MC_STATE_GP: u32 = 0x1aec;

    /// Real callees throughout. Arms: seed % 4 = 0 store not initialized
    /// (14 cycles), 1 null icon (card layer 0xc, mapped 0x70: 35), 2 card
    /// layer not initialized (2, mapped 0x66: 42), 3 icon accepted (0); the
    /// icon's alignment seed / 4 % 2 steers the card layer's copy.
    #[test]
    fn slogin_store_set_icon_sys_matches() {
        check_paths(
            ee::s15::f_003fcdc8__generated,
            slogin_store_set_icon_sys,
            48,
            &[14, 35, 42],
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_cdc8);
                let arm = seed % 4;
                let icon = SET_ICON_SYS_ICON + (seed / 4 % 2) * 4;
                c.bus.write32(
                    GAME_GP + STORE_STATE_GP,
                    if arm == 0 { 0 } else { STORE_QUERY_TEST_STATE },
                );
                c.bus.write32(
                    GAME_GP + SET_ICON_SYS_MC_STATE_GP,
                    if arm == 2 { 0 } else { SET_ICON_SYS_MC_STATE },
                );
                c.g[A0][0] = if arm == 1 { 0 } else { sx(icon) };
                for i in 0..0x3e0 {
                    c.bus.write8(SET_ICON_SYS_ICON + i, r.next_u32() as u8);
                }
                for i in 0..0x430 {
                    c.bus.write8(SET_ICON_SYS_MC_STATE + i, 0xa5);
                }
            },
        );
    }

    const STORE_BLOCK_TEST_CTRL: u32 = DATA + 0x5_0000;
    const STORE_BLOCK_TEST_MAGIC: u32 = 0x5342_4c4b;
    const STORE_BLOCK_TEST_BLOCK: u32 = DATA + 0x2000;
    const STORE_BLOCK_TEST_OUT: u32 = DATA + 0x80;
    static PREV_STORE_BLOCK_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static PREV_STORE_BLOCK_FREE: AtomicUsize = AtomicUsize::new(0);

    /// `TibExtMemAlloc` for the store block tests: logged, returning the
    /// block pointer at `STORE_BLOCK_TEST_CTRL + 4` (0: allocation fails).
    fn store_block_alloc_fake(c: &mut Cpu) {
        if c.bus.read32(STORE_BLOCK_TEST_CTRL) != STORE_BLOCK_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_STORE_BLOCK_ALLOC, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, 0x0026_6728);
        c.g[V0][0] = c.bus.read32(STORE_BLOCK_TEST_CTRL + 4) as u64;
    }

    /// `TibExtMemFree` for the store block tests: logged only.
    fn store_block_free_fake(c: &mut Cpu) {
        if c.bus.read32(STORE_BLOCK_TEST_CTRL) != STORE_BLOCK_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_STORE_BLOCK_FREE, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, 0x0026_6760);
    }

    /// Fake hooks, the game image (the alignment jump table at 0x497580 and
    /// the header pattern at 0x497550) and the fakes' gate.
    fn store_block_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            store_block_alloc_fake,
            &PREV_STORE_BLOCK_ALLOC,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            store_block_free_fake,
            &PREV_STORE_BLOCK_FREE,
        );
        let r = fakes::machine(c, seed, entry as u64, 1);
        c.pc = entry;
        elf_image(c);
        c.bus.write32(STORE_BLOCK_TEST_CTRL, STORE_BLOCK_TEST_MAGIC);
        r
    }

    /// Arms: seed % 4 = 0 null `out` (4), 1 allocation fails (6), 2 small
    /// payload, 3 payload 0xfffffff4 (the size check's overflow arm);
    /// format seed / 4 % 7 (table and out-of-range formats), the allocator
    /// unwinds when seed / 28 % 2 = 1, block misaligned by 3 when
    /// seed / 56 % 2 = 1.
    #[test]
    fn slogin_store_alloc_block_matches() {
        fakes::covered(
            ee::s15::f_003fce10__generated,
            slogin_store_alloc_block,
            112,
            &|c, seed| {
                let mut r = store_block_fixture(c, seed, 0x003f_ce10);
                let arm = seed % 4;
                let block = STORE_BLOCK_TEST_BLOCK + (seed / 56 % 2) * 3;
                c.g[A0][0] = if arm == 0 {
                    0
                } else {
                    sx(STORE_BLOCK_TEST_OUT)
                };
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = if arm == 3 {
                    0xffff_fff4
                } else {
                    (r.next_u32() % 0x80) as u64
                };
                c.g[A3][0] = (seed / 4 % 7) as u64;
                c.bus
                    .write32(STORE_BLOCK_TEST_CTRL + 4, if arm == 1 { 0 } else { block });
                c.bus.write32(STORE_BLOCK_TEST_OUT, 0x5a5a_5a5a);
                for i in 0..0xa0 {
                    c.bus.write8(STORE_BLOCK_TEST_BLOCK + i, 0xc3);
                }
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 28 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    /// Arms: seed % 2 = 0 null payload (4), 1 freed (0); format
    /// seed / 2 % 7, the free unwinds when seed / 14 % 2 = 1.
    #[test]
    fn slogin_store_free_block_matches() {
        fakes::covered(
            ee::s15::f_003fcf00__generated,
            slogin_store_free_block,
            56,
            &|c, seed| {
                let mut r = store_block_fixture(c, seed, 0x003f_cf00);
                c.g[A0][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(STORE_BLOCK_TEST_BLOCK + 12)
                };
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = (r.next_u32() % 0x80) as u64;
                c.g[A3][0] = (seed / 2 % 7) as u64;
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 14 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    /// Arms: seed % 3 = 0 null `size` (10 cycles), 1 payload 0xfffffff4
    /// (the size check's overflow arm: 51), 2 payload seed / 3 (60).
    #[test]
    fn slogin_store_get_block_size_matches() {
        check_paths(
            ee::s15::f_003fcf98__generated,
            slogin_store_get_block_size,
            48,
            &[10, 51, 60],
            &|c, seed| {
                let _r = slogin_store_query_fixture(c, seed, 0x003f_cf98);
                elf_image(c);
                let arm = seed % 3;
                c.g[A0][0] = if arm == 1 {
                    0xffff_fff4
                } else {
                    (seed / 3) as u64
                };
                c.g[A1][0] = if arm == 0 { 0 } else { sx(STORE_TEST_OUT) };
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
            },
        );
    }

    const DNAS_LOAD_IMAGE: u32 = DATA + 0x1000;
    /// The symbols `DnasRel_Load` looks up, in lookup order.
    const DNAS_LOAD_SYMBOLS: [&[u8]; 4] = [
        b"_reloctable",
        b"DirtyDnasRelInit",
        b"DirtyDnasRelUpdt",
        b"DirtyDnasRelExit",
    ];

    fn dnas_load_bytes(c: &mut Cpu, at: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + bytes.len() as u32, 0);
    }

    /// A relocatable MIPS ELF with three sections (section-name strings,
    /// `.symtab`, `.strtab`) and four symbols in section 1; the first
    /// `present` carry the looked-up names, the rest a decoy name. No
    /// `.text`/`.rel.*` sections, so relocation does nothing. `header`:
    /// 0 valid, 1 bad magic, 2 wrong type/machine word.
    fn dnas_load_image(c: &mut Cpu, present: usize, header: u32) {
        let img = DNAS_LOAD_IMAGE;
        c.bus.write32(
            img,
            if header == 1 {
                0x464c_4500
            } else {
                0x464c_457f
            },
        );
        c.bus.write32(
            img + 0x10,
            if header == 2 {
                0x0008_0002
            } else {
                0x0008_0001
            },
        );
        c.bus.write32(img + 0x20, 0x100);
        c.bus.write16(img + 0x30, 3);
        c.bus.write16(img + 0x32, 0);
        let sections = img + 0x100;
        for (i, (name, offset, size)) in [(0, 0x200, 0x20), (1, 0x300, 0x40), (9, 0x400, 0x80)]
            .iter()
            .enumerate()
        {
            let sh = sections + 0x28 * i as u32;
            c.bus.write32(sh, *name);
            c.bus.write32(sh + 0x10, *offset);
            c.bus.write32(sh + 0x14, *size);
        }
        dnas_load_bytes(c, img + 0x200, b"");
        dnas_load_bytes(c, img + 0x201, b".symtab");
        dnas_load_bytes(c, img + 0x209, b".strtab");
        dnas_load_bytes(c, img + 0x400, b"");
        dnas_load_bytes(c, img + 0x401, b"Xnone");
        let mut name_at = 7u32;
        for (i, name) in DNAS_LOAD_SYMBOLS.iter().enumerate() {
            dnas_load_bytes(c, img + 0x400 + name_at, name);
            let sym = img + 0x300 + 0x10 * i as u32;
            c.bus.write32(sym, if i < present { name_at } else { 1 });
            c.bus.write32(sym + 4, 0x10 * (i as u32 + 1));
            c.bus.write32(sym + 8, 0);
            c.bus.write16(sym + 0xc, 0);
            c.bus.write16(sym + 0xe, 1);
            name_at += name.len() as u32 + 1;
        }
    }

    /// Real ELF callees; the two `FlushCache` syscalls are faked. Arms:
    /// seed % 6 = 0 not a relay ELF (-1; bad magic or bad type by
    /// seed / 6 % 2), 1 no `_reloctable` (-2), 2 no init (-3), 3 no update
    /// (-4), 4 no exit (-5), 5 loaded (0); in arm 5, seed / 6 % 3 = 1/2
    /// unwinds at the first/second `FlushCache`.
    #[test]
    fn dnas_rel_load_matches() {
        fakes::covered(
            ee::s15::f_003fcfd8__generated,
            dnas_rel_load,
            72,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_cfd8, 2);
                c.pc = 0x003f_cfd8;
                let arm = seed % 6;
                let present = (arm as usize).saturating_sub(1);
                dnas_load_image(c, present, if arm == 0 { 1 + seed / 6 % 2 } else { 0 });
                c.g[A0][0] = sx(DNAS_LOAD_IMAGE);
                c.g[A1][0] = sx(DATA + 0x3000);
                for slot in [DNAS_INIT_SLOT, DNAS_UPDATE_SLOT, DNAS_EXIT_SLOT] {
                    c.bus.write32(DNAS_RELAY_BASE + slot, 0x1357_9bdf);
                }
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    match seed / 6 % 3 {
                        0 => u32::MAX,
                        n => n - 1,
                    },
                );
            },
        );
    }

    /// The relay's init entry is a fake (one of three addresses by
    /// seed % 3) that unwinds when seed / 3 % 3 = 0.
    #[test]
    fn dnas_rel_call_init_matches() {
        fakes::covered(
            ee::s15::f_003fd0d8__generated,
            dnas_rel_call_init,
            72,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_d0d8, 1);
                c.pc = 0x003f_d0d8;
                c.bus.write32(
                    DNAS_RELAY_BASE + DNAS_INIT_SLOT,
                    fakes::FAKE + (seed % 3) * 0x10,
                );
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 3 % 3 == 0 { 0 } else { u32::MAX },
                );
            },
        );
    }
}
