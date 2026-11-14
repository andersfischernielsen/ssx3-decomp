//! Staging module oc331 (wave wave214): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc331`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_a418,
        spch_build_sentence_with_fallback,
        "SPCH_BuildSentenceWithFallback",
    ),
    (0x003c_e078, snd_utk_get_state, "SND_UtkGetState"),
    (0x0031_7830, rand_float_range, "Rand_FloatRange"),
    (
        0x0027_08f0,
        replay_on_rider_wipeout,
        "Replay_OnRiderWipeout",
    ),
    (
        0x002f_5a70,
        world_light_man_get_overlap_region,
        "WorldLightMan_GetOverlapRegion",
    ),
    (0x0035_6780, mod_obj_v48, "ModObj_v48"),
    (
        0x0034_b7b8,
        c_flag_manager_group_free_mesh,
        "cFlagManager_GroupFreeMesh",
    ),
    (0x003c_7ce0, snd_dec_free_thunk, "SND_DecFreeThunk"),
    (
        0x0029_f620,
        ssx_audio_system_get_grunt_bank_id,
        "SSXAudioSystem_GetGruntBankId",
    ),
    (0x002a_a320, stream_buffer_queue, "StreamBuffer_Queue"),
];

// ---------------------------------------------------------------------------
// SPCH_BuildSentenceWithFallback (0x3da418)

/// Speech sentence table entry size (`&D_0044FFF8.e[idx]`, the field at
/// `0x45006c`).
pub const SPCH_SENTENCE_STRIDE: u32 = 0x60;

/// Speech sentence entries base: `0x450000 + 0x6c`.
pub const SPCH_SENTENCE_BASE: u32 = 0x0045_006c;

/// The `lui` half of the sentence table address (`0x450000`).
pub const SPCH_TABLE_HI: u32 = 0x0045_0000;

/// The `addiu` offset of the sentence table (`0x6c`).
pub const SPCH_SENTENCE_OFF: u32 = 0x6c;

/// The fallback string callback (`D_0044FF90.cb` at `0x44ffa0`).
pub const SPCH_FALLBACK_CB: u32 = 0x0044_ffa0;

/// `SPCH_BuildSentence` (0x3d9fb0).
pub const SPCH_BUILD_SENTENCE: Callee = Callee {
    addr: 0x003d_9fb0,
    f: ee::s14::SPCH_BuildSentence,
};

/// `int SPCH_BuildSentenceWithFallback(int idx)` (0x3da418): build the
/// sentence for entry `idx` (`&0x45006c + idx*0x60`); when that yields
/// nothing, ask the fallback callback (`*(0x44ffa0)`) for successive strings
/// (index 0, 1, ...) and retry, stopping at a non-positive callback result
/// or a built sentence. Returns the sentence id, or 0 when nothing was built.
pub fn spch_build_sentence_with_fallback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[V1][0] = SPCH_SENTENCE_STRIDE as u64;
        let frame = Frame::open(c, 0x60);
        // a0 = the entry offset (idx * 0x60), then `addiu` the base.
        c.mult(0, A0, A0, V1);
        c.g[V0][0] = SPCH_TABLE_HI as u64;
        frame.save(c, S3, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SPCH_SENTENCE_OFF));
        frame.save(c, S1, 0x10);
        frame.save(c, 31, 0x50); // `sq ra`
        c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S4, 0x40);
        frame.save(c, S2, 0x20);
        c.g[A0][0] = c.g[S3][0];
        frame.save(c, S0, 0x0);
        SPCH_BUILD_SENTENCE.call(c, 0x003d_a450)?;
        c.g[S1][0] = c.g[V0][0];
        let mut built = c.g[S1][0] != 0;
        c.g[V1][0] = u64::MAX;
        if !built {
            c.cycles += 5;
            c.g[V0][0] = 0x0045_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70));
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
            let no_cb = c.g[V1][0] == 0;
            c.g[S4][0] = c.g[V0][0];
            if !no_cb {
                c.cycles += 2;
                c.g[S2][0] = 0;
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
                'fallback: loop {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    let target = c.g[V0][0] as u32;
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, target, 0x003d_a484)?;
                    c.g[S0][0] = c.g[V0][0];
                    let k = c.g[S0][0];
                    if (k as i64) >= 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S3][0];
                        SPCH_BUILD_SENTENCE.call(c, 0x003d_a498)?;
                        c.g[S1][0] = c.g[V0][0];
                    }
                    c.cycles += 2;
                    built = c.g[S1][0] != 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    if !built {
                        c.cycles += 2;
                        if (k as i64) > 0 {
                            c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x003d_a478);
                            }
                            continue 'fallback;
                        }
                    }
                    break;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = u64::MAX;
        }
        c.cycles += 11;
        c.g[V0][0] = 0;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
        frame.restore(c, 31, 0x50);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[S1][0];
        }
        frame.restore(c, S4, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkGetState (0x3ce078)

/// Tool-kit state word offsets in the sound system: `+4`, `+0xd44`, `+0xd5c`.
pub const SND_UTK_STATE_FIELDS: [u32; 3] = [0x4, 0xd44, 0xd5c];

/// `void SND_UtkGetState(void* out, void* self)` (0x3ce078): copy the three
/// tool-kit state words of `self` (`+4`, `+0xd44`, `+0xd5c`) to `out`
/// (`out[0..3]`). The copy is a 3-word store through an unaligned
/// doubleword load/store pair, as compiled.
pub fn snd_utk_get_state(c: &mut Cpu) {
    c.cycles += 16;
    let src = c.g[A1][0] as u32;
    let dst = c.g[A0][0] as u32;
    c.g[A3][0] = Addr(src.wrapping_add(SND_UTK_STATE_FIELDS[1])).lw(c);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    c.g[V1][0] = Addr(src.wrapping_add(SND_UTK_STATE_FIELDS[0])).lw(c);
    c.g[V0][0] = c.g[A0][0];
    c.g[A2][0] = Addr(src.wrapping_add(SND_UTK_STATE_FIELDS[2])).lw(c);
    c.sw(sp(c), c.g[V1][0]);
    c.sw(sp(c).wrapping_add(8), c.g[A2][0]);
    c.sw(sp(c).wrapping_add(4), c.g[A3][0]);
    c.ldl(3, sp(c).wrapping_add(7));
    c.ldr(3, sp(c));
    c.g[A1][0] = Addr(sp(c).wrapping_add(8)).lw(c);
    c.sdl(3, dst.wrapping_add(7));
    c.sdr(3, dst);
    c.sw(dst.wrapping_add(8), c.g[A1][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Rand_FloatRange (0x317830)

/// `Rand_NextGame` (0x317810).
pub const OC331_RAND_NEXT_GAME: Callee = Callee {
    addr: 0x0031_7810,
    f: ee::s10::Rand_NextGame,
};

/// `float Rand_FloatRange(float min, float max)` (0x317830): a uniform float
/// in `[min, max)`, from the game RNG's mantissa bits (`1.0 + m/2^23`).
pub fn rand_float_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        c.mov_s(21, 12); // f21 = min
        frame.save_ra(c, 0);
        c.mov_s(20, 13); // f20 = max
        OC331_RAND_NEXT_GAME.call(c, 0x0031_784c)?;
        c.g[V1][0] = 0x7f_0000;
        c.g[A0][0] = 0x3f80_0000;
        c.g[V1][0] |= 0xffff;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0); // f0 = 1.0f
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= c.g[A0][0]; // a float in [1, 2)
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.mtc1(2, 1); // f1 = that float
        let unit = FpuF32(c.f[1]) - FpuF32(c.f[0]); // x - 1.0 in [0, 1)
        c.f[20] = (FpuF32(c.f[20]) - FpuF32(c.f[21])).0; // max - min
        c.f[0] = unit.0;
        c.f[20] = (FpuF32(c.f[20]) * FpuF32(c.f[0])).0;
        c.f[0] = (FpuF32(c.f[21]) + FpuF32(c.f[20])).0;
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay_OnRiderWipeout (0x2708f0)

/// `Replay` flags that block the wipeout bookkeeping: `+0x61c` (playing
/// back), `+0x620` (instant replay), then the replay state at `+0`.
pub const OC331_REPLAY_WIPEOUT_BUSY: [u32; 3] = [0x61c, 0x620, 0x0];

/// `Replay_FindRiderIndex` (0x270730).
pub const OC331_REPLAY_FIND_RIDER_INDEX: Callee = Callee {
    addr: 0x0027_0730,
    f: ee::s07::Replay_FindRiderIndex,
};

/// `Replay_EndHighlight` (0x270ce8).
pub const OC331_REPLAY_END_HIGHLIGHT: Callee = Callee {
    addr: 0x0027_0ce8,
    f: ee::s07::Replay_EndHighlight,
};

/// `Replay_CheckInstantReplay` (0x2705b0).
pub const OC331_REPLAY_CHECK_INSTANT_REPLAY: Callee = Callee {
    addr: 0x0027_05b0,
    f: ee::s07::Replay_CheckInstantReplay,
};

/// `void Replay_OnRiderWipeout(Replay* self, int key, void* data)`
/// (0x2708f0): when the replay is idle (none of `+0x61c`, `+0x620`, `+0`
/// set), find the rider's index and, when found, close its highlight
/// (`Replay_EndHighlight(self, index, data, 0)`) and run the wipeout
/// instant-replay check (`Replay_CheckInstantReplay(self, data, 0)`).
pub fn replay_on_rider_wipeout(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let selfp = c.g[A0][0];
        c.g[S0][0] = selfp;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(OC331_REPLAY_WIPEOUT_BUSY[0]).lw(c);
        c.g[S1][0] = c.g[A2][0];
        // The first flag set, or the full path: one more cycle at the tail.
        let mut tail = c.g[V0][0] != 0;
        if !tail {
            let mut busy = false;
            for off in OC331_REPLAY_WIPEOUT_BUSY[1..].iter().copied() {
                c.cycles += 3;
                c.g[V0][0] = Addr(this).off(off).lw(c);
                if c.g[V0][0] != 0 {
                    busy = true;
                    break;
                }
            }
            if !busy {
                tail = true;
                c.cycles += 4;
                OC331_REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_0934)?;
                let found = (c.g[V0][0] as i64) >= 0;
                c.g[A1][0] = c.g[V0][0];
                if found {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = 0;
                    OC331_REPLAY_END_HIGHLIGHT.call(c, 0x0027_094c)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 0;
                    OC331_REPLAY_CHECK_INSTANT_REPLAY.call(c, 0x0027_095c)?;
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldLightMan_GetOverlapRegion (0x2f5a70)

/// The overlap region table in the world light manager (`self+0x10`).
pub const WORLD_LIGHT_OVERLAP_REGIONS: u32 = 0x10;

/// Per-region stride of the overlap region table.
pub const WORLD_LIGHT_REGION_STRIDE: u32 = 0x40;

/// `sBox_5400* WorldLightMan_GetOverlapRegion(sMan_5400* self, int priority)`
/// (0x2f5a70): the 8-entry table at `self+0x10`; each entry has a state word
/// (`-1` = free) and a priority. Returns the entry with the least priority
/// strictly above `priority`, or the first free entry (state `-1`), or null
/// when all 8 are used.
pub fn world_light_man_get_overlap_region(c: &mut Cpu) {
    c.cycles += 11;
    let selfp = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(selfp.wrapping_add(WORLD_LIGHT_OVERLAP_REGIONS)).lw(c);
    c.g[A3][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[T3][0] = 0;
    c.g[T2][0] = 0;
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    c.g[A3][0] |= 0xffff;
    c.g[T0][0] = WORLD_LIGHT_OVERLAP_REGIONS as u64;
    c.g[T1][0] = WORLD_LIGHT_OVERLAP_REGIONS as u64;
    let t = c.g[V0][0] != 0;
    c.g[A2][0] = sx(selfp.wrapping_add(WORLD_LIGHT_OVERLAP_REGIONS));
    if !t {
        // A free first entry.
        c.cycles += 2;
        c.g[T3][0] = c.g[A2][0];
    } else {
        c.cycles += 1;
        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
        'scan: loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let skip = c.g[V0][0] == 0;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
            if !skip {
                c.cycles += 2;
                c.g[A1][0] = c.g[V1][0];
                c.g[T3][0] = sx(selfp.wrapping_add(c.g[T0][0] as u32));
            }
            c.cycles += 5;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(WORLD_LIGHT_REGION_STRIDE));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(WORLD_LIGHT_REGION_STRIDE));
            c.g[V0][0] = (c.g[T2][0] < 8) as u64;
            let done = c.g[V0][0] == 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(WORLD_LIGHT_REGION_STRIDE));
            if done {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002f_5aac);
                }
                continue 'scan;
            }
            c.cycles += 1;
            c.g[T3][0] = sx(selfp.wrapping_add(c.g[T1][0] as u32));
            break;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T3][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// ModObj_v48 (0x356780)

/// The move node's modifier block pointer (`self+0x1c`).
pub const MODOBJ_V48_BLOCK: u32 = 0x1c;

/// The move node's instance node (bound box source, `self+0x18`).
pub const MODOBJ_V48_INSTANCE: u32 = 0x18;

/// The bound box written by `WorldObj_GetBoundBox` (8 floats: box, then the
/// radius at `+0x20`).
pub const MODOBJ_V48_BOX: u32 = 0x0;

/// `WorldObj_GetBoundBox` (0x350570).
pub const WORLD_OBJ_GET_BOUND_BOX: Callee = Callee {
    addr: 0x0035_0570,
    f: ee::s11::WorldObj_GetBoundBox,
};

/// `ModifierBlock_SetTransformBounds` (0x352bc0).
pub const MODIFIER_BLOCK_SET_TRANSFORM_BOUNDS: Callee = Callee {
    addr: 0x0035_2bc0,
    f: ee::s11::ModifierBlock_SetTransformBounds,
};

/// `ModifierBlock_SetTransformRadius` (0x352c38).
pub const MODIFIER_BLOCK_SET_TRANSFORM_RADIUS: Callee = Callee {
    addr: 0x0035_2c38,
    f: ee::s11::ModifierBlock_SetTransformRadius,
};

/// `void cMoveNode_setupOverlapSystem(void* self)` (0x356780): when the
/// move node has an active modifier block (`*(self+0x1c)` non-null and its
/// first word non-zero), take the node's bound box and hand it, and its
/// radius, to the modifier block.
pub fn mod_obj_v48(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MODOBJ_V48_BLOCK)).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 2;
        let no_block = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[SP][0];
        if !no_block {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            WORLD_OBJ_GET_BOUND_BOX.call(c, 0x0035_67b4)?;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MODOBJ_V48_BLOCK)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            MODIFIER_BLOCK_SET_TRANSFORM_BOUNDS.call(c, 0x0035_67c0)?;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(MODOBJ_V48_BLOCK)).lw(c);
            c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
            MODIFIER_BLOCK_SET_TRANSFORM_RADIUS.call(c, 0x0035_67cc)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupFreeMesh (0x34b7b8)

/// The flag manager pointer (`gp+0x2a90`).
pub const FLAG_MANAGER_GLOBAL: u32 = 0x2a90;

/// The group's mesh handle (`self+0x60`).
pub const FLAG_MANAGER_GROUP_MESH: u32 = 0x60;

/// The group's memory block (`self+0x74`).
pub const FLAG_MANAGER_GROUP_MEM: u32 = 0x74;

/// `EA_MemFree` (0x317e98).
pub const FLAG_MANAGER_MEM_FREE: u32 = 0x0031_7e98;

/// `void cFlagManager_GroupFreeMesh(void* self)` (0x34b7b8): release the
/// group's mesh through the flag manager's mesh-manager vtable (slot at
/// `+0x268`) and free its memory block; both pointers are cleared.
pub fn c_flag_manager_group_free_mesh(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(FLAG_MANAGER_GROUP_MESH)).lw(c);
        let no_mesh = c.g[A1][0] == 0;
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(FLAG_MANAGER_GLOBAL)).lw(c);
        if !no_mesh {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10d8)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x268)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x26c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0034_b7e8);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_b7e8)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(FLAG_MANAGER_GROUP_MESH), 0);
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(FLAG_MANAGER_GROUP_MEM)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0034_b800);
            call_ptr(c, FLAG_MANAGER_MEM_FREE, 0x0034_b800)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(FLAG_MANAGER_GROUP_MEM), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_DecFreeThunk (0x3c7ce0)

/// `void SND_DecFreeThunk(void* p)` (0x3c7ce0): the 7-cycle forwarding
/// wrapper around `SND_MemFree(p)` (through the function table).
pub fn snd_dec_free_thunk(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003b_5e98), 0x003c_7cf0);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_GetGruntBankId (0x29f620)

/// The rider's audio object (`self+0x6c0`).
pub const GRUNT_RIDER_AUDIO: u32 = 0x6c0;

/// `int SSXAudioSystem_GetGruntBankId(void* self, void* rider)` (0x29f620):
/// ask the rider's audio object (vtable slot at `+0x38`) whether it has an
/// audio bank; 0xf when it does, 0xe otherwise.
pub fn ssx_audio_system_get_grunt_bank_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(GRUNT_RIDER_AUDIO)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(GRUNT_RIDER_AUDIO));
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x38)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x3c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0029_f640);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0029_f640)?;
        c.g[A0][0] = 0xe;
        c.g[V1][0] = 0xf;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.g[A0][0];
        }
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_Queue (0x2aa320)

/// The stream's big-file handle (`self+0x74`).
pub const STREAM_BUFFER_BIG: u32 = 0x74;

/// The stream handle (`self+8`).
pub const STREAM_BUFFER_HANDLE: u32 = 0x8;

/// The stream's inline request buffer (`self+0x14`).
pub const STREAM_BUFFER_INLINE: u32 = 0x14;

/// `BigFile_FindEntry` (0x3e2ce0).
pub const BIG_FILE_FIND_ENTRY: u32 = 0x003e_2ce0;

/// `SNDSTRM_Queue` (0x3b78d8).
pub const OC331_SNDSTRM_QUEUE: u32 = 0x003b_78d8;

/// `int StreamBuffer_Queue(void* self, int handle, void* src, int off)`
/// (0x2aa320): queue a stream request. When the stream has a big-file
/// handle, look the entry up first (its size at `sp`) and add it to the
/// offset; then `SNDSTRM_Queue(stream, handle, buffer, off)`, where the
/// buffer is `src` for the big-file case and the caller's `src` otherwise.
pub fn stream_buffer_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(STREAM_BUFFER_BIG)).lw(c);
        let has_big = c.g[A0][0] != 0;
        c.g[S2][0] = c.g[A3][0];
        if has_big {
            c.cycles += 10;
            c.g[A1][0] = c.g[A2][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, BIG_FILE_FIND_ENTRY, 0x002a_a364)?;
            c.g[A3][0] = Addr(c.g[SP][0] as u32).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(STREAM_BUFFER_HANDLE)).lw(c);
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(STREAM_BUFFER_INLINE));
            c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        } else {
            c.cycles += 2;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(STREAM_BUFFER_HANDLE)).lw(c);
        }
        c.cycles += 8;
        call_ptr(c, OC331_SNDSTRM_QUEUE, 0x002a_a380)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{machine, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// `SPCH_BuildSentenceWithFallback`: no entry matches (the real
    /// `SPCH_BuildSentence` returns 0 through `SPCH_FindEvent` on empty
    /// event tables), the fallback callback null (return 0 at once), a real
    /// leaf returning 0 (one iteration), and a fake callback that unwinds on
    /// its second call (the retry loop, then the unwind).
    #[test]
    fn spch_build_sentence_with_fallback_matches() {
        check(
            ee::s14::f_003da418__generated,
            spch_build_sentence_with_fallback,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3da418, 0);
                // a0 = the entry index; the entry lands at DATA+0x4c.
                c.g[A0][0] = 40223;
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x003d_a418;
                c.bus.write32(
                    SPCH_FALLBACK_CB,
                    match seed % 4 {
                        0 | 3 => 0,
                        1 => 0x0014_06c0, // RiderIface_HasVisited: returns 0
                        _ => 0x0000_5000, // outside the function table: the fake
                    },
                );
                if seed % 4 == 2 {
                    // The fake returns a positive value: the loop retries,
                    // and the second call unwinds.
                    c.bus.write32(LOG_UNWIND_AT, 1);
                }
                fill(c, &mut r, 0x0080_004c, 0x20);
                c.bus.write32(0x0080_004c, 0);
            },
        );
    }

    /// `SND_UtkGetState`: random state words, copied into a destination in
    /// the test data area (the unaligned doubleword copy).
    #[test]
    fn snd_utk_get_state_matches() {
        check(ee::s14::f_003ce078__generated, snd_utk_get_state, 120, &|c, seed| {
            let mut r = Rng(0x3ce0_7800 ^ seed as u64);
            scramble(c, &mut r);
            let src = DATA + 0x2000;
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = sx(src);
            for off in SND_UTK_STATE_FIELDS {
                c.bus.write32(src + off, r.next_u32());
            }
            fill(c, &mut r, DATA + 0x400, 4);
        });
    }

    /// `Rand_FloatRange`: the real `Rand_NextGame` on random generator
    /// states, an ordinary and an edge-case range.
    #[test]
    fn rand_float_range_matches() {
        use crate::game::bx::bxrandom::RAND_GAME_STATE;
        check(ee::s10::f_00317830__generated, rand_float_range, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x317830, 0);
            c.g[RA][0] = 0x0012_3450;
            c.pc = 0x0031_7830;
            let (lo, hi) = match seed % 4 {
                0 => (r.f32(10.0), r.f32(10.0)),
                1 => (r.edge_f32(), r.edge_f32()),
                2 => (0x3f80_0000, 0x4000_0000), // 1.0 .. 2.0
                _ => (r.f32(1e6), r.f32(1e6)),
            };
            c.f[12] = lo;
            c.f[13] = hi;
            fill(c, &mut r, RAND_GAME_STATE, 8);
        });
    }

    /// `Replay_OnRiderWipeout`: the find-rider world of
    /// `Replay_FindRiderIndex`'s test, each busy flag set or all clear, the
    /// rider's highlight slot free (the real `Replay_EndHighlight` returns
    /// early), and the real `Replay_CheckInstantReplay` with instant replays
    /// armed or not, the wipeout time close to or past the last one.
    #[test]
    fn replay_on_rider_wipeout_matches() {
        use crate::game::replay::replaycache::tests::replay_find_rider_fixture;
        check(
            ee::s07::f_002708f0__generated,
            replay_on_rider_wipeout,
            300,
            &|c, seed| {
                replay_find_rider_fixture(c, seed);
                let key = c.g[A0][0];
                let mut r = Rng(0x2708_f000 ^ seed as u64);
                let this = DATA + 0x3_0000;
                let data = DATA + 0x3_9000;
                let busy = seed / 2 % 5;
                for (i, off) in OC331_REPLAY_WIPEOUT_BUSY.into_iter().enumerate() {
                    let set = busy as usize == i + 1;
                    c.bus
                        .write32(this + off, if set { 1 + r.next_u32() % 4 } else { 0 });
                }
                for k in 0..5u32 {
                    c.bus.write32(this + 0x494 + 0xb4 * k + 0xb0, 0);
                }
                c.bus.write32(this + 0x3d4, (seed % 3 != 0) as u32);
                let last = r.next_u32() % 5000;
                c.bus.write32(this + 0x47c, last);
                let now = last.wrapping_add(if seed % 4 < 2 { 500 } else { 1500 });
                c.bus.write32(data + 0xa4, now);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = key;
                c.g[A2][0] = sx(data);
                c.pc = 0x0027_08f0;
            },
        );
    }

    /// `WorldLightMan_GetOverlapRegion`: all 8 table entries random, with
    /// the state words set so the scan, the free-entry hit, the
    /// priority hit and the "all used" exit are all reached; events off,
    /// delivered or unwinding at the loop head.
    #[test]
    fn world_light_man_get_overlap_region_matches() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s09::f_002f5a70__generated,
            world_light_man_get_overlap_region,
            240,
            &|c, seed| {
                let mut r = Rng(0x2f5a_7000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                let prio = (r.next_u32() % 0x10) as u64;
                let free_at = (seed / 3) % 10; // 0..7: that entry; 8-9: none
                for i in 0..8u32 {
                    let e = this + WORLD_LIGHT_OVERLAP_REGIONS + WORLD_LIGHT_REGION_STRIDE * i;
                    let state = if i == free_at {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(e, state);
                    c.bus.write32(e + 4, r.next_u32() % 0x18);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = prio;
                schedule(c, &mut r, seed, 0x002f_5a70)
            },
        );
    }

    /// `ModObj_v48`: no modifier block, a block whose first word is null
    /// (no-op), and a live block whose vtable calls (fakes) run, against a
    /// node with a valid bound-box source.
    #[test]
    fn mod_obj_v48_matches() {
        check(ee::s11::f_00356780__generated, mod_obj_v48, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x356780, 0);
            let this = DATA;
            let block = DATA + 0x3000;
            let impl_ = DATA + 0x5000;
            let bounds = DATA + 0x4000;
            let vt = DATA + 0x8_0000;
            let d_box = (r.next_u32() as i16 as i32 & !3) as u16;
            let d_radius = (r.next_u32() as i16 as i32 & !3) as u16;
            c.bus.write16(vt + 0x68, d_box);
            c.bus.write32(vt + 0x6c, FAKE);
            c.bus.write16(vt + 0x78, d_radius);
            c.bus.write32(vt + 0x7c, FAKE + 0x10);
            match seed % 3 {
                0 => c.bus.write32(this + MODOBJ_V48_BLOCK, 0),
                1 => {
                    c.bus.write32(this + MODOBJ_V48_BLOCK, block);
                    c.bus.write32(block, 0);
                }
                _ => {
                    c.bus.write32(this + MODOBJ_V48_BLOCK, block);
                    c.bus.write32(block, impl_);
                    c.bus.write32(impl_, vt);
                }
            }
            c.bus.write32(this + MODOBJ_V48_INSTANCE, bounds);
            fill(c, &mut r, bounds, 0x80);
            c.g[A0][0] = sx(this);
            c.pc = 0x0035_6780;
        });
    }

    /// `cFlagManager_GroupFreeMesh`: the mesh handle present or absent
    /// (vtable slot through a fake), the memory block present (the real
    /// `EA_MemFree` with no pool/heap state) or absent.
    #[test]
    fn c_flag_manager_group_free_mesh_matches() {
        check(
            ee::s11::f_0034b7b8__generated,
            c_flag_manager_group_free_mesh,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x34b7b8, 0);
                let this = DATA;
                let mgr = DATA + 0x3000;
                let mesh = DATA + 0x2000;
                let mem = DATA + 0x6000;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + FLAG_MANAGER_GLOBAL, mgr);
                let vt = DATA + 0x8_0000;
                let delta = (r.next_u32() as i16 as i32 & !3) as u16;
                c.bus.write16(vt + 0x268, delta);
                c.bus.write32(vt + 0x26c, FAKE);
                c.bus.write32(mgr + 0x10d8, vt);
                // A "free block" header that makes `DLHeap_Free` bail out
                // before it writes anything.
                c.bus.write32(mem - 4, 0xffff_fffe);
                let has_mesh = seed % 2 == 0;
                c.bus.write32(
                    this + FLAG_MANAGER_GROUP_MESH,
                    if has_mesh { mesh } else { 0 },
                );
                let has_mem = seed % 4 >= 2;
                c.bus
                    .write32(this + FLAG_MANAGER_GROUP_MEM, if has_mem { mem } else { 0 });
                c.g[A0][0] = sx(this);
                c.pc = 0x0034_b7b8;
            },
        );
    }

    /// `SND_DecFreeThunk`: the real `SND_MemFree` with no sound heap state
    /// (its "unknown pointer" short path), any pointer.
    #[test]
    fn snd_dec_free_thunk_matches() {
        check(ee::s14::f_003c7ce0__generated, snd_dec_free_thunk, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x3c7ce0, 0);
            c.g[A0][0] = (r.next_u32() as u64) & 0x0fff_ffff;
            c.pc = 0x003c_7ce0;
        });
    }

    /// `SSXAudioSystem_GetGruntBankId`: the rider's audio object vtable
    /// (fake) reports a bank, or a real leaf returning 0 reports none.
    #[test]
    fn ssx_audio_system_get_grunt_bank_id_matches() {
        check(
            ee::s08::f_0029f620__generated,
            ssx_audio_system_get_grunt_bank_id,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29f620, 0);
                let this = DATA;
                let vt = DATA + 0x8_0000;
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(vt + 0x38, delta as u16);
                c.bus.write32(
                    vt + 0x3c,
                    if seed % 2 == 0 {
                        0x0014_06c0 // RiderIface_HasVisited: returns 0
                    } else {
                        FAKE
                    },
                );
                c.bus.write32(this + GRUNT_RIDER_AUDIO, vt);
                c.g[A1][0] = sx(this);
                c.pc = 0x0029_f620;
            },
        );
    }

    /// `StreamBuffer_Queue`: the real `SNDSTRM_Queue` takes its "no such
    /// stream" short path (no stream table); handles positive and negative.
    #[test]
    fn stream_buffer_queue_matches() {
        check(ee::s08::f_002aa320__generated, stream_buffer_queue, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2aa320, 0);
            let this = DATA;
            // (The big-file branch is not exercised: it runs the
            // `BigFile_FindEntry` dispatcher, whose zeroed-data path
            // could not be bounded.)
            c.bus.write32(this + STREAM_BUFFER_BIG, 0);
            let handle = if seed % 3 == 0 {
                0xffff_ffff
            } else {
                r.next_u32() % 0x100
            };
            c.bus.write32(this + STREAM_BUFFER_HANDLE, handle);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x1_8000);
            c.g[A2][0] = sx(DATA + 0x1_9000);
            c.g[A3][0] = (r.next_u32() % 0x4000) as u64;
            c.pc = 0x002a_a320;
        });
    }
}
