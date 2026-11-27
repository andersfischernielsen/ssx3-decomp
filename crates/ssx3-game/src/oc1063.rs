//! Staging module oc1063 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1063`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_97a8, avalanche_start, "Avalanche_Start"),
    (0x002d_9d68, avalanche_load_state, "Avalanche_LoadState"),
    (
        0x0036_9cf8,
        graphics_man_draw_indexed_triangles,
        "GraphicsMan_DrawIndexedTriangles",
    ),
    (
        0x0036_ae20,
        ps2_graphics_man_emit_depth_fog_z_copy,
        "PS2GraphicsMan_EmitDepthFogZCopy",
    ),
    (
        0x0036_b158,
        ps2_graphics_man_emit_depth_fog_blend,
        "PS2GraphicsMan_EmitDepthFogBlend",
    ),
];

// ---------------------------------------------------------------------------
// Avalanche_Start (0x2d97a8) and Avalanche_LoadState (0x2d9d68)

/// The 16 active-avalanche slots (`cAvalanche_m_activeAvalanches`, 0x1c bytes
/// each: avalanche, active-node list, .., time at +0x18).
pub(crate) const AVS_SLOTS: u32 = 0x0054_0000 - 0x76c8;
pub(crate) const AVS_SLOTS_END: u32 = AVS_SLOTS + 0x1c0;
pub(crate) const AVS_SLOT_SIZE: u32 = 0x1c;
/// The 64 active nodes (`cAvalanche_m_activeNodes`, 0x2f0 bytes each).
pub(crate) const AVS_NODES: u32 = 0x004f_0000 - 0x1890;
pub(crate) const AVS_NODE_SIZE: u32 = 0x2f0;
/// `t4mat_Identity`.
pub(crate) const AVS_IDENTITY: u32 = 0x0050_0000 - 0xe60;
/// "No slot": `0x80000000` as the sign-extended return value.
pub(crate) const AVS_NONE: u64 = 0xffff_ffff_8000_0000;
/// `gp` offsets: the avalanche list head and the node-table fields.
pub(crate) const AVS_GP_LIST: u32 = 0x9c0;

const AVS_APPLY_HISTORY: Callee = Callee {
    addr: 0x002d_a028,
    f: ee::s09::Avalanche_ApplyHistoryData,
};
const AVS_FADE: Callee = Callee {
    addr: 0x002d_7c00,
    f: ee::s09::Avalanche_ComputeInstanceFade,
};
const AVS_BUILD_ARRAY: Callee = Callee {
    addr: 0x002d_7e60,
    f: ee::s09::Avalanche_BuildActiveNodeArray,
};
const AVS_STOP: Callee = Callee {
    addr: 0x002d_81b0,
    f: ee::s09::Avalanche_Stop,
};
const AVS_START: Callee = Callee {
    addr: 0x002d_97a8,
    f: ee::s09::Avalanche_Start,
};
const AVS_SAMPLE_HISTORY: Callee = Callee {
    addr: 0x002d_5778,
    f: ee::s09::Avalanche_SampleHistory,
};
const AVS_WEATHER_R: Callee = Callee {
    addr: 0x002e_e7c8,
    f: ee::s09::WorldPainter_GetWeather68,
};
const AVS_WEATHER_G: Callee = Callee {
    addr: 0x002e_e810,
    f: ee::s09::WorldPainter_GetWeather70,
};
const AVS_WEATHER_B: Callee = Callee {
    addr: 0x002e_e858,
    f: ee::s09::WorldPainter_GetWeather78,
};

/// `int Avalanche_Start(int id)` (0x2d97a8): start the avalanche with this id
/// in a free one of the 16 slots (-0x80000000 when unknown, already running or
/// out of active nodes, else the slot index). Every node of the avalanche
/// takes a free active node (64 in all) with identity matrices and white
/// colour; emitter nodes also reset their dynamic emitter with the snow-flake
/// colour. Then the node array is built, the avalanche sound is started and the
/// danger manager counts one more active avalanche.
pub fn avalanche_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 21, 0x98);
        frame.save_fpr(c, 20, 0x90);
        AVS_APPLY_HISTORY.call(c, 0x002d_97dc)?;
        let gp = c.g[GP][0] as u32;
        c.g[A1][0] = c.lw(gp.wrapping_add(AVS_GP_LIST));
        c.g[V0][0] = 0x0054_0000;
        'done: {
            'none: {
                if c.g[A1][0] == 0 {
                    break 'none;
                }
                c.cycles += 2;
                c.g[S6][0] = sx(AVS_SLOTS);
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                // For each avalanche in the list, until the id matches.
                'avalanches: loop {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = 0x0054_0000;
                        c.g[V1][0] = c.g[S6][0];
                        c.g[A2][0] = sx(AVS_SLOTS_END);
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        // Already running in a slot?
                        loop {
                            c.cycles += 2;
                            let running = c.g[V0][0] == c.g[A1][0];
                            c.g[V0][0] = AVS_NONE;
                            if running {
                                break 'done;
                            }
                            c.cycles += 4;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(AVS_SLOT_SIZE));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_9808);
                            }
                        }
                        c.cycles += 2;
                        c.g[S3][0] = sx(AVS_SLOTS);
                        c.g[V1][0] = 0;
                        // The first free slot.
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                            if c.g[V0][0] == 0 {
                                avalanche_start_in_slot(c)?;
                                break 'done;
                            }
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 16i64) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(AVS_SLOT_SIZE));
                            if !more {
                                c.cycles += 1;
                                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002d_9828);
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        break 'none;
                    }
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_97f0);
                    }
                    continue 'avalanches;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = AVS_NONE;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x98);
        frame.restore_fpr(c, 20, 0x90);
        frame.close(c);
        Ok(())
    })
}

/// Avalanche_Start, free slot `s3` found (`v1` its index): claim it, give every
/// node of the avalanche (`a1`) an active node, then finish (node array, sound,
/// danger counter) leaving the slot index in `v0`; or release everything again
/// and leave -0x80000000 in `v0` when the 64 active nodes run out.
fn avalanche_start_in_slot(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.sw(c.g[S3][0] as u32, c.g[A1][0]);
    c.g[V0][0] = 0x004f_0000;
    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0);
    c.g[S1][0] = sx(AVS_NODES);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), 0);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), 0);
    c.g[S2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    let no_nodes = c.g[S2][0] == 0;
    c.g[S4][0] = 0;
    if !no_nodes {
        c.cycles += 1;
        c.g[V0][0] = ((c.g[S4][0] as i64) < 64i64) as u64;
        // For each node of the avalanche (s2, linked at +0xf4).
        loop {
            c.cycles += 2;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0050_0000;
            if !full {
                c.cycles += 4;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = sx(AVS_IDENTITY);
                c.g[S5][0] = 1;
                // The next free active node (s1).
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e0));
                    if c.g[V0][0] == 0 {
                        avalanche_start_init_node(c)?;
                        break;
                    }
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 64i64) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(AVS_NODE_SIZE));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_9878);
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = 0x40;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[S2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf4));
            } else {
                // Out of active nodes: free the ones taken so far.
                c.cycles += 3;
                c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                let empty = c.g[S1][0] == 0;
                c.sw(c.g[S3][0] as u32, 0);
                if empty {
                    c.cycles += 1;
                    c.g[V0][0] = AVS_NONE;
                    return Ok(());
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e0));
                loop {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x114), 0);
                    }
                    c.cycles += 4;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2e0), 0);
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e4));
                    if c.g[S1][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e0));
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_99b8);
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = AVS_NONE;
                return Ok(());
            }
            c.cycles += 2;
            let more = c.g[S2][0] != 0;
            c.g[V0][0] = ((c.g[S4][0] as i64) < 64i64) as u64;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9860);
            }
        }
    }
    c.cycles += 20;
    c.g[A0][0] = c.g[S3][0];
    AVS_BUILD_ARRAY.call(c, 0x002d_99e8)?;
    call_ptr(c, 0x0028_b180, 0x002d_99f0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, 0x0029_def0, 0x002d_99fc)?;
    let gp = c.g[GP][0] as u32;
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
    c.g[A0][0] = 0xffff_ffff_b6db_0000;
    c.g[A0][0] |= 0x6db7;
    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S6][0] as u32));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    // The slot index: (slot - slots) / 0x1c as a multiply by the inverse.
    c.mult(0, 2, 2, 4);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x708));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x708), c.g[V1][0]);
    Ok(())
}

/// Avalanche_Start, free active node `s1` for avalanche node `s2`, slot `s3`:
/// link it into the slot's list, load identity matrices and white colour, scale
/// at 0, and for emitter nodes reset and set up the dynamic emitter with the
/// snow-flake colour (weather 0).
fn avalanche_start_init_node(c: &mut Cpu) -> Flow {
    c.cycles += 43;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x2e0), c.g[S2][0]);
    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x114), c.g[S1][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
    c.sw((c.g[S1][0] as u32).wrapping_add(0x2e4), c.g[V0][0]);
    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S1][0]);
    c.sw(c.g[S1][0] as u32, 0);
    c.lq(2, (c.g[S2][0] as u32).wrapping_add(0xe0));
    c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x10));
    let ident = c.g[A1][0] as u32;
    let to = c.g[V1][0] as u32;
    avalanche_start_copy_matrix(c, ident, to);
    c.sw((c.g[S1][0] as u32).wrapping_add(0x2ec), 0);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf8));
    c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
    let sp0 = c.g[SP][0] as u32;
    c.sq(2, sp0);
    c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x60));
    let to = c.g[A0][0] as u32;
    avalanche_start_copy_matrix(c, ident, to);
    c.swc1(0, sp0.wrapping_add(0xc));
    c.g[A0][0] = c.g[S1][0];
    c.swc1(0, sp0);
    c.swc1(0, sp0.wrapping_add(0x4));
    c.swc1(0, sp0.wrapping_add(0x8));
    c.mtc1(0, 12);
    c.lq(2, sp0);
    c.sq(2, (c.g[S1][0] as u32).wrapping_add(0xc0));
    AVS_FADE.call(c, 0x002d_9924)?;
    c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0xf2));
    let has_emitter = c.g[V0][0] != 0;
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
    if has_emitter {
        c.cycles += 23;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0037_1600, 0x002d_9938)?;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0037_0dc8, 0x002d_994c)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2d0), c.g[S5][0]);
        c.g[A0][0] = 0;
        AVS_WEATHER_R.call(c, 0x002d_9958)?;
        c.mov_s(21, 0);
        c.g[A0][0] = 0;
        AVS_WEATHER_G.call(c, 0x002d_9964)?;
        c.mov_s(20, 0);
        c.g[A0][0] = 0;
        AVS_WEATHER_B.call(c, 0x002d_9970)?;
        let sp0 = c.g[SP][0] as u32;
        c.swc1(21, sp0);
        c.swc1(20, sp0.wrapping_add(0x4));
        c.swc1(0, sp0.wrapping_add(0x8));
        c.sw(sp0.wrapping_add(0xc), 0);
        c.lq(2, sp0);
        c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x1e0));
    }
    Ok(())
}

/// `vu0CopyMatrixAV`: copy the 4x4 matrix at `from` to `to` through VU0
/// registers vf1..vf4.
fn avalanche_start_copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, from.wrapping_add(0x10 * i));
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, to.wrapping_add(0x10 * i));
    }
}

/// `void Avalanche_LoadState(ReplayStream *s)` (0x2d9d68): restore the active
/// avalanches from a replay frame. Stops all 16 slots, reads the 0x80-byte
/// record (16 ids, 16 times) through the stream's vtable (+0x10 / +0x14), then
/// restarts each avalanche with a valid id and winds it to its saved time: the
/// time is stored in the slot, each active node recomputes its fade and
/// history sample (emitter nodes flagged to re-emit), and the slot's collision
/// and ground-collision cursors skip every event up to that time.
pub fn avalanche_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = 0x0054_0000;
        frame.save(c, S0, 0x140);
        c.g[V0][0] = sx(AVS_SLOTS);
        frame.save(c, S1, 0x130);
        c.g[S0][0] = c.g[V0][0];
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x110);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
        frame.save(c, S4, 0x100);
        frame.save(c, S5, 0xf0);
        frame.save_ra(c, 0xe0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        // Stop every slot in use.
        loop {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            let idle = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(AVS_SLOT_SIZE));
            if !idle {
                c.cycles += 2;
                AVS_STOP.call(c, 0x002d_9db4)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x002d_9da0);
            }
        }
        // s->Read(buf, 0x80) through the vtable at +0x10 (delta) / +0x14 (fn).
        c.cycles += 12;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x80;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let read = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, read, 0x002d_9ddc)?;
        c.g[V1][0] = 0x0054_0000;
        c.g[A0][0] = 0;
        c.g[S5][0] = sx(AVS_SLOTS);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 2);
        // For each of the 16 saved entries (a0 = i, s0 = 4 * i).
        loop {
            c.cycles += 5;
            c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            if (c.g[A0][0] as i64) < 0 {
                c.g[A0][0] = c.g[S4][0];
            } else {
                avalanche_load_state_entry(c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 16i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[A0][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9df0);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore_ra(c, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// Avalanche_LoadState, one saved entry whose id (`a0`) is not negative:
/// start it and wind the new slot to the saved time. Ends with `a0 = s4` (the
/// next index).
fn avalanche_load_state_entry(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    AVS_START.call(c, 0x002d_9e0c)?;
    c.g[A0][0] = c.g[V0][0];
    let failed = (c.g[A0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    'next: {
        if failed {
            break 'next;
        }
        c.cycles += 9;
        c.g[V1][0] = AVS_SLOT_SIZE as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.mult(0, 5, 4, 3);
        c.lwc1(1, c.g[V0][0] as u32);
        c.mtc1(0, 0);
        c.c_eq_s(1, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[S5][0] as u32));
        let zero_time = c.fcc();
        c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
        if zero_time {
            break 'next;
        }
        // The slot's active nodes.
        c.cycles += 3;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if c.g[S0][0] == 0 {
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        } else {
            c.cycles += 4;
            c.g[S2][0] = c.g[V0][0];
            c.g[S3][0] = 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e0));
            loop {
                c.cycles += 3;
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xf2));
                let no_emitter = c.g[V1][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_emitter {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), c.g[S3][0]);
                }
                c.cycles += 9;
                c.lwc1(12, c.g[S2][0] as u32);
                AVS_FADE.call(c, 0x002d_9e70)?;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                AVS_SAMPLE_HISTORY.call(c, 0x002d_9e80)?;
                c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e4));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e0));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9e58);
                }
            }
        }
        // Skip the collisions (8-byte records, time first) up to the time.
        'collisions: {
            c.cycles += 5;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if !none {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
                c.lwc1(0, c.g[V0][0] as u32);
                c.c_le_s(0, 1);
                if !c.fcc() {
                    c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                    break 'collisions;
                }
                c.cycles += 3;
                c.g[A3][0] = c.g[V1][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                loop {
                    c.cycles += 7;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let end = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if end {
                        break;
                    }
                    c.cycles += 5;
                    c.lwc1(0, c.g[V1][0] as u32);
                    c.c_le_s(0, 1);
                    let passed = c.fcc();
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    if !passed {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_9ed0);
                    }
                }
            }
            c.cycles += 1;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        }
        // Ground collisions (4-byte records at +0x110: node id, time).
        c.cycles += 5;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xe));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        if none {
            break 'next;
        }
        c.cycles += 10;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x110));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.c_le_s(0, 1);
        if !c.fcc() {
            // The first ground collision is later than the time: next entry
            // without the extra cycle.
            c.g[A0][0] = c.g[S4][0];
            return Ok(());
        }
        c.cycles += 2;
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 7;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0xe));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let end = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            if end {
                break 'next;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
            c.c_le_s(0, 1);
            let passed = c.fcc();
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if !passed {
                break 'next;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9f48);
            }
        }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S4][0];
    Ok(())
}

// ---------------------------------------------------------------------------
// GraphicsMan_DrawIndexedTriangles (0x369cf8)

/// `void GraphicsMan_DrawIndexedTriangles(GraphicsMan *this, int numTris,
/// .., const u8 *indices (a3), const Vec2 *uv (t1), const Color *colours
/// (t2), const Vec3 *positions (t0), const Vec3 *offset (t3))` (0x369cf8):
/// draw `numTris` indexed triangles, each as a 3-vertex strip through the
/// manager's `RenderTriStrip` (vtable +0x238/+0x23c, `(3, verts, 0)`). Each
/// corner is the indexed position plus `*offset` (w = 1), the colour halved
/// and truncated to integers, and the texture coordinate (u, v, 1) from `uv`,
/// or (0, 0, 1) when `uv` is null. The 3 corners (0x30 bytes each) are built
/// on the stack.
pub fn graphics_man_draw_indexed_triangles(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
        c.g[V0][0] = 0x2;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[V1][0] = u64::MAX;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[S0][0] = c.g[A3][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[S1][0] = c.g[T3][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S2][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S4][0] = c.g[T1][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S5][0] = c.g[T2][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S6][0] = c.g[T0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S7][0] = c.g[A1][0];
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        'epilogue: {
            'short: {
                if c.g[S4][0] != 0 {
                    // Textured: (u, v) from the UV array.
                    c.cycles += 2;
                    let no_tris = (c.g[S7][0] as i64) <= 0;
                    c.g[V1][0] = 0;
                    if no_tris {
                        break 'short;
                    }
                    c.cycles += 6;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 20);
                    c.mov_s(21, 0);
                    c.g[FP][0] = 0xc;
                    c.lwc1(9, (c.g[S1][0] as u32));
                    loop {
                        c.cycles += 6;
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.lwc1(8, (c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A3][0] = 0u64;
                        c.lwc1(7, (c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                        c.g[A2][0] = c.g[SP][0];
                        loop {
                            c.cycles += 47;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                            c.g[A0][0] = c.lbu((c.g[V0][0] as u32));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            c.g[A1][0] = ((c.g[A3][0] as i64) < 3i64) as u64;
                            c.mult(0, 2, 4, 30);
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                            c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                            c.lwc1(0, (c.g[V1][0] as u32));
                            c.add_s(2, 2, 7);
                            c.add_s(1, 1, 8);
                            c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.add_s(0, 0, 9);
                            c.lwc1(5, (c.g[V0][0] as u32));
                            c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                            c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x98));
                            c.mul_s(6, 6, 20);
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                            c.mul_s(5, 5, 20);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.mul_s(3, 3, 20);
                            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x9c));
                            c.mul_s(4, 4, 20);
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xa8));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.lwc1(0, (c.g[A0][0] as u32));
                            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                            c.swc1(0, (c.g[A2][0] as u32));
                            c.cvt_w_s(1, 5);
                            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x1c));
                            c.sq(2, (c.g[A2][0] as u32).wrapping_add(0x20));
                            c.cvt_w_s(1, 3);
                            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x10));
                            c.cvt_w_s(1, 4);
                            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0x14));
                            c.cvt_w_s(0, 6);
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x18));
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            let t = c.g[A1][0] != 0u64;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0036_9da0);
                                }
                                continue;
                            }
                            break;
                        }
                        draw_tri(c, 0x0036_9e80)?;
                        c.g[V1][0] = c.g[S3][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.lwc1(9, c.g[S1][0] as u32);
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_9d88);
                            }
                            continue;
                        }
                        c.cycles += 2;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
                        break 'epilogue;
                    }
                }
                // Untextured: (0, 0, 1).
                c.cycles += 2;
                let no_tris = (c.g[S7][0] as i64) <= 0;
                c.g[V1][0] = 0;
                if no_tris {
                    break 'short;
                }
                c.cycles += 6;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 20);
                c.mov_s(21, 0);
                c.g[S4][0] = 0xc;
                c.lwc1(9, (c.g[S1][0] as u32));
                loop {
                    c.cycles += 6;
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.lwc1(8, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0u64;
                    c.lwc1(7, (c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                    c.g[A1][0] = c.g[SP][0];
                    loop {
                        c.cycles += 44;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = ((c.g[A2][0] as i64) < 3i64) as u64;
                        c.mult(0, 7, 3, 20);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.lwc1(6, (c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[S6][0] as u32)));
                        c.lwc1(5, (c.g[V1][0] as u32));
                        c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x8));
                        c.mul_s(6, 6, 20);
                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                        c.mul_s(5, 5, 20);
                        c.lwc1(0, (c.g[V0][0] as u32));
                        c.add_s(2, 2, 7);
                        c.add_s(1, 1, 8);
                        c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                        c.add_s(0, 0, 9);
                        c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x98));
                        c.mul_s(3, 3, 20);
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x94));
                        c.mul_s(4, 4, 20);
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x9c));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb8));
                        c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x20));
                        c.cvt_w_s(0, 5);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x1c));
                        c.swc1(21, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.cvt_w_s(0, 3);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[A1][0] as u32), 0u64);
                        c.cvt_w_s(0, 4);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x14));
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), 0u64);
                        c.cvt_w_s(0, 6);
                        c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x18));
                        let t = c.g[A0][0] != 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_9ed0);
                            }
                            continue;
                        }
                        break;
                    }
                    draw_tri(c, 0x0036_9fa4)?;
                    c.g[V1][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.lwc1(9, c.g[S1][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x0036_9eb8);
                        }
                        continue;
                    }
                    break 'short;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
        }
        c.cycles += 13;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
        c.pc = t;
        Ok(())
    })
}

/// DrawIndexedTriangles: `this->RenderTriStrip(3, verts, 0)` through the
/// manager's vtable (`s2` is `this`, the vertices are at `sp`).
fn draw_tri(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 13;
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10d8));
    c.g[S0][0] = c.g[T0][0];
    c.g[A1][0] = 0x3;
    c.g[A2][0] = c.g[SP][0];
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
    c.g[A3][0] = 0;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_EmitDepthFogZCopy (0x36ae20), PS2GraphicsMan_EmitDepthFogBlend (0x36b158)

/// `void PS2GraphicsMan_EmitDepthFogZCopy(PS2GraphicsMan *this, int idx,
/// u32 texBase, u32 frameBase, int n, u64 **pp)` (0x36ae20; the generated
/// body does not use `idx`): append a GIF packet at `**pp` that copies the
/// Z buffer page into the frame buffer as a row of sprites (the depth-fog
/// pre-pass): a DMA tag, the register setup (FRAME at `frameBase`, ZBUF at
/// the current Z page, XYOFFSET centred in the 4096x4096 window, SCISSOR,
/// TEX0 at `texBase`, PRIM sprite), then one pair of sprites per two 8-pixel
/// columns of the framebuffer, `n` rows high. `*pp` advances past the packet.
pub fn ps2_graphics_man_emit_depth_fog_z_copy(c: &mut Cpu) {
    guest(c, |c| {
        'packet: {
            // Prologue; fb offset X = (0x1000 - width) / 2 << 4 (at sp+8).
            c.cycles += 141;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = 0x1000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = c.g[A0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A2][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A3][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T1][0]);
            c.sw((c.g[SP][0] as u32), c.g[T0][0]);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            c.g[S2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a30));
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            let t = c.g[V0][0] as u32;
            c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a34));
            call_ptr(c, t, 0x0036ae80)?;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036aea4)?;
            // The packet header: DMA tag, then GIF tag (A+D) with 2*cols+11 qwords.
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[S2][0];
            }
            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a80));
            c.g[FP][0] = sx(((c.g[A0][0] as u32) as i32 >> 3) as u32);
            c.g[S7][0] = c.lw((c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[FP][0] as u32) << 1);
            c.g[A1][0] = 0x8000;
            c.g[A1][0] = c.g[A1][0] << 45;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xb));
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[A0][0] = 0xe;
            c.sd((c.g[S7][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.sd((c.g[S7][0] as u32), c.g[V1][0]);
            c.g[S3][0] = sx((c.g[S3][0] as u32) >> 1);
            c.g[S3][0] = sx((c.g[S3][0] as u32) << 4);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[S3][0]);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036af0c)?;
            // FRAME_1 / ZBUF_1 / XYOFFSET / SCISSOR / TEX0 and the sprite PRIM, as 64-bit stores.
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[V1][0] = 0x1000000;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[V1][0] = 0x3fff;
            c.g[V1][0] = c.g[V1][0] << 16;
            c.g[V1][0] = c.g[V1][0] | 0x200;
            c.g[V1][0] = c.g[V1][0] << 16;
            c.g[V0][0] = c.g[S6][0] | c.g[V0][0];
            c.g[S0][0] = c.g[S0][0] << 32;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 1);
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = c.g[S0][0] >> 32;
            c.g[A0][0] = 0x8000;
            c.g[A0][0] = c.g[A0][0] << 17;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            c.g[A1][0] = c.g[S3][0] << 32;
            c.g[A1][0] = c.g[V1][0] | c.g[A1][0];
            c.g[S0][0] = c.g[S0][0] | c.g[A0][0];
            c.g[S2][0] = c.g[S2][0] << 16;
            c.g[S1][0] = c.g[S1][0] << 48;
            c.g[A2][0] = 0x30000;
            c.g[T1][0] = 0x40;
            c.g[S2][0] = c.g[S2][0] | c.g[S1][0];
            c.g[V1][0] = 0x4c;
            c.g[A0][0] = 0x4e;
            c.g[A3][0] = 0x47;
            c.g[T0][0] = 0x18;
            c.g[T2][0] = 0x3f;
            c.g[T3][0] = 0x8000;
            c.g[T3][0] = c.g[T3][0] << 24;
            c.g[T4][0] = 0x3b;
            c.g[T5][0] = 0x14;
            c.sd((c.g[S7][0] as u32).wrapping_add(0x30), c.g[A2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x40), c.g[A1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x58), c.g[T1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x28), c.g[A0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x38), c.g[A3][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x48), c.g[T0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x50), c.g[S2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x68), c.g[T2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x70), c.g[T3][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x78), c.g[T4][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x88), c.g[T5][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x60), 0u64);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x80), 0u64);
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036aff0)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[S5][0] = sx((c.g[S5][0] as u32) << 5);
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V1][0] = 0xeab2;
            c.g[V1][0] = c.g[V1][0] << 20;
            c.g[V0][0] = c.g[V0][0] >> 18;
            c.g[A1][0] = 0x6;
            c.g[V0][0] = c.g[S5][0] | c.g[V0][0];
            c.g[A2][0] = 0x116;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.g[A0][0] = 0x3f800000;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[V1][0] = 0x1;
            c.sd((c.g[S7][0] as u32).wrapping_add(0x90), c.g[V0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0x98), c.g[A1][0]);
            c.g[T1][0] = 0u64;
            c.sd((c.g[S7][0] as u32).wrapping_add(0xa0), c.g[A2][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0xb0), c.g[A0][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0xb8), c.g[V1][0]);
            c.sd((c.g[S7][0] as u32).wrapping_add(0xa8), 0u64);
            let t = (c.g[FP][0] as i64) <= 0;
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0xc0));
            if t {
                break 'packet;
            }
            c.cycles += 17;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32));
            c.g[T7][0] = c.g[S3][0] << 16;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x80000;
            c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[T5][0] = 0x3;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[T4][0] = 0x5;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[T0][0] = c.g[V0][0] << 16;
            c.g[T6][0] = c.g[V1][0] << 16;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.g[T3][0] = 0x88;
            c.g[T2][0] = 0x8;
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x100));
            // Sprite pairs, two columns per iteration.
            loop {
                c.cycles += 20;
                c.g[V0][0] = c.g[T2][0] | c.g[S0][0];
                c.g[V1][0] = c.g[A3][0] | c.g[T7][0];
                c.g[A0][0] = c.g[T3][0] | c.g[T6][0];
                c.g[A1][0] = c.g[A2][0] | c.g[T0][0];
                c.sd((c.g[S7][0] as u32), c.g[V0][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x100));
                c.sd((c.g[S7][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[FP][0] as i64)) as u64;
                c.sd((c.g[S7][0] as u32).wrapping_add(0x28), c.g[T5][0]);
                c.sd((c.g[S7][0] as u32).wrapping_add(0x38), c.g[T4][0]);
                let t = c.g[V0][0] != 0u64;
                c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x40));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_b098);
                    }
                    continue;
                }
                break;
            }
        }
        // Patch the DMA tag with the qword count and publish the new end.
        c.cycles += 27;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 0x10000000;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[A0][0] as u32), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
        c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[A0][0] as u32), c.g[S7][0]);
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

/// `void PS2GraphicsMan_EmitDepthFogBlend(PS2GraphicsMan *this, int idx,
/// u64 y, u64 **pp)` (0x36b158): append the second depth-fog packet at
/// `**pp`: blend the frame against the Z-copied texture, as vertical sprite
/// strips over the region `idx` (the rectangle at `this + idx*8 + 0x59b0`:
/// x, y, w, h), `w/32` strips of 32 pixels (two sprites each), the final
/// fog-colour sprite and the closing GIF tag. `*pp` advances past it.
pub fn ps2_graphics_man_emit_depth_fog_blend(c: &mut Cpu) {
    guest(c, |c| {
        'strips: {
            // Prologue; the region rectangle and the framebuffer offsets.
            c.cycles += 166;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[T1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[T0][0] = c.g[T1][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.g[T0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.g[A1][0] = c.g[V0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S4][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S1][0] = 0x1000;
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[FP][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b2));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.sw((c.g[SP][0] as u32), c.g[A3][0]);
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[S3][0] = c.lhu((c.g[T0][0] as u32).wrapping_add(0x59b0));
            c.g[S0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x59b4));
            c.g[S2][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0x59b6));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S3][0]);
            let t = c.g[V0][0] as u32;
            c.g[S2][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[S2][0] as u32)));
            call_ptr(c, t, 0x0036b1d4)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            let t = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036b200)?;
            // The packet header and the register setup (TEX0 at the Z page, FRAME, SCISSOR, PRIM).
            c.g[V1][0] = u64::MAX;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[S0][0];
            }
            c.g[S6][0] = c.lw((c.g[A1][0] as u32));
            c.g[S5][0] = sx(((c.g[A0][0] as u32) as i32 >> 5) as u32);
            c.g[A2][0] = 0x10000000;
            c.g[A1][0] = sx((c.g[S5][0] as u32) << 2);
            c.g[S3][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x5a84));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[S2][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x5a80));
            c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb));
            c.sd((c.g[S6][0] as u32), c.g[A2][0]);
            c.g[A0][0] = 0x8000;
            c.g[A0][0] = c.g[A0][0] << 45;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32));
            c.g[A1][0] = c.g[A1][0] | c.g[A0][0];
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0xe;
            c.g[V1][0] = c.lw((c.g[A2][0] as u32));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
            c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[FP][0] as u32)));
            c.sd((c.g[S6][0] as u32), c.g[A1][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036b288)?;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A1][0] = c.g[S3][0] << 32;
            c.g[V1][0] = 0x1000000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[S2][0] = c.g[S2][0] | c.g[V1][0];
            c.g[S0][0] = c.g[S0][0] << 16;
            c.g[S0][0] = c.g[A0][0] | c.g[S0][0];
            c.g[V1][0] = c.g[FP][0] << 32;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] >> 16;
            c.g[S2][0] = c.g[S2][0] << 32;
            c.g[A1][0] = c.g[A1][0] >> 32;
            c.g[S2][0] = c.g[S2][0] >> 32;
            c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
            c.g[V1][0] = 0x8000;
            c.g[V1][0] = c.g[V1][0] << 17;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[S2][0] = c.g[S2][0] | c.g[V1][0];
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.g[V1][0] = 0x4e;
            c.g[A0][0] = c.g[A0][0] << 48;
            c.g[A2][0] = c.g[S1][0] << 32;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[S0][0] = c.g[S0][0] | c.g[A0][0];
            c.g[A2][0] = c.g[V0][0] | c.g[A2][0];
            c.g[A0][0] = 0x70000;
            c.g[V0][0] = 0x4c;
            c.g[A3][0] = 0x47;
            c.g[T0][0] = 0x18;
            c.g[T1][0] = 0x42;
            c.g[T2][0] = 0x3f;
            c.g[T3][0] = 0x40;
            c.g[T4][0] = 0x14;
            c.g[V1][0] = 0x1;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[A3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x40), c.g[A2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x48), c.g[T0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x58), c.g[T1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x68), c.g[T2][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x70), c.g[S0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x78), c.g[T3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x88), c.g[T4][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x50), c.g[V1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x60), 0u64);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x80), 0u64);
            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x0036b374)?;
            c.g[S3][0] = sx((c.g[S3][0] as u32) << 5);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
            c.g[S3][0] = c.g[S3][0] << 32;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = c.g[V0][0] >> 18;
            c.g[V1][0] = 0xea9b;
            c.g[V1][0] = c.g[V1][0] << 20;
            c.g[S3][0] = c.g[S3][0] >> 32;
            c.g[S4][0] = c.g[S4][0] << 37;
            c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
            c.g[S4][0] = c.g[S4][0] | c.g[V1][0];
            c.g[V0][0] = 0x8000;
            c.g[V0][0] = c.g[V0][0] << 46;
            c.g[A0][0] = 0x3f800000;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[A0][0] = c.g[A0][0] << 16;
            c.g[A0][0] = c.g[A0][0] | 0x8080;
            c.g[S3][0] = c.g[S3][0] | c.g[S4][0];
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb0), c.g[A0][0]);
            c.g[S3][0] = c.g[S3][0] | c.g[V0][0];
            c.g[V1][0] = 0x6;
            c.g[V0][0] = 0x156;
            c.g[A0][0] = 0x1;
            c.sd((c.g[S6][0] as u32).wrapping_add(0x90), c.g[S3][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0x98), c.g[V1][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xb8), c.g[A0][0]);
            c.sd((c.g[S6][0] as u32).wrapping_add(0xa8), 0u64);
            let t = c.g[S5][0] == 0u64;
            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc0));
            if t {
                break 'strips;
            }
            c.cycles += 24;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[FP][0] as u32) << 4);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[T2][0] = c.g[S5][0];
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[T5][0] = 0x3;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 4);
            c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[S1][0] = c.g[A1][0] << 16;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[T7][0] = c.g[V0][0] << 16;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.g[S0][0] = c.g[A0][0] << 16;
            c.g[T6][0] = c.g[V1][0] << 16;
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.g[T4][0] = 0xffff;
            c.g[T4][0] = c.g[T4][0] << 32;
            c.g[T3][0] = 0x5;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x208));
            // One iteration per strip.
            loop {
                c.cycles += 21;
                c.g[V1][0] = c.g[T0][0] | c.g[S0][0];
                c.g[A0][0] = c.g[T1][0] | c.g[S1][0];
                c.g[V0][0] = c.g[A3][0] | c.g[T7][0];
                c.g[A1][0] = c.g[A2][0] | c.g[T6][0];
                c.g[V1][0] = c.g[V1][0] | c.g[T4][0];
                c.g[A0][0] = c.g[A0][0] | c.g[T4][0];
                c.sd((c.g[S6][0] as u32), c.g[V0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x200));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[T5][0]);
                c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[T3][0]);
                let t = c.g[T2][0] != 0u64;
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0036_b450);
                    }
                    continue;
                }
                break;
            }
        }
        // The closing sprites (fog colour over the framebuffer) and the epilogue.
        c.cycles += 81;
        c.g[V0][0] = 0x10000000;
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.g[V1][0] = 0x10000000;
        c.g[V1][0] = c.g[V1][0] << 32;
        c.g[V1][0] = c.g[V1][0] | 0x8003;
        c.g[A0][0] = 0xe;
        c.sd((c.g[S6][0] as u32), c.g[V0][0]);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[S1][0] = 0x1000;
        c.sd((c.g[S6][0] as u32).wrapping_add(0x18), c.g[A0][0]);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x8), 0u64);
        c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x5a84));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0036b4e8)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
        c.g[S0][0] = c.g[S0][0] << 32;
        c.g[V0][0] = c.g[V0][0] << 32;
        c.g[S0][0] = c.g[S0][0] >> 32;
        c.g[V0][0] = c.g[V0][0] >> 16;
        c.g[V1][0] = 0x4c;
        c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
        c.sd((c.g[S6][0] as u32).wrapping_add(0x28), c.g[V1][0]);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0036b520)?;
        c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x10d8));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[S0][0] = sx((c.g[S0][0] as u32) >> 1);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0036b540)?;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
        c.g[A3][0] = 0x40;
        c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
        c.g[V0][0] = 0x18;
        c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
        c.sd((c.g[S6][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[S1][0] = c.g[S1][0] << 32;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[S0][0] | c.g[S1][0];
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sd((c.g[S6][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5900));
        c.g[V1][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5904));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5902));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A2][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5906));
        c.g[V1][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5900));
        c.g[V0][0] = c.g[V0][0] << 16;
        c.g[A1][0] = c.lhu((c.g[S7][0] as u32).wrapping_add(0x5902));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[A1][0] = c.g[A1][0] << 32;
        c.g[A0][0] = c.g[A0][0] << 48;
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.sd((c.g[S6][0] as u32).wrapping_add(0x48), c.g[A3][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sd((c.g[S6][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A2][0] = c.lw((c.g[SP][0] as u32));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[A2][0] as u32), c.g[S6][0]);
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP, SP as STACK_TOP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Avalanche records (0x400 bytes each), their nodes (0x200 bytes, 4 per
    /// avalanche), the nodes' instances and the danger-manager chain.
    const OC1063_AV: u32 = DATA + 0x5_0000;
    const OC1063_AV_NODES: u32 = DATA + 0x6_0000;
    const OC1063_INST: u32 = DATA + 0x7_0000;
    const OC1063_DANGER: u32 = DATA + 0x8_0000;
    /// A harmless node-sized block that "occupied" active nodes point at.
    const OC1063_DUMMY: u32 = DATA + 0x9_0000;
    pub(crate) const OC1063_IDS: [u32; 3] = [10, 20, 30];

    /// An avalanche list of `n_av` avalanches (ids 10, 20, 30) at `gp+0x9c0`,
    /// each with up to `max_nodes` nodes (emitter flag at +0xf2 random when
    /// `emitters`), 0-5 collisions (ascending times) and 0-5 ground collisions,
    /// and the danger-manager chain reached through the world at `gp-0x848`.
    pub(crate) fn oc1063_world(
        c: &mut Cpu,
        r: &mut Rng,
        n_av: u32,
        max_nodes: u32,
        emitters: bool,
    ) {
        for a in 0..n_av {
            let av = OC1063_AV + 0x400 * a;
            c.bus.write32(av, OC1063_IDS[a as usize]);
            c.bus
                .write32(av + 8, if a + 1 < n_av { av + 0x400 } else { 0 });
            let n_nodes = r.next_u32() % (max_nodes + 1);
            c.bus.write32(
                av + 4,
                if n_nodes == 0 {
                    0
                } else {
                    OC1063_AV_NODES + 0x1000 * a
                },
            );
            for k in 0..n_nodes {
                let node = OC1063_AV_NODES + 0x1000 * a + 0x200 * k;
                for off in (0..0x120).step_by(4) {
                    c.bus.write32(node + off, r.f32(100.0));
                }
                c.bus.write16(node + 0xf0, (r.next_u32() % 4) as u16);
                let emitter = emitters && r.next_u32() % 2 == 0;
                c.bus.write8(node + 0xf2, emitter as u8);
                let next = if k + 1 < n_nodes { node + 0x200 } else { 0 };
                c.bus.write32(node + 0xf4, next);
                let inst = OC1063_INST + 0x100 * (4 * a + k);
                c.bus.write32(node + 0xf8, inst);
                c.bus.write32(node + 0x114, 0);
                for off in (0..0x80).step_by(4) {
                    c.bus.write32(inst + off, r.f32(100.0));
                }
            }
            // Collisions: {time f32, 0} at +0x10; ground: {u16 node, u16 time} at +0x110.
            let n_coll = r.next_u32() % 6;
            let n_ground = r.next_u32() % 6;
            c.bus.write16(av + 0xc, n_coll as u16);
            c.bus.write16(av + 0xe, n_ground as u16);
            let mut t = 0.0f32;
            for i in 0..n_coll {
                t += (r.next_u32() % 20) as f32 * 0.25;
                c.bus.write32(av + 0x10 + 8 * i, t.to_bits());
            }
            let mut t = 0u32;
            for i in 0..n_ground {
                t += r.next_u32() % 6;
                c.bus.write16(av + 0x110 + 4 * i, r.next_u32() as u16);
                c.bus.write16(av + 0x112 + 4 * i, t as u16);
            }
        }
        let gp = GAME_GP;
        c.bus
            .write32(gp + AVS_GP_LIST, if n_av == 0 { 0 } else { OC1063_AV });
        // Loaded history data that is already applied (ApplyHistoryData returns at once).
        let loaded = r.next_u32() % 2 == 0;
        c.bus
            .write32(gp + 0x9c4, if loaded { OC1063_DUMMY } else { 0 });
        c.bus.write32(gp + 0x9cc, loaded as u32);
        // gp-0x848: the world; +0x84 -> +0xc -> +0xa8 -> the danger manager (+0x708: active count).
        c.bus.write32(gp.wrapping_sub(0x848), OC1063_DANGER);
        c.bus.write32(OC1063_DANGER + 0x84, OC1063_DANGER + 0x100);
        c.bus.write32(OC1063_DANGER + 0x10c, OC1063_DANGER + 0x200);
        c.bus.write32(OC1063_DANGER + 0x2a8, OC1063_DANGER + 0x300);
        c.bus.write32(OC1063_DANGER + 0xa08, r.next_u32() % 4);
    }

    /// Avalanche_Start: ids that match the first, a later or no avalanche; the
    /// slots free, partly used, all used (the next avalanche is tried), or
    /// holding this avalanche already; the 64 active nodes free, partly used
    /// or all but 1-2 used (the nodes run out, the slot is given back).
    #[test]
    fn avalanche_start_matches() {
        covered(
            ee::s09::f_002d97a8__generated,
            avalanche_start,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d97a8, 6);
                c.pc = 0x002d_97a8;
                let n_av = 1 + r.next_u32() % 3;
                oc1063_world(c, &mut r, n_av, 3, seed % 3 != 0);
                let target = r.next_u32() % n_av;
                c.g[A0][0] = if seed % 13 == 0 {
                    99
                } else {
                    OC1063_IDS[target as usize] as u64
                };
                let slots_mode = seed % 6;
                for i in 0..16u32 {
                    let slot = AVS_SLOTS + 0x1c * i;
                    let used = match slots_mode {
                        1 => r.next_u32() % 2 == 0,
                        2 => i == (seed / 6) % 16,
                        3 => true,
                        _ => false,
                    };
                    if used {
                        let v = if slots_mode == 2 {
                            OC1063_AV + 0x400 * target
                        } else {
                            OC1063_DUMMY + 0x40 * i
                        };
                        c.bus.write32(slot, v);
                    }
                }
                let nodes_mode = (seed / 6) % 4;
                let free = 1 + r.next_u32() % 2;
                let free_at = [r.next_u32() % 64, r.next_u32() % 64];
                for i in 0..64u32 {
                    let used = match nodes_mode {
                        1 => i < r.next_u32() % 6,
                        2 => !(free_at[..free as usize].contains(&i)),
                        3 => true,
                        _ => false,
                    };
                    if used {
                        c.bus
                            .write32(AVS_NODES + AVS_NODE_SIZE * i + 0x2e0, OC1063_DUMMY);
                    }
                }
            },
        );
    }

    /// Gate and mailbox of this module's fake `Avalanche_SampleHistory`.
    const OC1063_GATE: u32 = DATA + 0xf_e000;
    const OC1063_MAGIC: u32 = 0x4156_5331;
    static OC1063_PREV_SAMPLE: AtomicUsize = AtomicUsize::new(0);

    fn oc1063_fake_sample_history(c: &mut Cpu) {
        if c.bus.read32(OC1063_GATE) != OC1063_MAGIC {
            return fakes::fall_back(c, &OC1063_PREV_SAMPLE, ee::s09::f_002d5778__generated);
        }
        fakes::fake_call(c, 0x002d_5778);
    }

    /// Avalanche_LoadState: 16 saved entries (valid ids, repeats, unknown ids,
    /// negative ids; zero and positive times) restore into slots that were
    /// idle or in use; `Read` and `Avalanche_SampleHistory` are fakes.
    #[test]
    fn avalanche_load_state_matches() {
        const STREAM: u32 = DATA + 0x400;
        const STREAM_VT: u32 = DATA + 0x500;
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D5778),
            oc1063_fake_sample_history,
            &OC1063_PREV_SAMPLE,
        );
        covered(
            ee::s09::f_002d9d68__generated,
            avalanche_load_state,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d9d68, 6);
                c.pc = 0x002d_9d68;
                c.bus.write32(OC1063_GATE, OC1063_MAGIC);
                let n_av = r.next_u32() % 4;
                oc1063_world(c, &mut r, n_av, 2, seed % 4 == 0);
                c.bus.write32(STREAM, STREAM_VT);
                c.bus
                    .write16(STREAM_VT + 0x10, (4 * (r.next_u32() % 4)) as u16);
                c.bus.write32(STREAM_VT + 0x14, FAKE);
                c.g[A0][0] = STREAM as u64;
                // The record the (fake) Read leaves in the frame at sp - 0x150.
                let buf = STACK_TOP - 0x150;
                for i in 0..16u32 {
                    let id = match r.next_u32() % 6 {
                        0 => 0xffff_ffff,
                        1 => 77,
                        _ => OC1063_IDS[(r.next_u32() % 3) as usize],
                    };
                    c.bus.write32(buf + 4 * i, id);
                    let time = match r.next_u32() % 4 {
                        0 => 0,
                        _ => ((r.next_u32() % 40) as f32 * 0.25).to_bits(),
                    };
                    c.bus.write32(buf + 0x40 + 4 * i, time);
                }
                for i in 0..16u32 {
                    if r.next_u32() % 4 == 0 {
                        c.bus.write32(AVS_SLOTS + 0x1c * i, OC1063_DUMMY + 0x40 * i);
                    }
                }
            },
        );
    }

    /// Fixture of the three graphics functions: the manager at `this`, its
    /// sub-object vtable (`this+0x10d8`: method deltas at +0x238/+0x50/+0x58
    /// = 0, entries at +0x23c/+0x54/+0x5c = fake methods).
    const OC1063_THIS: u32 = DATA + 0x400;
    const OC1063_VT: u32 = DATA + 0x600;

    fn oc1063_manager(c: &mut Cpu) {
        c.bus.write32(OC1063_THIS + 0x10d8, OC1063_VT);
        for off in [0x238, 0x50, 0x58] {
            c.bus.write16(OC1063_VT + off, 0);
        }
        for off in [0x23c, 0x54, 0x5c] {
            c.bus.write32(OC1063_VT + off, FAKE);
        }
    }

    /// GraphicsMan_DrawIndexedTriangles: 0-3 triangles (and a negative count)
    /// with and without UVs; `RenderTriStrip` is a fake that may unwind.
    #[test]
    fn graphics_man_draw_indexed_triangles_matches() {
        check_lohi(
            ee::s12::f_00369cf8__generated,
            graphics_man_draw_indexed_triangles,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x369cf8, 3);
                c.pc = 0x0036_9cf8;
                oc1063_manager(c);
                let (indices, uv, colours, positions, offset) = (
                    DATA + 0x800,
                    DATA + 0xa00,
                    DATA + 0xc00,
                    DATA + 0xe00,
                    DATA + 0xf80,
                );
                for i in 0..12 {
                    c.bus.write8(indices + i, (r.next_u32() % 8) as u8);
                }
                for i in 0..16 {
                    c.bus.write32(uv + 4 * i, r.edge_f32());
                }
                for i in 0..32 {
                    c.bus.write32(colours + 4 * i, r.edge_f32());
                }
                for i in 0..24 {
                    c.bus.write32(positions + 4 * i, r.edge_f32());
                }
                for i in 0..3 {
                    c.bus.write32(offset + 4 * i, r.edge_f32());
                }
                c.g[A0][0] = OC1063_THIS as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    n => n as u64 - 1,
                };
                c.g[A3][0] = indices as u64;
                c.g[T1][0] = if seed % 2 == 0 { uv as u64 } else { 0 };
                c.g[T2][0] = colours as u64;
                c.g[T0][0] = positions as u64;
                c.g[T3][0] = offset as u64;
            },
        );
    }

    /// Force the results of the first fake calls to plausible widths.
    fn oc1063_widths(c: &mut Cpu, r: &mut Rng, n: u32) {
        for i in 0..n {
            c.bus
                .write32(LOG_RESULTS + 4 * i, 512 + 16 * (r.next_u32() % 32));
        }
    }

    /// PS2GraphicsMan_EmitDepthFogZCopy: frame sizes from 0 to 130 pixels
    /// (and a negative one) so the sprite loop runs 0 to 16 times; the width
    /// queries are fake method calls.
    #[test]
    fn ps2_graphics_man_emit_depth_fog_z_copy_matches() {
        const PP: u32 = DATA + 0x500;
        const PACKET: u32 = DATA + 0x2_0000;
        covered(
            ee::s12::f_0036ae20__generated,
            ps2_graphics_man_emit_depth_fog_z_copy,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36ae20, 4);
                c.pc = 0x0036_ae20;
                oc1063_manager(c);
                oc1063_widths(c, &mut r, 4);
                let width = if seed % 11 == 0 {
                    0xffff_fffd
                } else if seed % 7 == 0 {
                    0
                } else {
                    r.next_u32() % 131
                };
                c.bus.write32(OC1063_THIS + 0x5a30, width);
                c.bus.write32(OC1063_THIS + 0x5a34, 1 + r.next_u32() % 600);
                c.bus.write32(OC1063_THIS + 0x5a80, r.next_u32() & 0xffff);
                c.bus.write32(PP, PACKET + 16 * (r.next_u32() % 4));
                c.g[A0][0] = OC1063_THIS as u64;
                c.g[T0][0] = (r.next_u32() % 64) as u64;
                c.g[T1][0] = PP as u64;
            },
        );
    }

    /// PS2GraphicsMan_EmitDepthFogBlend: region widths of 0 to 4 strips (and
    /// more), four regions to pick from; the width queries are fake calls.
    #[test]
    fn ps2_graphics_man_emit_depth_fog_blend_matches() {
        const PP: u32 = DATA + 0x500;
        const PACKET: u32 = DATA + 0x2_0000;
        covered(
            ee::s12::f_0036b158__generated,
            ps2_graphics_man_emit_depth_fog_blend,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36b158, 7);
                c.pc = 0x0036_b158;
                oc1063_manager(c);
                oc1063_widths(c, &mut r, 8);
                for idx in 0..4u32 {
                    let rect = OC1063_THIS + 0x59b0 + 8 * idx;
                    c.bus.write16(rect, (r.next_u32() % 640) as u16);
                    c.bus.write16(rect + 2, (r.next_u32() % 480) as u16);
                    let w = match r.next_u32() % 5 {
                        0 => 0,
                        1 => 31,
                        _ => r.next_u32() % 200,
                    };
                    c.bus.write16(rect + 4, w as u16);
                    c.bus.write16(rect + 6, (r.next_u32() % 480) as u16);
                }
                for off in [0x5900, 0x5902, 0x5904, 0x5906] {
                    c.bus
                        .write16(OC1063_THIS + off, (r.next_u32() % 700) as u16);
                }
                c.bus.write32(OC1063_THIS + 0x5a80, r.next_u32() & 0xffff);
                c.bus.write32(OC1063_THIS + 0x5a84, r.next_u32() & 0xffff);
                c.bus.write32(PP, PACKET + 16 * (r.next_u32() % 4));
                c.g[A0][0] = OC1063_THIS as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = PP as u64;
            },
        );
    }
}
