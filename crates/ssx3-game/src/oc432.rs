//! Staging module oc432 (wave wave221): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc432`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_7890, ai_fwd_diff_cache_init, "AIFwdDiffCache_Init"),
    (0x0022_8a78, ssx_app_load_locale, "SSXApp_LoadLocale"),
    (
        0x0023_4f40,
        game_state_reset_overlay_enter,
        "GameStateReset_OverlayEnter",
    ),
    (
        0x0027_8b98,
        script_engine_load_world_scripts,
        "ScriptEngine_LoadWorldScripts",
    ),
    (0x0027_2788, in_seq_ctor, "InSeq_ctor"),
    (0x0027_7838, script_list_next_entry, "ScriptList_NextEntry"),
    (0x0030_9f18, wscript_man_shutdown, "WScriptMan_Shutdown"),
    (0x0028_3c80, script_cmd_queue_push, "ScriptCmdQueue_Push"),
    (
        0x0023_8160,
        game_mode_man_start_mode,
        "GameModeMan_StartMode",
    ),
    (0x0035_49e0, process_list_init, "ProcessList_Init"),
    (
        0x0016_bf40,
        camera_trigger_man_free_triggers,
        "CameraTriggerMan_FreeTriggers",
    ),
    (
        0x0016_c228,
        camera_trigger_man_parse,
        "CameraTriggerMan_Parse",
    ),
    (0x0022_cd40, section_man_init, "SectionMan_Init"),
    (
        0x0023_a4f0,
        gm_handler1_pick_opponents,
        "GMHandler1_PickOpponents",
    ),
    (
        0x0012_e468,
        rider_wipeout_sub_play_final_anim,
        "RiderWipeoutSub_PlayFinalAnim",
    ),
    (
        0x0017_6448,
        camera_bound_obj_get_rot_x,
        "CameraBoundObj_GetRotX",
    ),
    (
        0x0017_6450,
        camera_bound_obj_get_rot_y,
        "CameraBoundObj_GetRotY",
    ),
    (
        0x003a_9000,
        cworld_cache_streamer_attach_sdb,
        "cWorldCache_StreamerAttachSdb",
    ),
];

/// `float& cCTObject::heading()` / `CameraBoundObj_GetRotX` (0x176448):
/// returns `this + 0xc`.
pub fn camera_bound_obj_get_rot_x(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0xc));
    ret(c);
}

/// `float& cCTObject::pitch()` / `CameraBoundObj_GetRotY` (0x176450):
/// returns `this + 0x10`.
pub fn camera_bound_obj_get_rot_y(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x10));
    ret(c);
}

/// `bool cWorldCache::StreamerAttachSdb(self, tStreamDataHeader* hdr, int id)`
/// (0x3a9000, `cWorldView_init`): points the streamer at a world cache
/// header. Stores id/hdr, lays out the section table (`hdr + 0x50`, 0x58
/// bytes each), the node table (16-aligned after it, 0x60 bytes each) and the
/// leaf table (0x44 bytes each); rebases every section's node index and every
/// node's left/right/leaf index (negative -> 0) into pointers, clears the
/// per-section pairs and copies the first section's 32-byte bounds to
/// `self + 0x220`. Returns 1.
pub fn cworld_cache_streamer_attach_sdb(c: &mut Cpu) {
    const HDR_SECTS: u32 = 0x8;
    const HDR_NODES: u32 = 0xc;
    const SECT_NODE: u32 = 0x1c;
    const NODE_LEFT: u32 = 0x50;
    const NODE_RIGHT: u32 = 0x54;
    const NODE_LEAF: u32 = 0x58;
    c.cycles += 22;
    let this = r32(c, A0);
    let hdr = r32(c, A1);
    c.g[T1][0] = c.g[A0][0];
    c.g[A3][0] = sx(hdr.wrapping_add(0x50));
    c.sw(this, c.g[A2][0]);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.sw(this.wrapping_add(0x4), c.g[A1][0]);
    c.g[A2][0] = 0x58;
    c.sw(this.wrapping_add(0x8), c.g[A3][0]);
    c.g[V1][0] |= 0xfff0;
    c.g[T0][0] = 0x60;
    c.g[A0][0] = c.lw(hdr.wrapping_add(HDR_SECTS));
    c.mult(0, V0, A0, A2);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(0xf));
    c.g[A0][0] = c.g[V0][0] & c.g[V1][0];
    c.sw(this.wrapping_add(0xc), c.g[A0][0]);
    c.g[V0][0] = c.lw(hdr.wrapping_add(HDR_NODES));
    c.mult(0, V1, V0, T0);
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.sw(this.wrapping_add(0x10), c.g[V0][0]);
    c.g[V1][0] = c.lw(hdr.wrapping_add(HDR_SECTS));
    c.g[T2][0] = 0;
    // Sections: node index -> node pointer, pair slots cleared.
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[A3][0] = 0x60;
        c.g[A1][0] = sx(this.wrapping_add(0x18));
        c.g[A2][0] = 0;
        loop {
            c.cycles += 16;
            c.g[A0][0] = c.lw(this.wrapping_add(0x8));
            c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
            c.g[V1][0] = c.lw(this.wrapping_add(0xc));
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(SECT_NODE));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x58));
            c.mult(0, V0, V0, A3);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.sw(r32(c, A0).wrapping_add(SECT_NODE), c.g[V1][0]);
            c.sw(r32(c, A1).wrapping_sub(0x4), 0);
            c.sw(r32(c, A1), 0);
            c.g[V1][0] = c.lw(this.wrapping_add(0x4));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(HDR_SECTS));
            c.g[V0][0] = (c.g[T2][0] < c.g[V0][0]) as u64;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x8));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_9068);
            }
        }
    }
    // Nodes: left/right/leaf index -> pointer or 0 when negative.
    c.cycles += 4;
    c.g[V1][0] = c.lw(this.wrapping_add(0x4));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(HDR_NODES));
    c.g[T2][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[T3][0] = 0x60;
        c.g[T0][0] = 0x44;
        c.g[A3][0] = 0;
        c.g[A2][0] = 0;
        loop {
            // left (nodes[i].left, scaled by 0x60)
            c.cycles += 5;
            c.g[A0][0] = c.lw(this.wrapping_add(0xc));
            c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(NODE_LEFT));
            let neg = (c.g[V0][0] as i64) < 0;
            c.mult(0, A1, V0, T3);
            if neg {
                c.cycles += 1;
                c.sw(r32(c, V1).wrapping_add(NODE_LEFT), 0);
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                c.sw(r32(c, V1).wrapping_add(NODE_LEFT), c.g[V0][0]);
            }
            // right
            c.cycles += 7;
            c.g[A1][0] = 0x60;
            c.g[A0][0] = c.lw(this.wrapping_add(0xc));
            c.mult(0, V0, T2, A1);
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(NODE_RIGHT));
            let neg = (c.g[V0][0] as i64) < 0;
            c.mult(0, T4, V0, A1);
            if neg {
                c.cycles += 1;
                c.sw(r32(c, V1).wrapping_add(NODE_RIGHT), 0);
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx(r32(c, T4).wrapping_add(r32(c, A0)));
                c.sw(r32(c, V1).wrapping_add(NODE_RIGHT), c.g[V0][0]);
            }
            // leaf (scaled by 0x44, base = self.leaves)
            c.cycles += 5;
            c.g[V0][0] = c.lw(this.wrapping_add(0xc));
            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(NODE_LEAF));
            let neg = (c.g[V1][0] as i64) < 0;
            c.mult(0, V1, V1, T0);
            if neg {
                c.cycles += 1;
                c.sw(r32(c, A0).wrapping_add(NODE_LEAF), 0);
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw(this.wrapping_add(0x10));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.sw(r32(c, A0).wrapping_add(NODE_LEAF), c.g[V0][0]);
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw(this.wrapping_add(0x4));
            c.g[T2][0] = sx(r32(c, T2).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x60));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(HDR_NODES));
            c.g[V0][0] = (c.g[T2][0] < c.g[V0][0]) as u64;
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x60));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003a_90c8);
            }
        }
    }
    // The bounds of the first section's root node.
    c.cycles += 9;
    c.g[A1][0] = c.lw(this.wrapping_add(0x8));
    c.g[V0][0] = 1;
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(SECT_NODE));
    c.lq(V1, r32(c, A0));
    c.sq(V1, this.wrapping_add(0x220));
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(SECT_NODE));
    c.lq(V1, r32(c, A0).wrapping_add(0x10));
    c.sq(V1, this.wrapping_add(0x230));
    ret(c);
}

/// `void RiderWipeoutSub_PlayFinalAnim(self)` (0x12e468,
/// `cWipeoutControl_playWipeoutGroundImpactAnim`): looks at the rider's
/// current anim on layer 2 (`rider->animBase`: `*(*(self + 0x80) + 0x784)`)
/// and, for ids 0x17b..=0x17f (a `jr` table at 0x458180), plays the matching
/// follow-up anim 0x180..=0x184 (`PlayAnim(base, id, 0, -1.0f)`). Other ids
/// do nothing. Frame 0x20 (s0).
pub fn rider_wipeout_sub_play_final_anim(c: &mut Cpu) {
    const RIDER: u32 = 0x80;
    const ANIM_BASE: u32 = 0x784;
    if c.pc != 0x0012_e468 {
        return ee::s00::f_0012e468__generated(c);
    }
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s0 = r32(c, S0);
        c.g[V0][0] = c.lw(s0.wrapping_add(RIDER));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(ANIM_BASE));
        call_ptr(c, 0x0031_2aa0, 0x0012_e488)?;
        c.g[V1][0] = sx(r32(c, V0).wrapping_sub(0x17b));
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x46_0000;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x7e80));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            // (cycles before the shared tail, anim id, return address of the
            // play call, last case). The last case (0x184) carries its own
            // 7 cycles and one more after the call.
            let (pre_cycles, id, play_ra, last) = match target {
                0x0012_e4b0 => (3, 0x180, 0x0012_e4f0, false),
                0x0012_e4bc => (3, 0x181, 0x0012_e4f0, false),
                0x0012_e4c8 => (3, 0x182, 0x0012_e4f0, false),
                0x0012_e4d4 => (2, 0x183, 0x0012_e4f0, false),
                0x0012_e4f8 => (7, 0x184, 0x0012_e514, true),
                _ => {
                    c.jump_out(target, 0x0012_e468, 0x0012_e524);
                    return Ok(());
                }
            };
            c.cycles += pre_cycles;
            c.g[V0][0] = c.lw(s0.wrapping_add(RIDER));
            c.g[A1][0] = id;
            if !last {
                c.cycles += 7;
            }
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0;
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(ANIM_BASE));
            call_ptr(c, 0x0031_28e8, play_ra)?;
            if last {
                c.cycles += 1;
            }
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GMHandler1_PickOpponents(self)` (0x23a4f0): picks the opponents of
/// a single-player event. Asks the BE for the player (interface 1) and race
/// (interface 0) objects, the human rider's character (`charA`) and the peak
/// rival's character (`charB`), then builds `arr[10]` on the stack:
/// `arr[0] = GMrand() % 7 + 10`, `arr[1] = GMrand() % 6 + 10` (+1 when not
/// below `arr[0]`), `arr[2..]` = the characters 0..9 except `charA`/`charB`
/// (while there is room), shuffles the array (`GameRand_Shuffle(arr, 10)`)
/// and writes charA and the array's entries into the handler's slots
/// (+0x10, +0x14..+0x24, +0x34..+0x3c, +0x50, +0x54). Frame 0x70 (s0-s2).
pub fn gm_handler1_pick_opponents(c: &mut Cpu) {
    const GET_BE: u32 = 0x0014_dc80;
    const GET_INTERFACE: u32 = 0x0014_dd58;
    const GET_HUMAN_RIDER: u32 = 0x0014_6e98;
    const GET_CHARACTER: u32 = 0x0014_7398;
    const GET_PEAK_RIVAL: u32 = 0x0014_5750;
    const RAND: Callee = Callee {
        addr: 0x0023_7cd8,
        f: ee::s06::f_00237cd8,
    };
    const SHUFFLE: Callee = Callee {
        addr: 0x0023_c770,
        f: ee::s06::f_0023c770,
    };
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x30);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, GET_BE, 0x0023_a50c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, GET_INTERFACE, 0x0023_a518)?;
        c.g[S0][0] = c.g[V0][0]; // player interface
        call_ptr(c, GET_BE, 0x0023_a520)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, GET_INTERFACE, 0x0023_a52c)?;
        c.g[S1][0] = c.g[V0][0]; // race interface
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, GET_HUMAN_RIDER, 0x0023_a53c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, GET_CHARACTER, 0x0023_a548)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0]; // charA
        call_ptr(c, GET_PEAK_RIVAL, 0x0023_a554)?;
        c.g[S1][0] = c.g[V0][0]; // charB

        RAND.call(c, 0x0023_a55c)?;
        c.g[V1][0] = 7;
        c.divu(0, V0, V1);
        c.cycles += 8;
        c.g[A0][0] = c.hi[0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xa));
        let arr = r32(c, SP);
        c.sw(arr, c.g[A0][0]);
        RAND.call(c, 0x0023_a57c)?;
        c.g[V1][0] = 6;
        c.divu(0, V0, V1);

        c.cycles += 6;
        c.g[V0][0] = c.lw(arr);
        c.g[A0][0] = c.hi[0];
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0xa));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let below = c.g[V0][0] != 0;
        c.sw(arr.wrapping_add(4), c.g[V1][0]);
        if !below {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(0xb));
            c.sw(arr.wrapping_add(4), c.g[V0][0]);
        }

        // arr[n + 2] = i for every character i in 0..10 other than charA and
        // charB, while n + 2 < 10.
        c.cycles += 3;
        c.g[A1][0] = 0;
        c.g[V1][0] = 0;
        c.g[A0][0] = sx(arr.wrapping_add(8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[SP][0];
                break;
            }
            c.cycles += 2;
            let is_a = c.g[V1][0] == c.g[S0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(2));
            let mut still_room_known = false;
            if is_a {
                // skipped: fall to the common "v0 = (a1 + 2) < 10" below
            } else {
                c.cycles += 2;
                let is_b = c.g[V1][0] == c.g[S1][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
                if is_b {
                    still_room_known = true;
                } else {
                    c.cycles += 4;
                    c.sw(r32(c, A0), c.g[V1][0]);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(2));
                }
            }
            if !still_room_known {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
            }
            c.cycles += 2;
            let room = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if room {
                if event_due(c) {
                    deliver_event_here(c, 0x0023_a5b8);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[SP][0];
            break;
        }

        c.cycles += 29;
        c.g[A1][0] = 10;
        SHUFFLE.call(c, 0x0023_a5fc)?;
        let arr = r32(c, SP);
        c.g[V1][0] = c.lw(arr);
        c.g[A0][0] = c.lw(arr.wrapping_add(0x4));
        c.g[A1][0] = c.lw(arr.wrapping_add(0x8));
        c.g[A2][0] = c.lw(arr.wrapping_add(0xc));
        c.g[A3][0] = c.lw(arr.wrapping_add(0x10));
        c.g[T0][0] = c.lw(arr.wrapping_add(0x14));
        c.g[T1][0] = c.lw(arr.wrapping_add(0x18));
        c.g[T2][0] = c.lw(arr.wrapping_add(0x1c));
        c.g[T3][0] = c.lw(arr.wrapping_add(0x20));
        c.g[V0][0] = c.lw(arr.wrapping_add(0x24));
        let this = r32(c, S2);
        c.sw(this.wrapping_add(0x10), c.g[S0][0]);
        c.sw(this.wrapping_add(0x54), c.g[V0][0]);
        c.sw(this.wrapping_add(0x14), c.g[V1][0]);
        c.sw(this.wrapping_add(0x18), c.g[A0][0]);
        c.sw(this.wrapping_add(0x1c), c.g[A1][0]);
        c.sw(this.wrapping_add(0x20), c.g[A2][0]);
        c.sw(this.wrapping_add(0x24), c.g[A3][0]);
        c.sw(this.wrapping_add(0x34), c.g[T0][0]);
        c.sw(this.wrapping_add(0x38), c.g[T1][0]);
        c.sw(this.wrapping_add(0x3c), c.g[T2][0]);
        c.sw(this.wrapping_add(0x50), c.g[T3][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_Init(self)` (0x22cd40): resets the section manager.
/// Clears the 50-entry section map at 0x442168 (`{.., idx = -1, 0, ..}`, 16
/// bytes each, then entry 49's index = 0), then for each of the 50 section
/// files of the race interface matches the file name (`strcmp`) against every
/// streamer section name and stores the matching section index in the map.
/// Finally resets the mode/state fields (+0x1a4..+0x1d0), initialises the
/// node pool and sets `+0x1ac = -1`. A dispatcher: its loop back edges
/// deliver events and may unwind, so it is re-entered at `0x22cd90`,
/// `0x22cdc0` or `0x22cdc8`. Frame 0x80 (s0-s6).
pub fn section_man_init(c: &mut Cpu) {
    guest(c, section_man_init_stages)
}

fn section_man_init_stages(c: &mut Cpu) -> Flow {
    const ENTRY: u32 = 0x0022_cd40;
    const CLEAR_MAP: u32 = 0x0022_cd90;
    const START_SCAN: u32 = 0x0022_cdac;
    const NEXT_ENTRY: u32 = 0x0022_cdc0;
    const COMPARE: u32 = 0x0022_cdc8;
    const STORE_MATCH: u32 = 0x0022_ce00;
    const SECTION_LOOP: u32 = 0x0022_ce10;
    const NEXT_FILE: u32 = 0x0022_ce30;
    const FINISH: u32 = 0x0022_ce40;
    let frame = Frame { size: 0x80 };
    let mut pc = c.pc;
    loop {
        match pc {
            ENTRY => {
                c.cycles += 20;
                Frame::open(c, 0x80);
                frame.save(c, S2, 0x50);
                frame.save(c, S5, 0x20);
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                frame.save(c, S3, 0x40);
                frame.save(c, S4, 0x30);
                frame.save(c, S6, 0x10);
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014_dc80, 0x0022_cd6c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_dd58, 0x0022_cd78)?;
                c.g[S5][0] = c.g[V0][0];
                c.g[A1][0] = 0x44_0000;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x2168));
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x8));
                c.g[V1][0] = 0x31;
                pc = CLEAR_MAP;
            }
            CLEAR_MAP => {
                c.cycles += 7;
                c.sw(r32(c, V0).wrapping_sub(4), c.g[A0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                c.sw(r32(c, V0), 0);
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x10));
                if more {
                    if back_edge_unwinds(c, CLEAR_MAP) {
                        return Ok(());
                    }
                    pc = CLEAR_MAP;
                } else {
                    pc = START_SCAN;
                }
            }
            START_SCAN => {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x2168));
                c.g[S3][0] = 0;
                c.sw(r32(c, V0).wrapping_add(0x314), 0);
                c.g[S6][0] = sx(r32(c, V0).wrapping_add(4));
                c.g[S1][0] = 0;
                pc = NEXT_ENTRY;
            }
            NEXT_ENTRY => {
                c.cycles += 2;
                c.g[S4][0] = sx(r32(c, S3).wrapping_add(1));
                pc = SECTION_LOOP;
            }
            COMPARE => {
                c.cycles += 14;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0014_4d38, 0x0022_cdd0)?;
                let s2 = r32(c, S2);
                c.g[V1][0] = c.lw(s2.wrapping_add(0x1b0));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x10));
                c.g[A0][0] = c.lw(r32(c, V0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                call_ptr(c, 0x003a_9820, 0x0022_cdec)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0041_65a8, 0x0022_cdf8)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    pc = SECTION_LOOP;
                } else {
                    pc = STORE_MATCH;
                }
            }
            STORE_MATCH => {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, S3) << 4);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                c.sw(r32(c, V0), c.g[S1][0]);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                pc = SECTION_LOOP;
            }
            SECTION_LOOP => {
                c.cycles += 8;
                let s2 = r32(c, S2);
                c.g[V0][0] = c.lw(s2.wrapping_add(0x1b0));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x10));
                c.g[A0][0] = c.lw(r32(c, V1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                call_ptr(c, 0x003a_9800, 0x0022_ce24)?;
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S5][0];
                    if back_edge_unwinds(c, COMPARE) {
                        return Ok(());
                    }
                    pc = COMPARE;
                } else {
                    pc = NEXT_FILE;
                }
            }
            NEXT_FILE => {
                c.cycles += 4;
                c.g[S3][0] = c.g[S4][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 50) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = 0;
                if more {
                    if back_edge_unwinds(c, NEXT_ENTRY) {
                        return Ok(());
                    }
                    pc = NEXT_ENTRY;
                } else {
                    pc = FINISH;
                }
            }
            FINISH => {
                c.cycles += 26;
                c.g[V1][0] = 0x16;
                c.g[V0][0] = 1;
                let s2 = r32(c, S2);
                c.sw(s2.wrapping_add(0x1b8), 0);
                c.g[A0][0] = c.g[S2][0];
                c.sw(s2.wrapping_add(0x1b4), 0);
                c.sw(s2.wrapping_add(0x1c0), 0);
                c.sw(s2.wrapping_add(0x1c8), 0);
                c.sw(s2.wrapping_add(0x1cc), 0);
                c.sw(s2.wrapping_add(0x1bc), c.g[V1][0]);
                c.sw(s2.wrapping_add(0x1d0), c.g[V0][0]);
                SECTION_MAN_INIT_NODE_POOL.call(c, 0x0022_ce6c)?;
                c.g[V0][0] = u64::MAX;
                let s2 = r32(c, S2);
                c.sw(s2.wrapping_add(0x1c4), 0);
                c.sw(s2.wrapping_add(0x1ac), c.g[V0][0]);
                c.sw(s2.wrapping_add(0x1a4), 0);
                c.sw(s2.wrapping_add(0x1a8), 0);
                frame.restore(c, S0, 0x70);
                frame.restore(c, S1, 0x60);
                frame.restore(c, S2, 0x50);
                frame.restore(c, S3, 0x40);
                frame.restore(c, S4, 0x30);
                frame.restore(c, S5, 0x20);
                frame.restore(c, S6, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, 0x0022_cd40, 0x0022_cea8);
                return Ok(());
            }
        }
    }
}

const SECTION_MAN_INIT_NODE_POOL: Callee = Callee {
    addr: 0x0022_e228,
    f: ee::s05::f_0022e228,
};

/// `int CameraTriggerMan_Parse(self, void** cursor)` (0x16c228): reads the
/// camera trigger list from a stream. Resets the header, reads the version
/// cookie (`self + 0xc`; anything but 7 resets it to 7 and returns 3), reads
/// the header (count at +4; a non-zero status is returned as is), allocates
/// the `count * 4` array (stored at `self + 0`) and for each trigger allocates
/// 0x24 bytes (`{kind 4, type 3}`, fields cleared), reads it with
/// `ReadTrigger` and adds it with `AddTrigger`. A failing read frees the
/// trigger with `CameraTrigger_dtor(t, 3)` (when allocated) and returns its
/// status. Returns 0 and sets `self + 0xc = 7` on success. Frame 0x80 (s0-s5).
pub fn camera_trigger_man_parse(c: &mut Cpu) {
    const RESET_HEADER: Callee = Callee {
        addr: 0x0016_c390,
        f: ee::s02::f_0016c390,
    };
    const READ_VERSION: Callee = Callee {
        addr: 0x0016_c3a8,
        f: ee::s02::f_0016c3a8,
    };
    const READ_HEADER: Callee = Callee {
        addr: 0x0016_c3d0,
        f: ee::s02::f_0016c3d0,
    };
    const READ_TRIGGER: Callee = Callee {
        addr: 0x0016_c428,
        f: ee::s02::f_0016c428,
    };
    const ADD_TRIGGER: Callee = Callee {
        addr: 0x0016_c120,
        f: ee::s02::f_0016c120,
    };
    const TRIGGER_DTOR: Callee = Callee {
        addr: 0x0016_c5a8,
        f: ee::s02::f_0016c5a8,
    };
    const COUNT: u32 = 0x4;
    const VERSION: u32 = 0xc;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S1, 0x60);
        let cursor_slot = r32(c, SP);
        c.sw(cursor_slot, c.g[A1][0]);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        RESET_HEADER.call(c, 0x0016_c254)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        READ_VERSION.call(c, 0x0016_c260)?;
        let this = r32(c, S1);
        c.g[V0][0] = c.lw(this.wrapping_add(VERSION));
        c.g[V1][0] = 7;
        let version_ok = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 3;
        // 'status: the epilogue after `lq s0`; 'tail: the `cycles += 1; lq s0`
        // exit every non-early path takes.
        'status: {
            'tail: {
                if !version_ok {
                    c.cycles += 2;
                    c.sw(this.wrapping_add(VERSION), c.g[V1][0]);
                    break 'tail;
                }
                c.cycles += 7;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 7;
                READ_HEADER.call(c, 0x0016_c288)?;
                c.g[S0][0] = c.g[V0][0];
                let failed = c.g[S0][0] != 0;
                frame.restore(c, S0, 0x70);
                if failed {
                    break 'status;
                }
                // Trigger array: operator_new(count * 4).
                c.cycles += 11;
                c.g[A0][0] = c.lw(this.wrapping_add(COUNT));
                c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x40c0));
                c.g[A2][0] = 0;
                c.g[A0][0] = sx(r32(c, A0) << 2);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x0016_c2b0)?;
                c.g[S2][0] = 0;
                c.g[V1][0] = c.lw(this.wrapping_add(COUNT));
                let empty = (c.g[V1][0] as i64) <= 0;
                c.sw(this, c.g[V0][0]);
                'done: {
                    if empty {
                        break 'done;
                    }
                    c.cycles += 4;
                    c.g[S5][0] = 0x46_0000;
                    c.g[S4][0] = 4;
                    c.g[S3][0] = 3;
                    c.g[A0][0] = 0x24;
                    loop {
                        c.cycles += 18;
                        c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x40a8));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0016_c2e0)?;
                        c.g[A2][0] = c.lw(this.wrapping_add(VERSION));
                        c.g[A0][0] = c.g[SP][0];
                        let slot = cursor_slot.wrapping_add(4);
                        c.sw(slot, c.g[V0][0]);
                        c.g[A1][0] = sx(slot);
                        let trig = r32(c, V0);
                        c.sw(trig.wrapping_add(0x1c), c.g[S4][0]);
                        c.sw(trig.wrapping_add(0x8), 0);
                        c.sw(trig.wrapping_add(0xc), 0);
                        c.sw(trig.wrapping_add(0x10), 0);
                        c.sw(trig, 0);
                        c.sw(trig.wrapping_add(0x4), c.g[S3][0]);
                        READ_TRIGGER.call(c, 0x0016_c30c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw(slot);
                        if c.g[S0][0] != 0 {
                            c.cycles += 2;
                            let none = c.g[A0][0] == 0;
                            c.g[V0][0] = c.g[S0][0];
                            if !none {
                                c.cycles += 4;
                                c.g[A1][0] = 3;
                                TRIGGER_DTOR.call(c, 0x0016_c328)?;
                                c.g[V0][0] = c.g[S0][0];
                            }
                            break 'tail;
                        }
                        c.cycles += 9;
                        c.g[A1][0] = c.lw(slot);
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                        ADD_TRIGGER.call(c, 0x0016_c344)?;
                        c.g[V1][0] = c.lw(this.wrapping_add(COUNT));
                        c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0;
                        c.g[A0][0] = 0x24;
                        if !more {
                            break 'done;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0016_c2d0);
                        }
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = 7;
                c.g[V0][0] = 0;
                c.sw(this.wrapping_add(VERSION), c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 8;
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

/// One owned part of a camera trigger in `CameraTriggerMan_FreeTriggers`:
/// if `trigger[off]` (+0xc or +0x10) is set, call its virtual destructor
/// (`vtable = *(part + 4)`, `fn = *(vtable + 0xc)`, `this = part +
/// s16(vtable + 8)`), delete the part and clear the field. Entered and left
/// with `v0 = triggers array`.
fn camera_trigger_free_part(
    c: &mut Cpu,
    s1: u32,
    off: u32,
    ra_virtual: u32,
    ra_delete: u32,
) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(off));
    if c.g[A1][0] == 0 {
        c.g[V0][0] = c.lw(s1);
        return Ok(());
    }
    c.cycles += 15;
    c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(8));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ra_virtual)?;
    c.g[V0][0] = c.lw(s1);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(off));
    call_ptr(c, 0x0031_7e50, ra_delete)?;
    c.g[V0][0] = c.lw(s1);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lw(r32(c, V0));
    c.sw(r32(c, V1).wrapping_add(off), 0);
    c.g[V0][0] = c.lw(s1);
    Ok(())
}

/// `void CameraTriggerMan_FreeTriggers(self)` (0x16bf40): destroys every
/// trigger of the list (`self + 0` = array, `self + 4` = count): deletes its
/// buffer at +8, destroys and deletes its parts at +0xc and +0x10 (virtual
/// destructor), runs `CameraTrigger_dtor(t, 3)` on it, clears the slot, then
/// resets the header. Frame 0x40 (s0-s2).
pub fn camera_trigger_man_free_triggers(c: &mut Cpu) {
    const RESET_HEADER: Callee = Callee {
        addr: 0x0016_c390,
        f: ee::s02::f_0016c390,
    };
    const TRIGGER_DTOR: Callee = Callee {
        addr: 0x0016_c5a8,
        f: ee::s02::f_0016c5a8,
    };
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.g[V0][0] = c.lw(this.wrapping_add(4));
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = c.lw(this);
            loop {
                c.cycles += 6;
                c.g[S0][0] = sx(r32(c, S2) << 2);
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = c.lw(r32(c, V0));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = c.lw(this);
                } else {
                    c.cycles += 7;
                    call_ptr(c, 0x0031_7e50, 0x0016_bf88)?;
                    c.g[V0][0] = c.lw(this);
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.sw(r32(c, V1).wrapping_add(8), 0);
                    c.g[V0][0] = c.lw(this);
                }
                camera_trigger_free_part(c, this, 0xc, 0x0016_bfc4, 0x0016_bfd8)?;
                camera_trigger_free_part(c, this, 0x10, 0x0016_c014, 0x0016_c028)?;
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V0));
                if c.g[A0][0] == 0 {
                    c.g[V1][0] = c.lw(this);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 3;
                    TRIGGER_DTOR.call(c, 0x0016_c054)?;
                    c.g[V1][0] = c.lw(this);
                }
                c.cycles += 7;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                c.sw(r32(c, V1), 0);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(this);
                if event_due(c) {
                    deliver_event_here(c, 0x0016_bf68);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        RESET_HEADER.call(c, 0x0016_c07c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ProcessList_Init(self, int n)` (0x3549e0): allocates the bucket
/// array (`operator_new(n * 0x44 + 0x10, "Buckets")`; its first word holds
/// `n`, the buckets start at +0x10), constructs every bucket (four
/// self-linked list heads at +0x8/+0x18/+0x28/+0x38 whose vtable words at
/// +0x10/+0x20/+0x30/+0x40 are 0x491f00), stores the array at `self + 4` and
/// `n` at `self + 0`, then links every bucket's lists (`l0.prev = &l1`,
/// `l1.next = &l0`, `l2.prev = &l3`, `l3.next = &l2`) and clears its flags
/// (+0). Frame 0x40 (s0-s2).
pub fn process_list_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x44;
        frame.save(c, S1, 0x20);
        c.g[A2][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.mult(0, V0, S1, V0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x4a_0000;
        frame.save(c, S2, 0x10);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x4020));
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[S2][0] = sx(r32(c, S0).wrapping_add(4));
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x10));
        call_ptr(c, 0x0031_7e30, 0x0035_4a20)?;
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x10));
        c.g[A0][0] = sx(r32(c, S1).wrapping_sub(1));
        c.sw(r32(c, V0), c.g[S1][0]);
        let zero = c.g[S1][0] == 0;
        c.g[V1][0] = c.g[A1][0];
        // Construct the buckets: vtable words and cleared list links.
        if !zero {
            c.cycles += 3;
            c.g[V0][0] = 0x49_0000;
            c.g[A2][0] = u64::MAX;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1f00));
            loop {
                c.cycles += 15;
                let b = r32(c, V1);
                c.sw(b.wrapping_add(0x10), c.g[V0][0]);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                c.sw(b.wrapping_add(0x8), 0);
                c.sw(b.wrapping_add(0xc), 0);
                c.sw(b.wrapping_add(0x20), c.g[V0][0]);
                c.sw(b.wrapping_add(0x18), 0);
                c.sw(b.wrapping_add(0x1c), 0);
                c.sw(b.wrapping_add(0x30), c.g[V0][0]);
                c.sw(b.wrapping_add(0x28), 0);
                c.sw(b.wrapping_add(0x2c), 0);
                c.sw(b.wrapping_add(0x40), c.g[V0][0]);
                c.sw(b.wrapping_add(0x38), 0);
                c.sw(b.wrapping_add(0x3c), 0);
                let more = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx(b.wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_4a40);
                }
            }
        }
        c.cycles += 4;
        c.sw(r32(c, S2), c.g[A1][0]);
        c.g[A2][0] = 0;
        let empty = (c.g[S1][0] as i64) <= 0;
        c.sw(r32(c, S0), c.g[S1][0]);
        if !empty {
            c.cycles += 1;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 36;
                let this = r32(c, S0);
                // The bucket's address is re-read from `self + 4` each time.
                c.g[A0][0] = c.lw(this.wrapping_add(4));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x14));
                c.sw(r32(c, A0).wrapping_add(0x8), c.g[V1][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0).wrapping_add(0xc), 0);
                c.g[V1][0] = c.lw(this.wrapping_add(4));
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.sw(r32(c, V1).wrapping_add(0x18), 0);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(4));
                c.sw(r32(c, V0).wrapping_add(0x1c), c.g[A0][0]);
                c.g[V1][0] = c.lw(this.wrapping_add(4));
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x34));
                c.sw(r32(c, V1).wrapping_add(0x28), c.g[A0][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0).wrapping_add(0x2c), 0);
                c.g[V1][0] = c.lw(this.wrapping_add(4));
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.sw(r32(c, V1).wrapping_add(0x38), 0);
                c.g[A0][0] = c.lw(this.wrapping_add(4));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x24));
                c.sw(r32(c, A0).wrapping_add(0x3c), c.g[V1][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), 0);
                c.g[V0][0] = c.lw(this);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x44));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_4a90);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameModeMan_StartMode(self, int type)` (0x238160): switches the
/// game mode manager to a new event type. Asks the BE for interface 0 (result
/// unused), calls the current mode handler's virtual method 3 (handler table
/// `0x536668[self->iCurrModeHandler]`), stores `type` at `self + 8` and the
/// network-path byte (0x535c11) at `self + 0xc`, picks the new handler index
/// from `type` (a `jr` table at 0x47c0d0 for the network path 2: types 0..5;
/// otherwise a table at 0x47c0f0: types 0..12, out of range keeps the old
/// handler), stores it at `self + 4`, sets `self + 0x84 = 1` and calls
/// `GameModeMan_ResetMode(self)`. Frame 0x30 (s0, s1).
pub fn game_mode_man_start_mode(c: &mut Cpu) {
    const RESET_MODE: Callee = Callee {
        addr: 0x0023_82d8,
        f: ee::s06::f_002382d8,
    };
    const ENTRY: u32 = 0x0023_8160;
    const NET_PATH_NOT_2: u32 = 0x0023_8228;
    const NET_PATH_2: u32 = 0x0023_81cc;
    const NET_TABLE_JUMP: u32 = 0x0023_81d8;
    const OFFLINE_TABLE_JUMP: u32 = 0x0023_8234;
    const NET_OUT_OF_RANGE: u32 = 0x0023_8220;
    const KEEP_HANDLER: u32 = 0x0023_824c;
    const STORE_HANDLER: u32 = 0x0023_8290;
    const FINISH: u32 = 0x0023_8294;
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 27;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    call_ptr(c, 0x0014_dc80, 0x0023_817c)?;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dd58, 0x0023_8188)?;
                    // handler = HANDLERS[self->iCurrModeHandler]; handler->vt[3]()
                    let this = r32(c, S0);
                    c.g[V1][0] = c.lw(this.wrapping_add(4));
                    c.g[V0][0] = 0x53_0000;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6668));
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = c.lw(r32(c, V1));
                    c.g[V0][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x18));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x1c));
                    let method = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0023_81b4)?;
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(8), c.g[S1][0]);
                    c.g[V1][0] = 0x53_0000;
                    c.g[A0][0] = 2;
                    c.g[V0][0] = c.lb(r32(c, V1).wrapping_add(0x5c11));
                    let not_net2 = c.g[V0][0] != c.g[A0][0];
                    c.sw(this.wrapping_add(0xc), c.g[V0][0]);
                    pc = if not_net2 { NET_PATH_NOT_2 } else { NET_PATH_2 };
                }
                NET_PATH_2 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 6) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x48_0000;
                    pc = if out_of_range {
                        NET_OUT_OF_RANGE
                    } else {
                        NET_TABLE_JUMP
                    };
                }
                NET_TABLE_JUMP | OFFLINE_TABLE_JUMP => {
                    c.cycles += 6;
                    let table_hi = if pc == NET_TABLE_JUMP { 0x3f30 } else { 0x3f10 };
                    c.g[V1][0] = sx(r32(c, S1) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(table_hi));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = c.lw(r32(c, V1));
                    pc = r32(c, A0);
                }
                // The switch cases: each stores a handler index (or, for
                // KEEP_HANDLER, clears `self + 4`).
                0x0023_81f0 | 0x0023_81f8 | 0x0023_8200 | 0x0023_8208 | 0x0023_8210
                | 0x0023_8218 | NET_OUT_OF_RANGE | 0x0023_8254 | 0x0023_825c | 0x0023_8264
                | 0x0023_826c | 0x0023_8274 | 0x0023_827c | 0x0023_8284 => {
                    let v = match pc {
                        0x0023_81f0 => 8,
                        0x0023_81f8 | 0x0023_8200 | 0x0023_8208 => 9,
                        0x0023_8210 => 5,
                        0x0023_8218 => 6,
                        NET_OUT_OF_RANGE => 8,
                        0x0023_8254 => 1,
                        0x0023_825c => 2,
                        0x0023_8264 => 5,
                        0x0023_826c | 0x0023_8274 | 0x0023_827c => 4,
                        _ => 6,
                    };
                    c.cycles += 2;
                    c.g[V0][0] = v;
                    pc = STORE_HANDLER;
                }
                0x0023_828c => {
                    c.cycles += 1;
                    c.g[V0][0] = 7;
                    pc = STORE_HANDLER;
                }
                NET_PATH_NOT_2 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S1][0] < 0xd) as u64;
                    let out_of_range = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x48_0000;
                    pc = if out_of_range {
                        FINISH
                    } else {
                        OFFLINE_TABLE_JUMP
                    };
                }
                KEEP_HANDLER => {
                    c.cycles += 2;
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(4), 0);
                    pc = FINISH;
                }
                STORE_HANDLER => {
                    c.cycles += 1;
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(4), c.g[V0][0]);
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 9;
                    c.g[V0][0] = 1;
                    c.g[A0][0] = c.g[S0][0];
                    let this = r32(c, S0);
                    c.sw(this.wrapping_add(0x84), c.g[V0][0]);
                    RESET_MODE.call(c, 0x0023_82a4)?;
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0023_8160, 0x0023_82b8);
                    return Ok(());
                }
            }
        }
    })
}

/// `bool ScriptCmdQueue_Push(self, const Cmd* cmd)` (0x283c80): pushes a
/// 0x1c-byte script command into the ring buffer (entries at `self + i *
/// 0x1c`, 4 slots, head at +0x90, tail at +0x8c, "full" flag at +0x94).
/// Returns 0 when full; otherwise copies the command to the head slot,
/// advances the head (wrapping 4 -> 0) and sets the full flag when it reaches
/// the tail; returns 1. Frame 0x30 (s0, s1).
pub fn script_cmd_queue_push(c: &mut Cpu) {
    const IS_FULL: Callee = Callee {
        addr: 0x0028_3c58,
        f: ee::s07::f_00283c58,
    };
    const NEXT_INDEX: Callee = Callee {
        addr: 0x0028_3c60,
        f: ee::s07::f_00283c60,
    };
    const TAIL: u32 = 0x8c;
    const HEAD: u32 = 0x90;
    const FULL: u32 = 0x94;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        IS_FULL.call(c, 0x0028_3c9c)?;
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 24;
            let this = r32(c, S0);
            let cmd = r32(c, S1);
            c.g[V0][0] = c.lw(this.wrapping_add(HEAD));
            c.g[V1][0] = 0x1c;
            c.g[A0][0] = c.g[S0][0];
            c.mult(0, A1, V0, V1);
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(this));
            let slot = r32(c, V0);
            c.ldl(A2, cmd.wrapping_add(0x7));
            c.ldr(A2, cmd);
            c.ldl(A3, cmd.wrapping_add(0xf));
            c.ldr(A3, cmd.wrapping_add(0x8));
            c.ldl(V1, cmd.wrapping_add(0x17));
            c.ldr(V1, cmd.wrapping_add(0x10));
            c.g[A1][0] = c.lw(cmd.wrapping_add(0x18));
            c.sdl(A2, slot.wrapping_add(0x7));
            c.sdr(A2, slot);
            c.sdl(A3, slot.wrapping_add(0xf));
            c.sdr(A3, slot.wrapping_add(0x8));
            c.sdl(V1, slot.wrapping_add(0x17));
            c.sdr(V1, slot.wrapping_add(0x10));
            c.sw(slot.wrapping_add(0x18), c.g[A1][0]);
            c.g[A1][0] = c.lw(this.wrapping_add(HEAD));
            NEXT_INDEX.call(c, 0x0028_3cf8)?;
            c.g[V1][0] = c.lw(this.wrapping_add(TAIL));
            let not_full_now = c.g[V0][0] != c.g[V1][0];
            c.sw(this.wrapping_add(HEAD), c.g[V0][0]);
            if !not_full_now {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.sw(this.wrapping_add(FULL), c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_Shutdown(self)` (0x309f18): destroys all Avalanche
/// objects (`0x2d9b40`), resets the collect sets (the table at +0x2c0, one
/// entry of 0x10c bytes, only one is walked), then runs the global shutdown
/// scripts of every world section of type 3: `self + 0x28c` is the world;
/// section `i` (`info->sections + i * 0xc`) of type 3 yields
/// `World_GetSectionShutdownFuncs` (`globals`) and their count, and each
/// function is run with `WScriptMan_CallFunc(self, fn)`. The script id slot
/// `self + 0` is set to -1 meanwhile (restored at the end) and the counters
/// at +0x290, +0x294, +0x298, +0x2a4, +0x2a8 are cleared before and after.
/// Frame 0x80 (s0-s6).
pub fn wscript_man_shutdown(c: &mut Cpu) {
    const RESET_COLLECT_SET: Callee = Callee {
        addr: 0x0030_c390,
        f: ee::s10::f_0030c390,
    };
    const CALL_FUNC: Callee = Callee {
        addr: 0x0030_9c88,
        f: ee::s10::f_00309c88,
    };
    const WORLD: u32 = 0x28c;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x2c0));
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        call_ptr(c, 0x002d_9b40, 0x0030_9f4c)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            RESET_COLLECT_SET.call(c, 0x0030_9f5c)?;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_9f50);
            }
        }
        // Clear the busy state, remembering the current script id.
        c.cycles += 16;
        let this = r32(c, S2);
        c.g[S6][0] = c.lw(this);
        c.g[V0][0] = u64::MAX;
        c.sw(this, c.g[V0][0]);
        c.g[S0][0] = 0;
        for off in [0x290, 0x294, 0x298, 0x2a4, 0x2a8] {
            c.sw(this.wrapping_add(off), 0);
        }
        c.g[V0][0] = c.lw(this.wrapping_add(WORLD));
        c.g[A0][0] = c.lw(r32(c, V0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        call_ptr(c, 0x003a_9800, 0x0030_9f98)?;
        c.g[S4][0] = c.g[V0][0];
        let no_sections = (c.g[S4][0] as i64) <= 0;
        c.g[S5][0] = 0xc;
        if !no_sections {
            c.cycles += 1;
            c.g[A1][0] = c.lw(this.wrapping_add(WORLD));
            loop {
                'next: {
                    'skip: {
                        c.cycles += 8;
                        c.mult(0, A0, S0, S5);
                        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(4));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        c.g[V0][0] = c.lh(r32(c, A0));
                        c.g[V0][0] ^= 3;
                        let not_type3 = c.g[V0][0] != 0;
                        c.g[S3][0] = sx(r32(c, S0).wrapping_add(1));
                        if not_type3 {
                            break 'skip;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[A1][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_6bd8, 0x0030_9fd4)?;
                        c.g[S1][0] = c.g[V0][0];
                        if c.g[S1][0] == 0 {
                            c.g[S0][0] = c.g[S3][0];
                            break 'next;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(this.wrapping_add(WORLD));
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_6ba8, 0x0030_9fec)?;
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[S0][0] = c.g[V0][0];
                        if none {
                            break 'skip;
                        }
                        c.cycles += 1;
                        loop {
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S2][0];
                            CALL_FUNC.call(c, 0x0030_a004)?;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                            let more = c.g[S0][0] != 0;
                            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                            if !more {
                                break 'skip;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_9ff8);
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw(this.wrapping_add(WORLD));
                if event_due(c) {
                    deliver_event_here(c, 0x0030_9fa8);
                }
            }
        }
        c.cycles += 16;
        c.sw(this, c.g[S6][0]);
        for off in [0x290, 0x294, 0x298, 0x2a4, 0x2a8] {
            c.sw(this.wrapping_add(off), 0);
        }
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptList_NextEntry(self)` (0x277838): advances the script list.
/// With a pending entry (`self + 0xbc > 0`) it pops the list entries, clears
/// the pending slot, takes the next command (`ScriptCmdQueue_At(self + 0xc,
/// 0)`) and starts it (a movie via `ScriptMovie_Start(engine + 0x500)`, else
/// `ScriptEngine_PlayScriptInstance` plus the engine's virtual method 6),
/// marks `self + 0xac` when `self + 0xb0` is set and prepares the next entry
/// into `self + 0xbc`. Without one it stops the list and, when `self + 4` is
/// set, fades via the game's transition manager (the remaining time of the
/// running transition when `self + 0xb0` was set, else 0.5s). Frame 0x30.
pub fn script_list_next_entry(c: &mut Cpu) {
    const POP_ENTRIES: Callee = Callee {
        addr: 0x0027_7598,
        f: ee::s07::f_00277598,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PLAY_SCRIPT_INSTANCE: Callee = Callee {
        addr: 0x0027_9720,
        f: ee::s07::f_00279720,
    };
    const MOVIE_START: Callee = Callee {
        addr: 0x0028_39a8,
        f: ee::s07::f_002839a8,
    };
    const PREPARE_ENTRY: Callee = Callee {
        addr: 0x0027_7450,
        f: ee::s07::f_00277450,
    };
    const LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };
    const PENDING: u32 = 0xbc;
    const ENABLED: u32 = 0xb0;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[A1][0] = c.lw(this.wrapping_add(PENDING));
        'ret: {
            'end: {
                if (c.g[A1][0] as i64) > 0 {
                    c.cycles += 14;
                    c.sw(this.wrapping_add(0xc8), 0);
                    POP_ENTRIES.call(c, 0x0027_7860)?;
                    c.g[V0][0] = u64::MAX;
                    c.sw(this.wrapping_add(0xc0), 0);
                    c.g[A0][0] = sx(this.wrapping_add(0xc));
                    c.g[A1][0] = 0;
                    c.sw(this.wrapping_add(PENDING), c.g[V0][0]);
                    CMD_QUEUE_AT.call(c, 0x0027_7878)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    IS_MOVIE_ENTRY.call(c, 0x0027_7888)?;
                    let movie = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(this);
                    if !movie {
                        let cmd = r32(c, S1);
                        c.cycles += 16;
                        c.g[T0][0] = c.lw(cmd.wrapping_add(8));
                        c.g[A3][0] = 0;
                        c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 3) as u32);
                        c.g[A1][0] = c.lw(this.wrapping_add(8));
                        c.g[A2][0] = c.lw(cmd.wrapping_add(0xc));
                        c.g[T0][0] &= 1;
                        PLAY_SCRIPT_INSTANCE.call(c, 0x0027_78ac)?;
                        c.g[A2][0] = c.lw(this);
                        c.g[A1][0] = c.lw(cmd.wrapping_add(0xc));
                        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x2a8));
                        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x30));
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x34));
                        let method = r32(c, V0);
                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                        call_ptr(c, method, 0x0027_78c8)?;
                        c.g[V0][0] = c.lw(this.wrapping_add(ENABLED));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x500));
                        MOVIE_START.call(c, 0x0027_78d8)?;
                        c.g[V0][0] = c.lw(this.wrapping_add(ENABLED));
                    }
                    c.cycles += 2;
                    let off = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if !off {
                        c.cycles += 1;
                        c.sw(this.wrapping_add(0xac), c.g[V0][0]);
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = 0;
                    PREPARE_ENTRY.call(c, 0x0027_78f8)?;
                    c.sw(this.wrapping_add(PENDING), c.g[V0][0]);
                    break 'end;
                }
                // Nothing pending: stop the list and fade out.
                c.g[S1][0] = c.lw(this.wrapping_add(ENABLED));
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                LIST_STOP.call(c, 0x0027_790c)?;
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                let no_fade = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x848));
                if no_fade {
                    break 'end;
                }
                'fade_half: {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                    c.g[S0][0] = c.lw(r32(c, A0).wrapping_add(0x64));
                    call_ptr(c, 0x0023_0698, 0x0027_7928)?;
                    let was_off = c.g[S1][0] == 0;
                    c.g[A0][0] = c.g[V0][0];
                    if was_off {
                        break 'fade_half;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x44));
                    if c.g[V0][0] == 0 {
                        break 'fade_half;
                    }
                    c.cycles += 7;
                    call_ptr(c, 0x002e_4678, 0x0027_7944)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(12, 0);
                    call_ptr(c, 0x002e_a780, 0x0027_7950)?;
                    frame.restore(c, S0, 0x20);
                    break 'ret;
                }
                c.cycles += 4;
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(AT, 12);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002e_a780, 0x0027_7968)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `gp - off` as an address (the globals the game addresses off `gp`).
fn oc432_gp_minus(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `InSeq* InSeq_ctor(self, pHeap, heapSize, count, tag)` (0x272788): sets up
/// an instance pool. Initialises the heap (`self + 8`), stores `tag` (+0) and
/// `count` (+4), clears +0x14/+0x18, allocates `count * 0x58 + 0x10` bytes
/// (the first word holds the count, the items start at +0x10), constructs
/// every item (`InSeq_Instance_ctor(item + 8)`), stores the items at +0x14,
/// links them in a free list (`item[i].index = i`, `item[i].next = &item[i +
/// 1]`, `item[i].prev = &item[i - 1]`), terminates both ends (the first item's
/// prev and the last one's next = 0), clears the used list (+0x1c) and points
/// the free list (+0x18) at the first item. Returns `self`. Frame 0x70 (s0-s5).
pub fn in_seq_ctor(c: &mut Cpu) {
    const HEAP_INIT: Callee = Callee {
        addr: 0x0027_22e0,
        f: ee::s07::f_002722e0,
    };
    const HEAP_NEW_ARRAY: Callee = Callee {
        addr: 0x0027_24c8,
        f: ee::s07::f_002724c8,
    };
    const INSTANCE_CTOR: Callee = Callee {
        addr: 0x0027_4518,
        f: ee::s07::f_00274518,
    };
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        let this = r32(c, S3);
        c.g[S2][0] = sx(this.wrapping_add(8));
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S2][0];
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = sx(r32(c, S0).wrapping_sub(1));
        frame.save_ra(c, 0);
        c.g[S5][0] = sx(this.wrapping_add(0x14));
        HEAP_INIT.call(c, 0x0027_27c8)?;
        c.g[A0][0] = 0x58;
        c.sw(this, c.g[S1][0]);
        c.mult(0, A0, S0, A0);
        c.sw(this.wrapping_add(4), c.g[S0][0]);
        c.sw(this.wrapping_add(0x14), 0);
        c.g[A1][0] = c.g[S2][0];
        c.sw(this.wrapping_add(0x18), 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        HEAP_NEW_ARRAY.call(c, 0x0027_27ec)?;
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(0x10));
        c.sw(r32(c, V0), c.g[S0][0]);
        let empty = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[S2][0];
        // Construct the items.
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = u64::MAX;
            loop {
                c.cycles += 5;
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(8));
                c.g[S4][0] = sx(r32(c, S4).wrapping_sub(1));
                INSTANCE_CTOR.call(c, 0x0027_280c)?;
                let more = c.g[S4][0] != c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x58));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2800);
                }
            }
        }
        // Link the items: index, next (+0x50), prev (+0x54).
        c.cycles += 4;
        c.sw(r32(c, S5), c.g[S2][0]);
        c.g[V0][0] = c.lw(this.wrapping_add(4));
        let none = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A3][0] = 0xffff_ffff_ffff_ffa8;
            c.g[A2][0] = 0x58;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 18;
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0).wrapping_add(4), c.g[A0][0]);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x58));
                c.sw(r32(c, V0).wrapping_add(0x50), c.g[V1][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x58));
                c.sw(r32(c, V0).wrapping_add(0x54), c.g[V1][0]);
                c.g[V0][0] = c.lw(this.wrapping_add(4));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x58));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_2830);
                }
            }
        }
        c.cycles += 21;
        c.g[A0][0] = c.lw(this.wrapping_add(0x14));
        c.g[A1][0] = 0x58;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x60);
        c.sw(r32(c, A0).wrapping_add(0x54), 0);
        frame.restore(c, S1, 0x50);
        c.g[V1][0] = c.lw(this.wrapping_add(4));
        c.g[A0][0] = c.lw(this.wrapping_add(0x14));
        c.mult(0, V1, V1, A1);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        c.sw(r32(c, V1).wrapping_sub(8), 0);
        c.sw(this.wrapping_add(0x1c), 0);
        c.g[V1][0] = c.lw(this.wrapping_add(0x14));
        c.sw(this.wrapping_add(0x18), c.g[V1][0]);
        frame.restore(c, S3, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_LoadWorldScripts(self)` (0x278b98): starts loading the
/// current track's script data. Builds the track's file name from the race
/// interface's track info (`name = trackInfo + 0x34`), kicks off the async
/// load of the filter table (`"data/scripts/scdat_big/data/scripts/scfi" +
/// name + ".dat"`, handle stored at `self + 0x538`, `self + 0x620` counts the
/// pending loads) and then loads the script bank
/// (`ScriptEngine_LoadBankAsync(self, 1, "…/scda" + name + ".big", 0, -1, 0,
/// 0)`). The `cBXString` temporaries live in the frame (0x90, s0-s3).
pub fn script_engine_load_world_scripts(c: &mut Cpu) {
    const LOAD_BANK_ASYNC: Callee = Callee {
        addr: 0x0027_b370,
        f: ee::s07::f_0027b370,
    };
    const FILTER_HANDLE: u32 = 0x538;
    const PENDING_LOADS: u32 = 0x620;
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S0, 0x80);
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0027_8bb8)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0027_8bc4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x0027_8bcc)?;
        // name = cBXString(trackInfo + 0x34)
        let name = r32(c, SP);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(0x34));
        call_ptr(c, 0x0031_82d8, 0x0027_8bd8)?;
        c.g[S1][0] = sx(name.wrapping_add(0x20));
        c.g[A1][0] = 0x48_0000;
        c.g[A2][0] = c.g[SP][0];
        c.g[S3][0] = sx(name.wrapping_add(0x10));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1c08));
        c.g[A0][0] = c.g[S1][0];
        // tmp = "data/scripts/scdat_big/data/scripts/scfi" + name
        call_ptr(c, 0x0031_86d0, 0x0027_8bf4)?;
        c.g[A2][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x34b8));
        c.g[A1][0] = c.g[S1][0];
        // path = tmp + ".dat"
        call_ptr(c, 0x0031_8630, 0x0027_8c08)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0027_8c14)?;
        c.g[A0][0] = c.lw(name.wrapping_add(0x10));
        c.g[A1][0] = 0;
        call_ptr(c, 0x003d_f690, 0x0027_8c20)?;
        let failed = c.g[V0][0] == 0;
        let this = r32(c, S2);
        c.sw(this.wrapping_add(FILTER_HANDLE), c.g[V0][0]);
        if !failed {
            c.cycles += 3;
            c.g[V0][0] = c.lw(this.wrapping_add(PENDING_LOADS));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(this.wrapping_add(PENDING_LOADS), c.g[V0][0]);
        }
        c.cycles += 41;
        c.g[S0][0] = sx(name.wrapping_add(0x30));
        c.g[A1][0] = 0x48_0000;
        c.g[A2][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1c38));
        // tmp2 = "data/scripts/scdat_big/data/scripts/scda" + name
        call_ptr(c, 0x0031_86d0, 0x0027_8c4c)?;
        c.g[A2][0] = 0x4a_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x34c0));
        c.g[A1][0] = c.g[S0][0];
        // tmp = tmp2 + ".big"
        call_ptr(c, 0x0031_8630, 0x0027_8c60)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0031_8448, 0x0027_8c6c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0027_8c78)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0027_8c84)?;
        c.g[A2][0] = c.lw(name.wrapping_add(0x10));
        c.g[A3][0] = 0;
        c.g[T0][0] = u64::MAX;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        LOAD_BANK_ASYNC.call(c, 0x0027_8ca4)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0027_8cb0)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0027_8cbc)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReset_OverlayEnter(self)` (0x234f40): enters the reset
/// overlay. Clears `self + 0x18`, resets the replay, the script engine's NIS
/// list (`ScriptEngine_ResetNISList(engine, 0)`), unloads and reloads the
/// world scripts, resets the AI render slots, clears `self + 0x14`. On the
/// network path (byte 0x535c11 != 0) queues the intro scripts; otherwise, for
/// locations 14..=16 (`SectionMan + 0x1bc`), asks `IsCharProfileFlagClear`
/// whether the rider's first time on the track and, if so, sets the audio
/// flags (+0x5790 = 0, +0x578c = 1, +0x6254 = 1) and `self + 0x14 = 1`. Ends
/// with `self + 0x10 = 0`. Frame 0x40 (s0-s2).
pub fn game_state_reset_overlay_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x488));
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.sw(this.wrapping_add(0x18), 0);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
        call_ptr(c, 0x0026_f228, 0x0023_4f68)?;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_90a0, 0x0023_4f74)?;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        call_ptr(c, 0x0027_8cd8, 0x0023_4f7c)?;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        call_ptr(c, 0x0027_8b98, 0x0023_4f84)?;
        call_ptr(c, 0x0014_dc80, 0x0023_4f8c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_4f98)?;
        c.sw(this.wrapping_add(0x14), 0);
        c.g[V0][0] = 0x53_0000;
        c.g[V1][0] = c.lw(oc432_gp_minus(c, 0x488));
        c.g[S0][0] = c.lb(r32(c, V0).wrapping_add(0x5c11));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        call_ptr(c, 0x0012_ab20, 0x0023_4fb0)?;
        c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x488));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        call_ptr(c, 0x0012_ac48, 0x0023_4fbc)?;
        let network = c.g[S0][0] != 0;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        if network {
            // Network path: queue the intro scripts.
            c.cycles += 2;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0027_aaf8, 0x0023_5064)?;
            c.cycles += 1;
            c.sw(this.wrapping_add(0x10), 0);
        } else {
            'out: {
                c.cycles += 8;
                c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x848));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x78));
                c.g[S2][0] = c.lw(r32(c, A0).wrapping_add(0x1bc));
                c.g[V0][0] = sx(r32(c, S2).wrapping_sub(0xe));
                c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                if c.g[V0][0] == 0 {
                    c.sw(this.wrapping_add(0x10), 0);
                    break 'out;
                }
                c.cycles += 22;
                call_ptr(c, 0x0014_dc80, 0x0023_4fec)?;
                c.g[A1][0] = 1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x0023_4ff8)?;
                c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x488));
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x40));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh(r32(c, A1).wrapping_add(0x38));
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x3c));
                let method = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                call_ptr(c, method, 0x0023_5024)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0014_5d38, 0x0023_5034)?;
                if c.g[V0][0] == 0 {
                    c.sw(this.wrapping_add(0x10), 0);
                    break 'out;
                }
                c.cycles += 8;
                call_ptr(c, 0x0028_b180, 0x0023_5044)?;
                c.g[V1][0] = 1;
                let audio = r32(c, V0);
                c.sw(audio.wrapping_add(0x5790), 0);
                c.sw(audio.wrapping_add(0x6254), c.g[V1][0]);
                c.sw(audio.wrapping_add(0x578c), c.g[V1][0]);
                c.sw(this.wrapping_add(0x14), c.g[V1][0]);
                c.cycles += 1;
                c.sw(this.wrapping_add(0x10), 0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_LoadLocale(self)` (0x228a78): makes sure the front-end locale
/// exists (`self + 0x8c`: allocates 0xc0 bytes named "FELocale" and runs its
/// constructor), asks the BE for interface 4, copies the 0x288-byte options
/// block at 0x535610 to the stack and calls the locale's virtual method 3
/// with the language (`options.word0 >> 22 & 7`), then loads the common
/// table (`FELocale_LoadCommonTable`). Frame 0x2b0 (s0).
pub fn ssx_app_load_locale(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, S0, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = c.lw(this.wrapping_add(0x8c));
        let have_locale = c.g[V0][0] != 0;
        c.g[A0][0] = 0xc0;
        if !have_locale {
            c.cycles += 8;
            c.g[A1][0] = 0x48_0000;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x5688));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0022_8aa8)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_5880, 0x0022_8ab0)?;
            c.sw(this.wrapping_add(0x8c), c.g[V0][0]);
        }
        c.cycles += 15;
        call_ptr(c, 0x0014_dc80, 0x0022_8abc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0022_8ac8)?;
        c.g[A1][0] = c.lw(this.wrapping_add(0x8c));
        c.g[V0][0] = 0x53_0000;
        c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x5610));
        c.g[A2][0] = c.g[SP][0];
        let locale = r32(c, A1);
        c.g[V1][0] = c.lw(locale.wrapping_add(4));
        c.g[A0][0] = c.g[A3][0] & 7;
        c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(0x18));
        c.g[T0][0] = sx(r32(c, V1).wrapping_add(0x1c));
        // 0x535610 is 8-aligned: the aligned 8-byte copy loop always runs.
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(0x280));
        loop {
            c.cycles += 11;
            let src = r32(c, A3);
            let dst = r32(c, A2);
            c.g[V1][0] = c.ld(src);
            c.g[A0][0] = c.ld(src.wrapping_add(8));
            c.g[T1][0] = c.ld(src.wrapping_add(0x10));
            c.g[T2][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[V1][0]);
            c.sd(dst.wrapping_add(8), c.g[A0][0]);
            c.sd(dst.wrapping_add(0x10), c.g[T1][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T2][0]);
            c.g[A3][0] = sx(src.wrapping_add(0x20));
            let more = c.g[A3][0] != c.g[V0][0];
            c.g[A2][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_8b4c);
            }
        }
        c.cycles += 16;
        c.ldl(V0, r32(c, A3).wrapping_add(7));
        c.ldr(V0, r32(c, A3));
        c.sdl(V0, r32(c, A2).wrapping_add(7));
        c.sdr(V0, r32(c, A2));
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = c.lw(r32(c, T0));
        c.g[A1][0] = c.lw(r32(c, SP));
        c.g[A1][0] = sx(r32(c, A1) >> 22);
        let method = r32(c, V0);
        c.g[A1][0] &= 7;
        call_ptr(c, method, 0x0022_8ba0)?;
        c.g[A0][0] = c.lw(this.wrapping_add(0x8c));
        call_ptr(c, 0x0019_5b70, 0x0022_8ba8)?;
        frame.restore(c, S0, 0x2a0);
        frame.restore_ra(c, 0x290);
        frame.close(c);
        Ok(())
    })
}

/// `void AIFwdDiffCache_Init(self, int n)` (0x327890): allocates the cache
/// entries (`n * 0x1cd0` bytes, "AIFwdDiffCache"), constructs each, stores
/// them at `self + 0`, links them into a ring (`entry[i].next = &entry[i +
/// 1]`, `.prev = &entry[i - 1]`, `.key = -1`; the first entry's prev is the
/// last entry, the last one's next is the first), sets the LRU head
/// (`self + 4`) to the first entry, the bucket count (`self + 0xc`) to `n * 2
/// + 13` and allocates the bucket table (`count * 4` bytes, "AIFwdDiffHash",
/// flag 0x80000000) at `self + 8`. Frame 0x70 (s0-s5).
pub fn ai_fwd_diff_cache_init(c: &mut Cpu) {
    const NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    const ENTRY_CTOR: u32 = 0x0034_0970;
    const ENTRY_SIZE: u32 = 0x1cd0;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x70);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[A0][0] = ENTRY_SIZE as u64;
        c.mult(0, 0, S3, A0);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x60);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1b08));
        frame.save(c, S1, 0x50);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[A0][0] = c.lo[0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(r32(c, S3).wrapping_sub(1));
        NEW_NAMED.call(c, 0x0032_78dc)?;
        c.g[S4][0] = c.g[V0][0];
        let none = c.g[S3][0] == 0;
        c.g[S0][0] = c.g[S4][0];
        // Construct the entries.
        if !none {
            c.cycles += 2;
            c.g[S5][0] = u64::MAX;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                call_ptr(c, ENTRY_CTOR, 0x0032_78fc)?;
                let more = c.g[S1][0] != c.g[S5][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(ENTRY_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_78f0);
                }
            }
        }
        // Link the entries: next, prev, key = -1.
        c.cycles += 3;
        let this = r32(c, S2);
        c.sw(this, c.g[S4][0]);
        let none = (c.g[S3][0] as i64) <= 0;
        c.g[T2][0] = sx(r32(c, S3) << 1);
        if !none {
            c.cycles += 4;
            c.g[A3][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = 0;
            c.g[A3][0] |= 0xffff;
            c.g[A2][0] = c.g[S3][0];
            loop {
                c.cycles += 14;
                c.g[A0][0] = c.lw(this);
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(ENTRY_SIZE));
                c.sw(r32(c, A0).wrapping_add(4), c.g[V0][0]);
                c.g[V1][0] = c.lw(this);
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[A0][0] = sx(r32(c, V1).wrapping_sub(ENTRY_SIZE));
                c.sw(r32(c, V1).wrapping_add(8), c.g[A0][0]);
                c.g[V0][0] = c.lw(this);
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), c.g[A3][0]);
                let more = c.g[A2][0] != 0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(ENTRY_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_7920);
                }
            }
        }
        // Close the ring, LRU head, bucket table.
        c.cycles += 29;
        c.g[T0][0] = ENTRY_SIZE as u64;
        c.g[T1][0] = c.lw(this);
        c.mult(0, T0, S3, T0);
        c.g[T2][0] = sx(r32(c, T2).wrapping_add(0xd));
        c.g[A1][0] = 0x49_0000;
        c.g[A0][0] = sx(r32(c, T2) << 2);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1af8));
        c.g[A2][0] = 0xffff_ffff_8000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, T0)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(ENTRY_SIZE));
        c.sw(r32(c, T1).wrapping_add(8), c.g[V0][0]);
        c.g[V1][0] = c.lw(this);
        c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
        c.sw(r32(c, T0).wrapping_sub(0x1ccc), c.g[V1][0]);
        c.sw(this.wrapping_add(0xc), c.g[T2][0]);
        c.g[V0][0] = c.lw(this);
        c.sw(this.wrapping_add(4), c.g[V0][0]);
        NEW_NAMED.call(c, 0x0032_79a4)?;
        c.sw(this.wrapping_add(8), c.g[V0][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Zeroes `v1 + 1` words downwards from `v0` (the `sw 0, 0(v0); v1 -= 1;
/// v0 -= 4` loops of `AI_Init`), leaving v0/v1 as the loop does.
fn ai_init_clear_words(c: &mut Cpu, head: u32) {
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void AI_Init(self)` (0x128818): resets the AI manager. Resets the focus
/// and danger managers (`self + 0xa4`, `+0xa8`), clears the state fields
/// (+0x10, 0x14, 0x90, 0x78..0x88, 0x1c, 0x98, 0), sets race state 1, clears
/// the arrays at +0x28 (6 words), +0x70 (1), +0x48 (5) and +0x20 (2), loads the
/// anim data sources, calls the object's virtual method 6 and resets the
/// riders (`AI_ResetRiders(self, 1)`). Frame 0x30 (s0, s1).
///
/// NOT installed: `AI_LoadAnimDataSources` (direct callee, loads files
/// through callees that have no override slot) cannot be bounded in a test.
pub fn ai_init(c: &mut Cpu) {
    const FOCUS_MAN_RESET: Callee = Callee {
        addr: 0x0010_3358,
        f: ee::s00::f_00103358,
    };
    const DANGER_MAN_RESET: Callee = Callee {
        addr: 0x0010_1688,
        f: ee::s00::f_00101688,
    };
    const SET_RACE_STATE: Callee = Callee {
        addr: 0x0011_3b10,
        f: ee::s00::f_00113b10,
    };
    const LOAD_ANIM_DATA_SOURCES: Callee = Callee {
        addr: 0x0012_b280,
        f: ee::s00::f_0012b280,
    };
    const RESET_RIDERS: Callee = Callee {
        addr: 0x0012_97c8,
        f: ee::s00::f_001297c8,
    };
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[A0][0] = c.lw(this.wrapping_add(0xa4));
        FOCUS_MAN_RESET.call(c, 0x0012_8834)?;
        c.g[A0][0] = c.lw(this.wrapping_add(0xa8));
        DANGER_MAN_RESET.call(c, 0x0012_883c)?;
        c.sw(this.wrapping_add(0x10), 0);
        c.g[A0][0] = c.g[S0][0];
        c.sw(this.wrapping_add(0x14), 0);
        c.g[A1][0] = 1;
        for off in [0x90, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x1c, 0x98] {
            c.sw(this.wrapping_add(off), 0);
        }
        c.sw(this, 0);
        SET_RACE_STATE.call(c, 0x0012_8874)?;
        c.g[V1][0] = 5;
        c.g[V0][0] = sx(this.wrapping_add(0x3c));
        ai_init_clear_words(c, 0x0012_8880);
        c.cycles += 5;
        c.g[A0][0] = sx(this.wrapping_add(0x48));
        c.g[S1][0] = sx(this.wrapping_add(0x20));
        c.g[V1][0] = 0;
        c.g[V0][0] = sx(this.wrapping_add(0x70));
        ai_init_clear_words(c, 0x0012_88b0);
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x10));
        c.g[V1][0] = 4;
        ai_init_clear_words(c, 0x0012_88d8);
        c.cycles += 13;
        c.g[A0][0] = c.g[S0][0];
        LOAD_ANIM_DATA_SOURCES.call(c, 0x0012_88fc)?;
        c.g[V0][0] = c.lw(this.wrapping_add(0xcc));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x30));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x34));
        let method = r32(c, V1);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0012_8910)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        RESET_RIDERS.call(c, 0x0012_891c)?;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(4));
        c.g[V1][0] = 1;
        ai_init_clear_words(c, 0x0012_8928);
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BELibrary* BELibrary_ctor(self)` (0x149860): loads the character DB
/// (`BELibrary_LoadCharDB`), copies the 0xa28-byte default block at 0x43fb28
/// to 0x535c18 (8-byte aligned: the aligned 32-byte copy loop always runs; 81
/// iterations plus 8 bytes) and creates every interface singleton, in order
/// (race, player, char DB, char, option, save, BAGT, network, score, bolt,
/// mission, econ, aggression, reward). Returns `self`. Frame 0x20 (s0).
///
/// NOT installed: no test. `BELibrary_LoadCharDB` is a direct callee with no
/// override slot, and its own callees (`RewardsFile_Load`, the async file
/// open) cannot be bounded on test data.
pub fn be_library_ctor(c: &mut Cpu) {
    const LOAD_CHAR_DB: Callee = Callee {
        addr: 0x0014_9bb8,
        f: ee::s01::f_00149bb8,
    };
    const INSTANCES: [(Callee, u32); 14] = [
        (
            Callee {
                addr: 0x0014_48d8,
                f: ee::s01::f_001448d8,
            },
            0x0014_9928,
        ),
        (
            Callee {
                addr: 0x0014_5970,
                f: ee::s01::f_00145970,
            },
            0x0014_9930,
        ),
        (
            Callee {
                addr: 0x0014_edd8,
                f: ee::s01::f_0014edd8,
            },
            0x0014_9938,
        ),
        (
            Callee {
                addr: 0x0014_7f78,
                f: ee::s01::f_00147f78,
            },
            0x0014_9940,
        ),
        (
            Callee {
                addr: 0x0014_f250,
                f: ee::s01::f_0014f250,
            },
            0x0014_9948,
        ),
        (
            Callee {
                addr: 0x0015_2638,
                f: ee::s01::f_00152638,
            },
            0x0014_9950,
        ),
        (
            Callee {
                addr: 0x0014_f900,
                f: ee::s01::f_0014f900,
            },
            0x0014_9958,
        ),
        (
            Callee {
                addr: 0x0014_e0e0,
                f: ee::s01::f_0014e0e0,
            },
            0x0014_9960,
        ),
        (
            Callee {
                addr: 0x0015_4948,
                f: ee::s01::f_00154948,
            },
            0x0014_9968,
        ),
        (
            Callee {
                addr: 0x0014_abe0,
                f: ee::s01::f_0014abe0,
            },
            0x0014_9970,
        ),
        (
            Callee {
                addr: 0x0015_32a0,
                f: ee::s01::f_001532a0,
            },
            0x0014_9978,
        ),
        (
            Callee {
                addr: 0x0015_08d8,
                f: ee::s01::f_001508d8,
            },
            0x0014_9980,
        ),
        (
            Callee {
                addr: 0x0015_5a50,
                f: ee::s01::f_00155a50,
            },
            0x0014_9988,
        ),
        (
            Callee {
                addr: 0x0015_69c0,
                f: ee::s01::f_001569c0,
            },
            0x0014_9990,
        ),
    ];
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        LOAD_CHAR_DB.call(c, 0x0014_9874)?;
        c.g[V0][0] = 0x53_0000;
        c.g[V1][0] = 0x44_0000;
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x5c18));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4d8));
        c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] &= 7;
        let unaligned = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0xa20));
        debug_assert!(!unaligned, "0x43fb28 and 0x535c18 are 8-byte aligned");
        loop {
            c.cycles += 11;
            let src = r32(c, V1);
            let dst = r32(c, A0);
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_98e8);
            }
        }
        c.cycles += 36;
        let src = r32(c, V1);
        let dst = r32(c, A0);
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        for (callee, ret) in INSTANCES {
            callee.call(c, ret)?;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn camera_bound_obj_get_rot_matches() {
        check(
            ee::s02::f_00176448__generated,
            camera_bound_obj_get_rot_x,
            100,
            &|c, seed| {
                let mut r = Rng(0x176448 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
        check(
            ee::s02::f_00176450__generated,
            camera_bound_obj_get_rot_y,
            100,
            &|c, seed| {
                let mut r = Rng(0x176450 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    /// Streamer attach: random section/node counts (0..3), indices that are
    /// negative or in range, so every branch of the three rebasing steps runs.
    #[test]
    fn cworld_cache_streamer_attach_sdb_matches() {
        check(
            ee::s13::f_003a9000__generated,
            cworld_cache_streamer_attach_sdb,
            300,
            &|c, seed| {
                let mut r = Rng(0x3a9000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA;
                let hdr = DATA + 0x2000;
                let nsect = r.next_u32() % 4;
                let nnodes = r.next_u32() % 4;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = hdr as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(hdr + 8, nsect);
                c.bus.write32(hdr + 0xc, nnodes);
                let sects = hdr + 0x50;
                let nodes = (sects + nsect * 0x58 + 0xf) & !0xf;
                for i in 0..4u32 {
                    // section node index (any small value keeps the pointer in RAM)
                    c.bus.write32(sects + i * 0x58 + 0x1c, r.next_u32() % 4);
                }
                for i in 0..4u32 {
                    for off in [0x50u32, 0x54, 0x58] {
                        let v = if r.next_u32() % 3 == 0 {
                            0xffff_ffff
                        } else {
                            r.next_u32() % 4
                        };
                        c.bus.write32(nodes + i * 0x60 + off, v);
                    }
                }
            },
        );
    }

    use crate::difftest::fakes::{self, machine};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the oc432 fakes act only on machines whose RAM holds
    /// `OC432_MAGIC` here (the suite runs tests in parallel; the slots are
    /// global).
    pub(crate) const OC432_CTRL: u32 = 0x00a0_0000;
    pub(crate) const OC432_MAGIC: u32 = 0x4f43_4333;

    /// Stamps a gated fake for `$addr` that logs its call (`fakes::fake`) and
    /// falls back to the previous body in the slot when the gate is off.
    macro_rules! oc432_fake {
        ($name:ident, $prev:ident, $gen:path, $addr:expr) => {
            oc432_fake!($name, $prev, $gen, $addr, |_c: &mut Cpu| {});
        };
        ($name:ident, $prev:ident, $gen:path, $addr:expr, $post:expr) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC432_CTRL) != OC432_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                if c.pc == ra {
                    ($post)(c);
                }
            }
        };
    }

    oc432_fake!(
        fake_play_anim,
        PREV_PLAY_ANIM,
        ee::s10::f_003128e8__generated,
        0x0031_28e8
    );

    #[test]
    fn rider_wipeout_sub_play_final_anim_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003128E8),
            fake_play_anim,
            &PREV_PLAY_ANIM,
        );
        fakes::covered(
            ee::s00::f_0012e468__generated,
            rider_wipeout_sub_play_final_anim,
            120,
            &|c, seed| {
                crate::difftest::elf_image(c);
                let mut r = machine(c, seed, 0x12e468, 2);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.pc = 0x0012_e468;
                let obj = DATA;
                let rider = DATA + 0x100;
                let base = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.bus.write32(obj + 0x80, rider);
                c.bus.write32(rider + 0x784, base);
                // Layer 2's anim id: 0x178..=0x182 covers every table entry and
                // both sides of the range check.
                c.bus.write32(base + 8, 0x178 + seed % 11);
                c.g[RA][0] = crate::difftest::RA as u64;
                let _ = &mut r;
            },
        );
    }

    oc432_fake!(
        fake_get_be,
        PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );
    oc432_fake!(
        fake_be_get_interface,
        PREV_BE_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );
    oc432_fake!(
        fake_get_human_rider,
        PREV_GET_HUMAN_RIDER,
        ee::s01::f_00146e98__generated,
        0x0014_6e98
    );
    oc432_fake!(
        fake_get_character,
        PREV_GET_CHARACTER,
        ee::s01::f_00147398__generated,
        0x0014_7398
    );
    oc432_fake!(
        fake_get_peak_rival,
        PREV_GET_PEAK_RIVAL,
        ee::s01::f_00145750__generated,
        0x0014_5750
    );
    oc432_fake!(
        fake_game_rand_next,
        PREV_GAME_RAND_NEXT,
        ee::s06::f_00237cd8__generated,
        0x0023_7cd8
    );
    oc432_fake!(
        fake_game_rand_shuffle,
        PREV_GAME_RAND_SHUFFLE,
        ee::s06::f_0023c770__generated,
        0x0023_c770
    );

    /// `GMHandler1_PickOpponents`: the BE getters, the rand and the shuffle
    /// are logging fakes; the two characters come from forced results (some
    /// equal to loop indices, so the skip branches run, some out of range).
    #[test]
    fn gm_handler1_pick_opponents_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            fake_get_human_rider,
            &PREV_GET_HUMAN_RIDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147398),
            fake_get_character,
            &PREV_GET_CHARACTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145750),
            fake_get_peak_rival,
            &PREV_GET_PEAK_RIVAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00237CD8),
            fake_game_rand_next,
            &PREV_GAME_RAND_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0023C770),
            fake_game_rand_shuffle,
            &PREV_GAME_RAND_SHUFFLE,
        );
        fakes::covered(
            ee::s06::f_0023a4f0__generated,
            gm_handler1_pick_opponents,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23a4f0, 10);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.g[A0][0] = DATA as u64;
                let pick =
                    |r: &mut Rng| [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 20][(r.next_u32() % 10) as usize];
                c.bus.write32(fakes::LOG_RESULTS + 4 * 5, pick(&mut r));
                c.bus.write32(fakes::LOG_RESULTS + 4 * 6, pick(&mut r));
                c.bus.write32(fakes::LOG_RESULTS + 4 * 7, r.next_u32() | 1);
                let _ = seed;
            },
        );
    }

    oc432_fake!(
        fake_streamer_get_section,
        PREV_STREAMER_GET_SECTION,
        ee::s13::f_003a9820__generated,
        0x003a_9820
    );
    oc432_fake!(
        fake_streamer_section_count,
        PREV_STREAMER_SECTION_COUNT,
        ee::s13::f_003a9800__generated,
        0x003a_9800,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 4) as u64;
        }
    );
    oc432_fake!(
        fake_get_section_entry,
        PREV_GET_SECTION_ENTRY,
        ee::s01::f_00144d38__generated,
        0x0014_4d38
    );
    oc432_fake!(
        fake_strcmp,
        PREV_STRCMP,
        ee::s15::f_004165a8__generated,
        0x0041_65a8,
        |c: &mut Cpu| {
            let n = c.bus.read32(fakes::LOG_COUNT);
            c.g[V0][0] = (n % 3 != 0) as u64;
        }
    );
    oc432_fake!(
        fake_section_init_node_pool,
        PREV_SECTION_INIT_NODE_POOL,
        ee::s05::f_0022e228__generated,
        0x0022_e228
    );

    /// `SectionMan_Init`: the BE getters, the streamer's section accessors,
    /// strcmp and the node pool init are logging fakes (the section count is
    /// 0..2 and strcmp matches on some calls, so the map update and both loop
    /// back edges run; fake unwinds exercise the call-unwind exits).
    #[test]
    fn section_man_init_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9820),
            fake_streamer_get_section,
            &PREV_STREAMER_GET_SECTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9800),
            fake_streamer_section_count,
            &PREV_STREAMER_SECTION_COUNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144D38),
            fake_get_section_entry,
            &PREV_GET_SECTION_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            fake_strcmp,
            &PREV_STRCMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E228),
            fake_section_init_node_pool,
            &PREV_SECTION_INIT_NODE_POOL,
        );
        fakes::covered(ee::s05::f_0022cd40__generated, section_man_init, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x22cd40, 60);
            c.bus.write32(OC432_CTRL, OC432_MAGIC);
            c.bus.write32(OC432_CTRL + 4, r.next_u32() % 3);
            c.pc = 0x0022_cd40;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x1b0, DATA + 0x400);
            c.bus.write32(DATA + 0x410, DATA + 0x500);
            c.bus.write32(DATA + 0x500, DATA + 0x600);
        });
    }

    oc432_fake!(
        fake_delete,
        PREV_DELETE,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );
    oc432_fake!(
        fake_trigger_reset_header,
        PREV_TRIGGER_RESET_HEADER,
        ee::s02::f_0016c390__generated,
        0x0016_c390
    );
    oc432_fake!(
        fake_trigger_read_header,
        PREV_TRIGGER_READ_HEADER,
        ee::s02::f_0016c3d0__generated,
        0x0016_c3d0,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 8) as u64;
        }
    );
    oc432_fake!(
        fake_trigger_add,
        PREV_TRIGGER_ADD,
        ee::s02::f_0016c120__generated,
        0x0016_c120
    );
    oc432_fake!(
        fake_operator_new_named,
        PREV_OPERATOR_NEW_NAMED,
        ee::s10::f_00317e30__generated,
        0x0031_7e30,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x6_0000) as u64;
        }
    );
    oc432_fake!(
        fake_mem_alloc_named,
        PREV_MEM_ALLOC_NAMED,
        ee::s10::f_00317d70__generated,
        0x0031_7d70,
        |c: &mut Cpu| {
            if c.bus.read32(OC432_CTRL + 0x18) != 0 {
                c.g[V0][0] = 0;
            }
        }
    );
    oc432_fake!(
        fake_read_u32_2,
        PREV_READ_U32_2,
        ee::s09::f_002d2558__generated,
        0x002d_2558
    );
    oc432_fake!(
        fake_read_sphere_trigger,
        PREV_READ_SPHERE_TRIGGER,
        ee::s02::f_00172e18__generated,
        0x0017_2e18,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x10) as u64;
        }
    );
    oc432_fake!(
        fake_read_action,
        PREV_READ_ACTION,
        ee::s02::f_001712b0__generated,
        0x0017_12b0,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x14) as u64;
        }
    );

    pub(crate) fn camera_trigger_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            fake_delete,
            &PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C390),
            fake_trigger_reset_header,
            &PREV_TRIGGER_RESET_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C3D0),
            fake_trigger_read_header,
            &PREV_TRIGGER_READ_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C120),
            fake_trigger_add,
            &PREV_TRIGGER_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            fake_operator_new_named,
            &PREV_OPERATOR_NEW_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            fake_mem_alloc_named,
            &PREV_MEM_ALLOC_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2558),
            fake_read_u32_2,
            &PREV_READ_U32_2,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00172E18),
            fake_read_sphere_trigger,
            &PREV_READ_SPHERE_TRIGGER,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001712B0),
            fake_read_action,
            &PREV_READ_ACTION,
        );
    }

    /// `CameraTriggerMan_Parse`: the stream readers, allocator, header
    /// reset/read and AddTrigger are fakes; ReadTrigger and ReadVersion run
    /// real on top of them (version 7 or not, header status 0 or not, 0..3
    /// triggers, allocation that fails, sphere/action reads that fail).
    #[test]
    fn camera_trigger_man_parse_matches() {
        camera_trigger_install_fakes();
        fakes::covered(
            ee::s02::f_0016c228__generated,
            camera_trigger_man_parse,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16c228, 14);
                let this = DATA + 0x5_0000;
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 8, if r.next_u32() % 4 == 0 { 5 } else { 0 });
                c.bus
                    .write32(OC432_CTRL + 0x10, (r.next_u32() % 5 != 0) as u32);
                c.bus
                    .write32(OC432_CTRL + 0x14, (r.next_u32() % 5 != 0) as u32);
                c.bus
                    .write32(OC432_CTRL + 0x18, (r.next_u32() % 6 == 0) as u32);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x5_8000) as u64;
                c.bus.write32(this + 0x4, r.next_u32() % 4);
                c.bus
                    .write32(this + 0xc, if r.next_u32() % 5 == 0 { 3 } else { 7 });
            },
        );
    }

    /// `CameraTriggerMan_FreeTriggers`: 0..3 triggers whose buffer (+8) and
    /// parts (+0xc, +0x10) are set or not (parts have a vtable whose
    /// destructor is a fake), some trigger slots null.
    #[test]
    fn camera_trigger_man_free_triggers_matches() {
        camera_trigger_install_fakes();
        fakes::covered(
            ee::s02::f_0016bf40__generated,
            camera_trigger_man_free_triggers,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16bf40, 14);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                let this = DATA + 0x5_0000;
                let arr = this + 0x100;
                let vt = this + 0x3000;
                let n = r.next_u32() % 4;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, arr);
                c.bus.write32(this + 4, n);
                c.bus.write16(vt + 8, (r.next_u32() % 64) as u16);
                c.bus.write32(vt + 0xc, fakes::FAKE);
                for i in 0..4u32 {
                    let trig = if r.next_u32() % 6 == 0 {
                        0
                    } else {
                        this + 0x200 + i * 0x40
                    };
                    c.bus.write32(arr + 4 * i, trig);
                    for (k, off) in [0x8u32, 0xc, 0x10].into_iter().enumerate() {
                        let v = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            let part = this + 0x1000 + (i * 3 + k as u32) * 0x40;
                            c.bus.write32(part + 4, vt);
                            part
                        };
                        c.bus.write32(this + 0x200 + i * 0x40 + off, v);
                    }
                }
            },
        );
    }

    /// `ProcessList_Init`: n = 0..4 buckets (including the empty case); the
    /// allocation is a fake returning a pointer into the data area.
    #[test]
    fn process_list_init_matches() {
        camera_trigger_install_fakes();
        fakes::covered(ee::s11::f_003549e0__generated, process_list_init, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3549e0, 3);
            c.bus.write32(OC432_CTRL, OC432_MAGIC);
            c.g[A0][0] = (DATA + 0x5_0000) as u64;
            c.g[A1][0] = (r.next_u32() % 5) as u64;
            let _ = seed;
        });
    }

    oc432_fake!(
        fake_game_mode_reset,
        PREV_GAME_MODE_RESET,
        ee::s06::f_002382d8__generated,
        0x0023_82d8
    );

    /// `GameModeMan_StartMode`: every event type 0..14 on both network paths
    /// (byte at 0x535c11 equal to 2 or not); the current handler's virtual
    /// method is a fake, reset mode is a fake.
    #[test]
    fn game_mode_man_start_mode_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002382D8),
            fake_game_mode_reset,
            &PREV_GAME_MODE_RESET,
        );
        fakes::covered(
            ee::s06::f_00238160__generated,
            game_mode_man_start_mode,
            200,
            &|c, seed| {
                crate::difftest::elf_image(c);
                let mut r = machine(c, seed, 0x238160, 4);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.pc = 0x0023_8160;
                let this = DATA + 0x5_0000;
                let handler = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let cur = r.next_u32() % 10;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 15) as u64;
                c.bus.write32(this + 4, cur);
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6668 + 4 * i, handler);
                }
                c.bus.write32(handler, vt);
                c.bus.write16(vt + 0x18, (r.next_u32() % 32) as u16);
                c.bus.write32(vt + 0x1c, fakes::FAKE);
                c.bus
                    .write8(0x0053_5c11, if r.next_u32() % 2 == 0 { 2 } else { 1 });
            },
        );
    }

    /// `ScriptCmdQueue_Push`: full/not-full queue, head 0..4 (wrapping), tail
    /// equal to the next head or not; the command lives at an unaligned
    /// address.
    #[test]
    fn script_cmd_queue_push_matches() {
        check(
            ee::s07::f_00283c80__generated,
            script_cmd_queue_push,
            200,
            &|c, seed| {
                let mut r = Rng(0x283c80 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let this = DATA + 0x5_0000;
                let cmd = DATA + 0x5_1000 + seed % 8;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = cmd as u64;
                let head = r.next_u32() % 5;
                c.bus.write32(this + 0x90, head);
                c.bus.write32(this + 0x94, (r.next_u32() % 4 == 0) as u32);
                let next = if head == 4 { 0 } else { head + 1 };
                c.bus.write32(
                    this + 0x8c,
                    if r.next_u32() % 2 == 0 {
                        next
                    } else {
                        r.next_u32() % 5
                    },
                );
                crate::difftest::fill(c, &mut r, cmd, 7);
            },
        );
    }

    oc432_fake!(
        fake_avalanche_destroy_all,
        PREV_AVALANCHE_DESTROY_ALL,
        ee::s09::f_002d9b40__generated,
        0x002d_9b40
    );
    oc432_fake!(
        fake_wscript_reset_collect_set,
        PREV_WSCRIPT_RESET_COLLECT_SET,
        ee::s10::f_0030c390__generated,
        0x0030_c390
    );
    oc432_fake!(
        fake_wscript_call_func,
        PREV_WSCRIPT_CALL_FUNC,
        ee::s10::f_00309c88__generated,
        0x0030_9c88
    );

    /// `WScriptMan_Shutdown`: 0..3 world sections of type 3 or not, with or
    /// without a script cache, 0..3 global shutdown functions each (the two
    /// world accessors run real on the fixture's tables).
    #[test]
    fn wscript_man_shutdown_matches() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9B40),
            fake_avalanche_destroy_all,
            &PREV_AVALANCHE_DESTROY_ALL,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030C390),
            fake_wscript_reset_collect_set,
            &PREV_WSCRIPT_RESET_COLLECT_SET,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00309C88),
            fake_wscript_call_func,
            &PREV_WSCRIPT_CALL_FUNC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9800),
            fake_streamer_section_count,
            &PREV_STREAMER_SECTION_COUNT,
        );
        fakes::covered(
            ee::s10::f_00309f18__generated,
            wscript_man_shutdown,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x309f18, 12);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                let nsect = r.next_u32() % 4;
                c.bus.write32(OC432_CTRL + 4, nsect);
                let this = DATA + 0x5_0000;
                let world = DATA + 0x5_1000;
                let view = DATA + 0x5_2000;
                let info = DATA + 0x5_3000;
                let sections = DATA + 0x5_4000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x28c, world);
                c.bus.write32(world, view);
                c.bus.write32(world + 4, info);
                c.bus.write32(info + 4, sections);
                for i in 0..3u32 {
                    let sect = sections + i * 0xc;
                    let cache = DATA + 0x5_5000 + i * 0x40;
                    let ty = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xffff
                    } else {
                        3
                    };
                    c.bus.write16(sect, ty as u16);
                    c.bus
                        .write32(sect + 8, if r.next_u32() % 5 == 0 { 0 } else { cache });
                    c.bus.write32(cache + 0x10, r.next_u32() % 4);
                    c.bus.write32(
                        cache + 0x14,
                        if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            DATA + 0x5_6000 + i * 0x40
                        },
                    );
                }
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    oc432_fake!(
        fake_script_pop_entries,
        PREV_SCRIPT_POP_ENTRIES,
        ee::s07::f_00277598__generated,
        0x0027_7598
    );
    oc432_fake!(
        fake_script_cmd_queue_at,
        PREV_SCRIPT_CMD_QUEUE_AT,
        ee::s07::f_00283d70__generated,
        0x0028_3d70
    );
    oc432_fake!(
        fake_script_is_movie_entry,
        PREV_SCRIPT_IS_MOVIE_ENTRY,
        ee::s07::f_00277dd8__generated,
        0x0027_7dd8,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x20) as u64;
        }
    );
    oc432_fake!(
        fake_script_play_instance,
        PREV_SCRIPT_PLAY_INSTANCE,
        ee::s07::f_00279720__generated,
        0x0027_9720
    );
    oc432_fake!(
        fake_script_prepare_entry,
        PREV_SCRIPT_PREPARE_ENTRY,
        ee::s07::f_00277450__generated,
        0x0027_7450
    );
    oc432_fake!(
        fake_script_list_stop,
        PREV_SCRIPT_LIST_STOP,
        ee::s07::f_00276868__generated,
        0x0027_6868
    );
    oc432_fake!(
        fake_get_transition_man,
        PREV_GET_TRANSITION_MAN,
        ee::s06::f_00230698__generated,
        0x0023_0698,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_6000) as u64;
        }
    );
    oc432_fake!(
        fake_transition_time_remaining,
        PREV_TRANSITION_TIME_REMAINING,
        ee::s09::f_002e4678__generated,
        0x002e_4678
    );
    oc432_fake!(
        fake_transition_start_out,
        PREV_TRANSITION_START_OUT,
        ee::s09::f_002ea780__generated,
        0x002e_a780
    );

    /// `ScriptList_NextEntry`: pending entry (> 0 or not), movie or script
    /// command (the movie start runs real and returns early on a null
    /// movie), `self + 0xb0` on/off, fade on/off, transition running or not.
    #[test]
    fn script_list_next_entry_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277598),
            fake_script_pop_entries,
            &PREV_SCRIPT_POP_ENTRIES,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283D70),
            fake_script_cmd_queue_at,
            &PREV_SCRIPT_CMD_QUEUE_AT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277DD8),
            fake_script_is_movie_entry,
            &PREV_SCRIPT_IS_MOVIE_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00279720),
            fake_script_play_instance,
            &PREV_SCRIPT_PLAY_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277450),
            fake_script_prepare_entry,
            &PREV_SCRIPT_PREPARE_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            fake_script_list_stop,
            &PREV_SCRIPT_LIST_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            fake_get_transition_man,
            &PREV_GET_TRANSITION_MAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E4678),
            fake_transition_time_remaining,
            &PREV_TRANSITION_TIME_REMAINING,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002EA780),
            fake_transition_start_out,
            &PREV_TRANSITION_START_OUT,
        );
        fakes::covered(
            ee::s07::f_00277838__generated,
            script_list_next_entry,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x277838, 9);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 0x20, (r.next_u32() % 3 == 0) as u32);
                let this = DATA + 0x5_0000;
                let engine = DATA + 0x5_1000;
                let vt = DATA + 0x5_2000;
                let world = DATA + 0x5_3000;
                let game = DATA + 0x5_4000;
                let fade = DATA + 0x5_5000;
                let tm = DATA + 0x5_6000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, engine);
                c.bus.write32(engine + 0x2a8, vt);
                c.bus.write16(vt + 0x30, (r.next_u32() % 32) as u16);
                c.bus.write32(vt + 0x34, fakes::FAKE);
                c.bus.write32(
                    this + 0xbc,
                    [0xffff_ffffu32, 0, 1, 5][(r.next_u32() % 4) as usize],
                );
                c.bus.write32(this + 0xb0, (r.next_u32() % 2) as u32);
                c.bus.write32(this + 0x4, (r.next_u32() % 3 != 0) as u32);
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x64, fade);
                c.bus.write32(tm + 0x44, (r.next_u32() % 2) as u32);
            },
        );
    }

    oc432_fake!(
        fake_in_seq_heap_init,
        PREV_IN_SEQ_HEAP_INIT,
        ee::s07::f_002722e0__generated,
        0x0027_22e0
    );
    oc432_fake!(
        fake_in_seq_heap_new,
        PREV_IN_SEQ_HEAP_NEW,
        ee::s07::f_00272488__generated,
        0x0027_2488,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_8000) as u64;
        }
    );
    oc432_fake!(
        fake_in_seq_instance_ctor,
        PREV_IN_SEQ_INSTANCE_CTOR,
        ee::s07::f_00274518__generated,
        0x0027_4518
    );

    /// `InSeq_ctor`: 0..4 items (including the empty pool); the heap init,
    /// the array allocation and the item constructor are fakes.
    #[test]
    fn in_seq_ctor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002722E0),
            fake_in_seq_heap_init,
            &PREV_IN_SEQ_HEAP_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00272488),
            fake_in_seq_heap_new,
            &PREV_IN_SEQ_HEAP_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274518),
            fake_in_seq_instance_ctor,
            &PREV_IN_SEQ_INSTANCE_CTOR,
        );
        fakes::covered(ee::s07::f_00272788__generated, in_seq_ctor, 160, &|c, seed| {
            let mut r = machine(c, seed, 0x272788, 6);
            c.bus.write32(OC432_CTRL, OC432_MAGIC);
            c.g[A0][0] = (DATA + 0x5_0000) as u64;
            c.g[A3][0] = (r.next_u32() % 5) as u64;
            let _ = seed;
        });
    }

    oc432_fake!(
        fake_get_track_info,
        PREV_GET_TRACK_INFO,
        ee::s01::f_00144bc0__generated,
        0x0014_4bc0,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_9000) as u64;
        }
    );
    oc432_fake!(
        fake_bxstring_ctor_cstr,
        PREV_BXSTRING_CTOR_CSTR,
        ee::s10::f_003182d8__generated,
        0x0031_82d8
    );
    oc432_fake!(
        fake_bxstring_plus_cstr_str,
        PREV_BXSTRING_PLUS_CSTR_STR,
        ee::s10::f_003186d0__generated,
        0x0031_86d0
    );
    oc432_fake!(
        fake_bxstring_plus_str_cstr,
        PREV_BXSTRING_PLUS_STR_CSTR,
        ee::s10::f_00318630__generated,
        0x0031_8630
    );
    oc432_fake!(
        fake_bxstring_dtor,
        PREV_BXSTRING_DTOR,
        ee::s10::f_00318260__generated,
        0x0031_8260
    );
    oc432_fake!(
        fake_bxstring_assign,
        PREV_BXSTRING_ASSIGN,
        ee::s10::f_00318448__generated,
        0x0031_8448
    );
    oc432_fake!(
        fake_script_load_bank_async,
        PREV_SCRIPT_LOAD_BANK_ASYNC,
        ee::s07::f_0027b370__generated,
        0x0027_b370
    );
    oc432_fake!(
        fake_eamutex_lock,
        PREV_EAMUTEX_LOCK,
        ee::s14::f_003e5700__generated,
        0x003e_5700
    );
    oc432_fake!(
        fake_eamutex_unlock,
        PREV_EAMUTEX_UNLOCK,
        ee::s14::f_003e5760__generated,
        0x003e_5760
    );
    oc432_fake!(
        fake_async_file_assign_serial,
        PREV_ASYNC_FILE_ASSIGN_SERIAL,
        ee::s14::f_003def40__generated,
        0x003d_ef40
    );
    oc432_fake!(
        fake_eafile_queue_open,
        PREV_EAFILE_QUEUE_OPEN,
        ee::s14::f_003dd4e8__generated,
        0x003d_d4e8,
        |c: &mut Cpu| {
            if c.bus.read32(OC432_CTRL + 0x24) != 0 {
                c.g[V0][0] = 0;
            }
        }
    );
    oc432_fake!(
        fake_eafile_set_request_callback,
        PREV_EAFILE_SET_REQUEST_CALLBACK,
        ee::s14::f_003dcf70__generated,
        0x003d_cf70
    );

    /// `ScriptEngine_LoadWorldScripts`: the string operations, track info and
    /// bank load are fakes; the async file open (`0x3df690`) runs real on a
    /// fixture file pool that is empty or has a free node, with a queue-open
    /// that fails or not.
    #[test]
    fn script_engine_load_world_scripts_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
            fake_get_track_info,
            &PREV_GET_TRACK_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            fake_bxstring_ctor_cstr,
            &PREV_BXSTRING_CTOR_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            fake_bxstring_plus_cstr_str,
            &PREV_BXSTRING_PLUS_CSTR_STR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            fake_bxstring_plus_str_cstr,
            &PREV_BXSTRING_PLUS_STR_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            fake_bxstring_dtor,
            &PREV_BXSTRING_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318448),
            fake_bxstring_assign,
            &PREV_BXSTRING_ASSIGN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027B370),
            fake_script_load_bank_async,
            &PREV_SCRIPT_LOAD_BANK_ASYNC,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            fake_eamutex_lock,
            &PREV_EAMUTEX_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            fake_eamutex_unlock,
            &PREV_EAMUTEX_UNLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEF40),
            fake_async_file_assign_serial,
            &PREV_ASYNC_FILE_ASSIGN_SERIAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            fake_eafile_queue_open,
            &PREV_EAFILE_QUEUE_OPEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
            fake_eafile_set_request_callback,
            &PREV_EAFILE_SET_REQUEST_CALLBACK,
        );
        fakes::covered(
            ee::s07::f_00278b98__generated,
            script_engine_load_world_scripts,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x278b98, 16);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 0x24, (r.next_u32() % 3 == 0) as u32);
                let this = DATA + 0x5_0000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x620, r.next_u32() % 4);
                // The async file pool (0x519bf8): a free node or none.
                let node = DATA + 0x5_a000;
                c.bus
                    .write32(0x0051_9bf8, if r.next_u32() % 3 == 0 { 0 } else { node });
                c.bus.write32(node + 4, 0);
                c.bus.write32(node, r.next_u32());
            },
        );
    }

    oc432_fake!(
        fake_replay_reset,
        PREV_REPLAY_RESET,
        ee::s07::f_0026f228__generated,
        0x0026_f228
    );
    oc432_fake!(
        fake_script_unload_world,
        PREV_SCRIPT_UNLOAD_WORLD,
        ee::s07::f_00278cd8__generated,
        0x0027_8cd8
    );
    oc432_fake!(
        fake_ai_reset_computer_slots,
        PREV_AI_RESET_COMPUTER_SLOTS,
        ee::s00::f_0012ab20__generated,
        0x0012_ab20
    );
    oc432_fake!(
        fake_ai_reset_actor_slots,
        PREV_AI_RESET_ACTOR_SLOTS,
        ee::s00::f_0012ac48__generated,
        0x0012_ac48
    );
    oc432_fake!(
        fake_script_queue_intro,
        PREV_SCRIPT_QUEUE_INTRO,
        ee::s07::f_0027aaf8__generated,
        0x0027_aaf8
    );
    oc432_fake!(
        fake_char_profile_flag_clear,
        PREV_CHAR_PROFILE_FLAG_CLEAR,
        ee::s01::f_00145d38__generated,
        0x0014_5d38,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x30) as u64;
        }
    );
    oc432_fake!(
        fake_audio_instance,
        PREV_AUDIO_INSTANCE,
        ee::s07::f_0028b180__generated,
        0x0028_b180,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_c000) as u64;
        }
    );

    /// `GameStateReset_OverlayEnter`: the world-script reload runs real on
    /// the `LoadWorldScripts` fixture (its callees are fakes), the NIS list
    /// reset runs real with `ScriptList_Stop` faked; network path on/off,
    /// location 10..17, first-time-on-track flag on/off.
    #[test]
    fn game_state_reset_overlay_enter_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F228),
            fake_replay_reset,
            &PREV_REPLAY_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278CD8),
            fake_script_unload_world,
            &PREV_SCRIPT_UNLOAD_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012AB20),
            fake_ai_reset_computer_slots,
            &PREV_AI_RESET_COMPUTER_SLOTS,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012AC48),
            fake_ai_reset_actor_slots,
            &PREV_AI_RESET_ACTOR_SLOTS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027AAF8),
            fake_script_queue_intro,
            &PREV_SCRIPT_QUEUE_INTRO,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145D38),
            fake_char_profile_flag_clear,
            &PREV_CHAR_PROFILE_FLAG_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            fake_audio_instance,
            &PREV_AUDIO_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            fake_script_list_stop,
            &PREV_SCRIPT_LIST_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
            fake_get_track_info,
            &PREV_GET_TRACK_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            fake_bxstring_ctor_cstr,
            &PREV_BXSTRING_CTOR_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            fake_bxstring_plus_cstr_str,
            &PREV_BXSTRING_PLUS_CSTR_STR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            fake_bxstring_plus_str_cstr,
            &PREV_BXSTRING_PLUS_STR_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            fake_bxstring_dtor,
            &PREV_BXSTRING_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318448),
            fake_bxstring_assign,
            &PREV_BXSTRING_ASSIGN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027B370),
            fake_script_load_bank_async,
            &PREV_SCRIPT_LOAD_BANK_ASYNC,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            fake_eamutex_lock,
            &PREV_EAMUTEX_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            fake_eamutex_unlock,
            &PREV_EAMUTEX_UNLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEF40),
            fake_async_file_assign_serial,
            &PREV_ASYNC_FILE_ASSIGN_SERIAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            fake_eafile_queue_open,
            &PREV_EAFILE_QUEUE_OPEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
            fake_eafile_set_request_callback,
            &PREV_EAFILE_SET_REQUEST_CALLBACK,
        );
        fakes::covered(
            ee::s06::f_00234f40__generated,
            game_state_reset_overlay_enter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x234f40, 40);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 0x24, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(OC432_CTRL + 0x30, (r.next_u32() % 2) as u32);
                let gp = crate::difftest::GAME_GP;
                let this = DATA + 0x5_0000;
                let engine = DATA + 0x5_1000;
                let game = DATA + 0x5_2000;
                let world = DATA + 0x5_3000;
                let x = DATA + 0x5_4000;
                let section_man = DATA + 0x5_5000;
                let ai = DATA + 0x5_6000;
                let b = DATA + 0x5_7000;
                let cc = DATA + 0x5_7100;
                let d = DATA + 0x5_7200;
                let vt = DATA + 0x5_7300;
                c.g[A0][0] = this as u64;
                c.bus.write32(gp - 0x488, game);
                c.bus.write32(gp - 0x84c, engine);
                c.bus.write32(gp - 0x848, world);
                c.bus.write32(game + 0x28, DATA + 0x5_7400);
                c.bus.write32(game + 0xc, ai);
                c.bus.write32(world + 0x84, x);
                c.bus.write32(x + 0x78, section_man);
                c.bus.write32(section_man + 0x1bc, 10 + r.next_u32() % 8);
                c.bus.write32(ai + 0x40, b);
                c.bus.write32(b + 0x18, cc);
                c.bus.write32(cc + 0x6c0, d);
                c.bus.write16(d + 0x38, (r.next_u32() % 32) as u16);
                c.bus.write32(d + 0x3c, fakes::FAKE);
                let _ = vt;
                c.bus.write8(0x0053_5c11, (r.next_u32() % 3 == 0) as u8);
                // NIS list reset: a non-zero +0x550 skips ExitScriptMode.
                c.bus.write32(engine + 0x550, 1);
                c.bus.write32(engine + 0x620, r.next_u32() % 4);
                let node = DATA + 0x5_a000;
                c.bus
                    .write32(0x0051_9bf8, if r.next_u32() % 3 == 0 { 0 } else { node });
                c.bus.write32(node + 4, 0);
            },
        );
    }

    oc432_fake!(
        fake_fe_locale_ctor,
        PREV_FE_LOCALE_CTOR,
        ee::s02::f_00195880__generated,
        0x0019_5880,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_2000) as u64;
        }
    );
    oc432_fake!(
        fake_fe_locale_load_common,
        PREV_FE_LOCALE_LOAD_COMMON,
        ee::s02::f_00195b70__generated,
        0x0019_5b70
    );

    /// `SSXApp_LoadLocale`: locale already there or created (the allocation
    /// and constructor are fakes), the options block (0x535610) with random
    /// words so every language value shows, the locale's virtual method 3 is
    /// a fake.
    #[test]
    fn ssx_app_load_locale_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            fake_mem_alloc_named,
            &PREV_MEM_ALLOC_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195880),
            fake_fe_locale_ctor,
            &PREV_FE_LOCALE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195B70),
            fake_fe_locale_load_common,
            &PREV_FE_LOCALE_LOAD_COMMON,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::covered(ee::s05::f_00228a78__generated, ssx_app_load_locale, 160, &|c, seed| {
            let mut r = machine(c, seed, 0x228a78, 8);
            c.bus.write32(OC432_CTRL, OC432_MAGIC);
            let this = DATA + 0x5_0000;
            let locale = DATA + 0x5_2000;
            let vt = DATA + 0x5_3000;
            c.g[A0][0] = this as u64;
            c.bus
                .write32(this + 0x8c, if r.next_u32() % 2 == 0 { 0 } else { locale });
            c.bus.write32(locale + 4, vt);
            c.bus.write16(vt + 0x18, (r.next_u32() % 16) as u16);
            c.bus.write32(vt + 0x1c, fakes::FAKE);
            for i in 0..(0x288 / 4) {
                c.bus.write32(0x0053_5610 + 4 * i, r.next_u32());
            }
        });
    }

    oc432_fake!(
        fake_ai_fwd_diff_entry_ctor,
        PREV_AI_FWD_DIFF_ENTRY_CTOR,
        ee::s11::f_00340970__generated,
        0x0034_0970
    );

    /// `AIFwdDiffCache_Init`: n = 0..4 entries (including the empty cache);
    /// the allocator and the entry constructor are fakes.
    #[test]
    fn ai_fwd_diff_cache_init_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            fake_operator_new_named,
            &PREV_OPERATOR_NEW_NAMED,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00340970),
            fake_ai_fwd_diff_entry_ctor,
            &PREV_AI_FWD_DIFF_ENTRY_CTOR,
        );
        fakes::covered(
            ee::s10::f_00327890__generated,
            ai_fwd_diff_cache_init,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x327890, 6);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.g[A0][0] = (DATA + 0x5_0000) as u64;
                c.g[A1][0] = (r.next_u32() % 5) as u64;
                let _ = seed;
            },
        );
    }
}
