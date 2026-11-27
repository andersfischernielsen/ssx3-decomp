//! Staging module oc975 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc975`.

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
    0x001f_9188,
    ov_screen_be_options_init,
    "OVScreenBEOptions_Init",
)];

/// The static options block the screen copies (`cBEOptionInterface` options,
/// 0x288 bytes) every time it reads a field: `0x535610`, 8-byte aligned, so
/// the generated body always takes its doubleword copy loop (its `ldl`/`ldr`
/// loop for a misaligned source is dead code and is not ported).
#[allow(dead_code)]
pub(crate) const OVBO_OPTIONS_STATIC: u32 = 0x0053_5610;

/// The doubleword copy loop of one inlined 0x288-byte options copy: 20
/// iterations of 32 bytes through the four temporaries `tmp` (their last
/// values stay in the registers), `src`/`dst` advance, V0 = end of the
/// loop part of the source. `head` is the loop head the event check reports.
fn ovbo_copy_loop(c: &mut Cpu, src: usize, dst: usize, tmp: [usize; 4], head: u32) {
    c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        for (i, &t) in tmp.iter().enumerate() {
            let a = (c.g[src][0] as u32).wrapping_add(8 * i as u32);
            c.g[t][0] = c.ld(a);
        }
        for (i, &t) in tmp.iter().enumerate() {
            let a = (c.g[dst][0] as u32).wrapping_add(8 * i as u32);
            let v = c.g[t][0];
            c.sd(a, v);
        }
        c.g[src][0] = sx((c.g[src][0] as u32).wrapping_add(0x20));
        let again = c.g[src][0] != c.g[V0][0];
        c.g[dst][0] = sx((c.g[dst][0] as u32).wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The last 8 bytes of the options copy (`ldl`/`ldr` + `sdl`/`sdr` through
/// register `reg`), source and destination in the registers `src` and `dst`.
fn ovbo_copy_tail(c: &mut Cpu, reg: usize, src: usize, dst: usize) {
    let s = c.g[src][0] as u32;
    let d = c.g[dst][0] as u32;
    c.ldl(reg, s.wrapping_add(7));
    c.ldr(reg, s);
    c.sdl(reg, d.wrapping_add(7));
    c.sdr(reg, d);
}

/// `A0 = 0x470000 - sub` (a string address in the screen's string table).
fn ovbo_str_a0(c: &mut Cpu, sub: u32) {
    c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(sub));
}

/// Hash the string in A0, then look the widget up in the screen
/// (`UIScreen_FindObject(self->mpScreen, hash)`).
fn ovbo_find(c: &mut Cpu, ret_hash: u32, ret_find: u32) -> Flow {
    call_ptr(c, 0x0031_7670, ret_hash)?;
    let screen = (c.g[S4][0] as u32).wrapping_add(0x40);
    c.g[A0][0] = c.lw(screen);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)
}

/// `UIListBox_AddItemStrId(list = S0, hash(string at 0x470000 - sub), id)`.
fn ovbo_add_entry(c: &mut Cpu, sub: u32, ret_hash: u32, id: u32, ret_add: u32) -> Flow {
    ovbo_str_a0(c, sub);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = id as u64;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a568, ret_add)
}

/// `if UIListBox_FindItemByValue(list = S0, value in A1, &sel = A2) {
/// UIListBox_SetSelection(list, sel) }` after the caller loaded A1/A2; the
/// found byte is at `sel_off(sp)`.
fn ovbo_select(c: &mut Cpu, sel_off: u32, ret_find: u32, ret_set: u32) -> Flow {
    call_ptr(c, 0x0039_a768, ret_find)?;
    let found = c.g[V0][0] != 0;
    let a = sp(c).wrapping_add(sel_off);
    c.g[A1][0] = c.lbu(a);
    if found {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_a7a8, ret_set)?;
    }
    Ok(())
}

/// `cOVStatePauseOptions::onCreateScreen` (the pause menu's "options" page, SSX 3
/// `OVScreenBEOptions_Init(self)`, 0x1f9188): creates the "37beoptions"
/// screen, then fills its widgets from the saved options (a 0x288-byte
/// options block at 0x535610 that the compiled code copies to the stack for
/// every field it reads): HUD list box, the two camera list boxes (chase
/// camera algorithm per player), the music/SFX/speech volume sliders (and
/// the audio system's volumes), the DJ-speech and arcade-SFX list boxes, and
/// the voice-chat widgets (visible only in a network game).
///
/// Registers and cycles follow the generated body exactly.
pub fn ov_screen_be_options_init(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0x350-byte frame, s0-s7/fp/ra saved.
        c.cycles += 23;
        let f = Frame::open(c, 0x350);
        f.save(c, S4, 0x300);
        c.g[S4][0] = c.g[A0][0];
        f.save(c, S1, 0x330);
        f.save(c, S2, 0x320);
        ovbo_str_a0(c, 0x728);
        f.save(c, S3, 0x310);
        f.save(c, S5, 0x2f0);
        f.save(c, S6, 0x2e0);
        f.save(c, S7, 0x2d0);
        f.save(c, FP, 0x2c0);
        f.save_ra(c, 0x2b0);
        f.save(c, S0, 0x340);
        let engine = (c.g[S4][0] as u32).wrapping_add(0x10);
        c.g[S0][0] = c.lw(engine);

        // self->mpScreen = engine->addScreenByHashName(self, hash("37beoptions")).
        call_ptr(c, 0x0031_7670, 0x001f_91c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_91dc)?;
        let none = c.g[V0][0] == 0;
        let at = (c.g[S4][0] as u32).wrapping_add(0x40);
        c.sw(at, c.g[V0][0]);
        if !none {
            // screen->playFrame(0, 0)
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_91f4)?;
        }

        // cOVState::onCreateScreen(self); cBE_getBE()->getInterface(4).
        c.cycles += 21;
        c.g[A0][0] = c.g[S4][0];
        c.g[S1][0] = 0x004a_0000;
        call_ptr(c, 0x0020_a380, 0x001f_9200)?;
        c.g[S3][0] = 0x004a_0000;
        call_ptr(c, 0x0014_dc80, 0x001f_9208)?;
        c.g[S5][0] = 0x004a_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x001f_9218)?;
        c.g[S6][0] = 0x004a_0000;
        c.g[A0][0] = 0x004a_0000;
        c.g[S7][0] = 0x004a_0000;
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2498));
        c.g[FP][0] = 0x004a_0000;
        ovbo_find(c, 0x001f_9230, 0x001f_923c)?;

        // HUD list box ("0lb"): Full / Minimal / None, current = options.f30.
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 25;
            ovbo_add_entry(c, 0x718, 0x001f_9250, 0, 0x001f_9260)?;
            ovbo_add_entry(c, 0x708, 0x001f_926c, 1, 0x001f_927c)?;
            ovbo_add_entry(c, 0x6f0, 0x001f_9288, 2, 0x001f_9298)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = c.g[V1][0] & 7;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.cycles += 1;
            ovbo_copy_loop(c, V1, A0, [T1, A1, A2, A3], 0x001f_9308);
            c.cycles += 11;
            ovbo_copy_tail(c, T0, V1, A0);
            c.g[A2][0] = c.g[SP][0];
            let opt = sp(c).wrapping_add(0x10);
            c.g[A1][0] = c.lw(opt);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 30);
            ovbo_select(c, 0, 0x001f_9358, 0x001f_9368)?;
        }

        // Camera list box of player 1 ("1lb") and its "camera" label.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24a0));
        ovbo_find(c, 0x001f_9370, 0x001f_937c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 31;
            ovbo_add_entry(c, 0x6e0, 0x001f_9390, 0x3c, 0x001f_93a0)?;
            ovbo_add_entry(c, 0x6d0, 0x001f_93ac, 0x3d, 0x001f_93bc)?;
            ovbo_add_entry(c, 0x6c0, 0x001f_93c8, 0x3e, 0x001f_93d8)?;
            // camera = world->views->PlayerCamera[0]; algo = controller->+0x18
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[A1][0] = c.lw(world);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[SP][0];
            let a = (c.g[A1][0] as u32).wrapping_add(0x84);
            c.g[V0][0] = c.lw(a);
            let a = (c.g[V0][0] as u32).wrapping_add(0x84);
            c.g[V1][0] = c.lw(a);
            let a = (c.g[V1][0] as u32).wrapping_add(4);
            c.g[A1][0] = c.lw(a);
            let a = (c.g[A1][0] as u32).wrapping_add(0xa8);
            c.g[V0][0] = c.lw(a);
            let a = (c.g[V0][0] as u32).wrapping_add(0x18);
            c.g[A1][0] = c.lw(a);
            ovbo_select(c, 0, 0x001f_93fc, 0x001f_940c)?;

            c.cycles += 9;
            ovbo_str_a0(c, 0x6b0);
            ovbo_find(c, 0x001f_9418, 0x001f_9424)?;
            c.g[S2][0] = c.g[V0][0];
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V0][0] = c.lw(world);
            if c.g[S2][0] != 0 {
                ovbo_camera_label(c, 0x001f_944c, 0x001f_945c, 0x001f_9474, 0x001f_9480, 1)?;
            }
        }

        // Camera list box of player 2 ("2lb") and its label.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x24a8));
        ovbo_find(c, 0x001f_9488, 0x001f_9494)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 27;
            ovbo_add_entry(c, 0x6e0, 0x001f_94a8, 0x3c, 0x001f_94b8)?;
            ovbo_add_entry(c, 0x6d0, 0x001f_94c4, 0x3d, 0x001f_94d4)?;
            ovbo_add_entry(c, 0x6c0, 0x001f_94e0, 0x3e, 0x001f_94f0)?;
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V1][0] = c.lw(world);
            let a = (c.g[V1][0] as u32).wrapping_add(0x84);
            c.g[A0][0] = c.lw(a);
            let a = (c.g[A0][0] as u32).wrapping_add(0x84);
            c.g[V1][0] = c.lw(a);
            let a = (c.g[V1][0] as u32).wrapping_add(0x10);
            c.g[V0][0] = c.lw(a);
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            c.g[A0][0] = 0x0047_0000;
            if c.g[V0][0] == 0 {
                // two or more views: select by PlayerCamera[1]'s algorithm
                c.cycles += 8;
                let a = (c.g[V1][0] as u32).wrapping_add(8);
                c.g[V1][0] = c.lw(a);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[SP][0];
                let a = (c.g[V1][0] as u32).wrapping_add(0xa8);
                c.g[V0][0] = c.lw(a);
                let a = (c.g[V0][0] as u32).wrapping_add(0x18);
                c.g[A1][0] = c.lw(a);
                ovbo_select(c, 0, 0x001f_9524, 0x001f_9534)?;
                c.cycles += 1;
                c.g[A0][0] = 0x0047_0000;
            }
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x690));
            ovbo_find(c, 0x001f_9540, 0x001f_954c)?;
            c.g[S2][0] = c.g[V0][0];
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V0][0] = c.lw(world);
            if c.g[S2][0] != 0 {
                ovbo_camera_label(c, 0x001f_9574, 0x001f_9584, 0x001f_959c, 0x001f_95a8, 2)?;
            }
        }

        // Music volume slider ("4sl"): 12 steps, value options.f0; applies it.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x24b0));
        ovbo_find(c, 0x001f_95b0, 0x001f_95bc)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0053_0000;
        if c.g[A2][0] != 0 {
            ovbo_slider(
                c,
                &OvboSlider {
                    heads: [0x001f_9644, 0x001f_9700],
                    tmps: [[A3, T0, T1, V1], [A2, A3, T0, T1]],
                    tails: [A3, V0],
                    cycles: [13, 7],
                    field: OvboField::Low4,
                    ret_audio: 0x001f_9690,
                    ret_set: 0x001f_9748,
                    set: 0x0028_7410,
                },
            )?;
        }
        // SFX volume slider ("5sl"): value options.f8 (bits 8..11).
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x24b8));
        ovbo_find(c, 0x001f_9750, 0x001f_975c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0053_0000;
        if c.g[A2][0] != 0 {
            ovbo_slider(
                c,
                &OvboSlider {
                    heads: [0x001f_97e4, 0x001f_98a4],
                    tmps: [[T1, V1, A3, T0], [T0, T1, A2, A3]],
                    tails: [T1, T0],
                    cycles: [14, 8],
                    field: OvboField::Bits8,
                    ret_audio: 0x001f_9834,
                    ret_set: 0x001f_98f0,
                    set: 0x0028_7488,
                },
            )?;
        }
        // Speech volume slider ("6sl"): value options.f4 (bits 4..7).
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x24c0));
        ovbo_find(c, 0x001f_98f8, 0x001f_9904)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0053_0000;
        if c.g[A2][0] != 0 {
            ovbo_slider(
                c,
                &OvboSlider {
                    heads: [0x001f_998c, 0x001f_9a48],
                    tmps: [[A3, T0, T1, V1], [A2, A3, T0, T1]],
                    tails: [A3, V0],
                    cycles: [13, 7],
                    field: OvboField::Bits4,
                    ret_audio: 0x001f_99d8,
                    ret_set: 0x001f_9a90,
                    set: 0x0028_7520,
                },
            )?;
        }

        // DJ speech on/off list box ("7lb").
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x24c8));
        ovbo_find(c, 0x001f_9a98, 0x001f_9aa4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 20;
            ovbo_add_entry(c, 0x680, 0x001f_9ab8, 0, 0x001f_9ac8)?;
            ovbo_add_entry(c, 0x670, 0x001f_9ad4, 1, 0x001f_9ae4)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
            c.g[S2][0] = c.g[V1][0];
            c.g[V0][0] = c.g[A1][0] & 7;
            c.g[S1][0] = c.g[A0][0];
            c.cycles += 2;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x2a0));
            ovbo_copy_loop(c, A1, A0, [T1, V1, A3, T0], 0x001f_9b64);
            c.cycles += 11;
            ovbo_copy_tail(c, T1, A1, A0);
            let opt = sp(c).wrapping_add(0x10);
            c.g[A1][0] = c.lw(opt);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 17);
            c.g[A1][0] &= 1;
            ovbo_select(c, 0x2a0, 0x001f_9bb4, 0x001f_9bc4)?;
            // Cfg_PO() copy again, then SetDJSpeech(audio, cfg.iUseDJSpeech != 0).
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x001f_9bcc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = c.g[V1][0] & 7;
            c.g[A1][0] = c.g[S1][0];
            c.cycles += 1;
            ovbo_copy_loop(c, V1, A1, [T0, T1, A2, A3], 0x001f_9c3c);
            c.cycles += 8;
            ovbo_copy_tail(c, T0, V1, A1);
            let opt = sp(c).wrapping_add(0x10);
            c.g[A1][0] = c.lw(opt);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[A1][0] &= 1;
            call_ptr(c, 0x0028_7558, 0x001f_9c88)?;
        }

        // Arcade SFX on/off list box ("8lb").
        c.cycles += 9;
        c.g[A3][0] = 0x004a_0000;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x24d0));
        ovbo_find(c, 0x001f_9c94, 0x001f_9ca0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 20;
            ovbo_add_entry(c, 0x680, 0x001f_9cb4, 0, 0x001f_9cc4)?;
            ovbo_add_entry(c, 0x670, 0x001f_9cd0, 1, 0x001f_9ce0)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
            c.g[S2][0] = c.g[V1][0];
            c.g[V0][0] = c.g[A1][0] & 7;
            c.g[S1][0] = c.g[A0][0];
            c.cycles += 2;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x2a1));
            ovbo_copy_loop(c, A1, A0, [T0, T1, V1, A3], 0x001f_9d60);
            c.cycles += 10;
            ovbo_copy_tail(c, T0, A1, A0);
            let opt = sp(c).wrapping_add(0x12);
            c.g[A1][0] = c.lhu(opt);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] &= 1;
            ovbo_select(c, 0x2a1, 0x001f_9dac, 0x001f_9dbc)?;
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x001f_9dc4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = c.g[V1][0] & 7;
            c.g[A1][0] = c.g[S1][0];
            c.cycles += 1;
            ovbo_copy_loop(c, V1, A1, [A2, A3, T0, T1], 0x001f_9e34);
            c.cycles += 7;
            ovbo_copy_tail(c, V0, V1, A1);
            let opt = sp(c).wrapping_add(0x12);
            c.g[A1][0] = c.lh(opt);
            c.g[A1][0] &= 1;
            call_ptr(c, 0x0028_75d0, 0x001f_9e7c)?;
        }

        // "EATalkPair": hidden and static (shown again for network games below).
        c.cycles += 10;
        c.g[T1][0] = 0x0047_0000;
        c.g[S1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x660));
        c.g[A0][0] = c.g[S1][0];
        ovbo_find(c, 0x001f_9e8c, 0x001f_9e98)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        if c.g[S0][0] != 0 {
            c.cycles += 11;
            ovbo_vcall(c, S0, V0, V1, 0x48, 0x4c, 0x001f_9eb8)?;
            c.g[A1][0] = 1;
            ovbo_vcall(c, S0, V0, V1, 0x40, 0x44, 0x001f_9ed0)?;
        }

        // Network game: voice-chat widgets; otherwise grey out the label.
        // `end0` paths charge one more cycle before the epilogue.
        'end: {
            'end0: {
                c.cycles += 8;
                let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
                c.g[V0][0] = c.lw(world);
                c.g[A1][0] = 7;
                let a = (c.g[V0][0] as u32).wrapping_add(0x78);
                c.g[A0][0] = c.lw(a);
                call_ptr(c, 0x0014_dd58, 0x001f_9ee0)?;
                c.g[V0][0] = 0x0053_0000;
                let a = (c.g[V0][0] as u32).wrapping_add(0x4b30);
                c.g[V1][0] = c.lw(a);
                c.g[V0][0] = 0x004a_0000;
                if c.g[V1][0] == 0 {
                    // Not a network game: the talk label is static and hidden.
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24d8));
                    ovbo_find(c, 0x001f_a04c, 0x001f_a058)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    if c.g[S0][0] == 0 {
                        break 'end0;
                    }
                    c.cycles += 27;
                    ovbo_vcall(c, S0, V0, V1, 0xc8, 0xcc, 0x001f_a078)?;
                    // staticColour = *(0x4c66c8) (16 bytes at +0x88).
                    c.g[A0][0] = 0x004c_0000;
                    c.g[A1][0] = 1;
                    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66c8));
                    let t0 = c.g[T0][0] as u32;
                    let s0 = c.g[S0][0] as u32;
                    c.ldl(V1, t0.wrapping_add(7));
                    c.ldr(V1, t0);
                    c.ldl(A2, t0.wrapping_add(0xf));
                    c.ldr(A2, t0.wrapping_add(8));
                    c.sdl(V1, s0.wrapping_add(0x8f));
                    c.sdr(V1, s0.wrapping_add(0x88));
                    c.sdl(A2, s0.wrapping_add(0x97));
                    c.sdr(A2, s0.wrapping_add(0x90));
                    ovbo_vcall(c, S0, V0, V1, 0x40, 0x44, 0x001f_a0b8)?;
                    c.g[A1][0] = 0;
                    ovbo_vcall(c, S0, V0, V1, 0x48, 0x4c, 0x001f_a0d0)?;
                    break 'end0;
                }
                // Network game.
                c.cycles += 21;
                c.g[A0][0] = c.g[S1][0];
                ovbo_find(c, 0x001f_9ef8, 0x001f_9f04)?;
                c.g[V1][0] = 0x004a_0000;
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x24d8));
                ovbo_find(c, 0x001f_9f14, 0x001f_9f20)?;
                c.g[A0][0] = 0x0047_0000;
                c.g[S3][0] = c.g[V0][0];
                ovbo_str_a0(c, 0x650);
                ovbo_find(c, 0x001f_9f30, 0x001f_9f3c)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S2][0] == 0 {
                    break 'end0;
                }
                c.cycles += 2;
                f.restore(c, S0, 0x340);
                if c.g[S3][0] == 0 {
                    break 'end;
                }
                c.cycles += 2;
                c.g[A1][0] = 1;
                if c.g[S1][0] == 0 {
                    break 'end;
                }
                c.cycles += 45;
                c.g[S0][0] = 0xa;
                // talkPair->setVisible(1) / setStatic(0), then talk item and slider.
                ovbo_vcall(c, S2, V1, V0, 0x48, 0x4c, 0x001f_9f6c)?;
                c.g[A1][0] = 0;
                ovbo_vcall(c, S2, V1, V0, 0x40, 0x44, 0x001f_9f84)?;
                c.g[A1][0] = 1;
                ovbo_vcall(c, S3, V1, V0, 0x48, 0x4c, 0x001f_9f9c)?;
                c.g[A1][0] = 0;
                ovbo_vcall(c, S3, V1, V0, 0x40, 0x44, 0x001f_9fb4)?;
                c.g[A1][0] = 1;
                ovbo_vcall(c, S1, V1, V0, 0x48, 0x4c, 0x001f_9fcc)?;
                c.g[A1][0] = 0;
                ovbo_vcall(c, S1, V1, V0, 0x40, 0x44, 0x001f_9fe4)?;
                // slider: 10 values, value = level / 10; VoipMan level query.
                let a = (c.g[S1][0] as u32).wrapping_add(0x78);
                c.sw(a, c.g[S0][0]);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x2a4));
                let voip = (c.g[GP][0] as u32).wrapping_add(0x304);
                c.g[A0][0] = c.lw(voip);
                c.g[A2][0] = sx(sp(c).wrapping_add(0x2a8));
                call_ptr(c, 0x0026_6f90, 0x001f_9ff8)?;
                let a = sp(c).wrapping_add(0x2a4);
                c.g[V1][0] = c.lw(a);
                c.g[V0][0] = 0xa;
                c.cycles += 7;
                c.div(0, V1, S0);
                c.g[V1][0] = c.lo[0];
                let a = (c.g[S1][0] as u32).wrapping_add(0x74);
                c.sw(a, c.g[V1][0]);
                let a = sp(c).wrapping_add(0x2a4);
                c.g[V0][0] = c.lw(a);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
                c.cycles += 4;
                let voip = (c.g[GP][0] as u32).wrapping_add(0x304);
                c.g[A0][0] = c.lw(voip);
                if c.g[V0][0] == 0 {
                    // level >= 10: VoipMan::Activate
                    call_ptr(c, 0x0026_6e30, 0x001f_a03c)?;
                } else {
                    // level < 10: VoipMan::Deactivate
                    call_ptr(c, 0x0026_6e88, 0x001f_a02c)?;
                }
                f.restore(c, S0, 0x340);
                break 'end;
            }
            c.cycles += 1;
            f.restore(c, S0, 0x340);
        }

        // Epilogue.
        c.cycles += 11;
        f.restore(c, S1, 0x330);
        f.restore(c, S2, 0x320);
        f.restore(c, S3, 0x310);
        f.restore(c, S4, 0x300);
        f.restore(c, S5, 0x2f0);
        f.restore(c, S6, 0x2e0);
        f.restore(c, S7, 0x2d0);
        f.restore(c, FP, 0x2c0);
        f.restore_ra(c, 0x2b0);
        f.close(c);
        Ok(())
    });
}

/// `lw off(reg)` as the generated body reads it (sign-extended).
fn ovbo_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// Virtual call of a gcc 2.9x vtable slot: `vt = *(obj + 8)` into register
/// `vt`, `A0 = obj + (lh vt+delta_off)`, `fn = lw vt+fn_off` into register
/// `func`, then `jalr func` returning to `ret` (A1 = the argument, set by the
/// caller).
fn ovbo_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    func: usize,
    delta_off: u32,
    fn_off: u32,
    ret: u32,
) -> Flow {
    c.g[vt][0] = ovbo_lw(c, obj, 8);
    let a = (c.g[vt][0] as u32).wrapping_add(delta_off);
    c.g[A0][0] = c.lh(a);
    let a = (c.g[vt][0] as u32).wrapping_add(fn_off);
    c.g[func][0] = c.lw(a);
    let target = c.g[func][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// How a slider's value is cut out of the copied options word at `sp+0x10`.
#[derive(Clone, Copy)]
enum OvboField {
    /// `lw & 0xf`
    Low4,
    /// `(lw >> 8) & 0xf`
    Bits8,
    /// `lbu >> 4`
    Bits4,
}

/// Read the slider value from the stack copy of the options.
fn ovbo_field(c: &mut Cpu, field: OvboField) -> u64 {
    let a = sp(c).wrapping_add(0x10);
    match field {
        OvboField::Low4 => c.lw(a) & 0xf,
        OvboField::Bits8 => sx((c.lw(a) as u32) >> 8) & 0xf,
        OvboField::Bits4 => c.lbu(a) >> 4,
    }
}

/// One volume slider of the options screen (A2 = widget, V1 = 0x530000):
/// 12 values, current value from a copy of the options block (widget +0x74),
/// then the audio system is told the volume from a second copy
/// (`SSXAudioSystem_Set*Volume(GetInstance(), field)`).
struct OvboSlider {
    /// Loop heads and temporaries of the first and second copy.
    heads: [u32; 2],
    tmps: [[usize; 4]; 2],
    /// Tail registers of the two copies.
    tails: [usize; 2],
    /// Cycles of the blocks after the first and second copy.
    cycles: [u64; 2],
    field: OvboField,
    /// Return of `SSXAudioSystem_GetInstance` and of the volume call.
    ret_audio: u32,
    ret_set: u32,
    set: u32,
}

fn ovbo_slider(c: &mut Cpu, p: &OvboSlider) -> Flow {
    c.cycles += 8;
    c.g[V0][0] = 0xc;
    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
    let a = (c.g[A2][0] as u32).wrapping_add(0x78);
    c.sw(a, c.g[V0][0]);
    c.g[S2][0] = c.g[V1][0];
    c.g[V0][0] = c.g[A1][0] & 7;
    c.g[S1][0] = c.g[A0][0];
    c.cycles += 1;
    ovbo_copy_loop(c, A1, A0, p.tmps[0], p.heads[0]);
    c.cycles += p.cycles[0];
    ovbo_copy_tail(c, p.tails[0], A1, A0);
    c.g[V0][0] = ovbo_field(c, p.field);
    let a = (c.g[A2][0] as u32).wrapping_add(0x74);
    c.sw(a, c.g[V0][0]);
    call_ptr(c, 0x0028_b180, p.ret_audio)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5610));
    c.g[V0][0] = c.g[V1][0] & 7;
    c.g[A1][0] = c.g[S1][0];
    c.cycles += 1;
    ovbo_copy_loop(c, V1, A1, p.tmps[1], p.heads[1]);
    c.cycles += p.cycles[1];
    ovbo_copy_tail(c, p.tails[1], V1, A1);
    c.g[A1][0] = ovbo_field(c, p.field);
    call_ptr(c, p.set, p.ret_set)
}

/// The "camera N" label under a camera list box (`setFormatted("kT_CMNCameraVar",
/// N)` through the widget's vtable), with the world in V0. The two copies of
/// this code differ in where S0/A1 are loaded (`player` 1: S0 before the
/// test, A1 after).
fn ovbo_camera_label(
    c: &mut Cpu,
    ret_hash: u32,
    ret_call: u32,
    ret_fmt: u32,
    ret_set: u32,
    player: u32,
) -> Flow {
    c.cycles += 13;
    ovbo_str_a0(c, 0x6a0);
    c.g[S0][0] = ovbo_lw(c, V0, 0x8c);
    c.g[S1][0] = ovbo_lw(c, S0, 4);
    c.g[V0][0] = {
        let a = (c.g[S1][0] as u32).wrapping_add(0x20);
        c.lh(a)
    };
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[V1][0] = ovbo_lw(c, S1, 0x24);
    c.g[A0][0] = c.g[S0][0];
    let target = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target, ret_call)?;
    let none = c.g[V0][0] == 0;
    if player == 1 {
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    } else {
        c.g[A1][0] = c.g[V0][0];
    }
    if !none {
        c.cycles += 7;
        if player == 1 {
            c.g[A1][0] = c.g[V0][0];
        } else {
            c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = player as u64;
        call_ptr(c, 0x002c_26d0, ret_fmt)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_0e90, ret_set)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes, and its magic value.
    const OC975_CTRL: u32 = DATA + 0x20_0000;
    const OC975_MAGIC: u32 = 0x4f43_3937;
    /// The state object (`self`) and the engine it points to.
    const OC975_SELF: u32 = DATA + 0x21_0000;
    /// World (gp-0x848) and the camera/label object graph below it.
    const OC975_WORLD: u32 = DATA + 0x22_0000;
    const OC975_OBJ: u32 = DATA + 0x23_0000;

    /// Per-callee mailbox: bit n of the word makes the n-th call of that
    /// callee return 0 (lookups that find nothing, items not found, ...).
    fn oc975_mail(addr: u32) -> u32 {
        OC975_CTRL + 0x100 + (addr & 0xffc)
    }

    /// Per-callee call counter.
    fn oc975_count(addr: u32) -> u32 {
        OC975_CTRL + 0x1100 + (addr & 0xffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns 0
    /// if its mailbox asks for it on this call.
    fn oc975_fake_call(c: &mut Cpu, addr: u32) {
        let mail = c.bus.read32(oc975_mail(addr));
        let n = c.bus.read32(oc975_count(addr));
        c.bus.write32(oc975_count(addr), n + 1);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && (mail >> (n & 31)) & 1 == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc975_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC975_CTRL) != OC975_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc975_fake_call(c, $addr);
                }
            )*
            fn oc975_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc975_fakes!(
        (
            oc975_fake_317670,
            OC975_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc975_fake_397cc0,
            OC975_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc975_fake_39c870,
            OC975_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc975_fake_20a380,
            OC975_PREV_20A380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020a380
        ),
        (
            oc975_fake_14dc80,
            OC975_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc975_fake_14dd58,
            OC975_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc975_fake_39d860,
            OC975_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc975_fake_39a568,
            OC975_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039a568
        ),
        (
            oc975_fake_39a768,
            OC975_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039a768
        ),
        (
            oc975_fake_39a7a8,
            OC975_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039a7a8
        ),
        (
            oc975_fake_2c26d0,
            OC975_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc975_fake_3a0e90,
            OC975_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc975_fake_28b180,
            OC975_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc975_fake_287410,
            OC975_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x00287410
        ),
        (
            oc975_fake_287488,
            OC975_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x00287488
        ),
        (
            oc975_fake_287520,
            OC975_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x00287520
        ),
        (
            oc975_fake_287558,
            OC975_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x00287558
        ),
        (
            oc975_fake_2875d0,
            OC975_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x002875d0
        ),
        (
            oc975_fake_266f90,
            OC975_PREV_266F90,
            ee::s07::OVERRIDE_00266F90,
            ee::s07::f_00266f90__generated,
            0x00266f90
        ),
        (
            oc975_fake_266e88,
            OC975_PREV_266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x00266e88
        ),
        (
            oc975_fake_266e30,
            OC975_PREV_266E30,
            ee::s07::OVERRIDE_00266E30,
            ee::s07::f_00266e30__generated,
            0x00266e30
        ),
    );

    fn oc975_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// One seed's machine: fakes gated on, the options block, the world with
    /// its camera graph, the label object, the network flag and mailboxes.
    fn oc975_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        oc975_install_fakes();
        let mut r = machine(c, seed, 0x1f_9188, calls);
        oc975_w(c, OC975_CTRL, OC975_MAGIC);
        // Lookups that find nothing (all found / none found / random).
        let missing = match seed % 5 {
            0 => 0,
            1 => u32::MAX,
            _ => r.next_u32() & r.next_u32(),
        };
        oc975_w(c, oc975_mail(0x0039_d860), missing);
        // List box value lookups that find nothing.
        let not_found = match seed % 3 {
            0 => 0,
            1 => u32::MAX,
            _ => r.next_u32(),
        };
        oc975_w(c, oc975_mail(0x0039_a768), not_found);
        // The screen could not be created.
        oc975_w(c, oc975_mail(0x0039_7cc0), (seed % 7 == 0) as u32);
        // self, engine.
        oc975_w(c, OC975_SELF + 0x10, OC975_OBJ + 0x8000);
        // The world and the camera graph: world+0x84 -> views -> +0x84 ->
        // man; man+4/+8 = cameras (+0xa8 -> controller, +0x18 = algorithm),
        // man+0x10 = number of views.
        let (man, cam0, cam1, cc0, cc1) = (
            OC975_OBJ + 0x100,
            OC975_OBJ + 0x200,
            OC975_OBJ + 0x300,
            OC975_OBJ + 0x400,
            OC975_OBJ + 0x500,
        );
        oc975_w(c, GAME_GP.wrapping_sub(0x848), OC975_WORLD);
        oc975_w(c, OC975_WORLD + 0x78, OC975_OBJ);
        oc975_w(c, OC975_WORLD + 0x84, OC975_OBJ + 0x1000);
        oc975_w(c, OC975_OBJ + 0x1000 + 0x84, man);
        oc975_w(c, man + 4, cam0);
        oc975_w(c, man + 8, cam1);
        oc975_w(c, man + 0x10, seed % 4);
        oc975_w(c, cam0 + 0xa8, cc0);
        oc975_w(c, cam1 + 0xa8, cc1);
        oc975_w(c, cc0 + 0x18, 0x3c + r.next_u32() % 4);
        oc975_w(c, cc1 + 0x18, 0x3c + r.next_u32() % 4);
        // The camera label object: world+0x8c -> label; label+4 -> vtable
        // (delta at +0x20, function at +0x24: a real leaf returning 0, or a
        // fake that returns a pointer).
        let (label, vt) = (OC975_OBJ + 0x600, OC975_OBJ + 0x700);
        oc975_w(c, OC975_WORLD + 0x8c, label);
        oc975_w(c, label + 4, vt);
        c.bus.write16(vt + 0x20, (r.next_u32() % 8 * 16) as u16);
        oc975_w(
            c,
            vt + 0x24,
            [0x0014_06c0, fakes::FAKE][((seed / 4) % 2) as usize],
        );
        oc975_w(c, GAME_GP.wrapping_add(0x304), OC975_OBJ + 0x900);
        // The options block, the network flag and the static colour.
        fill(c, &mut r, OVBO_OPTIONS_STATIC, 0x288 / 4);
        oc975_w(c, 0x0053_4b30, (seed / 2) % 2);
        fill(c, &mut r, 0x004c_66c8, 4);
        c.g[A0][0] = sx(OC975_SELF);
        r
    }

    /// Every branch: screen creation failing or not, each widget found or
    /// missing, list box entries found or not, one or two views, the label
    /// callback returning 0 or not, network game or not, both voice-chat
    /// level paths; with unwinding fake callees (host-thread unwind after any
    /// call).
    #[test]
    fn ov_screen_be_options_init_matches() {
        covered(
            ee::s04::f_001f9188__generated,
            ov_screen_be_options_init,
            400,
            &|c, seed| {
                oc975_setup(c, seed, 60);
            },
        );
    }

    /// The call arms the seeds reach (return addresses of logged calls).
    #[test]
    fn ov_screen_be_options_init_reaches_every_call() {
        fakes::check_reaching(
            ee::s04::f_001f9188__generated,
            ov_screen_be_options_init,
            300,
            &[
                0x001f_91f4,
                0x001f_9358,
                0x001f_9368,
                0x001f_93fc,
                0x001f_940c,
                0x001f_9474,
                0x001f_9480,
                0x001f_9524,
                0x001f_9534,
                0x001f_959c,
                0x001f_95a8,
                0x001f_9690,
                0x001f_9748,
                0x001f_9834,
                0x001f_98f0,
                0x001f_99d8,
                0x001f_9a90,
                0x001f_9bb4,
                0x001f_9bc4,
                0x001f_9bcc,
                0x001f_9c88,
                0x001f_9dac,
                0x001f_9dbc,
                0x001f_9dc4,
                0x001f_9e7c,
                0x001f_9eb8,
                0x001f_9ed0,
                0x001f_9f6c,
                0x001f_9fe4,
                0x001f_9ff8,
                0x001f_a02c,
                0x001f_a03c,
                0x001f_a078,
                0x001f_a0b8,
                0x001f_a0d0,
            ],
            &|c, seed| {
                oc975_setup(c, seed, 0);
            },
        );
    }

    /// The seeds make many different paths (distinct cycle totals).
    #[test]
    fn ov_screen_be_options_init_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc975_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s04::f_001f9188__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 40, "only {} distinct paths", seen.len());
    }

    /// Events delivered inside the copy loops (cycle count, event schedule).
    #[test]
    fn ov_screen_be_options_init_events() {
        check_events(
            ee::s04::f_001f9188__generated,
            ov_screen_be_options_init,
            120,
            &|c, seed| {
                let mut r = oc975_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x001f_9188)
            },
        );
    }
}
