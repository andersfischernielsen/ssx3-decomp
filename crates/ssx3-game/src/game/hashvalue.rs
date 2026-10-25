//! `src/hashvalue.cpp`: hand-written replacements.

use crate::call::sp;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0031_7670, str_hash_guest, "StrHash")];

/// One step of the game's string hash: an ELF hash over *signed* chars that
/// folds the top nibble back in at bit 5 (`>> 23`, where the textbook ELF
/// hash uses `>> 24`).
#[inline(always)]
fn hash_step(h: u32, ch: u8) -> u32 {
    let h = (h << 4).wrapping_add(ch as i8 as i32 as u32);
    let top = h & 0xf000_0000;
    if top != 0 {
        h ^ (top >> 23) ^ top
    } else {
        h
    }
}

/// `GetHashValue32(const char*)` computed natively: the hash of `s` up to its
/// first NUL (or the end of the slice). Equals what the guest `StrHash`
/// (0x317670) returns for the same bytes.
pub fn str_hash(s: &[u8]) -> u32 {
    s.iter()
        .take_while(|&&b| b != 0)
        .fold(0, |h, &b| hash_step(h, b))
}

/// `int StrHash(const char* s)` (0x317670, `GetHashValue32`): the hash of a
/// NUL-terminated string. The original calls `StrHashElf(&tmp, s)`
/// (0x317618) on a stack temporary and returns its result; the callee is
/// inlined here (same frame, same stores, same cycles, the same event
/// check at its loop head 0x317630).
pub fn str_hash_guest(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    let mut p = c.g[A0][0] as u32;
    c.g[A1][0] = c.g[A0][0];
    frame.save_ra(c, 0x10);
    c.g[RA][0] = sx(0x0031_7684);
    let out = sp(c);
    c.g[A0][0] = sx(out);

    // StrHashElf(out, s).
    c.cycles += 4;
    let mut ch = Addr(p).lb(c);
    let mut h: u32 = 0;
    if ch != 0 {
        c.cycles += 2;
        const TOP: u64 = 0xffff_ffff_f000_0000;
        c.g[A3][0] = TOP;
        loop {
            c.cycles += 7;
            let mixed = (h << 4).wrapping_add(ch as u32);
            p = p.wrapping_add(1);
            h = mixed;
            let top = mixed & 0xf000_0000;
            if top != 0 {
                c.cycles += 3;
                h = mixed ^ (top >> 23) ^ top;
            }
            c.cycles += 3;
            ch = Addr(p).lb(c);
            if ch == 0 {
                break;
            }
            if event_due(c) {
                c.g[V0][0] = ch;
                c.g[A2][0] = ch & 0xff;
                c.g[V1][0] = sx(h);
                c.g[A1][0] = sx(p);
                deliver_event_here(c, 0x0031_7630);
            }
        }
    }
    c.cycles += 3;
    Addr(out).sw(c, h as u64);
    c.g[V1][0] = sx(h);
    c.g[V0][0] = sx(h);
    c.g[A1][0] = sx(p);
    c.g[A2][0] = 0;

    frame.restore_ra(c, 0x10);
    frame.close(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::schedule;
    use crate::difftest::fresh;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::RA as SENTINEL;

    /// Random strings: empty, short, long (the top-nibble fold), with high
    /// (negative) bytes.
    fn put_string(c: &mut Cpu, r: &mut Rng, seed: u32, at: u32) -> Vec<u8> {
        let len = match seed % 5 {
            0 => 0,
            1 => 1 + r.next_u32() % 4,
            2 => 7 + r.next_u32() % 3,
            _ => r.next_u32() % 200,
        };
        let mut s = Vec::new();
        for _ in 0..len {
            let mut b = r.next_u32() as u8;
            if seed % 2 == 0 {
                b &= 0x7f;
            }
            if b == 0 {
                b = b'a';
            }
            s.push(b);
            c.bus.write8(at + s.len() as u32 - 1, b);
        }
        c.bus.write8(at + len, 0);
        s
    }

    #[test]
    fn str_hash_guest_matches() {
        check(
            ssx3_ee::s10::f_00317670__generated,
            str_hash_guest,
            300,
            &|c, seed| {
                let mut r = Rng(0x3176_7000 ^ seed as u64);
                scramble(c, &mut r);
                let at = DATA + (r.next_u32() & 0xfff);
                put_string(c, &mut r, seed, at);
                c.g[A0][0] = sx(at);
            },
        );
    }

    /// Event delivery at the loop head (0x317630): delivered and
    /// unwind-requesting events (ignored there, as in the original).
    #[test]
    fn str_hash_guest_events_match() {
        check(
            ssx3_ee::s10::f_00317670__generated,
            str_hash_guest,
            90,
            &|c, seed| {
                let mut r = Rng(0x3176_7001 ^ seed as u64);
                scramble(c, &mut r);
                let at = DATA + 0x100;
                put_string(c, &mut r, 3 + seed % 2, at);
                c.g[A0][0] = sx(at);
                c.g[RA][0] = SENTINEL as u64;
                schedule(c, &mut r, seed, 0x0031_7670);
            },
        );
    }

    /// The native helper agrees with the guest function.
    #[test]
    fn str_hash_native_matches_guest() {
        let mut r = Rng(0x5157_4a54);
        let mut cases: Vec<Vec<u8>> = [
            &b"cGameLoadState"[..],
            &b""[..],
            &b"a"[..],
            &b"EventSelect"[..],
            &b"\xff\x80\x7f long string with high bytes \xe9\xe8 and more text"[..],
        ]
        .iter()
        .map(|s| s.to_vec())
        .collect();
        for _ in 0..40 {
            let n = r.next_u32() % 64;
            cases.push((0..n).map(|_| (r.next_u32() as u8).max(1)).collect());
        }
        for s in cases {
            let mut c = fresh();
            for (i, &b) in s.iter().enumerate() {
                c.bus.write8(DATA + i as u32, b);
            }
            c.bus.write8(DATA + s.len() as u32, 0);
            c.g[A0][0] = sx(DATA);
            c.g[RA][0] = SENTINEL as u64;
            c.pc = 0x0031_7670;
            ssx3_ee::s10::f_00317670__generated(&mut c);
            assert_eq!(c.pc, SENTINEL);
            assert_eq!(c.g[V0][0] as u32, str_hash(&s), "{s:x?}");
            assert_eq!(c.g[V0][0], sx(str_hash(&s)));
        }
    }
}
