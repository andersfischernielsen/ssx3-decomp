//! Staging wave 211 (oc297): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Direct callees of `RiderMetrix_LandTrick` (same shard; through the
/// override slots).
mod land_trick_callee {
    use super::*;

    pub const FINISH_TRICK: Callee = Callee {
        addr: 0x0011_a228,
        f: ee::s00::RiderMetrix_FinishTrick,
    };

    pub const RESET_TRICK: Callee = Callee {
        addr: 0x0011_7838,
        f: ee::s00::RiderMetrix_ResetTrick,
    };
}

/// RiderMetrix trick record fields written on landing.
const METRIX_STANCE: u32 = 0x0;
const METRIX_ALTERNATE: u32 = 0x4;
const METRIX_STYLE: u32 = 0xc;
const METRIX_FLAG: u32 = 0x10;
const METRIX_SCORE: u32 = 0x14;
const METRIX_LAST_STYLE: u32 = 0x20;
const METRIX_LAST_STYLE_TIME: u32 = 0x24;
const METRIX_LAST_FLAG: u32 = 0x28;
const METRIX_LAST_FLAG_TIME: u32 = 0x2c;
const METRIX_CHAIN_ACTIVE: u32 = 0x5c;
const METRIX_CHAIN_TIME: u32 = 0x6c;
const METRIX_CHAIN: u32 = 0x70;

/// `float RiderMetrix_LandTrick(RiderMetrix* m, int stance, int alternate,
/// int style, int flag)` (0x119d40): a styled landing with a positive score
/// adds the style bonus (`gp-0x7a00`, 0.13) to the score; then the trick is
/// banked (`RiderMetrix_FinishTrick(m, stance, alternate, style, flag, 0)`)
/// and reset, and the landing's stance/alternate/style/flag are recorded
/// (style and flag also as "last" with their timers cleared). A trick chain
/// (`+0x70`) running before the landing survives the reset and is marked
/// active again. Returns FinishTrick's points.
pub fn rider_metrix_land_trick(c: &mut Cpu) {
    use land_trick_callee as f;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x70);
        c.g[S0][0] = c.g[A0][0]; // m
        c.g[S1][0] = c.g[A3][0]; // style
        c.g[S2][0] = c.g[T0][0]; // flag
        c.g[S4][0] = c.g[A1][0]; // stance
        c.g[S5][0] = c.g[A2][0]; // alternate
        let m = r32(c, S0);

        // Style bonus on a positive score.
        if c.g[S1][0] == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 6;
            c.lwc1(1, m + METRIX_SCORE);
            c.mtc1(0, 0);
            c.c_lt_s(0, 1);
            if c.fcc() {
                c.cycles += 4;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x7a00));
                c.add_s(0, 1, 0);
                c.swc1(0, m + METRIX_SCORE);
            }
        }
        c.g[A0][0] = c.g[S0][0];

        c.cycles += 15;
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = 0; // takeoff
        c.g[S3][0] = Addr(m + METRIX_CHAIN).lw(c);
        f::FINISH_TRICK.call(c, 0x0011_9dc0)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        f::RESET_TRICK.call(c, 0x0011_9dcc)?;

        let m = r32(c, S0);
        Addr(m + METRIX_STANCE).sw(c, c.g[S4][0]);
        Addr(m + METRIX_ALTERNATE).sw(c, c.g[S5][0]);
        Addr(m + METRIX_STYLE).sw(c, c.g[S1][0]);
        Addr(m + METRIX_FLAG).sw(c, c.g[S2][0]);
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            Addr(m + METRIX_LAST_STYLE).sw(c, c.g[S1][0]);
            Addr(m + METRIX_LAST_STYLE_TIME).sw(c, 0);
        }
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 2;
            Addr(m + METRIX_LAST_FLAG).sw(c, c.g[S2][0]);
            Addr(m + METRIX_LAST_FLAG_TIME).sw(c, 0);
        }
        c.cycles += 2;
        c.g[V0][0] = 1;
        if c.g[S3][0] != 0 {
            c.cycles += 3;
            Addr(m + METRIX_CHAIN).sw(c, c.g[S3][0]);
            Addr(m + METRIX_CHAIN_ACTIVE).sw(c, 1);
            Addr(m + METRIX_CHAIN_TIME).sw(c, 0);
        }

        c.cycles += 11;
        c.mov_s(0, 20);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// Callees of `RiderMetrix_AddTrick`.
mod add_trick_callee {
    use super::*;

    /// `cBE_getBE`, `BELibrary_GetInterface`, `BEBAGTInt_GetTrickPoints`,
    /// `BEBAGTInt_GetTrickPoints2` (other shard: through the function table).
    pub const BE_GET_BE: u32 = 0x0014_dc80;
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const GET_TRICK_POINTS: u32 = 0x0015_0528;
    pub const GET_TRICK_POINTS_2: u32 = 0x0015_0540;

    pub const CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::RiderMetrix_ClearPeakA4,
    };
}

/// RiderMetrix fields `RiderMetrix_AddTrick` updates.
const METRIX_TRICK_RATE: u32 = 0x3c;
const METRIX_TRICK_TIMER: u32 = 0x40;

/// `float RiderMetrix_AddTrick(RiderMetrix* m, int id)` (0x119708): look the
/// trick up in the BE trick table (interface 6): its points times 0.0001
/// (`gp-0x7a1c`) are added to the running score (`+0x14`), its second
/// value times 1/600000 (`gp-0x7a18`) becomes the rate at `+0x3c`; a
/// negative timer at `+0x40` is clamped to 0, an uber trick (id >= 35) sets
/// the chain flag `+0x5c`, then `+0xa4` is cleared. Returns 0.
pub fn rider_metrix_add_trick(c: &mut Cpu) {
    use add_trick_callee as f;
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // m
        c.g[S2][0] = c.g[A1][0]; // id
        call_ptr(c, f::BE_GET_BE, 0x0011_9730)?;
        c.mtc1(0, 21); // zero
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 6;
        call_ptr(c, f::BE_GET_INTERFACE, 0x0011_9740)?;
        c.g[S0][0] = c.g[V0][0]; // trick table interface
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, f::GET_TRICK_POINTS, 0x0011_9750)?;
        c.mtc1(2, 20);
        c.cvt_s_w(20, 20);
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7a1c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        c.mul_s(20, 20, 0); // points * 0.0001
        call_ptr(c, f::GET_TRICK_POINTS_2, 0x0011_976c)?;

        let m = r32(c, S1);
        c.mtc1(2, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, m + METRIX_SCORE);
        c.lwc1(2, r32(c, GP).wrapping_sub(0x7a18));
        c.lwc1(3, m + METRIX_TRICK_TIMER);
        c.add_s(0, 0, 20);
        c.mul_s(1, 1, 2);
        c.c_lt_s(3, 21);
        c.swc1(0, m + METRIX_SCORE);
        c.swc1(1, m + METRIX_TRICK_RATE);
        if c.fcc() {
            c.cycles += 1;
            c.swc1(21, m + METRIX_TRICK_TIMER);
        }

        c.cycles += 3;
        let uber = (c.g[S2][0] as i64) >= 35;
        c.g[V0][0] = 1;
        if uber {
            c.cycles += 1;
            Addr(m + METRIX_CHAIN_ACTIVE).sw(c, 1);
        }

        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        f::CLEAR_PEAK_A4.call(c, 0x0011_97b4)?;
        c.mov_s(0, 21);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Callees of `SND_EeVoiceStart`.
mod voice_start_callee {
    use super::*;

    /// `SND_MemAlloc` (other shard: through the function table).
    pub const MEM_ALLOC: u32 = 0x003b_5c60;

    pub const DSP_INSERT_SORTED: Callee = Callee {
        addr: 0x003c_8f48,
        f: ee::s14::SND_DspInsertSorted,
    };

    pub const DSP_STRETCH_INIT: Callee = Callee {
        addr: 0x003c_b280,
        f: ee::s14::SND_DspStretch_Init,
    };
}

/// The sound driver's state block (`D_00515B40`): the voice entries
/// (`+0x1dc`, 0x60 bytes each), and per sample format (6 rows of 3) the
/// DSP start functions (`+0x18`) and object sizes (`+0x60`).
const VOICE_STATE: u32 = 0x0051_5b40;
const VOICE_STATE_ENTRIES: u32 = 0x1dc;
const VOICE_STATE_START_FNS: u32 = 0x18;
const VOICE_STATE_SIZES: u32 = 0x60;
/// The global flag byte passed in the start parameters (`D_0050AA62`).
const VOICE_START_FLAG: u32 = 0x0050_aa62;

/// Voice entry fields.
const VOICE_ACTIVE: u32 = 0x0;
const VOICE_DSP_LIST: u32 = 0x40;
const VOICE_RESULT: u32 = 0x44;
const VOICE_OBJ: u32 = 0x48;
const VOICE_STREAM: u32 = 0x4c;

/// `void SND_EeVoiceStart(int id, int fmt, int ch, int a, int b, int strm,
/// int c, int g, int d, int f, ..., int e, int prio)` (0x3c8158): reset voice
/// `id`'s DSP chain; pick the start function for the sample format (8, 10,
/// 4, 22, 16, 23 -> rows 0..5) and channel mode (mono: 0/1 by `f >= 1`;
/// stereo 2; other 0), and if there is one allocate its DSP object
/// (`SND_MemAlloc(size)`, priority 0xf0 / `prio`), call it with the start
/// parameters on the stack and insert it into the chain; with a stream,
/// allocate a time-stretch object (0x1828 bytes, priority 0xc8), initialise
/// it for the stream (voice -1 unless mono-0) and insert it too. Marks the
/// voice active.
pub fn snd_ee_voice_start(c: &mut Cpu) {
    use voice_start_callee as f;
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save(c, S2, 0xa0);
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A0][0]; // id
        c.g[V1][0] = 0x60;
        c.mult(0, V1, S1, V1); // id * sizeof(entry)
        c.g[T5][0] = 0x51_0000;
        c.g[T4][0] = 1;
        c.g[S3][0] = c.g[A2][0]; // ch
        c.g[S6][0] = c.g[A3][0];
        c.g[S7][0] = c.g[T0][0];
        c.g[FP][0] = c.g[T1][0]; // strm
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        let entries = Addr(VOICE_STATE + VOICE_STATE_ENTRIES).lw(c);
        c.g[S0][0] = sx((entries as u32).wrapping_add(c.g[V1][0] as u32));
        let ent = r32(c, S0);
        Addr(ent + VOICE_DSP_LIST).sw(c, 0);
        Addr(ent + VOICE_OBJ).sw(c, 0);
        Addr(ent + 0x50).sw(c, 0);
        Addr(ent + 0x58).sw(c, 0);
        Addr(ent + 0x5c).sw(c, 0);
        Addr(ent + VOICE_STREAM).sw(c, 0);
        Addr(sp(c) + 0x18).sw(c, 1); // p.one
        Addr(sp(c) + 0x28).sw(c, 0); // p.result

        // The format row (a0; -1 for an unknown format).
        let fmt = c.g[A1][0];
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = 8;
        if fmt == 8 {
            c.cycles += 2;
            c.g[A0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 4;
            if fmt == 0xa {
                c.cycles += 2;
                c.g[A0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x16;
                if fmt == 4 {
                    c.cycles += 2;
                    c.g[A0][0] = 2;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x10;
                    if fmt == 0x16 {
                        c.cycles += 2;
                        c.g[A0][0] = 3;
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = fmt ^ 0x17;
                        if fmt == 0x10 {
                            c.cycles += 2;
                            c.g[A0][0] = 4;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 5;
                            if c.g[V1][0] == 0 {
                                c.g[A0][0] = 5;
                            }
                        }
                    }
                }
            }
        }

        // The channel column (v1).
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[S3][0] == 1 {
            c.cycles += 5;
            let f_arg = Addr(sp(c) + 0xd8).lw(c);
            c.g[V0][0] = ((f_arg as i64) < 1) as u64;
            c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { c.g[S3][0] };
        } else {
            c.cycles += 1;
            c.g[V1][0] = if c.g[S3][0] != 0 { 0 } else { 2 };
        }

        // The format's start function.
        c.cycles += 10;
        c.g[V0][0] = 0xc;
        c.g[A1][0] = sx(VOICE_STATE);
        c.mult(0, V0, A0, V0); // row * 12
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = sx(VOICE_STATE + VOICE_STATE_START_FNS);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let start_fn = Addr(r32(c, S2)).lw(c);
        c.g[V0][0] = sx(VOICE_STATE.wrapping_add(c.g[V1][0] as u32));
        if start_fn != 0 {
            c.cycles += 34;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(VOICE_STATE_SIZES)).lw(c);
            call_ptr(c, f::MEM_ALLOC, 0x003c_8294)?;
            let ent = r32(c, S0);
            Addr(ent + VOICE_OBJ).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xf0;
            c.g[V0][0] = 0x51_0000;
            let p = sp(c);
            Addr(p).sw(c, c.g[S6][0]); // p.a
            c.g[V1][0] = Addr(VOICE_START_FLAG).lbu(c);
            c.g[A1][0] = c.g[SP][0]; // &p
            c.g[V0][0] = Addr(p + 0xf0).lw(c);
            Addr(p + 0x4).sw(c, c.g[S7][0]); // p.b
            Addr(p + 0x24).sw(c, c.g[V0][0]); // p.e
            c.g[V0][0] = Addr(p + 0xd0).lw(c);
            Addr(p + 0x8).sw(c, c.g[S4][0]); // p.c
            Addr(p + 0x10).sw(c, c.g[V0][0]); // p.d
            c.g[V0][0] = Addr(p + 0xd8).lw(c);
            Addr(p + 0xc).sw(c, c.g[S5][0]); // p.g
            Addr(p + 0x14).sw(c, c.g[V0][0]); // p.f
            Addr(p + 0x20).sw(c, c.g[V1][0]); // p.flag
            Addr(p + 0x1c).sw(c, c.g[S1][0]); // p.id
            c.g[V0][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(r32(c, V0) + 0x4).sw(c, 0);
            c.g[V1][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(r32(c, V1) + 0x18).sh(c, 0xf0);
            c.g[V1][0] = Addr(p + 0xf8).lw(c); // prio
            c.g[V0][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(r32(c, V0) + 0x1a).sb(c, c.g[V1][0]);
            c.g[V1][0] = Addr(r32(c, S2)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = Addr(ent + VOICE_OBJ).lw(c);
            call_ptr(c, target, 0x003c_8300)?;
            let ent = r32(c, S0);
            c.g[V0][0] = Addr(sp(c) + 0x28).lw(c);
            c.g[A0][0] = sx(ent + VOICE_DSP_LIST);
            c.g[A1][0] = Addr(ent + VOICE_OBJ).lw(c);
            Addr(ent + VOICE_RESULT).sw(c, c.g[V0][0]);
            f::DSP_INSERT_SORTED.call(c, 0x003c_8314)?;
        }

        // The stream's time-stretch stage.
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        if c.g[FP][0] != 0 {
            c.cycles += 18;
            c.g[A0][0] = 0x1828;
            if c.g[S3][0] != 0 {
                c.g[S1][0] = c.g[V0][0]; // voice -1
            }
            call_ptr(c, f::MEM_ALLOC, 0x003c_8328)?;
            let ent = r32(c, S0);
            Addr(ent + VOICE_STREAM).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc8;
            Addr(r32(c, V0) + 0x4).sw(c, 0);
            c.g[A1][0] = c.g[FP][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[V0][0] = Addr(ent + VOICE_STREAM).lw(c);
            Addr(r32(c, V0) + 0x18).sh(c, 0xc8);
            c.g[V0][0] = Addr(sp(c) + 0xf8).lw(c); // prio
            c.g[V1][0] = Addr(ent + VOICE_STREAM).lw(c);
            Addr(r32(c, V1) + 0x1a).sb(c, c.g[V0][0]);
            c.g[A0][0] = Addr(ent + VOICE_STREAM).lw(c);
            f::DSP_STRETCH_INIT.call(c, 0x003c_8358)?;
            let ent = r32(c, S0);
            c.g[A1][0] = Addr(ent + VOICE_STREAM).lw(c);
            c.g[A0][0] = sx(ent + VOICE_DSP_LIST);
            f::DSP_INSERT_SORTED.call(c, 0x003c_8364)?;
        }

        c.cycles += 14;
        c.g[V0][0] = 1;
        Addr(r32(c, S0) + VOICE_ACTIVE).sb(c, 1);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S0, 0xc0);
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

/// Callees of `ModObj_TakeOverObject` (same shard; through the override
/// slots).
mod take_over_callee {
    use super::*;

    pub const TAKE_PARTICLE_MODIFIERS: Callee = Callee {
        addr: 0x0035_6020,
        f: ee::s11::ModObj_TakeParticleModifiers,
    };

    pub const ADD_POSITION_MODIFIER_FROM_MATRIX: Callee = Callee {
        addr: 0x0035_5918,
        f: ee::s11::ModObj_AddPositionModifierFromMatrix,
    };
}

/// A virtual call through the vtable at `obj + 0xc` (entries
/// `{short delta; fn}`): a0 = obj + delta, `jalr fn`.
fn take_over_vcall(c: &mut Cpu, vt: u32, slot: u32, obj: u32, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = Addr(vt + slot).lh(c);
    c.g[V0][0] = Addr(vt + slot + 4).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void ModObj_TakeOverObject(ModObj* self, cMoveNode* src)` (0x355f10):
/// take over another node's place. If `src` has particle modifiers
/// (vtable +0x80) they move to `self`. If `src`'s world matrix (vtable
/// +0xc0) is `self`'s own transform (`self->+0x18 + 0x10`), `src` is just
/// destroyed (vtable +0x8, mode 3); otherwise the matrix is copied, `src`
/// destroyed (keeping `self`'s transform flags at `+0x18->+8`), a position
/// modifier built from the copy, `self+0x28` set, and `self`'s vtable
/// +0x190 called.
pub fn mod_obj_take_over_object(c: &mut Cpu) {
    use take_over_callee as f;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0]; // src
        frame.save_ra(c, 0x40);
        c.g[S1][0] = c.g[A0][0]; // self
        if c.g[S0][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        } else {
            // Particle modifiers.
            c.cycles += 7;
            let src = r32(c, S0);
            c.g[V1][0] = Addr(src + 0xc).lw(c);
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt + 0x80).lh(c);
            c.g[V0][0] = Addr(vt + 0x84).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(src.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0035_5f40)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                f::TAKE_PARTICLE_MODIFIERS.call(c, 0x0035_5f54)?;
            }
            let src = r32(c, S0);
            c.g[V1][0] = Addr(src + 0xc).lw(c);

            // The source's world matrix.
            c.cycles += 9;
            take_over_vcall(c, r32(c, V1), 0xc0, src, 0x0035_5f68)?;
            c.g[V1][0] = c.g[V0][0];
            let own = Addr(r32(c, S1) + 0x18).lw(c);
            c.g[V0][0] = sx((own as u32).wrapping_add(0x10));
            c.g[A1][0] = 3;
            if c.g[V1][0] == c.g[V0][0] {
                // Already ours: destroy the source.
                c.cycles += 7;
                let src = r32(c, S0);
                c.g[V0][0] = Addr(src + 0xc).lw(c);
                let vt = r32(c, V0);
                c.g[A0][0] = Addr(vt + 0x8).lh(c);
                c.g[V1][0] = Addr(vt + 0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx(src.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0035_5f90)?;
                frame.restore(c, S0, 0x60);
            } else {
                // Copy the matrix, destroy the source keeping our flags.
                c.cycles += 28;
                let m = r32(c, V1);
                let copy = sp(c);
                c.lqc2(1, m);
                c.lqc2(2, m + 0x10);
                c.lqc2(3, m + 0x20);
                c.lqc2(4, m + 0x30);
                c.sqc2(1, copy);
                c.sqc2(2, copy + 0x10);
                c.sqc2(3, copy + 0x20);
                c.sqc2(4, copy + 0x30);
                let src = r32(c, S0);
                c.g[V0][0] = Addr(src + 0xc).lw(c);
                let vt = r32(c, V0);
                c.g[A1][0] = 3;
                c.g[A2][0] = Addr(r32(c, S1) + 0x18).lw(c);
                c.g[A0][0] = Addr(vt + 0x8).lh(c);
                c.g[V1][0] = Addr(vt + 0xc).lw(c);
                c.g[A0][0] = sx(src.wrapping_add(c.g[A0][0] as u32));
                let target = r32(c, V1);
                c.g[S0][0] = Addr(r32(c, A2) + 0x8).lw(c); // kept flags
                call_ptr(c, target, 0x0035_5fd8)?;
                let me = r32(c, S1);
                c.g[V0][0] = Addr(me + 0x18).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                Addr(r32(c, V0) + 0x8).sw(c, c.g[S0][0]);
                f::ADD_POSITION_MODIFIER_FROM_MATRIX.call(c, 0x0035_5fec)?;
                let me = r32(c, S1);
                c.g[V0][0] = 1;
                c.g[V1][0] = Addr(me + 0xc).lw(c);
                Addr(me + 0x28).sw(c, 1);
                take_over_vcall(c, r32(c, V1), 0x190, me, 0x0035_6008)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Callees of `Emitter_Setup` (same shard; through the override slots).
mod emitter_setup_callee {
    use super::*;

    pub const INIT_DESCRIPTOR: Callee = Callee {
        addr: 0x0036_cbf8,
        f: ee::s12::Emitter_InitDescriptor,
    };

    pub const BUILD_DESCRIPTOR: Callee = Callee {
        addr: 0x0037_0058,
        f: ee::s12::Emitter_BuildDescriptor,
    };
}

/// `void Emitter::Setup(Matrix* m, EmitterDef* def)` (0x3705e0): copy the
/// definition's GS state (`+0xd0`, `+0xd4`) and texture ids (`+0xc4`,
/// `+0xc8`); the start time `self+0` is `def->start` (`+8`) when negative,
/// else `start + fade_in + fade / 2` (`+0x14`, `+0x1c`); then initialise
/// the particle descriptor at `self+0x10` (grid `+0`/`+4`, span = the
/// non-negative start or `fade_in + fade / 2`, scale `+0x20`, life `+0xc`,
/// velocity `+0x78..`) and build it for time -1 with bounds at `self+0x160`.
pub fn emitter_setup(c: &mut Cpu) {
    use emitter_setup_callee as f;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x60);
        c.mtc1(0, 1);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0]; // def
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x10);
        let me = r32(c, S2);
        let def = r32(c, S1);
        c.g[V0][0] = Addr(def + 0xd0).lw(c);
        Addr(me + 0x180).sw(c, c.g[V0][0]);
        c.lwc1(0, def + 0xd4);
        c.swc1(0, me + 0x188);
        c.g[V0][0] = Addr(def + 0xc4).lw(c);
        Addr(me + 0x4).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(def + 0xc8).lw(c);
        Addr(me + 0x8).sw(c, c.g[V1][0]);
        c.lwc1(3, def + 0x8);
        c.c_lt_s(3, 1); // start < 0
        c.g[S3][0] = c.g[A1][0]; // m
        if c.fcc() {
            c.cycles += 2;
            c.mov_s(0, 3);
        } else {
            c.cycles += 7;
            c.lwc1(0, def + 0x14);
            c.lwc1(1, def + 0x1c);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 2); // 0.5
            c.add_s(0, 3, 0);
            c.mul_s(1, 1, 2);
            c.add_s(0, 0, 1);
        }

        // The descriptor's span.
        c.cycles += 7;
        c.swc1(0, me);
        c.mtc1(0, 0);
        c.lwc1(12, def + 0x8);
        c.c_le_s(0, 12); // 0 <= start
        if !c.fcc() {
            c.cycles += 7;
            c.lwc1(0, def + 0x1c);
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 1); // 0.5
            c.lwc1(2, def + 0x14);
            c.mul_s(0, 0, 1);
            c.add_s(12, 2, 0);
        }
        c.lwc1(2, def + 0x78);

        c.cycles += 28;
        c.g[S0][0] = sx(me.wrapping_add(0x10)); // the descriptor
        c.lwc1(1, def + 0x7c);
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(0, def + 0x80);
        c.g[A3][0] = c.g[SP][0]; // &velocity
        c.g[A1][0] = Addr(def).lw(c);
        c.g[A2][0] = Addr(def + 0x4).lw(c);
        c.lwc1(13, def + 0x20);
        c.lwc1(14, def + 0xc);
        let v = sp(c);
        c.swc1(2, v);
        c.swc1(1, v + 0x4);
        c.swc1(0, v + 0x8);
        Addr(v + 0xc).sw(c, 0);
        f::INIT_DESCRIPTOR.call(c, 0x0037_06cc)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12); // -1.0
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = sx(r32(c, S2).wrapping_add(0x160)); // bounds
        f::BUILD_DESCRIPTOR.call(c, 0x0037_06e8)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Callees of `AnimObject_ctor`.
mod anim_ctor_callee {
    use super::*;

    pub const BASE_CTOR: Callee = Callee {
        addr: 0x0034_d9b0,
        f: ee::s11::AnimObject_BaseCtor,
    };

    pub const GET_DURATION: Callee = Callee {
        addr: 0x0035_1508,
        f: ee::s11::IntersectInfo_GetDuration,
    };

    /// `Rand_FloatRange`, `Rand_FloatSpread` (other shard: through the
    /// function table).
    pub const RAND_FLOAT_RANGE: u32 = 0x0031_7830;
    pub const RAND_FLOAT_SPREAD: u32 = 0x0031_7890;
}

/// AnimObject (float animator) fields.
const ANIM_ID: u32 = 0x0;
const ANIM_ACTIVE: u32 = 0x4;
const ANIM_AT_LIMIT: u32 = 0x8;
const ANIM_FC: u32 = 0xc;
const ANIM_SPEED: u32 = 0x10;
const ANIM_MIN: u32 = 0x14;
const ANIM_MAX: u32 = 0x18;
const ANIM_VALUE: u32 = 0x1c;
const ANIM_START: u32 = 0x20;
const ANIM_VTABLE: u32 = 0x3c;
const ANIM_FLAGS: u32 = 0x42;
const ANIM_INSTANCE: u32 = 0x64;
/// The AnimObject class vtable.
const ANIM_OBJECT_VTABLE: u32 = 0x0049_0b10;

/// `AnimObject* AnimObject_ctor(AnimObject* self, int list, int kind,
/// Node* node, AnimDef* def)` (0x341aa0): build the animated node at
/// `self+0x1c` (restorable when `def+0x24`), install the class vtable, then
/// set the float animator up from the definition: id (`+4`), min (`+0xc`
/// per 1/30 s, or the instance's duration when negative), max (`+0x10`, or
/// the node info's `+0x14` when negative), value (`+0x1c` per 1/30 s, the
/// min when negative; clamped to [min, max]; random in the range when
/// `+0x20`), speed (`+0x14` per 1/30 s, spread by `+0x18`, over the frame
/// rate at `gp+0x2a74 -> +0x10`; reversed when the direction `+8` is 1),
/// start = value, active, flag bit 0. Returns `self`.
pub fn anim_object_ctor(c: &mut Cpu) {
    use anim_ctor_callee as f;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0]; // node
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0]; // self
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0]; // def
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x1c));
        let restorable = Addr(r32(c, S2) + 0x24).lw(c);
        c.g[T0][0] = (0 < restorable) as u64;
        f::BASE_CTOR.call(c, 0x0034_1ad0)?;

        let me = r32(c, S1);
        c.g[A0][0] = Addr(me + ANIM_INSTANCE).lw(c);
        c.g[V0][0] = sx(ANIM_OBJECT_VTABLE);
        c.mtc1(0, 1);
        Addr(me + ANIM_VTABLE).sw(c, c.g[V0][0]);
        c.g[S0][0] = Addr(r32(c, S0) + 0x80).lw(c); // node info
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            f::GET_DURATION.call(c, 0x0034_1af4)?;
            c.mov_s(1, 0);
        }

        let me = r32(c, S1);
        let def = r32(c, S2);
        let info = r32(c, S0);
        let gp = r32(c, GP);
        // Id and min.
        c.cycles += 8;
        c.g[V0][0] = Addr(def + 0x4).lhu(c);
        c.mtc1(0, 0);
        Addr(me + ANIM_ID).sh(c, c.g[V0][0]);
        c.lwc1(2, def + 0xc);
        c.c_le_s(0, 2); // 0 <= min
        c.g[V1][0] = sx(me + ANIM_MIN);
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(0x2b94));
            c.mul_s(0, 2, 0);
        } else {
            c.cycles += 1;
            c.mov_s(0, 1); // the duration
        }

        // Max.
        c.cycles += 7;
        c.swc1(0, me + ANIM_MIN);
        c.mtc1(0, 0);
        c.lwc1(1, def + 0x10);
        c.c_lt_s(1, 0); // max < 0
        c.g[V0][0] = sx(me + ANIM_MAX);
        c.cycles += 2;
        if c.fcc() {
            c.lwc1(0, info + 0x14);
        } else {
            c.lwc1(0, gp.wrapping_sub(0x2b90));
            c.mul_s(0, 1, 0);
        }

        // Value, clamped.
        c.cycles += 7;
        c.swc1(0, me + ANIM_MAX);
        c.mtc1(0, 0);
        c.lwc1(1, def + 0x1c);
        c.c_lt_s(1, 0); // value < 0
        if !c.fcc() {
            c.lwc1(12, me + ANIM_MIN);
            c.cycles += 2;
            c.lwc1(0, gp.wrapping_sub(0x2b8c));
            c.mul_s(1, 1, 0);
        } else {
            c.cycles += 3;
            c.lwc1(1, me + ANIM_MIN);
            c.mov_s(12, 1);
        }
        c.cycles += 4;
        c.c_lt_s(1, 12);
        if c.fcc() {
            c.mov_s(1, 12);
        }
        c.cycles += 5;
        c.lwc1(13, me + ANIM_MAX);
        c.c_lt_s(13, 1);
        if c.fcc() {
            c.mov_s(1, 13);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(def + 0x20).lw(c);
        if c.g[V0][0] != 0 {
            // Random in [min, max] (f12, f13).
            c.cycles += 4;
            call_ptr(c, f::RAND_FLOAT_RANGE, 0x0034_1bbc)?;
            c.mov_s(1, 0);
        }
        let me = r32(c, S1);
        let def = r32(c, S2);
        c.swc1(1, me + ANIM_VALUE);

        // Speed.
        c.cycles += 6;
        c.mtc1(0, 0);
        c.lwc1(13, def + 0x18);
        c.c_eq_s(13, 0);
        if c.fcc() {
            c.lwc1(0, def + 0x14);
            c.cycles += 7;
            c.g[V0][0] = Addr(gp.wrapping_add(0x2a74)).lw(c);
            c.lwc1(1, gp.wrapping_sub(0x2b84));
            c.lwc1(2, r32(c, V0) + 0x10);
            c.cvt_s_w(2, 2);
            c.mul_s(0, 0, 1);
            c.div_s(0, 0, 2);
            c.swc1(0, me + ANIM_SPEED);
        } else {
            c.cycles += 11;
            c.lwc1(0, gp.wrapping_sub(0x2b88));
            c.lwc1(12, def + 0x14);
            c.mul_s(13, 13, 0);
            c.mul_s(12, 12, 0);
            call_ptr(c, f::RAND_FLOAT_SPREAD, 0x0034_1bf0)?;
            let gp = r32(c, GP);
            c.g[V0][0] = Addr(gp.wrapping_add(0x2a74)).lw(c);
            c.lwc1(1, r32(c, V0) + 0x10);
            c.cvt_s_w(1, 1);
            c.div_s(0, 0, 1);
            c.swc1(0, r32(c, S1) + ANIM_SPEED);
        }

        // Direction.
        let me = r32(c, S1);
        let def = r32(c, S2);
        c.cycles += 4;
        c.g[V0][0] = Addr(def + 0x8).lw(c);
        c.g[A0][0] = 1;
        if c.g[V0][0] == 1 {
            c.cycles += 4;
            c.lwc1(0, me + ANIM_SPEED);
            c.neg_s(0, 0);
            c.swc1(0, me + ANIM_SPEED);
        }
        c.g[V1][0] = Addr(me + ANIM_FLAGS).lhu(c);

        c.cycles += 14;
        c.g[V0][0] = c.g[S1][0];
        c.lwc1(0, me + ANIM_VALUE);
        c.g[V1][0] |= 1;
        Addr(me + ANIM_ACTIVE).sw(c, c.g[A0][0]);
        c.swc1(0, me + ANIM_START);
        Addr(me + ANIM_FLAGS).sh(c, c.g[V1][0]);
        Addr(me + ANIM_FC).sw(c, 0);
        Addr(me + ANIM_AT_LIMIT).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameState_SaveState` (same shard; through the override slot).
const EVENT_SELECT_BASE_SAVE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateEventSelect_SaveState(GameStateEventSelect* self,
/// Stream* stream)` (0x236728): save the base game state, then the four
/// selection words at `+0x10`, `+0x14`, `+0x8`, `+0x18` (4 bytes each,
/// through the stream's write method, vtable slot 1).
pub fn game_state_event_select_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // self
        EVENT_SELECT_BASE_SAVE.call(c, 0x0023_6744)?;
        for (field, ret_addr) in [
            (0x10u32, 0x0023_6760u32),
            (0x14, 0x0023_677c),
            (0x8, 0x0023_6798),
            (0x18, 0x0023_67b4),
        ] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            let vt = r32(c, V1);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(vt + 0x8).lh(c);
            c.g[V0][0] = Addr(vt + 0xc).lw(c);
            let write = r32(c, V0);
            c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, write, ret_addr)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Callees of `ParticleModifier_ctor`.
mod particle_modifier_ctor_callee {
    use super::*;

    pub const BASE_CTOR: Callee = Callee {
        addr: 0x0034_5798,
        f: ee::s11::ParticleModifier_BaseCtor,
    };

    /// `Emitter_ctor`, `Emitter_Setup` (other shard: through the function
    /// table).
    pub const EMITTER_CTOR: u32 = 0x0037_0018;
    pub const EMITTER_SETUP: u32 = 0x0037_05e0;
}

/// The ParticleModifier class vtable.
const PARTICLE_MODIFIER_VTABLE: u32 = 0x0049_12b0;
/// ParticleModifier fields.
const PMOD_OWNER: u32 = 0xc;
const PMOD_MATRIX: u32 = 0x10;
const PMOD_EMITTER: u32 = 0x50;
const PMOD_DEF: u32 = 0x1e0;
const PMOD_1E4: u32 = 0x1e4;

/// `ParticleModifier* ParticleModifier_ctor(void* mem, EmitterDef* def,
/// ModObj* owner)` (0x3458c0): the base modifier for `owner`, the class
/// vtable, an emitter at `+0x50`; the matrix at `+0x10` is the owner's
/// object's world matrix (its vtable +0xc0) or, without an object, the
/// owner's own matrix (`+0x10`); then the emitter is set up from `def` with
/// that matrix. Counts the instance (`gp+0xf08`). Returns `mem`.
pub fn particle_modifier_ctor(c: &mut Cpu) {
    use particle_modifier_ctor_callee as f;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // def
        c.g[S1][0] = c.g[A0][0]; // self
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0]; // owner
        f::BASE_CTOR.call(c, 0x0034_58e0)?;
        let me = r32(c, S1);
        c.g[V1][0] = sx(PARTICLE_MODIFIER_VTABLE);
        c.g[A0][0] = sx(me.wrapping_add(PMOD_EMITTER));
        Addr(me + 0x8).sw(c, c.g[V1][0]);
        call_ptr(c, f::EMITTER_CTOR, 0x0034_58f4)?;

        let me = r32(c, S1);
        c.g[V1][0] = Addr(me + PMOD_OWNER).lw(c);
        Addr(me + PMOD_DEF).sw(c, c.g[S0][0]);
        let owner = r32(c, V1);
        c.g[A1][0] = Addr(owner + 0xc).lw(c); // the owner's object
        c.g[V1][0] = sx(owner.wrapping_add(0x10));
        // The matrix to copy: the object's world matrix, else the owner's.
        let from;
        let to;
        if c.g[A1][0] != 0 {
            c.cycles += 16;
            let obj = r32(c, A1);
            c.g[V1][0] = Addr(obj + 0xc).lw(c);
            let vt = r32(c, V1);
            c.g[A0][0] = Addr(vt + 0xc0).lh(c);
            c.g[V0][0] = Addr(vt + 0xc4).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0034_591c)?;
            c.g[V1][0] = sx(r32(c, S1).wrapping_add(PMOD_MATRIX));
            from = r32(c, V0);
            to = r32(c, V1);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.cycles += 9;
            c.g[V0][0] = sx(me.wrapping_add(PMOD_MATRIX));
            from = r32(c, V1);
            to = r32(c, V0);
        }
        c.lqc2(1, from);
        c.lqc2(2, from + 0x10);
        c.lqc2(3, from + 0x20);
        c.lqc2(4, from + 0x30);
        c.sqc2(1, to);
        c.sqc2(2, to + 0x10);
        c.sqc2(3, to + 0x20);
        c.sqc2(4, to + 0x30);

        c.cycles += 14;
        let me = r32(c, S1);
        c.g[A2][0] = Addr(me + PMOD_DEF).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(me.wrapping_add(PMOD_EMITTER));
        call_ptr(c, f::EMITTER_SETUP, 0x0034_597c)?;
        let me = r32(c, S1);
        let gp = r32(c, GP);
        c.g[V1][0] = Addr(gp.wrapping_add(0xf08)).lw(c);
        c.g[V0][0] = c.g[S1][0];
        Addr(me + PMOD_1E4).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp.wrapping_add(0xf08)).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_9d40, rider_metrix_land_trick, "RiderMetrix_LandTrick"),
    (0x0011_9708, rider_metrix_add_trick, "RiderMetrix_AddTrick"),
    (0x003c_8158, snd_ee_voice_start, "SND_EeVoiceStart"),
    (0x0035_5f10, mod_obj_take_over_object, "ModObj_TakeOverObject"),
    (0x0037_05e0, emitter_setup, "Emitter_Setup"),
    (0x0034_1aa0, anim_object_ctor, "AnimObject_ctor"),
    (
        0x0023_6728,
        game_state_event_select_save_state,
        "GameStateEventSelect_SaveState",
    ),
    (0x0034_58c0, particle_modifier_ctor, "ParticleModifier_ctor"),
];

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};

    /// A RiderMetrix record at `DATA + 0x1000` that `RiderMetrix_FinishTrick`
    /// can bank for real: a zeroed BELibrary dummy and audio system, a world
    /// with one race player (finished on some seeds), the event settings, and
    /// random score/points/statistics. Returns the record.
    fn land_trick_metrix_setup(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        elf_image(c);
        let be = DATA + 0x20_0000;
        let world = DATA + 0x21_0000;
        let game = world + 0x1000;
        let race = world + 0x2000;
        let player = world + 0x3000;
        let player_state = world + 0x4000;
        let audio = DATA + 0x22_0000;
        for a in [be, world, audio] {
            for i in 0..0x1000 / 4 {
                c.bus.write32(a + 4 * i, 0);
            }
        }
        for i in 0..0x5000 / 4 {
            c.bus.write32(world + 4 * i, 0);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), be);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(GAME_GP + 0x410, audio);
        c.bus.write32(world + 0x84, game);
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 1);
        c.bus.write32(race + 0x40, player);
        c.bus.write32(player + 0x18, player_state);
        let finished = seed % 10 == 0;
        c.bus.write32(
            player_state + 0x470,
            (if finished { 1.0f32 } else { -1.0 }).to_bits(),
        );
        // The event settings (`D_00535BC8`): kind, game type.
        c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 5) as u8);
        c.bus.write8(0x0053_5bc8 + 0x49, (seed / 3 % 2) as u8);

        let m = DATA + 0x1000;
        for i in 0..0x200 / 4 {
            c.bus.write32(m + 4 * i, 0);
        }
        let points = if seed % 7 == 0 {
            0.0f32
        } else {
            (r.next_u32() % 4000) as f32
        };
        c.bus.write32(m + 0x1c4, points.to_bits());
        c.bus.write32(m + 0x34, r.f32(20.0));
        c.bus.write32(m + 0x38, r.f32(20.0));
        for off in [
            0x44, 0x30, 0x48, 0x24, 0x78, 0x14c, 0x150, 0x154, 0x158, 0x144,
        ] {
            c.bus.write32(m + off, r.f32(10.0));
        }
        for off in [
            0x4c, 0x80, 0x58, 0x74, 0x84, 0xfc, 0x100, 0x104, 0x108, 0x114, 0x118, 0x11c,
        ] {
            c.bus.write32(m + off, r.next_u32() % 100);
        }
        for off in [0x198, 0x19c, 0x110, 0x18c] {
            c.bus.write32(m + off, r.next_u32() % 3000);
        }
        c.bus.write32(m + 0xa4, r.f32(2.0));
        m
    }

    /// `RiderMetrix_LandTrick`: styled and plain landings, positive, zero
    /// and negative scores (the style bonus), flags on/off, and a trick
    /// chain (`+0x70`) running or not, banked by the real FinishTrick and
    /// ResetTrick.
    #[test]
    fn rider_metrix_land_trick_matches() {
        check(
            ee::s00::f_00119d40__generated,
            rider_metrix_land_trick,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x11_9d40, 0);
                let m = land_trick_metrix_setup(c, &mut r, seed);
                let score = match seed % 5 {
                    0 => 0.0f32,
                    1 => -((r.next_u32() % 50) as f32),
                    _ => (r.next_u32() % 500) as f32,
                };
                c.bus.write32(m + METRIX_SCORE, score.to_bits());
                c.bus.write32(
                    m + METRIX_CHAIN,
                    if seed % 3 == 0 { 0 } else { r.next_u32() % 40 + 1 },
                );
                c.bus.write32(m + 0x20, if seed % 4 == 1 { 1 } else { 0 });
                c.bus.write32(m + 0x28, if seed % 6 == 2 { 1 } else { 0 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = (r.next_u32() % 2) as u64;
                c.g[A2][0] = (r.next_u32() % 2) as u64;
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { (r.next_u32() % 8 + 1) as u64 };
                c.g[T0][0] = if seed % 8 < 3 { (r.next_u32() % 8 + 1) as u64 } else { 0 };
                c.pc = 0x0011_9d40;
            },
        );
    }
    /// `RiderMetrix_AddTrick`: a BELibrary whose interface 6 is a fake
    /// (unwinding on some seeds), a trick table at 0x530600 with random
    /// points, ids on both sides of the uber threshold (35), and timers
    /// `+0x40` negative or not.
    #[test]
    fn rider_metrix_add_trick_matches() {
        use crate::difftest::fakes::covered;
        use crate::game::ai::ai::tests::belibrary;
        covered(ee::s00::f_00119708__generated, rider_metrix_add_trick, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x11_9708, 1);
            elf_image(c);
            belibrary(c, &mut r);
            let id = match seed % 4 {
                0 => 34,
                1 => 35,
                _ => r.next_u32() % 60,
            };
            c.bus.write32(0x0053_0600 + id * 8, r.next_u32() % 20000);
            c.bus.write32(0x0053_0604 + id * 8, r.next_u32() % 2_000_000);
            let m = DATA + 0x2000;
            c.bus.write32(m + METRIX_SCORE, r.f32(100.0));
            c.bus.write32(m + METRIX_TRICK_TIMER, r.f32(5.0));
            c.bus.write32(m + 0xa4, r.f32(5.0));
            c.g[A0][0] = sx(m);
            c.g[A1][0] = id as u64;
            c.pc = 0x0011_9708;
        });
    }
    /// `SND_EeVoiceStart`: every sample format (and an unknown one), every
    /// channel mode (mono with `f` below or above 1, stereo, other), start
    /// functions present (fakes, unwinding on some seeds) or not, with and
    /// without a stream. The real `SND_MemAlloc` allocates from an empty
    /// heap at `DATA + 0x20_0000`; the real time-stretch init reads a random
    /// stream header.
    #[test]
    fn snd_ee_voice_start_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        use crate::difftest::SP as STACK;
        covered(ee::s14::f_003c8158__generated, snd_ee_voice_start, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3c_8158, 1);
            // The voice entries.
            let entries = DATA + 0x3000;
            for i in 0..0x60 * 8 / 4 {
                c.bus.write32(entries + 4 * i, r.next_u32());
            }
            c.bus.write32(VOICE_STATE + VOICE_STATE_ENTRIES, entries);
            // The driver heap (`STATE + 0x234`): empty, room for both blocks.
            let heap = DATA + 0x100;
            c.bus.write32(VOICE_STATE + 0x234, heap);
            c.bus.write32(heap, DATA + 0x20_0000); // base
            c.bus.write32(heap + 0x4, DATA + 0x2000); // blocks
            c.bus.write32(heap + 0xc, 0x8_0000); // limit
            c.bus.write32(heap + 0x10, 0x100); // low water
            c.bus.write32(heap + 0x14, 0); // lowest: empty
            // Object sizes, then start functions (row -1 reads below them).
            for i in 0..0x48 / 4 + 3 {
                c.bus.write32(VOICE_STATE + 0x54 + 4 * i, 0x20 + r.next_u32() % 0x200);
            }
            for i in 0..0x48 / 4 + 3 {
                let fun = if (seed + i) % 3 == 0 { 0 } else { FAKE + 0x10 * (i % 4) };
                c.bus.write32(VOICE_STATE + 0xc + 4 * i, fun);
            }
            c.bus.write8(VOICE_START_FLAG, r.next_u32() as u8);
            // The stream header.
            let strm = DATA + 0x5000;
            for i in 0..0x10 {
                c.bus.write32(strm + 4 * i, r.next_u32());
            }
            // Stack arguments: d, f, e, prio.
            let f_arg: i32 = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => -1,
                _ => (r.next_u32() % 10) as i32,
            };
            c.bus.write32(STACK, r.next_u32());
            c.bus.write32(STACK + 0x8, f_arg as u32);
            c.bus.write32(STACK + 0x20, r.next_u32());
            c.bus.write32(STACK + 0x28, r.next_u32());

            let formats = [8u64, 10, 4, 22, 16, 23, 5];
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            c.g[A1][0] = formats[(seed / 2 % 7) as usize];
            c.g[A2][0] = match seed / 3 % 4 {
                0 => 0,
                1 | 2 => 1,
                _ => 2,
            };
            c.g[T1][0] = if seed % 5 < 2 { 0 } else { sx(strm) };
            c.pc = 0x003c_8158;
        });
    }
    /// `ModObj_TakeOverObject`: no source; a source with or without
    /// particle modifiers (real `ModObj_TakeParticleModifiers`, short path),
    /// whose world matrix (a fake, forced result) is `self`'s own transform
    /// or a separate matrix that is copied into a real position modifier
    /// (pool allocator, `ModObj_SetTransformModifier`). Destroy and the
    /// final `self` method are fakes, unwinding on some seeds.
    #[test]
    fn mod_obj_take_over_object_matches() {
        use crate::difftest::fakes::{covered, FAKE, LOG_RESULTS};
        use crate::game::movie::movieplayer::tests::{alloc_base, ret_slot, RET_ONE, RET_ZERO};
        covered(ee::s11::f_00355f10__generated, mod_obj_take_over_object, 64, &|c, seed| {
            let mut r = alloc_base(c, seed, 0x35_5f10, 3);
            let me = DATA + 0x1000;
            let me_vt = DATA + 0x1400;
            let src = DATA + 0x2000;
            let src_vt = DATA + 0x2400;
            let transform = DATA + 0x3000;
            let block = DATA + 0x3100;
            let matrix = DATA + 0x3200;
            for a in [me, src] {
                for i in 0..0x40 {
                    c.bus.write32(a + 4 * i, 0);
                }
            }
            c.bus.write32(me + 0xc, me_vt);
            c.bus.write32(me + 0x18, transform);
            c.bus.write32(me + 0x1c, block);
            c.bus.write32(block, 0);
            c.bus.write32(transform + 0x8, r.next_u32());
            ret_slot(c, me_vt, 0x188, RET_ZERO);
            ret_slot(c, me_vt, 0x180, RET_ZERO);
            ret_slot(c, me_vt, 0x190, FAKE + 0x20);
            c.bus.write32(src + 0xc, src_vt);
            ret_slot(c, src_vt, 0x80, if seed % 2 == 0 { RET_ONE } else { RET_ZERO });
            ret_slot(c, src_vt, 0xc0, FAKE);
            ret_slot(c, src_vt, 0x8, FAKE + 0x10);
            for i in 0..16 {
                c.bus.write32(matrix + 4 * i, r.f32(100.0));
            }
            let own = seed % 3 == 0;
            c.bus.write32(LOG_RESULTS, if own { transform + 0x10 } else { matrix });
            c.g[A0][0] = sx(me);
            c.g[A1][0] = if seed % 7 == 6 { 0 } else { sx(src) };
            c.pc = 0x0035_5f10;
        });
    }
    /// An emitter at `emitter` and its definition at `def` that the real
    /// `Emitter_InitDescriptor`/`Emitter_BuildDescriptor` handle quickly:
    /// random fields, start time negative, zero or positive, small grid
    /// sizes, rates; the default GS state, the random generator state and
    /// the gp-relative table `Emitter_BuildDescriptor` reads.
    fn emitter_def_setup(c: &mut Cpu, r: &mut Rng, seed: u32, emitter: u32, def: u32) {
        use crate::difftest::fill;
        const RAND_STATE: u32 = 0x0050_0000 - 0xfe8;
        for i in 0..0x190 / 4 {
            c.bus.write32(emitter + 4 * i, r.f32(50.0));
        }
        for i in 0..0xd8 / 4 {
            c.bus.write32(def + 4 * i, r.f32(20.0));
        }
        fill(c, r, 0x004f_f120, 4); // default GS state
        // Grid sizes.
        c.bus.write32(def, r.next_u32() % 8 + 1);
        c.bus.write32(def + 0x4, r.next_u32() % 8 + 1);
        let start = match seed % 4 {
            0 => 0.0f32.to_bits(),
            1 => r.f32(5.0) | 0x8000_0000,
            _ => r.f32(5.0) & 0x7fff_ffff,
        };
        c.bus.write32(def + 0x8, start);
        if seed % 5 != 0 {
            c.bus.write32(def + 0xc, 1.5f32.to_bits());
            c.bus.write32(emitter + 0x10 + 0x4c, 0.25f32.to_bits());
        }
        for i in 0..6 {
            c.bus.write32(RAND_STATE + 4 * i, r.next_u32());
        }
        let base = (GAME_GP as i32 - 0x2868) as u32;
        for i in 0..24 {
            c.bus.write32(base + 4 * i, r.f32(10.0));
        }
    }

    /// `Emitter_Setup` with the real descriptor init/build on
    /// [`emitter_def_setup`] and a random matrix.
    #[test]
    fn emitter_setup_matches() {
        use crate::difftest::scramble;
        const EMITTER: u32 = DATA;
        const DEF: u32 = DATA + 0x400;
        const MATRIX: u32 = DATA + 0x600;
        check(ee::s12::f_003705e0__generated, emitter_setup, 200, &|c, seed| {
            let mut r = Rng(0x0037_05e0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0037_05e0;
            c.g[GP][0] = sx(GAME_GP);
            emitter_def_setup(c, &mut r, seed, EMITTER, DEF);
            for i in 0..0x10 {
                c.bus.write32(MATRIX + 4 * i, r.f32(4.0));
            }
            c.g[A0][0] = sx(EMITTER);
            c.g[A1][0] = sx(MATRIX);
            c.g[A2][0] = sx(DEF);
        });
    }

    /// `AnimObject_ctor` with the real constructor chain
    /// (`AnimObject_BaseCtor` -> `ModObj_ctor` -> `WorldObj_ctor` on the
    /// `world_obj_ctor_matches` fixture: process lists, world -> focus
    /// manager; a node whose info has no entries, so `AllocNodeInfo` and
    /// `AnimObject_v52` take their short paths and the instance stays 0),
    /// the pool allocator, and definitions with min/max/value negative or
    /// not, random value or not, speed spread or not, both directions; the
    /// frame rate at `gp+0x2a74 -> +0x10`.
    #[test]
    fn anim_object_ctor_matches() {
        use crate::difftest::fill;
        use crate::game::fe::feasyncfile::tests::{heap_init_4, pool_allocator_7};
        const ANIM: u32 = DATA + 0x800;
        const NODE: u32 = DATA + 0x200;
        const INFO: u32 = DATA + 0x600;
        const DEF: u32 = DATA + 0x700;
        const RATE: u32 = DATA + 0x780;
        const PROCS: u32 = DATA + 0x1000;
        const TAILS: u32 = DATA + 0x1400;
        const WORLD: u32 = DATA + 0x2000;
        const FOCUS: u32 = DATA + 0x3000;
        check(ee::s11::f_00341aa0__generated, anim_object_ctor, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x34_1aa0, 0);
            pool_allocator_7(c);
            heap_init_4(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            fill(c, &mut r, ANIM, 0x20);
            fill(c, &mut r, NODE, 0x10);
            let flags = r.next_u32();
            w(c, NODE + 0x8, if seed % 4 == 0 { flags | 0x100 } else { flags & !0x100 });
            w(c, NODE + 0x80, INFO);
            // Node info: no entries; the max used for a negative max.
            w(c, INFO + 0x4, 0);
            w(c, INFO + 0x8, 0);
            w(c, INFO + 0x14, r.f32(10.0));
            // Process lists and the world -> focus manager chain.
            w(c, GAME_GP + 0x2898 + 4, PROCS);
            for i in 0..4 {
                w(c, PROCS + 0x44 * i, r.next_u32());
                w(c, PROCS + 0x44 * i + 8, TAILS + 0x40 * i);
                w(c, PROCS + 0x44 * i + 0x28, TAILS + 0x40 * i + 0x20);
            }
            fill(c, &mut r, 0x0044_aff0, 0x20);
            w(c, GAME_GP - 0x848, WORLD);
            w(c, WORLD + 0x84, WORLD + 0x100);
            w(c, WORLD + 0x100 + 0xc, WORLD + 0x200);
            w(c, WORLD + 0x200 + 0xa4, FOCUS);
            w(c, FOCUS + 0xd4, if seed % 3 == 0 { 0 } else { r.next_u32() });
            w(c, FOCUS + 0xd8, r.next_u32() % 8);
            // Scales (1/30 s) and the frame rate.
            for off in [0x2b94u32, 0x2b90, 0x2b8c, 0x2b88, 0x2b84] {
                w(c, GAME_GP - off, (1.0f32 / 30.0).to_bits());
            }
            w(c, GAME_GP + 0x2a74, RATE);
            w(c, RATE + 0x10, if seed % 5 == 0 { 30 } else { 60 });
            // The definition.
            let signed = |r: &mut Rng, neg: bool| {
                let v = r.f32(20.0) & 0x7fff_ffff;
                if neg { v | 0x8000_0000 } else { v }
            };
            w(c, DEF + 0x4, r.next_u32());
            w(c, DEF + 0x8, if seed % 3 == 1 { 1 } else { r.next_u32() % 3 });
            w(c, DEF + 0xc, signed(&mut r, seed % 4 == 1));
            w(c, DEF + 0x10, signed(&mut r, seed % 5 == 2));
            w(c, DEF + 0x14, r.f32(30.0));
            w(c, DEF + 0x18, if seed % 2 == 0 { 0 } else { r.f32(10.0) });
            w(c, DEF + 0x1c, signed(&mut r, seed % 6 == 3));
            w(c, DEF + 0x20, if seed % 7 == 4 { 1 } else { 0 });
            w(c, DEF + 0x24, if seed % 3 == 0 { 1 } else { 0 });
            let kind = match (seed / 4) % 6 {
                0 => 6,
                1 => 0x10,
                2 => 0x16,
                _ => r.next_u32() % 0x20,
            };
            c.g[A0][0] = sx(ANIM);
            c.g[A1][0] = sx(seed % 4);
            c.g[A2][0] = sx(kind);
            c.g[A3][0] = sx(NODE);
            c.g[T0][0] = sx(DEF);
            c.pc = 0x0034_1aa0;
        });
    }
    /// `GameStateEventSelect_SaveState`: the base save (real
    /// `GameState_SaveState`) and the four field writes all go to a fake
    /// stream write method (logged, unwinding at a random call).
    #[test]
    fn game_state_event_select_save_state_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(
            ee::s06::f_00236728__generated,
            game_state_event_select_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_6728, 5);
                c.pc = 0x0023_6728;
                c.bus.write32(STREAM, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.g[A0][0] = if seed % 4 == 0 {
                    (r.next_u32() as u64) << 32 | ME as u64
                } else {
                    sx(ME)
                };
                c.g[A1][0] = sx(STREAM);
            },
        );
    }
    /// `ParticleModifier_ctor`: an owner with an object (its world matrix a
    /// fake with a forced result, unwinding on some seeds) or without (its
    /// own matrix); the real base constructor, `Emitter_ctor` and
    /// `Emitter_Setup` on [`emitter_def_setup`].
    #[test]
    fn particle_modifier_ctor_matches() {
        use crate::difftest::fakes::{covered, FAKE, LOG_RESULTS};
        const ME: u32 = DATA;
        const DEF: u32 = DATA + 0x400;
        const OWNER: u32 = DATA + 0x600;
        const OBJ: u32 = DATA + 0x700;
        const OBJ_VT: u32 = DATA + 0x800;
        const MATRIX: u32 = DATA + 0x900;
        covered(ee::s11::f_003458c0__generated, particle_modifier_ctor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_58c0, 1);
            c.pc = 0x0034_58c0;
            emitter_def_setup(c, &mut r, seed, ME + PMOD_EMITTER, DEF);
            for i in 0..0x20 {
                c.bus.write32(OWNER + 4 * i, r.f32(4.0));
            }
            for i in 0..0x10 {
                c.bus.write32(MATRIX + 4 * i, r.f32(4.0));
            }
            let has_obj = seed % 4 != 0;
            c.bus.write32(OWNER + 0xc, if has_obj { OBJ } else { 0 });
            c.bus.write32(OBJ + 0xc, OBJ_VT);
            c.bus.write16(OBJ_VT + 0xc0, (r.next_u32() % 0x20) as u16 & !3);
            c.bus.write32(OBJ_VT + 0xc4, FAKE);
            c.bus.write32(LOG_RESULTS, MATRIX);
            c.bus.write32(GAME_GP + 0xf04, r.next_u32());
            c.bus.write32(GAME_GP + 0xf08, r.next_u32());
            c.g[A0][0] = sx(ME);
            c.g[A1][0] = sx(DEF);
            c.g[A2][0] = sx(OWNER);
        });
    }
}
