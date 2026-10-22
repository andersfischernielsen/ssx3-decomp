//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacement for one function, with a differential test against the
//! generated body.
//!
//! `cBoardTrailFX_Update` (0x2e8938) is the per-frame update of the trail a
//! rider's board leaves in the snow (`src/visualfx/boardtrailfx.cpp`). Its
//! object is the `cBoardTrailFX` layout `cBoardTrailFX_initialize` (0x2e83f0)
//! builds: the rider pointer at +0x00, the fade values at +0x04/+0x08, the six
//! strip pointers at +0x10..+0x24 (54 vertices of 0x30 bytes each), the ring
//! head at +0x8c, the sample count at +0x90, the trail state at +0x94, the
//! fade rate at +0x9c, the vertex width at +0xa0, three colour quads at
//! +0xb0/+0xc0/+0xd0 and the emit cooldown at +0xe0.
//!
//! * `cBoardTrailFX_IsActive` (0x2e87e8) decides whether the rider is leaving
//!   a trail and leaves its fade rate at +0x9c. When it says no the trail is
//!   ending: state 1 rewinds the ring by three samples, any other non-zero
//!   state wipes the strips (six passes over the strip pointers), and the
//!   state goes back to 0.
//! * When it says yes, a fresh trail (state 0) advances the ring head by three
//!   vertices and keeps the resulting sample index in `s3`. The current
//!   vertex is rebuilt from the rider's board frame (`rider+0x370`,
//!   `rider+0x460`) and the animation entry
//!   `*(*(rider+0x780) + 0x34) + (*(rider+0x8a4) << 6)`.
//! * Its colour and width are blended by a long VU0 dot-product sequence
//!   (`f20`-`f27` hold the intermediate results) against the gp float
//!   constants, with the rider sub-state `func_0011FE98` (0x11fe98, through
//!   the function table), the animation-base query `func_00312AA0`
//!   (0x312aa0) and the float helper `func_0031C228` (0x31c228, returns f0)
//!   choosing the fade.
//! * The rider's animation-base virtual method (`*(rider+0x6c0)+0x3c`, the
//!   function's only `jalr`) returns an index; it scales a 0xf0-byte-stride
//!   table near 0x4fa398 into the sample's weights.
//! * `cBoardTrailFX_ShouldEmit` (0x2e86f0) finally decides whether this frame
//!   pushes a sample: the emit path writes the colour quad at +0xd0, the
//!   fades at +0x04/+0x08, advances the head/count and shifts the six strips
//!   around the new vertex.
//!
//! The original is one gcc basic-block chain that falls through, so the port
//! keeps its labelled blocks, every cycle charge at the same point, every
//! register write (including the v0/v1 scratch values and f20-f27) and every
//! VU0 macro op in the same order. Calls go through [`crate::call`], and the
//! five loop back edges deliver events where the original checks
//! `cycles >= next_event`.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002e_8938,
    c_board_trail_fx_update,
    "cBoardTrailFX_Update",
)];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct `jal`s (one shard-local alias each).

mod callee {
    use super::{ee, Callee};

    /// `cBoardTrailFX_IsActive(self)` (0x2e87e8).
    pub const IS_ACTIVE: Callee = Callee {
        addr: 0x002e_87e8,
        f: ee::s09::cBoardTrailFX_IsActive,
    };
    /// `cBoardTrailFX_ShouldEmit(self)` (0x2e86f0).
    pub const SHOULD_EMIT: Callee = Callee {
        addr: 0x002e_86f0,
        f: ee::s09::cBoardTrailFX_ShouldEmit,
    };
}

// ---------------------------------------------------------------------------
// Register and address helpers.

/// The low 32 bits of a GPR (what the generated code's `as u32` reads).
#[inline(always)]
fn g(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `a + off`, wrapping like `addu`/`addiu`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    g(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// cBoardTrailFX_Update.

/// `void cBoardTrailFX_Update(cBoardTrailFX* self)` (0x2e8938): advance the
/// board trail of one rider by a frame (see the module comment).
pub fn c_board_trail_fx_update(c: &mut Cpu) {
    guest(c, body)
}

/// The body, split out so `?` on the call protocol has a `Flow` to return.
fn body(c: &mut Cpu) -> Flow {
    let frame = Frame::open(c, 0x2b0);
    'b_2e8980: {
        c.cycles += 18;
        c.sq(16, frame.at(c, 0x260));
        c.sq(17, frame.at(c, 0x250));
        c.g[S0][0] = c.g[A0][0];
        c.sq(19, frame.at(c, 0x230));
        c.g[S1][0] = 0x36;
        c.sq(18, frame.at(c, 0x240));
        c.sd(frame.at(c, 0x220), c.g[RA][0]);
        c.swc1(27, frame.at(c, 0x2a8));
        c.swc1(26, frame.at(c, 0x2a0));
        c.swc1(25, frame.at(c, 0x298));
        c.swc1(24, frame.at(c, 0x290));
        c.swc1(23, frame.at(c, 0x288));
        c.swc1(22, frame.at(c, 0x280));
        c.swc1(21, frame.at(c, 0x278));
        c.swc1(20, frame.at(c, 0x270));
        if c.g[S1][0] == 0u64 {
            break 'b_2e8980;
        } else {
            break 'b_2e8980;
        }
    }
    // The update proper. `s0` holds the trail object, `s1` is 54 (vertices
    // per strip) and `s3` the vertex index the emit path will write.
    'b_2ea440: {
        'b_2e8b38: {
            c.cycles += 8;
            c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
            c.g[V0][0] = sx(p(g(c, V0), 0x35));
            c.div(0, 2, 17);
            c.g[V1][0] = c.hi[0];
            c.g[S3][0] = c.g[V1][0];
            callee::IS_ACTIVE.call(c, 0x002e_8998)?;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw(p(g(c, S0), 0x94));
                break 'b_2e8b38;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x94));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_2ea440;
                } else {
                    'b_2e8b30: {
                        'b_2e89f4: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = 0x30;
                            if t {
                                break 'b_2e89f4;
                            } else {
                                'b_2e89c4: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[V0][0] = 0x36;
                                    if c.g[V0][0] == 0u64 {
                                        break 'b_2e89c4;
                                    } else {
                                        break 'b_2e89c4;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A1][0] = 0x36;
                                c.g[V1][0] = sx(p(g(c, V1), 0x33));
                                c.g[A0][0] = 0xffffffffffffffff;
                                c.div(0, 3, 17);
                                c.g[V0][0] = c.hi[0];
                                c.sw(p(g(c, S0), 0x8c), c.g[V0][0]);
                                c.g[V0][0] = c.lw(p(g(c, S0), 0x90));
                                c.g[V0][0] = sx(g(c, V0).wrapping_sub(0x3));
                                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[A0][0] == 0 { c.g[V0][0] = 0u64; }
                                let t = true;
                                c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                if t {
                                    break 'b_2e8b30;
                                } else {
                                    break 'b_2e89f4;
                                }
                            }
                        }
                        c.cycles += 31;
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x10));
                        c.mult(0, 5, 19, 5);
                        c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                        c.g[A2][0] = c.lw(p(g(c, S0), 0x18));
                        c.g[T1][0] = 0x30;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[A3][0] = sx(p(g(c, S0), 0x10));
                        c.g[T2][0] = 0x5;
                        c.g[V0][0] = sx(g(c, A1).wrapping_add(g(c, V0)));
                        c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                        c.lq(3, p(g(c, V0), 0x20));
                        c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                        c.g[T0][0] = c.g[A1][0];
                        c.sq(3, p(g(c, A0), 0x20));
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x1c));
                        c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A1)));
                        c.lq(3, p(g(c, V0), 0x20));
                        c.sq(3, p(g(c, A2), 0x20));
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                        c.g[V1][0] = c.lw(p(g(c, S0), 0x20));
                        c.g[V0][0] = sx(g(c, A1).wrapping_add(g(c, V0)));
                        c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                        c.lq(4, p(g(c, V0), 0x20));
                        c.g[V1][0] = sx(g(c, A1).wrapping_add(g(c, V1)));
                        c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                        c.sq(4, p(g(c, V1), 0x20));
                        c.g[V0][0] = c.lw(p(g(c, S0), 0x20));
                        c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A1)));
                        c.lq(3, p(g(c, V0), 0x20));
                        c.sq(3, p(g(c, A2), 0x20));
                        'l_2e8a70: loop {
                            c.cycles += 48;
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[T2][0] = sx(g(c, T2).wrapping_sub(0x1));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 4, 4, 9);
                            c.g[A1][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.lq(3, p(g(c, V0), 0x20));
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
                            c.sq(3, p(g(c, A0), 0x20));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V1][0] = c.lw(g(c, A3));
                            c.mult(0, 2, 2, 9);
                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                            c.swc1(0, p(g(c, V0), 0x2c));
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 3, 3, 9);
                            c.g[A0][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.sw(p(g(c, V0), 0x1c), 0u64);
                            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                            c.sw(p(g(c, V1), 0x1c), 0u64);
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 4, 4, 9);
                            c.g[V1][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.g[A1][0] = c.lw(p(g(c, V0), 0x10));
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                            c.sw(p(g(c, A0), 0x10), c.g[A1][0]);
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 3, 3, 9);
                            c.g[A0][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.g[A1][0] = c.lw(p(g(c, V0), 0x14));
                            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                            c.sw(p(g(c, V1), 0x14), c.g[A1][0]);
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 4, 4, 9);
                            c.g[A1][0] = c.lw(g(c, A3));
                            c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                            c.g[A3][0] = sx(p(g(c, A3), 0x4));
                            c.g[V1][0] = c.lw(p(g(c, V0), 0x18));
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
                            let t = (c.g[T2][0] as i64) >= 0;
                            c.sw(p(g(c, A0), 0x18), c.g[V1][0]);
                            if t {
                                if event_due(c) { deliver_event_here(c, 0x002e8a70); } continue 'l_2e8a70;
                            } else {
                                break 'b_2e8b30;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.sw(p(g(c, S0), 0x94), 0u64);
                    if t {
                        break 'b_2ea440;
                    } else {
                        break 'b_2e8b38;
                    }
                }
            }
        }
        // `cBoardTrailFX_IsActive` said yes: pick up the fade it stores at
        // `self+0x9c`; a fresh trail (state 0) also advances the ring head.
        'b_2e8b78: {
            c.cycles += 2;
            if c.g[V0][0] != 0u64 {
                c.lwc1(1, p(g(c, S0), 0x9c));
                break 'b_2e8b78;
            } else {
                'b_2e8b50: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                    c.g[V1][0] = 0x36;
                    if c.g[V1][0] == 0u64 {
                        break 'b_2e8b50;
                    } else {
                        break 'b_2e8b50;
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = 0x36;
                c.g[V0][0] = sx(p(g(c, V0), 0x3));
                c.div(0, 2, 17);
                c.g[V1][0] = c.hi[0];
                c.g[A0][0] = sx(p(g(c, V1), 0x35));
                c.sw(p(g(c, S0), 0x8c), c.g[V1][0]);
                c.div(0, 4, 17);
                c.g[V0][0] = c.hi[0];
                c.g[S3][0] = c.g[V0][0];
                c.lwc1(1, p(g(c, S0), 0x9c));
                break 'b_2e8b78;
            }
        }
        // Rebuild the current vertex from the rider's board frame (`rider+0x370`
        // / `+0x460`) and the animation entry
        // `*(*(rider+0x780)+0x34) + (*(rider+0x8a4) << 6)`; the
        // dot products and clamps below blend its colour/width against the gp
        // float constants (f20-f27 keep the intermediate results).
        'b_2e8f1c: {
            'b_2e8e3c: {
                'b_2e8c7c: {
                    c.cycles += 51;
                    c.g[S2][0] = sx(frame.at(c, 0x40));
                    c.lwc1(2, gp(c, 0x149c));
                    c.g[S1][0] = sx(frame.at(c, 0x60));
                    c.lwc1(0, gp(c, 0x1498));
                    c.mul_s(1, 1, 2);
                    c.g[A2][0] = c.lw(g(c, S0));
                    c.g[AT][0] = 0x42af0000;
                    c.mtc1(1, 2);
                    c.g[AT][0] = 0x418c0000;
                    c.mtc1(1, 3);
                    c.sub_s(0, 0, 1);
                    c.swc1(0, p(g(c, S0), 0xa0));
                    c.g[V0][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8a4));
                    c.g[A0][0] = c.lw(p(g(c, V0), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x30));
                    c.sq(2, frame.at(c, 0x40));
                    c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8a4));
                    c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x20));
                    c.sq(2, frame.at(c, 0x50));
                    c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8a4));
                    c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, g(c, A0));
                    c.sq(2, frame.at(c, 0x60));
                    c.mfc1(3, 2);
                    c.lqc2(4, frame.at(c, 0x60));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x80));
                    c.lq(2, frame.at(c, 0x80));
                    c.sq(2, frame.at(c, 0x70));
                    c.mfc1(3, 3);
                    c.lqc2(4, frame.at(c, 0x50));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x90));
                    c.lq(2, frame.at(c, 0x90));
                    c.g[A0][0] = c.lw(g(c, S0));
                    c.sq(2, frame.at(c, 0x80));
                    call_ptr(c, 0x0011fe98, 0x002e8c3c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_2e8c7c;
                    } else {
                        'b_2e8c68: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.g[V0][0] = c.lw(p(g(c, V1), 0xac4));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[V1][0];
                            if t {
                                break 'b_2e8c68;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(p(g(c, A0), 0xad0));
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_2e8c68;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw(p(g(c, A0), 0xafc));
                                    c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
                                    break 'b_2e8c68;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] == 0u64 {
                            c.g[A2][0] = c.lw(g(c, S0));
                            break 'b_2e8e3c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(p(g(c, A0), 0xb00));
                            if c.g[V0][0] == 0u64 {
                                c.g[A2][0] = c.lw(g(c, S0));
                                break 'b_2e8e3c;
                            } else {
                                break 'b_2e8c7c;
                            }
                        }
                    }
                }
                'b_2e8d1c: {
                    c.cycles += 17;
                    c.lqc2(3, g(c, S2));
                    c.lqc2(4, frame.at(c, 0x70));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0xa0));
                    c.mtc1(0, 2);
                    c.lq(2, frame.at(c, 0xa0));
                    c.sq(2, frame.at(c, 0x90));
                    c.lqc2(3, g(c, S2));
                    c.lqc2(4, frame.at(c, 0x70));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0xb0));
                    c.lq(2, frame.at(c, 0xb0));
                    c.g[A0][0] = c.lw(g(c, S0));
                    c.sq(2, frame.at(c, 0xa0));
                    c.g[V1][0] = c.lw(p(g(c, A0), 0x330));
                    let t = c.g[V1][0] == 0u64;

                    if t {
                        break 'b_2e8d1c;
                    } else {
                        'b_2e8d00: {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(p(g(c, A0), 0x784));
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x00312aa0, 0x002e8ccc)?;
                            c.g[V0][0] = sx(g(c, V0).wrapping_sub(0x17));
                            c.g[V0][0] = (c.g[V0][0] < 0x8u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lw(g(c, S0));
                            if t {
                                break 'b_2e8d00;
                            } else {
                                c.cycles += 5;
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 2);
                                c.g[V0][0] = c.lw(p(g(c, V1), 0x320));
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_2e8d1c;
                                } else {
                                    c.cycles += 4;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    let t = true;

                                    if t {
                                        break 'b_2e8d1c;
                                    } else {
                                        break 'b_2e8d00;
                                    }
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 2);
                        c.g[V0][0] = c.lw(p(g(c, V1), 0x320));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_2e8d1c;
                        } else {
                            c.cycles += 2;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 2);
                            break 'b_2e8d1c;
                        }
                    }
                }
                'b_2e8dc8: {
                    'b_2e8d78: {
                        c.cycles += 4;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 2);
                        let t = !c.fcc();

                        if t {
                            break 'b_2e8d78;
                        } else {
                            c.cycles += 19;
                            c.g[AT][0] = 0x432f0000;
                            c.mtc1(1, 0);
                            c.g[AT][0] = 0x430c0000;
                            c.mtc1(1, 1);
                            c.mul_s(0, 2, 0);
                            c.min_s(0, 0, 1);
                            c.mfc1(3, 0);
                            c.lqc2(4, g(c, S1));
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0xc0));
                            c.lq(2, frame.at(c, 0xc0));
                            c.sq(2, frame.at(c, 0xb0));
                            c.lqc2(3, frame.at(c, 0x90));
                            c.lqc2(4, frame.at(c, 0xb0));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x90));
                            let t = true;

                            if t {
                                break 'b_2e8dc8;
                            } else {
                                break 'b_2e8d78;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.c_lt_s(2, 0);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e8dc8;
                    } else {
                        c.cycles += 17;
                        c.g[AT][0] = 0x432f0000;
                        c.mtc1(1, 0);
                        c.g[AT][0] = 0xffffffffc30c0000;
                        c.mtc1(1, 1);
                        c.mul_s(0, 2, 0);
                        c.max_s(0, 0, 1);
                        c.mfc1(3, 0);
                        c.lqc2(4, g(c, S1));
                        c.qmtc2(3, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, frame.at(c, 0xc0));
                        c.lq(2, frame.at(c, 0xc0));
                        c.sq(2, frame.at(c, 0xb0));
                        c.lqc2(3, frame.at(c, 0xa0));
                        c.lqc2(4, frame.at(c, 0xb0));
                        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                        c.sqc2(5, frame.at(c, 0xa0));
                        break 'b_2e8dc8;
                    }
                }
                c.cycles += 29;
                c.lqc2(3, frame.at(c, 0xa0));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xb0));
                c.sq(2, frame.at(c, 0));
                c.lqc2(3, frame.at(c, 0x90));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xb0));
                c.sq(2, frame.at(c, 0x20));
                c.lqc2(3, frame.at(c, 0xa0));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xb0));
                c.sq(2, frame.at(c, 0x10));
                c.lqc2(3, frame.at(c, 0x90));
                c.lqc2(4, frame.at(c, 0x80));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0x30));
                let t = true;
                c.sq(2, frame.at(c, 0xb0));
                if t {
                    break 'b_2e8f1c;
                } else {
                    break 'b_2e8e3c;
                }
            }
            'b_2e8ecc: {
                c.cycles += 19;
                c.g[V0][0] = c.lw(p(g(c, A2), 0x780));
                c.g[V1][0] = c.lw(p(g(c, A2), 0x8b0));
                c.g[A0][0] = c.lw(p(g(c, V0), 0x34));
                c.g[V1][0] = sx(g(c, V1) << 6);
                c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                c.lq(2, p(g(c, A0), 0x30));
                c.sq(2, frame.at(c, 0x90));
                c.sq(2, frame.at(c, 0));
                c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                c.g[V1][0] = c.lw(p(g(c, A2), 0x8b8));
                c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                c.g[V1][0] = sx(g(c, V1) << 6);
                c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                c.lq(2, p(g(c, A0), 0x30));
                c.sq(2, frame.at(c, 0x10));
                c.sq(2, frame.at(c, 0x90));
                c.g[V1][0] = c.lw(p(g(c, A2), 0x150));
                let t = c.g[V1][0] == 0u64;

                if t {
                    break 'b_2e8ecc;
                } else {
                    c.cycles += 17;
                    c.g[V0][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x918));
                    c.g[A0][0] = c.lw(p(g(c, V0), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x30));
                    c.sq(2, frame.at(c, 0x90));
                    c.sq(2, frame.at(c, 0x20));
                    c.g[A1][0] = c.lw(p(g(c, A2), 0x780));
                    c.g[V1][0] = c.lw(p(g(c, A2), 0x8e8));
                    c.g[A0][0] = c.lw(p(g(c, A1), 0x34));
                    c.g[V1][0] = sx(g(c, V1) << 6);
                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                    c.lq(2, p(g(c, A0), 0x30));
                    c.sq(2, frame.at(c, 0x30));
                    let t = true;
                    c.sq(2, frame.at(c, 0x90));
                    if t {
                        break 'b_2e8f1c;
                    } else {
                        break 'b_2e8ecc;
                    }
                }
            }
            c.cycles += 20;
            c.lqc2(3, g(c, S2));
            c.lqc2(4, frame.at(c, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xa0));
            c.lq(2, frame.at(c, 0xa0));
            c.sq(2, frame.at(c, 0x90));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0x80));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0xa0));
            c.sq(2, frame.at(c, 0x20));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0x80));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0x30));
            c.sq(2, frame.at(c, 0xa0));
            break 'b_2e8f1c;
        }
        'b_2e8f8c: {
            'b_2e8f64: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(g(c, S0));
                c.lwc1(0, gp(c, -0x39cc));
                c.lqc2(3, p(g(c, V1), 0x1e0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(2, 22);
                c.mtc1(2, 26);
                c.c_lt_s(26, 0);
                let t = !c.fcc();

                if t {
                    break 'b_2e8f64;
                } else {
                    c.cycles += 3;
                    c.lq(2, p(g(c, S0), 0xc0));
                    let t = true;
                    c.sq(2, frame.at(c, 0x90));
                    if t {
                        break 'b_2e8f8c;
                    } else {
                        break 'b_2e8f64;
                    }
                }
            }
            c.cycles += 10;
            c.mfc1(2, 26);
            c.qmtc2(2, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, p(g(c, V1), 0x1e0));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0x90));
            c.sq(2, frame.at(c, 0xa0));
            break 'b_2e8f8c;
        }
        'b_2e90a4: {
            c.cycles += 59;
            c.g[V1][0] = c.lw(g(c, S0));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(5, p(g(c, V1), 0x370));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.mtc1(0, 1);
            c.mfc1(4, 0);
            c.lqc2(4, p(g(c, V1), 0x370));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0xd0));
            c.lq(2, frame.at(c, 0xd0));
            c.sq(2, frame.at(c, 0xc0));
            c.lqc2(3, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0xc0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0xd0));
            c.lq(2, frame.at(c, 0xd0));
            c.sq(2, frame.at(c, 0xb0));
            c.lqc2(3, frame.at(c, 0xb0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, frame.at(c, 0xd0));
            c.lq(2, frame.at(c, 0xd0));
            c.g[V1][0] = c.lw(g(c, S0));
            c.sq(2, frame.at(c, 0xa0));
            c.sq(2, frame.at(c, 0x90));
            c.lqc2(4, frame.at(c, 0x90));
            c.lqc2(5, p(g(c, V1), 0x370));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, frame.at(c, 0xb0));
            c.lq(2, frame.at(c, 0xb0));
            c.sq(2, frame.at(c, 0xa0));
            c.lqc2(3, frame.at(c, 0xa0));
            c.lqc2(5, p(g(c, S0), 0xd0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.c_lt_s(0, 1);
            if !c.fcc() {
                c.g[V0][0] = c.lw(g(c, S0));
                break 'b_2e90a4;
            } else {
                c.cycles += 11;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 0);
                c.mfc1(3, 0);
                c.lqc2(4, frame.at(c, 0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(2, frame.at(c, 0xc0));
                c.sq(2, frame.at(c, 0xa0));
                c.sq(2, frame.at(c, 0xb0));
                c.g[V0][0] = c.lw(g(c, S0));
                break 'b_2e90a4;
            }
        }
        'b_2e93a0: {
            'b_2e9228: {
                c.cycles += 66;
                c.lqc2(3, frame.at(c, 0x10));
                c.lqc2(4, p(g(c, V0), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xc0));
                c.lq(3, frame.at(c, 0xc0));
                c.g[A0][0] = c.lw(g(c, S0));
                c.sq(3, frame.at(c, 0xb0));
                c.lqc2(3, frame.at(c, 0));
                c.lqc2(4, p(g(c, A0), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xd0));
                c.lq(2, frame.at(c, 0xd0));
                c.g[V1][0] = c.lw(g(c, S0));
                c.sq(2, frame.at(c, 0xc0));
                c.lqc2(3, frame.at(c, 0x30));
                c.lqc2(4, p(g(c, V1), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xe0));
                c.lq(2, frame.at(c, 0xe0));
                c.g[V1][0] = c.lw(g(c, S0));
                c.sq(2, frame.at(c, 0xd0));
                c.lqc2(3, frame.at(c, 0x20));
                c.lqc2(4, p(g(c, V1), 0x460));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0xf0));
                c.lq(2, frame.at(c, 0xf0));
                c.sq(2, frame.at(c, 0xe0));
                c.lqc2(3, frame.at(c, 0xb0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 2);
                c.lqc2(3, frame.at(c, 0xc0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 3);
                c.c_lt_s(3, 2);
                c.lqc2(3, frame.at(c, 0xd0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 4);
                c.lqc2(3, frame.at(c, 0xe0));
                c.lqc2(5, frame.at(c, 0xa0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 5);
                let t = !c.fcc();

                if t {
                    break 'b_2e9228;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(4, 2);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e9228;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(5, 2);
                        let t = !c.fcc();
                        c.lwc1(1, gp(c, 0xa38));
                        if t {
                            break 'b_2e9228;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xb0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0x10));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e93a0;
                            } else {
                                break 'b_2e9228;
                            }
                        }
                    }
                }
            }
            'b_2e92b0: {
                c.cycles += 3;
                c.c_lt_s(2, 3);
                let t = !c.fcc();

                if t {
                    break 'b_2e92b0;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(4, 3);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e92b0;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(5, 3);
                        let t = !c.fcc();
                        c.lwc1(1, gp(c, 0xa38));
                        if t {
                            break 'b_2e92b0;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xc0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e93a0;
                            } else {
                                break 'b_2e92b0;
                            }
                        }
                    }
                }
            }
            'b_2e9344: {
                c.cycles += 4;
                c.c_lt_s(2, 4);
                if !c.fcc() {
                    c.g[V1][0] = c.lw(g(c, S0));
                    break 'b_2e9344;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(3, 4);
                    if !c.fcc() {
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e9344;
                    } else {
                        'b_2e9340: {
                            c.cycles += 4;
                            c.c_lt_s(5, 4);
                            let t = !c.fcc();
                            c.lwc1(1, gp(c, 0xa38));
                            if t {
                                break 'b_2e9340;
                            } else {
                                c.cycles += 24;
                                c.g[V1][0] = c.lw(g(c, S0));
                                c.lqc2(3, p(g(c, V1), 0x370));
                                c.lqc2(5, frame.at(c, 0xd0));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.qmfc2(2, 4);
                                c.mtc1(2, 0);
                                c.sub_s(0, 0, 1);
                                c.mfc1(4, 0);
                                c.lqc2(4, p(g(c, V1), 0x370));
                                c.qmtc2(4, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, frame.at(c, 0x130));
                                c.lq(2, frame.at(c, 0x130));
                                c.sq(2, frame.at(c, 0x120));
                                c.lqc2(3, frame.at(c, 0x30));
                                c.lqc2(4, frame.at(c, 0x120));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, frame.at(c, 0x130));
                                let t = true;
                                c.lq(2, frame.at(c, 0x130));
                                if t {
                                    break 'b_2e93a0;
                                } else {
                                    break 'b_2e9340;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e9344;
                    }
                }
            }
            c.cycles += 23;
            c.lwc1(1, gp(c, 0xa38));
            c.lqc2(3, p(g(c, V1), 0x370));
            c.lqc2(5, frame.at(c, 0xe0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.sub_s(0, 0, 1);
            c.mfc1(4, 0);
            c.lqc2(4, p(g(c, V1), 0x370));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            c.sq(2, frame.at(c, 0x120));
            c.lqc2(3, frame.at(c, 0x20));
            c.lqc2(4, frame.at(c, 0x120));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            break 'b_2e93a0;
        }
        'b_2e95a8: {
            'b_2e9430: {
                c.cycles += 5;
                c.sq(2, frame.at(c, 0xf0));
                c.c_lt_s(2, 3);
                let t = !c.fcc();
                c.sq(2, frame.at(c, 0x110));
                if t {
                    break 'b_2e9430;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(2, 4);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e9430;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(2, 5);
                        let t = !c.fcc();
                        c.lwc1(1, gp(c, 0xa38));
                        if t {
                            break 'b_2e9430;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xb0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0x10));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e95a8;
                            } else {
                                break 'b_2e9430;
                            }
                        }
                    }
                }
            }
            'b_2e94b8: {
                c.cycles += 3;
                c.c_lt_s(3, 2);
                let t = !c.fcc();

                if t {
                    break 'b_2e94b8;
                } else {
                    c.cycles += 3;
                    c.c_lt_s(3, 4);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e94b8;
                    } else {
                        c.cycles += 4;
                        c.c_lt_s(3, 5);
                        let t = !c.fcc();
                        c.lwc1(1, gp(c, 0xa38));
                        if t {
                            break 'b_2e94b8;
                        } else {
                            c.cycles += 24;
                            c.g[V1][0] = c.lw(g(c, S0));
                            c.lqc2(3, p(g(c, V1), 0x370));
                            c.lqc2(5, frame.at(c, 0xc0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.sub_s(0, 0, 1);
                            c.mfc1(4, 0);
                            c.lqc2(4, p(g(c, V1), 0x370));
                            c.qmtc2(4, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x130));
                            c.lq(2, frame.at(c, 0x130));
                            c.sq(2, frame.at(c, 0x120));
                            c.lqc2(3, frame.at(c, 0));
                            c.lqc2(4, frame.at(c, 0x120));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x130));
                            let t = true;
                            c.lq(2, frame.at(c, 0x130));
                            if t {
                                break 'b_2e95a8;
                            } else {
                                break 'b_2e94b8;
                            }
                        }
                    }
                }
            }
            'b_2e954c: {
                c.cycles += 4;
                c.c_lt_s(4, 2);
                if !c.fcc() {
                    c.g[V1][0] = c.lw(g(c, S0));
                    break 'b_2e954c;
                } else {
                    c.cycles += 4;
                    c.c_lt_s(4, 3);
                    if !c.fcc() {
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e954c;
                    } else {
                        'b_2e9548: {
                            c.cycles += 4;
                            c.c_lt_s(4, 5);
                            let t = !c.fcc();
                            c.lwc1(1, gp(c, 0xa38));
                            if t {
                                break 'b_2e9548;
                            } else {
                                c.cycles += 24;
                                c.g[V1][0] = c.lw(g(c, S0));
                                c.lqc2(3, p(g(c, V1), 0x370));
                                c.lqc2(5, frame.at(c, 0xd0));
                                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                                c.qmfc2(2, 4);
                                c.mtc1(2, 0);
                                c.sub_s(0, 0, 1);
                                c.mfc1(4, 0);
                                c.lqc2(4, p(g(c, V1), 0x370));
                                c.qmtc2(4, 3);
                                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                c.sqc2(5, frame.at(c, 0x130));
                                c.lq(2, frame.at(c, 0x130));
                                c.sq(2, frame.at(c, 0x120));
                                c.lqc2(3, frame.at(c, 0x30));
                                c.lqc2(4, frame.at(c, 0x120));
                                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                                c.sqc2(5, frame.at(c, 0x130));
                                let t = true;
                                c.lq(2, frame.at(c, 0x130));
                                if t {
                                    break 'b_2e95a8;
                                } else {
                                    break 'b_2e9548;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(g(c, S0));
                        break 'b_2e954c;
                    }
                }
            }
            c.cycles += 23;
            c.lwc1(1, gp(c, 0xa38));
            c.lqc2(3, p(g(c, V1), 0x370));
            c.lqc2(5, frame.at(c, 0xe0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.sub_s(0, 0, 1);
            c.mfc1(4, 0);
            c.lqc2(4, p(g(c, V1), 0x370));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            c.sq(2, frame.at(c, 0x120));
            c.lqc2(3, frame.at(c, 0x20));
            c.lqc2(4, frame.at(c, 0x120));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            break 'b_2e95a8;
        }
        'b_2e9794: {
            c.cycles += 58;
            c.sq(2, frame.at(c, 0x100));
            c.sq(2, frame.at(c, 0x110));
            c.g[V0][0] = c.lw(g(c, S0));
            c.lqc2(3, frame.at(c, 0xf0));
            c.lqc2(4, p(g(c, V0), 0x460));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x120));
            c.lq(3, frame.at(c, 0x120));
            c.g[A0][0] = c.lw(g(c, S0));
            c.sq(3, frame.at(c, 0x110));
            c.lqc2(3, frame.at(c, 0x100));
            c.lqc2(4, p(g(c, A0), 0x460));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x130));
            c.lq(2, frame.at(c, 0x130));
            c.g[A0][0] = c.lw(g(c, S0));
            c.sq(2, frame.at(c, 0x120));
            c.lqc2(3, frame.at(c, 0x110));
            c.lqc2(5, frame.at(c, 0x90));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 24);
            c.lqc2(3, frame.at(c, 0x120));
            c.lqc2(5, frame.at(c, 0x90));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 27);
            c.lqc2(3, frame.at(c, 0x110));
            c.lqc2(5, frame.at(c, 0xa0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 23);
            c.lqc2(3, frame.at(c, 0x120));
            c.lqc2(5, frame.at(c, 0xa0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 25);
            call_ptr(c, 0x0011fe98, 0x002e9684)?;
            c.g[V1][0] = 0x2;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                break 'b_2e9794;
            } else {
                c.cycles += 65;
                c.g[AT][0] = 0x3f400000;
                c.mtc1(1, 1);
                c.mul_s(0, 23, 1);
                c.mfc1(2, 0);
                c.lqc2(4, frame.at(c, 0xa0));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x160));
                c.mul_s(1, 25, 1);
                c.lq(2, frame.at(c, 0x160));
                c.g[V1][0] = c.lw(g(c, S0));
                c.sq(2, frame.at(c, 0x150));
                c.lqc2(3, p(g(c, V1), 0x460));
                c.lqc2(4, frame.at(c, 0x150));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x160));
                c.lq(2, frame.at(c, 0x160));
                c.lwc1(0, gp(c, 0xa38));
                c.sq(2, frame.at(c, 0x140));
                c.g[V1][0] = c.lw(g(c, S0));
                c.mfc1(4, 0);
                c.lqc2(4, p(g(c, V1), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0x160));
                c.lqc2(3, frame.at(c, 0x140));
                c.lqc2(4, frame.at(c, 0x160));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0xf0));
                c.sq(2, frame.at(c, 0x130));
                c.mfc1(3, 1);
                c.lqc2(4, frame.at(c, 0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x160));
                c.lq(2, frame.at(c, 0x160));
                c.sq(2, frame.at(c, 0x150));
                c.g[V1][0] = c.lw(g(c, S0));
                c.lqc2(3, p(g(c, V1), 0x460));
                c.lqc2(4, frame.at(c, 0x150));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x160));
                c.lq(2, frame.at(c, 0x160));
                c.lwc1(0, gp(c, 0xa38));
                c.sq(2, frame.at(c, 0x140));
                c.g[V1][0] = c.lw(g(c, S0));
                c.mfc1(4, 0);
                c.lqc2(4, p(g(c, V1), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0x160));
                c.lqc2(3, frame.at(c, 0x140));
                c.lqc2(4, frame.at(c, 0x160));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x170));
                c.lq(2, frame.at(c, 0x170));
                c.sq(2, frame.at(c, 0x100));
                c.sq(2, frame.at(c, 0x130));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                break 'b_2e9794;
            }
        }
        // Scale and clamp the blended value with the gp float constants at
        // -0x39ac..-0x39cc.
        'b_2e98a0: {
            'b_2e9864: {
                c.cycles += 41;
                c.g[A1][0] = 0x30;
                c.g[V0][0] = c.lw(p(g(c, S0), 0x18));
                c.abs_s(21, 23);
                c.mult(0, 3, 3, 5);
                c.lq(4, frame.at(c, 0xf0));
                c.mtc1(0, 22);
                c.abs_s(20, 24);
                c.c_eq_s(21, 22);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, V0)));
                c.sq(4, p(g(c, V1), 0x20));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                c.mult(0, 2, 2, 5);
                c.lq(4, frame.at(c, 0xf0));
                c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                c.sq(4, p(g(c, V0), 0x20));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x1c));
                c.mult(0, 3, 3, 5);
                c.lq(4, frame.at(c, 0xf0));
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, V0)));
                c.sq(4, p(g(c, V1), 0x20));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x20));
                c.mult(0, 2, 2, 5);
                c.lq(4, frame.at(c, 0x100));
                c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                c.sq(4, p(g(c, V0), 0x20));
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                c.mult(0, 3, 3, 5);
                c.lq(4, frame.at(c, 0x100));
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, V0)));
                c.sq(4, p(g(c, V1), 0x20));
                c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[A0][0] = c.lw(p(g(c, S0), 0x24));
                c.mult(0, 2, 2, 5);
                c.lq(3, frame.at(c, 0x100));
                c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                let t = !c.fcc();
                c.sq(3, p(g(c, V0), 0x20));
                if t {
                    break 'b_2e9864;
                } else {
                    c.cycles += 4;
                    c.c_eq_s(20, 22);
                    let t = c.fcc();
                    c.mov_s(12, 20);
                    if t {
                        break 'b_2e98a0;
                    } else {
                        c.cycles += 4;
                        c.c_le_s(22, 20);
                        c.lwc1(12, gp(c, -0x39c8));
                        let t = !c.fcc();

                        if t {
                            break 'b_2e98a0;
                        } else {
                            c.cycles += 3;
                            c.lwc1(12, gp(c, -0x39c4));
                            let t = true;

                            if t {
                                break 'b_2e98a0;
                            } else {
                                break 'b_2e9864;
                            }
                        }
                    }
                }
            }
            c.cycles += 7;
            c.div_s(12, 20, 21);
            call_ptr(c, 0x0031c228, 0x002e9870)?;
            c.c_lt_s(21, 22);
            let t = !c.fcc();
            c.mov_s(12, 0);
            if t {
                break 'b_2e98a0;
            } else {
                'b_2e9898: {
                    c.cycles += 3;
                    c.c_lt_s(22, 20);
                    let t = !c.fcc();

                    if t {
                        break 'b_2e9898;
                    } else {
                        c.cycles += 3;
                        c.lwc1(0, gp(c, -0x39c0));
                        let t = true;
                        c.add_s(12, 12, 0);
                        if t {
                            break 'b_2e98a0;
                        } else {
                            break 'b_2e9898;
                        }
                    }
                }
                c.cycles += 2;
                c.lwc1(0, gp(c, -0x39bc));
                c.sub_s(12, 12, 0);
                break 'b_2e98a0;
            }
        }
        'b_2e9914: {
            'b_2e98e8: {
                c.cycles += 4;
                c.lwc1(0, gp(c, -0x39b8));
                c.c_lt_s(0, 12);
                let t = !c.fcc();

                if t {
                    break 'b_2e98e8;
                } else {
                    c.cycles += 12;
                    c.lwc1(0, gp(c, -0x39b4));
                    c.g[AT][0] = 0x40000000;
                    c.mtc1(1, 2);
                    c.mul_s(0, 12, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.mtc1(0, 1);
                    c.sub_s(0, 2, 0);
                    c.c_le_s(1, 0);
                    let t = !c.fcc();
                    c.mov_s(2, 1);
                    if t {
                        break 'b_2e9914;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.min_s(2, 0, 3);
                        if t {
                            break 'b_2e9914;
                        } else {
                            break 'b_2e98e8;
                        }
                    }
                }
            }
            'b_2e9910: {
                c.cycles += 9;
                c.lwc1(0, gp(c, -0x39b0));
                c.mtc1(0, 1);
                c.mul_s(0, 12, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 2);
                c.c_le_s(1, 0);
                let t = !c.fcc();
                c.mov_s(3, 1);
                if t {
                    break 'b_2e9910;
                } else {
                    c.cycles += 1;
                    c.min_s(3, 0, 2);
                    break 'b_2e9910;
                }
            }
            c.cycles += 1;
            c.mov_s(2, 3);
            break 'b_2e9914;
        }
        'b_2e9934: {
            c.cycles += 8;
            c.lwc1(0, gp(c, -0x39ac));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.mul_s(0, 26, 0);
            c.c_le_s(0, 1);
            if c.fcc() {
                c.mul_s(2, 2, 0);
                break 'b_2e9934;
            } else {
                break 'b_2e9934;
            }
        }
        'b_2e9988: {
            'b_2e9984: {
                c.cycles += 18;
                c.g[V0][0] = c.lw(g(c, S0));
                c.c_lt_s(24, 27);
                c.lwc1(0, gp(c, 0xa38));
                c.mfc1(4, 0);
                c.lqc2(4, p(g(c, V0), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x150));
                c.lq(3, frame.at(c, 0x150));
                c.g[A0][0] = c.lw(g(c, S0));
                c.sq(3, frame.at(c, 0x140));
                c.lqc2(3, p(g(c, A0), 0x460));
                c.lqc2(4, frame.at(c, 0x140));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x150));
                c.lq(2, frame.at(c, 0x150));
                let t = !c.fcc();
                c.sq(2, frame.at(c, 0x130));
                if t {
                    break 'b_2e9984;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.mul_s(0, 2, 24);
                    if t {
                        break 'b_2e9988;
                    } else {
                        break 'b_2e9984;
                    }
                }
            }
            c.cycles += 1;
            c.mul_s(0, 2, 27);
            break 'b_2e9988;
        }
        c.cycles += 242;
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x90));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x170));
        c.lq(2, frame.at(c, 0x170));
        c.sq(2, frame.at(c, 0x160));
        c.lqc2(3, frame.at(c, 0x130));
        c.lqc2(4, frame.at(c, 0x160));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x170));
        c.lq(2, frame.at(c, 0x170));
        c.sq(2, frame.at(c, 0x140));
        c.sq(2, frame.at(c, 0x150));
        c.mfc1(2, 23);
        c.lqc2(4, frame.at(c, 0xa0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x170));
        c.mtc1(0, 20);
        c.lq(2, frame.at(c, 0x170));
        c.g[A2][0] = 0x30;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 1);
        c.mult(0, 6, 19, 6);
        c.sq(2, frame.at(c, 0x160));
        c.g[S1][0] = sx(p(g(c, S0), 0x10));
        c.lqc2(3, frame.at(c, 0x140));
        c.lqc2(4, frame.at(c, 0x160));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x170));
        c.lq(2, frame.at(c, 0x170));
        c.sq(2, frame.at(c, 0x150));
        c.mfc1(3, 25);
        c.lqc2(4, frame.at(c, 0xa0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x180));
        c.lq(2, frame.at(c, 0x180));
        c.sq(2, frame.at(c, 0x170));
        c.lqc2(3, frame.at(c, 0x140));
        c.lqc2(4, frame.at(c, 0x170));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x180));
        c.lq(2, frame.at(c, 0x180));
        c.sq(2, frame.at(c, 0x160));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x150));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x190));
        c.lq(2, frame.at(c, 0x190));
        c.sq(2, frame.at(c, 0x180));
        c.lqc2(3, frame.at(c, 0x180));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, frame.at(c, 0x190));
        c.lq(2, frame.at(c, 0x190));
        c.sq(2, frame.at(c, 0x170));
        c.lwc1(0, gp(c, 0x1490));
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x170));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x190));
        c.lq(2, frame.at(c, 0x190));
        c.lwc1(0, gp(c, 0x1494));
        c.sq(2, frame.at(c, 0x180));
        c.mfc1(3, 0);
        c.lqc2(4, frame.at(c, 0x170));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1b0));
        c.lq(2, frame.at(c, 0x1b0));
        c.g[V1][0] = c.lw(g(c, S0));
        c.sq(2, frame.at(c, 0x1a0));
        c.lwc1(0, p(g(c, V1), 0x454));
        c.min_s(0, 0, 20);
        c.abs_s(0, 0);
        c.add_s(0, 0, 1);
        c.mfc1(2, 0);
        c.lqc2(4, frame.at(c, 0x1a0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1b0));
        c.lq(3, frame.at(c, 0x1b0));
        c.lwc1(0, p(g(c, S0), 0xa0));
        c.sq(3, frame.at(c, 0x190));
        c.g[V0][0] = c.lw(g(c, S0));
        c.mfc1(4, 0);
        c.lqc2(4, p(g(c, V0), 0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1b0));
        c.lq(3, frame.at(c, 0x1b0));
        c.g[V0][0] = c.lw(g(c, S0));
        c.sq(3, frame.at(c, 0x1a0));
        c.lwc1(0, p(g(c, V0), 0x454));
        c.min_s(0, 0, 20);
        c.mfc1(4, 0);
        c.lqc2(4, p(g(c, V0), 0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1c0));
        c.lq(3, frame.at(c, 0x1c0));
        c.g[A0][0] = c.lw(p(g(c, S0), 0x18));
        c.sq(3, frame.at(c, 0x1b0));
        c.lqc2(3, frame.at(c, 0x150));
        c.lqc2(4, frame.at(c, 0x190));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.g[A0][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x1e0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A0), 0x20));
        c.g[A1][0] = c.lw(p(g(c, S0), 0x10));
        c.lwc1(0, p(g(c, S0), 0x4));
        c.mfc1(2, 0);
        c.lqc2(4, frame.at(c, 0x180));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.g[A1][0] = sx(g(c, A2).wrapping_add(g(c, A1)));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x150));
        c.lqc2(4, frame.at(c, 0x1e0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1a0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A1), 0x20));
        c.g[V1][0] = c.lw(p(g(c, S0), 0x1c));
        c.lqc2(3, frame.at(c, 0x150));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1d0));
        c.lq(2, frame.at(c, 0x1d0));
        c.g[V1][0] = sx(g(c, A2).wrapping_add(g(c, V1)));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, V1), 0x20));
        c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1d0));
        c.lq(2, frame.at(c, 0x1d0));
        c.g[A0][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A0), 0x20));
        c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
        c.lwc1(0, p(g(c, S0), 0x8));
        c.mfc1(2, 0);
        c.lqc2(4, frame.at(c, 0x180));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.g[V1][0] = sx(g(c, A2).wrapping_add(g(c, V1)));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x1e0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1a0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, V1), 0x20));
        c.g[A0][0] = c.lw(p(g(c, S0), 0x24));
        c.lqc2(3, frame.at(c, 0x160));
        c.lqc2(4, frame.at(c, 0x190));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.g[A2][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
        c.sq(2, frame.at(c, 0x1e0));
        c.lqc2(3, frame.at(c, 0x1e0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1d0));
        c.lqc2(3, frame.at(c, 0x1d0));
        c.lqc2(4, frame.at(c, 0x1b0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x1f0));
        c.lq(2, frame.at(c, 0x1f0));
        c.sq(2, frame.at(c, 0x1c0));
        c.sq(2, p(g(c, A2), 0x20));
        c.g[A0][0] = c.lw(g(c, S0));
        c.g[V1][0] = c.lw(p(g(c, A0), 0x6c0));
        c.g[A0][0] = sx(p(g(c, A0), 0x6c0));
        c.g[A1][0] = c.lh(p(g(c, V1), 0x38));
        c.g[V0][0] = c.lw(p(g(c, V1), 0x3c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
        call_ptr(c, t, 0x002e9d08)?;
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x500000;
        c.mult(0, 2, 2, 4);
        c.g[V1][0] = sx(g(c, V1).wrapping_sub(0x5c68));
        c.g[AT][0] = 0x437f0000;
        c.mtc1(1, 1);
        c.g[T2][0] = 0u64;
        c.g[T0][0] = c.g[S1][0];
        c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
        c.ldl(3, p(g(c, V0), 0x7));
        c.ldr(3, g(c, V0));
        c.ldl(4, p(g(c, V0), 0xf));
        c.ldr(4, p(g(c, V0), 0x8));
        c.sdl(3, frame.at(c, 0x1c7));
        c.sdr(3, frame.at(c, 0x1c0));
        c.sdl(4, frame.at(c, 0x1cf));
        c.sdr(4, frame.at(c, 0x1c8));
        // Shift the six strips down by one vertex (`strip[k]` takes the head
        // vertex of `strip[k+1]`), then store the seam value at +0x2c in all six.
        'l_2e9d50: loop {
            'b_2e9d6c: {
                c.cycles += 6;
                c.lwc1(0, frame.at(c, 0x1c0));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9d6c;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9d6c;
                }
            }
            'b_2e9db4: {
                c.cycles += 17;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.g[T1][0] = 0x30;
                c.mult(0, 7, 19, 9);
                c.cvt_w_s(0, 2);
                c.mfc1(4, 0);
                c.mult(0, 3, 3, 9);
                c.g[A1][0] = c.lw(g(c, T0));
                c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A1)));
                c.sw(p(g(c, V0), 0x1c), c.g[A0][0]);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A1)));
                c.sw(p(g(c, V1), 0x1c), c.g[A0][0]);
                c.lwc1(0, frame.at(c, 0x1c4));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9db4;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9db4;
                }
            }
            'b_2e9df4: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.cvt_w_s(0, 2);
                c.mfc1(5, 0);
                c.g[A0][0] = c.lw(g(c, T0));
                c.mult(0, 3, 3, 9);
                c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V0), 0x10), c.g[A1][0]);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V1), 0x10), c.g[A1][0]);
                c.lwc1(0, frame.at(c, 0x1c8));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9df4;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9df4;
                }
            }
            'b_2e9e34: {
                c.cycles += 15;
                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                c.cvt_w_s(0, 2);
                c.mfc1(5, 0);
                c.g[A0][0] = c.lw(g(c, T0));
                c.mult(0, 3, 3, 9);
                c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V0), 0x14), c.g[A1][0]);
                c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                c.sw(p(g(c, V1), 0x14), c.g[A1][0]);
                c.lwc1(0, frame.at(c, 0x1cc));
                c.mul_s(0, 0, 1);
                c.c_le_s(20, 0);
                let t = !c.fcc();
                c.mov_s(2, 20);
                if t {
                    break 'b_2e9e34;
                } else {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                    break 'b_2e9e34;
                }
            }
            c.cycles += 13;
            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
            c.cvt_w_s(0, 2);
            c.mfc1(6, 0);
            c.g[A1][0] = c.lw(g(c, T0));
            c.g[T2][0] = sx(p(g(c, T2), 0x1));
            c.mult(0, 3, 3, 9);
            c.g[T0][0] = sx(p(g(c, T0), 0x4));
            c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A1)));
            c.g[A0][0] = ((c.g[T2][0] as i64) < 6i64) as u64;
            c.sw(p(g(c, V0), 0x18), c.g[A2][0]);
            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A1)));
            let t = c.g[A0][0] != 0u64;
            c.sw(p(g(c, V1), 0x18), c.g[A2][0]);
            if t {
                if event_due(c) { deliver_event_here(c, 0x002e9d50); } continue 'l_2e9d50;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A2][0] = 0x30;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x5;
                'l_2e9e80: loop {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                    c.g[A1][0] = sx(g(c, A1).wrapping_sub(0x1));
                    c.g[V1][0] = c.lw(g(c, A0));
                    c.mult(0, 2, 2, 6);
                    c.g[A0][0] = sx(p(g(c, A0), 0x4));
                    c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                    let t = (c.g[A1][0] as i64) >= 0;
                    c.swc1(0, p(g(c, V0), 0x2c));
                    if t {
                        if event_due(c) { deliver_event_here(c, 0x002e9e80); } continue 'l_2e9e80;
                    } else {
                        'b_2e9f3c: {
                            c.cycles += 39;
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[T0][0] = 0x30;
                            c.mult(0, 9, 19, 8);
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                            c.mult(0, 2, 2, 8);
                            c.mtc1(0, 1);
                            c.g[A2][0] = 0x1;
                            c.g[A0][0] = sx(g(c, T1).wrapping_add(g(c, V1)));
                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, V1)));
                            c.lqc2(3, p(g(c, V0), 0x20));
                            c.lqc2(4, p(g(c, A0), 0x20));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x1e0));
                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                            c.g[A0][0] = c.lw(p(g(c, S0), 0x14));
                            c.mult(0, 3, 3, 8);
                            c.lq(2, frame.at(c, 0x1e0));
                            c.g[A1][0] = sx(g(c, T1).wrapping_add(g(c, A0)));
                            c.sq(2, frame.at(c, 0x1d0));
                            c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                            c.lqc2(3, p(g(c, V1), 0x20));
                            c.lqc2(4, p(g(c, A1), 0x20));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, frame.at(c, 0x1f0));
                            c.lq(2, frame.at(c, 0x1f0));
                            c.sq(2, frame.at(c, 0x1e0));
                            c.lqc2(3, frame.at(c, 0x1d0));
                            c.lqc2(5, frame.at(c, 0x90));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(3, 4);
                            c.mtc1(3, 0);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.g[A2][0] = 0u64;
                                break 'b_2e9f3c;
                            } else {
                                break 'b_2e9f3c;
                            }
                        }
                        'b_2e9f74: {
                            c.cycles += 13;
                            c.lqc2(3, frame.at(c, 0x1e0));
                            c.lqc2(5, frame.at(c, 0x90));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 0);
                            c.c_lt_s(0, 1);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_2e9f74;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_2e9f74;
                            }
                        }
                        'b_2ea138: {
                            'b_2ea124: {
                                'b_2ea100: {
                                    'b_2ea0cc: {
                                        c.cycles += 2;
                                        let t = c.g[A2][0] == 0u64;

                                        if t {
                                            break 'b_2ea0cc;
                                        } else {
                                            'b_2ea078: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lwc1(0, gp(c, 0xa3c));
                                                if t {
                                                    break 'b_2ea078;
                                                } else {
                                                    c.cycles += 61;
                                                    c.g[A3][0] = c.lw(p(g(c, S0), 0x10));
                                                    c.g[A2][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A1][0] = c.lw(p(g(c, S0), 0x18));
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                                    c.mult(0, 6, 6, 8);
                                                    c.mfc1(3, 0);
                                                    c.lqc2(4, frame.at(c, 0x90));
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    c.g[A0][0] = sx(g(c, A2).wrapping_add(g(c, A0)));
                                                    c.g[V1][0] = sx(g(c, T1).wrapping_add(g(c, V1)));
                                                    c.g[A1][0] = sx(g(c, A2).wrapping_add(g(c, A1)));
                                                    c.sq(2, frame.at(c, 0x200));
                                                    c.g[A2][0] = sx(g(c, A2).wrapping_add(g(c, A3)));
                                                    c.lqc2(3, p(g(c, V1), 0x20));
                                                    c.lqc2(4, frame.at(c, 0x200));
                                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    c.lwc1(0, gp(c, 0xa3c));
                                                    c.sq(2, frame.at(c, 0x1f0));
                                                    c.sq(2, p(g(c, A0), 0x20));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                                    c.mult(0, 3, 3, 8);
                                                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                                    c.lq(2, p(g(c, A0), 0x20));
                                                    c.sq(2, p(g(c, A1), 0x20));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x18));
                                                    c.mult(0, 3, 3, 8);
                                                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                                    c.lq(2, p(g(c, A0), 0x20));
                                                    c.sq(2, p(g(c, A2), 0x20));
                                                    c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                                    c.g[A3][0] = c.lw(p(g(c, S0), 0x14));
                                                    c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                                                    c.mult(0, 5, 5, 8);
                                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
                                                    c.mfc1(3, 0);
                                                    c.lqc2(4, frame.at(c, 0x90));
                                                    c.qmtc2(3, 3);
                                                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                                    c.g[V1][0] = sx(g(c, T1).wrapping_add(g(c, V1)));
                                                    c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                                                    c.sq(2, frame.at(c, 0x200));
                                                    c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A3)));
                                                    c.lqc2(3, p(g(c, V1), 0x20));
                                                    c.lqc2(4, frame.at(c, 0x200));
                                                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                                                    c.sqc2(5, frame.at(c, 0x210));
                                                    c.lq(2, frame.at(c, 0x210));
                                                    let t = true;
                                                    c.sq(2, frame.at(c, 0x1f0));
                                                    if t {
                                                        break 'b_2ea100;
                                                    } else {
                                                        break 'b_2ea078;
                                                    }
                                                }
                                            }
                                            c.cycles += 21;
                                            c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                            c.mult(0, 5, 5, 8);
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.g[A3][0] = c.lw(p(g(c, S0), 0x10));
                                            c.g[A2][0] = c.lw(p(g(c, S0), 0x18));
                                            c.g[V1][0] = sx(g(c, A1).wrapping_add(g(c, V1)));
                                            c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                            c.lq(2, p(g(c, V1), 0x20));
                                            c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                                            c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A3)));
                                            c.sq(2, p(g(c, A0), 0x20));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.mult(0, 3, 3, 8);
                                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                            c.lq(2, p(g(c, A0), 0x20));
                                            c.sq(2, p(g(c, A2), 0x20));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                            let t = true;
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x18));
                                            if t {
                                                break 'b_2ea124;
                                            } else {
                                                break 'b_2ea0cc;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw(p(g(c, S0), 0x94));
                                        break 'b_2ea138;
                                    } else {
                                        c.cycles += 11;
                                        c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                        c.g[V1][0] = c.lw(p(g(c, S0), 0x10));
                                        c.mult(0, 5, 5, 8);
                                        c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
                                        c.g[A3][0] = c.lw(p(g(c, S0), 0x14));
                                        c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                                        c.g[V1][0] = sx(g(c, A1).wrapping_add(g(c, V1)));
                                        c.g[A0][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                        c.lq(2, p(g(c, V1), 0x20));
                                        c.g[A2][0] = sx(g(c, A1).wrapping_add(g(c, A2)));
                                        c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A3)));
                                        break 'b_2ea100;
                                    }
                                }
                                c.cycles += 9;
                                c.sq(2, p(g(c, A0), 0x20));
                                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                c.g[A0][0] = c.lw(p(g(c, S0), 0x20));
                                c.mult(0, 3, 3, 8);
                                c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                c.lq(2, p(g(c, A0), 0x20));
                                c.sq(2, p(g(c, A2), 0x20));
                                c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                c.g[A0][0] = c.lw(p(g(c, S0), 0x24));
                                break 'b_2ea124;
                            }
                            c.cycles += 5;
                            c.mult(0, 3, 3, 8);
                            c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                            c.lq(2, p(g(c, A0), 0x20));
                            c.sq(2, p(g(c, A1), 0x20));
                            c.g[V0][0] = c.lw(p(g(c, S0), 0x94));
                            break 'b_2ea138;
                        }
                        // Post-sample bookkeeping: raise the sample count (`+0x90`) by three,
                        // clamped at 54, and mark the trail running (`+0x94` = 1).
                        'b_2ea28c: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = 0x36;
                            if t {
                                break 'b_2ea28c;
                            } else {
                                'b_2ea154: {
                                    c.cycles += 5;
                                    c.g[V0][0] = sx(p(g(c, S3), 0x35));
                                    c.div(0, 2, 3);
                                    c.g[A0][0] = 0x30;
                                    if c.g[V1][0] == 0u64 {
                                        break 'b_2ea154;
                                    } else {
                                        break 'b_2ea154;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[T1][0] = 0x5;
                                c.g[A1][0] = c.hi[0];
                                c.mult(0, 8, 19, 4);
                                c.mult(0, 7, 5, 4);
                                'l_2ea168: loop {
                                    c.cycles += 32;
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[T1][0] = sx(g(c, T1).wrapping_sub(0x1));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.g[V1][0] = sx(g(c, T0).wrapping_add(g(c, V1)));
                                    c.lq(2, p(g(c, V1), 0x20));
                                    c.g[A0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.sq(2, p(g(c, A0), 0x20));
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.g[V1][0] = sx(g(c, T0).wrapping_add(g(c, V1)));
                                    c.g[A1][0] = c.lw(p(g(c, V1), 0x10));
                                    c.g[A0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.sw(p(g(c, A0), 0x10), c.g[A1][0]);
                                    c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                                    c.g[A0][0] = c.lw(p(g(c, V0), 0x14));
                                    c.g[V1][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, V1), 0x14), c.g[A0][0]);
                                    c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.g[V0][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                                    c.g[A0][0] = c.lw(p(g(c, V0), 0x18));
                                    c.g[V1][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, V1), 0x18), c.g[A0][0]);
                                    c.g[V0][0] = c.lw(g(c, A2));
                                    c.g[A2][0] = sx(p(g(c, A2), 0x4));
                                    c.g[V1][0] = sx(g(c, T0).wrapping_add(g(c, V0)));
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, V0)));
                                    c.sw(p(g(c, V1), 0x1c), 0u64);
                                    let t = (c.g[T1][0] as i64) >= 0;
                                    c.sw(p(g(c, V0), 0x1c), 0u64);
                                    if t {
                                        if event_due(c) { deliver_event_here(c, 0x002ea168); } continue 'l_2ea168;
                                    } else {
                                        'b_2ea284: {
                                            c.cycles += 30;
                                            c.g[A0][0] = 0x30;
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x10));
                                            c.mult(0, 4, 19, 4);
                                            c.g[A1][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.g[A2][0] = c.lw(p(g(c, S0), 0x18));
                                            c.g[V0][0] = sx(g(c, A0).wrapping_add(g(c, V0)));
                                            c.g[A1][0] = sx(g(c, A0).wrapping_add(g(c, A1)));
                                            c.lq(3, p(g(c, V0), 0x20));
                                            c.g[A2][0] = sx(g(c, A0).wrapping_add(g(c, A2)));
                                            c.sq(3, p(g(c, A1), 0x20));
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x1c));
                                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                                            c.lq(3, p(g(c, V0), 0x20));
                                            c.sq(3, p(g(c, A2), 0x20));
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x14));
                                            c.g[V1][0] = c.lw(p(g(c, S0), 0x20));
                                            c.g[V0][0] = sx(g(c, A0).wrapping_add(g(c, V0)));
                                            c.g[A2][0] = c.lw(p(g(c, S0), 0x24));
                                            c.lq(5, p(g(c, V0), 0x20));
                                            c.g[V1][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                            c.g[A2][0] = sx(g(c, A0).wrapping_add(g(c, A2)));
                                            c.sq(5, p(g(c, V1), 0x20));
                                            c.g[V0][0] = c.lw(p(g(c, S0), 0x20));
                                            c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                                            c.lq(3, p(g(c, V0), 0x20));
                                            c.sq(3, p(g(c, A2), 0x20));
                                            c.g[A0][0] = c.lw(p(g(c, S0), 0x90));
                                            c.g[V0][0] = ((c.g[A0][0] as i64) < 54i64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = sx(p(g(c, A0), 0x1));
                                            if t {
                                                break 'b_2ea284;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V1][0] = ((c.g[V0][0] as i64) < 54i64) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                                if t {
                                                    break 'b_2ea284;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx(p(g(c, A0), 0x2));
                                                    c.g[V1][0] = ((c.g[V0][0] as i64) < 54i64) as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                                    if t {
                                                        break 'b_2ea284;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[V0][0] = sx(p(g(c, A0), 0x3));
                                                        c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                                        break 'b_2ea284;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x1;
                                        c.sw(p(g(c, S0), 0x94), c.g[V0][0]);
                                        break 'b_2ea28c;
                                    }
                                }
                            }
                        }
                        // Ask `cBoardTrailFX_ShouldEmit` whether this frame emits a sample;
                        // when it does, write the colour at `+0xd0`, blend the fades at
                        // `+0x04`/`+0x08` and shift the strips again.
                        'b_2ea428: {
                            c.cycles += 13;
                            c.lqc2(3, frame.at(c, 0x90));
                            c.lqc2(5, p(g(c, S0), 0xc0));
                            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                            c.qmfc2(2, 4);
                            c.mtc1(2, 12);
                            c.g[A0][0] = c.g[S0][0];
                            callee::SHOULD_EMIT.call(c, 0x002e_a2b8)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw(gp(c, 0xa0c));
                            if t {
                                break 'b_2ea428;
                            } else {
                                'b_2ea360: {
                                    c.cycles += 40;
                                    c.g[A2][0] = 0x10000;
                                    c.g[T2][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[A2][0] = c.g[A2][0] | 0x8fcd;
                                    c.mult(0, 2, 2, 6);
                                    c.g[T0][0] = 0x36;
                                    c.g[V1][0] = sx(p(g(c, T2), 0x1));
                                    c.g[A0][0] = 0xe90000;
                                    c.div(0, 3, 8);
                                    c.g[A0][0] = c.g[A0][0] | 0x507c;
                                    c.g[A1][0] = 0x7f0000;
                                    c.lq(3, frame.at(c, 0xa0));
                                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                                    c.g[V0][0] = sx(g(c, V0).wrapping_add(g(c, A0)));
                                    c.g[A3][0] = 0x3f800000;
                                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                    c.sq(3, p(g(c, S0), 0xd0));
                                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                    c.sw(gp(c, 0xa0c), c.g[V0][0]);
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.lwc1(1, gp(c, 0xa0c));
                                    c.lwc1(0, gp(c, 0x14bc));
                                    c.sub_s(1, 1, 2);
                                    c.mul_s(0, 0, 1);
                                    c.add_s(0, 0, 2);
                                    c.swc1(0, p(g(c, S0), 0x4));
                                    c.lwc1(1, gp(c, 0x14bc));
                                    c.g[T1][0] = c.hi[0];
                                    c.mult(0, 3, 2, 6);
                                    c.g[V0][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                                    c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                                    c.sw(gp(c, 0xa0c), c.g[V0][0]);
                                    c.lwc1(0, gp(c, 0xa0c));
                                    c.sub_s(0, 0, 2);
                                    c.mul_s(1, 1, 0);
                                    c.add_s(1, 1, 2);
                                    c.swc1(1, p(g(c, S0), 0x8));
                                    if c.g[T0][0] == 0u64 {
                                        break 'b_2ea360;
                                    } else {
                                        break 'b_2ea360;
                                    }
                                }
                                'b_2ea378: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x90));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 54i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw(p(g(c, S0), 0x8c), c.g[T1][0]);
                                    if t {
                                        break 'b_2ea378;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = sx(p(g(c, V1), 0x1));
                                        c.sw(p(g(c, S0), 0x90), c.g[V0][0]);
                                        break 'b_2ea378;
                                    }
                                }
                                c.cycles += 4;
                                c.g[T0][0] = 0x30;
                                c.g[A2][0] = c.g[S1][0];
                                c.mult(0, 7, 10, 8);
                                c.g[T1][0] = 0x5;
                                'l_2ea388: loop {
                                    c.cycles += 38;
                                    c.g[A1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[T1][0] = sx(g(c, T1).wrapping_sub(0x1));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.mult(0, 5, 5, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.lq(3, p(g(c, V0), 0x20));
                                    c.g[A1][0] = sx(g(c, A1).wrapping_add(g(c, A0)));
                                    c.sq(3, p(g(c, A1), 0x20));
                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.mult(0, 4, 4, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.g[A1][0] = c.lw(p(g(c, V0), 0x1c));
                                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, A0), 0x1c), c.g[A1][0]);
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[A0][0] = c.lw(g(c, A2));
                                    c.mult(0, 3, 3, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A0)));
                                    c.g[A1][0] = c.lw(p(g(c, V0), 0x10));
                                    c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A0)));
                                    c.sw(p(g(c, V1), 0x10), c.g[A1][0]);
                                    c.g[A0][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[V1][0] = c.lw(g(c, A2));
                                    c.mult(0, 4, 4, 8);
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, V1)));
                                    c.g[A1][0] = c.lw(p(g(c, V0), 0x14));
                                    c.g[A0][0] = sx(g(c, A0).wrapping_add(g(c, V1)));
                                    c.sw(p(g(c, A0), 0x14), c.g[A1][0]);
                                    c.g[V1][0] = c.lw(p(g(c, S0), 0x8c));
                                    c.g[A1][0] = c.lw(g(c, A2));
                                    c.mult(0, 3, 3, 8);
                                    c.g[A2][0] = sx(p(g(c, A2), 0x4));
                                    c.g[V0][0] = sx(g(c, A3).wrapping_add(g(c, A1)));
                                    c.g[A0][0] = c.lw(p(g(c, V0), 0x18));
                                    c.g[V1][0] = sx(g(c, V1).wrapping_add(g(c, A1)));
                                    let t = (c.g[T1][0] as i64) >= 0;
                                    c.sw(p(g(c, V1), 0x18), c.g[A0][0]);
                                    if t {
                                        if event_due(c) { deliver_event_here(c, 0x002ea388); } continue 'l_2ea388;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x2;
                                        c.sw(p(g(c, S0), 0x94), c.g[V0][0]);
                                        break 'b_2ea428;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(p(g(c, S0), 0xc));
                        c.g[V1][0] = 0x1;
                        c.lq(4, frame.at(c, 0x90));
                        c.g[V1][0] = sx(g(c, V1).wrapping_sub(g(c, V0)));
                        c.sw(p(g(c, S0), 0xc), c.g[V1][0]);
                        c.sq(4, p(g(c, S0), 0xc0));
                        break 'b_2ea440;
                    }
                }
            }
        }
    }
    c.cycles += 15;
    c.lq(16, frame.at(c, 0x260));
    c.lq(17, frame.at(c, 0x250));
    c.lq(18, frame.at(c, 0x240));
    c.lq(19, frame.at(c, 0x230));
    c.g[RA][0] = c.ld(frame.at(c, 0x220));
    c.lwc1(27, frame.at(c, 0x2a8));
    c.lwc1(26, frame.at(c, 0x2a0));
    c.lwc1(25, frame.at(c, 0x298));
    c.lwc1(24, frame.at(c, 0x290));
    c.lwc1(23, frame.at(c, 0x288));
    c.lwc1(22, frame.at(c, 0x280));
    c.lwc1(21, frame.at(c, 0x278));
    c.lwc1(20, frame.at(c, 0x270));
    frame.close(c);
    abi::ret(c);
    Ok(())

}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{compare, put_vec, seed_count, Rng, DATA, GAME_GP};

    /// The trail object.
    const SELF: u32 = DATA + 0x1000;
    /// The rider the trail belongs to (`*(self)`).
    const RIDER: u32 = DATA + 0x4000;
    /// The zeroed BELibrary dummy (`gp-0x1f04`).
    const BE: u32 = DATA + 0x4800;
    /// The dummy world (`gp-0x848`) with its module and race manager.
    const WORLD: u32 = DATA + 0x5000;
    /// The rider's animation controller (`rider+0x784`).
    const ANIM: u32 = DATA + 0x6000;
    /// The rider's animation node (`rider+0x780`) and its entry table.
    const ANIM_NODE: u32 = DATA + 0x7000;
    const ANIM_TABLE: u32 = DATA + 0x20000;
    /// The rider's animation-base object (`rider+0x6c0`): a delta at +0x38
    /// and the virtual method at +0x3c, which the fixture points at [`FAKE`].
    const VSELF: u32 = DATA + 0x8000;
    /// The six 54-vertex strips (`self+0x10..+0x24`).
    const STRIPS: u32 = DATA + 0x10000;
    const STRIP_STRIDE: u32 = 54 * 0x30;
    /// Control words the fake callees read.
    const CTRL: u32 = DATA + 0x200;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// `difftest::check`, also requiring the caller-saved scratch state and
    /// LO/HI to match: the fake callees hash every register into their result
    /// and into RAM, and `div`/`mult` leave LO/HI a later read could see.
    fn check_strict(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(
                r.soft.is_empty(),
                "seed {seed} (scratch state): {:#?}",
                r.soft
            );
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    /// `cBoardTrailFX_IsActive(self)`: bit 0 of the control word is the
    /// answer, the fade it stores at `self+0x9c` comes from the upper bits.
    fn fake_is_active(c: &mut Cpu) {
        let r = c.bus.read32(CTRL);
        w(c, SELF + 0x9c, r >> 8);
        c.g[V0][0] = (r & 1) as u64;
        abi::ret(c);
    }

    /// `func_0011FE98(rider)`: the rider sub-state. 0/1/2/12 reach every
    /// comparison the body makes on it.
    fn fake_sub_state(c: &mut Cpu) {
        let r = c.bus.read32(CTRL + 4);
        c.g[V0][0] = [0u64, 1, 2, 0xc][(r & 3) as usize];
        abi::ret(c);
    }

    /// `func_00312AA0(animbase, 2)`: the animation state. The body tests
    /// `(v0 - 0x17) < 8`, so 0x16/0x17/0x1e/0x1f cover both sides.
    fn fake_anim_state(c: &mut Cpu) {
        let r = c.bus.read32(CTRL + 8);
        c.g[V0][0] = [0x16u64, 0x17, 0x1e, 0x1f][(r & 3) as usize];
        abi::ret(c);
    }

    /// `func_0031C228(x)`: a float blend helper returning f0 (the body then
    /// does `mov.s f12, f0`).
    fn fake_blend(c: &mut Cpu) {
        let r = c.bus.read32(CTRL + 0xc);
        c.f[0] = r;
        abi::ret(c);
    }

    /// Installs the fake callees and puts the generated defaults back.
    struct Slots {
        is_active: fn(&mut Cpu),
        sub_state: fn(&mut Cpu),
        anim_state: fn(&mut Cpu),
        blend: fn(&mut Cpu),
    }

    impl Slots {
        fn install() -> Slots {
            unsafe {
                let saved = Slots {
                    is_active: ssx3_ee::s09::OVERRIDE_002E87E8,
                    sub_state: ssx3_ee::s00::OVERRIDE_0011FE98,
                    anim_state: ssx3_ee::s10::OVERRIDE_00312AA0,
                    blend: ssx3_ee::s10::OVERRIDE_0031C228,
                };
                ssx3_ee::s09::OVERRIDE_002E87E8 = fake_is_active;
                ssx3_ee::s00::OVERRIDE_0011FE98 = fake_sub_state;
                ssx3_ee::s10::OVERRIDE_00312AA0 = fake_anim_state;
                ssx3_ee::s10::OVERRIDE_0031C228 = fake_blend;
                saved
            }
        }
    }

    impl Drop for Slots {
        fn drop(&mut self) {
            unsafe {
                ssx3_ee::s09::OVERRIDE_002E87E8 = self.is_active;
                ssx3_ee::s00::OVERRIDE_0011FE98 = self.sub_state;
                ssx3_ee::s10::OVERRIDE_00312AA0 = self.anim_state;
                ssx3_ee::s10::OVERRIDE_0031C228 = self.blend;
            }
        }
    }

    /// The trail, its rider, the six strips and the globals the body and its
    /// callees read, all varied by `seed`.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x002e_8938, 4);
        c.g[A0][0] = SELF as u64;

        // `gp`-relative globals.
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        w(c, WORLD + 0x84, WORLD + 0x100);
        w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
        w(c, WORLD + 0x200 + 0x7c, 0);
        // The float constants the blend reads (and the emit cooldown rate).
        for off in [0xa38u32, 0xa3c, 0x1490, 0x1494, 0x1498, 0x149c, 0x14bc] {
            w(c, GAME_GP + off, r.edge_f32());
        }
        for off in [
            0x39acu32, 0x39b0, 0x39b4, 0x39b8, 0x39bc, 0x39c0, 0x39c4, 0x39c8, 0x39cc,
            0x39d0,
        ] {
            w(c, GAME_GP.wrapping_sub(off), r.edge_f32());
        }
        // The pseudo-random word at `gp+0xa0c` that the emit path advances.
        w(c, GAME_GP + 0xa0c, r.next_u32());

        // The fake callees' answers.
        w(c, CTRL, r.next_u32());
        w(c, CTRL + 4, r.next_u32());
        w(c, CTRL + 8, r.next_u32());
        w(c, CTRL + 0xc, r.edge_f32());

        // The trail object.
        w(c, SELF + 0x00, RIDER);
        w(c, SELF + 0x04, r.edge_f32());
        w(c, SELF + 0x08, r.edge_f32());
        w(c, SELF + 0x0c, r.next_u32() & 1);
        for k in 0..6u32 {
            w(c, SELF + 0x10 + 4 * k, STRIPS + k * STRIP_STRIDE);
        }
        w(c, SELF + 0x8c, seed.wrapping_mul(7) % 54);
        w(c, SELF + 0x90, seed.wrapping_mul(13) % 54);
        // 0 = fresh trail, 1 = rewinding, >= 2 = live (both the live and the
        // "not active" clearing paths).
        w(c, SELF + 0x94, [0u32, 1, 2, 2, 3][(seed % 5) as usize]);
        w(c, SELF + 0x9c, r.edge_f32());
        w(c, SELF + 0xa0, r.edge_f32());
        for off in [0xb0u32, 0xc0, 0xd0] {
            put_vec(
                c,
                SELF + off,
                [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
            );
        }
        // 0 takes the long `ShouldEmit` path, 1 the cooldown shortcut.
        w(c, SELF + 0xe0, seed % 2);

        // The rider fields the body reads.
        w(c, RIDER + 0x04, r.next_u32());
        w(c, RIDER + 0x150, r.next_u32() % 4);
        w(c, RIDER + 0x320, r.next_u32() & 1);
        w(c, RIDER + 0x330, r.next_u32() % 3);
        w(c, RIDER + 0x454, r.edge_f32());
        w(c, RIDER + 0x6c0, VSELF);
        w(c, RIDER + 0x780, ANIM_NODE);
        w(c, RIDER + 0x784, ANIM);
        for off in [0x8a4u32, 0x8b0, 0x8b8, 0x8e8, 0x918] {
            w(c, RIDER + off, r.next_u32() % 4);
        }
        // `boardTrailCanDraw`: all four combinations.
        w(c, RIDER + 0xac4, seed & 1);
        w(c, RIDER + 0xad0, (seed >> 1) & 1);
        w(c, RIDER + 0xafc, (seed >> 2) & 1);
        w(c, RIDER + 0xb00, (seed >> 3) & 1);
        w(c, ANIM_NODE + 0x34, ANIM_TABLE);
        // The rider's animation-base virtual method (the one `jalr`).
        c.bus.write16(VSELF + 0x38, 0);
        w(c, VSELF + 0x3c, FAKE);

        // The vectors the dot products consume.
        let q = |r: &mut Rng| {
            [
                r.edge_f32(),
                r.edge_f32(),
                r.edge_f32(),
                r.edge_f32(),
            ]
        };
        put_vec(c, RIDER + 0x370, q(&mut r));
        put_vec(c, RIDER + 0x460, q(&mut r));
        put_vec(c, RIDER + 0x1e0, q(&mut r));
        for k in 0..4u32 {
            let e = ANIM_TABLE + 0x40 * k;
            put_vec(c, e, q(&mut r));
            put_vec(c, e + 0x20, q(&mut r));
            put_vec(c, e + 0x30, q(&mut r));
        }

        // The strip vertices (the shift loops read them back).
        for i in 0..(6 * 54 * 0x30 / 4) {
            w(c, STRIPS + 4 * i, r.next_u32());
        }
    }

    #[test]
    fn c_board_trail_fx_update_matches() {
        let _slots = Slots::install();
        check_strict(
            ssx3_ee::s09::f_002e8938__generated,
            c_board_trail_fx_update,
            96,
            &setup,
        );
    }
}
