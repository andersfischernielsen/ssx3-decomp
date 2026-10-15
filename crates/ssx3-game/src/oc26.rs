//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Hand-written, typed replacements for small generated functions, each with a
//! differential test against the generated body.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame, VSlot};
use crate::events::{deliver_event, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `World_GetAvalancheMatrix` (0x2d1cf0), a direct-call target of
/// `AvaSplineModifier_Update`.
const WORLD_GET_AVALANCHE_MATRIX: Callee =
    Callee { addr: 0x002d_1cf0, f: ssx3_ee::s09::f_002d1cf0 };

const ATTACHED_EMITTER_FX_FREE_TYPE_DATA: Callee =
    Callee { addr: 0x002f_1810, f: ssx3_ee::s09::f_002f1810 };

const ATTACHED_EMITTER_FX_LOAD_TYPE_DATA: Callee =
    Callee { addr: 0x002f_17d0, f: ssx3_ee::s09::f_002f17d0 };
const ATTACHED_EMITTER_FX_INIT: Callee =
    Callee { addr: 0x002f_18d8, f: ssx3_ee::s09::f_002f18d8 };
const ATTACHED_EMITTER_FX_RESET: Callee =
    Callee { addr: 0x002f_1a00, f: ssx3_ee::s09::f_002f1a00 };
const RETURN_TRIVIAL: Callee = Callee { addr: 0x002f_8ff8, f: ssx3_ee::s09::f_002f8ff8 };

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_3508, anim_sequence_ctor, "AnimSequence_ctor"),
    (0x0031_3868, anim_sequence_fire_triggers_in_range, "AnimSequence_FireTriggersInRange"),
    (0x0031_3938, anim_sequence_fire_triggers_at_time, "AnimSequence_FireTriggersAtTime"),
    (0x0031_39a8, anim_sequence_fire_end_triggers, "AnimSequence_FireEndTriggers"),
    (0x0031_4368, anim_seq_list_append, "AnimSeqList_Append"),
    (0x0029_0b58, audio_eval_curve5, "Audio_EvalCurve5"),
    (0x003d_ef40, async_file_assign_serial, "AsyncFile_AssignSerial"),
    (0x0035_7750, ava_spline_modifier_ctor, "AvaSplineModifier_ctor"),
    (0x0036_1aa8, ava_spline_modifier_v01, "AvaSplineModifier_v01"),
    (0x0036_1de0, anim_delta_dtor, "AnimDelta_dtor"),
    (0x002d_9738, avalanche_add_node_event, "Avalanche_AddNodeEvent"),
    (0x0035_7820, ava_spline_modifier_update, "AvaSplineModifier_Update"),
    (0x002f_2218, attached_emitter_fx_destroy_emitter, "AttachedEmitterFX_DestroyEmitter"),
    (0x0034_e600, anim_object_get_node_position, "AnimObject_GetNodePosition"),
    (0x002f_1810, attached_emitter_fx_free_type_data, "AttachedEmitterFX_FreeTypeData"),
    (0x002f_1850, attached_emitter_fx_ctor, "AttachedEmitterFX_ctor"),
    (0x002f_1890, attached_emitter_fx_ctor_2, "AttachedEmitterFX_ctor_2"),
    (0x002f_18d8, attached_emitter_fx_init, "AttachedEmitterFX_Init"),
    (0x002f_1940, attached_emitter_fx_dtor, "AttachedEmitterFX_dtor"),
    (0x002d_7e60, avalanche_build_active_node_array, "Avalanche_BuildActiveNodeArray"),
];

// ---------------------------------------------------------------------------
// AnimSequence trigger lists (0x0031_38xx / 0x0031_39xx / 0x0031_35xx).

/// Trigger-chain node layout (stride 0x10 in practice; only the first four
/// words are used by the firing helpers).
mod trigger {
    pub const BIT: u32 = 0x0;
    pub const COUNT: u32 = 0x4;
    pub const TIME: u32 = 0x8;
    pub const NEXT: u32 = 0xc;
}

/// `AnimSequence` fields the firing helpers touch.
const TRIGGERS: u32 = 0xac;
const MASK_A: u32 = 0xb0;
const MASK_B: u32 = 0xb8;
const END_FLAG: u32 = 0xc0;

/// Set bit `bit` of the two trigger masks from `base` (a `T0`/`T1` holding 1),
/// mirroring the original's `dsll32`-free `dsll` + `or`/`nor`/`and` sequence.
fn fire_mask_bit(c: &mut Cpu, seq: u32, base: u64, bit: u64) {
    let mask_a = Addr(seq.wrapping_add(MASK_A)).ld(c);
    c.g[V1][0] = mask_a;
    let set = base << (bit & 63);
    c.g[V0][0] = set;
    let mask_b = Addr(seq.wrapping_add(MASK_B)).ld(c);
    c.g[A0][0] = mask_b;
    let inv = !mask_a;
    c.g[A1][0] = inv;
    let new_a = mask_a | set;
    c.g[V1][0] = new_a;
    let clear = set & inv;
    c.g[V0][0] = clear;
    Addr(seq.wrapping_add(MASK_A)).sd(c, new_a);
    let new_b = mask_b | clear;
    c.g[A0][0] = new_b;
    Addr(seq.wrapping_add(MASK_B)).sd(c, new_b);
}

/// `void AnimSequence_FireEndTriggers(AnimSequence *this)` (0x3139a8): set the
/// end flag (+0xc0) and, for every trigger in the chain at +0xac whose count
/// (+4) is non-zero, set bit `n` of the mask at +0xb8 (`n` = the trigger's word
/// at +0); bit `n` is cleared from +0xb0. 8 cycles per node plus a back-edge
/// event check at 0x3139c8.
pub fn anim_sequence_fire_end_triggers(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_39c8;
    c.cycles += 5;
    let seq = Addr(abi::arg(c, 0));
    c.g[A3][0] = seq.0 as u64;
    c.g[V0][0] = 1;
    let mut trigger = seq.off(TRIGGERS).lw(c);
    c.g[A2][0] = trigger;
    seq.off(END_FLAG).sw(c, 1);
    if trigger != 0 {
        c.cycles += 3;
        c.g[T0][0] = 1;
        let mut count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
        c.g[V0][0] = count;
        loop {
            c.cycles += 2;
            if count == 0 {
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            } else {
                c.cycles += 11;
                let bit = Addr(trigger as u32).off(trigger::BIT).lw(c);
                c.g[V0][0] = bit;
                fire_mask_bit(c, seq.0, c.g[T0][0], bit);
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            }
            c.cycles += 2;
            if trigger == 0 {
                break;
            }
            count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
            c.g[V0][0] = count;
            if event_due(c) {
                deliver_event(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AnimSequence_FireTriggersAtTime(AnimSequence *this, float time)`
/// (0x313938): for every trigger at +0xac whose count (+4) is non-zero and
/// whose time (+8) equals the argument (f12), set bit `n` (word at +0) of the
/// mask at +0xb0, clear it from +0xb8. Back-edge event check at 0x313950.
pub fn anim_sequence_fire_triggers_at_time(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_3950;
    let at = FpuF32(c.f[12]);
    c.cycles += 4;
    let seq = Addr(abi::arg(c, 0));
    c.g[A3][0] = seq.0 as u64;
    let mut trigger = seq.off(TRIGGERS).lw(c);
    c.g[A2][0] = trigger;
    c.g[T0][0] = 1;
    if trigger != 0 {
        c.cycles += 2;
        let mut count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
        c.g[V0][0] = count;
        loop {
            c.cycles += 2;
            if count != 0 {
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            } else {
                c.cycles += 5;
                let t = Addr(trigger as u32).off(trigger::TIME).read_fpu(c);
                c.f[0] = t.0;
                let eq = t.eq_s(at);
                abi::set_fcc(c, eq);
                if !eq {
                    trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                    c.g[A2][0] = trigger;
                } else {
                    c.cycles += 11;
                    let bit = Addr(trigger as u32).off(trigger::BIT).lw(c);
                    c.g[V0][0] = bit;
                    fire_mask_bit(c, seq.0, c.g[T0][0], bit);
                    trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                    c.g[A2][0] = trigger;
                }
            }
            c.cycles += 2;
            if trigger == 0 {
                break;
            }
            count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
            c.g[V0][0] = count;
            if event_due(c) {
                deliver_event(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void AnimSequence_FireTriggersInRange(AnimSequence *this, float lo,
/// float hi)` (0x313868): fire every trigger whose time lies in [lo, hi]
/// (inclusive, either edge may be open); `lo == hi` fires nothing. Bit `n`
/// (word at +0) is set in +0xb8 and cleared from +0xb0. Back-edge event check
/// at 0x3138a0.
pub fn anim_sequence_fire_triggers_in_range(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_38a0;
    let lo = FpuF32(c.f[12]);
    let hi = FpuF32(c.f[13]);
    c.cycles += 4;
    let empty = lo.eq_s(hi);
    abi::set_fcc(c, empty);
    let seq = Addr(abi::arg(c, 0));
    c.g[A3][0] = seq.0 as u64;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    let lo_le_hi = lo.le(hi);
    abi::set_fcc(c, lo_le_hi);
    if lo_le_hi {
        c.g[T0][0] = 1;
    } else {
        c.cycles += 1;
        c.g[T0][0] = 0;
    }
    c.cycles += 3;
    let mut trigger = seq.off(TRIGGERS).lw(c);
    c.g[A2][0] = trigger;
    c.g[T1][0] = 1;
    if trigger == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let mut count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
    c.g[V0][0] = count;
    loop {
        c.cycles += 2;
        let mut fire = false;
        if count != 0 {
            trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
            c.g[A2][0] = trigger;
        } else {
            c.cycles += 2;
            let ascending = c.g[T0][0] != 0;
            let t = Addr(trigger as u32).off(trigger::TIME).read_fpu(c);
            c.f[0] = t.0;
            'test: {
                if ascending {
                    c.cycles += 4;
                    let lt = lo.lt(t);
                    abi::set_fcc(c, lt);
                    if !lt {
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                    c.cycles += 4;
                    let le = t.le(hi);
                    abi::set_fcc(c, le);
                    if !le {
                        c.cycles += 2;
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                } else {
                    c.cycles += 4;
                    let le = hi.le(t);
                    abi::set_fcc(c, le);
                    if !le {
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                    c.cycles += 4;
                    let lt = t.lt(lo);
                    abi::set_fcc(c, lt);
                    if !lt {
                        trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                        c.g[A2][0] = trigger;
                        break 'test;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = Addr(trigger as u32).off(trigger::BIT).lw(c);
                fire = true;
            }
            if fire {
                c.cycles += 10;
                fire_mask_bit(c, seq.0, c.g[T1][0], c.g[V0][0]);
                trigger = Addr(trigger as u32).off(trigger::NEXT).lw(c);
                c.g[A2][0] = trigger;
            }
        }
        c.cycles += 2;
        if trigger == 0 {
            break;
        }
        count = Addr(trigger as u32).off(trigger::COUNT).lw(c);
        c.g[V0][0] = count;
        if event_due(c) {
            deliver_event(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AnimSeqList.

/// `AnimSeqList` layout.
const LIST_HEAD: u32 = 0x4;
const SEQ_NEXT: u32 = 0xc8;

/// `void AnimSeqList_Append(AnimSeqList *list, AnimSequence *seq)`
/// (0x314368): append `seq` to the singly-linked list at +0x4 (following
/// +0xc8), or set it as the head when the list is empty; clear the new node's
/// link and bump the count at +0. Back-edge event check at 0x314388.
pub fn anim_seq_list_append(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_4388;
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let seq = abi::arg(c, 1);
    let head = list.off(LIST_HEAD).lw(c);
    c.g[V0][0] = head;
    c.g[V1][0] = head;
    if head == 0 {
        c.cycles += 1;
        list.off(LIST_HEAD).sw(c, seq as u64);
    } else {
        c.cycles += 3;
        let first_link = Addr(head as u32).off(SEQ_NEXT).lw(c);
        c.g[V0][0] = first_link;
        if first_link == 0 {
            Addr(head as u32).off(SEQ_NEXT).sw(c, seq as u64);
        } else {
            c.cycles += 2;
            let mut node = first_link;
            c.g[V1][0] = node;
            loop {
                c.cycles += 7;
                let link = Addr(node as u32).off(SEQ_NEXT).lw(c);
                c.g[V0][0] = link;
                if link != 0 {
                    node = link;
                    c.g[V1][0] = link;
                    if event_due(c) {
                        deliver_event(c, HEAD);
                    }
                } else {
                    c.cycles += 2;
                    Addr(node as u32).off(SEQ_NEXT).sw(c, seq as u64);
                    break;
                }
            }
        }
    }
    c.cycles += 5;
    Addr(seq).off(SEQ_NEXT).sw(c, 0);
    let count = list.lw(c) as u32;
    c.g[V0][0] = sx(count.wrapping_add(1));
    list.sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void AnimSequence_ctor(AnimSequence *this, void *arg)` (0x313508):
/// store `arg` at +0, clear the three track records at +0x18.., then fill the
/// trigger/mask block from the constants at 0x4ff230. Back-edge event check
/// at 0x313528.
pub fn anim_sequence_ctor(c: &mut Cpu) {
    const HEAD: u32 = 0x0031_3528;
    c.cycles += 8;
    let seq = abi::arg(c, 0);
    let arg = abi::arg64(c, 1);
    c.g[A3][0] = seq as u64;
    c.g[V0][0] = 2;
    c.g[V1][0] = u64::MAX;
    Addr(seq).sw(c, arg);
    c.g[V1][0] = 2;
    let mut p = seq.wrapping_add(0x50);
    c.g[V0][0] = sx(p);
    loop {
        c.cycles += 7;
        Addr(p).sw(c, 0);
        let k = (c.g[V1][0] as u32).wrapping_sub(1);
        c.g[V1][0] = sx(k);
        let more = (c.g[V1][0] as i64) >= 0;
        let np = p.wrapping_sub(0x1c);
        c.g[V0][0] = sx(np);
        p = np;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event(c, HEAD);
        }
    }
    c.cycles += 26;
    c.g[A0][0] = 0x0050_0000;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0); // f0 = 1.0f
    let table = 0x0050_0000u32.wrapping_sub(0xdd0);
    c.g[A0][0] = sx(table);
    c.g[A2][0] = u64::MAX;
    c.lq(5, table);
    c.g[V0][0] = seq as u64;
    c.sq(5, seq.wrapping_add(0x60));
    c.lq(3, table.wrapping_add(0x10));
    Addr(seq).off(0x88).sd(c, u64::MAX);
    c.sq(3, seq.wrapping_add(0x70));
    Addr(seq).off(0x98).write_u32(c, 0x3f80_0000);
    Addr(seq).off(0x80).sw(c, 0);
    Addr(seq).off(0x84).sw(c, 0);
    Addr(seq).off(0x90).write_u32(c, 0x3f80_0000);
    Addr(seq).off(0x9c).sw(c, 0);
    Addr(seq).off(0x94).write_u32(c, 0x3f80_0000);
    Addr(seq).off(0xa0).sw(c, 0);
    Addr(seq).off(0xa4).sw(c, 0);
    Addr(seq).off(0xa8).sw(c, 0);
    Addr(seq).off(0xb8).sd(c, 0);
    Addr(seq).off(0xb0).sd(c, 0);
    Addr(seq).off(0xac).sw(c, 0);
    Addr(seq).off(0xc0).sw(c, 0);
    c.sw(seq.wrapping_add(0xc4), 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio curve evaluation.

/// `float Audio_EvalCurve5(const float *curve, float x)` (0x290b58): find the
/// segment [curve[4k], curve[4k+4]) containing `x` over the five control
/// points and interpolate `value[k] + (x - t0) * (value[k+1] - value[k]) /
/// (t1 - t0)`; the value words live at +0x14 and the segment index is scaled by
/// 4. Falls back to the first value when `x` is below the first key, and to the
/// last or 0 when above. Back-edge event check at 0x290b68.
pub fn audio_eval_curve5(c: &mut Cpu) {
    const HEAD: u32 = 0x0029_0b68;
    c.cycles += 4;
    let curve = abi::arg(c, 0);
    let x = FpuF32(c.f[12]);
    c.g[A1][0] = 1;
    c.g[V1][0] = sx(curve.wrapping_add(4));
    c.g[A3][0] = 0;
    loop {
        c.cycles += 5;
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_sub(4));
        let f1 = FpuF32(c.f[1]);
        let le = f1.le(x);
        abi::set_fcc(c, le);
        c.g[V0][0] = c.g[A3][0];
        if !le {
            // 'b_290b90 tail
        } else {
            c.cycles += 5;
            c.lwc1(3, (c.g[V1][0] as u32));
            let f3 = FpuF32(c.f[3]);
            let lt = x.lt(f3);
            abi::set_fcc(c, lt);
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
            if lt {
                c.cycles += 12;
                let base = curve.wrapping_add(0x14);
                c.g[V0][0] = sx(base);
                let f0 = x - f1;
                c.f[0] = f0.0;
                let va = base.wrapping_add(c.g[A3][0] as u32);
                c.g[V1][0] = sx(va);
                let f3b = f3 - f1;
                c.f[3] = f3b.0;
                let vb = base.wrapping_add(c.g[A2][0] as u32);
                c.g[V0][0] = sx(vb);
                c.lwc1(2, va);
                c.lwc1(1, vb);
                let f1b = FpuF32(c.f[1]) - FpuF32(c.f[2]);
                c.f[1] = f1b.0;
                let f0b = f0 * f1b;
                c.f[0] = f0b.0;
                let q = f0b.div(f3b);
                c.f[0] = q.0;
                let r = FpuF32(c.f[2]) + q;
                c.f[0] = r.0;
                abi::ret(c);
                return;
            }
        }
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let k = c.g[A1][0];
        c.g[V0][0] = ((k as i64) < 5) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event(c, HEAD);
            }
            continue;
        }
        break;
    }
    // Past the last segment: compare `x` against the first key.
    c.cycles += 5;
    c.lwc1(0, curve);
    let first = FpuF32(c.f[0]);
    let x_lt_first = x.lt(first);
    abi::set_fcc(c, x_lt_first);
    if !x_lt_first {
        c.lwc1(0, curve.wrapping_add(0x10));
        c.cycles += 3;
        let last = FpuF32(c.f[0]);
        let last_le_x = last.le(x);
        abi::set_fcc(c, last_le_x);
        if last_le_x {
            c.cycles += 2;
            c.lwc1(0, curve.wrapping_add(0x24));
            abi::ret(c);
        } else {
            c.cycles += 3;
            c.mtc1(0, 0); // f0 = 0
            abi::ret(c);
        }
    } else {
        c.cycles += 2;
        c.lwc1(0, curve.wrapping_add(0x14));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// AsyncFile pool / serial.

/// The serial counter the file system hands out.
const ASYNC_SERIAL: u32 = 0x0052_0000 - 0x6400;

/// `void AsyncFile_AssignSerial(AsyncFile *f)` (0x3def40): bump the pool's
/// serial counter by 0x100 (or reset it to 0x100 after the wrap to 0) and OR
/// its low byte's zero-extended... it stores `serial | f->serial_byte(+0)`.
pub fn async_file_assign_serial(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = 0x0052_0000;
    let value = Addr(ASYNC_SERIAL).lw(c) as u32;
    let next = value.wrapping_add(0x100);
    c.g[V0][0] = sx(next);
    if next != 0 {
        Addr(ASYNC_SERIAL).sw(c, sx(next));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x100;
        Addr(ASYNC_SERIAL).sw(c, 0x100);
    }
    c.cycles += 5;
    let serial = Addr(ASYNC_SERIAL).lw(c);
    c.g[V1][0] = serial;
    let base = Addr(abi::arg(c, 0));
    let b = base.lbu(c);
    c.g[V0][0] = b | serial;
    base.sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AvaSplineModifier.

/// `AvaSplineModifier *AvaSplineModifier_ctor(AvaSplineModifier *this,
/// void *arg)` (0x357750): set the vtable, store `arg` at +0x80, clear +0x30,
/// and copy the 0x40-byte block at `arg + 0x10` into `this + 0x40` with four
/// `lqc2`/`sqc2` pairs.
pub fn ava_spline_modifier_ctor(c: &mut Cpu) {
    c.cycles += 18;
    let this = abi::arg(c, 0);
    let arg = abi::arg(c, 1);
    c.g[V1][0] = 0x0049_0000;
    c.g[V0][0] = this as u64;
    let vtable = 0x0049_0000u32.wrapping_sub(0xcc8);
    c.g[V1][0] = sx(vtable);
    let src = arg.wrapping_add(0x10);
    c.g[A0][0] = sx(src);
    c.sw(this, vtable as u64);
    c.sw(this.wrapping_add(0x80), arg as u64);
    let dst = this.wrapping_add(0x40);
    c.g[V1][0] = sx(dst);
    c.sw(this.wrapping_add(0x30), 0);
    for i in 0..4 {
        c.lqc2(1 + i, src.wrapping_add(0x10 * i as u32));
    }
    for i in 0..4 {
        c.sqc2(1 + i, dst.wrapping_add(0x10 * i as u32));
    }
    abi::ret(c);
}

/// `void AvaSplineModifier_v01(AvaSplineModifier *this, int flags)`
/// (0x361aa8): scalar deleting destructor: set the vtable and, if `flags & 1`,
/// free `this`.
pub fn ava_spline_modifier_v01(c: &mut Cpu) {
    c.cycles += 7;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0x0049_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    let vtable = 0x0049_0000u32.wrapping_add(0x13f8);
    c.g[V0][0] = sx(vtable);
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    let flags = c.g[A1][0];
    c.g[A1][0] = flags & 1;
    let free = c.g[A1][0] == 0;
    c.sw(this, vtable as u64);
    if !free {
        c.cycles += 2;
        call_ptr(c, 0x0031_7e50, 0x0036_1acc).unwrap_or(());
        if c.pc != 0x0036_1acc {
            return;
        }
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// AnimObject / AnimDelta destructors.

/// `void AnimDelta_dtor(AnimDelta *this, int flags)` (0x361de0): set the
/// vtable then run `AnimObject_dtor(this, flags)`.
pub fn anim_delta_dtor(c: &mut Cpu) {
    c.cycles += 10;
    let this = abi::arg(c, 0);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V0][0] = 0x0049_0000;
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[V1][0] = this as u64;
    let vtable = 0x0049_0000u32.wrapping_sub(0x1590);
    c.g[V0][0] = sx(vtable);
    c.g[RA][0] = sx(0x0036_1dfc);
    c.sw(this.wrapping_add(0x3c), vtable as u64);
    call_ptr(c, 0x0034_1cf0, 0x0036_1dfc).unwrap_or(());
    if c.pc != 0x0036_1dfc {
        return;
    }
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Avalanche node events.

/// `void Avalanche_AddNodeEvent(Avalanche *ava, int event, int node)`
/// (0x2d9738): if the node's event count at +0xe is below 0x40, look up the
/// node in the list at +4 (following +0xf4) and store `event` in the halfword
/// table at `ava + 0x110 + count*2` and the node's index in the table at
/// `ava + 0x112 + count*2`, then bump the count. Back-edge event check at
/// 0x2d9768.
pub fn avalanche_add_node_event(c: &mut Cpu) {
    const COUNT: u32 = 0xe;
    const LIST: u32 = 0x4;
    const LINK: u32 = 0xf4;
    const EVENTS: u32 = 0x110;
    const NODES: u32 = 0x112;
    const HEAD: u32 = 0x002d_9768;
    c.cycles += 4;
    let ava = abi::arg(c, 0);
    let event = abi::arg64(c, 1);
    let node = abi::arg64(c, 2);
    c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V0][0] = (c.g[V0][0] < 0x40) as u64;
    let full = c.g[V0][0] == 0;
    c.g[A3][0] = 0;
    if full {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    let mut immediate = false;
    {
        c.cycles += 5;
        let mut link = Addr(ava).off(LIST).lw(c);
        c.g[V1][0] = link;
        c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
        c.g[T0][0] = sx(ava.wrapping_add(EVENTS));
        let empty = link == 0;
        c.g[T1][0] = sx(ava.wrapping_add(NODES));
        if !empty {
            c.cycles += 2;
            if link == node {
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                immediate = true;
            } else {
                c.cycles += 1;
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[V1][0] = link;
                loop {
                    c.cycles += 2;
                    let end = link == 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    if link != node {
                        link = Addr(link as u32).off(LINK).lw(c);
                        c.g[V1][0] = link;
                        if event_due(c) {
                            deliver_event(c, HEAD);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
    }
    if !immediate {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    c.cycles += 9;
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sh(c.g[V0][0] as u32, event);
    c.g[V1][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sh(c.g[V1][0] as u32, c.g[A3][0]);
    c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sh(ava.wrapping_add(COUNT), c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// AvaSplineModifier.

/// `void AvaSplineModifier_Update(AvaSplineModifier *this)` (0x357820):
/// `World_GetAvalancheMatrix(this->matrix(+0x80), &this->+0x40)`.
pub fn ava_spline_modifier_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[A1][0] = this as u64;
        c.g[A0][0] = Addr(this).off(0x80).lw(c);
        c.g[A1][0] = sx(this.wrapping_add(0x40));
        WORLD_GET_AVALANCHE_MATRIX.call(c, 0x0035_7838)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Attached emitter FX.

/// `void AttachedEmitterFX_DestroyEmitter(AttachedEmitterFX *this, int index)`
/// (0x2f2218): clear `this->emitters[index]` (+0x6c + index*4) and, if set,
/// call its virtual release (slot +0x20 of `*(emitter + 0x1f8)`).
pub fn attached_emitter_fx_destroy_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        c.g[A1][0] = sx(index.wrapping_shl(2));
        frame.save(c, S0, 0x10);
        c.g[A0][0] = sx(this.wrapping_add(0x6c));
        frame.save_ra(c, 0);
        let slot = this.wrapping_add(0x6c).wrapping_add(index.wrapping_shl(2));
        c.g[S0][0] = sx(slot);
        let obj = Addr(slot).lw(c);
        c.g[A2][0] = obj;
        let empty = obj == 0;
        c.g[A1][0] = 3;
        if !empty {
            c.cycles += 5;
            let vt = Addr(obj as u32).off(0x1f8).lw(c);
            c.g[V0][0] = vt;
            let m = VSlot::read(c, vt, 0x20);
            c.g[A0][0] = m.this(obj);
            c.g[V1][0] = m.func;
            call_ptr(c, m.func as u32, 0x002f_2250)?;
        }
        c.cycles += 5;
        Addr(slot).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void AnimObject_GetNodePosition(AnimObject *obj, int node, void *out,
/// void *arg)` (0x34e600): if `node` equals `obj->selected(+0x2c)`, copy the
/// node's 16-byte position (from `*(obj->model(+0x20))->+0x30` via a virtual
/// call) to `out` and to the stack; else copy the node's entry (at
/// `out = obj->model + node*0x40 + 0x40`, via a virtual call) to `out`.
pub fn anim_object_get_node_position(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x640);
        let obj = abi::arg(c, 0);
        let t0 = obj.wrapping_add(0x14);
        c.g[T0][0] = sx(t0);
        frame.save(c, S1, 0x620);
        frame.save(c, S0, 0x630);
        frame.save_ra(c, 0x610);
        let selected = Addr(obj).off(0x2c).lw(c);
        c.g[V0][0] = selected;
        let matches = abi::arg64(c, 1) == selected;
        c.g[S1][0] = abi::arg64(c, 3);
        if matches {
            c.cycles += 10;
            let vt = Addr(obj).off(0x20).lw(c);
            c.g[V1][0] = vt;
            c.g[A1][0] = abi::arg64(c, 2);
            let m = VSlot::read(c, vt, 0xe8);
            c.g[A0][0] = m.this(t0 as u64);
            c.g[V0][0] = m.func;
            call_ptr(c, m.func as u32, 0x0034_e638)?;
            c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.sq(3, c.g[S1][0] as u32);
            c.sq(3, c.g[SP][0] as u32);
        } else {
            c.cycles += 14;
            let idx = abi::arg(c, 2);
            c.g[S0][0] = sx(idx << 6);
            let vt = Addr(obj).off(0x20).lw(c);
            c.g[A3][0] = vt;
            c.g[A1][0] = sx(abi::arg(c, 1).wrapping_add(0x10));
            c.g[V0][0] = 0x17;
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = u64::MAX;
            let m = VSlot::read(c, vt, 0x108);
            c.g[A0][0] = m.this(t0 as u64);
            c.g[V0][0] = m.func;
            call_ptr(c, m.func as u32, 0x0034_e670)?;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[SP][0] as u32));
            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x40));
            c.sq(2, c.g[S1][0] as u32);
            c.sq(2, c.g[SP][0] as u32);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore_ra(c, 0x610);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AttachedEmitterFX type data / constructors.

/// `void AttachedEmitterFX_FreeTypeData(void)` (0x2f1810): run the type
/// data's two shutdown hooks, and if the global type data (`gp+0xac0`) is
/// set, free it and clear the global.
pub fn attached_emitter_fx_free_type_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002f_1820);
        RETURN_TRIVIAL.call(c, 0x002f_1820)?;
        c.g[RA][0] = sx(0x002f_1828);
        call_ptr(c, 0x002f_a5f8, 0x002f_1828)?;
        let gp = c.g[GP][0] as u32;
        let data = Addr(gp.wrapping_add(0xac0)).lw(c);
        c.g[A0][0] = data;
        if data != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002f_183c)?;
        }
        c.cycles += 3;
        Addr(gp.wrapping_add(0xac0)).sw(c, 0);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `AttachedEmitterFX *AttachedEmitterFX_ctor(AttachedEmitterFX *this,
/// void *type)` (0x2f1850): set the vtable, store `type` at +0x98, and run
/// `AttachedEmitterFX_Init`; returns `this`.
pub fn attached_emitter_fx_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let ty = abi::arg64(c, 1);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        let vtable = 0x0049_0000u32.wrapping_sub(0x7e30);
        c.g[V0][0] = sx(vtable);
        frame.save_ra(c, 0);
        c.g[S0][0] = this as u64;
        Addr(this).off(0x94).sw(c, 0);
        Addr(this).off(0x9c).sw(c, 0);
        Addr(this).off(0xa4).sw(c, vtable as u64);
        c.g[RA][0] = sx(0x002f_187c);
        Addr(this).off(0x98).sw(c, ty);
        ATTACHED_EMITTER_FX_INIT.call(c, 0x002f_187c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `AttachedEmitterFX *AttachedEmitterFX_ctor_2(AttachedEmitterFX *this,
/// int index, void *type)` (0x2f1890): as the ctor but store `index` at +0x9c
/// (leaving +0x98 clear) and run `AttachedEmitterFX_Reset` afterwards.
pub fn attached_emitter_fx_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let index = abi::arg64(c, 1);
        let ty = abi::arg64(c, 2);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x10);
        let vtable = 0x0049_0000u32.wrapping_sub(0x7e30);
        c.g[V0][0] = sx(vtable);
        frame.save_ra(c, 0);
        c.g[S0][0] = this as u64;
        Addr(this).off(0x94).sw(c, 0);
        Addr(this).off(0x98).sw(c, 0);
        Addr(this).off(0x9c).sw(c, index);
        c.g[RA][0] = sx(0x002f_18bc);
        Addr(this).off(0xa4).sw(c, vtable as u64);
        ATTACHED_EMITTER_FX_INIT.call(c, 0x002f_18bc)?;
        c.g[RA][0] = sx(0x002f_18c4);
        c.g[A0][0] = c.g[S0][0];
        ATTACHED_EMITTER_FX_RESET.call(c, 0x002f_18c4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void AttachedEmitterFX_Init(AttachedEmitterFX *this)` (0x2f18d8): if the
/// global type data (`gp+0xac4`) is absent, load it; then bump the type
/// data's reference count and clear the ten words at +0x90..0x68 with a
/// back-edge event check at 0x2f1910.
pub fn attached_emitter_fx_init(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x002f_1910;
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        let gp = c.g[GP][0] as u32;
        let loaded = Addr(gp.wrapping_add(0xac4)).lw(c);
        c.g[V0][0] = loaded;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = this as u64;
        if loaded == 0 {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002f_18f8);
            ATTACHED_EMITTER_FX_LOAD_TYPE_DATA.call(c, 0x002f_18f8)?;
        }
        c.cycles += 6;
        let count = Addr(gp.wrapping_add(0xac4)).lw(c);
        c.g[V0][0] = count;
        let mut p = this.wrapping_add(0x90);
        c.g[A0][0] = sx(p);
        c.g[V1][0] = 9;
        c.g[V0][0] = sx((count as u32).wrapping_add(1));
        Addr(gp.wrapping_add(0xac4)).sw(c, c.g[V0][0]);
        let mut k = 9i64;
        loop {
            c.cycles += 7;
            Addr(p).sw(c, 0);
            k -= 1;
            c.g[V1][0] = sx(k as u32);
            let more = k >= 0;
            p = p.wrapping_sub(4);
            c.g[A0][0] = sx(p);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event(c, HEAD);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AttachedEmitterFX destructor.

/// `void AttachedEmitterFX_dtor(AttachedEmitterFX *this, int flags)`
/// (0x2f1940): drop the type data reference (freeing it at 0), release and
/// clear the ten emitter slots at +0x6c, then free `this` when `flags & 1`.
/// Back-edge event check at 0x2f1990.
pub fn attached_emitter_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x002f_1990;
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        let gp = c.g[GP][0] as u32;
        let refcount = Addr(gp.wrapping_add(0xac4)).lw(c);
        c.g[V1][0] = refcount;
        frame.save(c, S2, 0x20);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S0, 0x40);
        let this = abi::arg(c, 0);
        c.g[S2][0] = this as u64;
        frame.save(c, S1, 0x30);
        let vtable = 0x0049_0000u32.wrapping_sub(0x7e30);
        c.g[V0][0] = sx(vtable);
        frame.save(c, S3, 0x10);
        let count = (refcount as u32).wrapping_sub(1);
        c.g[V1][0] = sx(count);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 1);
        Addr(this).off(0xa4).sw(c, vtable as u64);
        let still_used = count != 0;
        Addr(gp.wrapping_add(0xac4)).sw(c, c.g[V1][0]);
        if !still_used {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002f_1984);
            ATTACHED_EMITTER_FX_FREE_TYPE_DATA.call(c, 0x002f_1984)?;
        }
        c.cycles += 3;
        c.g[S3][0] = c.g[S0][0] & 1;
        c.g[S1][0] = 9;
        let mut p = this.wrapping_add(0x6c);
        c.g[S0][0] = sx(p);
        let mut k = 9i64;
        loop {
            c.cycles += 3;
            let obj = Addr(p).lw(c);
            c.g[A2][0] = obj;
            c.g[A1][0] = 3;
            if obj != 0 {
                c.cycles += 5;
                let vt = Addr(obj as u32).off(0x1f8).lw(c);
                c.g[V0][0] = vt;
                let m = VSlot::read(c, vt, 0x20);
                c.g[A0][0] = m.this(obj);
                c.g[V1][0] = m.func;
                call_ptr(c, m.func as u32, 0x002f_19b0)?;
            }
            c.cycles += 4;
            Addr(p).sw(c, 0);
            k -= 1;
            c.g[S1][0] = sx(k as u32);
            let more = k >= 0;
            p = p.wrapping_add(4);
            c.g[S0][0] = sx(p);
            if more {
                if event_due(c) {
                    deliver_event(c, HEAD);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        let free = c.g[S3][0] == 0;
        frame.restore(c, S0, 0x40);
        if !free {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002f_19d0);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0031_7e50, 0x002f_19d0)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Avalanche active-node array.

/// `void Avalanche_BuildActiveNodeArray(Avalanche *ava)` (0x2d7e60): count
/// the nodes reachable through `ava->nodes(+4)` -> +0x2e4, allocate
/// `count*4` bytes (name 0x487548) and store the pointer at +8, then fill the
/// array with the node pointers. Back-edge event checks at 0x2d7e80 /
/// 0x2d7ec8.
pub fn avalanche_build_active_node_array(c: &mut Cpu) {
    guest(c, |c| {
        const NODES: u32 = 0x4;
        const ARRAY: u32 = 0x8;
        const LINK: u32 = 0x2e4;
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let ava = abi::arg(c, 0);
        c.g[S0][0] = ava as u64;
        let mut link = Addr(ava).off(NODES).lw(c);
        c.g[A1][0] = link;
        c.g[A0][0] = 0;
        if link != 0 {
            c.cycles += 1;
            let mut count = 0u32;
            loop {
                c.cycles += 7;
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[A1][0] = link;
                let more = link != 0;
                count = count.wrapping_add(1);
                c.g[A0][0] = sx(count);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event(c, 0x002d_7e80);
                }
            }
        }
        // operator_new_named(count*4, 0x487548, 0, 0)
        c.cycles += 10;
        c.g[A1][0] = 0x0048_0000;
        let size = (c.g[A0][0] as u32) << 2;
        c.g[A0][0] = sx(size);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x7548));
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002d_7eb4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002d_7eb4)?;
        let array = c.g[V0][0];
        let mut link = Addr(ava).off(NODES).lw(c);
        c.g[A1][0] = link;
        c.g[A0][0] = 0;
        let empty = link == 0;
        Addr(ava).off(ARRAY).sw(c, array);
        if !empty {
            c.cycles += 1;
            let mut i = 0u32;
            loop {
                c.cycles += 7;
                let base = Addr(ava).off(ARRAY).lw(c);
                let p = (i << 2).wrapping_add(base as u32);
                c.g[V0][0] = sx(p);
                c.g[V1][0] = base;
                Addr(p).sw(c, link);
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[A1][0] = link;
                let more = link != 0;
                i = i.wrapping_add(1);
                c.g[A0][0] = sx(i);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event(c, 0x002d_7ec8);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::{check, scramble, Rng, DATA};

    /// A chain of trigger nodes at DATA + 0x1000, each `{bit, count, time,
    /// next}`, with the header's `+0xac` pointing at the first. `mask` controls
    /// how many counts are non-zero.
    fn build_triggers(c: &mut Cpu, r: &mut Rng, seq: u32, mask: u64) {
        let n = (r.next_u32() % 4) as u32;
        let mut node = seq + 0x1000;
        c.bus.write32(seq + TRIGGERS, if n == 0 { 0 } else { node });
        for i in 0..n {
            c.bus.write32(node + trigger::BIT, r.next_u32() % 64);
            let count = if mask >> i & 1 != 0 { r.next_u32() % 3 + 1 } else { 0 };
            c.bus.write32(node + trigger::COUNT, count);
            c.bus.write32(node + trigger::TIME, r.edge_f32());
            c.bus.write32(node + trigger::NEXT, if i + 1 < n { node + 0x20 } else { 0 });
            node += 0x20;
        }
    }

    #[test]
    fn anim_sequence_fire_end_triggers_matches() {
        check_events(
            ssx3_ee::s10::f_003139a8,
            anim_sequence_fire_end_triggers,
            60,
            &|c, seed| {
                let mut r = Rng(0x3139_a800 ^ seed as u64);
                scramble(c, &mut r);
                let seq = DATA;
                c.g[A0][0] = seq as u64;
                c.bus.write64(seq + MASK_A, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                c.bus.write64(seq + MASK_B, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                let mask = (r.next_u32() as u64) | 1;
                build_triggers(c, &mut r, seq, mask);
                let mode = schedule(c, &mut r, seed, 0x0031_39a8);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn anim_sequence_fire_triggers_at_time_matches() {
        check_events(
            ssx3_ee::s10::f_00313938,
            anim_sequence_fire_triggers_at_time,
            60,
            &|c, seed| {
                let mut r = Rng(0x3139_3800 ^ seed as u64);
                scramble(c, &mut r);
                let seq = DATA;
                c.g[A0][0] = seq as u64;
                c.bus.write64(seq + MASK_A, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                c.bus.write64(seq + MASK_B, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                let mask = (r.next_u32() as u64) | 1;
                build_triggers(c, &mut r, seq, mask);
                // Half the seeds match a node's time exactly.
                let target = if seed % 2 == 0 {
                    let first = c.bus.read32(seq + TRIGGERS);
                    if first != 0 { c.bus.read32(first + trigger::TIME) } else { r.edge_f32() }
                } else {
                    r.edge_f32()
                };
                c.f[12] = target;
                let mode = schedule(c, &mut r, seed, 0x0031_3938);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn anim_sequence_fire_triggers_in_range_matches() {
        check_events(
            ssx3_ee::s10::f_00313868,
            anim_sequence_fire_triggers_in_range,
            80,
            &|c, seed| {
                let mut r = Rng(0x3138_6800 ^ seed as u64);
                scramble(c, &mut r);
                let seq = DATA;
                c.g[A0][0] = seq as u64;
                c.bus.write64(seq + MASK_A, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                c.bus.write64(seq + MASK_B, ((r.next_u32() as u64) << 32) | r.next_u32() as u64);
                let mask = (r.next_u32() as u64) | 1;
                build_triggers(c, &mut r, seq, mask);
                let (lo, hi) = match seed % 4 {
                    0 => (1.0f32.to_bits(), 1.0f32.to_bits()),
                    1 => (0.0f32.to_bits(), 10.0f32.to_bits()),
                    2 => (10.0f32.to_bits(), 0.0f32.to_bits()),
                    _ => (r.f32(10.0), r.f32(10.0)),
                };
                c.f[12] = lo;
                c.f[13] = hi;
                let mode = schedule(c, &mut r, seed, 0x0031_3868);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn anim_seq_list_append_matches() {
        check_events(
            ssx3_ee::s10::f_00314368,
            anim_seq_list_append,
            80,
            &|c, seed| {
                let mut r = Rng(0x3143_6800 ^ seed as u64);
                scramble(c, &mut r);
                let list = DATA;
                c.g[A0][0] = list as u64;
                let seq = DATA + 0x800;
                c.g[A1][0] = seq as u64;
                // Always three or more links so every loop reaches the
                // back-edge event check.
                let n = 3 + (r.next_u32() % 3);
                let mut node = DATA + 0x1000;
                c.bus.write32(list + LIST_HEAD, node);
                for i in 0..n {
                    c.bus.write32(node + SEQ_NEXT, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(list, r.next_u32());
                c.bus.write32(seq + SEQ_NEXT, r.next_u32());
                let mode = schedule(c, &mut r, seed, 0x0031_4368);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn anim_seq_list_append_short_matches() {
        check(ssx3_ee::s10::f_00314368, anim_seq_list_append, 80, &|c, seed| {
            let mut r = Rng(0x3143_68f0 ^ seed as u64);
            scramble(c, &mut r);
            let list = DATA;
            c.g[A0][0] = list as u64;
            let seq = DATA + 0x800;
            c.g[A1][0] = seq as u64;
            let n = r.next_u32() % 4;
            let mut node = DATA + 0x1000;
            c.bus.write32(list + LIST_HEAD, if n == 0 { 0 } else { node });
            for i in 0..n {
                c.bus.write32(node + SEQ_NEXT, if i + 1 < n { node + 0x10 } else { 0 });
                node += 0x10;
            }
            c.bus.write32(list, r.next_u32());
            c.bus.write32(seq + SEQ_NEXT, r.next_u32());
        });
    }

    #[test]
    fn anim_sequence_ctor_matches() {
        check_events(
            ssx3_ee::s10::f_00313508,
            anim_sequence_ctor,
            40,
            &|c, seed| {
                let mut r = Rng(0x3135_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                let mode = schedule(c, &mut r, seed, 0x0031_3508);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn audio_eval_curve5_matches() {
        check_events(
            ssx3_ee::s07::f_00290b58,
            audio_eval_curve5,
            120,
            &|c, seed| {
                let mut r = Rng(0x290b_5800 ^ seed as u64);
                scramble(c, &mut r);
                let curve = DATA;
                c.g[A0][0] = curve as u64;
                for i in 0..0x40 / 4 {
                    c.bus.write32(curve + 4 * i, r.edge_f32());
                }
                c.f[12] = r.edge_f32();
                let mode = schedule(c, &mut r, seed, 0x0029_0b58);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn async_file_assign_serial_matches() {
        check(ssx3_ee::s14::f_003def40, async_file_assign_serial, 100, &|c, seed| {
            let mut r = Rng(0x3def_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            let serial = match seed % 4 {
                0 => 0xffff_ff00,
                1 => 0,
                2 => r.next_u32(),
                _ => 0xffff_ffff,
            };
            c.bus.write32(ASYNC_SERIAL, serial);
        });
    }

    #[test]
    fn ava_spline_modifier_ctor_matches() {
        check(ssx3_ee::s11::f_00357750, ava_spline_modifier_ctor, 100, &|c, seed| {
            let mut r = Rng(0x3577_5000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let arg = DATA + 0x200;
            c.g[A1][0] = arg as u64;
            for i in 0..0x40 / 4 {
                c.bus.write32(arg + 0x10 + 4 * i, r.next_u32());
            }
        });
    }

    #[test]
    fn ava_spline_modifier_v01_matches() {
        check(ssx3_ee::s12::f_00361aa8, ava_spline_modifier_v01, 40, &|c, seed| {
            let mut r = Rng(0x361a_a800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() & !1) as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }

    #[test]
    fn anim_delta_dtor_matches() {
        check(ssx3_ee::s12::f_00361de0, anim_delta_dtor, 40, &|c, seed| {
            let mut r = Rng(0x361d_e000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            c.bus.write32(DATA + 0x44, 0);
        });
    }

    use crate::difftest::fakes::*;

    /// A node list at DATA + 0x1000, linked at +0xf4, returning the first node
    /// (or 0 for an empty list) and the target node (`node`).
    fn node_list(c: &mut Cpu, r: &mut Rng, ava: u32, found: bool) -> u64 {
        let n = (r.next_u32() % 4) as u32;
        let mut node = DATA + 0x1000;
        c.bus.write32(ava + 4, if n == 0 { 0 } else { node });
        let mut target = DATA + 0x900;
        for i in 0..n {
            if found && i == n - 1 {
                target = node;
            }
            c.bus.write32(node + 0xf4, if i + 1 < n { node + 0x10 } else { 0 });
            node += 0x10;
        }
        target as u64
    }

    #[test]
    fn avalanche_add_node_event_matches() {
        check_events(
            ssx3_ee::s09::f_002d9738,
            avalanche_add_node_event,
            80,
            &|c, seed| {
                let mut r = Rng(0x2d97_3800 ^ seed as u64);
                scramble(c, &mut r);
                let ava = DATA;
                c.g[A0][0] = ava as u64;
                c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
                let found = r.next_u32() % 2 == 0;
                let target = node_list(c, &mut r, ava, found);
                c.g[A2][0] = if seed % 7 == 0 { r.next_u32() as u64 } else { target };
                c.bus.write16(ava + 0xe, if seed % 5 == 0 { 0x40 } else { (r.next_u32() % 0x40) as u16 });
                for i in 0..0x40 {
                    c.bus.write16(ava + 0x110 + 2 * i, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x002d_9738);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn ava_spline_modifier_update_matches() {
        check(ssx3_ee::s11::f_00357820, ava_spline_modifier_update, 60, &|c, seed| {
            let mut r = Rng(0x3578_2000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x80, DATA + 0x100);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 0x40 + 4 * i, r.edge_f32());
            }
        });
    }

    #[test]
    fn attached_emitter_fx_destroy_emitter_matches() {
        covered(
            ssx3_ee::s09::f_002f2218,
            attached_emitter_fx_destroy_emitter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 21, 1);
                let this = ptr(&mut r);
                c.g[A0][0] = this as u64;
                let idx = r.next_u32() % 8;
                c.g[A1][0] = idx as u64;
                let obj = if r.next_u32() % 4 == 0 { 0 } else { ptr(&mut r) };
                c.bus.write32(this + 0x6c + idx * 4, obj);
                if obj != 0 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(obj + 0x1f8, vt);
                }
            },
        );
    }

    #[test]
    fn anim_object_get_node_position_matches() {
        covered(
            ssx3_ee::s11::f_0034e600,
            anim_object_get_node_position,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 22, 2);
                let obj = ptr(&mut r);
                c.g[A0][0] = obj as u64;
                let node = r.next_u32();
                let selected = if r.next_u32() % 2 == 0 { node } else { r.next_u32() };
                c.bus.write32(obj + 0x2c, selected);
                c.g[A1][0] = node as u64;
                c.g[A2][0] = ptr(&mut r) as u64;
                c.g[A3][0] = ptr(&mut r) as u64;
                let vt = vtable(c, &mut r, &[0xe8, 0x108]);
                c.bus.write32(obj + 0x20, vt);
            },
        );
    }

    #[test]
    fn attached_emitter_fx_free_type_data_matches() {
        check(ssx3_ee::s09::f_002f1810, attached_emitter_fx_free_type_data, 40, &|c, seed| {
            let mut r = Rng(0x2f18_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            let data = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
            c.bus.write32(crate::difftest::GAME_GP + 0xac0, data);
        });
    }

    /// Shared setup for the AttachedEmitterFX constructors: a global type
    /// data pointer already present, so `Init` skips loading it.
    #[test]
    fn attached_emitter_fx_ctor_matches() {
        check_events(ssx3_ee::s09::f_002f1850, attached_emitter_fx_ctor, 40, &|c, seed| {
            let mut r = Rng(0x2f18_5000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.bus.write32(crate::difftest::GAME_GP + 0xac4, 1);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            let mode = schedule(c, &mut r, seed, 0x002f_1850);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    #[test]
    fn attached_emitter_fx_ctor_2_matches() {
        check_events(ssx3_ee::s09::f_002f1890, attached_emitter_fx_ctor_2, 40, &|c, seed| {
            let mut r = Rng(0x2f18_9000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 8) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.bus.write32(crate::difftest::GAME_GP + 0xac4, 1);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            let mode = schedule(c, &mut r, seed, 0x002f_1890);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    #[test]
    fn attached_emitter_fx_init_matches() {
        check_events(ssx3_ee::s09::f_002f18d8, attached_emitter_fx_init, 40, &|c, seed| {
            let mut r = Rng(0x2f18_d800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            c.bus.write32(crate::difftest::GAME_GP + 0xac4, 1);
            c.bus.write32(crate::difftest::GAME_GP + 0xac0, DATA + 0x400);
            for i in 0..0x40 / 4 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
            let mode = schedule(c, &mut r, seed, 0x002f_18d8);
            if mode != Events::Off {
                c.next_event = c.cycles;
            }
            mode
        });
    }

    #[test]
    fn attached_emitter_fx_dtor_matches() {
        covered(
            ssx3_ee::s09::f_002f1940,
            attached_emitter_fx_dtor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 23, 10);
                let this = ptr(&mut r);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() & !1) as u64; // do not free
                c.bus.write32(crate::difftest::GAME_GP + 0xac4, 2); // keep type data
                for i in 0..10 {
                    let obj = if r.next_u32() % 4 == 0 { 0 } else { ptr(&mut r) };
                    c.bus.write32(this + 0x6c + 4 * i, obj);
                    if obj != 0 {
                        let vt = vtable(c, &mut r, &[0x20]);
                        c.bus.write32(obj + 0x1f8, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn avalanche_build_active_node_array_matches() {
        check_events(
            ssx3_ee::s09::f_002d7e60,
            avalanche_build_active_node_array,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 24, 0);
                let ava = DATA;
                c.g[A0][0] = ava as u64;
                let n = r.next_u32() % 4;
                let mut node = DATA + 0x1000;
                c.bus.write32(ava + 0x4, if n == 0 { 0 } else { node });
                for i in 0..n {
                    c.bus.write32(node + 0x2e4, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(ava + 0x8, r.next_u32());
                let mode = schedule(c, &mut r, seed, 0x002d_7e60);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }
}
