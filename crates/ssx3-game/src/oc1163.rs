//! Staging module oc1163 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1163`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due, Resume};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_fb68, snd_isqrt, "SND_ISqrt"),
    (0x003c_fb98, snd_next_prime_delay, "SND_NextPrimeDelay"),
    (0x0040_2bb0, libmpeg_clear_frame_flags, "libmpeg_ClearFrameFlags"),
];

/// Loop head of `SND_ISqrt`'s counting loop (0x3cfb70): its event check.
const ISQRT_LOOP: u32 = 0x003c_fb70;

/// `int SND_ISqrt(int n)` (0x3cfb68, `SNDI_cheapsqrt`): the integer square
/// root by counting squares, floor(sqrt(n)) for n >= 0 and -1 for n < 0.
/// The loop's event check ignores an unwind request, as the generated body
/// does (the function returns normally either way).
pub fn snd_isqrt(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0;
    c.mult(0, V0, V1, V1);
    loop {
        c.cycles += 2;
        let not_square = c.g[V0][0] != c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        if not_square {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let keep_counting = c.g[V0][0] == 0u64;
            c.mult(0, V0, V1, V1);
            if keep_counting {
                if event_due(c) {
                    deliver_event_here(c, ISQRT_LOOP);
                }
                continue;
            }
            // n < i*i: the root is i - 2.
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
            ret(c);
            return;
        }
        // i*i == n: the root is i - 1.
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        ret(c);
        return;
    }
}

/// Entry of `SND_NextPrimeDelay` (0x3cfb98).
const NEXT_PRIME_DELAY: u32 = 0x003c_fb98;
/// Head of the outer loop, after the entry block (0x3cfbd0): the resume
/// point of its event checks.
const NEXT_PRIME_OUTER: u32 = 0x003c_fbd0;
/// The divisor test (0x3cfbe8): the resume point of the inner loop's check.
const NEXT_PRIME_TEST: u32 = 0x003c_fbe8;
const NEXT_PRIME_END: u32 = 0x003c_fc30;

#[derive(Clone, Copy, PartialEq, Debug)]
enum NextPrimeStage {
    /// `lim = isqrt(n) + 1`, then the divisor loop's first test.
    Outer,
    /// Loop-continue test of the outer loop (0x3cfbf4).
    Check,
    /// `n % divisor`; a zero remainder bumps n (0x3cfbfc, 0x3cfc08).
    Divide,
    /// `divisor == lim` (0x3cfbe8), then the divisor step.
    Test,
    /// `n += 1` and restart the outer loop (0x3cfc14).
    Bump,
    /// Return `n` (0x3cfc1c).
    Exit,
}

/// `int SND_NextPrimeDelay(int duration, int rate)` (0x3cfb98,
/// `SNDI_findprime`): the first prime at or above the number of samples
/// `duration * rate / 1000` (and at least 3). The 32-bit product wraps as
/// the original's `mult` does.
///
/// A `match pc` dispatcher: the sample-count search calls `SND_ISqrt`, and
/// the loop checks events at 0x3cfbd0 and 0x3cfbe8, which are its entry
/// points. Loop state lives in s0 (n), v1 (divisor) and a0 (lim).
pub fn snd_next_prime_delay(c: &mut Cpu) {
    let mut stage = match c.pc {
        NEXT_PRIME_DELAY => np_entry(c),
        NEXT_PRIME_OUTER => NextPrimeStage::Outer,
        NEXT_PRIME_TEST => NextPrimeStage::Test,
        pc => {
            c.jump_out(pc, NEXT_PRIME_DELAY, NEXT_PRIME_END);
            return;
        }
    };
    loop {
        let flow = match stage {
            NextPrimeStage::Outer => np_outer(c),
            NextPrimeStage::Check => np_check(c),
            NextPrimeStage::Divide => np_divide(c),
            NextPrimeStage::Test => np_test(c),
            NextPrimeStage::Bump => np_bump(c),
            NextPrimeStage::Exit => np_exit(c),
        };
        match flow {
            Resume::Go(next) => stage = next,
            // A callee's unwind (`Done`) or an event's (`Unwind`) returns
            // with `pc` where it was left.
            Resume::Done | Resume::Unwind => return,
        }
    }
}

/// The entry block: `n = duration * rate / 1000`, at least 3 (0x3cfb98 and
/// 0x3cfbc0). Leaves a1 = 3, as the original does.
fn np_entry(c: &mut Cpu) -> NextPrimeStage {
    c.cycles += 10;
    c.mult(0, A0, A1, A0);
    c.g[V0][0] = 0x3e8;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.g[V1][0] = 0x2;
    c.sq(S0, sp.wrapping_add(0x10));
    c.g[A1][0] = 0x3;
    c.sd(sp, c.g[RA][0]);
    c.div(0, A0, V0);

    c.cycles += 4;
    c.g[A0][0] = c.lo[0];
    c.g[S0][0] = c.g[A0][0];
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        c.g[S0][0] = c.g[A1][0];
    }
    NextPrimeStage::Outer
}

/// Outer loop head (0x3cfbd0): `lim = SND_ISqrt(n) + 1`, `divisor = 2`.
fn np_outer(c: &mut Cpu) -> Resume<NextPrimeStage> {
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    let callee = Callee {
        addr: 0x003c_fb68,
        f: ee::s14::f_003cfb68,
    };
    // Return address: the `SND_ISqrt` call site (0x3cfbd8).
    if callee.call(c, 0x003c_fbd8).is_err() {
        // The callee unwound: return with its `pc`.
        return Resume::Done;
    }
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[V1][0] = 0x2;
    c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
    Resume::Go(NextPrimeStage::Check)
}

/// `if (lim < divisor)` (0x3cfbf4): the outer loop repeats, else the divisor
/// test runs.
fn np_check(c: &mut Cpu) -> Resume<NextPrimeStage> {
    c.cycles += 2;
    if c.g[V0][0] != 0u64 {
        if back_edge_unwinds(c, NEXT_PRIME_OUTER) {
            return Resume::Unwind;
        }
        return Resume::Go(NextPrimeStage::Outer);
    }
    Resume::Go(NextPrimeStage::Divide)
}

/// `n % divisor` (0x3cfbfc, 0x3cfc08): a non-zero remainder goes on to the
/// `divisor == lim` test, a zero one bumps n.
fn np_divide(c: &mut Cpu) -> Resume<NextPrimeStage> {
    c.cycles += 3;
    c.div(0, S0, V1);
    c.cycles += 3;
    c.g[V0][0] = c.hi[0];
    if c.g[V0][0] != 0u64 {
        if back_edge_unwinds(c, NEXT_PRIME_TEST) {
            return Resume::Unwind;
        }
        return Resume::Go(NextPrimeStage::Test);
    }
    Resume::Go(NextPrimeStage::Bump)
}

/// `if (divisor == lim) return n;` then `divisor++` and the loop test
/// `lim >= divisor` (0x3cfbe8, 0x3cfbf0). The loop test leaves its result
/// in v0 for `np_check`.
fn np_test(c: &mut Cpu) -> Resume<NextPrimeStage> {
    c.cycles += 2;
    let found = c.g[V1][0] == c.g[A0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    if found {
        return Resume::Go(NextPrimeStage::Exit);
    }
    c.cycles += 1;
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    Resume::Go(NextPrimeStage::Check)
}

/// `n += 1`, then the outer loop again (0x3cfc14).
fn np_bump(c: &mut Cpu) -> Resume<NextPrimeStage> {
    c.cycles += 2;
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    if back_edge_unwinds(c, NEXT_PRIME_OUTER) {
        return Resume::Unwind;
    }
    Resume::Go(NextPrimeStage::Outer)
}

/// Return `n` in v0 and restore s0 and ra (0x3cfc1c).
fn np_exit(c: &mut Cpu) -> Resume<NextPrimeStage> {
    c.cycles += 5;
    c.g[V0][0] = c.g[S0][0];
    let sp = c.g[SP][0] as u32;
    c.g[RA][0] = c.ld(sp);
    c.lq(S0, sp.wrapping_add(0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp.wrapping_add(0x20));
    c.pc = ra;
    Resume::Done
}

/// Offsets used by `libmpeg_ClearFrameFlags` (0x402bb0). `mpeg[16]` (the
/// MPEG system block) sits at +0x40 of the argument; the picture records
/// hang off it in six slots, and each record's flag word is at +0x28.
const CLEAR_FRAME_SYS: u32 = 0x40;
const CLEAR_FRAME_FLAGS: u32 = 0x28;

/// `int libmpeg_ClearFrameFlags(mpeg)` (0x402bb0, `sceMpegClearRefBuff`):
/// clears the flag word of each reference picture record that is present,
/// then returns 1. The six slots are read in the original's order.
pub fn libmpeg_clear_frame_flags(c: &mut Cpu) {
    c.cycles += 4;
    let mpeg = c.g[A0][0] as u32;
    let sys = Addr(mpeg).off(CLEAR_FRAME_SYS).lw(c);
    c.g[V1][0] = sys;
    clear_frame_slot(c, sys, 0x1b8);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1c8);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1d8);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1bc);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1cc);
    c.cycles += 3;
    clear_frame_slot(c, sys, 0x1dc);
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// One slot of `libmpeg_ClearFrameFlags`: `v0 = sys[slot]`, then clear the
/// flag word of that record if it is present.
fn clear_frame_slot(c: &mut Cpu, sys: u64, slot: u32) {
    let record = Addr(sys as u32).off(slot).lw(c);
    c.g[V0][0] = record;
    if record != 0u64 {
        Addr(record as u32).off(CLEAR_FRAME_FLAGS).sw(c, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn snd_isqrt_matches() {
        check_events(ee::s14::f_003cfb68__generated, snd_isqrt, 300, &|c, seed| {
            let mut r = Rng(0x534e_4453 ^ seed as u64);
            scramble(c, &mut r);
            let n: i32 = match seed % 6 {
                0 => (r.next_u32() % 1000) as i32,
                1 => {
                    let s = r.next_u32() % 46_000;
                    (s * s) as i32
                }
                2 => {
                    let s = r.next_u32() % 46_000;
                    (s * s + r.next_u32() % 3) as i32
                }
                3 => -((r.next_u32() % 100_000) as i32) - 1,
                // Below 46340^2: i*i stays positive in 32 bits, so the loop
                // ends (above that, i*i wraps negative and the generated
                // loop never sees n < i*i).
                4 => 2_147_000_000 - (r.next_u32() % 4) as i32,
                _ => (r.next_u32() >> 2) as i32,
            };
            c.g[A0][0] = sx(n as u32);
            schedule(c, &mut r, seed, 0x003c_fb68)
        });
    }

    driven!(gen_next_prime_delay, ssx3_ee::s14::f_003cfb98__generated);
    driven!(hand_next_prime_delay, snd_next_prime_delay);

    #[test]
    fn snd_next_prime_delay_matches() {
        check_events(gen_next_prime_delay, hand_next_prime_delay, 300, &|c, seed| {
            let mut r = Rng(0x4e50_4431 ^ seed as u64);
            scramble(c, &mut r);
            let rates = [22_050u32, 44_100, 48_000, 11_025, 8_000];
            let rate = rates[(r.next_u32() as usize) % rates.len()];
            let duration: i32 = match seed % 5 {
                0 => -((r.next_u32() % 500) as i32),
                1 => (r.next_u32() % 4) as i32,
                _ => (r.next_u32() % 20_000) as i32,
            };
            c.g[A0][0] = sx(duration as u32);
            c.g[A1][0] = sx(rate);
            schedule(c, &mut r, seed, NEXT_PRIME_DELAY)
        });
    }

    #[test]
    fn libmpeg_clear_frame_flags_matches() {
        check(
            ssx3_ee::s15::f_00402bb0__generated,
            libmpeg_clear_frame_flags,
            200,
            &|c, seed| {
                let mut r = Rng(0x4c4d_5045 ^ seed as u64);
                scramble(c, &mut r);
                let sys = DATA + 0x1000;
                c.bus.write32(DATA + CLEAR_FRAME_SYS, sys);
                for (i, slot) in [0x1b8u32, 0x1c8, 0x1d8, 0x1bc, 0x1cc, 0x1dc].iter().enumerate() {
                    let record = DATA + 0x2000 + (i as u32) * 0x100;
                    let present = seed % 7 != 0 && (seed >> i) & 1 == 1;
                    c.bus
                        .write32(sys + slot, if present { record } else { 0 });
                    c.bus.write32(record + CLEAR_FRAME_FLAGS, r.next_u32());
                }
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
