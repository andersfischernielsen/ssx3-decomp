//! Staging wave 208 (oc262): hand-written replacements for the hottest
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
use crate::abi::{self, addu, reg, set_reg};
use crate::call::{Flow, VSlot};
use crate::game::lib::snd;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_27e0, snd_channel_apply_send, "SND_ChannelApplySend"),
    (0x002d_1ba0, world_get_peak_number, "World_GetPeakNumber"),
    (0x0032_f840, coll_hull_transform, "CollHull_Transform"),
    (0x003b_7080, sndstrm_handle_chunk, "SNDSTRM_HandleChunk"),
    (0x0034_c2e0, cflag_manager_group_save, "cFlagManager_GroupSave"),
    (0x0035_0f40, hide_save, "Hide_Save"),
    (0x003b_5450, snd_timer_service_callback, "SND_TimerServiceCallback"),
    (0x0034_5bf0, particle_modifier_save, "ParticleModifier_Save"),
    (0x003c_2ec8, snd_channel_apply_fx_enable, "SND_ChannelApplyFxEnable"),
];

// ---------------------------------------------------------------------------
// SND_ChannelApplySend (0x3c27e0).

/// `SndChannel` field: `s16[]` send levels, one per aux send (index 0 is
/// the SPU effect on/off switch).
const SEND_LEVELS: u32 = 0x62;

/// `1/32767 * 127` (the IOP DSP send byte scale).
const SEND_SCALE_IOP: FpuF32 = FpuF32(0x3b7e_01fc);

/// `32767.0` (the EE send divisor).
const SEND_DIVISOR_EE: FpuF32 = FpuF32(0x46ff_fe00);

/// `SND_EeVoiceSetAuxSend(int voice, int send, float level)`.
const EE_VOICE_SET_AUX_SEND: u32 = 0x003c_8598;

/// IOP DSP voice block: the send bytes (`+ (v - base) * 16 + send`).
const IOP_SEND_BYTES: u32 = 0x1ac;

/// `int SND_ChannelApplySend(int channel, int send)` (0x3c27e0): push the
/// channel's aux send level to its voices. Returns 0.
///
/// ```text
/// SPU voices:  effect mask (u64 at *params) bit v (and v + 0x18 in output
///              mode 2) = send_levels[0] != 0
/// IOP voices:  params[(v - base) * 16 + send + 0x1ac] =
///                  (int)(volume * send_levels[send] * 127/32767)
/// EE voices:   SND_EeVoiceSetAuxSend(v - base - ee_base, send,
///                  volume * send_levels[send] / 32767)
/// ```
///
/// Frame 0x70 (s0 +0x50, s1 +0x40, s2 +0x30, s3 +0x20, s4 +0x10, ra +0,
/// f20 +0x60).
pub fn snd_channel_apply_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        set_reg(c, V0, snd::chan::SIZE as u64);
        frame.save(c, S1, 0x40);
        set_reg(c, V1, snd::STATE_HI);
        frame.save(c, S4, 0x10);
        let offset = snd::channel_at(c);
        frame.save(c, S0, 0x50);
        set_reg(c, A3, sx(snd::STATE));
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x60);
        let channels = Addr(snd::STATE).off(snd::state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset as u32));
        set_reg(c, S1, ch);
        let flags = snd::at(ch, snd::chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & snd::chan::SPU);
        let send = reg(c, A1);
        set_reg(c, S4, send);
        if flags & snd::chan::SPU != 0 {
            return snd_send_spu(c, frame, ch);
        }
        c.cycles += 3;
        let level_off = snd::sll(send, 1);
        set_reg(c, V0, level_off);
        if flags & snd::chan::IOP != 0 {
            snd_send_iop(c, frame, ch, level_off)
        } else {
            snd_send_ee(c, frame, ch, level_off)
        }
    })
}

/// `volume * (float)send_levels[send]` (f1; f0 the converted level, AT and
/// f2 the scale constant, v1 the level, v0 the voice count, s2 = 0).
/// Returns (product, voice count).
fn snd_send_level(c: &mut Cpu, ch: u64, level_off: u64, scale: FpuF32) -> (FpuF32, u64) {
    let volume = snd::at(ch, snd::chan::VOLUME).read_fpu(c);
    let level = snd::at(addu(ch, level_off), SEND_LEVELS).lh(c);
    set_reg(c, AT, scale.0 as u64);
    abi::set_fpr(c, 2, scale);
    set_reg(c, V1, level);
    let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
    set_reg(c, V0, count);
    let level_f = FpuF32::from_i32(level as u32 as i32);
    abi::set_fpr(c, 0, level_f);
    let product = volume * level_f;
    abi::set_fpr(c, 1, product);
    set_reg(c, S2, 0);
    (product, count)
}

/// SPU voices: set or clear each voice's bit in the effect mask.
fn snd_send_spu(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 3;
    let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
    set_reg(c, V0, count);
    set_reg(c, S2, 0);
    if count == 0 {
        return snd_send_exit(c, frame, 1);
    }
    c.cycles += 9;
    set_reg(c, V0, snd::STATE_HI);
    let state_ptr = reg(c, A3);
    set_reg(c, T4, state_ptr);
    set_reg(c, T0, sx(snd::SYNC));
    set_reg(c, A3, 1);
    set_reg(c, T3, 2);
    set_reg(c, T2, 0);
    set_reg(c, T1, 0);
    set_reg(c, A2, snd::addiu(ch, snd::chan::VOICES as i32));
    loop {
        // Loop head 0x3c2858 (a2 the voice slot, s2 the voices done, t1/t2
        // the slot offset).
        c.cycles += 3 + 6;
        let on = snd::at(ch, SEND_LEVELS).lh(c);
        let slots = snd::addiu(ch, snd::chan::VOICES as i32);
        set_reg(c, A1, slots);
        let mask_at = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
        set_reg(c, V0, mask_at);
        let voice = snd::at(reg(c, A2), 0).lh(c);
        let mask = snd::at(mask_at, 0).ld(c);
        let bit = 1u64 << (voice & 63);
        let (v1, new_mask) = if on != 0 { (bit, mask | bit) } else { (!bit, mask & !bit) };
        set_reg(c, V1, v1);
        set_reg(c, A0, new_mask);
        c.cycles += 4;
        snd::at(mask_at, 0).sd(c, new_mask);
        let mode = snd::at(reg(c, T4), snd::state::OUTPUT_MODE).lbu(c);
        set_reg(c, V0, mode);
        if mode == 2 {
            // The second voice block (bits + 0x18).
            c.cycles += 3;
            let on = snd::at(ch, SEND_LEVELS).lh(c);
            set_reg(c, V0, on);
            let slot_off = reg(c, T1);
            set_reg(c, A0, addu(slots, slot_off));
            let (mask_at, v0, v1);
            if on != 0 {
                c.cycles += 7;
                mask_at = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let voice = snd::at(reg(c, A0), 0).lh(c);
                let mask = snd::at(mask_at, 0).ld(c);
                v0 = 1u64 << (snd::addiu(voice, 0x18) & 63);
                v1 = mask | v0;
            } else {
                c.cycles += 8;
                let slot_off = reg(c, T2);
                set_reg(c, A0, addu(slots, slot_off));
                let voice = snd::at(reg(c, A0), 0).lh(c);
                mask_at = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let mask = snd::at(mask_at, 0).ld(c);
                v0 = !(1u64 << (snd::addiu(voice, 0x18) & 63));
                v1 = mask & v0;
            }
            set_reg(c, A1, mask_at);
            set_reg(c, V0, v0);
            set_reg(c, V1, v1);
            c.cycles += 2;
            snd::at(mask_at, 0).sd(c, v1);
        }
        let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
        c.cycles += 6;
        let done = snd::addiu(reg(c, S2), 1);
        set_reg(c, S2, done);
        let t2 = reg(c, T2);
        set_reg(c, T2, snd::addiu(t2, 2));
        let t1 = reg(c, T1);
        set_reg(c, T1, snd::addiu(t1, 2));
        let more = snd::slt(done, count);
        set_reg(c, V0, more);
        let slot = reg(c, A2);
        set_reg(c, A2, snd::addiu(slot, 2));
        if more == 0 {
            return snd_send_exit(c, frame, 2);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2858);
        }
    }
}

/// IOP DSP voices: one send byte per voice.
fn snd_send_iop(c: &mut Cpu, frame: Frame, ch: u64, level_off: u64) -> Flow {
    c.cycles += 15;
    let (product, count) = snd_send_level(c, ch, level_off, SEND_SCALE_IOP);
    let scaled = product * SEND_SCALE_IOP;
    abi::set_fpr(c, 1, scaled);
    let bits = snd::cvt_w(scaled);
    c.f[0] = bits;
    set_reg(c, A2, sx(bits));
    if count == 0 {
        return snd_send_exit(c, frame, 1);
    }
    c.cycles += 5;
    set_reg(c, V0, snd::STATE_HI);
    let state_ptr = reg(c, A3);
    set_reg(c, T0, state_ptr);
    set_reg(c, A3, sx(snd::SYNC));
    set_reg(c, A1, snd::addiu(ch, snd::chan::VOICES as i32));
    loop {
        // Loop head 0x3c2970 (a1 the voice slot, s2 the voices done).
        c.cycles += 13;
        let slot = reg(c, A1);
        let voice = snd::at(slot, 0).lh(c);
        let done = snd::addiu(reg(c, S2), 1);
        set_reg(c, S2, done);
        let base = snd::at(reg(c, T0), snd::state::VOICE_BASE).lbu(c);
        set_reg(c, A0, base);
        let params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
        let off = addu(reg(c, S4), snd::sll(snd::subu(voice, base), 4));
        set_reg(c, V0, off);
        let dst = addu(params, off);
        set_reg(c, V1, dst);
        let level = reg(c, A2);
        snd::at(dst, IOP_SEND_BYTES).sb(c, level);
        let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
        let more = snd::slt(done, count);
        set_reg(c, V0, more);
        set_reg(c, A1, snd::addiu(slot, 2));
        if more == 0 {
            return snd_send_exit(c, frame, 2);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2970);
        }
    }
}

/// EE software voices: `SND_EeVoiceSetAuxSend(voice, send, level)` per
/// voice.
fn snd_send_ee(c: &mut Cpu, frame: Frame, ch: u64, level_off: u64) -> Flow {
    c.cycles += 13;
    let (product, count) = snd_send_level(c, ch, level_off, SEND_DIVISOR_EE);
    abi::set_fpr(c, 20, product.div(SEND_DIVISOR_EE));
    if count == 0 {
        return snd_send_exit(c, frame, 1);
    }
    c.cycles += 4;
    let state_ptr = reg(c, A3);
    set_reg(c, S3, state_ptr);
    set_reg(c, S0, snd::addiu(ch, snd::chan::VOICES as i32));
    let ee_base = snd::at(state_ptr, snd::state::EE_VOICE_BASE).lbu(c);
    set_reg(c, V0, ee_base);
    loop {
        // Loop head 0x3c29f0 (s0 the voice slot, s2 the voices done, v0 the
        // EE voice base).
        c.cycles += 13;
        let send = reg(c, S4);
        set_reg(c, A1, send);
        let base = snd::at(reg(c, S3), snd::state::VOICE_BASE).lbu(c);
        c.f[12] = c.f[20];
        let slot = reg(c, S0);
        let voice = snd::at(slot, 0).lh(c);
        set_reg(c, V1, voice);
        let done = snd::addiu(reg(c, S2), 1);
        set_reg(c, S2, done);
        let first = addu(base, reg(c, V0));
        set_reg(c, S0, snd::addiu(slot, 2));
        set_reg(c, A0, snd::subu(voice, first));
        call_ptr(c, EE_VOICE_SET_AUX_SEND, 0x003c_2a14)?;
        let count = snd::at(reg(c, S1), snd::chan::VOICE_COUNT).lbu(c);
        let more = snd::slt(reg(c, S2), count);
        set_reg(c, V0, more);
        if more == 0 {
            return snd_send_exit(c, frame, 1);
        }
        let ee_base = snd::at(reg(c, S3), snd::state::EE_VOICE_BASE).lbu(c);
        set_reg(c, V0, ee_base);
        if event_due(c) {
            deliver_event_here(c, 0x003c_29f0);
        }
    }
}

/// `lq s0` (its block costs `cycles`), then the epilogue: return 0.
fn snd_send_exit(c: &mut Cpu, frame: Frame, cycles: u64) -> Flow {
    c.cycles += cycles;
    frame.restore(c, S0, 0x50);
    c.cycles += 9;
    set_reg(c, V0, 0);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x60);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// World_GetPeakNumber (0x2d1ba0).

/// `BELibrary* BELibrary_GetInstance()`.
const WORLD_BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

/// `void* BELibrary_GetInterface(BELibrary*, int)`.
const WORLD_BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

/// `int BENewRaceInt_GetTrackPeak(BENewRaceInt*)`.
const WORLD_BE_NEW_RACE_GET_TRACK_PEAK: u32 = 0x0014_4c98;

/// `int World_GetPeakNumber()` (0x2d1ba0): the current track's peak,
/// counted from 1:
/// `BENewRaceInt_GetTrackPeak(BELibrary_GetInterface(BELibrary_GetInstance(), 0)) + 1`.
/// Frame 0x10 (ra +0).
pub fn world_get_peak_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, WORLD_BE_LIBRARY_GET_INSTANCE, 0x002d_1bb0)?;
        set_reg(c, A1, 0);
        let library = reg(c, V0);
        set_reg(c, A0, library);
        call_ptr(c, WORLD_BE_LIBRARY_GET_INTERFACE, 0x002d_1bbc)?;
        let race = reg(c, V0);
        set_reg(c, A0, race);
        call_ptr(c, WORLD_BE_NEW_RACE_GET_TRACK_PEAK, 0x002d_1bc4)?;
        frame.restore_ra(c, 0);
        let peak = reg(c, V0);
        set_reg(c, V0, addu(peak, 1));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CollHull_Transform (0x32f840).

/// `CollHull` field: the source sphere tree (`sSphereTreeNode*`).
const HULL_SOURCE_TREE: u32 = 0x60;

/// `CollHull` field: the transformed sphere tree (inline).
const HULL_TREE: u32 = 0x70;

/// `CollHull` field: "transformed" flag.
const HULL_TRANSFORMED: u32 = 0x4;

/// `RiderSphereTree_Assign(dst, src)`.
const HULL_SPHERE_TREE_ASSIGN: u32 = 0x0032_9a28;

/// `RiderSphereTree_Scale(tree, matrix, scale)`.
const HULL_SPHERE_TREE_SCALE: u32 = 0x0032_9b90;

/// `void CollHull_Transform(CollHull* hull, sSphereMtx* m, float scale)`
/// (0x32f840): copy the hull's source sphere tree into its own tree,
/// point the source at the copy, transform the copy by `m` and `scale`,
/// and mark the hull transformed.
///
/// Frame 0x50 (s0 +0x30, s1 +0x20, s2 +0x10, ra +0, f20 +0x40).
pub fn coll_hull_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let hull = reg(c, A0);
        set_reg(c, S1, hull);
        frame.save_ra(c, 0);
        let matrix = reg(c, A1);
        set_reg(c, S2, matrix);
        frame.save_fpr(c, 20, 0x40);
        let tree = addu(hull, HULL_TREE as u64);
        set_reg(c, S0, tree);
        c.f[20] = c.f[12];
        set_reg(c, A0, tree);
        let source = Addr(hull as u32).off(HULL_SOURCE_TREE).lw(c);
        set_reg(c, A1, source);
        call_ptr(c, HULL_SPHERE_TREE_ASSIGN, 0x0032_f874)?;
        let hull = reg(c, S1);
        let tree = reg(c, S0);
        Addr(hull as u32).off(HULL_SOURCE_TREE).sw(c, tree);
        set_reg(c, A0, tree);
        let matrix = reg(c, S2);
        set_reg(c, A1, matrix);
        c.f[12] = c.f[20];
        call_ptr(c, HULL_SPHERE_TREE_SCALE, 0x0032_f888)?;
        set_reg(c, V0, 1);
        frame.restore(c, S0, 0x30);
        let hull = reg(c, S1);
        Addr(hull as u32).off(HULL_TRANSFORMED).sw(c, 1);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_HandleChunk (0x3b7080).

/// `SndStream*[]`: the open streams by slot.
const SNDSTRM_STREAMS: u32 = 0x0050_a0a0;

/// Chunk tag `SCDl` (sample data).
const SNDSTRM_TAG_DATA: u64 = 0x6c44_4353;

/// Chunk tag `SCHl` (stream header).
const SNDSTRM_TAG_HEADER: u64 = 0x6c48_4353;

/// `SNDSTRM_ParseData(stream, chunk)`.
const SNDSTRM_PARSE_DATA: Callee = Callee { addr: 0x003b_6ea8, f: ee::s13::SNDSTRM_ParseData };

/// `SNDSTRM_ParseHeader(slot, chunk)`.
const SNDSTRM_PARSE_HEADER: Callee = Callee { addr: 0x003b_6bd0, f: ee::s13::SNDSTRM_ParseHeader };

/// `StreamFile_ReleaseChunk(file)`.
const SNDSTRM_RELEASE_CHUNK: u32 = 0x003e_13e8;

/// `int SNDSTRM_HandleChunk(int slot, Chunk* chunk)` (0x3b7080): dispatch
/// a chunk read for stream `slot` by its tag: `SCDl` data goes to
/// `SNDSTRM_ParseData(stream, chunk)` (returns 1), an `SCHl` header to
/// `SNDSTRM_ParseHeader(slot, chunk)` (returns 0), anything else is
/// released back to the stream's file (returns 1). Frame 0x10 (ra +0).
pub fn sndstrm_handle_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let slot = reg(c, A0);
        set_reg(c, A3, slot);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let entry = sx(((slot as u32) << 2).wrapping_add(SNDSTRM_STREAMS));
        set_reg(c, V1, entry);
        set_reg(c, V0, SNDSTRM_TAG_DATA);
        let tag = Addr(r32(c, A1)).lw(c);
        set_reg(c, A2, tag);
        let stream = Addr(entry as u32).lw(c);
        set_reg(c, A0, stream);
        if tag == SNDSTRM_TAG_DATA {
            c.cycles += 4;
            SNDSTRM_PARSE_DATA.call(c, 0x003b_70b8)?;
            set_reg(c, V0, 1);
        } else {
            c.cycles += 4;
            set_reg(c, V0, SNDSTRM_TAG_HEADER);
            if tag == SNDSTRM_TAG_HEADER {
                c.cycles += 4;
                let slot = reg(c, A3);
                set_reg(c, A0, slot);
                SNDSTRM_PARSE_HEADER.call(c, 0x003b_70d8)?;
                set_reg(c, V0, 0);
            } else {
                c.cycles += 3;
                let file = Addr(r32(c, A0)).lw(c);
                set_reg(c, A0, file);
                call_ptr(c, SNDSTRM_RELEASE_CHUNK, 0x003b_70e8)?;
                set_reg(c, V0, 1);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFlagManager_GroupSave (0x34c2e0).

/// `cFlagGroup` field: `int` item count.
const FLAG_GROUP_COUNT: u32 = 0x5c;

/// `cFlagGroup` field: `cFlag*[]` items.
const FLAG_GROUP_ITEMS: u32 = 0x94;

/// Stream vtable slot: `Write(this, const void* data, int size)`.
const FLAG_STREAM_WRITE: u32 = 0x8;

/// Stream vtable slot: `WriteObject(this, void* object)`.
const FLAG_STREAM_WRITE_OBJECT: u32 = 0x28;

/// `void cFlagManager_GroupSave(cFlagGroup* group, Stream* stream)`
/// (0x34c2e0): write the group's 0x60-byte header to the stream, then each
/// of its items (`count` at +0x5c, pointers from +0x94) through the
/// stream's object writer.
///
/// Frame 0x50 (s0 +0x40, s1 +0x30, s2 +0x20, s3 +0x10, ra +0).
pub fn cflag_manager_group_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        set_reg(c, A2, 0x60);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        set_reg(c, S1, 0);
        frame.save(c, S3, 0x10);
        let stream = reg(c, A1);
        set_reg(c, S2, stream);
        frame.save(c, S0, 0x40);
        let group = reg(c, A0);
        set_reg(c, S3, group);
        frame.save_ra(c, 0);
        set_reg(c, A1, group);
        let vt = Addr(stream as u32).lw(c);
        set_reg(c, V0, vt);
        let write = VSlot::read(c, vt, FLAG_STREAM_WRITE);
        set_reg(c, A0, write.delta);
        set_reg(c, V1, write.func);
        set_reg(c, A0, write.this(stream));
        call_ptr(c, write.func as u32, 0x0034_c320)?;
        let group = reg(c, S3);
        let count = Addr(group as u32).off(FLAG_GROUP_COUNT).lw(c);
        set_reg(c, V0, count);
        set_reg(c, S0, addu(group, FLAG_GROUP_ITEMS as u64));
        if (count as i64) > 0 {
            c.cycles += 1;
            let vt = Addr(r32(c, S2)).lw(c);
            set_reg(c, V0, vt);
            loop {
                // Loop head 0x34c330 (s0 the item pointer, s1 the items done,
                // v0 the stream's vtable).
                c.cycles += 11;
                let done = addu(reg(c, S1), 1);
                set_reg(c, S1, done);
                let item_at = reg(c, S0);
                let item = Addr(item_at as u32).lw(c);
                set_reg(c, A1, item);
                let vt = reg(c, V0);
                let write = VSlot::read(c, vt, FLAG_STREAM_WRITE_OBJECT);
                set_reg(c, S0, addu(item_at, 4));
                set_reg(c, V1, write.func);
                let stream = reg(c, S2);
                set_reg(c, A0, write.this(stream));
                call_ptr(c, write.func as u32, 0x0034_c34c)?;
                let count = Addr(r32(c, S3)).off(FLAG_GROUP_COUNT).lw(c);
                let more = ((reg(c, S1) as i64) < (count as i64)) as u64;
                set_reg(c, V0, more);
                if more == 0 {
                    break;
                }
                let vt = Addr(r32(c, S2)).lw(c);
                set_reg(c, V0, vt);
                if event_due(c) {
                    deliver_event_here(c, 0x0034_c330);
                }
            }
        }
        c.cycles += 7;
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
// Hide_Save (0x350f40).

/// `WorldObj_Save(self, stream)`.
const HIDE_WORLD_OBJ_SAVE: Callee = Callee { addr: 0x0034_fe90, f: ee::s11::WorldObj_Save };

/// `void Hide_Save(Hide* self, Stream* stream)` (0x350f40): a hide node
/// saves only its world-object part: `WorldObj_Save(self, stream)`.
/// Frame 0x10 (ra +0).
pub fn hide_save(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, crate::call::Target::Direct(HIDE_WORLD_OBJ_SAVE), 0x0035_0f50);
}

// ---------------------------------------------------------------------------
// SND_TimerServiceCallback (0x3b5450).

/// `SND_RunServiceCallbacks()`.
const SND_TIMER_RUN_SERVICE_CALLBACKS: Callee =
    Callee { addr: 0x003c_60e0, f: ee::s13::SND_RunServiceCallbacks };

/// `int SND_TimerServiceCallback()` (0x3b5450): the sound timer's service
/// tick: run the registered service callbacks, return 0. Frame 0x10 (ra +0).
pub fn snd_timer_service_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        SND_TIMER_RUN_SERVICE_CALLBACKS.call(c, 0x003b_5460)?;
        frame.restore_ra(c, 0);
        set_reg(c, V0, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ParticleModifier_Save (0x345bf0).

/// `ParticleModifier_BaseSave(self, stream)`.
const PMOD_BASE_SAVE: Callee = Callee { addr: 0x0034_5890, f: ee::s11::ParticleModifier_BaseSave };

/// `Emitter_Save(emitter, stream)`.
const PMOD_EMITTER_SAVE: u32 = 0x0037_0aa8;

/// Stream vtable slot: `Write(this, const void* data, int size)`.
const PMOD_STREAM_WRITE: u32 = 0x8;

/// `ParticleModifier` field: the particle template (`void*`, 0xd8 bytes).
const PMOD_TEMPLATE: u32 = 0x1e0;

/// `ParticleModifier` field: `int` "uses a shared template" (nothing more
/// is saved when set).
const PMOD_SHARED: u32 = 0x1e4;

/// `ParticleModifier` field: a 0x40-byte block (the transform).
const PMOD_TRANSFORM: u32 = 0x10;

/// `ParticleModifier` field: the emitter (inline).
const PMOD_EMITTER: u32 = 0x50;

/// `void ParticleModifier_Save(ParticleModifier* self, Stream* stream)`
/// (0x345bf0): save the base part, then the "shared" flag (4 bytes); a
/// modifier with its own template also writes the template (0xd8 bytes),
/// its 0x40-byte transform and its emitter.
///
/// Frame 0x30 (s0 +0x20, s1 +0x10, ra +0).
pub fn particle_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let stream = reg(c, A1);
        set_reg(c, S0, stream);
        frame.save_ra(c, 0);
        let this = reg(c, A0);
        set_reg(c, S1, this);
        PMOD_BASE_SAVE.call(c, 0x0034_5c0c)?;
        pmod_stream_write(c, PMOD_SHARED, false, 4, 0x0034_5c28)?;
        let shared = Addr(r32(c, S1)).off(PMOD_SHARED).lw(c);
        set_reg(c, V0, shared);
        if shared == 0 {
            c.cycles += 18;
            pmod_stream_write(c, PMOD_TEMPLATE, true, 0xd8, 0x0034_5c50)?;
            pmod_stream_write(c, PMOD_TRANSFORM, false, 0x40, 0x0034_5c6c)?;
            let emitter = addu(reg(c, S1), PMOD_EMITTER as u64);
            set_reg(c, A0, emitter);
            let stream = reg(c, S0);
            set_reg(c, A1, stream);
            call_ptr(c, PMOD_EMITTER_SAVE, 0x0034_5c78)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `stream->Write(self + field, size)` (or, with `deref`, the pointer
/// stored at `self + field`), with the original's register use: the
/// first write leaves the vtable in v0 and the function in v1, the later
/// ones the other way round.
fn pmod_stream_write(c: &mut Cpu, field: u32, deref: bool, size: u64, ret: u32) -> Flow {
    let stream = reg(c, S0);
    let this = reg(c, S1);
    let vt = Addr(stream as u32).lw(c);
    let data = if deref {
        Addr(this as u32).off(field).lw(c)
    } else {
        addu(this, field as u64)
    };
    set_reg(c, A1, data);
    set_reg(c, A2, size);
    let write = VSlot::read(c, vt, PMOD_STREAM_WRITE);
    if field == PMOD_SHARED {
        set_reg(c, V0, vt);
        set_reg(c, V1, write.func);
    } else {
        set_reg(c, V1, vt);
        set_reg(c, V0, write.func);
    }
    set_reg(c, A0, write.this(stream));
    call_ptr(c, write.func as u32, ret)
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyFxEnable (0x3c2ec8).

/// `SndChannel` field: `s8` effect enable (the SPU path tests it, as `lb`).
const FX_ENABLE: u32 = 0x5f;

/// Offset of the SPU effect-enable mask (`u64`) in the voice parameter
/// block.
const FX_SPU_MASK: u32 = 0x8;

/// `SND_ChannelApplyVolume(channel)`.
const FX_APPLY_VOLUME: Callee = Callee { addr: 0x003c_2a50, f: ee::s13::SND_ChannelApplyVolume };

/// `void SND_ChannelApplyFxEnable(int channel)` (0x3c2ec8): push the
/// channel's effect-enable switch to its voices. SPU voices get their bit
/// (and bit + 0x18 in output mode 2) set or cleared in the effect mask
/// (`u64` at params + 8); IOP and EE voices take it through their levels:
/// `SND_ChannelApplyVolume(channel)`.
///
/// Frame 0x10 (ra +0).
pub fn snd_channel_apply_fx_enable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let index = reg(c, A0);
        set_reg(c, A1, index);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        set_reg(c, V0, snd::STATE_HI);
        let offset = abi::mult(c, index as u32, snd::chan::SIZE);
        set_reg(c, A0, sx(offset));
        set_reg(c, A3, sx(snd::STATE));
        let channels = Addr(snd::STATE).off(snd::state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset));
        set_reg(c, A2, ch);
        let flags = snd::at(ch, snd::chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & snd::chan::IOP);
        if flags & snd::chan::SPU != 0 {
            c.cycles += 3;
            let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
            set_reg(c, V0, count);
            set_reg(c, T2, 0);
            if count == 0 {
                c.cycles += 1;
            } else {
                fx_enable_spu(c, ch);
                c.cycles += 2;
            }
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            let index = reg(c, A1);
            set_reg(c, A0, index);
            if flags & snd::chan::IOP != 0 {
                c.cycles += 4;
                FX_APPLY_VOLUME.call(c, 0x003c_2ff4)?;
            } else {
                c.cycles += 2;
                FX_APPLY_VOLUME.call(c, 0x003c_3004)?;
                c.cycles += 1;
            }
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// The SPU voices' mask loop (0x3c2f28; a3 the voice slot, t2 the voices
/// done, t3/t4 the slot offset).
fn fx_enable_spu(c: &mut Cpu, ch: u64) {
    c.cycles += 8;
    set_reg(c, V0, snd::STATE_HI);
    let state_ptr = reg(c, A3);
    set_reg(c, T6, state_ptr);
    set_reg(c, T1, sx(snd::SYNC));
    set_reg(c, T0, 1);
    set_reg(c, T5, 2);
    set_reg(c, T4, 0);
    set_reg(c, T3, 0);
    set_reg(c, A3, snd::addiu(ch, snd::chan::VOICES as i32));
    loop {
        c.cycles += 3 + 6;
        let on = snd::at(ch, FX_ENABLE).lb(c);
        let slots = snd::addiu(ch, snd::chan::VOICES as i32);
        set_reg(c, A1, slots);
        let params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
        set_reg(c, V0, params);
        let voice = snd::at(reg(c, A3), 0).lh(c);
        let mask = snd::at(params, FX_SPU_MASK).ld(c);
        let bit = 1u64 << (voice & 63);
        let (v1, new_mask) = if on != 0 { (bit, mask | bit) } else { (!bit, mask & !bit) };
        set_reg(c, V1, v1);
        set_reg(c, A0, new_mask);
        c.cycles += 4;
        snd::at(params, FX_SPU_MASK).sd(c, new_mask);
        let mode = snd::at(reg(c, T6), snd::state::OUTPUT_MODE).lbu(c);
        set_reg(c, V0, mode);
        if mode == 2 {
            // The second voice block (bits + 0x18).
            c.cycles += 3;
            let on = snd::at(ch, FX_ENABLE).lb(c);
            set_reg(c, V0, on);
            let slot_off = reg(c, T3);
            set_reg(c, A0, addu(slots, slot_off));
            let (params, v0, v1);
            if on != 0 {
                c.cycles += 7;
                params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let voice = snd::at(reg(c, A0), 0).lh(c);
                let mask = snd::at(params, FX_SPU_MASK).ld(c);
                v0 = 1u64 << (snd::addiu(voice, 0x18) & 63);
                v1 = mask | v0;
            } else {
                c.cycles += 8;
                let slot_off = reg(c, T4);
                set_reg(c, A0, addu(slots, slot_off));
                let voice = snd::at(reg(c, A0), 0).lh(c);
                params = Addr(snd::SYNC).off(snd::sync::VOICE_PARAMS).lw(c);
                let mask = snd::at(params, FX_SPU_MASK).ld(c);
                v0 = !(1u64 << (snd::addiu(voice, 0x18) & 63));
                v1 = mask & v0;
            }
            set_reg(c, A1, params);
            set_reg(c, V0, v0);
            set_reg(c, V1, v1);
            c.cycles += 2;
            snd::at(params, FX_SPU_MASK).sd(c, v1);
        }
        let count = snd::at(ch, snd::chan::VOICE_COUNT).lbu(c);
        c.cycles += 6;
        let done = snd::addiu(reg(c, T2), 1);
        set_reg(c, T2, done);
        let t4 = reg(c, T4);
        set_reg(c, T4, snd::addiu(t4, 2));
        let t3 = reg(c, T3);
        set_reg(c, T3, snd::addiu(t3, 2));
        let more = snd::slt(done, count);
        set_reg(c, V0, more);
        let slot = reg(c, A3);
        set_reg(c, A3, snd::addiu(slot, 2));
        if more == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2f28);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};
    use crate::game::lib::snd::tests as snd_tests;

    /// `SND_ChannelApplySend` over SPU channels (output mode 2 or not, send
    /// level 0 set or clear), IOP and EE channels, 0..4 voices.
    #[test]
    fn snd_channel_apply_send_matches() {
        check(ee::s13::f_003c27e0__generated, snd_channel_apply_send, 240, &|c, seed| {
            let mut r = Rng(0x5e4d_0001 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let flags = snd_tests::channel_flags(&mut r, seed);
            snd_tests::setup_channel(c, &mut r, seed, flags);
            let ch = DATA + (c.g[A0][0] as u32) * snd::chan::SIZE;
            for i in 0..4 {
                let level = if (seed / 5 + i) % 3 == 0 { 0 } else { r.next_u32() as u16 };
                c.bus.write16(ch + SEND_LEVELS + 2 * i, level);
            }
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.pc = 0x003c_27e0;
        });
    }

    /// `World_GetPeakNumber` through the real BE library (a dummy singleton
    /// at gp-0x1f04, interface 0 resolving to a fake) and the real
    /// `BENewRaceInt_GetTrackPeak`; the fake call may unwind.
    #[test]
    fn world_get_peak_number_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::GAME_GP;
        check(ee::s09::f_002d1ba0__generated, world_get_peak_number, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2d_1ba0, 1);
            // A created BE library singleton, interface index 0.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
            c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
            c.bus.write32(0x0053_08a8, 0);
            // The current track (BENewRaceInt) and its peak in the table.
            let track = seed % 8;
            c.bus.write32(0x0053_5c08, track);
            c.bus.write32(0x0043_d950 + track * 0x64 + 0x54, r.next_u32() % 5);
            c.pc = 0x002d_1ba0;
        });
    }

    /// `CollHull_Transform` with the real `RiderSphereTree_Assign` (and its
    /// memcpy) and `RiderSphereTree_Scale`: source trees of 0..5 child
    /// spheres, random matrices and scales.
    #[test]
    fn coll_hull_transform_matches() {
        use crate::difftest::fill;
        check(ee::s11::f_0032f840__generated, coll_hull_transform, 200, &|c, seed| {
            let mut r = Rng(0x0032_f840 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let hull = DATA;
            let matrix = DATA + 0x1000;
            let source = DATA + 0x2000;
            c.g[A0][0] = sx(hull);
            c.g[A1][0] = sx(matrix);
            c.f[12] = if seed % 7 == 0 { r.edge_f32() } else { r.f32(4.0) };
            for i in 0..16 {
                c.bus.write32(matrix + 4 * i, r.f32(100.0));
            }
            fill(c, &mut r, hull, 0x70 + 0x30 + 6 * 0x20);
            c.bus.write32(hull + HULL_SOURCE_TREE, source);
            fill(c, &mut r, source, 0x30);
            // Centre, radius and children: floats; the child count small.
            for i in 0..4 + 6 * 8 {
                let a = source + 0x10 + 4 * i;
                if a != source + 0x2c {
                    c.bus.write32(a, r.f32(500.0));
                }
            }
            c.bus.write32(source + 0x2c, seed % 6);
            c.pc = 0x0032_f840;
        });
    }

    /// `SNDSTRM_HandleChunk` with `SCDl` chunks (the real
    /// `SNDSTRM_ParseData`, fixture as in its own test: empty chunks
    /// released through a null file handle, others pushed through the real
    /// `SNDSTRM_PushBlock`) and other tags (the real
    /// `StreamFile_ReleaseChunk`, null file handle). The `SCHl` branch (the
    /// real `SNDSTRM_ParseHeader`, a large parser) is not reached.
    #[test]
    fn sndstrm_handle_chunk_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::fill;
        use crate::game::lib::snd::{
            SS_CHANNELS, SS_CHUNK_OFFSETS, SS_CHUNK_SAMPLES, SS_PUSHED, SS_STATE, SS_SWAP,
        };
        const STREAM: u32 = DATA;
        const STATE: u32 = DATA + 0x200;
        const CHUNK: u32 = DATA + 0x400;
        const BLOCKS: u32 = DATA + 0x800;
        check(ee::s13::f_003b7080__generated, sndstrm_handle_chunk, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_7080, 0);
            c.pc = 0x003b_7080;
            let slot = r.next_u32() % 4;
            for i in 0..4 {
                c.bus.write32(SNDSTRM_STREAMS + 4 * i, if i == slot { STREAM } else { r.next_u32() });
            }
            let swap = seed % 2 == 1;
            let bytes = |v: u32| if swap { v.swap_bytes() } else { v };
            let count = match seed % 7 {
                0 => 0,
                1 => 0x8000_0000,
                _ => 1 + r.next_u32() % 0x4000 | (r.next_u32() & 0x8000_0000),
            };
            let channels = if count & 0x7fff_ffff == 0 && seed % 3 == 0 {
                0
            } else {
                1 + r.next_u32() % 4
            };
            let idx = r.next_u32() % 4;
            fill(c, &mut r, STREAM, 0x10);
            c.bus.write32(STREAM, 0);
            c.bus.write32(STREAM + 0x8, idx);
            c.bus.write8(STREAM + SS_CHANNELS, channels as u8);
            c.bus.write32(STREAM + SS_STATE, STATE);
            fill(c, &mut r, STATE, 0x10);
            c.bus.write8(STATE + SS_SWAP, swap as u8);
            c.bus.write8(STATE + SS_PUSHED, (r.next_u32() % 2) as u8);
            fill(c, &mut r, CHUNK, 0x10);
            let tag = match seed % 3 {
                0 => r.next_u32() & !0x0400_0000,
                _ => SNDSTRM_TAG_DATA as u32,
            };
            c.bus.write32(CHUNK, tag);
            c.bus.write32(CHUNK + SS_CHUNK_SAMPLES, bytes(count));
            for i in 0..channels {
                let off = 4 + 0x40 * i + (r.next_u32() % 0x10) * 4;
                c.bus.write32(CHUNK + SS_CHUNK_OFFSETS + 4 * i, bytes(off));
            }
            // SNDSTRM_PushBlock's stream table and stream (no queue).
            for i in 0..4 {
                let b = BLOCKS + 0x100 * i;
                c.bus.write32(0x0051_57b8 + 0x304 + 4 * i, b);
                fill(c, &mut r, b, 0x40);
                c.bus.write8(b + 0x62, 0);
                c.bus.write16(b + 0x46, (r.next_u32() % 4) as u16);
                c.bus.write16(b + 0x40, 1 + (r.next_u32() % 4) as u16);
            }
            c.g[A0][0] = slot as u64;
            c.g[A1][0] = sx(CHUNK);
        });
    }

    /// `cFlagManager_GroupSave` with a stream whose two writers are fakes
    /// (some calls unwind), groups of a negative, zero or 1..5 items.
    #[test]
    fn cflag_manager_group_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::fill;
        covered(ee::s11::f_0034c2e0__generated, cflag_manager_group_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_c2e0, 6);
            c.pc = 0x0034_c2e0;
            let group = DATA;
            let stream = DATA + 0x400;
            let vt = vtable(c, &mut r, &[FLAG_STREAM_WRITE, FLAG_STREAM_WRITE_OBJECT]);
            c.bus.write32(stream, vt);
            let count = match seed % 6 {
                0 => 0,
                1 => (-(1 + (r.next_u32() % 4) as i32)) as u32,
                _ => 1 + r.next_u32() % 5,
            };
            c.bus.write32(group + FLAG_GROUP_COUNT, count);
            fill(c, &mut r, group + FLAG_GROUP_ITEMS, 6);
            c.g[A0][0] = sx(group);
            c.g[A1][0] = sx(stream);
        });
    }

    /// `Hide_Save` through the real `WorldObj_Save` (and
    /// `SyncProcess_Save`), the stream's writers fakes (some unwind).
    #[test]
    fn hide_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(ee::s11::f_00350f40__generated, hide_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x35_0f40, 3);
            c.pc = 0x0035_0f40;
            let this = DATA + 0x100;
            let stream = DATA + 0x400;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            let vt = vtable(c, &mut r, &[8, 0x30]);
            c.bus.write32(stream, vt);
        });
    }

    /// `SND_TimerServiceCallback` through the real `SND_RunServiceCallbacks`:
    /// service disabled, a zero or negative count, or 1..5 fake callbacks
    /// (each may unwind).
    #[test]
    fn snd_timer_service_callback_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        use crate::game::lib::snd::{SND_GLOBALS_2, SND_SERVICE_COUNT, SND_SERVICE_FNS, SND_SERVICE_ON};
        covered(ee::s13::f_003b5450__generated, snd_timer_service_callback, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_5450, 5);
            c.pc = 0x003b_5450;
            let on = if seed % 5 == 0 { 0 } else { 1 + r.next_u32() % 0x7f };
            c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_ON, on as u8);
            let count: i8 = match seed % 7 {
                0 => 0,
                1 => -3,
                _ => (1 + r.next_u32() % 5) as i8,
            };
            c.bus.write8(SND_GLOBALS_2 + SND_SERVICE_COUNT, count as u8);
            for i in 0..6 {
                c.bus.write32(SND_GLOBALS_2 + SND_SERVICE_FNS + 4 * i, FAKE + 0x10 * i);
            }
        });
    }

    /// `ParticleModifier_Save`: the real `ParticleModifier_BaseSave` and
    /// `Emitter_Save`, the stream's writers fakes (some calls unwind);
    /// shared-template flag set or clear.
    #[test]
    fn particle_modifier_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::fill;
        covered(ee::s11::f_00345bf0__generated, particle_modifier_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_5bf0, 5);
            c.pc = 0x0034_5bf0;
            let this = DATA;
            let stream = DATA + 0x400;
            fill(c, &mut r, this, 0x80);
            c.bus.write32(this + PMOD_SHARED, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            let vt = vtable(c, &mut r, &[PMOD_STREAM_WRITE, 0x28]);
            c.bus.write32(stream, vt);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(stream);
        });
    }

    /// `SND_ChannelApplyFxEnable` over SPU channels (output mode 2 or not,
    /// the switch on or off, 0..4 voices), and IOP and EE channels (the
    /// real `SND_ChannelApplyVolume`, its EE voices through the real
    /// `SND_EeVoiceSetVolume`).
    #[test]
    fn snd_channel_apply_fx_enable_matches() {
        check(ee::s13::f_003c2ec8__generated, snd_channel_apply_fx_enable, 240, &|c, seed| {
            let mut r = Rng(0x5e4d_0002 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let flags = snd_tests::channel_flags(&mut r, seed);
            snd_tests::setup_channel(c, &mut r, seed, flags);
            let ch = DATA + (c.g[A0][0] as u32) * snd::chan::SIZE;
            let on = if (seed / 5) % 2 == 0 { 0 } else { r.next_u32() as u8 | 1 };
            c.bus.write8(ch + FX_ENABLE, on);
            c.pc = 0x003c_2ec8;
        });
    }
}
