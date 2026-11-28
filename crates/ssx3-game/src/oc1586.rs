//! Staging module oc1586 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1586`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_559c, store_const_4958e8, "store_const_4958e8"),
    (0x003c_55ac, store_const_4958f0, "store_const_4958f0"),
    (0x003c_55bc, store_const_4958f8, "store_const_4958f8"),
    (0x003c_55cc, store_const_495910, "store_const_495910"),
    (0x003c_c878, snd_mix_scale_wet_bus, "SND_MixScaleWetBus"),
    (0x003d_0c78, path_set_flag_bit26_if_reached, "PATH_SetFlagBit26IfReached"),
    (0x003d_15c8, path_set_track_lookahead, "PATH_SetTrackLookahead"),
    (0x003d_2138, pending_item_flag, "PendingItem_Bit26"),
];

/// Stores `value` at `*a1` and returns 0 through the shared tail at
/// `0x3c55f4` (which costs 2 cycles and sets v0 = 0). The four stubs at
/// `0x3c559c`..`0x3c55cc` differ only in the stored constant.
fn stub_store_then_return_zero(c: &mut Cpu, value: u32) {
    c.cycles += 4;
    c.g[V0][0] = sx(value);
    let a1 = c.g[A1][0] as u32;
    let v = c.g[V0][0];
    c.sw(a1, v);
    // Tail at 0x3c55f4: v0 = 0 and return through ra.
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `0x3c559c`: stores `0x4958e8` at `*a1`, returns 0.
pub fn store_const_4958e8(c: &mut Cpu) {
    stub_store_then_return_zero(c, 0x0049_58e8);
}

/// `0x3c55ac`: stores `0x4958f0` at `*a1`, returns 0.
pub fn store_const_4958f0(c: &mut Cpu) {
    stub_store_then_return_zero(c, 0x0049_58f0);
}

/// `0x3c55bc`: stores `0x4958f8` at `*a1`, returns 0.
pub fn store_const_4958f8(c: &mut Cpu) {
    stub_store_then_return_zero(c, 0x0049_58f8);
}

/// `0x3c55cc`: stores `0x495910` at `*a1`, returns 0.
pub fn store_const_495910(c: &mut Cpu) {
    stub_store_then_return_zero(c, 0x0049_5910);
}

/// `void SND_MixScaleWetBus(int n)` (0x3cc878): when the wet-bus buffer
/// pointer (`sndmix.wetbusactive`, at 0x515b54) is set, scales `4 * n` bytes
/// of float vectors at 0x515d80 by 127 with VU0, converts them to integers
/// into the wet-bus buffer and clears the source. `n` counts words and must
/// be a multiple of 16 (the loop runs in 0x40-byte blocks).
pub fn snd_mix_scale_wet_bus(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0051_0000;
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5b54)).lw(c);
    let wet_null = c.g[V1][0] == 0;
    c.g[A3][0] = c.g[A0][0];
    if !wet_null {
        c.cycles += 10;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d80));
        c.g[A0][0] = c.g[A3][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[V1][0];
        c.g[T0][0] = 0x7f;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.qmtc2(8, 1);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.vu0().itof(0, 0b1111, 1, 1);
        loop {
            c.cycles += 23;
            c.lqc2(2, (c.g[A1][0] as u32));
            c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.vu0().mul(0b1111, 2, 2, Src::Bc(1, X));
            c.sq(0, (c.g[A1][0] as u32));
            c.vu0().mul(0b1111, 3, 3, Src::Bc(1, X));
            c.sq(0, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.vu0().mul(0b1111, 4, 4, Src::Bc(1, X));
            c.sq(0, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.vu0().mul(0b1111, 5, 5, Src::Bc(1, X));
            c.sq(0, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
            c.vu0().ftoi(0, 0b1111, 2, 2);
            c.vu0().ftoi(0, 0b1111, 3, 3);
            c.vu0().ftoi(0, 0b1111, 4, 4);
            c.vu0().ftoi(0, 0b1111, 5, 5);
            c.sqc2(2, (c.g[A2][0] as u32).wrapping_sub(0x40));
            c.sqc2(3, (c.g[A2][0] as u32).wrapping_sub(0x30));
            c.sqc2(4, (c.g[A2][0] as u32).wrapping_sub(0x20));
            let more = c.g[A0][0] != c.g[A1][0];
            c.sqc2(5, (c.g[A2][0] as u32).wrapping_sub(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_c8b0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void PATH_SetFlagBit26IfReached(char* p, unsigned limit)` (0x3d0c78):
/// when `p + 0x10` is not above `limit`, sets bit 26 of the word at `p + 0x18`.
pub fn path_set_flag_bit26_if_reached(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
    let past_limit = c.g[A1][0] != 0;
    c.g[V1][0] = 0x0400_0000;
    if !past_limit {
        c.cycles += 3;
        let flags = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lw(c) | c.g[V1][0];
        c.g[V0][0] = flags;
        let v = c.g[V0][0];
        Addr((c.g[A0][0] as u32).wrapping_add(0x8)).sw(c, v);
    }
    c.cycles += 2;
    ret(c);
}

/// The path table's FindTrack (`0x3d5290`), reached through its override slot.
const PATH_FIND_TRACK: Callee = Callee {
    addr: 0x003d_5290,
    f: ee::s14::f_003d5290,
};

/// `int PATH_SetTrackLookahead(int mask, int value)` (0x3d15c8): stores
/// `value & 0xffff` at `+0x24` of the channel that `PATH_FindTrack` returns;
/// -8 when there is no channel, else 0.
pub fn path_set_track_lookahead(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        PATH_FIND_TRACK.call(c, 0x003d_15dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = c.g[S0][0] & 0xffff;
        if c.g[A0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_fff8;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0;
            let v = c.g[V1][0];
            Addr((c.g[A0][0] as u32).wrapping_add(0x24)).sw(c, v);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PendingItem_Bit26(event, itemType)` (0x3d2138): looks through the
/// sixteen pending-item lists (the table at `*(0x44f420) + 0xdc`) for one
/// whose key byte (+0xc) equals the event's byte at +0xc and whose items
/// (count `h & 0x3f` at +0xe, 0x10-byte items from +0x10) hold `itemType` at
/// +9. Returns bit 26 of that item's word at +8, or 0 when none matches.
pub fn pending_item_flag(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0045_0000;
    c.g[T2][0] = 0;
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0xbe0)).lw(c);
    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xdc));
    'l_3d2148: loop {
        'b_3d21ac: {
            c.cycles += 3;
            c.g[A3][0] = Addr(c.g[T1][0] as u32).lw(c);
            if c.g[A3][0] == 0 {
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                break 'b_3d21ac;
            }
            c.cycles += 4;
            c.g[V1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xc)).lbu(c);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lbu(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                break 'b_3d21ac;
            }
            'b_3d21a8: {
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0xe)).lhu(c);
                c.g[V0][0] &= 0x3f;
                let empty = c.g[V0][0] == 0;
                c.g[A2][0] = 0;
                if empty {
                    break 'b_3d21a8;
                }
                c.cycles += 3;
                c.g[T0][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                'l_3d2180: loop {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x9)).lbu(c);
                    let other_type = c.g[V0][0] != c.g[A1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if other_type {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A2][0] < c.g[T0][0]) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x003d_2180);
                            }
                            continue 'l_3d2180;
                        } else {
                            break 'b_3d21a8;
                        }
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 26);
                        c.g[V0][0] &= 0x1;
                        ret(c);
                        return;
                    }
                }
            }
            c.cycles += 1;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T2][0] as i64) < 16i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_2148);
            }
            continue 'l_3d2148;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            ret(c);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word and mailbox for this module's gated fake (unique in the
    /// machine's RAM; the fake answers only when the magic is set).
    const OC1586_CTRL: u32 = DATA + 0x0020_0000;
    const OC1586_MAGIC: u32 = 0x1586_0d15;

    /// Leaf stand-in for the path table's FindTrack (`0x3d5290`): returns the
    /// mailbox word at `CTRL + 4` in v0 and leaves the other registers alone.
    pub(crate) fn oc1586_fake_find_track(c: &mut Cpu) {
        if c.bus.read32(OC1586_CTRL) != OC1586_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1586_fake_find_track),
                ee::s14::f_003d5290__generated,
            );
        }
        c.g[V0][0] = sx(c.bus.read32(OC1586_CTRL + 4));
        c.pc = c.g[RA][0] as u32;
    }

    /// Puts the gated fake in the FindTrack override slot (once per process).
    pub(crate) fn oc1586_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D5290),
            oc1586_fake_find_track,
            fakes::prev_cell(oc1586_fake_find_track),
        );
    }

    #[test]
    fn store_const_stubs_match() {
        let stubs: [(GuestFn, GuestFn); 4] = [
            (ee::s13::f_003c559c__generated, store_const_4958e8),
            (ee::s13::f_003c55ac__generated, store_const_4958f0),
            (ee::s13::f_003c55bc__generated, store_const_4958f8),
            (ee::s13::f_003c55cc__generated, store_const_495910),
        ];
        for (generated, hand) in stubs {
            check(generated, hand, 64, &|c, seed| {
                let mut r = Rng(0x1586_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = sx(DATA + 0x5000 + (seed % 16) * 4);
            });
        }
    }

    #[test]
    fn snd_mix_scale_wet_bus_matches() {
        check(
            ee::s14::f_003cc878__generated,
            snd_mix_scale_wet_bus,
            200,
            &|c, seed| {
                let mut r = Rng(0x1586_2000 ^ seed as u64);
                scramble(c, &mut r);
                // n is a word count and must be a multiple of 16.
                let n = 16 * (1 + seed % 4);
                c.g[A0][0] = n as u64;
                // Null wet-bus pointer skips the loop; otherwise the output
                // is a fresh buffer.
                let dst = if seed % 5 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(0x0051_5b54, dst);
                fill(c, &mut r, 0x0051_5d80, n);
            },
        );
    }

    #[test]
    fn path_set_flag_bit26_if_reached_matches() {
        check(
            ee::s14::f_003d0c78__generated,
            path_set_flag_bit26_if_reached,
            200,
            &|c, seed| {
                let mut r = Rng(0x1586_3000 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + 0x4000 + (seed % 4) * 0x40;
                c.g[A0][0] = sx(base);
                // A limit at or above p + 0x10 sets the bit; below it does not.
                let limit = if seed % 2 == 0 {
                    base + 0x10 + seed % 5
                } else {
                    base + 0xf - seed % 3
                };
                c.g[A1][0] = sx(limit);
                c.bus.write32(base + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn path_set_track_lookahead_matches() {
        oc1586_hook();
        check(
            ee::s14::f_003d15c8__generated,
            path_set_track_lookahead,
            200,
            &|c, seed| {
                let mut r = Rng(0x1586_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = sx(r.next_u32());
                // Mailbox: 0 = no channel (returns -8), else a channel.
                c.bus.write32(OC1586_CTRL, OC1586_MAGIC);
                let channel = if seed % 3 == 0 { 0 } else { DATA + 0x6000 };
                c.bus.write32(OC1586_CTRL + 4, channel);
            },
        );
    }

    #[test]
    fn pending_item_flag_matches() {
        check(
            ee::s14::f_003d2138__generated,
            pending_item_flag,
            200,
            &|c, seed| {
                let mut r = Rng(0x1586_5000 ^ seed as u64);
                scramble(c, &mut r);
                let event = DATA + 0x1000;
                c.bus.write8(event + 0xc, (seed % 4) as u8);
                c.g[A0][0] = sx(event);
                c.g[A1][0] = (seed % 6) as u64;
                // The table pointer at 0x44f420; its lists start at +0xdc.
                let table = DATA + 0x2000;
                c.bus.write32(0x0044_f420, table);
                for i in 0..16u32 {
                    let list = DATA + 0x3000 + i * 0x100;
                    if (seed + i) % 3 == 0 {
                        c.bus.write32(table + 0xdc + i * 4, 0);
                        continue;
                    }
                    c.bus.write32(table + 0xdc + i * 4, list);
                    c.bus.write8(list + 0xc, ((seed + i) % 4) as u8);
                    c.bus.write16(list + 0xe, (r.next_u32() % 6) as u16);
                    for j in 0..6u32 {
                        let item = list + 0x10 + j * 0x10;
                        c.bus.write8(item + 9, (r.next_u32() % 6) as u8);
                        let flag = if (seed + j) % 2 == 0 { 0x0400_0000 } else { 0 };
                        c.bus.write32(item + 8, flag | (r.next_u32() & 0x00ff_ffff));
                    }
                }
            },
        );
    }
}
