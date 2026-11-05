//! Staging wave 205 (oc242): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, addu, r32, reg, ret, set_fcc, set_fpr, set_reg};
use crate::call::{call_ptr, guest, sp, Callee, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_e058, board_wake_fx_advance_ring, "cBoardWakeFX_AdvanceRing"),
    (0x003c_8968, snd_ee_mix, "SND_EeMix"),
    (0x0013_5db0, rider_spin_sub_get_spin_progress, "RiderSpinSub_GetSpinProgress"),
    (0x0041_1438, floatdidf, "__floatdidf"),
    (0x0036_2cc8, defer_man_reset_frame, "DeferMan_ResetFrame"),
    (0x0012_f118, rider_prewind_sub_get_hud_angle, "RiderPrewindSub_GetHUDAngle"),
    (0x0036_1940, position_modifier_get_matrix, "PositionModifier_GetMatrix"),
    (0x0031_77f0, rand_next_main, "Rand_NextMain"),
    (0x0026_d2b0, replay_cache_record_frame, "ReplayCache_RecordFrame"),
    (0x002b_cc00, music_trigger_blend, "tWPIGD_MusicTrigger_Blend"),
];

// ---------------------------------------------------------------------------
// cBoardWakeFX_AdvanceRing (0x2de058)

/// cBoardWakeFX: the owner (rider) whose colour source sits at +0x6c0.
const WAKE_OWNER: u32 = 0x00;
/// Live segments in the ring (grows up to the capacity).
const WAKE_COUNT: u32 = 0x04;
/// Ring index of the newest segment.
const WAKE_HEAD: u32 = 0x08;
/// The main ring: `cap` entries of 0x30 bytes (`f32 pos; ... int r, g, b`
/// at +0x10/+0x14/+0x18).
const WAKE_RING: u32 = 0x10;
/// Four trail rings (pointers), mirrored from the main ring's head entry.
const WAKE_TRAILS: u32 = 0x14;
/// Animation phase, wrapped into [0, 4).
const WAKE_PHASE: u32 = 0x38;
/// Frame counter (an int, converted to float for the segment position).
const WAKE_FRAME: u32 = 0x40;
/// Current wake colour (four floats, clamped to [0, 1]).
const WAKE_COLOUR: u32 = 0xa0;
/// Ring capacity.
const WAKE_CAP: u32 = 0xb8;
/// Bytes per ring entry.
const RING_STRIDE: u32 = 0x30;
/// The colour source object inside the owner (a vtable at its start).
const OWNER_COLOUR_SOURCE: u32 = 0x6c0;
/// Its vtable slot returning the colour index.
const SLOT_COLOUR_INDEX: u32 = 0x38;
/// `.lit4` phase step per tick (1/30), at gp-0x3bdc.
const GP_WAKE_PHASE_STEP: u32 = 0x3bdc;
/// Colour table `D_004FA398`: 0xf0-byte records starting with four floats.
const WAKE_COLOUR_TABLE: u32 = 0x004f_a398;
const WAKE_COLOUR_STRIDE: u32 = 0xf0;
/// Loop head of the trail copy (event checks).
const WAKE_TRAIL_HEAD: u32 = 0x002d_e2c0;

/// `void cBoardWakeFX::AdvanceRing(cBoardWakeFX* fx)` (0x2de058): one tick
/// of a board's wake ribbon. Advances the phase (wrapping at 4), grows the
/// live count up to the capacity, steps the ring head back one slot, fetches
/// the owner's wake colour (a virtual call for the colour index into the
/// colour table), doubles and clamps it to [0, 1], then fills the new head
/// entry: position `frame / 2` folded into [-32, 32) per 64 and the colour
/// as 0..128 ints; copies that entry into the four trail rings and bumps the
/// frame counter.
pub fn board_wake_fx_advance_ring(c: &mut Cpu) {
    guest(c, |c| {
        // Phase += step; wrap at 4.
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let step = Addr(r32(c, GP).wrapping_sub(GP_WAKE_PHASE_STEP)).read_fpu(c);
        set_fpr(c, 1, step);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        set_reg(c, S0, reg(c, A0));
        let fx = r32(c, S0);
        let at = |o: u32| Addr(fx.wrapping_add(o));
        set_reg(c, AT, 0x4080_0000);
        let four = FpuF32(0x4080_0000);
        set_fpr(c, 2, four);
        let mut phase = at(WAKE_PHASE).read_fpu(c) + step;
        let wrap = four.le(phase);
        set_fcc(c, wrap);
        at(WAKE_PHASE).write_fpu(c, phase);
        if wrap {
            c.cycles += 2;
            phase = phase - four;
            at(WAKE_PHASE).write_fpu(c, phase);
        }
        set_fpr(c, 0, phase);

        // count = min(count + 1, cap).
        c.cycles += 5;
        let count = at(WAKE_COUNT).lw(c);
        let cap = at(WAKE_CAP).lw(c);
        set_reg(c, V1, count);
        set_reg(c, V0, addu(count, 1));
        if (count as i64) < (cap as i64) {
            c.cycles += 1;
            at(WAKE_COUNT).sw(c, reg(c, V0));
        }

        // head = (head + cap - 1) % cap.
        c.cycles += 4;
        let cap = at(WAKE_CAP).lw(c);
        let head = at(WAKE_HEAD).lw(c);
        set_reg(c, V1, cap);
        c.cycles += 43;
        let n = (head as u32).wrapping_add(cap as u32).wrapping_sub(1);
        set_reg(c, V0, sx(n));
        let owner = at(WAKE_OWNER).lw(c);
        set_reg(c, A1, owner);
        let (_, new_head) = abi::div(c, n, cap as u32);
        let source = (owner as u32).wrapping_add(OWNER_COLOUR_SOURCE);
        set_reg(c, A2, sx(source));
        set_reg(c, A0, sx(new_head));
        at(WAKE_HEAD).sw(c, sx(new_head));

        // The owner's colour index (virtual call).
        let vt = Addr(source).lw(c);
        set_reg(c, V1, vt);
        let slot = VSlot::read(c, vt, SLOT_COLOUR_INDEX);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(source)));
        call_ptr(c, slot.func as u32, 0x002d_e0ec)?;

        // Copy the table colour to the stack and double it.
        let index = r32(c, V0);
        set_reg(c, A0, WAKE_COLOUR_STRIDE as u64);
        set_reg(c, V1, 0x50_0000);
        let off = abi::mult(c, index, WAKE_COLOUR_STRIDE);
        set_reg(c, V0, sx(off));
        set_reg(c, V1, sx(WAKE_COLOUR_TABLE));
        let zero = FpuF32::ZERO;
        let one = FpuF32::ONE;
        set_fpr(c, 4, zero);
        set_reg(c, AT, 0x3f80_0000);
        set_fpr(c, 5, one);
        set_fpr(c, 6, zero);
        let entry = off.wrapping_add(WAKE_COLOUR_TABLE);
        set_reg(c, V0, sx(entry));
        c.ldl(V1, entry.wrapping_add(7));
        c.ldr(V1, entry);
        c.ldl(A0, entry.wrapping_add(0xf));
        c.ldr(A0, entry.wrapping_add(8));
        let s = sp(c);
        c.sdl(V1, s.wrapping_add(7));
        c.sdr(V1, s);
        c.sdl(A0, s.wrapping_add(0xf));
        c.sdr(A0, s.wrapping_add(8));
        let tmp = |o: u32| Addr(s.wrapping_add(o));
        let y = tmp(4).read_fpu(c);
        let x = tmp(0).read_fpu(c);
        let y2 = y + y;
        let z = tmp(8).read_fpu(c);
        let w = tmp(0xc).read_fpu(c);
        let x2 = x + x;
        let z2 = z + z;
        let w2 = w + w;
        tmp(4).write_fpu(c, y2);
        // Clamp each lane to [0, 1] (negative -> 0).
        let pos = zero.le(y2);
        set_fcc(c, pos);
        tmp(0).write_fpu(c, x2);
        tmp(8).write_fpu(c, z2);
        tmp(0xc).write_fpu(c, w2);
        let mut cy = zero;
        if pos {
            c.cycles += 1;
            cy = y2.min(one);
        }
        c.cycles += 4;
        let pos = zero.le(z2);
        set_fcc(c, pos);
        tmp(4).write_fpu(c, cy);
        let mut cz = zero;
        if pos {
            c.cycles += 1;
            cz = z2.min(one);
        }
        c.cycles += 4;
        let pos = zero.le(w2);
        set_fcc(c, pos);
        tmp(8).write_fpu(c, cz);
        let mut cw = zero;
        if pos {
            c.cycles += 1;
            cw = w2.min(one);
        }
        c.cycles += 4;
        let pos = zero.le(x2);
        set_fcc(c, pos);
        tmp(0xc).write_fpu(c, cw);
        let mut cx = zero;
        if pos {
            c.cycles += 1;
            cx = x2.min(one);
        }

        // Store the colour in the effect.
        c.cycles += 70;
        tmp(0).write_fpu(c, cx);
        set_reg(c, A0, RING_STRIDE as u64);
        set_reg(c, AT, 0x3f00_0000);
        let half = FpuF32(0x3f00_0000);
        set_reg(c, A3, RING_STRIDE as u64);
        c.ldl(V0, s.wrapping_add(7));
        c.ldr(V0, s);
        c.ldl(V1, s.wrapping_add(0xf));
        c.ldr(V1, s.wrapping_add(8));
        let col = fx.wrapping_add(WAKE_COLOUR);
        c.sdl(V0, col.wrapping_add(7));
        c.sdr(V0, col);
        c.sdl(V1, col.wrapping_add(0xf));
        c.sdr(V1, col.wrapping_add(8));
        set_reg(c, T0, 3);
        set_reg(c, AT, 0x3c80_0000);
        let sixty_fourth = FpuF32(0x3c80_0000);
        let trails = fx.wrapping_add(WAKE_TRAILS);
        set_reg(c, A2, sx(trails));

        // Head entry position: frame / 2, folded per 64 into [-32, 32).
        let head = at(WAKE_HEAD).lw(c);
        set_reg(c, V0, head);
        let frame_no = FpuF32::from_i32(at(WAKE_FRAME).read_u32(c) as i32);
        let ring = at(WAKE_RING).lw(c);
        set_reg(c, V1, ring);
        let off = abi::mult(c, head as u32, RING_STRIDE);
        set_reg(c, AT, 0x4280_0000);
        let sixty_four = FpuF32(0x4280_0000);
        let pos0 = frame_no * half;
        set_reg(c, AT, 0x4200_0000);
        let thirty_two = FpuF32(0x4200_0000);
        set_reg(c, AT, 0x4300_0000);
        let one_two_eight = FpuF32(0x4300_0000);
        let e = off.wrapping_add(ring as u32);
        set_reg(c, V0, sx(e));
        Addr(e).write_fpu(c, pos0);
        let head = at(WAKE_HEAD).lw(c);
        let ring = at(WAKE_RING).lw(c);
        set_reg(c, V0, ring);
        let off = abi::mult(c, head as u32, RING_STRIDE);
        let e = off.wrapping_add(ring as u32);
        set_reg(c, V1, sx(e));
        let pos = Addr(e).read_fpu(c);
        let scaled = pos * sixty_fourth;
        set_fpr(c, 2, scaled);
        let whole = scaled.to_i32();
        set_reg(c, V0, sx(whole as u32));
        let fold = FpuF32::from_i32(whole) * sixty_four + thirty_two;
        let pos = pos - fold;
        Addr(e).write_fpu(c, pos);

        // Colour channels as 0..128 ints at +0x10/+0x14/+0x18 (the head and
        // ring registers alternate between v0 and v1).
        let mut last = zero;
        for (k, (src, dst)) in [(0xa4u32, 0x10u32), (0xa8, 0x14), (0xac, 0x18)].into_iter().enumerate() {
            let (hreg, rreg) = if k == 1 { (V1, V0) } else { (V0, V1) };
            let head = at(WAKE_HEAD).lw(c);
            let chan = at(src).read_fpu(c);
            let off = abi::mult(c, head as u32, RING_STRIDE);
            let ring = at(WAKE_RING).lw(c);
            set_reg(c, rreg, ring);
            let v = chan * one_two_eight;
            let e = off.wrapping_add(ring as u32);
            set_reg(c, hreg, sx(e));
            Addr(e.wrapping_add(dst)).write_u32(c, v.to_i32() as u32);
            last = v;
        }
        set_fpr(c, 0, last);
        set_fpr(c, 1, FpuF32(last.to_i32() as u32));
        set_fpr(c, 3, one_two_eight);
        set_fpr(c, 4, thirty_two);
        set_fpr(c, 5, sixty_four);
        set_fpr(c, 6, cy);

        // Mirror the head entry into the four trail rings.
        let mut trail_at = trails;
        let mut left: u32 = 3;
        loop {
            c.cycles += 35;
            let head = at(WAKE_HEAD).lw(c);
            set_reg(c, V0, head);
            left = left.wrapping_sub(1);
            set_reg(c, T0, sx(left));
            let ring = at(WAKE_RING).lw(c);
            set_reg(c, V1, ring);
            let off = abi::mult(c, head as u32, RING_STRIDE);
            set_reg(c, V0, sx(off));
            let trail = Addr(trail_at).lw(c);
            set_reg(c, A0, trail);
            let src = off.wrapping_add(ring as u32);
            set_reg(c, V1, sx(src));
            let p = Addr(src).read_fpu(c);
            set_fpr(c, 0, p);
            let dst = off.wrapping_add(trail as u32);
            set_reg(c, V0, sx(dst));
            Addr(dst).write_fpu(c, p);
            // r, g, b: registers rotate (head, ring, off, src, dst).
            for (field, h, rg, sr, ds) in [
                (0x10u32, V1, A0, A0, V1),
                (0x14, A0, V1, V1, A0),
                (0x18, V1, A0, A0, V1),
            ] {
                let head = at(WAKE_HEAD).lw(c);
                set_reg(c, h, head);
                let ring = at(WAKE_RING).lw(c);
                set_reg(c, rg, ring);
                let off = abi::mult(c, head as u32, RING_STRIDE);
                set_reg(c, h, sx(off));
                let trail = Addr(trail_at).lw(c);
                set_reg(c, A1, trail);
                if field == 0x18 {
                    trail_at = trail_at.wrapping_add(4);
                    set_reg(c, A2, sx(trail_at));
                }
                let src = off.wrapping_add(ring as u32);
                set_reg(c, sr, sx(src));
                let v = Addr(src.wrapping_add(field)).lw(c);
                set_reg(c, V0, v);
                let dst = off.wrapping_add(trail as u32);
                set_reg(c, ds, sx(dst));
                Addr(dst.wrapping_add(field)).sw(c, v);
            }
            if (left as i32) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, WAKE_TRAIL_HEAD);
            }
        }

        // frame++.
        c.cycles += 7;
        let n = at(WAKE_FRAME).lw(c);
        frame.restore_ra(c, 0x10);
        set_reg(c, V0, addu(n, 1));
        at(WAKE_FRAME).sw(c, reg(c, V0));
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeMix (0x3c8968)

/// The EE software mixer state (`D_00515B40`).
const MIXER: u32 = 0x0051_5b40;
/// Number of output buses (one output pointer each).
const MIXER_BUS_COUNT: u32 = 0x5;
/// Bytes each output pointer advances per mixed block.
const MIXER_OUT_STEP: u32 = 0xc;
/// Samples per mixed block.
const MIX_BLOCK: u32 = 0x200;
const SND_EE_MIX_BLOCK: Callee = Callee {
    addr: 0x003c_85d0,
    f: ee::s14::SND_EeMixBlock,
};

/// `void SND_EeMix(int** outs, int len)` (0x3c8968): mix `len` samples in
/// blocks of 0x200. Copies the bus output pointers into a local array, then
/// per block calls `SND_EeMixBlock(array, min(len, 0x200))` and advances
/// every output pointer by the mixer's step (+0xc).
pub fn snd_ee_mix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        set_reg(c, V0, 0x51_0000);
        frame.save(c, S0, 0x60);
        let buses = Addr(MIXER + MIXER_BUS_COUNT).lbu(c);
        set_reg(c, V0, buses);
        set_reg(c, S0, reg(c, A1));
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);

        // outs[0..buses] -> the local array at sp.
        if buses != 0 {
            c.cycles += 2;
            set_reg(c, A2, buses);
            set_reg(c, V1, reg(c, SP));
            loop {
                c.cycles += 7;
                let (src, dst) = (r32(c, A0), r32(c, V1));
                let p = Addr(src).lw(c);
                set_reg(c, V0, p);
                set_reg(c, A2, addu(reg(c, A2), u32::MAX as u64));
                set_reg(c, A0, sx(src.wrapping_add(4)));
                Addr(dst).sw(c, p);
                let more = reg(c, A2) != 0;
                set_reg(c, V1, sx(dst.wrapping_add(4)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_8998);
                }
            }
        }

        // Blocks of up to 0x200 samples.
        c.cycles += 2;
        set_reg(c, V0, 0x51_0000);
        if reg(c, S0) as i64 > 0 {
            c.cycles += 5;
            set_reg(c, S3, MIX_BLOCK as u64);
            set_reg(c, S2, sx(MIXER));
            set_reg(c, S1, sx(MIXER));
            set_reg(c, V0, ((reg(c, S0) as i64) < 513) as u64);
            loop {
                c.cycles += 7;
                let n = if reg(c, V0) != 0 { reg(c, S0) } else { reg(c, S3) };
                set_reg(c, A1, n);
                set_reg(c, A0, reg(c, SP));
                SND_EE_MIX_BLOCK.call(c, 0x003c_89e0)?;
                let buses = Addr(r32(c, S2).wrapping_add(MIXER_BUS_COUNT)).lbu(c);
                set_reg(c, V0, buses);
                set_reg(c, A1, sx(r32(c, S0).wrapping_sub(MIX_BLOCK)));
                if buses != 0 {
                    // Advance every output pointer.
                    c.cycles += 3;
                    let mixer = r32(c, S1);
                    let step = Addr(mixer.wrapping_add(MIXER_OUT_STEP)).lhu(c);
                    set_reg(c, A0, step);
                    set_reg(c, V1, reg(c, SP));
                    let count = Addr(mixer.wrapping_add(MIXER_BUS_COUNT)).lbu(c);
                    set_reg(c, A2, count);
                    loop {
                        c.cycles += 7;
                        let at = r32(c, V1);
                        let p = Addr(at).lw(c);
                        set_reg(c, A2, addu(reg(c, A2), u32::MAX as u64));
                        let p = addu(p, reg(c, A0));
                        set_reg(c, V0, p);
                        Addr(at).sw(c, p);
                        let more = reg(c, A2) != 0;
                        set_reg(c, V1, sx(at.wrapping_add(4)));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_89f8);
                        }
                    }
                }
                c.cycles += 3;
                set_reg(c, S0, reg(c, A1));
                let more = reg(c, S0) as i64 > 0;
                set_reg(c, V0, ((reg(c, S0) as i64) < 513) as u64);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_89d0);
                }
            }
        }

        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderSpinSub_GetSpinProgress (0x135db0)

/// RiderSpinSub: spin start angle.
const SPIN_START: u32 = 0x10;
/// Current spin angle.
const SPIN_ANGLE: u32 = 0x18;
/// Spin rate (radians per tick).
const SPIN_RATE: u32 = 0x3c;
/// The rider.
const SPIN_RIDER: u32 = 0x58;
/// Rider: a float that, when positive, halves the spin progress scale.
const RIDER_SPIN_HALVE: u32 = 0x2ec;
/// Rider: player index and character (for the behaviour attribute).
const RIDER_PLAYER: u32 = 0x86c;
const RIDER_CHARACTER: u32 = 0xb34;
/// `.lit4`: 2*pi, ~0.5 (0.50006247), the attribute factor scale (~0.5003).
const GP_SPIN_TWO_PI: u32 = 0x7324;
const GP_SPIN_HALF: u32 = 0x7320;
const GP_SPIN_ATTRIB_SCALE: u32 = 0x731c;
const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};
const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};
const BE_CHAR_INT_GET_ATTRIB4_FACTOR: Callee = Callee {
    addr: 0x0014_95a8,
    f: ee::s01::BECharInt_GetAttrib4Factor,
};

/// `void RiderSpinSub::GetSpinProgress(RiderSpinSub* spin, float* done,
/// float* full)` (0x135db0): how far the current spin has turned in ticks
/// (`|angle - start|`, less one turn past 2*pi, over the rate) and how many
/// ticks a full turn takes (2*pi over the rate); both halved (x0.50006) when
/// the rider's +0x2ec is positive, then divided by `1 + 0.50026 *` the
/// character's spin attribute (`BECharInt_GetAttrib4Factor`).
pub fn rider_spin_sub_get_spin_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        let gp = r32(c, GP);
        let two_pi = Addr(gp.wrapping_sub(GP_SPIN_TWO_PI)).read_fpu(c);
        set_fpr(c, 3, two_pi);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        set_reg(c, S0, reg(c, A0));
        frame.save(c, S2, 0x10);
        set_reg(c, S1, reg(c, A1));
        frame.save_ra(c, 0);
        set_reg(c, S2, reg(c, A2));
        let spin = r32(c, S0);
        let (done, full) = (Addr(r32(c, S1)), Addr(r32(c, S2)));
        let start = Addr(spin.wrapping_add(SPIN_START)).read_fpu(c);
        let angle = Addr(spin.wrapping_add(SPIN_ANGLE)).read_fpu(c);
        let diff = angle - start;
        set_fpr(c, 0, diff);
        let mut turned = diff.abs();
        let over = two_pi.lt(turned);
        set_fcc(c, over);
        if over {
            turned = turned - two_pi;
        }
        set_fpr(c, 1, turned);

        c.cycles += 12;
        let rate = Addr(spin.wrapping_add(SPIN_RATE)).read_fpu(c);
        let zero = FpuF32::ZERO;
        set_fpr(c, 2, zero);
        let d = turned.div(rate);
        set_fpr(c, 0, d);
        done.write_fpu(c, d);
        let rate = Addr(spin.wrapping_add(SPIN_RATE)).read_fpu(c);
        let f = two_pi.div(rate);
        set_fpr(c, 1, f);
        full.write_fpu(c, f);
        let rider = Addr(spin.wrapping_add(SPIN_RIDER)).lw(c);
        set_reg(c, V0, rider);
        let halve = Addr((rider as u32).wrapping_add(RIDER_SPIN_HALVE)).read_fpu(c);
        set_fpr(c, 0, halve);
        let pos = zero.lt(halve);
        set_fcc(c, pos);
        if pos {
            c.cycles += 6;
            let h = Addr(gp.wrapping_sub(GP_SPIN_HALF)).read_fpu(c);
            set_fpr(c, 2, h);
            let f = f * h;
            set_fpr(c, 1, f);
            full.write_fpu(c, f);
            let d = done.read_fpu(c) * h;
            set_fpr(c, 0, d);
            done.write_fpu(c, d);
        }

        // The character's spin attribute.
        c.cycles += 28;
        BE_LIBRARY_GET_INSTANCE.call(c, 0x0013_5e44)?;
        set_reg(c, A0, reg(c, V0));
        set_reg(c, A1, 3);
        BE_LIBRARY_GET_INTERFACE.call(c, 0x0013_5e50)?;
        let rider = Addr(spin.wrapping_add(SPIN_RIDER)).lw(c);
        set_reg(c, V1, rider);
        set_reg(c, A0, reg(c, V0));
        let character = Addr((rider as u32).wrapping_add(RIDER_CHARACTER)).lw(c);
        set_reg(c, A2, character);
        let player = Addr((rider as u32).wrapping_add(RIDER_PLAYER)).lw(c);
        set_reg(c, A1, player);
        BE_CHAR_INT_GET_ATTRIB4_FACTOR.call(c, 0x0013_5e64)?;

        // Both divided by 1 + attribute * scale.
        let scale = Addr(gp.wrapping_sub(GP_SPIN_ATTRIB_SCALE)).read_fpu(c);
        set_fpr(c, 3, scale);
        set_reg(c, AT, 0x3f80_0000);
        let attrib = FpuF32(c.f[0]) * scale;
        let f = full.read_fpu(c);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0);
        let div = attrib + FpuF32::ONE;
        set_fpr(c, 0, div);
        let m = FpuF32::ONE.div(div);
        set_fpr(c, 2, m);
        let f = f * m;
        set_fpr(c, 1, f);
        full.write_fpu(c, f);
        frame.restore(c, S2, 0x10);
        let d = done.read_fpu(c) * m;
        set_fpr(c, 0, d);
        done.write_fpu(c, d);
        frame.restore(c, S1, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __floatdidf (0x411438)

const FLOATSIDF: Callee = Callee {
    addr: 0x0041_2550,
    f: ee::s15::__floatsidf,
};
const MULDF3: Callee = Callee {
    addr: 0x0041_1fd8,
    f: ee::s15::__muldf3,
};
const ADDDF3: Callee = Callee {
    addr: 0x0041_1f18,
    f: ee::s15::__adddf3,
};
/// 65536.0 and 2^32 as IEEE doubles (`0x81e0 << 47`, `0x83e0 << 47`).
const DF_TWO_16: u64 = 0x81e0 << 47;
const DF_TWO_32: u64 = 0x83e0 << 47;

/// `double __floatdidf(long long x)` (0x411438, libgcc soft-float): the
/// high word converted and scaled by 65536 twice, plus the low word
/// converted as signed and corrected by 2^32 when its top bit is set.
pub fn floatdidf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        let at = frame.at(c, 0);
        let v = reg(c, S0);
        c.sd(at, v);
        let x = reg(c, A0);
        set_reg(c, S0, x);
        let at = frame.at(c, 0x10);
        let v = reg(c, S1);
        c.sd(at, v);
        set_reg(c, S1, DF_TWO_16);
        set_reg(c, A0, ((x as i64) >> 32) as u64);
        frame.save_ra(c, 0x20);
        FLOATSIDF.call(c, 0x0041_1460)?;
        // hi * 65536 * 65536.
        set_reg(c, A1, reg(c, S1));
        set_reg(c, A0, reg(c, V0));
        MULDF3.call(c, 0x0041_146c)?;
        set_reg(c, A1, reg(c, S1));
        set_reg(c, A0, reg(c, V0));
        MULDF3.call(c, 0x0041_1478)?;
        set_reg(c, S1, reg(c, V0));
        // The low word, as a signed int.
        set_reg(c, V0, 0xffff_ffff);
        let lo = x as u32 as i32 as i64 as u64;
        set_reg(c, S0, lo);
        set_reg(c, A0, lo);
        FLOATSIDF.call(c, 0x0041_1498)?;
        set_reg(c, A0, reg(c, S1));
        if (lo as i64) < 0 {
            c.cycles += 5;
            set_reg(c, A1, DF_TWO_32);
            set_reg(c, A0, reg(c, V0));
            ADDDF3.call(c, 0x0041_14b0)?;
            set_reg(c, A0, reg(c, S1));
        }
        c.cycles += 7;
        set_reg(c, A1, reg(c, V0));
        ADDDF3.call(c, 0x0041_14bc)?;
        frame.restore_ra(c, 0x20);
        let at = frame.at(c, 0x10);
        let v = c.ld(at);
        set_reg(c, S1, v);
        let at = frame.at(c, 0);
        let v = c.ld(at);
        set_reg(c, S0, v);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DeferMan_ResetFrame (0x362cc8)

/// DeferMan (a ~430 KB frame record): the deferred item count.
const DEFER_COUNT: u32 = 0x0;
/// Copied from +0x57484 to +0x51480 (the frame's starting value).
const DEFER_RESET_FROM: u32 = 0x5_7484;
const DEFER_RESET_TO: u32 = 0x5_1480;
const DEFER_57494: u32 = 0x5_7494;
/// Two 0x400-byte buffers refilled from the scratchpad.
const DEFER_BUF_A: u32 = 0x6_74a0;
const DEFER_BUF_B: u32 = 0x6_78a0;
const DEFER_67CA4: u32 = 0x6_7ca4;
/// A 20-byte record reset from the template `D_00501420`.
const DEFER_RECORD: u32 = 0x6_9ca8;
const DEFER_RECORD_TEMPLATE: u32 = 0x0050_1420;
/// Fields relative to +0x68000 (as the original addresses them).
const DEFER_HI: u32 = 0x6_8000;
const DEFER_69CC0: u32 = 0x1cc0;
const DEFER_69CC4: u32 = 0x1cc4;
const DEFER_69CC8: u32 = 0x1cc8;
const DEFER_6AED0: u32 = 0x2ed0;
/// `InvalidDCache(from, to)` and `EA_MemSet` (cross-shard, through the table).
const INVALID_DCACHE: u32 = 0x0042_47d8;
const EA_MEMSET: u32 = 0x003e_6448;
const SPR_STREAM_DOWNLOAD: Callee = Callee {
    addr: 0x0038_f738,
    f: ee::s12::SprStream_Download,
};
/// `gp`-relative: the scratchpad stream's free pointer.
const GP_SPR_FREE: u32 = 0x2a94;

/// `void DeferMan::ResetFrame(DeferMan* dm)` (0x362cc8): start a new frame
/// of deferred work: zero the count, restore +0x51480 from +0x57484,
/// invalidate the data cache over both 0x400-byte buffers, clear 0x400
/// bytes of free scratchpad and DMA them over both buffers, reset the
/// 20-byte record from its template and the trailing fields (two -1, two 0).
pub fn defer_man_reset_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 72;
        let frame = Frame::open(c, 0x40);
        set_reg(c, V0, 0x5_0000);
        frame.save(c, S0, 0x30);
        set_reg(c, A1, 0x6_0000);
        frame.save(c, S1, 0x20);
        set_reg(c, S0, reg(c, A0));
        frame.save(c, S2, 0x10);
        let dm = r32(c, S0);
        let f = |o: u32| Addr(dm.wrapping_add(o));
        set_reg(c, V0, sx(dm.wrapping_add(0x5_0000)));
        frame.save_ra(c, 0);
        f(DEFER_COUNT).sw(c, 0);
        let buf_a = dm.wrapping_add(DEFER_BUF_A);
        set_reg(c, S2, sx(buf_a));
        let start = f(DEFER_RESET_FROM).lw(c);
        set_reg(c, V1, start);
        set_reg(c, A1, sx(dm.wrapping_add(DEFER_BUF_A + 0x3ff)));
        f(DEFER_57494).sw(c, 0);
        set_reg(c, A0, sx(buf_a));
        f(DEFER_RESET_TO).sw(c, start);
        call_ptr(c, INVALID_DCACHE, 0x0036_2d18)?;
        let buf_b = dm.wrapping_add(DEFER_BUF_B);
        set_reg(c, S1, sx(buf_b));
        set_reg(c, A1, sx(dm.wrapping_add(DEFER_BUF_B + 0x3ff)));
        set_reg(c, A0, sx(buf_b));
        call_ptr(c, INVALID_DCACHE, 0x0036_2d38)?;

        // Clear 0x400 bytes of scratchpad, DMA them over both buffers.
        let spr = Addr(r32(c, GP).wrapping_add(GP_SPR_FREE)).lw(c);
        set_reg(c, A0, spr);
        set_reg(c, A1, 0);
        set_reg(c, A2, 0x400);
        call_ptr(c, EA_MEMSET, 0x0036_2d48)?;
        set_reg(c, A0, sx(buf_a));
        set_reg(c, A1, 0x400);
        set_reg(c, A2, 2);
        SPR_STREAM_DOWNLOAD.call(c, 0x0036_2d58)?;
        set_reg(c, A0, sx(buf_b));
        set_reg(c, A2, 2);
        set_reg(c, A1, 0x400);
        SPR_STREAM_DOWNLOAD.call(c, 0x0036_2d68)?;

        // The record from its template, the trailing fields.
        let record = dm.wrapping_add(DEFER_RECORD);
        set_reg(c, A0, sx(record));
        set_reg(c, V0, sx(dm.wrapping_add(0x6_0000)));
        set_reg(c, V1, DEFER_HI as u64);
        f(DEFER_67CA4).sw(c, 0);
        let hi = dm.wrapping_add(DEFER_HI);
        set_reg(c, S0, sx(hi));
        set_reg(c, A1, 0x50_0000);
        let t = DEFER_RECORD_TEMPLATE;
        set_reg(c, T0, sx(t));
        c.ldl(V1, t + 7);
        c.ldr(V1, t);
        c.ldl(A2, t + 0xf);
        c.ldr(A2, t + 8);
        let last = Addr(t + 0x10).lw(c);
        set_reg(c, A3, last);
        c.sdl(V1, record.wrapping_add(7));
        c.sdr(V1, record);
        c.sdl(A2, record.wrapping_add(0xf));
        c.sdr(A2, record.wrapping_add(8));
        Addr(record.wrapping_add(0x10)).sw(c, last);
        set_reg(c, V0, u64::MAX);
        let h = |o: u32| Addr(hi.wrapping_add(o));
        h(DEFER_69CC4).sw(c, u64::MAX);
        h(DEFER_6AED0).sw(c, 0);
        h(DEFER_69CC0).sw(c, u64::MAX);
        h(DEFER_69CC8).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderPrewindSub_GetHUDAngle (0x12f118)

/// Rider (through the sub-state's first word): the stick's y and x.
const RIDER_STICK_Y: u32 = 0x2b0;
const RIDER_STICK_X: u32 = 0x2a4;
/// `.lit4`: the dead zone (0.1), the angles for x == 0 (y < 0, y >= 0), the
/// corrections for x < 0 (y > 0: added; else subtracted).
const GP_HUD_DEAD_ZONE: u32 = 0x7730;
const GP_HUD_DOWN: u32 = 0x772c;
const GP_HUD_UP: u32 = 0x7728;
const GP_HUD_ADD_PI: u32 = 0x7724;
const GP_HUD_SUB_PI: u32 = 0x7720;
/// `Math_Atan` (cross-shard, through the table): f0 = atan(f12).
const MATH_ATAN: u32 = 0x0031_c228;

/// `float RiderPrewindSub::GetHUDAngle(RiderPrewindSub* sub, int* centred)`
/// (0x12f118): the stick direction for the prewind HUD. Each stick axis
/// under 0.1 counts as 0; with both 0, `*centred = 1` and 0 is returned.
/// Otherwise `*centred = 0` and the angle is `atan2(y, x)` built from
/// `Math_Atan(y / x)` with the quadrant corrections (x == 0: the up or down
/// angle).
pub fn rider_prewind_sub_get_hud_angle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let rider = Addr(r32(c, A0)).lw(c);
        set_reg(c, V0, rider);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x20);
        frame.save_fpr(c, 21, 0x18);
        frame.save_fpr(c, 20, 0x10);
        let gp = r32(c, GP);
        let lit = |c: &mut Cpu, o: u32| Addr(gp.wrapping_sub(o)).read_fpu(c);
        let dead = lit(c, GP_HUD_DEAD_ZONE);
        set_fpr(c, 1, dead);
        let rider = rider as u32;
        let mut y = Addr(rider.wrapping_add(RIDER_STICK_Y)).read_fpu(c);
        set_fpr(c, 0, y.abs());
        let small = abi::c_lt_s(c, y.abs(), dead);
        let mut x = Addr(rider.wrapping_add(RIDER_STICK_X)).read_fpu(c);
        let zero = FpuF32::ZERO;
        if small {
            c.cycles += 1;
            y = zero;
        }
        c.cycles += 4;
        set_fpr(c, 0, x.abs());
        if abi::c_lt_s(c, x.abs(), dead) {
            c.cycles += 1;
            x = zero;
        }
        set_fpr(c, 20, y);
        set_fpr(c, 21, x);

        // Centred: both axes zero.
        c.cycles += 5;
        set_fpr(c, 0, zero);
        let mut centred = 0u64;
        let y_zero = y.eq_s(zero);
        set_fcc(c, y_zero);
        if y_zero {
            c.cycles += 4;
            let x_zero = x.eq_s(zero);
            set_fcc(c, x_zero);
            if x_zero {
                centred = 1;
            }
        }
        set_reg(c, V0, centred);
        c.cycles += 2;
        Addr(r32(c, A1)).sw(c, centred);
        if centred != 0 {
            c.cycles += 3;
            set_fpr(c, 0, zero);
            frame.restore_ra(c, 0);
            c.cycles += 1;
            frame.restore_fpr(c, 22, 0x20);
        } else {
            c.cycles += 4;
            set_fpr(c, 22, zero);
            let x_zero = x.eq_s(zero);
            set_fcc(c, x_zero);
            if x_zero {
                // Straight up or down.
                c.cycles += 4;
                let y_zero = y.eq_s(zero);
                set_fcc(c, y_zero);
                set_fpr(c, 0, y);
                if y_zero {
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                    c.cycles += 1;
                    frame.restore_fpr(c, 22, 0x20);
                } else {
                    c.cycles += 4;
                    let up = zero.le(y);
                    set_fcc(c, up);
                    let down = lit(c, GP_HUD_DOWN);
                    set_fpr(c, 0, down);
                    frame.restore_ra(c, 0);
                    if !up {
                        c.cycles += 1;
                        frame.restore_ra(c, 0);
                        c.cycles += 1;
                        frame.restore_fpr(c, 22, 0x20);
                    } else {
                        c.cycles += 3;
                        let up = lit(c, GP_HUD_UP);
                        set_fpr(c, 0, up);
                        frame.restore_fpr(c, 22, 0x20);
                    }
                }
            } else {
                // atan(y / x), corrected into the left half-plane.
                c.cycles += 7;
                set_fpr(c, 12, y.div(x));
                call_ptr(c, MATH_ATAN, 0x0012_f1e4)?;
                let left = abi::c_lt_s(c, x, zero);
                let mut angle = FpuF32(c.f[0]);
                set_fpr(c, 12, angle);
                if left {
                    c.cycles += 3;
                    if abi::c_lt_s(c, zero, y) {
                        c.cycles += 3;
                        let k = lit(c, GP_HUD_ADD_PI);
                        set_fpr(c, 0, k);
                        angle = angle + k;
                    } else {
                        c.cycles += 2;
                        let k = lit(c, GP_HUD_SUB_PI);
                        set_fpr(c, 0, k);
                        angle = angle - k;
                    }
                    set_fpr(c, 12, angle);
                }
                c.cycles += 1;
                set_fpr(c, 0, angle);
                c.cycles += 1;
                frame.restore_ra(c, 0);
                c.cycles += 1;
                frame.restore_fpr(c, 22, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore_fpr(c, 21, 0x18);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PositionModifier_GetMatrix (0x361940)

/// PositionModifier: non-zero when the matrix is stale.
const POSMOD_DIRTY: u32 = 0x44;
/// The matrix.
const POSMOD_MATRIX: u32 = 0x50;
/// Vtable slot that rebuilds the matrix.
const SLOT_POSMOD_UPDATE: u32 = 0x18;

/// `Matrix* PositionModifier::GetMatrix(PositionModifier* m)` (0x361940):
/// rebuilds the matrix through the virtual update (slot 0x18) when the
/// dirty word (+0x44) is set, and returns the matrix at +0x50.
pub fn position_modifier_get_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        set_reg(c, S0, reg(c, A0));
        let m = r32(c, S0);
        let dirty = Addr(m.wrapping_add(POSMOD_DIRTY)).lw(c);
        set_reg(c, V0, sx(m.wrapping_add(POSMOD_MATRIX)));
        if dirty != 0 {
            c.cycles += 6;
            let vt = Addr(m).lw(c);
            set_reg(c, V0, vt);
            let slot = VSlot::read(c, vt, SLOT_POSMOD_UPDATE);
            set_reg(c, V1, slot.func);
            set_reg(c, A0, slot.this(sx(m)));
            call_ptr(c, slot.func as u32, 0x0036_1970)?;
            set_reg(c, V0, sx(m.wrapping_add(POSMOD_MATRIX)));
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rand_NextMain (0x3177f0)

/// The game's main random generator state (`0x500000 - 0xfe8`).
const RAND_MAIN: u32 = 0x004f_f018;
const RAND_NEXT: Callee = Callee {
    addr: 0x0031_7a08,
    f: ee::s10::Rand_Next,
};

/// `unsigned BXrand(void)` (0x3177f0): the next number of the main random
/// generator (`Rand_Next(&main_rand)`).
pub fn rand_next_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        set_reg(c, A0, 0x50_0000);
        frame.save_ra(c, 0);
        set_reg(c, A0, sx(RAND_MAIN));
        RAND_NEXT.call(c, 0x0031_7804)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ReplayCache_RecordFrame (0x26d2b0)

/// ReplayCache: number of chunks.
const REPLAY_COUNT: u32 = 0x0;
/// The chunk array: 8 bytes each, the low 12 bits of the first word a
/// repeat count.
const REPLAY_CHUNKS: u32 = 0xc;
const REPLAY_REPEAT_MASK: u64 = 0xfff;
/// `memcmp` (cross-shard, through the table).
const MEMCMP: u32 = 0x0041_5fc8;
const REPLAY_CACHE_PUSH_FRAME: Callee = Callee {
    addr: 0x0026_d228,
    f: ee::s07::ReplayCache_PushFrame,
};

/// `int ReplayCache::RecordFrame(ReplayCache* cache, const u64* frame)`
/// (0x26d2b0): run-length records one 8-byte frame. If the last chunk
/// (repeat count masked out) equals the frame and its count is below 0xfff,
/// the count goes up; otherwise the frame is pushed as a new chunk
/// (`ReplayCache_PushFrame`). Returns 1.
pub fn replay_cache_record_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        set_reg(c, S0, reg(c, A0));
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        let cache = r32(c, S0);
        let count = Addr(cache.wrapping_add(REPLAY_COUNT)).lw(c);
        set_reg(c, V1, count);
        set_reg(c, S1, reg(c, A1));
        let mut extended = false;
        if count as i64 <= 0 {
            c.cycles += 3;
            set_reg(c, A0, reg(c, S0));
            set_reg(c, A1, reg(c, S1));
            REPLAY_CACHE_PUSH_FRAME.call(c, 0x0026_d370)?;
            extended = true;
        } else {
            // Compare the last chunk, repeat count cleared, with the frame.
            c.cycles += 18;
            let chunks = Addr(cache.wrapping_add(REPLAY_CHUNKS)).lw(c);
            set_reg(c, V0, chunks);
            let last = (count as u32).wrapping_sub(1);
            set_reg(c, V1, sx(last));
            let off = last << 3;
            set_reg(c, S2, sx(off));
            set_reg(c, A3, !REPLAY_REPEAT_MASK);
            let entry = off.wrapping_add(chunks as u32);
            set_reg(c, V0, sx(entry));
            let s = sp(c);
            set_reg(c, A0, reg(c, SP));
            c.ldl(V1, entry.wrapping_add(7));
            c.ldr(V1, entry);
            c.sdl(V1, s.wrapping_add(7));
            c.sdr(V1, s);
            let key = c.ld(s) & !REPLAY_REPEAT_MASK;
            set_reg(c, A1, reg(c, S1));
            set_reg(c, A2, 8);
            set_reg(c, V0, key);
            c.sd(s, key);
            call_ptr(c, MEMCMP, 0x0026_d314)?;
            let differs = reg(c, V0) != 0;
            set_reg(c, A0, reg(c, S0));
            if !differs {
                c.cycles += 7;
                let chunks = Addr(cache.wrapping_add(REPLAY_CHUNKS)).lw(c);
                set_reg(c, V0, chunks);
                let at = sx(r32(c, S2).wrapping_add(chunks as u32));
                set_reg(c, A1, at);
                let word = Addr(at as u32).lw(c);
                set_reg(c, A0, word);
                let repeat = word & REPLAY_REPEAT_MASK;
                let room = (repeat as i64) < 0xfff;
                set_reg(c, V0, room as u64);
                set_reg(c, V1, addu(repeat, 1));
                if room {
                    c.cycles += 6;
                    let v = (word & !REPLAY_REPEAT_MASK) | (reg(c, V1) & REPLAY_REPEAT_MASK);
                    set_reg(c, V0, v);
                    set_reg(c, V1, reg(c, V1) & REPLAY_REPEAT_MASK);
                    Addr(at as u32).sw(c, v);
                    extended = true;
                } else {
                    c.cycles += 1;
                    set_reg(c, A0, reg(c, S0));
                }
            }
            if !extended {
                c.cycles += 4;
                set_reg(c, A1, reg(c, S1));
                REPLAY_CACHE_PUSH_FRAME.call(c, 0x0026_d35c)?;
                frame.restore(c, S0, 0x40);
            }
        }
        if extended {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        set_reg(c, V0, 1);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// tWPIGD_MusicTrigger_Blend (0x2bcc00)

/// Music trigger: vtable pointer at +4 (gcc 2.9x layout).
const MUSIC_TRIGGER_VT: u32 = 0x4;
/// Two blended integer parameters, each followed by its last target.
const MUSIC_TRIGGER_A: u32 = 0x8;
const MUSIC_TRIGGER_A_TARGET: u32 = 0xc;
const MUSIC_TRIGGER_B: u32 = 0x10;
const MUSIC_TRIGGER_B_TARGET: u32 = 0x14;
/// The source trigger's parameters.
const MUSIC_SOURCE_A: u32 = 0x4;
const MUSIC_SOURCE_B: u32 = 0x8;
/// The vtable slot called before and after the blend.
const SLOT_MUSIC_TRIGGER_SYNC: u32 = 0x218;

/// `void tWPIGD_MusicTrigger::Blend(tWPIGD_MusicTrigger* t, const Source* p,
/// float f)` (0x2bcc00): with `w = f * f`, sets both integer parameters to
/// `(int)(w * p.x + (1 - w) * t.x)`, remembers `p.x` as their targets, and
/// calls the virtual slot 0x218 before and after.
pub fn music_trigger_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        set_reg(c, S0, reg(c, A0));
        frame.save_ra(c, 0);
        set_reg(c, S1, reg(c, A1));
        frame.save_fpr(c, 20, 0x30);
        let (t, p) = (r32(c, S0), r32(c, S1));
        let tf = |o: u32| Addr(t.wrapping_add(o));
        let pf = |o: u32| Addr(p.wrapping_add(o));
        let f = abi::farg(c, 0);
        let w = f * f;
        set_fpr(c, 20, w);
        let vt = tf(MUSIC_TRIGGER_VT).lw(c);
        set_reg(c, V1, vt);
        let slot = VSlot::read(c, vt, SLOT_MUSIC_TRIGGER_SYNC);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(t)));
        call_ptr(c, slot.func as u32, 0x002b_cc38)?;

        set_reg(c, AT, 0x3f80_0000);
        let int_f = |c: &mut Cpu, a: Addr| FpuF32::from_i32(a.read_u32(c) as i32);
        let to_a = int_f(c, pf(MUSIC_SOURCE_A));
        let cur_a = int_f(c, tf(MUSIC_TRIGGER_A));
        let vt = tf(MUSIC_TRIGGER_VT).lw(c);
        set_reg(c, V1, vt);
        let keep = FpuF32::ONE - w;
        let new_a = w * to_a;
        let cur_b = int_f(c, tf(MUSIC_TRIGGER_B));
        set_fpr(c, 2, cur_b);
        let old_a = keep * cur_a;
        let old_b = keep * cur_b;
        set_fpr(c, 3, old_b);
        let a = new_a + old_a;
        let a_bits = a.to_i32() as u32;
        set_fpr(c, 1, FpuF32(a_bits));
        tf(MUSIC_TRIGGER_A).write_u32(c, a_bits);
        let target = pf(MUSIC_SOURCE_A).lw(c);
        tf(MUSIC_TRIGGER_A_TARGET).sw(c, target);
        let to_b = int_f(c, pf(MUSIC_SOURCE_B));
        let b = w * to_b + old_b;
        set_fpr(c, 20, b);
        let b_bits = b.to_i32() as u32;
        set_fpr(c, 0, FpuF32(b_bits));
        tf(MUSIC_TRIGGER_B).write_u32(c, b_bits);
        let target = pf(MUSIC_SOURCE_B).lw(c);
        set_reg(c, V0, target);
        tf(MUSIC_TRIGGER_B_TARGET).sw(c, target);
        let slot = VSlot::read(c, vt, SLOT_MUSIC_TRIGGER_SYNC);
        set_reg(c, V0, slot.func);
        set_reg(c, A0, slot.this(sx(t)));
        call_ptr(c, slot.func as u32, 0x002b_ccb0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{fill, put_words, Rng, DATA, GAME_GP};

    /// `cBoardWakeFX_AdvanceRing`: an effect at DATA with phase 0..4.5 (wraps
    /// or not), count 0..9 against capacity 1..8 (grows or not), head below
    /// the capacity, a ring and four trail rings of random words, a random
    /// frame counter. The owner's colour source has a vtable whose colour
    /// index slot is a fake (forced index 1..12, sometimes unwinding); the
    /// colour table records hold floats in [-1.5, 1.5] or edge values, so
    /// the doubled lanes hit all three clamp cases.
    #[test]
    fn board_wake_fx_advance_ring_matches() {
        const FX: u32 = DATA;
        const OWNER: u32 = DATA + 0x1000;
        const VT: u32 = DATA + 0x3000;
        const RING: u32 = DATA + 0x4000;
        const TRAIL: u32 = DATA + 0x5000;
        covered(
            ee::s09::f_002de058__generated,
            board_wake_fx_advance_ring,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2de058, 1);
                c.pc = 0x002d_e058;
                let step = if seed % 2 == 0 { 0x3d08_8889 } else { r.f32(1.0) };
                put_words(c, GAME_GP - GP_WAKE_PHASE_STEP, &[step]);
                fill(c, &mut r, FX, 0x30);
                let cap = 1 + r.next_u32() % 8;
                c.bus.write32(FX + WAKE_OWNER, OWNER);
                c.bus.write32(FX + WAKE_COUNT, r.next_u32() % 10);
                c.bus.write32(FX + WAKE_HEAD, r.next_u32() % cap);
                c.bus.write32(FX + WAKE_RING, RING);
                for i in 0..4 {
                    c.bus.write32(FX + WAKE_TRAILS + 4 * i, TRAIL + 0x200 * i);
                }
                let phase = (r.next_u32() % 4500) as f32 / 1000.0;
                c.bus.write32(FX + WAKE_PHASE, phase.to_bits());
                c.bus.write32(FX + WAKE_FRAME, if seed % 5 == 0 { r.next_u32() } else { r.next_u32() % 100_000 });
                c.bus.write32(FX + WAKE_CAP, cap);
                fill(c, &mut r, RING, 0x60);
                fill(c, &mut r, TRAIL, 0x200);
                c.bus.write32(OWNER + OWNER_COLOUR_SOURCE, VT);
                c.bus.write16(VT + SLOT_COLOUR_INDEX, (r.next_u32() as u16) & 0xfffc);
                c.bus.write32(VT + SLOT_COLOUR_INDEX + 4, FAKE);
                let index = 1 + r.next_u32() % 12;
                c.bus.write32(LOG_RESULTS, index);
                for k in 0..4 {
                    let v = if seed % 7 == 0 { r.edge_f32() } else { r.f32(1.5) };
                    c.bus.write32(WAKE_COLOUR_TABLE + index * WAKE_COLOUR_STRIDE + 4 * k, v);
                }
                c.g[A0][0] = sx(FX);
            },
        );
    }
    /// `SND_EeMix`: the mixer at 0x515b40 with 0-2 buses (output pointers
    /// copied, then advanced by a random step after every block), no voices,
    /// the declick bus off, declick gains zero, the post-mix hook set or not,
    /// each bus with or without an effect and an output stage (fakes that log
    /// and sometimes unwind); `SND_EeMixBlock` runs for real. Lengths -3..0x700
    /// (no block, one partial, several full and a partial).
    #[test]
    fn snd_ee_mix_matches() {
        covered(
            ee::s14::f_003c8968__generated,
            snd_ee_mix,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c8968, 4);
                c.pc = 0x003c_8968;
                let m = MIXER;
                let fake = |k: u32| FAKE + 0x10 * k;
                let objs = DATA + 0x6000;
                let obj = |c: &mut Cpu, k: u32, f: u32| -> u32 {
                    let o = objs + 0x10 * k;
                    c.bus.write32(o, f);
                    o
                };
                let buses = seed % 3;
                c.bus.write8(m + 4, 0);
                c.bus.write8(m + MIXER_BUS_COUNT, buses as u8);
                c.bus.write16(m + MIXER_OUT_STEP, r.next_u32() as u16);
                c.bus.write8(m + 0xf, 0);
                c.bus.write32(m + 8, fake(7));
                c.bus.write32(m + 0x100, 0);
                c.bus
                    .write32(m + 0x10, if r.next_u32() % 2 == 0 { 0 } else { fake(6) });
                c.bus.write32(m + 0xb0, DATA + 0x2000);
                c.bus.write32(m + 0xb4, DATA + 0x3000);
                c.bus.write32(m + 0x1e0, fake(5));
                c.bus.write32(m + 0x1dc, DATA + 0x4000);
                for i in 0..buses {
                    c.bus.write32(m + 0xd0 + 4 * i, DATA + 0x1000 + 0x800 * i);
                    c.bus.write32(m + 0xe8 + 4 * i, 0);
                    let fx = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        obj(c, i, fake(1))
                    };
                    c.bus.write32(m + 0x11c + 4 * i, fx);
                    let out = obj(c, 2 + i, fake(2));
                    c.bus.write32(m + 0x104 + 4 * i, out);
                }
                let outs = DATA + 0x5000;
                fill(c, &mut r, outs, 4);
                c.g[A0][0] = sx(outs);
                let len = match seed % 5 {
                    0 => (r.next_u32() % 4) as i32 - 3,
                    1 => 0x200,
                    2 => 1 + (r.next_u32() % 0x200) as i32,
                    _ => (r.next_u32() % 0x700) as i32,
                };
                c.g[A1][0] = len as i64 as u64;
            },
        );
    }
    /// `RiderSpinSub_GetSpinProgress`: spin angles in [-10, 10] (the turn
    /// is past 2*pi or not), rates 0.05..0.5 or edge values, the rider's
    /// halving float positive, zero or negative, `done` and `full` distinct
    /// or the same pointer. The behaviour library exists (gp-0x1f04); its
    /// interface getter is a fake (sometimes unwinding);
    /// `BECharInt_GetAttrib4Factor` runs for real on the game image
    /// (player 0, characters 1..5).
    #[test]
    fn rider_spin_sub_get_spin_progress_matches() {
        const SPIN: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const OUT: u32 = DATA + 0x3000;
        const BE_LIBRARY: u32 = DATA + 0x5000;
        const BE_OBJ: u32 = DATA + 0x5100;
        const BE_VT: u32 = DATA + 0x5200;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        covered(
            ee::s01::f_00135db0__generated,
            rider_spin_sub_get_spin_progress,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x135db0, 1);
                c.pc = 0x0013_5db0;
                crate::difftest::elf_image(c);
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_INTERFACES, BE_OBJ);
                c.bus.write32(BE_OBJ + 4, BE_VT);
                c.bus.write32(BE_VT + 0xc, FAKE);
                c.bus.write8(0x0053_08e2, 10);
                c.bus.write8(0x0053_08e4, 10);
                c.bus.write32(SPIN + SPIN_START, r.f32(10.0));
                c.bus.write32(SPIN + SPIN_ANGLE, r.f32(10.0));
                let rate = if seed % 9 == 0 {
                    r.edge_f32()
                } else {
                    (0.05 + (r.next_u32() % 450) as f32 / 1000.0).to_bits()
                };
                c.bus.write32(SPIN + SPIN_RATE, rate);
                c.bus.write32(SPIN + SPIN_RIDER, RIDER);
                let halve = match seed % 3 {
                    0 => 0,
                    1 => 1.0f32.to_bits(),
                    _ => r.f32(2.0),
                };
                c.bus.write32(RIDER + RIDER_SPIN_HALVE, halve);
                c.bus.write32(RIDER + RIDER_PLAYER, 0);
                c.bus.write32(RIDER + RIDER_CHARACTER, 1 + r.next_u32() % 5);
                c.g[A0][0] = sx(SPIN);
                c.g[A1][0] = sx(OUT);
                c.g[A2][0] = sx(if seed % 7 == 0 { OUT } else { OUT + 4 });
            },
        );
    }
    /// `__floatdidf`: 64-bit values with every combination of high-word
    /// sign and low-word top bit, zero, small numbers, the extremes; the
    /// soft-float callees run for real.
    #[test]
    fn floatdidf_matches() {
        crate::difftest::check(ee::s15::f_00411438__generated, floatdidf, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x411438, 0);
            c.pc = 0x0041_1438;
            let hi = r.next_u32() as u64;
            let lo = r.next_u32() as u64;
            let x = match seed % 8 {
                0 => 0,
                1 => lo & 0x7fff_ffff,
                2 => (lo | 0x8000_0000) & 0xffff_ffff,
                3 => (-(lo as i64 & 0xffff)) as u64,
                4 => i64::MIN as u64,
                5 => i64::MAX as u64,
                _ => hi << 32 | lo,
            };
            c.g[A0][0] = x;
        });
    }
    /// `DeferMan_ResetFrame`: a DeferMan at DATA with random words in the
    /// fields it touches, a random template record, the scratchpad stream's
    /// free pointer leaving 0x400..0x4000 free bytes and random scratchpad
    /// contents. The real InvalidDCache, EA_MemSet and SprStream_Download
    /// run (DMA from the scratchpad, waiting).
    #[test]
    fn defer_man_reset_frame_matches() {
        const DM: u32 = DATA;
        crate::difftest::check(ee::s12::f_00362cc8__generated, defer_man_reset_frame, 200, &|c, seed| {
            let mut r = Rng(0x0036_2cc8 ^ (seed as u64) << 17);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.pc = 0x0036_2cc8;
            let free = 0x400 + (r.next_u32() % 0x3c1) * 0x10;
            c.bus.write32(GAME_GP + GP_SPR_FREE, 0x7000_4000 - free);
            for i in 0..0x1000 {
                c.bus.spr[i * 4..i * 4 + 4].copy_from_slice(&r.next_u32().to_le_bytes());
            }
            for o in [DEFER_COUNT, DEFER_RESET_FROM, DEFER_RESET_TO, DEFER_57494, DEFER_67CA4] {
                c.bus.write32(DM + o, r.next_u32());
            }
            fill(c, &mut r, DM + DEFER_BUF_A, 0x200);
            fill(c, &mut r, DM + DEFER_RECORD, 8);
            fill(c, &mut r, DEFER_RECORD_TEMPLATE, 5);
            for o in [DEFER_69CC0, DEFER_69CC4, DEFER_69CC8, DEFER_6AED0] {
                c.bus.write32(DM + DEFER_HI + o, r.next_u32());
            }
            c.g[A0][0] = sx(DM);
        });
    }
    /// `RiderPrewindSub_GetHUDAngle`: each stick axis zero, inside the dead
    /// zone (either sign), or anywhere in [-2, 2] (or an edge value), so
    /// every quadrant, the x == 0 up/down cases and the centred case occur;
    /// the real `Math_Atan` runs on the game image's constants.
    #[test]
    fn rider_prewind_sub_get_hud_angle_matches() {
        const SUB: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const OUT: u32 = DATA + 0x2000;
        crate::difftest::check(
            ee::s00::f_0012f118__generated,
            rider_prewind_sub_get_hud_angle,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12f118, 0);
                crate::difftest::elf_image(c);
                c.pc = 0x0012_f118;
                let axis = |r: &mut Rng, k: u32| match k % 6 {
                    0 => 0,
                    1 => r.f32(0.099),
                    2 => 0x8000_0000,
                    3 => r.edge_f32(),
                    _ => r.f32(2.0),
                };
                let y = axis(&mut r, seed);
                let x = axis(&mut r, seed / 6);
                c.bus.write32(SUB, RIDER);
                c.bus.write32(RIDER + RIDER_STICK_Y, y);
                c.bus.write32(RIDER + RIDER_STICK_X, x);
                fill(c, &mut r, OUT, 1);
                c.g[A0][0] = sx(SUB);
                c.g[A1][0] = sx(OUT);
            },
        );
    }
    /// `PositionModifier_GetMatrix`: the dirty word zero (no call) or not;
    /// the update slot is a fake (logs, sometimes unwinds).
    #[test]
    fn position_modifier_get_matrix_matches() {
        const M: u32 = DATA;
        const VT: u32 = DATA + 0x1000;
        covered(
            ee::s12::f_00361940__generated,
            position_modifier_get_matrix,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361940, 1);
                c.pc = 0x0036_1940;
                c.bus.write32(M, VT);
                c.bus.write32(M + POSMOD_DIRTY, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write16(VT + SLOT_POSMOD_UPDATE, (r.next_u32() as u16) & 0x00fc);
                c.bus.write32(VT + SLOT_POSMOD_UPDATE + 4, FAKE);
                c.g[A0][0] = sx(M);
            },
        );
    }
    /// `Rand_NextMain`: random generator state words (indices and table
    /// entries); the real `Rand_Next` runs.
    #[test]
    fn rand_next_main_matches() {
        crate::difftest::check(ee::s10::f_003177f0__generated, rand_next_main, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x3177f0, 0);
            c.pc = 0x0031_77f0;
            fill(c, &mut r, RAND_MAIN, 0x10);
        });
    }
    /// `ReplayCache_RecordFrame`: chunk counts -1..12 (zero or less: pushed
    /// at once), the last chunk's repeat count anywhere up to 0xfff, the
    /// frame equal to the last chunk (repeat bits ignored) or differing in
    /// one byte; the real `memcmp` and `ReplayCache_PushFrame` run.
    #[test]
    fn replay_cache_record_frame_matches() {
        const CACHE: u32 = DATA;
        const CHUNKS: u32 = DATA + 0x1000;
        const NEW: u32 = DATA + 0x2000;
        crate::difftest::check(
            ee::s07::f_0026d2b0__generated,
            replay_cache_record_frame,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26d2b0, 0);
                c.pc = 0x0026_d2b0;
                let count = match seed % 4 {
                    0 => (r.next_u32() % 2) as i32 - 1,
                    _ => 1 + (r.next_u32() % 12) as i32,
                };
                c.bus.write32(CACHE + REPLAY_COUNT, count as u32);
                c.bus.write32(CACHE + REPLAY_CHUNKS, CHUNKS);
                fill(c, &mut r, CHUNKS, 32);
                let last = CHUNKS + 8 * (count.max(1) as u32 - 1);
                let repeat = match (seed / 4) % 4 {
                    0 => 0xfff,
                    1 => 0xffe,
                    _ => r.next_u32() & 0xfff,
                };
                let w0 = c.bus.read32(last) & !0xfff | repeat;
                c.bus.write32(last, w0);
                let mut frame = c.bus.read64(last) & !0xfff;
                frame |= (r.next_u32() & 0xfff) as u64;
                frame &= !0xfffu64;
                if seed % 3 == 0 {
                    frame ^= 1u64 << (12 + r.next_u32() % 52);
                }
                c.bus.write64(NEW, frame);
                c.g[A0][0] = sx(CACHE);
                c.g[A1][0] = sx(NEW);
            },
        );
    }
    /// `tWPIGD_MusicTrigger_Blend`: blend factors in [0, 1] or edge values,
    /// random integer parameters (small or full range) on both sides, the
    /// source sometimes the trigger itself; the slot 0x218 method is a fake
    /// (logs, sometimes unwinds on either call).
    #[test]
    fn music_trigger_blend_matches() {
        const T: u32 = DATA;
        const SRC: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x1000;
        covered(ee::s08::f_002bcc00__generated, music_trigger_blend, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2bcc00, 2);
            c.pc = 0x002b_cc00;
            let int = |r: &mut Rng| {
                if r.next_u32() % 4 == 0 {
                    r.next_u32()
                } else {
                    ((r.next_u32() % 2000) as i32 - 1000) as u32
                }
            };
            c.bus.write32(T + MUSIC_TRIGGER_VT, VT);
            for o in [MUSIC_TRIGGER_A, MUSIC_TRIGGER_A_TARGET, MUSIC_TRIGGER_B, MUSIC_TRIGGER_B_TARGET] {
                let v = int(&mut r);
                c.bus.write32(T + o, v);
            }
            let src = if seed % 11 == 0 { T + 4 } else { SRC };
            for o in [MUSIC_SOURCE_A, MUSIC_SOURCE_B] {
                let v = int(&mut r);
                c.bus.write32(SRC + o, v);
            }
            c.bus.write16(VT + SLOT_MUSIC_TRIGGER_SYNC, (r.next_u32() as u16) & 0x00fc);
            c.bus.write32(VT + SLOT_MUSIC_TRIGGER_SYNC + 4, FAKE);
            c.f[12] = if seed % 5 == 0 {
                r.edge_f32()
            } else {
                ((r.next_u32() % 1001) as f32 / 1000.0).to_bits()
            };
            c.g[A0][0] = sx(T);
            c.g[A1][0] = sx(src);
        });
    }
}
