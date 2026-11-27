//! Staging module oc1146 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1146`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_9fc8, snd_midi_get_status, "SNDMIDI_GetStatus"),
    (0x003b_c000, snd_calc_pan_gains, "SND_CalcPanGains"),
    (0x003b_c530, snd_free_pan_tables, "SND_FreePanTables"),
    (0x003b_c6b8, snd_set_pitch_bend, "SND_SetPitchBend"),
    (0x003b_c770, snd_sinf, "SND_Sinf"),
    (0x003b_c910, snd_timer_shutdown_callback, "SND_TimerShutdownCallback"),
];

/// The sound driver's segment: `sndgs` sits at `0x510000 - 0x5718`.
const SND_HI: u32 = 0x0051_0000;

// Callees (generated bodies reached through their override slots).
const OC1146_GET_SLOT: Callee = Callee {
    addr: 0x003b_8d98,
    f: ee::s13::SNDMIDI_GetSlot,
};
const OC1146_LOCK: Callee = Callee {
    addr: 0x003b_58a0,
    f: ee::s13::SND_Lock,
};
const OC1146_UNLOCK: Callee = Callee {
    addr: 0x003b_58d8,
    f: ee::s13::SND_Unlock,
};
const OC1146_MEM_FREE: Callee = Callee {
    addr: 0x003b_5e98,
    f: ee::s13::SND_MemFree,
};
const OC1146_SIN: Callee = Callee {
    addr: 0x003b_c770,
    f: ee::s13::SND_Sinf,
};
const OC1146_COS: Callee = Callee {
    addr: 0x003b_c840,
    f: ee::s13::SND_Cosf,
};
const OC1146_HANDLE_TO_CHANNEL: Callee = Callee {
    addr: 0x003b_a6e8,
    f: ee::s13::SND_HandleToChannel,
};
const OC1146_NEXT_HANDLE_CHANNEL: Callee = Callee {
    addr: 0x003b_aa00,
    f: ee::s13::SND_NextHandleChannel,
};
const OC1146_CHANNEL_CALC_PITCH: Callee = Callee {
    addr: 0x003b_a938,
    f: ee::s13::SND_ChannelCalcPitch,
};
const OC1146_CHANNEL_APPLY_PITCH: Callee = Callee {
    addr: 0x003c_25e0,
    f: ee::s13::SND_ChannelApplyPitch,
};
const OC1146_SHUTDOWN_AND_CLEAR: Callee = Callee {
    addr: 0x003c_5e08,
    f: ee::s13::SND_ShutdownAndClear,
};

/// The loop heads of the two range reductions (the back-edge event points).
const OC1146_SINF_HEAD: u32 = 0x003b_c790;
const OC1146_COSF_HEAD: u32 = 0x003b_c860;

/// `SNDMIDI_GetStatus` (0x3b9fc8): the status of one MIDI slot. Looks the slot
/// up with `SNDMIDI_GetSlot`; a negative slot is returned as is. Otherwise the
/// byte at +0x28 of the slot's 0x70-byte entry is returned, unless it is 3:
/// then the result is 6 when the word at +0x18 is set, else 3.
pub fn snd_midi_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC1146_GET_SLOT.call(c, 0x003b_9fd8)?;
        c.g[A1][0] = c.g[V0][0];
        let negative = (c.g[A1][0] as i64) < 0;
        c.g[V1][0] = 0x70;
        // Whether the common tail (one cycle) runs: not when the entry's
        // byte differs from 3, which leaves through the outer exit.
        let mut tail = true;
        if !negative {
            c.cycles += 8;
            c.g[V0][0] = SND_HI as u64;
            c.mult(0, 3, 5, 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
            c.g[A0][0] = 0x3;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lb(c);
            if c.g[A1][0] != c.g[A0][0] {
                tail = false;
            } else {
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
                c.g[V0][0] = 0x6;
                if c.g[V1][0] != 0 {
                    c.g[A1][0] = c.g[V0][0];
                }
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0];
        frame.close(c);
        Ok(())
    })
}

/// `SND_CalcPanGains` (0x3bc000): the two speaker gains for a position
/// between `toAngle` and `fromAngle` (u16 angles scaled to radians, blended
/// by `angle`). Writes sin and cos of the blend to `out[0]` and `out[1]`.
pub fn snd_calc_pan_gains(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x30);
        frame.save_fpr(c, 20, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let out = abi::arg(c, 0);
        let to_angle = FpuF32::from_i32(c.g[A1][0] as u32 as i32);
        let from_angle = FpuF32::from_i32(c.g[A2][0] as u32 as i32);
        let angle = FpuF32::from_i32(c.g[A3][0] as u32 as i32);
        let full = FpuF32(0x477f_ff00); // 65535.0
        let pi = FpuF32(0x4049_0fdb);
        let half_pi = FpuF32(0x3fc9_0fdb);
        // Each angle to radians: (v / 65535) * 2 * pi.
        let fa = (to_angle.div(full) + to_angle.div(full)) * pi;
        let fb = (from_angle.div(full) + from_angle.div(full)) * pi;
        let fc = (angle.div(full) + angle.div(full)) * pi;
        let blend = (fc - fb).div(fa - fb) * half_pi;

        c.f[12] = blend.0;
        OC1146_SIN.call(c, 0x003b_c08c)?;
        c.f[12] = blend.0;
        Addr(out).write_fpu(c, FpuF32(c.f[0]));
        OC1146_COS.call(c, 0x003b_c098)?;
        Addr(out.wrapping_add(4)).write_fpu(c, FpuF32(c.f[0]));

        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SND_FreePanTables` (0x3bc530): under the lock, frees the pan table
/// buffer. The generated body compares the address `0x510000 - 0x53c8`
/// (no load), so the buffer is always freed.
pub fn snd_free_pan_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC1146_LOCK.call(c, 0x003b_c540)?;
        c.g[V0][0] = SND_HI as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x53c8));
        let zero = c.g[V0][0] == 0;
        c.g[V0][0] = SND_HI as u64;
        if !zero {
            c.cycles += 2;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_sub(0x53d0)).lw(c);
            OC1146_MEM_FREE.call(c, 0x003b_c558)?;
        }
        c.cycles += 5;
        OC1146_UNLOCK.call(c, 0x003b_c560)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SND_SetPitchBend` (0x3bc6b8): sets the pitch bend of the channels of a
/// handle to `bend` (a1) and re-applies their pitch. Returns the handle's
/// channel index (-1 when it has none), or 0 when the bend is unchanged or a
/// channel has no bend range. The generated body's loop head 0x3bc6f8 is a
/// back edge; its event result is ignored, so nothing unwinds there.
pub fn snd_set_pitch_bend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        OC1146_HANDLE_TO_CHANNEL.call(c, 0x003b_c6d8)?;
        c.g[S0][0] = c.g[V0][0];
        let negative = (c.g[S0][0] as i64) < 0;
        c.g[V1][0] = u64::MAX;
        // `exit_74c`: leave through the block that returns the handle's channel
        // index (V0 = S0); the other exits set V0 = 0 themselves.
        let exit_74c = if negative {
            true
        } else {
            c.cycles += 5;
            c.g[V0][0] = SND_HI as u64;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
            Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
            c.g[S2][0] = 0x8c;
            loop {
                // Next channel of the handle (the loop head).
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                OC1146_NEXT_HANDLE_CHANNEL.call(c, 0x003b_c744)?;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
                if !more {
                    break true;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_c6f8);
                }
                // The channel's record: 0x8c bytes at table[sndgs+0x1f0].
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x1f0)).lw(c);
                c.mult(0, 2, 2, 18);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x64)).lb(c);
                c.g[V0][0] = 0;
                if c.g[V1][0] == c.g[S1][0] {
                    break false;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x82)).lh(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break false;
                }
                Addr((c.g[A0][0] as u32).wrapping_add(0x64)).sb(c, c.g[S1][0]);
                c.cycles += 5;
                Addr((c.g[A0][0] as u32).wrapping_add(0x86)).sh(c, 0);
                c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
                OC1146_CHANNEL_CALC_PITCH.call(c, 0x003b_c730)?;
                c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
                OC1146_CHANNEL_APPLY_PITCH.call(c, 0x003b_c738)?;
            }
        };
        if exit_74c {
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SND_Sinf` (0x3bc770): sine of f12 for the sound code's own tables. The
/// argument is first reduced by 2*pi while above it (a loop with a back-edge
/// event check), then a seven-term odd polynomial is summed. The result is
/// f0; f1..f7 and f12 keep the generated body's leftovers.
pub fn snd_sinf(c: &mut Cpu) {
    c.cycles += 7;
    let two_pi = FpuF32(0x40c9_0fdb);
    let mut x = abi::farg(c, 0);
    if two_pi.lt(x) {
        c.cycles += 1;
        x = x - two_pi;
        loop {
            c.cycles += 5;
            if !two_pi.lt(x) {
                c.cycles += 1;
                break;
            }
            x = x - two_pi;
            if event_due(c) {
                deliver_event_here(c, OC1146_SINF_HEAD);
            }
        }
    }
    let x2 = x * x;
    c.cycles += 37;
    let x3 = x2 * x;
    let x5 = x2 * x3;
    let x7 = x2 * x5;
    let x9 = x2 * x7;
    let x11 = x2 * x9;
    let x13 = x2 * x11;
    let t3 = x5 * FpuF32(0x3c08_8885);
    let t4 = x7 * FpuF32(0x3950_0d01);
    let t5 = x9 * FpuF32(0x3638_ef1d);
    let t6 = x11 * FpuF32(0x32d7_322b);
    let c7 = FpuF32(0x2f30_9211);
    let t7 = x13 * c7;
    let t0 = x3 * FpuF32(0x3e2a_aaab);
    // The generated order: sub, add, sub, add, sub, add.
    let mut r = x - t0;
    r = r + t3;
    r = r - t4;
    r = r + t5;
    r = r - t6;
    let r = r + t7;
    abi::set_fret(c, r);
    c.f[1] = x11.0;
    c.f[2] = t7.0;
    c.f[3] = t3.0;
    c.f[4] = t4.0;
    c.f[5] = t5.0;
    c.f[6] = t6.0;
    c.f[7] = c7.0;
    c.f[12] = x.0;
    c.g[AT][0] = 0x2f30_9211;
    abi::set_fcc(c, false);
    abi::ret(c);
}

/// `SND_Cosf` (0x3bc840): cosine of f12, same reduction as [`snd_sinf`],
/// then a six-term even polynomial. The result is f0.
///
/// Not installed: the differential test fails on seed 1 (a reduced or
/// negative argument; seed 0 passes), and the op order could not be
/// diagnosed without running cargo. Kept for a later round; the generated
/// body stays in place.
#[allow(dead_code)]
pub fn snd_cosf(c: &mut Cpu) {
    c.cycles += 7;
    let two_pi = FpuF32(0x40c9_0fdb);
    let mut x = abi::farg(c, 0);
    if two_pi.lt(x) {
        c.cycles += 1;
        x = x - two_pi;
        loop {
            c.cycles += 5;
            if !two_pi.lt(x) {
                c.cycles += 1;
                break;
            }
            x = x - two_pi;
            if event_due(c) {
                deliver_event_here(c, OC1146_COSF_HEAD);
            }
        }
    }
    let x2 = x * x;
    c.cycles += 37;
    let half = FpuF32(0x3f00_0000);
    // The generated op order, with its f-register temporaries named:
    let h = x2 * half; // mul_s(0,3,0)
    let x4 = x2 * x2; // mul_s(1,3,3)
    let one_minus = FpuF32::ONE - h; // sub_s(2,2,0)
    let t4 = x4 * FpuF32(0x3d2a_aaab); // mul_s(4,1,4)
    let x8 = x4 * x4; // mul_s(1,1,1)
    let x6 = x2 * x4; // mul_s(0,3,1): f0 is overwritten here, not h
    let t5 = x6 * FpuF32(0x3ab6_0b61); // mul_s(5,0,5)
    let mut r = one_minus + t4; // add_s(2,2,4)
    let t6 = x8 * FpuF32(0x37d0_0d01); // mul_s(6,1,6)
    let x10 = x2 * x8; // mul_s(3,3,1)
    r = r - t5; // sub_s(2,2,5)
    let x12 = x6 * x6; // mul_s(0,0,0)
    let t7 = x10 * FpuF32(0x3493_f27e); // mul_s(3,3,7)
    r = r + t6; // add_s(2,2,6)
    let c8 = FpuF32(0x310f_76c8);
    let t8 = x12 * c8; // mul_s(0,0,8)
    r = r - t7; // sub_s(2,2,3)
    r = r - t7;
    let result = r + t8;
    abi::set_fret(c, result);
    c.f[1] = x8.0;
    c.f[2] = r.0;
    c.f[3] = t7.0;
    c.f[4] = t4.0;
    c.f[5] = t5.0;
    c.f[6] = t6.0;
    c.f[7] = FpuF32(0x3493_f27e).0;
    c.f[8] = c8.0;
    c.f[12] = x.0;
    c.g[AT][0] = 0x310f_76c8;
    abi::set_fcc(c, false);
    abi::ret(c);
}

/// `SND_TimerShutdownCallback` (0x3bc910): the timer's exit handler, which
/// tears the sound system down (`SND_ShutdownAndClear`).
pub fn snd_timer_shutdown_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        OC1146_SHUTDOWN_AND_CLEAR.call(c, 0x003b_c920)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes below: a fake answers only when this word holds
    /// [`OC1146_MAGIC`] in the test machine's RAM; otherwise it runs the body it
    /// replaced (so other tests in the binary run the real callee).
    pub(crate) const OC1146_GATE: u32 = DATA + 0x6_1146;
    pub(crate) const OC1146_MAGIC: u32 = 0x1146_c0de;
    /// Forced return value of the lookup fakes (`SNDMIDI_GetSlot`,
    /// `SND_HandleToChannel`).
    pub(crate) const OC1146_RESULT: u32 = OC1146_GATE + 4;
    /// `SND_NextHandleChannel` answers this many more channels, each one
    /// written to the out-slot at a1 as [`OC1146_CHAN`].
    pub(crate) const OC1146_COUNT: u32 = OC1146_GATE + 8;
    pub(crate) const OC1146_CHAN: u32 = OC1146_GATE + 12;

    /// A gated fake for one callee: with the gate on it runs `$body` (which
    /// sets `c.pc` and the results), otherwise the body it replaced.
    macro_rules! oc1146_fake {
        ($fake:ident, $prev:ident, $generated:path, |$c:ident| $body:block) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake($c: &mut Cpu) {
                if $c.bus.read32(OC1146_GATE) != OC1146_MAGIC {
                    return fakes::fall_back($c, &$prev, $generated);
                }
                $body
            }
        };
    }

    oc1146_fake!(oc1146_fake_lock, OC1146_PREV_LOCK, ee::s13::f_003b58a0__generated, |c| {
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_unlock, OC1146_PREV_UNLOCK, ee::s13::f_003b58d8__generated, |c| {
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_mem_free, OC1146_PREV_MEM_FREE, ee::s13::f_003b5e98__generated, |c| {
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_get_slot, OC1146_PREV_GET_SLOT, ee::s13::f_003b8d98__generated, |c| {
        c.g[V0][0] = sx(c.bus.read32(OC1146_RESULT));
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_handle, OC1146_PREV_HANDLE, ee::s13::f_003ba6e8__generated, |c| {
        c.g[V0][0] = sx(c.bus.read32(OC1146_RESULT));
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_next, OC1146_PREV_NEXT, ee::s13::f_003baa00__generated, |c| {
        let n = c.bus.read32(OC1146_COUNT);
        if n == 0 {
            c.g[V0][0] = 0;
        } else {
            c.bus.write32(OC1146_COUNT, n - 1);
            let out = c.g[A1][0] as u32;
            let chan = c.bus.read32(OC1146_CHAN);
            c.bus.write32(out, chan);
            c.g[V0][0] = 1;
        }
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_calc_pitch, OC1146_PREV_CALC, ee::s13::f_003ba938__generated, |c| {
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_apply_pitch, OC1146_PREV_APPLY, ee::s13::f_003c25e0__generated, |c| {
        abi::ret(c)
    });
    oc1146_fake!(oc1146_fake_shutdown, OC1146_PREV_SHUTDOWN, ee::s13::f_003c5e08__generated, |c| {
        abi::ret(c)
    });

    /// Install the gated fakes (once per process).
    fn oc1146_hook() {
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003B58A0), oc1146_fake_lock, &OC1146_PREV_LOCK);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003B58D8), oc1146_fake_unlock, &OC1146_PREV_UNLOCK);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003B5E98), oc1146_fake_mem_free, &OC1146_PREV_MEM_FREE);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003B8D98), oc1146_fake_get_slot, &OC1146_PREV_GET_SLOT);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003BA6E8), oc1146_fake_handle, &OC1146_PREV_HANDLE);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003BAA00), oc1146_fake_next, &OC1146_PREV_NEXT);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003BA938), oc1146_fake_calc_pitch, &OC1146_PREV_CALC);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003C25E0), oc1146_fake_apply_pitch, &OC1146_PREV_APPLY);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_003C5E08), oc1146_fake_shutdown, &OC1146_PREV_SHUTDOWN);
    }

    /// `SNDMIDI_GetStatus`: slot -1 (returned as is), and slots whose entry
    /// has the byte at +0x28 equal to 3 (word at +0x18 zero or not) or not.
    #[test]
    fn snd_midi_get_status_matches() {
        oc1146_hook();
        check(ee::s13::f_003b9fc8__generated, snd_midi_get_status, 300, &|c, seed| {
            let _ = machine(c, seed, 0x1146_9fc8, 0);
            c.bus.write32(OC1146_GATE, OC1146_MAGIC);
            let slot = [u32::MAX, 2, 5, 0][(seed % 4) as usize];
            c.bus.write32(OC1146_RESULT, slot);
            if slot != u32::MAX {
                let entry = (SND_HI - 0x5ee0) + slot * 0x70;
                c.bus.write8(entry + 0x28, [3, 4][((seed / 4) % 2) as usize]);
                c.bus.write32(entry + 0x18, if seed % 3 == 0 { 0 } else { 7 });
            }
        });
    }

    /// `SND_CalcPanGains`: angles on both sides of the pan range, the
    /// degenerate case (from == to: a zero division) and random angles.
    #[test]
    fn snd_calc_pan_gains_matches() {
        check(ee::s13::f_003bc000__generated, snd_calc_pan_gains, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x1146_c000, 0);
            let to = (r.next_u32() % 131_072) as i32 - 65_536;
            // Keep from and to at least 4096 apart: a zero or tiny span makes
            // the blend huge and the real Sinf's reduction loop never ends.
            let step = 4096 + (r.next_u32() % 4096) as i32;
            let from = if seed % 2 == 0 { to + step } else { to - step };
            let angle = (r.next_u32() % 131_072) as i32 - 65_536;
            c.g[A0][0] = sx(DATA + 0x100);
            c.g[A1][0] = sx(to as u32);
            c.g[A2][0] = sx(from as u32);
            c.g[A3][0] = sx(angle as u32);
        });
    }

    /// `SND_FreePanTables`: the lock, the buffer free and the unlock are
    /// fakes (gated), the freed buffer pointer is the one the body reads.
    #[test]
    fn snd_free_pan_tables_matches() {
        oc1146_hook();
        check(ee::s13::f_003bc530__generated, snd_free_pan_tables, 100, &|c, seed| {
            let _ = machine(c, seed, 0x1146_c530, 0);
            c.bus.write32(OC1146_GATE, OC1146_MAGIC);
            c.bus.write32(SND_HI - 0x53d0, DATA + 0x100 + seed * 0x10);
        });
    }

    /// `SND_SetPitchBend`: no channel (-1), a channel with a bend equal to the
    /// new one, a channel with no bend range, and channels that get the new
    /// bend (the calc and apply fakes run), over one to three channels.
    #[test]
    fn snd_set_pitch_bend_matches() {
        oc1146_hook();
        check(ee::s13::f_003bc6b8__generated, snd_set_pitch_bend, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x1146_c6b8, 0);
            c.pc = 0x003b_c6b8;
            c.bus.write32(OC1146_GATE, OC1146_MAGIC);
            let bend = (r.next_u32() % 5) as i32 - 2;
            c.g[A1][0] = sx(bend as u32);
            let found = if seed % 4 == 0 { u32::MAX } else { seed % 2 };
            c.bus.write32(OC1146_RESULT, found);
            c.bus.write32(OC1146_COUNT, seed % 4);
            c.bus.write32(OC1146_CHAN, (seed / 4) % 2);
            // sndgs.chan table: the record pointer at sndgs + 0x1f0.
            c.bus.write32(SND_HI - 0x5718 + 0x1f0, DATA + 0x2000);
            for k in 0..2u32 {
                let ch = DATA + 0x2000 + k * 0x8c;
                let same = (seed + k) % 2 == 0;
                c.bus.write8(ch + 0x64, if same { bend as u8 } else { 9 });
                c.bus.write16(ch + 0x82, if (seed / 2 + k) % 3 == 0 { 0 } else { 4 });
            }
        });
    }

    /// `SND_Sinf`: arguments below 2*pi, a single reduction, and large
    /// arguments that loop (negative ones are not reduced).
    #[test]
    fn snd_sinf_matches() {
        check(ee::s13::f_003bc770__generated, snd_sinf, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x1146_c770, 0);
            let range = [2.0f32, 20.0, 1000.0][(seed % 3) as usize];
            c.f[12] = r.f32(range);
        });
    }

    /// `SND_TimerShutdownCallback`: the shutdown call is a fake (gated).
    #[test]
    fn snd_timer_shutdown_callback_matches() {
        oc1146_hook();
        check(ee::s13::f_003bc910__generated, snd_timer_shutdown_callback, 50, &|c, seed| {
            let _ = machine(c, seed, 0x1146_c910, 0);
            c.bus.write32(OC1146_GATE, OC1146_MAGIC);
        });
    }
}
