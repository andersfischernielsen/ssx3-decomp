//! Staging module oc976 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc976`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `lw off(reg)` (sign-extended, as the generated body loads it).
fn fa238_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `lbu off(reg)`.
fn fa238_lbu(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lbu(a)
}

/// `sw src, off(base)`.
fn fa238_sw(c: &mut Cpu, base: usize, off: u32, src: usize) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    c.sw(a, v);
}

/// `lw` of the world pointer at `gp - 0x848`.
fn fa238_world(c: &mut Cpu) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(0x848);
    c.lw(a)
}

/// `lw` of the word at `gp + 0x304` (the options-menu cursor object).
fn fa238_gp304(c: &mut Cpu) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_add(0x304);
    c.lw(a)
}

/// The shared epilogue. Level 0 enters at the label that restores `s0`, level
/// 1 at the one that restores `s1`, level 2 at the final restore of `s2`/`s3`/`ra`.
fn fa238_leave(c: &mut Cpu, frame: Frame, level: u8) {
    if level == 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x2e0);
    }
    if level <= 1 {
        c.cycles += 1;
        frame.restore(c, S1, 0x2d0);
    }
    c.cycles += 5;
    frame.restore(c, S2, 0x2c0);
    frame.restore(c, S3, 0x2b0);
    frame.restore_ra(c, 0x2a0);
    frame.close(c);
}

/// One `Is_FA238(widget->name, D_004A....)` test (`StrHash(name) == hash`):
/// loads `s0 = *(widget + 0x38)`, hashes the string at `0x4a0000 + off` and
/// returns true when the two differ. Not `last`: the delay slot reloads
/// `a0 = 0x4a0000`; `last`: it loads `v0 = world` instead.
fn fa238_name_miss(
    c: &mut Cpu,
    cycles: u64,
    off: u32,
    ret: u32,
    last: bool,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += cycles;
    c.g[A0][0] = 0x4a_0000;
    c.g[S0][0] = fa238_lw(c, S1, 0x38);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(off));
    if last {
        c.g[S2][0] = 0x53_0000;
    }
    call_ptr(c, 0x0031_7670, ret)?;
    let miss = c.g[S0][0] != c.g[V0][0];
    if last {
        c.g[V0][0] = fa238_world(c);
    } else {
        c.g[A0][0] = 0x4a_0000;
    }
    Ok(miss)
}

/// The inlined 0x288-byte copy of the options struct (`D_00535610`) to the
/// stack, 32 bytes per round (`regs` are the four registers the round loads
/// through, in the compiler's order for that site). The address is the constant
/// 0x535610, 8-byte aligned, so the original's unaligned `ldl/ldr` variant of the
/// loop (taken when `s0 & 7 != 0`) never runs and is not ported.
fn fa238_copy_config(c: &mut Cpu, regs: [usize; 4], enter: u64, set_s2: bool, head: u32) {
    c.cycles += enter;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
    if set_s2 {
        c.g[S2][0] = 0x53_0000;
    }
    loop {
        c.cycles += 11;
        for (i, &r) in regs.iter().enumerate() {
            let a = (c.g[S0][0] as u32).wrapping_add(8 * i as u32);
            c.g[r][0] = c.ld(a);
        }
        for (i, &r) in regs.iter().enumerate() {
            let a = (c.g[V1][0] as u32).wrapping_add(8 * i as u32);
            c.sd(a, c.g[r][0]);
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        let again = c.g[S0][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The 8-byte tail of that copy (`ldl/ldr` + `sdl/sdr` through register `reg`).
fn fa238_copy_tail(c: &mut Cpu, reg: usize) {
    let s0 = c.g[S0][0] as u32;
    let v1 = c.g[V1][0] as u32;
    c.ldl(reg, s0.wrapping_add(7));
    c.ldr(reg, s0);
    c.sdl(reg, v1.wrapping_add(7));
    c.sdr(reg, v1);
}

/// The shared tail of the two race-slot cases: `func_00161EF0(*(race_slot + 0xa8), a1)`
/// with the slot pointer in `a2`; then reload the world pointer.
fn fa238_race_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = fa238_lw(c, A2, 0xa8);
    call_ptr(c, 0x0016_1ef0, 0x001f_a400)?;
    c.g[V0][0] = fa238_world(c);
    Ok(())
}

/// `OVScreenBEOptions::OnWidgetAction(self /*a0*/, widget /*a1*/, event /*a2*/)`
/// (0x1fa238, `cOVState_PAUSE_OPTIONS::onWidgetEvent`): the pause-menu options
/// screen's widget handler. A null widget does nothing. Event 5 on the "back"
/// widget pushes a new screen state; event 6 refreshes the state stack; event 9
/// (a value changed) copies the widget's value into the options struct
/// (`D_00535610`: music/speech/sfx volume nibbles, two bits at +0x30 and the
/// two sound flags) and tells the audio module, forwards the two race-slot
/// values to the rider modules, adjusts the lobby value on the last widget when
/// the lobby flag at 0x534b30 is set, then calls slot 1 of the back-end
/// interface 7 vtable.
#[allow(dead_code)]
pub fn ov_screen_be_options_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x2f0);
        frame.save(c, S0, 0x2e0);
        frame.save(c, S1, 0x2d0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x2c0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x2b0);
        c.g[S2][0] = c.g[A0][0];
        let no_widget = c.g[S1][0] == 0;
        frame.save_ra(c, 0x2a0);
        if no_widget {
            fa238_leave(c, frame, 0);
            return Ok(());
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_dc80, 0x001f_a268)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x0014_dd58, 0x001f_a274)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = 0x6;
        let is_six = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x7) as u64;

        #[derive(PartialEq)]
        enum Event {
            Five,
            Six,
            Nine,
            Other,
        }
        let event = if is_six {
            Event::Six
        } else {
            c.cycles += 2;
            let at_least_seven = c.g[V0][0] == 0;
            c.g[V0][0] = 0x5;
            if at_least_seven {
                c.cycles += 3;
                c.g[V0][0] = 0x9;
                let is_nine = c.g[S0][0] == c.g[V0][0];
                frame.restore(c, S0, 0x2e0);
                if is_nine {
                    Event::Nine
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x2d0);
                    Event::Other
                }
            } else {
                c.cycles += 2;
                let is_five = c.g[S0][0] == c.g[V0][0];
                frame.restore(c, S0, 0x2e0);
                if is_five {
                    Event::Five
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x2d0);
                    Event::Other
                }
            }
        };

        match event {
            Event::Other => {
                fa238_leave(c, frame, 2);
                return Ok(());
            }
            Event::Six => {
                c.cycles += 6;
                c.g[A0][0] = fa238_lw(c, S2, 0x10);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_a340)?;
                frame.restore(c, S0, 0x2e0);
                fa238_leave(c, frame, 1);
                return Ok(());
            }
            Event::Five => {
                c.cycles += 6;
                c.g[A0][0] = 0x4a_0000;
                c.g[S0][0] = fa238_lw(c, S1, 0x38);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24e8));
                call_ptr(c, 0x0031_7670, 0x001f_a2c0)?;
                let miss = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, S0, 0x2e0);
                if !miss {
                    c.cycles += 26;
                    c.g[V1][0] = fa238_world(c);
                    c.g[A1][0] = 0x47_0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x640));
                    c.g[A0][0] = 0x27c;
                    c.g[V0][0] = fa238_lw(c, V1, 0x84);
                    c.g[A2][0] = 0x100;
                    c.g[A3][0] = 0;
                    c.g[S0][0] = fa238_lw(c, V0, 0x48);
                    call_ptr(c, 0x0031_7d70, 0x001f_a2ec)?;
                    c.g[A2][0] = fa238_lbu(c, S2, 0x14);
                    c.g[A3][0] = 0x2a;
                    c.g[A1][0] = fa238_lw(c, S2, 0x10);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0021_1088, 0x001f_a300)?;
                    c.g[V1][0] = fa238_lw(c, V0, 0x1c);
                    c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] &= c.g[A3][0];
                    c.g[A2][0] = 0;
                    c.g[V1][0] |= 0x100;
                    c.g[A3][0] = 0;
                    fa238_sw(c, V0, 0x1c, V1);
                    call_ptr(c, 0x0039_f320, 0x001f_a328)?;
                    frame.restore(c, S0, 0x2e0);
                }
                fa238_leave(c, frame, 1);
                return Ok(());
            }
            Event::Nine => {}
        }

        // Event 9: find which widget changed and store its value.
        'chain: {
            // D_004A2498: two bits of the options word.
            if !fa238_name_miss(c, 6, 0x2498, 0x001f_a358, false)? {
                c.cycles += 11;
                c.g[A1][0] = 0x53_0000;
                c.g[A0][0] = fa238_lbu(c, S1, 0x319);
                c.g[V1][0] = fa238_lw(c, A1, 0x5610);
                c.g[V0][0] = 0x3fff_0000;
                c.g[V0][0] |= 0xffff;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 30);
                c.g[V1][0] &= c.g[V0][0];
                c.g[S2][0] = 0x53_0000;
                c.g[V1][0] |= c.g[A0][0];
                fa238_sw(c, A1, 0x5610, V1);
                c.cycles += 1;
                c.g[V0][0] = fa238_world(c);
                break 'chain;
            }
            // D_004A24A0: race slot 0 value.
            if !fa238_name_miss(c, 5, 0x24a0, 0x001f_a398, false)? {
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[S2][0] = 0x53_0000;
                call_ptr(c, 0x0039_a738, 0x001f_a3ac)?;
                c.g[A0][0] = fa238_world(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = fa238_lw(c, A0, 0x84);
                c.g[V1][0] = fa238_lw(c, V0, 0x84);
                c.g[A2][0] = fa238_lw(c, V1, 0x4);
                fa238_race_tail(c)?;
                break 'chain;
            }
            // D_004A24A8: race slot 1 value.
            if !fa238_name_miss(c, 5, 0x24a8, 0x001f_a3d0, false)? {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                c.g[S2][0] = 0x53_0000;
                call_ptr(c, 0x0039_a738, 0x001f_a3e4)?;
                c.g[A0][0] = fa238_world(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = fa238_lw(c, A0, 0x84);
                c.g[V1][0] = fa238_lw(c, V0, 0x84);
                c.g[A2][0] = fa238_lw(c, V1, 0x8);
                fa238_race_tail(c)?;
                break 'chain;
            }
            // D_004A24B0: sfx volume (nibble 0).
            if !fa238_name_miss(c, 5, 0x24b0, 0x001f_a414, false)? {
                c.cycles += 14;
                c.g[A0][0] = 0x53_0000;
                c.g[V1][0] = fa238_lbu(c, S1, 0x74);
                c.g[V0][0] = fa238_lw(c, A0, 0x5610);
                c.g[A1][0] = 0xffff_ffff_ffff_fff0;
                c.g[V1][0] &= 0xf;
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] &= c.g[A1][0];
                c.g[V0][0] |= c.g[V1][0];
                fa238_sw(c, A0, 0x5610, V0);
                call_ptr(c, 0x0028_b180, 0x001f_a444)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = c.g[SP][0];
                fa238_copy_config(c, [A1, A2, A3, T0], 2, true, 0x001f_a4b8);
                c.cycles += 9;
                fa238_copy_tail(c, V0);
                c.g[A1][0] = fa238_lw(c, SP, 0);
                c.g[A1][0] &= 0xf;
                call_ptr(c, 0x0028_7410, 0x001f_a500)?;
                c.g[V0][0] = fa238_world(c);
                break 'chain;
            }
            // D_004A24B8: speech volume (nibble 2).
            if !fa238_name_miss(c, 5, 0x24b8, 0x001f_a514, false)? {
                c.cycles += 15;
                c.g[A0][0] = 0x53_0000;
                c.g[V1][0] = fa238_lbu(c, S1, 0x74);
                c.g[V0][0] = fa238_lw(c, A0, 0x5610);
                c.g[A1][0] = 0xffff_ffff_ffff_f0ff;
                c.g[V1][0] &= 0xf;
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5610));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                c.g[V0][0] &= c.g[A1][0];
                c.g[V0][0] |= c.g[V1][0];
                fa238_sw(c, A0, 0x5610, V0);
                call_ptr(c, 0x0028_b180, 0x001f_a548)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = c.g[SP][0];
                fa238_copy_config(c, [T0, A1, A2, A3], 2, true, 0x001f_a5bc);
                c.cycles += 10;
                fa238_copy_tail(c, T0);
                c.g[A1][0] = fa238_lw(c, SP, 0);
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[A1][0] &= 0xf;
                call_ptr(c, 0x0028_7488, 0x001f_a608)?;
                c.g[V0][0] = fa238_world(c);
                break 'chain;
            }
            // D_004A24C0: music volume (nibble 1).
            if !fa238_name_miss(c, 5, 0x24c0, 0x001f_a61c, false)? {
                c.cycles += 15;
                c.g[A0][0] = 0x53_0000;
                c.g[V1][0] = fa238_lbu(c, S1, 0x74);
                c.g[V0][0] = fa238_lw(c, A0, 0x5610);
                c.g[A1][0] = 0xffff_ffff_ffff_ff0f;
                c.g[V1][0] &= 0xf;
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5610));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[V0][0] &= c.g[A1][0];
                c.g[V0][0] |= c.g[V1][0];
                fa238_sw(c, A0, 0x5610, V0);
                call_ptr(c, 0x0028_b180, 0x001f_a650)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = c.g[SP][0];
                fa238_copy_config(c, [A3, T0, A1, A2], 2, true, 0x001f_a6c4);
                c.cycles += 9;
                fa238_copy_tail(c, A3);
                let low = c.lbu(sp(c));
                c.g[A1][0] = low;
                c.g[A1][0] >>= 4;
                call_ptr(c, 0x0028_7520, 0x001f_a70c)?;
                c.g[V0][0] = fa238_world(c);
                break 'chain;
            }
            // D_004A24C8: sound flag at bit 17.
            if !fa238_name_miss(c, 5, 0x24c8, 0x001f_a720, false)? {
                c.cycles += 17;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0039_a738, 0x001f_a730)?;
                c.g[A1][0] = 0x53_0000;
                c.g[A0][0] = 0xffff_ffff_fffd_0000;
                c.g[V1][0] = fa238_lw(c, A1, 0x5610);
                c.g[A0][0] |= 0xffff;
                c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5610));
                c.g[V1][0] &= c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 17);
                c.g[V1][0] |= c.g[V0][0];
                fa238_sw(c, A1, 0x5610, V1);
                call_ptr(c, 0x0028_b180, 0x001f_a75c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = c.g[SP][0];
                fa238_copy_config(c, [A2, A3, T0, A1], 2, true, 0x001f_a7d0);
                c.cycles += 10;
                fa238_copy_tail(c, A2);
                c.g[A1][0] = fa238_lw(c, SP, 0);
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
                c.g[A1][0] &= 0x1;
                call_ptr(c, 0x0028_7558, 0x001f_a81c)?;
                c.g[V0][0] = fa238_world(c);
                break 'chain;
            }
            // D_004A24D0: sound flag at bit 16.
            if !fa238_name_miss(c, 6, 0x24d0, 0x001f_a834, true)? {
                c.cycles += 17;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0039_a738, 0x001f_a844)?;
                c.g[A1][0] = 0x53_0000;
                c.g[A0][0] = 0xffff_ffff_fffe_0000;
                c.g[V1][0] = fa238_lw(c, A1, 0x5610);
                c.g[A0][0] |= 0xffff;
                c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5610));
                c.g[V1][0] &= c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[V1][0] |= c.g[V0][0];
                fa238_sw(c, A1, 0x5610, V1);
                call_ptr(c, 0x0028_b180, 0x001f_a870)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[V1][0] = c.g[SP][0];
                fa238_copy_config(c, [A1, A2, A3, T0], 1, false, 0x001f_a8dc);
                c.cycles += 7;
                fa238_copy_tail(c, V0);
                let half = c.lh(sp(c).wrapping_add(2));
                c.g[A1][0] = half;
                c.g[A1][0] &= 0x1;
                call_ptr(c, 0x0028_75d0, 0x001f_a924)?;
                c.cycles += 1;
                c.g[V0][0] = fa238_world(c);
                break 'chain;
            }
        }

        // Tell the back-end interface 7 (`world + 0x78`) and, with the lobby
        // flag set, update the lobby value of the last widget.
        c.cycles += 6;
        c.g[A1][0] = 0x7;
        c.g[A0][0] = fa238_lw(c, V0, 0x78);
        call_ptr(c, 0x0014_dd58, 0x001f_a934)?;
        c.g[V1][0] = fa238_lw(c, S2, 0x4b30);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = fa238_lw(c, S3, 0xc);
        } else {
            c.cycles += 6;
            c.g[A0][0] = 0x47_0000;
            c.g[S0][0] = fa238_lw(c, S1, 0x38);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x650));
            call_ptr(c, 0x0031_7670, 0x001f_a950)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[V0][0] = fa238_lw(c, S3, 0xc);
            } else {
                c.cycles += 16;
                c.g[A0][0] = fa238_gp304(c);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x290));
                c.g[A2][0] = sx(sp(c).wrapping_add(0x294));
                call_ptr(c, 0x0026_6f90, 0x001f_a968)?;
                c.g[V0][0] = fa238_lw(c, S1, 0x74);
                c.g[V1][0] = 0xa;
                c.g[A0][0] = fa238_gp304(c);
                c.mult(0, 2, 2, 3);
                c.g[A2][0] = c.lw(sp(c).wrapping_add(0x294));
                c.g[A1][0] = c.g[V0][0];
                c.sw(sp(c).wrapping_add(0x290), c.g[V0][0]);
                call_ptr(c, 0x0026_6f58, 0x001f_a988)?;
                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x290));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[A0][0] = fa238_gp304(c);
                    call_ptr(c, 0x0026_6e30, 0x001f_a9b0)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = fa238_gp304(c);
                    call_ptr(c, 0x0026_6e88, 0x001f_a9a0)?;
                }
                c.g[V0][0] = fa238_lw(c, S3, 0xc);
            }
        }
        // Slot 1 of the interface vtable: `vt[1].fn(iface + vt[1].delta)`.
        c.cycles += 4;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = fa238_lw(c, V0, 0xc);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_a9c4)?;
        fa238_leave(c, frame, 0);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::RA as DT_RA;
    use crate::difftest::{elf_image, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC976_BASE: u32 = DATA + 0x20_0000;
    pub(crate) const OC976_CTRL: u32 = OC976_BASE;
    pub(crate) const OC976_MAGIC: u32 = 0x4f43_3937;
    pub(crate) const OC976_SELF: u32 = OC976_BASE + 0x1000;
    pub(crate) const OC976_WIDGET: u32 = OC976_BASE + 0x1200;
    pub(crate) const OC976_WORLD: u32 = OC976_BASE + 0x3000;
    pub(crate) const OC976_X: u32 = OC976_BASE + 0x3100;
    pub(crate) const OC976_STACK: u32 = OC976_BASE + 0x3200;
    pub(crate) const OC976_Y: u32 = OC976_BASE + 0x3300;
    pub(crate) const OC976_P1: u32 = OC976_BASE + 0x3400;
    pub(crate) const OC976_P2: u32 = OC976_BASE + 0x3500;
    pub(crate) const OC976_IFACE: u32 = OC976_BASE + 0x3800;
    pub(crate) const OC976_VT: u32 = OC976_BASE + 0x3900;
    /// `D_00535610` (the options struct) and the lobby flag `D_00534B30`.
    pub(crate) const OC976_CFG: u32 = 0x0053_5610;
    pub(crate) const OC976_FLAG: u32 = 0x0053_4b30;
    /// The strings of the case-9 chain, then the lobby widget's name.
    pub(crate) const OC976_NAMES: [u32; 9] = [
        0x004a_2498,
        0x004a_24a0,
        0x004a_24a8,
        0x004a_24b0,
        0x004a_24b8,
        0x004a_24c0,
        0x004a_24c8,
        0x004a_24d0,
        0x0046_f9b0,
    ];
    /// The name event 5 compares against.
    pub(crate) const OC976_BACK: u32 = 0x004a_24e8;

    /// The mailbox word of the fake for `addr` (the result it returns, +3).
    pub(crate) fn oc976_mail(addr: u32) -> u32 {
        OC976_BASE + 0x10_0000 + (addr & 0xf_fffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's own pointer).
    pub(crate) fn oc976_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc976_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    macro_rules! oc976_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC976_CTRL) != OC976_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc976_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc976_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc976_fakes!(
        (
            oc976_fake_14dc80,
            OC976_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc976_fake_14dd58,
            OC976_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc976_fake_161ef0,
            OC976_PREV_161EF0,
            ee::s01::OVERRIDE_00161EF0,
            ee::s01::f_00161ef0__generated,
            0x0016_1ef0
        ),
        (
            oc976_fake_211088,
            OC976_PREV_211088,
            ee::s05::OVERRIDE_00211088,
            ee::s05::f_00211088__generated,
            0x0021_1088
        ),
        (
            oc976_fake_266e30,
            OC976_PREV_266E30,
            ee::s07::OVERRIDE_00266E30,
            ee::s07::f_00266e30__generated,
            0x0026_6e30
        ),
        (
            oc976_fake_266e88,
            OC976_PREV_266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x0026_6e88
        ),
        (
            oc976_fake_266f58,
            OC976_PREV_266F58,
            ee::s07::OVERRIDE_00266F58,
            ee::s07::f_00266f58__generated,
            0x0026_6f58
        ),
        (
            oc976_fake_266f90,
            OC976_PREV_266F90,
            ee::s07::OVERRIDE_00266F90,
            ee::s07::f_00266f90__generated,
            0x0026_6f90
        ),
        (
            oc976_fake_287410,
            OC976_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc976_fake_287488,
            OC976_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc976_fake_287520,
            OC976_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc976_fake_287558,
            OC976_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc976_fake_2875d0,
            OC976_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
        (
            oc976_fake_28b180,
            OC976_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc976_fake_317d70,
            OC976_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc976_fake_39a738,
            OC976_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc976_fake_39f190,
            OC976_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc976_fake_39f320,
            OC976_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039_f320
        ),
    );

    /// `StrHash` of the string at `s` (the native helper over the bytes in RAM).
    pub(crate) fn oc976_hash(c: &mut Cpu, s: u32) -> u32 {
        let mut bytes = Vec::new();
        loop {
            let b = c.bus.read8(s + bytes.len() as u32);
            if b == 0 {
                break;
            }
            bytes.push(b);
        }
        crate::game::hashvalue::str_hash(&bytes)
    }

    /// Build the world for one call: `widget` null or not, `event`, the widget's
    /// name hash from the string at `name` (none: a random hash) and the lobby
    /// flag. `calls` is the number of fake calls a seed may unwind in.
    pub(crate) fn oc976_fixture(
        c: &mut Cpu,
        seed: u32,
        event: u64,
        null_widget: bool,
        name: Option<u32>,
        flag: bool,
        calls: u32,
    ) {
        let mut r = machine(c, seed, 0x1fa238, calls);
        elf_image(c);
        oc976_install_fakes();
        c.bus.write32(OC976_CTRL, OC976_MAGIC);
        let hash = match name {
            Some(s) => oc976_hash(c, s),
            None => r.next_u32(),
        };
        // The world chain: world -> +0x84 -> (+0x48 state stack, +0x84 -> race slots).
        c.bus.write32(GAME_GP - 0x848, OC976_WORLD);
        c.bus.write32(OC976_WORLD + 0x78, r.next_u32());
        c.bus.write32(OC976_WORLD + 0x84, OC976_X);
        c.bus.write32(OC976_X + 0x48, OC976_STACK);
        c.bus.write32(OC976_X + 0x84, OC976_Y);
        c.bus.write32(OC976_Y + 4, OC976_P1);
        c.bus.write32(OC976_Y + 8, OC976_P2);
        c.bus.write32(OC976_P1 + 0xa8, r.next_u32());
        c.bus.write32(OC976_P2 + 0xa8, r.next_u32());
        c.bus.write32(GAME_GP + 0x304, r.next_u32());
        // The back-end interface: vtable slot 1 calls a fake.
        c.bus.write32(oc976_mail(0x0014_dd58), OC976_IFACE + 3);
        c.bus.write32(OC976_IFACE + 0xc, OC976_VT);
        c.bus.write16(OC976_VT + 8, r.next_u32() as u16 & !3);
        c.bus.write32(OC976_VT + 0xc, fakes::FAKE);
        // The options struct and the lobby flag.
        for i in 0..0xa4 {
            c.bus.write32(OC976_CFG + 4 * i, r.next_u32());
        }
        c.bus
            .write32(OC976_FLAG, if flag { r.next_u32() | 1 } else { 0 });
        // `self` and the widget.
        c.bus.write32(OC976_SELF + 0x10, r.next_u32());
        c.bus.write8(OC976_SELF + 0x14, r.next_u32() as u8);
        c.bus.write32(OC976_WIDGET + 0x38, hash);
        c.bus.write8(OC976_WIDGET + 0x319, r.next_u32() as u8);
        let value = match (seed / 34) % 3 {
            0 => 0,
            1 => 7,
            _ => 0xffff_ffff,
        };
        c.bus.write32(
            OC976_WIDGET + 0x74,
            if name == Some(OC976_NAMES[8]) {
                value
            } else {
                r.next_u32()
            },
        );
        c.g[A0][0] = OC976_SELF as u64;
        c.g[A1][0] = if null_widget { 0 } else { OC976_WIDGET as u64 };
        c.g[A2][0] = event;
        c.g[RA][0] = DT_RA as u64;
    }

    /// The scenario of a seed: (event, null widget, name, lobby flag).
    pub(crate) fn oc976_scenario(seed: u32) -> (u64, bool, Option<u32>, bool) {
        let flag = (seed / 17) % 2 == 1;
        match seed % 17 {
            0 => (9, true, None, flag),
            1 => (5, false, Some(OC976_BACK), flag),
            2 => (5, false, None, flag),
            3 => (6, false, None, flag),
            4 => (3, false, None, flag),
            5 => (8, false, None, flag),
            6 => (0xffff_fff0, false, None, flag),
            n @ 7..=15 => (9, false, Some(OC976_NAMES[(n - 7) as usize]), flag),
            _ => (9, false, None, flag),
        }
    }
}
