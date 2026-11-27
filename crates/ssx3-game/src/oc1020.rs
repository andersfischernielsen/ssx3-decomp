//! Staging module oc1020 (wave wave262): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1020`.

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
    0x001f_a238,
    ov_screen_be_options_on_widget_action,
    "OVScreenBEOptions_OnWidgetAction",
)];

/// Address of the global options record (`D_00535610`): bit fields for the
/// race-view flag, volumes and the two audio switches, copied to the stack by
/// `Cfg_FA238()` (0x288 bytes).
#[cfg(test)]
pub(crate) const OVBEO_CONFIG: u32 = 0x0053_5610;

/// `void OVScreenBEOptions_OnWidgetAction(self, widget, event)` (0x1fa238),
/// the pause-menu options screen (`cOVState_PAUSE_OPTIONS::onWidgetEvent`).
/// Event 5 opens the 0x27c-byte sub-screen when the widget is the entry
/// `D_004A24E8`, event 6 pops the screen's state, event 9 applies a changed
/// widget: the radio entry `D_004A2498` (config bits 30..31), two race values
/// (`D_004A24A0`/`A8`, through the world's race object), the three volumes
/// (config nibbles 0, 8, 4) and the two switches (bits 17 and 16), each
/// pushed to the sound manager from a stack copy of the config; the
/// `D_0046F9B0` slider sets the 0x266f58 value (`widget+0x74 * 10`). Event 9
/// finishes with the vtable call of the interface's slot 1. Widgets are
/// identified by comparing `widget+0x38` with the result of 0x317670 (the
/// string hash) of the entry's name.
pub fn ov_screen_be_options_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[S2][0] = c.g[A0][0];
        let no_widget = c.g[S1][0] == 0;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x2a0), c.g[RA][0]);
        // Exits: `Full` restores s0 and s1 (cycles 1 + 1), `AfterS0` only s1
        // (cycle 1), `Direct` neither.
        let exit = if no_widget {
            Exit::Full
        } else {
            ovbeo_dispatch(c)?
        };
        match exit {
            Exit::Full => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                c.cycles += 1;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            }
            Exit::AfterS0 => {
                c.cycles += 1;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            }
            Exit::Direct => {}
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2a0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.pc = t;
        Ok(())
    })
}

/// How far the epilogue's register restores were already done.
enum Exit {
    Full,
    AfterS0,
    Direct,
}

/// The event switch (s0 = event, s1 = widget, s2 = self).
fn ovbeo_dispatch(c: &mut Cpu) -> Result<Exit, crate::call::Unwound> {
    c.cycles += 9;
    call_ptr(c, 0x0014_dc80, 0x001f_a268)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x4;
    call_ptr(c, 0x0014_dd58, 0x001f_a274)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[V0][0] = 0x6;
    let is_six = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[S0][0] < 0x7u64) as u64;
    if is_six {
        ovbeo_event6(c)?;
        return Ok(Exit::AfterS0);
    }
    c.cycles += 2;
    let big = c.g[V0][0] == 0;
    c.g[V0][0] = 0x5;
    if big {
        c.cycles += 3;
        c.g[V0][0] = 0x9;
        let is_nine = c.g[S0][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        if is_nine {
            ovbeo_event9(c)?;
            return Ok(Exit::Full);
        }
        c.cycles += 2;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        return Ok(Exit::Direct);
    }
    c.cycles += 2;
    let is_five = c.g[S0][0] == c.g[V0][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
    if is_five {
        ovbeo_event5(c)?;
        return Ok(Exit::AfterS0);
    }
    c.cycles += 2;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
    Ok(Exit::Direct)
}

/// Event 5: open the sub-screen if the widget is `D_004A24E8`.
fn ovbeo_event5(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = 0x4a0000;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24e8));
    call_ptr(c, 0x0031_7670, 0x001f_a2c0)?;
    let other = c.g[S0][0] != c.g[V0][0];
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
    if other {
        return Ok(());
    }
    c.cycles += 26;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = 0x470000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x640));
    c.g[A0][0] = 0x27c;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[A2][0] = 0x100;
    c.g[A3][0] = 0;
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
    call_ptr(c, 0x0031_7d70, 0x001f_a2ec)?;
    c.g[A2][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x14));
    c.g[A3][0] = 0x2a;
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0021_1088, 0x001f_a300)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
    c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
    c.g[A1][0] = c.g[V0][0];
    c.g[V1][0] &= c.g[A3][0];
    c.g[A2][0] = 0;
    c.g[V1][0] |= 0x100;
    c.g[A3][0] = 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
    call_ptr(c, 0x0039_f320, 0x001f_a328)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
    Ok(())
}

/// Event 6: pop the state of the screen `self+0x10`.
fn ovbeo_event6(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = 0x1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f190, 0x001f_a340)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
    Ok(())
}

/// `Cfg_FA238()`: copy the 0x288-byte options record (`s0`, always 8-aligned:
/// a constant address) to the stack (`v1`). The registers the 8-byte moves use
/// differ per call site (`regs` for the loop, `tail` for the last 8 bytes).
fn ovbeo_copy_config(
    c: &mut Cpu,
    regs: [usize; 4],
    tail: usize,
    pre_cycles: u64,
    set_s2: bool,
    tail_cycles: u64,
    head: u32,
) {
    c.cycles += pre_cycles;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x280));
    if set_s2 {
        c.g[S2][0] = 0x530000;
    }
    loop {
        c.cycles += 11;
        for (i, r) in regs.iter().enumerate() {
            c.g[*r][0] = c.ld((c.g[S0][0] as u32).wrapping_add(8 * i as u32));
        }
        for (i, r) in regs.iter().enumerate() {
            c.sd((c.g[V1][0] as u32).wrapping_add(8 * i as u32), c.g[*r][0]);
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
        let more = c.g[S0][0] != c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        break;
    }
    c.cycles += tail_cycles;
    c.ldl(tail, (c.g[S0][0] as u32).wrapping_add(0x7));
    c.ldr(tail, c.g[S0][0] as u32);
    c.sdl(tail, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(tail, c.g[V1][0] as u32);
}

/// One audio-setting widget: store `v1`'s field into the options record, get
/// the sound manager (0x28b180) and copy the record (`Cfg_FA238`).
/// Returns after the copy with `a0` = the sound manager.
fn ovbeo_store_and_copy(
    c: &mut Cpu,
    ret_28b180: u32,
    regs: [usize; 4],
    tail: usize,
    tail_cycles: u64,
    loop_head: u32,
) -> Flow {
    call_ptr(c, 0x0028_b180, ret_28b180)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];
    // The record is at 0x535610: 8-aligned, so the unaligned copy is dead.
    ovbeo_copy_config(c, regs, tail, 2, true, tail_cycles, loop_head);
    Ok(())
}

/// A volume widget (sfx, speech, music): merge the widget's nibble
/// (`widget+0x74 & 0xf`) into the options record at `shift`.
fn ovbeo_volume_prologue(c: &mut Cpu, cycles: u64, mask: u64, shift: u32, ret: u32) {
    c.cycles += cycles;
    c.g[A0][0] = 0x530000;
    c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5610));
    c.g[A1][0] = mask;
    c.g[V1][0] &= 0xf;
    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5610));
    if shift != 0 {
        c.g[V1][0] = sx((c.g[V1][0] as u32) << shift);
    }
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[V1][0];
    c.g[RA][0] = sx(ret);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x5610), c.g[V0][0]);
}

/// Event 9: one of the options widgets changed; returns after the final
/// vtable call.
fn ovbeo_event9(c: &mut Cpu) -> Flow {
    'done: {
        // The one-step tests: `Is_FA238(widget+0x38, D_...)`.
        'next: {
            c.cycles += 6;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2498));
            call_ptr(c, 0x0031_7670, 0x001f_a358)?;
            let other = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = 0x4a0000;
            if other {
                break 'next;
            }
            // Radio entry: config bits 30..31 = widget+0x319.
            c.cycles += 11;
            c.g[A1][0] = 0x530000;
            c.g[A0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x319));
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = 0x3fff_ffff;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 30);
            c.g[V1][0] &= c.g[V0][0];
            c.g[S2][0] = 0x530000;
            c.g[V1][0] |= c.g[A0][0];
            c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[V1][0]);
            // 1fa924
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'done;
        }
        // D_004A24A0 / D_004A24A8: a race value, through the world's race.
        'race: {
            let mut matched = false;
            'first: {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24a0));
                call_ptr(c, 0x0031_7670, 0x001f_a398)?;
                let other = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = 0x4a0000;
                if other {
                    break 'first;
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[S2][0] = 0x530000;
                call_ptr(c, 0x0039_a738, 0x001f_a3ac)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                matched = true;
            }
            if !matched {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24a8));
                call_ptr(c, 0x0031_7670, 0x001f_a3d0)?;
                let other = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = 0x4a0000;
                if other {
                    break 'race;
                }
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                c.g[S2][0] = 0x530000;
                call_ptr(c, 0x0039_a738, 0x001f_a3e4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa8));
            call_ptr(c, 0x0016_1ef0, 0x001f_a400)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'done;
        }
        // D_004A24B0: sfx volume.
        'sfx: {
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24b0));
            call_ptr(c, 0x0031_7670, 0x001f_a414)?;
            let other = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = 0x4a0000;
            if other {
                break 'sfx;
            }
            ovbeo_volume_prologue(c, 14, 0xffff_ffff_ffff_fff0, 0, 0x001f_a444);
            ovbeo_store_and_copy(c, 0x001f_a444, [A1, A2, A3, T0], V0, 9, 0x001f_a4b8)?;
            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] &= 0xf;
            call_ptr(c, 0x0028_7410, 0x001f_a500)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'done;
        }
        // D_004A24B8: speech volume.
        'speech: {
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24b8));
            call_ptr(c, 0x0031_7670, 0x001f_a514)?;
            let other = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = 0x4a0000;
            if other {
                break 'speech;
            }
            ovbeo_volume_prologue(c, 15, 0xffff_ffff_ffff_f0ff, 8, 0x001f_a548);
            ovbeo_store_and_copy(c, 0x001f_a548, [T0, A1, A2, A3], T0, 10, 0x001f_a5bc)?;
            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
            c.g[A1][0] &= 0xf;
            call_ptr(c, 0x0028_7488, 0x001f_a608)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'done;
        }
        // D_004A24C0: music volume.
        'music: {
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24c0));
            call_ptr(c, 0x0031_7670, 0x001f_a61c)?;
            let other = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = 0x4a0000;
            if other {
                break 'music;
            }
            ovbeo_volume_prologue(c, 15, 0xffff_ffff_ffff_ff0f, 4, 0x001f_a650);
            ovbeo_store_and_copy(c, 0x001f_a650, [A3, T0, A1, A2], A3, 9, 0x001f_a6c4)?;
            c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
            c.g[A1][0] >>= 4;
            call_ptr(c, 0x0028_7520, 0x001f_a70c)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'done;
        }
        // D_004A24C8: switch at config bit 17.
        'sw17: {
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24c8));
            call_ptr(c, 0x0031_7670, 0x001f_a720)?;
            let other = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = 0x4a0000;
            if other {
                break 'sw17;
            }
            c.cycles += 17;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0039_a738, 0x001f_a730)?;
            ovbeo_switch_merge(c, 0xffff_ffff_fffd_0000, 17, 0x001f_a75c);
            ovbeo_store_and_copy(c, 0x001f_a75c, [A2, A3, T0, A1], A2, 10, 0x001f_a7d0)?;
            c.g[A1][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[A1][0] &= 0x1;
            call_ptr(c, 0x0028_7558, 0x001f_a81c)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'done;
        }
        // D_004A24D0: switch at config bit 16, the last test.
        c.cycles += 6;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24d0));
        c.g[S2][0] = 0x530000;
        call_ptr(c, 0x0031_7670, 0x001f_a834)?;
        let other = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if other {
            break 'done;
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_a738, 0x001f_a844)?;
        ovbeo_switch_merge(c, 0xffff_ffff_fffe_0000, 16, 0x001f_a870);
        call_ptr(c, 0x0028_b180, 0x001f_a870)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = c.g[S0][0] & 0x7;
        c.g[V1][0] = c.g[SP][0];
        ovbeo_copy_config(c, [A1, A2, A3, T0], V0, 1, false, 7, 0x001f_a8dc);
        c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x2));
        c.g[A1][0] &= 0x1;
        call_ptr(c, 0x0028_75d0, 0x001f_a924)?;
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    }
    // 1fa928: the slider and the interface's update call.
    c.cycles += 6;
    c.g[A1][0] = 0x7;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
    call_ptr(c, 0x0014_dd58, 0x001f_a934)?;
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4b30));
    'slider: {
        if c.g[V1][0] == 0 {
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            break 'slider;
        }
        c.cycles += 6;
        c.g[A0][0] = 0x470000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x650));
        call_ptr(c, 0x0031_7670, 0x001f_a950)?;
        if c.g[S0][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            break 'slider;
        }
        c.cycles += 16;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x294));
        call_ptr(c, 0x0026_6f90, 0x001f_a968)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = 0xa;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
        c.mult(0, 2, 2, 3);
        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x294));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f_a988);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x290), c.g[V0][0]);
        call_ptr(c, 0x0026_6f58, 0x001f_a988)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x290));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
            call_ptr(c, 0x0026_6e30, 0x001f_a9b0)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
            call_ptr(c, 0x0026_6e88, 0x001f_a9a0)?;
        }
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
    }
    c.cycles += 4;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x001f_a9c4)
}

/// The two switch widgets: set config bit `bit` to `0 < v0` (the 0x39a738
/// result) and leave `ra`/store as the generated body does before 0x28b180.
fn ovbeo_switch_merge(c: &mut Cpu, mask: u64, bit: u32, ret: u32) {
    c.g[A1][0] = 0x530000;
    c.g[A0][0] = mask;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x5610));
    c.g[A0][0] |= 0xffff;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5610));
    c.g[V1][0] &= c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << bit);
    c.g[V1][0] |= c.g[V0][0];
    c.g[RA][0] = sx(ret);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x5610), c.g[V1][0]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes and its magic value (the fakes
    /// answer only on machines that wrote it).
    const OC1020_CTRL: u32 = DATA + 0x20_0000;
    const OC1020_MAGIC: u32 = 0x4f43_3130;
    const OC1020_SELF: u32 = DATA + 0x21_0000;
    const OC1020_WIDGET: u32 = DATA + 0x22_0000;
    /// The world at `gp-0x848`, its race chain (`+0x84`, `+0x84`, `+4`/`+8`).
    const OC1020_WORLD: u32 = DATA + 0x23_0000;
    const OC1020_R1: u32 = DATA + 0x24_0000;
    const OC1020_R2: u32 = DATA + 0x25_0000;
    const OC1020_X1: u32 = DATA + 0x26_0000;
    const OC1020_X2: u32 = DATA + 0x27_0000;
    /// The interface the first fake `getInterface` returns, and its vtable.
    const OC1020_IFACE: u32 = DATA + 0x28_0000;
    const OC1020_VT: u32 = DATA + 0x29_0000;
    /// Results of the fake allocator (0x317d70) and screen constructor.
    const OC1020_ALLOC: u32 = DATA + 0x2a_0000;
    const OC1020_SCREEN: u32 = DATA + 0x2b_0000;
    /// Forced result of the 0x39a738 fake (a widget's value).
    const OC1020_VALUE: u32 = OC1020_CTRL + 0x10;

    /// The fake callee: logs the call like `fakes::fake_call`, then forces the
    /// result the function acts on (hash: the argument itself).
    fn oc1020_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let value = c.bus.read32(OC1020_VALUE);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0031_7670 => c.g[V0][0] = a0,
            0x0014_dd58 => c.g[V0][0] = OC1020_IFACE as u64,
            0x0031_7d70 => c.g[V0][0] = OC1020_ALLOC as u64,
            0x0021_1088 => c.g[V0][0] = OC1020_SCREEN as u64,
            0x0039_a738 => c.g[V0][0] = sx(value),
            _ => {}
        }
    }

    macro_rules! oc1020_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1020_CTRL) != OC1020_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1020_fake_call(c, $addr);
                }
            )*
            fn oc1020_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1020_fakes!(
        (
            oc1020_fake_14dc80,
            OC1020_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc1020_fake_14dd58,
            OC1020_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc1020_fake_317670,
            OC1020_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc1020_fake_317d70,
            OC1020_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc1020_fake_211088,
            OC1020_PREV_211088,
            ee::s05::OVERRIDE_00211088,
            ee::s05::f_00211088__generated,
            0x00211088
        ),
        (
            oc1020_fake_39f320,
            OC1020_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc1020_fake_39f190,
            OC1020_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039f190
        ),
        (
            oc1020_fake_39a738,
            OC1020_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039a738
        ),
        (
            oc1020_fake_161ef0,
            OC1020_PREV_161EF0,
            ee::s01::OVERRIDE_00161EF0,
            ee::s01::f_00161ef0__generated,
            0x00161ef0
        ),
        (
            oc1020_fake_287410,
            OC1020_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x00287410
        ),
        (
            oc1020_fake_28b180,
            OC1020_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc1020_fake_287488,
            OC1020_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x00287488
        ),
        (
            oc1020_fake_287520,
            OC1020_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x00287520
        ),
        (
            oc1020_fake_287558,
            OC1020_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x00287558
        ),
        (
            oc1020_fake_2875d0,
            OC1020_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x002875d0
        ),
        (
            oc1020_fake_266f90,
            OC1020_PREV_266F90,
            ee::s07::OVERRIDE_00266F90,
            ee::s07::f_00266f90__generated,
            0x00266f90
        ),
        (
            oc1020_fake_266f58,
            OC1020_PREV_266F58,
            ee::s07::OVERRIDE_00266F58,
            ee::s07::f_00266f58__generated,
            0x00266f58
        ),
        (
            oc1020_fake_266e88,
            OC1020_PREV_266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x00266e88
        ),
        (
            oc1020_fake_266e30,
            OC1020_PREV_266E30,
            ee::s07::OVERRIDE_00266E30,
            ee::s07::f_00266e30__generated,
            0x00266e30
        ),
    );

    /// The widget-name addresses the function compares `widget+0x38` with.
    const OC1020_NAMES: [u32; 12] = [
        0x004a_2498,
        0x004a_24a0,
        0x004a_24a8,
        0x004a_24b0,
        0x004a_24b8,
        0x004a_24c0,
        0x004a_24c8,
        0x004a_24d0,
        0x0046_f9b0,
        0x0046_f9b0,
        0x1234,
        0x004a_24d0,
    ];

    /// The pause-options widget handler: a null widget, events 5 (open the
    /// sub-screen, match and mismatch), 6, other events, and event 9 for each
    /// widget (radio, two race values, three volumes, two switches, the
    /// slider with the flag on/off and a value below/above 10) with the
    /// 39a738 value 0/1/-1, every callee unwinding at some index.
    #[test]
    fn ov_screen_be_options_on_widget_action_matches() {
        fakes::covered(
            ee::s04::f_001fa238__generated,
            ov_screen_be_options_on_widget_action,
            400,
            &|c, seed| {
                oc1020_install_fakes();
                let mut r = fakes::machine(c, seed, 0x1fa238, 24);
                c.bus.write32(OC1020_CTRL, OC1020_MAGIC);
                let kind = seed % 20;
                let (event, hash): (u64, u32) = match kind {
                    0 | 1 => (5, 0x004a_24e8),
                    2 => (5, 0x004a_24e8),
                    3 => (5, 0x77),
                    4 => (6, 0),
                    5 => (0, 0),
                    6 => (7, 0),
                    7 => (4, 0),
                    k => (9, OC1020_NAMES[(k - 8) as usize]),
                };
                c.g[A0][0] = sx(OC1020_SELF);
                c.g[A1][0] = if kind == 0 && seed % 40 == 0 {
                    0
                } else {
                    sx(OC1020_WIDGET)
                };
                c.g[A2][0] = event;
                c.bus.write32(OC1020_SELF + 0x10, OC1020_R2);
                c.bus.write8(OC1020_SELF + 0x14, r.next_u32() as u8);
                c.bus.write32(OC1020_WIDGET + 0x38, hash);
                c.bus.write8(OC1020_WIDGET + 0x319, r.next_u32() as u8);
                c.bus.write32(
                    OC1020_WIDGET + 0x74,
                    [0u32, 1, 5, 0xffff_ffff][(seed / 20 % 4) as usize],
                );
                c.bus.write32(
                    OC1020_VALUE,
                    [0u32, 1, 0xffff_ffff][(seed / 7 % 3) as usize],
                );
                // world, race chain, interface and its vtable
                let gp = c.g[GP][0] as u32;
                c.bus.write32(gp.wrapping_sub(0x848), OC1020_WORLD);
                c.bus.write32(OC1020_WORLD + 0x84, OC1020_R1);
                c.bus.write32(OC1020_WORLD + 0x78, r.next_u32());
                c.bus.write32(OC1020_R1 + 0x48, OC1020_ALLOC);
                c.bus.write32(OC1020_R1 + 0x84, OC1020_R2);
                c.bus.write32(OC1020_R2 + 4, OC1020_X1);
                c.bus.write32(OC1020_R2 + 8, OC1020_X2);
                c.bus.write32(OC1020_X1 + 0xa8, r.next_u32());
                c.bus.write32(OC1020_X2 + 0xa8, r.next_u32());
                c.bus.write32(OC1020_SCREEN + 0x1c, r.next_u32());
                c.bus.write32(OC1020_IFACE + 0xc, OC1020_VT);
                c.bus.write16(OC1020_VT + 8, (seed % 3) as u16 * 0x10);
                let target = if seed % 5 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE
                };
                c.bus.write32(OC1020_VT + 0xc, target);
                // the options record and the slider flag
                for i in 0..0xa4 {
                    c.bus.write32(OVBEO_CONFIG + 4 * i, r.next_u32());
                }
                c.bus.write32(
                    0x0053_4b30,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(c.g[GP][0] as u32 + 0x304, r.next_u32());
                c.pc = 0x001f_a238;
            },
        );
    }
}
