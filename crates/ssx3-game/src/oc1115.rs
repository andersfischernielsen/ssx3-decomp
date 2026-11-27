//! Staging module oc1115 (wave wave269): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1115`.

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
    (0x003d_a910, spch_calc_load_percent, "SPCH_CalcLoadPercent"),
    (0x003d_ab58, spch_shuffle_choices, "SPCH_ShuffleChoices"),
    (0x003d_aca8, spch_roll_variation, "SPCH_RollVariation"),
    (
        0x003d_ace8,
        spch_prepare_event_samples,
        "SPCH_PrepareEventSamples",
    ),
    (
        0x003d_ae48,
        spch_notify_event_callback,
        "SPCH_NotifyEventCallback",
    ),
    (0x003d_b008, spch_roll_chance, "SPCH_RollChance"),
    (0x003d_b3e0, spch_get_event_param, "SPCH_GetEventParam"),
    (
        0x0040_2c30,
        libmpeg_dispatch_callback,
        "libmpeg_DispatchCallback",
    ),
];

/// `gp+0x1734` (0x4a4824): the word holding the speech bank table's address;
/// the table's word `[x]` is the bank pointer of entry `x`.
pub(crate) const SPCH_BANK_TABLE_GP: u32 = 0x1734;

/// `gp+0x1748` (0x4a4838): the load total, the denominator of the load percent.
pub(crate) const SPCH_LOAD_TOTAL_GP: u32 = 0x1748;

/// `SPCH_DecodeSampleEntry(bank, idx, row)` (0x3db838).
const SPCH_CALL_DECODE: Callee = Callee {
    addr: 0x003d_b838,
    f: ee::s14::SPCH_DecodeSampleEntry,
};

/// `SPCH_RandomNoRepeat(range, key)` (0x3db5c0).
const SPCH_CALL_RANDOM: Callee = Callee {
    addr: 0x003d_b5c0,
    f: ee::s14::SPCH_RandomNoRepeat,
};

/// `SPCH_FindBankDataPtr(speech_key, &out)` (0x3d83b8).
const SPCH_CALL_FIND_DATA: Callee = Callee {
    addr: 0x003d_83b8,
    f: ee::s14::SPCH_FindBankDataPtr,
};

/// `SPCH_FindBankType(speech_key, &out)` (0x3d8330).
const SPCH_CALL_FIND_TYPE: Callee = Callee {
    addr: 0x003d_8330,
    f: ee::s14::SPCH_FindBankType,
};

/// `SPCH_GetEventParam(header, idx)` (0x3db3e0).
const SPCH_CALL_EVENT_PARAM: Callee = Callee {
    addr: 0x003d_b3e0,
    f: ee::s14::SPCH_GetEventParam,
};

/// `int SPCH_CalcLoadPercent(SpeechPhrase* phrase, SpeechSel* sel)` (0x3da910):
/// the share of the load total (`gp+0x1748`) that a phrase takes, in percent.
/// Each of the phrase's `phrase[4] >> 2` slot conditions decodes its sample
/// (bank `table[sel.ent[i].x]`, the byte `sel.tbl[sel.ent[i].idx]`) into a row
/// info on the stack; the row's first word (length) is summed. The result is
/// `total * 100 / denominator`, or 0 when the denominator is 0.
pub fn spch_calc_load_percent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, RA, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = Addr(r32(c, A0)).off(4).lbu(c);
        c.g[V0][0] = sx(r32(c, V0) >> 2);
        let count = c.g[V0][0];
        c.g[S2][0] = 0;
        if count != 0 {
            c.cycles += 4;
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = count;
            c.g[S3][0] = sx(r32(c, A1).wrapping_add(0x62));
            loop {
                c.cycles += 12;
                c.g[V0][0] = Addr(r32(c, S0)).lh(c);
                c.g[A2][0] = c.g[SP][0];
                c.g[A0][0] = Addr(r32(c, GP)).off(SPCH_BANK_TABLE_GP).lw(c);
                c.g[V1][0] = Addr(r32(c, S0)).off(6).lbu(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                c.g[A1][0] = Addr(r32(c, V1)).lbu(c);
                SPCH_CALL_DECODE.call(c, 0x003d_a970)?;
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(sp(c)).lw(c);
                if found {
                    c.cycles += 1;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                }
                c.cycles += 3;
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                let more = c.g[S1][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_a948);
                }
            }
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, GP)).off(SPCH_LOAD_TOTAL_GP).lw(c);
        c.g[V0][0] = 0;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = 0x64;
            c.cycles += 3;
            c.mult(0, 2, 18, 2);
            c.div(0, 2, 3);
            c.g[V0][0] = c.lo[0];
        }
        c.cycles += 7;
        frame.restore(c, RA, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void SPCH_ShuffleChoices(SpeechPhrase* phrase, SpeechSel* sel)` (0x3dab58):
/// shuffles, for each of the phrase's `phrase[4] >> 2` entries, the sample
/// bytes of `sel.ent[i]` (`cnt` bytes at `sel.tbl[off]`) with a Fisher-Yates
/// pass that draws each position through `SPCH_RandomNoRepeat(j, -1)`.
pub fn spch_shuffle_choices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = 0;
        frame.save(c, S5, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, RA, 0x60);
        frame.save(c, S3, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x00);
        c.g[V0][0] = Addr(r32(c, A0)).off(4).lbu(c);
        c.g[S5][0] = sx(r32(c, V0) >> 2);
        c.g[S4][0] = c.g[A1][0];
        if c.g[S5][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, V1) << 3);
            loop {
                // One selection entry (index in V1, entry at V0 = V1 << 3).
                c.cycles += 8;
                c.g[S3][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V0)).off(4).lbu(c);
                c.g[S0][0] = Addr(r32(c, V0)).off(5).lbu(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x62));
                c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
                let short = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
                if !short {
                    c.cycles += 2;
                    c.g[S1][0] = sx(r32(c, S0).wrapping_add(r32(c, S2)));
                    loop {
                        // One draw: swap items[j-1] with items[pick], j counting down.
                        c.cycles += 12;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = u64::MAX;
                        SPCH_CALL_RANDOM.call(c, 0x003d_abc4)?;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_sub(1)).lbu(c);
                        c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                        c.g[A1][0] = ((c.g[S0][0] as i64) < 2) as u64;
                        let moved_in = c.g[V1][0];
                        let moved_out = c.g[A0][0];
                        Addr(r32(c, S1).wrapping_sub(1)).sb(c, moved_in);
                        Addr(r32(c, V0)).sb(c, moved_out);
                        let more = c.g[A1][0] == 0;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_abb8);
                        }
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.g[S3][0];
                let more = (c.g[V1][0] as i64) < (c.g[S5][0] as i64);
                c.g[V0][0] = sx(r32(c, V1) << 3);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_ab90);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, RA, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `int SPCH_RollVariation(SpeechSlot* slot)` (0x3daca8): rolls the slot's
/// variation: `slot[6] = slot[4] + SPCH_RandomNoRepeat(slot[5], key)` with the
/// signed halfword key at `slot[0]`; returns 1.
pub fn spch_roll_variation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, RA, 0x10);
        frame.save(c, S0, 0x00);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).off(5).lbu(c);
        c.g[A1][0] = Addr(r32(c, S0)).lh(c);
        SPCH_CALL_RANDOM.call(c, 0x003d_acc4)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lbu(c);
        frame.restore(c, RA, 0x10);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let rolled = c.g[V1][0];
        Addr(r32(c, S0)).off(6).sb(c, rolled);
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x00);
        frame.close(c);
        Ok(())
    })
}

/// `void SPCH_PrepareEventSamples(u8 speech_key, SpeechSel* sel, SpeechPhrase* phrase)`
/// (0x3dace8): when the speech bank has data for the key, decodes the samples
/// of the phrase's slot conditions and writes each `col` byte of a decoded row
/// into the bank's output table at the condition's `b[j]` index. Only the
/// conditions whose `a[j]` is 0xfe and whose `b[j]` has bit 7 clear are used.
pub fn spch_prepare_event_samples(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, FP, 0xb0);
        frame.save(c, S7, 0xa0);
        c.g[FP][0] = c.g[A1][0];
        frame.save(c, RA, 0xc0);
        c.g[S7][0] = c.g[A2][0];
        frame.save(c, S6, 0x90);
        c.g[A1][0] = sx(frame.at(c, 0x10));
        frame.save(c, S5, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x30);
        SPCH_CALL_FIND_DATA.call(c, 0x003d_ad24)?;
        let found = c.g[V0][0] != 0;
        frame.restore(c, RA, 0xc0);
        'epi: {
            if !found {
                break 'epi;
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S7)).off(4).lbu(c);
            c.g[A0][0] = 0;
            c.g[V0][0] = sx(r32(c, V0) >> 2);
            let n_zero = c.g[V0][0] == 0;
            let count = c.g[V0][0];
            Addr(frame.at(c, 0x14)).sw(c, count);
            if n_zero {
                break 'epi;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, S7).wrapping_add(8));
            let list = c.g[V0][0];
            Addr(frame.at(c, 0x18)).sw(c, list);
            c.g[V1][0] = Addr(frame.at(c, 0x18)).lw(c);
            loop {
                // One slot condition: list[i] is the index in A0.
                c.cycles += 8;
                c.g[S1][0] = 0;
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[S3][0] = sx(r32(c, S7).wrapping_add(r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, S3)).off(4).lb(c);
                let skip = (c.g[V0][0] as i64) <= 0;
                c.g[S6][0] = sx(r32(c, A0).wrapping_add(1));
                if !skip {
                    c.cycles += 6;
                    c.g[V0][0] = sx(r32(c, A0) << 3);
                    c.g[A3][0] = 0xfe;
                    c.g[S2][0] = sx(r32(c, FP).wrapping_add(r32(c, V0)));
                    c.g[S5][0] = sx(r32(c, FP).wrapping_add(0x62));
                    c.g[S4][0] = sx(r32(c, SP).wrapping_add(0xc));
                    c.g[S0][0] = sx(r32(c, S3).wrapping_add(0x10));
                    loop {
                        // One j of the condition: a[j] at S0-8, b[j] at S0, col[j] at S4+S1.
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_sub(8)).lbu(c);
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = Addr(r32(c, S3)).off(4).lb(c);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                            c.g[V0][0] &= 0x80;
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = Addr(r32(c, S3)).off(4).lb(c);
                            } else {
                                c.cycles += 13;
                                c.g[V0][0] = Addr(r32(c, S2)).lh(c);
                                c.g[A2][0] = c.g[SP][0];
                                c.g[A0][0] = Addr(r32(c, GP)).off(SPCH_BANK_TABLE_GP).lw(c);
                                c.g[V1][0] = Addr(r32(c, S2)).off(6).lbu(c);
                                c.g[V0][0] = sx(r32(c, V0) << 2);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
                                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                                c.g[A1][0] = Addr(r32(c, V1)).lbu(c);
                                frame.save(c, A3, 0x20);
                                SPCH_CALL_DECODE.call(c, 0x003d_add0)?;
                                let decoded = c.g[V0][0] != 0;
                                frame.restore(c, A3, 0x20);
                                if decoded {
                                    c.cycles += 6;
                                    c.g[A1][0] = Addr(r32(c, S0)).lbu(c);
                                    c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, S1)));
                                    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                                    c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                                    let col = c.g[A0][0];
                                    Addr(r32(c, V0)).sb(c, col);
                                }
                                c.cycles += 1;
                                c.g[V0][0] = Addr(r32(c, S3)).off(4).lb(c);
                            }
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_ad88);
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(frame.at(c, 0x14)).lw(c);
                c.g[A0][0] = c.g[S6][0];
                let more = (c.g[A0][0] as i64) < (c.g[V1][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = Addr(frame.at(c, 0x18)).lw(c);
                if !more {
                    c.cycles += 1;
                    frame.restore(c, RA, 0xc0);
                    break 'epi;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_ad50);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, FP, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SPCH_NotifyEventCallback(u8 speech_key, void* event, SpeechPhrase* phrase,
/// int first)` (0x3dae48): when the notify callback (`gp-0x70` table, field
/// +8) is set and the bank has a type for the key, calls it for every tag of
/// every slot condition of the phrase. The tag id comes from
/// `SPCH_GetEventParam(event, tag)`, the bit `1 << col[j]` from the decoded row.
pub fn spch_notify_event_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xd0);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, FP, 0xb0);
        frame.save(c, S7, 0xa0);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S7][0] = c.g[A2][0];
        frame.save(c, RA, 0xc0);
        c.g[S0][0] = sx(r32(c, V0).wrapping_sub(0x70));
        frame.save(c, S6, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S1, 0x40);
        let event = c.g[A1][0];
        Addr(frame.at(c, 0x14)).sw(c, event);
        c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        let no_callback = c.g[V0][0] == 0;
        let first = c.g[A3][0];
        Addr(frame.at(c, 0x18)).sw(c, first);
        'epi: {
            if no_callback {
                break 'epi;
            }
            c.cycles += 7;
            c.g[A1][0] = sx(frame.at(c, 0x10));
            SPCH_CALL_FIND_TYPE.call(c, 0x003d_ae9c)?;
            c.g[V1][0] = Addr(r32(c, S7)).off(4).lbu(c);
            c.g[A1][0] = 0;
            c.g[V1][0] = sx(r32(c, V1) >> 2);
            let no_tags = c.g[V1][0] == 0;
            let count = c.g[V1][0];
            Addr(frame.at(c, 0x1c)).sw(c, count);
            if no_tags {
                break 'epi;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, S7).wrapping_add(8));
            c.g[S6][0] = sx(frame.at(c, 0xc));
            let list = c.g[V0][0];
            Addr(frame.at(c, 0x20)).sw(c, list);
            c.g[V1][0] = Addr(frame.at(c, 0x20)).lw(c);
            loop {
                // One slot condition i (index in A1), first entry = firstEntry + i.
                c.cycles += 18;
                c.g[S0][0] = 0;
                c.g[A0][0] = Addr(frame.at(c, 0x18)).lw(c);
                c.g[S5][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                c.g[A0][0] = Addr(r32(c, V0)).lbu(c);
                c.g[V1][0] = sx(r32(c, V1) << 3);
                c.g[V0][0] = 0x0045_0000;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x70c));
                c.g[A0][0] = sx(r32(c, A0) << 2);
                c.g[S4][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = sx(r32(c, S7).wrapping_add(r32(c, A0)));
                c.g[V0][0] = 0x0045_0000;
                c.g[S2][0] = sx(r32(c, A0).wrapping_add(0xc));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x710));
                c.g[S3][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S0)));
                loop {
                    // One tag ref tbl[j] (j in S0, byte at V0).
                    c.cycles += 4;
                    c.g[V1][0] = 0xff;
                    c.g[A1][0] = Addr(r32(c, V0)).lbu(c);
                    if c.g[A1][0] == c.g[V1][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    } else {
                        c.cycles += 13;
                        c.g[A0][0] = Addr(frame.at(c, 0x14)).lw(c);
                        SPCH_CALL_EVENT_PARAM.call(c, 0x003d_af20)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V1][0] = Addr(r32(c, GP)).off(SPCH_BANK_TABLE_GP).lw(c);
                        c.g[V0][0] = Addr(r32(c, S4)).lh(c);
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A1][0] = Addr(r32(c, S3)).lhu(c);
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                        SPCH_CALL_DECODE.call(c, 0x003d_af44)?;
                        let decoded = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r32(c, S6).wrapping_add(r32(c, S0)));
                        if decoded {
                            c.cycles += 12;
                            c.g[A0][0] = 0x0045_0000;
                            c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x70));
                            c.g[V0][0] = 1;
                            c.g[T0][0] = Addr(r32(c, A0)).off(8).lw(c);
                            c.g[V1][0] = sx((c.g[V0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                            c.g[A3][0] = Addr(frame.at(c, 0x10)).lw(c);
                            c.g[A1][0] = c.g[S1][0];
                            let shifted = c.g[V1][0] << 32;
                            c.g[A2][0] = ((shifted as i64) >> 32) as u64;
                            let target = r32(c, T0);
                            c.g[A0][0] = c.g[FP][0];
                            call_ptr(c, target, 0x003d_af7c)?;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    }
                    c.cycles += 3;
                    let more_tags = (c.g[S0][0] as i64) < 4;
                    c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S0)));
                    if more_tags {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_af08);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(frame.at(c, 0x1c)).lw(c);
                c.g[A1][0] = c.g[S5][0];
                let more = (c.g[A1][0] as i64) < (c.g[V1][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[V1][0] = Addr(frame.at(c, 0x20)).lw(c);
                if !more {
                    break 'epi;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_aec0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, RA, 0xc0);
        frame.restore(c, FP, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SPCH_RollChance(u8* chance)` (0x3db008): true when a fresh draw of
/// `SPCH_RandomNoRepeat(100, -1)` is below `chance[1]`.
pub fn spch_roll_chance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = u64::MAX;
        frame.save(c, S0, 0x00);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, RA, 0x10);
        c.g[A0][0] = 0x64;
        SPCH_CALL_RANDOM.call(c, 0x003d_b024)?;
        c.g[V1][0] = Addr(r32(c, S0)).off(1).lbu(c);
        frame.restore(c, RA, 0x10);
        frame.restore(c, S0, 0x00);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `int SPCH_GetEventParam(const u8* header, int idx)` (0x3db3e0): the
/// `idx`-th of the header's three-byte entries (its tbl starts after
/// `((n * 2 + 3) & 0x3fc) + 0xc`, n at +6). Returns the entry's first byte,
/// or -1 for `idx >= 8`; the entry's three bytes are left on the stack.
pub fn spch_get_event_param(c: &mut Cpu) {
    c.cycles += 10;
    let n = Addr(r32(c, A0)).off(6).lbu(c) as u32;
    let frame = Frame::open(c, 0x10);
    c.g[A2][0] = u64::MAX;
    let stride = ((n << 1).wrapping_add(3) & 0x3fc).wrapping_add(0xc);
    c.g[V0][0] = sx(stride);
    let in_range = c.g[A1][0] < 8;
    c.g[V1][0] = in_range as u64;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(stride));
    if in_range {
        c.cycles += 10;
        let idx = r32(c, A1);
        c.g[V0][0] = sx(idx << 1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(idx));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        let entry = r32(c, V0);
        c.g[A1][0] = Addr(entry).lbu(c);
        c.g[V1][0] = Addr(entry).off(2).lbu(c);
        c.g[A0][0] = Addr(entry).off(1).lbu(c);
        c.g[A2][0] = c.g[A1][0];
        let (v1, a0, a1) = (c.g[V1][0], c.g[A0][0], c.g[A1][0]);
        Addr(frame.at(c, 8)).sw(c, v1);
        Addr(frame.at(c, 4)).sw(c, a0);
        Addr(frame.at(c, 0)).sw(c, a1);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0];
    frame.close(c);
}

/// `int libmpeg_DispatchCallback(mpeg, cbData, ...)` (0x402c30): calls the
/// callback of the mpeg system's entry `*cbData` (entries at `sys + 8 * idx`,
/// the function at +0xc, its argument at +0x10) and returns its result, or 0
/// when there is no mpeg, system, or callback.
pub fn libmpeg_dispatch_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        // The exit block (+1, ra reload) runs unless the call was skipped.
        let mut exit_block = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(r32(c, A0)).off(0x40).lw(c);
            frame.restore_ra(c, 0);
            exit_block = false;
            if c.g[A2][0] != 0 {
                c.cycles += 6;
                let idx = Addr(r32(c, A1)).lw(c) as u32;
                c.g[V0][0] = sx(idx << 3);
                c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
                c.g[V1][0] = Addr(r32(c, V1)).off(0xc).lw(c);
                let has_fn = c.g[V1][0] != 0;
                c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
                if has_fn {
                    c.cycles += 3;
                    let target = r32(c, V1);
                    c.g[A2][0] = Addr(r32(c, V0)).off(0x10).lw(c);
                    call_ptr(c, target, 0x0040_2c6c)?;
                    c.g[A3][0] = c.g[V0][0];
                    exit_block = true;
                }
            }
        }
        if exit_block {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A3][0];
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;
    const MAGIC: u32 = 0x1115_c0de;
    /// Gate words of this module's fakes (their own cells in machine RAM).
    const GATE_DECODE: u32 = DATA + 0x0071_0000;
    const GATE_FIND_DATA: u32 = DATA + 0x0071_0004;
    const GATE_FIND_TYPE: u32 = DATA + 0x0071_0008;
    /// The result the gated FindBankDataPtr fake returns (0 = no bank data).
    const RES_FIND_DATA: u32 = DATA + 0x0071_0010;
    /// Fixture addresses.
    const PHRASE: u32 = DATA + 0x1000;
    const SEL: u32 = DATA + 0x2000;
    const TABLE: u32 = DATA + 0x4000;
    const OUT: u32 = DATA + 0x6000;
    const SPEECH_KEY: u32 = DATA + 0x7000;
    const EVENT: u32 = DATA + 0x3000;
    /// `D_0044FF90.f08`: the notify callback.
    const NOTIFY_CALLBACK: u32 = 0x0044_ff98;
    /// `D_00450700`'s 8-byte entries (x halfword, y halfword at +4).
    const EVENT_QUEUE: u32 = 0x0045_070c;

    static PREV_DECODE: AtomicUsize = AtomicUsize::new(0);
    static PREV_FIND_DATA: AtomicUsize = AtomicUsize::new(0);
    static PREV_FIND_TYPE: AtomicUsize = AtomicUsize::new(0);

    /// Gated `SPCH_DecodeSampleEntry`: writes a row (length word, four col
    /// bytes at +0xc) at the row pointer and returns found or not from its
    /// arguments.
    fn oc1115_fake_decode(c: &mut Cpu) {
        if c.bus.read32(GATE_DECODE) != MAGIC {
            return fakes::fall_back(c, &PREV_DECODE, ssx3_ee::s14::f_003db838__generated);
        }
        let shape = c.g[A1][0] as u32;
        let row = c.g[A2][0] as u32;
        let bank = c.g[A0][0] as u32;
        c.bus.write32(row, (shape & 0x3f) + 1);
        for k in 0..4u32 {
            c.bus.write8(row + 0xc + k, shape.wrapping_add(5 * k) as u8);
        }
        c.g[V0][0] = (bank.wrapping_add(shape) % 3 != 0) as u64;
        ret(c);
    }

    /// Gated `SPCH_FindBankDataPtr`: the out pointer to the bank data, and the
    /// forced result from the mailbox.
    fn oc1115_fake_find_data(c: &mut Cpu) {
        if c.bus.read32(GATE_FIND_DATA) != MAGIC {
            return fakes::fall_back(c, &PREV_FIND_DATA, ssx3_ee::s14::f_003d83b8__generated);
        }
        let out = c.g[A1][0] as u32;
        c.bus.write32(out, OUT);
        c.g[V0][0] = c.bus.read32(RES_FIND_DATA) as u64;
        ret(c);
    }

    /// Gated `SPCH_FindBankType`: a type pointer for the out argument.
    fn oc1115_fake_find_type(c: &mut Cpu) {
        if c.bus.read32(GATE_FIND_TYPE) != MAGIC {
            return fakes::fall_back(c, &PREV_FIND_TYPE, ssx3_ee::s14::f_003d8330__generated);
        }
        let out = c.g[A1][0] as u32;
        c.bus.write32(out, OUT + 0x40);
        c.g[V0][0] = 1;
        ret(c);
    }

    fn install_fakes() {
        fakes::hook(
            addr_of_mut!(ssx3_ee::s14::OVERRIDE_003DB838),
            oc1115_fake_decode,
            &PREV_DECODE,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s14::OVERRIDE_003D83B8),
            oc1115_fake_find_data,
            &PREV_FIND_DATA,
        );
        fakes::hook(
            addr_of_mut!(ssx3_ee::s14::OVERRIDE_003D8330),
            oc1115_fake_find_type,
            &PREV_FIND_TYPE,
        );
    }

    /// The speech bank table: its word at `gp+0x1734` points at `TABLE`,
    /// entry `x` is a bank pointer.
    fn write_bank_table(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0x1734, TABLE);
        for x in 0..8u32 {
            c.bus.write32(TABLE + 4 * x, DATA + 0x5000 + 0x100 * x);
        }
    }

    /// A speech phrase at `p`: the count byte (`count << 2`, low bits random),
    /// four slot conditions listed at +8 (condition i at `4 * list[i]`: n at
    /// +4, a[] at +8, tag refs at +0xc, b[] at +0x10).
    fn write_phrase(c: &mut Cpu, r: &mut Rng, p: u32, count: u32) {
        c.bus
            .write8(p + 4, ((count << 2) | (r.next_u32() & 3)) as u8);
        for i in 0..4u32 {
            let list = 0x10 + 8 * i;
            c.bus.write8(p + 8 + i, list as u8);
            let cond = p + 4 * list;
            c.bus.write8(cond + 4, (r.next_u32() % 4) as u8);
            for j in 0..4u32 {
                let a = if r.next_u32() % 4 == 0 { 0x40 } else { 0xfe };
                c.bus.write8(cond + 8 + j, a);
                let tag = if r.next_u32() % 3 == 0 {
                    0xff
                } else {
                    ((i + j) & 7) as u8
                };
                c.bus.write8(cond + 0xc + j, tag);
                c.bus.write8(cond + 0x10 + j, r.next_u32() as u8);
            }
        }
    }

    /// A speech selection at `s`: four 8-byte entries (x, off at +4, cnt at
    /// +5, idx at +6) and the byte table at +0x62.
    fn write_sel(c: &mut Cpu, r: &mut Rng, s: u32) {
        for i in 0..4u32 {
            let e = s + 8 * i;
            c.bus.write16(e, (r.next_u32() % 8) as u16);
            c.bus.write8(e + 4, (0x10 * i) as u8);
            c.bus.write8(e + 5, (r.next_u32() % 6) as u8);
            c.bus.write8(e + 6, (r.next_u32() % 8) as u8);
        }
        fill(c, r, s + 0x62, 0x20);
    }

    #[test]
    fn spch_calc_load_percent_matches() {
        install_fakes();
        check(
            ssx3_ee::s14::f_003da910__generated,
            spch_calc_load_percent,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3da9_1000, 0);
                c.bus.write32(GATE_DECODE, MAGIC);
                write_bank_table(c);
                c.bus.write32(
                    GAME_GP + 0x1748,
                    if seed % 3 == 0 {
                        0
                    } else {
                        37 * (seed % 5 + 1)
                    },
                );
                write_phrase(c, &mut r, PHRASE, seed % 4);
                write_sel(c, &mut r, SEL);
                c.g[A0][0] = PHRASE as u64;
                c.g[A1][0] = SEL as u64;
            },
        );
    }

    #[test]
    fn spch_shuffle_choices_matches() {
        check(
            ssx3_ee::s14::f_003dab58__generated,
            spch_shuffle_choices,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3dab_5800, 0);
                write_phrase(c, &mut r, PHRASE, seed % 4);
                write_sel(c, &mut r, SEL);
                c.g[A0][0] = PHRASE as u64;
                c.g[A1][0] = SEL as u64;
            },
        );
    }

    #[test]
    fn spch_roll_variation_matches() {
        check(
            ssx3_ee::s14::f_003daca8__generated,
            spch_roll_variation,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3daca800, 0);
                let slot = DATA + 0x1000;
                let key = if seed % 4 == 0 {
                    0xffff
                } else {
                    r.next_u32() as u16
                };
                c.bus.write16(slot, key);
                c.bus.write8(slot + 4, r.next_u32() as u8);
                c.bus.write8(slot + 5, (seed % 6) as u8);
                c.bus.write8(slot + 6, r.next_u32() as u8);
                c.g[A0][0] = slot as u64;
            },
        );
    }

    #[test]
    fn spch_prepare_event_samples_matches() {
        install_fakes();
        check(
            ssx3_ee::s14::f_003dace8__generated,
            spch_prepare_event_samples,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3dac_e800, 0);
                c.bus.write32(GATE_DECODE, MAGIC);
                c.bus.write32(GATE_FIND_DATA, MAGIC);
                c.bus
                    .write32(RES_FIND_DATA, if seed % 5 == 0 { 0 } else { 1 });
                write_bank_table(c);
                write_sel(c, &mut r, SEL);
                write_phrase(c, &mut r, PHRASE, seed % 4);
                c.g[A0][0] = SPEECH_KEY as u64;
                c.g[A1][0] = SEL as u64;
                c.g[A2][0] = PHRASE as u64;
            },
        );
    }

    #[test]
    fn spch_notify_event_callback_matches() {
        install_fakes();
        check(
            ssx3_ee::s14::f_003dae48__generated,
            spch_notify_event_callback,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3dae_4800, 0);
                c.bus.write32(GATE_DECODE, MAGIC);
                c.bus.write32(GATE_FIND_TYPE, MAGIC);
                c.bus.write32(
                    NOTIFY_CALLBACK,
                    if seed % 5 == 0 { 0 } else { fakes::FAKE + 0x40 },
                );
                write_bank_table(c);
                for k in 0..8u32 {
                    let e = EVENT_QUEUE + 8 * k;
                    c.bus.write16(e, (r.next_u32() % 8) as u16);
                    c.bus.write16(e + 4, r.next_u32() as u16);
                }
                fill(c, &mut r, EVENT, 0x10);
                write_phrase(c, &mut r, PHRASE, seed % 4);
                c.g[A0][0] = SPEECH_KEY as u64;
                c.g[A1][0] = EVENT as u64;
                c.g[A2][0] = PHRASE as u64;
                c.g[A3][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn spch_roll_chance_matches() {
        check(
            ssx3_ee::s14::f_003db008__generated,
            spch_roll_chance,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3db0_0800, 0);
                let rec = DATA + 0x1000;
                c.bus.write8(rec, r.next_u32() as u8);
                c.bus.write8(rec + 1, (seed % 101) as u8);
                c.g[A0][0] = rec as u64;
            },
        );
    }

    #[test]
    fn spch_get_event_param_matches() {
        check(
            ssx3_ee::s14::f_003db3e0__generated,
            spch_get_event_param,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3db3_e000, 0);
                let header = DATA + 0x1000;
                fill(c, &mut r, header, 0x40);
                c.bus.write8(header + 6, (seed % 7) as u8);
                c.g[A0][0] = header as u64;
                c.g[A1][0] = (seed % 11) as u64;
            },
        );
    }

    #[test]
    fn libmpeg_dispatch_callback_matches() {
        check(
            ssx3_ee::s15::f_00402c30__generated,
            libmpeg_dispatch_callback,
            SEEDS,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0040_2c30, 0);
                let mpeg = DATA + 0x1000;
                let sys = DATA + 0x2000;
                let cb = DATA + 0x3000;
                c.bus
                    .write32(mpeg + 0x40, if seed % 6 == 0 { 0 } else { sys });
                c.bus.write32(cb, seed % 4);
                for k in 0..4u32 {
                    let e = sys + 8 * k;
                    let func = if (seed + k) % 5 == 0 {
                        0
                    } else {
                        fakes::FAKE + 0x40 + 0x10 * k
                    };
                    c.bus.write32(e + 0xc, func);
                    c.bus.write32(e + 0x10, r.next_u32());
                }
                c.g[A0][0] = if seed % 7 == 0 { 0 } else { mpeg as u64 };
                c.g[A1][0] = cb as u64;
            },
        );
    }
}
