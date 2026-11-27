//! Staging module oc944 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc944`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001d_f050,
    cfe_popup_screen_pos_update,
    "cFEPopupScreenPos_Update",
)];

/// `s0`..`s7` and the like: the low word of a callee-saved register.
#[inline]
fn oc944_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// The `sUISound` message call of the popup: `p = *(owner + 0x14)`, falling
/// back to the global sound object in `s4`; `p->vt[2](p, msg)` when `p != 0`.
/// `first` is the block charge before the lookups, `ret` the return address.
fn oc944_sound(c: &mut Cpu, msg: u64, first: u64, ret: u32) -> Flow {
    c.cycles += first;
    let s1 = oc944_r(c, S1);
    let owner = Addr(s1.wrapping_add(0x10)).lw(c);
    c.g[V0][0] = owner;
    c.g[A2][0] = Addr((owner as u32).wrapping_add(0x14)).lw(c);
    if c.g[A2][0] == 0 {
        c.g[A2][0] = c.g[S4][0];
    }
    let none = c.g[A2][0] == 0;
    c.g[A1][0] = msg;
    if !none {
        c.cycles += 5;
        let p = oc944_r(c, A2);
        let vt = Addr(p).lw(c);
        c.g[V0][0] = vt;
        c.g[A0][0] = Addr((vt as u32).wrapping_add(0x10)).lh(c);
        let f = Addr((vt as u32).wrapping_add(0x14)).lw(c);
        c.g[V1][0] = f;
        c.g[A0][0] = sx(p.wrapping_add(oc944_r(c, A0)));
        call_ptr(c, f as u32, ret)?;
    }
    Ok(())
}

/// A virtual call on the input object `s0`: `v1 = in->vt`; `a0 = s0 + vt[off]`
/// (halfword delta), call `vt[off + 4]`.
fn oc944_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    let s0 = oc944_r(c, S0);
    let vt = Addr(s0.wrapping_add(8)).lw(c);
    c.g[V1][0] = vt;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(off)).lh(c);
    let f = Addr((vt as u32).wrapping_add(off + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(s0.wrapping_add(oc944_r(c, A0)));
    call_ptr(c, f as u32, ret)
}

/// `cFEPopupScreenPos_Update` (0x1df050): the screen-position popup (the
/// "adjust the picture" calibration). For each pad `i` in 0..2 (until one
/// reports activity): accept (pad 5, sound 6, accepted = 1) or cancel (pad 6,
/// sound 3, accepted = 0) set flag `0x80` and call `func_0039f190(owner + 0x18, 1)`;
/// the four direction pads move `x`/`y` by one and play the matching frame; then
/// the position is clamped to +-20 (sound 4 when it was outside) and handed to
/// the renderer's slot 12 (`*(gp - 0x854) + 0x10d8`).
pub fn cfe_popup_screen_pos_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff;
        frame.save(c, S0, 0x50);
        c.g[S4][0] = sx(oc944_r(c, GP).wrapping_add(0x2968));
        frame.save_ra(c, 0);
        let s1 = oc944_r(c, S1);
        let owner = Addr(s1.wrapping_add(0x10)).lw(c);
        c.g[V0][0] = owner;
        c.g[S0][0] = Addr((owner as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = Addr(oc944_r(c, S0).wrapping_add(8)).lw(c);
        'iter: loop {
            // pad->vt[44](pad, i)
            c.cycles += 12;
            c.g[A1][0] = c.g[S2][0] & 0xff;
            let vt = oc944_r(c, V0);
            c.g[A0][0] = Addr(vt.wrapping_add(0x160)).lh(c);
            c.g[A2][0] = Addr(vt.wrapping_add(0x164)).lw(c);
            let f = oc944_r(c, A2);
            c.g[A0][0] = sx(oc944_r(c, S0).wrapping_add(oc944_r(c, A0)));
            call_ptr(c, f, 0x001d_f09c)?;
            // if (pad->vt[40](pad) == 0) continue
            oc944_vcall(c, 0x140, 0x001d_f0b0)?;
            if c.g[V0][0] == 0 {
                c.g[S2][0] = sx(oc944_r(c, S2).wrapping_add(1));
            } else {
                oc944_handle_pad(c)?;
                frame.restore(c, S0, 0x50);
                break 'iter;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(oc944_r(c, S0).wrapping_add(8)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x001d_f088);
                }
                continue 'iter;
            } else {
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
                break 'iter;
            }
        }
        c.cycles += 7;
        frame.restore(c, S4, 0x10);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The part of the update after pad `i` reported input: the buttons, the
/// clamp and the renderer call. Ends with `s0` still to be restored by the
/// caller (the generated `lq s0` sits after the renderer call).
fn oc944_handle_pad(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    let s0 = oc944_r(c, S0);
    c.g[V1][0] = Addr(s0.wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr(oc944_r(c, V1).wrapping_add(0x28)).lh(c);
    let f = Addr(oc944_r(c, V1).wrapping_add(0x2c)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(s0.wrapping_add(oc944_r(c, A0)));
    call_ptr(c, f as u32, 0x001d_f0cc)?;
    let mut done = false;
    if c.g[V0][0] != 0 {
        // accept: sound 6, accepted = 1
        oc944_sound(c, 6, 5, 0x001d_f0fc)?;
        c.cycles += 7;
        let s1 = oc944_r(c, S1);
        c.g[V0][0] = Addr(s1.wrapping_add(0x1c)).lw(c);
        c.g[V1][0] = 1;
        c.g[A0][0] = Addr(s1.wrapping_add(0x10)).lw(c);
        c.g[A1][0] = 1;
        c.g[V0][0] |= 0x80;
        let v1 = c.g[V1][0];
        Addr(s1.wrapping_add(0x50)).sb(c, v1);
        oc944_notify(c)?;
        done = true;
    } else {
        c.g[V1][0] = Addr(oc944_r(c, S0).wrapping_add(8)).lw(c);
        c.cycles += 6;
        let v1 = oc944_r(c, V1);
        c.g[A0][0] = Addr(v1.wrapping_add(0x30)).lh(c);
        let f = Addr(v1.wrapping_add(0x34)).lw(c);
        c.g[V0][0] = f;
        c.g[A0][0] = sx(oc944_r(c, S0).wrapping_add(oc944_r(c, A0)));
        call_ptr(c, f as u32, 0x001d_f128)?;
        if c.g[V0][0] != 0 {
            // cancel: sound 3, accepted = 0
            oc944_sound(c, 3, 5, 0x001d_f158)?;
            c.cycles += 5;
            let s1 = oc944_r(c, S1);
            c.g[V0][0] = Addr(s1.wrapping_add(0x1c)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(s1.wrapping_add(0x10)).lw(c);
            c.g[V0][0] |= 0x80;
            Addr(s1.wrapping_add(0x50)).sb(c, 0);
            oc944_notify(c)?;
            done = true;
        } else {
            c.g[V1][0] = Addr(oc944_r(c, S0).wrapping_add(8)).lw(c);
        }
    }
    if !done {
        oc944_direction(c)?;
    }
    oc944_clamp_and_report(c)
}

/// Store the flags and call `func_0039f190(owner + 0x18, 1)`; then reload `x`.
fn oc944_notify(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let s1 = oc944_r(c, S1);
    let v0 = c.g[V0][0];
    Addr(s1.wrapping_add(0x1c)).sw(c, v0);
    c.g[A0][0] = sx(oc944_r(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f190, 0x001d_f178)?;
    c.g[V0][0] = Addr(oc944_r(c, S1).wrapping_add(0x48)).lw(c);
    Ok(())
}

/// The "play the move animation" tail of a direction pad: `a0` holds the
/// label string; `frame = getFrameByLabel(screen, hash(label))`, then
/// `playFrame(screen, frame, 1)` unless the frame is `0xffff`. Then `v0 = x`.
/// `rets` are the three return addresses, `charges` the block charges before
/// the hash call and before the play call, `tail` the extra charge of the
/// play path before it rejoins.
fn oc944_play(c: &mut Cpu, rets: [u32; 3], charges: [u64; 2], tail: u64) -> Flow {
    c.cycles += charges[0];
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = Addr(oc944_r(c, S1).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_c778, rets[1])?;
    let same = c.g[V0][0] == c.g[S3][0];
    c.g[A1][0] = c.g[V0][0];
    if same {
        c.cycles += 1;
    } else {
        c.cycles += charges[1];
        c.g[A0][0] = Addr(oc944_r(c, S1).wrapping_add(0x40)).lw(c);
        c.g[A2][0] = 1;
        call_ptr(c, 0x0039_c870, rets[2])?;
        c.cycles += tail;
    }
    c.g[V0][0] = Addr(oc944_r(c, S1).wrapping_add(0x48)).lw(c);
    Ok(())
}

/// One direction pad's virtual call (`v1` = the pad's vtable): block charge 6,
/// `vt[off]` / `vt[off + 4]`, result in `v0`.
fn oc944_dir_call(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.cycles += 6;
    let v1 = oc944_r(c, V1);
    c.g[A0][0] = Addr(v1.wrapping_add(off)).lh(c);
    let f = Addr(v1.wrapping_add(off + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(oc944_r(c, S0).wrapping_add(oc944_r(c, A0)));
    call_ptr(c, f as u32, ret)
}

/// Reload `v1 = pad->vt` after a direction pad that reported nothing.
fn oc944_reload_vt(c: &mut Cpu) {
    c.g[V1][0] = Addr(oc944_r(c, S0).wrapping_add(8)).lw(c);
}

/// The four direction pads: 19 = x-1 (string `0x4a1550`), 20 = x+1
/// (`0x45ddf0`), 17 = y+1 (`0x4a1558`), 18 = y-1 (`0x4a1560`); the first that
/// fires changes the position and plays its frame. Leaves `v0 = x`.
fn oc944_direction(c: &mut Cpu) -> Flow {
    let s1 = oc944_r(c, S1);
    oc944_dir_call(c, 0x98, 0x001d_f190)?;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = Addr(s1.wrapping_add(0x48)).lw(c);
        c.g[A0][0] = sx(0x4a_0000u32.wrapping_add(0x1550));
        c.g[V0][0] = sx(oc944_r(c, V0).wrapping_sub(1));
        let v0 = c.g[V0][0];
        Addr(s1.wrapping_add(0x48)).sw(c, v0);
        return oc944_play(c, [0x001d_f214, 0x001d_f220, 0x001d_f234], [7, 5], 0);
    }
    oc944_reload_vt(c);
    oc944_dir_call(c, 0xa0, 0x001d_f1c0)?;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = Addr(s1.wrapping_add(0x48)).lw(c);
        c.g[A0][0] = sx(0x46_0000u32.wrapping_sub(0x2210));
        c.g[V0][0] = sx(oc944_r(c, V0).wrapping_add(1));
        let v0 = c.g[V0][0];
        Addr(s1.wrapping_add(0x48)).sw(c, v0);
        return oc944_play(c, [0x001d_f214, 0x001d_f220, 0x001d_f234], [7, 5], 0);
    }
    oc944_reload_vt(c);
    oc944_dir_call(c, 0x88, 0x001d_f1f0)?;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(s1.wrapping_add(0x4c)).lw(c);
        c.g[A0][0] = sx(0x4a_0000u32.wrapping_add(0x1558));
        c.g[V0][0] = sx(oc944_r(c, V0).wrapping_add(1));
        let v0 = c.g[V0][0];
        Addr(s1.wrapping_add(0x4c)).sw(c, v0);
        return oc944_play(c, [0x001d_f214, 0x001d_f220, 0x001d_f234], [7, 5], 0);
    }
    oc944_reload_vt(c);
    oc944_dir_call(c, 0x90, 0x001d_f24c)?;
    c.g[A0][0] = 0x4a_0000;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(s1.wrapping_add(0x48)).lw(c);
        return Ok(());
    }
    c.cycles += 10;
    c.g[V0][0] = Addr(s1.wrapping_add(0x4c)).lw(c);
    c.g[A0][0] = sx(oc944_r(c, A0).wrapping_add(0x1560));
    c.g[V0][0] = sx(oc944_r(c, V0).wrapping_sub(1));
    let v0 = c.g[V0][0];
    Addr(s1.wrapping_add(0x4c)).sw(c, v0);
    oc944_play(c, [0x001d_f268, 0x001d_f274, 0x001d_f288], [0, 3], 1)
}

/// Clamp `x`/`y` to +-20 (sound 4 when either was outside) and call the
/// renderer's slot 12 with them. `v0 = x` on entry.
fn oc944_clamp_and_report(c: &mut Cpu) -> Flow {
    let s1 = oc944_r(c, S1);
    c.cycles += 4;
    c.g[V0][0] = sx(oc944_r(c, V0).wrapping_add(0x14));
    c.g[V0][0] = (c.g[V0][0] < 0x29) as u64;
    let mut sound = false;
    if c.g[V0][0] == 0 {
        sound = true;
    } else {
        c.cycles += 4;
        c.g[V1][0] = Addr(s1.wrapping_add(0x4c)).lw(c);
        c.g[V0][0] = ((c.g[V1][0] as i64) < -20) as u64;
        if c.g[V0][0] != 0 {
            sound = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 21) as u64;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = Addr(s1.wrapping_add(0x48)).lw(c);
            } else {
                c.cycles += 1;
                sound = true;
            }
        }
    }
    if sound {
        oc944_sound(c, 4, 4, 0x001d_f2e0)?;
        c.cycles += 1;
        c.g[V1][0] = Addr(s1.wrapping_add(0x48)).lw(c);
    }
    // x = clamp(x, -20, 20)
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V1][0] as i64) < -20) as u64;
    let low = c.g[V0][0] != 0;
    c.g[A0][0] = 0x14;
    if low {
        c.cycles += 1;
        c.g[V1][0] = 0xffff_ffff_ffff_ffec;
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[A0][0];
        }
    }
    // y = clamp(y, -20, 20); x stored in the delay slot of the first compare
    c.cycles += 5;
    c.g[A2][0] = Addr(s1.wrapping_add(0x4c)).lw(c);
    c.g[A0][0] = 0x14;
    c.g[V0][0] = ((c.g[A2][0] as i64) < -20) as u64;
    let low = c.g[V0][0] != 0;
    let v1 = c.g[V1][0];
    Addr(s1.wrapping_add(0x48)).sw(c, v1);
    if low {
        c.cycles += 1;
        c.g[A2][0] = 0xffff_ffff_ffff_ffec;
    } else {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = c.g[A0][0];
        }
    }
    c.cycles += 10;
    let a2 = c.g[A2][0];
    Addr(s1.wrapping_add(0x4c)).sw(c, a2);
    let a3 = Addr(oc944_r(c, GP).wrapping_sub(0x854)).lw(c);
    c.g[A3][0] = a3;
    c.g[A1][0] = Addr(s1.wrapping_add(0x48)).lw(c);
    let vt = Addr((a3 as u32).wrapping_add(0x10d8)).lw(c);
    c.g[V0][0] = vt;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(0x60)).lh(c);
    let f = Addr((vt as u32).wrapping_add(0x64)).lw(c);
    c.g[V1][0] = f;
    c.g[A0][0] = sx((a3 as u32).wrapping_add(oc944_r(c, A0)));
    call_ptr(c, f as u32, 0x001d_f344)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};

    const OC944_SELF: u32 = DATA + 0x1000;
    const OC944_OWNER: u32 = DATA + 0x2000;
    const OC944_PAD: u32 = DATA + 0x3000;
    const OC944_SOUND: u32 = DATA + 0x4000;
    const OC944_REND: u32 = DATA + 0x5000;
    const OC944_RVT: u32 = DATA + 0x6000;
    const OC944_PADVT: u32 = DATA + 0x7000;
    const OC944_SVT: u32 = DATA + 0x8000;
    const OC944_GSOUND: u32 = DATA + 0x9000;
    /// A real leaf returning 0 (`jr ra; move v0, zero`).
    const OC944_ZERO: u32 = 0x0014_06c0;

    /// Fixture: a popup whose pad object has virtual slots that return 0 (a
    /// real leaf) or non-zero (a fake call), random positions around the
    /// clamp range, a sound object that is sometimes missing, and fake
    /// results that sometimes force the "frame not found" (0xffff) branch.
    fn oc944_setup(c: &mut Cpu, seed: u32) {
        let mut r: Rng = machine(c, seed, 0x1df0_50, 8);
        let bits = r.next_u32();
        c.bus.write32(OC944_SELF + 0x10, OC944_OWNER);
        c.bus.write32(OC944_OWNER + 0xc, OC944_PAD);
        c.bus.write32(OC944_PAD + 8, OC944_PADVT);
        let sound = if bits & 1 == 0 { OC944_SOUND } else { 0 };
        c.bus.write32(OC944_OWNER + 0x14, sound);
        for o in [OC944_SOUND, GAME_GP + 0x2968] {
            c.bus.write32(o, OC944_SVT);
        }
        c.bus.write16(OC944_SVT + 0x10, 0x20);
        c.bus.write32(OC944_SVT + 0x14, FAKE + 0x100);
        // pad vtable: slot 44 (set input) is a fake; the others by seed
        c.bus.write16(OC944_PADVT + 0x160, 0x10);
        c.bus.write32(OC944_PADVT + 0x164, FAKE);
        // which pads report: pad 40 first (0 on iteration 0 sometimes)
        for (n, off) in [0x140u32, 0x28, 0x30, 0x98, 0xa0, 0x88, 0x90]
            .into_iter()
            .enumerate()
        {
            let on = if n == 0 {
                (r.next_u32() % 4) != 0
            } else {
                r.next_u32() % 3 == 0
            };
            c.bus.write16(OC944_PADVT + off, 0x30 + 8 * n as u16);
            c.bus.write32(
                OC944_PADVT + off + 4,
                if on {
                    FAKE + 0x10 * (n as u32 + 1)
                } else {
                    OC944_ZERO
                },
            );
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x854), OC944_REND);
        c.bus.write32(OC944_REND + 0x10d8, OC944_RVT);
        c.bus.write16(OC944_RVT + 0x60, 0x18);
        c.bus.write32(OC944_RVT + 0x64, FAKE + 0x200);
        c.bus.write32(OC944_SELF + 0x1c, r.next_u32());
        c.bus.write32(OC944_SELF + 0x40, OC944_GSOUND);
        let pos = |r: &mut Rng| (r.next_u32() % 49) as i32 - 24;
        c.bus.write32(OC944_SELF + 0x48, pos(&mut r) as u32);
        c.bus.write32(OC944_SELF + 0x4c, pos(&mut r) as u32);
        if bits & 2 != 0 {
            for n in 0..8u32 {
                c.bus.write32(LOG_RESULTS + 4 * n, 0xffff);
            }
        }
        c.g[A0][0] = sx(OC944_SELF);
    }

    /// `cFEPopupScreenPos_Update` against the generated body: all pad
    /// combinations, in and out of range positions, a missing sound object
    /// and unwinding callees.
    #[test]
    fn cfe_popup_screen_pos_update_matches() {
        crate::difftest::fakes::covered(
            ee::s04::f_001df050__generated,
            cfe_popup_screen_pos_update,
            400,
            &oc944_setup,
        );
    }
}
