//! Staging wave 211 (oc301): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::{addu, reg, set_reg};
use crate::call::{Flow, VSlot};
use crate::guest::Addr;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_8380, multi_particle_save, "MultiParticle_Save"),
    (0x0032_7c00, ai_fwd_diff_cache_build_entry, "AIFwdDiffCache_BuildEntry"),
    (0x0032_7c68, ai_fwd_diff_cache_reset_entries, "AIFwdDiffCache_ResetEntries"),
    (0x002a_9988, snd_play_params_set_radius_vanish, "SndPlayParams_SetRadiusVanish"),
    (0x0015_47e8, be_mission_int_save_state, "BEMissionInt_SaveState"),
    (0x0036_cbf8, emitter_init_descriptor, "Emitter_InitDescriptor"),
    (0x0014_df08, be_library_save_state, "BELibrary_SaveState"),
    (0x0031_1bf0, anim_controller_transform_roots, "AnimController_TransformRoots"),
    (0x0035_7ca8, multi_particle_man_save, "MultiParticleMan_Save"),
];

// ---------------------------------------------------------------------------
// MultiParticle_Save (0x358380)

/// MultiParticle fields written by its save.
const MP_COUNT: u32 = 0x4;
const MP_TEMPLATE: u32 = 0x8;
const MP_ITEMS: u32 = 0xc;
const MP_EMITTER: u32 = 0x10;
const MP_BOUNDS: u32 = 0x20;
const MP_ORIGIN: u32 = 0x40;
/// Stream vtable slot `Write(data, size)`.
const MP_STREAM_WRITE: u32 = 0x8;
/// `Emitter_Save(emitter, stream)`.
const MP_EMITTER_SAVE: u32 = 0x0037_0aa8;

/// `stream->Write(a1, size)` with the stream in s0, as the original issues
/// it: v1 = vtable and v0 = function, or (`swapped`) the other way round.
fn multi_particle_write(c: &mut Cpu, data: u64, size: u64, swapped: bool, ret: u32) -> Flow {
    let stream = reg(c, S0);
    set_reg(c, A1, data);
    set_reg(c, A2, size);
    let vt = Addr(stream as u32).lw(c);
    let write = VSlot::read(c, vt, MP_STREAM_WRITE);
    if swapped {
        set_reg(c, V0, vt);
        set_reg(c, V1, write.func);
    } else {
        set_reg(c, V1, vt);
        set_reg(c, V0, write.func);
    }
    set_reg(c, A0, write.this(stream));
    call_ptr(c, write.func as u32, ret)
}

/// `void MultiParticle_Save(MultiParticle* self, Stream* stream)`
/// (0x358380): writes the object's first word, its item count, the shared
/// 0xd8-byte particle template, its emitter (`Emitter_Save`), the origin
/// (0x10 bytes) and bounds (0x20 bytes), then the item array (count words)
/// if it has any.
pub fn multi_particle_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x30);
        set_reg(c, A2, 4);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let stream = reg(c, A1);
        set_reg(c, S0, stream);
        frame.save_ra(c, 0);
        let this = reg(c, A0);
        set_reg(c, S1, this);

        multi_particle_write(c, this, 4, false, 0x0035_83b4)?;
        let this = reg(c, S1);
        multi_particle_write(c, addu(this, MP_COUNT as u64), 4, false, 0x0035_83d0)?;
        let template = Addr(r32(c, S1)).off(MP_TEMPLATE).lw(c);
        multi_particle_write(c, template, 0xd8, false, 0x0035_83ec)?;
        let emitter = Addr(r32(c, S1)).off(MP_EMITTER).lw(c);
        set_reg(c, A0, emitter);
        let stream = reg(c, S0);
        set_reg(c, A1, stream);
        call_ptr(c, MP_EMITTER_SAVE, 0x0035_83f8)?;
        let this = reg(c, S1);
        multi_particle_write(c, addu(this, MP_ORIGIN as u64), 0x10, false, 0x0035_8414)?;
        let this = reg(c, S1);
        multi_particle_write(c, addu(this, MP_BOUNDS as u64), 0x20, false, 0x0035_8430)?;

        let count = Addr(r32(c, S1)).off(MP_COUNT).lw(c);
        set_reg(c, A2, sx((count as u32) << 2));
        if (count as i64) > 0 {
            c.cycles += 6;
            let items = Addr(r32(c, S1)).off(MP_ITEMS).lw(c);
            let size = reg(c, A2);
            multi_particle_write(c, items, size, true, 0x0035_8454)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AIFwdDiffCache_BuildEntry (0x327c00) / AIFwdDiffCache_ResetEntries (0x327c68)

/// The VU0 register block the forward-difference microprogram works on
/// (`gp+0x2930`).
const AFD_VU_REGS: u32 = 0x2930;
/// Offset of the source curve's coefficient block in the source node.
const AFD_SRC_COEFFS: u32 = 0x40;
/// Offset of the output samples in the cache entry.
const AFD_ENTRY_SAMPLES: u32 = 0x10;
/// The two blocks of sample rows ResetEntries fills, their row counts
/// (minus one) and the gp-relative "far away" fill values.
const AFD_ROWS_A: u32 = 0x650;
const AFD_ROWS_A_LAST: u64 = 0x50;
const AFD_FILL_A: u32 = 0x2e04;
const AFD_ROWS_B: u32 = 0x1a90;
const AFD_ROWS_B_LAST: u64 = 0x8;
const AFD_FILL_B: u32 = 0x2e00;
const AFD_VU0_LOAD_REGS_AND_CALL: u32 = 0x0039_1418;
const AFD_VU0_READ_VI1: u32 = 0x0039_14f8;
const AFD_VU0_COPY_OUTPUT_MEM: u32 = 0x0039_1480;
const AFD_RESET_ENTRIES: Callee = Callee {
    addr: 0x0032_7c68,
    f: ee::s10::AIFwdDiffCache_ResetEntries,
};

/// `void AIFwdDiffCache_BuildEntry(AIFwdDiffCache* self, Node* src,
/// Entry* entry)` (0x327c00): loads the source curve's 16 coefficient
/// quadwords into VU0 and runs the forward-difference microprogram, resets
/// the entry's sample rows to "far away", reads VI1 and copies the VU0
/// output (10 x 0xa0 bytes) into the entry's samples.
pub fn ai_fwd_diff_cache_build_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        let src = reg(c, A1);
        set_reg(c, A1, addu(src, AFD_SRC_COEFFS as u64));
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = reg(c, A0);
        set_reg(c, S0, this);
        frame.save(c, S2, 0x10);
        let regs = sx((c.g[GP][0] as u32).wrapping_add(AFD_VU_REGS));
        set_reg(c, S1, regs);
        let entry = reg(c, A2);
        set_reg(c, S2, entry);
        frame.save_ra(c, 0);
        set_reg(c, A0, regs);
        call_ptr(c, AFD_VU0_LOAD_REGS_AND_CALL, 0x0032_7c2c)?;

        let entry = reg(c, S2);
        set_reg(c, A1, entry);
        let this = reg(c, S0);
        set_reg(c, A0, this);
        AFD_RESET_ENTRIES.call(c, 0x0032_7c38)?;

        let regs = reg(c, S1);
        set_reg(c, A0, regs);
        call_ptr(c, AFD_VU0_READ_VI1, 0x0032_7c40)?;

        let regs = reg(c, S1);
        set_reg(c, A0, regs);
        let entry = reg(c, S2);
        set_reg(c, A1, addu(entry, AFD_ENTRY_SAMPLES as u64));
        call_ptr(c, AFD_VU0_COPY_OUTPUT_MEM, 0x0032_7c4c)?;

        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Fill rows `x, +0x10, +0x20` of `last + 1` 0x40-byte sample rows starting
/// at `entry + rows` with f0 (the loop head is at `head`).
fn afd_fill_rows(c: &mut Cpu, entry: u32, rows: u32, last: u64, head: u32) {
    c.g[V1][0] = sx(entry.wrapping_add(rows));
    c.g[A0][0] = last;
    loop {
        c.cycles += 7;
        let row = r32(c, V1);
        c.swc1(0, row);
        c.g[V0][0] = c.g[A0][0];
        c.swc1(0, row.wrapping_add(0x10));
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
        c.swc1(0, row.wrapping_add(0x20));
        let more = (c.g[V0][0] as i64) > 0;
        c.g[V1][0] = sx(row.wrapping_add(0x40));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void AIFwdDiffCache_ResetEntries(AIFwdDiffCache* self, Entry* entry)`
/// (0x327c68): marks every sample of the entry's two row blocks (81 rows
/// at +0x650, 9 rows at +0x1a90; three floats per row) as "far away" with
/// the gp-relative fill values, so stale samples never win a distance test.
pub fn ai_fwd_diff_cache_reset_entries(c: &mut Cpu) {
    c.cycles += 4;
    let gp = c.g[GP][0] as u32;
    let entry = r32(c, A1);
    c.lwc1(0, gp.wrapping_sub(AFD_FILL_A));
    afd_fill_rows(c, entry, AFD_ROWS_A, AFD_ROWS_A_LAST, 0x0032_7c78);
    c.cycles += 3;
    let gp = c.g[GP][0] as u32;
    let entry = r32(c, A1);
    c.lwc1(0, gp.wrapping_sub(AFD_FILL_B));
    afd_fill_rows(c, entry, AFD_ROWS_B, AFD_ROWS_B_LAST, 0x0032_7ca0);
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SndPlayParams_SetRadiusVanish (0x2a9988)

/// Index of the current voice slot.
const SPP_SLOT: u32 = 0x1c;
/// Per-slot vanish (outer) and audible (inner) radius tables.
const SPP_VANISH_RADII: u32 = 0x68;
const SPP_RADII: u32 = 0x6c;

/// The address of entry `slot` of the float table at `params + table`
/// (left in v0, the table in v1, as the original does).
fn spp_slot_entry(c: &mut Cpu, params: u32, table: u32) -> u32 {
    let slot = Addr(params).off(SPP_SLOT).lw(c);
    let base = Addr(params).off(table).lw(c);
    c.g[V1][0] = base;
    let entry = (slot as u32) << 2;
    let entry = entry.wrapping_add(base as u32);
    c.g[V0][0] = sx(entry);
    entry
}

/// `void SndPlayParams_SetRadiusVanish(SndPlayParams* p, float radius,
/// float vanish)` (0x2a9988): sets the current slot's radius, and its
/// vanish radius to `vanish`, or to `radius` when `vanish` is negative.
pub fn snd_play_params_set_radius_vanish(c: &mut Cpu) {
    c.cycles += 8;
    let params = r32(c, A0);
    let radius = crate::abi::farg(c, 0);
    let vanish = crate::abi::farg(c, 1);
    let zero = crate::ps2f::FpuF32::ZERO;
    crate::abi::set_fpr(c, 0, zero);
    let entry = spp_slot_entry(c, params, SPP_RADII);
    let use_radius = crate::abi::c_lt_s(c, vanish, zero);
    Addr(entry).write_fpu(c, radius);
    c.cycles += 6;
    let entry = spp_slot_entry(c, params, SPP_VANISH_RADII);
    Addr(entry).write_fpu(c, if use_radius { radius } else { vanish });
    ret(c);
}

// ---------------------------------------------------------------------------
// BEMissionInt_SaveState (0x1547e8)

/// The per-character mission records (`0x4a6cac`, 0xf88 bytes per
/// character): a 0x114-byte block and a 0x160-byte block.
const BEMI_RECORDS: u32 = 0x004a_6cac;
const BEMI_RECORD_SIZE: u32 = 0xf88;
const BEMI_BLOCK_A: u64 = 0x114;
const BEMI_BLOCK_B: u64 = 0x160;
/// The mission interface's own saved word.
const BEMI_STATE: u32 = 0x8;
/// Stream vtable slot `Write(data, size)`.
const BEMI_STREAM_WRITE: u32 = 0x8;
const BEMI_GET_RIDER_CHARACTER: Callee = Callee {
    addr: 0x0014_a080,
    f: ee::s01::f_0014a080,
};

/// `stream->Write(a1, size)` with the stream in s1 (v1 = vtable, v0 =
/// function, a0 = adjusted stream).
fn bemi_stream_write(c: &mut Cpu, data: u64, size: u64, ret: u32) -> Flow {
    let stream = reg(c, S1);
    let vt = Addr(stream as u32).lw(c);
    set_reg(c, V1, vt);
    set_reg(c, A1, data);
    set_reg(c, A2, size);
    let write = VSlot::read(c, vt, BEMI_STREAM_WRITE);
    set_reg(c, V0, write.func);
    set_reg(c, A0, write.this(stream));
    call_ptr(c, write.func as u32, ret)
}

/// `void BEMissionInt_SaveState(BEMissionInt* self, Stream* stream)`
/// (0x1547e8): writes player 0's character's two mission record blocks,
/// then the interface's own state word.
pub fn be_mission_int_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        let this = reg(c, A0);
        set_reg(c, S3, this);
        let stream = reg(c, A1);
        set_reg(c, S1, stream);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        set_reg(c, A0, 0);
        BEMI_GET_RIDER_CHARACTER.call(c, 0x0015_4810)?;

        // Block A of this character's record.
        set_reg(c, V1, BEMI_RECORD_SIZE as u64);
        let stream = reg(c, S1);
        let vt = Addr(stream as u32).lw(c);
        set_reg(c, A1, vt);
        let character = r32(c, V0);
        let offset = crate::abi::mult(c, character, BEMI_RECORD_SIZE);
        set_reg(c, S2, sx(offset));
        set_reg(c, S0, BEMI_RECORDS as u64);
        let write = VSlot::read(c, vt, BEMI_STREAM_WRITE);
        set_reg(c, V0, write.func);
        set_reg(c, A2, BEMI_BLOCK_A);
        set_reg(c, A0, write.this(stream));
        set_reg(c, A1, sx(offset.wrapping_add(BEMI_RECORDS)));
        call_ptr(c, write.func as u32, 0x0015_483c)?;

        // Block B follows block A.
        let block_b = r32(c, S0).wrapping_add(BEMI_BLOCK_A as u32);
        set_reg(c, S0, sx(block_b));
        let data = sx(r32(c, S2).wrapping_add(block_b));
        bemi_stream_write(c, data, BEMI_BLOCK_B, 0x0015_485c)?;

        let state = addu(reg(c, S3), BEMI_STATE as u64);
        bemi_stream_write(c, state, 4, 0x0015_4878)?;

        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Emitter_InitDescriptor (0x36cbf8)

/// Particle-system descriptor fields.
const EMD_WIDTH: u32 = 0x0;
const EMD_HEIGHT: u32 = 0x4;
const EMD_U_STEP: u32 = 0xc;
const EMD_INV_HEIGHT: u32 = 0x30;
const EMD_V_SPAN: u32 = 0x34;
const EMD_SCALE: u32 = 0x4c;
const EMD_VELOCITY: u32 = 0x60;
const EMD_GS_STATE: u32 = 0x130;
const EMD_UV_SCALE: u32 = 0x140;
/// The default GS state quadword copied into every descriptor.
const EMD_DEFAULT_GS_STATE: u32 = 0x004f_f120;

/// `vf4 = *from / scale` through VU0 (`vdiv Q` of f14, `vmul.xyzw vf4,
/// vf4, Q`), stored to `sp+0x20`.
fn emd_divide_by_scale(c: &mut Cpu, from: u32) {
    use ssx3_ee_rt::vu0::Src;
    c.mfc1(V1, 14);
    c.qmtc2(V1, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, from);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, sp(c).wrapping_add(0x20));
}

/// `void Emitter_InitDescriptor(Desc* d, int width, int height, float x,
/// float y, float scale, const Vec4* velocity)` (0x36cbf8): sets up a
/// particle system's descriptor: its grid size, the per-column u step
/// (`x * scale / width`), the v span (`y * scale`), `1 / height`, the scale,
/// the velocity divided twice by the scale, the default GS state and a
/// (1, 1, 0, 0) uv scale.
pub fn emitter_init_descriptor(c: &mut Cpu) {
    use crate::abi::{farg, set_fpr};
    use crate::ps2f::FpuF32;
    c.cycles += 48;
    let desc = Addr(r32(c, A0));
    let width = FpuF32::from_i32(r32(c, A1) as i32);
    set_fpr(c, 0, width);
    let height = reg(c, A2);
    desc.off(EMD_HEIGHT).sw(c, height);
    let (x, y, scale) = (farg(c, 0), farg(c, 1), farg(c, 2));
    let frame = Frame::open(c, 0x30);
    let width_bits = reg(c, A1);
    desc.off(EMD_WIDTH).sw(c, width_bits);
    let v_span = y * scale;
    set_fpr(c, 13, v_span);
    set_reg(c, AT, 0x3f80_0000);
    set_fpr(c, 1, FpuF32::ONE);
    set_reg(c, A1, EMD_DEFAULT_GS_STATE as u64);
    let u_step = (x * scale).div(width);
    set_fpr(c, 12, u_step);
    desc.off(EMD_U_STEP).write_fpu(c, u_step);

    // velocity / scale / scale (VU0), through the stack.
    let velocity = r32(c, A3);
    emd_divide_by_scale(c, velocity);
    let tmp = sp(c);
    c.lq(V0, tmp.wrapping_add(0x20));
    c.sq(V0, tmp.wrapping_add(0x10));
    emd_divide_by_scale(c, tmp.wrapping_add(0x10));

    let height = FpuF32::from_i32(desc.off(EMD_HEIGHT).lw(c) as i32);
    c.lq(V0, tmp.wrapping_add(0x20));
    desc.off(EMD_V_SPAN).write_fpu(c, v_span);
    c.sq(V0, tmp);
    let inv_height = FpuF32::ONE.div(height);
    set_fpr(c, 0, inv_height);
    c.sq(V0, desc.off(EMD_VELOCITY).0);

    // uv scale (1, 1, 0, 0) and the default GS state.
    Addr(tmp).off(4).write_fpu(c, FpuF32::ONE);
    desc.off(EMD_SCALE).write_fpu(c, scale);
    Addr(tmp).write_fpu(c, FpuF32::ONE);
    Addr(tmp).off(8).sw(c, 0);
    Addr(tmp).off(0xc).sw(c, 0);
    desc.off(EMD_INV_HEIGHT).write_fpu(c, inv_height);
    c.lq(V0, tmp);
    c.lq(V1, EMD_DEFAULT_GS_STATE);
    c.sq(V0, desc.off(EMD_UV_SCALE).0);
    c.sq(V1, desc.off(EMD_GS_STATE).0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// BELibrary_SaveState (0x14df08)

/// The library's current state index (`gp-0x1efc`), saved first.
const BEL_STATE_INDEX: u32 = 0x1efc;
/// Number of interfaces a library holds.
const BEL_INTERFACES: i64 = 14;
/// Stream vtable slot `Write(data, size)`.
const BEL_STREAM_WRITE: u32 = 0x8;
/// Interface vtable (at +0xc) slot `WriteToReplayFrame(stream)`.
const BEL_IFACE_VT: u32 = 0xc;
const BEL_IFACE_SAVE: u32 = 0x18;
const BEL_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};

/// `void BELibrary_SaveState(BELibrary* self, Stream* stream)` (0x14df08):
/// writes the current state index, then lets each of the 14 interfaces
/// that exists write its replay frame to the stream.
pub fn be_library_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        set_reg(c, A2, 4);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        set_reg(c, S0, 0);
        frame.save(c, S2, 0x10);
        let this = reg(c, A0);
        set_reg(c, S1, this);
        frame.save_ra(c, 0);
        let stream = reg(c, A1);
        set_reg(c, S2, stream);
        let index = sx((c.g[GP][0] as u32).wrapping_sub(BEL_STATE_INDEX));
        set_reg(c, A1, index);
        let vt = Addr(stream as u32).lw(c);
        set_reg(c, V0, vt);
        let write = VSlot::read(c, vt, BEL_STREAM_WRITE);
        set_reg(c, V1, write.func);
        set_reg(c, A0, write.this(stream));
        call_ptr(c, write.func as u32, 0x0014_df44)?;

        let i = reg(c, S0);
        set_reg(c, A1, i);
        loop {
            c.cycles += 6;
            let this = reg(c, S1);
            set_reg(c, A0, this);
            BEL_GET_INTERFACE.call(c, 0x0014_df50)?;
            let i = reg(c, S0);
            set_reg(c, A1, i);
            let this = reg(c, S1);
            set_reg(c, A0, this);
            let present = reg(c, V0) != 0;
            set_reg(c, S0, sx((i as u32).wrapping_add(1)));
            if present {
                c.cycles += 8;
                BEL_GET_INTERFACE.call(c, 0x0014_df68)?;
                let iface = reg(c, V0);
                let vt = Addr(iface as u32).off(BEL_IFACE_VT).lw(c);
                set_reg(c, V1, vt);
                let stream = reg(c, S2);
                set_reg(c, A1, stream);
                let save = VSlot::read(c, vt, BEL_IFACE_SAVE);
                set_reg(c, A2, save.func);
                set_reg(c, A0, save.this(iface));
                call_ptr(c, save.func as u32, 0x0014_df80)?;
            }
            c.cycles += 3;
            let next = reg(c, S0);
            let more = (next as i64) < BEL_INTERFACES;
            set_reg(c, V0, more as u64);
            set_reg(c, A1, next);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_df48);
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

// ---------------------------------------------------------------------------
// AnimController_TransformRoots (0x311bf0)

/// The controller's six layer lists (`{count, head}`, 8 bytes each).
const ACT_LAYERS: u32 = 0x50;
const ACT_LAYER_COUNT: i64 = 6;
/// A sequence's root transform: position (+0x60) and rotation quaternion
/// (+0x70); the transform argument has them at +0 and +0x10.
const ACT_SEQ_ROOT_POS: u32 = 0x60;
const ACT_SEQ_ROOT_ROT: u32 = 0x70;
const ACT_XF_POS: u32 = 0x0;
const ACT_XF_ROT: u32 = 0x10;
/// The root pivot (`0x500000 - 0xed0`, the zero translation).
const ACT_ROOT_PIVOT_HI: u64 = 0x0050_0000;
const ACT_ROOT_PIVOT_LO: u32 = 0xed0;
const ACT_SEQ_LIST_GET_AT: Callee = Callee {
    addr: 0x0031_4760,
    f: ee::s10::AnimSeqList_GetAt,
};

/// One sequence's root: `rot = xf.rot * src.rot`, `pos = xf.rot *
/// (src.pos - pivot) * conj(xf.rot) + xf.pos`, through VU0 and the stack as
/// the original does; written to `dst` (the same sequence). Leaves a0 and
/// v1 (all 128 bits) as the original's quadword moves do.
fn act_transform_root(c: &mut Cpu, xf: u32, src: u32, dst: u32, pivot: u32) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    let sp = sp(c);
    // Quaternion product xf.rot * src.rot.
    c.lqc2(4, xf.wrapping_add(ACT_XF_ROT));
    c.lqc2(5, src.wrapping_add(ACT_SEQ_ROOT_ROT));
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, sp.wrapping_add(0x50));
    c.lq(V1, sp.wrapping_add(0x50));
    c.sq(V1, sp.wrapping_add(0x30));
    c.sq(V1, sp.wrapping_add(0x40));
    // Offset from the pivot.
    c.lqc2(3, src.wrapping_add(ACT_SEQ_ROOT_POS));
    c.lqc2(4, pivot);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x70));
    c.lq(V1, sp.wrapping_add(0x70));
    c.sq(V1, sp.wrapping_add(0x60));
    // Rotate it by xf.rot.
    c.lqc2(4, xf.wrapping_add(ACT_XF_ROT));
    c.lqc2(5, sp.wrapping_add(0x60));
    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().opmula(4, 6);
    c.vu0().opmsub(7, 6, 4);
    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, sp.wrapping_add(0x70));
    c.lq(V1, sp.wrapping_add(0x70));
    c.sq(V1, sp.wrapping_add(0x50));
    // Plus xf.pos.
    c.lqc2(3, sp.wrapping_add(0x50));
    c.lqc2(4, xf.wrapping_add(ACT_XF_POS));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp.wrapping_add(0x70));
    // Copy the new transform out (through the stack temporaries).
    c.lq(A0, sp.wrapping_add(0x70));
    c.lq(V1, sp.wrapping_add(0x30));
    c.sq(A0, sp.wrapping_add(0x40));
    c.sq(V1, sp.wrapping_add(0x10));
    c.sq(A0, sp.wrapping_add(0x20));
    c.sq(A0, sp);
    c.sq(A0, dst.wrapping_add(ACT_SEQ_ROOT_POS));
    c.lq(V1, sp.wrapping_add(0x10));
    c.sq(V1, dst.wrapping_add(ACT_SEQ_ROOT_ROT));
}

/// `void AnimController_TransformRoots(AnimController* ctrl, const Xform*
/// xf)` (0x311bf0): applies `xf` (position + rotation quaternion) to the
/// root transform of every sequence in the controller's six layers.
pub fn anim_controller_transform_roots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        let ctrl = reg(c, A0);
        set_reg(c, S3, ctrl);
        frame.save(c, S0, 0xf0);
        let xf = reg(c, A1);
        set_reg(c, S4, xf);
        frame.save(c, S1, 0xe0);
        set_reg(c, A0, 0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save_ra(c, 0x80);
        let layers = Addr(r32(c, S3)).off(ACT_LAYERS).lw(c);
        set_reg(c, V1, layers);
        loop {
            c.cycles += 6;
            let layer = r32(c, A0);
            let list_off = layer << 3;
            set_reg(c, A1, sx(list_off));
            set_reg(c, S5, sx(layer.wrapping_add(1)));
            let list = list_off.wrapping_add(r32(c, V1));
            set_reg(c, V1, sx(list));
            let count = Addr(list).lw(c);
            set_reg(c, V0, count);
            set_reg(c, S1, 0);
            if (count as i64) > 0 {
                c.cycles += 4;
                set_reg(c, S2, sx(list_off));
                set_reg(c, S6, ACT_ROOT_PIVOT_HI);
                let layers = Addr(r32(c, S3)).off(ACT_LAYERS).lw(c);
                set_reg(c, A0, layers);
                loop {
                    c.cycles += 64;
                    let i = reg(c, S1);
                    set_reg(c, A1, i);
                    let list = r32(c, A0).wrapping_add(r32(c, S2));
                    set_reg(c, A0, sx(list));
                    ACT_SEQ_LIST_GET_AT.call(c, 0x0031_1c5c)?;
                    let layers = Addr(r32(c, S3)).off(ACT_LAYERS).lw(c);
                    let src = reg(c, V0);
                    set_reg(c, S0, src);
                    let i = reg(c, S1);
                    set_reg(c, A1, i);
                    set_reg(c, A0, sx((layers as u32).wrapping_add(r32(c, S2))));
                    set_reg(c, S1, sx((i as u32).wrapping_add(1)));
                    ACT_SEQ_LIST_GET_AT.call(c, 0x0031_1c74)?;
                    let pivot = r32(c, S6).wrapping_sub(ACT_ROOT_PIVOT_LO);
                    let (xf, src, dst) = (r32(c, S4), r32(c, S0), r32(c, V0));
                    act_transform_root(c, xf, src, dst, pivot);

                    let layers = Addr(r32(c, S3)).off(ACT_LAYERS).lw(c);
                    let list = r32(c, S2).wrapping_add(layers as u32);
                    set_reg(c, V0, sx(list));
                    let count = Addr(list).lw(c);
                    let more = (reg(c, S1) as i64) < (count as i64);
                    set_reg(c, V1, more as u64);
                    if !more {
                        break;
                    }
                    let layers = Addr(r32(c, S3)).off(ACT_LAYERS).lw(c);
                    set_reg(c, A0, layers);
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_1c50);
                    }
                }
            }
            c.cycles += 4;
            let next = reg(c, S5);
            set_reg(c, A0, next);
            let more = (next as i64) < ACT_LAYER_COUNT;
            set_reg(c, V0, more as u64);
            if !more {
                break;
            }
            let layers = Addr(r32(c, S3)).off(ACT_LAYERS).lw(c);
            set_reg(c, V1, layers);
            if event_due(c) {
                deliver_event_here(c, 0x0031_1c28);
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MultiParticleMan_Save (0x357ca8)

/// The manager's four MultiParticle slots (one word each).
const MPM_SLOTS_LAST: u64 = 3;
const MPM_MULTI_PARTICLE_SAVE: Callee = Callee {
    addr: 0x0035_8380,
    f: ee::s11::MultiParticle_Save,
};

/// `void MultiParticleMan_Save(MultiParticleMan* self, Stream* stream)`
/// (0x357ca8): for each of the four slots writes a "present" word (staged
/// at `sp+0`) and, when present, the MultiParticle itself.
pub fn multi_particle_man_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        let this = reg(c, A0);
        set_reg(c, S0, this);
        frame.save(c, S2, 0x20);
        let stream = reg(c, A1);
        set_reg(c, S1, stream);
        frame.save_ra(c, 0x10);
        set_reg(c, S2, MPM_SLOTS_LAST);
        loop {
            c.cycles += 14;
            let stream = reg(c, S1);
            let vt = Addr(stream as u32).lw(c);
            set_reg(c, A3, vt);
            let flag_at = sp(c);
            set_reg(c, A1, sx(flag_at));
            let particle = Addr(r32(c, S0)).lw(c);
            set_reg(c, A2, 4);
            let write = VSlot::read(c, vt, MP_STREAM_WRITE);
            set_reg(c, S2, sx(r32(c, S2).wrapping_sub(1)));
            let present = (particle != 0) as u64;
            set_reg(c, V0, present);
            set_reg(c, V1, write.func);
            Addr(flag_at).sw(c, present);
            set_reg(c, A0, write.this(stream));
            call_ptr(c, write.func as u32, 0x0035_7cf4)?;

            let present = Addr(sp(c)).lw(c);
            set_reg(c, V0, present);
            let stream = reg(c, S1);
            set_reg(c, A1, stream);
            if present != 0 {
                c.cycles += 2;
                let particle = Addr(r32(c, S0)).lw(c);
                set_reg(c, A0, particle);
                MPM_MULTI_PARTICLE_SAVE.call(c, 0x0035_7d08)?;
            }
            c.cycles += 2;
            let more = (reg(c, S2) as i64) >= 0;
            set_reg(c, S0, sx(r32(c, S0).wrapping_add(4)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_7cc8);
            }
        }
        c.cycles += 6;
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
    use crate::difftest::fakes::{covered, machine, vtable, FAKE};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// MultiParticle_Save: a stream whose `Write` slot is a fake (random
    /// `this` delta, some calls unwind), the real Emitter_Save, and an item
    /// count that is negative, zero or positive.
    #[test]
    fn multi_particle_save_matches() {
        covered(ee::s11::f_00358380__generated, multi_particle_save, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_8380, 7);
            c.pc = 0x0035_8380;
            let vt = vtable(c, &mut r, &[MP_STREAM_WRITE]);
            let this = DATA + 0x20_0000;
            let stream = DATA + 0x20_0400;
            let emitter = DATA + 0x20_0800;
            fill(c, &mut r, this, 0x20);
            fill(c, &mut r, emitter, 0x10);
            c.bus.write32(this + MP_EMITTER, emitter);
            let count = match seed % 4 {
                0 => 0,
                1 => (r.next_u32() | 0x8000_0000) as i32,
                _ => (r.next_u32() % 0x40) as i32 + 1,
            };
            c.bus.write32(this + MP_COUNT, count as u32);
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }

    /// Stage the VU0 microprogram at address 0 as `NOP / NOP[E]` so
    /// Vu0_LoadRegsAndCall's `vcallmsr` (CMSAR0 = 0) returns at once.
    fn afd_stub_microprogram(c: &mut Cpu) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0.fallback =
            Some(ssx3_ps2::vu::reference::run_program);
        c.bus.write32(0x1100_0000, 0x8000_033c); // NOP (lower)
        c.bus.write32(0x1100_0004, 0x4000_02ff); // NOP, E (upper)
        c.bus.write32(0x1100_0008, 0x8000_033c); // NOP
        c.bus.write32(0x1100_000c, 0x0000_02ff); // NOP
    }

    /// AIFwdDiffCache_BuildEntry: real callees (the VU0 loader with a stub
    /// microprogram, ResetEntries, VI1 read, output copy) on random source
    /// coefficients, random VU0 output memory and random fill values.
    #[test]
    fn ai_fwd_diff_cache_build_entry_matches() {
        check(
            ee::s10::f_00327c00__generated,
            ai_fwd_diff_cache_build_entry,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32_7c00, 0);
                c.pc = 0x0032_7c00;
                afd_stub_microprogram(c);
                let this = DATA + 0x20_0000;
                let src = DATA + 0x20_0100;
                let entry = DATA + 0x20_1000;
                fill(c, &mut r, src + AFD_SRC_COEFFS, 0x40);
                fill(c, &mut r, 0x1100_4200, 0x190);
                fill(c, &mut r, entry, 0x740);
                c.bus.write32(GAME_GP - AFD_FILL_A, r.next_u32());
                c.bus.write32(GAME_GP - AFD_FILL_B, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(entry);
            },
        );
    }

    /// ResetEntries fixture: an entry at a random place, random rows and
    /// fill values.
    fn afd_reset_entries_setup(c: &mut Cpu, r: &mut Rng) {
        scramble(c, r);
        c.pc = 0x0032_7c68;
        c.g[GP][0] = GAME_GP as u64;
        let entry = DATA + 0x20_0000 + (r.next_u32() & 0xff0);
        fill(c, r, entry + AFD_ROWS_A, 0x80);
        c.bus.write32(GAME_GP - AFD_FILL_A, r.edge_f32());
        c.bus.write32(GAME_GP - AFD_FILL_B, r.edge_f32());
        c.g[A1][0] = sx(entry);
    }

    /// AIFwdDiffCache_ResetEntries: both row blocks filled.
    #[test]
    fn ai_fwd_diff_cache_reset_entries_matches() {
        check(
            ee::s10::f_00327c68__generated,
            ai_fwd_diff_cache_reset_entries,
            200,
            &|c, seed| {
                let mut r = Rng(0x32_7c68 ^ (seed as u64) << 16);
                afd_reset_entries_setup(c, &mut r);
            },
        );
    }

    /// AIFwdDiffCache_ResetEntries with events due at the loops' back edges.
    #[test]
    fn ai_fwd_diff_cache_reset_entries_events_match() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s10::f_00327c68__generated,
            ai_fwd_diff_cache_reset_entries,
            90,
            &|c, seed| {
                let mut r = Rng(0x32_7c69 ^ (seed as u64) << 16);
                afd_reset_entries_setup(c, &mut r);
                let ev = schedule(c, &mut r, seed, 0x0032_7c68);
                c.g[GP][0] = GAME_GP as u64;
                ev
            },
        );
    }

    /// SndPlayParams_SetRadiusVanish: random slot and tables, edge-case
    /// radii (negative, zero, -0, NaN-like) for both branches.
    #[test]
    fn snd_play_params_set_radius_vanish_matches() {
        check(
            ee::s08::f_002a9988__generated,
            snd_play_params_set_radius_vanish,
            300,
            &|c, seed| {
                let mut r = Rng(0x2a_9988 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x002a_9988;
                let params = DATA;
                c.bus.write32(params + SPP_SLOT, r.next_u32() % 16);
                c.bus.write32(params + SPP_RADII, DATA + 0x100);
                let vanish_table = if seed % 5 == 0 { DATA + 0x100 } else { DATA + 0x200 };
                c.bus.write32(params + SPP_VANISH_RADII, vanish_table);
                c.f[12] = r.edge_f32();
                c.f[13] = match seed % 3 {
                    0 => r.f32(100.0) | 0x8000_0000,
                    1 => r.f32(100.0) & 0x7fff_ffff,
                    _ => r.edge_f32(),
                };
                c.g[A0][0] = sx(params);
            },
        );
    }

    /// BEMissionInt_SaveState: player 0's slot and character byte chosen
    /// per seed (BE_GetRiderCharacter is real), the stream's `Write` a fake.
    #[test]
    fn be_mission_int_save_state_matches() {
        covered(ee::s01::f_001547e8__generated, be_mission_int_save_state, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x15_47e8, 3);
            c.pc = 0x0015_47e8;
            let vt = vtable(c, &mut r, &[BEMI_STREAM_WRITE]);
            // BE_GetRiderCharacter(0): slot word at 0x5305b0, character byte
            // at 0x535b20 + slot * 0x1c + 0x11.
            let slot = r.next_u32() % 8;
            c.bus.write32(0x0053_05b0, slot);
            c.bus.write8(0x0053_5b20 + slot * 0x1c + 0x11, (r.next_u32() % 12) as u8);
            if seed % 7 == 0 {
                c.bus.write8(0x0053_5b20 + slot * 0x1c + 0x11, 0xff);
            }
            let this = DATA + 0x20_0000;
            let stream = DATA + 0x20_0400;
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }

    /// Emitter_InitDescriptor: random grid sizes (zero and negative
    /// included), edge-case floats for x, y and the scale, a random velocity
    /// and default GS state.
    #[test]
    fn emitter_init_descriptor_matches() {
        check(
            ee::s12::f_0036cbf8__generated,
            emitter_init_descriptor,
            300,
            &|c, seed| {
                let mut r = Rng(0x36_cbf8 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0036_cbf8;
                let desc = DATA;
                let velocity = DATA + 0x400;
                fill(c, &mut r, desc, 0x60);
                fill(c, &mut r, EMD_DEFAULT_GS_STATE, 4);
                for i in 0..4 {
                    let v = if seed % 4 == 0 { r.edge_f32() } else { r.f32(50.0) };
                    c.bus.write32(velocity + 4 * i, v);
                }
                let dim = |r: &mut Rng| match seed % 6 {
                    0 => 0,
                    1 => r.next_u32(),
                    _ => r.next_u32() % 64 + 1,
                };
                c.g[A1][0] = (c.g[A1][0] & !0xffff_ffff) | dim(&mut r) as u64;
                c.g[A2][0] = (c.g[A2][0] & !0xffff_ffff) | dim(&mut r) as u64;
                c.f[12] = if seed % 3 == 0 { r.edge_f32() } else { r.f32(10.0) };
                c.f[13] = if seed % 3 == 1 { r.edge_f32() } else { r.f32(10.0) };
                c.f[14] = if seed % 5 == 0 { r.edge_f32() } else { r.f32(4.0) };
                c.g[A0][0] = sx(desc);
                c.g[A3][0] = sx(velocity);
            },
        );
    }

    /// BELibrary_SaveState: state 0's object (table 0x5308a8) answers
    /// GetInterface through a fake (every interface present) or through the
    /// real `return 0` leaf 0x1406c0 (none present). A fake's interface
    /// pointer lands anywhere in 0x800000..0x840000, so every 16-byte slot
    /// there gets the interface vtable at +0xc.
    #[test]
    fn be_library_save_state_matches() {
        covered(ee::s01::f_0014df08__generated, be_library_save_state, 80, &|c, seed| {
            let mut r = machine(c, seed, 0x14_df08, 30);
            c.pc = 0x0014_df08;
            let lib = DATA + 0x20_0000;
            let stream = DATA + 0x20_0100;
            let stream_vt = DATA + 0x20_0200;
            let state = DATA + 0x20_0300;
            let state_vt = DATA + 0x20_0400;
            let iface_vt = DATA + 0x20_0500;
            for slot in (0x0080_0000..0x0084_0000u32).step_by(16) {
                c.bus.write32(slot + BEL_IFACE_VT, iface_vt);
            }
            c.bus.write32(GAME_GP - BEL_STATE_INDEX, 0);
            c.bus.write32(0x0053_08a8, state);
            c.bus.write32(state + 4, state_vt);
            c.bus.write16(state_vt + 0x8, 0);
            let lookup = if seed % 4 == 0 { 0x0014_06c0 } else { FAKE };
            c.bus.write32(state_vt + 0xc, lookup);
            c.bus.write32(stream, stream_vt);
            c.bus.write16(stream_vt + BEL_STREAM_WRITE, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(stream_vt + BEL_STREAM_WRITE + 4, FAKE + 0x10);
            c.bus.write16(iface_vt + BEL_IFACE_SAVE, (r.next_u32() as i16 & !3) as u16);
            c.bus.write32(iface_vt + BEL_IFACE_SAVE + 4, FAKE + 0x20);
            c.g[A0][0] = sx(lib);
            c.g[A1][0] = sx(stream);
        });
    }

    /// AnimController_TransformRoots fixture: six layer lists of 0..3
    /// sequences (0xd0 bytes, linked at +0xc8) with random root transforms,
    /// a random transform argument and pivot.
    fn act_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let ctrl = DATA + 0x20_0000;
        let lists = DATA + 0x20_0400;
        let seqs = DATA + 0x20_1000;
        let xf = DATA + 0x20_0800;
        c.bus.write32(ctrl + ACT_LAYERS, lists);
        for layer in 0..6 {
            let n = if seed % 5 == 0 { 0 } else { r.next_u32() % 4 };
            let mut head = 0;
            for i in 0..n {
                let s = seqs + 0x100 * (layer * 4 + i);
                for k in 0..8 {
                    c.bus.write32(s + ACT_SEQ_ROOT_POS + 4 * k, r.f32(4.0));
                }
                c.bus.write32(s + 0xc8, head);
                head = s;
            }
            c.bus.write32(lists + 8 * layer, n);
            c.bus.write32(lists + 8 * layer + 4, head);
        }
        for k in 0..8 {
            let v = if seed % 7 == 3 { r.edge_f32() } else { r.f32(2.0) };
            c.bus.write32(xf + 4 * k, v);
        }
        for k in 0..4 {
            c.bus.write32(0x004f_f130 + 4 * k, r.f32(1.0));
        }
        c.g[A0][0] = sx(ctrl);
        c.g[A1][0] = sx(xf);
    }

    /// AnimController_TransformRoots: real AnimSeqList_GetAt on the
    /// fixture's lists (empty layers included).
    #[test]
    fn anim_controller_transform_roots_matches() {
        check(
            ee::s10::f_00311bf0__generated,
            anim_controller_transform_roots,
            200,
            &|c, seed| {
                let mut r = Rng(0x31_1bf0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0031_1bf0;
                act_fixture(c, &mut r, seed);
            },
        );
    }

    /// MultiParticleMan_Save: each of the four slots empty or holding a
    /// MultiParticle (saved by the real MultiParticle_Save and Emitter_Save
    /// into the fake stream `Write`, some calls unwinding).
    #[test]
    fn multi_particle_man_save_matches() {
        covered(ee::s11::f_00357ca8__generated, multi_particle_man_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x35_7ca8, 20);
            c.pc = 0x0035_7ca8;
            let vt = vtable(c, &mut r, &[MP_STREAM_WRITE]);
            let man = DATA + 0x20_0000;
            let stream = DATA + 0x20_0100;
            let bits = if seed % 9 == 0 { 0 } else { r.next_u32() };
            for i in 0..4 {
                let particle = DATA + 0x20_1000 + 0x400 * i;
                let emitter = particle + 0x200;
                fill(c, &mut r, particle, 0x20);
                fill(c, &mut r, emitter, 0x10);
                c.bus.write32(particle + MP_EMITTER, emitter);
                c.bus.write32(particle + MP_COUNT, r.next_u32() % 8);
                let present = (bits >> i) & 1 != 0;
                c.bus.write32(man + 4 * i, if present { particle } else { 0 });
            }
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(stream);
        });
    }
}
