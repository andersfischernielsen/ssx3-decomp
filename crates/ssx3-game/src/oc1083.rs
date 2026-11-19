use crate::abi::{addu, r32, ret, set_word};
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

mod callee {
    use super::{ee, Callee};

    pub(super) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(super) const LOBBY_MEM_ALLOC: Callee = Callee {
        addr: 0x0025_91e8,
        f: ee::s06::Lobby_MemAlloc,
    };
    pub(super) const LOBBY_MEM_FREE: Callee = Callee {
        addr: 0x0025_9210,
        f: ee::s06::Lobby_MemFree,
    };
    pub(super) const LOBBY_HASH_STRING: Callee = Callee {
        addr: 0x003e_ea48,
        f: ee::s14::Lobby_HashString,
    };
    pub(super) const LOBBY_HASH_GROW_POOL: Callee = Callee {
        addr: 0x003e_e900,
        f: ee::s14::Lobby_HashGrowPool,
    };
    pub(super) const LOBBY_MERGE_SORT: Callee = Callee {
        addr: 0x003e_f210,
        f: ee::s14::Lobby_MergeSort,
    };
    pub(super) const SCE_SCF_GET_GMT_FROM_RTC: Callee = Callee {
        addr: 0x0040_bfd8,
        f: ee::s15::sceScfGetGMTfromRTC,
    };
    pub(super) const SCE_SCF_GET_LOCAL_TIME_FROM_RTC: Callee = Callee {
        addr: 0x0040_bfe0,
        f: ee::s15::sceScfGetLocalTimefromRTC,
    };
}

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_f090, ds_timezone, "ds_timezone"),
    (0x003e_f088, ds_timeinsecs, "ds_timeinsecs"),
    (0x003e_ef40, lobby_hash_del_int, "Lobby_HashDelInt"),
    (0x003e_eee8, lobby_hash_find_int, "Lobby_HashFindInt"),
    (0x003e_eda0, lobby_hash_add_int, "Lobby_HashAddInt"),
    (0x003e_ed08, lobby_hash_del_str, "Lobby_HashDelStr"),
    (0x003e_ec88, lobby_hash_find_str, "Lobby_HashFindStr"),
    (0x003e_ebf8, lobby_hash_replace_str, "Lobby_HashReplaceStr"),
    (0x003e_eb30, lobby_hash_add_str, "Lobby_HashAddStr"),
    (0x003e_ea48, lobby_hash_string, "Lobby_HashString"),
    (0x003e_e9a0, lobby_hash_pop_any, "Lobby_HashPopAny"),
    (0x003e_e900, lobby_hash_grow_pool, "Lobby_HashGrowPool"),
    (0x003e_e820, lobby_hash_destroy, "Lobby_HashDestroy"),
    (0x003e_e778, lobby_hash_create, "Lobby_HashCreate"),
    (0x003e_e770, lobby_list_get_hash, "Lobby_ListGetHash"),
    (0x003e_e768, lobby_list_set_hash, "Lobby_ListSetHash"),
    (0x003e_e620, lobby_list_sort, "Lobby_ListSort"),
    (0x003e_e600, lobby_list_mark_dirty, "Lobby_ListMarkDirty"),
    (0x003e_e5a0, lobby_list_set_sort, "Lobby_ListSetSort"),
    (0x003e_e560, lobby_list_get_at, "Lobby_ListGetAt"),
];

const DS_TIMEZONE_GP: u32 = 0x19a4;
const DS_TIMEZONE_UNSET: u64 = u64::MAX;
const DS_RTC_SECOND: u32 = 1;
const DS_RTC_MINUTE: u32 = 2;
const DS_RTC_HOUR: u32 = 3;
const DS_RTC_DAY: u32 = 5;
const DS_RTC_MONTH: u32 = 6;
const DS_RTC_YEAR: u32 = 7;
const DS_RTC_DAY_OF_TEST_DATE: u64 = 2;
const DS_RTC_MONTH_YEAR_OF_TEST_DATE: u64 = 1;
const DECIMAL_BASE: u64 = 10;
const BCD_NIBBLE_BITS: u32 = 4;
const BCD_NIBBLE_MASK: u64 = 0xf;
const DS_SECONDS_PER_MINUTE: u64 = 60;
const DS_SECONDS_PER_HOUR: u64 = 60 * DS_SECONDS_PER_MINUTE;
const DS_SECONDS_PER_DAY: u64 = 24 * DS_SECONDS_PER_HOUR;
const DS_TIMEZONE_FRAME_BYTES: u32 = 0x90;
const DS_TIMEZONE_SAVED_RA: u32 = 0x10;
const DS_TIMEZONE_SAVED_REGISTERS: [(usize, u32); 7] = [
    (S0, 0x80),
    (S1, 0x70),
    (S2, 0x60),
    (S3, 0x50),
    (S4, 0x40),
    (S5, 0x30),
    (S6, 0x20),
];

/// Returns the cached DirtySDK time-zone offset in seconds, deriving it from
/// the difference between the local and GMT conversion of a fixed RTC date.
pub fn ds_timezone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, DS_TIMEZONE_FRAME_BYTES);
        c.g[V1][0] = Addr(r32(c, GP)).off(DS_TIMEZONE_GP).lw(c);
        for (reg, offset) in DS_TIMEZONE_SAVED_REGISTERS {
            frame.save(c, reg, offset);
        }
        c.g[V0][0] = DS_TIMEZONE_UNSET;
        let cold = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, DS_TIMEZONE_SAVED_RA);
        if cold {
            c.cycles += 73;
            let rtc = Addr(r32(c, SP));
            rtc.sd(c, 0);
            c.g[S2][0] = DS_RTC_MONTH_YEAR_OF_TEST_DATE;
            c.g[S4][0] = DS_RTC_DAY_OF_TEST_DATE;
            c.g[A0][0] = c.g[SP][0];
            rtc.off(DS_RTC_DAY).sb(c, c.g[S4][0]);
            c.g[S1][0] = DECIMAL_BASE;
            rtc.off(DS_RTC_MONTH).sb(c, c.g[S2][0]);
            c.g[S3][0] = DS_SECONDS_PER_DAY;
            rtc.off(DS_RTC_YEAR).sb(c, c.g[S2][0]);
            callee::SCE_SCF_GET_GMT_FROM_RTC.call(c, 0x003e_f0ec)?;

            let rtc = Addr(r32(c, SP));
            c.g[S5][0] = DS_SECONDS_PER_HOUR;
            c.g[A1][0] = rtc.off(DS_RTC_DAY).lbu(c);
            c.g[S6][0] = DS_SECONDS_PER_MINUTE;
            c.g[A2][0] = rtc.off(DS_RTC_HOUR).lbu(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[A3][0] = rtc.off(DS_RTC_MINUTE).lbu(c);
            set_word(c, S0, r32(c, A1) >> BCD_NIBBLE_BITS);
            set_word(c, V1, r32(c, A2) >> BCD_NIBBLE_BITS);
            c.mult(0, S0, S0, S1);
            set_word(c, V0, r32(c, A3) >> BCD_NIBBLE_BITS);
            c.mult(0, V1, V1, S1);
            c.mult(0, V0, V0, S1);
            c.g[T0][0] = rtc.off(DS_RTC_SECOND).lbu(c);
            c.g[A1][0] &= BCD_NIBBLE_MASK;
            c.g[A2][0] &= BCD_NIBBLE_MASK;
            c.g[S0][0] = addu(c.g[S0][0], c.g[A1][0]);
            c.g[A3][0] &= BCD_NIBBLE_MASK;
            c.g[V1][0] = addu(c.g[V1][0], c.g[A2][0]);
            set_word(c, A1, r32(c, T0) >> BCD_NIBBLE_BITS);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A3][0]);
            c.mult(0, V1, V1, S5);
            c.mult(0, S0, S0, S3);
            c.g[T0][0] &= BCD_NIBBLE_MASK;
            c.mult(0, V0, V0, S6);
            rtc.sd(c, 0);
            c.mult(0, A1, A1, S1);
            rtc.off(DS_RTC_DAY).sb(c, c.g[S4][0]);
            rtc.off(DS_RTC_YEAR).sb(c, c.g[S2][0]);
            c.g[S0][0] = addu(c.g[S0][0], c.g[V1][0]);
            rtc.off(DS_RTC_MONTH).sb(c, c.g[S2][0]);
            c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
            c.g[A1][0] = addu(c.g[A1][0], c.g[T0][0]);
            c.g[S0][0] = addu(c.g[S0][0], c.g[A1][0]);
            callee::SCE_SCF_GET_LOCAL_TIME_FROM_RTC.call(c, 0x003e_f174)?;

            let rtc = Addr(r32(c, SP));
            c.g[V1][0] = rtc.off(DS_RTC_DAY).lbu(c);
            c.g[A2][0] = rtc.off(DS_RTC_HOUR).lbu(c);
            c.g[T0][0] = rtc.off(DS_RTC_MINUTE).lbu(c);
            set_word(c, V0, r32(c, V1) >> BCD_NIBBLE_BITS);
            set_word(c, A1, r32(c, A2) >> BCD_NIBBLE_BITS);
            c.mult(0, V0, V0, S1);
            set_word(c, A0, r32(c, T0) >> BCD_NIBBLE_BITS);
            c.mult(0, A1, A1, S1);
            c.mult(0, A0, A0, S1);
            c.g[V1][0] &= BCD_NIBBLE_MASK;
            c.g[A3][0] = rtc.off(DS_RTC_SECOND).lbu(c);
            c.g[A2][0] &= BCD_NIBBLE_MASK;
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            c.g[T0][0] &= BCD_NIBBLE_MASK;
            c.g[A1][0] = addu(c.g[A1][0], c.g[A2][0]);
            set_word(c, V1, r32(c, A3) >> BCD_NIBBLE_BITS);
            c.mult(0, V0, V0, S3);
            c.mult(0, A1, A1, S5);
            c.g[A0][0] = addu(c.g[A0][0], c.g[T0][0]);
            c.g[A3][0] &= BCD_NIBBLE_MASK;
            c.mult(0, A0, A0, S6);
            c.mult(0, V1, V1, S1);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
            c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
            c.g[V1][0] = addu(c.g[V1][0], c.g[A3][0]);
            c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
            set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, S0)));
            Addr(r32(c, GP)).off(DS_TIMEZONE_GP).sw(c, c.g[V0][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = Addr(r32(c, GP)).off(DS_TIMEZONE_GP).lw(c);
        for (reg, offset) in DS_TIMEZONE_SAVED_REGISTERS {
            frame.restore(c, reg, offset);
        }
        frame.restore_ra(c, DS_TIMEZONE_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Returns the DirtySDK clock stub's zero wall-clock time.
pub fn ds_timeinsecs(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

const HASH_ENTRY_COUNT: u32 = 0;
const HASH_BUCKET_COUNT: u32 = 0x10;
const HASH_BUCKETS: u32 = 0x14;
const HASH_FREE_HEAD: u32 = 0x1c;
const HASH_ENTRY_VALUE: u32 = 4;
const HASH_ENTRY_KEY: u32 = 8;
const HASH_DEL_INT_LOOP: u32 = 0x003e_ef70;
const HASH_FIND_INT_LOOP: u32 = 0x003e_ef1c;

fn hash_entry_recycle(c: &mut Cpu, table: usize, entry: usize, link: usize) {
    c.g[V0][0] = Addr(r32(c, entry)).lw(c);
    Addr(r32(c, link)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, table)).off(HASH_FREE_HEAD).lw(c);
    Addr(r32(c, entry)).sw(c, c.g[V1][0]);
    Addr(r32(c, table)).off(HASH_FREE_HEAD).sw(c, c.g[entry][0]);
    c.g[V0][0] = Addr(r32(c, table)).off(HASH_ENTRY_COUNT).lw(c);
    set_word(c, V0, r32(c, V0).wrapping_sub(1));
    Addr(r32(c, table)).off(HASH_ENTRY_COUNT).sw(c, c.g[V0][0]);
    c.g[V0][0] = Addr(r32(c, entry)).off(HASH_ENTRY_VALUE).lw(c);
}

/// Removes an integer key, returns its value, and recycles its entry.
pub fn lobby_hash_del_int(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = c.g[A1][0];
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 1;
        set_word(c, V0, r32(c, V0).wrapping_neg());
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A2)).off(HASH_BUCKET_COUNT).lw(c);
    c.div(0, V0, V1);
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_BUCKETS).lw(c);
    c.g[A0][0] = sx((c.hi[0] as u32) << 2);
    c.g[V1][0] = addu(c.g[V0][0], c.g[A0][0]);

    loop {
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).off(HASH_ENTRY_KEY).lw(c);
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 10;
            hash_entry_recycle(c, A2, A0, V1);
            break;
        }
        c.g[V1][0] = c.g[A0][0];
        if event_due(c) {
            deliver_event_here(c, HASH_DEL_INT_LOOP);
        }
    }
    ret(c);
}

/// Returns the value held for an integer key, or zero if it is missing.
pub fn lobby_hash_find_int(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.g[A1][0];
    c.g[V1][0] = Addr(r32(c, A0)).off(HASH_BUCKET_COUNT).lw(c);
    c.g[V0][0] = c.g[A2][0];
    if (c.g[A2][0] as i64) < 0 {
        c.cycles += 1;
        set_word(c, V0, r32(c, V0).wrapping_neg());
    }
    c.cycles += 3;
    c.div(0, V0, V1);
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, A0)).off(HASH_BUCKETS).lw(c);
    c.g[A1][0] = sx((c.hi[0] as u32) << 2);
    c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
    c.g[A1][0] = Addr(r32(c, A1)).lw(c);

    loop {
        c.cycles += 2;
        if c.g[A1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A1)).off(HASH_ENTRY_KEY).lw(c);
        if c.g[V0][0] == c.g[A2][0] {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(HASH_ENTRY_VALUE).lw(c);
            break;
        }
        c.g[A1][0] = Addr(r32(c, A1)).lw(c);
        if event_due(c) {
            deliver_event_here(c, HASH_FIND_INT_LOOP);
        }
    }
    ret(c);
}

const HASH_COMPARE: u32 = 0x20;
const HASH_DEL_STR_LOOP: u32 = 0x003e_ed38;
const HASH_FIND_STR_LOOP: u32 = 0x003e_ecc8;
const HASH_REPLACE_STR_LOOP: u32 = 0x003e_ec40;
const HASH_STRING_LOOP: u32 = 0x003e_ea60;
const HASH_STRING_MULTIPLIER: u64 = 131;
const HASH_STRING_CASE_MASK: u64 = 0x5f;
const HASH_STRING_FIRST_SIGNIFICANT: i64 = b'!' as i64;
const HASH_STRING_NONNEGATIVE: u64 = i32::MAX as u64;
const HASH_GROW_BASE: u32 = 10;
const HASH_SAVED_RA: u32 = 0;
const HASH_FRAME3_BYTES: u32 = 0x40;
const HASH_FRAME4_BYTES: u32 = 0x50;
const HASH_FRAME3_S0: u32 = 0x30;
const HASH_FRAME3_S1: u32 = 0x20;
const HASH_FRAME3_S2: u32 = 0x10;
const HASH_FRAME4_S0: u32 = 0x40;
const HASH_FRAME4_S1: u32 = 0x30;
const HASH_FRAME4_S2: u32 = 0x20;
const HASH_FRAME4_S3: u32 = 0x10;

fn hash_close_frame4(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, HASH_FRAME4_S0);
    frame.restore(c, S1, HASH_FRAME4_S1);
    frame.restore(c, S2, HASH_FRAME4_S2);
    frame.restore(c, S3, HASH_FRAME4_S3);
    frame.restore_ra(c, HASH_SAVED_RA);
    frame.close(c);
}

fn hash_close_frame3(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, HASH_FRAME3_S0);
    frame.restore(c, S1, HASH_FRAME3_S1);
    frame.restore(c, S2, HASH_FRAME3_S2);
    frame.restore_ra(c, HASH_SAVED_RA);
    frame.close(c);
}

fn hash_grow_for_insert(c: &mut Cpu, return_pc: u32) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_COUNT).lw(c);
    c.g[A0][0] = c.g[S0][0];
    let count = r32(c, V0);
    let scaled = count.wrapping_mul(3).wrapping_add(HASH_GROW_BASE) as i32;
    let requested = scaled / 2;
    set_word(c, V0, (scaled as u32) >> 31);
    set_word(c, A1, requested as u32);
    callee::LOBBY_HASH_GROW_POOL.call(c, return_pc)
}

fn hash_insert_from_free_head(c: &mut Cpu) {
    c.cycles += 16;
    c.g[A2][0] = sx(r32(c, S1) << 2);
    c.g[V0][0] = 1;
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, S0)).off(HASH_FREE_HEAD).sw(c, c.g[V1][0]);
    Addr(r32(c, A1)).off(HASH_ENTRY_KEY).sw(c, c.g[S2][0]);
    Addr(r32(c, A1)).off(HASH_ENTRY_VALUE).sw(c, c.g[S3][0]);
    c.g[V1][0] = Addr(r32(c, S0)).off(HASH_BUCKETS).lw(c);
    c.g[V1][0] = addu(c.g[A2][0], c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[A0][0]);
    c.g[V1][0] = Addr(r32(c, S0)).off(HASH_BUCKETS).lw(c);
    c.g[A2][0] = addu(c.g[A2][0], c.g[V1][0]);
    Addr(r32(c, A2)).sw(c, c.g[A1][0]);
    c.g[V1][0] = Addr(r32(c, S0)).off(HASH_ENTRY_COUNT).lw(c);
    set_word(c, V1, r32(c, V1).wrapping_add(1));
    Addr(r32(c, S0)).off(HASH_ENTRY_COUNT).sw(c, c.g[V1][0]);
}

/// Inserts an integer key and value at the head of its hash bucket.
pub fn lobby_hash_add_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save(c, S1, HASH_FRAME4_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, HASH_FRAME4_S2);
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[A0][0] = c.g[S2][0];
        if (c.g[S2][0] as i64) < 0 {
            c.cycles += 1;
            set_word(c, A0, r32(c, A0).wrapping_neg());
        }
        c.cycles += 5;
        c.g[S3][0] = c.g[A2][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(HASH_BUCKET_COUNT).lw(c);
        c.div(0, A0, V0);
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        c.g[S1][0] = c.hi[0];
        if c.g[V1][0] == 0 {
            hash_grow_for_insert(c, 0x003e_ee14)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 9;
                c.g[V0][0] = u64::MAX;
                hash_close_frame4(c, frame);
                return Ok(());
            }
        }
        c.g[A1][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        hash_insert_from_free_head(c);
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Removes the matching string entry, returning its value or zero.
pub fn lobby_hash_del_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S2, HASH_FRAME4_S2);
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S3][0] = c.g[A1][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_ed2c)?;
        c.g[V1][0] = Addr(r32(c, S2)).off(HASH_BUCKETS).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[S1][0] = addu(c.g[V1][0], c.g[V0][0]);

        loop {
            c.cycles += 3;
            c.g[S0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A1][0] = c.g[S3][0];
            if c.g[S0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S2)).off(HASH_COMPARE).lw(c);
            let compare = r32(c, V0);
            c.g[A0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_KEY).lw(c);
            call_ptr(c, compare, 0x003e_ed50)?;
            if c.g[V0][0] == 0 {
                c.cycles += 10;
                hash_entry_recycle(c, S2, S0, S1);
                break;
            }
            c.g[S1][0] = Addr(r32(c, S1)).lw(c);
            if event_due(c) {
                deliver_event_here(c, HASH_DEL_STR_LOOP);
            }
        }

        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

fn hash_find_string_entry(c: &mut Cpu, compare_return: u32, loop_head: u32, replace: bool) -> Flow {
    c.g[V1][0] = Addr(r32(c, S1)).off(HASH_BUCKETS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[S0][0] = Addr(r32(c, V0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[S0][0] == 0 {
        return Ok(());
    }

    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, S1)).off(HASH_COMPARE).lw(c);
    loop {
        c.cycles += 5;
        c.g[A1][0] = c.g[S2][0];
        let compare = r32(c, V0);
        c.g[A0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_KEY).lw(c);
        call_ptr(c, compare, compare_return)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).off(HASH_ENTRY_VALUE).lw(c);
            if replace {
                c.cycles += 1;
                Addr(r32(c, S0)).off(HASH_ENTRY_VALUE).sw(c, c.g[S3][0]);
            }
            return Ok(());
        }
        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0;
            return Ok(());
        }
        c.g[V0][0] = Addr(r32(c, S1)).off(HASH_COMPARE).lw(c);
        if event_due(c) {
            deliver_event_here(c, loop_head);
        }
    }
}

/// Returns the value held for a string key, or zero if it is missing.
pub fn lobby_hash_find_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, HASH_FRAME3_BYTES);
        frame.save(c, S1, HASH_FRAME3_S1);
        frame.save(c, S2, HASH_FRAME3_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, HASH_FRAME3_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S2][0] = c.g[A1][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_eca8)?;
        hash_find_string_entry(c, 0x003e_ecd4, HASH_FIND_STR_LOOP, false)?;
        c.cycles += 6;
        hash_close_frame3(c, frame);
        Ok(())
    });
}

/// Changes the value held for an existing string key and returns its old value.
pub fn lobby_hash_replace_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save(c, S2, HASH_FRAME4_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S3][0] = c.g[A2][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_ec20)?;
        hash_find_string_entry(c, 0x003e_ec4c, HASH_REPLACE_STR_LOOP, true)?;
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Inserts a string key and value at the head of its hash bucket.
pub fn lobby_hash_add_str(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S0, HASH_FRAME4_S0);
        frame.save(c, S2, HASH_FRAME4_S2);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, HASH_FRAME4_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S3][0] = c.g[A2][0];
        callee::LOBBY_HASH_STRING.call(c, 0x003e_eb58)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        if c.g[V0][0] == 0 {
            hash_grow_for_insert(c, 0x003e_eb8c)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 9;
                c.g[V0][0] = u64::MAX;
                hash_close_frame4(c, frame);
                return Ok(());
            }
        }
        c.g[A1][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
        hash_insert_from_free_head(c);
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Hashes significant single-byte characters into a lobby hash bucket.
pub fn lobby_hash_string(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A1)).lb(c);
    c.g[A3][0] = 0;
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = Addr(r32(c, A1)).lbu(c);
    if !empty {
        c.cycles += 2;
        c.g[T0][0] = HASH_STRING_MULTIPLIER;
        loop {
            c.cycles += 5;
            c.g[V0][0] = u64::from(i64::from(r32(c, A2) as i8) < HASH_STRING_FIRST_SIGNIFICANT);
            set_word(c, A1, r32(c, A1).wrapping_add(1));
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.mult(0, V1, A3, T0);
                c.g[V0][0] = c.g[A2][0] & HASH_STRING_CASE_MASK;
                c.g[A3][0] = addu(c.g[V1][0], c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A1)).lb(c);
            let more = c.g[V0][0] != 0;
            c.g[A2][0] = Addr(r32(c, A1)).lbu(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HASH_STRING_LOOP);
            }
        }
    }
    c.cycles += 6;
    c.g[V1][0] = Addr(r32(c, A0)).off(HASH_BUCKET_COUNT).lw(c);
    c.g[V0][0] = HASH_STRING_NONNEGATIVE;
    c.g[A3][0] &= c.g[V0][0];
    c.cycles += 4;
    c.div(0, A3, V1);
    c.g[V0][0] = c.hi[0];
    ret(c);
}

const HASH_POP_BUCKET: u32 = 4;
const HASH_POOL_BLOCKS: u32 = 0x18;
const HASH_ENTRY_BYTES: u64 = 12;
const HASH_HEADER_BYTES: u64 = 0x24;
const HASH_MIN_BUCKETS: u64 = 16;
const HASH_DEFAULT_COMPARATOR: u32 = 0x003e_eab8;
const HASH_POP_LOOP: u32 = 0x003e_e9e0;
const HASH_GROW_POOL_LOOP: u32 = 0x003e_e968;
const HASH_DESTROY_LOOP: u32 = 0x003e_e840;
const HASH_FRAME1_BYTES: u32 = 0x20;
const HASH_FRAME1_S0: u32 = 0x10;

/// Removes and returns an entry from the first occupied bucket at or after the cursor.
pub fn lobby_hash_pop_any(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = c.g[A0][0];
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_ENTRY_COUNT).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_POP_BUCKET).lw(c);
    c.cycles += 6;
    c.g[A0][0] = Addr(r32(c, A2)).off(HASH_BUCKETS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[A0][0]);
    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
    let empty = c.g[V1][0] == 0;
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_POP_BUCKET).lw(c);
    if empty {
        c.cycles += 4;
        c.g[A1][0] = Addr(r32(c, A2)).off(HASH_BUCKET_COUNT).lw(c);
        c.g[A3][0] = c.g[A0][0];
        c.g[T0][0] = 0;
        loop {
            c.cycles += 11;
            set_word(c, V0, r32(c, V0).wrapping_add(1));
            c.div(0, V0, A1);
            c.g[A0][0] = c.hi[0];
            c.g[V1][0] = sx(r32(c, A0) << 2);
            Addr(r32(c, A2)).off(HASH_POP_BUCKET).sw(c, c.g[A0][0]);
            c.g[V1][0] = addu(c.g[V1][0], c.g[A3][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            let occupied = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(r32(c, A2)).off(HASH_POP_BUCKET).lw(c);
            if occupied {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HASH_POP_LOOP);
            }
        }
    }
    c.cycles += 14;
    c.g[V1][0] = Addr(r32(c, A2)).off(HASH_BUCKETS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[A1][0] = Addr(r32(c, V0)).lw(c);
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, V0)).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, A2)).off(HASH_FREE_HEAD).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[A0][0]);
    Addr(r32(c, A2)).off(HASH_FREE_HEAD).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(r32(c, A2)).off(HASH_ENTRY_COUNT).lw(c);
    set_word(c, V0, r32(c, V0).wrapping_sub(1));
    Addr(r32(c, A2)).off(HASH_ENTRY_COUNT).sw(c, c.g[V0][0]);
    c.g[V0][0] = Addr(r32(c, A1)).off(HASH_ENTRY_VALUE).lw(c);
    ret(c);
}

/// Allocates another block of hash entries and links its entries into the free pool.
pub fn lobby_hash_grow_pool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, HASH_FRAME3_BYTES);
        let positive_count = (c.g[A1][0] as i64) > 0;
        c.g[V0][0] = u64::from(positive_count);
        frame.save(c, S1, HASH_FRAME3_S1);
        c.g[V1][0] = HASH_ENTRY_BYTES;
        c.g[S1][0] = 1;
        frame.save(c, S2, HASH_FRAME3_S2);
        if positive_count {
            c.g[S1][0] = c.g[A1][0];
        }
        frame.save(c, S0, HASH_FRAME3_S0);
        set_word(c, S1, r32(c, S1).wrapping_add(1));
        c.g[S0][0] = c.g[A0][0];
        c.mult(0, S2, S1, V1);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[A0][0] = c.g[S2][0];
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e938)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        if c.g[A1][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
            c.g[A0][0] = addu(c.g[A1][0], c.g[S2][0]);
            set_word(c, A2, r32(c, S1).wrapping_sub(1));
            set_word(c, V1, r32(c, A1).wrapping_add(HASH_ENTRY_BYTES as u32));
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            let have_entries = c.g[V1][0] != c.g[A0][0];
            Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).sw(c, c.g[A1][0]);
            if have_entries {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
                loop {
                    c.cycles += 7;
                    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
                    Addr(r32(c, S0)).off(HASH_FREE_HEAD).sw(c, c.g[V1][0]);
                    set_word(c, V1, r32(c, V1).wrapping_add(HASH_ENTRY_BYTES as u32));
                    if c.g[V1][0] == c.g[A0][0] {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).off(HASH_FREE_HEAD).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, HASH_GROW_POOL_LOOP);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A2][0];
        }
        c.cycles += 6;
        hash_close_frame3(c, frame);
        Ok(())
    });
}

/// Frees the pool blocks, bucket array, and lobby hash table.
pub fn lobby_hash_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, HASH_FRAME1_BYTES);
        frame.save(c, S0, HASH_FRAME1_S0);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
            loop {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).sw(c, c.g[V0][0]);
                callee::LOBBY_MEM_FREE.call(c, 0x003e_e84c)?;
                c.g[V0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S0)).off(HASH_POOL_BLOCKS).lw(c);
                if event_due(c) {
                    deliver_event_here(c, HASH_DESTROY_LOOP);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = Addr(r32(c, S0)).off(HASH_BUCKETS).lw(c);
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e860)?;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e868)?;
        frame.restore(c, S0, HASH_FRAME1_S0);
        frame.restore_ra(c, HASH_SAVED_RA);
        frame.close(c);
        Ok(())
    });
}

/// Creates a lobby hash table with a bucket array and an initial entry pool.
pub fn lobby_hash_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, HASH_FRAME3_BYTES);
        let positive_entries = (c.g[A0][0] as i64) > 0;
        c.g[V1][0] = u64::from(positive_entries);
        frame.save(c, S2, HASH_FRAME3_S2);
        c.g[V0][0] = HASH_MIN_BUCKETS - 1;
        frame.save(c, S0, HASH_FRAME3_S0);
        c.g[S2][0] = 1;
        if positive_entries {
            c.g[S2][0] = c.g[A0][0];
        }
        let enough_buckets = (c.g[A1][0] as i64) > (c.g[V0][0] as i64);
        c.g[V0][0] = u64::from(enough_buckets);
        c.g[S0][0] = HASH_MIN_BUCKETS;
        frame.save(c, S1, HASH_FRAME3_S1);
        frame.save_ra(c, HASH_SAVED_RA);
        if enough_buckets {
            c.g[S0][0] = c.g[A1][0];
        }
        c.g[A0][0] = HASH_HEADER_BYTES;
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e7b0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = HASH_HEADER_BYTES;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        callee::MEMSET.call(c, 0x003e_e7c4)?;
        Addr(r32(c, S1)).off(HASH_BUCKET_COUNT).sw(c, c.g[S0][0]);
        c.g[A0][0] = sx(r32(c, S0) << 2);
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e7d0)?;
        c.g[A2][0] = Addr(r32(c, S1)).off(HASH_BUCKET_COUNT).lw(c);
        c.g[A0][0] = c.g[V0][0];
        Addr(r32(c, S1)).off(HASH_BUCKETS).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(r32(c, A2) << 2);
        callee::MEMSET.call(c, 0x003e_e7e8)?;
        c.g[V1][0] = sx(HASH_DEFAULT_COMPARATOR);
        let have_entries = (c.g[S2][0] as i64) > 0;
        Addr(r32(c, S1)).off(HASH_COMPARE).sw(c, c.g[V1][0]);
        if have_entries {
            c.cycles += 3;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            callee::LOBBY_HASH_GROW_POOL.call(c, 0x003e_e804)?;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        hash_close_frame3(c, frame);
        Ok(())
    });
}

const LIST_SORT_DIRTY: u32 = 0x08;
const LIST_NOTIFY_DIRTY: u32 = 0x0c;
const LIST_VISIBLE_COUNT: u32 = 0x14;
const LIST_ENTRY_COUNT: u32 = 0x18;
const LIST_FILTER_ARG1: u32 = 0x1c;
const LIST_FILTER_ARG0: u32 = 0x20;
const LIST_FILTER: u32 = 0x24;
const LIST_SORT_ARG1: u32 = 0x28;
const LIST_SORT_ARG0: u32 = 0x2c;
const LIST_COMPARE: u32 = 0x30;
const LIST_HASH: u32 = 0x34;
const LIST_ENTRIES: u32 = 0x40;
const LIST_ENTRY_POINTER_BYTES: u32 = 4;
const LIST_COMPARE_THUNK: u32 = 0x003e_e1b0;
const LIST_FILTER_LOOP: u32 = 0x003e_e680;
const LIST_APPEND_HIDDEN_LOOP: u32 = 0x003e_e6f0;

fn list_sort_partition(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[S1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[S3][0] = addu(c.g[S1][0], c.g[V0][0]);
    c.g[S0][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S1][0];
    if c.g[S1][0] == c.g[S3][0] {
        return Ok(());
    }

    c.cycles += 1;
    loop {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, S2)).off(LIST_FILTER).lw(c);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let mut visible = true;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S2)).off(LIST_FILTER_ARG0).lw(c);
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            let filter = r32(c, V1);
            c.g[A1][0] = Addr(r32(c, S2)).off(LIST_FILTER_ARG1).lw(c);
            call_ptr(c, filter, 0x003e_e69c)?;
            visible = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        }
        if visible {
            c.cycles += 3;
            Addr(r32(c, S1)).sw(c, c.g[V0][0]);
            set_word(c, S1, r32(c, S1).wrapping_add(LIST_ENTRY_POINTER_BYTES));
        } else {
            c.cycles += 2;
            Addr(r32(c, S3)).sw(c, c.g[V0][0]);
            set_word(c, S3, r32(c, S3).wrapping_add(LIST_ENTRY_POINTER_BYTES));
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
        set_word(c, S0, r32(c, S0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
        c.g[V1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = addu(c.g[V1][0], c.g[V0][0]);
        let more = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[V1][0];
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, LIST_FILTER_LOOP);
        }
    }
}

fn list_sort_append_hidden(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
    let visible_bytes = r32(c, S1).wrapping_sub(r32(c, A0)) as i32;
    set_word(c, V0, (visible_bytes >> 2) as u32);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[S0][0] = addu(c.g[A0][0], c.g[V1][0]);
    let have_hidden = c.g[S0][0] != c.g[S3][0];
    Addr(r32(c, S2)).off(LIST_VISIBLE_COUNT).sw(c, c.g[V0][0]);
    if have_hidden {
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            set_word(c, S0, r32(c, S0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
            Addr(r32(c, S1)).sw(c, c.g[V0][0]);
            let more = c.g[S0][0] != c.g[S3][0];
            set_word(c, S1, r32(c, S1).wrapping_add(LIST_ENTRY_POINTER_BYTES));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LIST_APPEND_HIDDEN_LOOP);
            }
        }
    }
}

/// Returns the hash-table pointer attached to the lobby list.
pub fn lobby_list_get_hash(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(LIST_HASH).lw(c);
    ret(c);
}

/// Attaches a hash-table pointer to the lobby list.
pub fn lobby_list_set_hash(c: &mut Cpu) {
    c.cycles += 2;
    Addr(r32(c, A0)).off(LIST_HASH).sw(c, c.g[A1][0]);
    ret(c);
}

/// Repartitions a changed lobby list by visibility, then optionally sorts visible entries.
pub fn lobby_list_sort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, HASH_FRAME4_BYTES);
        frame.save(c, S2, HASH_FRAME4_S2);
        frame.save(c, S0, HASH_FRAME4_S0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, HASH_FRAME4_S1);
        frame.save(c, S3, HASH_FRAME4_S3);
        frame.save_ra(c, HASH_SAVED_RA);
        c.g[V1][0] = Addr(r32(c, S2)).off(LIST_SORT_DIRTY).lw(c);
        c.g[V0][0] = Addr(r32(c, S2)).off(LIST_NOTIFY_DIRTY).lw(c);
        let dirty = addu(c.g[V1][0], c.g[V0][0]) != 0;
        c.g[V0][0] = 0;
        if dirty {
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                Addr(r32(c, S2)).off(LIST_NOTIFY_DIRTY).sw(c, 0);
                c.g[V0][0] = 1;
            } else {
                c.g[V0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
                list_sort_partition(c)?;
                list_sort_append_hidden(c);
                c.cycles += 6;
                c.g[A1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
                c.g[V1][0] = Addr(r32(c, S2)).off(LIST_COMPARE).lw(c);
                let list_bytes = r32(c, S1).wrapping_sub(r32(c, A1)) as i32;
                set_word(c, V0, (list_bytes >> 2) as u32);
                let needs_sort = c.g[V1][0] != 0;
                Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).sw(c, c.g[V0][0]);
                if needs_sort {
                    c.cycles += 6;
                    c.g[A2][0] = Addr(r32(c, S2)).off(LIST_VISIBLE_COUNT).lw(c);
                    c.g[T0][0] = sx(LIST_COMPARE_THUNK);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A3][0] = u64::from(LIST_ENTRY_POINTER_BYTES);
                    callee::LOBBY_MERGE_SORT.call(c, 0x003e_e73c)?;
                }
                c.cycles += 3;
                Addr(r32(c, S2)).off(LIST_NOTIFY_DIRTY).sw(c, 0);
                c.g[V0][0] = 2;
                Addr(r32(c, S2)).off(LIST_SORT_DIRTY).sw(c, 0);
            }
        }
        c.cycles += 7;
        hash_close_frame4(c, frame);
        Ok(())
    });
}

/// Optionally increments the lobby list's notification counter and returns whether it is positive.
pub fn lobby_list_mark_dirty(c: &mut Cpu) {
    c.cycles += 2;
    let list = Addr(r32(c, A0));
    let mark = c.g[A1][0] != 0;
    c.g[V0][0] = list.off(LIST_NOTIFY_DIRTY).lw(c);
    if mark {
        c.cycles += 3;
        set_word(c, V0, r32(c, V0).wrapping_add(1));
        list.off(LIST_NOTIFY_DIRTY).sw(c, c.g[V0][0]);
        c.g[V0][0] = list.off(LIST_NOTIFY_DIRTY).lw(c);
    }
    c.cycles += 2;
    c.g[V0][0] = u64::from((c.g[V0][0] as i64) > 0);
    ret(c);
}

/// Sets the list's comparison callback and arguments, requesting a sort when entries exist.
pub fn lobby_list_set_sort(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(r32(c, A0));
    c.g[V0][0] = list.off(LIST_ENTRY_COUNT).lw(c);
    let has_entries = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 1;
    if has_entries {
        c.cycles += 1;
        list.off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
    }
    c.cycles += 4;
    list.off(LIST_COMPARE).sw(c, c.g[A3][0]);
    list.off(LIST_SORT_ARG0).sw(c, c.g[A1][0]);
    list.off(LIST_SORT_ARG1).sw(c, c.g[A2][0]);
    ret(c);
}

/// Returns the item at a valid lobby list index, otherwise zero.
pub fn lobby_list_get_at(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 4;
            let list = Addr(r32(c, A0));
            c.g[V0][0] = list.off(LIST_ENTRY_COUNT).lw(c);
            let in_bounds = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = u64::from(in_bounds);
            if in_bounds {
                c.g[V0][0] = list.off(LIST_ENTRIES).lw(c);
                c.cycles += 5;
                c.g[V1][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::{check_paths, check_paths_lohi, scramble, Rng, DATA};
    use crate::game::dirtysock::tagsunk::tests::{
        mem_fakes_hook_with_new, own_check_delivers, strcmp_fake_on, MEM_FAKES_CTRL,
        MEM_FAKES_MAGIC,
    };

    const TABLE: u32 = DATA + 0x100;
    const BUCKETS: u32 = DATA + 0x200;
    const FIRST_ENTRY: u32 = DATA + 0x300;
    const SECOND_ENTRY: u32 = DATA + 0x310;
    const FREE_ENTRY: u32 = DATA + 0x320;
    const POOL: u32 = DATA + 0x800;
    const STRING_KEY: u32 = DATA + 0x400;
    const OTHER_KEY: u32 = DATA + 0x410;
    const STRCMP_CALLBACK: u32 = 0x0041_65a8;
    const ALLOC_RESULT: u32 = MEM_FAKES_CTRL + 0x18;

    fn check_lobby_loop_events(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32) -> Events,
    ) {
        check_events(generated, hand, seeds, setup);
        own_check_delivers("lobby utility loop", generated, hand, seeds, setup);
    }

    fn hash_fixture(c: &mut Cpu, seed: u32, entry: u32, count: u32) -> Rng {
        let mut r = Rng(u64::from(entry) ^ u64::from(seed));
        scramble(c, &mut r);
        c.pc = entry;
        c.g[A0][0] = sx(TABLE);
        c.bus.write32(TABLE + HASH_BUCKET_COUNT, count);
        c.bus.write32(TABLE + HASH_BUCKETS, BUCKETS);
        c.bus.write32(TABLE + HASH_FREE_HEAD, FREE_ENTRY);
        c.bus.write32(TABLE + HASH_ENTRY_COUNT, 2);
        c.bus.write32(BUCKETS, 0);
        c.bus.write32(BUCKETS + 4, 0);
        c.bus.write32(FIRST_ENTRY, SECOND_ENTRY);
        c.bus.write32(SECOND_ENTRY, 0);
        c.bus
            .write32(FIRST_ENTRY + HASH_ENTRY_VALUE, r.next_u32() | 1);
        c.bus
            .write32(SECOND_ENTRY + HASH_ENTRY_VALUE, r.next_u32() | 1);
        r
    }

    #[test]
    fn ds_timezone_matches() {
        check_paths_lohi(
            ee::s14::f_003ef090__generated,
            ds_timezone,
            8,
            &[23, 762],
            &|c, seed| {
                scramble(c, &mut Rng(u64::from(seed)));
                c.pc = 0x003e_f090;
                let cached = [u32::MAX, 0, 3_600, (-18_000_i32) as u32][(seed % 4) as usize];
                c.bus.write32(r32(c, GP) + DS_TIMEZONE_GP, cached);
                // Select the SDK's precached development-kit settings path.
                c.bus.write8(0x0045_3400, b'0');
                c.bus.write8(0x0045_3404, b'T');
                c.bus.write8(0x0045_33fe, 0);
            },
        );
    }

    #[test]
    fn ds_timeinsecs_matches() {
        check_paths(ee::s14::f_003ef088__generated, ds_timeinsecs, 8, &[2], &|c, seed| {
            scramble(c, &mut Rng(u64::from(seed)));
            c.pc = 0x003e_f088;
        });
    }

    fn hash_int_lookup_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let r = hash_fixture(c, seed, entry, 2);
        let needle: i32 = if seed % 2 == 0 { 7 } else { -7 };
        c.g[A1][0] = sx(needle as u32);
        let bucket = BUCKETS + 4;
        let (head, first_key, second_key) = match seed / 2 % 4 {
            0 => (0, 1, 3),
            1 => (FIRST_ENTRY, needle, 3),
            2 => (FIRST_ENTRY, 1, needle),
            _ => (FIRST_ENTRY, 1, 3),
        };
        c.bus.write32(bucket, head);
        c.bus
            .write32(FIRST_ENTRY + HASH_ENTRY_KEY, first_key as u32);
        c.bus
            .write32(SECOND_ENTRY + HASH_ENTRY_KEY, second_key as u32);
        r
    }

    #[test]
    fn lobby_hash_del_int_matches() {
        check_paths_lohi(
            ee::s14::f_003eef40__generated,
            lobby_hash_del_int,
            8,
            &[16, 17, 27, 28, 33, 34, 29],
            &|c, seed| {
                hash_int_lookup_fixture(c, seed, 0x003e_ef40);
            },
        );
    }

    #[test]
    fn lobby_hash_find_int_matches() {
        check_paths_lohi(
            ee::s14::f_003eeee8__generated,
            lobby_hash_find_int,
            8,
            &[16, 17, 19, 20, 24, 25, 26, 27],
            &|c, seed| {
                hash_int_lookup_fixture(c, seed, 0x003e_eee8);
            },
        );
    }

    #[test]
    fn lobby_hash_int_lookup_events() {
        for (entry, original, hand, delay) in [
            (
                0x003e_ef40,
                ee::s14::f_003eef40__generated as GuestFn,
                lobby_hash_del_int as GuestFn,
                16,
            ),
            (
                0x003e_eee8,
                ee::s14::f_003eeee8__generated as GuestFn,
                lobby_hash_find_int as GuestFn,
                16,
            ),
        ] {
            check_lobby_loop_events(original, hand, 6, &|c, seed| {
                let mut r = hash_int_lookup_fixture(c, (seed / 3) + 4, entry);
                let mode = schedule(c, &mut r, seed, entry);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            });
        }
    }

    fn hash_insert_fixture(c: &mut Cpu, seed: u32, entry: u32, string_key: bool) {
        let mut r = hash_fixture(c, seed, entry, 2);
        let mode = seed / 2 % 3;
        c.bus.write32(
            TABLE + HASH_FREE_HEAD,
            if mode == 0 { FREE_ENTRY } else { 0 },
        );
        c.bus.write32(FREE_ENTRY, 0);
        c.bus.write32(TABLE + HASH_POOL_BLOCKS, 0);
        c.bus.write32(BUCKETS, FIRST_ENTRY);
        c.bus.write32(BUCKETS + 4, FIRST_ENTRY);
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        c.bus
            .write32(ALLOC_RESULT, if mode == 2 { POOL } else { 0 });
        c.bus.write8(STRING_KEY, 0);
        let key: i32 = if seed % 2 == 0 { 7 } else { -7 };
        c.g[A1][0] = if string_key {
            sx(STRING_KEY)
        } else {
            sx(key as u32)
        };
        c.g[A2][0] = sx(r.next_u32() | 1);
    }

    #[test]
    fn lobby_hash_add_int_matches() {
        mem_fakes_hook_with_new();
        check_paths_lohi(
            ee::s14::f_003eeda0__generated,
            lobby_hash_add_int,
            6,
            &[42, 43, 79, 80, 159, 160],
            &|c, seed| hash_insert_fixture(c, seed, 0x003e_eda0, false),
        );
    }

    #[test]
    fn lobby_hash_add_str_matches() {
        mem_fakes_hook_with_new();
        check_paths_lohi(
            ee::s14::f_003eeb30__generated,
            lobby_hash_add_str,
            6,
            &[51, 88, 168],
            &|c, seed| hash_insert_fixture(c, seed, 0x003e_eb30, true),
        );
    }

    fn hash_str_lookup_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = hash_fixture(c, seed, entry, 2);
        strcmp_fake_on(c);
        c.bus.write32(TABLE + HASH_COMPARE, STRCMP_CALLBACK);
        c.g[A1][0] = sx(STRING_KEY);
        c.g[A2][0] = sx(r.next_u32() | 1);
        c.bus.write8(STRING_KEY, 0);
        c.bus.write8(OTHER_KEY, b'x');
        c.bus.write8(OTHER_KEY + 1, 0);
        let (head, first_key, second_key) = match seed % 4 {
            0 => (0, OTHER_KEY, OTHER_KEY),
            1 => (FIRST_ENTRY, STRING_KEY, OTHER_KEY),
            2 => (FIRST_ENTRY, OTHER_KEY, STRING_KEY),
            _ => (FIRST_ENTRY, OTHER_KEY, OTHER_KEY),
        };
        c.bus.write32(BUCKETS, head);
        c.bus.write32(FIRST_ENTRY + HASH_ENTRY_KEY, first_key);
        c.bus.write32(SECOND_ENTRY + HASH_ENTRY_KEY, second_key);
        r
    }

    #[test]
    fn lobby_hash_del_str_matches() {
        check_paths(
            ee::s14::f_003eed08__generated,
            lobby_hash_del_str,
            4,
            &[37, 61, 79, 73],
            &|c, seed| {
                hash_str_lookup_fixture(c, seed, 0x003e_ed08);
            },
        );
    }

    #[test]
    fn lobby_hash_find_str_matches() {
        check_paths(
            ee::s14::f_003eec88__generated,
            lobby_hash_find_str,
            4,
            &[34, 53, 70, 71],
            &|c, seed| {
                hash_str_lookup_fixture(c, seed, 0x003e_ec88);
            },
        );
    }

    #[test]
    fn lobby_hash_replace_str_matches() {
        check_paths(
            ee::s14::f_003eebf8__generated,
            lobby_hash_replace_str,
            4,
            &[37, 57, 74],
            &|c, seed| {
                hash_str_lookup_fixture(c, seed, 0x003e_ebf8);
            },
        );
    }

    #[test]
    fn lobby_hash_str_lookup_events() {
        for (entry, original, hand, delay) in [
            (
                0x003e_ed08,
                ee::s14::f_003eed08__generated as GuestFn,
                lobby_hash_del_str as GuestFn,
                43,
            ),
            (
                0x003e_ec88,
                ee::s14::f_003eec88__generated as GuestFn,
                lobby_hash_find_str as GuestFn,
                46,
            ),
            (
                0x003e_ebf8,
                ee::s14::f_003eebf8__generated as GuestFn,
                lobby_hash_replace_str as GuestFn,
                48,
            ),
        ] {
            check_lobby_loop_events(original, hand, 6, &|c, seed| {
                let mut r = hash_str_lookup_fixture(c, 2 + seed / 3, entry);
                let mode = schedule(c, &mut r, seed, entry);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            });
        }
    }

    fn hash_string_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = hash_fixture(c, seed, 0x003e_ea48, 13);
        c.g[A1][0] = sx(STRING_KEY);
        let input: &[u8] = match seed % 4 {
            0 => b"\0",
            1 => b" A\0",
            2 => b"abC!\0",
            _ => b"\xff \0",
        };
        for (offset, &byte) in input.iter().enumerate() {
            c.bus.write8(STRING_KEY + offset as u32, byte);
        }
        r
    }

    #[test]
    fn lobby_hash_string_matches() {
        check_paths_lohi(
            ee::s14::f_003eea48__generated,
            lobby_hash_string,
            4,
            &[14, 35, 60, 32],
            &|c, seed| {
                hash_string_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_string_events() {
        check_lobby_loop_events(ee::s14::f_003eea48__generated, lobby_hash_string, 9, &|c, seed| {
            let mut r = hash_string_fixture(c, 2);
            let mode = schedule(c, &mut r, seed, 0x003e_ea48);
            if mode != Events::Off {
                c.next_event = c.cycles + 16 + 11 * u64::from(seed / 3);
            }
            mode
        });
    }

    fn hash_pop_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = hash_fixture(c, seed, 0x003e_e9a0, 3);
        let (count, cursor, occupied) =
            [(0, 0, 0), (2, 0, 0), (2, 0, 1), (2, 1, 0)][(seed % 4) as usize];
        c.bus.write32(TABLE + HASH_ENTRY_COUNT, count);
        c.bus.write32(TABLE + HASH_POP_BUCKET, cursor);
        c.bus.write32(BUCKETS + 8, 0);
        c.bus.write32(BUCKETS + 4 * occupied, FIRST_ENTRY);
        r
    }

    #[test]
    fn lobby_hash_pop_any_matches() {
        check_paths_lohi(
            ee::s14::f_003ee9a0__generated,
            lobby_hash_pop_any,
            4,
            &[6, 24, 39, 50],
            &|c, seed| {
                hash_pop_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_pop_any_events() {
        check_lobby_loop_events(ee::s14::f_003ee9a0__generated, lobby_hash_pop_any, 3, &|c, seed| {
            let mut r = hash_pop_fixture(c, 3);
            let mode = schedule(c, &mut r, seed, 0x003e_e9a0);
            if mode != Events::Off {
                c.next_event = c.cycles + 24;
            }
            mode
        });
    }

    fn hash_grow_pool_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = hash_fixture(c, seed, 0x003e_e900, 2);
        let count = [0, 1, 3, 1 << 30][(seed % 4) as usize];
        c.g[A1][0] = sx(count);
        c.bus.write32(TABLE + HASH_POOL_BLOCKS, DATA + 0x900);
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        c.bus
            .write32(ALLOC_RESULT, if seed / 4 % 2 == 0 { POOL } else { 0 });
        crate::difftest::fill(c, &mut r, POOL, 12);
        r
    }

    #[test]
    fn lobby_hash_grow_pool_matches() {
        mem_fakes_hook_with_new();
        check_paths_lohi(
            ee::s14::f_003ee900__generated,
            lobby_hash_grow_pool,
            8,
            &[40, 48, 57, 71],
            &|c, seed| {
                hash_grow_pool_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_grow_pool_events() {
        mem_fakes_hook_with_new();
        check_lobby_loop_events(ee::s14::f_003ee900__generated, lobby_hash_grow_pool, 6, &|c, seed| {
            let mut r = hash_grow_pool_fixture(c, 2);
            let mode = schedule(c, &mut r, seed, 0x003e_e900);
            if mode != Events::Off {
                c.next_event = c.cycles + 49 + 7 * u64::from(seed / 3);
            }
            mode
        });
    }

    fn hash_destroy_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = hash_fixture(c, seed, 0x003e_e820, 2);
        let blocks = seed % 3;
        c.bus.write32(
            TABLE + HASH_POOL_BLOCKS,
            if blocks == 0 { 0 } else { FIRST_ENTRY },
        );
        c.bus
            .write32(FIRST_ENTRY, if blocks == 2 { SECOND_ENTRY } else { 0 });
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        r
    }

    #[test]
    fn lobby_hash_destroy_matches() {
        mem_fakes_hook_with_new();
        check_paths(
            ee::s14::f_003ee820__generated,
            lobby_hash_destroy,
            3,
            &[37, 55, 72],
            &|c, seed| {
                hash_destroy_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_hash_destroy_events() {
        mem_fakes_hook_with_new();
        check_lobby_loop_events(ee::s14::f_003ee820__generated, lobby_hash_destroy, 3, &|c, seed| {
            let mut r = hash_destroy_fixture(c, 2);
            let mode = schedule(c, &mut r, seed, 0x003e_e820);
            if mode != Events::Off {
                c.next_event = c.cycles + 24;
            }
            mode
        });
    }

    const HASH_CREATE_ALLOC_CTRL: u32 = DATA + 0xc_4000;
    const HASH_CREATE_ALLOC_MAGIC: u32 = 0x003e_e778;
    const HASH_CREATE_ALLOC_INDEX: u32 = HASH_CREATE_ALLOC_CTRL + 4;

    fn hash_create_alloc(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};

        if c.bus.read32(HASH_CREATE_ALLOC_CTRL) != HASH_CREATE_ALLOC_MAGIC {
            return fall_back(
                c,
                prev_cell(hash_create_alloc),
                ee::s10::f_00317e30__generated,
            );
        }
        let index = c.bus.read32(HASH_CREATE_ALLOC_INDEX);
        c.bus.write32(HASH_CREATE_ALLOC_INDEX, index + 1);
        c.g[V0][0] = sx([TABLE, BUCKETS, POOL][index as usize]);
        c.cycles += 7;
        ret(c);
    }

    #[test]
    fn lobby_hash_create_matches() {
        use crate::difftest::fakes::{hook, prev_cell};

        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            hash_create_alloc,
            prev_cell(hash_create_alloc),
        );
        check_paths_lohi(
            ee::s14::f_003ee778__generated,
            lobby_hash_create,
            4,
            &[242, 256, 272, 286],
            &|c, seed| {
                let mut r = hash_fixture(c, seed, 0x003e_e778, 2);
                c.bus
                    .write32(HASH_CREATE_ALLOC_CTRL, HASH_CREATE_ALLOC_MAGIC);
                c.g[A0][0] = u64::from((seed % 2) * 3);
                c.g[A1][0] = u64::from((seed / 2 % 2) * 17);
                crate::difftest::fill(c, &mut r, TABLE, 9);
                crate::difftest::fill(c, &mut r, BUCKETS, 17);
                crate::difftest::fill(c, &mut r, POOL, 12);
            },
        );
    }

    fn list_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(u64::from(entry) ^ u64::from(seed));
        scramble(c, &mut r);
        c.pc = entry;
        c.g[A0][0] = sx(TABLE);
        crate::difftest::fill(c, &mut r, TABLE, 17);
        c.bus.write32(TABLE + LIST_ENTRIES, BUCKETS);
        c.bus.write32(BUCKETS, FIRST_ENTRY);
        c.bus.write32(BUCKETS + 4, SECOND_ENTRY);
        c.bus.write32(FIRST_ENTRY, r.next_u32() | 1);
        c.bus.write32(SECOND_ENTRY, r.next_u32() | 1);
        r
    }

    #[test]
    fn lobby_list_get_hash_matches() {
        check_paths(
            ee::s14::f_003ee770__generated,
            lobby_list_get_hash,
            6,
            &[2],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e770);
            },
        );
    }

    #[test]
    fn lobby_list_set_hash_matches() {
        check_paths(
            ee::s14::f_003ee768__generated,
            lobby_list_set_hash,
            6,
            &[2],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e768);
            },
        );
    }

    fn list_sort_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = list_fixture(c, seed, 0x003e_e620);
        let (sort_dirty, notify_dirty, count, filter, comparator, filter_sign) = match seed % 8 {
            0 => (0, 0, 0, 0, 0, false),
            1 => (0, 1, 0, 0, 0, false),
            2 => (1, 0, 0, 0, 0, false),
            3 => (1, 0, 1, 0, 0, false),
            4 => (1, 0, 2, 0, 0, false),
            5 => (1, 0, 2, STRCMP_CALLBACK, 0, true),
            6 => (1, 0, 2, STRCMP_CALLBACK, 0, false),
            _ => (1, 0, 1, 0, STRCMP_CALLBACK, false),
        };
        strcmp_fake_on(c);
        c.bus.write32(TABLE + LIST_SORT_DIRTY, sort_dirty);
        c.bus.write32(TABLE + LIST_NOTIFY_DIRTY, notify_dirty);
        c.bus.write32(TABLE + LIST_ENTRY_COUNT, count);
        c.bus.write32(TABLE + LIST_FILTER_ARG0, STRING_KEY);
        c.bus.write32(TABLE + LIST_FILTER_ARG1, OTHER_KEY);
        c.bus.write32(TABLE + LIST_FILTER, filter);
        c.bus.write32(TABLE + LIST_COMPARE, comparator);
        c.bus
            .write8(STRING_KEY, if filter_sign { b'z' } else { b'a' });
        c.bus.write8(STRING_KEY + 1, 0);
        c.bus.write8(OTHER_KEY, b'm');
        c.bus.write8(OTHER_KEY + 1, 0);
        c.bus.write32(BUCKETS + 8, FREE_ENTRY);
        c.bus.write32(BUCKETS + 12, FREE_ENTRY);
        r
    }

    #[test]
    fn lobby_list_sort_matches() {
        check_paths(
            ee::s14::f_003ee620__generated,
            lobby_list_sort,
            8,
            &[19, 24, 43, 57, 70, 102, 114, 93],
            &|c, seed| {
                list_sort_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_list_sort_events() {
        check_lobby_loop_events(ee::s14::f_003ee620__generated, lobby_list_sort, 6, &|c, seed| {
            let mut r = list_sort_fixture(c, 6);
            let mode = schedule(c, &mut r, seed, 0x003e_e620);
            if mode != Events::Off {
                let first_due = [48, 90][(seed / 3) as usize];
                c.next_event = c.cycles + first_due;
            }
            mode
        });
    }

    #[test]
    fn lobby_list_mark_dirty_matches() {
        check_paths(
            ee::s14::f_003ee600__generated,
            lobby_list_mark_dirty,
            8,
            &[4, 7],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e600);
                c.g[A1][0] = u64::from(seed % 2);
                c.bus.write32(
                    TABLE + LIST_NOTIFY_DIRTY,
                    [0, 2, u32::MAX, i32::MAX as u32][(seed / 2 % 4) as usize],
                );
            },
        );
    }

    #[test]
    fn lobby_list_set_sort_matches() {
        check_paths(
            ee::s14::f_003ee5a0__generated,
            lobby_list_set_sort,
            6,
            &[7, 8],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e5a0);
                c.bus.write32(
                    TABLE + LIST_ENTRY_COUNT,
                    [0, 1, u32::MAX][(seed % 3) as usize],
                );
            },
        );
    }

    #[test]
    fn lobby_list_get_at_matches() {
        check_paths(
            ee::s14::f_003ee560__generated,
            lobby_list_get_at,
            8,
            &[4, 6, 10, 13],
            &|c, seed| {
                list_fixture(c, seed, 0x003e_e560);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(TABLE) };
                c.g[A1][0] = sx([u32::MAX, 0, 1, 2][(seed / 2) as usize]);
                c.bus.write32(TABLE + LIST_ENTRY_COUNT, 2);
            },
        );
    }
}
