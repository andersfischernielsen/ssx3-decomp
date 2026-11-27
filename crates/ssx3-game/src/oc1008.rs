//! Staging module oc1008 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1008`.

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
    0x0024_6278,
    c_game_load_state_setup_mp_widget,
    "cGameLoadState_SetupMPWidget",
)];

/// `void cGameLoadState_SetupMPWidget(void* self, UIObject* obj)` (0x00246278):
/// multiplayer load-screen widget setup. If the object's name hash (`obj+0x38`)
/// is the "track peak" label, formats the track name into a 200-byte buffer
/// (format picked by `BENewRaceInt_GetTrackPeak`) and sets it as the widget's
/// wide string. Otherwise, depending on the game mode byte at `0x535c10`, if
/// the object is that mode's label, calls its activate method (`obj+8`
/// vtable `+0x4c`). Any other `pc` (a resume after an unwound callee) runs
/// the generated body.
pub fn c_game_load_state_setup_mp_widget(c: &mut Cpu) {
    if c.pc != 0x00246278 {
        return ee::s06::f_00246278__generated(c);
    }
    guest(c, |c| {
        'b_2464c8: {
            'b_2463e0: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
                c.g[RA][0] = sx(0x0024629c);
                c.g[S2][0] = c.g[A1][0];
                call_ptr(c, 0x0014dc80, 0x0024629c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002462a8);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002462a8)?;
                c.g[A0][0] = 0x480000;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                c.g[S3][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002462bc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45b8));
                call_ptr(c, 0x00317670, 0x002462bc)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = 0x530000;
                if t {
                    break 'b_2463e0;
                } else {
                    'b_2463d0: {
                        'b_246380: {
                            'b_246334: {
                                'b_246328: {
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x002462d4);
                                    c.g[A2][0] = 0xc8;
                                    call_ptr(c, 0x00416210, 0x002462d4)?;
                                    c.g[V1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x002462e4);
                                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2bd8));
                                    call_ptr(c, 0x00144c98, 0x002462e4)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_246328;
                                    } else {
                                        'b_24631c: {
                                            'b_24630c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_24630c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_24631c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_2463d0;
                                                        } else {
                                                            break 'b_24630c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_246380;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_2463d0;
                                                } else {
                                                    break 'b_24631c;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x480000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45a8));
                                        if t {
                                            break 'b_246334;
                                        } else {
                                            break 'b_246328;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x480000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4598));
                                break 'b_246334;
                            }
                            c.cycles += 19;
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x00246348);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x00317670, 0x00246348)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A1][0] = c.g[V0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00246358);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, t, 0x00246358)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246364);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00144bf0, 0x00246364)?;
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00246378);
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x002c27c0, 0x00246378)?;
                            let t = true;
                            c.g[A0][0] = c.g[S2][0];
                            if t {
                                break 'b_2463d0;
                            } else {
                                break 'b_246380;
                            }
                        }
                        c.cycles += 20;
                        c.g[A0][0] = 0x480000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4588));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x0024639c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x0024639c)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A1][0] = c.g[V0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002463ac);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, t, 0x002463ac)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x002463b8);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00144bf0, 0x002463b8)?;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[A3][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x002463cc);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x002c27c0, 0x002463cc)?;
                        c.g[A0][0] = c.g[S2][0];
                        break 'b_2463d0;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002463d8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0e90, 0x002463d8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        break 'b_2464c8;
                    } else {
                        break 'b_2463e0;
                    }
                }
            }
            'b_2464c4: {
                c.cycles += 4;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c10));
                c.g[V1][0] = (c.g[A0][0] < 0x7u64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    break 'b_2464c4;
                } else {
                    'b_246494: {
                        'b_246464: {
                            'b_246458: {
                                'b_246448: {
                                    'b_246438: {
                                        'b_246428: {
                                            'b_246418: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x24b0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x00246408 => {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x480000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x4578));
                                                        if t {
                                                            break 'b_246464;
                                                        } else {
                                                            break 'b_246418;
                                                        }
                                                    }
                                                    0x00246418 => {
                                                        break 'b_246418;
                                                    }
                                                    0x00246428 => {
                                                        break 'b_246428;
                                                    }
                                                    0x00246438 => {
                                                        break 'b_246438;
                                                    }
                                                    0x00246448 => {
                                                        break 'b_246448;
                                                    }
                                                    0x00246458 => {
                                                        break 'b_246458;
                                                    }
                                                    0x00246494 => {
                                                        break 'b_246494;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x00246278, 0x002464e4);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[A0][0] = 0x480000;
                                            c.g[S0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x4560));
                                            if t {
                                                break 'b_246464;
                                            } else {
                                                break 'b_246428;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = 0x480000;
                                        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4550));
                                        if t {
                                            break 'b_246464;
                                        } else {
                                            break 'b_246438;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x480000;
                                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4540));
                                    if t {
                                        break 'b_246464;
                                    } else {
                                        break 'b_246448;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = 0x480000;
                                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4578));
                                if t {
                                    break 'b_246464;
                                } else {
                                    break 'b_246458;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x480000;
                            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4530));
                            break 'b_246464;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0024646c);
                        call_ptr(c, 0x00317670, 0x0024646c)?;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                        if t {
                            break 'b_2464c8;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0024648c);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0024648c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                            if t {
                                break 'b_2464c8;
                            } else {
                                break 'b_246494;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = 0x480000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x002464a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4530));
                    call_ptr(c, 0x00317670, 0x002464a4)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                    if t {
                        break 'b_2464c8;
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002464c4);
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002464c4)?;
                        break 'b_2464c4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            break 'b_2464c8;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's RAM).
    pub(crate) const OC1008_CTRL: u32 = DATA + 0xe_1000;
    pub(crate) const OC1008_MAGIC: u32 = 0x1008_c0de;
    pub(crate) const OC1008_OBJ: u32 = DATA + 0x5_0000;
    pub(crate) const OC1008_OBJ2: u32 = DATA + 0x5_1000;
    pub(crate) const OC1008_WORLD: u32 = DATA + 0x6_0000;
    pub(crate) const OC1008_IFACE: u32 = DATA + 0x6_2000;
    pub(crate) const OC1008_IFACE_VT: u32 = DATA + 0x6_3000;
    /// The game-mode byte read by the second half.
    pub(crate) const OC1008_MODE: u32 = 0x0053_5c10;

    /// Callee fake: logs the call; `StrHash` (0x317670) answers its argument
    /// (so `obj+0x38` equal to the string address matches), `GetTrackPeak`
    /// (0x144c98) answers the value in the mailbox at `CTRL+0x10`.
    fn oc1008_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if addr == 0x0031_7670 {
            c.g[V0][0] = a0;
        } else if addr == 0x0014_4c98 {
            let peak = c.bus.read32(OC1008_CTRL + 0x10);
            c.g[V0][0] = sx(peak);
        }
    }

    macro_rules! oc1008_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1008_CTRL) != OC1008_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1008_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1008_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1008_fakes!(
        (
            oc1008_fake_00144bf0,
            OC1008_PREV_00144bf0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x00144bf0
        ),
        (
            oc1008_fake_00144c98,
            OC1008_PREV_00144c98,
            ee::s01::OVERRIDE_00144C98,
            ee::s01::f_00144c98__generated,
            0x00144c98
        ),
        (
            oc1008_fake_0014dc80,
            OC1008_PREV_0014dc80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc1008_fake_0014dd58,
            OC1008_PREV_0014dd58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc1008_fake_002c27c0,
            OC1008_PREV_002c27c0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c27c0
        ),
        (
            oc1008_fake_00317670,
            OC1008_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc1008_fake_003a0e90,
            OC1008_PREV_003a0e90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc1008_fake_00416210,
            OC1008_PREV_00416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x00416210
        ),
    );

    /// The load-state widget fixture: the object whose name hash is one of
    /// the label strings (or none), a track-peak answer, and a mode byte.
    fn setup_mp_widget_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x246278, 8);
        elf_image(c);
        oc1008_install_fakes();
        c.bus.write32(OC1008_CTRL, OC1008_MAGIC);
        // Track-peak result: 0, 1, 2, 3, -1, 5.
        let peak: [i32; 6] = [0, 1, 2, 3, -1, 5];
        c.bus
            .write32(OC1008_CTRL + 0x10, peak[(seed / 7 % 6) as usize] as u32);
        // Mode byte 0..6, 7 and 0x80 (outside the table).
        let mode = [0u8, 1, 2, 3, 4, 5, 6, 7, 0x80][(seed / 3 % 9) as usize];
        c.bus.write8(OC1008_MODE, mode);
        // The label this mode looks for, per the jump table.
        let label = [
            0x0047_ba88u32,
            0x0047_baa0,
            0x0047_bab0,
            0x0047_bac0,
            0x0047_ba88,
            0x0047_bad0,
            0x0047_bad0,
        ];
        let key = match seed % 5 {
            0 | 1 => 0x0047_ba48,
            2 | 3 => label.get(mode as usize).copied().unwrap_or(0x0047_ba88),
            _ => r.next_u32() | 1,
        };
        c.bus.write32(OC1008_OBJ + 0x38, key);
        c.bus.write32(OC1008_OBJ + 8, OC1008_OBJ2);
        c.bus
            .write16(OC1008_OBJ2 + 0x48, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(OC1008_OBJ2 + 0x4c, fakes::FAKE);
        // The world and the interface object whose vtable it calls through.
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC1008_WORLD);
        c.bus.write32(OC1008_WORLD + 0x8c, OC1008_IFACE);
        c.bus.write32(OC1008_IFACE + 4, OC1008_IFACE_VT);
        c.bus
            .write16(OC1008_IFACE_VT + 0x20, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(OC1008_IFACE_VT + 0x24, fakes::FAKE);
        c.g[A0][0] = DATA as u64 + 0x7_0000;
        c.g[A1][0] = OC1008_OBJ as u64;
        c.pc = 0x0024_6278;
    }

    /// cGameLoadState_SetupMPWidget against the generated body, with every
    /// callee faked: the peak-label arm (all `GetTrackPeak` answers) and the
    /// per-mode label arms, matching and not, some seeds unwinding in a call.
    #[test]
    fn game_load_state_setup_mp_widget_matches() {
        check(
            ee::s06::f_00246278__generated,
            c_game_load_state_setup_mp_widget,
            180,
            &setup_mp_widget_setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..180 {
            let mut c = crate::difftest::fresh();
            setup_mp_widget_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s06::f_00246278__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 8,
            "fixture reaches too few paths: {}",
            seen.len()
        );
    }
}
