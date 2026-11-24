//! Staging module oc550 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc550`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0026caf8,
    playback_man_load_playback,
    "PlaybackMan_LoadPlayback",
)];

/// `PlaybackMan_LoadPlayback` (0x0026caf8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn playback_man_load_playback(c: &mut Cpu) {
    if c.pc != 0x0026caf8 {
        return ee::s07::f_0026caf8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0026caf8;
        loop {
            match pc {
                0x0026caf8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        pc = 0x0026cb94;
                        continue;
                    }
                    pc = 0x0026cb1c;
                    continue;
                }
                0x0026cb1c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0026cb24);
                    Callee {
                        addr: 0x0026cfe8,
                        f: ee::s07::PlaybackMan_FindFile,
                    }
                    .call(c, 0x0026cb24)?;
                    c.g[S2][0] = c.g[V0][0];
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0026cb94;
                        continue;
                    }
                    pc = 0x0026cb30;
                    continue;
                }
                0x0026cb30 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0026cb38);
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0026c9f0,
                        f: ee::s07::PlaybackMan_StartFileLoad,
                    }
                    .call(c, 0x0026cb38)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0026cb94;
                        continue;
                    }
                    pc = 0x0026cb40;
                    continue;
                }
                0x0026cb40 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0026cb64;
                        continue;
                    }
                    pc = 0x0026cb48;
                    continue;
                }
                0x0026cb48 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0026cb50);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0026c5f8,
                        f: ee::s07::PlaybackMan_PollFileLoad,
                    }
                    .call(c, 0x0026cb50)?;
                    c.g[RA][0] = sx(0x0026cb58);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x003e5928, 0x0026cb58)?;
                    c.g[RA][0] = sx(0x0026cb60);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x003e52b0, 0x0026cb60)?;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x0026cb64;
                    continue;
                }
                0x0026cb64 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0026cb6c);
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0026c998,
                        f: ee::s07::PlaybackMan_IsFileLoaded,
                    }
                    .call(c, 0x0026cb6c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x0026cb48) {
                            return Ok(());
                        }
                        pc = 0x0026cb48;
                        continue;
                    }
                    pc = 0x0026cb74;
                    continue;
                }
                0x0026cb74 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0026cb7c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0026d020,
                        f: ee::s07::PlaybackMan_ApplyFile,
                    }
                    .call(c, 0x0026cb7c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0026cb98;
                        continue;
                    }
                    pc = 0x0026cb84;
                    continue;
                }
                0x0026cb84 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    let t = (c.g[V1][0] as i64) > 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x0026cb98;
                        continue;
                    }
                    pc = 0x0026cb94;
                    continue;
                }
                0x0026cb94 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0026cb98;
                    continue;
                }
                0x0026cb98 => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026caf8, 0x0026cbb0);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn playback_man_load_playback_matches() {
        check(
            ee::s07::f_0026caf8__generated,
            playback_man_load_playback,
            200,
            &|c, seed| {
                machine(c, seed, 0xcaf8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0026caf8;
            },
        );
    }
}
