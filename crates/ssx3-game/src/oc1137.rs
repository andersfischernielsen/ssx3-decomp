//! Staging module oc1137 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1137`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_5800, path_fade_aux_send, "PATH_FadeAuxSend"),
    (0x003d_5968, path_update_fade_stops, "PATH_UpdateFadeStops"),
    (
        0x0040_7438,
        libmpeg_next_start_code,
        "libmpeg_NextStartCode",
    ),
];

const OC1137_SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::PATH_SelectMap,
};
const OC1137_TRACK_SET_AUX_SEND_FADE: Callee = Callee {
    addr: 0x003d_6360,
    f: ee::s14::PATH_TrackSetAuxSendFade,
};
const OC1137_IPU_WAIT_BUSY: Callee = Callee {
    addr: 0x0040_6e20,
    f: ee::s15::libmpeg_IpuWaitBusy,
};
const OC1137_FLUSH_BITS: Callee = Callee {
    addr: 0x0040_71f8,
    f: ee::s15::libmpeg_FlushBits,
};
const OC1137_SHOW_BITS: Callee = Callee {
    addr: 0x0040_70f0,
    f: ee::s15::libmpeg_ShowBits,
};

/// `PATH_FadeAuxSend(mask, sendTarget, b)` (0x3d5800): when the path system
/// is active and not already fading (`0x44f42c` set, `0x44f430` clear) and
/// `PATH_SelectMap(-1)` agrees, walks the four players (stride 0x928 at
/// `0x517610`; `+0x34` active, `+0x30` the player mask). For every player
/// whose mask hits `mask` in both of its nibbles (`0xf0000000`,
/// `0x0f000000`) and for which `PATH_SelectMap(i)` is non-zero, each of the
/// 24 path voices of the current list (`0x44f420`, entries at `+0x58`) whose
/// bit is set in `mask` gets `PATH_TrackSetAuxSendFade(voice, sendTarget, b,
/// list->f18)`. Ends by clearing `0x44f430` and `0x44f420`.
pub fn path_fade_aux_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, 17, 0x80);
        frame.save(c, 19, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 20, 0x50);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0xbd4));
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 16, 0x90);
        frame.save(c, 18, 0x70);
        frame.save(c, 21, 0x40);
        frame.save(c, 22, 0x30);
        frame.save(c, 23, 0x20);
        frame.save(c, 30, 0x10);
        let t = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        // `out`: leave with s0 already restored (the delay-slot `lq`).
        'out: {
            // `early`: leave through the extra cycle and the s0 restore.
            'early: {
                if t {
                    break 'early;
                }
                c.cycles += 4;
                c.g[V1][0] = 0x0045_0000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xbd0));
                let t = c.g[V0][0] != 0;
                frame.restore(c, 16, 0x90);
                if t {
                    break 'out;
                }
                c.cycles += 4;
                c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                OC1137_SELECT_MAP.call(c, 0x003d_585c)?;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x0051_0000;
                if t {
                    break 'early;
                }
                c.cycles += 7;
                c.g[A0][0] = 0;
                c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
                c.g[S5][0] = 0x0045_0000;
                c.g[FP][0] = 0xffff_ffff_f000_0000;
                c.g[S7][0] = 0x0f00_0000;
                c.g[V0][0] = 0x928;
                // Players.
                loop {
                    'next: {
                        c.cycles += 5;
                        c.mult(0, 5, 4, 2);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S6][0] as u32));
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        let t = c.g[V1][0] == 0;
                        c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        if !t {
                            c.cycles += 8;
                            c.g[V0][0] = 0x0051_0000;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7640));
                            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = c.g[V1][0] & c.g[FP][0];
                            c.g[V0][0] &= c.g[S1][0];
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[V1][0] & c.g[S7][0];
                            if !t {
                                c.cycles += 3;
                                c.g[V0][0] &= c.g[S1][0];
                                if c.g[V0][0] == 0 {
                                    c.g[A0][0] = c.g[S2][0];
                                    break 'next;
                                }
                                c.cycles += 4;
                                OC1137_SELECT_MAP.call(c, 0x003d_58c8)?;
                                let t = c.g[V0][0] == 0;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'next;
                                }
                                c.cycles += 2;
                                c.g[S0][0] = 0;
                                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0xbe0));
                                // The 24 voices of the list.
                                loop {
                                    c.cycles += 5;
                                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                                    let t = c.g[A0][0] == 0;
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32) >> ((c.g[S0][0] as u32) & 31));
                                    if t {
                                        c.cycles += 1;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] ^= 1;
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] == 0 {
                                            c.cycles += 4;
                                            c.g[A3][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[A2][0] = c.g[S3][0];
                                            OC1137_TRACK_SET_AUX_SEND_FADE.call(c, 0x003d_590c)?;
                                            c.cycles += 1;
                                        }
                                    }
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < 24) as u64;
                                    let t = c.g[V0][0] != 0;
                                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0xbe0));
                                    if !t {
                                        break;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003d_58d8);
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S2][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x928;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_5880);
                        }
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x0045_0000;
                    c.sw((c.g[S5][0] as u32).wrapping_sub(0xbe0), 0);
                    c.sw((c.g[V0][0] as u32).wrapping_sub(0xbd0), 0);
                    break 'early;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PATH_UpdateFadeStops(mask)` (0x3d5968): for each of the 24 voices of the
/// current path list (`0x44f420`, entries at `+0x58`) whose bit is set in
/// `mask` and whose `+0x36` is not negative, scales the node volume
/// (`+0x34`) and the fade target (`+0xe`) by the voice gain (`+4`) / 100. When
/// the fade target is reached (`PATH_PlatGetTrackVolume(+0x54)`): a zero
/// target stops the track (`PATH_PlatStopTrackSound`, `+0x36 = -1`), else the
/// node volume is set (`PATH_PlatSetVolume`); the fade target is then reset
/// to -1.
pub fn path_update_fade_stops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 19, 0x50);
        frame.save(c, 20, 0x40);
        c.g[S3][0] = 0;
        frame.save(c, 21, 0x30);
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, 22, 0x20);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 23, 0x10);
        c.g[S6][0] = 0x64;
        frame.save(c, 16, 0x80);
        c.g[S7][0] = 0x0045_0000;
        frame.save(c, 17, 0x70);
        frame.save(c, 18, 0x60);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
        loop {
            // `next`: s3 is already advanced; `done`: add the cycle and advance.
            'next: {
                'done: {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                    let t = c.g[S1][0] == 0;
                    c.g[V0][0] = sx((c.g[S5][0] as u32) >> ((c.g[S3][0] as u32) & 31));
                    if t {
                        break 'done;
                    }
                    c.cycles += 4;
                    c.g[V0][0] ^= 1;
                    c.g[V0][0] &= 1;
                    if c.g[V0][0] != 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                        break 'next;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x36));
                    if (c.g[V0][0] as i64) < 0 {
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                        break 'next;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[A1][0] = 0x64;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4));
                    'reset: {
                        'apply: {
                            c.cycles += 6;
                            c.g[S0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xe));
                            c.mult(0, 2, 2, 3);
                            c.div(0, 2, 5);
                            c.g[S2][0] = c.lo[0];
                            let t = (c.g[S0][0] as i64) < 0;
                            c.mult(0, 16, 16, 3);
                            if t {
                                break 'apply;
                            }
                            c.cycles += 2;
                            c.cycles += 6;
                            c.div(0, 16, 5);
                            c.g[S0][0] = c.lo[0];
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                            call_ptr(c, 0x002a_f428, 0x003d_5a1c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                                break 'next;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0xe));
                            if c.g[V0][0] != 0 {
                                break 'apply;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                            call_ptr(c, 0x002a_f838, 0x003d_5a38)?;
                            c.sh((c.g[S1][0] as u32).wrapping_add(0x36), c.g[S4][0]);
                            break 'reset;
                        }
                        c.cycles += 2;
                        if (c.g[S2][0] as i64) < 0 {
                            c.sb((c.g[S1][0] as u32).wrapping_add(0xe), c.g[S4][0]);
                            break 'done;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x002a_f6c0, 0x003d_5a54)?;
                    }
                    c.cycles += 1;
                    c.sb((c.g[S1][0] as u32).wrapping_add(0xe), c.g[S4][0]);
                }
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 24) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0xbe0));
            if event_due(c) {
                deliver_event_here(c, 0x003d_59a8);
            }
        }
        c.cycles += 11;
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `libmpeg_NextStartCode(sys)` (0x407438): byte-aligns the bit stream (the
/// IPU's bit position `0x10002020 & 7`, `FlushBits` by the missing bits),
/// then flushes bytes until `ShowBits(sys, 24)` is 1 (the `00 00 01` start
/// code prefix). Delivers a due event at the loop's back edge; a resume at
/// that head (`0x407480`) runs through the generated body.
pub fn libmpeg_next_start_code(c: &mut Cpu) {
    if c.pc != 0x0040_7438 {
        return ee::s15::f_00407438__generated(c);
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        let a = frame.at(c, 0);
        c.sd(a, c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.sd(a, c.g[S1][0]);
        OC1137_IPU_WAIT_BUSY.call(c, 0x0040_7450)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[V0][0] |= 0x2020;
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] &= 7;
        c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
        c.g[A1][0] = c.g[V1][0] & 7;
        let aligned = c.g[A1][0] == 0;
        c.g[S1][0] = 1;
        if !aligned {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            OC1137_FLUSH_BITS.call(c, 0x0040_7478)?;
            c.g[S1][0] = 1;
        }
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x18;
            OC1137_SHOW_BITS.call(c, 0x0040_7494)?;
            c.g[A0][0] = c.g[S0][0];
            let more = c.g[V0][0] != c.g[S1][0];
            c.g[A1][0] = 8;
            if !more {
                break;
            }
            if back_edge_unwinds(c, 0x0040_7480) {
                return Ok(());
            }
            c.cycles += 2;
            OC1137_FLUSH_BITS.call(c, 0x0040_7488)?;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        let a = frame.at(c, 0x10);
        c.g[S1][0] = c.ld(a);
        let a = frame.at(c, 0);
        c.g[S0][0] = c.ld(a);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check_lohi, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: they answer only while this word
    /// of the fixture's RAM holds `OC1137_MAGIC` (else the previous body runs).
    const OC1137_CTRL: u32 = DATA + 0x0002_0000;
    const OC1137_MAGIC: u32 = 0x1137_0271;
    /// One seeded word per fake call (indexed by the call number).
    const OC1137_RESULTS: u32 = DATA + 0x0002_0100;
    /// Path state in the game image: list, "fading" flag, "active" flag, players.
    const OC1137_LIST_PTR: u32 = 0x0044_f420;
    const OC1137_ACTIVE: u32 = 0x0044_f42c;
    const OC1137_FADING: u32 = 0x0044_f430;
    const OC1137_PLAYERS: u32 = 0x0051_7610;
    const OC1137_LIST: u32 = DATA + 0x1000;
    const OC1137_VOICES: u32 = DATA + 0x2000;

    macro_rules! oc1137_fake {
        ($name:ident, $prev:ident, $gen:path, $addr:expr, $set:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1137_CTRL) != OC1137_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let n = c.bus.read32(fakes::LOG_COUNT);
                let w = c.bus.read32(OC1137_RESULTS + 4 * (n % 64));
                fakes::fake_call(c, $addr);
                let set: fn(&mut Cpu, u32, u32) = $set;
                set(c, w, n);
            }
        };
    }

    // `PATH_SelectMap`: zero a quarter of the time.
    oc1137_fake!(
        oc1137_fake_select_map,
        OC1137_PREV_SELECT_MAP,
        ee::s14::f_003d5450__generated,
        0x003d_5450,
        |c, w, _n| c.g[V0][0] = (w % 4 != 0) as u64
    );
    oc1137_fake!(
        oc1137_fake_aux_send_fade,
        OC1137_PREV_AUX_SEND_FADE,
        ee::s14::f_003d6360__generated,
        0x003d_6360,
        |_c, _w, _n| {}
    );
    // `PATH_PlatGetTrackVolume`: equal to the caller's target (`s0`) a third
    // of the time, else something else.
    oc1137_fake!(
        oc1137_fake_get_track_volume,
        OC1137_PREV_GET_TRACK_VOLUME,
        ee::s08::f_002af428__generated,
        0x002a_f428,
        |c, w, _n| c.g[V0][0] = if w % 3 == 0 {
            c.g[S0][0]
        } else {
            (w | 1) as u64
        }
    );
    oc1137_fake!(
        oc1137_fake_set_volume,
        OC1137_PREV_SET_VOLUME,
        ee::s08::f_002af6c0__generated,
        0x002a_f6c0,
        |_c, _w, _n| {}
    );
    oc1137_fake!(
        oc1137_fake_stop_track_sound,
        OC1137_PREV_STOP_TRACK_SOUND,
        ee::s08::f_002af838__generated,
        0x002a_f838,
        |_c, _w, _n| {}
    );
    oc1137_fake!(
        oc1137_fake_ipu_wait_busy,
        OC1137_PREV_IPU_WAIT_BUSY,
        ee::s15::f_00406e20__generated,
        0x0040_6e20,
        |_c, _w, _n| {}
    );
    oc1137_fake!(
        oc1137_fake_flush_bits,
        OC1137_PREV_FLUSH_BITS,
        ee::s15::f_004071f8__generated,
        0x0040_71f8,
        |_c, _w, _n| {}
    );
    // `libmpeg_ShowBits`: the start-code prefix (1) a third of the time, and
    // always from the 20th call on, so the byte loop ends.
    oc1137_fake!(
        oc1137_fake_show_bits,
        OC1137_PREV_SHOW_BITS,
        ee::s15::f_004070f0__generated,
        0x0040_70f0,
        |c, w, n| c.g[V0][0] = if n >= 20 || w % 3 == 0 {
            1
        } else {
            0x00ab_cdef
        }
    );

    fn oc1137_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D5450),
            oc1137_fake_select_map,
            &OC1137_PREV_SELECT_MAP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D6360),
            oc1137_fake_aux_send_fade,
            &OC1137_PREV_AUX_SEND_FADE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AF428),
            oc1137_fake_get_track_volume,
            &OC1137_PREV_GET_TRACK_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AF6C0),
            oc1137_fake_set_volume,
            &OC1137_PREV_SET_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AF838),
            oc1137_fake_stop_track_sound,
            &OC1137_PREV_STOP_TRACK_SOUND,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00406E20),
            oc1137_fake_ipu_wait_busy,
            &OC1137_PREV_IPU_WAIT_BUSY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004071F8),
            oc1137_fake_flush_bits,
            &OC1137_PREV_FLUSH_BITS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004070F0),
            oc1137_fake_show_bits,
            &OC1137_PREV_SHOW_BITS,
        );
    }

    /// Fakes armed, seeded per-call words, `pc` at the entry.
    fn oc1137_setup(c: &mut Cpu, seed: u32, entry: u32, calls: u32) -> Rng {
        oc1137_install_fakes();
        let mut r = fakes::machine(c, seed, 0x1137_0000 ^ entry as u64, calls);
        c.bus.write32(OC1137_CTRL, OC1137_MAGIC);
        for k in 0..64 {
            let w = r.next_u32();
            c.bus.write32(OC1137_RESULTS + 4 * k, w);
        }
        c.pc = entry;
        r
    }

    /// The path voice list: 24 entries at `+0x58`, some null.
    fn oc1137_write_list(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC1137_LIST_PTR, OC1137_LIST);
        let w = r.next_u32();
        c.bus.write32(OC1137_LIST + 0x18, w);
        for j in 0..24 {
            let v = if r.next_u32() % 4 == 0 {
                0
            } else {
                OC1137_VOICES + 0x80 * j
            };
            c.bus.write32(OC1137_LIST + 0x58 + 4 * j, v);
        }
    }

    #[test]
    fn path_fade_aux_send_matches() {
        check_lohi(
            ee::s14::f_003d5800__generated,
            path_fade_aux_send,
            300,
            &|c, seed| {
                let mut r = oc1137_setup(c, seed, 0x003d_5800, 24);
                c.bus.write8(OC1137_ACTIVE, (seed % 6 != 0) as u8);
                c.bus.write32(OC1137_FADING, (seed % 7 == 0) as u32);
                oc1137_write_list(c, &mut r);
                for i in 0..4 {
                    let base = OC1137_PLAYERS + 0x928 * i;
                    let w = r.next_u32();
                    c.bus.write32(base + 0x30, w);
                    c.bus.write32(base + 0x34, (r.next_u32() % 4 != 0) as u32);
                }
                let mask = if seed % 9 == 0 { 0 } else { r.next_u32() };
                c.g[A0][0] = sx(mask);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn path_update_fade_stops_matches() {
        check_lohi(
            ee::s14::f_003d5968__generated,
            path_update_fade_stops,
            300,
            &|c, seed| {
                let mut r = oc1137_setup(c, seed, 0x003d_5968, 24);
                oc1137_write_list(c, &mut r);
                for j in 0..24 {
                    let v = OC1137_VOICES + 0x80 * j;
                    // Gain, node volume (signed bytes), fade target: -1, 0 or 1..100.
                    c.bus.write8(v + 4, (r.next_u32() % 101) as u8);
                    c.bus.write8(v + 0x34, (r.next_u32() % 140) as u8);
                    let target = match r.next_u32() % 4 {
                        0 => 0xff,
                        1 => 0,
                        _ => (r.next_u32() % 101) as u8,
                    };
                    c.bus.write8(v + 0xe, target);
                    let h = if r.next_u32() % 5 == 0 { 0xffff } else { 0 };
                    c.bus.write16(v + 0x36, h);
                    let w = r.next_u32();
                    c.bus.write32(v + 0x54, w);
                }
                let mask = r.next_u32();
                c.g[A0][0] = sx(mask);
            },
        );
    }

    /// The IPU bit position register reads 0 on the test platform, so the
    /// alignment flush is not taken here; the start-code loop is.
    #[test]
    fn libmpeg_next_start_code_matches() {
        check_lohi(
            ee::s15::f_00407438__generated,
            libmpeg_next_start_code,
            200,
            &|c, seed| {
                let _ = oc1137_setup(c, seed, 0x0040_7438, 8);
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
