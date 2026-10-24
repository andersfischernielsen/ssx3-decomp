//! `src/worldpainter/worldpainterqueryctor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events;
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
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
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
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::AI_FWD_DIFF_CACHE_COLLIDE_ENTRY;
    pub(crate) use crate::game::intersect::worldsphtree::callee::AI_FWD_DIFF_CACHE_GET;
    pub(crate) use crate::game::intersect::worldsphtree::callee::ANIM_OBJECT_GET_NODE_INTERSECT_MATRIX;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FLEX_RAIL_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::FOCUS_MAN_COLLECT_OCTREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_BUILTIN;
    pub(crate) use crate::game::intersect::worldsphtree::callee::INPUT_VM_READ_PAD_CHANNEL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_FIND_NEAREST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::RAIL_MODIFIER_FIND_CLOSEST_POINT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_PAINTER_LOOKUP_CELL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_SOUND_QUERY_TREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WP_DATA_FIND_CELL_INDEX;
}

// ---------------------------------------------------------------------------
// Cells.

/// `WorldObject` fields.
pub(crate) mod obj {
    pub(crate) use crate::game::intersect::worldsphtree::obj::BOX_MAX;
    pub(crate) use crate::game::intersect::worldsphtree::obj::BOX_MIN;
    pub(crate) use crate::game::intersect::worldsphtree::obj::FLAGS;
    pub(crate) use crate::game::intersect::worldsphtree::obj::NEXT;
    pub(crate) use crate::game::intersect::worldsphtree::obj::SHAPE;
    pub(crate) use crate::game::intersect::worldsphtree::obj::SUB;
}

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
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::intersect::worldsphtree::tests::f;
    pub(crate) use crate::game::intersect::worldsphtree::tests::fake_varied;
    pub(crate) use crate::game::intersect::worldsphtree::tests::put_vtables;
    pub(crate) use crate::game::intersect::worldsphtree::tests::setup_machine;
    pub(crate) use crate::game::intersect::worldsphtree::tests::uf;
    pub(crate) use crate::game::intersect::worldsphtree::tests::BOX2;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS_2;
    pub(crate) use crate::game::intersect::worldsphtree::tests::VT_PAINTER;
    pub(crate) use crate::game::intersect::worldsphtree::tests::VT_SUB;
    pub(crate) use crate::game::intersect::worldsphtree::tests::VT_VOL;

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
}
