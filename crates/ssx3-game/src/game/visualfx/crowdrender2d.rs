//! `src/visualfx/crowdrender2d.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_b9c8, c_crowd_anim2d_ctor, "cCrowdAnim2D_ctor"),
    (0x002d_bac0, c_crowd_anim2d_update, "cCrowdAnim2D_Update"),
    (
        0x002d_ba08,
        crowd_anim2d_step_frame,
        "cCrowdAnim2D_StepFrame",
    ),
    (
        0x002d_b748,
        ccrowd_render2d_load_textures,
        "cCrowdRender2D_LoadTextures",
    ),
    (0x002d_b980, ccrowd_anim2d_create, "cCrowdAnim2D_Create"),
    (0x002d_b6d8, crowd_render_2d_ctor, "cCrowdRender2D_ctor"),
    (0x002d_b700, ccrowd_render2d_dtor, "cCrowdRender2D_dtor"),
    (
        0x002d_b890,
        ccrowd_render2d_release_textures,
        "cCrowdRender2D_ReleaseTextures",
    ),
];

// ---------------------------------------------------------------------------
// 2D crowd / fog particles.

/// `cCrowdAnim2D *cCrowdAnim2D_ctor(cCrowdAnim2D *this, cCrowdAnim2DData *data)`
/// (0x2db9c8): initialise the animation record, storing the data pointer at
/// +0x1c and resolving the shared tables.
pub fn c_crowd_anim2d_ctor(c: &mut Cpu) {
    c.cycles += 16;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x4;
    Addr(this).off(0x1c).sw(c, c.g[A1][0]);
    Addr(this).off(0x8).sw(c, 0x4);
    c.g[A1][0] = 0x44_0000;
    Addr(this).off(0x4).sw(c, 0);
    c.g[A3][0] = 0x3;
    Addr(this).off(0xc).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x5dd0)).lw(c);
    Addr(this).sw(c, 0);
    Addr(this).off(0x10).sw(c, c.g[V1][0]);
    c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x1c)).lw(c);
    Addr(this).off(0x14).sw(c, c.g[A3][0]);
    Addr(this).off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// The 2D crowd's animation tables in `.data`: `D_00445D38` (0xd8-byte
/// animations: `+4` frame count, `+0x98 + 4 * frame` texture), the
/// transition table `D_00445E10` (12-byte entries: next animation, mode,
/// kind) and its per-type base index `D_00445E20`.
const ANIMS: u32 = 0x0044_5d38;
const TRANSITIONS: u32 = 0x0044_5e10;
const TRANSITION_BASE: u32 = 0x0044_5e20;

/// Where a `cCrowdAnim2D_Update` path continues (the shared tails of the
/// compiled code).
enum Next {
    /// `v0` = the animation entry: load its frame count, then step.
    Count,
    /// Step the frame (`a0..a3` set).
    Step,
    /// Store `v0` as the frame.
    StoreFrame,
    /// Reload the animation.
    LoadAnim,
    /// `v1` = the animation: pick the texture.
    Texture,
}

/// `void cCrowdAnim2D_Update(cCrowdAnim2D* self)` (0x2dbac0): once per tick
/// for each 2D crowd sprite. When `+0x14` ticks have passed since the last
/// step (`+0x18`, against the app manager's update count `+0x1c`), advance
/// the animation: follow the transition table for the sprite's type `+0`
/// and animation `+4` (kind 0: switch at frame 0, kind 1: at the last
/// frame, kind 2: at once), step the frame `+0xc` in direction `+8`
/// (`cCrowdAnim2D_StepFrame`), and store the frame's texture at `+0x10`.
///
/// Tick rate: the interval is in 60 Hz frames (3 from the ctor), so above
/// 60 it is compared as `ticks_from_frames60(interval)` ticks.
pub fn c_crowd_anim2d_update(c: &mut Cpu) {
    const STEP_FRAME: Callee = Callee {
        addr: 0x002d_ba08,
        f: ee::s09::f_002dba08,
    };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a74)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[V1][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        c.g[A0][0] = Addr(this).off(0x14).lw(c);
        if crate::tick::rate(c) != 60 && (c.g[A0][0] as i64) >= 0 {
            c.g[A0][0] = crate::tick::ticks_from_frames60(c, c.g[A0][0] as u32) as u64;
        }
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            // The transition for this type and animation.
            c.cycles += 18;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x0044_0000;
            Addr(this).off(0x18).sw(c, c.g[A1][0]);
            c.g[V0][0] = sx(TRANSITION_BASE);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = 0xc;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = Addr(this).off(4).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = 0x0044_0000;
            c.mult(0, A0, A2, A1);
            c.g[V1][0] = sx(TRANSITIONS);
            c.mult(0, V0, V0, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let tr = r32(c, T1);
            c.g[A0][0] = Addr(tr).lw(c);
            let mut next;
            if c.g[A0][0] == c.g[A2][0] {
                // No transition: keep stepping this animation.
                c.cycles += 10;
                c.g[V1][0] = 0xd8;
                c.g[A2][0] = 0x0044_0000;
                c.mult(0, V1, A0, V1);
                c.g[V0][0] = sx(ANIMS);
                c.g[S1][0] = c.g[A2][0];
                c.g[A1][0] = Addr(this).off(0xc).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A3][0] = sx(this.wrapping_add(8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                next = Next::Count;
            } else {
                c.g[T2][0] = Addr(tr).off(8).lw(c);
                let kind = c.g[T2][0];
                c.cycles += 3;
                c.g[A2][0] = 1;
                c.g[V0][0] = ((kind as i64) < 2) as u64;
                if kind == 1 {
                    // Kind 1: run backwards to the last frame, then switch.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this).off(8).lw(c);
                    c.g[A3][0] = sx(this.wrapping_add(8));
                    if c.g[V0][0] == kind {
                        c.cycles += 1;
                        Addr(this).off(8).sw(c, 0);
                    }
                    c.cycles += 12;
                    c.g[V1][0] = Addr(this).off(4).lw(c);
                    c.g[T3][0] = 0xd8;
                    c.g[A0][0] = 0x0044_0000;
                    c.g[A1][0] = Addr(this).off(0xc).lw(c);
                    c.mult(0, V1, V1, T3);
                    c.g[V0][0] = sx(ANIMS);
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                    c.g[A2][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                    c.g[S1][0] = c.g[A0][0];
                    if c.g[A1][0] != c.g[V0][0] {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        next = Next::Step;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = Addr(tr).lw(c);
                        Addr(this).off(4).sw(c, c.g[V1][0]);
                        c.g[V0][0] = Addr(tr).off(4).lw(c);
                        Addr(this).off(8).sw(c, c.g[V0][0]);
                        if c.g[V0][0] != kind {
                            c.cycles += 2;
                            Addr(this).off(0xc).sw(c, 0);
                            next = Next::LoadAnim;
                        } else {
                            c.cycles += 5;
                            c.mult(0, V0, V1, T3);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            next = Next::StoreFrame;
                        }
                    }
                } else {
                    c.cycles += 2;
                    let below_two = c.g[V0][0] != 0;
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    c.g[S1][0] = 0x0044_0000;
                    if !below_two && kind == 2 {
                        // Kind 2: switch at once and step the new animation.
                        c.cycles += 12;
                        Addr(this).off(4).sw(c, c.g[A0][0]);
                        c.g[V1][0] = 0xd8;
                        c.mult(0, V1, A0, V1);
                        c.g[T0][0] = 0x0044_0000;
                        c.g[A2][0] = Addr(tr).off(4).lw(c);
                        c.g[V0][0] = sx(ANIMS);
                        c.g[A1][0] = Addr(this).off(0xc).lw(c);
                        c.g[A0][0] = c.g[S0][0];
                        Addr(this).off(8).sw(c, c.g[A2][0]);
                        c.g[A3][0] = sx(this.wrapping_add(8));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S1][0] = c.g[T0][0];
                        next = Next::Count;
                    } else if below_two && kind == 0 {
                        // Kind 0: run forwards to frame 0, then switch.
                        c.cycles += 3;
                        c.g[V0][0] = Addr(this).off(8).lw(c);
                        c.g[A3][0] = sx(this.wrapping_add(8));
                        if c.g[V0][0] == 0 {
                            c.cycles += 1;
                            Addr(this).off(8).sw(c, c.g[A2][0]);
                        }
                        c.cycles += 3;
                        c.g[A1][0] = Addr(this).off(0xc).lw(c);
                        if c.g[A1][0] != 0 {
                            c.g[V1][0] = Addr(this).off(4).lw(c);
                            c.cycles += 8;
                            c.g[A0][0] = 0xd8;
                            c.g[A2][0] = 0x0044_0000;
                            c.mult(0, V1, V1, A0);
                            c.g[V0][0] = sx(ANIMS);
                            c.g[S1][0] = c.g[A2][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            next = Next::Count;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = Addr(tr).lw(c);
                            Addr(this).off(4).sw(c, c.g[V1][0]);
                            c.g[V0][0] = Addr(tr).off(4).lw(c);
                            Addr(this).off(8).sw(c, c.g[V0][0]);
                            if c.g[V0][0] != c.g[A2][0] {
                                c.cycles += 3;
                                Addr(this).off(0xc).sw(c, 0);
                                c.g[S1][0] = 0x0044_0000;
                                next = Next::LoadAnim;
                            } else {
                                c.cycles += 9;
                                c.g[A0][0] = 0xd8;
                                c.g[V0][0] = 0x0044_0000;
                                c.mult(0, A0, V1, A0);
                                c.g[S1][0] = c.g[V0][0];
                                c.g[V1][0] = sx(ANIMS);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V0][0] = Addr(r32(c, V1)).off(4).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                                next = Next::StoreFrame;
                            }
                        }
                    } else {
                        // Any other kind: no change.
                        c.cycles += 2;
                        c.g[V1][0] = Addr(this).off(4).lw(c);
                        next = Next::Texture;
                    }
                }
            }

            // The shared tails.
            loop {
                match next {
                    Next::Count => {
                        c.cycles += 1;
                        c.g[A2][0] = Addr(r32(c, V0)).off(4).lw(c);
                        next = Next::Step;
                    }
                    Next::Step => {
                        c.cycles += 2;
                        STEP_FRAME.call(c, 0x002d_bccc)?;
                        next = Next::StoreFrame;
                    }
                    Next::StoreFrame => {
                        c.cycles += 1;
                        Addr(this).off(0xc).sw(c, c.g[V0][0]);
                        next = Next::LoadAnim;
                    }
                    Next::LoadAnim => {
                        c.cycles += 1;
                        c.g[V1][0] = Addr(this).off(4).lw(c);
                        next = Next::Texture;
                    }
                    Next::Texture => break,
                }
            }
            c.cycles += 10;
            c.g[A1][0] = 0xd8;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5d38));
            c.mult(0, V1, V1, A1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = Addr(r32(c, A0)).off(0x98).lw(c);
            Addr(this).off(0x10).sw(c, c.g[V1][0]);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cCrowdAnim2D_StepFrame (0x2dba08)

/// The step-mode jump table (`0x480000 + 0x75f0`, five entries in .rodata).
pub(crate) const CROWD_STEP_TABLE: u32 = 0x0048_75f0;

/// The five step modes' handlers (the table's entries).
pub(crate) const CROWD_STEP_FORWARD: u32 = 0x002d_ba30;

pub(crate) const CROWD_STEP_BACKWARD: u32 = 0x002d_ba44;

pub(crate) const CROWD_STEP_PINGPONG_UP: u32 = 0x002d_ba54;

pub(crate) const CROWD_STEP_PINGPONG_DOWN: u32 = 0x002d_ba7c;

pub(crate) const CROWD_STEP_LOOP: u32 = 0x002d_baa0;

/// `int cCrowdAnim2D_StepFrame(void* self, int frame, int count, int* mode)`
/// (0x2dba08): the next frame of a 2D crowd animation of `count` frames.
/// Mode 0 plays forward and holds the last frame, 1 backward to frame 0,
/// 2/3 ping-pong (switching `*mode` at the ends), 4 loops; single-frame
/// animations stay put in modes 2-4, unknown modes return `frame`.
pub fn crowd_anim2d_step_frame(c: &mut Cpu) {
    c.cycles += 4;
    let mode_at = Addr(r32(c, A3));
    let mode = mode_at.lw(c);
    c.g[V1][0] = mode;
    c.g[V0][0] = 0x48_0000;
    if mode < 5 {
        c.cycles += 6;
        let entry = ((mode as u32) << 2).wrapping_add(CROWD_STEP_TABLE);
        c.g[V1][0] = sx(entry);
        c.g[V0][0] = sx(CROWD_STEP_TABLE);
        let target = Addr(entry).lw(c);
        c.g[A0][0] = target;
        let count = c.g[A2][0];
        // `slt frame, count` on the full registers.
        fn below(c: &Cpu, count: u64) -> u64 {
            ((c.g[A1][0] as i64) < (count as i64)) as u64
        }
        match target as u32 {
            CROWD_STEP_FORWARD => {
                c.cycles += 5;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx((count as u32).wrapping_sub(1));
                c.g[V0][0] = below(c, count);
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
            }
            CROWD_STEP_BACKWARD => {
                c.cycles += 4;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                c.g[V0][0] = (-1i64 < c.g[A1][0] as i64) as u64;
                c.cycles += 1;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = 0;
                }
            }
            CROWD_STEP_PINGPONG_UP => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if count != 1 {
                    c.cycles += 4;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    let inside = below(c, count) != 0;
                    c.g[V0][0] = 0x3;
                    if !inside {
                        // Past the end: turn around.
                        c.cycles += 3;
                        c.g[A1][0] = sx((count as u32).wrapping_sub(2));
                        mode_at.sw(c, 0x3);
                    }
                }
            }
            CROWD_STEP_PINGPONG_DOWN => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if count != 1 {
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    let inside = (c.g[A1][0] as i64) >= 0;
                    c.g[V0][0] = 0x2;
                    if !inside {
                        // Before the start: turn around.
                        c.cycles += 3;
                        c.g[A1][0] = 0x1;
                        mode_at.sw(c, 0x2);
                    }
                }
            }
            CROWD_STEP_LOOP => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if count != 1 {
                    c.cycles += 2;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    c.g[V0][0] = below(c, count);
                    c.cycles += 1;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    }
                }
            }
            other => {
                c.jump_out(other, 0x002d_ba08, 0x002d_bac0);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

/// `void cCrowdRender2D::LoadTextures(cCrowdRender2D* this)` (0x2db748).
///
/// Loads the crowd sprite sheet (`FILE_load` 0x3e18c8 of "crwd.ssh", stored at
/// `this + 0`), then, for the one animation entry of the table at 0x445d3c
/// (count in the first word, names 5 bytes apart, shape and handle arrays),
/// finds each shape by name (`SHAPE_locate`, 0x3b39b8) and registers it with
/// the graphics manager's virtual slot `+0x170` (handle stored in the table).
/// Finally copies the first shape's `short`/`short`/`byte` fields (`+4`, `+6`,
/// `+0`) into `this + 4`, `+8`, `+0xc` (sprite width, height, format).
pub fn ccrowd_render2d_load_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[A1][0] = 0x5000000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.g[A0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = 0x480000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x75c0));
        call_ptr(c, 0x003e18c8, 0x002db788)?;
        c.g[V1][0] = 0x440000;
        c.sw(c.g[S3][0] as u32, c.g[V0][0]);
        c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5d3c));
        c.g[V0][0] = 0u64;
        c.g[S6][0] = c.g[S7][0];
        c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_add(0x94));
        c.g[V1][0] = 0xd8;
        'l_2db7a8: loop {
            'b_2db830: {
                c.cycles += 6;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.mult(0, 4, 2, 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let t = (c.g[V1][0] as i64) <= 0;
                c.g[S2][0] = 0u64;
                if t {
                    break 'b_2db830;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d40));
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    'l_2db7d8: loop {
                        c.cycles += 22;
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x003b39b8, 0x002db7e8)?;
                        c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                        c.g[A1][0] = c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_sub(0x40), c.g[V0][0]);
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A3][0] = 0u64;
                        c.g[T0][0] = 0u64;
                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002db81c)?;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002db7d8);
                            }
                            continue 'l_2db7d8;
                        } else {
                            break 'b_2db830;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S5][0];
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V1][0] = 0xd8;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002db7a8);
                }
                continue 'l_2db7a8;
            } else {
                c.cycles += 21;
                c.g[V0][0] = 0x440000;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d38));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x6));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `cCrowdAnim2D* cCrowdAnim2D_Create(void* arg)` (0x2db980): allocates 0x20
/// bytes with `operator_new_named(0x20, name 0x4875d8, flags 0x20000000, 0)`
/// and runs `cCrowdAnim2D_ctor(mem, arg)`; returns the constructor's result.
pub fn ccrowd_anim2d_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75d8));
        c.g[A0][0] = 0x20;
        call_ptr(c, 0x0031_7d70, 0x002d_b9a8)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x002d_b9c8,
            f: ee::s09::f_002db9c8,
        }
        .call(c, 0x002d_b9b4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cCrowdRender2D* cCrowdRender2D_ctor(cCrowdRender2D* this)` (0x2db6d8):
/// runs `cCrowdRender2D_LoadTextures(this)` and returns `this`.
pub fn crowd_render_2d_ctor(c: &mut Cpu) {
    if c.pc != 0x002d_b6d8 {
        return ee::s09::f_002db6d8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002d_b748,
            f: ee::s09::f_002db748,
        }
        .call(c, 0x002d_b6ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cCrowdRender2D::~cCrowdRender2D` (`cCrowdRender2D_dtor`, 0x2db700): releases
/// the textures (`cCrowdRender2D_ReleaseTextures`, 0x2db890), then, when the
/// delete flag (bit 0 of the second argument) is set, frees the object with
/// `__builtin_delete` (0x317e50).
pub fn ccrowd_render2d_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x002d_b890, 0x002d_b71c)?;
        c.g[S0][0] &= 0x1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002d_b730)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cCrowdRender2D::ReleaseTextures` (0x2db890): for the one record at
/// `0x445d3c` (the loop only runs once: `A0` starts at 0), reads its count at
/// `+0` and, for each of the `count` pointers in the list at `+0x94`, calls the
/// function pointer at `*(*(gp+0x2a90) + 0x10d8) + 0x194` with that pointer
/// (and the `lh +0x190` offset of the table added to the table pointer). Then
/// it calls `0x2523a8` with the object (`s7`) and returns.
pub fn ccrowd_render2d_release_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S5, 0x40);
        frame.save(c, S4, 0x50);
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d3c));
        frame.save(c, S7, 0x20);
        c.g[S4][0] = c.g[S5][0];
        frame.save(c, S6, 0x30);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, FP, 0x10);
        c.g[A0][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0x94));
        frame.save(c, S1, 0x80);
        c.g[FP][0] = 0xd8;
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xd8;
        'outer: loop {
            'record: {
                c.cycles += 6;
                c.g[S1][0] = 0;
                c.mult(0, 2, 4, 2);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if empty {
                    break 'record;
                }
                c.cycles += 4;
                c.mult(0, 2, 4, 30);
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                c.g[A2][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
                loop {
                    c.cycles += 12;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002d_b928)?;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = Addr(c.g[GP][0] as u32).off(0x2a90).lw(c);
                    if !more {
                        break 'record;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_b908);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            let again = (c.g[A0][0] as i64) <= 0;
            c.g[V0][0] = 0xd8;
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x002d_b8e0);
                }
                continue 'outer;
            }
            break 'outer;
        }
        c.cycles += 14;
        c.g[A0][0] = Addr(c.g[S7][0] as u32).lw(c);
        call_ptr(c, 0x0025_23a8, 0x002d_b94c)?;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::fe::ovstates::tests::oc783_install_fakes;

    #[test]
    fn c_crowd_anim2d_ctor_matches() {
        check(
            ssx3_ee::s09::f_002db9c8,
            c_crowd_anim2d_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x1004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let table = DATA + 0x500;
                c.bus.write32(GAME_GP + 0x2a74, table);
                c.bus.write32(table + 0x1c, r.next_u32());
                c.bus.write32(0x44_5dd0, r.next_u32());
            },
        );
    }

    /// A crowd sprite at `DATA` and the animation tables: animations 0, 8
    /// and 9 (1..6 frames, textures `0x7e00_0000 | anim << 8 | frame`), and
    /// the transitions of types 0 and 1 (base index `20 + 4 * type`), with
    /// random next animation, mode and kind (0..3) per seed.
    pub(crate) fn crowd_anim_fixture(c: &mut Cpu, r: &mut Rng, kind: Option<u32>) {
        const ANIM_IDS: [u32; 3] = [0, 8, 9];
        for &a in &ANIM_IDS {
            let e = ANIMS + 0xd8 * a;
            c.bus.write32(e + 4, 1 + r.next_u32() % 6);
            for f in 0..16 {
                c.bus.write32(e + 0x98 + 4 * f, 0x7e00_0000 | a << 8 | f);
            }
        }
        for t in 0..2u32 {
            let base = 20 + 4 * t;
            c.bus.write32(TRANSITION_BASE + 4 * t, base);
            for &a in &ANIM_IDS {
                let e = TRANSITIONS + 12 * (base + a);
                c.bus.write32(e, ANIM_IDS[(r.next_u32() % 3) as usize]);
                c.bus.write32(e + 4, r.next_u32() % 2);
                c.bus.write32(e + 8, kind.unwrap_or(r.next_u32() % 4));
            }
        }
        let this = DATA;
        let anim = ANIM_IDS[(r.next_u32() % 3) as usize];
        let count = c.bus.read32(ANIMS + 0xd8 * anim + 4);
        c.bus.write32(this, r.next_u32() % 2);
        c.bus.write32(this + 4, anim);
        c.bus.write32(this + 8, r.next_u32() % 3);
        let frame = match r.next_u32() % 3 {
            0 => 0,
            1 => count - 1,
            _ => r.next_u32() % count,
        };
        c.bus.write32(this + 0xc, frame);
        c.bus.write32(this + 0x10, r.next_u32());
        c.bus.write32(this + 0x14, 3);
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn c_crowd_anim2d_update_matches() {
        check_lohi(
            ssx3_ee::s09::f_002dbac0__generated,
            c_crowd_anim2d_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x2dbac0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let app = DATA + 0x800;
                c.bus.write32(GAME_GP + 0x2a74, app);
                crowd_anim_fixture(c, &mut r, None);
                // Interval 3: due (3 or more ticks since the last step) or not.
                let now = r.next_u32() >> 4;
                c.bus.write32(app + 0x1c, now);
                let since = if seed % 5 == 0 {
                    r.next_u32() % 3
                } else {
                    3 + r.next_u32() % 4
                };
                c.bus.write32(DATA + 0x18, now.wrapping_sub(since));
            },
        );
    }

    /// The sprite steps once per 3 frames at 60 Hz at any rate: 20 steps in
    /// one second of ticks (60, 120 or 240 updates).
    #[test]
    fn c_crowd_anim2d_update_follows_the_rate() {
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = Rng(0x2dbac1);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            crate::tick::set_for_test(c, hz);
            let app = c.bus.read32(GAME_GP + 0x2a74);
            crowd_anim_fixture(c, &mut r, Some(3));
            c.bus.write32(DATA + 0x18, 0);
            let mut steps = 0;
            for now in 1..=hz {
                c.bus.write32(app + 0x1c, now);
                c.g[A0][0] = DATA as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let last = c.bus.read32(DATA + 0x18);
                c_crowd_anim2d_update(c);
                assert_eq!(c.pc, crate::difftest::RA);
                if c.bus.read32(DATA + 0x18) != last {
                    steps += 1;
                }
            }
            assert_eq!(steps, 20, "{hz} Hz");
        }
    }

    use crate::difftest::fakes::machine;

    /// `cCrowdAnim2D_StepFrame`: modes 0..4 and out of range, one-frame and
    /// longer animations, frames at / near both ends and random (the real
    /// jump table from the game image).
    #[test]
    fn crowd_anim2d_step_frame_matches() {
        use crate::difftest::elf_image;
        check(
            ee::s09::f_002dba08__generated,
            crowd_anim2d_step_frame,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002d_ba08, 0);
                elf_image(c);
                c.pc = 0x002d_ba08;
                let mode = if seed % 13 == 12 {
                    r.next_u32()
                } else {
                    seed % 5
                };
                c.bus.write32(DATA, mode);
                let count = match (seed / 5) % 4 {
                    0 => 1,
                    1 => 2,
                    2 => r.next_u32() % 40,
                    _ => r.next_u32(),
                };
                let frame = match (seed / 20) % 5 {
                    0 => 0,
                    1 => count.wrapping_sub(1),
                    2 => count,
                    3 => u32::MAX,
                    _ => r.next_u32() % 40,
                };
                c.g[A1][0] = sx(frame);
                c.g[A2][0] = sx(count);
                c.g[A3][0] = sx(DATA);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::FAKE;
    pub(crate) use crate::game::fe::debugfe::tests::oc793_machine;

    pub(crate) use crate::game::fe::ovstates::tests::oc783_arm;

    pub(crate) const OC783_CROWD: u32 = DATA + 0x1_0000;

    /// The animation table of `cCrowdRender2D::LoadTextures` (count in word 0,
    /// shape pointer array at `+0x54`).
    pub(crate) const OC783_CROWD_TABLE: u32 = 0x0044_5d3c;

    pub(crate) const OC783_RENDERER: u32 = DATA + 0x2_0000;

    pub(crate) const OC783_RENDERER_VT: u32 = DATA + 0x2_1000;

    pub(crate) const OC783_SHAPE: u32 = DATA + 0x3_0000;

    /// `cCrowdRender2D_LoadTextures`: no, one and three table entries (the
    /// shape lookups and the renderer's virtual `+0x170` calls are faked),
    /// the first shape's size fields (the table's own pointer when empty) and
    /// the unwinds of every call.
    #[test]
    fn ccrowd_render2d_load_textures_matches() {
        oc783_install_fakes();
        fakes::covered(
            ee::s09::f_002db748__generated,
            ccrowd_render2d_load_textures,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x783b, 5);
                oc783_arm(c);
                c.g[A0][0] = sx(OC783_CROWD);
                let n = [0u32, 1, 3, 2][(seed % 4) as usize];
                c.bus.write32(OC783_CROWD_TABLE, n);
                // The shape of entry 0 (read back from the table at the end).
                c.bus.write32(OC783_CROWD_TABLE + 0x54, OC783_SHAPE);
                c.bus.write16(OC783_SHAPE, r.next_u32() as u16);
                c.bus.write16(OC783_SHAPE + 4, r.next_u32() as u16);
                c.bus.write16(OC783_SHAPE + 6, r.next_u32() as u16);
                // The graphics manager behind `gp + 0x2a90`.
                c.bus.write32(GAME_GP + 0x2a90, OC783_RENDERER);
                c.bus.write32(OC783_RENDERER + 0x10d8, OC783_RENDERER_VT);
                c.bus
                    .write16(OC783_RENDERER_VT + 0x170, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(OC783_RENDERER_VT + 0x174, FAKE);
            },
        );
    }

    /// `cCrowdAnim2D_Create`: the allocation and the constructor are fakes.
    #[test]
    fn ccrowd_anim2d_create_matches() {
        covered(
            ee::s09::f_002db980__generated,
            ccrowd_anim2d_create,
            80,
            &|c, seed| {
                let _ = oc793_machine(c, seed, 0x002d_b980, 2);
            },
        );
    }

    pub(crate) use crate::game::worldpainter::worldpainterqueryctor::tests::oc809_machine;

    /// `cCrowdRender2D_ctor` against the generated body: `LoadTextures` is a
    /// gated fake (it unwinds in some seeds).
    #[test]
    fn crowd_render_2d_ctor_matches() {
        fakes::covered(
            ee::s09::f_002db6d8__generated,
            crowd_render_2d_ctor,
            60,
            &|c, seed| {
                oc809_machine(c, seed, 0x2db6d8, 1, 0x002d_b6d8);
                c.g[A0][0] = (DATA + 0x1000 + 16 * seed) as u64;
            },
        );
    }

    pub(crate) use crate::game::visualfx::avalanche::tests::oc1067_gate;

    /// 0..3 records of the pointer list, with the table's offset and calls
    /// that unwind now and then.
    #[test]
    fn ccrowd_render2d_release_textures_matches() {
        check(
            ee::s09::f_002db890__generated,
            ccrowd_render2d_release_textures,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_b890, 4);
                oc1067_gate(c);
                let obj = DATA + 0x2000;
                let table = DATA + 0x3000;
                c.bus.write32(GAME_GP.wrapping_add(0x2a90), obj);
                c.bus.write32(obj + 0x10d8, table);
                c.bus.write16(table + 0x190, ((seed % 3) * 0x10) as u16);
                c.bus.write32(table + 0x194, 0x0012_3000);
                let count = seed % 4;
                c.bus.write32(0x0044_5d3c, count);
                for i in 0..count {
                    c.bus
                        .write32(0x0044_5d3c + 0x94 + 4 * i, DATA + 0x5000 + 0x10 * i);
                }
                c.g[A0][0] = sx(DATA + 0x6000);
            },
        );
    }

    /// Both flag values (delete or not), with and without unwinding calls.
    #[test]
    fn ccrowd_render2d_dtor_matches() {
        check(
            ee::s09::f_002db700__generated,
            ccrowd_render2d_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x002d_b700, 2);
                oc1067_gate(c);
                c.g[A1][0] = sx(seed);
            },
        );
    }
}
