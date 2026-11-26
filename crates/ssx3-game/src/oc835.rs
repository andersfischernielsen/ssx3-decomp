//! Staging module oc835 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc835`.

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
    0x0018_a8d0,
    cfe_state_options_sound_on_widget_action,
    "cFEStateOptionsSound_OnWidgetAction",
)];

/// The back-end option block (`cBEOptionInterface` mOption).
#[cfg(test)]
pub(crate) const SOUND_OPTION_BLOCK: u32 = 0x0053_5610;
/// Word `gp - 0x1680` (`D_004A1A70`): the "7lb" list box only counts when it is 1.
pub(crate) const SOUND_FLAG_GP: u32 = 0x1680;
/// Word `gp - 0x1c1c` (`D_004A14D4`): "options changed" flag.
pub(crate) const SOUND_CHANGED_GP: u32 = 0x1c1c;

#[inline(always)]
fn sound_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// The shared tail of `OnWidgetAction`: the three label levels of the
/// generated epilogue (`level` 0 = after `cycles += 1; lq s0`, 1 = after
/// `cycles += 1; lq s1`, 2 = the final restore of `s2`, `ra` and `sp`).
fn sound_widget_exit(c: &mut Cpu, frame: Frame, level: u8) {
    if level == 0 {
        c.cycles += 1;
        frame.restore(c, 16, 0x2c0);
    }
    if level <= 1 {
        c.cycles += 1;
        frame.restore(c, 17, 0x2b0);
    }
    c.cycles += 4;
    frame.restore(c, 18, 0x2a0);
    frame.restore_ra(c, 0x290);
    frame.close(c);
}

/// The compiled `Cfg()` copy of the option block (0x280 bytes in four-dword
/// steps from `src` to the stack copy at `v1`). `src` always holds
/// `0x535610` here (8-aligned), so only the dword loop of the compiler's
/// aligned/unaligned pair is reachable. `regs` are the scratch registers the
/// compiler picked for this copy, `head` the loop's address (event resume).
fn sound_cfg_copy(c: &mut Cpu, src: usize, regs: [usize; 4], head: u32) {
    c.cycles += 1;
    c.g[V0][0] = sx(sound_r(c, src).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        for (i, &reg) in regs.iter().enumerate() {
            let a = sound_r(c, src).wrapping_add(8 * i as u32);
            c.g[reg][0] = c.ld(a);
        }
        for (i, &reg) in regs.iter().enumerate() {
            let a = sound_r(c, V1).wrapping_add(8 * i as u32);
            let v = c.g[reg][0];
            c.sd(a, v);
        }
        c.g[src][0] = sx(sound_r(c, src).wrapping_add(0x20));
        let more = c.g[src][0] != c.g[V0][0];
        c.g[V1][0] = sx(sound_r(c, V1).wrapping_add(0x20));
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        break;
    }
}

/// The last eight bytes of the option block copy (`ldl/ldr` + `sdl/sdr` pair
/// through scratch register `reg`).
fn sound_cfg_tail(c: &mut Cpu, src: usize, reg: usize) {
    let s = sound_r(c, src);
    c.ldl(reg, s.wrapping_add(0x7));
    c.ldr(reg, s);
    let d = sound_r(c, V1);
    c.sdl(reg, d.wrapping_add(0x7));
    c.sdr(reg, d);
}

/// `Is(widget->mHashName, str)`: hash the string at `0x4a0000 + str_off`
/// (`0x317670`) and compare with the widget's name hash (`s0` after the call
/// is the widget hash). `cycles` is the block charge before the call.
fn sound_hash_is(
    c: &mut Cpu,
    cycles: u64,
    str_off: u32,
    ret_addr: u32,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += cycles;
    c.g[A0][0] = 0x4a_0000;
    let widget = sound_r(c, S1);
    c.g[S0][0] = c.lw(widget.wrapping_add(0x38));
    c.g[A0][0] = sx(sound_r(c, A0).wrapping_add(str_off));
    call_ptr(c, 0x0031_7670, ret_addr)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// `v0 = D_004A1A70` (the `lw v0, -0x1680(gp)` the chain leaves behind).
fn sound_load_flag(c: &mut Cpu) {
    c.g[V0][0] = c.lw(sound_r(c, GP).wrapping_sub(SOUND_FLAG_GP));
}

/// Store a volume slider's low nibble (shifted into place) into the option
/// block, then `bxAudio()` and snapshot the block: the common start of the
/// three slider cases. Leaves `a0 = audio`, `v1 = sp`, `s0 = block`, `v0 = 0`.
fn sound_slider_store(c: &mut Cpu, cycles: u64, mask: u64, shift: u32, ret_addr: u32) -> Flow {
    c.cycles += cycles;
    c.g[A0][0] = 0x53_0000;
    let widget = sound_r(c, S1);
    c.g[V1][0] = c.lbu(widget.wrapping_add(0x74));
    let block = sound_r(c, A0).wrapping_add(0x5610);
    c.g[V0][0] = c.lw(block);
    c.g[A1][0] = mask;
    c.g[V1][0] &= 0xf;
    c.g[S0][0] = sx(block);
    if shift != 0 {
        c.g[V1][0] = sx(sound_r(c, V1) << shift);
    }
    c.g[V0][0] &= c.g[A1][0];
    c.g[V0][0] |= c.g[V1][0];
    c.sw(block, c.g[V0][0]);
    call_ptr(c, 0x0028_b180, ret_addr)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];
    Ok(())
}

/// Store a list box's boolean into one bit of the option block (`sw` of
/// `(block & !bit) | (getCurrentEntryData(widget) > 0) << pos`) and
/// start the block snapshot: the two DJ-speech / arcade-SFX cases.
fn sound_flag_store(c: &mut Cpu, mask_top: u64, pos: u32, entry_ret: u32, ret_addr: u32) -> Flow {
    c.cycles += 17;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0039_a738, entry_ret)?;
    c.g[A1][0] = 0x53_0000;
    c.g[A0][0] = mask_top;
    let block = sound_r(c, A1).wrapping_add(0x5610);
    c.g[V1][0] = c.lw(block);
    c.g[A0][0] |= 0xffff;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.g[S0][0] = sx(block);
    c.g[V1][0] &= c.g[A0][0];
    c.g[V0][0] = sx(sound_r(c, V0) << pos);
    c.g[V1][0] |= c.g[V0][0];
    c.sw(block, c.g[V1][0]);
    call_ptr(c, 0x0028_b180, ret_addr)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];
    Ok(())
}

/// The `case 9` (value changed) widget-name chain: which control moved, store
/// it in the option block and push it to the audio engine. Ends with
/// `v0 = D_004A1A70` (the generated `b_18b30c` join).
fn sound_value_dispatch(c: &mut Cpu) -> Flow {
    // 1lb: speaker mode list box.
    let hit = sound_hash_is(c, 6, 0x1568, 0x0018_adc4)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039_a738, 0x0018_add4)?;
        c.g[A2][0] = 0x53_0000;
        c.g[A1][0] = 0xffff_ffff_ffff_cfff;
        let block = sound_r(c, A2).wrapping_add(0x5610);
        c.g[V1][0] = c.lw(block);
        c.g[V0][0] &= 0x3;
        c.g[V0][0] = sx(sound_r(c, V0) << 12);
        c.g[A0][0] = c.g[S1][0];
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= c.g[V0][0];
        c.sw(block, c.g[V1][0]);
        call_ptr(c, 0x0039_a738, 0x0018_adfc)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[V0][0];
        Callee {
            addr: 0x0018_bef8,
            f: ee::s02::f_0018bef8,
        }
        .call(c, 0x0018_ae08)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 2sl: sfx volume slider.
    let hit = sound_hash_is(c, 5, 0x1590, 0x0018_ae1c)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_slider_store(c, 14, 0xffff_ffff_ffff_fff0, 0, 0x0018_ae4c)?;
        sound_cfg_copy(c, S0, [T0, T1, A1, A2], 0x0018_aeb8);
        c.cycles += 9;
        sound_cfg_tail(c, S0, A3);
        c.g[A1][0] = c.lw(sound_r(c, SP));
        c.g[A1][0] &= 0xf;
        call_ptr(c, 0x0028_7410, 0x0018_af00)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 3sl: speech volume slider.
    let hit = sound_hash_is(c, 5, 0x1598, 0x0018_af14)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_slider_store(c, 15, 0xffff_ffff_ffff_f0ff, 8, 0x0018_af48)?;
        sound_cfg_copy(c, S0, [T1, A1, A2, A3], 0x0018_afb4);
        c.cycles += 10;
        sound_cfg_tail(c, S0, T0);
        c.g[A1][0] = c.lw(sound_r(c, SP));
        c.g[A1][0] = sx(sound_r(c, A1) >> 8);
        c.g[A1][0] &= 0xf;
        call_ptr(c, 0x0028_7488, 0x0018_b000)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 4sl: music volume slider.
    let hit = sound_hash_is(c, 5, 0x15a0, 0x0018_b014)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_slider_store(c, 15, 0xffff_ffff_ffff_ff0f, 4, 0x0018_b048)?;
        sound_cfg_copy(c, S0, [A1, A2, A3, T0], 0x0018_b0b4);
        c.cycles += 9;
        sound_cfg_tail(c, S0, T1);
        c.g[A1][0] = c.lbu(sound_r(c, SP));
        c.g[A1][0] >>= 4;
        call_ptr(c, 0x0028_7520, 0x0018_b0fc)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 5lb: DJ speech on/off.
    let hit = sound_hash_is(c, 5, 0x15a8, 0x0018_b110)?;
    c.g[A0][0] = 0x4a_0000;
    if hit {
        sound_flag_store(c, 0xffff_ffff_fffd_0000, 17, 0x0018_b120, 0x0018_b14c)?;
        sound_cfg_copy(c, S0, [A2, A3, T0, T1], 0x0018_b1b8);
        c.cycles += 10;
        sound_cfg_tail(c, S0, V0);
        c.g[A1][0] = c.lw(sound_r(c, SP));
        c.g[A1][0] = sx(((sound_r(c, A1)) as i32 >> 17) as u32);
        c.g[A1][0] &= 0x1;
        call_ptr(c, 0x0028_7558, 0x0018_b204)?;
        sound_load_flag(c);
        return Ok(());
    }

    // 6lb: arcade SFX on/off (the compare's delay slot loads the flag).
    let hit = sound_hash_is(c, 5, 0x15b0, 0x0018_b218)?;
    sound_load_flag(c);
    if hit {
        sound_flag_store(c, 0xffff_ffff_fffe_0000, 16, 0x0018_b228, 0x0018_b254)?;
        sound_cfg_copy(c, S0, [A3, T0, T1, A1], 0x0018_b2c0);
        c.cycles += 8;
        sound_cfg_tail(c, S0, A2);
        c.g[A1][0] = c.lh(sound_r(c, SP).wrapping_add(2));
        c.g[A1][0] &= 0x1;
        call_ptr(c, 0x0028_75d0, 0x0018_b308)?;
        sound_load_flag(c);
    }
    Ok(())
}

/// `case 9` of the widget action: a control's value changed. After the name
/// chain, "7lb" (only while `D_004A1A70 == 1`) stores its boolean into bit 18
/// of the option block, and the options-changed flag is set.
fn sound_value_changed(c: &mut Cpu, frame: Frame) -> Flow {
    sound_value_dispatch(c)?;
    c.cycles += 3;
    c.g[V0][0] ^= 0x1;
    let skip = c.g[V0][0] != 0;
    c.g[V0][0] = 0x1;
    if !skip {
        let hit = sound_hash_is(c, 6, 0x15b8, 0x0018_b328)?;
        c.g[V0][0] = 0x1;
        if hit {
            c.cycles += 12;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0039_a738, 0x0018_b338)?;
            c.g[A1][0] = 0x53_0000;
            c.g[A0][0] = 0xffff_ffff_fffb_0000;
            let block = sound_r(c, A1).wrapping_add(0x5610);
            c.g[V1][0] = c.lw(block);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[A0][0] |= 0xffff;
            c.g[V0][0] = sx(sound_r(c, V0) << 18);
            c.g[V1][0] &= c.g[A0][0];
            c.g[V1][0] |= c.g[V0][0];
            c.sw(block, c.g[V1][0]);
            c.g[V0][0] = 0x1;
        }
    }
    c.cycles += 1;
    let changed = sound_r(c, GP).wrapping_sub(SOUND_CHANGED_GP);
    c.sw(changed, c.g[V0][0]);
    sound_widget_exit(c, frame, 0);
    Ok(())
}

/// `case 6` of the widget action (the "accept" button): `iface->submit()`, push
/// each option-block field (volumes, DJ speech, arcade SFX) to the audio
/// engine through five fresh `Cfg()` snapshots, then ask the engine's state
/// factory for the next state (vtable slot at `+0x2c`) and push it.
fn sound_accept(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    // iface->submit(): `a1` is the interface from 0x14dd58.
    c.g[V0][0] = c.lw(sound_r(c, A1).wrapping_add(0xc));
    c.g[A0][0] = c.lh(sound_r(c, V0).wrapping_add(0x8));
    c.g[V1][0] = c.lw(sound_r(c, V0).wrapping_add(0xc));
    let target = sound_r(c, V1);
    c.g[A0][0] = sx(sound_r(c, A1).wrapping_add(sound_r(c, A0)));
    call_ptr(c, target, 0x0018_a9a4)?;
    call_ptr(c, 0x0028_b180, 0x0018_a9ac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = 0x53_0000;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, V0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 1: sfx volume (low nibble of word 0).
    sound_cfg_copy(c, A1, [A2, A3, T0, T1], 0x0018_aa24);
    c.cycles += 14;
    sound_cfg_tail(c, A1, V0);
    c.g[A1][0] = c.lw(sound_r(c, SP));
    c.g[A1][0] &= 0xf;
    call_ptr(c, 0x0028_7410, 0x0018_aa6c)?;
    call_ptr(c, 0x0028_b180, 0x0018_aa74)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 2: speech volume (bits 8..12).
    sound_cfg_copy(c, A1, [T1, A2, A3, T0], 0x0018_aae4);
    c.cycles += 15;
    sound_cfg_tail(c, A1, T1);
    c.g[A1][0] = c.lw(sound_r(c, SP));
    c.g[A1][0] = sx(sound_r(c, A1) >> 8);
    c.g[A1][0] &= 0xf;
    call_ptr(c, 0x0028_7488, 0x0018_ab30)?;
    call_ptr(c, 0x0028_b180, 0x0018_ab38)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 3: music volume (bits 4..8 of byte 0).
    sound_cfg_copy(c, A1, [T0, T1, A2, A3], 0x0018_aba8);
    c.cycles += 14;
    sound_cfg_tail(c, A1, T0);
    c.g[A1][0] = c.lbu(sound_r(c, SP));
    c.g[A1][0] >>= 4;
    call_ptr(c, 0x0028_7520, 0x0018_abf0)?;
    call_ptr(c, 0x0028_b180, 0x0018_abf8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 4: DJ speech (bit 17).
    sound_cfg_copy(c, A1, [A3, T0, T1, A2], 0x0018_ac68);
    c.cycles += 15;
    sound_cfg_tail(c, A1, A3);
    c.g[A1][0] = c.lw(sound_r(c, SP));
    c.g[A1][0] = sx(((sound_r(c, A1)) as i32 >> 17) as u32);
    c.g[A1][0] &= 0x1;
    call_ptr(c, 0x0028_7558, 0x0018_acb4)?;
    call_ptr(c, 0x0028_b180, 0x0018_acbc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = sx(sound_r(c, S0).wrapping_add(0x5610));
    c.g[V0][0] = c.g[A1][0] & 0x7;
    c.g[V1][0] = c.g[SP][0];

    // Snapshot 5: arcade SFX (bit 16 = bit 0 of the high half-word).
    sound_cfg_copy(c, A1, [A2, A3, T0, T1], 0x0018_ad2c);
    c.cycles += 18;
    sound_cfg_tail(c, A1, V0);
    c.g[A1][0] = c.lh(sound_r(c, SP).wrapping_add(0x2));
    c.g[A1][0] &= 0x1;
    call_ptr(c, 0x0028_75d0, 0x0018_ad74)?;

    // next = engine->vtable[+0x2c](engine + delta, self, widget->mMiscData)
    c.g[V0][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.lw(sound_r(c, S1).wrapping_add(0x18));
    c.g[A3][0] = c.lw(sound_r(c, V0));
    c.g[V1][0] = c.lw(sound_r(c, A3).wrapping_add(0x4));
    c.g[A0][0] = c.lh(sound_r(c, V1).wrapping_add(0x28));
    c.g[V0][0] = c.lw(sound_r(c, V1).wrapping_add(0x2c));
    let factory = sound_r(c, V0);
    c.g[A0][0] = sx(sound_r(c, A3).wrapping_add(sound_r(c, A0)));
    call_ptr(c, factory, 0x0018_ad98)?;
    let no_state = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[V0][0];
    if no_state {
        sound_widget_exit(c, frame, 0);
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
    c.g[A0][0] = sx(sound_r(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x0018_adac)?;
    frame.restore(c, 16, 0x2c0);
    sound_widget_exit(c, frame, 1);
    Ok(())
}

/// `cFEStateOptionsSound::OnWidgetAction(self = a0, widget = a1, event = a2)`
/// (0x18a8d0): the sound options screen's widget handler. A null widget does
/// nothing; event 6 (accept) applies every option and pushes the next state;
/// event 7 pushes the "reset options" popup; event 9 (value changed) stores
/// the moved control (speaker mode, three volumes, DJ speech, arcade SFX,
/// one more list box) into the option block and updates the audio engine and
/// the changed flag; other events do nothing.
///
/// The compiler's unaligned alternative of each option-block snapshot copy is
/// unreachable (the block address is the constant `0x535610`), so only the
/// aligned loops are ported.
pub fn cfe_state_options_sound_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x2d0);
        frame.save(c, 16, 0x2c0);
        frame.save(c, 17, 0x2b0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, 18, 0x2a0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x290);
        let no_widget = c.g[S1][0] == 0;
        c.g[S2][0] = c.g[A0][0];
        if no_widget {
            sound_widget_exit(c, frame, 0);
            return Ok(());
        }
        c.cycles += 9;
        call_ptr(c, 0x0014_dc80, 0x0018_a8fc)?;
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_a908)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0x6;
        let accept = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x7u64) as u64;
        if accept {
            return sound_accept(c, frame);
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            frame.restore(c, 16, 0x2c0);
            sound_widget_exit(c, frame, 1);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 0x7;
        let popup = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x9;
        if !popup {
            c.cycles += 2;
            let changed = c.g[S0][0] == c.g[V0][0];
            frame.restore(c, 16, 0x2c0);
            if changed {
                return sound_value_changed(c, frame);
            }
            c.cycles += 2;
            frame.restore(c, 17, 0x2b0);
            sound_widget_exit(c, frame, 2);
            return Ok(());
        }
        // Event 7: new cFEPopupOptionReset(76 bytes), parent = self's engine.
        c.cycles += 21;
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = 0x4c;
        c.g[A1][0] = sx(sound_r(c, A1).wrapping_sub(0x2298));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0018_a954)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e318, 0x0018_a968)?;
        c.g[A0][0] = c.lw(sound_r(c, S2).wrapping_add(0x10));
        c.g[V0][0] = 0x47_0000;
        c.g[V0][0] = sx(sound_r(c, V0).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw(sound_r(c, S0).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = sx(sound_r(c, A0).wrapping_add(0x18));
        c.sb(sound_r(c, S0).wrapping_add(0x48), 0);
        call_ptr(c, 0x0039_f290, 0x0018_a988)?;
        frame.restore(c, 16, 0x2c0);
        sound_widget_exit(c, frame, 1);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, SP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const OC835_CTRL: u32 = DATA + 0x20_0000;
    const OC835_MAGIC: u32 = 0x4f43_3835;
    const OC835_SELF: u32 = DATA + 0x21_0000;
    const OC835_ENGINE: u32 = DATA + 0x22_0000;
    const OC835_ENG: u32 = DATA + 0x23_0000;
    const OC835_ENGVT: u32 = DATA + 0x24_0000;
    const OC835_WIDGET: u32 = DATA + 0x25_0000;
    const OC835_IFACE: u32 = DATA + 0x26_0000;
    const OC835_IFVT: u32 = DATA + 0x27_0000;
    const OC835_FORCE: u32 = DATA + 0x30_0000;

    fn oc835_mail(addr: u32) -> u32 {
        OC835_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3; 1 = zero).
    fn oc835_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc835_mail(addr), v.wrapping_add(3));
    }

    fn oc835_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc835_mail(addr), 1);
    }

    /// The fake callee: logs the call, then returns its mailbox value; the
    /// string hash (`0x317670`) echoes its argument (the string address).
    fn oc835_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc835_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if addr == 0x0031_7670 {
            c.g[V0][0] = a0;
        } else if mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc835_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC835_CTRL) != OC835_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc835_fake_call(c, $addr);
                }
            )*
            fn oc835_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc835_fakes!(
        (
            oc835_fake_14dc80,
            OC835_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc835_fake_14dd58,
            OC835_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc835_fake_317d70,
            OC835_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc835_fake_317670,
            OC835_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc835_fake_39e318,
            OC835_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc835_fake_39f400,
            OC835_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc835_fake_39a738,
            OC835_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc835_fake_397840,
            OC835_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc835_fake_3978f0,
            OC835_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
        (
            oc835_fake_28b180,
            OC835_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc835_fake_287410,
            OC835_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x0028_7410
        ),
        (
            oc835_fake_287488,
            OC835_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x0028_7488
        ),
        (
            oc835_fake_287520,
            OC835_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x0028_7520
        ),
        (
            oc835_fake_287558,
            OC835_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x0028_7558
        ),
        (
            oc835_fake_2875d0,
            OC835_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0
        ),
    );

    /// Name-hash string offsets of the nine `Is()` tests in order of the
    /// chain (1lb, 2sl, 3sl, 4sl, 5lb, 6lb, 7lb) plus "none".
    const OC835_STR_OFFS: [u32; 8] = [
        0x1568, 0x1590, 0x1598, 0x15a0, 0x15a8, 0x15b0, 0x15b8, 0x0001,
    ];

    /// `OnWidgetAction`: null widget, events 5/6/7/9/others; event 9 over
    /// every control (and none), the flag word at gp-0x1680 at 1 and not,
    /// the engine's state factory returning 0 (real leaf) or a state, the
    /// list box returning 0/1/2/3/5. The 0x39f290 push and its `UIList`
    /// callees run real on the fixture engine; the compiler's unaligned
    /// snapshot loops are unreachable.
    #[test]
    fn cfe_state_options_sound_on_widget_action_matches() {
        oc835_install_fakes();
        check(
            ee::s02::f_0018a8d0__generated,
            cfe_state_options_sound_on_widget_action,
            320,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0835_0001, 14);
                c.bus.write32(OC835_CTRL, OC835_MAGIC);
                for i in 1..=0x300u32 {
                    let w = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(SP - 4 * i, w);
                }
                // The option block the snapshots copy.
                fill(c, &mut r, SOUND_OPTION_BLOCK, 0xa2);
                let gp = c.g[GP][0] as u32;
                let flag = match (seed / 7) % 3 {
                    0 => 1,
                    1 => 0,
                    _ => 2,
                };
                c.bus.write32(gp.wrapping_sub(SOUND_FLAG_GP), flag);
                c.bus
                    .write32(gp.wrapping_sub(SOUND_CHANGED_GP), r.next_u32());
                // The back-end interface (0x14dd58) and its vtable slot.
                oc835_force(c, 0x0014_dd58, OC835_IFACE);
                c.bus.write32(OC835_IFACE + 0xc, OC835_IFVT);
                c.bus.write16(OC835_IFVT + 8, 0x10 * (seed % 3) as u16);
                c.bus.write32(OC835_IFVT + 0xc, FAKE + 0x20);
                // The engine, its vtable and the state factory.
                c.bus.write32(OC835_SELF + 0x10, OC835_ENGINE);
                c.bus.write32(OC835_ENGINE, OC835_ENG);
                c.bus.write32(OC835_ENG + 4, OC835_ENGVT);
                c.bus
                    .write16(OC835_ENGVT + 0x28, 0x10 * ((seed / 3) % 3) as u16);
                let factory = if seed % 2 == 0 {
                    0x0014_06c0
                } else {
                    FAKE + 0x40
                };
                c.bus.write32(OC835_ENGVT + 0x2c, factory);
                // The widget.
                fill(c, &mut r, OC835_WIDGET, 0x40);
                let k = seed % 16;
                let sub = ((seed / 16) % 8) as usize;
                let hash = 0x4a_0000 + OC835_STR_OFFS[sub];
                c.bus
                    .write32(OC835_WIDGET + 0x38, if sub == 7 { 0x1_2345 } else { hash });
                c.bus.write8(OC835_WIDGET + 0x74, r.next_u32() as u8);
                let value = [0u32, 1, 2, 3, 5, 0x7fff_ffff][((seed / 5) % 6) as usize];
                oc835_force(c, 0x0039_a738, value);
                if (seed / 3) % 2 == 0 {
                    oc835_force_zero(c, 0x0039_7840);
                }
                let events: [u32; 16] = [0, 6, 9, 9, 9, 7, 9, 9, 9, 5, 0, 8, 10, 0xffff_ffff, 6, 9];
                c.g[A0][0] = sx(OC835_SELF);
                c.g[A1][0] = if k == 0 { 0 } else { sx(OC835_WIDGET) };
                c.g[A2][0] = if k == 0 {
                    sx(r.next_u32() % 12)
                } else {
                    sx(events[k as usize])
                };
                c.pc = 0x0018_a8d0;
            },
        );
    }
}
