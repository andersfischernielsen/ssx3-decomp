//! Staging module oc374 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc374`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_3900, path_track_queue_node, "PATH_TrackQueueNode"),
    (0x003b_6bd0, sndstrm_parse_header, "SNDSTRM_ParseHeader"),
    (0x003d_3d20, path_track_advance, "PATH_TrackAdvance"),
    (0x0040_db58, ieee754_log, "__ieee754_log"),
    (0x0010_8388, rider_play_bump_anim, "Rider_PlayBumpAnim"),
    (
        0x002f_d420,
        world_obj_factory_particle_node,
        "WorldObjFactory_ParticleNode",
    ),
    (
        0x0012_d4e8,
        rider_wipeout_sub_update_phase1,
        "RiderWipeoutSub_UpdatePhase1",
    ),
    (0x0026_d818, world_state_write, "WorldState_Write"),
    (0x0032_8c20, spatial_hash_insert, "SpatialHash_Insert"),
    (
        0x002a_0a30,
        ssx_audio_system_speech_rider_collision,
        "SSXAudioSystem_SpeechRiderCollision",
    ),
];

// ---------------------------------------------------------------------------
// PATH_TrackQueueNode (0x3d3900, EA path/music-track library)

/// The path library's state pointer (`0x450000 - 0xbe0`).
pub(crate) const PATH_STATE_PTR: u32 = 0x0044_f420;
/// Per-node callback `void (*)(arg, int node)` and its first argument.
pub(crate) const PATH_NODE_CALLBACK: u32 = 0x0044_f424;
pub(crate) const PATH_NODE_CALLBACK_ARG: u32 = 0x0044_f430;
/// The current track time (divided by 23 into a 7-bit "velocity").
pub(crate) const PATH_TRACK_TIME: u32 = 0x0044_f438;

/// Path state fields: node pool base, node index table (shorts, in words
/// from the pool), branch-target index table, event table, voice pointers.
pub(crate) const PATH_NODES: u32 = 0x34;
pub(crate) const PATH_NODE_INDEX: u32 = 0x38;
pub(crate) const PATH_BRANCH_INDEX: u32 = 0x40;
pub(crate) const PATH_EVENT_TABLE: u32 = 0x54;
pub(crate) const PATH_VOICES: u32 = 0x58;

/// Voice fields: flags word, current node index (short), current node.
pub(crate) const PATH_VOICE_FLAGS: u32 = 0x20;
pub(crate) const PATH_VOICE_NODE: u32 = 0x2c;
pub(crate) const PATH_VOICE_CUR: u32 = 0x30;

const PTQN_REMAP_NODE: Callee = Callee {
    addr: 0x003d_3c90,
    f: ee::s14::PATH_RemapNode,
};
const PTQN_SELECT_BRANCH: Callee = Callee {
    addr: 0x003d_37a0,
    f: ee::s14::PATH_SelectBranch,
};
const PTQN_QUEUE_APPEND: Callee = Callee {
    addr: 0x003d_0b18,
    f: ee::s14::PATH_QueueAppend,
};
const PTQN_COPY_EVENT: Callee = Callee {
    addr: 0x003d_2068,
    f: ee::s14::PATHI_copyevent,
};
const PTQN_ADD_EVENT_TO_MAPS: Callee = Callee {
    addr: 0x003d_1f80,
    f: ee::s14::PATH_AddEventToMaps,
};
/// `memset` (through the function table).
const PTQN_MEMSET: u32 = 0x0041_6210;
/// `PATH_EventMaps` base (0x517610); its +0x30 is the event map handle.
const PTQN_EVENT_MAPS: u32 = 0x0051_7610;

/// `k = (short)RemapNode(...)` (v0 on return), then the node `e` of index `k`
/// into s0, the raw first halfword (its type) into a1 and the loaded type in
/// v1 / v0 as the loop-end reload does.
fn ptqn_reload_node(c: &mut Cpu) {
    c.g[A1][0] = c.lw(PATH_STATE_PTR);
    c.g[V0][0] = sx(r32(c, V0) << 16);
    c.g[S1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODE_INDEX));
    c.g[V1][0] = sx(r32(c, S1) << 1);
    c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODES));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[V0][0] = c.lh(r32(c, V1));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[S0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lh(r32(c, S0));
}

/// `vel = (track time / 23) & 0x7f` into s3 (with the trap-check cycles).
fn ptqn_vel_from_time(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(PATH_TRACK_TIME);
    c.cycles += 4;
    c.divu(0, V0, S4);
    c.g[V0][0] = c.lo[0];
    c.g[S3][0] = c.g[V0][0] & 0x7f;
}

/// Type-0 node with an event (`w8 & 0x7c00`): builds a 16-byte event and a
/// queue on the stack, appends, copies it and adds it to the event maps.
fn ptqn_queue_node_event(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 17;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    call_ptr(c, PTQN_MEMSET, 0x003d_39c8)?; // memset(&q, 0, 16)
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    call_ptr(c, PTQN_MEMSET, 0x003d_39d8)?; // memset(&ev, 0, 16)
    let stk = sp(c);
    c.g[V1][0] = c.lw(PATH_STATE_PTR);
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(PATH_EVENT_TABLE));
    c.g[V0][0] = sx(r32(c, V0) >> 8);
    c.g[V0][0] &= 0x7c;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    // t = event table[((w8 >> 10) & 0x1f) - 1]
    c.g[A0][0] = c.lw(r32(c, V0).wrapping_sub(4));
    c.g[V1][0] = c.g[A0][0] & 0x80;
    c.g[S5][0] = sx(stk.wrapping_add(0x10));
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x64;
        c.sb(stk.wrapping_add(0xc), 0x64); // ev.bC = 100
    } else {
        c.cycles += 1;
        c.sb(stk.wrapping_add(0xc), 0);
    }
    c.cycles += 62;
    c.g[V1][0] = sx(r32(c, A0) >> 7);
    c.g[V0][0] = 0xffff_7fff_ffff_ffff; // clears ev.f1 (bit 47)
    c.g[A0][0] = c.ld(stk.wrapping_add(8));
    c.g[V1][0] &= 1;
    c.g[V1][0] <<= 47;
    c.g[A2][0] = 6;
    c.g[A0][0] &= c.g[V0][0];
    c.g[T0][0] = 0xffff_80ff_ffff_ffff; // clears ev.f7 (bits 40..46)
    c.g[A0][0] |= c.g[V1][0];
    c.g[V0][0] = 0x25a0;
    c.sd(stk.wrapping_add(8), c.g[A0][0]);
    c.g[A1][0] = 0x1f00_0001;
    c.sh(stk.wrapping_add(0xe), c.g[V0][0]); // ev.hE = 0x25a0
    c.sb(stk.wrapping_add(9), c.g[A2][0]); // ev.b9 = 6
    c.g[A0][0] = 0xffff_0fff_ffff_ffff; // clears q.b (bits 44..47)
    c.g[A2][0] = c.ld(stk.wrapping_add(0x18));
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8));
    c.g[V1][0] = c.ld(stk.wrapping_add(8));
    c.g[A2][0] &= c.g[A0][0];
    c.g[V0][0] = sx(r32(c, V0) >> 10);
    c.g[A3][0] = c.lw(PATH_TRACK_TIME);
    c.g[V0][0] &= 0x1f;
    c.g[V1][0] &= c.g[T0][0];
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    c.g[T0][0] = u64::MAX;
    c.g[V0][0] &= 0x7f;
    c.sw(stk, c.g[A1][0]); // ev.w0 = 0x1f000001
    c.g[V0][0] <<= 40;
    c.g[A1][0] = c.g[SP][0];
    c.g[V1][0] |= c.g[V0][0];
    c.sd(stk.wrapping_add(0x18), c.g[A2][0]);
    c.sd(stk.wrapping_add(8), c.g[V1][0]);
    c.g[A0][0] = c.g[S5][0];
    c.sb(stk.wrapping_add(8), c.g[T0][0]); // ev.b8 = -1
    c.g[V0][0] = c.lhu(stk.wrapping_add(0xe));
    c.g[V1][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.sw(stk.wrapping_add(0x14), c.g[V1][0]); // q.p1 = ev.hE
    c.sw(stk.wrapping_add(4), c.g[A3][0]); // ev.w4 = time + ev.hE
    PTQN_QUEUE_APPEND.call(c, 0x003d_3aec)?;
    c.g[A0][0] = c.g[S5][0];
    PTQN_COPY_EVENT.call(c, 0x003d_3af4)?;
    c.g[V1][0] = sx(PTQN_EVENT_MAPS);
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.lw(PTQN_EVENT_MAPS.wrapping_add(0x30));
    PTQN_ADD_EVENT_TO_MAPS.call(c, 0x003d_3b08)?;
    Ok(())
}

/// `short PATH_TrackQueueNode(int track, int node, int vel)` (0x3d3900):
/// walks the track's node graph from `node` (remapped through the track),
/// handling the control nodes (type <= 0) on the way: type 0 makes the
/// node current on its voice, queues its event (if any) and calls the node
/// callback; -1 sets the voice's repeat count once per node; -2 and type-0
/// nodes with the time flag take the velocity from the track time; -3 adds
/// a branch event to the event maps. Each step picks the next node with
/// `PATH_SelectBranch`. Returns the first playable node (type > 0),
/// remapped.
pub fn path_track_queue_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S3, 0x90);
        frame.save(c, S6, 0x60);
        c.g[S3][0] = c.g[A2][0]; // vel
        c.g[S6][0] = c.g[A0][0]; // track
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, FP, 0x40);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S7, 0x50);
        frame.save_ra(c, 0x30);
        let saved_vel = frame.at(c, 0x20);
        c.sw(saved_vel, c.g[S3][0]);
        PTQN_REMAP_NODE.call(c, 0x003d_393c)?;

        // k = (short)remap; e = node k; v = voices[e->h2 & 0x1f]
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[A0][0] = c.lw(PATH_STATE_PTR);
        c.g[S1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
        c.g[FP][0] = c.g[V1][0];
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODE_INDEX));
        c.g[V1][0] = sx(r32(c, S1) << 1);
        c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODES));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lh(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[S0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lhu(r32(c, S0).wrapping_add(2));
        c.g[V0][0] = c.lh(r32(c, S0));
        c.g[V1][0] &= 0x1f;
        c.g[A1][0] = c.lhu(r32(c, S0));
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        let playable = (c.g[V0][0] as i64) > 0;
        c.g[S2][0] = c.lw(r32(c, A0).wrapping_add(PATH_VOICES));

        if !playable {
            c.cycles += 3;
            c.g[S7][0] = 0x450000;
            c.g[S4][0] = 23;
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, A1) << 16);
                c.g[V1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
                let ty = c.g[V1][0];
                c.g[V0][0] = u64::MAX;
                // true: the paths through 0x3d3bfc (one more cycle).
                let via_bfc;
                if ty == 0 {
                    c.cycles += 5;
                    let cur = r32(c, S2).wrapping_add(PATH_VOICE_CUR);
                    c.sw(cur, c.g[S0][0]); // v->cur = e
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8)) & 0x7c00;
                    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
                    if c.g[V0][0] != 0 {
                        ptqn_queue_node_event(c)?;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(4));
                    c.g[V1][0] = 0xe0000;
                    c.g[A0][0] = 0x20000;
                    c.g[V0][0] &= c.g[V1][0];
                    let timed = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = 0x450000;
                    if timed {
                        ptqn_vel_from_time(c);
                        c.g[V0][0] = 0x450000;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(PATH_NODE_CALLBACK);
                    let callback = c.g[V1][0];
                    c.g[V0][0] = 0x450000;
                    if callback == 0 {
                        via_bfc = true;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.lw(PATH_NODE_CALLBACK_ARG);
                        call_ptr(c, callback as u32, 0x003d_3b54)?;
                        c.g[A0][0] = c.g[S1][0];
                        via_bfc = false;
                    }
                } else {
                    c.cycles += 2;
                    let is_repeat = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = (-2i64) as u64;
                    if is_repeat {
                        // -1: repeat count in the top nibble of the node word.
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(r32(c, S0));
                        c.g[V0][0] = 0xffff_ffff_f000_0000;
                        c.g[V0][0] &= c.g[A0][0];
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = c.g[S1][0];
                            via_bfc = false;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lh(r32(c, S2).wrapping_add(PATH_VOICE_NODE));
                            let same = c.g[S1][0] == c.g[V0][0];
                            c.g[V1][0] = 0xffff_ffff_ffff_fff0;
                            if !same {
                                c.cycles += 7;
                                let flags = r32(c, S2).wrapping_add(PATH_VOICE_FLAGS);
                                c.g[V0][0] = c.lw(flags);
                                c.g[A0][0] = sx(r32(c, A0) >> 28);
                                let node = r32(c, S2).wrapping_add(PATH_VOICE_NODE);
                                c.sh(node, c.g[S1][0]);
                                c.g[V0][0] &= c.g[V1][0];
                                c.g[V0][0] |= c.g[A0][0];
                                c.sw(flags, c.g[V0][0]);
                            }
                            via_bfc = true;
                        }
                    } else {
                        c.cycles += 2;
                        let is_timed = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = (-3i64) as u64;
                        if is_timed {
                            ptqn_vel_from_time(c);
                            via_bfc = true;
                        } else {
                            c.cycles += 2;
                            let is_branch_event = c.g[V1][0] == c.g[V0][0];
                            c.g[A0][0] = c.g[S1][0];
                            if !is_branch_event {
                                via_bfc = false;
                            } else {
                                // -3: AddEventToMaps(state->x30, node b[(wC >> 2) & 0xfff])
                                c.cycles += 12;
                                c.g[V1][0] = c.lw(PATH_STATE_PTR);
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
                                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(PATH_BRANCH_INDEX));
                                c.g[V0][0] = sx(r32(c, V0) >> 1);
                                c.g[A2][0] = c.lw(r32(c, V1).wrapping_add(PATH_NODES));
                                c.g[V0][0] &= 0x1ffe;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[A1][0] = c.lh(r32(c, V0));
                                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x30));
                                c.g[A1][0] = sx(r32(c, A1) << 2);
                                c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
                                PTQN_ADD_EVENT_TO_MAPS.call(c, 0x003d_3bfc)?;
                                via_bfc = true;
                            }
                        }
                    }
                }
                if via_bfc {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                }
                // next = RemapNode(track, SelectBranch(k, vel)); vel = saved
                c.cycles += 19;
                c.g[A1][0] = c.g[S3][0];
                PTQN_SELECT_BRANCH.call(c, 0x003d_3c08)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                let saved_vel = sp(c).wrapping_add(0x20);
                c.g[S3][0] = c.lw(saved_vel);
                PTQN_REMAP_NODE.call(c, 0x003d_3c18)?;
                ptqn_reload_node(c);
                let more = (c.g[V1][0] as i64) <= 0;
                c.g[A1][0] = c.lhu(r32(c, S0));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_3998);
                }
            }
        }

        c.cycles += 17;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[S1][0];
        PTQN_REMAP_NODE.call(c, 0x003d_3c58)?;
        c.g[V0][0] = sx(r32(c, V0) << 16);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 16) as u32);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared: gcc's inline 32-byte block copy

/// gcc's inline struct copy loop: 32-byte blocks from register `src` into
/// register `dst` until `src` reaches the value in register `end`, through
/// the temporaries `tmp` (`ldl/ldr` + `sdl/sdr`, 19 cycles a block, when
/// `unaligned`; `ld`/`sd`, 11 cycles, through `tmp_aligned` otherwise).
/// Back edges check events at `head` / `head_aligned`.
fn oc374_block_copy(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    end: usize,
    unaligned: bool,
    tmp: [usize; 4],
    head: u32,
    tmp_aligned: [usize; 4],
    head_aligned: u32,
) {
    loop {
        let s = r32(c, src);
        let d = r32(c, dst);
        if unaligned {
            c.cycles += 19;
            for (i, &t) in tmp.iter().enumerate() {
                let o = 8 * i as u32;
                c.ldl(t, s.wrapping_add(o + 7));
                c.ldr(t, s.wrapping_add(o));
            }
            for (i, &t) in tmp.iter().enumerate() {
                let o = 8 * i as u32;
                c.sdl(t, d.wrapping_add(o + 7));
                c.sdr(t, d.wrapping_add(o));
            }
        } else {
            c.cycles += 11;
            for (i, &t) in tmp_aligned.iter().enumerate() {
                c.g[t][0] = c.ld(s.wrapping_add(8 * i as u32));
            }
            for (i, &t) in tmp_aligned.iter().enumerate() {
                c.sd(d.wrapping_add(8 * i as u32), c.g[t][0]);
            }
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[end][0];
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if !more {
            if unaligned {
                c.cycles += 2;
            }
            return;
        }
        if event_due(c) {
            deliver_event_here(c, if unaligned { head } else { head_aligned });
        }
    }
}

// ---------------------------------------------------------------------------
// SNDSTRM_ParseHeader (0x3b6bd0, EA sound library streams)

/// The stream table (`SndStream*[]`, 0x510000 - 0x5f60).
pub(crate) const SNDSTRM_TABLE: u32 = 0x0050_a0a0;
/// The sound system globals (0x510000 - 0x5718): +0x179 the number of
/// stream-marker callbacks (signed byte), +0x1c8 the callbacks.
pub(crate) const SNDSTRM_SYS: u32 = 0x0050_a8e8;
pub(crate) const SNDSTRM_SYS_CALLBACK_COUNT: u32 = 0x179;
pub(crate) const SNDSTRM_SYS_CALLBACKS: u32 = 0x1c8;

/// Stream fields: file handle, play state byte, current/new header words,
/// current/new format blocks (0x68 bytes), "format changed" flag, marker
/// ids and values (zero-terminated), the current and next segment.
pub(crate) const SNDSTRM_FILE: u32 = 0x0;
pub(crate) const SNDSTRM_STATE: u32 = 0x10;
pub(crate) const SNDSTRM_HEADER: u32 = 0x14;
pub(crate) const SNDSTRM_NEW_HEADER: u32 = 0x18;
pub(crate) const SNDSTRM_FORMAT: u32 = 0x1c;
pub(crate) const SNDSTRM_NEW_FORMAT: u32 = 0x84;
pub(crate) const SNDSTRM_FORMAT_CHANGED: u32 = 0x9c;
pub(crate) const SNDSTRM_MARKERS: u32 = 0xcc;
pub(crate) const SNDSTRM_MARKER_VALUES: u32 = 0xdc;
pub(crate) const SNDSTRM_FIRST_SEGMENT: u32 = 0x11c;
pub(crate) const SNDSTRM_SEGMENT: u32 = 0x134;

const SNDSTRM_PARSE_STREAM_HEADER: Callee = Callee {
    addr: 0x003b_b820,
    f: ee::s13::SND_ParseStreamHeader,
};
const SNDSTRM_CALC_DATA_RATE: Callee = Callee {
    addr: 0x003b_6948,
    f: ee::s13::SNDSTRM_CalcDataRate,
};
const SNDSTRM_START_PLAYBACK: Callee = Callee {
    addr: 0x003b_68b8,
    f: ee::s13::SNDSTRM_StartPlayback,
};
/// `StreamFile_ReleaseChunk(file, chunk)` and `memcmp` (through the table).
const SNDSTRM_RELEASE_CHUNK: u32 = 0x003e_13e8;
const SNDSTRM_MEMCMP: u32 = 0x0041_5fc8;

/// `int SNDSTRM_ParseHeader(int stream, StreamChunk* chunk)` (0x3b6bd0):
/// moves the stream to its next segment, parses the chunk's header into
/// the stream's new header and format, reports the pending markers to the
/// marker callbacks (message 3: id, value, segment's +0xc) and clears
/// them, releases the chunk and sets the segment's data rate. A header or
/// format that differs from the current one (or a flagged change) while
/// one is playing stops the stream (state 2); otherwise the new one is
/// taken over and playback starts (state 1) if it is not running yet.
/// Returns 0.
pub fn sndstrm_parse_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        frame.save(c, S2, 0xc0);
        c.g[V0][0] = sx(SNDSTRM_TABLE);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        frame.save(c, S4, 0xa0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SNDSTRM_TABLE));
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);
        let chunk_slot = frame.at(c, 0x40);
        c.sw(chunk_slot, c.g[A1][0]);
        frame.save(c, S3, 0xb0);
        c.g[S3][0] = c.lw(r32(c, A0)); // the stream
        let stream = r32(c, S3);
        c.g[V0][0] = c.lw(stream.wrapping_add(SNDSTRM_SEGMENT));
        // segment = segment ? segment->next : first segment
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(r32(c, V0));
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw(stream.wrapping_add(SNDSTRM_FIRST_SEGMENT));
        }

        c.cycles += 24;
        c.sw(stream.wrapping_add(SNDSTRM_SEGMENT), c.g[V0][0]);
        c.g[S6][0] = c.lw(stream.wrapping_add(SNDSTRM_SEGMENT));
        let stk = sp(c);
        c.g[V1][0] = sx(stream.wrapping_add(SNDSTRM_HEADER));
        c.g[V0][0] = c.lw(stk.wrapping_add(0x40));
        c.g[A2][0] = sx(stream.wrapping_add(SNDSTRM_NEW_HEADER));
        c.g[A3][0] = sx(stream.wrapping_add(SNDSTRM_NEW_FORMAT));
        c.g[A0][0] = 0;
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(8));
        c.sw(stk.wrapping_add(0x48), c.g[V1][0]);
        c.sw(stk.wrapping_add(0x4c), c.g[A2][0]);
        c.g[T0][0] = c.g[SP][0];
        c.sw(stk.wrapping_add(0x44), c.g[A3][0]);
        c.g[T1][0] = sx(r32(c, S6).wrapping_add(0x25));
        SNDSTRM_PARSE_STREAM_HEADER.call(c, 0x003b_6c64)?;

        let stk = sp(c);
        let stream = r32(c, S3);
        c.g[V1][0] = c.lw(stk);
        c.g[V0][0] = 0;
        c.g[A0][0] = sx(stream.wrapping_add(SNDSTRM_MARKERS));
        c.g[A1][0] = 0;
        c.sb(r32(c, S6).wrapping_add(0x24), 0);
        c.g[V0][0] = c.g[A0][0];
        c.sw(r32(c, S6).wrapping_add(0x18), c.g[V1][0]);
        c.g[V1][0] = c.lw(r32(c, V0));
        c.g[S7][0] = c.g[A0][0];
        if c.g[V1][0] != 0 {
            // Report each pending marker to every marker callback.
            c.cycles += 5;
            c.g[S4][0] = c.g[A1][0];
            c.g[FP][0] = 3;
            c.g[S2][0] = sx(stream.wrapping_add(SNDSTRM_MARKER_VALUES));
            c.sw(stk.wrapping_add(0x20), c.g[FP][0]); // msg.type = 3
            loop {
                c.cycles += 16;
                let stk = sp(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(4));
                c.g[S1][0] = 0;
                c.g[V0][0] = c.lw(r32(c, A0));
                c.sw(stk.wrapping_add(0x24), c.g[V0][0]);
                c.g[V1][0] = c.lw(r32(c, S2));
                c.sw(stk.wrapping_add(0x28), c.g[V1][0]);
                c.g[V0][0] = c.lw(r32(c, S6).wrapping_add(0xc));
                c.sw(stk.wrapping_add(0x30), c.g[V0][0]);
                c.sw(r32(c, A0), 0);
                c.sw(r32(c, S2), 0);
                c.g[A0][0] = sx(SNDSTRM_SYS);
                c.g[V0][0] = c.lb(SNDSTRM_SYS + SNDSTRM_SYS_CALLBACK_COUNT);
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                if !none {
                    c.cycles += 4;
                    c.g[A2][0] = 0x510000;
                    c.g[S5][0] = sx(SNDSTRM_SYS);
                    c.g[S0][0] = sx(SNDSTRM_SYS + SNDSTRM_SYS_CALLBACKS);
                    c.g[V1][0] = c.lw(r32(c, S0));
                    loop {
                        c.cycles += 8;
                        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                        let callback = r32(c, V1);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                        call_ptr(c, callback, 0x003b_6d00)?;
                        c.g[V0][0] = c.lb(r32(c, S5).wrapping_add(SNDSTRM_SYS_CALLBACK_COUNT));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[V1][0] = c.lw(r32(c, S0));
                        if event_due(c) {
                            deliver_event_here(c, 0x003b_6cf0);
                        }
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S7][0];
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
                c.g[V1][0] = c.lw(r32(c, V0));
                if c.g[V1][0] == 0 {
                    break;
                }
                let msg = sp(c).wrapping_add(0x20);
                c.sw(msg, c.g[FP][0]);
                if event_due(c) {
                    deliver_event_here(c, 0x003b_6ca0);
                }
            }
        }

        c.cycles += 12;
        let stk = sp(c);
        c.g[A1][0] = c.lw(stk.wrapping_add(0x40));
        c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(SNDSTRM_FILE));
        call_ptr(c, SNDSTRM_RELEASE_CHUNK, 0x003b_6d30)?;
        c.g[A0][0] = c.lw(sp(c).wrapping_add(0x4c));
        SNDSTRM_CALC_DATA_RATE.call(c, 0x003b_6d38)?;
        let stk = sp(c);
        c.g[A0][0] = c.lw(stk.wrapping_add(0x48));
        c.g[A2][0] = 4;
        c.sw(r32(c, S6).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = c.lw(stk.wrapping_add(0x4c));
        call_ptr(c, SNDSTRM_MEMCMP, 0x003b_6d4c)?;
        // The header or the format changed (or a change is flagged).
        let mut changed = false;
        if c.g[V0][0] != 0 {
            changed = true;
        } else {
            c.cycles += 6;
            c.g[A1][0] = c.lw(sp(c).wrapping_add(0x44));
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(SNDSTRM_FORMAT));
            c.g[A2][0] = 0x68;
            call_ptr(c, SNDSTRM_MEMCMP, 0x003b_6d64)?;
            if c.g[V0][0] != 0 {
                changed = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(SNDSTRM_FORMAT_CHANGED));
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    changed = true;
                }
            }
        }
        let mut stopped = false;
        if changed {
            c.g[V0][0] = c.lhu(r32(c, S3).wrapping_add(SNDSTRM_HEADER));
            c.cycles += 2;
            let idle = c.g[V0][0] == 0;
            c.g[V1][0] = 2;
            if !idle {
                // A stream is playing in another format: stop it.
                c.cycles += 4;
                c.g[V0][0] = 0;
                c.sb(r32(c, S3).wrapping_add(SNDSTRM_STATE), c.g[V1][0]);
                frame.restore(c, S0, 0xe0);
                stopped = true;
            } else {
                // Take over the new header word and format block.
                c.cycles += 10;
                let stream = r32(c, S3);
                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x44));
                c.g[V1][0] = sx(stream.wrapping_add(SNDSTRM_FORMAT));
                c.lwl(A3, stream.wrapping_add(SNDSTRM_NEW_HEADER + 3));
                c.lwr(A3, stream.wrapping_add(SNDSTRM_NEW_HEADER));
                c.swl(A3, stream.wrapping_add(SNDSTRM_HEADER + 3));
                c.swr(A3, stream.wrapping_add(SNDSTRM_HEADER));
                c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
                c.g[V0][0] &= 7;
                let aligned = c.g[V0][0] == 0;
                c.g[V0][0] = sx(stream.wrapping_add(0xe4));
                oc374_block_copy(
                    c,
                    A1,
                    V1,
                    V0,
                    !aligned,
                    [A3, T0, A0, A2],
                    0x003b_6dbc,
                    [A3, T0, A0, A2],
                    0x003b_6e10,
                );
                c.cycles += 5;
                let (s, d) = (r32(c, A1), r32(c, V1));
                c.ldl(A3, s.wrapping_add(7));
                c.ldr(A3, s);
                c.sdl(A3, d.wrapping_add(7));
                c.sdr(A3, d);
                c.sw(r32(c, S3).wrapping_add(SNDSTRM_FORMAT_CHANGED), 0);
            }
        }
        if !stopped {
            c.cycles += 6;
            c.g[V0][0] = c.lbu(r32(c, S3).wrapping_add(SNDSTRM_STATE));
            c.g[S0][0] = 1;
            c.g[V0][0] = sx(((r32(c, V0) << 24) as i32 >> 24) as u32);
            let playing = c.g[V0][0] == c.g[S0][0];
            c.g[V0][0] = 0;
            if !playing {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                SNDSTRM_START_PLAYBACK.call(c, 0x003b_6e70)?;
                c.sb(r32(c, S3).wrapping_add(SNDSTRM_STATE), c.g[S0][0]);
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xe0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_TrackAdvance (0x3d3d20)

/// Path state: the queued-event write pointer (restored on a failed submit).
pub(crate) const PATH_EVENT_WRITE: u32 = 0x91c;
/// Voice fields: velocity byte, "chord" flag byte, current node (short);
/// the voice struct is 0x110 bytes.
pub(crate) const PATH_VOICE_VEL: u32 = 0x1;
pub(crate) const PATH_VOICE_CHORD: u32 = 0x2f;
pub(crate) const PATH_VOICE_CUR_NODE: u32 = 0x36;

const PTA_SELECT_BRANCH: Callee = Callee {
    addr: 0x003d_37a0,
    f: ee::s14::PATH_SelectBranch,
};
const PTA_TRACK_QUEUE_NODE: Callee = Callee {
    addr: 0x003d_3900,
    f: ee::s14::PATH_TrackQueueNode,
};
const PTA_POST_NODE_EVENTS: Callee = Callee {
    addr: 0x003d_4050,
    f: ee::s14::PATH_PostNodeEvents,
};
const PTA_TRACK_SUBMIT_QUEUE: Callee = Callee {
    addr: 0x003d_41a8,
    f: ee::s14::PATH_TrackSubmitQueue,
};

/// `void PATH_TrackAdvance(int voice)` (0x3d3d20): advances a voice to its
/// next node. Saves the voice (0x110 bytes on the stack) and the event
/// write pointer; repeats the current node while its repeat count allows
/// (`wC & 2`, count <= rows * columns of w4), else selects the branch for
/// the voice's velocity and queues the path to the next playable node. A
/// node owned by another voice only posts its events (the voice stops:
/// node -1); otherwise the voice moves there and submits its queue, and a
/// failed submit (negative, not -9999) restores the saved voice and
/// pointer.
pub fn path_track_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x160);
        c.g[V1][0] = 0x450000;
        frame.save(c, S2, 0x130);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = c.lw(PATH_STATE_PTR);
        c.g[S2][0] = c.g[V1][0];
        frame.save(c, S0, 0x150);
        c.g[T0][0] = u64::MAX; // sel = -1
        frame.save(c, S1, 0x140);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x110);
        frame.save(c, S3, 0x120);
        c.g[S3][0] = c.lw(r32(c, V0).wrapping_add(PATH_EVENT_WRITE)); // old
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(PATH_VOICES)); // v
        c.g[V0][0] = c.g[S0][0] & 7;
        let unaligned = c.g[V0][0] != 0;
        c.g[V1][0] = c.g[S0][0];
        // save = *v
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x100));
        oc374_block_copy(
            c,
            V1,
            A1,
            V0,
            unaligned,
            [A0, A2, A3, T1],
            0x003d_3d6c,
            [A0, A2, A3, T1],
            0x003d_3dc4,
        );
        c.cycles += 25;
        let (src, dst) = (r32(c, V1), r32(c, A1));
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.ldl(A0, src.wrapping_add(0xf));
        c.ldr(A0, src.wrapping_add(8));
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        c.sdl(A0, dst.wrapping_add(0xf));
        c.g[S1][0] = u64::MAX; // ch = -1
        c.sdr(A0, dst.wrapping_add(8));

        // k = v->h36; e = node k; n = v->ch; ch = v->b2F ? 1 : -1
        let voice = r32(c, S0);
        c.g[A1][0] = c.lw(PATH_STATE_PTR);
        c.g[A3][0] = 1;
        c.g[T1][0] = c.lh(voice.wrapping_add(PATH_VOICE_CUR_NODE));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODE_INDEX));
        c.g[V0][0] = sx(r32(c, T1) << 1);
        c.g[A0][0] = c.lw(voice.wrapping_add(PATH_VOICE_FLAGS));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODES));
        c.g[V1][0] = c.lh(r32(c, V0));
        c.g[A0][0] = sx(r32(c, A0) << 23);
        c.g[V0][0] = c.lbu(voice.wrapping_add(PATH_VOICE_CHORD));
        c.g[A0][0] = sx((r32(c, A0) as i32 >> 27) as u32); // n
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
        let no_count = (c.g[A0][0] as i64) <= 0;
        if c.g[V0][0] != 0 {
            c.g[S1][0] = c.g[A3][0];
        }
        // The current node repeats (sel = k, ch = n).
        let mut repeat = false;
        if no_count {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0xc)) & 2;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(4));
                c.g[V1][0] = sx(r32(c, V0) >> 20);
                c.g[V0][0] = sx(r32(c, V0) >> 24);
                c.g[V1][0] &= 0xf;
                c.g[V0][0] &= 0xf;
                c.mult(0, V0, V0, V1);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[T0][0] = c.g[T1][0];
                    c.g[S1][0] = c.g[A0][0];
                    repeat = true;
                }
            }
        }
        if !repeat {
            c.g[A0][0] = c.lh(r32(c, S0).wrapping_add(PATH_VOICE_CUR_NODE));
            c.cycles += 2;
            if (c.g[A0][0] as i64) >= 0 {
                c.cycles += 3;
                c.g[A1][0] = c.lbu(r32(c, S0).wrapping_add(PATH_VOICE_VEL));
                PTA_SELECT_BRANCH.call(c, 0x003d_3ea8)?;
                c.g[T0][0] = c.g[V0][0];
            }
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[T0][0];
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lh(r32(c, S0).wrapping_add(PATH_VOICE_CUR_NODE));
            c.g[A2][0] = c.lbu(r32(c, S0).wrapping_add(PATH_VOICE_VEL));
            PTA_TRACK_QUEUE_NODE.call(c, 0x003d_3ec0)?;
            c.g[T0][0] = c.g[V0][0];
        }

        // w = voices[node sel ->h2 & 0x1f]
        c.cycles += 15;
        c.g[A0][0] = c.lw(PATH_STATE_PTR);
        c.g[V1][0] = sx(r32(c, T0) << 1);
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODE_INDEX));
        c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODES));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lh(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lhu(r32(c, A2).wrapping_add(2)) & 0x1f;
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(PATH_VOICES));
        let voice = r32(c, S0);
        if c.g[A2][0] != 0 {
            c.cycles += 2;
            let own = c.g[A2][0] == c.g[S0][0];
            c.g[A0][0] = 0xffff_ffff_ffff_fe0f;
            if !own {
                // Another voice's node: post its events, stop this voice.
                c.cycles += 12;
                c.g[V1][0] = c.lw(voice.wrapping_add(PATH_VOICE_FLAGS));
                c.g[V0][0] = c.g[S1][0] & 0x1f;
                c.g[A1][0] = u64::MAX;
                c.g[V1][0] &= c.g[A0][0];
                c.g[V0][0] = sx(r32(c, V0) << 4);
                c.g[V1][0] |= c.g[V0][0];
                c.sh(voice.wrapping_add(PATH_VOICE_CUR_NODE), c.g[A1][0]);
                c.sw(voice.wrapping_add(PATH_VOICE_FLAGS), c.g[V1][0]);
                c.g[A0][0] = c.g[T0][0];
                PTA_POST_NODE_EVENTS.call(c, 0x003d_3f30)?;
                frame.restore(c, S0, 0x150);
                return pta_epilogue(c, frame);
            }
            c.cycles += 1;
        }
        c.g[V0][0] = c.lw(voice.wrapping_add(PATH_VOICE_FLAGS));
        // v->h36 = sel; v->ch = ch; r = SubmitQueue(v)
        c.cycles += 12;
        c.g[A0][0] = 0xffff_ffff_ffff_fe0f;
        c.g[V1][0] = c.g[S1][0] & 0x1f;
        c.sh(voice.wrapping_add(PATH_VOICE_CUR_NODE), c.g[T0][0]);
        c.g[V1][0] = sx(r32(c, V1) << 4);
        c.g[V0][0] &= c.g[A0][0];
        c.g[V0][0] |= c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        c.sw(voice.wrapping_add(PATH_VOICE_FLAGS), c.g[V0][0]);
        PTA_TRACK_SUBMIT_QUEUE.call(c, 0x003d_3f60)?;
        c.g[V1][0] = c.g[V0][0];
        if (c.g[V1][0] as i64) >= 0 {
            frame.restore(c, S0, 0x150);
            return pta_epilogue(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = (-9999i64) as u64;
        let busy = c.g[V1][0] == c.g[V0][0];
        c.g[V1][0] = c.lw(PATH_STATE_PTR);
        if !busy {
            // Failed: restore the event write pointer and the voice.
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] = c.g[A0][0] & 7;
            let unaligned = c.g[V0][0] != 0;
            c.sw(r32(c, V1).wrapping_add(PATH_EVENT_WRITE), c.g[S3][0]);
            c.cycles += 1;
            c.g[V0][0] = sx(sp(c).wrapping_add(0x100));
            oc374_block_copy(
                c,
                A1,
                A0,
                V0,
                unaligned,
                [T0, T1, V1, A2],
                0x003d_3f90,
                [A3, T0, T1, V1],
                0x003d_3fe8,
            );
            c.cycles += 8;
            let (src, dst) = (r32(c, A1), r32(c, A0));
            c.ldl(A2, src.wrapping_add(7));
            c.ldr(A2, src);
            c.ldl(A3, src.wrapping_add(0xf));
            c.ldr(A3, src.wrapping_add(8));
            c.sdl(A2, dst.wrapping_add(7));
            c.sdr(A2, dst);
            c.sdl(A3, dst.wrapping_add(0xf));
            c.sdr(A3, dst.wrapping_add(8));
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x150);
        pta_epilogue(c, frame)
    })
}

/// `PATH_TrackAdvance`'s epilogue (s0 already reloaded).
fn pta_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    frame.restore(c, S1, 0x140);
    frame.restore(c, S2, 0x130);
    frame.restore(c, S3, 0x120);
    frame.restore_ra(c, 0x110);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// __ieee754_log (0x40db58, fdlibm e_log.c on the soft-float double ABI)

const IEEE_LOG_ADDDF3: Callee = Callee {
    addr: 0x0041_1f18,
    f: ee::s15::__adddf3,
};
const IEEE_LOG_SUBDF3: Callee = Callee {
    addr: 0x0041_1f70,
    f: ee::s15::__subdf3,
};
const IEEE_LOG_MULDF3: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ee::s15::__muldf3,
};
const IEEE_LOG_DIVDF3: Callee = Callee {
    addr: 0x0041_2280,
    f: ee::s15::__divdf3,
};
const IEEE_LOG_CMPDF2: Callee = Callee {
    addr: 0x0041_2500,
    f: ee::s15::__cmpdf2,
};
const IEEE_LOG_FLOATSIDF: Callee = Callee {
    addr: 0x0041_2550,
    f: ee::s15::__floatsidf,
};

/// The constant pool of e_log.c (doubles at `0x4a0000 - off`): -two54/zero,
/// ln2_hi, ln2_lo, 1/3, ln2_hi, ln2_lo, Lg7, Lg5, Lg3, Lg1, Lg6, Lg4, Lg2,
/// then ln2_hi/ln2_lo for the two `k != 0` tails.
const IEEE_LOG_POOL: u32 = 0x004a_0000;

/// `a1 = pool constant` (`lui at, 0x4a; ld a1, -off(at)`).
fn ieee_log_k(c: &mut Cpu, off: u32) {
    c.g[AT][0] = IEEE_LOG_POOL as u64;
    c.g[A1][0] = c.ld(IEEE_LOG_POOL - off);
}

/// Where a path of `__ieee754_log` joins the shared tail.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum IeeeLogTail {
    /// `a1 = v0`, then `a0 - a1`.
    SubResult,
    /// `v0 = a0 - a1`.
    Sub,
    /// Return v0 (reload ra).
    Ret,
    /// Return v0 (ra already reloaded).
    Epilogue,
}

/// `double __ieee754_log(double x)` (0x40db58; x and the result as 64-bit
/// integers in a0 / v0): fdlibm's natural logarithm on the soft-float
/// helpers. x = 2^k * (1 + f) with sqrt(2)/2 < 1 + f < sqrt(2); small f
/// take the short series, the rest `s = f / (2 + f)` with the Lg1..Lg7
/// polynomial. log(+-0) = -inf, log(x < 0) = NaN, log(inf/NaN) = x + x.
pub fn ieee754_log(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        c.g[A2][0] = c.g[A0][0];
        let stk = sp(c);
        c.sd(stk.wrapping_add(0x90), c.g[RA][0]);
        c.sd(stk.wrapping_add(0x80), c.g[FP][0]);
        c.sd(stk.wrapping_add(0x70), c.g[S7][0]);
        c.sd(stk.wrapping_add(0x60), c.g[S6][0]);
        c.sd(stk.wrapping_add(0x50), c.g[S5][0]);
        c.sd(stk.wrapping_add(0x40), c.g[S4][0]);
        c.sd(stk.wrapping_add(0x30), c.g[S3][0]);
        c.sd(stk.wrapping_add(0x20), c.g[S2][0]);
        c.sd(stk.wrapping_add(0x10), c.g[S1][0]);
        c.sd(stk, c.g[S0][0]);
        c.g[V0][0] = c.g[A2][0];
        c.g[S3][0] = ((c.g[V0][0] as i64) >> 32) as u64; // hx
        c.g[V1][0] = ((c.g[V0][0] << 32) as i64 >> 32) as u64; // lx
        c.g[V0][0] = 0xfffff;
        let normal = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
        c.g[V0][0] = normal as u64;
        c.g[FP][0] = 0; // k

        let tail = 'body: {
            if !normal {
                // hx < 0x00100000: zero, negative or subnormal.
                c.cycles += 6;
                c.g[V0][0] = 0x7fff_ffff;
                c.g[V0][0] &= c.g[S3][0];
                c.g[V0][0] |= c.g[V1][0];
                let zero = c.g[V0][0] == 0;
                c.g[V1][0] = 0x4a0000;
                if zero {
                    // -two54 / zero = -inf
                    c.cycles += 2;
                    c.g[V0][0] = c.ld(IEEE_LOG_POOL - 0x6ef0);
                    break 'body IeeeLogTail::Ret;
                }
                c.cycles += 2;
                let negative = (c.g[S3][0] as i64) < 0;
                c.g[A0][0] = c.g[A2][0];
                if negative {
                    // (x - x) / zero = NaN
                    c.cycles += 7;
                    c.g[A1][0] = c.g[A0][0];
                    IEEE_LOG_SUBDF3.call(c, 0x0040_dbe0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    IEEE_LOG_DIVDF3.call(c, 0x0040_dbec)?;
                    frame.restore_ra(c, 0x90);
                    break 'body IeeeLogTail::Epilogue;
                }
                // Subnormal: scale up by two54.
                c.cycles += 7;
                c.g[A1][0] = 0x4350_0000_0000_0000;
                c.g[FP][0] = (-54i64) as u64;
                IEEE_LOG_MULDF3.call(c, 0x0040_dc04)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[S3][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            }

            c.cycles += 5;
            c.g[V0][0] = 0x7fef_ffff;
            let inf_nan = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
            c.g[V0][0] = inf_nan as u64;
            c.g[S0][0] = 0xf0000;
            if inf_nan {
                // x + x
                c.cycles += 5;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = c.g[A0][0];
                IEEE_LOG_ADDDF3.call(c, 0x0040_dc30)?;
                frame.restore_ra(c, 0x90);
                break 'body IeeeLogTail::Epilogue;
            }

            // k += (hx >> 20) - 1023; normalise x (or x/2) into [sqrt(2)/2, sqrt(2)).
            c.cycles += 31;
            c.g[A0][0] = sx((r32(c, S3) as i32 >> 20) as u32);
            c.g[S0][0] |= 0xffff;
            c.g[V0][0] = 0x95f64;
            c.g[S3][0] &= c.g[S0][0];
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, FP).wrapping_sub(0x3ff));
            c.g[A1][0] = 0x100000;
            c.g[FP][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[S4][0] = c.g[V0][0] & c.g[A1][0]; // i
            c.g[A0][0] = c.g[A2][0];
            c.g[V0][0] = 0x3ff00000;
            c.g[V1][0] = 0xffff_ffff;
            c.g[V0][0] = c.g[S4][0] ^ c.g[V0][0];
            c.g[A0][0] &= c.g[V1][0];
            c.g[V0][0] |= c.g[S3][0];
            c.g[V0][0] <<= 32;
            c.g[A2][0] = c.g[A0][0] | c.g[V0][0];
            c.g[V1][0] = sx((r32(c, S4) as i32 >> 20) as u32);
            c.g[A1][0] = 0x3ff0_0000_0000_0000; // 1.0
            c.g[FP][0] = sx(r32(c, FP).wrapping_add(r32(c, V1)));
            c.g[A0][0] = c.g[A2][0];
            IEEE_LOG_SUBDF3.call(c, 0x0040_dc9c)?;
            c.g[S5][0] = c.g[V0][0]; // f = x - 1
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(2));
            c.g[V0][0] &= c.g[S0][0];
            let tiny = (c.g[V0][0] as i64) < 3;
            c.g[V0][0] = tiny as u64;
            c.g[S0][0] = 0;

            if tiny {
                // |f| < 2^-20
                c.cycles += 5;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S0][0];
                IEEE_LOG_CMPDF2.call(c, 0x0040_dcc0)?;
                let f_nonzero = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S5][0];
                if !f_nonzero {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    if c.g[FP][0] == 0 {
                        break 'body IeeeLogTail::Ret; // log(1) = 0
                    }
                    // dk * ln2_hi + dk * ln2_lo
                    c.cycles += 17;
                    c.g[A0][0] = c.g[FP][0];
                    IEEE_LOG_FLOATSIDF.call(c, 0x0040_dcd8)?;
                    c.g[S6][0] = c.g[V0][0];
                    ieee_log_k(c, 0x6ee8);
                    c.g[A0][0] = c.g[S6][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_dcec)?;
                    ieee_log_k(c, 0x6ee0);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_dd00)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_ADDDF3.call(c, 0x0040_dd0c)?;
                    frame.restore_ra(c, 0x90);
                    break 'body IeeeLogTail::Epilogue;
                }
                // R = f*f*(0.5 - 1/3*f)
                c.cycles += 16;
                c.g[A1][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd20)?;
                ieee_log_k(c, 0x6ed8);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd34)?;
                c.g[A0][0] = 0x3fe0_0000_0000_0000; // 0.5
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_dd44)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd50)?;
                c.g[S2][0] = c.g[V0][0];
                if c.g[FP][0] == 0 {
                    // f - R
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S2][0];
                    break 'body IeeeLogTail::Sub;
                }
                // dk*ln2_hi - ((R - dk*ln2_lo) - f)
                c.cycles += 20;
                c.g[A0][0] = c.g[FP][0];
                IEEE_LOG_FLOATSIDF.call(c, 0x0040_dd6c)?;
                c.g[S6][0] = c.g[V0][0];
                ieee_log_k(c, 0x6ed0);
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd80)?;
                ieee_log_k(c, 0x6ec8);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dd94)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_dda0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_ddac)?;
                c.g[A0][0] = c.g[S0][0];
                break 'body IeeeLogTail::SubResult;
            }

            // s = f / (2 + f); z = s*s; w = z*z; R = t2 + t1 (Lg polynomial)
            c.cycles += 75;
            c.g[A1][0] = 0x4000_0000_0000_0000; // 2.0
            c.g[A0][0] = c.g[S5][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_ddc4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S5][0];
            IEEE_LOG_DIVDF3.call(c, 0x0040_ddd0)?;
            c.g[S7][0] = c.g[V0][0]; // s
            c.g[A0][0] = c.g[FP][0];
            IEEE_LOG_FLOATSIDF.call(c, 0x0040_dddc)?;
            c.g[S6][0] = c.g[V0][0]; // dk
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = c.g[S7][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_ddec)?;
            c.g[V1][0] = 0xffff_ffff_fff9_eb86;
            c.g[S2][0] = c.g[V0][0]; // z
            c.g[A0][0] = c.g[S2][0];
            c.g[S4][0] = sx(r32(c, S3).wrapping_add(r32(c, V1))); // i = hx - 0x6147a
            c.g[A1][0] = c.g[S2][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de08)?;
            c.g[S0][0] = 0x6b851;
            c.g[S1][0] = c.g[V0][0]; // w
            ieee_log_k(c, 0x6ec0);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, S3))); // j
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de28)?;
            c.g[S4][0] |= c.g[S0][0];
            ieee_log_k(c, 0x6eb8);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_de3c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de48)?;
            ieee_log_k(c, 0x6eb0);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_de58)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de64)?;
            ieee_log_k(c, 0x6ea8);
            c.g[S0][0] = c.g[V0][0]; // t1
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de78)?;
            ieee_log_k(c, 0x6ea0);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_de88)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_de94)?;
            ieee_log_k(c, 0x6e98);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_dea4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_deb0)?;
            ieee_log_k(c, 0x6e90);
            c.g[A0][0] = c.g[V0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_dec0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_MULDF3.call(c, 0x0040_decc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            IEEE_LOG_ADDDF3.call(c, 0x0040_ded8)?;
            let far = (c.g[S4][0] as i64) > 0;
            c.g[S2][0] = c.g[V0][0]; // R

            if far {
                // hfsq = 0.5*f*f
                c.cycles += 9;
                c.g[A1][0] = 0x3fe0_0000_0000_0000;
                c.g[A0][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_def0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S5][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_defc)?;
                c.g[S3][0] = c.g[V0][0];
                if c.g[FP][0] == 0 {
                    // f - (hfsq - s*(hfsq + R))
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    IEEE_LOG_ADDDF3.call(c, 0x0040_df10)?;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_df1c)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_SUBDF3.call(c, 0x0040_df28)?;
                    c.g[A0][0] = c.g[S5][0];
                    break 'body IeeeLogTail::SubResult;
                }
                // dk*ln2_hi - ((hfsq - (s*(hfsq + R) + dk*ln2_lo)) - f)
                c.cycles += 21;
                ieee_log_k(c, 0x6e88);
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_df40)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S3][0];
                IEEE_LOG_ADDDF3.call(c, 0x0040_df50)?;
                c.g[A0][0] = c.g[S7][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_df5c)?;
                ieee_log_k(c, 0x6e80);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_df70)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_ADDDF3.call(c, 0x0040_df7c)?;
                c.g[A0][0] = c.g[S3][0];
            } else {
                c.cycles += 2;
                if c.g[FP][0] == 0 {
                    // f - s*(f - R)
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S5][0];
                    IEEE_LOG_SUBDF3.call(c, 0x0040_df98)?;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[V0][0];
                    IEEE_LOG_MULDF3.call(c, 0x0040_dfa4)?;
                    c.g[A0][0] = c.g[S5][0];
                    break 'body IeeeLogTail::SubResult;
                }
                // dk*ln2_hi - ((s*(f - R) - dk*ln2_lo) - f)
                c.cycles += 17;
                ieee_log_k(c, 0x6e78);
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dfbc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S5][0];
                IEEE_LOG_SUBDF3.call(c, 0x0040_dfcc)?;
                c.g[A0][0] = c.g[S7][0];
                c.g[A1][0] = c.g[V0][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dfd8)?;
                ieee_log_k(c, 0x6e70);
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                IEEE_LOG_MULDF3.call(c, 0x0040_dfec)?;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 6;
            c.g[A1][0] = c.g[V0][0];
            IEEE_LOG_SUBDF3.call(c, 0x0040_dff8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            IEEE_LOG_SUBDF3.call(c, 0x0040_e004)?;
            c.g[A0][0] = c.g[S1][0];
            IeeeLogTail::SubResult
        };

        if tail <= IeeeLogTail::SubResult {
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        }
        if tail <= IeeeLogTail::Sub {
            c.cycles += 2;
            IEEE_LOG_SUBDF3.call(c, 0x0040_e014)?;
        }
        if tail <= IeeeLogTail::Ret {
            c.cycles += 1;
            frame.restore_ra(c, 0x90);
        }
        c.cycles += 11;
        let stk = sp(c);
        c.g[FP][0] = c.ld(stk.wrapping_add(0x80));
        c.g[S7][0] = c.ld(stk.wrapping_add(0x70));
        c.g[S6][0] = c.ld(stk.wrapping_add(0x60));
        c.g[S5][0] = c.ld(stk.wrapping_add(0x50));
        c.g[S4][0] = c.ld(stk.wrapping_add(0x40));
        c.g[S3][0] = c.ld(stk.wrapping_add(0x30));
        c.g[S2][0] = c.ld(stk.wrapping_add(0x20));
        c.g[S1][0] = c.ld(stk.wrapping_add(0x10));
        c.g[S0][0] = c.ld(stk);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_PlayBumpAnim (0x108388, rider.cpp)

/// Rider fields: the rider base (states at +0xde0 / +0xde4, board press at
/// +0x1d0), the anim controller, the forward and side axes, the velocity,
/// "goofy" flag and the spin angle.
pub(crate) const RPBA_BASE: u32 = 0x77c;
pub(crate) const RPBA_ANIM: u32 = 0x784;
pub(crate) const RPBA_SIDE: u32 = 0x1a0;
pub(crate) const RPBA_FORWARD: u32 = 0x1b0;
pub(crate) const RPBA_VELOCITY: u32 = 0x1e0;
pub(crate) const RPBA_GOOFY: u32 = 0x320;
pub(crate) const RPBA_SPIN: u32 = 0x2dc;
/// Bump info fields: the bump normal and the impact speed.
pub(crate) const RPBA_INFO_NORMAL: u32 = 0x20;
pub(crate) const RPBA_INFO_SPEED: u32 = 0x30;

const RPBA_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};
const RPBA_GET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fee8,
    f: ee::s00::Rider_GetSubState,
};
const RPBA_SET_SUB_STATE: Callee = Callee {
    addr: 0x0011_fec8,
    f: ee::s00::Rider_SetSubState,
};
const RPBA_BOARD_PRESS_CANCEL: Callee = Callee {
    addr: 0x0013_1348,
    f: ee::s00::RiderBoardPressSub_Cancel,
};
/// `cRiderAnimBase_play(anim, id, 0, -1.0f)`, `AIrand`, the audio system
/// instance and `SSXAudioSystem_SpeechRiderBump` (through the table).
const RPBA_ANIM_PLAY: u32 = 0x0031_28e8;
const RPBA_AI_RAND: u32 = 0x0031_7810;
const RPBA_AUDIO_INSTANCE: u32 = 0x0028_b180;
const RPBA_SPEECH_BUMP: u32 = 0x002a_0e70;

/// `f1 = dot(axis, normal)` the way the port's VU0 macro code does it
/// (vf3 = the axis spilled through the stack, vf5 = the normal).
fn rpba_dot_into_vf4(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `play(anim, id, 0, -1.0f)` with `a0`/`a1` set: `at = -1.0f` into f12.
fn rpba_play(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 12);
    c.g[A2][0] = 0;
    call_ptr(c, RPBA_ANIM_PLAY, ret)
}

/// One `spin += (AIrand() % n [- 2]) * k` step's random part: `a0` gets the
/// remainder, f2 the step (`.lit4` at `gp - lit`), f1 the spin.
fn rpba_rand_step(c: &mut Cpu, ret: u32, n: u64, lit: u32) -> crate::call::Flow {
    c.cycles += 7;
    call_ptr(c, RPBA_AI_RAND, ret)?;
    c.g[V1][0] = n;
    let k = r32(c, GP).wrapping_sub(lit);
    c.lwc1(2, k);
    c.divu(0, V0, V1);
    c.cycles += 3;
    Ok(())
}

/// `void Rider_PlayBumpAnim(Rider* self, BumpInfo* info, int speak)`
/// (0x108388): in main state 0 or 4 and sub state 0, 1 (cancels the board
/// press), 2 or 7, plays the bump animation for the bump normal: front
/// (0x37) or back (0x38) bumps, else a side bump by the side of the hit
/// (flipped for goofy riders): a hard one (0x3a/0x3c, with a random half
/// turn added to a small spin) for slow-ish bumps at speed outside main
/// state 4, a soft one (0x39/0x3b) otherwise. Then the bump speech (if
/// `speak`) and sub state 3.
pub fn rider_play_bump_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0x60);
        RPBA_GET_MAIN_STATE.call(c, 0x0010_83b4)?;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            RPBA_GET_MAIN_STATE.call(c, 0x0010_83c4)?;
            c.g[V1][0] = 4;
            if c.g[V0][0] != c.g[V1][0] {
                frame.restore(c, S0, 0x50);
                return rpba_epilogue(c, frame);
            }
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        RPBA_GET_SUB_STATE.call(c, 0x0010_83d8)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(RPBA_BASE));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d0));
            RPBA_BOARD_PRESS_CANCEL.call(c, 0x0010_83f0)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            RPBA_GET_SUB_STATE.call(c, 0x0010_8400)?;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                RPBA_GET_SUB_STATE.call(c, 0x0010_8410)?;
                c.g[V1][0] = 2;
                if c.g[V0][0] != c.g[V1][0] {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    RPBA_GET_SUB_STATE.call(c, 0x0010_8424)?;
                    c.g[V1][0] = 7;
                    if c.g[V0][0] != c.g[V1][0] {
                        frame.restore(c, S0, 0x50);
                        return rpba_epilogue(c, frame);
                    }
                    c.cycles += 1;
                }
            }
        }
        c.lq(V0, r32(c, S0).wrapping_add(RPBA_FORWARD));

        // d = dot(forward, normal)
        c.cycles += 16;
        c.g[S2][0] = 0; // played
        let gp = r32(c, GP);
        c.lwc1(0, gp.wrapping_sub(0x7e5c)); // 0.70710677
        let stk = sp(c);
        c.sq(V0, stk);
        c.lqc2(3, stk);
        c.lqc2(5, r32(c, S1).wrapping_add(RPBA_INFO_NORMAL));
        rpba_dot_into_vf4(c);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 1);
        c.c_lt_s(0, 1);
        c.g[A1][0] = 0x37;
        let mut front_back = c.fcc();
        if front_back {
            c.cycles += 2;
        } else {
            c.cycles += 5;
            c.lwc1(0, gp.wrapping_sub(0x7e58)); // -0.70710677
            c.c_lt_s(1, 0);
            c.g[A1][0] = 0x38;
            front_back = c.fcc();
            if front_back {
                c.cycles += 1;
            }
        }
        if front_back {
            c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(RPBA_ANIM));
            c.cycles += 6;
            rpba_play(c, 0x0010_84a4)?;
        } else {
            // side = dot(side axis, normal), negated for goofy riders
            c.cycles += 14;
            c.lq(V0, r32(c, S0).wrapping_add(RPBA_SIDE));
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(RPBA_GOOFY));
            let stk = sp(c);
            c.sq(V0, stk);
            c.lqc2(3, stk);
            c.lqc2(5, r32(c, S1).wrapping_add(RPBA_INFO_NORMAL));
            rpba_dot_into_vf4(c);
            c.qmfc2(A0, 4);
            c.mtc1(A0, 20);
            if c.g[V1][0] != 0 {
                c.neg_s(20, 20);
            }
            let hard = 'hard: {
                c.cycles += 5;
                c.lwc1(1, r32(c, S1).wrapping_add(RPBA_INFO_SPEED));
                c.lwc1(0, gp.wrapping_sub(0x7e54)); // 1111.1111
                c.c_lt_s(1, 0);
                if !c.fcc() {
                    break 'hard false;
                }
                // |velocity| > 1388.8889
                c.cycles += 14;
                c.lwc1(0, gp.wrapping_sub(0x7e50));
                c.lqc2(3, r32(c, S0).wrapping_add(RPBA_VELOCITY));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(V0, 22);
                c.mtc1(V0, 1);
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    break 'hard false;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                RPBA_GET_MAIN_STATE.call(c, 0x0010_8538)?;
                c.g[V1][0] = 4;
                c.g[V0][0] != c.g[V1][0]
            };
            if hard {
                // A small spin (|a| < 2 pi) gets a random number of half turns.
                c.cycles += 6;
                let spin = r32(c, S0).wrapping_add(RPBA_SPIN);
                c.lwc1(2, spin);
                c.lwc1(1, gp.wrapping_sub(0x7e4c)); // 2 pi
                c.abs_s(0, 2);
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.cycles += 4;
                    c.mtc1(0, 0);
                    c.c_lt_s(2, 0);
                    if c.fcc() {
                        rpba_rand_step(c, 0x0010_8574, 3, 0x7e48)?; // -pi
                        c.g[A0][0] = c.hi[0];
                        c.lwc1(1, r32(c, S0).wrapping_add(RPBA_SPIN));
                    } else {
                        c.cycles += 3;
                        c.c_lt_s(0, 2);
                        if c.fcc() {
                            rpba_rand_step(c, 0x0010_85a8, 3, 0x7e44)?; // pi
                            c.g[A0][0] = c.hi[0];
                            c.lwc1(1, r32(c, S0).wrapping_add(RPBA_SPIN));
                        } else {
                            rpba_rand_step(c, 0x0010_85d0, 5, 0x7e40)?; // pi
                            c.lwc1(1, r32(c, S0).wrapping_add(RPBA_SPIN));
                            c.g[A0][0] = c.hi[0];
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(2));
                        }
                    }
                    c.cycles += 5;
                    c.mtc1(A0, 0);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 2);
                    c.add_s(1, 1, 0);
                    c.swc1(1, r32(c, S0).wrapping_add(RPBA_SPIN));
                }
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(20, 0);
                c.g[A1][0] = 0x3c;
                if c.fcc() {
                    c.cycles += 1;
                    c.g[A1][0] = 0x3a;
                }
                c.cycles += 8;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(RPBA_ANIM));
                c.g[A2][0] = 0;
                c.g[S2][0] = 1; // played
                rpba_play(c, 0x0010_8634)?;
            } else {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(20, 0);
                c.g[A1][0] = 0x3b;
                if c.fcc() {
                    c.cycles += 1;
                    c.g[A1][0] = 0x39;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(RPBA_ANIM));
                rpba_play(c, 0x0010_8668)?;
            }
        }

        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        if c.g[S3][0] != 0 {
            c.cycles += 7;
            call_ptr(c, RPBA_AUDIO_INSTANCE, 0x0010_8678)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, RPBA_SPEECH_BUMP, 0x0010_8688)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 3;
        c.g[A1][0] = 3;
        RPBA_SET_SUB_STATE.call(c, 0x0010_8694)?;
        frame.restore(c, S0, 0x50);
        rpba_epilogue(c, frame)
    })
}

/// `Rider_PlayBumpAnim`'s epilogue (s0 already reloaded).
fn rpba_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore_ra(c, 0x10);
    frame.restore_fpr(c, 20, 0x60);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// WorldObjFactory_ParticleNode (0x2fd420, worldobjfactory.cpp)

/// The particle-node defaults: the guard word (`gp + 0x2514` = 0x4a5604)
/// and the 0xe0-byte template (0x500000 - 0x49c0).
pub(crate) const WOFPN_INIT_GUARD: u32 = 0x2514;
pub(crate) const WOFPN_TEMPLATE: u32 = 0x004f_b640;
/// Parameter types by index (2: float).
pub(crate) const WOFPN_PARAM_TYPES: u32 = 0x0044_60b0;
/// The world object table pointer (`gp + 0x16c8`) and the default world
/// object holder (`gp + 0xce8`, object at +0x290).
pub(crate) const WOFPN_OBJECT_TABLE: u32 = 0x16c8;
pub(crate) const WOFPN_DEFAULT_HOLDER: u32 = 0xce8;
/// The factory's 12-byte "done" result (0x4d0000 - 0x6f68).
pub(crate) const WOFPN_RESULT: u32 = 0x004c_9098;
/// Node fields: the object id word (+0xd8, -1: the default object), the
/// lifetime (+0x1c) clamped to the max (+0x14).
pub(crate) const WOFPN_NODE_OBJECT: u32 = 0xd8;

const WOFPN_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};
const WOFPN_DELETE: Callee = Callee {
    addr: 0x0031_7e50,
    f: ee::s10::__builtin_delete,
};
const WOFPN_ATTACH_MODIFIER: Callee = Callee {
    addr: 0x002f_ae38,
    f: ee::s10::WorldObjFactory_AttachModifier,
};
/// `memcpy` and the particle-node ctor (through the table).
const WOFPN_MEMCPY: u32 = 0x003e_6574;
const WOFPN_NODE_CTOR: u32 = 0x0035_78a8;

/// The guarded static init of the particle-node template.
fn wofpn_init_template(c: &mut Cpu) {
    c.cycles += 72;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 1); // -1.0
    c.g[V0][0] = sx(WOFPN_TEMPLATE);
    c.g[AT][0] = 0x4080_0000;
    c.mtc1(AT, 2); // 4.0
    c.g[A0][0] = 1;
    c.mtc1(0, 0); // 0.0
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 3); // 1.0
    c.g[A2][0] = 0x10;
    let t = WOFPN_TEMPLATE;
    c.swc1(1, t + 0x8);
    c.g[V1][0] |= 0xffff;
    c.swc1(2, t + 0x10);
    c.sw(t, c.g[A0][0]);
    c.sw(t + 0x4, 0);
    c.swc1(3, t + 0xc);
    for off in (0x14..=0x80).step_by(4) {
        c.swc1(0, t + off);
    }
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 1); // 20.0
    c.swc1(3, t + 0x84);
    c.g[A1][0] = c.lbu(t + 0xdc);
    c.swc1(3, t + 0xb4);
    c.sw(t + 0xc4, c.g[A2][0]);
    c.swc1(0, t + 0xcc);
    c.swc1(1, t + 0xd4);
    c.sw(t + 0xd8, c.g[V1][0]);
    let guard = r32(c, GP).wrapping_add(WOFPN_INIT_GUARD);
    c.sw(guard, c.g[A0][0]);
    for off in [0x88, 0x8c, 0x90] {
        c.swc1(0, t + off);
    }
    for off in [0x94, 0x98, 0x9c, 0xa0, 0xa4] {
        c.swc1(3, t + off);
    }
    for off in [0xa8, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
        c.swc1(0, t + off);
    }
    c.sw(t + 0xc8, 0);
    c.sw(t + 0xd0, c.g[A0][0]);
    c.sb(sp(c), c.g[A1][0]);
}

/// `Result WorldObjFactory_ParticleNode(int n, Param* params)` (0x2fd420;
/// the 12-byte result through the hidden pointer in a0): allocates a
/// particle node from the default template, applies the `n` params
/// (16 bytes each: index, value, -, type; an int given for a float slot
/// is converted), looks up its world object (by id, or the default one),
/// and unless the object's modifier refuses it (AttachModifier: > 0 frees
/// the node, < 0 keeps it) clamps the lifetime and creates the particle
/// node on the object. A missing object frees the node.
pub fn world_obj_factory_particle_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let guard = r32(c, GP).wrapping_add(WOFPN_INIT_GUARD);
        c.g[V0][0] = c.lw(guard);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = 0x500000;
        frame.save(c, S4, 0x30);
        c.g[S2][0] = c.g[A0][0]; // result
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A1][0]; // n
        frame.save(c, S0, 0x70);
        c.g[S5][0] = c.g[A2][0]; // params
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x10);
        if c.g[V0][0] == 0 {
            wofpn_init_template(c);
        }

        // node = alloc(0xe0); node->object = -1; memcpy(node, template, 0xe0)
        c.cycles += 17;
        c.g[A0][0] = 0xe0;
        c.g[A1][0] = sx(0x0049_0000 - 0x68e8);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S3][0] = 0x4d0000;
        WOFPN_ALLOC.call(c, 0x002f_d594)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = 0xe0;
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = sx(WOFPN_TEMPLATE);
        c.g[A0][0] = c.g[S0][0];
        c.sw(r32(c, S0).wrapping_add(WOFPN_NODE_OBJECT), c.g[V0][0]);
        call_ptr(c, WOFPN_MEMCPY, 0x002f_d5b4)?;
        c.g[A2][0] = c.g[S5][0];
        if (c.g[S4][0] as i64) > 0 {
            c.cycles += 5;
            c.g[V0][0] = 0x440000;
            c.g[A1][0] = c.g[S4][0];
            c.g[T0][0] = sx(WOFPN_PARAM_TYPES);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                let param = r32(c, A2);
                c.g[V0][0] = c.lw(param);
                c.g[V1][0] = c.lw(param.wrapping_add(0xc));
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                c.g[V0][0] = c.lw(r32(c, V0));
                let convert = c.g[V1][0] != c.g[V0][0] && {
                    c.cycles += 2;
                    c.g[V0][0] == c.g[A3][0]
                };
                if convert {
                    c.cycles += 4;
                    c.lwc1(0, param.wrapping_add(4));
                    c.cvt_s_w(0, 0);
                    c.swc1(0, r32(c, A0));
                } else {
                    c.g[V0][0] = c.lw(param.wrapping_add(4));
                    c.cycles += 1;
                    c.sw(r32(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(param.wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_d5d0);
                }
            }
        }

        // obj = id == -1 ? default object : table[id & 0xff] entry id >> 8
        c.cycles += 4;
        c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(WOFPN_NODE_OBJECT));
        c.g[V0][0] = !c.g[A2][0];
        let gp = r32(c, GP);
        c.g[A1][0] = c.lw(gp.wrapping_add(WOFPN_OBJECT_TABLE));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(gp.wrapping_add(WOFPN_DEFAULT_HOLDER));
            c.g[S1][0] = c.lw(r32(c, V0).wrapping_add(0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(8));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, A2) >> 8);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                let some = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                if !some {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A2][0] = 0;
            }
            c.cycles += 1;
            c.g[S1][0] = c.g[A2][0];
        }

        'done: {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                WOFPN_DELETE.call(c, 0x002f_d694)?;
                break 'done;
            }
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0xc));
            c.cycles += 2;
            c.g[A1][0] = 0xd;
            if c.g[A0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                WOFPN_ATTACH_MODIFIER.call(c, 0x002f_d6ac)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) > 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        WOFPN_DELETE.call(c, 0x002f_d6c4)?;
                    }
                    break 'done;
                }
            } else {
                c.cycles += 1;
            }
            // lifetime = min(lifetime, max)
            let node = r32(c, S0);
            c.lwc1(0, node.wrapping_add(0x1c));
            c.cycles += 5;
            c.lwc1(1, node.wrapping_add(0x14));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.swc1(1, node.wrapping_add(0x1c));
            }
            c.cycles += 12;
            c.g[A0][0] = 0x2c;
            c.g[A1][0] = sx(0x0049_0000 - 0x68d0);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            WOFPN_ALLOC.call(c, 0x002f_d6fc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 0;
            call_ptr(c, WOFPN_NODE_CTOR, 0x002f_d714)?;
        }

        // *result = factory "done" result
        c.cycles += 17;
        c.g[A1][0] = sx(WOFPN_RESULT); // s3 (0x4d0000) - 0x6f68
        let (src, dst) = (r32(c, A1), r32(c, S2));
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.g[V1][0] = c.lw(src.wrapping_add(8));
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        c.sw(dst.wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_SpeechRiderCollision (0x2a0a30, soundsys.cpp)

/// Rider fields: the "speech pitch" float (< 0: may speak), the rider
/// object (vtable at +0x6c0: slot 0x28 the character id, slot 0x48 "is
/// AI"), the "human" word.
pub(crate) const SSRC_RIDER_PITCH: u32 = 0x470;
pub(crate) const SSRC_RIDER_OBJECT: u32 = 0x6c0;
pub(crate) const SSRC_RIDER_HUMAN: u32 = 0x87c;
/// Globals: the game-mode holder (`gp - 0x84c`, mode at +0x550), the
/// speech distance (`gp + 0x5e8`), the speech announce hook (`gp + 0x173c`).
pub(crate) const SSRC_MODE_HOLDER: u32 = 0x84c;
pub(crate) const SSRC_SPEECH_DIST: u32 = 0x5e8;
pub(crate) const SSRC_ANNOUNCE: u32 = 0x173c;
/// The speech system inside the audio system.
pub(crate) const SSRC_SPEECH: u32 = 0x5560;

const SSRC_IS_SPEECH_VOLUME_ON: Callee = Callee {
    addr: 0x0029_f0a0,
    f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
};
const SSRC_PLAY_RIDER_GRUNT: Callee = Callee {
    addr: 0x0029_f660,
    f: ee::s08::SSXAudioSystem_PlayRiderGrunt,
};
const SSRC_IS_INSTANCE_READY: Callee = Callee {
    addr: 0x002b_0e28,
    f: ee::s08::SpeechSystem_IsInstanceReady,
};
const SSRC_GET_RIDER_SPEECH_MASK: Callee = Callee {
    addr: 0x0029_f330,
    f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
};
const SSRC_GET_RIDER_RIVALRY: Callee = Callee {
    addr: 0x002a_1820,
    f: ee::s08::SSXAudioSystem_GetRiderRivalry,
};
const SSRC_SET_EVENT_CONTEXT: Callee = Callee {
    addr: 0x002b_1458,
    f: ee::s08::SpeechSystem_SetEventContext,
};
/// Through the table: "speech blocked", "rider finished", the player
/// rider, the speech-in-range check, the rider's speech name, the speech
/// event lookup.
const SSRC_SPEECH_BLOCKED: u32 = 0x0028_8ae0;
const SSRC_RIDER_FINISHED: u32 = 0x0012_3168;
const SSRC_PLAYER_RIDER: u32 = 0x0028_5d98;
const SSRC_IN_RANGE: u32 = 0x0028_8b40;
const SSRC_RIDER_NAME: u32 = 0x0012_3128;
const SSRC_FIND_EVENT: u32 = 0x003d_8008;

/// `rider->object->vfunc(slot)` (gcc 2.9 vtable at the rider's +0x6c0;
/// `obj` the register holding the object address) returning to `ret`.
fn ssrc_rider_vcall(c: &mut Cpu, obj: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let vt = r32(c, V1);
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V0][0] = c.lw(vt.wrapping_add(slot + 4));
    let f = r32(c, V0);
    c.g[A0][0] = sx(r32(c, obj).wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret)
}

/// `SpeechSystem_SetEventContext(speech, 0, event, rider, name(rider),
/// 0.0f, 0, 0)` for the rider in register `rider`; a0 = 1 after; true if
/// the event got a context.
fn ssrc_set_context(
    c: &mut Cpu,
    rider: usize,
    event: u64,
    ret_name: u32,
    ret_set: u32,
) -> Result<bool, crate::call::Unwound> {
    c.g[A0][0] = c.g[rider][0];
    call_ptr(c, SSRC_RIDER_NAME, ret_name)?;
    c.g[A3][0] = c.g[rider][0];
    c.g[T0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = event;
    c.mov_s(12, 20);
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    SSRC_SET_EVENT_CONTEXT.call(c, ret_set)?;
    c.g[A0][0] = 1;
    Ok(c.g[V0][0] != 0)
}

/// `announce(FindEvent(1, 0, event), 2, mask, 3)` (a0 = 1 already).
fn ssrc_announce(
    c: &mut Cpu,
    event: u64,
    mask: usize,
    ret_find: u32,
    ret_announce: u32,
) -> crate::call::Flow {
    c.g[A1][0] = 0;
    c.g[A2][0] = event;
    call_ptr(c, SSRC_FIND_EVENT, ret_find)?;
    c.g[V1][0] = c.lw(r32(c, GP).wrapping_add(SSRC_ANNOUNCE));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = c.g[mask][0];
    c.g[A1][0] = 2;
    c.g[A3][0] = 3;
    let f = r32(c, V1);
    call_ptr(c, f, ret_announce)
}

/// `void SSXAudioSystem_SpeechRiderCollision(SSXAudioSystem* self, Rider* a,
/// Rider* b, int p7, int p8)` (0x2a0a30): when speech is on in race mode
/// 2 and allowed, neither rider has finished and both may speak, and the
/// player is involved unless both are humans/AI-checked: plays b's grunt
/// and, with the speech instance ready and one of them in range (and no
/// p7 / p8), the rivalry-dependent "hit" (b) and "got hit" (a) lines.
pub fn ssx_audio_system_speech_rider_collision(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x60);
        frame.save(c, S1, 0x80);
        c.g[S3][0] = c.g[A0][0]; // self
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0]; // b
        frame.save(c, S7, 0x20);
        c.g[S2][0] = c.g[A1][0]; // a
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[T0][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = c.g[A3][0];
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0xa0);
        SSRC_IS_SPEECH_VOLUME_ON.call(c, 0x002a_0a78)?;
        let off = c.g[V0][0] == 0;
        let gp = r32(c, GP);
        c.g[V1][0] = c.lw(gp.wrapping_sub(SSRC_MODE_HOLDER));
        // true: ends through 0x2a0e3c (one more cycle, reload s0).
        let via_e3c = 'body: {
            if off {
                break 'body true;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x550)) ^ 2;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] != 0 {
                break 'body false;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, SSRC_SPEECH_BLOCKED, 0x002a_0a98)?;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] != 0 {
                break 'body false;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, SSRC_RIDER_FINISHED, 0x002a_0aa8)?;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] != 0 {
                break 'body false;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, SSRC_RIDER_FINISHED, 0x002a_0ab8)?;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] != 0 {
                break 'body false;
            }
            // a->pitch >= 0 or b->pitch >= 0: no speech
            c.cycles += 6;
            c.lwc1(0, r32(c, S2).wrapping_add(SSRC_RIDER_PITCH));
            c.mtc1(0, 1);
            c.c_le_s(1, 0);
            c.g[V0][0] = 1;
            if !c.fcc() {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] != 0 {
                break 'body false;
            }
            c.cycles += 5;
            c.lwc1(0, r32(c, S1).wrapping_add(SSRC_RIDER_PITCH));
            c.c_le_s(1, 0);
            c.g[V0][0] = 1;
            if !c.fcc() {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            if c.g[V0][0] != 0 {
                break 'body false;
            }

            // chkA (s4) / chkB (s0): AI rider or not human
            c.cycles += 10;
            c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(SSRC_RIDER_OBJECT));
            c.g[A1][0] = sx(r32(c, S2).wrapping_add(SSRC_RIDER_OBJECT));
            c.g[S6][0] = c.g[A1][0];
            c.g[S4][0] = 0;
            ssrc_rider_vcall(c, A1, 0x48, 0x002a_0b24)?;
            let mut extra = true;
            if c.g[V0][0] != 0 {
                c.g[S4][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(SSRC_RIDER_HUMAN));
                if c.g[V0][0] != 0 {
                    extra = false;
                } else {
                    c.cycles += 1;
                    c.g[S4][0] = 1;
                }
            }
            if extra {
                c.cycles += 1;
            }
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(SSRC_RIDER_OBJECT));
            c.cycles += 9;
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(SSRC_RIDER_OBJECT));
            c.g[S5][0] = c.g[A1][0];
            c.g[S0][0] = 0;
            ssrc_rider_vcall(c, A1, 0x48, 0x002a_0b5c)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 1;
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(SSRC_RIDER_HUMAN));
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[S0][0] = 1;
                    c.cycles += 1;
                }
            }
            c.g[A0][0] = c.g[S3][0];

            // Both checked and the player not involved: no speech.
            c.cycles += 4;
            c.g[A1][0] = u64::MAX;
            call_ptr(c, SSRC_PLAYER_RIDER, 0x002a_0b80)?;
            c.g[A0][0] = c.g[S3][0];
            if c.g[S4][0] == 0 {
                c.cycles += 1;
            } else {
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    c.cycles += 2;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.cycles += 2;
                        let other = c.g[S1][0] != c.g[V0][0];
                        frame.restore(c, S0, 0x90);
                        if other {
                            break 'body false;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                        c.cycles += 1;
                    }
                }
            }
            c.g[A1][0] = c.g[S1][0];
            c.cycles += 9;
            c.g[A2][0] = 0;
            c.g[A3][0] = 1;
            SSRC_PLAY_RIDER_GRUNT.call(c, 0x002a_0bb4)?;
            c.g[S4][0] = sx(r32(c, S3).wrapping_add(SSRC_SPEECH));
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0;
            SSRC_IS_INSTANCE_READY.call(c, 0x002a_0bc4)?;
            let not_ready = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x90);
            if not_ready {
                break 'body false;
            }

            // okB (s0) / okA (v0): in speech range
            c.cycles += 23;
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(SSRC_RIDER_OBJECT));
            ssrc_rider_vcall(c, S5, 0x28, 0x002a_0be0)?;
            let dist = r32(c, GP).wrapping_add(SSRC_SPEECH_DIST);
            c.lwc1(12, dist);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, SSRC_IN_RANGE, 0x002a_0bf0)?;
            c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(SSRC_RIDER_OBJECT));
            c.g[S0][0] = !c.g[V0][0];
            c.g[S0][0] = sx(r32(c, S0) >> 31);
            ssrc_rider_vcall(c, S6, 0x28, 0x002a_0c0c)?;
            let dist = r32(c, GP).wrapping_add(SSRC_SPEECH_DIST);
            c.lwc1(12, dist);
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, SSRC_IN_RANGE, 0x002a_0c1c)?;
            c.g[V0][0] = !c.g[V0][0];
            c.g[V0][0] = sx(r32(c, V0) >> 31);
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                let out = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x90);
                if out {
                    break 'body false;
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S2][0];
            SSRC_GET_RIDER_SPEECH_MASK.call(c, 0x002a_0c38)?;
            c.g[S5][0] = c.g[V0][0]; // rA
            c.g[A0][0] = c.g[S1][0];
            SSRC_GET_RIDER_SPEECH_MASK.call(c, 0x002a_0c44)?;
            c.g[S0][0] = c.g[V0][0]; // rB
            if c.g[S7][0] != 0 {
                break 'body true;
            }
            c.cycles += 2;
            if c.g[FP][0] != 0 {
                frame.restore(c, S0, 0x90);
                break 'body false;
            }

            c.cycles += 8;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            SSRC_GET_RIDER_RIVALRY.call(c, 0x002a_0c64)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 2;
            let not_two = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 1;
            let rival_or_friend = if not_two {
                c.cycles += 2;
                c.g[V1][0] == c.g[V0][0]
            } else {
                true
            };
            if rival_or_friend {
                // Rivals (2) or friends (1): b says 0x208f / 0x207e.
                let event = if not_two { 0x207e } else { 0x208f };
                let rets = if not_two {
                    (0x002a_0ccc, 0x002a_0cf0)
                } else {
                    (0x002a_0c80, 0x002a_0ca4)
                };
                c.cycles += 14;
                c.mtc1(0, 20);
                if ssrc_set_context(c, S1, event, rets.0, rets.1)? {
                    c.cycles += if not_two { 2 } else { 3 };
                    c.cycles += 8;
                    ssrc_announce(c, event, S0, 0x002a_0d08, 0x002a_0d20)?;
                }
                c.cycles += 13;
                if ssrc_set_context(c, S2, 0x208a, 0x002a_0d28, 0x002a_0d4c)? {
                    c.cycles += 11;
                    ssrc_announce(c, 0x208a, S5, 0x002a_0d60, 0x002a_0d78)?;
                    frame.restore(c, S0, 0x90);
                    break 'body false;
                }
                break 'body true;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                frame.restore(c, S0, 0x90);
                break 'body false;
            }
            // Neutral (0): b says 0x2078, a 0x208e.
            c.cycles += 14;
            c.mtc1(0, 20);
            if ssrc_set_context(c, S1, 0x2078, 0x002a_0d94, 0x002a_0db8)? {
                c.cycles += 9;
                ssrc_announce(c, 0x2078, S0, 0x002a_0dcc, 0x002a_0de4)?;
            }
            c.cycles += 13;
            if ssrc_set_context(c, S2, 0x208e, 0x002a_0dec, 0x002a_0e10)? {
                c.cycles += 9;
                ssrc_announce(c, 0x208e, S5, 0x002a_0e24, 0x002a_0e3c)?;
            }
            true
        };
        if via_e3c {
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 12;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutSub_UpdatePhase1 (0x12d4e8, ai.cpp)

/// Wipeout sub fields: phase, "hit the ground" flag, impact velocity
/// (vec4), recovery timer, the rider.
pub(crate) const RWS_PHASE: u32 = 0x0;
pub(crate) const RWS_HIT: u32 = 0x54;
pub(crate) const RWS_IMPACT: u32 = 0x60;
pub(crate) const RWS_TIMER: u32 = 0x70;
pub(crate) const RWS_RIDER: u32 = 0x80;
/// Rider fields: ground normal (vec4) and its "up" amount, velocity,
/// speed float, "player" word, base (+0x30: airborne), anim controller.
pub(crate) const RWS_RIDER_NORMAL: u32 = 0x370;
pub(crate) const RWS_RIDER_UP: u32 = 0x378;
pub(crate) const RWS_RIDER_VELOCITY: u32 = 0x1e0;
pub(crate) const RWS_RIDER_SPEED: u32 = 0x470;
pub(crate) const RWS_RIDER_PLAYER: u32 = 0x150;
pub(crate) const RWS_RIDER_OBJECT: u32 = 0x6c0;
pub(crate) const RWS_RIDER_BASE: u32 = 0x77c;
pub(crate) const RWS_RIDER_ANIM: u32 = 0x784;

const RWS_UPDATE_ANIM_SPEED: Callee = Callee {
    addr: 0x0012_e528,
    f: ee::s00::RiderWipeoutSub_UpdateAnimSpeed,
};
const RWS_START_GET_UP: Callee = Callee {
    addr: 0x0012_e010,
    f: ee::s00::RiderWipeoutSub_StartGetUp,
};
const RWS_PLAY_FINAL_ANIM: Callee = Callee {
    addr: 0x0012_e468,
    f: ee::s00::RiderWipeoutSub_PlayFinalAnim,
};
const RWS_SWITCH_TO_ANIM_SET_A: Callee = Callee {
    addr: 0x0012_dd98,
    f: ee::s00::RiderWipeoutSub_SwitchToAnimSetA,
};
const RWS_SWITCH_TO_ANIM_SET_B: Callee = Callee {
    addr: 0x0012_dcb0,
    f: ee::s00::RiderWipeoutSub_SwitchToAnimSetB,
};
const RWS_PLAY_END_ANIM_B: Callee = Callee {
    addr: 0x0012_de80,
    f: ee::s00::RiderWipeoutSub_PlayEndAnimB,
};
const RWS_ADD_RECOVERY_BOOST: Callee = Callee {
    addr: 0x0010_f280,
    f: ee::s00::Rider_AddWipeoutRecoveryBoost,
};
const RWS_CHECK_RESET_PATH: Callee = Callee {
    addr: 0x0011_6120,
    f: ee::s00::Rider_CheckResetPath,
};
/// Through the table: anim layer current id / time / layer, set the
/// layer's speed, "layer finished", the audio system and its wipeout
/// sounds (final, land, phase end).
const RWS_ANIM_CURRENT: u32 = 0x0031_1ae8;
const RWS_ANIM_TIME: u32 = 0x0031_2ab0;
const RWS_ANIM_LAYER: u32 = 0x0031_1b20;
const RWS_ANIM_SET_SPEED: u32 = 0x0031_3cf0;
const RWS_ANIM_FINISHED: u32 = 0x0031_2ae8;
const RWS_AUDIO: u32 = 0x0028_b180;
const RWS_SOUND_FINAL: u32 = 0x0029_6868;
const RWS_SOUND_LAND: u32 = 0x0029_6e80;
const RWS_SOUND_PHASE_END: u32 = 0x0029_6310;

/// `vf4.x = |vf3|^2`-style dot of vf3 with `vf5` (`Src::Vec(src)`).
fn rws_dot(c: &mut Cpu, src: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(src));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `|impact|` into v0 (Q) from the vec4 at `s0 + 0x60`.
fn rws_impact_len(c: &mut Cpu) {
    c.lqc2(3, r32(c, S0).wrapping_add(RWS_IMPACT));
    rws_dot(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
}

/// `anim_current(rider->anim, 2)` with a0 = `rider + 0x784` loaded from
/// `s0 + 0x80` (the 6/7-cycle re-query blocks).
fn rws_requery_current(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
    c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RWS_RIDER_ANIM));
    call_ptr(c, RWS_ANIM_CURRENT, ret)
}

/// `void RiderWipeoutSub_UpdatePhase1(RiderWipeoutSub* self)` (0x12d4e8):
/// the tumbling phase. On the ground hit (`hit`): on the ground and
/// pushed against a steep enough normal it starts getting up (phase 4,
/// velocity projected on the normal); otherwise it finishes the final
/// anim (ids 0x1d / 0x19) or switches to anim set A slowed by the impact
/// (phase 2); then the phase-end sound, clears `hit` and reports the
/// impact to the rider. Otherwise, once the anim layer finished (or plays
/// 0x1d / 0x19): a recovery (timer >= 1 or speed >= 0) ends the wipeout
/// (phase 3; the player rider checks for a reset instead), else back on
/// the ground switches to set A (phase 2) or, when the layer finished,
/// to set B.
pub fn rider_wipeout_sub_update_phase1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        RWS_UPDATE_ANIM_SPEED.call(c, 0x0012_d504)?;
        let me = r32(c, S0);
        c.g[V0][0] = c.lw(me.wrapping_add(RWS_HIT));
        let hit = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(me.wrapping_add(RWS_RIDER));
        // true: ends through 0x12d82c (one more cycle).
        let via_82c = if hit {
            c.cycles += 4;
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RWS_RIDER_BASE));
            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x30));
            c.g[A2][0] = c.g[V0][0];
            if c.g[V1][0] == 0 {
                rws_landed(c)?;
            }
            // self->hit = 0; rider->vfunc(0x88)(|impact|)
            c.cycles += 19;
            let me = r32(c, S0);
            c.sw(me.wrapping_add(RWS_HIT), 0);
            c.g[A1][0] = c.lw(me.wrapping_add(RWS_RIDER));
            rws_impact_len(c);
            c.mtc1(V0, 12);
            c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(RWS_RIDER_OBJECT));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x88));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x8c));
            let f = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, f, 0x0012_d708)?;
            frame.restore(c, S0, 0x30);
            false
        } else {
            'idle: {
                c.cycles += 5;
                c.g[A1][0] = 2;
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RWS_RIDER_ANIM));
                call_ptr(c, RWS_ANIM_FINISHED, 0x0012_d71c)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 7;
                    c.g[A1][0] = 2;
                    rws_requery_current(c, 0x0012_d734)?;
                    c.g[V1][0] = 0x1d;
                    let final_anim = c.g[V0][0] == c.g[V1][0];
                    c.g[A1][0] = 2;
                    if !final_anim {
                        c.cycles += 6;
                        rws_requery_current(c, 0x0012_d74c)?;
                        c.g[V1][0] = 0x19;
                        if c.g[V0][0] != c.g[V1][0] {
                            frame.restore(c, S0, 0x30);
                            break 'idle false;
                        }
                    }
                    c.cycles += 1;
                }
                c.lwc1(1, r32(c, S0).wrapping_add(RWS_TIMER));

                // recover = timer >= 1 || rider speed >= 0
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.c_le_s(0, 1);
                let timed = c.fcc();
                c.g[A2][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
                let mut recover = timed;
                if !timed {
                    c.cycles += 6;
                    c.lwc1(1, r32(c, A2).wrapping_add(RWS_RIDER_SPEED));
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    c.g[V0][0] = 1;
                    if !c.fcc() {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                    c.cycles += 2;
                    recover = c.g[V0][0] != 0;
                }
                if recover {
                    c.cycles += 1;
                    c.g[A1][0] = 1;
                }
                c.cycles += 2;
                let rider = r32(c, A2);
                if recover {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(rider.wrapping_add(RWS_RIDER_PLAYER)) ^ 1;
                    c.g[A0][0] = c.g[A2][0];
                    if c.g[V0][0] != 0 {
                        c.cycles += 7;
                        RWS_ADD_RECOVERY_BOOST.call(c, 0x0012_d7c0)?;
                        c.g[A0][0] = c.g[S0][0];
                        RWS_PLAY_END_ANIM_B.call(c, 0x0012_d7c8)?;
                        c.g[V0][0] = 3;
                        c.sw(r32(c, S0).wrapping_add(RWS_PHASE), c.g[V0][0]);
                        break 'idle true;
                    }
                    // The player: reset check, boost 1.
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 3;
                    RWS_CHECK_RESET_PATH.call(c, 0x0012_d7e0)?;
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
                    c.g[A1][0] = 1;
                    RWS_ADD_RECOVERY_BOOST.call(c, 0x0012_d7ec)?;
                    frame.restore(c, S0, 0x30);
                    break 'idle false;
                }
                c.g[V0][0] = c.lw(rider.wrapping_add(RWS_RIDER_BASE));
                c.cycles += 3;
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x30));
                if c.g[V1][0] == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    RWS_SWITCH_TO_ANIM_SET_A.call(c, 0x0012_d808)?;
                    c.g[V0][0] = 2;
                    c.sw(r32(c, S0).wrapping_add(RWS_PHASE), c.g[V0][0]);
                    break 'idle true;
                }
                c.g[A0][0] = c.lw(rider.wrapping_add(RWS_RIDER_ANIM));
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, RWS_ANIM_FINISHED, 0x0012_d81c)?;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x30);
                    break 'idle false;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                RWS_SWITCH_TO_ANIM_SET_B.call(c, 0x0012_d82c)?;
                true
            }
        };
        if via_82c {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The landed part of `RiderWipeoutSub_UpdatePhase1` (a2 = the rider):
/// get up, the final anim or anim set A; then the phase-end sound.
fn rws_landed(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 6;
    let rider = r32(c, A2);
    c.lwc1(1, rider.wrapping_add(RWS_RIDER_UP));
    let gp = r32(c, GP);
    c.lwc1(0, gp.wrapping_sub(0x77ac)); // 0.9
    c.c_lt_s(0, 1);
    let mut get_up = false;
    if c.fcc() {
        // dot(impact, normal) < |impact| * -0.9
        c.cycles += 25;
        c.lwc1(2, gp.wrapping_sub(0x77a8)); // -0.9
        rws_impact_len(c);
        c.mtc1(V0, 0);
        c.lqc2(3, r32(c, S0).wrapping_add(RWS_IMPACT));
        c.lqc2(5, rider.wrapping_add(RWS_RIDER_NORMAL));
        rws_dot(c, 5);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 1);
        c.mul_s(0, 0, 2);
        c.c_lt_s(1, 0);
        get_up = c.fcc();
    }
    if get_up {
        // Get up; velocity = normal * dot(normal, velocity); phase 4.
        c.cycles += 23;
        c.g[A0][0] = c.g[S0][0];
        RWS_START_GET_UP.call(c, 0x0012_d5a4)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
        c.g[A0][0] = 4;
        let rider = r32(c, V0);
        c.lqc2(3, rider.wrapping_add(RWS_RIDER_NORMAL));
        c.lqc2(5, rider.wrapping_add(RWS_RIDER_VELOCITY));
        rws_dot(c, 5);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 0);
        c.mfc1(A1, 0);
        c.lqc2(4, rider.wrapping_add(RWS_RIDER_NORMAL));
        c.qmtc2(A1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        let stk = sp(c);
        c.sqc2(5, stk.wrapping_add(0x10));
        c.lq(V1, stk.wrapping_add(0x10));
        c.sq(V1, stk);
        c.sq(V1, rider.wrapping_add(RWS_RIDER_VELOCITY));
        c.sw(r32(c, S0).wrapping_add(RWS_PHASE), c.g[A0][0]);
    } else {
        c.g[A0][0] = c.lw(rider.wrapping_add(RWS_RIDER_ANIM));
        c.cycles += 5;
        c.g[A1][0] = 2;
        call_ptr(c, RWS_ANIM_CURRENT, 0x0012_d600)?;
        c.g[V1][0] = 0x1d;
        let mut final_anim = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 2;
        if !final_anim {
            c.cycles += 6;
            rws_requery_current(c, 0x0012_d618)?;
            c.g[V1][0] = 0x19;
            final_anim = c.g[V0][0] == c.g[V1][0];
            if !final_anim {
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
            }
        }
        if final_anim {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            RWS_PLAY_FINAL_ANIM.call(c, 0x0012_d62c)?;
            call_ptr(c, RWS_AUDIO, 0x0012_d634)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, RWS_SOUND_FINAL, 0x0012_d640)?;
        } else {
            // Anim set A at the layer speed scaled by the anim time.
            c.cycles += 25;
            c.g[A1][0] = 2;
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RWS_RIDER_ANIM));
            call_ptr(c, RWS_ANIM_TIME, 0x0012_d654)?;
            c.mov_s(21, 0);
            c.g[A0][0] = c.g[S0][0];
            RWS_SWITCH_TO_ANIM_SET_A.call(c, 0x0012_d660)?;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
            c.g[A1][0] = 2;
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(RWS_RIDER_ANIM));
            call_ptr(c, RWS_ANIM_LAYER, 0x0012_d670)?;
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
            c.g[A1][0] = 2;
            c.lwc1(20, r32(c, V0).wrapping_add(0x10));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(RWS_RIDER_ANIM));
            call_ptr(c, RWS_ANIM_LAYER, 0x0012_d684)?;
            c.mul_s(12, 20, 21);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, RWS_ANIM_SET_SPEED, 0x0012_d694)?;
            call_ptr(c, RWS_AUDIO, 0x0012_d69c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, RWS_SOUND_LAND, 0x0012_d6a8)?;
        }
        c.g[V0][0] = 2;
        c.cycles += 1;
        c.sw(r32(c, S0).wrapping_add(RWS_PHASE), c.g[V0][0]);
    }
    c.cycles += 5;
    call_ptr(c, RWS_AUDIO, 0x0012_d6b8)?;
    c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(RWS_RIDER));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, RWS_SOUND_PHASE_END, 0x0012_d6c4)
}

// ---------------------------------------------------------------------------
// WorldState_Write (0x26d818, cReplay::saveFrame, all_in_one_replay.cpp)

/// Globals (gp-relative): the two frame counters written after the time,
/// the world (`gp - 0x848`: +0x84 the object manager, whose +0xc is the
/// playback world), the managers that save their state.
pub(crate) const WSW_COUNTER_A: u32 = 0x2a88;
pub(crate) const WSW_COUNTER_B: u32 = 0x2a8c;
pub(crate) const WSW_WORLD: u32 = 0x848;
pub(crate) const WSW_OBJECT_HOLDER: u32 = 0xce8;
pub(crate) const WSW_CAMERA_MAN: u32 = 0x6f0;
pub(crate) const WSW_EFFECT_MAN: u32 = 0xf00;
pub(crate) const WSW_PARTICLE_MAN: u32 = 0xf38;

const WSW_REWIND_WRITE: Callee = Callee {
    addr: 0x0026_e9c0,
    f: ee::s07::ReplayStream_RewindWrite,
};
const WSW_WRITE_OBJECTS: Callee = Callee {
    addr: 0x0026_d988,
    f: ee::s07::WorldState_WriteObjects,
};
const WSW_WRITE_DEAD_OBJECTS: Callee = Callee {
    addr: 0x0026_da88,
    f: ee::s07::WorldState_WriteDeadObjects,
};
/// Through the table: the timer read, `cBE_getBE`, the BE save, the
/// savers of the other managers, the rider manager save.
const WSW_GET_TIME: u32 = 0x0031_78e0;
const WSW_GET_BE: u32 = 0x0014_dc80;
const WSW_BE_SAVE: u32 = 0x0014_df08;
const WSW_SAVE_2D9CB0: u32 = 0x002d_9cb0;
const WSW_SAVE_OBJECTS: u32 = 0x0030_bb10;
const WSW_SAVE_CAMERAS: u32 = 0x0022_9e20;
const WSW_SAVE_EFFECTS: u32 = 0x0034_41a8;
const WSW_SAVE_PARTICLES: u32 = 0x0035_7ca8;
const WSW_SAVE_RIDERS: u32 = 0x0010_3480;

/// `stream->write(a1, a2)` (vtable slot 8 of the stream in s0).
fn wsw_stream_write(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    let vt = r32(c, V1);
    c.g[A0][0] = c.lh(vt.wrapping_add(8));
    c.g[V0][0] = c.lw(vt.wrapping_add(0xc));
    let f = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret)
}

/// `void cReplay::saveFrame(cReplayFramePtr* stream)` (0x26d818): rewinds
/// the frame stream and writes the time (0x18 bytes), the two frame
/// counters, then lets the BE, the object manager, the playback world and
/// the other managers save into it, the live and dead objects and the
/// riders last.
pub fn world_state_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 92;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A1][0]; // stream
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x30);
        c.g[A0][0] = c.g[S0][0];
        WSW_REWIND_WRITE.call(c, 0x0026_d83c)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, WSW_GET_TIME, 0x0026_d844)?;
        // write(&time, 0x18)
        c.g[V1][0] = c.lw(r32(c, S0));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x18;
        wsw_stream_write(c, 0x0026_d860)?;
        // write(&counter_a, 4)
        let gp = r32(c, GP);
        c.g[V1][0] = c.lw(r32(c, S0));
        c.g[S1][0] = sx(sp(c).wrapping_add(0x20));
        c.g[A3][0] = c.lw(gp.wrapping_add(WSW_COUNTER_A));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 4;
        c.sw(sp(c).wrapping_add(0x20), c.g[A3][0]);
        wsw_stream_write(c, 0x0026_d888)?;
        // write(&counter_b, 4)
        let gp = r32(c, GP);
        c.g[A0][0] = c.lw(gp.wrapping_add(WSW_COUNTER_B));
        c.g[A2][0] = 4;
        c.g[V1][0] = c.lw(r32(c, S0));
        c.g[A1][0] = c.g[S1][0];
        c.sw(sp(c).wrapping_add(0x20), c.g[A0][0]);
        wsw_stream_write(c, 0x0026_d8ac)?;
        call_ptr(c, WSW_GET_BE, 0x0026_d8b4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, WSW_BE_SAVE, 0x0026_d8c0)?;
        // world->objects->save(stream)
        let gp = r32(c, GP);
        c.g[V1][0] = c.lw(gp.wrapping_sub(WSW_WORLD));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.lw(r32(c, V1).wrapping_add(0x84));
        c.g[V0][0] = c.lw(r32(c, A2));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x40));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x44));
        let f = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, f, 0x0026_d8e0)?;
        // pbWorld(world->objects)->save(stream)
        let gp = r32(c, GP);
        c.g[A0][0] = c.lw(gp.wrapping_sub(WSW_WORLD));
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x84));
        c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0xcc));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x38));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x3c));
        let f = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, f, 0x0026_d904)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, WSW_SAVE_2D9CB0, 0x0026_d90c)?;
        // The managers' savers: (manager, stream).
        for (global, minus, saver, ret) in [
            (WSW_OBJECT_HOLDER, false, WSW_SAVE_OBJECTS, 0x0026_d918),
            (WSW_CAMERA_MAN, true, WSW_SAVE_CAMERAS, 0x0026_d924),
            (WSW_EFFECT_MAN, false, WSW_SAVE_EFFECTS, 0x0026_d930),
            (WSW_PARTICLE_MAN, false, WSW_SAVE_PARTICLES, 0x0026_d93c),
        ] {
            let gp = r32(c, GP);
            let at = if minus {
                gp.wrapping_sub(global)
            } else {
                gp.wrapping_add(global)
            };
            c.g[A0][0] = c.lw(at);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, saver, ret)?;
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        WSW_WRITE_OBJECTS.call(c, 0x0026_d94c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        WSW_WRITE_DEAD_OBJECTS.call(c, 0x0026_d958)?;
        // riders: pbWorld(world->objects)->riders (+0xa4)
        let gp = r32(c, GP);
        c.g[A0][0] = c.lw(gp.wrapping_sub(WSW_WORLD));
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x84));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0xa4));
        call_ptr(c, WSW_SAVE_RIDERS, 0x0026_d970)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpatialHash_Insert (0x328c20, aifwddiff.cpp)

/// Spatial hash root: its cell key (level, x, y, z: 16 bytes) and node.
pub(crate) const SHI_ROOT_NODE: u32 = 0x10;
/// Node: 8 children (+0x00..0x20), 3 item lists (+0x20..0x2c).
pub(crate) const SHI_NODE_LISTS: u32 = 0x20;

const SHI_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::EA_MemAllocNamed,
};
const SHI_INSERT_AT_KEY: Callee = Callee {
    addr: 0x0032_8660,
    f: ee::s10::SpatialHash_InsertAtKey,
};
/// `MEM_fill` (through the table).
const SHI_MEM_FILL: u32 = 0x003e_6448;
/// The allocator's name for nodes (0x4a0000 + 0x3fc0).
const SHI_NODE_NAME: u32 = 0x004a_3fc0;

/// `v0 = (int)v0 >> (a0 & 31)` (`srav`: the item cell's coordinate in
/// the root's level).
fn shi_srav_v0(c: &mut Cpu) {
    let v = c.g[V0][0] as u32;
    c.g[V0][0] = sx((v as i32 >> (r32(c, A0) & 31)) as u32);
}

/// `nodes = alloc(0x2c)` result in v0: clears the three item lists from
/// `v0 + 0x20` (`a0 = -1`, `v1 = 2` count down), with the event check at
/// `head`. `Ok(false)` when an event unwinds.
fn shi_clear_lists(c: &mut Cpu, head: u32) -> Result<bool, crate::call::Unwound> {
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        if !more {
            return Ok(true);
        }
        if back_edge_unwinds(c, head) {
            return Ok(false);
        }
    }
}

/// `void SpatialHash_Insert(Root* root, int list, Item* item, Cell* cell)`
/// (0x328c20): an empty root takes the item's cell and a fresh node with
/// the item in list `list`. Otherwise, while the root's cell does not
/// contain the item's cell, the root grows to its parent cell (a new node
/// with the old one as the child at its octant); then
/// `SpatialHash_InsertAtKey` places the item. The generated body is a
/// dispatcher (re-entered at its loop heads after an event unwind), so the
/// port keeps the block addresses as states.
pub fn spatial_hash_insert(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xd0 };
        let mut pc = c.pc;
        loop {
            match pc {
                0x0032_8c20 => {
                    c.cycles += 18;
                    Frame::open(c, 0xd0);
                    frame.save(c, S2, 0xa0);
                    frame.save(c, S3, 0x90);
                    frame.save(c, S5, 0x70);
                    c.g[S3][0] = c.g[A0][0]; // root
                    frame.save(c, S7, 0x50);
                    c.g[S2][0] = sx(r32(c, S3).wrapping_add(SHI_ROOT_NODE));
                    frame.save(c, S0, 0xc0);
                    c.g[S7][0] = c.g[A2][0]; // item
                    frame.save(c, S1, 0xb0);
                    frame.save(c, S4, 0x80);
                    frame.save(c, S6, 0x60);
                    frame.save(c, FP, 0x40);
                    frame.save_ra(c, 0x30);
                    let list = frame.at(c, 0x20);
                    c.sw(list, c.g[A1][0]);
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(SHI_ROOT_NODE));
                    c.g[S5][0] = c.g[A3][0]; // cell
                    pc = if c.g[V0][0] != 0 {
                        0x0032_8d14
                    } else {
                        0x0032_8c68
                    };
                }
                0x0032_8c68 => {
                    // root->cell = *cell; node = new Node
                    c.cycles += 20;
                    let (cell, root) = (r32(c, S5), r32(c, S3));
                    c.ldl(V0, cell.wrapping_add(7));
                    c.ldr(V0, cell);
                    c.ldl(V1, cell.wrapping_add(0xf));
                    c.ldr(V1, cell.wrapping_add(8));
                    c.sdl(V0, root.wrapping_add(7));
                    c.sdr(V0, root);
                    c.sdl(V1, root.wrapping_add(0xf));
                    c.sdr(V1, root.wrapping_add(8));
                    c.g[A1][0] = sx(SHI_NODE_NAME);
                    c.g[A0][0] = 0x2c;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    SHI_ALLOC.call(c, 0x0032_8ca0)?;
                    c.g[T0][0] = c.lw(sp(c).wrapping_add(0x20));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = u64::MAX;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(SHI_NODE_LISTS));
                    c.g[S1][0] = sx(r32(c, T0) << 2);
                    c.g[V1][0] = 2;
                    pc = 0x0032_8cb8;
                }
                0x0032_8cb8 => {
                    if !shi_clear_lists(c, 0x0032_8cb8)? {
                        return Ok(());
                    }
                    pc = 0x0032_8cd4;
                }
                0x0032_8cd4 => {
                    // MEM_fill(children); root->node = node; push item
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x20;
                    call_ptr(c, SHI_MEM_FILL, 0x0032_8ce4)?;
                    c.sw(r32(c, S2), c.g[S0][0]);
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(SHI_ROOT_NODE));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(SHI_NODE_LISTS));
                    c.g[V0][0] = c.lw(r32(c, A0));
                    if c.g[V0][0] != 0 {
                        c.sw(r32(c, V0).wrapping_add(4), c.g[S7][0]); // head->prev
                    }
                    pc = 0x0032_8d00;
                }
                0x0032_8d00 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, A0));
                    let item = r32(c, S7);
                    c.sw(item.wrapping_add(4), 0);
                    c.sw(item, c.g[V0][0]);
                    c.sw(r32(c, A0), c.g[S7][0]);
                    pc = 0x0032_8ef8;
                }
                0x0032_8d14 => {
                    c.cycles += 2;
                    c.g[FP][0] = 1;
                    pc = 0x0032_8e3c;
                }
                0x0032_8d1c => {
                    c.cycles += 1;
                    pc = 0x0032_8d20;
                }
                0x0032_8d20 => {
                    // parent = (level + 1, x >> 1, y >> 1, z >> 1); new node
                    c.cycles += 32;
                    let root = r32(c, S3);
                    let stk = sp(c);
                    c.g[A1][0] = c.lw(root.wrapping_add(4));
                    c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                    c.g[V0][0] = c.lw(root.wrapping_add(8));
                    c.g[T1][0] = 0x4a0000;
                    c.g[V1][0] = c.lw(root.wrapping_add(0xc));
                    c.g[A1][0] = sx((r32(c, A1) as i32 >> 1) as u32);
                    c.g[V0][0] = sx((r32(c, V0) as i32 >> 1) as u32);
                    c.sw(stk.wrapping_add(0x14), c.g[A1][0]);
                    c.g[V1][0] = sx((r32(c, V1) as i32 >> 1) as u32);
                    c.sw(stk.wrapping_add(0x10), c.g[A2][0]);
                    c.sw(stk.wrapping_add(0x1c), c.g[V1][0]);
                    c.g[A1][0] = sx(SHI_NODE_NAME);
                    c.sw(stk.wrapping_add(0x18), c.g[V0][0]);
                    c.g[A0][0] = 0x2c;
                    c.g[A2][0] = 0x2000_0000;
                    c.ldl(V0, stk.wrapping_add(0x17));
                    c.ldr(V0, stk.wrapping_add(0x10));
                    c.ldl(V1, stk.wrapping_add(0x1f));
                    c.ldr(V1, stk.wrapping_add(0x18));
                    c.sdl(V0, stk.wrapping_add(7));
                    c.sdr(V0, stk);
                    c.sdl(V1, stk.wrapping_add(0xf));
                    c.sdr(V1, stk.wrapping_add(8));
                    c.g[A3][0] = 0;
                    SHI_ALLOC.call(c, 0x0032_8d84)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[S6][0] = c.lw(r32(c, S3).wrapping_add(SHI_ROOT_NODE));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A0][0] = u64::MAX;
                    c.g[V0][0] = sx(r32(c, S4).wrapping_add(SHI_NODE_LISTS));
                    c.g[V1][0] = 2;
                    pc = 0x0032_8da0;
                }
                0x0032_8da0 => {
                    if !shi_clear_lists(c, 0x0032_8da0)? {
                        return Ok(());
                    }
                    pc = 0x0032_8dbc;
                }
                0x0032_8dbc => {
                    // MEM_fill(children); the old node is the child at the
                    // octant (x - 2px) * 16 + (y - 2py) * 8 + (z - 2pz) * 4.
                    c.cycles += 32;
                    let parent = r32(c, A2);
                    let root = r32(c, S3);
                    c.g[A3][0] = c.lw(parent.wrapping_add(0xc));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V1][0] = c.lw(parent.wrapping_add(4));
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw(parent.wrapping_add(8));
                    c.g[A3][0] = sx(r32(c, A3) << 1);
                    c.g[S2][0] = c.lw(root.wrapping_add(4));
                    c.g[V1][0] = sx(r32(c, V1) << 1);
                    c.g[S1][0] = c.lw(root.wrapping_add(8));
                    c.g[V0][0] = sx(r32(c, V0) << 1);
                    c.g[S0][0] = c.lw(root.wrapping_add(0xc));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_sub(r32(c, V1)));
                    c.g[S1][0] = sx(r32(c, S1).wrapping_sub(r32(c, V0)));
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(r32(c, A3)));
                    call_ptr(c, SHI_MEM_FILL, 0x0032_8dfc)?;
                    let (stk, root) = (sp(c), r32(c, S3));
                    c.g[S0][0] = sx(r32(c, S0) << 2);
                    c.g[S1][0] = sx(r32(c, S1) << 3);
                    c.ldl(V0, stk.wrapping_add(7));
                    c.ldr(V0, stk);
                    c.ldl(V1, stk.wrapping_add(0xf));
                    c.ldr(V1, stk.wrapping_add(8));
                    c.sdl(V0, root.wrapping_add(7));
                    c.sdr(V0, root);
                    c.sdl(V1, root.wrapping_add(0xf));
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, S1)));
                    c.g[S2][0] = sx(r32(c, S2) << 4);
                    c.sdr(V1, root.wrapping_add(8));
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, S2)));
                    c.sw(root.wrapping_add(SHI_ROOT_NODE), c.g[S4][0]);
                    c.g[S0][0] = sx(r32(c, S4).wrapping_add(r32(c, S0)));
                    c.sw(r32(c, S0), c.g[S6][0]);
                    pc = 0x0032_8e3c;
                }
                0x0032_8e3c => {
                    // contains(root->cell, *cell): same level and same
                    // coordinates once shifted down by the level difference
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, S3));
                    c.g[V1][0] = c.lw(r32(c, S5));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                    c.g[A2][0] = c.g[V0][0];
                    pc = if (c.g[A0][0] as i64) >= 0 {
                        0x0032_8e58
                    } else {
                        0x0032_8e50
                    };
                }
                0x0032_8e50 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0032_8edc;
                }
                0x0032_8e58 => {
                    c.cycles += 2;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(4));
                        pc = 0x0032_8e9c;
                    } else {
                        pc = 0x0032_8e60;
                    }
                }
                0x0032_8e60 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, S5).wrapping_add(4));
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(4));
                    let differ = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = 0;
                    pc = if differ { 0x0032_8e94 } else { 0x0032_8e70 };
                }
                0x0032_8e70 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, S5).wrapping_add(8));
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(8));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = c.g[A0][0];
                        pc = 0x0032_8edc;
                    } else {
                        pc = 0x0032_8e80;
                    }
                }
                0x0032_8e80 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(r32(c, S5).wrapping_add(0xc));
                    c.g[A0][0] = c.g[FP][0];
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0xc));
                    c.g[V1][0] ^= c.g[V0][0];
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = 0;
                    }
                    pc = 0x0032_8e94;
                }
                0x0032_8e94 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A0][0];
                    pc = 0x0032_8edc;
                }
                0x0032_8e9c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(4));
                    shi_srav_v0(c);
                    let differ = c.g[V0][0] != c.g[V1][0];
                    c.g[A1][0] = 0;
                    pc = if differ { 0x0032_8ed8 } else { 0x0032_8eac };
                }
                0x0032_8eac => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(8));
                    c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(8));
                    shi_srav_v0(c);
                    let differ = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0];
                    pc = if differ { 0x0032_8edc } else { 0x0032_8ec0 };
                }
                0x0032_8ec0 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r32(c, S5).wrapping_add(0xc));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0xc));
                    shi_srav_v0(c);
                    c.g[V0][0] ^= c.g[V1][0];
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = 0;
                    }
                    pc = 0x0032_8ed8;
                }
                0x0032_8ed8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[A1][0];
                    pc = 0x0032_8edc;
                }
                0x0032_8edc => {
                    c.cycles += 2;
                    let outside = c.g[V0][0] == 0;
                    c.g[A1][0] = c.lw(sp(c).wrapping_add(0x20));
                    if outside {
                        if back_edge_unwinds(c, 0x0032_8d20) {
                            return Ok(());
                        }
                        pc = 0x0032_8d20;
                    } else {
                        pc = 0x0032_8ee4;
                    }
                }
                0x0032_8ee4 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(r32(c, S3).wrapping_add(SHI_ROOT_NODE));
                    c.g[A2][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = c.g[S3][0];
                    SHI_INSERT_AT_KEY.call(c, 0x0032_8ef8)?;
                    pc = 0x0032_8ef8;
                }
                0x0032_8ef8 => {
                    c.cycles += 12;
                    frame.restore(c, S0, 0xc0);
                    frame.restore(c, S1, 0xb0);
                    frame.restore(c, S2, 0xa0);
                    frame.restore(c, S3, 0x90);
                    frame.restore(c, S4, 0x80);
                    frame.restore(c, S5, 0x70);
                    frame.restore(c, S6, 0x60);
                    frame.restore(c, S7, 0x50);
                    frame.restore(c, FP, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0032_8c20, 0x0032_8f28);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{check, Rng, DATA};

    /// Scripted fake results for fixtures that need exact callee results
    /// (zero or negative, which the plain fakes cannot return): per callee
    /// slot up to 8 results (+0x20 * slot), a call counter (+0x400) and a
    /// "scripted" flag (+0x480). Unscripted slots behave like `fake`.
    pub(crate) const OC374_SCRIPT: u32 = DATA + 0x28_0000;

    /// A logged fake call to `target` returning the next scripted result
    /// of `slot` (sign-extended), if the slot is scripted.
    pub(crate) fn oc374_scripted(c: &mut Cpu, target: u32, slot: u32) {
        let ra = c.g[RA][0] as u32;
        let scripted = c.bus.read32(OC374_SCRIPT + 0x480 + 4 * slot) != 0;
        let mut i = 0;
        if scripted {
            let counter = OC374_SCRIPT + 0x400 + 4 * slot;
            i = c.bus.read32(counter);
            c.bus.write32(counter, i + 1);
        }
        crate::difftest::fakes::fake(c, target, ra);
        if scripted && c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(OC374_SCRIPT + 0x20 * slot + 4 * (i & 7)));
        }
    }

    /// Script the results of `slot`.
    pub(crate) fn oc374_script(c: &mut Cpu, slot: u32, results: &[u32]) {
        for (i, &v) in results.iter().enumerate() {
            c.bus.write32(OC374_SCRIPT + 0x20 * slot + 4 * i as u32, v);
        }
        c.bus.write32(OC374_SCRIPT + 0x400 + 4 * slot, 0);
        c.bus.write32(OC374_SCRIPT + 0x480 + 4 * slot, 1);
    }

    /// A gated scripted fake for an override slot (gate: `OC338_CTRL`).
    macro_rules! oc374_scripted_fake {
        ($name:ident, $prev:ident, $addr:expr, $slot:expr, $gen:path) => {
            static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                use crate::game::object::movenode::tests::{OC338_CTRL, OC338_MAGIC};
                if c.bus.read32(OC338_CTRL) != OC338_MAGIC {
                    return crate::difftest::fakes::fall_back(c, &$prev, $gen);
                }
                oc374_scripted(c, $addr, $slot);
            }
        };
    }

    /// PATH_TrackQueueNode: a chain of 1-6 nodes (types 0, -1, -2, -3, -4
    /// ending in a playable one), each with one catch-all branch to the
    /// next; type-0 nodes with and without an event (flag bit 7 set or
    /// not) and the time flag, a node callback (fake) or none, repeat
    /// nodes on the voice's current node or another; real RemapNode
    /// (negative track, or a node with no remap table), SelectBranch,
    /// QueueAppend, copyevent, memset and AddEventToMaps (no maps).
    #[test]
    fn path_track_queue_node_matches() {
        const STATE: u32 = DATA + 0x2000;
        const EVENTS: u32 = DATA + 0x4000;
        const NODES: u32 = DATA + 0x5000;
        const INDEX: u32 = DATA + 0x6000;
        const BRANCHES: u32 = DATA + 0x6800;
        const VOICES: u32 = DATA + 0x7000;
        check(
            ee::s14::f_003d3900__generated,
            path_track_queue_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_3900, 3);
                c.pc = 0x003d_3900;
                c.bus.write32(PATH_STATE_PTR, STATE);
                for off in (0..0xa00).step_by(4) {
                    c.bus.write32(STATE + off, 0);
                }
                c.bus.write32(STATE + PATH_NODES, NODES);
                c.bus.write32(STATE + PATH_NODE_INDEX, INDEX);
                c.bus.write32(STATE + PATH_BRANCH_INDEX, BRANCHES);
                c.bus.write32(STATE + PATH_EVENT_TABLE, EVENTS);
                for v in 0..4 {
                    let voice = VOICES + 0x40 * v;
                    c.bus.write32(STATE + PATH_VOICES + 4 * v, voice);
                    c.bus.write32(voice + PATH_VOICE_FLAGS, r.next_u32());
                    c.bus
                        .write16(voice + PATH_VOICE_NODE, (r.next_u32() % 4) as u16);
                }
                for i in 0..32 {
                    c.bus.write32(EVENTS + 4 * i, r.next_u32());
                }
                for i in 0..8 {
                    c.bus.write16(BRANCHES + 2 * i, (r.next_u32() % 8) as u16);
                }
                c.bus.write32(PATH_TRACK_TIME, r.next_u32() % 100_000);
                let callback = if seed % 3 == 0 { 0 } else { FAKE };
                c.bus.write32(PATH_NODE_CALLBACK, callback);
                c.bus.write32(PATH_NODE_CALLBACK_ARG, r.next_u32());
                let last = seed % 6;
                for k in 0..8u32 {
                    c.bus.write16(INDEX + 2 * k, (k * 8) as u16);
                    let node = NODES + 0x20 * k;
                    let ty: u16 = if k >= last {
                        1 + (r.next_u32() % 5) as u16
                    } else {
                        match r.next_u32() % 5 {
                            0 | 1 => 0,
                            2 => 0xffff,
                            3 => 0xfffe,
                            _ => {
                                if r.next_u32() % 2 == 0 {
                                    0xfffd
                                } else {
                                    0xfffc
                                }
                            }
                        }
                    };
                    c.bus.write16(node, ty);
                    let h2 = (r.next_u32() as u16 & 0xffe0) | (r.next_u32() % 4) as u16;
                    c.bus.write16(node + 2, h2);
                    let timed = if r.next_u32() % 2 == 0 {
                        0x20000
                    } else {
                        (r.next_u32() % 8) << 17
                    };
                    // One branch (count in bits 12..16); remap byte 0.
                    c.bus
                        .write32(node + 4, timed | 1 << 12 | (r.next_u32() & 0xfff0_0f00));
                    let event = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() % 32
                    };
                    c.bus
                        .write32(node + 8, (r.next_u32() & !0x7c00) | event << 10);
                    c.bus.write32(node + 0xc, (r.next_u32() % 8) << 2);
                    // Catch-all branch to the next node.
                    c.bus.write8(node + 0x10, 0x80);
                    c.bus.write8(node + 0x11, 0x7f);
                    c.bus.write16(node + 0x12, (k + 1) as u16);
                }
                c.g[A0][0] = if seed % 2 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() % 4) as u64
                };
                c.g[A1][0] = 0;
                c.g[A2][0] = (r.next_u32() % 128) as u64;
            },
        );
    }

    /// SNDSTRM_ParseHeader: first or next segment; 0-4 pending markers
    /// reported to 0-2 marker callbacks (fakes; or a negative count); a
    /// header that differs, a format that differs, a flagged change or
    /// none, with a playing (stop) or idle stream (take over, aligned and
    /// unaligned copies); playback already running or started. Gated fakes
    /// for SND_ParseStreamHeader, StreamFile_ReleaseChunk,
    /// SNDSTRM_CalcDataRate and SNDSTRM_StartPlayback; real memcmp.
    #[test]
    fn sndstrm_parse_header_matches() {
        use crate::difftest::fakes::{fake, fall_back, hook};
        use crate::game::object::movenode::tests::{oc338_fake, OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        use std::sync::atomic::AtomicUsize;
        oc338_fake!(
            oc374_fake_parse_stream_header,
            OC374_PREV_PARSE_STREAM_HEADER,
            0x003b_b820,
            ee::s13::f_003bb820__generated
        );
        oc338_fake!(
            oc374_fake_release_chunk,
            OC374_PREV_RELEASE_CHUNK,
            0x003e_13e8,
            ee::s14::f_003e13e8__generated
        );
        oc338_fake!(
            oc374_fake_calc_data_rate,
            OC374_PREV_CALC_DATA_RATE,
            0x003b_6948,
            ee::s13::f_003b6948__generated
        );
        oc338_fake!(
            oc374_fake_start_playback,
            OC374_PREV_START_PLAYBACK,
            0x003b_68b8,
            ee::s13::f_003b68b8__generated
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003BB820),
            oc374_fake_parse_stream_header,
            &OC374_PREV_PARSE_STREAM_HEADER,
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E13E8),
            oc374_fake_release_chunk,
            &OC374_PREV_RELEASE_CHUNK,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B6948),
            oc374_fake_calc_data_rate,
            &OC374_PREV_CALC_DATA_RATE,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B68B8),
            oc374_fake_start_playback,
            &OC374_PREV_START_PLAYBACK,
        );
        const CHUNK: u32 = DATA + 0x5000;
        const PREV: u32 = DATA + 0x3000;
        const SEGMENT: u32 = DATA + 0x3100;
        covered(
            ee::s13::f_003b6bd0__generated,
            sndstrm_parse_header,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3b_6bd0, 8);
                c.pc = 0x003b_6bd0;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let stream = DATA + 0x1000 + 4 * (seed / 7 % 2);
                let idx = seed % 4;
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = sx(CHUNK);
                c.bus.write32(SNDSTRM_TABLE + 4 * idx, stream);
                for off in (0..0x140).step_by(4) {
                    c.bus.write32(stream + off, r.next_u32());
                }
                for off in (0..0x40).step_by(4) {
                    c.bus.write32(SEGMENT + off, r.next_u32());
                    c.bus.write32(CHUNK + off, r.next_u32());
                }
                if seed % 2 == 0 {
                    c.bus.write32(stream + SNDSTRM_SEGMENT, 0);
                    c.bus.write32(stream + SNDSTRM_FIRST_SEGMENT, SEGMENT);
                } else {
                    c.bus.write32(stream + SNDSTRM_SEGMENT, PREV);
                    c.bus.write32(PREV, SEGMENT);
                }
                let case = seed % 6;
                // Pending markers (none when the formats are to match).
                let markers = if case >= 2 { 0 } else { r.next_u32() % 5 };
                for i in 0..4 {
                    let id = if i < markers {
                        1 + r.next_u32() % 1000
                    } else {
                        0
                    };
                    c.bus.write32(stream + SNDSTRM_MARKERS + 4 * i, id);
                    c.bus
                        .write32(stream + SNDSTRM_MARKER_VALUES + 4 * i, r.next_u32());
                }
                c.bus.write32(stream + SNDSTRM_MARKERS + 4 * markers, 0);
                if case >= 2 {
                    for i in 0..4 {
                        c.bus.write32(stream + SNDSTRM_MARKER_VALUES + 4 * i, 0);
                    }
                }
                let flag = if case == 2 { 1 + r.next_u32() % 4 } else { 0 };
                c.bus.write32(stream + SNDSTRM_FORMAT_CHANGED, flag);
                if case >= 1 {
                    let h = c.bus.read32(stream + SNDSTRM_HEADER);
                    c.bus.write32(stream + SNDSTRM_NEW_HEADER, h);
                }
                if case >= 2 {
                    for off in (0..0x68).step_by(4) {
                        let w = c.bus.read32(stream + SNDSTRM_NEW_FORMAT + off);
                        c.bus.write32(stream + SNDSTRM_FORMAT + off, w);
                    }
                }
                if seed / 6 % 2 == 0 {
                    c.bus.write16(stream + SNDSTRM_HEADER, 0);
                    if case >= 1 {
                        c.bus.write16(stream + SNDSTRM_NEW_HEADER, 0);
                    }
                }
                c.bus.write8(
                    stream + SNDSTRM_STATE,
                    if seed % 5 == 0 { 1 } else { r.next_u32() as u8 },
                );
                let callbacks = [0xffu8, 0, 1, 2][(seed / 3 % 4) as usize];
                c.bus
                    .write8(SNDSTRM_SYS + SNDSTRM_SYS_CALLBACK_COUNT, callbacks);
                for i in 0..2 {
                    c.bus
                        .write32(SNDSTRM_SYS + SNDSTRM_SYS_CALLBACKS + 4 * i, FAKE + 0x10 * i);
                }
            },
        );
    }

    /// __ieee754_log: +-0, negative, subnormal, inf/NaN, exact powers of
    /// two (k = 0 and not), |f| < 2^-20 (k = 0 and not) and general
    /// normal inputs on both sides of the `i > 0` split; real soft-float
    /// helpers, the constant pool from the ELF image.
    #[test]
    fn ieee754_log_matches() {
        check(
            ee::s15::f_0040db58__generated,
            ieee754_log,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x40_db58, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0040_db58;
                let lo = r.next_u32() as u64;
                let sign = ((r.next_u32() & 1) as u64) << 63;
                let exp =
                    |r: &mut Rng, span: u32| (0x3ff + r.next_u32() % (2 * span + 1) - span) as u64;
                let x: u64 = match seed % 10 {
                    0 => sign,
                    1 => 1 << 63 | (r.next_u32() as u64) << 32 | lo,
                    2 => ((r.next_u32() % 0x10_0000) as u64) << 32 | lo,
                    3 => {
                        let m = if r.next_u32() % 2 == 0 { 0 } else { lo };
                        sign | 0x7ff0_0000_0000_0000 | m
                    }
                    4 => exp(&mut r, 3) << 52,
                    5 => {
                        let hm = [0xf_fffeu64, 0xf_ffff, 0][(r.next_u32() % 3) as usize];
                        let low = if r.next_u32() % 3 == 0 { 0 } else { lo };
                        exp(&mut r, 3) << 52 | hm << 32 | low
                    }
                    6 | 7 => exp(&mut r, 2) << 52 | ((r.next_u32() % 0x10_0000) as u64) << 32 | lo,
                    _ => {
                        let e = 1 + (r.next_u32() % 0x7fe) as u64;
                        e << 52 | ((r.next_u32() % 0x10_0000) as u64) << 32 | lo
                    }
                };
                c.g[A0][0] = x;
            },
        );
    }

    /// PATH_TrackAdvance: an aligned or unaligned voice whose current node
    /// is none (-1) or 0-3; repeat counts off, below or within the node's
    /// rows * columns; SelectBranch to a node or to none (no branches);
    /// the target owned by another voice (PostNodeEvents) or, for "none",
    /// by the voice itself / nobody / another voice; real callees (the
    /// queue-node walk over playable nodes, SubmitQueue on its short path).
    /// Not reached: a failed submit (needs SubmitQueue's long path).
    #[test]
    fn path_track_advance_matches() {
        const STATE: u32 = DATA + 0x2000;
        const NODES: u32 = DATA + 0x5000;
        const INDEX: u32 = DATA + 0x6000;
        const VOICES: u32 = DATA + 0x7000;
        check(
            ee::s14::f_003d3d20__generated,
            path_track_advance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_3d20, 0);
                c.pc = 0x003d_3d20;
                c.bus.write32(PATH_STATE_PTR, STATE);
                for off in (0..0xa00).step_by(4) {
                    c.bus.write32(STATE + off, 0);
                }
                c.bus.write32(STATE, u32::MAX); // no current voice
                c.bus.write32(STATE + PATH_NODES, NODES);
                c.bus.write32(STATE + PATH_NODE_INDEX, INDEX);
                c.bus.write32(STATE + PATH_EVENT_WRITE, r.next_u32());
                let voice = VOICES + 4 * (seed / 2 % 2);
                let voices = [voice, VOICES + 0x200, VOICES + 0x400, 0];
                for (i, &v) in voices.iter().enumerate() {
                    c.bus.write32(STATE + PATH_VOICES + 4 * i as u32, v);
                }
                for off in (0..0x600).step_by(4) {
                    c.bus.write32(VOICES + off, r.next_u32());
                }
                let cur: u16 = if seed % 5 == 0 {
                    0xffff
                } else {
                    (r.next_u32() % 4) as u16
                };
                c.bus.write16(voice + PATH_VOICE_CUR_NODE, cur);
                let n = [0u32, 0x1f, 1, 2, 4, 9][(seed / 3 % 6) as usize];
                let flags = c.bus.read32(voice + PATH_VOICE_FLAGS) & !0x1f0 | n << 4;
                c.bus.write32(voice + PATH_VOICE_FLAGS, flags);
                let chord = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(voice + PATH_VOICE_CHORD, chord as u8);
                c.bus
                    .write8(voice + PATH_VOICE_VEL, (r.next_u32() % 128) as u8);
                for k in 0..9u32 {
                    c.bus.write16(INDEX + 2 * k, (k * 8) as u16);
                    let node = NODES + 0x20 * k;
                    c.bus.write16(node, 1 + (r.next_u32() % 4) as u16);
                    let slot = if k < 8 {
                        1 + r.next_u32() % 2
                    } else {
                        [0, 1, 3][(seed % 3) as usize]
                    };
                    c.bus
                        .write16(node + 2, (r.next_u32() as u16 & 0xffe0) | slot as u16);
                    let branches = if r.next_u32() % 4 == 0 { 0 } else { 1 };
                    c.bus.write32(
                        node + 4,
                        (r.next_u32() & 0x0ff0_0000)
                            | branches << 12
                            | (r.next_u32() & 0xf000_0f00),
                    );
                    c.bus.write32(node + 8, r.next_u32());
                    c.bus.write32(node + 0xc, r.next_u32());
                    c.bus.write8(node + 0x10, 0x80);
                    c.bus.write8(node + 0x11, 0x7f);
                    c.bus.write16(node + 0x12, (r.next_u32() % 8) as u16);
                }
                // node -1 is node 8
                c.bus.write16(INDEX - 2, 8 * 8);
                c.g[A0][0] = 0;
            },
        );
    }

    /// Rider_PlayBumpAnim: main states 0/4/other and sub states 0/1/2/7/
    /// other; front, back and side bumps (goofy or not), slow and fast
    /// impacts, velocity below and above the limit, a small negative /
    /// zero / positive or a large spin; with and without speech. Gated
    /// fakes for the board-press cancel, the anim play, AIrand, the bump
    /// speech and Rider_SetSubState; real state getters and audio instance.
    #[test]
    fn rider_play_bump_anim_matches() {
        use crate::difftest::fakes::{fake, fall_back, hook};
        use crate::game::object::movenode::tests::{oc338_fake, OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        use std::sync::atomic::AtomicUsize;
        oc338_fake!(
            oc374_fake_board_press_cancel,
            OC374_PREV_BOARD_PRESS_CANCEL,
            0x0013_1348,
            ee::s00::f_00131348__generated
        );
        oc338_fake!(
            oc374_fake_anim_play,
            OC374_PREV_ANIM_PLAY,
            0x0031_28e8,
            ee::s10::f_003128e8__generated
        );
        oc338_fake!(
            oc374_fake_ai_rand,
            OC374_PREV_AI_RAND,
            0x0031_7810,
            ee::s10::f_00317810__generated
        );
        oc338_fake!(
            oc374_fake_speech_bump,
            OC374_PREV_SPEECH_BUMP,
            0x002a_0e70,
            ee::s08::f_002a0e70__generated
        );
        oc338_fake!(
            oc374_fake_set_sub_state,
            OC374_PREV_SET_SUB_STATE,
            0x0011_fec8,
            ee::s00::f_0011fec8__generated
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00131348),
            oc374_fake_board_press_cancel,
            &OC374_PREV_BOARD_PRESS_CANCEL,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003128E8),
            oc374_fake_anim_play,
            &OC374_PREV_ANIM_PLAY,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317810),
            oc374_fake_ai_rand,
            &OC374_PREV_AI_RAND,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A0E70),
            oc374_fake_speech_bump,
            &OC374_PREV_SPEECH_BUMP,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011FEC8),
            oc374_fake_set_sub_state,
            &OC374_PREV_SET_SUB_STATE,
        );
        const RIDER: u32 = DATA + 0x1000;
        const BASE: u32 = DATA + 0x3000;
        const INFO: u32 = DATA + 0x5000;
        covered(
            ee::s00::f_00108388__generated,
            rider_play_bump_anim,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_8388, 6);
                crate::difftest::elf_image(c);
                c.pc = 0x0010_8388;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.g[A0][0] = sx(RIDER);
                c.g[A1][0] = sx(INFO);
                c.g[A2][0] = (seed / 2 % 2) as u64;
                c.bus.write32(RIDER + RPBA_BASE, BASE);
                c.bus.write32(RIDER + RPBA_ANIM, DATA + 0x6000);
                let main = [0u32, 0, 0, 4, 4, 2][(seed % 6) as usize];
                let sub = [0u32, 1, 2, 7, 0, 2, 3][(seed / 6 % 7) as usize];
                c.bus.write32(BASE + 0xde0, main);
                c.bus.write32(BASE + 0xde4, sub);
                for i in 0..4 {
                    let f = r.f32(1.0);
                    c.bus.write32(RIDER + RPBA_FORWARD + 4 * i, f);
                    let f = r.f32(1.0);
                    c.bus.write32(RIDER + RPBA_SIDE + 4 * i, f);
                    let f = r.f32(1.0);
                    c.bus.write32(INFO + RPBA_INFO_NORMAL + 4 * i, f);
                    let f = r.f32(1500.0);
                    c.bus.write32(RIDER + RPBA_VELOCITY + 4 * i, f);
                }
                c.bus.write32(RIDER + RPBA_GOOFY, r.next_u32() % 2);
                let speed = if seed % 3 == 0 {
                    r.f32(4000.0)
                } else {
                    r.f32(1100.0)
                };
                c.bus.write32(INFO + RPBA_INFO_SPEED, speed & 0x7fff_ffff);
                let spin = match seed % 5 {
                    0 => 0,
                    1 => r.f32(6.0),
                    2 => r.f32(6.0) | 0x8000_0000,
                    3 => r.f32(6.0) & 0x7fff_ffff,
                    _ => r.f32(100.0),
                };
                c.bus.write32(RIDER + RPBA_SPIN, spin);
            },
        );
    }

    /// WorldObjFactory_ParticleNode: template initialised or not; 0-3
    /// params (or a negative count) with matching / mismatching types
    /// (int to float conversions); the default object or one from the
    /// object table (missing slot, empty entry, found), with and without a
    /// modifier that accepts (0), frees (> 0) or keeps (< 0) the node.
    /// Gated fakes for alloc, delete, the particle-node ctor and
    /// AttachModifier (sign-extended result); real memcpy.
    #[test]
    fn world_obj_factory_particle_node_matches() {
        use crate::difftest::fakes::{fake, fall_back, hook};
        use crate::game::object::movenode::tests::{oc338_fake, OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        use std::sync::atomic::AtomicUsize;
        const BASE: u32 = DATA + 0x20_0000;
        const PARAMS: u32 = BASE;
        const RESULT: u32 = BASE + 0x100;
        const ATTACH_RESULT: u32 = BASE + 0x120;
        const HOLDER: u32 = BASE + 0x400;
        const TABLE_PTR: u32 = BASE + 0x800;
        const TABLE: u32 = BASE + 0x810;
        const SLOTS: u32 = BASE + 0x900;
        const SLOT: u32 = BASE + 0xa00;
        const ENTRIES: u32 = BASE + 0xb00;
        const OBJ: u32 = BASE + 0x1_0000;
        oc338_fake!(
            oc374_fake_wofpn_alloc,
            OC374_PREV_WOFPN_ALLOC,
            0x0031_7d70,
            ee::s10::f_00317d70__generated
        );
        oc338_fake!(
            oc374_fake_wofpn_delete,
            OC374_PREV_WOFPN_DELETE,
            0x0031_7e50,
            ee::s10::f_00317e50__generated
        );
        oc338_fake!(
            oc374_fake_wofpn_ctor,
            OC374_PREV_WOFPN_CTOR,
            0x0035_78a8,
            ee::s11::f_003578a8__generated
        );
        static OC374_PREV_ATTACH: AtomicUsize = AtomicUsize::new(0);
        fn oc374_fake_attach_modifier(c: &mut Cpu) {
            if c.bus.read32(OC338_CTRL) != OC338_MAGIC {
                return fall_back(c, &OC374_PREV_ATTACH, ee::s10::f_002fae38__generated);
            }
            let ra = c.g[RA][0] as u32;
            fake(c, 0x002f_ae38, ra);
            if c.pc == ra {
                c.g[V0][0] = sx(c.bus.read32(ATTACH_RESULT));
            }
        }
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc374_fake_wofpn_alloc,
            &OC374_PREV_WOFPN_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc374_fake_wofpn_delete,
            &OC374_PREV_WOFPN_DELETE,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003578A8),
            oc374_fake_wofpn_ctor,
            &OC374_PREV_WOFPN_CTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_002FAE38),
            oc374_fake_attach_modifier,
            &OC374_PREV_ATTACH,
        );
        covered(
            ee::s10::f_002fd420__generated,
            world_obj_factory_particle_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f_d420, 6);
                c.pc = 0x002f_d420;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let gp = r32(c, GP);
                c.g[A0][0] = sx(RESULT);
                for i in 0..3 {
                    c.bus.write32(WOFPN_RESULT + 4 * i, r.next_u32());
                }
                let initialised = seed % 2;
                c.bus.write32(gp + WOFPN_INIT_GUARD, initialised);
                for off in (0..0xe0).step_by(4) {
                    let f = r.f32(100.0);
                    c.bus.write32(WOFPN_TEMPLATE + off, f);
                }
                for i in 0..56 {
                    c.bus.write32(WOFPN_PARAM_TYPES + 4 * i, r.next_u32() % 4);
                }
                // The object: default (-1) or table entry (id >> 8, slot).
                let which = seed / 2 % 5;
                let id: u32 = match which {
                    0 => u32::MAX,
                    1 => 3,          // slot 3 is empty
                    2 => 2 << 8 | 1, // entry 2 is empty
                    _ => 1 << 8 | 1,
                };
                if initialised == 1 {
                    c.bus.write32(WOFPN_TEMPLATE + WOFPN_NODE_OBJECT, id);
                }
                let n = [0u32, 1, 2, 3, 0xffff_ffff][(seed / 10 % 5) as usize];
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(PARAMS);
                for i in 0..3 {
                    let p = PARAMS + 0x10 * i;
                    let idx = if i == 0 && initialised == 0 {
                        54
                    } else {
                        r.next_u32() % 56
                    };
                    c.bus.write32(p, idx);
                    let value = if idx == 54 { id } else { r.next_u32() };
                    c.bus.write32(p + 4, value);
                    c.bus.write32(p + 8, r.next_u32());
                    let ty = if idx == 54 {
                        c.bus.read32(WOFPN_PARAM_TYPES + 4 * 54)
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(p + 0xc, ty);
                }
                let obj = if seed % 7 == 0 { 0 } else { OBJ };
                c.bus.write32(gp + WOFPN_DEFAULT_HOLDER, HOLDER);
                c.bus.write32(HOLDER + 0x290, obj);
                c.bus.write32(gp + WOFPN_OBJECT_TABLE, TABLE_PTR);
                c.bus.write32(TABLE_PTR, TABLE);
                c.bus.write32(TABLE + 8, SLOTS);
                for i in 0..4 {
                    c.bus.write32(SLOTS + 4 * i, if i == 3 { 0 } else { SLOT });
                }
                c.bus.write32(SLOT + 0x1c, ENTRIES);
                c.bus
                    .write32(ENTRIES + 4, (OBJ >> 2) << 8 | (r.next_u32() & 0xff));
                c.bus.write32(ENTRIES + 8, r.next_u32() & 0xff);
                let modifier = if seed % 3 == 0 { 0 } else { DATA + 0x30_0000 };
                c.bus.write32(OBJ + 0xc, modifier);
                let attach = [0u32, 0, 1, 0xffff_ffff][(seed / 3 % 4) as usize];
                c.bus.write32(ATTACH_RESULT, attach);
            },
        );
    }

    /// SSXAudioSystem_SpeechRiderCollision: every early out (speech off,
    /// wrong mode, blocked, a or b finished, a or b may not speak, both
    /// checked riders without the player, instance not ready, neither in
    /// range, p7 / p8), then rivalry 2 / 1 / 0 / other with each event
    /// context found or not. All callees are scripted fakes (gated; a
    /// result list per callee, sign-extended); PlayRiderGrunt runs for
    /// real and stops at its own speech-volume check.
    #[test]
    fn ssx_audio_system_speech_rider_collision_matches() {
        use crate::difftest::fakes::{fake, fall_back, hook};
        use crate::game::object::movenode::tests::{OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        use std::sync::atomic::AtomicUsize;
        const SELF: u32 = DATA + 0x21_0000;
        const RIDER_A: u32 = DATA + 0x22_0000;
        const RIDER_B: u32 = DATA + 0x23_0000;
        const VTABLE: u32 = DATA + 0x24_0000;
        const HOLDER: u32 = DATA + 0x24_1000;
        /// Vtable methods and the announce hook (at `FAKE + 0x10 * k`).
        fn oc374_ssrc_miss(c: &mut Cpu, target: u32) {
            let slot = 11 + (target.wrapping_sub(FAKE) / 0x10).min(4);
            oc374_scripted(c, target, slot)
        }
        oc374_scripted_fake!(
            oc374_ssrc_volume_on,
            OC374_PREV_SSRC_0,
            0x0029_f0a0,
            0,
            ee::s08::f_0029f0a0__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_blocked,
            OC374_PREV_SSRC_1,
            0x0028_8ae0,
            1,
            ee::s07::f_00288ae0__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_finished,
            OC374_PREV_SSRC_2,
            0x0012_3168,
            2,
            ee::s00::f_00123168__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_player,
            OC374_PREV_SSRC_3,
            0x0028_5d98,
            3,
            ee::s07::f_00285d98__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_ready,
            OC374_PREV_SSRC_4,
            0x002b_0e28,
            4,
            ee::s08::f_002b0e28__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_in_range,
            OC374_PREV_SSRC_5,
            0x0028_8b40,
            5,
            ee::s07::f_00288b40__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_mask,
            OC374_PREV_SSRC_6,
            0x0029_f330,
            6,
            ee::s08::f_0029f330__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_rivalry,
            OC374_PREV_SSRC_7,
            0x002a_1820,
            7,
            ee::s08::f_002a1820__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_name,
            OC374_PREV_SSRC_8,
            0x0012_3128,
            8,
            ee::s00::f_00123128__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_context,
            OC374_PREV_SSRC_9,
            0x002b_1458,
            9,
            ee::s08::f_002b1458__generated
        );
        oc374_scripted_fake!(
            oc374_ssrc_find,
            OC374_PREV_SSRC_10,
            0x003d_8008,
            10,
            ee::s14::f_003d8008__generated
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F0A0),
            oc374_ssrc_volume_on,
            &OC374_PREV_SSRC_0,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288AE0),
            oc374_ssrc_blocked,
            &OC374_PREV_SSRC_1,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123168),
            oc374_ssrc_finished,
            &OC374_PREV_SSRC_2,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285D98),
            oc374_ssrc_player,
            &OC374_PREV_SSRC_3,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B0E28),
            oc374_ssrc_ready,
            &OC374_PREV_SSRC_4,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00288B40),
            oc374_ssrc_in_range,
            &OC374_PREV_SSRC_5,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F330),
            oc374_ssrc_mask,
            &OC374_PREV_SSRC_6,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A1820),
            oc374_ssrc_rivalry,
            &OC374_PREV_SSRC_7,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123128),
            oc374_ssrc_name,
            &OC374_PREV_SSRC_8,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B1458),
            oc374_ssrc_context,
            &OC374_PREV_SSRC_9,
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D8008),
            oc374_ssrc_find,
            &OC374_PREV_SSRC_10,
        );
        covered(
            ee::s08::f_002a0a30__generated,
            ssx_audio_system_speech_rider_collision,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_0a30, 12);
                c.hooks.miss = oc374_ssrc_miss;
                c.pc = 0x002a_0a30;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let gp = r32(c, GP);
                let script = oc374_script;
                // Mostly pass the early outs.
                let early = if seed % 4 == 0 { r.next_u32() % 12 } else { 99 };
                script(c, 0, &[(early != 0) as u32, 0]); // ours, the grunt's
                c.bus.write32(gp.wrapping_sub(SSRC_MODE_HOLDER), HOLDER);
                c.bus
                    .write32(HOLDER + 0x550, if early == 1 { 3 } else { 2 });
                script(c, 1, &[(early == 2) as u32]);
                script(c, 2, &[(early == 3) as u32, (early == 4) as u32]);
                let pitch_a = if early == 5 {
                    r.f32(1.0) & 0x7fff_ffff
                } else {
                    r.f32(1.0) | 0x8000_0000
                };
                let pitch_b = if early == 6 {
                    0
                } else {
                    r.f32(1.0) | 0x8000_0000
                };
                c.bus.write32(RIDER_A + SSRC_RIDER_PITCH, pitch_a);
                c.bus.write32(RIDER_B + SSRC_RIDER_PITCH, pitch_b);
                for rider in [RIDER_A, RIDER_B] {
                    c.bus.write32(rider + SSRC_RIDER_OBJECT, VTABLE);
                    c.bus.write32(rider + SSRC_RIDER_HUMAN, r.next_u32() % 2);
                }
                c.bus.write16(VTABLE + 0x48, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(VTABLE + 0x4c, FAKE);
                c.bus.write16(VTABLE + 0x28, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(VTABLE + 0x2c, FAKE + 0x10);
                script(c, 11, &[r.next_u32() % 2, r.next_u32() % 2]); // is AI
                script(c, 12, &[r.next_u32() % 40, r.next_u32() % 40]); // ids
                let player = [RIDER_A, RIDER_B, DATA + 0x25_0000][(r.next_u32() % 3) as usize];
                script(c, 3, &[player]);
                script(c, 4, &[(early != 7) as u32]);
                let range = |r: &mut Rng| [u32::MAX, 0, 5][(r.next_u32() % 3) as usize];
                let (rb, ra) = if early == 8 {
                    (u32::MAX, u32::MAX)
                } else {
                    (range(&mut r), range(&mut r))
                };
                script(c, 5, &[rb, ra]);
                script(c, 6, &[r.next_u32(), r.next_u32()]);
                script(c, 7, &[[2u32, 1, 0, 3][(seed / 4 % 4) as usize]]);
                script(c, 8, &[r.next_u32(), r.next_u32()]);
                script(c, 9, &[r.next_u32() % 2, r.next_u32() % 2]);
                script(c, 10, &[r.next_u32(), r.next_u32()]);
                script(c, 13, &[0]); // the announce hook (result unused)
                c.bus.write32(gp + SSRC_ANNOUNCE, FAKE + 0x20);
                c.bus.write32(gp + SSRC_SPEECH_DIST, r.f32(100.0));
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(RIDER_A);
                c.g[A2][0] = sx(RIDER_B);
                c.g[A3][0] = (early == 9) as u64; // p8
                c.g[T0][0] = (early == 10) as u64; // p7
            },
        );
    }

    /// RiderWipeoutSub_UpdatePhase1: on the ground hit (airborne or not):
    /// get up (up > 0.9 and pushed into the normal), the final anim (0x1d
    /// first or second query, 0x19) or anim set A; not hit: layer finished
    /// or final anim playing (or neither: nothing), then recovery by the
    /// timer or the rider speed (player: reset check), else set A on the
    /// ground or set B when the layer finished. Gated fakes (anim current
    /// and "finished" scripted); the wipeout helpers without override
    /// slots (StartGetUp, PlayFinalAnim, PlayEndAnimB, SwitchToAnimSetB)
    /// run for real on faked anim calls.
    #[test]
    fn rider_wipeout_sub_update_phase1_matches() {
        use crate::difftest::fakes::hook;
        use crate::game::object::movenode::tests::{OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        const SELF: u32 = DATA + 0x20_0000;
        const RIDER: u32 = DATA + 0x21_0000;
        const BASE: u32 = DATA + 0x22_0000;
        const VTABLE: u32 = DATA + 0x22_1000;
        oc374_scripted_fake!(
            oc374_rws_anim_speed,
            OC374_PREV_RWS_16,
            0x0012_e528,
            16,
            ee::s00::f_0012e528__generated
        );
        oc374_scripted_fake!(
            oc374_rws_current,
            OC374_PREV_RWS_17,
            0x0031_1ae8,
            17,
            ee::s10::f_00311ae8__generated
        );
        oc374_scripted_fake!(
            oc374_rws_time,
            OC374_PREV_RWS_18,
            0x0031_2ab0,
            18,
            ee::s10::f_00312ab0__generated
        );
        oc374_scripted_fake!(
            oc374_rws_layer,
            OC374_PREV_RWS_19,
            0x0031_1b20,
            19,
            ee::s10::f_00311b20__generated
        );
        oc374_scripted_fake!(
            oc374_rws_set_speed,
            OC374_PREV_RWS_20,
            0x0031_3cf0,
            20,
            ee::s10::f_00313cf0__generated
        );
        oc374_scripted_fake!(
            oc374_rws_finished,
            OC374_PREV_RWS_21,
            0x0031_2ae8,
            21,
            ee::s10::f_00312ae8__generated
        );
        oc374_scripted_fake!(
            oc374_rws_sound_final,
            OC374_PREV_RWS_22,
            0x0029_6868,
            22,
            ee::s08::f_00296868__generated
        );
        oc374_scripted_fake!(
            oc374_rws_sound_land,
            OC374_PREV_RWS_23,
            0x0029_6e80,
            23,
            ee::s08::f_00296e80__generated
        );
        oc374_scripted_fake!(
            oc374_rws_sound_end,
            OC374_PREV_RWS_24,
            0x0029_6310,
            24,
            ee::s08::f_00296310__generated
        );
        oc374_scripted_fake!(
            oc374_rws_set_a,
            OC374_PREV_RWS_25,
            0x0012_dd98,
            25,
            ee::s00::f_0012dd98__generated
        );
        oc374_scripted_fake!(
            oc374_rws_boost,
            OC374_PREV_RWS_26,
            0x0010_f280,
            26,
            ee::s00::f_0010f280__generated
        );
        oc374_scripted_fake!(
            oc374_rws_reset,
            OC374_PREV_RWS_27,
            0x0011_6120,
            27,
            ee::s00::f_00116120__generated
        );
        oc374_scripted_fake!(
            oc374_rws_play,
            OC374_PREV_RWS_28,
            0x0031_28e8,
            28,
            ee::s10::f_003128e8__generated
        );
        oc374_scripted_fake!(
            oc374_rws_312aa0,
            OC374_PREV_RWS_29,
            0x0031_2aa0,
            29,
            ee::s10::f_00312aa0__generated
        );
        oc374_scripted_fake!(
            oc374_rws_312660,
            OC374_PREV_RWS_30,
            0x0031_2660,
            30,
            ee::s10::f_00312660__generated
        );
        oc374_scripted_fake!(
            oc374_rws_311bf0,
            OC374_PREV_RWS_31,
            0x0031_1bf0,
            31,
            ee::s10::f_00311bf0__generated
        );
        oc374_scripted_fake!(
            oc374_rws_30ecd8,
            OC374_PREV_RWS_32,
            0x0030_ecd8,
            32,
            ee::s10::f_0030ecd8__generated
        );
        oc374_scripted_fake!(
            oc374_rws_world_matrix,
            OC374_PREV_RWS_33,
            0x0011_e098,
            33,
            ee::s00::f_0011e098__generated
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012E528),
            oc374_rws_anim_speed,
            &OC374_PREV_RWS_16,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
            oc374_rws_current,
            &OC374_PREV_RWS_17,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00312AB0),
            oc374_rws_time,
            &OC374_PREV_RWS_18,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00311B20),
            oc374_rws_layer,
            &OC374_PREV_RWS_19,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00313CF0),
            oc374_rws_set_speed,
            &OC374_PREV_RWS_20,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00312AE8),
            oc374_rws_finished,
            &OC374_PREV_RWS_21,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_00296868),
            oc374_rws_sound_final,
            &OC374_PREV_RWS_22,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_00296E80),
            oc374_rws_sound_land,
            &OC374_PREV_RWS_23,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_00296310),
            oc374_rws_sound_end,
            &OC374_PREV_RWS_24,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012DD98),
            oc374_rws_set_a,
            &OC374_PREV_RWS_25,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0010F280),
            oc374_rws_boost,
            &OC374_PREV_RWS_26,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00116120),
            oc374_rws_reset,
            &OC374_PREV_RWS_27,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003128E8),
            oc374_rws_play,
            &OC374_PREV_RWS_28,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00312AA0),
            oc374_rws_312aa0,
            &OC374_PREV_RWS_29,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00312660),
            oc374_rws_312660,
            &OC374_PREV_RWS_30,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00311BF0),
            oc374_rws_311bf0,
            &OC374_PREV_RWS_31,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030ECD8),
            oc374_rws_30ecd8,
            &OC374_PREV_RWS_32,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011E098),
            oc374_rws_world_matrix,
            &OC374_PREV_RWS_33,
        );
        covered(
            ee::s00::f_0012d4e8__generated,
            rider_wipeout_sub_update_phase1,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12_d4e8, 8);
                crate::difftest::elf_image(c);
                c.pc = 0x0012_d4e8;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.g[A0][0] = sx(SELF);
                for off in (0..0x80).step_by(4) {
                    c.bus.write32(SELF + off, r.next_u32());
                }
                c.bus.write32(SELF + RWS_HIT, seed % 2);
                for i in 0..4 {
                    let f = r.f32(50.0);
                    c.bus.write32(SELF + RWS_IMPACT + 4 * i, f);
                    let f = r.f32(1.0);
                    c.bus.write32(RIDER + RWS_RIDER_NORMAL + 4 * i, f);
                    let f = r.f32(40.0);
                    c.bus.write32(RIDER + RWS_RIDER_VELOCITY + 4 * i, f);
                }
                let timer = if r.next_u32() % 2 == 0 { 0.5f32 } else { 1.5 };
                c.bus.write32(SELF + RWS_TIMER, timer.to_bits());
                c.bus.write32(SELF + RWS_RIDER, RIDER);
                c.bus.write32(RIDER + RWS_RIDER_BASE, BASE);
                c.bus.write32(BASE + 0x30, (seed / 2 % 3 == 0) as u32); // airborne
                c.bus.write32(RIDER + RWS_RIDER_ANIM, DATA + 0x23_0000);
                let up = if seed % 3 == 0 { 0.5f32 } else { 0.95 };
                c.bus.write32(RIDER + RWS_RIDER_UP, up.to_bits());
                c.bus.write32(RIDER + RWS_RIDER_SPEED, r.f32(10.0));
                c.bus.write32(RIDER + RWS_RIDER_PLAYER, r.next_u32() % 2);
                c.bus.write32(RIDER + RWS_RIDER_OBJECT, VTABLE);
                c.bus.write16(VTABLE + 0x88, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(VTABLE + 0x8c, FAKE);
                let id = |r: &mut Rng| [0x1du32, 0x19, 5][(r.next_u32() % 3) as usize];
                let ids = [id(&mut r), id(&mut r)];
                oc374_script(c, 17, &ids);
                let finished = [r.next_u32() % 2, r.next_u32() % 2];
                oc374_script(c, 21, &finished);
            },
        );
    }

    /// WorldState_Write: the straight-line save; every callee a gated
    /// fake (override slots, the stream / object vtables at `FAKE`).
    #[test]
    fn world_state_write_matches() {
        use crate::difftest::fakes::hook;
        use crate::game::object::movenode::tests::{OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x21_0000;
        const WORLD: u32 = DATA + 0x22_0000;
        const OBJECTS: u32 = DATA + 0x22_1000;
        const PB_WORLD: u32 = DATA + 0x22_2000;
        const VTABLE: u32 = DATA + 0x22_3000;
        oc374_scripted_fake!(
            oc374_wsw_rewind,
            OC374_PREV_WSW_40,
            0x0026_e9c0,
            40,
            ee::s07::f_0026e9c0__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_time,
            OC374_PREV_WSW_41,
            0x0031_78e0,
            41,
            ee::s10::f_003178e0__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_get_be,
            OC374_PREV_WSW_42,
            0x0014_dc80,
            42,
            ee::s01::f_0014dc80__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_be_save,
            OC374_PREV_WSW_43,
            0x0014_df08,
            43,
            ee::s01::f_0014df08__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_2d9cb0,
            OC374_PREV_WSW_44,
            0x002d_9cb0,
            44,
            ee::s09::f_002d9cb0__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_objects,
            OC374_PREV_WSW_45,
            0x0030_bb10,
            45,
            ee::s10::f_0030bb10__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_cameras,
            OC374_PREV_WSW_46,
            0x0022_9e20,
            46,
            ee::s05::f_00229e20__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_effects,
            OC374_PREV_WSW_47,
            0x0034_41a8,
            47,
            ee::s11::f_003441a8__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_particles,
            OC374_PREV_WSW_48,
            0x0035_7ca8,
            48,
            ee::s11::f_00357ca8__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_write_objects,
            OC374_PREV_WSW_49,
            0x0026_d988,
            49,
            ee::s07::f_0026d988__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_write_dead,
            OC374_PREV_WSW_50,
            0x0026_da88,
            50,
            ee::s07::f_0026da88__generated
        );
        oc374_scripted_fake!(
            oc374_wsw_riders,
            OC374_PREV_WSW_51,
            0x0010_3480,
            51,
            ee::s00::f_00103480__generated
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E9C0),
            oc374_wsw_rewind,
            &OC374_PREV_WSW_40,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003178E0),
            oc374_wsw_time,
            &OC374_PREV_WSW_41,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc374_wsw_get_be,
            &OC374_PREV_WSW_42,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DF08),
            oc374_wsw_be_save,
            &OC374_PREV_WSW_43,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9CB0),
            oc374_wsw_2d9cb0,
            &OC374_PREV_WSW_44,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030BB10),
            oc374_wsw_objects,
            &OC374_PREV_WSW_45,
        );
        hook(
            addr_of_mut!(ee::s05::OVERRIDE_00229E20),
            oc374_wsw_cameras,
            &OC374_PREV_WSW_46,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_003441A8),
            oc374_wsw_effects,
            &OC374_PREV_WSW_47,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_00357CA8),
            oc374_wsw_particles,
            &OC374_PREV_WSW_48,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D988),
            oc374_wsw_write_objects,
            &OC374_PREV_WSW_49,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026DA88),
            oc374_wsw_write_dead,
            &OC374_PREV_WSW_50,
        );
        hook(
            addr_of_mut!(ee::s00::OVERRIDE_00103480),
            oc374_wsw_riders,
            &OC374_PREV_WSW_51,
        );
        covered(
            ee::s07::f_0026d818__generated,
            world_state_write,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26_d818, 20);
                c.pc = 0x0026_d818;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let gp = r32(c, GP);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(STREAM);
                c.bus.write32(STREAM, VTABLE);
                c.bus.write16(VTABLE + 8, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(VTABLE + 0xc, FAKE);
                c.bus.write16(VTABLE + 0x40, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(VTABLE + 0x44, FAKE + 0x10);
                c.bus.write16(VTABLE + 0x38, (r.next_u32() % 4 * 4) as u16);
                c.bus.write32(VTABLE + 0x3c, FAKE + 0x20);
                c.bus.write32(gp + WSW_COUNTER_A, r.next_u32());
                c.bus.write32(gp + WSW_COUNTER_B, r.next_u32());
                c.bus.write32(gp.wrapping_sub(WSW_WORLD), WORLD);
                c.bus.write32(WORLD + 0x84, OBJECTS);
                c.bus.write32(OBJECTS, VTABLE);
                c.bus.write32(OBJECTS + 0xc, PB_WORLD);
                c.bus.write32(PB_WORLD + 0xcc, VTABLE);
                c.bus.write32(PB_WORLD + 0xa4, r.next_u32());
                for g in [WSW_OBJECT_HOLDER, WSW_EFFECT_MAN, WSW_PARTICLE_MAN] {
                    c.bus.write32(gp + g, r.next_u32());
                }
                c.bus.write32(gp.wrapping_sub(WSW_CAMERA_MAN), r.next_u32());
            },
        );
    }

    /// SpatialHash_Insert: an empty root (new node; list 0-2, or 3 whose
    /// "head" is a live item) or a root at a level below, at or above the
    /// item's cell that contains it at once or grows 1-5 levels (coordinates
    /// 0..16, so the growth ends). Gated fakes for the allocator (the
    /// first node forced to a known address) and InsertAtKey; real MEM_fill.
    #[test]
    fn spatial_hash_insert_matches() {
        use crate::difftest::fakes::{hook, LOG_RESULTS};
        use crate::game::object::movenode::tests::{OC338_CTRL, OC338_MAGIC};
        use std::ptr::addr_of_mut;
        const ROOT: u32 = DATA + 0x20_0000;
        const CELL: u32 = DATA + 0x20_0100;
        const ITEM: u32 = DATA + 0x20_0200;
        const OTHER: u32 = DATA + 0x20_0300;
        const NODE: u32 = DATA + 0x20_1000;
        oc374_scripted_fake!(
            oc374_shi_alloc,
            OC374_PREV_SHI_52,
            0x0031_7d70,
            52,
            ee::s10::f_00317d70__generated
        );
        oc374_scripted_fake!(
            oc374_shi_insert_at_key,
            OC374_PREV_SHI_53,
            0x0032_8660,
            53,
            ee::s10::f_00328660__generated
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc374_shi_alloc,
            &OC374_PREV_SHI_52,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328660),
            oc374_shi_insert_at_key,
            &OC374_PREV_SHI_53,
        );
        covered(
            ee::s10::f_00328c20__generated,
            spatial_hash_insert,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_8c20, 4);
                c.pc = 0x0032_8c20;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                c.bus.write32(LOG_RESULTS, NODE);
                for off in (0..0x40).step_by(4) {
                    c.bus.write32(NODE + off, r.next_u32());
                }
                c.bus.write32(NODE + 0x2c, OTHER);
                let empty = seed % 4 == 0;
                let list = if empty && seed % 8 == 4 {
                    3
                } else {
                    r.next_u32() % 3
                };
                c.g[A0][0] = sx(ROOT);
                c.g[A1][0] = list as u64;
                c.g[A2][0] = sx(ITEM);
                c.g[A3][0] = sx(CELL);
                let level = r.next_u32() % 3;
                let xyz = [r.next_u32() % 16, r.next_u32() % 16, r.next_u32() % 16];
                c.bus.write32(CELL, level);
                for i in 0..3 {
                    c.bus.write32(CELL + 4 + 4 * i, xyz[i as usize]);
                }
                c.bus.write32(ITEM, r.next_u32());
                c.bus.write32(ITEM + 4, r.next_u32());
                if empty {
                    c.bus.write32(ROOT + SHI_ROOT_NODE, 0);
                    for i in 0..4 {
                        c.bus.write32(ROOT + 4 * i, r.next_u32());
                    }
                } else {
                    c.bus.write32(ROOT + SHI_ROOT_NODE, DATA + 0x20_2000);
                    // An ancestor of the cell (contains it), or any cell nearby.
                    let up = r.next_u32() % 3;
                    let root_level = if seed % 3 == 0 {
                        level + up
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(ROOT, root_level);
                    for i in 0..3u32 {
                        let v = if seed % 3 == 0 {
                            xyz[i as usize] >> up
                        } else {
                            r.next_u32() % 16
                        };
                        c.bus.write32(ROOT + 4 + 4 * i, v);
                    }
                }
            },
        );
    }
}
