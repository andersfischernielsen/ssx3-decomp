//! `src/worldpainter/worldpainterman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
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
    (
        0x002c_03e8,
        world_painter_query_reset_all,
        "WorldPainterQuery_ResetAll",
    ),
    (
        0x002b_b140,
        cworld_painter_man_dtor,
        "cWorldPainterMan_dtor",
    ),
    (
        0x002b_b1d0,
        cworld_painter_man_alloc_files,
        "cWorldPainterMan_AllocFiles",
    ),
    (
        0x002b_b2b8,
        cworld_painter_man_update,
        "cWorldPainterMan_Update",
    ),
    (
        0x002b_b318,
        cworld_painter_man_load_file,
        "cWorldPainterMan_LoadFile",
    ),
    (
        0x002b_b348,
        cworld_painter_man_set_file_data,
        "cWorldPainterMan_SetFileData",
    ),
    (
        0x002b_bbf8,
        c_world_painter_man_debug_printf,
        "cWorldPainterMan_DebugPrintf",
    ),
    (
        0x002b_bc38,
        c_world_painter_man_static_init_destroy,
        "cWorldPainterMan_StaticInitDestroy",
    ),
    (
        0x002b_c4e0,
        wp_man_global_ctors,
        "cWorldPainterMan_GlobalCtors",
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

/// `WorldPainterQuery_ResetList` (0x2c03a8).
pub(crate) const OC762_QUERY_RESET_LIST: Callee = Callee {
    addr: 0x002c_03a8,
    f: ee::s08::f_002c03a8,
};

/// `void WorldPainterQuery_ResetAll(void)` (0x2c03e8): forwards to
/// `WorldPainterQuery_ResetList()` (7-cycle wrapper, 0x10 frame holding `ra`).
pub fn world_painter_query_reset_all(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(OC762_QUERY_RESET_LIST), 0x002c_03f8)
}

/// `cWorldPainterMan_AllocFiles(WPArr* self, int n)` (0x2bb1d0): destroys the
/// files of `self`'s current array (the count is the word in front of the
/// array; the block goes back to the allocator), then allocates an array of
/// `n` 12-byte files with a count word in front, stores `n` at `self+0` and
/// the array at `self+4`, and constructs the files.
pub fn cworld_painter_man_alloc_files(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = c.g[A1][0] == 0u64;
        c.g[S3][0] = 0x480000;
        if !empty {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0xc;
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002b_b390,
                        f: ee::s08::f_002bb390,
                    }
                    .call(c, 0x002b_b22c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x002b_b220);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    break;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x002b_b244)?;
        }
        c.cycles += 14;
        c.g[A0][0] = 0xc;
        c.sw(c.g[S1][0] as u32, c.g[S2][0]);
        c.mult(0, 4, 18, 4);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c68));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0u64;
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7e30, 0x002b_b26c)?;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[S2][0]);
        let none = c.g[S2][0] == 0u64;
        c.g[S0][0] = c.g[S3][0];
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0xffff_ffff_ffff_ffff;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                Callee {
                    addr: 0x002b_b370,
                    f: ee::s08::f_002bb370,
                }
                .call(c, 0x002b_b28c)?;
                let more = c.g[S1][0] != c.g[S2][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b280);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        c.sw(c.g[S4][0] as u32, c.g[S3][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan_SetFileData(WPArr* self, int idx, int arg)` (0x2bb348):
/// `WPFile_SetData(self->arr + idx * 12, idx, arg)`.
pub fn cworld_painter_man_set_file_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 2, 5, 2);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Callee {
            addr: 0x002b_b570,
            f: ee::s08::f_002bb570,
        }
        .call(c, 0x002b_b364)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan_DebugPrintf` (0x002b_bbf8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn c_world_painter_man_debug_printf(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `cWorldPainterMan_StaticInitDestroy(int init, int priority)` (0x002bbc38): the world
/// painter manager's static initialiser. Only for `priority == 0xffff` and `init != 0`: fills
/// the 14 `sFlags` entries (a byte in the gp area plus 8-byte records at 0x4a5358..) and
/// the 62 `sColor` records at 0x4d42b8.. with constant stores, then returns; any other
/// arguments do nothing. The store sequence is kept as the generated body schedules it
/// (scratch registers and cycles included).
pub fn c_world_painter_man_static_init_destroy(c: &mut Cpu) {
    'b_2bc4b0: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0xffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_2bc4b0;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2bc4b0;
            } else {
                c.cycles += 528;
                c.g[V1][0] = 0x2;
                c.g[A0][0] = 0x3;
                c.g[A1][0] = 0x4;
                c.g[A2][0] = 0x5;
                c.g[A3][0] = 0x6;
                c.g[T0][0] = 0x7;
                c.g[T1][0] = 0x8;
                c.g[T2][0] = 0x9;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2278), c.g[V0][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2280), c.g[V1][0]);
                c.g[V0][0] = 0xa;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2288), c.g[A0][0]);
                c.g[S2][0] = 0xb;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2290), c.g[A1][0]);
                c.g[S3][0] = 0xc;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2298), c.g[A2][0]);
                c.g[T9][0] = 0xd;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22a0), c.g[A3][0]);
                c.g[S4][0] = 0x4d0000;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22a8), c.g[T0][0]);
                c.g[S5][0] = 0x4d0000;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22b0), c.g[T1][0]);
                c.g[S6][0] = 0x4d0000;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22b8), c.g[T2][0]);
                c.g[T5][0] = sx((c.g[S6][0] as u32).wrapping_add(0x42e8));
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2270), 0u64);
                c.g[T7][0] = sx((c.g[S5][0] as u32).wrapping_add(0x42d8));
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22c0), c.g[V0][0]);
                c.g[FP][0] = 0x4d0000;
                c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2278));
                c.g[S7][0] = 0x4d0000;
                c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2280));
                c.g[T8][0] = 0x4d0000;
                c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2288));
                c.g[V1][0] = c.g[V1][0] | 0x100;
                c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2290));
                c.g[A0][0] = c.g[A0][0] | 0x200;
                c.g[A3][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2298));
                c.g[A1][0] = c.g[A1][0] | 0x300;
                c.g[T0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x22a0));
                c.g[A2][0] = c.g[A2][0] | 0x400;
                c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x22a8));
                c.g[A3][0] = c.g[A3][0] | 0x500;
                c.g[T2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x22b0));
                c.g[T0][0] = c.g[T0][0] | 0x600;
                c.g[T3][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x22b8));
                c.g[T1][0] = c.g[T1][0] | 0x700;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2270));
                c.g[T2][0] = c.g[T2][0] | 0x800;
                c.g[T4][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x22c0));
                c.g[T3][0] = c.g[T3][0] | 0x900;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2270), c.g[V0][0]);
                c.g[S0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x42b8));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2278), c.g[V1][0]);
                c.g[T4][0] = c.g[T4][0] | 0xa00;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2280), c.g[A0][0]);
                c.g[V0][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2288), c.g[A1][0]);
                c.g[V1][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2290), c.g[A2][0]);
                c.g[A0][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2298), c.g[A3][0]);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4318));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22a0), c.g[T0][0]);
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4338));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22a8), c.g[T1][0]);
                c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x42c8));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22b0), c.g[T2][0]);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x42f8));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22b8), c.g[T3][0]);
                c.g[T2][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x226c), 0u64);
                c.g[T0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x4348));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2268), 0u64);
                c.g[T3][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22c0), c.g[T4][0]);
                c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4358));
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22c8), c.g[S2][0]);
                c.g[T4][0] = 0x4d0000;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22d0), c.g[S3][0]);
                c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4328));
                c.sb((c.g[GP][0] as u32).wrapping_add(0x22d8), c.g[T9][0]);
                c.g[T2][0] = sx((c.g[T4][0] as u32).wrapping_add(0x4368));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4410));
                c.g[V1][0] = c.g[S3][0] & 0xff;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x440c));
                c.g[A0][0] = c.g[T9][0] & 0xff;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4408));
                c.g[V1][0] = c.g[V1][0] | 0xc00;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x22c8));
                c.g[A0][0] = c.g[A0][0] | 0xd00;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4404));
                c.g[T9][0] = 0x4d0000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V0][0] = c.g[V0][0] | 0xb00;
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x42c8));
                c.g[T4][0] = sx((c.g[T9][0] as u32).wrapping_add(0x4388));
                c.swc1(6, (c.g[S5][0] as u32).wrapping_add(0x42d8));
                c.g[S3][0] = 0x4d0000;
                c.swc1(7, (c.g[S6][0] as u32).wrapping_add(0x42e8));
                c.g[S5][0] = 0x4d0000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4400));
                c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x43a8));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x43fc));
                c.g[S6][0] = 0x4d0000;
                c.swc1(3, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[T8][0] as u32).wrapping_add(0x4308));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 2);
                c.g[T3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4378));
                c.swc1(4, (c.g[S7][0] as u32).wrapping_add(0x42f8));
                c.g[S5][0] = sx((c.g[S6][0] as u32).wrapping_add(0x43b8));
                c.swc1(1, (c.g[T8][0] as u32).wrapping_add(0x4308));
                c.g[T9][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22c8), c.g[V0][0]);
                c.g[S7][0] = 0x4d0000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22d0), c.g[V1][0]);
                c.g[S3][0] = sx((c.g[T9][0] as u32).wrapping_add(0x4398));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x22d8), c.g[A0][0]);
                c.g[S6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x43c8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x42b8));
                c.g[T8][0] = 0x4d0000;
                c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S7][0] = sx((c.g[T8][0] as u32).wrapping_add(0x43d8));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T9][0] = 0x4d0000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x43f8));
                c.g[T8][0] = sx((c.g[T9][0] as u32).wrapping_add(0x43e8));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x43f4));
                c.g[V0][0] = 0x4d0000;
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T9][0] = sx((c.g[V0][0] as u32).wrapping_add(0x43f8));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x43f0));
                c.g[V1][0] = 0x4d0000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x43ec));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4408));
                c.swc1(10, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(11, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4418));
                c.swc1(8, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.g[T5][0] = 0x4d0000;
                c.swc1(9, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x4428));
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = 0x4d0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[T5][0] = sx((c.g[T6][0] as u32).wrapping_add(0x4458));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4d0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4468));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[S1][0] = 0x4d0000;
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4488));
                c.g[T7][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4498));
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x43e8));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = 0x4d0000;
                c.swc1(16, (c.g[FP][0] as u32).wrapping_add(0x4318));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44b8));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x43e4));
                c.g[FP][0] = 0x4d0000;
                c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4518));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x43e0));
                c.swc1(5, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4d0000;
                c.swc1(6, (c.g[FP][0] as u32).wrapping_add(0x4328));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x43dc));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x43d8));
                c.swc1(4, (c.g[FP][0] as u32).wrapping_add(0x4338));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x43d4));
                c.g[FP][0] = 0x4d0000;
                c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x4348));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x43d0));
                c.g[FP][0] = 0x4d0000;
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x43cc));
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x43c8));
                c.swc1(15, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x43c4));
                c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(13, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x43c0));
                c.g[A2][0] = 0x4d0000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x43bc));
                c.swc1(7, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.swc1(9, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x43b8));
                c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4548));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x43b4));
                c.g[A2][0] = 0x4d0000;
                c.swc1(10, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.swc1(11, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4358));
                c.g[T0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4568));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4368));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x45a8));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x43b0));
                c.g[FP][0] = 0x4d0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x43ac));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4378));
                c.swc1(14, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4388));
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4398));
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = 0x4d0000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T2][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(15, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x4d0000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = sx((c.g[T2][0] as u32).wrapping_add(0x45d8));
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = sx((c.g[T3][0] as u32).wrapping_add(0x45e8));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x45f8));
                c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0xc));
                c.g[S2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4608));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[T4][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4628));
                c.g[S3][0] = 0x4d0000;
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x43a8));
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S3][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x43b8));
                c.sw((c.g[S5][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x43c8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x43d8));
                c.swc1(2, (c.g[S7][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x43e8));
                c.swc1(2, (c.g[T8][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x43a8));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x43a4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x43f8));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4408));
                c.g[FP][0] = 0x4d0000;
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S3][0] = 0x4d0000;
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x43a0));
                c.g[V0][0] = 0x4d0000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4418));
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x4d0000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[S4][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S4][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[S5][0] = c.g[S3][0];
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4448));
                c.swc1(2, (c.g[S7][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = 0x4d0000;
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x8));
                c.swc1(2, (c.g[T8][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = 0x4d0000;
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x8));
                c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0x4));
                c.g[T8][0] = 0x4d0000;
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x8));
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x439c));
                c.g[T9][0] = c.g[T8][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x4478));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4398));
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x4428));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4394));
                c.g[V0][0] = 0x4d0000;
                c.swc1(2, (c.g[T6][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4390));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x438c));
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4388));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4438));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4438));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4384));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4448));
                c.swc1(5, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4380));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4458));
                c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x437c));
                c.g[T5][0] = 0x4d0000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4378));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x4468));
                c.swc1(8, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = 0x4d0000;
                c.swc1(9, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4374));
                c.g[T6][0] = 0x4d0000;
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x4478));
                c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x4528));
                c.swc1(10, (c.g[T9][0] as u32).wrapping_add(0x8));
                c.g[T8][0] = c.g[S7][0];
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0xc));
                c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(0x44e8));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4370));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4488));
                c.swc1(11, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4498));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[T5][0];
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x44a8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4508));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x436c));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44a8));
                c.swc1(12, (c.g[T9][0] as u32).wrapping_add(0x4));
                c.g[S5][0] = 0x4d0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = c.g[S5][0];
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x4d0000;
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[S3][0] = c.g[S0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x4d0000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4368));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x44c8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x44d8));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4364));
                c.g[T9][0] = 0x4d0000;
                c.swc1(3, (c.g[T5][0] as u32).wrapping_add(0x4508));
                c.g[FP][0] = c.g[T9][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4360));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x44f8));
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x4d0000;
                c.swc1(1, (c.g[T6][0] as u32).wrapping_add(0xc));
                c.g[T5][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x435c));
                c.g[A0][0] = 0x4d0000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4358));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x44b8));
                c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x4d0000;
                c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x44c8));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4d0000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4354));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x44d8));
                c.swc1(6, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.g[S5][0] = 0x4d0000;
                c.sw((c.g[S6][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x44e8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0xc));
                c.g[S7][0] = 0x4d0000;
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x44f8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T9][0] = 0x4d0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4350));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x434c));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4518));
                c.swc1(7, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x4d0000;
                c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4348));
                c.g[S1][0] = c.g[S0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4528));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4558));
                c.swc1(9, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x45b8));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x8));
                c.g[S3][0] = 0x4d0000;
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x4));
                c.g[S6][0] = c.g[S5][0];
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4578));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.g[T8][0] = c.g[S7][0];
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(0x4588));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.g[FP][0] = c.g[T9][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4598));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x4d0000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4344));
                c.g[T6][0] = c.g[T5][0];
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4538));
                c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x4538));
                c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4340));
                c.swc1(12, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x433c));
                c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[T6][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4338));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4548));
                c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = 0x4d0000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4558));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x4d0000;
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4568));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4578));
                c.sw((c.g[S6][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S5][0] = 0x4d0000;
                c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x4588));
                c.sw((c.g[T8][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x4598));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4334));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x45a8));
                c.swc1(5, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x45b8));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = 0x4d0000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[S1][0] = 0x4d0000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x4));
                c.sw((c.g[T8][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.sw((c.g[FP][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4330));
                c.g[A2][0] = 0x4d0000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T5][0] = c.g[A2][0];
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x432c));
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x45c8));
                c.swc1(13, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4328));
                c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4324));
                c.swc1(2, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4320));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x45c8));
                c.swc1(4, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x431c));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4318));
                c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x45d8));
                c.swc1(5, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.swc1(6, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4314));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4310));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x45e8));
                c.swc1(7, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(8, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x430c));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4308));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x45f8));
                c.swc1(9, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(10, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4304));
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4300));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4608));
                c.swc1(11, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.swc1(12, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[S2][0] = 0x4d0000;
                c.g[S3][0] = c.g[S2][0];
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x4618));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4618));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4628));
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_2bc4b0;
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

/// `cWorldPainterMan::~cWorldPainterMan(int flags)` (0x2bb140): destroys the
/// WPFile entries of the array at `this+4` (their count is the word before
/// the array, 12-byte stride, last entry first), frees the array (0x317e98),
/// and deletes `this` (0x317e50) when flags bit 0 is set.
pub fn cworld_painter_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        let empty = c.g[A1][0] == 0u64;
        c.g[V0][0] = 0xc;
        'done: {
            if empty {
                break 'done;
            }
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] != c.g[S0][0] {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002b_b390,
                        f: ee::s08::f_002bb390,
                    }
                    .call(c, 0x002b_b18c)?;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002b_b180);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    break;
                }
            } else {
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x002b_b1a4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        let delete_this = c.g[V0][0] != 0u64;
        frame.restore(c, S0, 0x30);
        if delete_this {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002b_b1b8)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan::Update(this)` (0x2bb2b8): polls each of the WPFile
/// entries (the count is the word at `this`, the array at `this+4`, 12 bytes
/// apart) through `WPFile_PollLoad`.
pub fn cworld_painter_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0u64;
        c.g[S0][0] = 0u64;
        'done: {
            if empty {
                break 'done;
            }
            c.cycles += 1;
            c.g[S2][0] = 0u64;
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                Callee {
                    addr: 0x002b_b3f0,
                    f: ee::s08::f_002bb3f0,
                }
                .call(c, 0x002b_b2f0)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b2e0);
                    }
                    continue;
                }
                break 'done;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan::LoadFile(this, index, flags)` (0x2bb318): starts the
/// load of WPFile entry `index` (12-byte stride in the array at `this+4`)
/// through `WPFile_BeginLoad`.
pub fn cworld_painter_man_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Callee {
            addr: 0x002b_b4d8,
            f: ee::s08::f_002bb4d8,
        }
        .call(c, 0x002b_b338)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cWorldPainterMan_GlobalCtors(void)` (0x2bc4e0): the static
/// constructor of the world painter manager, `StaticInitDestroy(1, 0xffff)`.
pub fn wp_man_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x002b_bc38,
            f: ee::s08::f_002bbc38,
        }
        .call(c, 0x002b_c4f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::sound::bankmanager::tests::oc762_install_fakes;

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

    /// Control word of this module's gated fakes (they only act when the
    /// machine's RAM holds the magic here).
    pub(crate) const OC762_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC762_MAGIC: u32 = 0x4f43_3736;

    /// Start a test machine: scrambled registers, fakes armed.
    pub(crate) fn oc762_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC762_CTRL, OC762_MAGIC);
        r
    }

    /// `WorldPainterQuery_ResetAll`: the one forwarded call, answered or unwound.
    #[test]
    fn world_painter_query_reset_all_matches() {
        oc762_install_fakes();
        covered(
            ssx3_ee::s08::f_002c03e8__generated,
            world_painter_query_reset_all,
            100,
            &|c, seed| {
                oc762_machine(c, seed, 0x002c_03e8, 1);
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::RA as RET_ADDR;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::leaf_val;
    pub(crate) use crate::game::util::locale::tests::oc1498_gated_fake;
    pub(crate) use crate::game::util::locale::tests::OC1498_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words of the tests of this module (one per test, in the fixture
    /// area above the call log).
    pub(crate) const OC1498_AF_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const OC1498_SRF_GATE: u32 = DATA + 0x20_0100;

    // cWorldPainterMan_AllocFiles: callees WPFile_dtor, WPFile_ctor, operator new, free.
    oc1498_gated_fake!(
        oc1498_af_dtor,
        OC1498_AF_PREV_DTOR,
        OC1498_AF_GATE,
        4,
        ee::s08::f_002bb390__generated,
        0x002b_b390
    );

    oc1498_gated_fake!(
        oc1498_af_ctor,
        OC1498_AF_PREV_CTOR,
        OC1498_AF_GATE,
        4,
        ee::s08::f_002bb370__generated,
        0x002b_b370
    );

    oc1498_gated_fake!(
        oc1498_af_new,
        OC1498_AF_PREV_NEW,
        OC1498_AF_GATE,
        4,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );

    oc1498_gated_fake!(
        oc1498_af_free,
        OC1498_AF_PREV_FREE,
        OC1498_AF_GATE,
        4,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );

    pub(crate) fn oc1498_af_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB390),
            oc1498_af_dtor,
            &OC1498_AF_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB370),
            oc1498_af_ctor,
            &OC1498_AF_PREV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1498_af_new,
            &OC1498_AF_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1498_af_free,
            &OC1498_AF_PREV_FREE,
        );
    }

    /// `cWorldPainterMan_AllocFiles`: an empty and a filled old array, zero and
    /// several new files, with unwinds from the fakes.
    #[test]
    fn cworld_painter_man_alloc_files_matches() {
        check(
            ee::s08::f_002bb1d0__generated,
            cworld_painter_man_alloc_files,
            300,
            &|c, seed| {
                oc1498_af_hook();
                machine(c, seed, 0x2b_b1d0, 3);
                c.pc = 0x002b_b1d0;
                c.bus.write32(OC1498_AF_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_AF_GATE + 4, DATA + 0x4000);
                let this = DATA + 0x1000;
                let arr = if seed % 4 == 0 { 0 } else { DATA + 0x2010 };
                c.bus.write32(this + 4, arr);
                if arr != 0 {
                    c.bus.write32(arr - 0x10, seed % 4);
                }
                c.bus.write32(this, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 3);
            },
        );
    }

    // cWorldPainterMan_SetFileData: callee WPFile_SetData.
    oc1498_gated_fake!(
        oc1498_srf_setdata,
        OC1498_SRF_PREV,
        OC1498_SRF_GATE,
        4,
        ee::s08::f_002bb570__generated,
        0x002b_b570
    );

    pub(crate) fn oc1498_srf_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB570),
            oc1498_srf_setdata,
            &OC1498_SRF_PREV,
        );
    }

    /// `cWorldPainterMan_SetFileData` for file indices 0..4 and random args.
    #[test]
    fn cworld_painter_man_set_file_data_matches() {
        check(
            ee::s08::f_002bb348__generated,
            cworld_painter_man_set_file_data,
            300,
            &|c, seed| {
                oc1498_srf_hook();
                let mut r = machine(c, seed, 0x2b_b348, 2);
                c.pc = 0x002b_b348;
                c.bus.write32(OC1498_SRF_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_SRF_GATE + 4, DATA + 0x4000);
                let this = DATA + 0x1000;
                c.bus.write32(this + 4, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 5);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    /// `cWorldPainterMan_DebugPrintf` (0x002b_bbf8) against the generated body.
    #[test]
    fn c_world_painter_man_debug_printf_matches() {
        check(
            ee::s08::f_002bbbf8__generated,
            c_world_painter_man_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bbbf8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// Init with `priority == 0xffff` (the long store sequence), init == 0 and another
    /// priority (bare prologue/epilogue), with random FPU registers and gp constants.
    #[test]
    fn c_world_painter_man_static_init_destroy_matches() {
        check(
            ee::s08::f_002bbc38__generated,
            c_world_painter_man_static_init_destroy,
            24,
            &|c, seed| {
                let mut r = Rng(0x2bbc38 + seed as u64);
                c.g[SP][0] = sx(0x0070_0000);
                c.g[GP][0] = sx(0x0050_8000);
                c.g[RA][0] = sx(RET_ADDR);
                c.pc = 0x002b_bc38;
                c.g[A0][0] = if seed % 4 == 1 { 0 } else { 1 };
                c.g[A1][0] = if seed % 4 == 2 { 0x1234 } else { 0xffff };
                for i in 0..32 {
                    c.f[i] = r.edge_f32();
                }
                for i in 0..0x400u32 {
                    let v = r.f32(2.0);
                    c.bus.write32(0x0050_8000 - 0x4800 + i * 4, v);
                }
            },
        );
    }

    /// Gate word value: a test's fakes run only while their gate word holds it.
    pub(crate) const OC1549_MAGIC: u32 = 0x1549_c0de;

    pub(crate) const OC1549_MD_GATE: u32 = DATA + 0x2e_0100;

    pub(crate) const OC1549_UP_GATE: u32 = DATA + 0x2e_0300;

    pub(crate) const OC1549_LF_GATE: u32 = DATA + 0x2e_0400;

    /// A gated fake of one callee's override slot. With its gate word set, it
    /// logs the call to `target` and returns the word at `gate + mail` in v0;
    /// with the gate off it runs the body the slot held before.
    macro_rules! oc1549_gated_fake {
        ($fake:ident, $prev:ident, $gate:expr, $mail:expr, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != OC1549_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32($gate + $mail) as u64;
            }
        };
    }
    pub(crate) use oc1549_gated_fake;

    // cWorldPainterMan_dtor's callees: WPFile_dtor, the free and the delete.
    oc1549_gated_fake!(
        oc1549_md_wpfile_dtor,
        OC1549_MD_PREV_DTOR,
        OC1549_MD_GATE,
        4,
        ee::s08::f_002bb390__generated,
        0x002b_b390
    );

    oc1549_gated_fake!(
        oc1549_md_free,
        OC1549_MD_PREV_FREE,
        OC1549_MD_GATE,
        8,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );

    oc1549_gated_fake!(
        oc1549_md_delete,
        OC1549_MD_PREV_DELETE,
        OC1549_MD_GATE,
        12,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );

    pub(crate) fn oc1549_md_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB390),
            oc1549_md_wpfile_dtor,
            &OC1549_MD_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1549_md_free,
            &OC1549_MD_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1549_md_delete,
            &OC1549_MD_PREV_DELETE,
        );
    }

    /// `cWorldPainterMan_dtor` for 0..3 entries (the empty array, the array
    /// of one, and the walk over several), with and without the delete.
    #[test]
    fn cworld_painter_man_dtor_matches() {
        check(
            ee::s08::f_002bb140__generated,
            cworld_painter_man_dtor,
            300,
            &|c, seed| {
                oc1549_md_hook();
                machine(c, seed, 0x2b_b140, 4);
                c.bus.write32(OC1549_MD_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_MD_GATE + 4, 0);
                c.bus.write32(OC1549_MD_GATE + 8, 0);
                c.bus.write32(OC1549_MD_GATE + 12, 0);
                let this = DATA + 0x1000;
                let arr = if seed % 5 == 0 { 0 } else { DATA + 0x2010 };
                c.bus.write32(this + 4, arr);
                if arr != 0 {
                    c.bus.write32(arr - 0x10, seed % 4);
                }
                c.bus.write32(this, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002b_b140;
            },
        );
    }

    // cWorldPainterMan_Update's callee WPFile_PollLoad (the poll is faked).
    oc1549_gated_fake!(
        oc1549_up_poll,
        OC1549_UP_PREV,
        OC1549_UP_GATE,
        4,
        ee::s08::f_002bb3f0__generated,
        0x002b_b3f0
    );

    pub(crate) fn oc1549_up_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB3F0),
            oc1549_up_poll,
            &OC1549_UP_PREV,
        );
    }

    /// `cWorldPainterMan_Update` for 0..3 entries.
    #[test]
    fn cworld_painter_man_update_matches() {
        check(
            ee::s08::f_002bb2b8__generated,
            cworld_painter_man_update,
            300,
            &|c, seed| {
                oc1549_up_hook();
                machine(c, seed, 0x2b_b2b8, 3);
                c.bus.write32(OC1549_UP_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_UP_GATE + 4, 0);
                let this = DATA + 0x1000;
                c.bus.write32(this, seed % 4);
                c.bus.write32(this + 4, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                c.pc = 0x002b_b2b8;
            },
        );
    }

    // cWorldPainterMan_LoadFile's callee WPFile_BeginLoad (faked).
    oc1549_gated_fake!(
        oc1549_lf_begin,
        OC1549_LF_PREV,
        OC1549_LF_GATE,
        4,
        ee::s08::f_002bb4d8__generated,
        0x002b_b4d8
    );

    pub(crate) fn oc1549_lf_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB4D8),
            oc1549_lf_begin,
            &OC1549_LF_PREV,
        );
    }

    /// `cWorldPainterMan_LoadFile` for several indices and flag words.
    #[test]
    fn cworld_painter_man_load_file_matches() {
        check(
            ee::s08::f_002bb318__generated,
            cworld_painter_man_load_file,
            300,
            &|c, seed| {
                oc1549_lf_hook();
                machine(c, seed, 0x2b_b318, 2);
                c.bus.write32(OC1549_LF_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_LF_GATE + 4, 0);
                let this = DATA + 0x1000;
                c.bus.write32(this + 4, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = seed as u64 % 7;
                c.g[A2][0] = (seed as u64 * 3) % 5;
                c.pc = 0x002b_b318;
            },
        );
    }

    /// Gate word of this module's gated fakes (in the machine's RAM).
    pub(crate) const OC1550_CTRL: u32 = DATA + 0x1_0000;

    pub(crate) const OC1550_MAGIC: u32 = 0x1550_c0de;

    pub(crate) static OC1550_PREV_CSTR_PLUS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1550_PREV_STR_PLUS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1550_PREV_ASYNC_LOAD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1550_PREV_BX_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1550_PREV_MEM_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1550_PREV_STATIC_INIT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1550_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1550_CTRL) == OC1550_MAGIC
    }

    /// `cBXString_OpPlusCStrStr`: a logged fake while the gate is set.
    pub(crate) fn oc1550_fake_cstr_plus(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_CSTR_PLUS, ee::s10::f_003186d0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_86d0, ra);
    }

    /// `cBXString_OpPlusStrCStr`: a logged fake while the gate is set.
    pub(crate) fn oc1550_fake_str_plus(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_STR_PLUS, ee::s10::f_00318630__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_8630, ra);
    }

    /// `cBXString` destructor: a logged fake while the gate is set.
    pub(crate) fn oc1550_fake_bx_dtor(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_BX_DTOR, ee::s10::f_00318260__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_8260, ra);
    }

    /// `AsyncFile_Load`: a logged fake. Mailbox word +4 picks the handle: 1
    /// returns 0, 2 returns -1, anything else the fake's pointer (positive).
    pub(crate) fn oc1550_fake_async_load(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_ASYNC_LOAD, ee::s14::f_003df690__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003d_f690, ra);
        if c.pc == ra {
            match c.bus.read32(OC1550_CTRL + 4) {
                1 => c.g[V0][0] = 0,
                2 => c.g[V0][0] = u64::MAX,
                _ => {}
            }
        }
    }

    /// `EA_MemFree`: a logged fake while the gate is set.
    pub(crate) fn oc1550_fake_mem_free(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_MEM_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
    }

    /// `cWorldPainterMan_StaticInitDestroy`: a logged fake while the gate is set.
    pub(crate) fn oc1550_fake_static_init(c: &mut Cpu) {
        if !oc1550_gated(c) {
            return fakes::fall_back(c, &OC1550_PREV_STATIC_INIT, ee::s08::f_002bbc38__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002b_bc38, ra);
    }

    pub(crate) fn oc1550_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc1550_fake_cstr_plus,
            &OC1550_PREV_CSTR_PLUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc1550_fake_str_plus,
            &OC1550_PREV_STR_PLUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            oc1550_fake_async_load,
            &OC1550_PREV_ASYNC_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc1550_fake_bx_dtor,
            &OC1550_PREV_BX_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1550_fake_mem_free,
            &OC1550_PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BBC38),
            oc1550_fake_static_init,
            &OC1550_PREV_STATIC_INIT,
        );
    }

    #[test]
    fn wp_man_global_ctors_matches() {
        oc1550_install();
        check(
            ee::s08::f_002bc4e0__generated,
            wp_man_global_ctors,
            100,
            &|c, seed| {
                let _r: Rng = fakes::machine(c, seed, 0x1550_03, 1);
                c.bus.write32(OC1550_CTRL, OC1550_MAGIC);
            },
        );
    }
}
