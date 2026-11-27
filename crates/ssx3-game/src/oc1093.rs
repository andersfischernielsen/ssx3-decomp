//! Staging module oc1093 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1093`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003b_e6e0,
    sndenc_pack_header_mode2,
    "SNDENC_PackHeaderMode2",
)];

/// The encoder state pointer (`D_0050AC68`): +1 = block count (byte),
/// +8 = output buffer, +0x84 + 4*i = block pointers (their scale factors
/// start at +4).
const SNDENC_STATE: u32 = 0x0050_ac68;

/// Middle entries of one 21-byte packed block, in store order:
/// (sf index of the high part, its left shift, sf index of the low part, its
/// right shift). The destination offsets differ for even and odd blocks.
const SNDENC_PACK_SRC: [(u32, u32, u32, u32); 20] = [
    (0, 4, 1, 3),
    (1, 5, 2, 2),
    (2, 6, 3, 1),
    (3, 7, 4, 0),
    (5, 1, 6, 6),
    (6, 2, 7, 5),
    (7, 3, 8, 4),
    (8, 4, 9, 3),
    (9, 5, 10, 2),
    (10, 6, 11, 1),
    (11, 7, 12, 0),
    (13, 1, 14, 6),
    (14, 2, 15, 5),
    (15, 3, 16, 4),
    (16, 4, 17, 3),
    (17, 5, 18, 2),
    (18, 6, 19, 1),
    (19, 7, 20, 0),
    (21, 1, 22, 6),
    (22, 2, 23, 5),
];
/// Destination offsets of [`SNDENC_PACK_SRC`] for odd blocks (relative to the
/// block's cursor).
const SNDENC_PACK_DST_ODD: [u32; 20] = [
    2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17, 20, 19,
];
/// The same for even blocks.
const SNDENC_PACK_DST_EVEN: [u32; 20] = [
    0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14, 17, 16, 19, 18, 21,
];

/// `void SNDENC_PackHeaderMode2(Encoder* enc)` (0x3be6e0): packs the
/// scale-factor bytes (24 per block, 5 bits each) of every block of the
/// global encoder state into the output frame header at +0xaa, 21 bytes per
/// block (even and odd blocks interleave their bytes in opposite order and
/// share the boundary byte with the neighbour), then asks
/// `SNDENC_EncodeLfe` for the 8+1 LFE values of `enc->+0x14` and packs their
/// 3-bit parts at +0x113 of the output.
pub fn sndenc_pack_header_mode2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        let sp = c.g[SP][0] as u32;
        c.g[A1][0] = 0x0051_0000;
        c.sq(S0, sp.wrapping_add(0x30));
        c.g[T2][0] = c.g[A0][0];
        let ra_in = c.g[RA][0];
        Addr(sp.wrapping_add(0x20)).sd(c, ra_in);
        c.g[T0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(SNDENC_STATE).lw(c);
        c.g[A3][0] = 0;
        c.g[T1][0] = sx(sp.wrapping_add(0x10));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
        let count = Addr(c.g[V0][0] as u32).off(1).lbu(c);
        c.g[A0][0] = count;
        let mut cursor = (c.g[V1][0] as u32).wrapping_add(0xaa);
        c.g[S0][0] = sx(cursor);
        if count != 0 {
            c.cycles += 1;
            let mut i: u32 = 0;
            loop {
                c.cycles += 7;
                let state = Addr(SNDENC_STATE).lw(c) as u32;
                let blk = Addr(state.wrapping_add(i << 2)).off(0x84).lw(c) as u32;
                let sf = blk.wrapping_add(4);
                let odd = i & 1 != 0;
                let byte = |c: &mut Cpu, k: u32| Addr(sf.wrapping_add(k)).lbu(c) as u32;
                // First entry: the low 3 bits of the previous (odd) or of
                // this block's byte 1 (even) take sf[0] >> 4.
                let (first, dsts) = if odd {
                    c.cycles += 130;
                    (cursor.wrapping_sub(1), &SNDENC_PACK_DST_ODD)
                } else {
                    c.cycles += 129;
                    (cursor.wrapping_add(1), &SNDENC_PACK_DST_EVEN)
                };
                let old = Addr(first).lbu(c) as u32;
                let sf0 = byte(c, 0);
                Addr(first).sb(c, ((old & 0xf8) | (sf0 >> 4)) as u64);
                for (k, &(a, sl, b, sr)) in SNDENC_PACK_SRC.iter().enumerate() {
                    let hi = byte(c, a);
                    let lo = byte(c, b);
                    let v = (hi << sl) | (lo >> sr);
                    Addr(cursor.wrapping_add(dsts[k])).sb(c, v as u64);
                }
                // Last entry: keeps the low 3 bits of the byte it shares.
                let last = cursor.wrapping_add(if odd { 21 } else { 20 });
                let old = Addr(last).lbu(c) as u32;
                let sf23 = byte(c, 23);
                Addr(last).sb(c, ((old & 7) | (sf23 << 3)) as u64);
                let sf21 = byte(c, 21);
                let sf22 = byte(c, 22);
                // Loop tail.
                c.cycles += 7;
                c.g[V1][0] = Addr(SNDENC_STATE).lw(c);
                i += 1;
                cursor = cursor.wrapping_add(0x15);
                let n = Addr(c.g[V1][0] as u32).off(1).lbu(c);
                let more = (i as i64) < (n as i64);
                // Registers as the original leaves them at the back edge.
                c.g[V0][0] = more as u64;
                c.g[A0][0] = ((sf21 << 1) | (sf22 >> 6)) as u64;
                c.g[A1][0] = c.g[T0][0];
                c.g[A2][0] = sx(sf);
                c.g[A3][0] = sx(i);
                c.g[S0][0] = sx(cursor);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_e718);
                }
            }
        }
        // Mode 2 LFE part.
        c.cycles += 80;
        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_sub(0x5398)).lw(c);
        c.g[A1][0] = c.g[T1][0];
        c.g[A0][0] = Addr(c.g[T2][0] as u32).off(0x14).lw(c);
        c.g[A2][0] = c.g[SP][0];
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(8).lw(c);
        Callee {
            addr: 0x003b_d078,
            f: ee::s13::SNDENC_EncodeLfe,
        }
        .call(c, 0x003b_eb74)?;
        let sp = c.g[SP][0] as u32;
        let vals = |c: &mut Cpu, k: u32| Addr(sp.wrapping_add(k)).lbu(c);
        let s0 = c.g[S0][0] as u32;
        // out[0x112] = (out[0x112] & 0xf8) | (vals[0] >> 5)  (signed shift)
        let sra5 = |x: u64| sx(((sx((x as u32) << 24) as u32) as i32 >> 29) as u32);
        c.g[V0][0] = vals(c, 0);
        c.g[A0][0] = sx(s0.wrapping_sub(0x7eed));
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x7fff)).lbu(c);
        let cur = s0.wrapping_add(0x113);
        c.g[S0][0] = sx(cur);
        c.g[V0][0] = sra5(c.g[V0][0]);
        c.g[RA][0] = Addr(sp.wrapping_add(0x20)).ld(c);
        c.g[V1][0] &= 0xf8;
        c.g[V1][0] |= c.g[V0][0];
        let (at, v) = ((c.g[A0][0] as u32).wrapping_add(0x7fff), c.g[V1][0]);
        Addr(at).sb(c, v);
        // vals[j] << 5 | vals[j+1] >> 5 (arithmetic), into cursor[dst].
        const LFE_DST: [u32; 7] = [2, 1, 4, 3, 6, 5, 8];
        for (j, &dst) in LFE_DST.iter().enumerate() {
            let j = j as u32;
            c.g[V0][0] = vals(c, j + 1);
            c.g[V1][0] = vals(c, j);
            if j == 0 {
                c.g[A1][0] = Addr(cur).off(0xa).lbu(c);
            }
            c.g[V0][0] = sra5(c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
            c.g[V1][0] |= c.g[V0][0];
            if j == 0 {
                c.g[A1][0] &= 7;
            }
            let v = c.g[V1][0];
            Addr(cur.wrapping_add(dst)).sb(c, v);
        }
        c.g[A0][0] = vals(c, 7);
        c.g[V0][0] = vals(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
        c.g[A0][0] |= c.g[V0][0];
        let v = c.g[A0][0];
        Addr(cur.wrapping_add(7)).sb(c, v);
        c.g[V0][0] = vals(c, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
        c.g[A1][0] |= c.g[V0][0];
        let v = c.g[A1][0];
        Addr(cur.wrapping_add(0xa)).sb(c, v);
        c.lq(S0, sp.wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp.wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate and mailbox of the `SNDENC_EncodeLfe` fake (machine RAM only).
    const OC1093_GATE: u32 = DATA + 0x20_0100;
    const OC1093_MAGIC: u32 = 0x1093_f00d;
    const OC1093_REGLOG: u32 = DATA + 0x20_0400;
    static OC1093_PREV_LFE: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `SNDENC_EncodeLfe`: logs every register at the call,
    /// writes 8 values (a2) and the index byte (a1) from the mailbox at
    /// GATE+0x10, and returns (or unwinds when GATE+4 is set).
    fn oc1093_fake_lfe(c: &mut Cpu) {
        if c.bus.read32(OC1093_GATE) != OC1093_MAGIC {
            return fakes::fall_back(c, &OC1093_PREV_LFE, ee::s13::f_003bd078__generated);
        }
        for i in 0..32u32 {
            let v = c.g[i as usize][0];
            c.bus.write64(OC1093_REGLOG + 8 * i, v);
        }
        c.bus.write64(OC1093_REGLOG + 0x100, c.cycles);
        let (vals, idx) = (c.g[A2][0] as u32, c.g[A1][0] as u32);
        for k in 0..8u32 {
            let b = c.bus.read8(OC1093_GATE + 0x10 + k);
            c.bus.write8(vals + k, b);
        }
        let b = c.bus.read8(OC1093_GATE + 0x18);
        c.bus.write8(idx, b);
        c.cycles += 11;
        c.g[V0][0] = 0x77;
        c.pc = if c.bus.read32(OC1093_GATE + 4) != 0 {
            0xdead_0001
        } else {
            c.g[RA][0] as u32
        };
    }

    /// `SNDENC_PackHeaderMode2`: 0 to 5 blocks (even and odd paths), random
    /// scale factors and pre-existing output bytes, random LFE values with
    /// the high bit set (the arithmetic-shift quirk), and an unwinding callee.
    #[test]
    fn sndenc_pack_header_mode2_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003BD078),
            oc1093_fake_lfe,
            &OC1093_PREV_LFE,
        );
        check(
            ee::s13::f_003be6e0__generated,
            sndenc_pack_header_mode2,
            180,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3be6e0, 0);
                let state = DATA + 0x1000;
                let out = DATA + 0x4000;
                let n = seed % 6;
                c.bus.write32(SNDENC_STATE, state);
                c.bus.write8(state + 1, n as u8);
                c.bus.write32(state + 8, out);
                for i in 0..5u32 {
                    c.bus
                        .write32(state + 0x84 + 4 * i, DATA + 0x2000 + 0x40 * i);
                    for k in 0..24u32 {
                        let v = if seed % 11 == 0 {
                            0xff
                        } else {
                            r.next_u32() as u8
                        };
                        c.bus.write8(DATA + 0x2000 + 0x40 * i + 4 + k, v);
                    }
                }
                for k in 0..0x140u32 {
                    c.bus.write8(out + k, r.next_u32() as u8);
                }
                let enc = DATA + 0x3000;
                c.bus.write32(enc + 0x14, r.next_u32());
                c.g[A0][0] = enc as u64;
                c.bus.write32(OC1093_GATE, OC1093_MAGIC);
                c.bus.write32(OC1093_GATE + 4, (seed % 7 == 3) as u32);
                for k in 0..9u32 {
                    let v = match seed % 3 {
                        0 => r.next_u32() as u8,
                        1 => 0x80 | r.next_u32() as u8,
                        _ => r.next_u32() as u8 & 0x7f,
                    };
                    c.bus.write8(OC1093_GATE + 0x10 + k, v);
                }
            },
        );
    }
}
