//! Phase 2 wave 55 (oc160): typed replacements for hot generated functions,
//! each with a differential test against the generated body.
//!
//! The ports below are literal translations of `generated/ee`: the same
//! instructions in the same order, the same block cycle charges, the same
//! register results (including scratch values left in `v0`/`v1`/`t*` and the
//! FPU file), the same load/store widths, and the same call protocol.  Every
//! call goes through the function table ([`call_ptr`]) exactly as the
//! generated body's `c.pc = addr; call_indirect(..)` does, so a replaced
//! callee is reached through its override slot.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_d710,
        render_toggles_menu_dtor,
        "RenderTogglesMenu_dtor",
    ),
    (0x002e_bc40, transition_wipe_draw, "TransitionWipe_Draw"),(0x002e_8560, c_board_trail_fx_reset, "cBoardTrailFX_Reset"),(0x003a_7598, world_block_pool_init, "WorldBlockPool_Init"),
    (
        0x002c_fd28,
        menu_sys_group_item_draw,
        "MenuSysGroupItem_Draw",
    ),
    (
        0x0035_1660,
        intersect_info_eval_derivative,
        "IntersectInfo_EvalDerivative",
    ),
    (
        0x001c_8758,
        c_fe_popup_confirm_layout_menu,
        "cFEPopupConfirm_LayoutMenu",
    ),(
        0x0039_f4c0,
        ui_state_mgr_switch_state_close_popups,
        "UIStateMgr_SwitchStateClosePopups",
    ),];

/// `RenderTogglesMenu_dtor` (0x0024d710): destroy the debug render-toggles
/// menu: run the destructor (flags 2) of its 49 `MenuSysItem` sub-objects
/// -- laid out from `+0xa18` down to `+0x130`, 0x30 bytes apart with a few
/// 0x34 gaps -- and then of the menu object itself.
pub fn render_toggles_menu_dtor(c: &mut Cpu) {
    /// The 49 item sub-objects, in destruction order.
    const ITEMS: [u32; 49] = [
        0xa18, 0x9e8, 0x9b8, 0x988, 0x958, 0x924, 0x8f0, 0x8bc, 0x888, 0x858, 0x828, 0x7f8, 0x7c4,
        0x794, 0x764, 0x734, 0x704, 0x6d4, 0x6a4, 0x674, 0x644, 0x614, 0x5e4, 0x5b4, 0x584, 0x554,
        0x524, 0x4f4, 0x4c4, 0x494, 0x464, 0x434, 0x404, 0x3d4, 0x3a4, 0x374, 0x344, 0x314, 0x2e4,
        0x2b4, 0x284, 0x254, 0x224, 0x1f4, 0x1c4, 0x194, 0x164, 0x14c, 0x130,
    ];
    guest(c, |c| {
        c.cycles += 161;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let menu = c.g[S0][0];
        for (i, off) in ITEMS.iter().enumerate() {
            c.g[A0][0] = sx((menu as u32).wrapping_add(*off));
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002c_a280, 0x0024_d734 + 0xc * i as u32)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x002c_aa80, 0x0024_d980)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `TransitionWipe_Draw` (0x002ebc40): draw the screen-wipe quad of a
/// transition effect.  `t` (f12) is the wipe progress, `self+0x14`/`self+0x18`
/// the wipe direction/size in pixels; the quad's screen rectangle is derived
/// from 40.0 and -480.0 and its texture coordinates from the globals at
/// `gp+0xa40..0xa54`, then submitted with the effect colour at `self+4`.
pub fn transition_wipe_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        let self_ = c.g[A0][0] as u32;
        let gp = c.g[GP][0] as u32;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1); // f1 = 1.0
        frame.save_ra(c, 0x20);
        c.sub_s(1, 1, 12); // f1 = 1.0 - t
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 3); // f3 = 40.0
        c.lwc1(2, self_.wrapping_add(0x14));
        c.cvt_s_w(2, 2); // f2 = (float)dx
        c.g[AT][0] = 0xffff_ffff_c3f0_0000;
        c.mtc1(1, 4); // f4 = -480.0
        c.lwc1(0, self_.wrapping_add(0x18));
        c.cvt_s_w(0, 0); // f0 = (float)dy
        c.mov_s(8, 3); // f8 = 40.0
        c.mtc1(0, 7); // f7 = (float)dy
        c.mul_s(2, 2, 1); // f2 = dx * k
        c.mul_s(0, 0, 1); // f0 = dy * k
        c.mul_s(5, 2, 3); // f5 = dx * k * 40.0
        c.c_lt_s(5, 7);
        let dx_left = !c.fcc();
        c.mul_s(6, 0, 4); // f6 = dy * k * -480.0
        if dx_left {
            c.cycles += 7;
            c.lwc1(0, gp.wrapping_add(0xa48));
            c.sub_s(8, 8, 5); // f8 = 40.0 - dx
            c.lwc1(1, gp.wrapping_sub(0x39a0));
            c.lwc1(7, gp.wrapping_add(0xa40));
            c.mul_s(0, 0, 1);
            c.mul_s(0, 0, 8);
            c.add_s(10, 7, 0); // u1 = base + step * f8
        } else {
            c.cycles += 10;
            c.lwc1(0, gp.wrapping_add(0xa48));
            c.add_s(8, 5, 8); // f8 = dx + 40.0
            c.lwc1(1, gp.wrapping_sub(0x39a4));
            c.lwc1(2, gp.wrapping_add(0xa40));
            c.mul_s(0, 0, 1);
            c.lwc1(10, gp.wrapping_add(0xa44));
            c.mul_s(0, 0, 5);
            c.mov_s(5, 7);
            c.add_s(7, 2, 0); // u0 = base + step * f5
        }
        c.cycles += 5;
        c.mtc1(0, 9); // f9 = 0.0
        c.c_lt_s(6, 9);
        let dy_top = !c.fcc();
        c.lwc1(0, gp.wrapping_add(0xa54));
        if dy_top {
            c.cycles += 9;
            c.lwc1(2, gp.wrapping_sub(0x3998));
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(1, 0); // f0 = 480.0
            c.lwc1(1, gp.wrapping_add(0xa54));
            c.sub_s(4, 0, 6); // f4 = 480.0 - dy
            c.lwc1(3, gp.wrapping_add(0xa4c));
            c.mul_s(1, 1, 2);
            c.mul_s(1, 1, 4);
            c.add_s(2, 3, 1); // v1 = base + step * f4
        } else {
            c.cycles += 11;
            c.lwc1(1, gp.wrapping_sub(0x399c));
            c.g[AT][0] = 0x43f0_0000;
            c.mtc1(1, 2); // f2 = 480.0
            c.mul_s(0, 0, 1);
            c.lwc1(3, gp.wrapping_add(0xa4c));
            c.add_s(4, 6, 2); // f4 = dy + 480.0
            c.lwc1(2, gp.wrapping_add(0xa50));
            c.mul_s(0, 0, 6);
            c.mov_s(6, 9); // dy = 0.0
            c.add_s(3, 3, 0); // v0 = base + step * f6
        }
        c.cycles += 18;
        c.sub_s(1, 10, 7); // uw = u1 - u0
        c.g[A2][0] = sx(self_.wrapping_add(0x4));
        c.sub_s(0, 2, 3); // vh = v1 - v0
        c.swc1(5, c.g[SP][0] as u32);
        c.mtc1(0, 12); // f12 = 0.0
        c.g[A0][0] = c.g[SP][0];
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[RA][0] = sx(0x002e_bd7c);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
        call_ptr(c, 0x002e_b938, 0x002e_bd7c)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `cBoardTrailFX_Reset` (0x002e8560): reinitialise a board-trail effect:
/// clear the fade/alpha words, install the two default colours, and fill the
/// 54 trail vertices with the template colour at `gp+0x14c0..0x14c8`.
pub fn c_board_trail_fx_reset(c: &mut Cpu) {
    c.cycles += 28;
    let this = c.g[A0][0];
    let gp = c.g[GP][0] as u32;
    c.g[T1][0] = this;
    c.g[V0][0] = 0x2;
    c.sw((this as u32).wrapping_add(0x94), c.g[V0][0]);
    c.g[A1][0] = 0x50_0000;
    c.sw((this as u32).wrapping_add(0x90), 0u64);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xed0));
    c.sw((this as u32).wrapping_add(0x8c), 0u64);
    c.g[A0][0] = 0x50_0000;
    c.sw((this as u32).wrapping_add(0x98), 0u64);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xee0));
    c.sw((this as u32).wrapping_add(0x9c), 0u64);
    c.g[T3][0] = 0u64;
    c.sw((this as u32).wrapping_add(0xa0), 0u64);
    c.g[T2][0] = 0u64;
    c.sw((this as u32).wrapping_add(0xa4), 0u64);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0); // f0 = 1.0
    c.lq(3, c.g[A1][0] as u32); // the default colour quad ...
    c.mov_s(3, 0); // ... with alpha forced to 1.0
    c.sq(3, (this as u32).wrapping_add(0xb0));
    c.lq(2, c.g[A0][0] as u32);
    c.sq(2, (this as u32).wrapping_add(0xc0));
    c.lq(3, c.g[A0][0] as u32);
    c.swc1(0, (this as u32).wrapping_add(0x4));
    c.sq(3, (this as u32).wrapping_add(0xd0));
    c.swc1(0, (this as u32).wrapping_add(0x8));
    c.sw((this as u32).wrapping_add(0xe0), 0u64);
    let this = c.g[T1][0] as u32;
    loop {
        c.cycles += 70;
        c.g[V0][0] = c.lw(this.wrapping_add(0x18));
        c.g[V1][0] = c.g[T3][0] & 0x1;
        c.lwc1(0, gp.wrapping_add(0x14c8));
        c.mtc1(3, 2); // f2 = (int) the colour template's first word
        c.cvt_s_w(2, 2);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
        c.swc1(0, c.g[V0][0] as u32);
        c.g[T0][0] = ((c.g[T3][0] as i64) < 54i64) as u64;
        c.g[V0][0] = c.lw(this.wrapping_add(0x10));
        c.lwc1(0, gp.wrapping_add(0x14c4));
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(0, c.g[V0][0] as u32);
        c.g[V1][0] = c.lw(this.wrapping_add(0x1c));
        c.lwc1(1, gp.wrapping_add(0x14c0));
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.swc1(1, c.g[V1][0] as u32);
        c.lwc1(0, gp.wrapping_add(0x14c0));
        c.g[V0][0] = c.lw(this.wrapping_add(0x20));
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(0, c.g[V0][0] as u32);
        c.lwc1(1, gp.wrapping_add(0x14c4));
        c.g[V0][0] = c.lw(this.wrapping_add(0x14));
        c.sub_s(1, 3, 1);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(1, c.g[V0][0] as u32);
        c.lwc1(0, gp.wrapping_add(0x14c8));
        c.g[V0][0] = c.lw(this.wrapping_add(0x24));
        c.sub_s(0, 3, 0);
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(0, c.g[V0][0] as u32);
        // Second channel: the same six words plus 4, at 1.0 / 1.0 / 1.0.
        c.g[A0][0] = c.lw(this.wrapping_add(0x24));
        c.g[A1][0] = c.lw(this.wrapping_add(0x14));
        c.g[A3][0] = c.lw(this.wrapping_add(0x18));
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A2][0] = c.lw(this.wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = c.lw(this.wrapping_add(0x1c));
        c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = c.lw(this.wrapping_add(0x20));
        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
        c.swc1(2, (c.g[A2][0] as u32).wrapping_add(0x4));
        c.swc1(2, (c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(this.wrapping_add(0x24));
        c.g[A1][0] = c.lw(this.wrapping_add(0x14));
        c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A3][0] = c.lw(this.wrapping_add(0x18));
        c.g[A2][0] = c.lw(this.wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.lw(this.wrapping_add(0x1c));
        c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = c.lw(this.wrapping_add(0x20));
        c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
        c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x8));
        c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x8));
        let more = c.g[T0][0] != 0;
        c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0x8));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002e_85d0);
            }
            continue;
        }
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
        return;
    }
}

/// `PS2GraphicsMan_FindDisplayMode` (0x003762f8): find the index of the
/// display-mode texture matching `(width, height, bpp, clutBpp, fmt, id)` in
/// the manager's texture list (`self+0x59c4` count, `self+0x59cc` array of
/// 0xa4-byte records), or -1.  Each of the three format fields must be
/// supported by the entry: bpp selects `psm` (+0x90), clutBpp selects
/// `clutPsm` (+0x94) and fmt 24 selects `fmt` (+0x98).
pub fn ps2_graphics_man_find_display_mode(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    let this = c.g[A0][0] as u32;
    c.g[T4][0] = 0u64;
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[T1][0]; // fmt
    frame.save(c, S2, 0x0);
    c.g[S1][0] = c.g[A2][0]; // height
    c.g[S2][0] = c.g[A1][0]; // width
    c.g[V0][0] = c.lw(this.wrapping_add(0x59c4));
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = c.lw(this.wrapping_add(0x59cc));
    // `i` in T4, the current record in V1, the record count in T1.
    if empty {
        c.cycles += 1;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    } else {
        c.cycles += 7;
        c.g[T1][0] = c.g[V0][0];
        c.g[T7][0] = 0x18;
        c.g[A2][0] = (c.g[A3][0] < 0x19u64) as u64; // bpp < 25
        c.g[T9][0] = 0x10;
        c.g[T8][0] = 0x20;
        c.g[T5][0] = 0x1;
        c.g[A1][0] = (c.g[T0][0] < 0x19u64) as u64; // clutBpp < 25
        loop {
            let rec = c.g[V1][0] as u32;
            c.cycles += 3;
            c.g[V0][0] = c.lw(rec.wrapping_add(0x88)); // width
            if c.g[V0][0] != c.g[S2][0] {
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(rec.wrapping_add(0x8c)); // height
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(rec.wrapping_add(0xa0)); // id
                    if c.g[V0][0] != c.g[T2][0] {
                        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                    } else {
                        // ok1: the entry's pixel format supports `bpp`.
                        c.cycles += 4;
                        c.g[T3][0] = 0u64;
                        c.g[A0][0] = 0u64;
                        let fmt_24 = c.g[A3][0] == c.g[T7][0];
                        c.g[T6][0] = 0u64;
                        'bpp: {
                            if fmt_24 {
                                c.cycles += 2;
                                c.g[T3][0] = c.g[T5][0];
                                if c.g[V0][0] != 0 {
                                    c.g[T3][0] = 0;
                                }
                                break 'bpp;
                            }
                            if c.g[A2][0] != 0 {
                                c.cycles += 2;
                                c.cycles += 2;
                                if c.g[A3][0] == c.g[T9][0] {
                                    c.g[V0][0] = c.lw(rec.wrapping_add(0x90));
                                    c.cycles += 3;
                                    c.g[T3][0] = c.g[T5][0];
                                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                    if c.g[V0][0] != 0 {
                                        c.g[T3][0] = 0;
                                    }
                                    c.cycles += 2;
                                    c.g[T3][0] = c.g[T5][0];
                                    if c.g[V0][0] != 0 {
                                        c.g[T3][0] = 0;
                                    }
                                    break 'bpp;
                                }
                                c.cycles += 2;
                                break 'bpp;
                            }
                            c.cycles += 2;
                            c.cycles += 2;
                            if c.g[A3][0] == c.g[T8][0] {
                                c.g[V0][0] = c.lw(rec.wrapping_add(0x90));
                                c.cycles += 4;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                if c.g[V0][0] == 0 {
                                    c.g[T3][0] = c.g[T5][0];
                                }
                            } else {
                                c.cycles += 2;
                            }
                        }
                        // ok2: the entry's CLUT format supports `clutBpp`.
                        c.cycles += 2;
                        let clut_24 = c.g[T0][0] == c.g[T7][0];
                        if clut_24 {
                            c.g[V0][0] = c.lw(rec.wrapping_add(0x94));
                            c.cycles += 3;
                            c.g[A0][0] = c.g[T5][0];
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            c.cycles += 1;
                            if c.g[V0][0] != 0 {
                                c.g[A0][0] = 0;
                            }
                        } else if c.g[A1][0] != 0 {
                            c.cycles += 2;
                            c.cycles += 2;
                            if c.g[T0][0] == c.g[T9][0] {
                                c.g[V0][0] = c.lw(rec.wrapping_add(0x94));
                                c.cycles += 3;
                                c.g[A0][0] = c.g[T5][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                c.cycles += 1;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = 0;
                                }
                            } else {
                                c.cycles += 2;
                            }
                        } else {
                            c.cycles += 2;
                            c.cycles += 2;
                            if c.g[T0][0] == c.g[T8][0] {
                                c.g[V0][0] = c.lw(rec.wrapping_add(0x94));
                                c.cycles += 1;
                                c.g[A0][0] = c.g[T5][0];
                                c.cycles += 1;
                                if c.g[V0][0] != 0 {
                                    c.g[A0][0] = 0;
                                }
                            } else {
                                c.cycles += 2;
                            }
                        }
                        // ok3: only fmt 24 constrains the texture format.
                        c.cycles += 2;
                        if c.g[S0][0] == c.g[T7][0] {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(rec.wrapping_add(0x98));
                            c.g[T6][0] = c.g[T5][0];
                            c.g[V0][0] = c.g[V0][0] ^ 0x31;
                            if c.g[V0][0] != 0 {
                                c.g[T6][0] = 0;
                            }
                        }
                        c.cycles += 2;
                        if c.g[T3][0] == 0 {
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 2;
                            if c.g[A0][0] == 0 {
                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                            } else {
                                c.cycles += 2;
                                let found = c.g[T6][0] != 0;
                                c.g[V0][0] = c.g[T4][0];
                                if found {
                                    // Return the index.
                                    c.cycles += 5;
                                    frame.restore(c, S0, 0x20);
                                    frame.restore(c, S1, 0x10);
                                    frame.restore(c, S2, 0x0);
                                    frame.restore_ra(c, 0);
                                    frame.close(c);
                                    return;
                                }
                                c.cycles += 1;
                                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
                            }
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[T1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xa4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_6340);
            }
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore(c, S2, 0x0);
    frame.restore_ra(c, 0);
    frame.close(c);
}
/// `Bolts_CollectByGroup` (0x0014d7e8): recursively collect the bolt nodes of
/// group `g` whose `parent` equals `key` into `out`, returning how many were
/// written.  The group's node range comes from `self+0xa4+4g` (first) and
/// `self+0x2c+4g` (count) over the 0x38-byte nodes at `*(self+4)`; a matching
/// node is also descended into when `depth != 0` and it is either not
/// `0x20`-flagged or `all` is set.
pub fn bolts_collect_by_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S7, 0x20);
        frame.save(c, S6, 0x30);
        c.g[S7][0] = c.g[A1][0]; // g
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x80);
        c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S4, 0x50);
        c.g[A0][0] = 0x38; // node stride
        frame.save(c, S5, 0x40);
        c.g[V0][0] = c.g[V1][0];
        frame.save(c, FP, 0x10);
        c.g[S1][0] = c.g[A3][0]; // out
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[A2][0]; // key
        frame.save(c, S2, 0x70);
        c.g[S5][0] = c.g[T0][0]; // depth
        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4)); // first
        c.g[S4][0] = c.g[T1][0]; // all
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c)); // count
        c.g[S3][0] = 0u64; // collected
        c.mult(0, A0, S2, A0); // first * 0x38
        c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4)); // nodes
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32)); // end
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32)); // remaining
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != c.g[FP][0] {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                } else {
                    c.cycles += 4;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    let leaf = c.g[S5][0] == 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    let mut stop = false;
                    if !leaf {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        c.g[V0][0] = c.g[V0][0] & 0x20;
                        if c.g[V0][0] == 0 {
                            c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                        } else {
                            c.cycles += 2;
                            if c.g[S4][0] == 0 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                                stop = true;
                            } else {
                                c.cycles += 1;
                                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                            }
                        }
                    }
                    if !stop {
                        c.cycles += 9;
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S6][0];
                        c.g[A1][0] = c.g[S7][0];
                        c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                        c.g[T1][0] = c.g[S4][0];
                        call_ptr(c, 0x0014_d7e8, 0x0014_d8b8)?;
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    } else {
                        // The child was skipped: the caller's loop continues.
                    }
                }
                c.cycles += 2;
                let more = c.g[S2][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_d868);
                }
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
/// `AnimDataBase_AllocChanCache` (0x00310f18): build the animation channel
/// cache of bank `bank`: allocate 20 `sAnimNode_310F18` (0x24 bytes each),
/// link them into a circular list (`next` at +0x14, `prev` at +0x1c), give
/// each node a freshly allocated array of `n` 0x14-byte `sAnimElem_310F18`
/// (+0x4), and publish the list in `D_004FE7A0[bank]`/`D_004FE860[bank]`.
pub fn anim_data_base_alloc_chan_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x50_0000;
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 0x2000_0000;
        c.g[S1][0] = c.g[A1][0]; // n
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[A0][0] as u32) << 2); // bank * 4
        frame.save(c, S5, 0x30);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S2, 0x20);
        c.g[S7][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1860)); // 0x004fe7a0
        c.g[S5][0] = c.g[S0][0];
        c.g[S6][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save(c, S4, 0x40);
        c.g[A0][0] = 0x2d0; // 20 * 0x24
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x62b0)); // 0x00489d50
        c.g[A3][0] = 0u64;
        c.g[S2][0] = c.g[S0][0];
        call_ptr(c, 0x0031_7e30, 0x0031_0f7c)?;
        c.g[S3][0] = 0x13;
        c.g[V1][0] = 0x14;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.mult(0, S4, S1, V1); // n * 0x14
        c.g[S1][0] = 0u64;
        loop {
            c.cycles += 22;
            c.g[T0][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x62b0));
            c.g[A2][0] = 0x2000_0000;
            c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[A3][0] = 0u64;
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x24));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            c.sw((c.g[T0][0] as u32).wrapping_add(0x20), c.g[V0][0]); // node[i].next
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x24));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]); // node[i].prev
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64); // node[i].used = 0
            c.g[S0][0] = c.lw(c.g[S2][0] as u32);
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            call_ptr(c, 0x0031_7e30, 0x0031_0fdc)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
            let more = (c.g[S3][0] as i64) >= 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]); // node[i].data
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0031_0f90);
            }
        }
        c.cycles += 23;
        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1860));
        c.g[A2][0] = 0x50_0000;
        c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x17a0));
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A2][0] as u32));
        frame.restore(c, S0, 0x80);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ac)); // node[19]
        frame.restore(c, S1, 0x70);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        frame.restore(c, S2, 0x60);
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        frame.restore(c, S3, 0x50);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
        frame.restore(c, S4, 0x40);
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        c.sw(c.g[A2][0] as u32, c.g[V0][0]);
        frame.close(c);
        Ok(())
    });
}

/// `WorldBlockPool_Init` (0x003a7598): initialise a world-block pool:
/// `self+0` = block size, `self+4`/`self+8` = block count, `self+0xc` = the
/// block array (`count * size` bytes), then link every block to the next
/// (`+0x14`) with `+0x4` = the block size, `+0x8`/`+0xc`/`+0x10` cleared and
/// `+0` = a freshly allocated per-block payload, and finally store the array
/// in `self+0x10`.
pub fn world_block_pool_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.g[A1][0]; // count
        frame.save(c, S1, 0x50);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A0][0]; // this
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x18; // block size
        frame.save(c, S2, 0x40);
        c.mult(0, 0, 2, 4); // blocks * 0x18
        frame.save(c, S4, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4dc0));
        frame.save(c, S5, 0x10);
        c.g[A3][0] = 0u64;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0u64;
        c.sw(c.g[S1][0] as u32, c.g[A2][0]);
        c.g[A0][0] = c.lo[0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A2][0] = 0x2000_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x003a_75f0)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = c.g[V1][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        if !empty {
            c.cycles += 3;
            c.g[S5][0] = 0x49_0000;
            c.g[S4][0] = 0x18;
            c.g[S2][0] = 0u64;
            loop {
                c.cycles += 35;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4dd8));
                c.g[A2][0] = 0x2500_0000;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                call_ptr(c, 0x0031_7e30, 0x003a_7658)?;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_7608);
                }
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        frame.restore(c, S0, 0x60);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
/// `MenuSysGroupItem_Draw` (0x002cfd28): lay out and draw the rows of a
/// grouped menu list.  Starting from the rect at `a1`, each visible item
/// (`MenuSysItem_IsVisible`) gets a 0x10-byte rect at `self+0x14c+i*16` --
/// `x` = pos.x + (item == current ? 0 : 20), `y` = the running pos.y,
/// `w` = pos.w - the same offset -- and is drawn through its vtable slot
/// `+0x2c` (with the `+0x28` this-adjustment); `pos.y` then advances by the
/// rect's height (`+0x158`).  The selected item is either the group's
/// current item or every item while `self` is its page's selected item.
pub fn menu_sys_group_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S1, 0x70);
        frame.save(c, S4, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[S4][0] = 0u64;
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18)); // count
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, c.g[SP][0] as u32);
        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        let no_rows = (c.g[V0][0] as i64) <= 0;
        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        if !no_rows {
            c.cycles += 5;
            c.g[S6][0] = 0x14; // indent of a non-selected row
            c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14c)); // rects
            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24)); // items
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x154));
            loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002c_fd98);
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                call_ptr(c, 0x002c_a378, 0x002c_fd98)?;
                let mut draw = false;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002c_fda8);
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    call_ptr(c, 0x002c_bf30, 0x002c_fda8)?;
                    if c.g[S1][0] == c.g[V0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        draw = true;
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            draw = true;
                        }
                    }
                }
                if draw {
                    c.cycles += 28;
                    c.g[V1][0] = c.g[S6][0];
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[A1][0] = c.g[S5][0];
                    c.lwc1(3, c.g[SP][0] as u32);
                    c.g[S0][0] = sx((c.g[S4][0] as u32) << 4);
                    c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0u64;
                    }
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.mtc1(3, 1); // f1 = (float)off
                    c.cvt_s_w(1, 1);
                    c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x4));
                    c.sub_s(2, 2, 1);
                    c.add_s(3, 3, 1);
                    c.swc1(2, c.g[S3][0] as u32);
                    c.swc1(3, (c.g[S3][0] as u32).wrapping_sub(0x8));
                    c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x002c_fe1c)?;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x158));
                    c.add_s(0, 0, 1);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                }
                c.cycles += 6;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_fd90);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `IntersectInfo_EvalDerivative` (0x00351660): evaluate the 16 parametric
/// curve segments of the intersect info at `t` (f12) and store the velocity
/// in `*(a1)` and the (negated) rotational velocity in `*(a2)`.  For every
/// bit set in the curve mask (`*(*(self+0x40)+0x18)`) the segment selected by
/// the current key (`*(self+i*4)`) is used, re-found with
/// `IntersectInfo_FindKeySegment` when `t` left it; the rest stay 0.
pub fn intersect_info_eval_derivative(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xe0);
        frame.save(c, S1, 0xd0);
        c.g[S0][0] = c.g[SP][0];
        frame.save(c, S3, 0xb0);
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x90);
        c.g[S1][0] = c.g[S3][0];
        frame.save(c, S6, 0x80);
        c.g[S5][0] = 0x1; // curve mask
        frame.save(c, S7, 0x70);
        c.g[S6][0] = 0x18;
        frame.save(c, FP, 0x60);
        c.g[S7][0] = c.g[A1][0]; // vel out
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
        c.g[FP][0] = c.g[A2][0]; // rotVel out
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S4][0] = 0xf;
        frame.save(c, S2, 0xc0);
        c.mov_s(20, 12); // f20 = t
        frame.save_ra(c, 0x50);
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 21); // f21 = 3.0
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
        c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x20)); // curves
        loop {
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
            let off = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(c.g[S2][0] as u32); // curve
            if !off {
                c.cycles += 9;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32); // segment index
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.mult(0, V0, V0, S6);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.c_le_s(0, 20);
                let outside = !c.fcc();
                c.g[A1][0] = 0u64;
                if !outside {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.c_lt_s(20, 0);
                    if c.fcc() {
                        c.g[A1][0] = 0x1;
                    }
                }
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    c.mul_s(2, 20, 21);
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0035_1728);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x0035_1a80, 0x0035_1728)?;
                    c.mul_s(2, 20, 21);
                }
                c.cycles += 13;
                c.lwc1(0, c.g[V0][0] as u32);
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.add_s(1, 1, 1);
                c.mul_s(0, 0, 20);
                c.mul_s(1, 1, 20);
                c.add_s(0, 0, 1);
                c.add_s(0, 0, 3);
                c.swc1(0, c.g[S0][0] as u32);
            }
            if off {
                c.cycles += 1;
                c.sw(c.g[S0][0] as u32, 0u64);
            }
            c.cycles += 4;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            c.g[S5][0] = sx((c.g[S5][0] as u32) << 1);
            let more = (c.g[S4][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_16c8);
            }
        }
        c.cycles += 35;
        c.lwc1(3, c.g[SP][0] as u32);
        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.neg_s(0, 0);
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.neg_s(1, 1);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.neg_s(2, 2);
        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
        frame.restore(c, S0, 0xe0);
        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
        c.sq(3, c.g[S7][0] as u32);
        frame.restore(c, S1, 0xd0);
        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        frame.restore(c, S2, 0xc0);
        c.sq(2, c.g[FP][0] as u32);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    });
}
/// `cFEPopupConfirm_LayoutMenu` (0x001c8758): lay out the popup's menu.  The
/// text widget at `self+0x6c` is centred horizontally ((40 - `self+0x29c`) /
/// 2) and moved by `self+0x288`, the widget at `self+0x88` gets the mirrored
/// placement (+5), and -- when `self+0x2c8` is set -- the five widgets at
/// `self+0x90..` are aligned with `cFEPopupConfirm_SetWidgetAlign`; the final
/// placement is recorded at `self+0x290`/`+0x294`.
pub fn c_fe_popup_confirm_layout_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
        if c.g[V1][0] == 0 {
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        } else {
            c.cycles += 57;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x29c));
            c.g[AT][0] = 0x4420_0000;
            c.mtc1(1, 1); // f1 = 40.0
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x90));
            c.sub_s(1, 1, 0); // f1 = 40.0 - self->29c
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 5); // f5 = 0.5
            c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
            c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(4, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[AT][0] = 0x40a0_0000;
            c.mtc1(1, 4); // f4 = 5.0
            c.mul_s(1, 1, 5); // f1 = (40 - f0) * 0.5
            c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x288));
            c.lwc1(2, c.g[SP][0] as u32);
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1a7));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1a0));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.sub_s(20, 1, 2); // f20 = centring offset
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.add_s(0, 0, 3);
            c.add_s(1, 1, 20);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x27));
            c.sdr(6, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sub_s(0, 0, 20);
            c.add_s(0, 0, 4); // f0 = mirrored placement + 5
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x27));
            c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
            c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c8));
            let no_align = c.g[V0][0] == 0;
            c.mov_s(21, 5); // f21 = 0.5
            if !no_align {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = 0x4;
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                    let absent = c.g[A1][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !absent {
                        c.cycles += 4;
                        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x29c));
                        c.g[A2][0] = 0x12;
                        c.g[RA][0] = sx(0x001c_888c);
                        c.mul_s(12, 12, 21);
                        call_ptr(c, 0x001c_8930, 0x001c_888c)?;
                    }
                    c.cycles += 3;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    let more = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_8870);
                    }
                }
            }
            c.cycles += 4;
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x288));
            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x290));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x294));
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    });
}

/// `cFEPopupConfirm_LayoutBodyText` (0x001c7258): lay out the popup's body
/// text: copy the body rect at `a1` into position (+0x260..+0x26c), add the
/// line-spacing at `self+0x264` to its `y`, and store the result in the text
/// widget `self+0x74`; then, when the optional second widget at `self+0x2cc`
/// exists, re-wrap the text to the width from `cFEPopupConfirm_GetDescLineWrapSize`
/// (`cFEPopupConfirm_ResizeTextWidgets` when the height overflowed), write it
/// with `cFEPopupConfirm_SetWrappedText` and align it with
/// `cFEPopupConfirm_SetWidgetAlign`.
pub fn c_fe_popup_confirm_layout_body_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        let no_widget = c.g[V0][0] == 0;
        if !no_widget {
            c.cycles += 19;
            c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x1bf));
            c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x1b8));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(3, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x264));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.add_s(1, 1, 0);
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(8, c.g[SP][0] as u32);
            c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
            c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[T1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
            let has_second = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            if has_second {
                c.cycles += 19;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x50_0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[V1][0] as u32));
                c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                c.g[RA][0] = sx(0x001c_72fc);
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x001c_93b0, 0x001c_72fc)?;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x260));
                c.c_lt_s(0, 1);
                let overflow = !c.fcc();
                c.g[A1][0] = c.g[S1][0];
                if overflow {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001c_731c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001c_70f0, 0x001c_731c)?;
                }
            }
            c.cycles += 10;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(1, 12); // f12 = 10.0
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x260));
            c.g[RA][0] = sx(0x001c_7338);
            c.g[A3][0] = 0x1;
            call_ptr(c, 0x001c_97e0, 0x001c_7338)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
            let done = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !done {
                c.cycles += 4;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.mtc1(0, 12);
                c.g[RA][0] = sx(0x001c_7354);
                c.g[A2][0] = 0x11;
                call_ptr(c, 0x001c_8930, 0x001c_7354)?;
            }
        }
        c.cycles += 12;
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x267));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x260));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x268));
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x28b));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x284));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x28c), c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}
/// The virtual method call the indicator destructor makes: `obj`'s vtable at
/// `+8`, slot `{delta at +delta_off, fn at +fn_off}`, called with
/// `this = obj + delta` and the caller's `a1` untouched.  Reproduces the
/// generated body's register dance (the target stays in `v1`, the loaded
/// delta in `a0`) and returns the unwind flow.
fn call_indicator_method(c: &mut Cpu, obj: u64, delta_off: u32, fn_off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((obj as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(delta_off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(fn_off));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `OVScreenSession_DestroyIndicators` (0x0020a088): destroy an overlay
/// screen session's indicator objects.  The object at `self+0xe0` gets its
/// `+0x4c` method (arg 0) and then its `+0xc` method (arg 3), and the eight
/// `self+0xe4..`/`self+0x104..` slots are cleared the same way, each pair
/// through the slot object's vtable at `+8`.
pub fn ov_screen_session_destroy_indicators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe0));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0u64;
        if !none {
            c.cycles += 8;
            call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a0c0)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe0));
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x3;
            if !none {
                c.cycles += 5;
                call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a0e0)?;
            }
        }
        c.cycles += 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), 0u64);
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe4));
        c.g[S2][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x104));
        loop {
            c.cycles += 3;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0u64;
            if !none {
                c.cycles += 8;
                call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a110)?;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !none {
                    c.cycles += 5;
                    call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a130)?;
                }
            }
            c.cycles += 1;
            c.sw(c.g[S1][0] as u32, 0u64);
            c.cycles += 3;
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0u64;
            if !none {
                c.cycles += 8;
                call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a154)?;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !none {
                    c.cycles += 5;
                    call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a174)?;
                }
            }
            c.cycles += 1;
            c.sw(c.g[S0][0] as u32, 0u64);
            c.cycles += 5;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_a0f0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `UIStateMgr_SwitchStateClosePopups` (0x0039f4c0): close the popup widgets
/// of a UI state switch.  Starting from the first widget of `a0`'s list
/// (`UIList_First`), every widget still marked visible (bit 0 of `+0x1c`)
/// is walked: its `0x3f`-field is raised to `0x7` when below 6, bit 14 is
/// set, and the loop stops at the first hidden one; finally the transition
/// flags at `a1+0x1c` are updated (bit 8 with bit 5 clear, or `0x220` if a
/// visible widget was seen) and the widget is pushed to the front of `a0`'s
/// list.
pub fn ui_state_mgr_switch_state_close_popups(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_f4e8);
        c.g[S3][0] = 0u64;
        call_ptr(c, 0x0039_7840, 0x0039_f4e8)?;
        c.g[A0][0] = c.g[V0][0];
        let no_list = c.g[A0][0] == 0;
        c.g[S0][0] = 0u64;
        'outer: {
            if no_list {
                break 'outer;
            }
            'walk: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let hidden = c.g[V0][0] == 0;
                if hidden {
                    break 'walk;
                }
                c.cycles += 3;
                c.g[A3][0] = 0xffff_ffff_ffff_c0ff;
                c.g[A2][0] = 0xffff_ffff_ffff_ff7f;
                c.g[A1][0] = 0xffff_ffff_ffff_bfff;
                'lv: loop {
                    'level: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                        c.g[V0][0] = c.g[V0][0] & 0x3f;
                        c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                        let low = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] & c.g[A3][0];
                        if !low {
                            c.cycles += 4;
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                            c.g[V0][0] = c.g[V0][0] | 0x80;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                        }
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] | 0x4000;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        let more = c.g[V0][0] != 0;
                        c.g[S0][0] = 0x1;
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x0039_f510);
                            }
                            continue 'lv;
                        }
                        break 'level;
                    }
                    break 'walk;
                }
            }
            'mark: {
                c.cycles += 2;
                let seen = c.g[S0][0] != 0;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                if seen {
                    break 'mark;
                }
                c.cycles += 4;
                c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let clear = c.g[V0][0] == 0;
                if clear {
                    break 'outer;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let set = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            if set {
                break 'outer;
            }
            c.cycles += 4;
            c.g[V0][0] = c.g[V0][0] & 0x3f;
            c.g[V0][0] = ((c.g[V0][0] as i64) < 6i64) as u64;
            let low = c.g[V0][0] == 0;
            c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
            if low {
                break 'outer;
            }
            c.cycles += 4;
            c.g[S3][0] = 0x1;
            c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
            c.g[V0][0] = c.g[V0][0] | 0x680;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        }
        'flags: {
            'plain: {
                c.cycles += 2;
                let no_seen = c.g[S3][0] == 0;
                c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
                if no_seen {
                    break 'plain;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = 0xffff_ffff_ffff_ffdf;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                break 'flags;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[V0][0] = c.g[V0][0] | 0x220;
            break 'flags;
        }
        c.cycles += 11;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039_f5e0);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_78f0, 0x0039_f5e0)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
/// `cWorldCache_FlushHullPages` (0x003a88a8): flush the world cache's hull
/// pages.  For each used page record at `self+0x3f0` (0x18 bytes, count at
/// `self+0x3ec`) the page's resource pointer (`+8`) is freed through
/// `cWorldCache_FreeHullPage(self, i)`, with the page's "keep" flag decided
/// by its `+4`/`+0x10` fields against the cache limit at
/// `(*(gp+0x2a74))+0x1c`; the page then counts as free (`*page = 0`) or as
/// retired (`*page = 4`).
pub fn c_world_cache_flush_hull_pages(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = 0u64; // i
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
        let none = c.g[V1][0] == 0;
        c.g[S5][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
        if !none {
            c.cycles += 6;
            c.g[S4][0] = 0x1;
            c.g[S7][0] = 0x2;
            c.g[S3][0] = 0x4;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3f0));
            c.g[S6][0] = 0x3;
            loop {
                'page: {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                        break 'page;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    let used = c.g[V0][0] != 0;
                    c.g[A0][0] = 0u64;
                    if !used {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[S4][0];
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = 0;
                        }
                    }
                    'kind: {
                        c.cycles += 2;
                        let kind2 = c.g[V1][0] == c.g[S7][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                        if kind2 {
                            // Kind 2 keeps its page: just the +1 below.
                        } else {
                            'callblk: {
                                'pres: {
                                    'keep: {
                                        'ge3: {
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0 {
                                                c.cycles += 2;
                                                if c.g[V1][0] == c.g[S4][0] {
                                                    break 'keep;
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                                break 'page;
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[S6][0] {
                                            break 'pres;
                                        }
                                        c.cycles += 2;
                                        let four = c.g[V1][0] == c.g[S3][0];
                                        c.g[A0][0] = c.g[S2][0];
                                        if four {
                                            break 'callblk;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                        break 'page;
                                    }
                                    c.cycles += 2;
                                    if c.g[A0][0] == 0 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                                        break 'page;
                                    }
                                    c.cycles += 2;
                                    c.sw(c.g[S0][0] as u32, 0u64);
                                    break 'kind;
                                }
                                c.cycles += 2;
                                let t = c.g[A0][0] == 0;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'kind;
                                }
                                c.cycles += 1;
                                c.sw(c.g[S0][0] as u32, c.g[S3][0]);
                                break 'callblk;
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x003a_8984);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x003a_8528, 0x003a_8984)?;
                            c.sw(c.g[S0][0] as u32, 0u64);
                            break 'kind;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3ec));
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_8900);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
/// The body of `WorldObj_BuildMatrices`' loop: compose bone matrix `mat` with
/// the scale at `sp+0..0xc` (the two paths of the original keep the first two
/// scale components in opposite FPU registers, hence `swap`) and copy the
/// product through the scratch matrices at `sp+0x50`/`sp+0x80`/`sp+0x90` to
/// `out` -- the VU0 ops are issued exactly as the original's COP2 macros.
fn world_obj_bone(c: &mut Cpu, mat: u32, swap: bool) {
    let sp = c.g[SP][0] as u32;
    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
    if swap {
        c.lwc1(6, sp);
        c.lq(2, mat);
        c.lwc1(7, sp.wrapping_add(0x4));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x50));
        c.lwc1(5, sp.wrapping_add(0x8));
        c.lq(2, mat.wrapping_add(0x10));
        c.lwc1(4, sp.wrapping_add(0xc));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x60));
        c.lq(3, mat.wrapping_add(0x20));
        c.sq(3, sp.wrapping_add(0x90));
        c.sq(3, sp.wrapping_add(0x70));
        c.lq(2, mat.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0xa0));
        c.lwc1(1, sp.wrapping_add(0xa0));
        c.lwc1(2, sp.wrapping_add(0xa4));
        c.lwc1(3, sp.wrapping_add(0xa8));
        c.mul_s(1, 1, 6);
        c.lwc1(0, sp.wrapping_add(0xac));
        c.mul_s(2, 2, 7);
    } else {
        c.lwc1(7, sp);
        c.lq(2, mat);
        c.lwc1(6, sp.wrapping_add(0x4));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x50));
        c.lwc1(5, sp.wrapping_add(0x8));
        c.lq(2, mat.wrapping_add(0x10));
        c.lwc1(4, sp.wrapping_add(0xc));
        c.sq(2, sp.wrapping_add(0x90));
        c.sq(2, sp.wrapping_add(0x60));
        c.lq(3, mat.wrapping_add(0x20));
        c.sq(3, sp.wrapping_add(0x90));
        c.sq(3, sp.wrapping_add(0x70));
        c.lq(2, mat.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0xa0));
        c.lwc1(1, sp.wrapping_add(0xa0));
        c.lwc1(2, sp.wrapping_add(0xa4));
        c.lwc1(3, sp.wrapping_add(0xa8));
        c.mul_s(1, 1, 7);
        c.lwc1(0, sp.wrapping_add(0xac));
        c.mul_s(2, 2, 6);
    }
    c.mul_s(3, 3, 5);
    c.mul_s(0, 0, 4);
    c.swc1(1, sp.wrapping_add(0x90));
    c.swc1(2, sp.wrapping_add(0x94));
    c.swc1(3, sp.wrapping_add(0x98));
    c.swc1(0, sp.wrapping_add(0x9c));
    c.lq(2, sp.wrapping_add(0x90));
    c.sq(2, sp.wrapping_add(0x80));
    let src = c.g[A1][0] as u32;
    let out = c.g[T0][0] as u32;
    let m2 = c.g[T5][0] as u32;
    let dst = c.g[T1][0] as u32;
    let dst2 = c.g[T2][0] as u32;
    c.lqc2(4, src);
    c.lqc2(5, src.wrapping_add(0x10));
    c.lqc2(6, src.wrapping_add(0x20));
    c.lqc2(7, src.wrapping_add(0x30));
    c.lqc2(8, m2);
    c.lqc2(9, m2.wrapping_add(0x10));
    c.lqc2(10, m2.wrapping_add(0x20));
    c.lqc2(11, m2.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, out);
    c.sqc2(13, out.wrapping_add(0x10));
    c.sqc2(14, out.wrapping_add(0x20));
    c.sqc2(15, out.wrapping_add(0x30));
    c.lqc2(1, out);
    c.lqc2(2, out.wrapping_add(0x10));
    c.lqc2(3, out.wrapping_add(0x20));
    c.lqc2(4, out.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
    c.lqc2(1, dst);
    c.lqc2(2, dst.wrapping_add(0x10));
    c.lqc2(3, dst.wrapping_add(0x20));
    c.lqc2(4, dst.wrapping_add(0x30));
    c.sqc2(1, dst2);
    c.sqc2(2, dst2.wrapping_add(0x10));
    c.sqc2(3, dst2.wrapping_add(0x20));
    c.sqc2(4, dst2.wrapping_add(0x30));
}

/// `WorldObj_BuildMatrices` (0x00350288): build the world matrices of a
/// world object's skeleton: for every bone (0x10-byte records at
/// `skel+8`, count at `skel+4`) compose the bone matrix with the object's
/// uniform scale (`skel+0x84`) over either its parent's already-built
/// matrix `out[bone->parent]` (`+0`) or the caller's `parent` matrix, and
/// store the result in `out[i]`.
pub fn world_obj_build_matrices(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1); // f1 = 1.0
    c.g[T7][0] = c.g[A1][0]; // the caller's parent matrix
    c.g[T6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80)); // skeleton
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x84)); // uniform scale
    c.g[A3][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8)); // bones
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
    c.swc1(0, c.g[SP][0] as u32);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x4)); // bone count
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T4][0] = 0u64;
    if !empty {
        c.cycles += 7;
        c.g[T2][0] = c.g[A2][0]; // out
        c.g[T8][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[T3][0] = c.g[T2][0];
        loop {
            c.cycles += 3;
            c.g[A1][0] = c.lw(c.g[A3][0] as u32); // bone->parent
            let no_parent = c.g[A1][0] == c.g[T8][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
            if no_parent {
                c.cycles += 75;
                world_obj_bone(c, c.g[T7][0] as u32, true);
            } else {
                c.cycles += 77;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                world_obj_bone(c, c.g[A1][0] as u32, false);
            }
            c.cycles += 6;
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x40));
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_02e0);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
    c.pc = ra;
}
/// `MenuSysGroupItem_Measure` (0x002cfb58): measure a grouped menu list.  For
/// every visible item (`MenuSysItem_IsVisible`) the item's measure method
/// (vtable `+0x24`, with the `+0x20` this-adjustment) fills the 0x10-byte rect
/// at `sp+0x10`, which is copied to `self+0x14c+i*16`; the item's height is
/// added to the running total at `sp+0xc` when it is the selected row, and
/// non-selected rows get the 20-pixel indent added before the tallest height
/// is kept at `sp+8` and finally written to the 16-byte result at `a0`.
pub fn menu_sys_group_item_measure(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0xc0);
        c.mtc1(0, 0); // f0 = 0.0
        frame.save(c, S0, 0xa0);
        frame.save(c, S4, 0x60);
        c.g[S0][0] = c.g[A1][0]; // the block
        frame.save(c, S7, 0x30);
        c.g[S4][0] = 0u64;
        frame.save(c, S1, 0x90);
        c.g[S7][0] = c.g[A0][0]; // the result rect
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xb0);
        c.g[A1][0] = c.lw(c.g[S0][0] as u32); // the page
        c.swc1(0, c.g[SP][0] as u32);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18)); // count
        let empty = (c.g[V1][0] as i64) <= 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
        if !empty {
            c.cycles += 8;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 20); // f20 = 20.0
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c)); // rects
            c.g[S3][0] = 0u64;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24)); // items
            c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x158));
            c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x154));
            loop {
                'iter: {
                    'a: {
                        'c: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x002c_fbe0);
                            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                            call_ptr(c, 0x002c_a378, 0x002c_fbe0)?;
                            let is_hidden = c.g[V0][0] == 0;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                            if !is_hidden {
                                'measured: {
                                    'selected: {
                                        c.cycles += 18;
                                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                                        let target = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x002c_fc00);
                                        c.g[A1][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        call_ptr(c, target, 0x002c_fc00)?;
                                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
                                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
                                        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x7));
                                        c.sdr(2, c.g[S2][0] as u32);
                                        c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0xf));
                                        c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0x8));
                                        c.g[RA][0] = sx(0x002c_fc28);
                                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                        call_ptr(c, 0x002c_bf30, 0x002c_fc28)?;
                                        let whole_group = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        if whole_group {
                                            break 'selected;
                                        }
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        let not_current = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        if not_current {
                                            break 'measured;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                        break 'selected;
                                    }
                                    c.cycles += 5;
                                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.lwc1(1, c.g[V0][0] as u32);
                                    c.add_s(0, 0, 1);
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                    break 'measured;
                                }
                                'row: {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    let current = c.g[A0][0] == c.g[V1][0];
                                    c.lwc1(1, c.g[V0][0] as u32);
                                    if !current {
                                        c.cycles += 1;
                                        c.add_s(1, 1, 20);
                                    }
                                }
                                c.cycles += 5;
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.c_lt_s(0, 1);
                                if c.fcc() {
                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    break 'a;
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                break 'iter;
                            }
                        }
                    }
                    // A hidden item contributes an empty rect.
                    c.cycles += 12;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
                    c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x1f));
                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x7));
                    c.sdr(6, c.g[S2][0] as u32);
                    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0xf));
                    c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x8));

                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.cycles += 6;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_fbd8);
                }
            }
        }
        c.cycles += 21;
        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(5, c.g[SP][0] as u32);
        c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(5, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.sdr(5, c.g[S7][0] as u32);
        c.sdl(6, (c.g[S7][0] as u32).wrapping_add(0xf));
        c.g[V0][0] = c.g[S7][0];
        c.sdr(6, (c.g[S7][0] as u32).wrapping_add(0x8));
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}
// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{seed_count, DATA};

    /// Like `difftest::check`, but caller-saved registers and LO/HI must match
    /// too: an indirect call's fake hashes every GPR and both multiplier
    /// halves, so a scratch register left different is a real difference.
    fn check_all(generated: GuestFn, hand: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", r.lohi);
        }
    }

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Float bits at `a`.
    fn f(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }

    #[test]
    fn render_toggles_menu_dtor_matches() {
        check_all(
            ee::s06::f_0024d710__generated,
            render_toggles_menu_dtor,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24d710, 0);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x40);
                // Destructor flags with bit 0 clear, so the items do not take the
                // `operator delete` branch.
                c.g[A1][0] = ((r.next_u32() as u64) << 1) | 0x2;
            },
        );
    }

    #[test]
    fn transition_wipe_draw_matches() {
        check_all(ee::s09::f_002ebc40__generated, transition_wipe_draw, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x2ebc40, 2);
            let self_ = DATA + 0x200;
            c.g[A0][0] = sx(self_);
            c.f[12] = r.edge_f32();
            w(c, self_ + 0x14, r.next_u32() % 3);
            w(c, self_ + 0x18, r.next_u32() % 3);
            w(c, self_ + 0x4, r.next_u32());
            for off in [0xa40u32, 0xa44, 0xa48, 0xa4c, 0xa50, 0xa54] {
                f(c, crate::difftest::GAME_GP + off, r.edge_f32());
            }
            for off in [0x39a0u32, 0x39a4, 0x3998, 0x399c] {
                f(c, crate::difftest::GAME_GP.wrapping_sub(off), r.edge_f32());
            }
        });
    }

    #[test]
    fn c_board_trail_fx_reset_matches() {
        check_all(
            ee::s09::f_002e8560__generated,
            c_board_trail_fx_reset,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e8560, 0);
                let self_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                for off in [0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24] {
                    w(c, self_ + off, 0x800 + (r.next_u32() & 0x3f0));
                }
                f(c, 0x004f_f120, r.next_u32());
                f(c, 0x004f_f124, r.next_u32());
                f(c, 0x004f_f128, r.next_u32());
                f(c, 0x004f_f12c, r.next_u32());
                f(c, 0x004f_f130, r.next_u32());
                f(c, 0x004f_f134, r.next_u32());
                for off in [0x14c0u32, 0x14c4, 0x14c8] {
                    f(c, crate::difftest::GAME_GP + off, r.edge_f32());
                }
            },
        );
    }

            /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    fn world(c: &mut Cpu) {
        // Minimal world singleton: world -> +0x84 game -> +0xc race manager.
        let w = DATA + 0x8_0000;
        c.bus.write32(crate::difftest::GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x84, w + 0x100);
        c.bus.write32(w + 0x100 + 0xc, w + 0x200);
    }

    fn be_library(c: &mut Cpu) {
        w(
            c,
            crate::difftest::GAME_GP.wrapping_sub(0x1f04),
            DATA + 0x30_0000,
        );
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap over test
    /// memory, for the allocations larger than the size-class pools.
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::f_0031ff60(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA as u32,
            "DLHeap_InitArena did not return"
        );
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage).
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, crate::difftest::GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool + head, base);
            w(c, crate::difftest::GAME_GP + slot, pool);
        }
    }

        #[test]
    fn world_block_pool_init_matches() {
        const S: u64 = 0x003a_7598;
        check_all(
            ee::s13::f_003a7598__generated,
            world_block_pool_init,
            40,
            &|c, seed| {
                machine(c, seed, S, 8);
                heap_init(c);
                pool(c);
                let this = DATA + 0x600;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64; // block count
                c.g[A2][0] = 3; // block size (the array is a2 * 0x18)
            },
        );
    }

        #[test]
    fn ui_state_mgr_switch_state_close_popups_matches() {
        const S: u64 = 0x0039_f4c0;
        check_all(
            ee::s13::f_0039f4c0__generated,
            ui_state_mgr_switch_state_close_popups,
            40,
            &|c, seed| {
                machine(c, seed, S, 0);
                let this = DATA + 0x1000;
                let head = DATA + 0x2000;
                let node0 = DATA + 0x3000;
                let node1 = DATA + 0x4000;
                let trans = DATA + 0x5000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(trans);
                w(c, this + 4, head);
                w(c, head, head);
                if seed % 5 == 0 {
                    // An empty list: `UIList_First` returns 0.
                    w(c, head + 4, head);
                } else {
                    w(c, head + 4, node0);
                    w(c, node0 + 4, node1);
                    w(c, node1 + 4, 0);
                    let f0 = (seed as u32) << 4 | 1; // bit 0: visible
                    let f1 = (seed as u32) << 8; // bit 0 clear: hidden
                    w(c, node0 + 0x1c, f0);
                    w(c, node1 + 0x1c, f1);
                    w(c, node0 + 0x10, (seed % 3) as u32);
                    w(c, node1 + 0x10, (seed % 3) as u32);
                }
                let tf = match seed % 4 {
                    0 => 0,
                    1 => 0x20,
                    2 => 0x40,
                    _ => 0x20 | 0x40,
                };
                w(c, trans + 0x1c, tf | ((seed as u32) << 12));
            },
        );
    }

        #[test]
    fn menu_sys_group_item_draw_matches() {
        const S: u64 = 0x002c_fd28;
        check_all(
            ee::s09::f_002cfd28__generated,
            menu_sys_group_item_draw,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 32);
                let this = DATA + 0x2000;
                let menu = DATA + 0x4000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x1000); // the block's starting rect
                let count = 1 + (seed % 3) as u32;
                w(c, this + 0x18, count);
                w(c, this, menu);
                let cur = DATA + 0x5000 + (seed % count) * 0x100;
                w(c, this + 0x14, cur);
                for i in 0..count {
                    let it = DATA + 0x5000 + i * 0x100;
                    // The item list is embedded at self+0x24.
                    w(c, this + 0x24 + i * 4, it);
                    // Visibility bit 0, then arbitrary flags.
                    w(c, it + 0x4, ((r.next_u32() & 1) << 1) | (r.next_u32() & 1));
                    let vt = DATA + 0x8000 + i * 0x100;
                    w(c, it + 0x10, vt);
                    c.bus.write16(vt + 0x28, (r.next_u32() as u16) & 0xfff0);
                    w(c, vt + 0x2c, 0x4000);
                    // The row height the vtable draw would leave.
                    w(c, this + 0x14c + i * 0x10 + 0xc, r.next_u32());
                }
                // The page's selected item: sometimes this group itself.
                let idx = seed % count;
                w(c, menu + 4, idx);
                w(
                    c,
                    menu + 0xc + idx * 4,
                    if seed % 2 == 0 { this } else { DATA + 0x9000 },
                );
                for k in 0..4u32 {
                    w(c, DATA + 0x1000 + k * 4, r.edge_f32());
                }
            },
        );
    }

    #[test]
    fn intersect_info_eval_derivative_matches() {
        const S: u64 = 0x0035_1660;
        check_all(
            ee::s11::f_00351660__generated,
            intersect_info_eval_derivative,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 0);
                let this = DATA + 0x1000;
                let curveset = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x6000); // velocity out
                c.g[A2][0] = sx(DATA + 0x6100); // rotational velocity out
                c.f[12] = r.edge_f32();
                w(c, this + 0x40, curveset);
                let mask = [0xffffu32, 0x5555, 0x00ff, 0x0001][seed as usize % 4];
                w(c, curveset + 0x18, mask);
                w(c, curveset + 0x20, DATA + 0x3000);
                for i in 0..16u32 {
                    w(c, this + i * 4, r.next_u32() % 3); // current segment index
                }
                for j in 0..16u32 {
                    let curve = DATA + 0x4000 + j * 0x100;
                    w(c, DATA + 0x3000 + j * 4, curve);
                    w(c, curve, 3); // segment count
                    w(c, curve + 4, DATA + 0x5000); // segments
                }
                for k in 0..8u32 {
                    let seg = DATA + 0x5000 + k * 0x18;
                    w(c, seg, r.edge_f32()); // a
                    w(c, seg + 4, r.edge_f32()); // b
                    w(c, seg + 8, r.edge_f32()); // c
                                                 // The segment's range sometimes contains t, sometimes not.
                    if seed % 2 == 0 {
                        w(c, seg + 0x10, 0u32); // 0.0
                        w(c, seg + 0x14, 0x40a0_0000); // 5.0
                    } else {
                        w(c, seg + 0x10, 0x40a0_0000);
                        w(c, seg + 0x14, 0x4120_0000); // 10.0
                    }
                }
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_layout_menu_matches() {
        const S: u64 = 0x001c_8758;
        check_all(
            ee::s03::f_001c8758__generated,
            c_fe_popup_confirm_layout_menu,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, S, 12);
                be_library(c);
                world(c);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                if seed % 6 != 0 {
                    w(c, this + 0x6c, DATA + 0x2000);
                }
                for i in 0..5u32 {
                    let obj = DATA + 0x2000 + i * 0x100;
                    let present = (seed >> i) & 1 != 0;
                    w(c, this + 0x90 + i * 4, if present { obj } else { 0 });
                    w(c, obj + 0x44, r.next_u32());
                    w(c, obj + 0x48, r.next_u32());
                    w(c, obj + 0x4c, r.next_u32());
                    let vt = DATA + 0x3000 + i * 0x100;
                    w(c, obj + 8, vt);
                    c.bus.write16(vt + 0xa0, (r.next_u32() as u16) & 0xfff0);
                    w(c, vt + 0xa4, 0x4000);
                    w(c, obj + 0x14, r.next_u32());
                }
                w(c, this + 0x1a0, r.next_u32());
                w(c, this + 0x1a4, r.next_u32());
                w(c, this + 0x1a8, r.next_u32());
                f(c, this + 0x288, r.edge_f32());
                f(c, this + 0x29c, r.edge_f32());
                w(c, this + 0x2c8, ((seed / 2) % 2) as u32);
            },
        );
    }

            }
