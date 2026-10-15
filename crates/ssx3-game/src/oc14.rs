//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_1348, replay_save_begin_snapshots, "Replay_SaveBeginSnapshots"),
    (0x0027_14e0, replay_save_begin_caches, "Replay_SaveBeginCaches"),
    (0x0027_22e0, in_seq_heap_init, "InSeq_HeapInit"),
    (0x0027_2570, math_eval_cubic, "Math_EvalCubic"),
    (0x0027_2cc0, in_seq_get_instance, "InSeq_GetInstance"),
    (0x0027_30c0, in_seq_span_track_activate_key, "InSeq_SpanTrackActivateKey"),
    (0x0027_3140, in_seq_span_track_next_key, "InSeq_SpanTrackNextKey"),
    (0x0027_3418, in_seq_event_track_fire_key, "InSeq_EventTrackFireKey"),
    (0x0027_3468, in_seq_event_track_next_key, "InSeq_EventTrackNextKey"),
    (0x0027_3860, in_seq_ramp_track_activate_key, "InSeq_RampTrackActivateKey"),
    (0x0027_38f8, in_seq_ramp_track_next_key, "InSeq_RampTrackNextKey"),
    (0x0027_3938, in_seq_channel_ctor, "InSeq_Channel_ctor"),
    (0x0027_39b8, in_seq_channel_setup, "InSeq_ChannelSetup"),
    (0x0027_42c8, in_seq_channel_get_track, "InSeq_ChannelGetTrack"),
    (0x0027_4348, in_seq_channel_get_length, "InSeq_ChannelGetLength"),
    (0x0027_43c8, in_seq_channel_get_id, "InSeq_ChannelGetId"),
    (0x0027_4518, in_seq_instance_ctor, "InSeq_Instance_ctor"),
    (0x0027_4830, in_seq_instance_start, "InSeq_InstanceStart"),
    (0x0027_49e8, in_seq_instance_pause, "InSeq_InstancePause"),
    (0x0027_4a08, in_seq_instance_resume, "InSeq_InstanceResume"),
];

// ---------------------------------------------------------------------------
// Replay recording.

/// `void Replay_SaveBeginSnapshots(Replay* r)` (0x271348): select the current
/// snapshot (the two pointers at +0x3b0/+0x3d0; when they are equal advance
/// through the node's +0x14 link into +0x65c), bump the record serial at
/// +0x638 and clear the per-record flags.
pub fn replay_save_begin_snapshots(c: &mut Cpu) {
    const FIRST: u32 = 0x3b0;
    const LAST: u32 = 0x3d0;
    const NODE_NEXT: u32 = 0x14;
    const CURRENT: u32 = 0x65c;
    const SERIAL: u32 = 0x638;
    const FLAG_660: u32 = 0x660;
    const FLAG_644: u32 = 0x644;
    const FLAG_648: u32 = 0x648;
    const FLAG_668: u32 = 0x668;
    c.cycles += 4;
    let r = Addr(abi::arg(c, 0));
    let first = r.off(FIRST).lw(c);
    c.g[V1][0] = first;
    let last = r.off(LAST).lw(c);
    c.g[V0][0] = last;
    let differs = first != last;
    r.off(CURRENT).sw(c, first);
    if !differs {
        c.cycles += 2;
        let next = Addr(first as u32).off(NODE_NEXT).lw(c);
        c.g[V0][0] = next;
        r.off(CURRENT).sw(c, next);
    }
    c.cycles += 8;
    let serial = r.off(SERIAL).lw(c);
    c.g[V0][0] = serial;
    r.off(FLAG_668).sw(c, 0);
    c.g[V0][0] = sx((serial as u32).wrapping_add(1));
    r.off(FLAG_660).sw(c, 0);
    r.off(SERIAL).sw(c, c.g[V0][0]);
    r.off(FLAG_644).sw(c, 0);
    r.off(FLAG_648).sw(c, 0);
    abi::ret(c);
}

/// `void Replay_SaveBeginCaches(Replay* r)` (0x2714e0): mark the cache phase
/// (state +0x658 = 2), bump the record serial at +0x638 and clear the cache
/// bookkeeping fields.
pub fn replay_save_begin_caches(c: &mut Cpu) {
    const SERIAL: u32 = 0x638;
    const STATE: u32 = 0x658;
    const CACHE_A: u32 = 0x654;
    const CACHE_B: u32 = 0x674;
    const CACHE_C: u32 = 0x66c;
    const CACHE_D: u32 = 0x670;
    const FLAG_644: u32 = 0x644;
    const FLAG_648: u32 = 0x648;
    c.cycles += 12;
    let r = Addr(abi::arg(c, 0));
    let serial = r.off(SERIAL).lw(c);
    c.g[V0][0] = serial;
    c.g[V1][0] = 0x2;
    r.off(STATE).sw(c, c.g[V1][0]);
    c.g[V0][0] = sx((serial as u32).wrapping_add(1));
    r.off(CACHE_A).sw(c, 0);
    r.off(SERIAL).sw(c, c.g[V0][0]);
    r.off(CACHE_B).sw(c, 0);
    r.off(CACHE_C).sw(c, 0);
    r.off(CACHE_D).sw(c, 0);
    r.off(FLAG_644).sw(c, 0);
    r.off(FLAG_648).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instrument-sequence heap.

/// `void* InSeq_HeapInit(Heap* heap, void* data, uint size)` (0x2722e0):
/// initialise a heap node (head at +0x0, tail +0x8, size +0x4), terminating the
/// block with a free marker at `data + 4` and the end offset.
pub fn in_seq_heap_init(c: &mut Cpu) {
    const TAIL: u32 = 0x8;
    const SIZE: u32 = 0x4;
    const BLOCK_NEXT: u32 = 0x4;
    c.cycles += 9;
    let heap = Addr(abi::arg(c, 0));
    let data = c.g[A1][0];
    heap.off(TAIL).sw(c, data);
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x8));
    heap.sw(c, data);
    c.g[V0][0] = c.g[A0][0];
    heap.off(SIZE).sw(c, c.g[A2][0]);
    Addr(data as u32).off(BLOCK_NEXT).sw(c, 0);
    c.g[V1][0] = heap.off(TAIL).lw(c);
    Addr(c.g[V1][0] as u32).sw(c, c.g[A3][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Cubic evaluation.

/// `float Math_EvalCubic(const float* coeffs, float t)` (0x272570): evaluate
/// `((c0*t + c1)*t + c2)*t + c3` (f12 = t), returning `c3` immediately when
/// `t` compares equal to zero.
pub fn math_eval_cubic(c: &mut Cpu) {
    const C1: u32 = 0x4;
    const C2: u32 = 0x8;
    const C3: u32 = 0xc;
    c.cycles += 4;
    let t = FpuF32(c.f[12]);
    // mtc1 f0, r0 / c.eq.s f12, f0.
    c.f[0] = 0;
    let eq = t.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, eq);
    let coeffs = Addr(abi::arg(c, 0));
    if eq {
        c.cycles += 2;
        let c3 = coeffs.off(C3).read_fpu(c);
        c.f[0] = c3.0;
        abi::ret(c);
        return;
    }
    c.cycles += 11;
    let c0 = coeffs.read_fpu(c);
    let c1 = coeffs.off(C1).read_fpu(c);
    let mut f1 = c0 * t;
    let c2 = coeffs.off(C2).read_fpu(c);
    let c3 = coeffs.off(C3).read_fpu(c);
    let mut f0 = f1 + c1;
    f0 = f0 * t;
    f0 = f0 + c2;
    f1 = f0 * t;
    f0 = f1 + c3;
    c.f[0] = f0.0;
    c.f[1] = f1.0;
    c.f[2] = c1.0;
    c.f[3] = c2.0;
    c.f[4] = c3.0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instrument-sequence instances.

/// `void* InSeq_GetInstance(Channel* ch, int index)` (0x272cc0): the instance
/// pointer `index * 0x58` into the table at `ch + 0x14`, returned as the record
/// plus 8 (null when the record's +0xc field is zero).
pub fn in_seq_get_instance(c: &mut Cpu) {
    const TABLE: u32 = 0x14;
    const RECORD: u32 = 0x58;
    const RECORD_FLAG: u32 = 0xc;
    c.cycles += 8;
    c.g[V1][0] = RECORD as u64;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let scaled = abi::mult(c, abi::arg(c, 1), RECORD);
    c.g[A1][0] = sx(scaled);
    let record = sx((table as u32).wrapping_add(scaled));
    c.g[V0][0] = record;
    let flag = Addr(record as u32).off(RECORD_FLAG).lw(c);
    c.g[V1][0] = flag;
    c.g[V0][0] = sx((record as u32).wrapping_add(0x8));
    if flag == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Span tracks.

/// `void InSeq_SpanTrackActivateKey(SpanTrack* track, SpanKey* key)`
/// (0x2730c0): copy a span key into the track's `index * 0x1c` record at
/// `track + 0x20` and link it into the active list at `track + 0x24`.
pub fn in_seq_span_track_activate_key(c: &mut Cpu) {
    const ACTIVE_LIST: u32 = 0x20;
    const LINK: u32 = 0x24;
    const COUNT: u32 = 0x28;
    const RECORD: u32 = 0x1c;
    const KEY_INDEX: u32 = 0x2;
    const KEY_START: u32 = 0x8;
    const KEY_END: u32 = 0xc;
    const KEY_FLAGS: u32 = 0x4;
    const KEY_KIND: u32 = 0x6;
    c.cycles += 22;
    let key = c.g[A1][0];
    c.g[A2][0] = key;
    let track = c.g[A0][0];
    c.g[A3][0] = track;
    let index = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index;
    c.g[A1][0] = RECORD as u64;
    let base = Addr(track as u32).off(ACTIVE_LIST).lw(c);
    c.g[A0][0] = base;
    let scaled = abi::mult(c, index as u32, RECORD);
    c.g[V0][0] = sx(scaled);
    let start = Addr(key as u32).off(KEY_START).lw(c);
    c.g[V1][0] = start;
    let slot = sx((base as u32).wrapping_add(scaled));
    c.g[A0][0] = slot;
    Addr(slot as u32).sw(c, start);
    let index2 = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index2;
    Addr(slot as u32).off(0x4).sw(c, index2);
    let end = Addr(key as u32).off(KEY_END).lw(c);
    c.g[V1][0] = end;
    Addr(slot as u32).off(0x10).sw(c, end);
    let flags = Addr(key as u32).off(KEY_FLAGS).lhu(c);
    c.g[V0][0] = flags;
    Addr(slot as u32).off(0x8).sw(c, flags);
    let kind = Addr(key as u32).off(KEY_KIND).lhu(c);
    c.g[V1][0] = kind;
    Addr(slot as u32).off(0x14).sw(c, 0);
    Addr(slot as u32).off(0xc).sw(c, kind);
    let kind_field = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind_field;
    let short = (kind_field < 0x11) as u64;
    c.g[V0][0] = short;
    if short != 0 {
        c.g[V1][0] = Addr(track as u32).off(LINK).lw(c);
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x10));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x14).sw(c, payload);
        c.g[V1][0] = Addr(track as u32).off(LINK).lw(c);
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[A0][0];
    Addr(slot as u32).off(0x18).sw(c, c.g[V1][0]);
    Addr(track as u32).off(LINK).sw(c, c.g[A0][0]);
    let count = Addr(track as u32).off(COUNT).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_SpanTrackNextKey(SpanTrack* track)` (0x273140): advance the
/// cursor at `track + 0x18` to the next span (adding the current span's length)
/// or clear the cursor at +0x14 at the end of the list.
pub fn in_seq_span_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Event tracks.

/// `void InSeq_EventTrackFireKey(EventTrack* track, EventKey* key)` (0x273418):
/// store a key into the event array element `track + 0x1c + index*8` and bump
/// the element count at +0x20.
pub fn in_seq_event_track_fire_key(c: &mut Cpu) {
    const ELEMS: u32 = 0x1c;
    const COUNT: u32 = 0x20;
    const KEY_VALUE: u32 = 0x4;
    c.cycles += 12;
    let track = c.g[A0][0];
    c.g[A2][0] = track;
    let key = c.g[A1][0];
    let value = Addr(key as u32).off(KEY_VALUE).lw(c);
    c.g[V1][0] = value;
    let index = Addr(track as u32).off(COUNT).lw(c);
    c.g[V0][0] = index;
    let elems = Addr(track as u32).off(ELEMS).lw(c);
    c.g[A0][0] = elems;
    let delta = sx((index as u32).wrapping_shl(3));
    c.g[V0][0] = delta;
    let slot = sx((elems as u32).wrapping_add(delta as u32));
    c.g[A0][0] = slot;
    Addr(slot as u32).sw(c, value);
    Addr(slot as u32).off(0x4).sw(c, 0);
    let kind = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind;
    let short = (kind < 9) as u64;
    c.g[V0][0] = short;
    let count;
    if short != 0 {
        count = Addr(track as u32).off(COUNT).lw(c);
        c.g[V1][0] = count;
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x8));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x4).sw(c, payload);
        count = Addr(track as u32).off(COUNT).lw(c);
        c.g[V1][0] = count;
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_EventTrackNextKey(EventTrack* track)` (0x273468): same cursor
/// walk as the span track, over the event list.
pub fn in_seq_event_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Ramp tracks.

/// `void InSeq_RampTrackActivateKey(RampTrack* track, RampKey* key)`
/// (0x273860): copy a ramp key (including its converted integer/float fields)
/// into the `index * 0x24` record at `track + 0x1c` and link it into the active
/// list at +0x20.
pub fn in_seq_ramp_track_activate_key(c: &mut Cpu) {
    const ACTIVE_LIST: u32 = 0x1c;
    const LINK: u32 = 0x20;
    const COUNT: u32 = 0x24;
    const RECORD: u32 = 0x24;
    const KEY_INDEX: u32 = 0x2;
    const KEY_START: u32 = 0x8;
    const KEY_INT: u32 = 0xc;
    const KEY_FLOAT: u32 = 0x10;
    const KEY_END: u32 = 0x14;
    const KEY_FLAGS: u32 = 0x4;
    const KEY_KIND: u32 = 0x6;
    c.cycles += 27;
    let key = c.g[A1][0];
    c.g[A3][0] = key;
    let track = c.g[A0][0];
    c.g[T0][0] = track;
    let index = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index;
    c.g[A1][0] = RECORD as u64;
    let base = Addr(track as u32).off(ACTIVE_LIST).lw(c);
    c.g[A0][0] = base;
    let scaled = abi::mult(c, index as u32, RECORD);
    c.g[V0][0] = sx(scaled);
    let start = Addr(key as u32).off(KEY_START).lw(c);
    c.g[V1][0] = start;
    let slot = sx((base as u32).wrapping_add(scaled));
    c.g[A2][0] = slot;
    Addr(slot as u32).sw(c, start);
    let end = Addr(key as u32).off(KEY_END).lw(c);
    c.g[V0][0] = end;
    Addr(slot as u32).off(0x18).sw(c, end);
    let converted = FpuF32::from_i32(Addr(key as u32).off(KEY_INT).read_fpu(c).0 as i32);
    c.f[0] = converted.0;
    Addr(slot as u32).off(0x10).write_fpu(c, converted);
    let rate = Addr(key as u32).off(KEY_FLOAT).read_fpu(c);
    c.f[1] = rate.0;
    Addr(slot as u32).off(0x14).write_fpu(c, rate);
    let index2 = Addr(key as u32).off(KEY_INDEX).lhu(c);
    c.g[V0][0] = index2;
    Addr(slot as u32).off(0xc).sw(c, index2);
    let flags = Addr(key as u32).off(KEY_FLAGS).lhu(c);
    c.g[V1][0] = flags;
    Addr(slot as u32).off(0x4).sw(c, flags);
    let kind = Addr(key as u32).off(KEY_KIND).lhu(c);
    c.g[A0][0] = kind;
    Addr(slot as u32).off(0x1c).sw(c, 0);
    Addr(slot as u32).off(0x8).sw(c, kind);
    let kind_field = Addr(key as u32).lhu(c);
    c.g[V0][0] = kind_field;
    let short = (kind_field < 0x19) as u64;
    c.g[V0][0] = short;
    let link;
    if short != 0 {
        link = Addr(track as u32).off(LINK).lw(c);
        c.g[V1][0] = link;
    } else {
        c.cycles += 3;
        let payload = sx((key as u32).wrapping_add(0x18));
        c.g[V0][0] = payload;
        Addr(slot as u32).off(0x1c).sw(c, payload);
        link = Addr(track as u32).off(LINK).lw(c);
        c.g[V1][0] = link;
    }
    c.cycles += 7;
    c.g[V0][0] = c.g[A2][0];
    Addr(slot as u32).off(0x20).sw(c, c.g[V1][0]);
    Addr(track as u32).off(LINK).sw(c, c.g[A2][0]);
    let count = Addr(track as u32).off(COUNT).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    Addr(track as u32).off(COUNT).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void* InSeq_RampTrackNextKey(RampTrack* track)` (0x2738f8): same cursor
/// walk as the span track, over the ramp list.
pub fn in_seq_ramp_track_next_key(c: &mut Cpu) {
    const KEYS: u32 = 0x10;
    const CURSOR: u32 = 0x14;
    const INDEX: u32 = 0x18;
    c.cycles += 7;
    let track = c.g[A0][0];
    c.g[A1][0] = track;
    let keys = Addr(track as u32).off(KEYS).lw(c);
    c.g[V1][0] = keys;
    let cur = Addr(track as u32).off(INDEX).lw(c);
    c.g[A0][0] = cur;
    let count = Addr(keys as u32).lhu(c);
    c.g[V0][0] = count;
    let last = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = last;
    if cur != last {
        let next = Addr(track as u32).off(CURSOR).lw(c);
        c.g[V0][0] = next;
        c.cycles += 6;
        c.g[V1][0] = sx((cur as u32).wrapping_add(1));
        let len = Addr(next as u32).lhu(c);
        c.g[A0][0] = len;
        Addr(track as u32).off(INDEX).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((next as u32).wrapping_add(len as u32));
        Addr(track as u32).off(CURSOR).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 2;
        Addr(track as u32).off(CURSOR).sw(c, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Channels.

/// `InSeq_Channel* InSeq_Channel_ctor(InSeq_Channel* self)` (0x273938): zero the
/// non-head fields (offsets 4..0x2c) and return the channel.
pub fn in_seq_channel_ctor(c: &mut Cpu) {
    c.cycles += 13;
    let ch = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    ch.off(0x4).sw(c, 0);
    ch.off(0x8).sw(c, 0);
    ch.off(0xc).sw(c, 0);
    ch.off(0x10).sw(c, 0);
    ch.off(0x14).sw(c, 0);
    ch.off(0x18).sw(c, 0);
    ch.off(0x1c).sw(c, 0);
    ch.off(0x20).sw(c, 0);
    ch.off(0x24).sw(c, 0);
    ch.off(0x28).sw(c, 0);
    ch.off(0x2c).sw(c, 0);
    abi::ret(c);
}

/// `void InSeq_ChannelSetup(InSeq_Channel* ch, void* data, void* listener)`
/// (0x2739b8): bind the data block and listener, record the first track at
/// `+0x1c` (+0x20 when the block's kind is short) and return the bound data.
pub fn in_seq_channel_setup(c: &mut Cpu) {
    const DATA: u32 = 0x4;
    const LISTENER: u32 = 0x8;
    const DATA_SAVED: u32 = 0x10;
    const DATA_END: u32 = 0x14;
    const TRACK: u32 = 0x1c;
    const TRACK_LONG: u32 = 0x20;
    const DATA_OFFSET: u32 = 0x8;
    c.cycles += 13;
    let ch = Addr(abi::arg(c, 0));
    let data = c.g[A1][0];
    c.g[V0][0] = sx((data as u32).wrapping_add(DATA_OFFSET));
    ch.off(DATA).sw(c, data);
    ch.off(DATA_SAVED).sw(c, data);
    ch.off(LISTENER).sw(c, c.g[A2][0]);
    ch.off(DATA_END).sw(c, c.g[V0][0]);
    ch.off(0x18).sw(c, 0);
    let delta = Addr(data as u32).off(DATA_OFFSET).lw(c);
    c.g[V0][0] = delta;
    let track = sx((data as u32).wrapping_add(delta as u32));
    c.g[A1][0] = track;
    ch.off(TRACK).sw(c, track);
    let kind = Addr(track as u32).lhu(c);
    c.g[V0][0] = kind;
    let short = (kind < 9) as u64;
    c.g[V0][0] = short;
    if short != 0 {
        ch.off(TRACK_LONG).sw(c, 0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = sx((track as u32).wrapping_add(DATA_OFFSET));
        ch.off(TRACK_LONG).sw(c, c.g[V0][0]);
    }
    c.cycles += 3;
    let saved = ch.off(DATA_SAVED).lw(c);
    c.g[V1][0] = saved;
    c.g[V0][0] = Addr(saved as u32).lw(c);
    abi::ret(c);
}

/// `void* InSeq_ChannelGetTrack(InSeq_Channel* ch, int index)` (0x2742c8): the
/// track pointer `index` in the table at `ch + 0x24`.
pub fn in_seq_channel_get_track(c: &mut Cpu) {
    const TABLE: u32 = 0x24;
    c.cycles += 5;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let offset = sx((c.g[A1][0] as u32).wrapping_shl(2));
    c.g[A1][0] = offset;
    let slot = sx((offset as u32).wrapping_add(table as u32));
    c.g[A1][0] = slot;
    c.g[V0][0] = Addr(slot as u32).lw(c);
    abi::ret(c);
}

/// `int InSeq_ChannelGetLength(InSeq_Channel* ch)` (0x274348): the length field
/// at `channel->track + 0x4`.
pub fn in_seq_channel_get_length(c: &mut Cpu) {
    const TRACK: u32 = 0x1c;
    c.cycles += 3;
    let track = Addr(abi::arg(c, 0)).off(TRACK).lw(c);
    c.g[V1][0] = track;
    c.g[V0][0] = Addr(track as u32).off(0x4).lw(c);
    abi::ret(c);
}

/// `int InSeq_ChannelGetId(InSeq_Channel* ch)` (0x2743c8): the id byte at
/// `channel->track + 0x3`.
pub fn in_seq_channel_get_id(c: &mut Cpu) {
    const TRACK: u32 = 0x1c;
    c.cycles += 3;
    let track = Addr(abi::arg(c, 0)).off(TRACK).lw(c);
    c.g[V1][0] = track;
    c.g[V0][0] = Addr(track as u32).off(0x3).lbu(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Instances.

/// `InSeq_Instance* InSeq_Instance_ctor(InSeq_Instance* self)` (0x274518):
/// initialise the instance (gain 1.0 at +0x24, -1 at +0/+0x30/+0x34, zero
/// elsewhere) and return it.
pub fn in_seq_instance_ctor(c: &mut Cpu) {
    const GAIN: u32 = 0x24;
    const MINUS_ONE_A: u32 = 0x30;
    const MINUS_ONE_B: u32 = 0x34;
    c.cycles += 23;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[AT][0] as u32;
    let minus_one = 0xffff_ffff_ffff_ffffu64;
    c.g[V1][0] = minus_one;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.off(MINUS_ONE_B).sw(c, minus_one);
    this.off(GAIN).write_fpu(c, FpuF32(c.f[0]));
    this.sw(c, minus_one);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    this.off(0x20).sw(c, 0);
    this.off(0x28).sw(c, 0);
    this.off(0x2c).sw(c, 0);
    this.off(MINUS_ONE_A).sw(c, minus_one);
    this.off(0x38).sw(c, 0);
    this.off(0x3c).sw(c, 0);
    this.off(0x40).sw(c, 0);
    this.off(0x44).sw(c, 0);
    abi::ret(c);
}

/// `void InSeq_InstanceStart(InSeq_Instance* inst, void* data)` (0x274830):
/// start playback, recording the current track at +0x1c; if the instance is
/// already running and has a pending start request at +0x3c, clear +0x40.
pub fn in_seq_instance_start(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const TRACK: u32 = 0x1c;
    const PENDING: u32 = 0x3c;
    const REQUEST: u32 = 0x40;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    let one = 0x1u64;
    c.g[V0][0] = one;
    if active == 0 {
        c.cycles += 2;
        inst.off(TRACK).sw(c, c.g[A1][0]);
        inst.off(ACTIVE).sw(c, one);
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let pending = inst.off(PENDING).lw(c);
    c.g[V0][0] = pending;
    if pending != 0 {
        c.cycles += 2;
        inst.off(REQUEST).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void InSeq_InstancePause(InSeq_Instance* inst)` (0x2749e8): while running,
/// increment the pause counter at +0x2c.
pub fn in_seq_instance_pause(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const PAUSE_COUNT: u32 = 0x2c;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    if active != 0 {
        c.cycles += 3;
        let count = inst.off(PAUSE_COUNT).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        inst.off(PAUSE_COUNT).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void InSeq_InstanceResume(InSeq_Instance* inst)` (0x274a08): while running,
/// decrement the pause counter at +0x2c, clamping at zero.
pub fn in_seq_instance_resume(c: &mut Cpu) {
    const ACTIVE: u32 = 0x20;
    const PAUSE_COUNT: u32 = 0x2c;
    c.cycles += 3;
    let inst = Addr(abi::arg(c, 0));
    let active = inst.off(ACTIVE).lw(c);
    c.g[V0][0] = active;
    if active != 0 {
        c.cycles += 4;
        let count = inst.off(PAUSE_COUNT).lw(c);
        c.g[V0][0] = sx((count as u32).wrapping_sub(1));
        let value = c.g[V0][0];
        inst.off(PAUSE_COUNT).sw(c, value);
        if (value as i64) < 0 {
            c.cycles += 1;
            inst.off(PAUSE_COUNT).sw(c, 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA};

    #[test]
    fn replay_save_begin_snapshots_matches() {
        check(ssx3_ee::s07::f_00271348, replay_save_begin_snapshots, 200, &|c, seed| {
            let mut r = Rng(0x71348 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let a = DATA + 0x100;
            let b = if seed % 2 == 0 { a } else { DATA + 0x140 };
            c.bus.write32(DATA + 0x3b0, a);
            c.bus.write32(DATA + 0x3d0, b);
            c.bus.write32(a + 0x14, r.next_u32());
            c.bus.write32(DATA + 0x638, r.next_u32());
        });
    }

    #[test]
    fn replay_save_begin_caches_matches() {
        check(ssx3_ee::s07::f_002714e0, replay_save_begin_caches, 100, &|c, seed| {
            let mut r = Rng(0x714e0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x638, r.next_u32());
        });
    }

    #[test]
    fn in_seq_heap_init_matches() {
        check(ssx3_ee::s07::f_002722e0, in_seq_heap_init, 100, &|c, seed| {
            let mut r = Rng(0x722e0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = if seed % 3 == 0 {
                0xffff_fff8
            } else {
                r.next_u32()
            } as u64;
        });
    }

    #[test]
    fn math_eval_cubic_matches() {
        check(ssx3_ee::s07::f_00272570, math_eval_cubic, 300, &|c, seed| {
            let mut r = Rng(0x72570 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.edge_f32());
            }
            // Cover t == 0, a denormal (compares as zero) and ordinary values.
            c.f[12] = match seed % 4 {
                0 => 0,
                1 => r.next_u32() | 0x3f80_0000,
                2 => r.next_u32() & 0x007f_ffff,
                _ => r.edge_f32(),
            };
        });
    }

    #[test]
    fn in_seq_get_instance_matches() {
        check_lohi(ssx3_ee::s07::f_00272cc0, in_seq_get_instance, 200, &|c, seed| {
            let mut r = Rng(0x72cc0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = seed % 4;
            c.g[A1][0] = index as u64 | ((r.next_u32() as u64) << 32);
            let table = DATA + 0x1000;
            c.bus.write32(DATA + 0x14, table);
            let record = table + index * 0x58;
            c.bus.write32(record + 0xc, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
        });
    }

    #[test]
    fn in_seq_span_track_activate_key_matches() {
        check_lohi(ssx3_ee::s07::f_002730c0, in_seq_span_track_activate_key, 200, &|c, seed| {
            let mut r = Rng(0x730c0 ^ seed as u64);
            scramble(c, &mut r);
            let track = DATA + 0x200;
            let key = DATA;
            let base = DATA + 0x1000;
            c.g[A0][0] = track as u64;
            c.g[A1][0] = key as u64;
            c.bus.write32(track + 0x20, base);
            c.bus.write32(track + 0x24, r.next_u32());
            c.bus.write32(track + 0x28, r.next_u32());
            c.bus.write32(key + 0x8, r.next_u32());
            c.bus.write32(key + 0xc, r.next_u32());
            let index = seed % 4;
            c.bus.write16(key + 2, index as u16);
            c.bus.write16(key + 4, (r.next_u32() & 0xffff) as u16);
            c.bus.write16(key + 6, (r.next_u32() & 0xffff) as u16);
            let kind = if seed % 2 == 0 {
                seed % 0x11
            } else {
                0x11 + seed % 8
            };
            c.bus.write16(key, kind as u16);
        });
    }

    #[test]
    fn in_seq_span_track_next_key_matches() {
        check(ssx3_ee::s07::f_00273140, in_seq_span_track_next_key, 200, &|c, seed| {
            let mut r = Rng(0x73140 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let keys = DATA + 0x100;
            let next = DATA + 0x200;
            c.bus.write32(DATA + 0x10, keys);
            c.bus.write32(DATA + 0x14, next);
            let count = (seed % 7) as u32 + 1;
            c.bus.write16(keys, count as u16);
            c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
            let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
            c.bus.write32(DATA + 0x18, cur);
        });
    }

    #[test]
    fn in_seq_event_track_fire_key_matches() {
        check(ssx3_ee::s07::f_00273418, in_seq_event_track_fire_key, 200, &|c, seed| {
            let mut r = Rng(0x73418 ^ seed as u64);
            scramble(c, &mut r);
            let track = DATA;
            let key = DATA + 0x100;
            c.g[A0][0] = track as u64;
            c.g[A1][0] = key as u64;
            c.bus.write32(track + 0x1c, DATA + 0x1000);
            c.bus.write32(track + 0x20, (seed % 4) as u32);
            c.bus.write32(key + 0x4, r.next_u32());
            let kind = if seed % 3 == 0 { seed % 9 } else { 9 + seed % 8 };
            c.bus.write16(key, kind as u16);
        });
    }

    #[test]
    fn in_seq_event_track_next_key_matches() {
        check(ssx3_ee::s07::f_00273468, in_seq_event_track_next_key, 200, &|c, seed| {
            let mut r = Rng(0x73468 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let keys = DATA + 0x100;
            let next = DATA + 0x200;
            c.bus.write32(DATA + 0x10, keys);
            c.bus.write32(DATA + 0x14, next);
            let count = (seed % 7) as u32 + 1;
            c.bus.write16(keys, count as u16);
            c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
            let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
            c.bus.write32(DATA + 0x18, cur);
        });
    }

    #[test]
    fn in_seq_ramp_track_activate_key_matches() {
        check_lohi(ssx3_ee::s07::f_00273860, in_seq_ramp_track_activate_key, 200, &|c, seed| {
            let mut r = Rng(0x73860 ^ seed as u64);
            scramble(c, &mut r);
            let track = DATA + 0x200;
            let key = DATA;
            let base = DATA + 0x1000;
            c.g[A0][0] = track as u64;
            c.g[A1][0] = key as u64;
            c.bus.write32(track + 0x1c, base);
            c.bus.write32(track + 0x20, r.next_u32());
            c.bus.write32(track + 0x24, r.next_u32());
            c.bus.write32(key + 0x8, r.next_u32());
            c.bus.write32(key + 0xc, r.next_u32());
            c.bus.write32(key + 0x10, r.edge_f32());
            c.bus.write32(key + 0x14, r.next_u32());
            let index = seed % 4;
            c.bus.write16(key + 2, index as u16);
            c.bus.write16(key + 4, (r.next_u32() & 0xffff) as u16);
            c.bus.write16(key + 6, (r.next_u32() & 0xffff) as u16);
            let kind = if seed % 2 == 0 {
                seed % 0x19
            } else {
                0x19 + seed % 8
            };
            c.bus.write16(key, kind as u16);
        });
    }

    #[test]
    fn in_seq_ramp_track_next_key_matches() {
        check(ssx3_ee::s07::f_002738f8, in_seq_ramp_track_next_key, 200, &|c, seed| {
            let mut r = Rng(0x738f8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let keys = DATA + 0x100;
            let next = DATA + 0x200;
            c.bus.write32(DATA + 0x10, keys);
            c.bus.write32(DATA + 0x14, next);
            let count = (seed % 7) as u32 + 1;
            c.bus.write16(keys, count as u16);
            c.bus.write16(next, (r.next_u32() & 0x3f) as u16);
            let cur = if seed % 2 == 0 { count - 1 } else { count + 3 };
            c.bus.write32(DATA + 0x18, cur);
        });
    }

    #[test]
    fn in_seq_channel_ctor_matches() {
        check(ssx3_ee::s07::f_00273938, in_seq_channel_ctor, 100, &|c, seed| {
            let mut r = Rng(0x73938 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..12 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn in_seq_channel_setup_matches() {
        check(ssx3_ee::s07::f_002739b8, in_seq_channel_setup, 200, &|c, seed| {
            let mut r = Rng(0x739b8 ^ seed as u64);
            scramble(c, &mut r);
            let data = DATA + 0x100;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = data as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            let extra = if seed % 2 == 0 { 0 } else { 0x10 };
            c.bus.write32(data + 8, extra);
            c.bus.write32(data, r.next_u32());
            let track = data + extra;
            let kind = if seed % 4 == 0 { seed % 9 } else { 9 + seed % 8 };
            c.bus.write16(track, kind as u16);
        });
    }

    #[test]
    fn in_seq_channel_get_track_matches() {
        check(ssx3_ee::s07::f_002742c8, in_seq_channel_get_track, 100, &|c, seed| {
            let mut r = Rng(0x742c8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = seed % 4;
            c.g[A1][0] = index as u64;
            let table = DATA + 0x1000;
            c.bus.write32(DATA + 0x24, table);
            c.bus.write32(table + index * 4, r.next_u32());
        });
    }

    #[test]
    fn in_seq_channel_get_length_matches() {
        check(ssx3_ee::s07::f_00274348, in_seq_channel_get_length, 100, &|c, seed| {
            let mut r = Rng(0x74348 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let track = DATA + 0x100;
            c.bus.write32(DATA + 0x1c, track);
            c.bus.write32(track + 4, r.next_u32());
        });
    }

    #[test]
    fn in_seq_channel_get_id_matches() {
        check(ssx3_ee::s07::f_002743c8, in_seq_channel_get_id, 100, &|c, seed| {
            let mut r = Rng(0x743c8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let track = DATA + 0x100;
            c.bus.write32(DATA + 0x1c, track);
            c.bus.write32(track, r.next_u32());
        });
    }

    #[test]
    fn in_seq_instance_ctor_matches() {
        check(ssx3_ee::s07::f_00274518, in_seq_instance_ctor, 100, &|c, seed| {
            let mut r = Rng(0x74518 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..0x12 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn in_seq_instance_start_matches() {
        check(ssx3_ee::s07::f_00274830, in_seq_instance_start, 300, &|c, seed| {
            let mut r = Rng(0x74830 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            // Path 0: not active. Path 1: active, no pending start. Path 2:
            // active with a pending start request.
            let path = seed % 3;
            c.bus.write32(DATA + 0x20, if path == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(DATA + 0x3c, if path == 2 { r.next_u32() | 1 } else { 0 });
        });
    }

    #[test]
    fn in_seq_instance_pause_matches() {
        check(ssx3_ee::s07::f_002749e8, in_seq_instance_pause, 200, &|c, seed| {
            let mut r = Rng(0x749e8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x20, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(DATA + 0x2c, r.next_u32());
        });
    }

    #[test]
    fn in_seq_instance_resume_matches() {
        check(ssx3_ee::s07::f_00274a08, in_seq_instance_resume, 300, &|c, seed| {
            let mut r = Rng(0x74a08 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let path = seed % 3;
            c.bus.write32(DATA + 0x20, if path == 0 { 0 } else { r.next_u32() | 1 });
            let count = match path {
                0 => r.next_u32(),
                1 => 1 + r.next_u32() % 100,
                _ => 0,
            };
            c.bus.write32(DATA + 0x2c, count);
        });
    }
}
