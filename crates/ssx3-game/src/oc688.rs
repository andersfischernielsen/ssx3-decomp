//! Staging module oc688 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc688`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0037_5a40, renderer_init, "Renderer_Init")];

/// Direct-call targets of `Renderer_Init` (all in shard 12, so the generated
/// code called them directly).
pub(crate) mod renderer_init_callee {
    use super::ee;
    use super::Callee;

    pub(crate) const VU0_UPLOAD_MICROCODE_A: Callee = Callee {
        addr: 0x0039_12a8,
        f: ee::s12::f_003912a8,
    };
    pub(crate) const VU0_UPLOAD_MICROCODE_B: Callee = Callee {
        addr: 0x0039_1360,
        f: ee::s12::f_00391360,
    };
    pub(crate) const BUILD_DISPLAY_MODE_TABLE: Callee = Callee {
        addr: 0x0037_bb10,
        f: ee::s12::f_0037bb10,
    };
    pub(crate) const DEFER_MAN_INIT: Callee = Callee {
        addr: 0x0036_29b8,
        f: ee::s12::f_003629b8,
    };
    pub(crate) const TEX_MAN_CTOR: Callee = Callee {
        addr: 0x0036_6fe8,
        f: ee::s12::f_00366fe8,
    };
    pub(crate) const INTC_TABLE_INIT: Callee = Callee {
        addr: 0x0036_1eb8,
        f: ee::s12::f_00361eb8,
    };
    pub(crate) const SET_DITHER_MATRIX: Callee = Callee {
        addr: 0x0037_c198,
        f: ee::s12::f_0037c198,
    };
    pub(crate) const SET_BRIGHTNESS_RGB: Callee = Callee {
        addr: 0x0037_c570,
        f: ee::s12::f_0037c570,
    };
    pub(crate) const SET_BRIGHTNESS_ALPHA: Callee = Callee {
        addr: 0x0037_c720,
        f: ee::s12::f_0037c720,
    };
}

/// `void Renderer_Init(PS2GraphicsMan *this)` (0x375a40,
/// `cPS2GraphicsMan_Init`): bring up the PS2 graphics manager. In order:
///
/// 1. gp+0x2930 = `sceDmaGetChan(0)`, upload the VU0 microcode (A and B);
/// 2. reset the draw state (`+0x13e0/+0x10dc/+0x6d3c`, scratch tag
///    `0x70000000` at `+0x13e8`, `+0x59c8 = -1`), then reset VIF0, VU0, VU1,
///    the GS path and DMAC;
/// 3. allocate the two render command buffers (`0x199940` and `0x4cc40`
///    bytes, 128-byte aligned, heads at `+0x5a00..` and `+0x5a14..`) and call
///    the GS vtable slot `+0x3a8` (`this + delta`, 0, 0);
/// 4. kick the hardware directly: DMAC_STAT/VIF1/GIF channel control, GIF
///    ctrl, VIF1 FBRST; a 10000-pass delay; a one-quadword VIF1 DMA from
///    the scratchpad; wait for VIF1/GIF DMA idle, VIF1 STAT idle and the GS
///    FIFO to report empty;
/// 5. zero the sync state (`+0x5a8c..+0x5abc`), create the two semaphores
///    (`+0x5ac8`, `+0x5acc`), the DMA-kick thread (`+0x5ad0`, entry
///    `Renderer_DmaKickThreadEntry` 0x382740, stack `+0x5ae0`, size `0x1000`, priority 5), the INTC 2
///    (VBlank-on) and DMAC 1 handlers (`+0x5ac0`, `+0x5ac4`) and start the
///    thread;
/// 6. `BuildDisplayModeTable`, the `DeferMan` (`+0x18f0`, 0x6b100 bytes) and
///    `TexMan` (`+0x18f4`, 0x6740 bytes with two sub-pointers) objects,
///    `IntcTable_Init`, copy a 20-byte block into `*(this+0xe84)`, the dither
///    matrix (64 bytes from `0x492898`), brightness RGB (1,1,1) and alpha 0;
/// 7. GS vtable calls (`+0x144`, `+0x44`, the display size via `+0x54` and
///    `+0x5c`, converted to float with the unsigned fix-up, `+0xc4`, `+0x12c`),
///    the camera/lighting constants in the `0x5047xx/0x5083xx` tables, and the
///    two-iteration `+0x3a4` loop followed by `+0x3a4(-1)`.
///
/// The body keeps the generated block structure (nested exit blocks and the
/// long run of calls); every call is checked for a host-thread unwind.
pub fn renderer_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 106;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[A0][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2930));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S1][0] = 0xffffffffffff0000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S1][0] = c.g[S1][0] | 0xff80;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
        c.g[RA][0] = sx(0x00375a88);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
        call_ptr(c, 0x003fdf88, 0x00375a88)?; // sceDmaGetChan
        c.sw((c.g[GP][0] as u32).wrapping_add(0x2930), c.g[V0][0]);
        c.g[RA][0] = sx(0x00375a94);
        c.g[A0][0] = c.g[S0][0];
        renderer_init_callee::VU0_UPLOAD_MICROCODE_A.call(c, 0x00375a94)?; // vu0_upload_microcode_a
        c.g[RA][0] = sx(0x00375a9c);
        c.g[A0][0] = c.g[S0][0];
        renderer_init_callee::VU0_UPLOAD_MICROCODE_B.call(c, 0x00375a9c)?; // vu0_upload_microcode_b
        c.g[V1][0] = 0x70000000;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10e0));
        c.g[V0][0] = 0xffffffffffffffff;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x13e0), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10dc), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x6d3c), 0u64);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x13e8), c.g[V1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x13e4), c.g[A0][0]);
        c.g[RA][0] = sx(0x00375ac4);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x59c8), c.g[V0][0]);
        call_ptr(c, 0x003ff060, 0x00375ac4)?; // sceDevVif0Reset
        c.g[RA][0] = sx(0x00375acc);
        call_ptr(c, 0x003fe690, 0x00375acc)?; // sceDevVu0Reset
        c.g[RA][0] = sx(0x00375ad4);
        call_ptr(c, 0x003febc0, 0x00375ad4)?; // sceDevVu1Reset
        c.g[RA][0] = sx(0x00375adc);
        call_ptr(c, 0x003fdab0, 0x00375adc)?; // sceGsResetPath
        c.g[RA][0] = sx(0x00375ae4);
        c.g[A0][0] = 0x1;
        call_ptr(c, 0x003fdfb0, 0x00375ae4)?; // sceDmaReset
        c.sw((c.g[S2][0] as u32).wrapping_add(0x6b90), 0u64);
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = 0x190000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4368));
        c.g[A0][0] = c.g[A0][0] | 0x9940;
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00375b04);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x00375b04)?; // operator_new_named
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a10), 0u64);
        c.g[V1][0] = c.g[V1][0] & c.g[S1][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = 0x40000;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = c.g[A0][0] | 0xcc40;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4370));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
        c.g[A2][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a0c), c.g[V1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a04), c.g[V1][0]);
        c.g[RA][0] = sx(0x00375b3c);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a08), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x00375b3c)?; // operator_new_named
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a24), 0u64);
        c.g[V1][0] = c.g[V1][0] & c.g[S1][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a1c), c.g[V0][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a14), c.g[V1][0]);
        c.g[A1][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a20), c.g[V1][0]);
        c.g[A2][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5a18), c.g[V1][0]);
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x3a8));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x3ac));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00375b74);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00375b74)?; // virtual method (vtable slot)
        c.g[V0][0] = 0x10000000;
        c.g[A0][0] = 0xa006;
        c.g[V0][0] = c.g[V0][0] | 0xe010;
        c.g[V1][0] = 0x10000000;
        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
        c.g[A1][0] = 0x81;
        c.g[V1][0] = c.g[V1][0] | 0x9000;
        c.g[V0][0] = 0x10000000;
        c.sw((c.g[V1][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.g[V0][0] | 0xa000;
        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
        c.g[V1][0] = 0x10000000;
        c.g[V1][0] = c.g[V1][0] | 0x3000;
        c.g[V0][0] = 0x1;
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        c.g[A0][0] = 0x10000000;
        c.g[V1][0] = 0x9;
        c.g[A0][0] = c.g[A0][0] | 0x3c10;
        c.g[V0][0] = 0x10000000;
        c.sw((c.g[A0][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] | 0x3c20;
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V1][0] = 0x270f;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
        'l_375be8: loop {
            c.cycles += 7;
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00375be8);
                }
                continue 'l_375be8;
            } else {
                'b_375c54: {
                    c.cycles += 11;
                    c.g[AT][0] = 0x70000000;
                    c.sd((c.g[AT][0] as u32).wrapping_add(0x8), 0u64);
                    c.g[A1][0] = 0x70000000;
                    c.sd((c.g[A1][0] as u32), 0u64);
                    c.g[V0][0] = 0x10000000;
                    c.g[V0][0] = c.g[V0][0] | 0x9000;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5ae0));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.g[V1][0] & 0x100;
                    let t = c.g[V1][0] == 0u64;
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c54));
                    if t {
                        break 'b_375c54;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = 0x10000000;
                        c.g[V1][0] = c.g[V1][0] | 0x9000;
                        'l_375c38: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                            c.g[V0][0] = c.g[V0][0] & 0x100;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00375c38);
                                }
                                continue 'l_375c38;
                            } else {
                                break 'b_375c54;
                            }
                        }
                    }
                }
                'b_375cb4: {
                    c.cycles += 16;
                    c.g[V0][0] = 0x10000000;
                    c.g[V1][0] = 0xffffffff80000000;
                    c.g[V0][0] = c.g[V0][0] | 0x9010;
                    c.g[A0][0] = 0x10000000;
                    c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                    c.g[A0][0] = c.g[A0][0] | 0x9020;
                    c.g[V0][0] = 0x1;
                    c.g[V1][0] = 0x10000000;
                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                    c.g[V1][0] = c.g[V1][0] | 0x9000;
                    c.g[A0][0] = 0x101;
                    c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[V0][0] & 0x100;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x10000000;
                    if t {
                        break 'b_375cb4;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = c.g[V1][0] | 0x9000;
                        'l_375c98: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                            c.g[V0][0] = c.g[V0][0] & 0x100;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00375c98);
                                }
                                continue 'l_375c98;
                            } else {
                                break 'b_375cb4;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = 0x10000000;
                c.g[V1][0] = c.g[V1][0] | 0xa000;
                'l_375cc0: loop {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.g[V0][0] & 0x100;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x10000000;
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00375cc0);
                        }
                        continue 'l_375cc0;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = 0x1f000000;
                        c.g[A0][0] = c.g[A0][0] | 0x3c00;
                        c.g[V1][0] = c.g[V1][0] | 0x3;
                        'l_375ce8: loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00375ce8);
                                }
                                continue 'l_375ce8;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = 0x12000000;
                                c.g[A0][0] = 0x4000;
                                c.g[V1][0] = c.g[V1][0] | 0x1000;
                                'l_375d10: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.ld((c.g[V1][0] as u32));
                                    c.g[V0][0] = c.g[V0][0] & 0xc000;
                                    let t = c.g[V0][0] != c.g[A0][0];

                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00375d10);
                                        }
                                        continue 'l_375d10;
                                    } else {
                                        'b_376064: {
                                            'b_37604c: {
                                                c.cycles += 196;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5a8c),
                                                    0u64,
                                                );
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5a90),
                                                    0u64,
                                                );
                                                c.g[S1][0] = 0x1;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5a94),
                                                    0u64,
                                                );
                                                c.g[A0][0] = c.g[SP][0];
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5a98),
                                                    c.g[V0][0],
                                                );
                                                c.g[A1][0] = 0u64;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5aa4),
                                                    c.g[S1][0],
                                                );
                                                c.g[A2][0] = 0x18;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5ab8),
                                                    c.g[S1][0],
                                                );
                                                c.g[S0][0] = 0x10;
                                                c.g[RA][0] = sx(0x00375d64);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5abc),
                                                    0u64,
                                                );
                                                call_ptr(c, 0x00416210, 0x00375d64)?; // memset
                                                c.g[A0][0] = c.g[SP][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    c.g[S0][0],
                                                );
                                                c.g[RA][0] = sx(0x00375d74);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                                call_ptr(c, 0x00423da0, 0x00375d74)?; // CreateSema
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5ac8),
                                                    c.g[V0][0],
                                                );
                                                c.g[A0][0] = c.g[SP][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x4),
                                                    c.g[S1][0],
                                                );
                                                c.g[RA][0] = sx(0x00375d88);
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x8),
                                                    c.g[S1][0],
                                                );
                                                call_ptr(c, 0x00423da0, 0x00375d88)?; // CreateSema
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0x30;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[RA][0] = sx(0x00375d9c);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5acc),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x00416210, 0x00375d9c)?; // memset
                                                c.g[V0][0] = 0x1000;
                                                c.g[V1][0] = 0x380000;
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x2c),
                                                    c.g[V0][0],
                                                );
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x2740));
                                                c.g[A1][0] = 0x5;
                                                c.g[V0][0] = 0x4a0000;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x30f0));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    c.g[S0][0],
                                                );
                                                c.g[A0][0] = c.g[S3][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x24),
                                                    c.g[V1][0],
                                                );
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x38),
                                                    c.g[A1][0],
                                                );
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x34),
                                                    c.g[A1][0],
                                                );
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x28),
                                                    c.g[S4][0],
                                                );
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    c.g[V0][0],
                                                );
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), 0u64);
                                                c.g[RA][0] = sx(0x00375de0);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                                                call_ptr(c, 0x00423ba0, 0x00375de0)?; // CreateThread
                                                c.g[A0][0] = 0x2;
                                                c.g[RA][0] = sx(0x00375dec);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5ad0),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x00424880, 0x00375dec)?; // DisableIntc
                                                c.g[A1][0] = 0x380000;
                                                c.g[A2][0] = 0u64;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x25c0));
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x00375e04);
                                                c.g[A0][0] = 0x2;
                                                call_ptr(c, 0x00423a90, 0x00375e04)?; // AddIntcHandler
                                                c.g[A0][0] = 0x2;
                                                c.g[RA][0] = sx(0x00375e10);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5ac0),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x004248e8, 0x00375e10)?; // EnableIntc
                                                c.g[RA][0] = sx(0x00375e18);
                                                c.g[A0][0] = 0x1;
                                                call_ptr(c, 0x00424950, 0x00375e18)?; // DisableDmac
                                                c.g[A1][0] = 0x380000;
                                                c.g[A2][0] = 0u64;
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x2650));
                                                c.g[RA][0] = sx(0x00375e30);
                                                c.g[A0][0] = 0x1;
                                                call_ptr(c, 0x00423ac0, 0x00375e30)?; // AddDmacHandler
                                                c.g[A0][0] = 0x1;
                                                c.g[RA][0] = sx(0x00375e3c);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5ac4),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x004249b8, 0x00375e3c)?; // EnableDmac
                                                c.g[A0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x5ad0));
                                                c.g[RA][0] = sx(0x00375e48);
                                                c.g[A1][0] = c.g[S2][0];
                                                call_ptr(c, 0x00423bc0, 0x00375e48)?; // _StartThread
                                                c.g[RA][0] = sx(0x00375e50);
                                                c.g[A0][0] = c.g[S2][0];
                                                renderer_init_callee::BUILD_DISPLAY_MODE_TABLE
                                                    .call(c, 0x00375e50)?; // build_display_mode_table
                                                c.g[A0][0] = 0x60000;
                                                c.g[A1][0] = 0x490000;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x2888));
                                                c.g[A2][0] = 0u64;
                                                c.g[A3][0] = 0u64;
                                                c.g[RA][0] = sx(0x00375e6c);
                                                c.g[A0][0] = c.g[A0][0] | 0xb100;
                                                call_ptr(c, 0x00317d70, 0x00375e6c)?; // EA_MemAllocNamed
                                                c.g[RA][0] = sx(0x00375e74);
                                                c.g[A0][0] = c.g[V0][0];
                                                call_ptr(c, 0x00394d50, 0x00375e74)?; // DeferMan_ctor
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00375e80);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x18f0),
                                                    c.g[V0][0],
                                                );
                                                renderer_init_callee::DEFER_MAN_INIT
                                                    .call(c, 0x00375e80)?; // defer_man_init
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A2][0] = 0u64;
                                                c.g[A3][0] = 0u64;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4378));
                                                c.g[RA][0] = sx(0x00375e98);
                                                c.g[A0][0] = 0x6740;
                                                call_ptr(c, 0x00317d70, 0x00375e98)?; // EA_MemAllocNamed
                                                c.g[V1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1f60));
                                                c.g[A1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x4350));
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x3f50),
                                                    c.g[V1][0],
                                                );
                                                c.g[A0][0] = c.g[V0][0];
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x6340),
                                                    c.g[A1][0],
                                                );
                                                c.g[RA][0] = sx(0x00375eb4);
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x18f4),
                                                    c.g[V0][0],
                                                );
                                                renderer_init_callee::TEX_MAN_CTOR
                                                    .call(c, 0x00375eb4)?; // tex_man_ctor
                                                c.g[A0][0] = 0x500000;
                                                c.g[RA][0] = sx(0x00375ec0);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x59d8));
                                                renderer_init_callee::INTC_TABLE_INIT
                                                    .call(c, 0x00375ec0)?; // intc_table_init
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
                                                c.g[V0][0] = 0x500000;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[T1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1420));
                                                c.ldl(6, (c.g[T1][0] as u32).wrapping_add(0x7));
                                                c.ldr(6, (c.g[T1][0] as u32));
                                                c.ldl(7, (c.g[T1][0] as u32).wrapping_add(0xf));
                                                c.ldr(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                                                c.g[T0][0] =
                                                    c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                                                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                c.sdr(6, (c.g[V1][0] as u32));
                                                c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[T0][0],
                                                );
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x59d0),
                                                    c.g[S1][0],
                                                );
                                                c.g[V0][0] = 0x490000;
                                                c.g[T0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x2898));
                                                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
                                                c.ldr(3, (c.g[T0][0] as u32));
                                                c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
                                                c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
                                                c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x17));
                                                c.ldr(7, (c.g[T0][0] as u32).wrapping_add(0x10));
                                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
                                                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x5f));
                                                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x58));
                                                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x1f));
                                                c.ldr(3, (c.g[T0][0] as u32).wrapping_add(0x18));
                                                c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0x27));
                                                c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x20));
                                                c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x2f));
                                                c.ldr(7, (c.g[T0][0] as u32).wrapping_add(0x28));
                                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x6f));
                                                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x68));
                                                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x77));
                                                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7f));
                                                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x78));
                                                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x37));
                                                c.ldr(3, (c.g[T0][0] as u32).wrapping_add(0x30));
                                                c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0x3f));
                                                c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x38));
                                                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x87));
                                                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x8f));
                                                c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x88));
                                                c.g[RA][0] = sx(0x00375f8c);
                                                c.g[A1][0] = c.g[S6][0];
                                                renderer_init_callee::SET_DITHER_MATRIX
                                                    .call(c, 0x00375f8c)?; // set_dither_matrix
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                                c.g[A1][0] = c.g[S5][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x78));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x90), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x94), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x98), 0u64);
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x00375fb0);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00375fb0)?; // virtual method (vtable slot)
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[A1][0] = c.g[S5][0];
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x59e8),
                                                    c.g[S1][0],
                                                );
                                                c.g[A0][0] = c.g[S2][0];
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x98));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                                                c.g[RA][0] = sx(0x00375fd8);
                                                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x59fc));
                                                renderer_init_callee::SET_BRIGHTNESS_RGB
                                                    .call(c, 0x00375fd8)?; // set_brightness_rgb
                                                c.mtc1(0, 12);
                                                c.g[RA][0] = sx(0x00375fe4);
                                                c.g[A0][0] = c.g[S2][0];
                                                renderer_init_callee::SET_BRIGHTNESS_ALPHA
                                                    .call(c, 0x00375fe4)?; // set_brightness_alpha
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x140));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x144));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00375ffc);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00375ffc)?; // virtual method (vtable slot)
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                c.g[A2][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                let t = c.g[A2][0] as u32;
                                                c.g[RA][0] = sx(0x00376014);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00376014)?; // virtual method (vtable slot)
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                                c.g[V0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                                c.g[S1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0xc4));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                                                c.g[S0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x00376034);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00376034)?; // virtual method (vtable slot)
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                                if t {
                                                    break 'b_37604c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.mtc1(2, 20);
                                                    c.cvt_s_w(20, 20);
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S2][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                    if t {
                                                        break 'b_376064;
                                                    } else {
                                                        break 'b_37604c;
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                            c.mtc1(2, 20);
                                            c.cvt_s_w(20, 20);
                                            c.add_s(20, 20, 20);
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                            break 'b_376064;
                                        }
                                        'b_3760a4: {
                                            'b_37608c: {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x00376074);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00376074)?; // virtual method (vtable slot)
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                                if t {
                                                    break 'b_37608c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.mtc1(2, 0);
                                                    c.cvt_s_w(0, 0);
                                                    let t = true;
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0xac),
                                                    );
                                                    if t {
                                                        break 'b_3760a4;
                                                    } else {
                                                        break 'b_37608c;
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                            c.mtc1(2, 0);
                                            c.cvt_s_w(0, 0);
                                            c.add_s(0, 0, 0);
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xac));
                                            break 'b_3760a4;
                                        }
                                        c.cycles += 73;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa8));
                                        c.g[A1][0] = c.g[FP][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), 0u64);
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), 0u64);
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x003760c4);
                                        c.g[A2][0] = 0x1;
                                        call_ptr(c, t, 0x003760c4)?; // virtual method (vtable slot)
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x128));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x12c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x003760d8);
                                        c.g[A0][0] =
                                            sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x003760d8)?; // virtual method (vtable slot)
                                        c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x27ec));
                                        c.g[A0][0] = 0x500000;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6d14), 0u64);
                                        c.g[A3][0] = 0x500000;
                                        c.g[AT][0] = 0xffffffffc0c00000;
                                        c.mtc1(1, 2);
                                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4730));
                                        c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x4730));
                                        c.g[T1][0] = 0x510000;
                                        c.g[AT][0] = 0x43000000;
                                        c.mtc1(1, 1);
                                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7c80));
                                        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x20));
                                        c.g[A1][0] = 0x510000;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 0);
                                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7c40));
                                        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3c));
                                        c.g[T0][0] = 0x510000;
                                        c.g[AT][0] = 0x40c00000;
                                        c.mtc1(1, 6);
                                        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0xc));
                                        c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x27e8));
                                        c.g[A2][0] = 0x3;
                                        c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x27e4));
                                        c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x27e0));
                                        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                                        c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                                        c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                        c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x14));
                                        c.swc1(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                                        c.g[AT][0] = 0x3f000000;
                                        c.mtc1(1, 3);
                                        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x27dc));
                                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x27d8));
                                        c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x27d4));
                                        c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x27d0));
                                        c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x27cc));
                                        c.swc1(0, (c.g[T1][0] as u32).wrapping_sub(0x7c80));
                                        c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x14));
                                        c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x24));
                                        c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x28));
                                        c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x38));
                                        c.swc1(0, (c.g[T0][0] as u32).wrapping_sub(0x7c40));
                                        c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                                        c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                                        c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x10));
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                                        c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c));
                                        c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x30));
                                        c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x34));
                                        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                                        c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6c50), 0u64);
                                        'l_3761c8: loop {
                                            c.cycles += 7;
                                            c.sw((c.g[A1][0] as u32), 0u64);
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[A2][0] as i64) >= 0;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x003761c8);
                                                }
                                                continue 'l_3761c8;
                                            } else {
                                                c.cycles += 5;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5a28),
                                                    0u64,
                                                );
                                                c.g[S0][0] = 0u64;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x5a2c),
                                                    0u64,
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
                                                'l_3761f8: loop {
                                                    c.cycles += 9;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V0][0] as u32).wrapping_add(0x3a0)
                                                        );
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x3a4)
                                                        );
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00376210);
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x00376210)?; // virtual method (vtable slot)
                                                    c.g[V0][0] =
                                                        ((c.g[S0][0] as i64) < 2i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S2][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x003761f8);
                                                        }
                                                        continue 'l_3761f8;
                                                    } else {
                                                        c.cycles += 18;
                                                        c.g[A1][0] = 0xffffffffffffffff;
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x3a0));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x3a4));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[RA][0] = sx(0x00376230);
                                                        c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x00376230)?; // virtual method (vtable slot)
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x140),
                                                        );
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        c.lq(
                                                            18,
                                                            (c.g[SP][0] as u32).wrapping_add(0x120),
                                                        );
                                                        c.lq(
                                                            19,
                                                            (c.g[SP][0] as u32).wrapping_add(0x110),
                                                        );
                                                        c.lq(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x100),
                                                        );
                                                        c.lq(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                        );
                                                        c.lq(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xe0),
                                                        );
                                                        c.lq(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.lq(
                                                            30,
                                                            (c.g[SP][0] as u32).wrapping_add(0xc0),
                                                        );
                                                        c.g[RA][0] = c
                                                            .ld((c.g[SP][0] as u32)
                                                                .wrapping_add(0xb0));
                                                        c.lwc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0x150),
                                                        );
                                                        let t = c.g[RA][0] as u32;
                                                        c.g[SP][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x160));
                                                        c.pc = t;
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{Rng, DATA};

    /// Seeds (each runs both bodies, including the real `DeferMan_ctor`).
    const RINIT_SEEDS: u32 = 40;
    /// Gate word and magic: the callee fakes act only when it is set.
    const RINIT_GATE: u32 = DATA + 0x3000;
    const RINIT_MAGIC: u32 = 0x5249_4e54;
    /// Bump pointer of the allocation fakes.
    const RINIT_BUMP: u32 = DATA + 0x3008;
    /// Values the two display-size virtual methods return (`+0x54`, `+0x5c`).
    const RINIT_VALUE_W: u32 = DATA + 0x3010;
    const RINIT_VALUE_H: u32 = DATA + 0x3014;
    /// The object, its GS vtable and the 20-byte block at `+0xe84`.
    const RINIT_OBJ: u32 = DATA + 0x1_0000;
    const RINIT_VTABLE: u32 = DATA + 0x2_0000;
    const RINIT_BLOCK: u32 = DATA + 0x2000;
    /// Allocation arena of the fakes (clear of the fakes' call log).
    const RINIT_ARENA: u32 = DATA + 0x20_0000;
    /// Fake method addresses whose result the test chooses.
    const RINIT_FAKE_W: u32 = FAKE + 0x400;
    const RINIT_FAKE_H: u32 = FAKE + 0x410;
    /// The vtable slots the function uses (delta at `off`, method at `off+4`).
    const RINIT_SLOTS: [u32; 9] = [0x3a8, 0x78, 0x140, 0x40, 0xc0, 0x50, 0x58, 0x128, 0x3a0];

    /// A gated fake of a callee that has an override slot: logs the call
    /// (all registers) and returns like a fake callee; with the gate off it
    /// runs the body it replaced.
    macro_rules! renderer_init_fake {
        ($name:ident, $addr:expr, $generated:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(RINIT_GATE) != RINIT_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    renderer_init_fake!(
        oc688_fake_vu0_a,
        0x0039_12a8,
        ee::s12::f_003912a8__generated
    );
    renderer_init_fake!(
        oc688_fake_vu0_b,
        0x0039_1360,
        ee::s12::f_00391360__generated
    );
    renderer_init_fake!(
        oc688_fake_vif0_reset,
        0x003f_f060,
        ee::s15::f_003ff060__generated
    );
    renderer_init_fake!(
        oc688_fake_vu0_reset,
        0x003f_e690,
        ee::s15::f_003fe690__generated
    );
    renderer_init_fake!(
        oc688_fake_vu1_reset,
        0x003f_ebc0,
        ee::s15::f_003febc0__generated
    );
    renderer_init_fake!(
        oc688_fake_gs_reset_path,
        0x003f_dab0,
        ee::s15::f_003fdab0__generated
    );
    renderer_init_fake!(
        oc688_fake_defer_man_init,
        0x0036_29b8,
        ee::s12::f_003629b8__generated
    );
    renderer_init_fake!(
        oc688_fake_tex_man_ctor,
        0x0036_6fe8,
        ee::s12::f_00366fe8__generated
    );
    renderer_init_fake!(
        oc688_fake_intc_table_init,
        0x0036_1eb8,
        ee::s12::f_00361eb8__generated
    );
    renderer_init_fake!(
        oc688_fake_brightness_rgb,
        0x0037_c570,
        ee::s12::f_0037c570__generated
    );
    renderer_init_fake!(
        oc688_fake_brightness_alpha,
        0x0037_c720,
        ee::s12::f_0037c720__generated
    );

    /// Allocator fake: log the call like any fake, then hand out the next
    /// block of the arena (misaligned by 0x10, so the 128-byte round-up of
    /// the render buffers is exercised).
    fn oc688_alloc(
        c: &mut Cpu,
        addr: u32,
        generated: fn(&mut Cpu),
        prev: &'static std::sync::atomic::AtomicUsize,
    ) {
        if c.bus.read32(RINIT_GATE) != RINIT_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            let mut next = c.bus.read32(RINIT_BUMP);
            if next == 0 {
                next = RINIT_ARENA + 0x10;
            }
            c.g[V0][0] = sx(next);
            c.bus.write32(RINIT_BUMP, next + 0x8_0000);
        }
    }

    fn oc688_fake_new_named(c: &mut Cpu) {
        oc688_alloc(
            c,
            0x0031_7e30,
            ee::s10::f_00317e30__generated,
            fakes::prev_cell(oc688_fake_new_named),
        )
    }

    fn oc688_fake_mem_alloc_named(c: &mut Cpu) {
        oc688_alloc(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70__generated,
            fakes::prev_cell(oc688_fake_mem_alloc_named),
        )
    }

    /// The `miss` hook: the two display-size methods return the test's
    /// values (the fake log still records the call); everything else is a
    /// plain fake call.
    fn oc688_miss(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        match target {
            RINIT_FAKE_W => c.g[V0][0] = sx(c.bus.read32(RINIT_VALUE_W)),
            RINIT_FAKE_H => c.g[V0][0] = sx(c.bus.read32(RINIT_VALUE_H)),
            _ => {}
        }
    }

    /// Hook every callee fake once per process.
    fn oc688_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003912A8),
            oc688_fake_vu0_a,
            fakes::prev_cell(oc688_fake_vu0_a),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391360),
            oc688_fake_vu0_b,
            fakes::prev_cell(oc688_fake_vu0_b),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FF060),
            oc688_fake_vif0_reset,
            fakes::prev_cell(oc688_fake_vif0_reset),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FE690),
            oc688_fake_vu0_reset,
            fakes::prev_cell(oc688_fake_vu0_reset),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FEBC0),
            oc688_fake_vu1_reset,
            fakes::prev_cell(oc688_fake_vu1_reset),
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FDAB0),
            oc688_fake_gs_reset_path,
            fakes::prev_cell(oc688_fake_gs_reset_path),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_003629B8),
            oc688_fake_defer_man_init,
            fakes::prev_cell(oc688_fake_defer_man_init),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00366FE8),
            oc688_fake_tex_man_ctor,
            fakes::prev_cell(oc688_fake_tex_man_ctor),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00361EB8),
            oc688_fake_intc_table_init,
            fakes::prev_cell(oc688_fake_intc_table_init),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037C570),
            oc688_fake_brightness_rgb,
            fakes::prev_cell(oc688_fake_brightness_rgb),
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037C720),
            oc688_fake_brightness_alpha,
            fakes::prev_cell(oc688_fake_brightness_alpha),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc688_fake_new_named,
            fakes::prev_cell(oc688_fake_new_named),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc688_fake_mem_alloc_named,
            fakes::prev_cell(oc688_fake_mem_alloc_named),
        );
    }

    /// `Renderer_Init(this)`: the long bring-up run against fake callees
    /// (resets, microcode upload, allocators, manager constructors) and a
    /// fake GS vtable. The real hardware polls run against the machine's
    /// idle DMA channels, VIF1 and GS FIFO. Seeds vary the sizes returned by
    /// the display-size methods (positive, `0x7fffffff`, `0x80000000`
    /// (sign-extended negative: the unsigned convert fix-up, with and
    /// without the low bit), all ones) and the random vtable deltas; a third
    /// of the seeds unwind one of the ~35 calls.
    #[test]
    fn renderer_init_matches() {
        oc688_install_fakes();
        const VALUES: [u32; 6] = [
            0x1e0,
            0x7fff_ffff,
            0x8000_0000,
            0x8000_0001,
            0xffff_ffff,
            0xffff_fff0,
        ];
        covered(
            ee::s12::f_00375a40__generated,
            renderer_init,
            RINIT_SEEDS,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x0688_0001, 36);
                c.hooks.miss = oc688_miss;
                c.pc = 0x0037_5a40;
                c.g[A0][0] = sx(RINIT_OBJ);
                c.bus.write32(RINIT_GATE, RINIT_MAGIC);
                c.bus.write32(RINIT_BUMP, 0);
                c.bus.write32(RINIT_VALUE_W, VALUES[seed as usize % 6]);
                c.bus
                    .write32(RINIT_VALUE_H, VALUES[(seed as usize / 6) % 6]);
                c.bus.write32(RINIT_OBJ + 0x10d8, RINIT_VTABLE);
                c.bus.write32(RINIT_OBJ + 0xe84, RINIT_BLOCK);
                for (i, &off) in RINIT_SLOTS.iter().enumerate() {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(RINIT_VTABLE + off, delta as u16);
                    c.bus
                        .write32(RINIT_VTABLE + off + 4, FAKE + 0x10 * i as u32);
                }
                // The display-size slots (`+0x50`, `+0x58`) return the chosen values.
                c.bus.write32(RINIT_VTABLE + 0x50 + 4, RINIT_FAKE_W);
                c.bus.write32(RINIT_VTABLE + 0x58 + 4, RINIT_FAKE_H);
            },
        );
    }
}
