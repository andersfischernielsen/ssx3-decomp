//! Staging module oc831 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc831`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Cheat-character codes: (return address of the hash call, hash, character
/// type in `a3`, loop head). Each grants the character in ten slots.
const OC831_CHEAT_CHARS: [(u32, u32, u32, u32); 16] = [
    (0x00187dc0, 0x7e68d, 0xb, 0x00187dd8),
    (0x00187e0c, 0x699649f, 0xd, 0x00187e20),
    (0x00187e54, 0x80c4f45, 0xf, 0x00187e68),
    (0x00187e9c, 0xfa26044, 0x11, 0x00187eb0),
    (0x00187ee4, 0x539ab42, 0xa, 0x00187ef8),
    (0x00187f2c, 0x61c8197, 0x12, 0x00187f40),
    (0x00187f74, 0x64694e5, 0x13, 0x00187f88),
    (0x00187fbc, 0x3f22e2b, 0x14, 0x00187fd0),
    (0x00188004, 0xcd28689, 0x15, 0x00188018),
    (0x0018804c, 0xb582d1c, 0x16, 0x00188060),
    (0x00188094, 0x1cb90e5, 0x17, 0x001880a8),
    (0x001880dc, 0x715e0e4, 0x18, 0x001880f0),
    (0x00188124, 0xf3223e, 0x19, 0x00188138),
    (0x0018816c, 0xeafc69e, 0x1b, 0x00188180),
    (0x001881b4, 0xdd36d88, 0x1c, 0x001881c8),
    (0x001881fc, 0xde3475, 0x1a, 0x00188210),
];

/// "Buy everything of a kind" codes: (return address of the hash call, hash,
/// count getter, its return address, cycles before `s0 = 0`, outer loop head,
/// inner loop head, buy function, its return address).
const OC831_BUY_ALL: [(u32, u32, u32, u32, u64, u32, u32, u32, u32); 5] = [
    (
        0x00188380, 0x5b65653, 0x1572b0, 0x00188398, 1, 0x1883a8, 0x1883b0, 0x157fd0, 0x001883c8,
    ),
    (
        0x001883f4, 0xa0cf7c4, 0x1572d0, 0x0018840c, 2, 0x188420, 0x188428, 0x1580f8, 0x00188440,
    ),
    (
        0x0018846c, 0xb5d8914, 0x1572f0, 0x00188484, 2, 0x188498, 0x1884a0, 0x158220, 0x001884b8,
    ),
    (
        0x001884e4, 0x717257d, 0x157330, 0x001884fc, 2, 0x188510, 0x188518, 0x158470, 0x00188530,
    ),
    (
        0x0018855c, 0x74f6cf4, 0x157310, 0x00188574, 2, 0x188588, 0x188590, 0x158348, 0x001885a8,
    ),
];

/// Compare the hash in `v0` with `hash` (`v1`), as the original does.
#[inline(always)]
fn oc831_hash_differs(c: &mut Cpu, hash: u32) -> bool {
    c.g[V1][0] = hash as u64;
    c.g[V0][0] != c.g[V1][0]
}

/// `cFEStateOptions_ApplyCheatCode(name)`: hashes the typed code and, on a
/// match, unlocks the matching rewards (cheat characters, Peak Pass levels,
/// songs, posters, trading cards, concept art, videos, toys, all-tricks
/// items). Returns 1 when a code matched (0 if the last code check
/// failed).
pub fn cfe_state_options_apply_cheat_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x130);
        frame.save(c, 16, 0x120);
        frame.save(c, 17, 0x110);
        frame.save(c, 19, 0xf0);
        frame.save(c, 18, 0x100);
        frame.save(c, 20, 0xe0);
        frame.save(c, 21, 0xd0);
        frame.save(c, 22, 0xc0);
        frame.save(c, 23, 0xb0);
        frame.save(c, 30, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_7d6c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x0018_7d78)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_7d80)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0018_7d8c)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_7d94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x0014_dd58, 0x0018_7da0)?;
        let a0 = frame.at(c, 0);
        c.g[A0][0] = a0 as u64;
        let a = frame.at(c, 0x80);
        c.sw(a, c.g[V0][0]);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0041_66f4, 0x0018_7db0)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x002c_2868, 0x0018_7db8)?;
        c.g[A0][0] = c.g[SP][0];
        oc831_scan(c)?;
        c.cycles += 12;
        frame.restore(c, 16, 0x120);
        frame.restore(c, 17, 0x110);
        frame.restore(c, 18, 0x100);
        frame.restore(c, 19, 0xf0);
        frame.restore(c, 20, 0xe0);
        frame.restore(c, 21, 0xd0);
        frame.restore(c, 22, 0xc0);
        frame.restore(c, 23, 0xb0);
        frame.restore(c, 30, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// The code table of `cFEStateOptions_ApplyCheatCode`: hash the code once
/// per entry (`0x317670`) and run the first handler whose hash matches.
/// Leaves `v0` as the function returns it.
fn oc831_scan(c: &mut Cpu) -> Flow {
    // Cheat characters (`0x158618` ten times), the first entry's hash call
    // is made in the prologue's last block.
    for (i, &(hash_ra, hash, kind, head)) in OC831_CHEAT_CHARS.iter().enumerate() {
        if i != 0 {
            c.cycles += 6;
            c.g[A0][0] = c.g[SP][0];
        }
        call_ptr(c, 0x0031_7670, hash_ra)?;
        let differs = oc831_hash_differs(c, hash);
        c.g[S0][0] = 0;
        if differs {
            continue;
        }
        c.cycles += if i == 0 { 2 } else { 1 };
        c.g[A2][0] = c.g[S0][0];
        loop {
            c.cycles += 9;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = kind as u64;
            c.g[T0][0] = 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0015_8618, head + 0x18)?;
            c.g[V1][0] = ((c.g[S0][0] as i64) < 10) as u64;
            c.g[A2][0] = c.g[S0][0];
            if c.g[V1][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, head);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(());
        }
    }

    // Peak Pass level 1 (0x150b88 ten times).
    c.cycles += 6;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x0018_8244)?;
    let differs = oc831_hash_differs(c, 0x82d_e683);
    c.g[S0][0] = 0;
    if !differs {
        c.cycles += 1;
        c.g[A2][0] = c.g[S0][0];
        loop {
            c.cycles += 9;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = 1;
            c.g[T0][0] = 1;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0015_0b88, 0x0018_8270)?;
            c.g[V1][0] = ((c.g[S0][0] as i64) < 10) as u64;
            c.g[A2][0] = c.g[S0][0];
            if c.g[V1][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0018_8258);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(());
        }
    }

    // Peak Pass levels 1 and 2 (two 0x150b88 calls per round).
    c.cycles += 6;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x0018_828c)?;
    let differs = oc831_hash_differs(c, 0x4b1_ff67);
    c.g[S0][0] = 0;
    if !differs {
        c.cycles += 1;
        c.g[A2][0] = c.g[S0][0];
        loop {
            c.cycles += 15;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = 1;
            c.g[T0][0] = 1;
            call_ptr(c, 0x0015_0b88, 0x0018_82b4)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A3][0] = 2;
            c.g[T0][0] = 1;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            call_ptr(c, 0x0015_0b88, 0x0018_82d0)?;
            c.g[V1][0] = ((c.g[S0][0] as i64) < 10) as u64;
            c.g[A2][0] = c.g[S0][0];
            if c.g[V1][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0018_82a0);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(());
        }
    }

    // Songs: buy every song of every playlist slot, then set the playlist.
    c.cycles += 6;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x0018_82ec)?;
    let differs = oc831_hash_differs(c, 0x61f_9f02);
    c.g[A0][0] = c.g[S3][0];
    if !differs {
        c.cycles += 5;
        c.g[S2][0] = 0;
        call_ptr(c, 0x0015_7350, 0x0018_8304)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[S1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[S0][0] = 0;
            let empty = (c.g[S4][0] as i64) <= 0;
            c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            if !empty {
                c.cycles += 1;
                c.g[S5][0] = 1;
                loop {
                    c.cycles += 11;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[T0][0] = 0;
                    call_ptr(c, 0x0015_8558, 0x0018_8338)?;
                    c.g[V1][0] = c.g[S5][0] << (c.g[S0][0] & 63);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    c.g[S1][0] |= c.g[V1][0];
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018_8320);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 9;
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0015_8848, 0x0018_8360)?;
            c.g[S2][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[S2][0] as i64) < 10) as u64;
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, 0x0018_8310);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(());
        }
    }

    // Posters, trading cards, concept art, videos, toys: every item of the
    // kind, ten variants each.
    for &(hash_ra, hash, count_fn, count_ra, extra, outer, inner, buy_fn, buy_ra) in
        OC831_BUY_ALL.iter()
    {
        c.cycles += 6;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0031_7670, hash_ra)?;
        let differs = oc831_hash_differs(c, hash);
        c.g[A0][0] = c.g[S3][0];
        if differs {
            continue;
        }
        c.cycles += 5;
        c.g[S1][0] = 0;
        call_ptr(c, count_fn, count_ra)?;
        c.g[S4][0] = c.g[V0][0];
        let empty = (c.g[S4][0] as i64) <= 0;
        c.g[V0][0] = 1;
        if empty {
            return Ok(());
        }
        c.cycles += extra;
        c.g[S0][0] = 0;
        loop {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[A2][0] = c.g[S0][0];
            loop {
                c.cycles += 9;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = 0;
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                call_ptr(c, buy_fn, buy_ra)?;
                c.g[V1][0] = ((c.g[S0][0] as i64) < 10) as u64;
                c.g[A2][0] = c.g[S0][0];
                if c.g[V1][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, inner);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 4;
            c.g[S1][0] = c.g[S2][0];
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S4][0] as i64)) as u64;
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                if event_due(c) {
                    deliver_event_here(c, outer);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(());
        }
    }

    // Unlock all items of tier 0..2 (reward kind in s7).
    c.cycles += 6;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x0018_85d4)?;
    let differs = oc831_hash_differs(c, 0xbcf_3974);
    c.g[S7][0] = 0;
    if !differs {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 2;
            c.g[S2][0] = 0;
            let a = sp(c).wrapping_add(0x84);
            c.sw(a, c.g[V0][0]);
            loop {
                'scan: {
                    c.cycles += 17;
                    c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x0014_bdb8, 0x0018_85f8)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0014_d998, 0x0018_8608)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_d988, 0x0018_8618)?;
                    let a = sp(c).wrapping_add(0x80);
                    c.g[A0][0] = c.lw(a);
                    c.g[A2][0] = c.g[S2][0];
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S7][0];
                    call_ptr(c, 0x0014_ad28, 0x0018_862c)?;
                    let none = (c.g[S0][0] as i64) <= 0;
                    c.g[S2][0] = c.g[V0][0];
                    if none {
                        break 'scan;
                    }
                    c.cycles += 3;
                    c.g[S6][0] = 1;
                    c.g[S5][0] = 2;
                    c.g[S4][0] = 3;
                    loop {
                        'entry: {
                            'entry_buy: {
                                'entry_kind: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(2));
                                    let a = (c.g[GP][0] as u32).wrapping_sub(0x1f10);
                                    let is1 = c.g[V0][0] == c.g[S6][0];
                                    c.g[A2][0] = c.lw(a);
                                    if is1 {
                                        break 'entry_kind;
                                    }
                                    c.cycles += 2;
                                    let is2 = c.g[V0][0] == c.g[S5][0];
                                    c.g[A0][0] = c.g[S3][0];
                                    if is2 {
                                        break 'entry_buy;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != c.g[S4][0] {
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                                        break 'entry;
                                    }
                                    c.cycles += 1;
                                    c.g[A2][0] = c.lw(a);
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S3][0];
                            }
                            'owned: {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x0014_d908, 0x0018_866c)?;
                                let no = c.g[V0][0] == 0;
                                c.g[A0][0] = c.g[S2][0];
                                if no {
                                    break 'owned;
                                }
                                c.cycles += 3;
                                c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(4));
                                c.g[A2][0] = 1;
                                call_ptr(c, 0x0015_1c48, 0x0018_8680)?;
                            }
                            c.cycles += 1;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                        }
                        c.cycles += 2;
                        let more = c.g[S0][0] != 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018_8640);
                            }
                            continue;
                        }
                        break 'scan;
                    }
                }
                c.cycles += 4;
                c.g[S2][0] = c.g[FP][0];
                c.g[V0][0] = ((c.g[S2][0] as i64) < 10) as u64;
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_85f0);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 4;
            let a = sp(c).wrapping_add(0x84);
            c.g[S7][0] = c.lw(a);
            let again = (c.g[S7][0] as i64) < 3;
            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(1));
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0018_85e8);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            return Ok(());
        }
    }

    // Unlock all items of kind 1 for each tier.
    c.cycles += 6;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, 0x0031_7670, 0x0018_86bc)?;
    let differs = oc831_hash_differs(c, 0x706_94d5);
    c.g[V0][0] = 0;
    if differs {
        return Ok(());
    }
    c.cycles += 3;
    c.g[S5][0] = 0;
    c.g[S2][0] = 0;
    loop {
        c.cycles += 2;
        c.g[S6][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
        loop {
            'scan: {
                c.cycles += 17;
                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0014_bdb8, 0x0018_86e8)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_d998, 0x0018_86f4)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_bdb8, 0x0018_86fc)?;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_d988, 0x0018_8708)?;
                let a = sp(c).wrapping_add(0x80);
                c.g[A0][0] = c.lw(a);
                c.g[A2][0] = c.g[S2][0];
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0014_ad28, 0x0018_871c)?;
                let none = (c.g[S0][0] as i64) <= 0;
                c.g[S2][0] = c.g[V0][0];
                if none {
                    break 'scan;
                }
                c.cycles += 1;
                c.g[S3][0] = 1;
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(2));
                    if c.g[V0][0] != c.g[S3][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(4));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0015_1c48, 0x0018_8744)?;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                    }
                    c.cycles += 2;
                    let more = c.g[S0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x38));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018_8728);
                        }
                        continue;
                    }
                    break 'scan;
                }
            }
            c.cycles += 4;
            c.g[S2][0] = c.g[S4][0];
            c.g[V0][0] = ((c.g[S2][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 3) as u64;
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0018_86e0);
                }
                continue;
            }
            break;
        }
        c.cycles += 3;
        c.g[S5][0] = c.g[S6][0];
        let again = c.g[V0][0] != 0;
        c.g[S2][0] = 0;
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x0018_86d8);
            }
            continue;
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0018_7d38,
    cfe_state_options_apply_cheat_code,
    "cFEStateOptions_ApplyCheatCode",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (the fakes fall back to the real
    /// callees when it is absent, so other tests are unaffected).
    const OC831_CTRL: u32 = DATA + 0x50_0000;
    const OC831_MAGIC: u32 = 0x4f43_8331;
    /// Per-callee forced results: `OC831_MAIL + (addr & 0xfffc)`.
    const OC831_MAIL: u32 = DATA + 0x51_0000;
    /// Reward table entries (0x38 bytes each; byte +2 is the kind, half +4 the id).
    const OC831_ENTRIES: u32 = DATA + 0x54_0000;

    fn oc831_mail(addr: u32) -> u32 {
        OC831_MAIL + (addr & 0xfffc)
    }

    /// Force the fake for `addr` to return `v` (stored +3; 1 means 0).
    fn oc831_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc831_mail(addr), v.wrapping_add(3));
    }

    fn oc831_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc831_mail(addr), 1);
    }

    /// The fake callee: logs like `fakes::fake_call`, then returns the forced
    /// result unless it unwound.
    fn oc831_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc831_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = if mail == 1 { 0 } else { (mail - 3) as u64 };
        }
    }

    macro_rules! oc831_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC831_CTRL) != OC831_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc831_fake_call(c, $addr);
                }
            )*
            fn oc831_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc831_fakes!(
        (
            oc831_fake_14dc80,
            OC831_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc831_fake_14dd58,
            OC831_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc831_fake_4166f4,
            OC831_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x004166f4
        ),
        (
            oc831_fake_2c2868,
            OC831_PREV_2C2868,
            ee::s08::OVERRIDE_002C2868,
            ee::s08::f_002c2868__generated,
            0x002c2868
        ),
        (
            oc831_fake_317670,
            OC831_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc831_fake_158618,
            OC831_PREV_158618,
            ee::s01::OVERRIDE_00158618,
            ee::s01::f_00158618__generated,
            0x00158618
        ),
        (
            oc831_fake_150b88,
            OC831_PREV_150B88,
            ee::s01::OVERRIDE_00150B88,
            ee::s01::f_00150b88__generated,
            0x00150b88
        ),
        (
            oc831_fake_158558,
            OC831_PREV_158558,
            ee::s01::OVERRIDE_00158558,
            ee::s01::f_00158558__generated,
            0x00158558
        ),
        (
            oc831_fake_158848,
            OC831_PREV_158848,
            ee::s01::OVERRIDE_00158848,
            ee::s01::f_00158848__generated,
            0x00158848
        ),
        (
            oc831_fake_157350,
            OC831_PREV_157350,
            ee::s01::OVERRIDE_00157350,
            ee::s01::f_00157350__generated,
            0x00157350
        ),
        (
            oc831_fake_1572b0,
            OC831_PREV_1572B0,
            ee::s01::OVERRIDE_001572B0,
            ee::s01::f_001572b0__generated,
            0x001572b0
        ),
        (
            oc831_fake_1572d0,
            OC831_PREV_1572D0,
            ee::s01::OVERRIDE_001572D0,
            ee::s01::f_001572d0__generated,
            0x001572d0
        ),
        (
            oc831_fake_1572f0,
            OC831_PREV_1572F0,
            ee::s01::OVERRIDE_001572F0,
            ee::s01::f_001572f0__generated,
            0x001572f0
        ),
        (
            oc831_fake_157330,
            OC831_PREV_157330,
            ee::s01::OVERRIDE_00157330,
            ee::s01::f_00157330__generated,
            0x00157330
        ),
        (
            oc831_fake_157310,
            OC831_PREV_157310,
            ee::s01::OVERRIDE_00157310,
            ee::s01::f_00157310__generated,
            0x00157310
        ),
        (
            oc831_fake_157fd0,
            OC831_PREV_157FD0,
            ee::s01::OVERRIDE_00157FD0,
            ee::s01::f_00157fd0__generated,
            0x00157fd0
        ),
        (
            oc831_fake_1580f8,
            OC831_PREV_1580F8,
            ee::s01::OVERRIDE_001580F8,
            ee::s01::f_001580f8__generated,
            0x001580f8
        ),
        (
            oc831_fake_158220,
            OC831_PREV_158220,
            ee::s01::OVERRIDE_00158220,
            ee::s01::f_00158220__generated,
            0x00158220
        ),
        (
            oc831_fake_158470,
            OC831_PREV_158470,
            ee::s01::OVERRIDE_00158470,
            ee::s01::f_00158470__generated,
            0x00158470
        ),
        (
            oc831_fake_158348,
            OC831_PREV_158348,
            ee::s01::OVERRIDE_00158348,
            ee::s01::f_00158348__generated,
            0x00158348
        ),
        (
            oc831_fake_14bdb8,
            OC831_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014bdb8
        ),
        (
            oc831_fake_14d998,
            OC831_PREV_14D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014d998
        ),
        (
            oc831_fake_14d988,
            OC831_PREV_14D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014d988
        ),
        (
            oc831_fake_14ad28,
            OC831_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014ad28
        ),
        (
            oc831_fake_14d908,
            OC831_PREV_14D908,
            ee::s01::OVERRIDE_0014D908,
            ee::s01::f_0014d908__generated,
            0x0014d908
        ),
        (
            oc831_fake_151c48,
            OC831_PREV_151C48,
            ee::s01::OVERRIDE_00151C48,
            ee::s01::f_00151c48__generated,
            0x00151c48
        ),
    );

    /// Every code hash the function knows, in table order, then one that matches nothing.
    fn oc831_hashes() -> Vec<u32> {
        let mut v: Vec<u32> = OC831_CHEAT_CHARS.iter().map(|e| e.1).collect();
        v.extend([0x82d_e683, 0x4b1_ff67, 0x61f_9f02]);
        v.extend(OC831_BUY_ALL.iter().map(|e| e.1));
        v.extend([0xbcf_3974, 0x706_94d5, 0x1234_5678]);
        v
    }

    /// `cFEStateOptions_ApplyCheatCode`: one seed per code (and a miss), with
    /// 0..3 items per kind, 0..4 reward entries of every kind, owned or not.
    #[test]
    fn cfe_state_options_apply_cheat_code_matches() {
        oc831_install_fakes();
        let hashes = oc831_hashes();
        let n = hashes.len() as u32;
        check(
            ee::s02::f_00187d38__generated,
            cfe_state_options_apply_cheat_code,
            n * 8,
            &|c, seed| {
                machine(c, seed, 0x187d38, 70);
                c.bus.write32(OC831_CTRL, OC831_MAGIC);
                let v = seed / n;
                oc831_force(c, 0x0031_7670, hashes[(seed % n) as usize]);
                let count = v % 4;
                for a in [
                    0x15_7350, 0x15_72b0, 0x15_72d0, 0x15_72f0, 0x15_7330, 0x15_7310,
                ] {
                    if count == 0 {
                        oc831_force_zero(c, a);
                    } else {
                        oc831_force(c, a, count);
                    }
                }
                let entries = [0, 1, 3, 4][(v % 4) as usize];
                if entries == 0 {
                    oc831_force_zero(c, 0x14_d988);
                } else {
                    oc831_force(c, 0x14_d988, entries);
                }
                oc831_force(c, 0x14_d998, OC831_ENTRIES);
                if (v / 4) % 2 == 0 {
                    oc831_force_zero(c, 0x14_d908);
                } else {
                    oc831_force(c, 0x14_d908, 1);
                }
                for k in 0..4u32 {
                    let e = OC831_ENTRIES + 0x38 * k;
                    c.bus.write8(e + 2, ((seed + 3 * v + k) % 5) as u8);
                    c.bus.write16(e + 4, 0x100 + seed as u16 + k as u16);
                }
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.pc = 0x0018_7d38;
            },
        );
    }
}
