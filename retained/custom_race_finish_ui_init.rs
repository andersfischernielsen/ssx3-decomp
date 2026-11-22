//! Native Race finish-overlay initialization0x1e7bb0, retained uninstalled.
//!
//! The base overlay creates the finishov screen, then the native timer is
//! initialized to0. The local player's time/place select six position icons;
//! career/score status selects the original message and record widgets.
//! Widget lookups used by the native time/place paths are assumed to succeed.
//! No automated tests, build, replay or runtime installation is performed.

use crate::call::{Frame, Unwound, call_ptr, guest};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
const ENTRY: u32 = 0x001e_7bb0;
const END: u32 = 0x001e_8094;
const SCREEN: u32 = 0x40;
const FINISH_CLOCK: u32 = 0x9c;

const STR_HASH: u32 = 0x00317670;
const CREATE_SCREEN: u32 = 0x00397cc0;
const START_THREAD: u32 = 0x0039c870;
const OVERLAY_INIT: u32 = 0x0020a380;
const FIND_WIDGET: u32 = 0x0039d860;
const BE_GET: u32 = 0x0014dc80;
const BE_INTERFACE: u32 = 0x0014dd58;
const GET_HUMAN_RIDER: u32 = 0x00146e98;
const HIDE_WIDGET: u32 = 0x0039e8b8;
const SHOW_WIDGET: u32 = 0x0039e948;
const FLOAT_TO_UNSIGNED: u32 = 0x004139f8;
const FORMAT_TIME: u32 = 0x0020a1a8;
const SET_TEXT: u32 = 0x003a0c50;
const SPRINTF: u32 = 0x00417828;
const GET_AUDIO: u32 = 0x0028b180;
const PLAY_FE_SOUND: u32 = 0x00294f78;

/// `void OVScreenFinishOv_Init(OVScreenFinishOv* self)` (0x1e7bb0).
/// Populate the native Race finish overlay. Preserve each native frame spill,
/// block charge, UI lookup, unsigned time conversion and message branch.
/// Native call-return PCs retain the open frame across a suspended call.
pub fn ov_screen_finish_ov_init(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xd0 };
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                // Create the native finishov screen; only its optional start thread is guarded.
                0x001e7bb0 => {
                    c.cycles += 23;
                    Frame::open(c, 0xd0);
                    frame.save(c, S2, 0xa0);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S1, 0xb0);
                    frame.save(c, S3, 0x90);
                    c.g[A0][0] = 0x470000;
                    frame.save(c, S4, 0x80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                    frame.save(c, S5, 0x70);
                    frame.save(c, S6, 0x60);
                    frame.save(c, S7, 0x50);
                    frame.save(c, FP, 0x40);
                    frame.save_ra(c, 0x30);
                    frame.save(c, S0, 0xc0);
                    c.g[RA][0] = sx(0x001e7bf0);
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, STR_HASH, 0x001e7bf0)?;
                    pc = 0x001e7bf0;
                    continue;
                }
                0x001e7bf0 => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7c04);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, CREATE_SCREEN, 0x001e7c04)?;
                    pc = 0x001e7c04;
                    continue;
                }
                0x001e7c04 => {
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(SCREEN), c.g[V0][0]);
                    if t {
                        pc = 0x001e7c1c;
                        continue;
                    }
                    pc = 0x001e7c0c;
                    continue;
                }
                0x001e7c0c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001e7c1c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, START_THREAD, 0x001e7c1c)?;
                    pc = 0x001e7c1c;
                    continue;
                }
                0x001e7c1c => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x001e7c24);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, OVERLAY_INIT, 0x001e7c24)?;
                    pc = 0x001e7c24;
                    continue;
                }
                // Timer reset happens after the base overlay initializer returns.
                0x001e7c24 => {
                    // A completed base initialization always initializes the finish clock.
                    c.sw((c.g[S2][0] as u32).wrapping_add(FINISH_CLOCK), 0u64);
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001e7c34);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17e8));
                    call_ptr(c, STR_HASH, 0x001e7c34)?;
                    pc = 0x001e7c34;
                    continue;
                }
                0x001e7c34 => {
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SCREEN));
                    c.g[RA][0] = sx(0x001e7c40);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e7c40)?;
                    pc = 0x001e7c40;
                    continue;
                }
                0x001e7c40 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x001e7c5c;
                        continue;
                    }
                    pc = 0x001e7c4c;
                    continue;
                }
                0x001e7c4c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] | 0x8;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x001e7c5c;
                    continue;
                }
                0x001e7c5c => {
                    c.cycles += 60;
                    c.g[RA][0] = sx(0x001e7c64);
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                    call_ptr(c, BE_GET, 0x001e7c64)?;
                    pc = 0x001e7c64;
                    continue;
                }
                0x001e7c64 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e7c70);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, BE_INTERFACE, 0x001e7c70)?;
                    pc = 0x001e7c70;
                    continue;
                }
                0x001e7c70 => {
                    c.g[RA][0] = sx(0x001e7c78);

                    call_ptr(c, BE_GET, 0x001e7c78)?;
                    pc = 0x001e7c78;
                    continue;
                }
                0x001e7c78 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e7c84);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, BE_INTERFACE, 0x001e7c84)?;
                    pc = 0x001e7c84;
                    continue;
                }
                0x001e7c84 => {
                    c.g[RA][0] = sx(0x001e7c8c);
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, BE_GET, 0x001e7c8c)?;
                    pc = 0x001e7c8c;
                    continue;
                }
                0x001e7c8c => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e7c98);
                    c.g[A1][0] = 0x8;
                    call_ptr(c, BE_INTERFACE, 0x001e7c98)?;
                    pc = 0x001e7c98;
                    continue;
                }
                0x001e7c98 => {
                    c.g[RA][0] = sx(0x001e7ca0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    call_ptr(c, BE_GET, 0x001e7ca0)?;
                    pc = 0x001e7ca0;
                    continue;
                }
                0x001e7ca0 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e7cac);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, BE_INTERFACE, 0x001e7cac)?;
                    pc = 0x001e7cac;
                    continue;
                }
                0x001e7cac => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001e7cbc);
                    c.g[S5][0] = c.g[V0][0];
                    call_ptr(c, GET_HUMAN_RIDER, 0x001e7cbc)?;
                    pc = 0x001e7cbc;
                    continue;
                }
                // Keep the human rider index, current heat and published time/place arrays.
                0x001e7cbc => {
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A1][0] = 0x470000;
                    c.g[V0][0] = 0x530000;
                    c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6730));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17d8));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S3][0] = c.lw((c.g[S1][0] as u32));
                    c.g[S6][0] = c.lw((c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x001e7ce8);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, HIDE_WIDGET, 0x001e7ce8)?;
                    pc = 0x001e7ce8;
                    continue;
                }
                0x001e7ce8 => {
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7cf8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17c8));
                    call_ptr(c, HIDE_WIDGET, 0x001e7cf8)?;
                    pc = 0x001e7cf8;
                    continue;
                }
                0x001e7cf8 => {
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7d08);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17b8));
                    call_ptr(c, HIDE_WIDGET, 0x001e7d08)?;
                    pc = 0x001e7d08;
                    continue;
                }
                0x001e7d08 => {
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7d18);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17a8));
                    call_ptr(c, HIDE_WIDGET, 0x001e7d18)?;
                    pc = 0x001e7d18;
                    continue;
                }
                0x001e7d18 => {
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001e7d24);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1798));
                    call_ptr(c, STR_HASH, 0x001e7d24)?;
                    pc = 0x001e7d24;
                    continue;
                }
                0x001e7d24 => {
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SCREEN));
                    c.g[RA][0] = sx(0x001e7d30);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e7d30)?;
                    pc = 0x001e7d30;
                    continue;
                }
                0x001e7d30 => {
                    c.g[V1][0] = 0x530000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    if t {
                        pc = 0x001e7d5c;
                        continue;
                    }
                    pc = 0x001e7d4c;
                    continue;
                }
                0x001e7d4c => {
                    c.cycles += 4;
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    let t = true;

                    if t {
                        pc = 0x001e7d70;
                        continue;
                    }
                    pc = 0x001e7d5c;
                    continue;
                }
                0x001e7d5c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    c.add_s(0, 0, 0);
                    pc = 0x001e7d70;
                    continue;
                }
                0x001e7d70 => {
                    c.cycles += 16;
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5790));
                    c.g[S4][0] = sx((c.g[S4][0] as u32) << 2);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[S4][0]);
                    c.g[S0][0] = 0x1;
                    c.mul_s(12, 0, 12);
                    c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    c.g[S7][0] = 0x4a0000;
                    c.g[S6][0] = 0x530000;
                    c.g[RA][0] = sx(0x001e7d98);
                    c.g[FP][0] = 0x470000;
                    call_ptr(c, FLOAT_TO_UNSIGNED, 0x001e7d98)?;
                    pc = 0x001e7d98;
                    continue;
                }
                0x001e7d98 => {
                    c.g[RA][0] = sx(0x001e7da0);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, FORMAT_TIME, 0x001e7da0)?;
                    pc = 0x001e7da0;
                    continue;
                }
                0x001e7da0 => {
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001e7dac);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, SET_TEXT, 0x001e7dac)?;
                    pc = 0x001e7dac;
                    continue;
                }
                0x001e7dac => {
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x001e7db0;
                    continue;
                }
                // Six required place%d widgets, selected by the one-based final placement.
                0x001e7db0 => {
                    c.cycles += 17;
                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2198));
                    c.g[RA][0] = sx(0x001e7dbc);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, SPRINTF, 0x001e7dbc)?;
                    pc = 0x001e7dbc;
                    continue;
                }
                0x001e7dbc => {
                    c.g[RA][0] = sx(0x001e7dc4);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, STR_HASH, 0x001e7dc4)?;
                    pc = 0x001e7dc4;
                    continue;
                }
                0x001e7dc4 => {
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SCREEN));
                    c.g[RA][0] = sx(0x001e7dd0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e7dd0)?;
                    pc = 0x001e7dd0;
                    continue;
                }
                0x001e7dd0 => {
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e7dec);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e7dec)?;
                    pc = 0x001e7dec;
                    continue;
                }
                0x001e7dec => {
                    if c.g[S0][0] != c.g[S4][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        pc = 0x001e7e10;
                        continue;
                    }
                    pc = 0x001e7df4;
                    continue;
                }
                0x001e7df4 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e7e0c);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e7e0c)?;
                    pc = 0x001e7e0c;
                    continue;
                }
                0x001e7e0c => {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    pc = 0x001e7e10;
                    continue;
                }
                // Career rank and heat/event kind select the original medal messages.
                0x001e7e10 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 7i64) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.g[SP][0];
                        if back_edge_unwinds(c, 0x001e_7db0) {
                            return Err(Unwound);
                        }
                        pc = 0x001e7db0;
                        continue;
                    }
                    pc = 0x001e7e1c;
                    continue;
                }
                0x001e7e1c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1860));
                    if t {
                        pc = 0x001e7e60;
                        continue;
                    }
                    pc = 0x001e7e28;
                    continue;
                }
                0x001e7e28 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    if t {
                        pc = 0x001e7e44;
                        continue;
                    }
                    pc = 0x001e7e34;
                    continue;
                }
                0x001e7e34 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));

                    if c.g[V1][0] != c.g[A0][0] {
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), 0u64);
                        pc = 0x001e7e64;
                        continue;
                    }
                    pc = 0x001e7e44;
                    continue;
                }
                0x001e7e44 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x1;
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1860));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), c.g[V0][0]);
                    c.g[RA][0] = sx(0x001e7e58);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, SHOW_WIDGET, 0x001e7e58)?;
                    pc = 0x001e7e58;
                    continue;
                }
                0x001e7e58 => {
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    if t {
                        pc = 0x001e7e70;
                        continue;
                    }
                    pc = 0x001e7e60;
                    continue;
                }
                0x001e7e60 => {
                    c.cycles += 1;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xfb4), 0u64);
                    pc = 0x001e7e64;
                    continue;
                }
                0x001e7e64 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001e7e6c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, HIDE_WIDGET, 0x001e7e6c)?;
                    pc = 0x001e7e6c;
                    continue;
                }
                0x001e7e6c => {
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    pc = 0x001e7e70;
                    continue;
                }
                0x001e7e70 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = 0x4a0000;
                    if t {
                        pc = 0x001e7eb0;
                        continue;
                    }
                    pc = 0x001e7e7c;
                    continue;
                }
                0x001e7e7c => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    if t {
                        pc = 0x001e7e98;
                        continue;
                    }
                    pc = 0x001e7e88;
                    continue;
                }
                0x001e7e88 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V1][0] = 0x4a0000;
                    if t {
                        pc = 0x001e7eb0;
                        continue;
                    }
                    pc = 0x001e7e98;
                    continue;
                }
                0x001e7e98 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7ea8);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2180));
                    call_ptr(c, SHOW_WIDGET, 0x001e7ea8)?;
                    pc = 0x001e7ea8;
                    continue;
                }
                0x001e7ea8 => {
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    if t {
                        pc = 0x001e7ec0;
                        continue;
                    }
                    pc = 0x001e7eb0;
                    continue;
                }
                0x001e7eb0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7ebc);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2180));
                    call_ptr(c, HIDE_WIDGET, 0x001e7ebc)?;
                    pc = 0x001e7ebc;
                    continue;
                }
                0x001e7ebc => {
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    pc = 0x001e7ec0;
                    continue;
                }
                0x001e7ec0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x001e7f00;
                        continue;
                    }
                    pc = 0x001e7ecc;
                    continue;
                }
                0x001e7ecc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[S3][0] == c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    if t {
                        pc = 0x001e7ee8;
                        continue;
                    }
                    pc = 0x001e7ed8;
                    continue;
                }
                0x001e7ed8 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x001e7f00;
                        continue;
                    }
                    pc = 0x001e7ee8;
                    continue;
                }
                0x001e7ee8 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2188));
                    c.g[RA][0] = sx(0x001e7ef8);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, SHOW_WIDGET, 0x001e7ef8)?;
                    pc = 0x001e7ef8;
                    continue;
                }
                0x001e7ef8 => {
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    if t {
                        pc = 0x001e7f10;
                        continue;
                    }
                    pc = 0x001e7f00;
                    continue;
                }
                0x001e7f00 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7f0c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2188));
                    call_ptr(c, HIDE_WIDGET, 0x001e7f0c)?;
                    pc = 0x001e7f0c;
                    continue;
                }
                0x001e7f0c => {
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    pc = 0x001e7f10;
                    continue;
                }
                0x001e7f10 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001e7f4c;
                        continue;
                    }
                    pc = 0x001e7f1c;
                    continue;
                }
                0x001e7f1c => {
                    c.cycles += 2;
                    let t = c.g[S3][0] == c.g[V1][0];
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    if t {
                        pc = 0x001e7f34;
                        continue;
                    }
                    pc = 0x001e7f24;
                    continue;
                }
                0x001e7f24 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x4;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));

                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001e7f4c;
                        continue;
                    }
                    pc = 0x001e7f34;
                    continue;
                }
                0x001e7f34 => {
                    c.cycles += 6;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e7f44);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2190));
                    call_ptr(c, SHOW_WIDGET, 0x001e7f44)?;
                    pc = 0x001e7f44;
                    continue;
                }
                0x001e7f44 => {
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x001e7f5c;
                        continue;
                    }
                    pc = 0x001e7f4c;
                    continue;
                }
                0x001e7f4c => {
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2190));
                    c.g[RA][0] = sx(0x001e7f58);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, HIDE_WIDGET, 0x001e7f58)?;
                    pc = 0x001e7f58;
                    continue;
                }
                0x001e7f58 => {
                    c.g[A0][0] = 0x470000;
                    pc = 0x001e7f5c;
                    continue;
                }
                0x001e7f5c => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x001e7f64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1788));
                    call_ptr(c, STR_HASH, 0x001e7f64)?;
                    pc = 0x001e7f64;
                    continue;
                }
                0x001e7f64 => {
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(SCREEN));
                    c.g[RA][0] = sx(0x001e7f70);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e7f70)?;
                    pc = 0x001e7f70;
                    continue;
                }
                // Native record fields decide whether new_record is visible.
                0x001e7f70 => {
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x001e7f94;
                        continue;
                    }
                    pc = 0x001e7f84;
                    continue;
                }
                0x001e7f84 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = (c.g[V0][0] as i64) < 0;

                    if t {
                        pc = 0x001e7f98;
                        continue;
                    }
                    pc = 0x001e7f94;
                    continue;
                }
                0x001e7f94 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    pc = 0x001e7f98;
                    continue;
                }
                0x001e7f98 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x001e7fbc;
                        continue;
                    }
                    pc = 0x001e7fa0;
                    continue;
                }
                0x001e7fa0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e7fb4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e7fb4)?;
                    pc = 0x001e7fb4;
                    continue;
                }
                0x001e7fb4 => {
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x001e7fd8;
                        continue;
                    }
                    pc = 0x001e7fbc;
                    continue;
                }
                0x001e7fbc => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e7fd4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001e7fd4)?;
                    pc = 0x001e7fd4;
                    continue;
                }
                0x001e7fd4 => {
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    pc = 0x001e7fd8;
                    continue;
                }
                // DNF/multiplayer/special-event policy controls the remaining widget/audio.
                0x001e7fd8 => {
                    c.cycles += 9;
                    c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5bc8));
                    c.g[A2][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x49));
                    c.g[A1][0] = 0x2;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    let t = c.g[A2][0] == c.g[A1][0];
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    if t {
                        pc = 0x001e8028;
                        continue;
                    }
                    pc = 0x001e7ffc;
                    continue;
                }
                0x001e7ffc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x480));

                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = 0x470000;
                        pc = 0x001e802c;
                        continue;
                    }
                    pc = 0x001e8008;
                    continue;
                }
                0x001e8008 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x4a));
                    c.g[V0][0] = 0x6;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        pc = 0x001e8028;
                        continue;
                    }
                    pc = 0x001e8018;
                    continue;
                }
                0x001e8018 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x8;
                    if t {
                        pc = 0x001e8028;
                        continue;
                    }
                    pc = 0x001e8020;
                    continue;
                }
                0x001e8020 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        pc = 0x001e8050;
                        continue;
                    }
                    pc = 0x001e8028;
                    continue;
                }
                0x001e8028 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x470000;
                    pc = 0x001e802c;
                    continue;
                }
                0x001e802c => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e8038);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1778));
                    call_ptr(c, HIDE_WIDGET, 0x001e8038)?;
                    pc = 0x001e8038;
                    continue;
                }
                0x001e8038 => {
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001e8048);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x21a0));
                    call_ptr(c, HIDE_WIDGET, 0x001e8048)?;
                    pc = 0x001e8048;
                    continue;
                }
                0x001e8048 => {
                    let t = true;
                    frame.restore(c, S0, 0xc0);
                    if t {
                        pc = 0x001e8068;
                        continue;
                    }
                    pc = 0x001e8050;
                    continue;
                }
                0x001e8050 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001e8058);

                    call_ptr(c, GET_AUDIO, 0x001e8058)?;
                    pc = 0x001e8058;
                    continue;
                }
                0x001e8058 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001e8064);
                    c.g[A1][0] = 0xe;
                    call_ptr(c, PLAY_FE_SOUND, 0x001e8064)?;
                    pc = 0x001e8064;
                    continue;
                }
                0x001e8064 => {
                    frame.restore(c, S0, 0xc0);
                    pc = 0x001e8068;
                    continue;
                }
                // Restore the original frame, including all128 bits of saved GPRs.
                0x001e8068 => {
                    c.cycles += 11;
                    frame.restore(c, S1, 0xb0);
                    frame.restore(c, S2, 0xa0);
                    frame.restore(c, S3, 0x90);
                    frame.restore(c, S4, 0x80);
                    frame.restore(c, S5, 0x70);
                    frame.restore(c, S6, 0x60);
                    frame.restore(c, S7, 0x50);
                    frame.restore(c, FP, 0x40);
                    frame.restore_ra(c, 0x30);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
