//! Staging module oc391 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc391`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_9f00, ui_list_box_render, "UIListBox_Render"),
    (0x002b_d518, twpigd_surface_blend, "tWPIGD_Surface_Blend"),
    (0x002b_d2e8, twpigd_sky_box_blend, "tWPIGD_SkyBox_Blend"),
    (
        0x0039_5f80,
        ps2_graphics_man_is_idle,
        "PS2GraphicsMan_IsIdle",
    ),
    (0x0031_7618, str_hash_elf, "StrHashElf"),
    (0x0042_3ca0, refer_thread_status, "ReferThreadStatus"),
    (
        0x0042_3c50,
        rotate_thread_ready_queue,
        "RotateThreadReadyQueue",
    ),
    (
        0x0030_1680,
        wscript_fn_get_object_slot30,
        "WScriptFn_GetObjectSlot30",
    ),
    (
        0x0036_98e0,
        ps2_graphics_man_alloc_blended_matrix_history,
        "PS2GraphicsMan_AllocBlendedMatrixHistory",
    ),
    (0x0041_6b18, strncmp, "strncmp"),
    (0x0021_f338, hud_draw_pip_meter, "HUD_DrawPipMeter"),
    (0x0019_d250, bolt_file_cache_update, "BoltFile_CacheUpdate"),
    (
        0x003d_bb68,
        loc_string_table_get_string_ptr,
        "LocStringTable_GetStringPtr",
    ),
    (
        0x002a_78f8,
        ini_parser_skip_whitespace,
        "IniParser_SkipWhitespace",
    ),
    (0x0019_d4b8, bolt_file_free_slot, "BoltFile_FreeSlot"),
    (
        0x0013_c140,
        rider_railslide_state_collide_walls,
        "RiderRailslideState_CollideWalls",
    ),
    (0x001f_1e28, hud_draw_text_lerp, "HUD_DrawTextLerp"),
    (0x0026_3828, net_update_connect, "Net_UpdateConnect"),
];

/// `reg + off` as a 32-bit guest address.
#[inline(always)]
fn oc391_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// UIListBox_Render (0x399f00)

/// `UIListBox_GetModeEntry` (0x399920): the font of the current mode.
const UILB_GET_MODE_ENTRY: Callee = Callee {
    addr: 0x0039_9920,
    f: ee::s13::UIListBox_GetModeEntry,
};
/// `UIObject_ApplyAlignment` (0x39feb8).
const UILB_APPLY_ALIGNMENT: Callee = Callee {
    addr: 0x0039_feb8,
    f: ee::s13::UIObject_ApplyAlignment,
};
/// `Font_MeasureTextW` (0x3921f0, another shard: through the table).
const UILB_MEASURE_TEXT: u32 = 0x0039_21f0;
/// `Font_DrawTextW` (0x391e30, another shard: through the table).
const UILB_DRAW_TEXT: u32 = 0x0039_1e30;
/// The renderer global (`gp-0x854`): `+0xe84` its render state, `+0x10d8`
/// its vtable.
const UILB_RENDERER_GP: u32 = 0x854;

/// `ldl/ldr` 16 bytes at `src` into v0/v1 and `sdl/sdr` them to `dst`.
fn ui_list_box_render_copy16(c: &mut Cpu, src: u32, dst: u32) {
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.ldl(3, src.wrapping_add(0xf));
    c.ldr(3, src.wrapping_add(8));
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    c.sdl(3, dst.wrapping_add(0xf));
    c.sdr(3, dst.wrapping_add(8));
}

/// Set the render state's layer bits (`state+8`, bits 5..9) to the list
/// box's layer (`this+0x15 & 0x1f`) + 9. Leaves `v0`/`v1` as the original.
fn ui_list_box_render_set_layer(c: &mut Cpu, state_reg: usize) {
    let st = c.g[state_reg][0] as u32;
    c.g[V1][0] = Addr(st.wrapping_add(8)).lw(c);
    c.g[V0][0] &= 0x1f;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(9));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V1][0] &= c.g[S6][0];
    c.g[V0][0] &= 0x3e0;
    c.g[V1][0] |= c.g[V0][0];
    Addr(st.wrapping_add(8)).sw(c, c.g[V1][0]);
}

/// `void UIListBox_Render(UIListBox* self, float x, float y)` (0x399f00):
/// draw the selected string of a list box with its mode's font (scale,
/// colour, optional drop shadow), aligned at `pos + (x, y)`; with the debug
/// outline flag of the state data set, also draw the box outline (a red
/// line strip, or a dark red filled quad when the text overflows the box).
pub fn ui_list_box_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S0, 0x1b0);
        frame.save_fpr(c, 21, 0x1c8);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x1c0);
        c.mov_s(21, 13); // y
        frame.save(c, S1, 0x1a0);
        frame.save(c, S2, 0x190);
        frame.save(c, S3, 0x180);
        frame.save(c, S4, 0x170);
        frame.save(c, S5, 0x160);
        frame.save(c, S6, 0x150);
        frame.save_ra(c, 0x140);
        c.mov_s(20, 12); // x
        UILB_GET_MODE_ENTRY.call(c, 0x0039_9f3c)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(oc391_at(c, S0, 0x318)).lbu(c); // item count
            if c.g[V0][0] != 0 {
                ui_list_box_render_body(c)?;
            }
        }
        frame.restore(c, S0, 0x1b0);
        c.cycles += 11;
        frame.restore(c, S1, 0x1a0);
        frame.restore(c, S2, 0x190);
        frame.restore(c, S3, 0x180);
        frame.restore(c, S4, 0x170);
        frame.restore(c, S5, 0x160);
        frame.restore(c, S6, 0x150);
        frame.restore_ra(c, 0x140);
        frame.restore_fpr(c, 21, 0x1c8);
        frame.restore_fpr(c, 20, 0x1c0);
        frame.close(c);
        Ok(())
    })
}

/// The drawing part of [`ui_list_box_render`] (s0 = self, s1 = font).
fn ui_list_box_render_body(c: &mut Cpu) -> crate::call::Flow {
    let sp0 = sp(c);
    // Font scale times the list box's scale.
    c.cycles += 16;
    c.lwc1(3, oc391_at(c, S0, 0x50));
    c.lwc1(1, oc391_at(c, S0, 0x54));
    c.swc1(3, sp0);
    c.swc1(1, sp0 + 4);
    c.lwc1(0, oc391_at(c, S1, 0x30));
    c.lwc1(2, oc391_at(c, S1, 0x34));
    c.mul_s(0, 0, 3);
    c.swc1(0, oc391_at(c, S1, 0x38));
    c.lwc1(1, sp0 + 4);
    c.mul_s(2, 2, 1);
    c.swc1(2, oc391_at(c, S1, 0x3c));
    // Colour: the highlight colour when both highlight bits are set.
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x14)).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32) & 1;
    let mut highlight = false;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(oc391_at(c, S0, 0x74)).lw(c);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32) & 1;
        highlight = c.g[V0][0] != 0;
    }
    if highlight {
        c.cycles += 10;
        ui_list_box_render_copy16(c, oc391_at(c, S0, 0x88), oc391_at(c, S1, 0x40));
    } else {
        c.cycles += 9;
        ui_list_box_render_copy16(c, oc391_at(c, S0, 0x1c), oc391_at(c, S1, 0x40));
    }
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x74)).lw(c);
    // Drop shadow.
    c.cycles += 4;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32) & 1;
    if c.g[V0][0] == 0 {
        Addr(sp0 + 0x10).sw(c, 0);
        c.cycles += 6;
        Addr(sp0 + 0x14).sw(c, 0);
        c.ldl(2, sp0 + 0x17);
        c.ldr(2, sp0 + 0x10);
        c.sdl(2, oc391_at(c, S1, 0x2f));
        c.sdr(2, oc391_at(c, S1, 0x28));
    } else {
        c.cycles += 28;
        c.lwc1(0, oc391_at(c, S0, 0x1c));
        ui_list_box_render_copy16(c, oc391_at(c, S0, 0x78), sp0 + 0x10);
        c.swc1(0, sp0 + 0x10);
        c.g[AT][0] = 0x4000_0000; // 2.0f
        c.mtc1(1, 1);
        ui_list_box_render_copy16(c, sp0 + 0x10, oc391_at(c, S1, 0x50));
        c.swc1(1, sp0 + 0x24);
        c.swc1(1, sp0 + 0x20);
        c.ldl(2, sp0 + 0x27);
        c.ldr(2, sp0 + 0x20);
        c.sdl(2, oc391_at(c, S1, 0x2f));
        c.sdr(2, oc391_at(c, S1, 0x28));
    }
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x74)).lw(c);
    // The string: the first one, or the selected one (+0x319).
    c.cycles += 3;
    c.g[V0][0] &= 0x10;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(oc391_at(c, S0, 0x319)).lbu(c);
        c.cycles += 4;
        c.g[V1][0] = 0x14;
        c.mult(0, A0, V0, V1);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[S2][0] = Addr(oc391_at(c, V0, 0x9c)).lw(c);
    } else {
        c.cycles += 2;
        c.g[S2][0] = Addr(oc391_at(c, S0, 0x9c)).lw(c);
    }
    // Measure the string.
    c.cycles += 28;
    c.g[V1][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    c.g[S6][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x15)).lbu(c);
    c.g[S3][0] = sx(sp0 + 0x10);
    c.g[T0][0] = Addr(oc391_at(c, V1, 0xe84)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = 0;
    ui_list_box_render_set_layer(c, T0);
    c.g[S5][0] = 0;
    c.g[S4][0] = sx(oc391_at(c, S0, 0x60));
    c.lwc1(12, sp0);
    c.lwc1(13, sp0 + 4);
    call_ptr(c, UILB_MEASURE_TEXT, 0x0039_a108)?;
    c.lwc1(5, sp0 + 0x18);
    c.lwc1(0, oc391_at(c, S0, 0x60));
    c.c_lt_s(0, 5);
    if c.fcc() {
        c.g[S5][0] = 1; // the text is wider than the box
    }
    // Align and draw the text.
    c.cycles += 31;
    c.lwc1(2, oc391_at(c, S1, 0x14));
    c.cvt_s_w(2, 2);
    c.lwc1(4, oc391_at(c, S1, 0x34));
    c.lwc1(3, oc391_at(c, S0, 0x54));
    c.g[A1][0] = sx(sp0 + 0x40);
    c.lwc1(1, oc391_at(c, S0, 0x44));
    c.g[A0][0] = c.g[S0][0];
    c.mul_s(2, 2, 4);
    c.lwc1(0, oc391_at(c, S0, 0x48));
    c.add_s(1, 1, 20);
    c.swc1(5, sp0 + 0x30);
    c.add_s(0, 0, 21);
    Addr(sp0 + 0x38).sw(c, 0);
    c.mul_s(3, 3, 2);
    Addr(sp0 + 0x48).sw(c, 0);
    c.swc1(1, sp0 + 0x40);
    c.g[A2][0] = sx(sp0 + 0x30);
    c.swc1(0, sp0 + 0x44);
    c.swc1(3, sp0 + 0x34);
    UILB_APPLY_ALIGNMENT.call(c, 0x0039_a170)?;
    c.lwc1(12, sp0 + 0x40);
    c.g[A0][0] = c.g[S1][0];
    c.lwc1(13, sp0 + 0x44);
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, UILB_DRAW_TEXT, 0x0039_a184)?;
    // The debug outline: state data +0x10, bit 26 of its first word.
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x5c)).lw(c);
    c.g[V1][0] = Addr(oc391_at(c, V0, 0xd0)).lw(c);
    c.g[A0][0] = Addr(oc391_at(c, V1, 0x10)).lw(c);
    c.g[V0][0] = Addr(oc391_at(c, A0, 0x10)).lw(c);
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32) & 1;
    c.g[A2][0] = c.g[S4][0];
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 32;
    c.ldl(5, oc391_at(c, S0, 0x4b));
    c.ldr(5, oc391_at(c, S0, 0x44));
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x4c)).lw(c);
    c.sdl(5, sp0 + 0x17);
    c.sdr(5, sp0 + 0x10);
    Addr(sp0 + 0x18).sw(c, c.g[V0][0]);
    c.g[A1][0] = c.g[S3][0];
    c.g[A0][0] = c.g[S0][0];
    c.lwc1(0, sp0 + 0x10);
    c.lwc1(1, sp0 + 0x14);
    c.add_s(0, 0, 20);
    c.add_s(1, 1, 21);
    c.swc1(0, sp0 + 0x10);
    c.swc1(1, sp0 + 0x14);
    UILB_APPLY_ALIGNMENT.call(c, 0x0039_a1ec)?;
    c.g[V1][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    c.g[A2][0] = sx(sp0 + 0x50);
    c.g[V0][0] = Addr(oc391_at(c, S0, 0x15)).lbu(c);
    c.g[A3][0] = u64::MAX;
    c.g[A0][0] = Addr(oc391_at(c, V1, 0xe84)).lw(c);
    c.g[A1][0] = 4;
    ui_list_box_render_set_layer(c, A0);
    let over = c.g[S5][0] != 0;
    c.g[V0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    let verts = c.g[A2][0] as u32;
    if !over {
        // Five red vertices: a closed line strip around the box.
        c.cycles += 6;
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 3);
        c.g[A3][0] = 0xff;
        c.g[A1][0] = sx(sp0 + 0x70);
        c.g[A0][0] = c.g[A2][0];
        c.g[T0][0] = sx(verts.wrapping_add(0xf0));
        loop {
            ui_list_box_render_vertex(c, A0, A3, A3, A1);
            c.g[V1][0] = (c.g[A0][0] < c.g[T0][0]) as u64;
            if c.g[V1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_a248);
            }
        }
        c.cycles += 24;
        c.lwc1(5, oc391_at(c, S0, 0x64));
        c.g[A1][0] = 5;
        c.lwc1(4, oc391_at(c, S0, 0x60));
        c.g[A3][0] = 0;
        c.lwc1(3, sp0 + 0xa0);
        c.lwc1(1, sp0 + 0xd0);
        c.lwc1(2, sp0 + 0xd4);
        c.add_s(3, 3, 4);
        c.lwc1(0, sp0 + 0x104);
        c.add_s(1, 1, 4);
        c.add_s(2, 2, 5);
        c.g[T0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
        c.add_s(0, 0, 5);
        c.swc1(3, sp0 + 0xa0); // v1.x += w
        c.swc1(1, sp0 + 0xd0); // v2.x += w
        c.swc1(2, sp0 + 0xd4); // v2.y += h
        c.swc1(0, sp0 + 0x104); // v3.y += h
        ui_list_box_render_draw(c, 0x248, 0x0039_a2f0)?;
        return Ok(());
    }
    // Four dark red vertices: a filled quad over the box.
    c.cycles += 10;
    c.g[A0][0] = u64::MAX;
    c.g[AT][0] = 0x3f80_0000; // 1.0f
    c.mtc1(1, 3);
    c.g[T1][0] = 0x40;
    c.g[V1][0] = Addr(oc391_at(c, V0, 0xe84)).lw(c);
    c.g[T0][0] = 0x80;
    c.g[A3][0] = sx(sp0 + 0x70);
    c.g[A1][0] = c.g[A2][0];
    Addr(oc391_at(c, V1, 0x10)).sh(c, c.g[A0][0]);
    c.g[A0][0] = sx(verts.wrapping_add(0xc0));
    loop {
        ui_list_box_render_vertex(c, A1, T1, T0, A3);
        c.g[V1][0] = (c.g[A1][0] < c.g[A0][0]) as u64;
        if c.g[V1][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0039_a320);
        }
    }
    c.cycles += 22;
    c.lwc1(5, oc391_at(c, S0, 0x64));
    c.g[A1][0] = 4;
    c.lwc1(4, oc391_at(c, S0, 0x60));
    c.g[A3][0] = 0;
    c.lwc1(3, sp0 + 0xa0);
    c.lwc1(1, sp0 + 0xd4);
    c.lwc1(2, sp0 + 0x100);
    c.add_s(3, 3, 4);
    c.lwc1(0, sp0 + 0x104);
    c.add_s(1, 1, 5);
    c.add_s(2, 2, 4);
    c.g[T0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(UILB_RENDERER_GP))).lw(c);
    c.add_s(0, 0, 5);
    c.swc1(3, sp0 + 0xa0); // v1.x += w
    c.swc1(1, sp0 + 0xd4); // v2.y += h
    c.swc1(2, sp0 + 0x100); // v3.x += w
    c.swc1(0, sp0 + 0x104); // v3.y += h
    ui_list_box_render_draw(c, 0x238, 0x0039_a3c8)?;
    c.cycles += 1;
    Ok(())
}

/// One outline vertex at `reg vert` (0x30 bytes): q = 1 (f3), u = v = 0,
/// alpha from `alpha`, red from `red`, g = b = 0; its position (the aligned
/// box corner at sp+0x10, w = 1) goes through sp+0x20 to `pos` (sp+0x70 +
/// 0x30*i). Advances both pointers.
fn ui_list_box_render_vertex(c: &mut Cpu, vert: usize, alpha: usize, red: usize, pos: usize) {
    let sp0 = sp(c);
    c.cycles += 20;
    let v = c.g[vert][0] as u32;
    c.swc1(3, v + 8);
    Addr(v).sw(c, 0);
    Addr(v + 4).sw(c, 0);
    Addr(v + 0x1c).sw(c, c.g[alpha][0]);
    Addr(v + 0x10).sw(c, c.g[red][0]);
    Addr(v + 0x14).sw(c, 0);
    Addr(v + 0x18).sw(c, 0);
    c.swc1(3, sp0 + 0x2c);
    c.g[vert][0] = sx(v.wrapping_add(0x30));
    c.lwc1(2, sp0 + 0x10);
    c.lwc1(0, sp0 + 0x14);
    c.lwc1(1, sp0 + 0x18);
    c.swc1(2, sp0 + 0x20);
    c.swc1(0, sp0 + 0x24);
    c.swc1(1, sp0 + 0x28);
    c.lq(2, sp0 + 0x20);
    c.sq(2, c.g[pos][0] as u32);
    c.g[pos][0] = sx((c.g[pos][0] as u32).wrapping_add(0x30));
}

/// Call renderer method `slot` (`t0` = the renderer) with `a1` vertices at
/// `a2`, returning to `ret`.
fn ui_list_box_render_draw(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    let vt = Addr(oc391_at(c, T0, 0x10d8)).lw(c);
    c.g[V0][0] = vt;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(slot)).lh(c);
    c.g[V1][0] = Addr((vt as u32).wrapping_add(slot + 4)).lw(c);
    let func = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

// ---------------------------------------------------------------------------
// tWPIGD_*_Blend (0x2bd2e8, 0x2bd518)

/// The shared body of the world-painter "game data" blends (`SkyBox`,
/// `Surface`): `void Blend(tWPIGD* self, tWPIGD* other, float t)`. Calls the
/// object's vtable slot 0x218 (an update hook), then blends the value at
/// `+8` toward `other+4` by `t*t` (`v = t²·other + (1-t²)·v`), keeps
/// `other+4` at `+0xc`, and calls the hook again. The two siblings differ
/// only in their return addresses.
fn twpigd_blend(c: &mut Cpu, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[V1][0] = Addr(oc391_at(c, S0, 4)).lw(c); // vtable
        c.mul_s(20, 20, 20); // t*t
        twpigd_blend_hook(c, ret1)?;
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 0);
        c.lwc1(1, oc391_at(c, S1, 4));
        c.sub_s(0, 0, 20);
        c.lwc1(2, oc391_at(c, S0, 8));
        c.mul_s(20, 20, 1);
        c.g[V1][0] = Addr(oc391_at(c, S0, 4)).lw(c);
        c.mul_s(0, 0, 2);
        c.add_s(20, 20, 0);
        c.swc1(20, oc391_at(c, S0, 8));
        c.lwc1(0, oc391_at(c, S1, 4));
        c.swc1(0, oc391_at(c, S0, 0xc));
        twpigd_blend_hook(c, ret2)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Call vtable slot 0x218 of `s0` (vtable in `v1`), returning to `ret`.
fn twpigd_blend_hook(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = Addr(oc391_at(c, V1, 0x218)).lh(c);
    c.g[V0][0] = Addr(oc391_at(c, V1, 0x21c)).lw(c);
    let func = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

/// `void tWPIGD_Surface_Blend(tWPIGD_Surface* self, tWPIGD_Surface* other,
/// float t)` (0x2bd518): blend the snow sparkle density, see [`twpigd_blend`].
pub fn twpigd_surface_blend(c: &mut Cpu) {
    twpigd_blend(c, 0x002b_d550, 0x002b_d590)
}

/// `void tWPIGD_SkyBox_Blend(tWPIGD_SkyBox* self, tWPIGD_SkyBox* other,
/// float t)` (0x2bd2e8): blend the sky box value, see [`twpigd_blend`].
pub fn twpigd_sky_box_blend(c: &mut Cpu) {
    twpigd_blend(c, 0x002b_d320, 0x002b_d360)
}

// ---------------------------------------------------------------------------
// Small wrappers and leaves

/// `bool PS2GraphicsMan_IsIdle(PS2GraphicsMan* self)` (0x395f80): forwards
/// to `PS2GraphicsMan_IsRenderIdle` (0x382730, through the table).
pub fn ps2_graphics_man_is_idle(c: &mut Cpu) {
    crate::call::tail_wrapper(
        c,
        None,
        crate::call::Target::Table(0x0038_2730),
        0x0039_5f90,
    );
}

/// `u32 StrHashElf(u32* out, const char* s)` (0x317618): the classic ELF
/// string hash (`h = (h << 4) + c`, fold the top nibble back in), with the
/// characters sign-extended; stores it at `*out` and returns it.
pub fn str_hash_elf(c: &mut Cpu) {
    c.cycles += 4;
    let mut s = c.g[A1][0];
    c.g[V0][0] = Addr(s as u32).lb(c);
    c.g[V1][0] = 0;
    c.g[A2][0] = Addr(s as u32).lbu(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A3][0] = 0xffff_ffff_f000_0000;
        loop {
            c.cycles += 7;
            let ch = sx((((c.g[A2][0] as u32) << 24) as i32 >> 24) as u32);
            let h = sx((c.g[V1][0] as u32) << 4);
            let h = sx((h as u32).wrapping_add(ch as u32));
            c.g[V0][0] = ch;
            c.g[V1][0] = h;
            c.g[A2][0] = h & c.g[A3][0];
            s = sx((s as u32).wrapping_add(1));
            c.g[A1][0] = s;
            if c.g[A2][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = h ^ sx((c.g[A2][0] as u32) >> 23);
                c.g[V1][0] = c.g[V0][0] ^ c.g[A2][0];
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(s as u32).lb(c);
            c.g[A2][0] = Addr(s as u32).lbu(c);
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_7630);
            }
        }
    }
    c.cycles += 3;
    Addr(r32(c, A0)).sw(c, c.g[V1][0]);
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `int ReferThreadStatus(int thid, ThreadParam* info)` (0x423ca0): syscall
/// 0x30.
pub fn refer_thread_status(c: &mut Cpu) {
    crate::game::lib::kernel::syscall_wrapper(c, 0x30, 0x0042_3ca4, 0x0042_3ca8);
}

/// `int RotateThreadReadyQueue(int prio)` (0x423c50): syscall 0x2b.
pub fn rotate_thread_ready_queue(c: &mut Cpu) {
    crate::game::lib::kernel::syscall_wrapper(c, 0x2b, 0x0042_3c54, 0x0042_3c58);
}

// ---------------------------------------------------------------------------
// WScriptFn_GetObjectSlot30 (0x301680)

/// `gp+0x25e0`: the shared 8-byte id buffer (`D_1680`).
pub(crate) const WSCRIPT_GOS30_IDS: u32 = 0x25e0;
/// `gp+0x25e4`: its tag byte (copied to the stack on the first call).
pub(crate) const WSCRIPT_GOS30_TAG: u32 = 0x25e4;
/// `gp+0x25e8`: the one-time initialisation flag.
pub(crate) const WSCRIPT_GOS30_INIT: u32 = 0x25e8;
/// `gp+0xb98`: the script value type of each slot.
pub(crate) const WSCRIPT_GOS30_TYPES: u32 = 0xb98;
/// `gp+0x16c8`: the id table head; `gp+0xce8`: the world (`+0x290` is the
/// object of id -1).
pub(crate) const WSCRIPT_GOS30_TABLE: u32 = 0x16c8;
pub(crate) const WSCRIPT_GOS30_WORLD: u32 = 0xce8;

/// `cLunoVar* WScriptFn_GetObjectSlot30(cLunoVar* ret, int n, Ent* e)`
/// (0x301680): the script bridge's float query: copy the shared id buffer,
/// overlay the `n` argument entries (as `WScriptFn_ObjectExists`), look the
/// first id up and return (type 2, a float) the object's `+0xc` component's
/// vtable slot 0xf0 result, or -1 when there is no object or component.
pub fn wscript_fn_get_object_slot30(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_INIT)).lw(c);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0]; // n
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0]; // ret
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0]; // entries
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0x70);
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_TAG)).lbu(c);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            Addr(gp.wrapping_add(WSCRIPT_GOS30_IDS)).sw(c, c.g[V1][0]);
            Addr(gp.wrapping_add(WSCRIPT_GOS30_INIT)).sw(c, 1);
            Addr(sp(c)).sb(c, c.g[V0][0]);
        }
        // `buf[0] = -1; memcpy(buf, D_1680, 8)` and the entry overlay.
        'overlay: {
            c.cycles += 10;
            c.g[V0][0] = u64::MAX;
            c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A2][0] = 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(gp.wrapping_add(WSCRIPT_GOS30_IDS));
            Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x003e_6574, 0x0030_16ec)?;
            let no_entries = (c.g[S1][0] as i64) <= 0;
            c.g[A2][0] = c.g[S3][0];
            if no_entries {
                break 'overlay;
            }
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(gp.wrapping_add(WSCRIPT_GOS30_TYPES));
            c.g[A3][0] = 2; // the script type of a float slot
            loop {
                'store: {
                    'value: {
                        c.cycles += 8;
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c); // the slot
                        c.g[V1][0] = Addr(oc391_at(c, A2, 0xc)).lw(c); // type
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.g[V0][0] = Addr(r32(c, V0)).lw(c); // the slot's type
                        let raw = c.g[V1][0] == c.g[V0][0] || {
                            c.cycles += 2;
                            c.g[V0][0] != c.g[A3][0]
                        };
                        if raw {
                            c.g[V0][0] = Addr(oc391_at(c, A2, 4)).lw(c);
                            break 'value;
                        }
                        // A float value: `*(float*)d = (float)x->val`.
                        c.cycles += 4;
                        c.lwc1(0, oc391_at(c, A2, 4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r32(c, A0));
                        break 'store;
                    }
                    c.cycles += 1;
                    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
                if !more {
                    break 'overlay;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_1700);
                }
            }
        }
        // Look the first id up: -1 is the world's object, else the low byte
        // picks a descriptor and the next bits its entry.
        c.cycles += 4;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_TABLE)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp.wrapping_add(WSCRIPT_GOS30_WORLD)).lw(c);
            c.g[V0][0] = Addr(oc391_at(c, V0, 0x290)).lw(c);
        } else {
            c.cycles += 8;
            c.g[V0][0] = sx(((c.g[A2][0] & 0xff) as u32) << 2);
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(oc391_at(c, V1, 8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = sx(r32(c, A2) >> 8);
            let mut found = false;
            if c.g[V1][0] != 0 {
                c.cycles += 7;
                c.g[A0][0] = Addr(oc391_at(c, V1, 0x1c)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                c.g[V0][0] = sx(r32(c, V1) >> 8);
                found = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r32(c, V0) << 2);
            }
            if !found {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        // The value: the component's slot 0xf0, or -1.
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 2;
            c.cycles += 4;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(1, 20);
            Addr(oc391_at(c, S2, 8)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
        } else {
            c.cycles += 3;
            c.g[A1][0] = Addr(oc391_at(c, V0, 0xc)).lw(c);
            if c.g[A1][0] == 0 {
                c.cycles += 2;
                c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
                c.mtc1(1, 20);
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(oc391_at(c, A1, 0xc)).lw(c);
                c.g[A0][0] = Addr(oc391_at(c, V0, 0xf0)).lh(c);
                c.g[V1][0] = Addr(oc391_at(c, V0, 0xf4)).lw(c);
                let func = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0030_17cc)?;
                c.mov_s(20, 0);
            }
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[A0][0] = c.g[S2][0];
            Addr(oc391_at(c, S2, 8)).sw(c, 2);
        }
        c.cycles += 12;
        call_ptr(c, 0x0022_6618, 0x0030_1804)?; // LunoVMRegister_AsFloat
        c.swc1(20, r32(c, V0));
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_AllocBlendedMatrixHistory (0x3698e0)

/// `PS2GraphicsMan+0xf48`: the blended-matrix history (0x200 entries of
/// 0xc bytes: `{int count; Weight* weights; int}`, then their weights);
/// `+0xf4c`: the number of entries in use.
pub(crate) const PS2GM_BLEND_HISTORY: u32 = 0xf48;
pub(crate) const PS2GM_BLEND_COUNT: u32 = 0xf4c;

/// `PS2GraphicsMan_MatchBlendedMatrix` (0x369a78).
const PS2GM_MATCH_BLENDED_MATRIX: Callee = Callee {
    addr: 0x0036_9a78,
    f: ee::s12::PS2GraphicsMan_MatchBlendedMatrix,
};

/// `void PS2GraphicsMan_AllocBlendedMatrixHistory(PS2GraphicsMan* self,
/// BlendedMatrix* m)` (0x3698e0): allocate the history on first use (0x3000
/// bytes, each entry pointing at its 3 weight words), then look for an entry
/// that matches `m` within the tolerances (10, 20, 20): on a match copy its
/// bone ids into `m`'s weights; else append `m` (count and weights) as a new
/// entry.
pub fn ps2_graphics_man_alloc_blended_matrix_history(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
        c.g[S3][0] = c.g[A1][0];
        if c.g[V0][0] == 0 {
            c.cycles += 10;
            c.g[A1][0] = 0x0049_2638; // the allocation tag
            c.g[A0][0] = 0x3000;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0036_9920)?; // operator_new_named
            Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0;
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x1800));
            c.g[A0][0] = 0x1ff;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                Addr(oc391_at(c, V0, 4)).sw(c, c.g[V1][0]);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xc));
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9930);
                }
            }
        }
        // Look for a matching entry.
        c.cycles += 4;
        c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
        c.g[S2][0] = 0;
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
        'append: {
            if empty {
                break 'append;
            }
            c.cycles += 1;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = 0xa;
                c.g[A3][0] = 0x14;
                c.g[T0][0] = 0x14;
                PS2GM_MATCH_BLENDED_MATRIX.call(c, 0x0036_9978)?;
                if c.g[V0][0] != 0 {
                    // Found: copy the entry's bone ids into m.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    let none = (c.g[V0][0] as i64) <= 0;
                    c.g[A2][0] = 0;
                    if none {
                        c.cycles += 1;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(oc391_at(c, S0, 4)).lw(c);
                        loop {
                            c.cycles += 11;
                            c.g[V1][0] = sx(r32(c, A2) << 2);
                            c.g[A1][0] = Addr(oc391_at(c, S3, 4)).lw(c);
                            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                            c.g[A0][0] = Addr(r32(c, V0)).lhu(c);
                            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                            Addr(r32(c, V1)).sh(c, c.g[A0][0]);
                            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.g[V0][0] = Addr(oc391_at(c, S0, 4)).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0036_9990);
                            }
                        }
                        c.cycles += 2;
                    }
                    return ps2gm_blend_history_exit(c, frame);
                }
                c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
                c.cycles += 4;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    break 'append;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9960);
                }
            }
        }
        // Append m as entry nBlend.
        c.cycles += 10;
        c.g[V1][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
        c.g[V0][0] = 0xc;
        c.g[A1][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
        c.mult(0, V1, V1, V0);
        c.g[A0][0] = Addr(r32(c, S3)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        Addr(r32(c, V1)).sw(c, c.g[A0][0]);
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[T0][0] = 0xc;
            c.g[A0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
            loop {
                c.cycles += 17;
                c.g[A1][0] = sx(r32(c, A3) << 2);
                c.g[A2][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_HISTORY)).lw(c);
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.mult(0, A0, A0, T0);
                c.g[V1][0] = Addr(oc391_at(c, S3, 4)).lw(c);
                c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A2)));
                c.g[V0][0] = Addr(oc391_at(c, A0, 4)).lw(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                let src = r32(c, V1);
                let dst = r32(c, A1);
                c.lwl(2, src.wrapping_add(3));
                c.lwr(2, src);
                c.swl(2, dst.wrapping_add(3));
                c.swr(2, dst);
                c.g[V0][0] = Addr(r32(c, S3)).lw(c);
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0036_9a08);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        Addr(oc391_at(c, S1, PS2GM_BLEND_COUNT)).sw(c, c.g[V0][0]);
        c.cycles += 1;
        ps2gm_blend_history_exit(c, frame)
    })
}

/// The epilogue of [`ps2_graphics_man_alloc_blended_matrix_history`].
fn ps2gm_blend_history_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    frame.restore(c, S0, 0x40);
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// strncmp (0x416b18)

/// `0x0101010101010101`: the "subtract one from every byte" constant of the
/// NUL-byte test.
const STRNCMP_ONES: u64 = 0x0101_0101_0101_0101;
/// `0x8080808080808080`: the byte sign bits.
const STRNCMP_HIGHS: u64 = 0x8080_8080_8080_8080;
/// Resume points (loop heads with an event check) and the stages between.
const STRNCMP_ENTRY: u32 = 0x0041_6b18;
const STRNCMP_RET0: u32 = 0x0041_6b20;
const STRNCMP_LOOP16: u32 = 0x0041_6bac;
const STRNCMP_LOOP8: u32 = 0x0041_6c48;
const STRNCMP_BYTES: u32 = 0x0041_6ca8;
const STRNCMP_TAIL: u32 = 0x0041_6c94;
const STRNCMP_SETUP8: u32 = 0x0041_6c08;
const STRNCMP_TAIL_FROM_A3: u32 = 0x0041_6c88;
const STRNCMP_TAIL_SET: u32 = 0x0041_6c8c;
const STRNCMP_COMPARE: u32 = 0x0041_6cc4;
const STRNCMP_END: u32 = 0x0041_6cd0;

/// `int strncmp(const char* a, const char* b, size_t n)` (0x416b18): the
/// libc compare with the EE fast paths: both strings 16-byte aligned and
/// `n >= 16`: quadwords while equal and NUL-free (MMI `psubw`/`psubb`/`pnor`);
/// 8-byte aligned: doublewords the same way; the rest byte by byte. Returns
/// the difference of the first differing bytes (unsigned), 0 when equal up to
/// `n` or a NUL. Its loop heads are resume points (an event may unwind the
/// thread there), so it keeps the original's stages and dispatches on `pc`.
pub fn strncmp(c: &mut Cpu) {
    use crate::events::back_edge_unwinds;
    let mut pc = c.pc;
    loop {
        match pc {
            STRNCMP_ENTRY => {
                c.cycles += 2;
                c.g[V1][0] = c.g[A0][0] | c.g[A1][0];
                if c.g[A2][0] == 0 {
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = c.g[A2][0];
                if c.g[V1][0] & 7 != 0 {
                    pc = STRNCMP_TAIL; // unaligned: bytes
                    continue;
                }
                c.cycles += 12;
                c.g[A3][0] = (c.g[A2][0] < 0x10) as u64;
                c.g[T1][0] = STRNCMP_ONES;
                c.g[V0][0] = (c.g[V1][0] & 0xf) | c.g[A3][0];
                c.g[A3][0] = c.g[A0][0];
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_SETUP8;
                    continue;
                }
                // Quadwords: the first one must be equal.
                c.cycles += 17;
                c.lq(3, r32(c, A0));
                c.pcpyld(T2, T1, T1);
                c.lq(2, r32(c, A1));
                c.g[T0][0] = STRNCMP_HIGHS;
                c.psubw(V1, V1, V0);
                c.pcpyld(T1, T0, T0);
                c.pcpyud(V0, V1, A0);
                c.g[T0][0] = c.g[A1][0];
                c.g[V1][0] |= c.g[V0][0];
                let differ = c.g[V1][0] != 0;
                c.g[V0][0] = c.g[A2][0];
                if differ {
                    pc = STRNCMP_TAIL;
                    continue;
                }
                c.cycles += 1;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x10));
                pc = STRNCMP_LOOP16;
            }
            STRNCMP_RET0 => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                ret(c);
                return;
            }
            STRNCMP_LOOP16 => {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                // A NUL byte in the (equal) quadword ends the compare.
                c.cycles += 9;
                c.lq(2, r32(c, A3));
                c.pnor(V1, 0, V0);
                c.psubb(V0, V0, T2);
                c.pand(V0, V0, V1);
                c.pand(V1, V0, T1);
                c.pcpyud(V0, V1, A0);
                c.g[V0][0] |= c.g[V1][0];
                let nul = c.g[V0][0] != 0;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(0x10));
                if nul {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 4;
                c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
                c.lq(3, r32(c, A3));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(0x10));
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_TAIL_FROM_A3;
                    continue;
                }
                c.cycles += 6;
                c.lq(2, r32(c, T0));
                c.psubw(V1, V1, V0);
                c.pcpyud(V0, V1, A0);
                c.g[V0][0] |= c.g[V1][0];
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0x10));
                    if back_edge_unwinds(c, STRNCMP_LOOP16) {
                        return;
                    }
                    pc = STRNCMP_LOOP16;
                    continue;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[A3][0];
                pc = STRNCMP_TAIL_SET;
            }
            STRNCMP_SETUP8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < 8) as u64;
                c.g[T0][0] = c.g[A1][0];
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_TAIL_FROM_A3;
                    continue;
                }
                // Doublewords: the first one must be equal.
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A0)).ld(c);
                c.g[V0][0] = Addr(r32(c, A1)).ld(c);
                let differ = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = c.g[A2][0];
                if differ {
                    pc = STRNCMP_TAIL;
                    continue;
                }
                c.cycles += 9;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(8));
                c.g[T2][0] = STRNCMP_HIGHS;
                pc = STRNCMP_LOOP8;
            }
            STRNCMP_LOOP8 => {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                // `(w - 0x01..) & ~w & 0x80..`: a NUL byte in w.
                c.cycles += 7;
                c.g[V0][0] = Addr(r32(c, A3)).ld(c);
                c.g[V1][0] = !c.g[V0][0];
                c.g[V0][0] = c.g[V0][0].wrapping_sub(c.g[T1][0]);
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] &= c.g[T2][0];
                let nul = c.g[V0][0] != 0;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(8));
                if nul {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A2][0] < 8) as u64;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    pc = STRNCMP_TAIL_FROM_A3;
                    continue;
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A3)).ld(c);
                c.g[V0][0] = Addr(r32(c, T0)).ld(c);
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(8));
                    if back_edge_unwinds(c, STRNCMP_LOOP8) {
                        return;
                    }
                    pc = STRNCMP_LOOP8;
                    continue;
                }
                pc = STRNCMP_TAIL_FROM_A3;
            }
            STRNCMP_TAIL_FROM_A3 => {
                c.cycles += 1;
                c.g[A0][0] = c.g[A3][0];
                pc = STRNCMP_TAIL_SET;
            }
            STRNCMP_TAIL_SET => {
                c.cycles += 2;
                c.g[A1][0] = c.g[T0][0];
                c.g[V0][0] = c.g[A2][0];
                pc = STRNCMP_TAIL;
            }
            STRNCMP_TAIL => {
                // Byte by byte (v0 = bytes left).
                c.cycles += 2;
                let none = c.g[V0][0] == 0;
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                if none {
                    pc = STRNCMP_END;
                    continue;
                }
                c.cycles += 2;
                c.g[V1][0] = Addr(r32(c, A0)).lb(c);
                pc = STRNCMP_COMPARE;
            }
            STRNCMP_BYTES => {
                c.cycles += 2;
                if c.g[A2][0] == 0 {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 2;
                let nul = c.g[A3][0] == 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if nul {
                    if back_edge_unwinds(c, STRNCMP_RET0) {
                        return;
                    }
                    pc = STRNCMP_RET0;
                    continue;
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = Addr(r32(c, A0)).lb(c);
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                pc = STRNCMP_COMPARE;
            }
            STRNCMP_COMPARE => {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A1)).lb(c);
                let same = c.g[V1][0] == c.g[V0][0];
                c.g[A3][0] = Addr(r32(c, A0)).lbu(c);
                if same {
                    if back_edge_unwinds(c, STRNCMP_BYTES) {
                        return;
                    }
                    pc = STRNCMP_BYTES;
                    continue;
                }
                pc = STRNCMP_END;
            }
            STRNCMP_END => {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, A1)).lbu(c);
                c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, STRNCMP_ENTRY, 0x0041_6ce0);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// HUD_DrawPipMeter (0x21f338)

/// The five pip colours (`D_004C8568`..`D_004C85E8`, 0x20 apart): the colour
/// of a meter with n pips is entry n-1.
pub(crate) const HUD_PIP_COLOURS: u32 = 0x004c_8568;

/// `ldl/ldr` 16 bytes at `src` into GPRs `ra`, `rb` and `sdl/sdr` them to
/// `dst` (the original's struct copies, with its register choice).
fn hud_pip_copy16(c: &mut Cpu, ra: usize, rb: usize, src: u32, dst: u32) {
    c.ldl(ra, src.wrapping_add(7));
    c.ldr(ra, src);
    c.ldl(rb, src.wrapping_add(0xf));
    c.ldr(rb, src.wrapping_add(8));
    c.sdl(ra, dst.wrapping_add(7));
    c.sdr(ra, dst);
    c.sdl(rb, dst.wrapping_add(0xf));
    c.sdr(rb, dst.wrapping_add(8));
}

/// `void HUD_DrawPipMeter(HUD* self, HUDObj* obj, int n, int x, V2* pos,
/// V2* scale, float alpha)` (0x21f338): draw `n` pips of the element at
/// `obj+0x240` in a row 2 units apart, coloured by `n` (alpha-scaled), either
/// aligned by the element's alignment bytes (`pos == 0`) or centred on `pos`.
pub fn hud_draw_pip_meter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A2][0]; // n
        frame.save(c, S3, 0x110);
        c.g[S2][0] = c.g[T1][0]; // scale
        frame.save(c, S4, 0x100);
        c.g[S3][0] = c.g[A0][0]; // self
        frame.save(c, S6, 0xe0);
        c.g[S4][0] = c.g[A1][0]; // obj
        frame.save(c, FP, 0xc0);
        c.g[S6][0] = c.g[T0][0]; // pos
        frame.save_fpr(c, 21, 0x158);
        c.g[FP][0] = c.g[A3][0]; // x
        frame.save(c, S0, 0x140);
        c.mov_s(21, 12); // alpha
        frame.save(c, S5, 0xf0);
        frame.save(c, S7, 0xd0);
        frame.save_ra(c, 0xb0);
        frame.save_fpr(c, 20, 0x150);
        if (c.g[S1][0] as i64) > 0 {
            hud_draw_pip_meter_body(c)?;
        }
        c.cycles += 14;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 21, 0x158);
        frame.restore_fpr(c, 20, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// [`hud_draw_pip_meter`] for `n > 0`.
fn hud_draw_pip_meter_body(c: &mut Cpu) -> crate::call::Flow {
    let sp0 = sp(c);
    // The colour table onto the stack (sp+0..0x50).
    c.cycles += 65;
    c.g[V0][0] = 0x004d_0000;
    c.g[V1][0] = 0x004d_0000;
    c.g[A3][0] = sx(HUD_PIP_COLOURS);
    hud_pip_copy16(c, A0, A1, HUD_PIP_COLOURS, sp0);
    c.g[S5][0] = sx(sp0 + 0x50);
    c.g[A2][0] = sx(HUD_PIP_COLOURS + 0x20);
    hud_pip_copy16(c, T0, V0, HUD_PIP_COLOURS + 0x20, sp0 + 0x10);
    c.g[A0][0] = 0x004d_0000;
    c.g[V1][0] = sx(HUD_PIP_COLOURS + 0x40);
    hud_pip_copy16(c, A3, T0, HUD_PIP_COLOURS + 0x40, sp0 + 0x20);
    c.g[A1][0] = 0x004d_0000;
    c.g[T0][0] = sx(HUD_PIP_COLOURS + 0x60);
    hud_pip_copy16(c, A0, A2, HUD_PIP_COLOURS + 0x60, sp0 + 0x30);
    c.g[S0][0] = sx(r32(c, S4).wrapping_add(0x240)); // the element
    c.g[V0][0] = 0x004d_0000;
    c.g[A0][0] = c.g[S0][0];
    c.g[T0][0] = sx(HUD_PIP_COLOURS + 0x80);
    hud_pip_copy16(c, V1, A2, HUD_PIP_COLOURS + 0x80, sp0 + 0x40);
    c.g[A1][0] = c.g[S5][0];
    call_ptr(c, 0x001e_91f8, 0x0021_f468)?; // HUD_GetElemPos(elem, &a)
    c.g[A2][0] = Addr(oc391_at(c, S3, 0x4a0)).lw(c);
    c.g[A1][0] = sx(sp0 + 0x70);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001e_9220, 0x0021_f478)?; // HUD_GetElemSize(elem, &size, tex)
    c.g[A1][0] = sx(sp0 + 0x80);
    c.g[A0][0] = c.g[S0][0];
    c.g[S7][0] = c.g[A1][0];
    call_ptr(c, 0x001e_9290, 0x0021_f488)?; // HUD_GetElemScale(elem, &sc)
    c.lwc1(1, sp0 + 0x80);
    if c.g[S2][0] != 0 {
        c.cycles += 7;
        c.lwc1(3, oc391_at(c, S2, 4));
        c.lwc1(2, r32(c, S2));
        c.lwc1(0, sp0 + 0x84);
        c.mul_s(1, 1, 2);
        c.mul_s(0, 0, 3);
        c.swc1(1, sp0 + 0x80);
        c.swc1(0, sp0 + 0x84);
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(1));
    if c.g[S6][0] == 0 {
        // dim = (size.x*n + (n-1)*2, size.y), aligned by the element.
        c.cycles += 22;
        c.mtc1(S1, 2);
        c.cvt_s_w(2, 2);
        c.mtc1(V0, 1);
        c.cvt_s_w(1, 1);
        c.lwc1(0, sp0 + 0x70);
        c.lwc1(3, sp0 + 0x74);
        c.g[A2][0] = c.g[S5][0];
        c.mul_s(0, 0, 2);
        c.g[T1][0] = Addr(oc391_at(c, S4, 0x260)).lb(c);
        c.add_s(1, 1, 1);
        c.g[T2][0] = Addr(oc391_at(c, S4, 0x261)).lb(c);
        c.swc1(3, sp0 + 0x94);
        c.g[T3][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(sp0 + 0x60);
        c.add_s(0, 0, 1);
        c.g[A3][0] = sx(sp0 + 0x90);
        c.g[T0][0] = c.g[S7][0];
        c.swc1(0, sp0 + 0x90);
        call_ptr(c, 0x001f_10f8, 0x0021_f504)?; // HUD_AlignElemPos
    } else {
        // p = *pos - (dim * sc) / 2.
        c.cycles += 28;
        c.ldl(3, oc391_at(c, S6, 7));
        c.ldr(3, r32(c, S6));
        c.sdl(3, sp0 + 0x67);
        c.sdr(3, sp0 + 0x60);
        c.mtc1(S1, 2);
        c.cvt_s_w(2, 2);
        c.g[AT][0] = 0x3f00_0000; // 0.5f
        c.mtc1(AT, 6);
        c.mtc1(V0, 4);
        c.cvt_s_w(4, 4);
        c.lwc1(0, sp0 + 0x70);
        c.lwc1(7, sp0 + 0x80);
        c.mul_s(0, 0, 2);
        c.lwc1(1, sp0 + 0x74);
        c.add_s(4, 4, 4);
        c.lwc1(3, sp0 + 0x84);
        c.lwc1(5, sp0 + 0x60);
        c.mul_s(1, 1, 3);
        c.lwc1(2, sp0 + 0x64);
        c.add_s(0, 0, 4);
        c.mul_s(1, 1, 6);
        c.mul_s(0, 0, 7);
        c.sub_s(2, 2, 1);
        c.mul_s(0, 0, 6);
        c.swc1(2, sp0 + 0x64);
        c.sub_s(5, 5, 0);
        c.swc1(5, sp0 + 0x60);
    }
    c.g[V0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(0x854))).lw(c);
    // The draw state's layer (obj+0x262), step, colour.
    c.cycles += 29;
    c.g[A2][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V1][0] = Addr(oc391_at(c, S4, 0x262)).lb(c);
    c.g[A0][0] = sx(r32(c, S1) << 4);
    c.g[A1][0] = Addr(oc391_at(c, V0, 0xe84)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(sp0));
    c.g[V1][0] = sx(r32(c, V1) << 5);
    c.g[AT][0] = 0x4000_0000; // 2.0f
    c.mtc1(AT, 3);
    c.g[V0][0] = Addr(oc391_at(c, A1, 8)).lw(c);
    c.g[V1][0] &= 0x3e0;
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] |= c.g[V1][0];
    Addr(oc391_at(c, A1, 8)).sw(c, c.g[V0][0]);
    c.lwc1(1, sp0 + 0x70);
    c.lwc1(2, sp0 + 0x80);
    let colour = r32(c, A0).wrapping_sub(0x10);
    hud_pip_copy16(c, A3, T0, colour, sp0 + 0xa0);
    c.add_s(1, 1, 3);
    c.lwc1(0, sp0 + 0xa0);
    c.mul_s(20, 1, 2); // step = (size.x + 2) * sc.x
    c.mul_s(0, 0, 21);
    let none = (c.g[S1][0] as i64) <= 0;
    c.swc1(0, sp0 + 0xa0);
    if none {
        return Ok(());
    }
    c.cycles += 2;
    c.g[S0][0] = c.g[S1][0];
    loop {
        c.cycles += 12;
        c.g[A1][0] = Addr(oc391_at(c, S3, 0x4a0)).lw(c);
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = sx(sp0 + 0x60);
        c.g[A3][0] = sx(sp0 + 0x70);
        c.g[T0][0] = c.g[S7][0];
        c.g[T1][0] = sx(sp0 + 0xa0);
        call_ptr(c, 0x001f_1190, 0x0021_f614)?; // HUD_DrawSprite
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
        c.lwc1(0, sp0 + 0x60);
        c.add_s(0, 0, 20);
        let more = c.g[S0][0] != 0;
        c.swc1(0, sp0 + 0x60);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0021_f5f8);
        }
    }
}

// ---------------------------------------------------------------------------
// BoltFile_CacheUpdate (0x19d250)

/// The bolt file cache: 10 tables of 256 slots (0x120 bytes each, a table
/// every 0x12000 bytes); `+0xb4004` the number of pending loads.
pub(crate) const BOLT_CACHE_TABLES: u32 = 10;
#[cfg(test)]
pub(crate) const BOLT_CACHE_TABLE_SIZE: u32 = 0x12000;
pub(crate) const BOLT_CACHE_SLOT_SIZE: u32 = 0x120;
#[cfg(test)]
pub(crate) const BOLT_CACHE_PENDING: u32 = 0xb_4004;
/// Slot fields: state (2 = loading, 3 = queued), the async file handle,
/// the pending flag and the keep timer.
pub(crate) const BOLT_SLOT_STATE: u32 = 0x0;
pub(crate) const BOLT_SLOT_HANDLE: u32 = 0x108;
pub(crate) const BOLT_SLOT_PENDING: u32 = 0x10c;
pub(crate) const BOLT_SLOT_TIMER: u32 = 0x114;

const BOLT_UNPACK_SLOT: Callee = Callee {
    addr: 0x0019_d000,
    f: ee::s03::BoltFile_UnpackSlot,
};
const BOLT_START_NEXT_QUEUED: Callee = Callee {
    addr: 0x0019_d140,
    f: ee::s03::BoltFile_StartNextQueued,
};
const BOLT_LOAD_SLOT: Callee = Callee {
    addr: 0x0019_d8b8,
    f: ee::s03::BoltFile_LoadSlot,
};

/// `void BoltFile_CacheUpdate(BoltFileCache* self)` (0x19d250): poll every
/// slot of every table: a pending load whose async read finished
/// (`AsyncFile_IsDone == 1`) is released and unpacked, its state becomes 1
/// (or 0 without a keep timer) and, when it was the last pending load, the
/// next queued one starts; with no load pending, a queued slot (state 3)
/// starts loading.
pub fn bolt_file_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0xb_0000;
        frame.save(c, S2, 0x80);
        frame.save(c, S7, 0x30);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x70);
        c.g[S7][0] = 0x1_0000;
        frame.save(c, S4, 0x60);
        c.g[S3][0] = 0; // table
        frame.save(c, FP, 0x20);
        c.g[S4][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
        frame.save(c, S0, 0xa0);
        c.g[S7][0] |= 0x2000; // BOLT_CACHE_TABLE_SIZE
        frame.save(c, S1, 0x90);
        c.g[FP][0] = 3; // queued
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x10);
        c.mult(0, V0, S3, S7);
        loop {
            c.cycles += 4;
            c.g[S5][0] = 0; // slot
            c.g[S6][0] = sx(r32(c, S3).wrapping_add(1));
            c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
            loop {
                bolt_file_cache_update_slot(c)?;
                // Next slot.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S5][0] as i64) < 256) as u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(BOLT_CACHE_SLOT_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_d2b0);
                }
            }
            // Next table.
            c.cycles += 4;
            c.g[S3][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[S3][0] as i64) < BOLT_CACHE_TABLES as i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.mult(0, V0, S3, S7);
            if event_due(c) {
                deliver_event_here(c, 0x0019_d2a0);
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// One slot of [`bolt_file_cache_update`] (s0 = the slot, s5 = its index,
/// incremented here).
fn bolt_file_cache_update_slot(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_PENDING)).lw(c);
    c.g[V1][0] = 2;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_STATE)).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            // Loading: has the read finished?
            c.cycles += 6;
            c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_HANDLE)).lw(c);
            call_ptr(c, 0x003d_f980, 0x0019_d2d0)?; // AsyncFile_IsDone
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            if c.g[S1][0] != c.g[V0][0] {
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                return Ok(());
            }
            c.cycles += 11;
            c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_HANDLE)).lw(c);
            c.g[A2][0] = c.g[SP][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x003d_f9d8, 0x0019_d2f0)?; // AsyncFile release
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S0][0];
            BOLT_UNPACK_SLOT.call(c, 0x0019_d300)?;
            c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_TIMER)).lw(c);
            if c.g[V0][0] != 0 {
                Addr(r32(c, S0)).sw(c, c.g[S1][0]);
            } else {
                c.cycles += 1;
                Addr(r32(c, S0)).sw(c, 0);
            }
            c.cycles += 5;
            Addr(oc391_at(c, S0, BOLT_SLOT_PENDING)).sw(c, 0);
            c.g[V0][0] = Addr(oc391_at(c, S4, 0x4004)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let more = (c.g[V0][0] as i64) > 0;
            Addr(oc391_at(c, S4, 0x4004)).sw(c, c.g[V0][0]);
            if more {
                c.cycles += 1;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                BOLT_START_NEXT_QUEUED.call(c, 0x0019_d330)?;
            }
            c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
            return Ok(());
        }
        c.g[V0][0] = Addr(oc391_at(c, S4, 0x4004)).lw(c);
    } else {
        c.cycles += 1;
        c.g[V0][0] = Addr(oc391_at(c, S4, 0x4004)).lw(c);
    }
    // Nothing pending anywhere: start a queued slot.
    c.cycles += 2;
    if (c.g[V0][0] as i64) <= 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_STATE)).lw(c);
        if c.g[V0][0] == c.g[FP][0] {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            BOLT_LOAD_SLOT.call(c, 0x0019_d360)?;
            c.cycles += 1;
        }
    }
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
    Ok(())
}

// ---------------------------------------------------------------------------
// LocStringTable_GetStringPtr (0x3dbb68)

const LOC_STRING_TABLE_REMAP_ID: Callee = Callee {
    addr: 0x003d_bae8,
    f: ee::s14::LocStringTable_RemapId,
};

/// Read the unaligned little-endian word at `base` byte by byte as the
/// original does: byte 0 into `r0`, byte 1 (<< 8) into `r1`, byte 2 (<< 16)
/// into `r2`, and the word into `out` (which held byte 3 << 24).
fn loc_string_table_rd32(c: &mut Cpu, base: u32, r0: usize, r1: usize, r2: usize, out: usize) {
    c.g[r0][0] = Addr(base).lbu(c);
    c.g[out][0] = Addr(base.wrapping_add(3)).lbu(c);
    c.g[r2][0] = Addr(base.wrapping_add(2)).lbu(c);
    c.g[r1][0] = Addr(base.wrapping_add(1)).lbu(c);
    c.g[out][0] = sx(r32(c, out) << 24);
    c.g[r2][0] = sx(r32(c, r2) << 16);
    c.g[r1][0] = sx(r32(c, r1) << 8);
    c.g[out][0] |= c.g[r2][0] | c.g[r1][0] | c.g[r0][0];
}

/// `const u8* LocStringTable_GetStringPtr(LocStringTable* self, int id)`
/// (0x3dbb68): with the remap flag (`+8` bit 0) map `id` through the remap
/// table first; then, in the current language's string table (offset
/// `+0x10 + 4 * lang`, `lang` at `+0xe`), return the string `id` (offsets
/// relative to the table), or null when `id` is negative or out of range.
/// All fields are unaligned little-endian words.
pub fn loc_string_table_get_string_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(8)).lbu(c) & 1;
        c.g[T0][0] = c.g[A1][0]; // id
        if c.g[V0][0] != 0 {
            c.cycles += 15;
            c.g[V1][0] = sx(this.wrapping_add(4));
            c.g[A0][0] = c.g[T0][0] & 0xffff;
            loc_string_table_rd32(c, this.wrapping_add(4), A3, A2, V0, A1);
            c.g[A1][0] = sx(this.wrapping_add(r32(c, A1)));
            LOC_STRING_TABLE_REMAP_ID.call(c, 0x003d_bbc0)?;
            c.g[T0][0] = c.g[V0][0];
        }
        // The language's table.
        c.cycles += 19;
        c.g[V0][0] = Addr(this.wrapping_add(0xf)).lbu(c);
        c.g[V1][0] = Addr(this.wrapping_add(0xe)).lbu(c);
        c.g[V0][0] = sx(r32(c, V0) << 8) | c.g[V1][0];
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(this).wrapping_add(0x10));
        let entry = r32(c, V0);
        loc_string_table_rd32(c, entry, A2, A1, A0, V1);
        c.g[A3][0] = sx(this.wrapping_add(r32(c, V1)));
        let mut found = false;
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 14;
            let table = r32(c, A3);
            c.g[V1][0] = sx(table.wrapping_add(0xc));
            loc_string_table_rd32(c, table.wrapping_add(0xc), A2, A1, A0, V0);
            let in_range = c.g[T0][0] < c.g[V0][0];
            c.g[V0][0] = sx(r32(c, T0) << 2);
            if in_range {
                c.cycles += 14;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(table).wrapping_add(0x10));
                let entry = r32(c, V0);
                loc_string_table_rd32(c, entry, A2, A1, A0, V1);
                c.g[A3][0] = sx(table.wrapping_add(r32(c, V1)));
                found = true;
            }
        }
        if !found {
            c.cycles += 1;
            c.g[A3][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[A3][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// IniParser_SkipWhitespace (0x2a78f8)

const INI_PARSER_SKIP_COMMENT: Callee = Callee {
    addr: 0x002a_79c8,
    f: ee::s08::IniParser_SkipComment,
};
const INI_PARSER_GET_CHAR: Callee = Callee {
    addr: 0x002a_78e0,
    f: ee::s08::IniParser_GetChar,
};
/// `IniParser+8`: the read cursor (a `const char*`).
pub(crate) const INI_PARSER_CURSOR: u32 = 0x8;

/// `int IniParser_SkipWhitespace(IniParser* self)` (0x2a78f8): skip
/// comments, spaces, tabs, newlines and carriage returns. Returns 1 at the
/// end of the text, when a comment ran to the end (`SkipComment == 1`) or at
/// a section header (`[`), else 0 (another character).
pub fn ini_parser_skip_whitespace(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(oc391_at(c, S0, INI_PARSER_CURSOR)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).lb(c);
        c.g[S1][0] = 1;
        'done: {
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                c.g[S5][0] = b' ' as u64;
                c.g[S4][0] = b'\t' as u64;
                c.g[S3][0] = b'\n' as u64;
                c.g[S2][0] = b'\r' as u64;
                loop {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    INI_PARSER_SKIP_COMMENT.call(c, 0x002a_7948)?;
                    let at_end = c.g[V0][0] == c.g[S1][0];
                    c.g[V0][0] = 1;
                    if at_end {
                        break 'done;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(oc391_at(c, S0, INI_PARSER_CURSOR)).lw(c);
                    c.g[V1][0] = Addr(r32(c, V0)).lb(c);
                    let ch = c.g[V1][0];
                    let space = ch == c.g[S5][0]
                        || {
                            c.cycles += 2;
                            ch == c.g[S4][0]
                        }
                        || {
                            c.cycles += 2;
                            ch == c.g[S3][0]
                        }
                        || {
                            c.cycles += 2;
                            c.g[V1][0] = ch ^ 0x5b;
                            ch == c.g[S2][0]
                        };
                    if !space {
                        // `[` starts a section: 1; anything else: 0.
                        c.cycles += 3;
                        c.g[V0][0] = if c.g[V1][0] == 0 { c.g[S1][0] } else { 0 };
                        break 'done;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    INI_PARSER_GET_CHAR.call(c, 0x002a_798c)?;
                    c.g[V1][0] = Addr(oc391_at(c, S0, INI_PARSER_CURSOR)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002a_7940);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BoltFile_FreeSlot (0x19d4b8)

/// Slot fields freed by [`bolt_file_free_slot`]: the data block, the name's
/// first byte, the cache index (-1 = none) and the data size.
pub(crate) const BOLT_SLOT_DATA: u32 = 0x104;
pub(crate) const BOLT_SLOT_NAME: u32 = 0x4;
pub(crate) const BOLT_SLOT_INDEX: u32 = 0x110;
pub(crate) const BOLT_SLOT_SIZE: u32 = 0x118;
/// `BoltFileCache+0xb4000`: the bytes in use.
#[cfg(test)]
pub(crate) const BOLT_CACHE_BYTES: u32 = 0xb_4000;

/// `void BoltFile_FreeSlot(BoltFileCache* self, int table, int slot)`
/// (0x19d4b8): cancel a slot's pending read (state 2), take its size off
/// the cache's byte count, free its data and clear it.
pub fn bolt_file_free_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V0][0] = BOLT_CACHE_SLOT_SIZE as u64;
        c.g[V1][0] = 0x1_0000;
        c.mult(0, A2, A2, V0); // slot * 0x120
        c.g[V1][0] |= 0x2000;
        c.mult(0, V0, A1, V1); // table * 0x12000
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 2;
        c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.g[S0][0] = sx(r32(c, A1).wrapping_add(r32(c, A2)));
        c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_STATE)).lw(c);
        let loading = c.g[V0][0] == c.g[A0][0];
        c.g[V0][0] = 0xb_0000;
        if loading {
            c.cycles += 3;
            c.g[V0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_PENDING)).lw(c);
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_HANDLE)).lw(c);
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x003d_f9d8, 0x0019_d510)?; // AsyncFile release
            }
            c.cycles += 1;
            c.g[V0][0] = 0xb_0000;
        }
        c.cycles += 8;
        c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_SIZE)).lw(c);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = Addr(oc391_at(c, V0, 0x4000)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)));
        Addr(oc391_at(c, V0, 0x4000)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(oc391_at(c, S0, BOLT_SLOT_DATA)).lw(c);
        c.g[V0][0] = u64::MAX;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0019_d53c)?; // EA_MemFree
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 13;
        let slot = r32(c, S0);
        Addr(slot + BOLT_SLOT_DATA).sw(c, 0);
        Addr(slot + BOLT_SLOT_INDEX).sw(c, c.g[V0][0]);
        Addr(slot + BOLT_SLOT_SIZE).sw(c, 0);
        Addr(slot + BOLT_SLOT_STATE).sw(c, 0);
        Addr(slot + BOLT_SLOT_NAME).sb(c, 0);
        Addr(slot + BOLT_SLOT_HANDLE).sw(c, 0);
        Addr(slot + BOLT_SLOT_PENDING).sw(c, 0);
        Addr(slot + BOLT_SLOT_TIMER).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderRailslideState_CollideWalls (0x13c140)

/// The rail-slide state: `+0x30` the rail position the rider is held at,
/// `+0x40` the time the rider has been pushed away from it, `+0x50` the
/// rider.
pub(crate) const RRS_RAIL_POS: u32 = 0x30;
pub(crate) const RRS_OFF_RAIL_TIME: u32 = 0x40;
pub(crate) const RRS_RIDER: u32 = 0x50;
/// Rider fields: position, rail direction (`+0x1b0`), velocity, forward.
pub(crate) const RRS_R_POS: u32 = 0x110;
pub(crate) const RRS_R_RAIL_DIR: u32 = 0x1b0;
pub(crate) const RRS_R_VEL: u32 = 0x1e0;
pub(crate) const RRS_R_FORWARD: u32 = 0x370;
/// `.lit4` constants (gp-relative, subtracted): the head-on threshold, the
/// push-out factor, the off-rail time step, the along-rail threshold, the
/// along-rail factor and the minimal bounce.
pub(crate) const RRS_GP_HEAD_ON: u32 = 0x7110;
pub(crate) const RRS_GP_PUSH_OUT: u32 = 0x710c;
pub(crate) const RRS_GP_TIME_STEP: u32 = 0x7108;
pub(crate) const RRS_GP_ALONG_MIN: u32 = 0x7104;
pub(crate) const RRS_GP_ALONG_SCALE: u32 = 0x7100;
pub(crate) const RRS_GP_MIN_BOUNCE: u32 = 0x70fc;

/// `void RiderRailslideState_CollideWalls(RiderRailslideState* self)`
/// (0x13c140): the rail slide's wall collision. Skipped in sub-state 9.
/// The rider's hull is swept against its overlap list; on a hit (depth >=
/// 0) that is not head-on, the normal is flattened against the forward
/// direction when the rider faces the wall, the rider is pushed out by
/// `k * depth` along it and its matrix rebuilt, and it is pulled back to
/// the rail position: within 2 units of it the off-rail timer resets, else
/// the timer grows and after 0.5 s the velocity is turned along the rail
/// (kept when it already runs along it, else rebuilt from its length).
/// Then a rider still moving into the wall is bounced off (by half its
/// into-wall speed, at least the minimum) and the impact handled. The hull
/// is destroyed on every path after its construction.
pub fn rider_railslide_state_collide_walls(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x470);
        frame.save(c, S2, 0x430);
        frame.save(c, S0, 0x450);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x440);
        frame.save(c, S3, 0x420);
        frame.save_ra(c, 0x410);
        frame.save_fpr(c, 20, 0x460);
        c.g[A0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
        call_ptr(c, 0x0011_fee8, 0x0013_c168)?; // Rider_GetSubState
        c.g[V1][0] = 9;
        let skip = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = sx(frame.at(c, 0x80)); // the hull
        if skip {
            c.cycles += 1;
        } else {
            let bounced = rrs_collide(c, frame)?;
            if bounced {
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x450);
        c.cycles += 7;
        frame.restore(c, S1, 0x440);
        frame.restore(c, S2, 0x430);
        frame.restore(c, S3, 0x420);
        frame.restore_ra(c, 0x410);
        frame.restore_fpr(c, 20, 0x460);
        frame.close(c);
        Ok(())
    })
}

/// The hull sweep and response of [`rider_railslide_state_collide_walls`];
/// true when it went through the bounce (its exit costs one more cycle).
fn rrs_collide(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
    use crate::game::ai::control::handplantcontrol::{
        vu_dot3, vu_normalize3, vu_scale, vu_spill, vu_sub,
    };
    use ssx3_ee_rt::vu0::Src;
    let gp = c.g[GP][0] as u32;
    // CollHull_ctor(&hull, rider->hull, !rider->controller->vt[0x40]())
    c.cycles += 24;
    c.g[S0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    c.mtc1(0, 20); // f20 = 0
    c.g[S3][0] = c.g[S1][0];
    c.g[V1][0] = Addr(oc391_at(c, S0, 0x6c0)).lw(c);
    c.g[A1][0] = sx(oc391_at(c, S0, 0x6c0));
    c.g[A0][0] = Addr(oc391_at(c, V1, 0x40)).lh(c);
    c.g[V0][0] = Addr(oc391_at(c, V1, 0x44)).lw(c);
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0013_c198)?;
    c.g[A1][0] = Addr(oc391_at(c, S0, 0xaa0)).lw(c);
    c.g[A2][0] = c.g[V0][0] ^ 1;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0032_f650, 0x0013_c1a8)?; // CollHull_ctor
                                            // depth = OverlapList_FindClosestContact(rider->list, &hull, &hit, hint)
    c.g[V1][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = Addr(oc391_at(c, V1, 0x868)).lw(c);
    c.g[A0][0] = Addr(oc391_at(c, V1, 0x860)).lw(c);
    call_ptr(c, 0x0033_42d0, 0x0013_c1c0)?;
    c.mov_s(2, 0); // depth
    c.c_lt_s(2, 20);
    if c.fcc() {
        c.g[A0][0] = c.g[S3][0];
        return rrs_destroy_hull(c).map(|_| false);
    }
    // dot = forward . normal; head-on: give up.
    c.cycles += 15;
    c.g[V1][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    let rider = r32(c, V1);
    c.lwc1(0, gp.wrapping_sub(RRS_GP_HEAD_ON));
    c.lqc2(3, rider.wrapping_add(RRS_R_FORWARD));
    c.lqc2(5, frame.at(c, 0x10));
    vu_dot3(c, 5);
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.c_lt_s(1, 0);
    if c.fcc() {
        c.g[A0][0] = c.g[S3][0];
        return rrs_destroy_hull(c).map(|_| false);
    }
    c.cycles += 3;
    c.c_lt_s(1, 20);
    if c.fcc() {
        // Facing the wall: normal = normalize(normal - forward * dot).
        c.cycles += 26;
        c.mfc1(2, 1);
        vu_scale(c, 2, rider.wrapping_add(RRS_R_FORWARD));
        vu_spill(c, frame, 0x3d0, &[0x3c0]);
        vu_sub(c, frame.at(c, 0x10), frame.at(c, 0x3c0));
        vu_spill(c, frame, 0x3d0, &[0x3b0]);
        c.lqc2(3, frame.at(c, 0x3b0));
        vu_normalize3(c);
        vu_spill(c, frame, 0x3d0, &[0x10, 0x3a0]);
    }
    // Rider_Translate(rider, normal * k * depth); Rider_BuildWorldMatrix.
    c.cycles += 45;
    c.lwc1(0, gp.wrapping_sub(RRS_GP_PUSH_OUT));
    c.g[A1][0] = sx(frame.at(c, 0x3a0));
    c.mfc1(3, 0);
    vu_scale(c, 3, frame.at(c, 0x10));
    vu_spill(c, frame, 0x3c0, &[0x3b0]);
    c.mfc1(3, 2);
    vu_scale(c, 3, frame.at(c, 0x3b0));
    let t = frame.at(c, 0x3c0);
    c.sqc2(5, t);
    c.lq(2, t);
    c.g[A0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    let a = frame.at(c, 0x3a0);
    c.sq(2, a);
    call_ptr(c, 0x0010_6538, 0x0013_c2cc)?; // Rider_Translate
    c.g[A0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    call_ptr(c, 0x0011_e098, 0x0013_c2d4)?; // Rider_BuildWorldMatrix
                                            // Back to the rail: how far did the push move the rider?
    c.g[V0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    vu_sub(c, oc391_at(c, S2, RRS_RAIL_POS), oc391_at(c, V0, RRS_R_POS));
    let t = frame.at(c, 0x3c0);
    c.sqc2(5, t);
    c.g[AT][0] = 0x4000_0000; // 2.0f
    c.mtc1(1, 1);
    c.lq(2, t);
    c.g[A0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    let a = frame.at(c, 0x3b0);
    c.sq(2, a);
    c.lqc2(3, frame.at(c, 0x3b0));
    vu_dot3(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0); // distance
    c.lq(2, oc391_at(c, A0, RRS_R_POS));
    c.c_lt_s(1, 0);
    let far = c.fcc();
    c.sq(2, oc391_at(c, S2, RRS_RAIL_POS));
    if far {
        c.cycles += 2;
        c.swc1(20, oc391_at(c, S2, RRS_OFF_RAIL_TIME)); // timer = 0
    } else {
        c.cycles += 9;
        c.lwc1(0, oc391_at(c, S2, RRS_OFF_RAIL_TIME));
        c.lwc1(1, gp.wrapping_sub(RRS_GP_TIME_STEP));
        c.g[AT][0] = 0x3f00_0000; // 0.5f
        c.mtc1(1, 2);
        c.add_s(0, 0, 1);
        c.c_lt_s(2, 0);
        let expired = c.fcc();
        c.swc1(0, oc391_at(c, S2, RRS_OFF_RAIL_TIME));
        if expired {
            // Turn the velocity along the rail direction.
            c.cycles += 16;
            let rider = r32(c, A0);
            c.lq(2, rider.wrapping_add(RRS_R_RAIL_DIR));
            c.lwc1(0, gp.wrapping_sub(RRS_GP_ALONG_MIN));
            let a = frame.at(c, 0x3c0);
            c.sq(2, a);
            c.lqc2(3, frame.at(c, 0x3c0));
            c.lqc2(5, frame.at(c, 0x10));
            vu_dot3(c, 5);
            c.qmfc2(3, 4);
            c.mtc1(3, 1);
            c.lwc1(2, gp.wrapping_sub(RRS_GP_ALONG_SCALE));
            c.c_lt_s(1, 0);
            if c.fcc() {
                c.cycles += 9;
                c.mfc1(3, 2);
                vu_scale(c, 3, frame.at(c, 0x3c0));
                vu_spill(c, frame, 0x3e0, &[0x3d0]);
                c.sq(2, rider.wrapping_add(RRS_R_VEL));
            } else {
                c.cycles += 34;
                c.mfc1(3, 1);
                vu_scale(c, 3, frame.at(c, 0x10));
                vu_spill(c, frame, 0x3f0, &[0x3e0]);
                vu_sub(c, frame.at(c, 0x3c0), frame.at(c, 0x3e0));
                let t = frame.at(c, 0x3f0);
                c.sqc2(5, t);
                c.lq(2, t);
                c.g[V1][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
                let a = frame.at(c, 0x3d0);
                c.sq(2, a);
                c.lqc2(3, frame.at(c, 0x3d0));
                vu_normalize3(c);
                vu_spill(c, frame, 0x400, &[0x3f0]);
                c.mfc1(4, 2);
                vu_scale(c, 4, frame.at(c, 0x3f0));
                vu_spill(c, frame, 0x400, &[0x3e0]);
                c.sq(2, oc391_at(c, V1, RRS_R_VEL));
            }
        }
    }
    // Still moving into the wall? (vel . -normal >= 0)
    c.cycles += 23;
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
    c.mtc1(1, 0);
    c.mfc1(2, 0);
    vu_scale(c, 2, frame.at(c, 0x10));
    let t = frame.at(c, 0x3d0);
    c.sqc2(5, t);
    c.mtc1(0, 1); // f1 = 0
    c.lq(2, t);
    c.g[A0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    let a = frame.at(c, 0x3c0);
    c.sq(2, a);
    c.lqc2(3, frame.at(c, 0x3c0));
    c.lqc2(5, oc391_at(c, A0, RRS_R_VEL));
    vu_dot3(c, 5);
    c.qmfc2(3, 4);
    c.mtc1(3, 2); // into
    c.c_lt_s(2, 1);
    if c.fcc() {
        c.cycles += 1;
        c.g[A0][0] = c.g[S3][0];
        return rrs_destroy_hull(c).map(|_| false);
    }
    // Bounce: push = max(into * 0.5, min); vel += normal * (into + push).
    c.cycles += 20;
    c.g[AT][0] = 0x3f00_0000; // 0.5f
    c.mtc1(1, 0);
    c.lqc2(3, oc391_at(c, A0, RRS_R_VEL));
    vu_normalize3(c);
    let t = frame.at(c, 0x3d0);
    c.sqc2(5, t);
    c.lwc1(3, gp.wrapping_sub(RRS_GP_MIN_BOUNCE));
    c.mul_s(0, 2, 0);
    c.lq(2, t);
    let a = frame.at(c, 0x3c0);
    c.sq(2, a);
    c.c_lt_s(0, 3);
    let small = c.fcc();
    c.mov_s(1, 0);
    if small {
        c.cycles += 1;
        c.mov_s(1, 3);
    }
    c.cycles += 27;
    c.add_s(0, 2, 1);
    c.g[A1][0] = sx(frame.at(c, 0x3d0));
    c.mfc1(3, 0);
    vu_scale(c, 3, frame.at(c, 0x10));
    let t = frame.at(c, 0x3e0);
    c.sqc2(5, t);
    c.lq(2, t);
    c.g[V1][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    let a = frame.at(c, 0x3d0);
    c.sq(2, a);
    let vel = oc391_at(c, V1, RRS_R_VEL);
    c.lqc2(3, vel);
    c.lqc2(4, frame.at(c, 0x3d0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, vel);
    // Rider_HandleImpact(rider, &{ hit.pos, dir, hit.normal, push },
    // hit.material)
    let sp0 = sp(c);
    c.lq(2, sp0);
    c.lq(3, sp0 + 0x3c0);
    c.lq(7, sp0 + 0x10);
    c.g[A0][0] = Addr(oc391_at(c, S2, RRS_RIDER)).lw(c);
    c.g[A2][0] = Addr(sp0 + 0x4c).lw(c);
    c.sq(2, sp0 + 0x3d0);
    c.sq(3, sp0 + 0x3e0);
    c.sq(7, sp0 + 0x3f0);
    c.swc1(1, sp0 + 0x400);
    call_ptr(c, 0x0010_5d98, 0x0013_c574)?; // Rider_HandleImpact
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 2;
    call_ptr(c, 0x0032_f708, 0x0013_c580)?; // CollHull_dtor
    Ok(true)
}

/// `CollHull_dtor(&hull, 2)` (a0 already holds the hull).
fn rrs_destroy_hull(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    c.g[A1][0] = 2;
    call_ptr(c, 0x0032_f708, 0x0013_c4b8)
}

// ---------------------------------------------------------------------------
// HUD_DrawTextLerp (0x1f1e28)

const HUD_LERP_GET_ELEM_POS: Callee = Callee {
    addr: 0x001e_91f8,
    f: ee::s04::HUD_GetElemPos,
};
const HUD_LERP_GET_ELEM_SCALE: Callee = Callee {
    addr: 0x001e_9290,
    f: ee::s04::HUD_GetElemScale,
};
const HUD_LERP_ALIGN_ELEM_POS: Callee = Callee {
    addr: 0x001f_10f8,
    f: ee::s04::HUD_AlignElemPos,
};
const HUD_LERP_GET_ELEM_COLOR: Callee = Callee {
    addr: 0x001e_91d0,
    f: ee::s04::HUD_GetElemColorRotated,
};
/// A HUD element definition is 0x24 bytes; `+0x20`/`+0x21` its horizontal
/// and vertical alignment; 0x7f in the arguments means "the element's".
pub(crate) const HUD_ELEM_SIZE: u64 = 0x24;
pub(crate) const HUD_ALIGN_DEFAULT: u64 = 0x7f;

/// `void HUD_DrawTextLerp(HUD* ctx, Font* font, HUDElem* defs, int idx1,
/// int idx2, const char* str, int a5, V2* scale_mul, float t, V2* pos_off,
/// Colour* colour, float* rect, int align, int valign)` (0x1f1e28): draw
/// `str` between the layouts of elements `idx1` and `idx2` at `t`: both
/// positions (aligned for the text's size) and scales are interpolated, the
/// colour too (or `colour` faded by `1 - t`); optionally offset, and the
/// resulting rectangle stored in `rect`.
pub fn hud_draw_text_lerp(c: &mut Cpu) {
    use crate::game::ai::control::handplantcontrol::{vu_scale, vu_spill, vu_sub};
    use ssx3_ee_rt::vu0::Src;
    guest(c, |c| {
        c.cycles += 49;
        let frame = Frame::open(c, 0x210);
        let sp0 = sp(c);
        c.g[AT][0] = 0x3f80_0000; // 1.0f
        c.mtc1(1, 0);
        frame.save(c, S0, 0x1f0);
        frame.save(c, S1, 0x1e0);
        c.g[S0][0] = HUD_ELEM_SIZE;
        frame.save(c, S2, 0x1d0);
        c.g[S1][0] = c.g[T3][0]; // scale_mul
        frame.save(c, S3, 0x1c0);
        c.g[S2][0] = c.g[A1][0]; // font
        frame.save(c, S4, 0x1b0);
        frame.save(c, S5, 0x1a0);
        c.g[S4][0] = c.g[A2][0]; // defs
        frame.save(c, S6, 0x190);
        c.g[A2][0] = sx(sp0 + 0x10);
        frame.save(c, S7, 0x180);
        frame.save(c, FP, 0x170);
        c.g[S7][0] = c.g[T0][0]; // idx2
        frame.save_fpr(c, 20, 0x200);
        c.g[FP][0] = c.g[A3][0]; // idx1
        frame.save_ra(c, 0x160);
        c.mov_s(20, 12); // t
        c.g[S5][0] = Addr(sp0 + 0x228).lw(c); // align
        c.g[A3][0] = 0;
        c.lwc1(2, oc391_at(c, S2, 0x30));
        Addr(sp0 + 0x154).sw(c, c.g[T1][0]); // str
        c.g[S6][0] = Addr(sp0 + 0x230).lw(c); // valign
        c.mov_s(12, 2);
        c.lwc1(1, oc391_at(c, S2, 0x34));
        c.g[A1][0] = c.g[T1][0];
        Addr(sp0 + 0x150).sw(c, c.g[A0][0]); // ctx
        Addr(sp0 + 0x158).sw(c, c.g[T2][0]); // a5
        c.mov_s(13, 1);
        c.swc1(0, sp0 + 4); // scale = (1, 1)
        c.g[A0][0] = c.g[S2][0];
        c.swc1(0, sp0);
        c.swc1(2, oc391_at(c, S2, 0x38));
        c.swc1(1, oc391_at(c, S2, 0x3c));
        call_ptr(c, 0x0039_1fb0, 0x001f_1ec4)?; // Font_MeasureText(font, str, &r, 0)
                                                // Element 1: position and scale.
        c.mult(0, V0, FP, S0);
        c.g[A1][0] = sx(sp0 + 0x20);
        c.g[S3][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.g[S3][0];
        HUD_LERP_GET_ELEM_POS.call(c, 0x001f_1ed8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(sp0 + 0x30);
        HUD_LERP_GET_ELEM_SCALE.call(c, 0x001f_1ee4)?;
        c.lwc1(0, sp0 + 0x30);
        if c.g[S1][0] != 0 {
            c.cycles += 7;
            c.lwc1(2, r32(c, S1));
            c.lwc1(3, oc391_at(c, S1, 4));
            c.lwc1(1, sp0 + 0x34);
            c.mul_s(0, 0, 2);
            c.mul_s(1, 1, 3);
            c.swc1(0, sp0 + 0x30);
            c.swc1(1, sp0 + 0x34);
        }
        // Element 2.
        c.cycles += 10;
        c.mult(0, V0, S7, S0);
        c.g[A1][0] = sx(sp0 + 0x40);
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
        c.g[A0][0] = c.g[S0][0];
        HUD_LERP_GET_ELEM_POS.call(c, 0x001f_1f1c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(sp0 + 0x50);
        HUD_LERP_GET_ELEM_SCALE.call(c, 0x001f_1f28)?;
        c.lwc1(1, sp0 + 0x50);
        if c.g[S1][0] != 0 {
            c.cycles += 7;
            c.lwc1(3, oc391_at(c, S1, 4));
            c.lwc1(2, r32(c, S1));
            c.lwc1(0, sp0 + 0x54);
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, sp0 + 0x50);
            c.swc1(0, sp0 + 0x54);
        }
        // The text's size at both scales.
        c.cycles += 16;
        c.lwc1(3, sp0 + 0x18);
        c.g[V0][0] = HUD_ALIGN_DEFAULT;
        c.lwc1(2, sp0 + 0x1c);
        c.lwc1(0, sp0 + 0x30);
        c.lwc1(1, sp0 + 0x34);
        c.lwc1(5, sp0 + 0x50);
        c.mul_s(0, 3, 0);
        c.lwc1(4, sp0 + 0x54);
        c.mul_s(1, 2, 1);
        c.mul_s(3, 3, 5);
        c.mul_s(2, 2, 4);
        c.swc1(0, sp0 + 0x80);
        c.swc1(1, sp0 + 0x84);
        c.swc1(3, sp0 + 0x90);
        c.swc1(2, sp0 + 0x94);
        // The alignments: the argument, or the element's when 0x7f.
        if c.g[S5][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[T1][0] = Addr(oc391_at(c, S3, 0x20)).lb(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, S5) << 24);
            c.g[T1][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
            c.g[V0][0] = HUD_ALIGN_DEFAULT;
        }
        hud_lerp_align(c, S6, FP, 0x21, T2, 2);
        hud_lerp_align(c, S5, S7, 0x20, S1, 3);
        hud_lerp_align(c, S6, S7, 0x21, S0, 3);
        // Align both layouts for the text's size.
        c.cycles += 48;
        c.g[A0][0] = Addr(sp0 + 0x150).lw(c);
        c.g[A2][0] = sx(sp0 + 0x20);
        c.g[T3][0] = Addr(sp0 + 0x158).lw(c);
        c.g[A1][0] = sx(sp0 + 0x60);
        c.g[A3][0] = sx(sp0 + 0x80);
        c.g[T0][0] = c.g[SP][0];
        HUD_LERP_ALIGN_ELEM_POS.call(c, 0x001f_2024)?;
        c.g[A0][0] = Addr(sp0 + 0x150).lw(c);
        c.g[A2][0] = sx(sp0 + 0x40);
        c.g[T3][0] = Addr(sp0 + 0x158).lw(c);
        c.g[T1][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S0][0];
        c.g[A1][0] = sx(sp0 + 0x70);
        c.g[A3][0] = sx(sp0 + 0x90);
        c.g[T0][0] = c.g[SP][0];
        HUD_LERP_ALIGN_ELEM_POS.call(c, 0x001f_2048)?;
        // Interpolate: pos = out1 + (out2 - out1) t, esc = esc1 + (esc2 - esc1) t.
        c.lwc1(8, sp0 + 0x30);
        c.lwc1(9, sp0 + 0x34);
        c.lwc1(0, sp0 + 0x50);
        c.lwc1(1, sp0 + 0x54);
        c.lwc1(6, sp0 + 0x60);
        c.sub_s(0, 0, 8);
        c.lwc1(7, sp0 + 0x64);
        c.sub_s(1, 1, 9);
        c.lwc1(2, sp0 + 0x70);
        c.lwc1(3, sp0 + 0x74);
        c.sub_s(2, 2, 6);
        c.swc1(0, sp0 + 0xc0);
        c.sub_s(3, 3, 7);
        c.swc1(1, sp0 + 0xc4);
        c.mul_s(0, 0, 20);
        c.g[A0][0] = Addr(sp0 + 0x210).lw(c); // pos_off
        c.mul_s(4, 2, 20);
        c.swc1(2, sp0 + 0xe0);
        c.mul_s(5, 3, 20);
        c.swc1(3, sp0 + 0xe4);
        c.mul_s(1, 1, 20);
        c.add_s(3, 4, 6);
        c.swc1(4, sp0 + 0xd0);
        c.add_s(2, 5, 7);
        c.swc1(5, sp0 + 0xd4);
        c.add_s(0, 0, 8);
        c.add_s(1, 1, 9);
        c.swc1(3, sp0 + 0xb0);
        c.swc1(2, sp0 + 0xb4);
        c.swc1(0, sp0 + 0xa0);
        c.swc1(1, sp0 + 0xa4);
        if c.g[A0][0] != 0 {
            c.cycles += 6;
            c.lwc1(0, oc391_at(c, A0, 4));
            c.lwc1(1, r32(c, A0));
            c.add_s(0, 2, 0);
            c.add_s(1, 3, 1);
            c.swc1(0, sp0 + 0xb4);
            c.swc1(1, sp0 + 0xb0);
        }
        // The rectangle.
        c.cycles += 3;
        c.g[A1][0] = Addr(sp0 + 0x220).lw(c);
        c.lwc1(1, sp0 + 0xa0);
        if c.g[A1][0] != 0 {
            c.cycles += 11;
            let rect = r32(c, A1);
            c.lwc1(2, sp0 + 0x18);
            c.lwc1(0, sp0 + 0x1c);
            c.lwc1(3, sp0 + 0xa4);
            c.mul_s(2, 2, 1);
            c.lwc1(4, sp0 + 0xb0);
            c.mul_s(0, 0, 3);
            c.lwc1(1, sp0 + 0xb4);
            c.swc1(4, rect);
            c.swc1(1, rect + 4);
            c.swc1(0, rect + 0xc);
            c.swc1(2, rect + 8);
        }
        // The colour.
        c.cycles += 3;
        c.g[A2][0] = Addr(sp0 + 0x218).lw(c);
        c.g[A3][0] = c.g[A2][0];
        if c.g[A2][0] == 0 {
            // Interpolate the elements' colours (alpha of the second 0).
            c.cycles += 47;
            c.g[S0][0] = HUD_ELEM_SIZE;
            c.g[A1][0] = sx(sp0 + 0xf0);
            c.mult(0, V0, FP, S0);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
            HUD_LERP_GET_ELEM_COLOR.call(c, 0x001f_2138)?;
            c.mult(0, V0, S7, S0);
            c.g[A1][0] = sx(sp0 + 0x100);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
            HUD_LERP_GET_ELEM_COLOR.call(c, 0x001f_2148)?;
            Addr(sp0 + 0x10c).sw(c, 0);
            vu_sub(c, sp0 + 0x100, sp0 + 0xf0);
            vu_spill(c, frame, 0x140, &[0x130]);
            c.mfc1(3, 20);
            vu_scale(c, 3, sp0 + 0x130);
            vu_spill(c, frame, 0x140, &[0x120]);
            c.lqc2(3, sp0 + 0x120);
            c.lqc2(4, sp0 + 0xf0);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            vu_spill(c, frame, 0x140, &[0x110]);
            // (x, y, z, w) -> (w, x, y, z)
            c.lwc1(0, sp0 + 0x11c);
            c.lwc1(1, sp0 + 0x110);
            c.lwc1(2, sp0 + 0x114);
            c.lwc1(3, sp0 + 0x118);
            c.swc1(0, sp0 + 0x120);
            c.swc1(1, sp0 + 0x124);
            c.swc1(2, sp0 + 0x128);
            c.swc1(3, sp0 + 0x12c);
            hud_pip_copy16(c, V1, A0, sp0 + 0x120, sp0 + 0xd0);
        } else {
            // The given colour, its first component faded by 1 - t.
            c.cycles += 15;
            c.g[AT][0] = 0x3f80_0000; // 1.0f
            c.mtc1(1, 1);
            hud_pip_copy16(c, V0, V1, r32(c, A3), sp0 + 0xd0);
            c.sub_s(1, 1, 20);
            c.lwc1(0, sp0 + 0xd0);
            c.mul_s(0, 0, 1);
            c.swc1(0, sp0 + 0xd0);
        }
        c.g[V0][0] = Addr(oc391_at(c, GP, 0u32.wrapping_sub(0x854))).lw(c);
        // Layer, colour and scale onto the font, draw, restore the scale.
        c.cycles += 51;
        c.g[A3][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V1][0] = Addr(oc391_at(c, S4, 0x3ca)).lb(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = Addr(oc391_at(c, V0, 0xe84)).lw(c);
        c.g[V1][0] = sx(r32(c, V1) << 5);
        c.g[A1][0] = Addr(sp0 + 0x154).lw(c);
        c.g[V0][0] = Addr(oc391_at(c, A2, 8)).lw(c);
        c.g[V1][0] &= 0x3e0;
        c.g[V0][0] &= c.g[A3][0];
        c.g[V0][0] |= c.g[V1][0];
        Addr(oc391_at(c, A2, 8)).sw(c, c.g[V0][0]);
        hud_pip_copy16(c, A2, A3, sp0 + 0xd0, oc391_at(c, S2, 0x40));
        c.lwc1(2, sp0 + 0xa0);
        c.lwc1(3, sp0 + 0xa4);
        c.lwc1(1, oc391_at(c, S2, 0x30));
        c.lwc1(0, oc391_at(c, S2, 0x34));
        c.mul_s(1, 1, 2);
        c.lwc1(12, sp0 + 0xb0);
        c.mul_s(0, 0, 3);
        c.lwc1(13, sp0 + 0xb4);
        c.swc1(1, oc391_at(c, S2, 0x38));
        c.swc1(0, oc391_at(c, S2, 0x3c));
        call_ptr(c, 0x0039_1cb0, 0x001f_2294)?; // Font_DrawText
        c.lwc1(3, sp0);
        c.lwc1(2, sp0 + 4);
        c.lwc1(1, oc391_at(c, S2, 0x30));
        c.lwc1(0, oc391_at(c, S2, 0x34));
        c.mul_s(1, 1, 3);
        c.mul_s(0, 0, 2);
        c.swc1(1, oc391_at(c, S2, 0x38));
        c.swc1(0, oc391_at(c, S2, 0x3c));
        frame.restore(c, S0, 0x1f0);
        frame.restore(c, S1, 0x1e0);
        frame.restore(c, S3, 0x1c0);
        frame.restore(c, S2, 0x1d0);
        frame.restore(c, S4, 0x1b0);
        frame.restore(c, S5, 0x1a0);
        frame.restore(c, S6, 0x190);
        frame.restore(c, S7, 0x180);
        frame.restore(c, FP, 0x170);
        frame.restore_ra(c, 0x160);
        frame.restore_fpr(c, 20, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// One alignment of [`hud_draw_text_lerp`] (v0 = 0x7f on entry): `out` =
/// the argument in `arg` (sign-extended byte), or, when it is 0x7f, the byte
/// at `+field` of element `idx`; `head` is the original's entry cost (2 for
/// the first such block, 3 for the later ones, which reload v0).
fn hud_lerp_align(c: &mut Cpu, arg: usize, idx: usize, field: u32, out: usize, head: u64) {
    c.cycles += head;
    c.g[V0][0] = HUD_ALIGN_DEFAULT;
    let default = c.g[arg][0] == c.g[V0][0];
    c.g[V0][0] = sx(r32(c, arg) << 24);
    if default {
        c.cycles += 5;
        c.g[V0][0] = HUD_ELEM_SIZE;
        c.mult(0, V1, idx, V0);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S4)));
        c.g[out][0] = Addr(oc391_at(c, V0, field)).lb(c);
    } else {
        c.cycles += 1;
        c.g[out][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
    }
}

// ---------------------------------------------------------------------------
// Net_UpdateConnect (0x263828)

const NET_DISCONNECT_ALL: Callee = Callee {
    addr: 0x0026_36f0,
    f: ee::s07::Net_DisconnectAll,
};
/// `gp-0x238`: the comm system (`+0x20` its state: 5 connected, 6 failed;
/// `+0x28` the connection error); `gp-0xc8`: the lobby manager (`+0xf0` its
/// message list head); `gp-0x848`: the app (`+0x14` the frame time).
pub(crate) const NET_COMM_SYSTEM_GP: u32 = 0x238;
pub(crate) const NET_LOBBY_MAN_GP: u32 = 0xc8;
/// The connect timer: `+0` state (1 connecting, 2 connected), `+4` the time
/// left.
pub(crate) const NET_CONNECT_STATE: u32 = 0x0;
pub(crate) const NET_CONNECT_TIMER: u32 = 0x4;

/// How [`net_update_connect`] leaves: through which of the original's three
/// epilogue entries (each restores fewer registers and costs less).
#[derive(Clone, Copy)]
enum NetConnectExit {
    /// `lq s0; lq s1; ld ra` (5 cycles).
    Full,
    /// `lq s1; ld ra` (4).
    S1,
    /// `ld ra` (3).
    Ra,
}

/// `void Net_UpdateConnect(DsTimer* self)` (0x263828): the connect step.
/// Connecting (state 1): on the comm system reaching state 5 become
/// connected and post message 0x105 to the lobby manager; on its failure
/// (state 6) or when the timer runs out disconnect (with the comm error or
/// 0x2d3f3f3f); else count the timer down by the frame time. Connected
/// (state 2): disconnect when the comm system failed.
pub fn net_update_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = Addr(oc391_at(c, A0, NET_CONNECT_STATE)).lw(c);
        let connecting = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let exit = if connecting {
            net_update_connect_connecting(c, gp)?
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                NetConnectExit::S1
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let connected = c.g[V1][0] == c.g[V0][0];
                c.g[V1][0] = Addr(gp.wrapping_sub(NET_COMM_SYSTEM_GP)).lw(c);
                if connected {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(oc391_at(c, V1, 0x20)).lw(c) ^ 6;
                    if c.g[V0][0] != 0 {
                        NetConnectExit::S1
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = Addr(oc391_at(c, V1, 0x28)).lw(c);
                        NET_DISCONNECT_ALL.call(c, 0x0026_3980)?;
                        NetConnectExit::Full
                    }
                } else {
                    c.cycles += 2;
                    NetConnectExit::Ra
                }
            }
        };
        c.cycles += match exit {
            NetConnectExit::Full => 5,
            NetConnectExit::S1 => 4,
            NetConnectExit::Ra => 3,
        };
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// State 1 of [`net_update_connect`] (a0 = self).
fn net_update_connect_connecting(
    c: &mut Cpu,
    gp: u32,
) -> Result<NetConnectExit, crate::call::Unwound> {
    c.cycles += 5;
    c.g[A1][0] = Addr(gp.wrapping_sub(NET_COMM_SYSTEM_GP)).lw(c);
    c.g[V1][0] = Addr(oc391_at(c, A1, 0x20)).lw(c);
    let up = c.g[V1][0] == 5;
    c.g[V0][0] = c.g[V1][0] ^ 6;
    if up {
        // Connected: state 2 and message 0x105 to the lobby manager.
        c.cycles += 27;
        c.g[V0][0] = 2;
        Addr(oc391_at(c, A0, NET_CONNECT_STATE)).sw(c, 2);
        c.g[A1][0] = 0x0048_0488; // the allocation tag
        c.g[S0][0] = Addr(gp.wrapping_sub(NET_LOBBY_MAN_GP)).lw(c);
        c.g[A0][0] = 8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_389c)?; // EA_MemAllocNamed
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x105;
        let msg = r32(c, S1);
        Addr(msg).sw(c, 0x105);
        c.g[V0][0] = 0x0048_12b0; // the message vtable
        Addr(msg + 4).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x0048_04a8;
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(oc391_at(c, S0, 0xf0)).lw(c); // the list head
        c.g[A3][0] = 0;
        Addr(sp(c) + 0x10).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_38d4)?;
        // Link the node {next, prev, msg} in before the head.
        c.g[A1][0] = c.g[V0][0];
        let node = r32(c, A1);
        c.g[V0][0] = sx(node.wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(node + 8).sw(c, c.g[S1][0]);
        }
        c.cycles += 10;
        let sp0 = sp(c);
        c.g[V0][0] = Addr(sp0 + 0x10).lw(c);
        Addr(node).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(oc391_at(c, V0, 4)).lw(c);
        Addr(node + 4).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(oc391_at(c, V0, 4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(sp0 + 0x10).lw(c);
        Addr(oc391_at(c, V0, 4)).sw(c, c.g[A1][0]);
        Addr(sp0).sw(c, c.g[A1][0]);
        return Ok(NetConnectExit::Full);
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // The comm system failed.
        c.cycles += 4;
        c.g[A1][0] = Addr(oc391_at(c, A1, 0x28)).lw(c);
        NET_DISCONNECT_ALL.call(c, 0x0026_391c)?;
        return Ok(NetConnectExit::S1);
    }
    c.lwc1(1, oc391_at(c, A0, NET_CONNECT_TIMER));
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(1, 0);
    c.g[V0][0] = 1;
    if !c.fcc() {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    c.g[A1][0] = 0x2d3f_0000;
    if c.g[V0][0] != 0 {
        // Timed out.
        c.cycles += 4;
        c.g[A1][0] |= 0x3f3f;
        NET_DISCONNECT_ALL.call(c, 0x0026_394c)?;
        return Ok(NetConnectExit::S1);
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    c.lwc1(0, oc391_at(c, V0, 0x14));
    c.sub_s(0, 1, 0);
    c.swc1(0, oc391_at(c, A0, NET_CONNECT_TIMER));
    Ok(NetConnectExit::Full)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// UIListBox_Render: a list box with a mode font from its state data,
    /// empty strings (the measure call returns a zero rectangle), the
    /// renderer at `gp-0x854` and `gp+0x2a90` with fake draw methods; the
    /// seeds vary the early exits, the colour, shadow and string flags, the
    /// overflow test and the debug outline.
    #[test]
    fn ui_list_box_render_matches() {
        check(
            ee::s13::f_00399f00__generated,
            ui_list_box_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x399f00, 3);
                c.pc = 0x0039_9f00;
                let this = DATA + 0x1000;
                let font = DATA + 0x2000;
                let (p1, p2, p3) = (DATA + 0x3000, DATA + 0x3100, DATA + 0x3200);
                let table = DATA + 0x3300;
                let loc = DATA + 0x3400;
                let renderer = DATA + 0x5000;
                let state = DATA + 0x5800;
                c.g[A0][0] = sx(this);
                c.f[12] = r.f32(200.0);
                c.f[13] = r.f32(200.0);
                // The list box.
                for off in (0..0x320).step_by(4) {
                    c.bus.write32(this + off, r.next_u32());
                }
                for off in [0x1c, 0x44, 0x48, 0x4c, 0x50, 0x54, 0x60, 0x64] {
                    c.bus.write32(this + off, r.f32(100.0));
                }
                for i in 0..4u32 {
                    c.bus
                        .write32(this + 0x9c + 0x14 * i, DATA + 0x6000 + 0x20 * i);
                }
                c.bus
                    .write8(this + 0x318, if seed % 7 == 1 { 0 } else { 3 });
                c.bus.write8(this + 0x319, (seed % 3) as u8);
                c.bus.write32(this + 0x5c, p1);
                c.bus.write32(p1 + 0xd0, p2);
                c.bus.write32(p2 + 0x10, p3);
                c.bus.write32(p3 + 8, table);
                let has_font = seed % 11 != 3;
                for i in 0..4u32 {
                    c.bus
                        .write32(table + 0xc + 0xc * i, if has_font { font } else { 0 });
                }
                c.bus
                    .write32(p3 + 0x10, if seed % 5 == 0 { 0 } else { loc });
                c.bus.write32(loc, r.next_u32());
                // The font.
                for off in (0..0x60).step_by(4) {
                    c.bus.write32(font + off, r.f32(4.0));
                }
                c.bus.write32(font + 0x14, r.next_u32() & 0x3f);
                // The renderer.
                let vt = vtable(c, &mut r, &[0x288, 0x248, 0x238]);
                c.bus.write32(renderer + 0x10d8, vt);
                c.bus.write32(renderer + 0xe84, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(GAME_GP.wrapping_sub(0x854), renderer);
                c.bus.write32(GAME_GP + 0x2a90, renderer);
            },
        );
    }
    /// The blend fixture: self at DATA (fake vtable slot 0x218), other at
    /// DATA+0x100, random floats and t.
    fn twpigd_blend_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, entry as u64, 2);
        c.pc = entry;
        let this = DATA;
        let other = DATA + 0x100;
        let vt = vtable(c, &mut r, &[0x218]);
        c.bus.write32(this + 4, vt);
        c.bus.write32(this + 8, r.f32(10.0));
        c.bus.write32(this + 0xc, r.f32(10.0));
        c.bus.write32(other + 4, r.f32(10.0));
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(other);
        c.f[12] = if seed % 4 == 0 {
            r.edge_f32()
        } else {
            r.f32(1.5)
        };
    }

    /// tWPIGD_Surface_Blend: both hook calls, the density blend.
    #[test]
    fn twpigd_surface_blend_matches() {
        check(
            ee::s08::f_002bd518__generated,
            twpigd_surface_blend,
            200,
            &|c, seed| twpigd_blend_setup(c, seed, 0x002b_d518),
        );
    }

    /// tWPIGD_SkyBox_Blend: both hook calls, the blend.
    #[test]
    fn twpigd_sky_box_blend_matches() {
        check(
            ee::s08::f_002bd2e8__generated,
            twpigd_sky_box_blend,
            200,
            &|c, seed| twpigd_blend_setup(c, seed, 0x002b_d2e8),
        );
    }

    /// PS2GraphicsMan_IsIdle: the real IsRenderIdle on a busy or idle
    /// graphics manager.
    #[test]
    fn ps2_graphics_man_is_idle_matches() {
        check(
            ee::s13::f_00395f80__generated,
            ps2_graphics_man_is_idle,
            50,
            &|c, seed| {
                let mut r = machine(c, seed, 0x395f80, 0);
                c.pc = 0x0039_5f80;
                c.g[A0][0] = sx(DATA);
                let busy = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(DATA + 0x5a8c, busy);
            },
        );
    }

    /// StrHashElf: random strings of 0..40 bytes with high-bit characters
    /// (sign extension) and lengths that fold the top nibble.
    #[test]
    fn str_hash_elf_matches() {
        check(
            ee::s10::f_00317618__generated,
            str_hash_elf,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x317618, 0);
                c.pc = 0x0031_7618;
                let len = seed % 41;
                for i in 0..len {
                    let mut b = r.next_u32() as u8;
                    if b == 0 {
                        b = 0x41;
                    }
                    c.bus.write8(DATA + 0x100 + i, b);
                }
                c.bus.write8(DATA + 0x100 + len, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
            },
        );
    }

    /// ReferThreadStatus: fake syscalls (which may unwind).
    #[test]
    fn refer_thread_status_matches() {
        crate::game::lib::kernel::tests::syscall_test(
            ee::s15::f_00423ca0__generated,
            refer_thread_status,
            0x423ca0,
        );
    }

    /// RotateThreadReadyQueue: fake syscalls (which may unwind).
    #[test]
    fn rotate_thread_ready_queue_matches() {
        crate::game::lib::kernel::tests::syscall_test(
            ee::s15::f_00423c50__generated,
            rotate_thread_ready_queue,
            0x423c50,
        );
    }

    /// WScriptFn_GetObjectSlot30: initialised or not, -1..3 argument entries
    /// (raw and float stores), the world object (id -1), a found object with
    /// or without a component (fake slot 0xf0), an unknown descriptor or
    /// entry.
    #[test]
    fn wscript_fn_get_object_slot30_matches() {
        check(
            ee::s10::f_00301680__generated,
            wscript_fn_get_object_slot30,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x301680, 1);
                c.pc = 0x0030_1680;
                let gp = GAME_GP;
                let ret = DATA + 0x100;
                let entries = DATA + 0x200;
                let x = DATA + 0x2300;
                let base = DATA + 0x2400;
                let high = DATA + 0x2800;
                let desc = DATA + 0x2c00;
                let world = DATA + 0x2e00;
                let target = DATA + 0x3000;
                let comp = DATA + 0x3100;
                c.g[A0][0] = sx(ret);
                let n = [-1i64, 0, 1, 2, 3][(seed % 5) as usize];
                c.g[A1][0] = n as u64;
                c.g[A2][0] = sx(entries);
                for i in 0..4u32 {
                    c.bus.write32(gp + WSCRIPT_GOS30_TYPES + 4 * i, i % 3);
                }
                c.bus.write32(gp + WSCRIPT_GOS30_INIT, (seed / 5) % 2);
                c.bus.write8(gp + WSCRIPT_GOS30_TAG, r.next_u32() as u8);
                let id = match (seed / 2) % 5 {
                    0 => 0xffff_ffffu32, // the world object
                    1 => 0x0000_0101,    // lo=1, hi=1: found
                    2 => 0x0000_0103,    // lo=3: no such descriptor
                    3 => 0x0000_0201,    // hi=2: an empty entry
                    _ => 0x0000_0101,
                };
                c.bus.write32(gp + WSCRIPT_GOS30_IDS, id);
                c.bus.write32(gp + WSCRIPT_GOS30_IDS + 4, r.next_u32());
                for i in 0..4u32 {
                    let e = entries + 16 * i;
                    c.bus.write32(e, (seed >> (2 * i)) % 2);
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, (seed >> (3 + i)) % 4);
                }
                c.bus.write32(gp + WSCRIPT_GOS30_TABLE, x);
                c.bus.write32(x, x + 0x40);
                c.bus.write32(x + 0x48, base);
                c.bus.write32(gp + WSCRIPT_GOS30_WORLD, world);
                c.bus
                    .write32(world + 0x290, if seed % 7 == 0 { 0 } else { target });
                for lo in 0..4u32 {
                    c.bus
                        .write32(base + 4 * lo, if lo == 1 || lo == 2 { desc } else { 0 });
                }
                c.bus.write32(desc + 0x1c, high);
                for hi in 0..4u32 {
                    c.bus
                        .write32(high + 4 * hi, if hi == 1 { (target >> 2) << 8 } else { 0 });
                }
                c.bus
                    .write32(target + 0xc, if seed % 3 == 0 { 0 } else { comp });
                let vt = vtable(c, &mut r, &[0xf0]);
                c.bus.write32(comp + 0xc, vt);
            },
        );
    }

    /// PS2GraphicsMan_AllocBlendedMatrixHistory: a fresh history (the real
    /// allocator), or 0..4 entries of 0..3 weights of which one may equal
    /// the matrix (the real MatchBlendedMatrix): found (bone copy) or
    /// appended.
    #[test]
    fn ps2_graphics_man_alloc_blended_matrix_history_matches() {
        check(
            ee::s12::f_003698e0__generated,
            ps2_graphics_man_alloc_blended_matrix_history,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3698e0, 0);
                crate::game::wscript::wscriptman::tests::alloc_ready(c);
                c.pc = 0x0036_98e0;
                let this = DATA + 0x100;
                let m = DATA + 0x1800;
                let history = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(m);
                let fresh = seed % 6 == 0;
                c.bus
                    .write32(this + PS2GM_BLEND_HISTORY, if fresh { 0 } else { history });
                let n = (seed / 6) % 5;
                c.bus
                    .write32(this + PS2GM_BLEND_COUNT, if fresh { 0 } else { n });
                // The matrix: 0..3 weights of small values.
                let count = (seed / 2) % 4;
                c.bus.write32(m, count);
                c.bus.write32(m + 4, m + 0x40);
                for k in 0..4u32 {
                    c.bus.write32(m + 0x40 + 4 * k, r.next_u32() & 0x000f_000f);
                }
                // The entries; one of them may equal the matrix.
                let same = if seed % 3 == 0 { n } else { r.next_u32() % 4 };
                for i in 0..4u32 {
                    let e = history + 0xc * i;
                    let w = DATA + 0x3000 + 0x40 * i;
                    c.bus.write32(e + 4, w);
                    if i == same {
                        c.bus.write32(e, count);
                        for k in 0..4u32 {
                            let v = c.bus.read32(m + 0x40 + 4 * k);
                            c.bus.write32(w + 4 * k, v);
                        }
                    } else {
                        c.bus.write32(e, r.next_u32() % 4);
                        for k in 0..4u32 {
                            c.bus.write32(w + 4 * k, r.next_u32() & 0x000f_000f);
                        }
                    }
                }
                // The append target past the in-use entries.
                for i in 4..6u32 {
                    c.bus
                        .write32(history + 0xc * i + 4, DATA + 0x3000 + 0x40 * i);
                }
            },
        );
    }

    /// strncmp: strings at 16-, 8- and 1-byte alignments, equal prefixes of
    /// random length with a difference and/or a NUL somewhere, n from 0 to
    /// 80 (the quadword, doubleword and byte paths and their exits).
    #[test]
    fn strncmp_matches() {
        const A: u32 = DATA + 0x100;
        const B: u32 = DATA + 0x400;
        check(ee::s15::f_00416b18__generated, strncmp, 400, &|c, seed| {
            let mut r = machine(c, seed, 0x416b18, 0);
            c.pc = STRNCMP_ENTRY;
            let (oa, ob) = match seed % 5 {
                0 | 1 => (0, 0),
                2 => (8, 8),
                3 => (8 * (r.next_u32() % 2), 8 * (r.next_u32() % 2)),
                _ => (r.next_u32() % 16, r.next_u32() % 16),
            };
            let n = match seed % 7 {
                0 => r.next_u32() % 16,
                1 => 0,
                _ => r.next_u32() % 80,
            };
            // Non-NUL bytes, the same in both strings.
            for i in 0..0x120u32 {
                let v = (r.next_u32() % 255 + 1) as u8;
                c.bus.write8(A + oa + i, v);
                c.bus.write8(B + ob + i, v);
            }
            if seed % 3 != 0 {
                let at = r.next_u32() % (n + 8);
                let v = c.bus.read8(B + ob + at);
                c.bus
                    .write8(B + ob + at, v ^ (1 + r.next_u32() % 255) as u8);
            }
            if seed % 4 != 1 {
                let at = r.next_u32() % (n + 8);
                c.bus.write8(A + oa + at, 0);
                if seed % 2 == 0 {
                    c.bus.write8(B + ob + at, 0);
                }
            }
            c.g[A0][0] = sx(A + oa);
            c.g[A1][0] = sx(B + ob);
            c.g[A2][0] = n as u64;
        });
    }

    /// HUD_DrawPipMeter: 0..5 pips, aligned (the real HUD_AlignElemPos) or
    /// centred on a position, with or without a scale, with or without a
    /// texture (the real HUD_DrawSprite returns early); the sprite draw is a
    /// fake renderer method.
    #[test]
    fn hud_draw_pip_meter_matches() {
        check(
            ee::s05::f_0021f338__generated,
            hud_draw_pip_meter,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21f338, 6);
                c.pc = 0x0021_f338;
                let (hud, obj) = (DATA, DATA + 0x1000);
                let (pos, scale) = (DATA + 0x2000, DATA + 0x2100);
                let tex = DATA + 0x4000;
                c.g[A0][0] = sx(hud);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = sx(seed % 6);
                c.g[A3][0] = sx(r.next_u32() % 3);
                c.g[T0][0] = if seed % 3 == 0 { 0 } else { sx(pos) };
                c.g[T1][0] = if seed % 4 == 0 { 0 } else { sx(scale) };
                c.f[12] = r.f32(1.0);
                for k in 0..0x28u32 {
                    c.bus.write32(HUD_PIP_COLOURS + 4 * k, r.f32(1.0));
                }
                crate::game::fe::ovstatehud::tests::rider_ind_elem(c, &mut r, obj + 0x240);
                c.bus.write32(pos, r.f32(300.0));
                c.bus.write32(pos + 4, r.f32(300.0));
                c.bus.write32(scale, r.f32(2.0));
                c.bus.write32(scale + 4, r.f32(2.0));
                c.bus
                    .write32(hud + 0x4a0, if seed % 5 == 0 { 0 } else { tex });
                c.bus.write32(tex, r.next_u32() % 64);
                for j in 1..8 {
                    c.bus.write32(tex + 4 * j, r.f32(64.0));
                }
                // The renderer (gp-0x854): draw state and a fake sprite method.
                let renderer = DATA + 0x6000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), renderer);
                c.bus.write32(renderer + 0xe84, DATA + 0x7800);
                c.bus.write32(DATA + 0x7800 + 8, r.next_u32());
                let vt = vtable(c, &mut r, &[0x238]);
                c.bus.write32(renderer + 0x10d8, vt);
            },
        );
    }

    /// Gate of [`bolt_cu_fake_is_done`] and its answer/log area.
    pub(crate) const BOLT_CU_CTRL: u32 = DATA + 0xc_0000;
    pub(crate) const BOLT_CU_MAGIC: u32 = 0xb017_d250;

    /// `AsyncFile_IsDone` (0x3df980) for the cache update test: logs the
    /// handle and `ra`, answers 0 or 2 from the handle's low bit (never 1:
    /// the finished-read path runs the real release and unpack chain).
    fn bolt_cu_fake_is_done(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(BOLT_CU_CTRL) != BOLT_CU_MAGIC {
            return fall_back(
                c,
                prev_cell(bolt_cu_fake_is_done),
                ee::s14::f_003df980__generated,
            );
        }
        let n = c.bus.read32(BOLT_CU_CTRL + 4);
        c.bus.write32(BOLT_CU_CTRL + 4, n + 1);
        let at = BOLT_CU_CTRL + 0x10 + 8 * (n & 0x3ff);
        c.bus.write32(at, c.g[A0][0] as u32);
        c.bus.write32(at + 4, c.g[RA][0] as u32);
        c.g[V0][0] = (c.g[A0][0] & 1) * 2;
        c.cycles += 9;
        ret(c);
    }

    /// BoltFile_CacheUpdate: all 10 tables; pending slots (loading or not)
    /// whose read is not done (a fake IsDone), loads pending or not, and
    /// queued slots only while loads are pending. Not reached: a finished
    /// read (the real release/unpack chain) and starting a queued slot (the
    /// real BoltFile_LoadSlot): both need the whole streaming setup.
    #[test]
    fn bolt_file_cache_update_matches() {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            bolt_cu_fake_is_done,
            prev_cell(bolt_cu_fake_is_done),
        );
        check(
            ee::s03::f_0019d250__generated,
            bolt_file_cache_update,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19d250, 0);
                c.pc = 0x0019_d250;
                let this = DATA;
                c.g[A0][0] = sx(this);
                c.bus.write32(BOLT_CU_CTRL, BOLT_CU_MAGIC);
                let loads = if seed % 2 == 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write32(this + BOLT_CACHE_PENDING, loads);
                for k in 0..64 {
                    let i = r.next_u32() % BOLT_CACHE_TABLES;
                    let j = r.next_u32() % 256;
                    let slot = this + i * BOLT_CACHE_TABLE_SIZE + j * BOLT_CACHE_SLOT_SIZE;
                    let mut state = r.next_u32() % 5;
                    if state == 3 && loads == 0 {
                        state = 4;
                    }
                    c.bus.write32(slot + BOLT_SLOT_STATE, state);
                    c.bus.write32(slot + BOLT_SLOT_PENDING, (k % 3 != 0) as u32);
                    c.bus.write32(slot + BOLT_SLOT_HANDLE, r.next_u32());
                }
            },
        );
    }

    /// LocStringTable_GetStringPtr: unaligned tables, three languages, ids
    /// negative, in and out of range, with and without the remap table (the
    /// real RemapId and bsearch, ids found or not).
    #[test]
    fn loc_string_table_get_string_ptr_matches() {
        check(
            ee::s14::f_003dbb68__generated,
            loc_string_table_get_string_ptr,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3dbb68, 0);
                c.pc = 0x003d_bb68;
                let this = DATA + 0x1000 + (seed % 4);
                let put = |c: &mut Cpu, at: u32, v: u32| {
                    for k in 0..4 {
                        c.bus.write8(at + k, (v >> (8 * k)) as u8);
                    }
                };
                let lang = seed % 3;
                let remap = 0x101u32;
                put(c, this + 4, remap);
                c.bus.write8(this + 8, (seed % 2) as u8 | 0x10);
                c.bus.write8(this + 0xe, lang as u8);
                c.bus.write8(this + 0xf, 0);
                for l in 0..3u32 {
                    let table = 0x203 + 0x100 * l;
                    put(c, this + 0x10 + 4 * l, table);
                    let count = 5 + l;
                    put(c, this + table + 0xc, count);
                    for i in 0..count {
                        put(c, this + table + 0x10 + 4 * i, 0x40 + 3 * i + l);
                    }
                }
                // The remap table: 6 sorted keys 10, 20, .. mapping to 0..7.
                put(c, this + remap + 8, 6);
                for k in 0..6u32 {
                    c.bus
                        .write16(this + remap + 0x10 + 4 * k, (10 * (k + 1)) as u16);
                    c.bus
                        .write16(this + remap + 0x12 + 4 * k, (r.next_u32() % 8) as u16);
                }
                c.g[A0][0] = sx(this);
                let id: i32 = match seed % 5 {
                    0 => -(1 + (r.next_u32() % 5) as i32),
                    1 => 10 * (1 + (r.next_u32() % 7) as i32),
                    _ => (r.next_u32() % 9) as i32,
                };
                c.g[A1][0] = id as i64 as u64;
            },
        );
    }

    /// IniParser_SkipWhitespace: texts of whitespace, comments (the real
    /// SkipComment), `[`, other characters, and the end of the text.
    #[test]
    fn ini_parser_skip_whitespace_matches() {
        check(
            ee::s08::f_002a78f8__generated,
            ini_parser_skip_whitespace,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a78f8, 0);
                c.pc = 0x002a_78f8;
                let text = DATA + 0x100;
                c.bus.write32(DATA + INI_PARSER_CURSOR, text);
                let len = seed % 24;
                let set = b" \t\n\r\t ;/#[a=";
                for i in 0..len {
                    let k = r.next_u32() as usize % if seed % 3 == 0 { 6 } else { set.len() };
                    c.bus.write8(text + i, set[k]);
                }
                c.bus.write8(text + len, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// Gate of [`bolt_fs_fake_mem_free`] and its log.
    pub(crate) const BOLT_FS_CTRL: u32 = DATA + 0xc_8000;
    pub(crate) const BOLT_FS_MAGIC: u32 = 0xb017_d4b8;

    /// `EA_MemFree` (0x317e98) for the free-slot test: logs the block and
    /// `ra`.
    fn bolt_fs_fake_mem_free(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(BOLT_FS_CTRL) != BOLT_FS_MAGIC {
            return fall_back(
                c,
                prev_cell(bolt_fs_fake_mem_free),
                ee::s10::f_00317e98__generated,
            );
        }
        c.bus.write32(BOLT_FS_CTRL + 4, c.g[A0][0] as u32);
        c.bus.write32(BOLT_FS_CTRL + 8, c.g[RA][0] as u32);
        c.g[V0][0] = 0x1234;
        c.cycles += 13;
        ret(c);
    }

    /// BoltFile_FreeSlot: loading with or without a pending read (the real
    /// release chain on an unknown handle: mutex syscalls are fakes), other
    /// states, with or without data (a fake EA_MemFree).
    #[test]
    fn bolt_file_free_slot_matches() {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            bolt_fs_fake_mem_free,
            prev_cell(bolt_fs_fake_mem_free),
        );
        check(
            ee::s03::f_0019d4b8__generated,
            bolt_file_free_slot,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19d4b8, 0);
                c.pc = 0x0019_d4b8;
                c.bus.write32(BOLT_FS_CTRL, BOLT_FS_MAGIC);
                let this = DATA;
                let table = r.next_u32() % BOLT_CACHE_TABLES;
                let index = r.next_u32() % 256;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(table);
                c.g[A2][0] = sx(index);
                let slot = this + table * BOLT_CACHE_TABLE_SIZE + index * BOLT_CACHE_SLOT_SIZE;
                for off in (0..BOLT_CACHE_SLOT_SIZE).step_by(4) {
                    c.bus.write32(slot + off, r.next_u32());
                }
                c.bus.write32(
                    slot + BOLT_SLOT_STATE,
                    if seed % 2 == 0 { 2 } else { seed % 5 },
                );
                c.bus
                    .write32(slot + BOLT_SLOT_PENDING, (seed % 3 != 0) as u32);
                c.bus.write32(
                    slot + BOLT_SLOT_DATA,
                    if seed % 4 == 0 { 0 } else { DATA + 0xd_0000 },
                );
                c.bus.write32(this + BOLT_CACHE_BYTES, r.next_u32());
            },
        );
    }

    /// Gate, answers and call log of the `rrs_fake_*` callees.
    pub(crate) const RRS_CTRL: u32 = DATA + 0xd_0000;
    pub(crate) const RRS_MAGIC: u32 = 0x013c_1400;
    /// The contact record the fake closest-contact query copies to its a2.
    pub(crate) const RRS_RECORD: u32 = RRS_CTRL + 0x100;

    /// Log a fake call's arguments, `ra`, `sp`, cycles, f12 and the 0x40
    /// bytes at a1 at `RRS_CTRL + 0x200 + slot * 0x80`.
    fn rrs_record(c: &mut Cpu, slot: u32) {
        let at = RRS_CTRL + 0x200 + slot * 0x80;
        for (i, r) in [A0, A1, A2, A3, RA, SP].into_iter().enumerate() {
            c.bus.write64(at + 8 * i as u32, c.g[r][0]);
        }
        c.bus.write64(at + 0x30, c.cycles);
        c.bus.write32(at + 0x38, c.f[12]);
        let a1 = c.g[A1][0] as u32;
        if (0x0010_0000..0x0200_0000).contains(&a1) {
            for k in 0..0x10 {
                let w = c.bus.read32(a1 + 4 * k);
                c.bus.write32(at + 0x40 + 4 * k, w);
            }
        }
    }

    macro_rules! rrs_fake {
        ($name:ident, $gen:path, $slot:expr, $body:expr) => {
            fn $name(c: &mut Cpu) {
                use crate::difftest::fakes::{fall_back, prev_cell};
                if c.bus.read32(RRS_CTRL) != RRS_MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                rrs_record(c, $slot);
                let body: fn(&mut Cpu) = $body;
                body(c);
                c.cycles += 5 + $slot as u64;
                ret(c);
            }
        };
    }

    rrs_fake!(rrs_fake_hull_ctor, ee::s11::f_0032f650__generated, 0, |c| {
        c.g[V0][0] = c.g[A0][0];
    });
    rrs_fake!(
        rrs_fake_find_contact,
        ee::s11::f_003342d0__generated,
        1,
        |c| {
            let rec = c.g[A2][0] as u32;
            for k in 0..0x18 {
                let w = c.bus.read32(RRS_RECORD + 4 * k);
                c.bus.write32(rec + 4 * k, w);
            }
            c.f[0] = c.bus.read32(RRS_CTRL + 8);
        }
    );
    rrs_fake!(
        rrs_fake_hull_dtor,
        ee::s11::f_0032f708__generated,
        2,
        |_c| {}
    );
    rrs_fake!(
        rrs_fake_translate,
        ee::s00::f_00106538__generated,
        3,
        |_c| {}
    );
    rrs_fake!(
        rrs_fake_world_matrix,
        ee::s00::f_0011e098__generated,
        4,
        |_c| {}
    );
    rrs_fake!(
        rrs_fake_handle_impact,
        ee::s00::f_00105d98__generated,
        5,
        |_c| {}
    );

    fn rrs_install_fakes() {
        use crate::difftest::fakes::{hook, prev_cell};
        use std::ptr::addr_of_mut;
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 6] = [
            (addr_of_mut!(ee::s11::OVERRIDE_0032F650), rrs_fake_hull_ctor),
            (
                addr_of_mut!(ee::s11::OVERRIDE_003342D0),
                rrs_fake_find_contact,
            ),
            (addr_of_mut!(ee::s11::OVERRIDE_0032F708), rrs_fake_hull_dtor),
            (addr_of_mut!(ee::s00::OVERRIDE_00106538), rrs_fake_translate),
            (
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                rrs_fake_world_matrix,
            ),
            (
                addr_of_mut!(ee::s00::OVERRIDE_00105D98),
                rrs_fake_handle_impact,
            ),
        ];
        for (slot, fake) in slots {
            hook(slot, fake, prev_cell(fake));
        }
    }

    /// RiderRailslideState_CollideWalls: sub-state 9; a miss (negative
    /// depth); head-on; facing the wall or not; near the rail (timer reset)
    /// or not (timer below or past 0.5 s; velocity along the rail or
    /// rebuilt); receding from or moving into the wall (small or large
    /// bounce). The hull, contact query, translate, matrix and impact
    /// callees are gated fakes; the controller method is a fake that may
    /// unwind.
    #[test]
    fn rider_railslide_state_collide_walls_matches() {
        use crate::difftest::put_vec;
        rrs_install_fakes();
        let f = |x: f32| x.to_bits();
        check(
            ee::s01::f_0013c140__generated,
            rider_railslide_state_collide_walls,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13c140, 1);
                c.pc = 0x0013_c140;
                let this = DATA;
                let rider = DATA + 0x1000;
                let sub = DATA + 0x3000;
                c.bus.write32(RRS_CTRL, RRS_MAGIC);
                for (off, v) in [
                    (RRS_GP_HEAD_ON, -0.9999f32),
                    (RRS_GP_PUSH_OUT, 1.1),
                    (RRS_GP_TIME_STEP, 1.0 / 60.0),
                    (RRS_GP_ALONG_MIN, 0.3),
                    (RRS_GP_ALONG_SCALE, 20.0),
                    (RRS_GP_MIN_BOUNCE, 5.0),
                ] {
                    c.bus.write32(GAME_GP - off, f(v));
                }
                c.g[A0][0] = sx(this);
                c.bus.write32(this + RRS_RIDER, rider);
                c.bus.write32(rider + 0x77c, sub);
                c.bus.write32(
                    sub + 0xde4,
                    if seed % 11 == 0 { 9 } else { r.next_u32() % 8 },
                );
                let vt = vtable(c, &mut r, &[0x40]);
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write32(rider + 0xaa0, r.next_u32());
                c.bus.write32(rider + 0x860, r.next_u32());
                c.bus.write32(rider + 0x868, r.next_u32());
                let unit = |r: &mut Rng| {
                    let v = [
                        (r.next_u32() % 200) as f32 - 100.0,
                        (r.next_u32() % 200) as f32 - 100.0,
                        (r.next_u32() % 200) as f32 - 99.5,
                    ];
                    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                    [v[0] / n, v[1] / n, v[2] / n]
                };
                let fwd = unit(&mut r);
                let normal = if seed % 7 == 3 {
                    [-fwd[0], -fwd[1], -fwd[2]]
                } else {
                    unit(&mut r)
                };
                put_vec(
                    c,
                    rider + RRS_R_FORWARD,
                    [f(fwd[0]), f(fwd[1]), f(fwd[2]), 0],
                );
                put_vec(
                    c,
                    RRS_RECORD + 0x10,
                    [f(normal[0]), f(normal[1]), f(normal[2]), 0],
                );
                put_vec(
                    c,
                    RRS_RECORD,
                    [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)],
                );
                for k in 0x20..0x60 {
                    c.bus.write8(RRS_RECORD + k, r.next_u32() as u8);
                }
                let depth = if seed % 6 == 2 {
                    -0.5 - (r.next_u32() % 4) as f32
                } else {
                    (r.next_u32() % 10) as f32 * 0.25
                };
                c.bus.write32(RRS_CTRL + 8, f(depth));
                // Rail and rider positions: within 2 units or not.
                let pos = [r.f32(50.0), r.f32(50.0), r.f32(50.0), f(1.0)];
                put_vec(c, rider + RRS_R_POS, pos);
                let off = if seed % 3 == 0 { 0.5f32 } else { 5.0 };
                let rail: Vec<u32> = (0..3)
                    .map(|i| f(f32::from_bits(pos[i]) + off * normal[i]))
                    .chain([f(1.0)])
                    .collect();
                put_vec(c, this + RRS_RAIL_POS, [rail[0], rail[1], rail[2], rail[3]]);
                c.bus.write32(
                    this + RRS_OFF_RAIL_TIME,
                    f(if seed % 4 < 2 { 0.1 } else { 0.6 }),
                );
                let along = if seed % 5 < 2 { normal } else { unit(&mut r) };
                put_vec(
                    c,
                    rider + RRS_R_RAIL_DIR,
                    [f(along[0]), f(along[1]), f(along[2]), 0],
                );
                // Velocity into the wall (bounce) or away from it.
                let k = (1 + r.next_u32() % 40) as f32 * if seed % 2 == 0 { -1.0 } else { 1.0 };
                put_vec(
                    c,
                    rider + RRS_R_VEL,
                    [
                        f(normal[0] * k + 1.0),
                        f(normal[1] * k),
                        f(normal[2] * k),
                        0,
                    ],
                );
            },
        );
    }

    /// HUD_DrawTextLerp: two random elements, an empty string (the real
    /// MeasureText), with or without a scale multiplier, position offset,
    /// rectangle and colour, alignments given or 0x7f (the elements'), t in
    /// [0, 1]; the real HUD element getters, AlignElemPos and DrawText (its
    /// renderer method a fake).
    #[test]
    fn hud_draw_text_lerp_matches() {
        check(
            ee::s04::f_001f1e28__generated,
            hud_draw_text_lerp,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f1e28, 2);
                c.pc = 0x001f_1e28;
                let ctx = DATA;
                let font = DATA + 0x1000;
                let defs = DATA + 0x2000;
                let text = DATA + 0x3000;
                let mul = DATA + 0x3100;
                let off = DATA + 0x3200;
                let colour = DATA + 0x3300;
                let rect = DATA + 0x3400;
                let caller = crate::difftest::SP;
                for k in 0..0x40u32 {
                    c.bus.write32(ctx + 4 * k, r.f32(100.0));
                }
                for k in 0..0x20u32 {
                    c.bus.write32(font + 4 * k, r.f32(4.0));
                }
                for i in 0..27u32 {
                    crate::game::fe::ovstatehud::tests::rider_ind_elem(c, &mut r, defs + 0x24 * i);
                }
                let (i1, i2) = (r.next_u32() % 6, r.next_u32() % 6);
                c.g[A0][0] = sx(ctx);
                c.g[A1][0] = sx(font);
                c.g[A2][0] = sx(defs);
                c.g[A3][0] = sx(i1);
                c.g[T0][0] = sx(i2);
                c.g[T1][0] = sx(text);
                c.g[T2][0] = sx(r.next_u32() % 3);
                c.g[T3][0] = if seed % 3 == 0 { 0 } else { sx(mul) };
                c.f[12] = (((r.next_u32() % 1001) as f32) / 1000.0).to_bits();
                c.bus.write32(mul, r.f32(2.0));
                c.bus.write32(mul + 4, r.f32(2.0));
                c.bus.write32(off, r.f32(50.0));
                c.bus.write32(off + 4, r.f32(50.0));
                for k in 0..4 {
                    c.bus.write32(colour + 4 * k, r.f32(1.0));
                }
                // The stack arguments.
                c.bus.write32(caller, if seed % 4 == 0 { 0 } else { off });
                c.bus
                    .write32(caller + 8, if seed % 2 == 0 { 0 } else { colour });
                c.bus
                    .write32(caller + 0x10, if seed % 5 == 0 { 0 } else { rect });
                let align = |r: &mut Rng, s: u32| if s % 3 == 0 { 0x7f } else { r.next_u32() % 3 };
                let a = align(&mut r, seed / 2);
                let v = align(&mut r, seed / 3);
                c.bus.write32(caller + 0x18, a);
                c.bus.write32(caller + 0x20, v);
                // Renderers: the draw state (gp-0x854) and the font's (gp+0x2a90).
                let renderer = DATA + 0x6000;
                c.bus.write32(GAME_GP.wrapping_sub(0x854), renderer);
                c.bus.write32(GAME_GP + 0x2a90, renderer);
                c.bus.write32(renderer + 0xe84, DATA + 0x7800);
                c.bus.write32(DATA + 0x7800 + 8, r.next_u32());
                let vt = vtable(c, &mut r, &[0x288]);
                c.bus.write32(renderer + 0x10d8, vt);
            },
        );
    }

    /// Gate and answers of [`net_fake_mem_alloc`].
    pub(crate) const NET_CTRL: u32 = DATA + 0xd_8000;
    pub(crate) const NET_MAGIC: u32 = 0x0026_3828;

    /// `EA_MemAllocNamed` (0x317d70) for the net tests: logs the size, tag
    /// and `ra`, returns the n-th block from `NET_CTRL + 0x10`.
    fn net_fake_mem_alloc(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(NET_CTRL) != NET_MAGIC {
            return fall_back(
                c,
                prev_cell(net_fake_mem_alloc),
                ee::s10::f_00317d70__generated,
            );
        }
        let n = c.bus.read32(NET_CTRL + 4) & 3;
        c.bus.write32(NET_CTRL + 4, n + 1);
        let log = NET_CTRL + 0x40 + 0x10 * n;
        c.bus.write32(log, c.g[A0][0] as u32);
        c.bus.write32(log + 4, c.g[A1][0] as u32);
        c.bus.write32(log + 8, c.g[A2][0] as u32);
        c.bus.write32(log + 0xc, c.g[RA][0] as u32);
        c.g[V0][0] = c.bus.read32(NET_CTRL + 0x10 + 4 * n) as u64;
        c.cycles += 11;
        ret(c);
    }

    /// Net_UpdateConnect: idle, connecting (the comm system up: the message
    /// post through a fake allocator; or still waiting: the timer counts
    /// down), connected (no failure) and unknown states. Not reached: the
    /// three disconnect paths (comm failure, timeout): the real
    /// Net_DisconnectAll runs the whole lobby shutdown.
    #[test]
    fn net_update_connect_matches() {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            net_fake_mem_alloc,
            prev_cell(net_fake_mem_alloc),
        );
        check(
            ee::s07::f_00263828__generated,
            net_update_connect,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x263828, 0);
                c.pc = 0x0026_3828;
                let this = DATA;
                let comm = DATA + 0x100;
                let lobby = DATA + 0x200;
                let head = DATA + 0x300;
                let prev = DATA + 0x400;
                let app = DATA + 0x700;
                c.bus.write32(NET_CTRL, NET_MAGIC);
                c.bus.write32(NET_CTRL + 0x10, DATA + 0x500);
                c.bus.write32(NET_CTRL + 0x14, DATA + 0x600);
                c.g[A0][0] = sx(this);
                let state = [0u32, 1, 1, 2, 3, 0xffff_ffff][(seed % 6) as usize];
                c.bus.write32(this + NET_CONNECT_STATE, state);
                c.bus.write32(
                    this + NET_CONNECT_TIMER,
                    (0.01 + (r.next_u32() % 100) as f32).to_bits(),
                );
                c.bus.write32(GAME_GP - NET_COMM_SYSTEM_GP, comm);
                let comm_state = if seed % 3 == 0 {
                    5
                } else {
                    [0u32, 2, 4, 7][(seed / 3 % 4) as usize]
                };
                c.bus.write32(comm + 0x20, comm_state);
                c.bus.write32(comm + 0x28, r.next_u32());
                c.bus.write32(GAME_GP - NET_LOBBY_MAN_GP, lobby);
                c.bus.write32(lobby + 0xf0, head);
                c.bus.write32(head, r.next_u32());
                c.bus.write32(head + 4, prev);
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x14, r.f32(0.1));
            },
        );
    }
}
