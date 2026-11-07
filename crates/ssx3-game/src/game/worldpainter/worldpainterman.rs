//! `src/worldpainter/worldpainterman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::camera::gp_at;
pub(crate) use crate::game::visualfx::boardwakefx::copy_render_state_2 as copy_render_state;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_b130,
        cworld_painter_man_ctor,
        "cWorldPainterMan_ctor",
    ),
    (
        0x002b_b6e0,
        world_painter_man_debug_draw_current,
        "cWorldPainterMan_DebugDrawCurrent",
    ),
    (
        0x002b_b6b8,
        c_world_painter_man_debug_draw,
        "cWorldPainterMan_DebugDraw",
    ),
];

// ---------------------------------------------------------------------------
// World painter files.

/// `cWorldPainterMan* cWorldPainterMan_ctor(cWorldPainterMan* self)`
/// (0x2bb130): clear +4, return self.
pub fn cworld_painter_man_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).off(0x4).sw(c, 0);
    abi::ret(c);
}

pub(crate) mod rctx {
    #[cfg(test)]
    pub(crate) use crate::game::visualfx::boardwakefx::rctx::VPTR;

    pub(crate) use crate::game::visualfx::boardwakefx::rctx::STATE_TOP;
}

// ---------------------------------------------------------------------------
// cWorldPainterMan_DebugDrawCurrent (0x2bb6e0)

/// `void cWorldPainterMan_DebugDrawCurrent(cWorldPainterMan *this)`
/// (0x2bb6e0): the paint-map debug overlay. When enabled (`gp+0x13c4`),
/// take the current course's paint set (`this->sets[gp+0x770]`, 12 bytes
/// each, at this+4), pick its layer `gp+0x13c8` (clamped to the last; <= 0
/// the first), and if that layer's map has a quadtree, push the template
/// render state (0x501420), clear five 0x30-byte vertex records on the
/// stack, compute the map's bounds (origin, origin + 32768/scale) and draw
/// the tree from its root (`WorldPainter_DebugDrawNode`); pop the state.
pub fn world_painter_man_debug_draw_current(c: &mut Cpu) {
    const DRAW_NODE: Callee = Callee {
        addr: 0x002b_b928,
        f: ee::s08::f_002bb928,
    };
    const STATE_TEMPLATE: u32 = 0x0050_1420;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x130);
        c.g[V0][0] = Addr(gp_at(c, 0x13c4)).lw(c);
        frame.save(c, S0, 0x120);
        c.g[A3][0] = c.g[A0][0];
        let this = r32(c, A3);
        frame.save_ra(c, 0x110);
        'restore_s0: {
            'out: {
                if c.g[V0][0] == 0 {
                    break 'restore_s0;
                }
                // The paint set of the current course.
                c.cycles += 7;
                c.g[V0][0] = Addr(gp_at(c, 0x770)).lw(c);
                c.g[V1][0] = 0xc;
                c.g[A0][0] = Addr(this.wrapping_add(4)).lw(c);
                c.mult(0, V0, V0, V1);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                frame.restore(c, S0, 0x120);
                if c.g[A0][0] == 0 {
                    break 'out;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(8)).lw(c);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
                c.g[A1][0] = c.g[V0][0];
                if c.g[A0][0] == 0 {
                    break 'out;
                }
                // The layer: gp+0x13c8 clamped to count-1; <= 0 the first.
                let set = r32(c, A1);
                let layers = r32(c, A0);
                c.cycles += 4;
                c.g[V0][0] = Addr(set.wrapping_add(4)).lw(c);
                c.g[V1][0] = Addr(gp_at(c, 0x13c8)).lw(c);
                let want = c.g[V1][0] as i64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let last_index = if want > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = (want < c.g[V0][0] as i64) as u64;
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(set.wrapping_add(4)).lw(c);
                        true
                    } else {
                        false
                    }
                } else {
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[V0][0] = Addr(set.wrapping_add(4)).lw(c);
                        true
                    } else {
                        false
                    }
                };
                if last_index {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.cycles += 1;
                    c.g[V0][0] = sx(layers.wrapping_add(c.g[V0][0] as u32));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    if want > 0 {
                        c.cycles += 1;
                        c.g[V0][0] = sx(layers.wrapping_add(c.g[V0][0] as u32));
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[A0][0];
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                frame.restore(c, S0, 0x120);
                if c.g[V0][0] == 0 {
                    break 'out;
                }
                c.cycles += 3;
                c.g[T2][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[S0][0] = Addr(gp_at(c, 0x2a90)).lw(c);
                if c.g[T2][0] == 0 {
                    break 'restore_s0;
                }
                let map = r32(c, T2);
                let rc = r32(c, S0);

                // Push the template render state.
                c.cycles += 31;
                c.g[A0][0] = 0x50_0000;
                c.g[A1][0] = 0x4;
                c.g[T0][0] = sx(frame.at(c, 0xf0));
                let top = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32;
                c.g[V0][0] = sx(top);
                c.g[T1][0] = sx(frame.at(c, 0x100));
                c.g[A2][0] = u64::MAX;
                copy_render_state(c, top, top.wrapping_add(0x14), V1, T3, T4);
                c.g[V1][0] = Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c);
                let prev = r32(c, V1);
                c.g[V0][0] = sx(prev.wrapping_add(0x14));
                Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
                c.g[T5][0] = sx(STATE_TEMPLATE);
                copy_render_state(c, STATE_TEMPLATE, prev.wrapping_add(0x14), V0, T3, T4);

                // A delay loop the compiler left from the unrolled clear.
                loop {
                    c.cycles += 7;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
                    let more = c.g[A1][0] != c.g[A2][0];
                    c.g[A0][0] = 0xff;
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b800);
                    }
                }
                // Five vertex records: xyz 0, w 1, colour (0, 0xff, 0, 0xff).
                c.cycles += 5;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.g[V1][0] = c.g[SP][0];
                c.g[A1][0] = sx(frame.at(c, 0xf0));
                Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A0][0]);
                loop {
                    c.cycles += 11;
                    let v = r32(c, V1);
                    Addr(v.wrapping_add(0x10)).sw(c, 0);
                    Addr(v.wrapping_add(0x14)).sw(c, c.g[A0][0]);
                    Addr(v.wrapping_add(0x18)).sw(c, 0);
                    c.swc1(0, v.wrapping_add(0x8));
                    Addr(v.wrapping_add(0xc)).sw(c, 0);
                    Addr(v).sw(c, 0);
                    Addr(v.wrapping_add(0x4)).sw(c, 0);
                    c.g[V1][0] = sx(v.wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    Addr(r32(c, V1).wrapping_add(0x1c)).sw(c, c.g[A0][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b830);
                    }
                }

                // Bounds: min = (int)origin (sp+0xf0), max = (int)(origin +
                // 32768/scale) (sp+0x100); draw from the root node.
                c.cycles += 47;
                c.lwc1(0, map.wrapping_add(0x4));
                c.g[A0][0] = c.g[A3][0];
                c.lwc1(1, map.wrapping_add(0x8));
                c.g[A1][0] = c.g[T2][0];
                c.cvt_w_s(2, 0);
                c.mfc1(V0, 2);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 4);
                c.cvt_w_s(0, 1);
                c.mfc1(V1, 0);
                let at = |c: &Cpu, off: u32| sp_at(c, off);
                Addr(at(c, 0xf8)).sw(c, 0);
                c.swc1(4, at(c, 0xfc));
                c.g[A3][0] = c.g[SP][0];
                c.mtc1(V0, 1);
                c.cvt_s_w(1, 1);
                c.g[AT][0] = 0x4700_0000;
                c.mtc1(AT, 2);
                c.mtc1(V1, 0);
                c.cvt_s_w(0, 0);
                c.swc1(1, at(c, 0xf0));
                c.swc1(0, at(c, 0xf4));
                c.lwc1(1, map);
                c.lwc1(0, map.wrapping_add(0x8));
                c.div_s(2, 2, 1);
                c.lwc1(3, map.wrapping_add(0x4));
                c.swc1(4, at(c, 0x10c));
                Addr(at(c, 0x108)).sw(c, 0);
                c.add_s(0, 2, 0);
                c.add_s(2, 2, 3);
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.cvt_w_s(0, 2);
                c.mfc1(V1, 0);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.mtc1(V1, 1);
                c.cvt_s_w(1, 1);
                c.swc1(0, at(c, 0x104));
                c.swc1(1, at(c, 0x100));
                c.g[A2][0] = Addr(map.wrapping_add(0x14)).lhu(c);
                c.g[V0][0] = Addr(map.wrapping_add(0x20)).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                DRAW_NODE.call(c, 0x002b_b90c)?;
                c.g[V0][0] =
                    sx((Addr(rc.wrapping_add(rctx::STATE_TOP)).lw(c) as u32).wrapping_sub(0x14));
                Addr(rc.wrapping_add(rctx::STATE_TOP)).sw(c, c.g[V0][0]);
                break 'restore_s0;
            }
            // Out early (s0 already restored).
            c.cycles += 3;
            frame.restore_ra(c, 0x110);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x120);
        c.cycles += 3;
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

/// The address `sp + off`.
#[inline(always)]
pub(crate) fn sp_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// cWorldPainterMan_DebugDraw (0x2bb6b8)

pub(crate) const WPM_DEBUG_DRAW_CURRENT: Callee = Callee {
    addr: 0x002b_b6e0,
    f: ee::s08::cWorldPainterMan_DebugDrawCurrent,
};

/// World (`gp-0x848`) -> world painter manager (+0x84) -> current painter
/// (+0x10); the painter's +8 is the node set drawn.
pub(crate) const WPM_WORLD: i32 = -0x848;

pub(crate) const WORLD_PAINTER_MAN: u32 = 0x84;

pub(crate) const WPM_CURRENT: u32 = 0x10;

pub(crate) const PAINTER_NODES: u32 = 0x8;

/// `void cWorldPainterMan_DebugDraw(void)` (0x2bb6b8): debug-draw the
/// world's current world painter
/// (`DebugDrawCurrent(painter->nodes, painter)`).
pub fn c_world_painter_man_debug_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(WPM_WORLD as u32)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, V0)).off(WORLD_PAINTER_MAN).lw(c);
        c.g[A1][0] = Addr(r32(c, V1)).off(WPM_CURRENT).lw(c);
        c.g[A0][0] = Addr(r32(c, A1)).off(PAINTER_NODES).lw(c);
        WPM_DEBUG_DRAW_CURRENT.call(c, 0x002b_b6d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    #[test]
    fn cworld_painter_man_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bb130,
            cworld_painter_man_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0xb130 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::fill;

    /// cWorldPainterMan_DebugDrawCurrent: overlay off/on; no paint sets or
    /// a null set; every layer-clamp case (wanted layer <= 0, in range, past
    /// the last; one-layer sets); a null layer or map; a full draw of a leaf
    /// root node (WorldPainter_DebugDrawNode for real, its two virtual calls
    /// faked, sometimes unwinding).
    #[test]
    fn world_painter_man_debug_draw_current_matches() {
        covered(
            ee::s08::f_002bb6e0__generated,
            world_painter_man_debug_draw_current,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bb6e0, 2);
                c.pc = 0x002b_b6e0;
                let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
                let this = DATA;
                let (sets, set, rc) = (DATA + 0x100, DATA + 0x200, DATA + 0x5000);
                c.g[A0][0] = sx(this);
                c.bus.write32(gp(0x13c4), (seed % 8 != 0) as u32);
                let course = r.next_u32() % 3;
                c.bus.write32(gp(0x770), course);
                c.bus.write32(
                    this + 4,
                    if seed % 13 == 5 {
                        (course * 12).wrapping_neg()
                    } else {
                        sets
                    },
                );
                let set_ptr = if seed % 17 == 3 { 0xffff_fff8 } else { set };
                c.bus.write32(sets + course * 12 + 8, set_ptr);
                let n = 1 + r.next_u32() % 5;
                c.bus.write32(set + 4, n);
                c.bus
                    .write32(gp(0x13c8), (r.next_u32() % 9).wrapping_sub(2));
                for i in 0..5 {
                    let layer = DATA + 0x400 + 0x40 * i;
                    let map = DATA + 0x800 + 0x40 * i;
                    let nodes = DATA + 0xc00;
                    c.bus.write32(
                        set + 8 + 4 * i,
                        if r.next_u32() % 8 == 0 { 0 } else { layer },
                    );
                    c.bus
                        .write32(layer, if r.next_u32() % 8 == 0 { 0 } else { map });
                    let scale = 0.5 + (r.next_u32() % 8) as f32 * 0.5;
                    c.bus.write32(map, scale.to_bits());
                    c.bus.write32(map + 4, r.f32(2000.0));
                    c.bus.write32(map + 8, r.f32(2000.0));
                    c.bus.write16(map + 0x14, (r.next_u32() % 4) as u16);
                    c.bus.write32(map + 0x20, nodes);
                    for k in 0..4 {
                        c.bus.write16(nodes + 8 * k, (r.next_u32() as u16) & !1);
                        // leaves
                    }
                }
                // DebugDrawNode's leaf: the world chain to a vtable (+0x28) and
                // the renderer's vtable (+0x248).
                let w = DATA + 0x3000;
                c.bus.write32(gp(-0x848), w);
                c.bus.write32(w + 0x84, w + 0x100);
                c.bus.write32(w + 0x100 + 0xc, w + 0x200);
                c.bus.write32(w + 0x200 + 0x40, w + 0x300);
                c.bus.write32(w + 0x300 + 0x18, w + 0x400);
                c.bus.write32(w + 0x400 + 0x6c0, w + 0x1000);
                c.bus
                    .write16(w + 0x1000 + 0x28, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(w + 0x1000 + 0x2c, FAKE);
                c.bus.write32(gp(0x2a90), rc);
                c.bus.write32(rc + rctx::STATE_TOP, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 0x10);
                fill(c, &mut r, 0x0050_1420, 8);
                c.bus.write32(rc + rctx::VPTR, rc + 0x1100);
                c.bus
                    .write16(rc + 0x1100 + 0x248, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(rc + 0x1100 + 0x24c, FAKE);
            },
        );
    }

    // ------------------------------------------------ cWorldPainterMan_DebugDraw

    /// The world's painter manager and current painter; the real
    /// `DebugDrawCurrent` runs with the debug draw switched off
    /// (`gp+0x13c4` = 0, as in the game).
    #[test]
    fn c_world_painter_man_debug_draw_matches() {
        check(
            ee::s08::f_002bb6b8__generated,
            c_world_painter_man_debug_draw,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_b6b8, 0);
                c.pc = 0x002b_b6b8;
                let world = DATA + 0x1_0000;
                let man = DATA + 0x1_1000;
                let painter = DATA + 0x1_2000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + WORLD_PAINTER_MAN, man);
                c.bus.write32(man + WPM_CURRENT, painter);
                c.bus.write32(painter + PAINTER_NODES, r.next_u32());
                c.bus.write32(GAME_GP + 0x13c4, 0);
            },
        );
    }
}
