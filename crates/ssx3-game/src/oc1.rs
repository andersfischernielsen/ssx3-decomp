//! Staging module: sequence-instance update (`InSeq_InstanceUpdate`).
//!
//! Ported for the tick-rate pass so the script clock can be converted next
//! wave. This port is exactly one script frame per tick, like the generated
//! body; no rate conversion.

use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_4a30, in_seq_instance_update, "InSeq_InstanceUpdate"),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) const GET_LENGTH: Callee = Callee { addr: 0x0027_4c70, f: ee::s07::f_00274c70 };
    pub(crate) const TICK: Callee = Callee { addr: 0x0027_4de0, f: ee::s07::f_00274de0 };
    pub(crate) const STOP: Callee = Callee { addr: 0x0027_4918, f: ee::s07::f_00274918 };
    pub(crate) const RESTART: Callee = Callee { addr: 0x0027_4810, f: ee::s07::f_00274810 };
    pub(crate) const PAUSE: Callee = Callee { addr: 0x0027_49e8, f: ee::s07::f_002749e8 };
}

/// `a + off` on a guest pointer.
#[inline(always)]
fn at(a: u64, off: u32) -> u32 {
    (a as u32).wrapping_add(off)
}

/// `void InSeq_InstanceUpdate(InSeq_Instance* self)` (0x274a30): apply a
/// pending seek (`+0x34`) or a step count (`+0x38`), else advance exactly one
/// script frame; keeps the short step budget (`+0x2c`) and pauses when it runs
/// out.
///
/// `+0x1c` loop, `+0x20` playing, `+0x24` speed, `+0x28` pos (frames),
/// `+0x2c` frame budget, `+0x34` seek target, `+0x38` pending steps.
pub fn in_seq_instance_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x60);
        frame.save_fpr(c, 21, 0x58);
        frame.save_fpr(c, 20, 0x50);
        c.g[V1][0] = c.lw(at(this, 0x34));
        // `need` (`s2`): the final one-frame tick is due unless a seek or a
        // step run already handled the frame. It stays in `s2` because the
        // callees save that register to the stack.
        c.g[S2][0] = 1;
        if (c.g[V1][0] as i64) >= 0 {
            // 0x274a64: a pending seek.
            c.cycles += 9;
            c.g[V0][0] = u64::MAX;
            c.g[S1][0] = c.g[V1][0];
            c.sw(at(this, 0x34), c.g[V0][0]);
            callee::GET_LENGTH.call(c, 0x0027_4a74)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[S3][0] = c.lw(at(this, 0x2c)); // the saved frame budget
            c.g[V0][0] = c.lw(at(this, 0x1c));
            let no_loop = c.g[V0][0] == 0;
            c.sw(at(this, 0x2c), 0);
            if !no_loop {
                // 0x274a88: target %= length.
                c.cycles += 3;
                c.div(0, S1, V1);
                c.cycles += 2;
                c.g[V0][0] = c.hi[0];
                c.g[S1][0] = c.g[V0][0];
            }
            // 0x274a9c: f20 = (float)target.
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(this, 0x20));
            c.mtc1(17, 20);
            c.cvt_s_w(20, 20);
            if c.g[V0][0] != 1 {
                c.lwc1(0, at(this, 0x28));
            } else {
                // 0x274ab0: one tick before the seek comparison.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::TICK.call(c, 0x0027_4ab8)?;
                c.lwc1(0, at(this, 0x28));
            }
            // 0x274abc: already at the target?
            c.cycles += 4;
            c.c_eq_s(20, 0);
            if c.fcc() {
                // 0x274b6c: keep the budget while playing.
                c.g[V0][0] = c.lw(at(this, 0x20));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(at(this, 0x2c), c.g[S3][0]);
                }
            } else {
                // 0x274acc: target < pos: rewind by restarting.
                c.cycles += 4;
                c.c_lt_s(20, 0);
                if c.fcc() {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    callee::STOP.call(c, 0x0027_4ae4)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RESTART.call(c, 0x0027_4aec)?;
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4af4)?;
                }
                c.lwc1(0, at(this, 0x28));
                // 0x274af8: advance in 1.0 steps until the target is reached.
                c.cycles += 12;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 21); // f21 = 1.0
                c.cvt_w_s(1, 0); // f1 = (int)pos
                c.mfc1(2, 1);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.mtc1(2, 1);
                c.cvt_s_w(1, 1); // f1 = (float)((int)pos + 1)
                c.sub_s(1, 1, 0); // f1 = fraction to the next frame
                c.c_lt_s(1, 21); // fraction < 1.0
                let partial = c.fcc();
                c.lwc1(22, at(this, 0x24)); // f22 = speed
                if partial {
                    // 0x274b28: finish the current frame at the partial step.
                    c.cycles += 3;
                    c.swc1(1, at(this, 0x24));
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4b34)?;
                }
                // 0x274b34: full steps until pos reaches the target.
                c.cycles += 3;
                c.g[V0][0] = c.lw(at(this, 0x20));
                c.swc1(21, at(this, 0x24));
                loop {
                    // 0x274b4c
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.swc1(22, at(this, 0x24));
                        break;
                    }
                    // 0x274b54: pos < target?
                    c.cycles += 4;
                    c.lwc1(0, at(this, 0x28));
                    c.c_lt_s(0, 20);
                    if !c.fcc() {
                        c.cycles += 1;
                        c.swc1(22, at(this, 0x24));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4b40);
                    }
                    // 0x274b40
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    callee::TICK.call(c, 0x0027_4b48)?;
                    c.g[V0][0] = c.lw(at(this, 0x20));
                }
                // 0x274b68: keep the budget while playing.
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(this, 0x20));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(at(this, 0x2c), c.g[S3][0]);
                }
            }
            // 0x274b74
            c.cycles += 1;
            c.g[S2][0] = 0;
        }
        // 0x274b78: pending steps.
        c.cycles += 3;
        let mut steps = c.lw(at(this, 0x38));
        c.g[V0][0] = steps;
        c.g[S1][0] = steps;
        if steps != 0 {
            // 0x274b84
            c.cycles += 5;
            let saved_f2c = c.lw(at(this, 0x2c));
            c.g[S2][0] = saved_f2c;
            c.g[V0][0] = c.lw(at(this, 0x20));
            c.sw(at(this, 0x38), 0);
            let playing = c.g[V0][0];
            c.sw(at(this, 0x2c), 0);
            if playing != 0 {
                // 0x274b98
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                loop {
                    // 0x274ba0
                    c.cycles += 5;
                    steps = sx((steps as u32).wrapping_sub(1));
                    c.g[S1][0] = steps;
                    callee::TICK.call(c, 0x0027_4ba8)?;
                    c.g[V0][0] = c.lw(at(this, 0x20));
                    if c.g[V0][0] == 0 {
                        // The stop path jumps straight to 0x274bc4.
                        c.g[V0][0] = c.lw(at(this, 0x2c));
                        break;
                    }
                    // 0x274bb4
                    c.cycles += 2;
                    let more = c.g[S1][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !more {
                        // 0x274bbc: the steps ran out while still playing.
                        c.cycles += 1;
                        c.sw(at(this, 0x2c), c.g[S2][0]);
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(at(this, 0x2c));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_4ba0);
                    }
                }
            } else {
                // 0x274bc0
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(this, 0x2c));
            }
            // 0x274bc4: a positive budget keeps playing.
            c.cycles += 2;
            let positive = (c.g[V0][0] as i64) > 0;
            c.g[S2][0] = 0;
            if !positive {
                // 0x274bcc: the budget ran out: pause.
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::PAUSE.call(c, 0x0027_4bd4)?;
                c.g[S2][0] = 0;
            }
        }
        // 0x274bd8: the final one-frame tick (skipped after a seek or steps).
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::TICK.call(c, 0x0027_4be8)?;
        }
        c.lq(S0, frame.at(c, 0x40));
        // 0x274bec
        c.cycles += 9;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x60);
        frame.restore_fpr(c, 21, 0x58);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};

    /// One `InSeq_Instance` at `obj` with a one-second channel block (the
    /// cached length avoids the channel walk) and seed-picked seek/step/play
    /// state.
    fn fixture(c: &mut Cpu, seed: u32) {
        fakes::machine(c, seed, 0x274a30, 0);
        c.pc = 0x0027_4a30;
        let obj = DATA + 0x1000;
        let chan = DATA + 0x2000;
        c.g[A0][0] = obj as u64;
        c.bus.write8(chan + 1, 0);
        c.bus.write32(obj + 0x10, chan);
        c.bus.write32(obj + 0x18, 0);
        c.bus.write32(obj + 0x1c, if seed % 4 == 0 { 0 } else { 1 });
        c.bus.write32(obj + 0x20, [0, 1, 1, 2, 4][(seed % 5) as usize]);
        c.bus.write32(obj + 0x24, if seed % 2 == 0 { 0x3f80_0000 } else { 0x3f00_0000 });
        c.bus.write32(obj + 0x28, [0, 0x3f00_0000, 0x4020_0000][(seed % 3) as usize]);
        c.bus.write32(obj + 0x2c, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(obj + 0x30, 100);
        c.bus.write32(obj + 0x34, [0xffff_ffff, 0, 3][(seed % 3) as usize]);
        c.bus.write32(obj + 0x38, [0, 1, 3, 0][(seed % 4) as usize]);
        c.bus.write32(obj + 0x3c, 0);
        c.bus.write32(obj + 0x40, 0);
    }

    #[test]
    fn in_seq_instance_update_matches() {
        check(ee::s07::f_00274a30__generated, in_seq_instance_update, 24, &|c, seed| {
            fixture(c, seed);
        });
    }
}
