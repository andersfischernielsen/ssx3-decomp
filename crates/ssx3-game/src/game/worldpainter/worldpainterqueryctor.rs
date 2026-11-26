//! `src/worldpainter/worldpainterqueryctor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::intersect::aifwddiff::gp;
pub(crate) use crate::game::intersect::worldsphtree::at;
pub(crate) use crate::game::intersect::worldsphtree::fpu;
pub(crate) use crate::game::intersect::worldsphtree::lw;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::intersect::worldsphtree::store_fpr;
pub(crate) use crate::game::intersect::worldsphtree::virtual_call;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002c_0778,
        world_painter_query_update_at_pos,
        "WorldPainterQuery_UpdateAtPos",
    ),
    (
        0x002c_0a10,
        world_painter_query_lookup_cell,
        "WorldPainterQuery_LookupCell",
    ),
    (
        0x002c_0408,
        world_painter_query_ctor,
        "WorldPainterQuery_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {

    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;

    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_PAINTER_LOOKUP_CELL;

    pub(crate) use crate::game::intersect::worldsphtree::callee::WP_DATA_FIND_CELL_INDEX;
}

// ---------------------------------------------------------------------------
// Cells.

/// `WorldObject` fields.
pub(crate) mod obj {}

// ---------------------------------------------------------------------------
// World painter.

pub(crate) const WP_ENTRY: u32 = 0x002c_0778;

/// Loop head (probe the cell list of the current layer): resume entry.
pub(crate) const WP_PROBE: u32 = 0x002c_0810;

pub(crate) const WP_END: u32 = 0x002c_0a10;

#[derive(Clone, Copy)]
pub(crate) enum Wp {
    Probe,
    Release,
    Done,
    Exit,
}

/// `void WorldPainterQuery_UpdateAtPos(WorldPainterQuery* q, float x,
/// float y, float value)` (0x2c0778): moves the query to (x, y) (adding
/// the distance travelled to the painter's accumulator unless it holds the
/// "fresh" constant), then looks up the painted cell of the current layer
/// and feeds the painter (`q->painter`, virtual 0x210 "apply amount",
/// 0x220 "enter cell", 0x228 "release"). Dispatcher-shaped in the
/// original: re-entered at the probe after an unwind.
pub fn world_painter_query_update_at_pos(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        let mut stage = match c.pc {
            WP_ENTRY => {
                world_painter_enter(c);
                match world_painter_walk(c) {
                    Some(s) => s,
                    None => return Ok(()),
                }
            }
            WP_PROBE => Wp::Probe,
            pc => {
                c.jump_out(pc, WP_ENTRY, WP_END);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                Wp::Probe => world_painter_probe(c)?,
                Wp::Release => {
                    c.cycles += 5;
                    let painter = c.g[A1][0];
                    c.g[V0][0] = lw(c, p(painter as u32, 4));
                    let vt = c.g[V0][0];
                    virtual_call(c, painter, vt, 0x228, V1, 0x002c_09f0)?;
                    Wp::Done
                }
                Wp::Done => {
                    c.cycles += 1;
                    c.g[V0][0] = lw(c, at(c, S1, 0));
                    Wp::Exit
                }
                Wp::Exit => break,
            };
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Prologue and the move: s1 = query, s2 = "fresh", f20 = value.
pub(crate) fn world_painter_enter(c: &mut Cpu) {
    c.cycles += 19;
    let frame = Frame::open(c, 0x50);
    c.g[V0][0] = lw(c, gp(c, -0x848));
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save_fpr(c, 20, 0x40);
    c.g[S2][0] = 1;
    frame.save(c, S0, 0x30);
    c.f[20] = c.f[14];
    frame.save_ra(c, 0);
    let fresh_value = fpu(c, gp(c, -0x4284));
    c.f[0] = fresh_value.0;
    c.g[A1][0] = lw(c, at(c, S1, 0));
    c.g[V1][0] = lw(c, at(c, V0, 0x84));
    let acc = fpu(c, r32(c, A1));
    c.f[2] = acc.0;
    c.g[V0][0] = lw(c, at(c, V1, 0x10));
    let fresh = acc.eq_s(fresh_value);
    abi::set_fcc(c, fresh);
    c.g[A0][0] = lw(c, at(c, V0, 8));
    if !fresh {
        c.cycles += 1;
        c.g[S2][0] = 0;
    }
    c.cycles += 2;
    c.g[V1][0] = lw(c, gp(c, 0x770));
    if c.g[S2][0] == 0 {
        // painter->acc += |(x, y) - q->pos|
        c.cycles += 11;
        let dx = fpu(c, at(c, S1, 8)) - FpuF32(c.f[12]);
        let dy = fpu(c, at(c, S1, 0xc)) - FpuF32(c.f[13]);
        let dx2 = dx * dx;
        let dy2 = dy * dy;
        c.f[1] = dy2.0;
        let dist = (dx2 + dy2).sqrt();
        let sum = acc + dist;
        c.f[0] = sum.0;
        Addr(r32(c, A1)).write_fpu(c, sum);
        c.g[V1][0] = lw(c, gp(c, 0x770));
    }
    c.cycles += 5;
    c.g[V0][0] = 0xc;
    store_fpr(c, 12, at(c, S1, 8));
    c.mult(0, A2, V1, V0);
    store_fpr(c, 13, at(c, S1, 0xc));
}

/// The layer record (`a2 = 12 * layer`): probe its cells if it is a
/// populated list (type 3, non-empty), else release. `None` when an event
/// asks the thread to unwind at the probe.
pub(crate) fn world_painter_walk(c: &mut Cpu) -> Option<Wp> {
    c.cycles += 7;
    c.g[V0][0] = lw(c, at(c, A0, 4));
    c.g[A0][0] = 3;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = addu(c.g[A2][0], c.g[V0][0]);
    c.g[V1][0] = c.lh(r32(c, V0));
    c.g[A3][0] = 0;
    if c.g[V1][0] == c.g[A0][0] {
        c.cycles += 3;
        c.g[V0][0] = lw(c, at(c, V0, 8));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = lw(c, at(c, V0, 4));
            c.g[A3][0] = (0 < c.g[V0][0] as i64) as u64;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = addu(c.g[A1][0], c.g[A2][0]);
    if c.g[A3][0] != 0 {
        if back_edge_unwinds(c, WP_PROBE) {
            return None;
        }
        Some(Wp::Probe)
    } else {
        c.cycles += 1;
        c.g[A1][0] = lw(c, at(c, S1, 0));
        Some(Wp::Release)
    }
}

/// Look up the painted cell at the query position and feed the painter.
pub(crate) fn world_painter_probe(c: &mut Cpu) -> Result<Wp, Unwound> {
    c.cycles += 7;
    c.g[V1][0] = c.lbu(at(c, S1, 0x10));
    c.g[A0][0] = lw(c, at(c, V0, 8));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A0][0] = addu(c.g[A0][0], c.g[V1][0]);
    c.g[A1][0] = lw(c, at(c, A0, 8));
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = lw(c, at(c, S1, 0));
        if c.g[S2][0] == 0 {
            return Ok(Wp::Release);
        }
        c.cycles += 7;
        let painter = c.g[A1][0];
        c.g[V0][0] = lw(c, p(painter as u32, 4));
        let vt = c.g[V0][0];
        virtual_call(c, painter, vt, 0x228, V1, 0x002c_0848)?;
        c.g[A1][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Release);
    }
    c.f[12] = fpu(c, at(c, S1, 8)).0;
    // cell = WorldPainterQuery_LookupCell(q, data)
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    c.f[13] = fpu(c, at(c, S1, 0xc)).0;
    callee::WORLD_PAINTER_LOOKUP_CELL.call(c, 0x002c_085c)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[A1][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Release);
    }
    c.cycles += 5;
    c.g[V0][0] = lw(c, at(c, S1, 0x10));
    c.g[V1][0] = lw(c, at(c, S0, 0));
    c.g[V0][0] = sx(r32(c, V0) >> 8);
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Exit);
    }
    // painter->EnterCell(cell->data)
    c.cycles += 9;
    c.g[A2][0] = lw(c, at(c, S1, 0));
    c.g[A1][0] = lw(c, at(c, S0, 4));
    let painter = c.g[A2][0];
    c.g[V1][0] = lw(c, p(painter as u32, 4));
    let vt = c.g[V1][0];
    virtual_call(c, painter, vt, 0x220, V0, 0x002c_0898)?;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = lw(c, at(c, S1, 0));
        c.sw(r32(c, V0), 0);
    }
    c.cycles += 2;
    if c.g[S2][0] != 0 {
        // Fresh: apply -1 and reset the accumulator.
        c.cycles += 12;
        c.g[A2][0] = lw(c, at(c, S1, 0));
        c.g[A1][0] = lw(c, at(c, S0, 4));
        let painter = c.g[A2][0];
        c.g[V0][0] = lw(c, p(painter as u32, 4));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[12] = 0xbf80_0000;
        let vt = c.g[V0][0];
        virtual_call(c, painter, vt, 0x210, V1, 0x002c_08d4)?;
        c.g[V0][0] = lw(c, at(c, S1, 0));
        c.sw(r32(c, V0), 0);
        return Ok(Wp::Done);
    }
    c.cycles += 5;
    let none = fpu(c, gp(c, -0x4280));
    c.f[0] = none.0;
    let unset = FpuF32(c.f[20]).eq_s(none);
    abi::set_fcc(c, unset);
    if !unset {
        // Apply the value.
        c.g[A2][0] = lw(c, at(c, S1, 0));
        c.cycles += 9;
        c.f[12] = c.f[20];
        c.g[A1][0] = lw(c, at(c, S0, 4));
        let painter = c.g[A2][0];
        c.g[V0][0] = lw(c, p(painter as u32, 4));
        let vt = c.g[V0][0];
        virtual_call(c, painter, vt, 0x210, V1, 0x002c_0998)?;
        c.g[V0][0] = lw(c, at(c, S1, 0));
        return Ok(Wp::Exit);
    }
    c.cycles += 7;
    c.g[A1][0] = lw(c, at(c, S0, 4));
    c.f[20] = 0;
    let level = fpu(c, r32(c, A1));
    c.f[1] = level.0;
    let positive = FpuF32(0).le(level);
    abi::set_fcc(c, positive);
    c.g[V0][0] = c.g[A1][0];
    if positive {
        c.cycles += 6;
        c.g[A2][0] = lw(c, at(c, S1, 0));
        let acc = fpu(c, r32(c, A2));
        c.f[0] = acc.0;
        let reached = level.le(acc);
        abi::set_fcc(c, reached);
        if reached {
            // Apply +1 and clear the accumulator.
            c.cycles += 10;
            let painter = c.g[A2][0];
            c.g[V0][0] = lw(c, p(painter as u32, 4));
            c.g[AT][0] = 0x3f80_0000;
            c.f[12] = 0x3f80_0000;
            let vt = c.g[V0][0];
            virtual_call(c, painter, vt, 0x210, V1, 0x002c_0944)?;
            c.g[V0][0] = lw(c, at(c, S1, 0));
            store_fpr(c, 20, r32(c, V0));
            return Ok(Wp::Done);
        }
        c.g[A1][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[A2][0] = lw(c, at(c, S1, 0));
        c.g[A1][0] = c.g[V0][0];
    }
    // Apply -level.
    c.cycles += 9;
    let level = fpu(c, r32(c, V0));
    let painter = c.g[A2][0];
    c.g[V0][0] = lw(c, p(painter as u32, 4));
    c.f[12] = (-level).0;
    let vt = c.g[V0][0];
    virtual_call(c, painter, vt, 0x210, V1, 0x002c_0974)?;
    c.g[V0][0] = lw(c, at(c, S1, 0));
    Ok(Wp::Exit)
}

// ---------------------------------------------------------------------------
// World painter query: the cell a key falls in.

/// `sItem* WorldPainterQuery_LookupCell(sQuery* self, sList* list)`
/// (0x002c0a10): look the key up in the list's quad tree (the tree search
/// returns an index; -1 is "no cell") and return the cell record at
/// `list->items[index]`, or null.
pub fn world_painter_query_lookup_cell(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0]; // the list
        callee::WP_DATA_FIND_CELL_INDEX.call(c, 0x002c_0a28)?;
        // The tree search leaves the cell index in v0; -1 means outside.
        let index = c.g[V0][0];
        c.g[A0][0] = index;
        c.g[V0][0] = u64::MAX;
        let outside = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
        if outside {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x8)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter query list.

/// `void WorldPainterQuery_dtor(WorldPainterQuery* q, int free)` (0x2c06c0):
/// unlink the query from the painter's list (`gp_2+0x790`), release its payload
/// and optionally free it.
pub fn world_painter_query_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        'outer: {
            'mid: {
                'first: {
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[V0][0] = sx(0x0048_3e00);
                    let this = r(c, A0);
                    c.g[S0][0] = sx(this);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save_ra(c, 0);
                    Addr(p(this, 4)).sw(c, c.g[V0][0]);
                    c.g[RA][0] = sx(0x002c_06ec);
                    c.g[A0][0] = Addr(this).lw(c);
                    call_ptr(c, callee::FREE, 0x002c_06ec)?;
                    c.g[V1][0] = Addr(gp_2(c, 0x790)).lw(c);
                    if c.g[V1][0] != c.g[S0][0] {
                        c.g[V0][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                        break 'first;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x14)).lw(c);
                    c.g[A1][0] = c.g[S1][0] & 1;
                    c.g[A0][0] = sx(0x0048_0000);
                    Addr(gp_2(c, 0x790)).sw(c, c.g[V0][0]);
                    break 'mid;
                }
                'second: {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S1][0] & 1;
                    let hit = c.g[V0][0] == c.g[S0][0];
                    c.g[A0][0] = sx(0x0048_0000);
                    if hit {
                        break 'second;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                        if c.g[V0][0] == c.g[S0][0] {
                            break 'second;
                        }
                        c.g[V1][0] = Addr(p(r(c, V1), 0x14)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x002c_0720);
                        }
                    }
                }
                c.cycles += 2;
                let empty = c.g[V1][0] == 0;
                c.g[V0][0] = sx(p(r(c, A0), 0x5d30));
                if empty {
                    break 'outer;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r(c, S0), 0x14)).lw(c);
                Addr(p(r(c, V1), 0x14)).sw(c, c.g[V0][0]);
                break 'mid;
            }
            c.cycles += 1;
            c.g[V0][0] = sx(p(r(c, A0), 0x5d30));
        }
        'tail: {
            c.cycles += 2;
            let keep = c.g[A1][0] == 0;
            Addr(p(r(c, S0), 4)).sw(c, c.g[V0][0]);
            if keep {
                break 'tail;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x002c_0760);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::FREE, 0x002c_0760)?;
            break 'tail;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* WorldPainterQuery_ctor(void* self, u32 kind)` (0x2c0408): stores the
/// vtable (0x483e00), the float at gp-0x4288 (twice) and the query kind (low
/// byte at +0x10, the rest of `kind` in the word), then, for kinds 0..14 (the
/// byte-sized selector `kind >> 8`), allocates and constructs the matching
/// `tWPIGD_*` data object (jump table at 0x483db0) into `*self`; finally links
/// itself into the list at gp+0x790 (`self->next = head; head = self`). Returns
/// `self`.
///
/// Converted from the generated body (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn world_painter_query_ctor(c: &mut Cpu) {
    if c.pc != 0x002c0408 {
        return ee::s08::f_002c0408__generated(c);
    }
    guest(c, |c| {
        'b_2c069c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V0][0] = 0x480000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sb((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4288));
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[V0][0] = (c.g[A0][0] < 0xeu64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            if t {
                break 'b_2c069c;
            } else {
                'b_2c0678: {
                    'b_2c0650: {
                        'b_2c0628: {
                            'b_2c0600: {
                                'b_2c05d8: {
                                    'b_2c05b0: {
                                        'b_2c0588: {
                                            'b_2c0560: {
                                                'b_2c0538: {
                                                    'b_2c0510: {
                                                        'b_2c04e8: {
                                                            'b_2c04c0: {
                                                                'b_2c0498: {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = 0x480000;
                                                                    c.g[V1][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x3db0));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw(c.g[V1][0] as u32);
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x002c0470 => {
                                                                            c.cycles += 10;
                                                                            c.g[A1][0] = 0x480000;
                                                                            c.g[A0][0] = 0x18;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x3cb8,
                                                                                    ));
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x002c0488);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x002c0488,
                                                                            )?;
                                                                            c.g[RA][0] =
                                                                                sx(0x002c0490);
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            Callee { addr: 0x002bc6f8, f: ee::s08::tWPIGD_MusicTrigger_ctor }.call(c, 0x002c0490)?;
                                                                            let t = true;
                                                                            c.sw(
                                                                                c.g[S0][0] as u32,
                                                                                c.g[V0][0],
                                                                            );
                                                                            if t {
                                                                                break 'b_2c069c;
                                                                            } else {
                                                                                break 'b_2c0498;
                                                                            }
                                                                        }
                                                                        0x002c0498 => {
                                                                            break 'b_2c0498;
                                                                        }
                                                                        0x002c04c0 => {
                                                                            break 'b_2c04c0;
                                                                        }
                                                                        0x002c04e8 => {
                                                                            break 'b_2c04e8;
                                                                        }
                                                                        0x002c0510 => {
                                                                            break 'b_2c0510;
                                                                        }
                                                                        0x002c0538 => {
                                                                            break 'b_2c0538;
                                                                        }
                                                                        0x002c0560 => {
                                                                            break 'b_2c0560;
                                                                        }
                                                                        0x002c0588 => {
                                                                            break 'b_2c0588;
                                                                        }
                                                                        0x002c05b0 => {
                                                                            break 'b_2c05b0;
                                                                        }
                                                                        0x002c05d8 => {
                                                                            break 'b_2c05d8;
                                                                        }
                                                                        0x002c0600 => {
                                                                            break 'b_2c0600;
                                                                        }
                                                                        0x002c0628 => {
                                                                            break 'b_2c0628;
                                                                        }
                                                                        0x002c0650 => {
                                                                            break 'b_2c0650;
                                                                        }
                                                                        0x002c0678 => {
                                                                            break 'b_2c0678;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x002c0408,
                                                                                0x002c06bc,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 10;
                                                                c.g[A1][0] = 0x480000;
                                                                c.g[A0][0] = 0x10;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x3cd0));
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x002c04b0);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x002c04b0,
                                                                )?;
                                                                c.g[RA][0] = sx(0x002c04b8);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                Callee {
                                                                    addr: 0x002bc728,
                                                                    f: ee::s08::tWPIGD_Mix_ctor,
                                                                }
                                                                .call(c, 0x002c04b8)?;
                                                                let t = true;
                                                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                                if t {
                                                                    break 'b_2c069c;
                                                                } else {
                                                                    break 'b_2c04c0;
                                                                }
                                                            }
                                                            c.cycles += 10;
                                                            c.g[A1][0] = 0x480000;
                                                            c.g[A0][0] = 0x10;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x3ce0));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x002c04d8);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x002c04d8)?;
                                                            c.g[RA][0] = sx(0x002c04e0);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            Callee {
                                                                addr: 0x002bc750,
                                                                f: ee::s08::tWPIGD_Ambience_ctor,
                                                            }
                                                            .call(c, 0x002c04e0)?;
                                                            let t = true;
                                                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                            if t {
                                                                break 'b_2c069c;
                                                            } else {
                                                                break 'b_2c04e8;
                                                            }
                                                        }
                                                        c.cycles += 10;
                                                        c.g[A1][0] = 0x480000;
                                                        c.g[A0][0] = 0x10;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3cf0));
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x002c0500);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x002c0500)?;
                                                        c.g[RA][0] = sx(0x002c0508);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        Callee {
                                                            addr: 0x002bc778,
                                                            f: ee::s08::tWPIGD_Speech_ctor,
                                                        }
                                                        .call(c, 0x002c0508)?;
                                                        let t = true;
                                                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                        if t {
                                                            break 'b_2c069c;
                                                        } else {
                                                            break 'b_2c0510;
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[A1][0] = 0x480000;
                                                    c.g[A0][0] = 0x10;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3d00));
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x002c0528);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x002c0528)?;
                                                    c.g[RA][0] = sx(0x002c0530);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    Callee {
                                                        addr: 0x002bc7a0,
                                                        f: ee::s08::tWPIGD_Camera_ctor,
                                                    }
                                                    .call(c, 0x002c0530)?;
                                                    let t = true;
                                                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                    if t {
                                                        break 'b_2c069c;
                                                    } else {
                                                        break 'b_2c0538;
                                                    }
                                                }
                                                c.cycles += 10;
                                                c.g[A1][0] = 0x480000;
                                                c.g[A0][0] = 0x38;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x3d10));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x002c0550);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x002c0550)?;
                                                c.g[RA][0] = sx(0x002c0558);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x002bc7c8,
                                                    f: ee::s08::tWPIGD_Fog_ctor,
                                                }
                                                .call(c, 0x002c0558)?;
                                                let t = true;
                                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                if t {
                                                    break 'b_2c069c;
                                                } else {
                                                    break 'b_2c0560;
                                                }
                                            }
                                            c.cycles += 10;
                                            c.g[A1][0] = 0x480000;
                                            c.g[A0][0] = 0x40;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x3d20));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x002c0578);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x002c0578)?;
                                            c.g[RA][0] = sx(0x002c0580);
                                            c.g[A0][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x002bc830,
                                                f: ee::s08::tWPIGD_LightGlow_ctor,
                                            }
                                            .call(c, 0x002c0580)?;
                                            let t = true;
                                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                            if t {
                                                break 'b_2c069c;
                                            } else {
                                                break 'b_2c0588;
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[A1][0] = 0x480000;
                                        c.g[A0][0] = 0x38;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d38));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x002c05a0);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x002c05a0)?;
                                        c.g[RA][0] = sx(0x002c05a8);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x002bc890,
                                            f: ee::s08::tWPIGD_ScreenTint_ctor,
                                        }
                                        .call(c, 0x002c05a8)?;
                                        let t = true;
                                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                        if t {
                                            break 'b_2c069c;
                                        } else {
                                            break 'b_2c05b0;
                                        }
                                    }
                                    c.cycles += 10;
                                    c.g[A1][0] = 0x480000;
                                    c.g[A0][0] = 0x10;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d50));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x002c05c8);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x002c05c8)?;
                                    c.g[RA][0] = sx(0x002c05d0);
                                    c.g[A0][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x002bc8e8,
                                        f: ee::s08::tWPIGD_SkyBox_ctor,
                                    }
                                    .call(c, 0x002c05d0)?;
                                    let t = true;
                                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_2c069c;
                                    } else {
                                        break 'b_2c05d8;
                                    }
                                }
                                c.cycles += 10;
                                c.g[A1][0] = 0x480000;
                                c.g[A0][0] = 0x50;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d60));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x002c05f0);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x002c05f0)?;
                                c.g[RA][0] = sx(0x002c05f8);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x002bc910,
                                    f: ee::s08::tWPIGD_Sun_ctor,
                                }
                                .call(c, 0x002c05f8)?;
                                let t = true;
                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                if t {
                                    break 'b_2c069c;
                                } else {
                                    break 'b_2c0600;
                                }
                            }
                            c.cycles += 10;
                            c.g[A1][0] = 0x480000;
                            c.g[A0][0] = 0x10;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d70));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x002c0618);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x002c0618)?;
                            c.g[RA][0] = sx(0x002c0620);
                            c.g[A0][0] = c.g[V0][0];
                            Callee {
                                addr: 0x002bc980,
                                f: ee::s08::tWPIGD_Surface_ctor,
                            }
                            .call(c, 0x002c0620)?;
                            let t = true;
                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_2c069c;
                            } else {
                                break 'b_2c0628;
                            }
                        }
                        c.cycles += 10;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x58;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d80));
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x002c0640);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x002c0640)?;
                        c.g[RA][0] = sx(0x002c0648);
                        c.g[A0][0] = c.g[V0][0];
                        Callee {
                            addr: 0x002bc9b0,
                            f: ee::s08::tWPIGD_Lighting_ctor,
                        }
                        .call(c, 0x002c0648)?;
                        let t = true;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        if t {
                            break 'b_2c069c;
                        } else {
                            break 'b_2c0650;
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0xa0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d90));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x002c0668);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x002c0668)?;
                    c.g[RA][0] = sx(0x002c0670);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x002bcaf8,
                        f: ee::s08::tWPIGD_Weather_ctor,
                    }
                    .call(c, 0x002c0670)?;
                    let t = true;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    if t {
                        break 'b_2c069c;
                    } else {
                        break 'b_2c0678;
                    }
                }
                c.cycles += 9;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = 0x18;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3da0));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x002c0690);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x002c0690)?;
                c.g[RA][0] = sx(0x002c0698);
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x002bcbd0,
                    f: ee::s08::tWPIGD_Danger_ctor,
                }
                .call(c, 0x002c0698)?;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                break 'b_2c069c;
            }
        }
        c.cycles += 8;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x790));
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x790), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::intersect::worldsphtree::tests::f;

    pub(crate) use crate::game::intersect::worldsphtree::tests::setup_machine;
    pub(crate) use crate::game::intersect::worldsphtree::tests::uf;

    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2;
    pub(crate) use crate::game::intersect::worldsphtree::tests::VT_PAINTER;

    #[test]
    fn world_painter_query_update_at_pos_matches() {
        check(
            ee::s08::f_002c0778__generated,
            world_painter_query_update_at_pos,
            SEEDS,
            &|c, seed| {
                let mut r = setup_machine(c, seed, 0x0778, 4);
                c.pc = WP_ENTRY;
                let (p1, p2, p3, table, recs, list) = (
                    DATA + 0x100,
                    DATA + 0x180,
                    DATA + 0x200,
                    DATA + 0x280,
                    DATA + 0x300,
                    DATA + 0x600,
                );
                let (query, painter, data, quad, cells, value) = (
                    DATA + 0x800,
                    DATA + 0x900,
                    DATA + 0xa00,
                    DATA + 0xb00,
                    DATA + 0xc00,
                    DATA + 0xd00,
                );
                let (k1, k2) = (uf(&mut r, -1.0, 1.0), uf(&mut r, -1.0, 1.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x848), p1);
                c.bus.write32(p1 + 0x84, p2);
                c.bus.write32(p2 + 0x10, p3);
                c.bus.write32(p3 + 8, table);
                c.bus.write32(table + 4, recs);
                let layer = r.next_u32() % 3;
                c.bus.write32(GAME_GP.wrapping_add(0x770), layer);
                let rec = recs + 12 * layer;
                c.bus.write16(rec, if seed % 9 == 0 { 2 } else { 3 });
                c.bus
                    .write32(rec + 8, if seed % 11 == 0 { 0 } else { list });
                c.bus.write32(
                    list + 4,
                    if seed % 13 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 3
                    },
                );
                let k = r.next_u32() % 4;
                let tag = r.next_u32() & 0xff_ffff;
                c.bus.write32(query + 0x10, tag << 8 | k);
                c.bus
                    .write32(list + 8 + 4 * k, if seed % 5 == 0 { 0 } else { data });
                c.bus.write32(data, quad);
                c.bus
                    .write32(data + 8, if seed % 17 == 0 { 0 } else { cells });
                c.bus
                    .write32(cells, if seed % 4 == 1 { tag + 1 } else { tag });
                c.bus.write32(cells + 4, value);
                c.bus.write32(value, f(uf(&mut r, -1.0, 2.0)));
                c.bus.write32(query, painter);
                c.bus.write32(query + 8, f(uf(&mut r, -5.0, 5.0)));
                c.bus.write32(query + 0xc, f(uf(&mut r, -5.0, 5.0)));
                c.bus.write32(
                    painter,
                    f(if seed % 3 == 0 {
                        k1
                    } else {
                        uf(&mut r, -1.0, 2.0)
                    }),
                );
                c.bus.write32(painter + 4, VT_PAINTER);
                c.bus.write32(GAME_GP.wrapping_sub(0x4284), f(k1));
                c.bus.write32(GAME_GP.wrapping_sub(0x4280), f(k2));
                c.f[12] = f(uf(&mut r, -5.0, 5.0));
                c.f[13] = f(uf(&mut r, -5.0, 5.0));
                c.f[14] = if seed % 2 == 0 {
                    f(k2)
                } else {
                    f(uf(&mut r, -1.0, 2.0))
                };
                c.g[A0][0] = sx(query);
            },
        );
    }

    #[test]
    fn world_painter_query_lookup_cell_matches() {
        const LIST: u32 = DATA + 0x1000;
        check(
            ssx3_ee::s08::f_002c0a10__generated,
            world_painter_query_lookup_cell,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x51ed_0001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let node = LIST + 0x100;
                let cell = LIST + 0x200;
                c.g[A0][0] = sx(LIST + 0x800);
                c.g[A1][0] = sx(LIST);
                c.bus.write32(LIST, node); // the tree root
                c.bus.write32(LIST + 8, LIST + 0x300); // items
                for k in 0..4 {
                    c.bus.write32(node + 4 * k, 0);
                }
                c.bus.write16(node + 0x14, 0); // no children
                c.bus.write32(node + 0x20, cell); // leaf data
                c.bus.write16(cell, 0); // a leaf
                c.bus.write32(
                    cell + 4,
                    if seed % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.f[12] = 0;
                c.f[13] = 0;
            },
        );
    }

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC809_CTRL`.
    pub(crate) const OC809_CTRL: u32 = DATA + 0xf_d000;

    pub(crate) const OC809_MAGIC: u32 = 0x8095_a7c1;

    /// `(address, v0)` pairs: the result of every fake call to `address`.
    pub(crate) const OC809_BY_ADDR: u32 = OC809_CTRL + 0x100;

    /// A gated fake call: logs and clobbers like `fakes::fake`, then forces
    /// `v0` from the result table (unless the call unwound).
    pub(crate) fn oc809_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            for k in 0..8 {
                let slot = OC809_BY_ADDR + 8 * k;
                if c.bus.read32(slot) == addr {
                    c.g[V0][0] = sx(c.bus.read32(slot + 4));
                }
            }
        }
    }

    /// Gated fakes of the callees (each `hook`ed once per process; with the
    /// gate off they run the body they replaced).
    macro_rules! oc809_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC809_CTRL) != OC809_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc809_fake_body(c, $addr)
                }
            )*
            pub(crate) fn oc809_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc809_fakes! {
        oc809_fake_0011b698, OC809_PREV_0011B698, 0x0011_b698, ee::s00::OVERRIDE_0011B698, ee::s00::f_0011b698__generated;
        oc809_fake_002daa78, OC809_PREV_002DAA78, 0x002d_aa78, ee::s09::OVERRIDE_002DAA78, ee::s09::f_002daa78__generated;
        oc809_fake_002db748, OC809_PREV_002DB748, 0x002d_b748, ee::s09::OVERRIDE_002DB748, ee::s09::f_002db748__generated;
        oc809_fake_002df3b0, OC809_PREV_002DF3B0, 0x002d_f3b0, ee::s09::OVERRIDE_002DF3B0, ee::s09::f_002df3b0__generated;
        oc809_fake_002df448, OC809_PREV_002DF448, 0x002d_f448, ee::s09::OVERRIDE_002DF448, ee::s09::f_002df448__generated;
        oc809_fake_002e26b8, OC809_PREV_002E26B8, 0x002e_26b8, ee::s09::OVERRIDE_002E26B8, ee::s09::f_002e26b8__generated;
        oc809_fake_002f7a68, OC809_PREV_002F7A68, 0x002f_7a68, ee::s09::OVERRIDE_002F7A68, ee::s09::f_002f7a68__generated;
        oc809_fake_0036cbf8, OC809_PREV_0036CBF8, 0x0036_cbf8, ee::s12::OVERRIDE_0036CBF8, ee::s12::f_0036cbf8__generated;
        oc809_fake_0036ccb8, OC809_PREV_0036CCB8, 0x0036_ccb8, ee::s12::OVERRIDE_0036CCB8, ee::s12::f_0036ccb8__generated;
        oc809_fake_0036ce00, OC809_PREV_0036CE00, 0x0036_ce00, ee::s12::OVERRIDE_0036CE00, ee::s12::f_0036ce00__generated;
        oc809_fake_0036ce28, OC809_PREV_0036CE28, 0x0036_ce28, ee::s12::OVERRIDE_0036CE28, ee::s12::f_0036ce28__generated;
        oc809_fake_0036d008, OC809_PREV_0036D008, 0x0036_d008, ee::s12::OVERRIDE_0036D008, ee::s12::f_0036d008__generated;
        oc809_fake_0036d3d8, OC809_PREV_0036D3D8, 0x0036_d3d8, ee::s12::OVERRIDE_0036D3D8, ee::s12::f_0036d3d8__generated;
        oc809_fake_003714b8, OC809_PREV_003714B8, 0x0037_14b8, ee::s12::OVERRIDE_003714B8, ee::s12::f_003714b8__generated;
        oc809_fake_00317d70, OC809_PREV_00317D70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc809_fake_00317e30, OC809_PREV_00317E30, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc809_fake_00416210, OC809_PREV_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
    }

    /// A scrambled machine with the fakes on, the object registers set and
    /// `pc` at the function's entry; unwinds at one of `calls` fake calls.
    pub(crate) fn oc809_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc809_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC809_CTRL, OC809_MAGIC);
        r
    }

    /// The set of total cycle costs the generated body takes over the seeds.
    pub(crate) fn oc809_paths(
        f: ssx3_ee_rt::funcs::GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> std::collections::BTreeSet<u64> {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            f(&mut c);
            seen.insert(c.cycles - before);
        }
        seen
    }

    /// The object under construction, away from the fakes' allocations
    /// (`DATA..DATA+0x40000`) and the call log (`DATA+0x100000`).
    pub(crate) const OC809_THIS: u32 = DATA + 0x8_0000;

    /// `WorldPainterQuery_ctor` setup: `kind` selects the data object (0..15),
    /// the jump table of the generated body is written, the allocator is faked
    /// and the data-object constructors run for real.
    pub(crate) fn oc809_wpq_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc809_machine(c, seed, 0x2c0408, 1, 0x002c_0408);
        const TARGETS: [u32; 14] = [
            0x002c_0470,
            0x002c_0498,
            0x002c_04c0,
            0x002c_04e8,
            0x002c_0510,
            0x002c_0538,
            0x002c_0560,
            0x002c_0588,
            0x002c_05b0,
            0x002c_05d8,
            0x002c_0600,
            0x002c_0628,
            0x002c_0650,
            0x002c_0678,
        ];
        for (i, t) in TARGETS.iter().enumerate() {
            c.bus.write32(0x0048_3db0 + 4 * i as u32, *t);
        }
        // Kind 0..15: the selector `kind >> 8` is 14 or 15 for the no-object path.
        let sel = seed % 16;
        c.g[A0][0] = OC809_THIS as u64;
        c.g[A1][0] = ((sel << 8) | (r.next_u32() & 0xff)) as u64;
        c.bus
            .write32(GAME_GP + 0x790, DATA + 0x8_4000 + (r.next_u32() & 0xf0));
        c.bus.write32(GAME_GP.wrapping_sub(0x4288), r.next_u32());
    }

    /// `WorldPainterQuery_ctor` against the generated body: every selector
    /// (14 data-object kinds and the two without), the allocator faked (and
    /// unwinding), the data-object constructors real, the list link.
    #[test]
    fn world_painter_query_ctor_matches() {
        fakes::covered(
            ee::s08::f_002c0408__generated,
            world_painter_query_ctor,
            128,
            &oc809_wpq_setup,
        );
        let paths = oc809_paths(ee::s08::f_002c0408__generated, 128, &|c, seed| {
            oc809_wpq_setup(c, seed);
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        });
        assert!(paths.len() >= 8, "paths {paths:?}");
    }
}
