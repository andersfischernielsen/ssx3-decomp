//! Staging wave 211 (oc294): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_d988,
        world_state_write_objects,
        "WorldState_WriteObjects",
    ),
    (0x003d_3c90, path_remap_node, "PATH_RemapNode"),
    (
        0x0035_5468,
        mod_obj_create_modifier_block,
        "ModObj_CreateModifierBlock",
    ),
    (0x0037_0dc8, dynamic_emitter_setup, "DynamicEmitter_Setup"),
    (0x002d_9cb0, avalanche_save_state, "Avalanche_SaveState"),
    (
        0x0035_31d0,
        modifier_block_free_particles,
        "ModifierBlock_FreeParticles",
    ),
    (0x0035_4920, sync_process_dtor, "SyncProcess_dtor"),
    (
        0x002e_f248,
        world_painter_save_view_state,
        "WorldPainter_SaveViewState",
    ),
    (
        0x0029_f0a0,
        ssx_audio_system_is_speech_volume_on,
        "SSXAudioSystem_IsSpeechVolumeOn",
    ),
];

/// `void WorldState::WriteObjects(cPbStream* s, int list)` (0x26d988): counts
/// the processes in list `list` of the process manager (gp+0x2898), writes
/// the count as an 8-byte record (`s->Write(&rec, 8)`, vtable +8), then lets
/// each of those processes save itself into the stream (`proc->SaveState(s)`,
/// vtable +0x60 of the process's vtable at +0xc).
pub fn world_state_write_objects(c: &mut Cpu) {
    const PROCESS_MANAGER: u32 = 0x2898;
    const PROCESS_LIST_FIRST: u32 = 0x0035_4ea8;
    const PROCESS_LIST_NEXT: u32 = 0x0035_4ef8;
    const PROCESS_VTABLE: u32 = 0xc;
    const STREAM_WRITE: u32 = 0x8;
    const PROCESS_SAVE: u32 = 0x60;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(PROCESS_MANAGER));
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[S1][0];
        // rec = { count, 0 } on the stack
        c.sw(frame.at(c, 0), 0);
        c.sw(frame.at(c, 4), 0);
        call_ptr(c, PROCESS_LIST_FIRST, 0x0026_d9c0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw(frame.at(c, 0));
            loop {
                c.cycles += 9;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = c.g[S1][0];
                let count = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V0][0] = count;
                c.sw(frame.at(c, 0), count);
                call_ptr(c, PROCESS_LIST_NEXT, 0x0026_d9e8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(frame.at(c, 0));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_d9d0);
                }
            }
        }
        // s->Write(&rec, 8)
        c.cycles += 9;
        let vt = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x8;
        let delta = c.lh(vt.wrapping_add(STREAM_WRITE));
        let write = c.lw(vt.wrapping_add(STREAM_WRITE + 4));
        c.g[V1][0] = write;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0026_da10)?;
        let count = c.lw(frame.at(c, 0));
        c.g[V0][0] = count;
        c.g[A0][0] = c.g[S3][0];
        let mut tail = true;
        if count as i64 > 0 {
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, PROCESS_LIST_FIRST, 0x0026_da24)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                frame.restore(c, S0, 0x50);
                tail = false;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PROCESS_VTABLE));
                loop {
                    // proc->SaveState(s)
                    c.cycles += 12;
                    let vt = c.g[V0][0] as u32;
                    c.g[A1][0] = c.g[S2][0];
                    let delta = c.lh(vt.wrapping_add(PROCESS_SAVE));
                    let save = c.lw(vt.wrapping_add(PROCESS_SAVE + 4));
                    c.g[V1][0] = save;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(delta as u32));
                    call_ptr(c, save as u32, 0x0026_da4c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, PROCESS_LIST_NEXT, 0x0026_da5c)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PROCESS_VTABLE));
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_da38);
                    }
                }
            }
        }
        if tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_RemapNode(int id, int node)` (0x3d3c90): remaps a path node
/// number through the remap ranges of path `id` in the path table
/// (`*0x44f420`): the byte at +4 of the path's word (`words[index[id]]`)
/// picks range k (`ranges[k-1]..ranges[k]` of `words`); each word of the
/// range is `{from << 16 | to}`, and every word whose `from` equals the
/// current node replaces it by `to`. Negative ids and k == 0 return `node`.
pub fn path_remap_node(c: &mut Cpu) {
    const PATH_TABLE: u32 = 0x0045_0000 - 0xbe0;
    const WORDS: u32 = 0x34;
    const INDEX: u32 = 0x38;
    const RANGES: u32 = 0x48;
    c.cycles += 2;
    c.g[A2][0] = c.g[A1][0];
    if c.g[A0][0] as i64 >= 0 {
        c.cycles += 12;
        let table = c.lw(PATH_TABLE);
        c.g[A1][0] = table;
        let index = c.lw((table as u32).wrapping_add(INDEX));
        let words = c.lw((table as u32).wrapping_add(WORDS));
        c.g[A3][0] = words;
        let entry = sx(((c.g[A0][0] as u32) << 1).wrapping_add(index as u32));
        c.g[A0][0] = entry;
        let word = c.lh(entry as u32);
        let word_addr = ((word as u32) << 2).wrapping_add(words as u32);
        let k = c.lbu(word_addr.wrapping_add(4));
        let k4 = sx((k as u32) << 2);
        c.g[V0][0] = k4;
        if k == 0 {
            c.g[V1][0] = index;
        } else {
            c.cycles += 8;
            let ranges = c.lw((table as u32).wrapping_add(RANGES));
            let range = (k4 as u32).wrapping_add(ranges as u32);
            c.g[V0][0] = sx(range);
            let start = c.lw(range.wrapping_sub(4));
            let end = c.lw(range);
            c.g[A0][0] = start;
            c.g[A1][0] = sx((start as u32) << 2);
            let n = sx((end as u32).wrapping_sub(start as u32));
            c.g[V0][0] = n;
            let first = sx((words as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = first;
            if n as i64 > 0 {
                c.cycles += 2;
                c.g[A1][0] = first;
                c.g[A0][0] = n;
                loop {
                    c.cycles += 8;
                    let w = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                    let from = sx(((w as u32) as i32 >> 16) as u32);
                    let to = w & 0xffff;
                    c.g[V1][0] = to;
                    c.g[V0][0] = c.g[A2][0] ^ from;
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = to;
                    }
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_3cf8);
                    }
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    ret(c);
}

/// `void ModObj::CreateModifierBlock()` (0x355468): allocates a 0x28-byte
/// `ModifierBlock` (`EA_MemAllocNamed(0x28, "ModifierBlock"-ish name at
/// 0x48e9d8, 0x20000000, 0)`), constructs it and stores it at `this+0x1c`.
pub fn mod_obj_create_modifier_block(c: &mut Cpu) {
    const ALLOC: u32 = 0x0031_7d70;
    const MODIFIER_BLOCK_CTOR: Callee = Callee {
        addr: 0x0035_2aa8,
        f: ee::s11::ModifierBlock_ctor,
    };
    const MODIFIER_BLOCK: u32 = 0x1c;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x1628));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x28;
        call_ptr(c, ALLOC, 0x0035_5490)?;
        c.g[A0][0] = c.g[V0][0];
        MODIFIER_BLOCK_CTOR.call(c, 0x0035_5498)?;
        let block = c.g[V0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(MODIFIER_BLOCK), block);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `DynamicEmitter` fields (`self`, a0) and the definition it is set up
/// from (`def`, a1).
mod dyn_emitter {
    pub const START: u32 = 0x0;
    pub const RATE_A: u32 = 0x4;
    pub const RATE_B: u32 = 0x8;
    pub const BURST: u32 = 0xc;
    pub const BURST_DELAY: u32 = 0x10;
    pub const BURST_SPACING: u32 = 0x14;
    pub const SYSTEM: u32 = 0x20;
    pub const COUNT: u32 = 0x178;
    pub const ACTIVE: u32 = 0x170;
    pub const PARAMS: u32 = 0x180;
    pub const BOUNDS: u32 = 0x1b0;
    pub const GS_STATE: u32 = 0x1e4;
    pub const VTABLE: u32 = 0x1f8;
    pub const DEF_PER_EMIT: u32 = 0x0;
    pub const DEF_KIND: u32 = 0x4;
    pub const DEF_START: u32 = 0x8;
    pub const DEF_LIFE: u32 = 0xc;
    pub const DEF_FADE_IN: u32 = 0x14;
    pub const DEF_FADE: u32 = 0x1c;
    pub const DEF_SIZE: u32 = 0x20;
    pub const DEF_DIR: u32 = 0x78;
    pub const DEF_RATE_A: u32 = 0xc4;
    pub const DEF_RATE_B: u32 = 0xc8;
    pub const DEF_BURST: u32 = 0xd0;
    pub const DEF_BURST_SPACING: u32 = 0xd4;
    /// The zero bounds quadword (0x4ff120), the descriptor template
    /// (0x4ff1a0) and the GS state template (0x501420).
    pub const ZERO_BOUNDS: u32 = 0x0050_0000 - 0xee0;
    pub const TEMPLATE: u32 = 0x0050_0000 - 0xe60;
    pub const GS_TEMPLATE: u32 = 0x0050_1420;
}

/// The span for a negative `def->start`: `fade_in + fade / 2` (leaves
/// f0 = fade/2 and f1 = 0.5 as the original does).
fn dynamic_emitter_span(c: &mut Cpu, def: u32) -> FpuF32 {
    use dyn_emitter::*;
    c.cycles += 6;
    let fade = FpuF32(c.lw(def.wrapping_add(DEF_FADE)) as u32);
    let half = FpuF32(0x3f00_0000);
    c.g[AT][0] = 0x3f00_0000;
    abi::set_fpr(c, 1, half);
    let fade_in = FpuF32(c.lw(def.wrapping_add(DEF_FADE_IN)) as u32);
    let f0 = fade * half;
    abi::set_fpr(c, 0, f0);
    fade_in + f0
}

/// `ceil(frames)` as the original computes it (`cvt.w.s`, back, `+1` when
/// the truncation is below): f1 is left holding the rounded float.
fn dynamic_emitter_ceil(c: &mut Cpu, frames: FpuF32) -> FpuF32 {
    let mut up = FpuF32::from_i32(frames.to_i32());
    let below = abi::c_lt_s(c, up, frames);
    if below {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        abi::set_fpr(c, 0, FpuF32::ONE);
        up = up + FpuF32::ONE;
    }
    abi::set_fpr(c, 1, up);
    up
}

/// `void DynamicEmitter::Setup(EmitterDef* def, float time)` (0x370dc8):
/// copies the burst count/spacing and rates from the definition (a burst of
/// 2+ particles gets a random delay `n * rand[0,1)`), sets the start time and
/// the particle count (`ceil(span * 60)`, the span being `def->start` or
/// `fade_in + fade/2` when that is negative), clears the bounds, sets up the
/// particle system (`Emitter_InitDescriptor`, `Emitter_BuildDescriptor`),
/// resets the emitter through its vtable (+8, +0x28, +0x10) when the count
/// changed, marks it active and installs its GS state from the template.
pub fn dynamic_emitter_setup(c: &mut Cpu) {
    use dyn_emitter::*;
    const RAND: u32 = 0x0031_77f0;
    const INIT_DESCRIPTOR: Callee = Callee {
        addr: 0x0036_cbf8,
        f: ee::s12::Emitter_InitDescriptor,
    };
    const BUILD_DESCRIPTOR: Callee = Callee {
        addr: 0x0037_0058,
        f: ee::s12::Emitter_BuildDescriptor,
    };
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 21, 0x68);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.f[21] = c.f[12];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0x60);
        let this = r32(c, S2);
        let def = r32(c, S1);

        // Burst: count and spacing; a random delay for 2+ particles.
        let burst = c.lw(def.wrapping_add(DEF_BURST));
        c.g[V1][0] = burst;
        c.sw(this.wrapping_add(BURST), burst);
        c.g[V0][0] = ((burst as i64) < 2) as u64;
        let spacing = c.lw(def.wrapping_add(DEF_BURST_SPACING)) as u32;
        c.f[0] = spacing;
        c.sw(this.wrapping_add(BURST_SPACING), spacing as u64);
        if (burst as i64) < 2 {
            c.cycles += 1;
            c.sw(this.wrapping_add(BURST_DELAY), 0);
        } else {
            c.cycles += 16;
            c.f[20] = FpuF32::from_i32(burst as i32).0;
            call_ptr(c, RAND, 0x0037_0e1c)?;
            // A float in [1, 2) from the random mantissa, minus 1.
            c.g[V1][0] = 0x7f_ffff;
            c.g[A1][0] = 0x3f80_0000;
            c.g[AT][0] = 0x3f80_0000;
            abi::set_fpr(c, 0, FpuF32::ONE);
            let bits = (c.g[V0][0] & 0x7f_ffff) | 0x3f80_0000;
            c.g[V0][0] = bits;
            let r = FpuF32(bits as u32);
            abi::set_fpr(c, 1, r);
            let unit = r - FpuF32::ONE;
            abi::set_fpr(c, 0, unit);
            let delay = FpuF32(c.f[20]) * unit;
            c.f[20] = delay.0;
            c.sw(r32(c, S2).wrapping_add(BURST_DELAY), sx(delay.0));
        }
        let this = r32(c, S2);
        let def = r32(c, S1);

        // Rates and the start time.
        c.cycles += 10;
        let rate_a = c.lw(def.wrapping_add(DEF_RATE_A));
        c.g[V0][0] = rate_a;
        c.f[0] = 0;
        c.sw(this.wrapping_add(RATE_A), rate_a);
        let rate_b = c.lw(def.wrapping_add(DEF_RATE_B));
        c.g[V1][0] = rate_b;
        c.sw(this.wrapping_add(RATE_B), rate_b);
        let start = FpuF32(c.lw(def.wrapping_add(DEF_START)) as u32);
        c.f[3] = start.0;
        let negative = abi::c_lt_s(c, start, FpuF32::ZERO);
        let mut time = start;
        if !negative {
            c.cycles += 7;
            let fade_in = FpuF32(c.lw(def.wrapping_add(DEF_FADE_IN)) as u32);
            let fade = FpuF32(c.lw(def.wrapping_add(DEF_FADE)) as u32);
            c.g[AT][0] = 0x3f00_0000;
            let half = FpuF32(0x3f00_0000);
            abi::set_fpr(c, 2, half);
            let half_fade = fade * half;
            abi::set_fpr(c, 1, half_fade);
            time = start + fade_in + half_fade;
        }
        c.f[0] = time.0;

        // The particle count: ceil(span * 60).
        c.cycles += 7;
        c.sw(this.wrapping_add(START), sx(time.0));
        c.f[0] = 0;
        let mut span = FpuF32(c.lw(def.wrapping_add(DEF_START)) as u32);
        let positive = FpuF32::ZERO.le(span);
        abi::set_fcc(c, positive);
        c.g[S3][0] = 0;
        if !positive {
            span = dynamic_emitter_span(c, def);
        }
        c.cycles += 8;
        c.g[AT][0] = 0x4270_0000;
        let sixty = FpuF32(0x4270_0000);
        abi::set_fpr(c, 0, sixty);
        let frames = span * sixty;
        abi::set_fpr(c, 2, frames);
        let count = dynamic_emitter_ceil(c, frames);
        c.cycles += 5;
        let n = count.to_i32() as u32;
        c.f[0] = n;
        c.g[V1][0] = sx(n);
        let old = c.lw(this.wrapping_add(COUNT));
        c.g[V0][0] = old;
        c.g[A0][0] = 0x50_0000;
        if old != sx(n) {
            c.cycles += 5;
            let count = dynamic_emitter_ceil(c, frames);
            c.cycles += 4;
            let n = count.to_i32() as u32;
            c.f[0] = n;
            c.sw(this.wrapping_add(COUNT), n as u64);
            c.g[S3][0] = 1;
            c.g[A0][0] = 0x50_0000;
        }

        // Clear the bounds; the life time for the descriptor.
        c.cycles += 13;
        c.f[0] = 0;
        c.g[A0][0] = sx(ZERO_BOUNDS);
        c.lq(V0, ZERO_BOUNDS);
        c.sq(V0, this.wrapping_add(BOUNDS));
        c.lq(V1, ZERO_BOUNDS);
        c.sq(V1, this.wrapping_add(BOUNDS + 0x10));
        c.lq(V0, ZERO_BOUNDS);
        c.sq(V0, this.wrapping_add(BOUNDS + 0x20));
        let mut life = FpuF32(c.lw(def.wrapping_add(DEF_START)) as u32);
        let positive = FpuF32::ZERO.le(life);
        abi::set_fcc(c, positive);
        c.g[A1][0] = c.lw(this.wrapping_add(COUNT));
        if !positive {
            c.cycles += 6;
            let fade = FpuF32(c.lw(def.wrapping_add(DEF_FADE)) as u32);
            c.g[AT][0] = 0x3f00_0000;
            let half = FpuF32(0x3f00_0000);
            abi::set_fpr(c, 1, half);
            let fade_in = FpuF32(c.lw(def.wrapping_add(DEF_FADE_IN)) as u32);
            abi::set_fpr(c, 2, fade_in);
            let half_fade = fade * half;
            abi::set_fpr(c, 0, half_fade);
            life = fade_in + half_fade;
        }
        c.f[12] = life.0;

        // Emitter_InitDescriptor(&system, def->per_emit * count, def->kind,
        //                        &dir, life, def->size, def->life)
        c.cycles += 26;
        c.g[S0][0] = sx(this.wrapping_add(SYSTEM));
        let per_emit = c.lw(def.wrapping_add(DEF_PER_EMIT));
        c.g[V0][0] = per_emit;
        c.g[A0][0] = c.g[S0][0];
        let dir_x = c.lw(def.wrapping_add(DEF_DIR)) as u32;
        c.f[2] = dir_x;
        c.g[A3][0] = c.g[SP][0];
        let count = c.g[A1][0] as u32;
        let total = abi::mult(c, per_emit as u32, count);
        let dir_y = c.lw(def.wrapping_add(DEF_DIR + 4)) as u32;
        c.f[0] = dir_y;
        let dir_z = c.lw(def.wrapping_add(DEF_DIR + 8)) as u32;
        c.f[1] = dir_z;
        c.g[A2][0] = c.lw(def.wrapping_add(DEF_KIND));
        c.f[13] = c.lw(def.wrapping_add(DEF_SIZE)) as u32;
        c.f[14] = c.lw(def.wrapping_add(DEF_LIFE)) as u32;
        c.g[A1][0] = sx(total);
        c.sw(frame.at(c, 0), dir_x as u64);
        c.sw(frame.at(c, 4), dir_y as u64);
        c.sw(frame.at(c, 8), dir_z as u64);
        c.sw(frame.at(c, 0xc), 0);
        INIT_DESCRIPTOR.call(c, 0x0037_0fc8)?;

        // Emitter_BuildDescriptor(&system, template, def, &params, time)
        c.g[A1][0] = sx(TEMPLATE);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.f[12] = c.f[21];
        c.g[A3][0] = sx(r32(c, S2).wrapping_add(PARAMS));
        BUILD_DESCRIPTOR.call(c, 0x0037_0fe4)?;
        c.g[A1][0] = 0;
        if c.g[S3][0] != 0 {
            // The count changed: reset the emitter (vtable +8, +0x28, +0x10).
            c.cycles += 15;
            for (slot, back) in [(0x8, 0x0037_1000), (0x28, 0x0037_1014), (0x10, 0x0037_1028)] {
                let this = c.g[S2][0];
                let vt = c.lw((this as u32).wrapping_add(VTABLE));
                c.g[V0][0] = vt;
                let delta = c.lh((vt as u32).wrapping_add(slot));
                let method = c.lw((vt as u32).wrapping_add(slot + 4));
                c.g[V1][0] = method;
                c.g[A0][0] = sx((this as u32).wrapping_add(delta as u32));
                call_ptr(c, method as u32, back)?;
            }
        }

        // Active, and the GS state from the template (packed register bits
        // adjusted in the second and third words).
        c.cycles += 41;
        let this = r32(c, S2);
        c.g[V0][0] = 1;
        c.g[V1][0] = 0x50_0000;
        c.sw(this.wrapping_add(ACTIVE), 1);
        c.g[A0][0] = 0xffff_ffff_ffbf_0000;
        c.g[A3][0] = sx(GS_TEMPLATE);
        let lo = c.lw(GS_TEMPLATE) as u32 as u64 | (c.lw(GS_TEMPLATE + 4) as u32 as u64) << 32;
        let hi =
            c.lw(GS_TEMPLATE + 8) as u32 as u64 | (c.lw(GS_TEMPLATE + 0xc) as u32 as u64) << 32;
        c.g[V0][0] = lo;
        c.g[A1][0] = hi;
        let last = c.lw(GS_TEMPLATE + 0x10);
        c.g[A2][0] = last;
        c.sw(this.wrapping_add(GS_STATE), lo & 0xffff_ffff);
        c.sw(this.wrapping_add(GS_STATE + 4), lo >> 32);
        c.sw(this.wrapping_add(GS_STATE + 8), hi & 0xffff_ffff);
        c.sw(this.wrapping_add(GS_STATE + 0xc), hi >> 32);
        c.sw(this.wrapping_add(GS_STATE + 0x10), last);
        c.g[A0][0] = 0xffff_ffff_ffbf_ffff;
        c.g[T0][0] = 0x40_0000;
        c.g[A1][0] = 0xffff_ffff_ffcf_ffff;
        let word1 = c.lw(this.wrapping_add(GS_STATE + 4));
        let word2 = c.lw(this.wrapping_add(GS_STATE + 8));
        c.g[A3][0] = 0xffff_ffff_ffff_fffc;
        c.g[V1][0] = 0xffff_ffff_e000_03ff;
        let word2 = word2 & 0xffff_ffff_e000_03ff;
        c.g[A2][0] = word2;
        c.sw(this.wrapping_add(GS_STATE + 8), word2);
        let word1 = ((((word1 & 0xffff_ffff_ffbf_ffff) | 0x40_0000) & 0xffff_ffff_ffcf_ffff)
            & 0xffff_ffff_ffff_fffc)
            | 2;
        c.g[V0][0] = word1;
        frame.restore(c, S0, 0x50);
        c.sw(this.wrapping_add(GS_STATE + 4), word1);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `void Avalanche_SaveState(cPbStream* s)` (0x2d9cb0): writes the state of
/// the 16 avalanche slots (table at 0x5389c0, 0x1c bytes each) as one
/// 0x80-byte record: the definition index of each slot (`*slot->def`, -1
/// when idle), then each slot's playback time (+0x18; 0.0 when idle)
/// (`s->Write(&rec, 0x80)`, vtable +8).
pub fn avalanche_save_state(c: &mut Cpu) {
    const SLOTS: u32 = 0x0054_0000 - 0x76c8;
    const SLOT_SIZE: u64 = 0x1c;
    const SLOT_TIME: u32 = 0x18;
    const SLOT_COUNT: i64 = 16;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = sx(SLOTS);
        frame.save_ra(c, 0x80);
        c.g[T1][0] = sx(SLOTS);
        c.g[T2][0] = c.g[A0][0];
        c.g[A3][0] = 0;
        c.g[T3][0] = sx(SLOTS + SLOT_TIME);
        c.g[T4][0] = sx(SLOTS);
        c.g[T0][0] = sx(frame.at(c, 0x40));
        c.g[A2][0] = c.g[SP][0];
        loop {
            // The definition index (-1: idle slot).
            c.cycles += 3;
            let def = c.lw(c.g[T1][0] as u32);
            let index = if def == 0 {
                u64::MAX
            } else {
                c.cycles += 1;
                c.lw(def as u32)
            };
            c.g[V0][0] = index;
            c.cycles += 7;
            c.sw(c.g[A2][0] as u32, index);
            c.g[A1][0] = SLOT_SIZE;
            let i = c.g[A3][0] as u32;
            let off = abi::mult(c, i, SLOT_SIZE as u32);
            c.g[V0][0] = sx(off);
            let slot = sx(off.wrapping_add(c.g[T4][0] as u32));
            c.g[V1][0] = slot;
            let def = c.lw(slot as u32);
            c.g[V0][0] = def;
            c.g[A0][0] = c.g[T0][0];
            // The playback time (0.0: idle slot).
            if def != 0 {
                c.cycles += 4;
                let i = c.g[A3][0] as u32;
                let off = abi::mult(c, i, SLOT_SIZE as u32);
                c.g[V1][0] = sx(off);
                let at = sx(off.wrapping_add(c.g[T3][0] as u32));
                c.g[V0][0] = at;
                c.f[0] = c.lw(at as u32) as u32;
            } else {
                c.cycles += 1;
                c.f[0] = 0;
            }
            c.cycles += 7;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.sw(c.g[A0][0] as u32, c.f[0] as u64);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(4));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
            let more = (c.g[A3][0] as i64) < SLOT_COUNT;
            c.g[V0][0] = more as u64;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(SLOT_SIZE as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_9ce0);
            }
        }
        // s->Write(&rec, 0x80)
        c.cycles += 10;
        let stream = c.g[T2][0] as u32;
        let vt = c.lw(stream);
        c.g[V0][0] = vt;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x80;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V1][0] = write;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x002d_9d58)?;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void ModifierBlock::FreeParticles()` (0x3531d0): deletes every link of
/// the block's particle list (+0x10; each link's `next` at +0, its vtable at
/// +8) through its deleting destructor (`link->~Link(3)`, vtable +8).
pub fn modifier_block_free_particles(c: &mut Cpu) {
    const PARTICLES: u32 = 0x10;
    const LINK_NEXT: u32 = 0x0;
    const LINK_VTABLE: u32 = 0x8;
    const DELETE: u32 = 0x8;
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let head = c.lw((c.g[A0][0] as u32).wrapping_add(PARTICLES));
        c.g[A0][0] = head;
        frame.restore(c, S0, 0x10);
        if head != 0 {
            c.cycles += 2;
            c.g[A2][0] = head;
            loop {
                c.cycles += 2;
                let link = c.g[A2][0] as u32;
                c.g[S0][0] = c.lw(link.wrapping_add(LINK_NEXT));
                if link != 0 {
                    c.cycles += 6;
                    let vt = c.lw(link.wrapping_add(LINK_VTABLE));
                    c.g[V0][0] = vt;
                    c.g[A1][0] = 0x3;
                    let delta = c.lh((vt as u32).wrapping_add(DELETE));
                    let dtor = c.lw((vt as u32).wrapping_add(DELETE + 4));
                    c.g[V1][0] = dtor;
                    c.g[A0][0] = sx(link.wrapping_add(delta as u32));
                    call_ptr(c, dtor as u32, 0x0035_3210)?;
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0];
                frame.restore(c, S0, 0x10);
                if c.g[A2][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_31f0);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SyncProcess::~SyncProcess()` (0x354920): installs the `SyncProcess`
/// vtable (0x491e00 at +0xc) and runs the `TypedProcess` destructor (which
/// returns `this`, or frees it when the delete flag in a1 is set).
pub fn sync_process_dtor(c: &mut Cpu) {
    const SYNC_PROCESS_VTABLE: u32 = 0x0049_1e00;
    const PROCESS_VTABLE: u32 = 0xc;
    const TYPED_PROCESS_DTOR: Callee = Callee {
        addr: 0x0035_4808,
        f: ee::s11::TypedProcess_dtor,
    };
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(SYNC_PROCESS_VTABLE);
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(PROCESS_VTABLE),
            sx(SYNC_PROCESS_VTABLE),
        );
        TYPED_PROCESS_DTOR.call(c, 0x0035_493c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `RenderStateMan` entries (0x4fa370, 6 of 0xf0 bytes); each holds a
/// pointer (+0x20) to the slot of its render-state object.
const RSM_ENTRIES: u32 = 0x0050_0000 - 0x5c90;
const RSM_ENTRY_SIZE: u32 = 0xf0;
const RSM_ENTRY_OBJECT: u32 = 0x20;

/// One `(*entry->object)->SaveViewState(view)` call: the object's vtable is
/// at +4, the method at slot +0x200. a0 holds the entry's slot pointer.
fn world_painter_save_view_call(c: &mut Cpu, slot: u32, back: u32) -> crate::call::Flow {
    let object = c.lw(slot);
    c.g[A2][0] = object;
    let vt = c.lw((object as u32).wrapping_add(4));
    c.g[V0][0] = vt;
    let delta = c.lh((vt as u32).wrapping_add(0x200));
    let method = c.lw((vt as u32).wrapping_add(0x204));
    c.g[V1][0] = method;
    c.g[A0][0] = sx((object as u32).wrapping_add(delta as u32));
    call_ptr(c, method as u32, back)
}

/// `void WorldPainter_SaveViewState(void* view)` (0x2ef248): lets the
/// render-state object of each of the six `RenderStateMan` entries save the
/// view state (vtable +0x200), twice per entry as the original does.
pub fn world_painter_save_view_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(RSM_ENTRIES);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(RSM_ENTRIES + 6 * RSM_ENTRY_SIZE);
        c.g[V1][0] = c.lw(RSM_ENTRIES + RSM_ENTRY_OBJECT);
        loop {
            c.cycles += 19;
            c.g[A1][0] = c.g[S1][0];
            let slot = c.g[V1][0] as u32;
            world_painter_save_view_call(c, slot, 0x002e_f28c)?;
            let slot = c.lw((c.g[S0][0] as u32).wrapping_add(RSM_ENTRY_OBJECT));
            c.g[A0][0] = slot;
            c.g[A1][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(RSM_ENTRY_SIZE));
            world_painter_save_view_call(c, slot as u32, 0x002e_f2b0)?;
            let more = (c.g[S0][0] as i64) < (c.g[S2][0] as i64);
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RSM_ENTRY_OBJECT));
            if event_due(c) {
                deliver_event_here(c, 0x002e_f270);
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

/// `bool SSXAudioSystem::IsSpeechVolumeOn()` (0x29f0a0): whether the speech
/// channel's volume (`GetChannelVolume(4)`) is not zero.
pub fn ssx_audio_system_is_speech_volume_on(c: &mut Cpu) {
    const GET_CHANNEL_VOLUME: u32 = 0x0028_7920;
    const SPEECH_CHANNEL: u64 = 4;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = SPEECH_CHANNEL;
        call_ptr(c, GET_CHANNEL_VOLUME, 0x0029_f0b0)?;
        c.f[1] = 0;
        let silent = FpuF32(c.f[0]).eq_s(FpuF32::ZERO);
        abi::set_fcc(c, silent);
        c.g[V0][0] = 0;
        if !silent {
            c.cycles += 1;
            c.g[V0][0] = 1;
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
    use crate::difftest::check;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{fill, scramble, Rng, DATA, GAME_GP};

    /// `WorldState_WriteObjects`: the process manager at gp+0x2898 points at
    /// list headers (0x44 each); list `a2` (0..8) holds 0..4 processes
    /// (empty: first == the sentinel at header+0x14), each with a vtable at
    /// +0xc whose `SaveState` slot (+0x60) is a fake. ProcessList_First/Next
    /// run for real; the stream's `Write` (vtable +8) is a fake (logs the
    /// registers, sometimes unwinds).
    #[test]
    fn world_state_write_objects_matches() {
        const STREAM: u32 = DATA + 0x100;
        const STREAM_VT: u32 = DATA + 0x200;
        const PROCESS_VT: u32 = DATA + 0x300;
        const PROCS: u32 = DATA + 0x1000;
        const NODES: u32 = DATA + 0x2000;
        covered(
            ee::s07::f_0026d988__generated,
            world_state_write_objects,
            240,
            &|c, seed| {
                let nodes = seed % 5;
                let list = (seed / 5) % 9;
                let mut r = machine(c, seed, 0x26d988, 1 + nodes);
                c.pc = 0x0026_d988;
                c.bus.write32(GAME_GP + 0x2898 + 4, PROCS);
                let head = PROCS + list * 0x44;
                let sentinel = head + 0x14;
                let node = |i: u32| {
                    if i < nodes {
                        NODES + 0x40 * i
                    } else {
                        sentinel
                    }
                };
                c.bus.write32(head + 8, node(0));
                for i in 0..nodes {
                    c.bus.write32(node(i) + 4, node(i + 1));
                    c.bus.write32(node(i) + 0xc, PROCESS_VT);
                }
                c.bus
                    .write16(PROCESS_VT + 0x60, (r.next_u32() % 0x40) as u16 & !3);
                c.bus.write32(PROCESS_VT + 0x64, FAKE + 0x10);
                c.bus.write32(STREAM, STREAM_VT);
                c.bus
                    .write16(STREAM_VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(STREAM_VT + 0xc, FAKE);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(STREAM);
                c.g[A2][0] = list as u64;
            },
        );
    }

    /// `PATH_RemapNode`: the path table at `*0x44f420` with index (+0x38),
    /// words (+0x34) and ranges (+0x48). Ids are negative on some seeds; the
    /// range byte k is 0..3 (0: no remap); ranges are sometimes empty or
    /// reversed (n <= 0); range words remap small node numbers so that
    /// matches (and chains of them) happen.
    #[test]
    fn path_remap_node_matches() {
        const TABLE: u32 = DATA;
        const WORDS: u32 = DATA + 0x1000;
        const INDEX: u32 = DATA + 0x3000;
        const RANGES: u32 = DATA + 0x3800;
        check(ee::s14::f_003d3c90__generated, path_remap_node, 300, &|c, seed| {
            let mut r = Rng(0x3d3c90 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.pc = 0x003d_3c90;
            c.bus.write32(0x0045_0000 - 0xbe0, TABLE);
            c.bus.write32(TABLE + 0x34, WORDS);
            c.bus.write32(TABLE + 0x38, INDEX);
            c.bus.write32(TABLE + 0x48, RANGES);
            let id = r.next_u32() % 64;
            let word = r.next_u32() % 256;
            c.bus.write16(INDEX + 2 * id, word as u16);
            let k = seed % 4;
            c.bus.write8(WORDS + 4 * word + 4, k as u8);
            // ranges[0..4]: increasing, sometimes empty or reversed
            let mut at = r.next_u32() % 64;
            for i in 0..4 {
                c.bus.write32(RANGES + 4 * i, 0x200 + at);
                at = match r.next_u32() % 6 {
                    0 => at,
                    1 => at.saturating_sub(3),
                    _ => at + 1 + r.next_u32() % 12,
                };
            }
            for i in 0..0x300 {
                let from = r.next_u32() % 6;
                let to = if r.next_u32() % 4 == 0 {
                    r.next_u32() & 0xffff
                } else {
                    r.next_u32() % 6
                };
                c.bus.write32(WORDS + 4 * (0x200 + i), from << 16 | to);
            }
            c.g[A0][0] = if seed % 5 == 0 {
                sx(0x8000_0000 | r.next_u32())
            } else {
                id as u64
            };
            c.g[A1][0] = (r.next_u32() % 6) as u64;
        });
    }

    /// `ModObj_CreateModifierBlock`: the real allocator over the test pool
    /// and heap (`alloc_machine`) and the real `ModifierBlock_ctor`; the
    /// object is random bytes.
    #[test]
    fn mod_obj_create_modifier_block_matches() {
        use crate::game::dirtysock::tags2::tests::alloc_machine;
        check(
            ee::s11::f_00355468__generated,
            mod_obj_create_modifier_block,
            60,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x355468);
                c.pc = 0x0035_5468;
                fill(c, &mut r, DATA + 0x100, 0x10);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    /// `DynamicEmitter_Setup` with the real `Rand_NextMain` (random state at
    /// 0x4ff018), `Emitter_InitDescriptor` and `Emitter_BuildDescriptor`
    /// (random tables at gp-0x2868, template matrix at 0x4ff1a0); the reset
    /// methods (emitter vtable at +0x1f8, slots +8/+0x28/+0x10) are fakes.
    /// Bursts of 0..3 (random delay from 2), start time negative on some
    /// seeds (the fade span), and on some seeds the old particle count equals
    /// the new one (start 1.0: 60 frames), so no reset.
    #[test]
    fn dynamic_emitter_setup_matches() {
        use crate::difftest::fakes::vtable;
        const SELF: u32 = DATA + 0x20_0000;
        const DEF: u32 = DATA + 0x20_1000;
        covered(
            ee::s12::f_00370dc8__generated,
            dynamic_emitter_setup,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x370dc8, 3);
                c.pc = 0x0037_0dc8;
                for i in 0..0x80u32 {
                    let v = if i % 7 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(50.0)
                    };
                    c.bus.write32(SELF + 4 * i, v);
                }
                for i in 0..0x40u32 {
                    c.bus.write32(DEF + 4 * i, r.f32(50.0));
                }
                let vt = vtable(c, &mut r, &[0x8, 0x28, 0x10]);
                c.bus.write32(SELF + 0x1f8, vt);
                // Definition: per-emit count, kind, life, burst, rates.
                c.bus.write32(DEF, 1 + r.next_u32() % 4);
                c.bus.write32(DEF + 4, r.next_u32() % 4);
                if seed % 5 != 0 {
                    c.bus.write32(DEF + 0xc, 1.5f32.to_bits());
                }
                c.bus.write32(DEF + 0xd0, seed % 4);
                c.bus.write32(DEF + 0xc4, r.next_u32());
                c.bus.write32(DEF + 0xc8, r.next_u32());
                let start = match seed % 4 {
                    0 => 1.0f32.to_bits(),
                    1 => r.f32(5.0) | 0x8000_0000,
                    _ => r.f32(5.0) & 0x7fff_ffff,
                };
                c.bus.write32(DEF + 0x8, start);
                let old = if seed % 8 == 0 {
                    60
                } else {
                    r.next_u32() % 400
                };
                c.bus.write32(SELF + 0x178, old);
                // Rand_NextMain's state, the descriptor tables and templates.
                fill(c, &mut r, 0x004f_f018, 6);
                let base = (GAME_GP as i32 - 0x2868) as u32;
                for i in 0..24u32 {
                    c.bus.write32(base + 4 * i, r.f32(10.0));
                }
                fill(c, &mut r, 0x0050_0000 - 0xee0, 4);
                for i in 0..0x10u32 {
                    c.bus.write32(0x0050_0000 - 0xe60 + 4 * i, r.f32(4.0));
                }
                fill(c, &mut r, 0x0050_1420, 5);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(DEF);
                let time = r.f32(10.0) & 0x7fff_ffff;
                c.f[12] = if seed % 2 == 0 {
                    time | 0x8000_0000
                } else {
                    time
                };
            },
        );
    }

    /// `Avalanche_SaveState`: the 16 avalanche slots at 0x5389c0, each idle
    /// (def 0) or pointing at a definition with a random index, with a random
    /// time at +0x18; the stream's `Write` (vtable +8) is a fake.
    #[test]
    fn avalanche_save_state_matches() {
        const SLOTS: u32 = 0x0054_0000 - 0x76c8;
        const STREAM: u32 = DATA + 0x100;
        const STREAM_VT: u32 = DATA + 0x200;
        const DEFS: u32 = DATA + 0x1000;
        covered(
            ee::s09::f_002d9cb0__generated,
            avalanche_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d9cb0, 1);
                c.pc = 0x002d_9cb0;
                for i in 0..16u32 {
                    let slot = SLOTS + 0x1c * i;
                    fill(c, &mut r, slot, 7);
                    let idle = (seed + i) % 3 == 0 || seed % 11 == 0;
                    c.bus.write32(slot, if idle { 0 } else { DEFS + 0x10 * i });
                    c.bus.write32(DEFS + 0x10 * i, r.next_u32() % 40);
                    c.bus.write32(slot + 0x18, r.edge_f32());
                }
                c.bus.write32(STREAM, STREAM_VT);
                c.bus
                    .write16(STREAM_VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(STREAM_VT + 0xc, FAKE);
                c.g[A0][0] = sx(STREAM);
            },
        );
    }

    /// `ModifierBlock_FreeParticles`: an empty list, or 1..5 links (each
    /// `next` at +0, a vtable at +8 whose delete slot (+8) is a fake).
    #[test]
    fn modifier_block_free_particles_matches() {
        const BLOCK: u32 = DATA + 0x100;
        const LINKS: u32 = DATA + 0x1000;
        covered(
            ee::s11::f_003531d0__generated,
            modifier_block_free_particles,
            200,
            &|c, seed| {
                let links = seed % 6;
                let mut r = machine(c, seed, 0x3531d0, links);
                c.pc = 0x0035_31d0;
                let vt = DATA + 0x400;
                c.bus.write16(vt + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(vt + 0xc, FAKE);
                let link = |i: u32| if i < links { LINKS + 0x40 * i } else { 0 };
                fill(c, &mut r, BLOCK, 8);
                c.bus.write32(BLOCK + 0x10, link(0));
                for i in 0..links {
                    c.bus.write32(link(i), link(i + 1));
                    c.bus.write32(link(i) + 8, vt);
                }
                c.g[A0][0] = sx(BLOCK);
            },
        );
    }

    /// `SyncProcess_dtor` with the real `TypedProcess`/`GameProcess`
    /// destructors on a heap-allocated process (objnode's fixture: linked or
    /// not, delete flag clear or set).
    #[test]
    fn sync_process_dtor_matches() {
        use crate::game::object::objnode::tests::oc286_heap_process;
        check(ee::s11::f_00354920__generated, sync_process_dtor, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x35_4920, 0);
            let this = oc286_heap_process(c, &mut r, seed);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx((seed & 3) as u32);
            c.pc = 0x0035_4920;
        });
    }

    /// `WorldPainter_SaveViewState`: the six `RenderStateMan` entries at
    /// 0x4fa370 point (+0x20) at slots holding objects whose vtable (+4)
    /// `SaveViewState` slot (+0x200) is a fake (12 calls, some unwind).
    #[test]
    fn world_painter_save_view_state_matches() {
        const SLOTS: u32 = DATA + 0x100;
        const OBJECTS: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x4000;
        covered(
            ee::s09::f_002ef248__generated,
            world_painter_save_view_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ef248, 12);
                c.pc = 0x002e_f248;
                c.bus.write16(VT + 0x200, (r.next_u32() % 0x40) as u16 & !3);
                c.bus.write32(VT + 0x204, FAKE);
                for i in 0..6u32 {
                    let entry = RSM_ENTRIES + RSM_ENTRY_SIZE * i;
                    c.bus.write32(entry + 0x20, SLOTS + 4 * i);
                    c.bus.write32(SLOTS + 4 * i, OBJECTS + 0x100 * i);
                    c.bus.write32(OBJECTS + 0x100 * i + 4, VT);
                }
                c.g[A0][0] = sx(DATA + 0x8000 + (r.next_u32() & 0xff0));
            },
        );
    }

    /// `SSXAudioSystem_IsSpeechVolumeOn` with the real
    /// `SSXAudioSystem_GetChannelVolume`: the speech volume (+0x62fc) is
    /// +-0, a denormal (compares equal to 0) or a random float.
    #[test]
    fn ssx_audio_system_is_speech_volume_on_matches() {
        check(
            ee::s08::f_0029f0a0__generated,
            ssx_audio_system_is_speech_volume_on,
            200,
            &|c, seed| {
                let mut r = Rng(0x29f0a0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0029_f0a0;
                let volume = match seed % 4 {
                    0 => (r.next_u32() & 1) << 31,
                    1 => r.next_u32() & 0x807f_ffff,
                    _ => r.edge_f32(),
                };
                c.bus.write32(DATA + 0x40 + 0x62bc, volume);
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
