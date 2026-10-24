//! `src/visualfx/boardsparkfx.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::visualfx::attachedemitterfx::rand_step;
pub(crate) use crate::game::visualfx::attachedemitterfx::vu0_length;
pub(crate) use crate::game::visualfx::attachedemitterfx::RAND_SEED;
pub(crate) use crate::game::visualfx::boardwakefx::addiu;
pub(crate) use crate::game::visualfx::boardwakefx::at;
pub(crate) use crate::game::visualfx::boardwakefx::back_edge;
pub(crate) use crate::game::visualfx::boardwakefx::copy_quad;
pub(crate) use crate::game::visualfx::boardwakefx::fconst;
pub(crate) use crate::game::visualfx::boardwakefx::gp;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::visualfx::boardwakefx::vadd;
pub(crate) use crate::game::visualfx::boardwakefx::vmul_bc;
pub(crate) use crate::game::visualfx::boardwakefx::APP;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_abc8, c_board_spark_fx_update, "cBoardSparkFX_Update"),
    (0x002d_b478, board_spark_fx_render, "cBoardSparkFX_Render"),
];

// ---------------------------------------------------------------------------
// Board spark rendering.

/// `sSpark`: board spark particles.
pub(crate) mod spark {

    /// The rider (`+0x438` player index / character, `+0x1e0` velocity,
    /// `+0x460` position, `+0x1a0` direction, `+0x2f0` speed).
    pub const RIDER: u32 = 0;

    /// The spark source point (written each frame).
    pub const SOURCE: u32 = 0x10;

    /// The dynamic emitter the sparks spawn from, and the argument of its
    /// restart method.
    pub const EMITTER: u32 = 0x20;

    pub const RESTART_ARG: u32 = 0x24;

    /// The particle descriptor (`Emitter_*`).
    pub const DESC: u32 = 0x28;

    /// The burst's three points and random scales (written).
    pub const BURST_POINTS: u32 = 0x30;

    pub const BURST_SCALES: u32 = 0x60;

    /// Restart the emitter this frame.
    pub const RESTART: u32 = 0x6c;

    /// Spraying (the player is carving), bursting (landed carving), grinding.
    pub const SPRAY: u32 = 0x70;

    pub const BURST: u32 = 0x74;

    pub const GRIND: u32 = 0x78;

    /// Argument passed to the trail renderer.
    pub const TRAIL_ARG: u32 = 0x28;

    /// The three quad positions.
    pub const QUADS: u32 = 0x30;

    /// Their sizes.
    pub const SIZES: u32 = 0x60;

    /// Draw the connected trail strip.
    pub const DRAW_TRAIL: u32 = 0x70;

    /// Draw the three spark quads.
    pub const DRAW_QUADS: u32 = 0x74;

    /// Saved renderer state for the trail (20 bytes).
    pub const TRAIL_STATE: u32 = 0x7c;

    /// Saved renderer state for the quads (20 bytes).
    pub const QUAD_STATE: u32 = 0x90;
}

// ---------------------------------------------------------------------------
// Board wake strip rendering.

/// The renderer object (`cGfx`) at gp-0x854.
pub(crate) mod gfx {
    pub(crate) use crate::game::visualfx::boardwakefx::gfx::F1030;
    pub(crate) use crate::game::visualfx::boardwakefx::gfx::STATE_TOP;
    pub(crate) use crate::game::visualfx::boardwakefx::gfx::VTABLE;
}

/// The character id that always grinds sparks.
pub(crate) const SPARK_CHARACTER: u64 = 9;

/// Board colour table: 0xf0 bytes per board (`0x500000 - 0x5c68`).
pub(crate) const BOARD_COLOURS: u32 = 0x004f_a398;

pub(crate) const EMITTER_UPDATE_DESCRIPTOR: u32 = 0x0036_d428;

pub(crate) const EMITTER_SET_VELOCITY_CURVE: u32 = 0x0036_d318;

pub(crate) const EMITTER_SET_PATH_SCALED: u32 = 0x0036_d1f0;

pub(crate) const EMITTER_SET_PATH: u32 = 0x0036_cef8;

pub(crate) const DYNAMIC_EMITTER_SPAWN_AT: u32 = 0x0037_17c0;

pub(crate) const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;

/// `void cBoardSparkFX_Update(cBoardSparkFX *fx)` (0x2dabc8): advance the
/// descriptor by the frame time, restart the emitter when asked; from the
/// rider's speed, main state (4) and the player record's carve/grind
/// switches decide spraying, bursting and grinding; spawn one spark at the
/// board (grinding: with a velocity from the rider's direction and speed
/// and the board's colour, else an idle spark); while spraying set up the
/// descriptor's velocity curve and path from the rider's bone axes
/// (hot sparks when boosting or fast); when bursting, roll three random
/// burst points around the source.
///
/// Frame 0x130: s0-s7 at +0x100..+0x90, ra +0x80, f20-f22 at +0x110..+0x120.
pub fn c_board_spark_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        let sp = r32(c, SP).wrapping_sub(0x130);
        let s = |off: u32| sp.wrapping_add(off);
        c.cycles += 33;
        let frame = Frame::open(c, 0x130);
        let clock = c.lw(gp.wrapping_add(0x2a74));
        c.g[V0][0] = clock;
        frame.save(c, S1, 0xf0);
        c.g[A2][0] = 0xb0;
        frame.save(c, S2, 0xe0);
        frame.save(c, S0, 0x100);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save(c, S7, 0x90);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 22, 0x120);
        frame.save_fpr(c, 21, 0x118);
        frame.save_fpr(c, 20, 0x110);
        let fx = c.g[S2][0];
        // dt = 1 / frame rate; the player record (0xb0 bytes each).
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 12);
        let rider = c.lw(at(fx, spark::RIDER));
        c.g[A0][0] = rider;
        c.lwc1(0, at(clock, 0x10));
        c.cvt_s_w(0, 0);
        let app = c.lw(gp.wrapping_sub(APP));
        c.g[V0][0] = app;
        c.g[V1][0] = c.lw(at(rider, 0x438));
        c.div_s(12, 12, 0);
        c.g[A1][0] = c.lw(at(app, 0x84));
        c.mult(0, V1, V1, A2);
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.g[V0][0] = c.lw(at(c.g[A1][0], 0x44));
        c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        call_ptr(c, EMITTER_UPDATE_DESCRIPTOR, 0x002d_ac40)?;
        c.g[V0][0] = c.lw(at(fx, spark::RESTART));
        if c.g[V0][0] != 0 {
            // emitter->restart(arg, emitter->rate) (method 0x18 of +0x1f8).
            c.cycles += 11;
            let em = c.lw(at(fx, spark::EMITTER));
            c.g[A0][0] = em;
            c.g[A1][0] = c.lw(at(fx, spark::RESTART_ARG));
            let vt = c.lw(at(em, 0x1f8));
            c.g[V0][0] = vt;
            c.lwc1(12, at(em, 0x58));
            c.g[V1][0] = c.lh(at(vt, 0x18));
            let f = c.lw(at(vt, 0x1c));
            c.g[A2][0] = f;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            call_ptr(c, f as u32, 0x002d_ac6c)?;
            c.g[V0][0] = 1;
            c.sw(at(fx, spark::RESTART), 1);
        }
        c.sw(at(fx, spark::SPRAY), 0);
        // Fast enough? (|velocity| > gp-0x3c94)
        c.cycles += 19;
        c.g[S0][0] = 1;
        c.sw(at(fx, spark::BURST), 0);
        c.sw(at(fx, spark::GRIND), 0);
        let rider = c.lw(at(fx, spark::RIDER));
        c.g[A0][0] = rider;
        c.lwc1(0, gp.wrapping_sub(0x3c94));
        vu0_length(c, at(rider, 0x1e0), V0, 20);
        c.c_lt_s(0, 20);
        let fast = c.fcc();
        c.mov_s(22, 20);
        if !fast {
            c.cycles += 1;
            c.g[S0][0] = 0;
        }
        // Main state 4 (s7); the player record's switches.
        c.cycles += 5;
        call_ptr(c, RIDER_GET_MAIN_STATE, 0x002d_acd0)?;
        c.g[V0][0] ^= 4;
        let moving = c.g[S0][0] != 0;
        c.g[S7][0] = (c.g[V0][0] < 1) as u64;
        let mut reload = true;
        if moving {
            c.cycles += 3;
            let carving = c.lw(at(c.g[S1][0], 0x8c));
            c.g[V0][0] = 1;
            let mut check_grind = true;
            let mut check_char = false;
            if carving != 0 {
                c.cycles += 2;
                let plain = c.g[S7][0] == 0;
                c.sw(at(fx, spark::SPRAY), 1);
                if plain {
                    check_grind = false;
                    check_char = true;
                } else {
                    c.cycles += 1;
                    c.sw(at(fx, spark::BURST), 1);
                }
            }
            let mut grind = false;
            let mut char_loaded = false;
            if check_grind {
                c.cycles += 2;
                if c.g[S7][0] == 0 {
                    c.g[V0][0] = c.lw(at(fx, spark::RIDER));
                    char_loaded = true;
                } else {
                    c.cycles += 3;
                    let rail = c.lw(at(c.g[S1][0], 0x88));
                    c.g[V0][0] = 1;
                    if rail != 0 {
                        grind = true;
                    } else {
                        check_char = true;
                    }
                }
            }
            if check_char {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(fx, spark::RIDER));
                char_loaded = true;
            }
            if char_loaded {
                c.cycles += 4;
                c.g[A0][0] = SPARK_CHARACTER;
                c.g[V1][0] = c.lw(at(c.g[V0][0], 0x438));
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[V1][0] = c.lw(at(fx, spark::RIDER));
                    reload = false;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    grind = true;
                }
            }
            if grind {
                c.cycles += 1;
                let v0 = c.g[V0][0];
                c.sw(at(fx, spark::GRIND), v0);
            }
        }
        if reload {
            c.cycles += 1;
            c.g[V1][0] = c.lw(at(fx, spark::RIDER));
        }
        // The source: the rider's position + velocity * gp-0x3c90.
        c.cycles += 18;
        let rider = c.g[V1][0];
        c.lwc1(0, gp.wrapping_sub(0x3c90));
        vmul_bc(c, A0, 0, at(rider, 0x1e0), s(0x20));
        c.lq(V0, s(0x20));
        let rider = c.lw(at(fx, spark::RIDER));
        c.g[V1][0] = rider;
        c.sq(V0, s(0x10));
        vadd(c, at(rider, 0x460), s(0x10), s(0x20), false);
        c.lq(V0, s(0x20));
        c.g[V1][0] = c.lw(at(fx, spark::GRIND));
        c.sq(V0, at(fx, spark::SOURCE));
        c.sq(V0, sp);
        if c.g[V1][0] != 0 {
            spark_grind(c, sp, gp)?;
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.lw(at(fx, spark::EMITTER));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(12, gp.wrapping_sub(0x3c68));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002d_afe8)?;
        }
        c.g[V0][0] = c.lw(at(fx, spark::SPRAY));
        // Spraying: the descriptor's velocity curve and path.
        c.cycles += 2;
        let spraying = c.g[V0][0] != 0;
        c.g[S3][0] = sx(s(0x10));
        if spraying {
            spark_spray(c, sp, gp)?;
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = c.lw(at(fx, spark::BURST));
        // Bursting: three random points around the source.
        c.cycles += 2;
        let bursting = c.g[V0][0] != 0;
        c.g[V1][0] = c.lw(gp.wrapping_add(RAND_SEED));
        if !(bursting && spark_burst(c, sp, gp)) {
            c.cycles += 2;
            c.sw(at(fx, spark::BURST), 0);
        }
        frame.restore(c, S0, 0x100);
        c.cycles += 13;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0x120);
        frame.restore_fpr(c, 21, 0x118);
        frame.restore_fpr(c, 20, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// The grinding spark: the board colour (character 9: the constant colour
/// at gp-0x3c88, else the colour of the board the rider's method 0x38
/// names, doubled with alpha 1) at sp+0x10, the velocity along the rider's
/// direction (sp+0x30), a random size, spawned at the source.
pub(crate) fn spark_grind(c: &mut Cpu, sp: u32, gp: u32) -> Flow {
    let s = |off: u32| sp.wrapping_add(off);
    let fx = c.g[S2][0];
    c.cycles += 6;
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[A1][0] = rider;
    c.g[V1][0] = SPARK_CHARACTER;
    c.lwc1(21, gp.wrapping_sub(0x3c8c));
    c.g[V0][0] = c.lw(at(rider, 0x438));
    if c.g[V0][0] != c.g[V1][0] {
        let vt = c.lw(at(rider, 0x6c0));
        c.g[V1][0] = vt;
        c.cycles += 38;
        c.g[A1][0] = sx(at(rider, 0x6c0));
        c.g[A0][0] = c.lh(at(vt, 0x38));
        let f = c.lw(at(vt, 0x3c));
        c.g[V0][0] = f;
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, f as u32, 0x002d_add0)?;
        c.g[A0][0] = 0xf0;
        c.g[V1][0] = 0x50_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(BOARD_COLOURS);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 5);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(BOARD_COLOURS));
        let colour = r32(c, V0);
        c.ldl(V1, colour.wrapping_add(7));
        c.ldr(V1, colour);
        c.ldl(A0, colour.wrapping_add(0xf));
        c.ldr(A0, colour.wrapping_add(8));
        c.sdl(V1, s(0x27));
        c.sdr(V1, s(0x20));
        c.sdl(A0, s(0x2f));
        c.sdr(A0, s(0x28));
        // (a, r, g, b) -> (r, g, b, a).
        c.lwc1(0, s(0x24));
        c.lwc1(1, s(0x28));
        c.lwc1(2, s(0x2c));
        c.lwc1(3, s(0x20));
        c.swc1(0, s(0x30));
        c.swc1(1, s(0x34));
        c.swc1(2, s(0x38));
        c.swc1(3, s(0x3c));
        copy_quad(c, s(0x30), s(0x10));
        vmul_bc(c, V1, 4, s(0x10), s(0x10));
        c.swc1(5, s(0x1c));
    } else {
        c.cycles += 13;
        c.lwc1(0, gp.wrapping_sub(0x3c88));
        c.lwc1(1, gp.wrapping_sub(0x3c84));
        c.lwc1(2, gp.wrapping_sub(0x3c80));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 3);
        c.swc1(0, s(0x20));
        c.swc1(1, s(0x24));
        c.swc1(2, s(0x28));
        c.swc1(3, s(0x2c));
        c.lwc1(21, gp.wrapping_sub(0x3c7c));
        copy_quad(c, s(0x20), s(0x10));
    }
    // The velocity: direction * speed * gp-0x3c78 ... plus a random
    // sideways part; the spark size f3 (gp-0x3c8c or the constant).
    c.cycles += 72;
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[V1][0] = rider;
    c.g[A3][0] = 0x1_8fcd;
    c.g[AT][0] = 0x3e80_0000;
    c.mtc1(AT, 1);
    c.lq(V0, at(rider, 0x1a0));
    c.g[A2][0] = 0xe9_507c;
    c.lwc1(2, gp.wrapping_sub(0x3c78));
    c.sq(V0, s(0x40));
    c.g[A1][0] = 0x7f_ffff;
    vmul_bc(c, V0, 22, s(0x40), s(0x50));
    c.lwc1(0, gp.wrapping_sub(0x3c74));
    c.lq(V0, s(0x50));
    c.div_s(0, 20, 0);
    c.g[T1][0] = 0x3f80_0000;
    c.sq(V0, s(0x30));
    c.mov_s(3, 21);
    vmul_bc(c, V1, 1, s(0x30), s(0x50));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 4);
    c.lq(V0, s(0x50));
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[V1][0] = rider;
    c.sq(V0, s(0x20));
    vmul_bc(c, A0, 2, at(rider, 0x1e0), s(0x60));
    c.lwc1(1, gp.wrapping_sub(0x3c70));
    c.lq(V0, s(0x60));
    c.mul_s(1, 0, 1);
    c.sq(V0, s(0x50));
    vadd(c, s(0x50), s(0x20), s(0x60), false);
    c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
    c.c_lt_s(1, 21);
    c.lq(V1, s(0x60));
    c.mult(0, V0, V0, A3);
    c.sq(V1, s(0x40));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[T1][0];
    let seed = c.g[V0][0];
    c.sw(gp.wrapping_add(RAND_SEED), seed);
    c.lwc1(0, gp.wrapping_add(RAND_SEED));
    c.sub_s(0, 0, 4);
    c.add_s(0, 0, 0);
    vmul_bc(c, V1, 0, s(0x20), s(0x70));
    copy_quad(c, s(0x70), s(0x60));
    vadd(c, s(0x40), s(0x60), s(0x70), true);
    c.lq(V0, s(0x70));
    let small = c.fcc();
    c.sq(V0, s(0x30));
    if small {
        c.cycles += 1;
        c.mov_s(3, 1);
    }
    // A random "hot" flag: (random - 1) < size.
    c.cycles += 12;
    c.g[V0][0] = c.lw(gp.wrapping_add(RAND_SEED));
    rand_step(c, V1, A3, A2, A1, T1);
    let seed = c.g[V0][0];
    c.sw(gp.wrapping_add(RAND_SEED), seed);
    c.lwc1(0, gp.wrapping_add(RAND_SEED));
    c.sub_s(0, 0, 4);
    c.c_lt_s(0, 3);
    let hot = c.fcc();
    c.g[T0][0] = 1;
    if !hot {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 8;
    c.g[A0][0] = c.lw(at(fx, spark::EMITTER));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(12, gp.wrapping_sub(0x3c6c));
    c.g[A2][0] = sx(s(0x30));
    c.g[A3][0] = sx(s(0x10));
    call_ptr(c, DYNAMIC_EMITTER_SPAWN_AT, 0x002d_afc4)
}

/// Spraying: the bone's axes at sp+0x10 (s3) / sp+0x20 (s4); the
/// descriptor's velocity curve from two colours (hot: (255, 0, 255, 153)
/// -> (255, 0, 255, 102); else (77, 77, 255, 153) -> (255, 51, 51, 102)),
/// the scaled path (400 / 600) and the path along the bone (x10, or x15
/// and x90 when not in state 4).
pub(crate) fn spark_spray(c: &mut Cpu, sp: u32, gp: u32) -> Flow {
    let s = |off: u32| sp.wrapping_add(off);
    let fx = c.g[S2][0];
    c.cycles += 19;
    let rider = c.lw(at(fx, spark::RIDER));
    c.g[A3][0] = rider;
    c.g[A2][0] = c.lw(gp.wrapping_add(0x163c));
    c.g[S4][0] = sx(s(0x20));
    c.g[V0][0] = c.lw(at(rider, 0x780));
    c.g[T0][0] = 0;
    c.g[V1][0] = c.lw(at(rider, 0x8a4));
    c.g[A0][0] = c.lw(at(c.g[V0][0], 0x34));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let bone = r32(c, A0);
    copy_quad(c, bone.wrapping_add(0x20), s(0x10));
    c.g[A1][0] = c.lw(at(rider, 0x780));
    c.g[V1][0] = c.lw(at(rider, 0x8a4));
    c.g[A0][0] = c.lw(at(c.g[A1][0], 0x34));
    c.g[V1][0] = sx(r32(c, V1) << 6);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let bone = r32(c, A0);
    c.lq(V0, bone);
    let boosting = c.g[A2][0] != 0;
    c.sq(V0, s(0x20));
    let hot = if boosting {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp.wrapping_add(0x1638));
        if c.g[V0][0] != 0 {
            c.g[T0][0] = 1;
            false
        } else {
            c.cycles += 5;
            c.lwc1(1, at(rider, 0x2f0));
            c.mtc1(ZERO, 0);
            c.c_lt_s(0, 1);
            c.fcc()
        }
    };
    if hot {
        c.cycles += 1;
        c.g[T0][0] = 1;
    }
    c.cycles += 2;
    let plain = c.g[T0][0] == 0;
    c.g[V0][0] = sx(s(0x30));
    if !plain {
        c.cycles += 37;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 0);
        c.g[AT][0] = 0x4319_0000;
        c.mtc1(AT, 1);
        c.g[V1][0] = sx(s(0x40));
        c.g[AT][0] = 0x42cc_0000;
        c.mtc1(AT, 2);
        c.g[S0][0] = sx(s(0x50));
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.g[S1][0] = sx(s(0x60));
        c.lwc1(12, gp.wrapping_sub(0x3c64));
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        c.swc1(1, s(0x3c));
        c.g[A2][0] = c.g[V1][0];
        c.swc1(0, s(0x48));
        c.g[S5][0] = c.g[V0][0];
        c.swc1(2, s(0x4c));
        c.g[S6][0] = c.g[V1][0];
        c.swc1(0, s(0x30));
        c.sw(s(0x34), 0);
        c.swc1(0, s(0x38));
        c.swc1(0, s(0x40));
        c.sw(s(0x44), 0);
        for off in (0x50..0x70).step_by(4) {
            c.sw(s(off), 0);
        }
        call_ptr(c, EMITTER_SET_VELOCITY_CURVE, 0x002d_b0f4)?;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
    } else {
        c.cycles += 40;
        c.g[AT][0] = 0x429a_0000;
        c.mtc1(AT, 4);
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(AT, 3);
        c.g[V1][0] = sx(s(0x40));
        c.g[AT][0] = 0x424c_0000;
        c.mtc1(AT, 1);
        c.g[S1][0] = sx(s(0x50));
        c.g[AT][0] = 0x4319_0000;
        c.mtc1(AT, 0);
        c.g[S0][0] = sx(s(0x60));
        c.g[AT][0] = 0x42cc_0000;
        c.mtc1(AT, 2);
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.g[T0][0] = c.g[S0][0];
        c.lwc1(12, gp.wrapping_sub(0x3c60));
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[V1][0];
        c.swc1(4, s(0x34));
        c.swc1(0, s(0x3c));
        c.g[S5][0] = c.g[V0][0];
        c.swc1(3, s(0x40));
        c.g[S6][0] = c.g[V1][0];
        c.swc1(1, s(0x48));
        c.swc1(2, s(0x4c));
        c.swc1(4, s(0x30));
        c.swc1(3, s(0x38));
        c.swc1(1, s(0x44));
        for off in (0x50..0x70).step_by(4) {
            c.sw(s(off), 0);
        }
        call_ptr(c, EMITTER_SET_VELOCITY_CURVE, 0x002d_b198)?;
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
    }
    // The scaled path: (0, 0, 400, 0), (600, 0, 0, 0), (0, 600, 0, 0),
    // (0, 0, 600, 0).
    c.cycles += 26;
    c.g[AT][0] = 0x4416_0000;
    c.mtc1(AT, 1);
    c.g[A1][0] = c.g[S5][0];
    c.g[AT][0] = 0x43c8_0000;
    c.mtc1(AT, 0);
    c.g[A2][0] = c.g[S6][0];
    c.g[A0][0] = c.lw(at(fx, spark::DESC));
    c.swc1(0, s(0x38));
    c.swc1(1, s(0x68));
    c.sw(s(0x30), 0);
    c.sw(s(0x34), 0);
    c.sw(s(0x3c), 0);
    c.swc1(1, s(0x40));
    for off in [0x44, 0x48, 0x4c, 0x50] {
        c.sw(s(off), 0);
    }
    c.swc1(1, s(0x54));
    for off in [0x58, 0x5c, 0x60, 0x64] {
        c.sw(s(off), 0);
    }
    c.sw(s(0x6c), 0);
    call_ptr(c, EMITTER_SET_PATH_SCALED, 0x002d_b200)?;
    let landed = c.g[S7][0] != 0;
    c.g[A2][0] = c.g[S5][0];
    // The path along the bone's axes.
    c.cycles += 22;
    let (axis_a, axis_b) = (r32(c, S3), r32(c, S4));
    if landed {
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0);
        vmul_bc(c, V0, 0, axis_a, s(0x40));
        c.g[A3][0] = c.g[S6][0];
        c.lq(V0, s(0x40));
        c.g[A1][0] = c.g[SP][0];
        c.sq(V0, s(0x30));
        vmul_bc(c, V1, 0, axis_b, s(0x50));
        c.lq(V0, s(0x50));
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.sq(V0, s(0x40));
        call_ptr(c, EMITTER_SET_PATH, 0x002d_b258)?;
    } else {
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 0);
        vmul_bc(c, V0, 0, axis_a, s(0x40));
        c.g[AT][0] = 0x42b4_0000;
        c.mtc1(AT, 1);
        c.lq(V0, s(0x40));
        c.g[A3][0] = c.g[S6][0];
        c.g[A1][0] = c.g[SP][0];
        c.sq(V0, s(0x30));
        vmul_bc(c, V1, 1, axis_b, s(0x50));
        c.lq(V0, s(0x50));
        c.g[A0][0] = c.lw(at(fx, spark::DESC));
        c.sq(V0, s(0x40));
        call_ptr(c, EMITTER_SET_PATH, 0x002d_b2b8)?;
        c.cycles += 1;
    }
    Ok(())
}

/// The burst (when a random roll beats gp-0x3c5c): three points
/// `source + (8 * (r1 - 1) - 4, ...)` with random scales `10 * (r - 1) +
/// 20`, at `fx+0x30..` and `fx+0x60..`. Returns whether it ran (else the
/// caller clears the burst flag).
pub(crate) fn spark_burst(c: &mut Cpu, sp: u32, gp: u32) -> bool {
    let s = |off: u32| sp.wrapping_add(off);
    let fx = c.g[S2][0];
    let seed_at = gp.wrapping_add(RAND_SEED);
    c.cycles += 21;
    c.g[V0][0] = 0x1_8fcd;
    c.g[A0][0] = 0xe9_0000;
    c.mult(0, V1, V1, V0);
    c.g[A0][0] = 0xe9_507c;
    c.g[V0][0] = 0x7f_ffff;
    c.g[A1][0] = 0x3f80_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.lwc1(1, gp.wrapping_sub(0x3c5c));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[V1][0] &= c.g[V0][0];
    c.g[V1][0] |= c.g[A1][0];
    let seed = c.g[V1][0];
    c.sw(seed_at, seed);
    c.lwc1(0, seed_at);
    c.sub_s(0, 0, 2);
    c.c_lt_s(1, 0);
    let go = c.fcc();
    c.g[A2][0] = 0x1_0000;
    if !go {
        return false;
    }
    c.cycles += 18;
    c.g[T4][0] = 0xe9_507c;
    c.g[A1][0] = 0x7f_ffff;
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 7);
    c.g[AT][0] = 0x4120_0000;
    c.mtc1(AT, 6);
    c.mov_s(3, 2);
    c.g[AT][0] = 0xffff_ffff_c080_0000;
    c.mtc1(AT, 5);
    c.g[A2][0] = 0x1_8fcd;
    c.g[AT][0] = 0x4100_0000;
    c.mtc1(AT, 4);
    c.g[T1][0] = 0x3f80_0000;
    c.g[T0][0] = sx(at(fx, spark::BURST_POINTS));
    c.g[A3][0] = sx(at(fx, spark::BURST_SCALES));
    c.g[T2][0] = 2;
    loop {
        c.cycles += 53;
        c.g[V0][0] = c.lw(seed_at);
        addiu(c, T2, T2, u32::MAX);
        c.sw(s(0x2c), 0);
        // scale = 10 * (r - 1) + 20
        rand_step(c, V1, A2, T4, A1, T1);
        c.mult(0, V1, V0, A2);
        let seed = c.g[V0][0];
        c.sw(seed_at, seed);
        c.lwc1(0, seed_at);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, T4)));
        c.sub_s(0, 0, 3);
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= c.g[T1][0];
        c.mult(0, A0, V0, A2);
        c.mul_s(0, 0, 6);
        c.add_s(0, 0, 7);
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, T4)));
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= c.g[T1][0];
        let scale_at = r32(c, A3);
        c.swc1(0, scale_at);
        c.mult(0, T3, V1, A2);
        let seed = c.g[V0][0];
        c.sw(seed_at, seed);
        addiu(c, A3, A3, 4);
        // the offset (8 * (r - 1) - 4) per axis
        c.lwc1(2, seed_at);
        let seed = c.g[V1][0];
        c.sw(seed_at, seed);
        c.g[A0][0] = sx(r32(c, T3).wrapping_add(r32(c, T4)));
        c.sub_s(2, 2, 3);
        c.g[A0][0] &= c.g[A1][0];
        c.lwc1(1, seed_at);
        c.g[A0][0] |= c.g[T1][0];
        let seed = c.g[A0][0];
        c.sw(seed_at, seed);
        c.sub_s(1, 1, 3);
        c.mul_s(2, 2, 4);
        c.lwc1(0, seed_at);
        c.mul_s(1, 1, 4);
        c.sub_s(0, 0, 3);
        c.add_s(2, 2, 5);
        c.add_s(1, 1, 5);
        c.mul_s(0, 0, 4);
        c.swc1(2, s(0x20));
        c.swc1(1, s(0x24));
        c.add_s(0, 0, 5);
        c.swc1(0, s(0x28));
        vadd(c, at(fx, spark::SOURCE), s(0x20), s(0x30), false);
        c.lq(V0, s(0x30));
        let point = r32(c, T0);
        c.sq(V0, point);
        c.sq(V0, s(0x10));
        let more = (c.g[T2][0] as i64) >= 0;
        addiu(c, T0, T0, 0x10);
        if !more {
            break;
        }
        back_edge(c, 0x002d_b360);
    }
    c.cycles += 2;
    true
}

/// `PushState`: copy the current 20-byte renderer state one slot up and leave
/// the new top in v0 (as the original's `stateTop[1] = stateTop[0];
/// stateTop++`). Returns the new top.
pub(crate) fn push_renderer_state(c: &mut Cpu) -> u32 {
    let gfxp = c.lw(gp(c, -0x854));
    c.g[V0][0] = c.lw(p(gfxp as u32, gfx::STATE_TOP));
    let top = r32(c, V0);
    c.ldl(3, p(top, 7));
    c.ldr(3, p(top, 0));
    c.ldl(5, p(top, 0xf));
    c.ldr(5, p(top, 8));
    c.g[A2][0] = c.lw(p(top, 0x10));
    c.sdl(3, p(top, 0x1b));
    c.sdr(3, p(top, 0x14));
    c.sdl(5, p(top, 0x23));
    c.sdr(5, p(top, 0x1c));
    c.sw(p(top, 0x24), c.g[A2][0]);
    c.g[V1][0] = c.lw(p(gfxp as u32, gfx::STATE_TOP));
    c.g[A1][0] = gfxp;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
    c.sw(p(gfxp as u32, gfx::STATE_TOP), c.g[V1][0]);
    c.g[V0][0] = c.lw(p(r32(c, A1), gfx::STATE_TOP));
    r32(c, V0)
}

/// `void cBoardSparkFX_Render(Spark* self)` (0x2db478): step the spark
/// emitter, then (if enabled) draw the trail strip and the three spark quads
/// as textured, coloured quads, each in its own pushed renderer state.
pub fn board_spark_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        let self_at = r32(c, S0);
        let sp0 = sp(c);
        // Advance the spark emitter when it has live particles.
        c.g[A0][0] = c.lw(p(self_at, spark::EMITTER));
        c.g[V0][0] = c.lw(p(r32(c, A0), 0x1e0));
        if (c.g[V0][0] as i64) <= 0 {
            c.g[V0][0] = c.lw(p(self_at, spark::DRAW_TRAIL));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0037_1688, 0x002d_b4b4)?;
            c.g[V0][0] = c.lw(p(self_at, spark::DRAW_TRAIL));
        }
        // The connected trail strip.
        c.cycles += 2;
        let no_trail = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gp(c, -0x854));
        if !no_trail {
            c.cycles += 37;
            let top = push_renderer_state(c);
            c.ldl(3, p(self_at, spark::TRAIL_STATE + 7));
            c.ldr(3, p(self_at, spark::TRAIL_STATE));
            c.ldl(4, p(self_at, spark::TRAIL_STATE + 0xf));
            c.ldr(4, p(self_at, spark::TRAIL_STATE + 8));
            c.g[A1][0] = c.lw(p(self_at, spark::TRAIL_STATE + 0x10));
            c.sdl(3, p(top, 7));
            c.sdr(3, p(top, 0));
            c.sdl(4, p(top, 0xf));
            c.sdr(4, p(top, 8));
            c.sw(p(top, 0x10), c.g[A1][0]);
            c.g[A2][0] = c.lw(gp(c, -0x854));
            c.g[A1][0] = c.lw(p(self_at, spark::TRAIL_ARG));
            c.g[V0][0] = c.lw(p(r32(c, A2), gfx::VTABLE));
            c.g[A0][0] = c.lh(p(r32(c, V0), 0x290));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x294));
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002d_b544)?;
            c.g[A0][0] = c.lw(gp(c, -0x854));
            c.g[V0][0] = c.lw(p(r32(c, A0), gfx::STATE_TOP));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            c.sw(p(r32(c, A0), gfx::STATE_TOP), c.g[V0][0]);
        }
        // The three spark quads.
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(self_at, spark::DRAW_QUADS));
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            let top = push_renderer_state(c);
            c.ldl(7, p(self_at, spark::QUAD_STATE + 7));
            c.ldr(7, p(self_at, spark::QUAD_STATE));
            c.ldl(3, p(self_at, spark::QUAD_STATE + 0xf));
            c.ldr(3, p(self_at, spark::QUAD_STATE + 8));
            c.g[A0][0] = c.lw(p(self_at, spark::QUAD_STATE + 0x10));
            c.sdl(7, p(top, 7));
            c.sdr(7, p(top, 0));
            c.sdl(3, p(top, 0xf));
            c.sdr(3, p(top, 8));
            c.sw(p(top, 0x10), c.g[A0][0]);
            // One-time setup of the shared sprite UVs and colour.
            c.g[V1][0] = c.lw(gp(c, 0x23e8));
            let t = c.g[V1][0] != 0;
            c.g[V0][0] = c.lw(gp(c, 0x23f8));
            if !t {
                c.cycles += 5;
                c.g[V0][0] = 1;
                c.sw(gp(c, 0x23e0), 0);
                c.sw(gp(c, 0x23e8), c.g[V0][0]);
                c.sw(gp(c, 0x23e4), 0);
                c.g[V0][0] = c.lw(gp(c, 0x23f8));
            }
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(gp(c, 0x23fc));
            if !t {
                c.cycles += 7;
                fconst(c, 0, 0x3f80_0000);
                c.g[V0][0] = 1;
                c.sw(gp(c, 0x23f8), c.g[V0][0]);
                c.swc1(0, gp(c, 0x23f4));
                c.swc1(0, gp(c, 0x23f0));
                c.g[V0][0] = c.lw(gp(c, 0x23fc));
            }
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.g[S3][0] = 0x004d_0000;
            if !t {
                c.cycles += 9;
                fconst(c, 0, 0x3f80_0000);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x57a0));
                c.g[V1][0] = 1;
                c.swc1(0, p(r32(c, V0), 0xc));
                c.sw(gp(c, 0x23fc), c.g[V1][0]);
                c.swc1(0, p(r32(c, S3), 0x57a0));
                c.swc1(0, p(r32(c, V0), 4));
                c.swc1(0, p(r32(c, V0), 8));
            }
            c.cycles += 7;
            c.g[S1][0] = sx(self_at.wrapping_add(spark::SIZES));
            c.g[S5][0] = sx(r32(c, GP).wrapping_add(0x23e0));
            c.g[S4][0] = sx(r32(c, GP).wrapping_add(0x23f0));
            c.g[S0][0] = sx(self_at.wrapping_add(spark::QUADS));
            c.g[S2][0] = 2;
            c.lwc1(0, r32(c, S1));
            loop {
                c.cycles += 18;
                c.g[A1][0] = c.g[S0][0];
                c.g[T2][0] = c.lw(gp(c, 0x2a90));
                c.g[A2][0] = sx(sp0);
                c.swc1(0, p(sp0, 4));
                c.g[A3][0] = c.g[S5][0];
                c.swc1(0, sp0);
                c.g[T0][0] = c.g[S4][0];
                c.g[T1][0] = sx(r32(c, S3).wrapping_add(0x57a0));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10));
                c.g[V1][0] = c.lw(p(r32(c, T2), gfx::VTABLE));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                c.g[A0][0] = c.lh(p(r32(c, V1), 0x270));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x274));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002d_b698)?;
                if (c.g[S2][0] as i64) >= 0 {
                    c.lwc1(0, r32(c, S1));
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_b658);
                    }
                    continue;
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw(gp(c, -0x854));
                c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
                c.sw(p(r32(c, V1), gfx::STATE_TOP), c.g[V0][0]);
                break;
            }
        }
        frame.restore(c, S0, 0x70);
        c.cycles += 8;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::delta;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::gp_at;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::slot;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS;
    pub(crate) use crate::game::visualfx::boardwakefx::tests::SEEDS_2;

    #[test]
    fn c_board_spark_fx_update_matches() {
        covered(
            ssx3_ee::s09::f_002dabc8__generated,
            c_board_spark_fx_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2dab_c8, 3);
                let (fx, em, em_vt, desc) = (DATA, DATA + 0x1000, DATA + 0x1800, DATA + 0x2000);
                let (clock, app, module, players) =
                    (DATA + 0x3000, DATA + 0x3100, DATA + 0x3200, DATA + 0x3300);
                let (rider, state, rider_vt, model, bones) = (
                    DATA + 0x4000,
                    DATA + 0x4800,
                    DATA + 0x4900,
                    DATA + 0x5000,
                    DATA + 0x5100,
                );
                c.g[A0][0] = sx(fx);
                c.bus.write32(fx + spark::RIDER, rider);
                c.bus.write32(fx + spark::EMITTER, em);
                c.bus.write32(fx + spark::RESTART_ARG, r.next_u32());
                c.bus.write32(fx + spark::DESC, desc);
                c.bus.write32(fx + spark::RESTART, r.next_u32() % 2);
                // The dynamic emitter: restart method (fake), colour slots;
                // DynamicEmitter_Spawn returns at once (+0x174 = 0).
                c.bus.write32(em + 0x1f8, em_vt);
                slot(c, em_vt, 0x18, delta(&mut r), FAKE + 0x10);
                c.bus.write32(em + 0x58, r.f32(2.0));
                c.bus.write32(em + 0x174, 0);
                c.bus.write32(em + 0x17c, r.next_u32() % 4);
                c.bus.write32(em + 0x204, em + 0x400);
                // The descriptor: no catch-up loop in Emitter_UpdateDescriptor.
                for i in 0..0x50 {
                    c.bus.write32(desc + 4 * i, r.f32(4.0));
                }
                c.bus.write32(desc, 1000);
                c.bus.write32(desc + 8, 0);
                c.bus.write32(desc + 0xc, 1.0f32.to_bits());
                c.bus.write32(desc + 0x4c, 1.0f32.to_bits());
                // Frame rate, app, player records (+0x88 rail, +0x8c carving).
                c.bus.write32(GAME_GP + 0x2a74, clock);
                c.bus
                    .write32(clock + 0x10, if r.next_u32() % 2 == 0 { 30 } else { 60 });
                c.bus.write32(GAME_GP - APP, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x44, players);
                for i in 0..10 {
                    c.bus.write32(players + 0xb0 * i + 0x88, r.next_u32() % 2);
                    c.bus
                        .write32(players + 0xb0 * i + 0x8c, (r.next_u32() % 3 != 0) as u32);
                }
                // The rider: player index or the spark character, velocity,
                // position, direction, main state, board query (fake, forced
                // to a small board index), bone.
                let character = if r.next_u32() % 4 == 0 {
                    9
                } else {
                    r.next_u32() % 4
                };
                c.bus.write32(rider + 0x438, character);
                for off in [0x1a0, 0x1e0, 0x460] {
                    for k in 0..4 {
                        c.bus.write32(rider + off + 4 * k, r.f32(8.0));
                    }
                }
                c.bus.write32(rider + 0x2f0, r.f32(1.0));
                c.bus.write32(rider + 0x77c, state);
                c.bus.write32(
                    state + 0xde0,
                    if r.next_u32() % 2 == 0 {
                        4
                    } else {
                        r.next_u32() % 8
                    },
                );
                c.bus.write32(rider + 0x6c0, rider_vt);
                slot(c, rider_vt, 0x38, delta(&mut r), FAKE);
                for n in 0..8 {
                    c.bus.write32(
                        crate::difftest::fakes::LOG_RESULTS + 4 * n,
                        1 + r.next_u32() % 4,
                    );
                }
                for i in 0..0x140u32 {
                    c.bus.write32(BOARD_COLOURS + 4 * i, r.f32(1.0));
                }
                c.bus.write32(rider + 0x780, model);
                c.bus.write32(rider + 0x8a4, r.next_u32() % 4);
                c.bus.write32(model + 0x34, bones);
                fill(c, &mut r, bones, 0x40);
                // Globals: the threshold (speed 5), constants, switches, seed.
                c.bus.write32(GAME_GP - 0x3c94, 5.0f32.to_bits());
                for off in (0x3c5c..0x3c94).step_by(4) {
                    c.bus.write32(GAME_GP - off, r.f32(2.0));
                }
                c.bus
                    .write32(GAME_GP + 0x163c, (r.next_u32() % 3 == 0) as u32);
                c.bus
                    .write32(GAME_GP + 0x1638, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(GAME_GP + RAND_SEED, r.next_u32());
            },
        );
    }

    #[test]
    fn board_spark_fx_render_matches() {
        check(
            ee::s09::f_002db478__generated,
            board_spark_fx_render,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2db4, 2);
                let (s, em, gfx, state, vt1, ctx2, vt2) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x1000,
                    DATA + 0x2000,
                    DATA + 0x3000,
                    DATA + 0x7000,
                    DATA + 0x7200,
                );
                // Emitter step: enabled = 0 makes the real callee return at once.
                c.bus.write32(em + 0x174, 0);
                c.bus.write32(em + 0x1e0, seed % 2);
                // The renderers' state stack and virtual tables.
                c.bus.write32(gp_at(-0x854), gfx);
                c.bus.write32(gfx + 0xe84, state);
                c.bus.write32(gfx + 0x10d8, vt1);
                c.bus.write16(vt1 + 0x290, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt1 + 0x294, FAKE);
                c.bus.write32(gp_at(0x2a90), ctx2);
                c.bus.write32(ctx2 + 0x10d8, vt2);
                c.bus.write16(vt2 + 0x270, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt2 + 0x274, FAKE);
                // The one-time UV/colour globals.
                c.bus
                    .write32(gp_at(0x23e8), if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(gp_at(0x23f8), if seed % 4 == 0 { 0 } else { 1 });
                c.bus
                    .write32(gp_at(0x23fc), if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(gp_at(0x23e0), r.next_u32());
                c.bus.write32(gp_at(0x23e4), r.next_u32());
                c.bus.write32(gp_at(0x23f0), r.next_u32());
                c.bus.write32(gp_at(0x23f4), r.next_u32());
                for k in 0..4u32 {
                    c.bus.write32(0x004d_57a0 + 4 * k, r.edge_f32());
                }
                // The spark object.
                c.bus.write32(s + 0x20, em);
                c.bus.write32(s + 0x28, r.next_u32());
                for k in 0..12u32 {
                    c.bus.write32(s + 0x30 + 4 * k, r.edge_f32());
                }
                for k in 0..3u32 {
                    c.bus.write32(s + 0x60 + 4 * k, r.edge_f32());
                }
                for k in 0..5u32 {
                    c.bus.write32(s + 0x7c + 4 * k, r.next_u32());
                    c.bus.write32(s + 0x90 + 4 * k, r.next_u32());
                }
                c.bus.write32(s + 0x70, if seed % 6 == 0 { 0 } else { 1 });
                c.bus.write32(s + 0x74, if seed % 7 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(s);
            },
        );
    }
}
