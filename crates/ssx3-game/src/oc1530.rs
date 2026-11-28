//! Staging module oc1530 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1530`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_95b0,
        bank_monitor_push_default_params,
        "BankMonitor_PushDefaultParams",
    ),
    (
        0x002b_9ff8,
        audio_push_default_params,
        "Audio_PushDefaultParams",
    ),
    (0x002b_a208, audio_push_params_bank, "Audio_PushParamsBank"),
    (
        0x002b_a448,
        audio_push_params_bank_sound,
        "Audio_PushParamsBankSound",
    ),
    (0x002b_a6b0, audio_play_sound, "Audio_PlaySound"),
];

/// `memset` (called with the 8-byte per-slot parameter block).
const OC1530_MEMSET: u32 = 0x0041_6210;
/// The record stack index (`-1` when empty) inside the parameter bank `p`.
pub(crate) const OC1530_BANK_IDX: u32 = 0x1c;
/// Offset of the bank inside the audio object used by the Audio_* variants.
pub(crate) const OC1530_AUDIO_BANK: u32 = 0x1d8;

/// First half of every "push a default record" body, on the parameter bank
/// `p` (the register `preg` holds its address): bump the index, copy the
/// 24-byte current record (`p+4`) into `recs[idx]`, clear the per-slot
/// arrays (`+0x24`, `+0x34`), copy the record's signed first byte to
/// `vals[idx]` (`+0x38`), clear `+0x3c`, then `memset(p + idx*8 + 0x40, 0, 8)`
/// returning to `ret`. `T1` must hold `0x18` (the record size).
fn oc1530_push_head(c: &mut Cpu, preg: usize, ret: u32) -> Flow {
    let p = c.g[preg][0] as u32;
    let idx_at = p.wrapping_add(OC1530_BANK_IDX);
    let bumped = (c.lw(idx_at) as u32).wrapping_add(1);
    c.sw(idx_at, sx(bumped));
    let recs = c.lw(p.wrapping_add(0x20)) as u32;
    let idx = c.lw(idx_at) as u32;
    let rec = recs.wrapping_add(idx.wrapping_mul(0x18));
    // 24-byte copy `p+4 .. p+0x1c` -> recs[idx], three unaligned doublewords.
    let src = p.wrapping_add(4);
    for (reg, k) in [(V0, 0u32), (A0, 8), (A3, 0x10)] {
        c.ldl(reg, src.wrapping_add(k + 7));
        c.ldr(reg, src.wrapping_add(k));
    }
    for (reg, k) in [(V0, 0u32), (A0, 8), (A3, 0x10)] {
        c.sdl(reg, rec.wrapping_add(k + 7));
        c.sdr(reg, rec.wrapping_add(k));
    }
    // arr24[idx] = 0 (inline array at p+0x24)
    let i = c.lw(idx_at) as u32;
    c.sw(p.wrapping_add(0x24).wrapping_add(i << 2), 0);
    // p34[idx] = 0
    let i = c.lw(idx_at) as u32;
    let b34 = c.lw(p.wrapping_add(0x34)) as u32;
    c.sw(b34.wrapping_add(i << 2), 0);
    // vals[idx] = (i8) recs[idx].first_byte
    let vals_i = c.lw(idx_at) as u32;
    let rec_i = c.lw(idx_at) as u32;
    let recs = c.lw(p.wrapping_add(0x20)) as u32;
    c.g[A0][0] = sx(rec_i);
    c.mult(0, 4, 4, 9);
    let b38 = c.lw(p.wrapping_add(0x38)) as u32;
    let vals_at = (vals_i << 2).wrapping_add(b38);
    let rec_at = rec_i.wrapping_mul(0x18).wrapping_add(recs);
    let first = c.lb(rec_at);
    c.sw(vals_at, first);
    // p3C[idx] = 0
    let i = c.lw(idx_at) as u32;
    let b3c = c.lw(p.wrapping_add(0x3c)) as u32;
    let p3c_at = (i << 2).wrapping_add(b3c);
    c.sw(p3c_at, 0);
    // memset(p + idx*8 + 0x40, 0, 8)
    let i = c.lw(idx_at) as u32;
    let blk = p.wrapping_add(i << 3).wrapping_add(0x40);
    c.g[V0][0] = sx(p3c_at);
    c.g[V1][0] = sx(b3c);
    c.g[A0][0] = sx(blk);
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[A3][0] = sx(vals_at);
    c.g[T0][0] = sx(recs);
    c.g[T1][0] = 0x18;
    call_ptr(c, OC1530_MEMSET, ret)
}

/// Second half: the default values of the new record's per-slot arrays
/// (`+0x60..+0x7c`, `+0x80/+0x84`). Returns (base of the `+0x7c` array, the
/// address stored last), which the callers leave in `v0`/`v1`.
fn oc1530_push_defaults(c: &mut Cpu, preg: usize) -> (u32, u32) {
    let p = c.g[preg][0] as u32;
    let idx_at = p.wrapping_add(OC1530_BANK_IDX);
    // (array pointer offset, index shift, offset inside the entry, value)
    const STORES: [(u32, u32, u32, u32); 13] = [
        (0x60, 2, 0, 0),
        (0x64, 2, 0, 0),
        (0x68, 2, 0, 0),
        (0x6c, 2, 0, 0),
        (0x70, 4, 4, 0),
        (0x70, 4, 0, 0x64),
        (0x70, 4, 8, 0x5a),
        (0x70, 4, 0xc, 0x32),
        (0x74, 2, 0, 0),
        (0x80, 2, 0, 0),
        (0x84, 2, 0, 0),
        (0x78, 2, 0, 0x7f),
        (0x7c, 2, 0, 1),
    ];
    let mut last = (0, 0);
    for (off, shift, extra, v) in STORES {
        let i = c.lw(idx_at) as u32;
        let base = c.lw(p.wrapping_add(off)) as u32;
        let at = (i << shift).wrapping_add(base).wrapping_add(extra);
        c.sw(at, v as u64);
        last = (base, at);
    }
    last
}

/// `BankMonitor_PushDefaultParams(bank)` (0x2b95b0): push a default record
/// on the bank's parameter stack (the bank itself is the argument).
pub fn bank_monitor_push_default_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 129;
        let frame = Frame::open(c, 0x20);
        c.g[T1][0] = 0x18;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = 8;
        oc1530_push_head(c, S0, 0x002b_968c)?;
        let (base, at) = oc1530_push_defaults(c, S0);
        c.g[A0][0] = 0x64;
        c.g[A1][0] = 0x5a;
        c.g[A2][0] = 0x32;
        c.g[A3][0] = 0x7f;
        c.g[T0][0] = 1;
        c.g[V0][0] = sx(base);
        c.g[V1][0] = sx(at);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `Audio_PushDefaultParams(handle)` (0x2b9ff8): the same push on the
/// parameter bank embedded at `*handle + 0x1d8`.
pub fn audio_push_default_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 132;
        let frame = Frame::open(c, 0x30);
        c.g[T1][0] = 0x18;
        frame.save(c, S1, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x20);
        let obj = c.lw(c.g[A0][0] as u32);
        c.g[S0][0] = obj;
        c.g[S1][0] = sx((obj as u32).wrapping_add(OC1530_AUDIO_BANK));
        oc1530_push_head(c, S1, 0x002b_a0dc)?;
        let (base, at) = oc1530_push_defaults(c, S1);
        c.g[A0][0] = 0x64;
        c.g[A1][0] = 0x5a;
        c.g[A2][0] = 0x32;
        c.g[A3][0] = 0x7f;
        c.g[T0][0] = 1;
        c.g[V0][0] = sx(base);
        c.g[V1][0] = sx(at);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `Audio_PushParamsBank(handle, bank)` (0x2ba208): push a default record,
/// then store `bank` into the handle's bank-id array (`handle[1][idx]`).
pub fn audio_push_params_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 144;
        let frame = Frame::open(c, 0x50);
        c.g[T1][0] = 0x18;
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 8;
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[A1][0] = 0;
        let obj = c.lw(c.g[S2][0] as u32);
        c.g[S0][0] = obj;
        c.g[S1][0] = sx((obj as u32).wrapping_add(OC1530_AUDIO_BANK));
        oc1530_push_head(c, S1, 0x002b_a2fc)?;
        let (_, _) = oc1530_push_defaults(c, S1);
        c.g[A0][0] = 0x64;
        c.g[A1][0] = 0x5a;
        c.g[A2][0] = 0x32;
        c.g[A3][0] = 0x7f;
        c.g[T0][0] = 1;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0);
        let h = c.g[S2][0] as u32;
        let obj = c.lw(h);
        let arr = c.lw(h.wrapping_add(4));
        c.g[A0][0] = obj;
        c.g[V1][0] = arr;
        let idx = c.lw((obj as u32).wrapping_add(0x1f4));
        c.g[V0][0] = idx;
        frame.restore(c, S2, 0x20);
        let at = ((idx as u32) << 2).wrapping_add(arr as u32);
        c.g[V0][0] = sx(at);
        let v = c.g[S3][0];
        c.sw(at, v);
        frame.restore(c, S3, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `Audio_PushParamsBankSound(handle, bank, sound)` (0x2ba448): as
/// `Audio_PushParamsBank`, and also store `sound` into the handle's sound
/// array (`handle[2][idx]`).
pub fn audio_push_params_bank_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 153;
        let frame = Frame::open(c, 0x60);
        c.g[T1][0] = 0x18;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[A1][0] = 0;
        let obj = c.lw(c.g[S2][0] as u32);
        c.g[S0][0] = obj;
        c.g[A2][0] = 8;
        c.g[S1][0] = sx((obj as u32).wrapping_add(OC1530_AUDIO_BANK));
        oc1530_push_head(c, S1, 0x002b_a544)?;
        let (_, _) = oc1530_push_defaults(c, S1);
        c.g[A0][0] = 0x64;
        c.g[A1][0] = 0x5a;
        c.g[A2][0] = 0x32;
        c.g[A3][0] = 0x7f;
        c.g[T0][0] = 1;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        let h = c.g[S2][0] as u32;
        frame.restore_ra(c, 0);
        let obj = c.lw(h);
        let arr = c.lw(h.wrapping_add(4));
        c.g[A0][0] = obj;
        c.g[V1][0] = arr;
        let idx = c.lw((obj as u32).wrapping_add(0x1f4));
        c.g[V0][0] = idx;
        let at = ((idx as u32) << 2).wrapping_add(arr as u32);
        c.g[V0][0] = sx(at);
        let v = c.g[S3][0];
        c.sw(at, v);
        frame.restore(c, S3, 0x20);
        let obj = c.lw(h);
        let arr2 = c.lw(h.wrapping_add(8));
        c.g[V1][0] = obj;
        c.g[A0][0] = arr2;
        let idx = c.lw((obj as u32).wrapping_add(0x1f4));
        c.g[V0][0] = idx;
        frame.restore(c, S2, 0x30);
        let at = ((idx as u32) << 2).wrapping_add(arr2 as u32);
        c.g[V0][0] = sx(at);
        let v = c.g[S4][0];
        c.sw(at, v);
        frame.restore(c, S4, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `Audio_PlaySound(handle, bank, sound)` (0x2ba6b0): push the record with
/// both ids as above, then call `0x2906b8(handle, ...)` to start it.
pub fn audio_play_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 155;
        let frame = Frame::open(c, 0x60);
        c.g[T1][0] = 0x18;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[A1][0] = 0;
        let obj = c.lw(c.g[S2][0] as u32);
        c.g[S0][0] = obj;
        c.g[A2][0] = 8;
        c.g[S1][0] = sx((obj as u32).wrapping_add(OC1530_AUDIO_BANK));
        oc1530_push_head(c, S1, 0x002b_a7ac)?;
        let (_, _) = oc1530_push_defaults(c, S1);
        c.g[A1][0] = 0x64;
        c.g[A2][0] = 0x5a;
        c.g[A3][0] = 0x32;
        c.g[T0][0] = 0x7f;
        c.g[T1][0] = 1;
        c.g[A0][0] = c.g[S2][0];
        let h = c.g[S2][0] as u32;
        let obj = c.lw(h);
        let arr = c.lw(h.wrapping_add(4));
        c.g[A1][0] = obj;
        c.g[V1][0] = arr;
        let idx = c.lw((obj as u32).wrapping_add(0x1f4));
        c.g[V0][0] = idx;
        let at = ((idx as u32) << 2).wrapping_add(arr as u32);
        c.g[V0][0] = sx(at);
        let v = c.g[S3][0];
        c.sw(at, v);
        let obj = c.lw(h);
        let arr2 = c.lw(h.wrapping_add(8));
        c.g[V1][0] = obj;
        c.g[A1][0] = arr2;
        let idx = c.lw((obj as u32).wrapping_add(0x1f4));
        c.g[V0][0] = idx;
        let at = ((idx as u32) << 2).wrapping_add(arr2 as u32);
        c.g[V0][0] = sx(at);
        let v = c.g[S4][0];
        c.sw(at, v);
        call_ptr(c, 0x002906b8, 0x002b_a8fc)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{fill, Rng, DATA};

    /// A parameter bank `p`: random contents, the index (`-1`, 0, 1 or 2
    /// before the push), the record array and the per-slot arrays behind the
    /// bank's pointer table (`+0x34..+0x84`), all in `arrays..arrays+0xe00`.
    fn oc1530_bank(c: &mut Cpu, r: &mut Rng, seed: u32, p: u32, arrays: u32) {
        fill(c, r, p, 0x90 / 4);
        fill(c, r, arrays, 0xe00 / 4);
        let before = [0xffff_ffffu32, 0, 1, 2][(seed % 4) as usize];
        c.bus.write32(p + OC1530_BANK_IDX, before);
        c.bus.write32(p + 0x20, arrays);
        for (off, a) in [
            (0x34u32, 0x100u32),
            (0x38, 0x200),
            (0x3c, 0x300),
            (0x60, 0x400),
            (0x64, 0x500),
            (0x68, 0x600),
            (0x6c, 0x700),
            (0x70, 0x800),
            (0x74, 0x900),
            (0x78, 0xa00),
            (0x7c, 0xb00),
            (0x80, 0xc00),
            (0x84, 0xd00),
        ] {
            c.bus.write32(p + off, arrays + a);
        }
    }

    /// An audio handle at `DATA+0x3000` -> object at `DATA` whose bank sits
    /// at `+0x1d8`; the handle's id arrays are at `DATA+0x3100/0x3200`.
    fn oc1530_audio(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let obj = DATA;
        let h = DATA + 0x3000;
        fill(c, r, obj, 0x1d8 / 4);
        oc1530_bank(c, r, seed, obj + OC1530_AUDIO_BANK, DATA + 0x1000);
        fill(c, r, h, 0x300 / 4);
        c.bus.write32(h, obj);
        c.bus.write32(h + 4, DATA + 0x3100);
        c.bus.write32(h + 8, DATA + 0x3200);
        c.g[A0][0] = sx(h);
        c.g[A1][0] = sx(r.next_u32());
        c.g[A2][0] = sx(r.next_u32());
    }

    /// Pushes a default record on the bank given directly as the argument.
    #[test]
    fn bank_monitor_push_default_params_matches() {
        crate::difftest::check(
            ee::s08::f_002b95b0__generated,
            bank_monitor_push_default_params,
            256,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1530_0001, 1);
                oc1530_bank(c, &mut r, seed, DATA, DATA + 0x1000);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// The same push through an audio handle's embedded bank.
    #[test]
    fn audio_push_default_params_matches() {
        crate::difftest::check(
            ee::s08::f_002b9ff8__generated,
            audio_push_default_params,
            256,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1530_0002, 1);
                oc1530_audio(c, &mut r, seed);
            },
        );
    }

    /// Push plus the bank-id store.
    #[test]
    fn audio_push_params_bank_matches() {
        crate::difftest::check(
            ee::s08::f_002ba208__generated,
            audio_push_params_bank,
            256,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1530_0003, 1);
                oc1530_audio(c, &mut r, seed);
            },
        );
    }

    /// Push plus the bank-id and sound-id stores.
    #[test]
    fn audio_push_params_bank_sound_matches() {
        crate::difftest::check(
            ee::s08::f_002ba448__generated,
            audio_push_params_bank_sound,
            256,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1530_0004, 1);
                oc1530_audio(c, &mut r, seed);
            },
        );
    }

    /// Push, both id stores, then the start call (two fake calls).
    #[test]
    fn audio_play_sound_matches() {
        fakes::covered(
            ee::s08::f_002ba6b0__generated,
            audio_play_sound,
            256,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1530_0005, 2);
                oc1530_audio(c, &mut r, seed);
            },
        );
    }
}
