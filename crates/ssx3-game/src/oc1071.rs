//! Staging module oc1071 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1071`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0036_7310,
        texman_destroy_btexinfo_array,
        "TexMan_DestroyBTexInfoArray",
    ),
    (
        0x0036_7d20,
        texman_update_texture_data,
        "TexMan_UpdateTextureData",
    ),
    (0x0036_9098, texman_evict_texture, "TexMan_EvictTexture"),
    (
        0x0036_9ff0,
        graphics_man_empty_hook,
        "GraphicsMan_EmptyHook",
    ),
    (
        0x0036_ce80,
        emitter_set_desc_linear_path,
        "Emitter_SetDescLinearPath",
    ),
    (
        0x0036_cfa8,
        emitter_set_desc_accel_scaled,
        "Emitter_SetDescAccelScaled",
    ),
    (0x0038_50a8, line_strip_begin, "LineStrip_Begin"),
    (
        0x0038_b0f8,
        ps2_bezier_man_free_caches,
        "PS2BezierMan_FreeCaches",
    ),
];

/// `s0 + off` for the texture manager functions (`s0` holds the table base in
/// `TexMan_EvictTexture` and the table entry's record in `TexMan_UpdateTextureData`).
fn texman_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// `s0 + off` for the line strip (`s0` is the strip context).
fn strip_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// `s0 + off` for the Bezier manager (`s0` is `this`).
fn bezier_s0(c: &Cpu, off: u32) -> u32 {
    (c.g[S0][0] as u32).wrapping_add(off)
}

/// `void TexMan_DestroyBTexInfoArray(void)` (0x367310): counts v0 down from
/// 0x5da (one step per slot), checking events at each back edge, then returns.
pub fn texman_destroy_btexinfo_array(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(0x5db_u32.wrapping_sub(1));
    loop {
        c.cycles += 7;
        let more = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0036_7318);
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void TexMan_UpdateTextureData(TexMan* gm, int idx, int a2, int a3..a7,
/// int a8, int a9)` (0x367d20). Takes the record `gm[idx]` (its pointer at
/// +8). When `a9` is set it calls the table function 0x3e6574 once with the
/// record's +0x10 pointer if `a2` is set, and once with `a8` (shifted by the
/// record's +0x18) if `a8` is set. Then it evicts the texture.
pub fn texman_update_texture_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        // a9 and a8 are the 9th and 10th arguments, at the caller's sp.
        c.g[V1][0] = Addr(frame.at(c, 0x58)).lw(c);
        c.g[A1][0] = c.g[A2][0];
        c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[S3][0] = Addr(frame.at(c, 0x50)).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(texman_s0(c, 0x1c)).lw(c);
                c.g[A2][0] = Addr(texman_s0(c, 0x10)).lw(c);
                call_ptr(c, 0x003e_6574, 0x0036_7d70)?;
            }
            c.cycles += 2;
            c.g[A2][0] = 0x4;
            if c.g[S3][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = Addr(texman_s0(c, 0x18)).lw(c);
                c.g[A0][0] = Addr(texman_s0(c, 0x20)).lw(c);
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V0][0] as u32) & 31));
                call_ptr(c, 0x003e_6574, 0x0036_7d8c)?;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x0036_9098,
            f: ee::s12::f_00369098,
        }
        .call(c, 0x0036_7d98)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void TexMan_EvictTexture(TexMan* gm, int idx)` (0x369098). Looks at the
/// record `gm[idx]`: when its state word at +0x28 is -1 nothing is freed; else
/// the VRAM block list at `gm+0x1f60` is freed (tag 9 at +0xc: the list at
/// `gm+0x4350` instead). Then two words at `gm+0x1f50` and `gm+0x1f4c` are
/// set to -1, and `gm+0x1f54` gets the -1 as well.
pub fn texman_evict_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        let unused = c.g[V1][0] == c.g[A0][0];
        c.g[V1][0] = 0x1;
        if !unused {
            c.cycles += 4;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[V0][0] = 0x9;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = sx(texman_s0(c, 0x1f60));
                Callee {
                    addr: 0x0036_6e30,
                    f: ee::s12::f_00366e30,
                }
                .call(c, 0x0036_90dc)?;
                c.g[A0][0] = u64::MAX;
            } else {
                c.cycles += 3;
                c.g[A0][0] = sx(texman_s0(c, 0x4350));
                Callee {
                    addr: 0x0036_6e30,
                    f: ee::s12::f_00366e30,
                }
                .call(c, 0x0036_90ec)?;
                c.g[A0][0] = u64::MAX;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x1;
        }
        c.cycles += 1;
        c.g[V0][0] = sx(texman_s0(c, 0x1f50));
        loop {
            c.cycles += 7;
            let fill = c.g[A0][0];
            Addr(c.g[V0][0] as u32).sw(c, fill);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0036_90f8);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = u64::MAX;
        frame.restore_ra(c, 0);
        let minus_one = c.g[V0][0];
        Addr(texman_s0(c, 0x1f54)).sw(c, minus_one);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GraphicsMan_EmptyHook(void)` (0x369ff0): a hook that does nothing.
pub fn graphics_man_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void Emitter_SetDescLinearPath(Emitter* self, sGfxVec4* a, sGfxVec4* b)`
/// (0x36ce80, tParticleEmitter_setLine): stores `2a - b` at +0xB0, `b - a` at
/// +0xC0 and the constant quad at 0x4ff120 at +0xD0.
pub fn emitter_set_desc_linear_path(c: &mut Cpu) {
    c.cycles += 30;
    let frame = Frame::open(c, 0x30);
    let this = c.g[A0][0] as u32;
    let a = c.g[A1][0] as u32;
    let b = c.g[A2][0] as u32;
    let t20 = frame.at(c, 0x20);
    let t10 = frame.at(c, 0x10);
    let t0 = frame.at(c, 0);
    c.g[AT][0] = 0x4000_0000; // 2.0f
    c.mtc1(1, 0);
    c.mfc1(2, 0);
    c.lqc2(4, a);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X)); // 2 * a
    c.sqc2(5, t20);
    c.g[A3][0] = 0x0050_0000;
    c.lq(2, t20);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0xee0)); // 0x4ff120
    c.sq(2, t10);
    c.lqc2(3, t10);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4)); // 2a - b
    c.sqc2(5, t20);
    c.lq(2, t20);
    c.sq(2, this.wrapping_add(0xb0));
    c.sq(2, t0);
    c.lqc2(3, b);
    c.lqc2(4, a);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4)); // b - a
    c.sqc2(5, t10);
    c.lq(3, t10);
    c.sq(3, this.wrapping_add(0xc0));
    c.sq(3, t0);
    let quad_addr = c.g[A3][0] as u32;
    c.lq(2, quad_addr);
    c.sq(2, this.wrapping_add(0xd0));
    frame.close(c);
}

/// `void Emitter_SetDescAccelScaled(Emitter* self, sGfxVec4* v, float s)`
/// (0x36cfa8, tParticleEmitter_setMotion): stores `(v * self.+0xC) / (s *
/// self.+0x4C)` at +0xE0.
pub fn emitter_set_desc_accel_scaled(c: &mut Cpu) {
    c.cycles += 23;
    let frame = Frame::open(c, 0x30);
    let this = c.g[A0][0] as u32;
    let v = c.g[A1][0] as u32;
    let t20 = frame.at(c, 0x20);
    let t10 = frame.at(c, 0x10);
    let t0 = frame.at(c, 0);
    c.lwc1(0, this.wrapping_add(0xc));
    c.mfc1(3, 0);
    c.lqc2(4, v);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X)); // v * self.+0xC
    c.sqc2(5, t20);
    c.lq(2, t20);
    c.lwc1(0, this.wrapping_add(0x4c));
    c.sq(2, t10);
    c.mul_s(12, 12, 0); // s * self.+0x4C
    c.mfc1(3, 12);
    c.qmtc2(3, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, t10);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, t20);
    c.lq(2, t20);
    c.sq(2, this.wrapping_add(0xe0));
    c.sq(2, t0);
    frame.close(c);
}

/// `void LineStrip_Begin(LineStrip* ctx, Vtx* v, Pos* pos)` (0x3850a8,
/// func_003850A8). Opens a strip: when `ctx+8` is clear it takes the packet
/// pointer `ctx+4` (advanced by 0x20) as the start; otherwise it closes the
/// previous strip if `ctx+0xc` is set. Then it resets the count, points
/// `ctx+0xc` at the strip tag, moves `ctx+4` past its 6 quadwords and adds the
/// first vertex.
pub fn line_strip_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[V0][0] = Addr(strip_s0(c, 0x8)).lw(c);
        // Whether the path goes through the packet bookkeeping at the end
        // (the generated body's `+1` and reload); the early exit skips it.
        let mut through_tail = true;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(strip_s0(c, 0xc)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = Addr(strip_s0(c, 0x4)).lw(c);
                through_tail = false;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0038_4dc0,
                    f: ee::s12::f_00384dc0,
                }
                .call(c, 0x0038_50f8)?;
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(strip_s0(c, 0x4)).lw(c);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
            let packet = c.g[V0][0];
            Addr(strip_s0(c, 0x8)).sw(c, packet);
            let next = c.g[V1][0];
            Addr(strip_s0(c, 0x4)).sw(c, next);
        }
        if through_tail {
            c.cycles += 1;
            c.g[V1][0] = Addr(strip_s0(c, 0x4)).lw(c);
        }
        c.cycles += 14;
        c.g[A0][0] = c.g[S0][0];
        Addr(strip_s0(c, 0x10)).sw(c, 0);
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
        c.g[A2][0] = c.g[S2][0];
        let after = c.g[V0][0];
        Addr(strip_s0(c, 0x4)).sw(c, after);
        let tag = c.g[V1][0];
        Addr(strip_s0(c, 0xc)).sw(c, tag);
        Callee {
            addr: 0x0038_4fd0,
            f: ee::s12::f_00384fd0,
        }
        .call(c, 0x0038_511c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2BezierMan_FreeCaches(PS2BezierMan* self)` (0x38b0f8,
/// cPS2BezierMan_Purge): runs the stub at 0x38d660, then frees the blocks at
/// +0x440, +0x444 and +0x448 through EA_MemFree (0x317e98) where they are set.
pub fn ps2_bezier_man_free_caches(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x0038_d660,
            f: ee::s12::f_0038d660,
        }
        .call(c, 0x0038_b10c)?;
        c.g[A0][0] = Addr(bezier_s0(c, 0x440)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(bezier_s0(c, 0x444)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b120)?;
            c.g[A0][0] = Addr(bezier_s0(c, 0x444)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(bezier_s0(c, 0x448)).lw(c);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b134)?;
            c.g[A0][0] = Addr(bezier_s0(c, 0x448)).lw(c);
        }
        c.cycles += 2;
        let free_last = c.g[A0][0] != 0;
        frame.restore(c, S0, 0x10);
        if free_last {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0038_b148)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes;
    use crate::difftest::{check, put_vec, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes, in the machine's RAM.
    const OC1071_GATE: u32 = DATA + 0x0_f000;
    /// Written at `OC1071_GATE` by the fixtures that want the fakes.
    const OC1071_MAGIC: u32 = 0x1071_00de;

    /// A callee fake that answers only with the gate open, and otherwise runs
    /// the body it replaced.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1071_GATE) != OC1071_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc1071_fake_3e6574,
        OC1071_PREV_3E6574,
        0x003e_6574,
        ee::s14::f_003e6574__generated
    );
    gated_fake!(
        oc1071_fake_366e30,
        OC1071_PREV_366E30,
        0x0036_6e30,
        ee::s12::f_00366e30__generated
    );
    gated_fake!(
        oc1071_fake_384dc0,
        OC1071_PREV_384DC0,
        0x0038_4dc0,
        ee::s12::f_00384dc0__generated
    );
    gated_fake!(
        oc1071_fake_384fd0,
        OC1071_PREV_384FD0,
        0x0038_4fd0,
        ee::s12::f_00384fd0__generated
    );
    gated_fake!(
        oc1071_fake_317e98,
        OC1071_PREV_317E98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    /// Install the fakes above (once per process; each one is gated).
    fn oc1071_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc1071_fake_3e6574,
            &OC1071_PREV_3E6574,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00366E30),
            oc1071_fake_366e30,
            &OC1071_PREV_366E30,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00384DC0),
            oc1071_fake_384dc0,
            &OC1071_PREV_384DC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00384FD0),
            oc1071_fake_384fd0,
            &OC1071_PREV_384FD0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1071_fake_317e98,
            &OC1071_PREV_317E98,
        );
    }

    driven!(gen_destroy_btexinfo, ee::s12::f_00367310__generated);
    driven!(hand_destroy_btexinfo, texman_destroy_btexinfo_array);

    #[test]
    fn texman_destroy_btexinfo_array_matches() {
        check_events(
            gen_destroy_btexinfo,
            hand_destroy_btexinfo,
            64,
            &|c, seed| {
                let mut r = Rng(0x0036_7310 ^ seed as u64);
                scramble(c, &mut r);
                let mode = schedule(c, &mut r, seed, 0x0036_7310);
                // The loop is long: make the first back edges due, identically for both.
                c.next_event = c.cycles + 4 + (seed as u64 % 997);
                mode
            },
        );
    }

    #[test]
    fn texman_update_texture_data_matches() {
        oc1071_hook();
        check(
            ee::s12::f_00367d20__generated,
            texman_update_texture_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_7d20 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let gm = DATA + 0x1000;
                let idx = seed % 4;
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = idx as u64;
                // a2 zero or not: the first table call.
                c.g[A2][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                let rec = DATA + 0x2000;
                c.bus.write32(gm + idx * 4 + 8, rec);
                c.bus.write32(rec + 0x10, DATA + 0x3000);
                c.bus.write32(rec + 0x18, r.next_u32() % 32);
                c.bus.write32(rec + 0x1c, DATA + 0x3100);
                c.bus.write32(rec + 0x20, DATA + 0x3200);
                // Evict's record state: -1 (nothing freed), tag 9 or other.
                c.bus.write32(
                    rec + 0x28,
                    if seed % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
                c.bus
                    .write32(rec + 0xc, if seed % 2 == 0 { 9 } else { r.next_u32() % 8 });
                // a8 (sp+0) zero or not, a9 (sp+8) zero or not.
                let a8 = if seed % 2 == 0 { 0 } else { r.next_u32() };
                let a9 = if seed % 5 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(crate::difftest::SP, a8);
                c.bus.write32(crate::difftest::SP + 8, a9);
            },
        );
    }

    #[test]
    fn texman_evict_texture_matches() {
        oc1071_hook();
        check(
            ee::s12::f_00369098__generated,
            texman_evict_texture,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_9098 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let gm = DATA + 0x1000;
                let idx = seed % 4;
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = idx as u64;
                let rec = DATA + 0x2000;
                c.bus.write32(gm + idx * 4 + 8, rec);
                // seed % 3: 0 nothing freed, 1 tag 9 (the second list), 2 the first list.
                let state = if seed % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.bus.write32(rec + 0x28, state);
                c.bus
                    .write32(rec + 0xc, if seed % 3 == 1 { 9 } else { r.next_u32() % 8 });
            },
        );
    }

    #[test]
    fn graphics_man_empty_hook_matches() {
        check(
            ee::s12::f_00369ff0__generated,
            graphics_man_empty_hook,
            64,
            &|c, seed| {
                let mut r = Rng(0x0036_9ff0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn emitter_set_desc_linear_path_matches() {
        check(
            ee::s12::f_0036ce80__generated,
            emitter_set_desc_linear_path,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_ce80 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2010);
                let a = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                let b = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, DATA + 0x2000, a);
                put_vec(c, DATA + 0x2010, b);
            },
        );
    }

    #[test]
    fn emitter_set_desc_accel_scaled_matches() {
        check(
            ee::s12::f_0036cfa8__generated,
            emitter_set_desc_accel_scaled,
            200,
            &|c, seed| {
                let mut r = Rng(0x0036_cfa8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                let v = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                put_vec(c, DATA + 0x2000, v);
                c.bus.write32(DATA + 0x1000 + 0xc, r.edge_f32());
                c.bus.write32(DATA + 0x1000 + 0x4c, r.edge_f32());
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn line_strip_begin_matches() {
        oc1071_hook();
        check(
            ee::s12::f_003850a8__generated,
            line_strip_begin,
            200,
            &|c, seed| {
                let mut r = Rng(0x0038_50a8 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let ctx = DATA + 0x1000;
                c.g[A0][0] = sx(ctx);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x2100);
                // +8 open strip (0: start one), +0xc previous strip tag (0: none).
                c.bus
                    .write32(ctx + 0x8, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus
                    .write32(ctx + 0xc, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(ctx + 0x4, DATA + 0x4000 + (seed % 8) * 0x40);
                c.bus.write32(ctx + 0x10, r.next_u32());
            },
        );
    }

    #[test]
    fn ps2_bezier_man_free_caches_matches() {
        oc1071_hook();
        check(
            ee::s12::f_0038b0f8__generated,
            ps2_bezier_man_free_caches,
            200,
            &|c, seed| {
                let mut r = Rng(0x0038_b0f8 ^ seed as u64);
                scramble(c, &mut r);
                c.bus.write32(OC1071_GATE, OC1071_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                // Each of the three blocks is set or clear, by the seed's bits.
                for (i, off) in [0x440u32, 0x444, 0x448].iter().enumerate() {
                    let block = if (seed >> i) & 1 == 1 {
                        DATA + 0x5000 + i as u32 * 0x100
                    } else {
                        0
                    };
                    c.bus.write32(this + off, block);
                }
            },
        );
    }
}
