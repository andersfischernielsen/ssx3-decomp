//! Staging module oc377 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc377`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, VSlot};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003a_05e8, ui_text_render, "UIText_Render")];

// ------------------------------------------------------------ UIText_Render

/// `cUIText` fields read by `UIText_Render`.
pub(crate) const UI_TEXT_RENDER_VTABLE: u32 = 0x08;
/// Flags word: bit 2 "string built", bit 3 "ignore the off colour", bit 5
/// "highlight allowed"; byte +0x15 bits 0..4: the draw layer.
pub(crate) const UI_TEXT_RENDER_FLAGS14: u32 = 0x14;
pub(crate) const UI_TEXT_RENDER_COLOR: u32 = 0x1c;
pub(crate) const UI_TEXT_RENDER_X: u32 = 0x44;
pub(crate) const UI_TEXT_RENDER_Y: u32 = 0x48;
pub(crate) const UI_TEXT_RENDER_SCALE_X: u32 = 0x50;
pub(crate) const UI_TEXT_RENDER_SCALE_Y: u32 = 0x54;
pub(crate) const UI_TEXT_RENDER_OWNER: u32 = 0x5c;
pub(crate) const UI_TEXT_RENDER_WIDTH: u32 = 0x60;
/// Flags: bit 2 shadow, bit 4 highlighted, bit 5 off, bit 6 formatted lines.
pub(crate) const UI_TEXT_RENDER_FLAGS: u32 = 0x74;
pub(crate) const UI_TEXT_RENDER_SHADOW_COLOR: u32 = 0x78;
pub(crate) const UI_TEXT_RENDER_HI_COLOR: u32 = 0x88;
pub(crate) const UI_TEXT_RENDER_OFF_COLOR: u32 = 0x98;
pub(crate) const UI_TEXT_RENDER_NUM_LINES: u32 = 0xa8;
pub(crate) const UI_TEXT_RENDER_LINES: u32 = 0xac;
pub(crate) const UI_TEXT_RENDER_STR: u32 = 0xb4;
/// `sFont` fields.
pub(crate) const UI_TEXT_RENDER_FONT_HEIGHT: u32 = 0x14;
pub(crate) const UI_TEXT_RENDER_FONT_SHADOW_OFS: u32 = 0x28;
pub(crate) const UI_TEXT_RENDER_FONT_SCALE: u32 = 0x30;
pub(crate) const UI_TEXT_RENDER_FONT_CUR: u32 = 0x38;
pub(crate) const UI_TEXT_RENDER_FONT_COLOR: u32 = 0x40;
pub(crate) const UI_TEXT_RENDER_FONT_SHADOW_COLOR: u32 = 0x50;
/// `D_00508750`: the 0x400-char wide string of 'W's measuring a box.
pub(crate) const UI_TEXT_RENDER_W_BUF: u32 = 0x0050_8750;
/// `gp - 0x854`: the graphics manager (`GrMan`).
pub(crate) const UI_TEXT_RENDER_GRMAN_GP: u32 = 0x854;
/// `gp + 0x2970` (`D_004A5A60`): the default shadow offset.
pub(crate) const UI_TEXT_RENDER_SHADOW_OFS_GP: u32 = 0x2970;

const UI_TEXT_RENDER_GET_FONT: Callee = Callee {
    addr: 0x003a_04f0,
    f: ee::s13::f_003a04f0,
};
const UI_TEXT_RENDER_GET_CHAR_WIDTH: Callee = Callee {
    addr: 0x0039_2ba8,
    f: ee::s13::f_00392ba8,
};
const UI_TEXT_RENDER_APPLY_ALIGNMENT: Callee = Callee {
    addr: 0x0039_feb8,
    f: ee::s13::f_0039feb8,
};
const UI_TEXT_RENDER_USTR_LEN: u32 = 0x003d_bca0;
const UI_TEXT_RENDER_OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
const UI_TEXT_RENDER_USTR_SPRINTF_ASCII_FMT: u32 = 0x002c_2718;
const UI_TEXT_RENDER_FONT_MEASURE: u32 = 0x0039_21f0;
const UI_TEXT_RENDER_FONT_PRINT: u32 = 0x0039_1e30;
const UI_TEXT_RENDER_EA_MEM_FREE: u32 = 0x0031_7e98;

/// Bit `n` of a word, as the `sra; andi 1` pair leaves it.
#[inline]
fn ui_text_render_bit(w: u64, n: u32) -> u64 {
    ((w as u32 >> n) & 1) as u64
}

/// Copy 16 bytes (two unaligned doublewords through registers `r0`, `r1`:
/// `ldl/ldr` both, then `sdl/sdr` both).
fn ui_text_render_copy16(c: &mut Cpu, r0: usize, r1: usize, src: u32, dst: u32) {
    c.g[r0][0] = Addr(src).read_u64_unaligned(c);
    c.g[r1][0] = Addr(src.wrapping_add(8)).read_u64_unaligned(c);
    let v = c.g[r0][0];
    Addr(dst).write_u64_unaligned(c, v);
    let v = c.g[r1][0];
    Addr(dst.wrapping_add(8)).write_u64_unaligned(c, v);
}

/// `void cUIText_render2D(cUIText* self, float x, float y)` (0x3a05e8):
/// draws a UI text element at its position plus (x, y). Rebuilds the string
/// first (vtable slot 16) unless it is already built; if the owner's
/// override flag (bit 27) is set, fills the 'W' buffer to measure the box
/// width. Then sets the font scale, the colour (highlight, off or normal),
/// the shadow and the draw layer, and draws each line (measure, align,
/// print), advancing y by the font height plus 3. A formatted text with
/// several lines is formatted into a temporary buffer and drawn once.
pub fn ui_text_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S1, 0xc0);
        frame.save_fpr(c, 21, 0xe8);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xe0);
        c.f[21] = c.f[13];
        frame.save(c, S0, 0xd0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x60);
        c.f[20] = c.f[12];
        ui_text_render_body(c, frame)?;
        c.cycles += 11;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    })
}

/// Everything between the prologue and the epilogue (every exit has
/// restored `s0` already).
fn ui_text_render_body(c: &mut Cpu, frame: Frame) -> Flow {
    let this = |c: &Cpu| Addr(c.g[S1][0] as u32);
    let font = |c: &Cpu| Addr(c.g[S0][0] as u32);

    // Rebuild the string unless it is there and built.
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_STR).lw(c);
    let mut built = false;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        let flags = this(c).off(UI_TEXT_RENDER_FLAGS14).lw(c);
        c.g[V0][0] = ui_text_render_bit(flags, 2);
        built = c.g[V0][0] != 0;
    }
    if !built {
        c.cycles += 6;
        let vt = this(c).off(UI_TEXT_RENDER_VTABLE).lw(c);
        c.g[V0][0] = vt;
        let slot = VSlot::read(c, vt, 0x80);
        c.g[V1][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[S1][0]);
        call_ptr(c, slot.func as u32, 0x003a_0650)?;
    }
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_OWNER).lw(c);

    // The owner's override: fill the 'W' buffer to the box width.
    c.cycles += 5;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    c.g[S4][0] = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
    'measure: {
        if c.g[S4][0] == 0 {
            break 'measure;
        }
        c.cycles += 5;
        let ov = Addr(c.g[S4][0] as u32).lw(c);
        c.g[V0][0] = ui_text_render_bit(ov, 27);
        if c.g[V0][0] == 0 {
            break 'measure;
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        UI_TEXT_RENDER_GET_FONT.call(c, 0x003a_0684)?;
        c.g[A0][0] = c.g[V0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
            return Ok(());
        }
        c.cycles += 12;
        c.g[A1][0] = 0x57;
        UI_TEXT_RENDER_GET_CHAR_WIDTH.call(c, 0x003a_0694)?;
        let scale_x = this(c).off(UI_TEXT_RENDER_SCALE_X).read_fpu(c);
        c.g[A0][0] = 0;
        let width = this(c).off(UI_TEXT_RENDER_WIDTH).read_fpu(c);
        let char_w = FpuF32(c.f[0]) * scale_x;
        let q = width.div(char_w);
        let n = q.to_i32();
        c.f[2] = scale_x.0;
        c.f[1] = q.0;
        c.f[0] = n as u32;
        c.g[A1][0] = sx(n as u32);
        c.g[V0][0] = (n < 1024) as u64;
        c.g[V1][0] = 0x51_0000;
        if n < 1024 {
            c.cycles += 2;
            c.g[V0][0] = sx(UI_TEXT_RENDER_W_BUF);
            if n > 0 {
                c.cycles += 3;
                c.g[A2][0] = 0x57;
                c.g[A0][0] = c.g[A1][0];
                loop {
                    c.cycles += 7;
                    let w = c.g[A2][0];
                    Addr(c.g[V0][0] as u32).sh(c, w);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(2));
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_06d0);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[A1][0];
            }
        }
        c.cycles += 5;
        c.g[V1][0] = sx(UI_TEXT_RENDER_W_BUF);
        c.g[V0][0] = sx(((c.g[A0][0] as u32) << 1).wrapping_add(UI_TEXT_RENDER_W_BUF));
        Addr(c.g[V0][0] as u32).sh(c, 0);
    }
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_STR).lw(c);

    c.cycles += 2;
    frame.restore(c, S0, 0xd0);
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    UI_TEXT_RENDER_GET_FONT.call(c, 0x003a_0714)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        frame.restore(c, S0, 0xd0);
        return Ok(());
    }

    // Font scale: s = (sx, sy) kept at sp+0.
    c.cycles += 16;
    let spa = sp(c);
    let scale_x = this(c).off(UI_TEXT_RENDER_SCALE_X).read_fpu(c);
    let scale_y = this(c).off(UI_TEXT_RENDER_SCALE_Y).read_fpu(c);
    Addr(spa).write_fpu(c, scale_x);
    Addr(spa + 4).write_fpu(c, scale_y);
    let font_sx = font(c).off(UI_TEXT_RENDER_FONT_SCALE).read_fpu(c);
    let font_sy = font(c).off(UI_TEXT_RENDER_FONT_SCALE + 4).read_fpu(c);
    let cur_x = font_sx * scale_x;
    font(c).off(UI_TEXT_RENDER_FONT_CUR).write_fpu(c, cur_x);
    let s_y = Addr(spa + 4).read_fpu(c);
    let cur_y = font_sy * s_y;
    font(c).off(UI_TEXT_RENDER_FONT_CUR + 4).write_fpu(c, cur_y);
    c.f[3] = scale_x.0;
    c.f[1] = s_y.0;
    c.f[0] = cur_x.0;
    c.f[2] = cur_y.0;

    // Colour: highlight, off or normal.
    let flags14 = this(c).off(UI_TEXT_RENDER_FLAGS14).lw(c);
    c.g[V0][0] = ui_text_render_bit(flags14, 5);
    'color: {
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
            c.g[V0][0] = ui_text_render_bit(flags, 4);
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                let (src, dst) = (
                    this(c).off(UI_TEXT_RENDER_HI_COLOR).0,
                    font(c).off(UI_TEXT_RENDER_FONT_COLOR).0,
                );
                ui_text_render_copy16(c, V0, V1, src, dst);
                break 'color;
            }
        }
        c.cycles += 4;
        let flags14 = this(c).off(UI_TEXT_RENDER_FLAGS14).lw(c);
        c.g[V0][0] = ui_text_render_bit(flags14, 3);
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
            c.g[V0][0] = ui_text_render_bit(flags, 5);
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                let (src, dst) = (
                    this(c).off(UI_TEXT_RENDER_OFF_COLOR).0,
                    font(c).off(UI_TEXT_RENDER_FONT_COLOR).0,
                );
                ui_text_render_copy16(c, A3, T0, src, dst);
                break 'color;
            }
        }
        c.cycles += 9;
        let (src, dst) = (
            this(c).off(UI_TEXT_RENDER_COLOR).0,
            font(c).off(UI_TEXT_RENDER_FONT_COLOR).0,
        );
        ui_text_render_copy16(c, A0, A1, src, dst);
    }

    // Shadow: the shadow colour with the text's red, and the default offset.
    let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
    c.cycles += 4;
    c.g[V0][0] = ui_text_render_bit(flags, 2);
    let gp = c.g[GP][0] as u32;
    if c.g[V0][0] == 0 {
        Addr(spa + 0x10).sw(c, 0);
        c.cycles += 6;
        Addr(spa + 0x14).sw(c, 0);
        c.g[V0][0] = Addr(spa + 0x10).read_u64_unaligned(c);
        let zero = c.g[V0][0];
        let dst = font(c).off(UI_TEXT_RENDER_FONT_SHADOW_OFS);
        dst.write_u64_unaligned(c, zero);
    } else {
        c.cycles += 24;
        let red = this(c).off(UI_TEXT_RENDER_COLOR).read_fpu(c);
        c.f[0] = red.0;
        let src = this(c).off(UI_TEXT_RENDER_SHADOW_COLOR).0;
        ui_text_render_copy16(c, V0, V1, src, spa + 0x10);
        Addr(spa + 0x10).write_fpu(c, red);
        let dst = font(c).off(UI_TEXT_RENDER_FONT_SHADOW_COLOR).0;
        ui_text_render_copy16(c, A3, T0, spa + 0x10, dst);
        c.g[A0][0] = Addr(gp.wrapping_add(UI_TEXT_RENDER_SHADOW_OFS_GP)).read_u64_unaligned(c);
        let ofs = c.g[A0][0];
        let dst = font(c).off(UI_TEXT_RENDER_FONT_SHADOW_OFS);
        dst.write_u64_unaligned(c, ofs);
    }
    c.g[V1][0] = Addr(gp.wrapping_sub(UI_TEXT_RENDER_GRMAN_GP)).lw(c);

    // Draw layer (bits 5..9 of the draw state) and the start position.
    c.cycles += 21;
    c.g[A1][0] = 0xffff_ffff_ffff_fc1f;
    let layer = this(c).off(UI_TEXT_RENDER_FLAGS14 + 1).lbu(c);
    c.g[S2][0] = 0;
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xe84).lw(c);
    let state = Addr(c.g[A0][0] as u32).off(8).lw(c);
    c.g[V0][0] = (((layer & 0x1f) + 9) << 5) & 0x3e0;
    c.g[V1][0] = (state & c.g[A1][0]) | c.g[V0][0];
    let v = c.g[V1][0];
    Addr(c.g[A0][0] as u32).off(8).sw(c, v);
    Addr(spa + 0x18).sw(c, 0);
    let pos_x = this(c).off(UI_TEXT_RENDER_X).read_fpu(c) + FpuF32(c.f[20]);
    let y = this(c).off(UI_TEXT_RENDER_Y).read_fpu(c);
    c.g[V0][0] = this(c).off(UI_TEXT_RENDER_NUM_LINES).lw(c);
    let pos_y = y + FpuF32(c.f[21]);
    c.f[0] = pos_x.0;
    c.f[1] = pos_y.0;
    Addr(spa + 0x10).write_fpu(c, pos_x);
    Addr(spa + 0x14).write_fpu(c, pos_y);
    if (c.g[V0][0] as i64) <= 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0xd0);
        return Ok(());
    }
    c.cycles += 4;
    c.g[AT][0] = 0x4040_0000;
    c.f[20] = 0x4040_0000; // 3.0: the line gap
    c.g[S5][0] = 0x49_0000;
    c.g[S6][0] = 0x4a_0000;

    loop {
        // The line: the string for line 0, else lines[i].
        c.cycles += 2;
        let i = c.g[S2][0] as u32;
        c.g[V0][0] = sx(i << 2);
        if i == 0 {
            c.cycles += 1;
            c.g[S3][0] = this(c).off(UI_TEXT_RENDER_STR).lw(c);
        } else {
            c.cycles += 4;
            c.g[V1][0] = this(c).off(UI_TEXT_RENDER_LINES).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S3][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 2;
        if c.g[S4][0] != 0 {
            c.cycles += 5;
            let ov = Addr(c.g[S4][0] as u32).lw(c);
            let over = ui_text_render_bit(ov, 27) != 0;
            c.g[V0][0] = 0x51_0000;
            if over {
                c.cycles += 1;
                c.g[S3][0] = sx(UI_TEXT_RENDER_W_BUF);
            }
            c.cycles += 1;
        }
        let flags = this(c).off(UI_TEXT_RENDER_FLAGS).lw(c);
        c.cycles += 4;
        c.g[V0][0] = ui_text_render_bit(flags, 6);
        c.f[12] = Addr(spa).read_u32(c);
        let formatted = if c.g[V0][0] == 0 {
            c.cycles += 1;
            false
        } else {
            c.cycles += 4;
            let lines = this(c).off(UI_TEXT_RENDER_NUM_LINES).lw(c);
            c.g[V0][0] = ((lines as i64) < 2) as u64;
            c.g[V0][0] == 0
        };

        if formatted {
            // Format the line into a temporary buffer, draw it, done.
            c.cycles += 57;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, UI_TEXT_RENDER_USTR_LEN, 0x003a_0960)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(5));
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4078));
            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A2][0] = 0x100;
            call_ptr(c, UI_TEXT_RENDER_OPERATOR_NEW_NAMED, 0x003a_0978)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x47b0));
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, UI_TEXT_RENDER_USTR_SPRINTF_ASCII_FMT, 0x003a_098c)?;
            let spa = sp(c);
            c.f[12] = Addr(spa).read_u32(c);
            c.g[A2][0] = sx(spa + 0x20);
            c.f[13] = Addr(spa + 4).read_u32(c);
            c.g[A3][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, UI_TEXT_RENDER_FONT_MEASURE, 0x003a_09a8)?;
            ui_text_render_align(c, 0x40, 0x003a_09f8)?;
            ui_text_render_print(c, 0x40, S2, 0x003a_0a34)?;
            frame.restore(c, S0, 0xd0);
            if c.g[S2][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, UI_TEXT_RENDER_EA_MEM_FREE, 0x003a_0a44)?;
                frame.restore(c, S0, 0xd0);
            }
            return Ok(());
        }

        c.g[A2][0] = sx(spa + 0x20);
        c.cycles += 45;
        c.f[13] = Addr(spa + 4).read_u32(c);
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, UI_TEXT_RENDER_FONT_MEASURE, 0x003a_0a64)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        ui_text_render_align(c, 0x50, 0x003a_0ab8)?;
        ui_text_render_print(c, 0x50, S3, 0x003a_0af4)?;
        c.g[V0][0] = this(c).off(UI_TEXT_RENDER_NUM_LINES).lw(c);
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0xd0);
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_08f8);
        }
    }
}

/// After measuring the line into the rect at sp+0x20: the box size
/// (rect.x1, sy * height * font scale y) at sp+0x30, a copy of the pen
/// position (sp+0x10) at `p` (sp-relative), and `UIObject_ApplyAlignment
/// (self, &p, &size)`.
fn ui_text_render_align(c: &mut Cpu, p: u32, ret: u32) -> Flow {
    let spa = sp(c);
    let font = Addr(c.g[S0][0] as u32);
    let height = FpuF32::from_i32(font.off(UI_TEXT_RENDER_FONT_HEIGHT).read_u32(c) as i32);
    let font_sy = font.off(UI_TEXT_RENDER_FONT_SCALE + 4).read_fpu(c);
    let sy = Addr(c.g[S1][0] as u32)
        .off(UI_TEXT_RENDER_SCALE_Y)
        .read_fpu(c);
    c.g[A1][0] = sx(spa + p);
    let x1 = Addr(spa + 0x28).read_fpu(c);
    c.g[A0][0] = c.g[S1][0];
    let line_h = height * font_sy;
    Addr(spa + 0x38).sw(c, 0);
    Addr(spa + 0x30).write_fpu(c, x1);
    let size_y = sy * line_h;
    Addr(spa + 0x34).write_fpu(c, size_y);
    c.f[1] = line_h.0;
    c.f[3] = font_sy.0;
    c.f[2] = x1.0;
    c.f[0] = size_y.0;
    c.g[A3][0] = Addr(spa + 0x10).read_u64_unaligned(c);
    c.g[T0][0] = Addr(spa + 0x18).lw(c);
    let pos = c.g[A3][0];
    Addr(spa + p).write_u64_unaligned(c, pos);
    let z = c.g[T0][0];
    Addr(spa + p + 8).sw(c, z);
    c.g[A2][0] = sx(spa + 0x30);
    UI_TEXT_RENDER_APPLY_ALIGNMENT.call(c, ret)
}

/// Advance the pen by `height * font scale y * s.y + 3` and print the
/// string in register `text` at the aligned position at `p` (sp-relative).
fn ui_text_render_print(c: &mut Cpu, p: u32, text: usize, ret: u32) -> Flow {
    let spa = sp(c);
    let font = Addr(c.g[S0][0] as u32);
    let height = FpuF32::from_i32(font.off(UI_TEXT_RENDER_FONT_HEIGHT).read_u32(c) as i32);
    let font_sy = font.off(UI_TEXT_RENDER_FONT_SCALE + 4).read_fpu(c);
    let s_y = Addr(spa + 4).read_fpu(c);
    c.g[A0][0] = c.g[S0][0];
    let pen_y = Addr(spa + 0x14).read_fpu(c);
    c.g[A1][0] = c.g[text][0];
    let step = height * font_sy;
    c.f[12] = Addr(spa + p).read_u32(c);
    c.f[13] = Addr(spa + p + 4).read_u32(c);
    let step = step * s_y + FpuF32(c.f[20]);
    let pen_y = pen_y + step;
    c.f[0] = step.0;
    c.f[1] = pen_y.0;
    c.f[2] = font_sy.0;
    c.f[3] = s_y.0;
    Addr(spa + 0x14).write_fpu(c, pen_y);
    call_ptr(c, UI_TEXT_RENDER_FONT_PRINT, ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // ---------------------------------------------------- UIText_Render

    const UTR_THIS: u32 = DATA;
    const UTR_OWNER: u32 = DATA + 0x200;
    const UTR_P1: u32 = DATA + 0x300;
    const UTR_P2: u32 = DATA + 0x380;
    const UTR_OV: u32 = DATA + 0x400;
    const UTR_FONTS: u32 = DATA + 0x480;
    const UTR_FONT: u32 = DATA + 0x500;
    const UTR_GRMAN: u32 = DATA + 0x600;
    const UTR_STATE: u32 = DATA + 0x700;
    const UTR_DRAWER: u32 = DATA + 0x780;
    const UTR_STR: u32 = DATA + 0x800;
    const UTR_LINE_TABLE: u32 = DATA + 0x880;
    const UTR_LINES: u32 = DATA + 0x900;
    const UTR_VT: u32 = DATA + 0x1000;
    const UTR_GLYPHS: u32 = DATA + 0x2000;
    const UTR_BUF: u32 = DATA + 0x3000;
    /// Gate of the allocator/format/free fakes (+4: make the allocation fail).
    const UTR_CTRL: u32 = DATA + 0xf_f000;
    const UTR_MAGIC: u32 = 0x5554_5852;

    static UTR_PREV_NEW: AtomicUsize = AtomicUsize::new(0);
    static UTR_PREV_FMT: AtomicUsize = AtomicUsize::new(0);
    static UTR_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    fn utr_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if addr == 0x0031_7e30 && c.pc == ra && c.bus.read32(UTR_CTRL + 4) != 0 {
            c.g[V0][0] = 0;
        }
    }

    fn utr_fake_new(c: &mut Cpu) {
        if c.bus.read32(UTR_CTRL) != UTR_MAGIC {
            return fakes::fall_back(c, &UTR_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        utr_fake(c, 0x0031_7e30)
    }

    fn utr_fake_fmt(c: &mut Cpu) {
        if c.bus.read32(UTR_CTRL) != UTR_MAGIC {
            return fakes::fall_back(c, &UTR_PREV_FMT, ee::s08::f_002c2718__generated);
        }
        utr_fake(c, 0x002c_2718)
    }

    fn utr_fake_free(c: &mut Cpu) {
        if c.bus.read32(UTR_CTRL) != UTR_MAGIC {
            return fakes::fall_back(c, &UTR_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        utr_fake(c, 0x0031_7e98)
    }

    fn utr_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            utr_fake_new,
            &UTR_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2718),
            utr_fake_fmt,
            &UTR_PREV_FMT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            utr_fake_free,
            &UTR_PREV_FREE,
        );
    }

    /// A short wide string of printable characters.
    fn utr_wide(c: &mut Cpu, r: &mut crate::difftest::Rng, at: u32, max: u32) {
        let n = r.next_u32() % (max + 1);
        for i in 0..n {
            c.bus
                .write16(at + 2 * i, (0x20 + r.next_u32() % 0x5f) as u16);
        }
        c.bus.write16(at + 2 * n, 0);
    }

    /// `UIText_Render` against the generated body: built/unbuilt string
    /// (vtable rebuild), the owner's 'W' override (box widths below 0,
    /// small, and past the 0x400 buffer), missing fonts, the three colour
    /// choices, shadow on/off, 0..3 lines and the formatted multi-line path
    /// (with a failing allocation). Font measuring, alignment and the 'W'
    /// width run real; the print's vtable call, the allocator, the format
    /// and the free are fakes.
    #[test]
    fn ui_text_render_matches() {
        utr_install_fakes();
        check(
            ee::s13::f_003a05e8__generated,
            ui_text_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a_05e8, 6);
                c.pc = 0x003a_05e8;
                c.bus.write32(UTR_CTRL, UTR_MAGIC);
                c.bus.write32(UTR_CTRL + 4, (seed % 5 == 0) as u32);
                for i in 0..8 {
                    c.bus.write32(LOG_RESULTS + 4 * i, UTR_BUF);
                }
                // The formatted buffer holds a short string.
                utr_wide(c, &mut r, UTR_BUF, 4);
                let this = UTR_THIS;
                c.g[A0][0] = sx(this);
                c.f[12] = r.f32(300.0);
                c.f[13] = r.f32(300.0);
                for k in 0..0x30 {
                    c.bus.write32(this + 4 * k, r.next_u32());
                }
                // Rebuild vtable slot 16 (+0x80).
                c.bus.write32(this + 8, UTR_VT);
                c.bus.write16(UTR_VT + 0x80, (r.next_u32() as u16) & 0xfc);
                c.bus.write32(UTR_VT + 0x84, FAKE);
                c.bus.write32(
                    this + 0x50,
                    (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits(),
                );
                c.bus.write32(this + 0x54, r.f32(2.0));
                c.bus.write32(this + 0x44, r.f32(200.0));
                c.bus.write32(this + 0x48, r.f32(200.0));
                // Box width: around zero, far past the 0x400 'W's, or small.
                let width = match seed % 7 {
                    0 => r.f32(50.0),
                    1 => 1.0e7f32.to_bits(),
                    _ => ((r.next_u32() % 400) as f32).to_bits(),
                };
                c.bus.write32(this + 0x60, width);
                c.bus.write32(this + 0x5c, UTR_OWNER);
                let lines = match seed % 9 {
                    0 => 0,
                    1 => -1i32 as u32,
                    n => 1 + n % 3,
                };
                c.bus.write32(this + 0xa8, lines);
                c.bus.write32(this + 0xac, UTR_LINE_TABLE);
                let has_str = seed % 11 != 0;
                c.bus
                    .write32(this + 0xb4, if has_str { UTR_STR } else { 0 });
                utr_wide(c, &mut r, UTR_STR, 6);
                for i in 0..4 {
                    let at = UTR_LINES + 0x40 * i;
                    c.bus.write32(UTR_LINE_TABLE + 4 * i, at);
                    utr_wide(c, &mut r, at, 6);
                }
                // Owner -> +0xd0 -> +0x10: the font list and the override flag.
                c.bus.write32(UTR_OWNER + 0xd0, UTR_P1);
                c.bus.write32(UTR_P1 + 0x10, UTR_P2);
                c.bus.write32(UTR_P2 + 8, UTR_FONTS);
                let ov = if seed % 3 == 0 { 0 } else { UTR_OV };
                c.bus.write32(UTR_P2 + 0x10, ov);
                c.bus.write32(UTR_OV, r.next_u32());
                let no_font = seed % 13 == 5;
                for i in 0..4 {
                    c.bus
                        .write32(UTR_FONTS + 12 * i + 0xc, if no_font { 0 } else { UTR_FONT });
                }
                // The font: glyphs ' '..'~', 12 bytes each.
                let font = UTR_FONT;
                for k in 0..0x18 {
                    c.bus.write32(font + 4 * k, r.next_u32());
                }
                c.bus.write32(font + 4, UTR_GLYPHS);
                c.bus.write32(font + 0xc, 0x20);
                c.bus.write32(font + 0x10, 0x7e);
                c.bus.write32(font + 0x14, 6 + r.next_u32() % 20);
                for k in [0x18, 0x1c, 0x20, 0x24, 0x30, 0x34] {
                    c.bus.write32(font + k, r.f32(3.0));
                }
                for g in 0..0x5f {
                    let at = UTR_GLYPHS + 12 * g;
                    for b in 0..12 {
                        c.bus.write8(at + b, (r.next_u32() % 24) as u8);
                    }
                }
                if seed % 17 == 3 {
                    // A zero-width 'W'.
                    c.bus.write8(UTR_GLYPHS + 12 * (0x57 - 0x20) + 8, 0);
                }
                // The graphics manager: draw state and the printer's vtable.
                c.bus.write32(GAME_GP - UI_TEXT_RENDER_GRMAN_GP, UTR_GRMAN);
                c.bus.write32(GAME_GP + 0x2a90, UTR_GRMAN);
                c.bus.write32(UTR_GRMAN + 0xe84, UTR_STATE);
                c.bus.write32(UTR_GRMAN + 0x10d8, UTR_DRAWER);
                c.bus.write32(UTR_STATE + 8, r.next_u32());
                c.bus
                    .write16(UTR_DRAWER + 0x288, (r.next_u32() as u16) & 0xfc);
                c.bus.write32(UTR_DRAWER + 0x28c, FAKE + 0x10);
                c.bus
                    .write32(GAME_GP + UI_TEXT_RENDER_SHADOW_OFS_GP, r.next_u32());
                c.bus
                    .write32(GAME_GP + UI_TEXT_RENDER_SHADOW_OFS_GP + 4, r.next_u32());
            },
        );
    }
}
