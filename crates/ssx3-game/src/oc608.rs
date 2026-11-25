//! Staging module oc608 (wave l599a.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc608`.

#![allow(unused_imports)]

use crate::abi::{r32, ret, set_fcc, set_fpr};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0035_8eb8, anim_combo_v15, "AnimCombo_v15")];

/// `cAnimComboNode` fields the clock reads and writes (offsets from the
/// generated body; the names are the reference's).
const ANIM_COMBO_V15_SPEED: u32 = 0x00; // AnimSpeed2 (float)
const ANIM_COMBO_V15_CURR2: u32 = 0x04; // CurrTime2 (float)
const ANIM_COMBO_V15_TO2: u32 = 0x0c; // ToTime2 (float): the clamp
const ANIM_COMBO_V15_ACTIVE2: u32 = 0x14; // AnimActive2 (byte)
const ANIM_COMBO_V15_COMPLETE2: u32 = 0x15; // AnimComplete2 (byte)
const ANIM_COMBO_V15_STOP: u32 = 0x16; // Stop (byte)
const ANIM_COMBO_V15_ANIM_TYPE: u32 = 0x18; // AnimType: the object the step runs on
const ANIM_COMBO_V15_DELAY: u32 = 0x24; // Delay (int, frames to wait)
const ANIM_COMBO_V15_CURR: u32 = 0x34; // CurrTime: the frame time
const ANIM_COMBO_V15_FRAME_END: u32 = 0x3c; // FrameEnd (float)
const ANIM_COMBO_V15_FRAME_START: u32 = 0x40; // FrameStart (float)
const ANIM_COMBO_V15_FLAGS: u32 = 0x5a; // Flags (halfword)
/// gp-relative 1e10f: the "no frame" sentinel that FrameStart/FrameEnd get.
const ANIM_COMBO_V15_SENTINEL_GP: u32 = 0x2a70;
/// Return addresses of the two calls (the generated `c.g[RA] = ...` lines).
const ANIM_COMBO_V15_RET_STEP: u32 = 0x0035_8f64;
const ANIM_COMBO_V15_RET_FRAME: u32 = 0x0035_8f4c;

fn anim_combo_v15_fpr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// The `Flags |= 1` tail: v0 holds the flags halfword; the function then
/// returns 1.
fn anim_combo_v15_mark_flags(c: &mut Cpu, s: u32) {
    c.cycles += 2;
    c.g[V0][0] = c.g[V0][0] | 0x1;
    let flags = c.g[V0][0];
    Addr(s + ANIM_COMBO_V15_FLAGS).sh(c, flags);
    c.cycles += 1;
    c.g[V0][0] = 0x1;
}

/// The clock, entered when `AnimComplete2` is 0. Steps the object (when
/// `AnimActive2` is 0), or counts `Delay` down and advances the clock once it
/// has run out.
fn anim_combo_v15_clock(c: &mut Cpu, s: u32) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(s + ANIM_COMBO_V15_ACTIVE2).lb(c);
    if c.g[V0][0] == 0 {
        // Not active: step the attached object once and return 1.
        c.cycles += 4;
        c.g[A0][0] = sx(s.wrapping_add(ANIM_COMBO_V15_ANIM_TYPE));
        Callee {
            addr: 0x0034_1d48,
            f: ee::s11::AnimObject_v15,
        }
        .call(c, ANIM_COMBO_V15_RET_STEP)?;
        c.g[V0][0] = 0x1;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(s + ANIM_COMBO_V15_DELAY).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_FLAGS).lhu(c);
        anim_combo_v15_mark_flags(c, s);
        return Ok(());
    }
    c.cycles += 2;
    let expired = (c.g[V0][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    if !expired {
        c.cycles += 2;
        let delay = c.g[V0][0];
        Addr(s + ANIM_COMBO_V15_DELAY).sw(c, delay);
        c.cycles += 1;
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_FLAGS).lhu(c);
        anim_combo_v15_mark_flags(c, s);
        return Ok(());
    }
    anim_combo_v15_advance(c, s)
}

/// The countdown has run out: advance `CurrTime2` by `AnimSpeed2`, clamp it at
/// `ToTime2` (completing on `Stop`, and deactivating), then run the frame
/// update. A zero return from the update ends the function with 0.
fn anim_combo_v15_advance(c: &mut Cpu, s: u32) -> Flow {
    c.cycles += 10;
    let curr2 = Addr(s + ANIM_COMBO_V15_CURR2).read_fpu(c);
    set_fpr(c, 0, curr2);
    let speed = Addr(s + ANIM_COMBO_V15_SPEED).read_fpu(c);
    set_fpr(c, 1, speed);
    let to2 = Addr(s + ANIM_COMBO_V15_TO2).read_fpu(c);
    set_fpr(c, 3, to2);
    let pos = anim_combo_v15_fpr(c, 0) + anim_combo_v15_fpr(c, 1);
    set_fpr(c, 0, pos);
    let frame_time = Addr(s + ANIM_COMBO_V15_CURR).read_fpu(c);
    set_fpr(c, 2, frame_time);
    Addr(s + ANIM_COMBO_V15_FRAME_START).write_fpu(c, frame_time);
    let reached = anim_combo_v15_fpr(c, 3).le(anim_combo_v15_fpr(c, 0));
    set_fcc(c, reached);
    Addr(s + ANIM_COMBO_V15_FRAME_END).write_fpu(c, frame_time);
    Addr(s + ANIM_COMBO_V15_CURR2).write_fpu(c, pos);
    if reached {
        c.cycles += 3;
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_STOP).lb(c);
        let stop = c.g[V0][0] != 0;
        Addr(s + ANIM_COMBO_V15_CURR2).write_fpu(c, anim_combo_v15_fpr(c, 3));
        if stop {
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            let complete = c.g[V0][0];
            Addr(s + ANIM_COMBO_V15_COMPLETE2).sb(c, complete);
        }
        c.cycles += 1;
        Addr(s + ANIM_COMBO_V15_ACTIVE2).sb(c, 0);
    }
    c.cycles += 4;
    c.g[A0][0] = sx(s.wrapping_add(ANIM_COMBO_V15_CURR));
    Callee {
        addr: 0x0034_eba0,
        f: ee::s11::AnimObject_FireEndScript,
    }
    .call(c, ANIM_COMBO_V15_RET_FRAME)?;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_FLAGS).lhu(c);
        anim_combo_v15_mark_flags(c, s);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    Ok(())
}

/// `int AnimCombo_v15(cAnimComboNode* self)` (0x358eb8): one step of a
/// combo's own clock. It first sets FrameStart and FrameEnd to the 1e10
/// sentinel. Unless `AnimComplete2` is set, it steps the object when the
/// combo is not active, and otherwise runs the clock: count `Delay` down,
/// then advance `CurrTime2` by `AnimSpeed2`, clamp at `ToTime2` (completing
/// on `Stop`), and run `cAnimNode_frameUpdate`. Returns 1, or 0 when the frame
/// update says the clock has finished.
pub fn anim_combo_v15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        let sentinel =
            Addr((c.g[GP][0] as u32).wrapping_sub(ANIM_COMBO_V15_SENTINEL_GP)).read_fpu(c);
        set_fpr(c, 0, sentinel);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = c.g[S0][0] as u32;
        Addr(s + ANIM_COMBO_V15_FRAME_START).write_fpu(c, anim_combo_v15_fpr(c, 0));
        c.g[V0][0] = Addr(s + ANIM_COMBO_V15_COMPLETE2).lb(c);
        let complete = c.g[V0][0] != 0;
        Addr(s + ANIM_COMBO_V15_FRAME_END).write_fpu(c, anim_combo_v15_fpr(c, 0));
        if complete {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            anim_combo_v15_clock(c, s)?;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the fakes below: they act only when this machine's RAM
    /// holds `OC608_MAGIC` at `OC608_CTRL`.
    const OC608_CTRL: u32 = DATA + 0x2_0000;
    const OC608_MAGIC: u32 = 0x0608_0608;
    static OC608_PREV_STEP: AtomicUsize = AtomicUsize::new(0);
    static OC608_PREV_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// Gated `AnimObject_v15`: sets the object's `v` (+0x6c) from the fixture,
    /// or unwinds when asked to.
    fn oc608_fake_step(c: &mut Cpu) {
        if c.bus.read32(OC608_CTRL) != OC608_MAGIC {
            return fakes::fall_back(c, &OC608_PREV_STEP, ee::s11::f_00341d48__generated);
        }
        let this = c.g[A0][0] as u32;
        let v = c.bus.read32(OC608_CTRL + 4);
        c.bus.write32(this.wrapping_add(0x6c), v);
        if c.bus.read32(OC608_CTRL + 16) != 0 {
            c.pc = 0xdead_0000;
            return;
        }
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// Gated `cAnimNode_frameUpdate` (`AnimObject_FireEndScript`): returns the
    /// fixture's result, or unwinds when asked to.
    fn oc608_fake_frame(c: &mut Cpu) {
        if c.bus.read32(OC608_CTRL) != OC608_MAGIC {
            return fakes::fall_back(c, &OC608_PREV_FRAME, ee::s11::f_0034eba0__generated);
        }
        c.g[V0][0] = sx(c.bus.read32(OC608_CTRL + 8));
        if c.bus.read32(OC608_CTRL + 12) != 0 {
            c.pc = 0xdead_0000;
            return;
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn oc608_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_00341D48),
            oc608_fake_step,
            &OC608_PREV_STEP,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0034EBA0),
            oc608_fake_frame,
            &OC608_PREV_FRAME,
        );
    }

    #[test]
    fn anim_combo_v15_matches() {
        oc608_install();
        check(ee::s11::f_00358eb8__generated, anim_combo_v15, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x358e_b8, 0);
            c.bus
                .write32(GAME_GP.wrapping_sub(0x2a70), 1e10f32.to_bits());
            let s = DATA + 0x100;
            fill(c, &mut r, s, 0x80);
            c.bus.write32(OC608_CTRL, OC608_MAGIC);
            c.bus.write32(OC608_CTRL + 4, r.f32(1.5));
            c.bus
                .write32(OC608_CTRL + 8, if seed % 5 == 0 { 0 } else { 1 + seed % 3 });
            c.bus.write32(OC608_CTRL + 12, (seed % 11 == 0) as u32);
            c.bus.write32(OC608_CTRL + 16, (seed % 17 == 0) as u32);
            c.bus
                .write32(s + 0x00, (0.25 + (seed % 4) as f32 * 0.5).to_bits());
            c.bus.write32(s + 0x04, ((seed % 5) as f32).to_bits());
            c.bus
                .write32(s + 0x0c, (1.0 + (seed % 6) as f32 * 1.5).to_bits());
            c.bus.write32(s + 0x34, ((seed % 3) as f32 * 2.0).to_bits());
            c.bus.write32(
                s + 0x24,
                match (seed / 4) % 4 {
                    0 => (-2i32) as u32,
                    2 => 3,
                    _ => 0,
                },
            );
            c.bus.write8(s + 0x14, if seed % 4 == 0 { 0 } else { 1 });
            c.bus.write8(s + 0x15, if seed % 9 == 0 { 1 } else { 0 });
            c.bus.write8(s + 0x16, ((seed / 2) % 2) as u8);
            c.g[A0][0] = sx(s);
            c.pc = 0x0035_8eb8;
        });
    }
}
