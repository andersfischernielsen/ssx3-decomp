//! Staging module oc521 (wave wave230): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc521`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0024_9148, light_glow_menu_ctor, "LightGlowMenu_ctor")];

/// Offsets of the menu's items and vtable in the `cLightGlowMenu` object.
const LGM_VTBL: u32 = 0x12c;
const LGM_TITLE: u32 = 0x130;
const LGM_SPACE: u32 = 0x14c;
const LGM_ENABLE: u32 = 0x164;
const LGM_OVERRIDE_WP: u32 = 0x194;
const LGM_PS2_CAPTURE_SIZE: u32 = 0x1c4;
const LGM_PS2_MIN_INTENSITY: u32 = 0x1f4;
const LGM_PS2_MOD_INTENSITY: u32 = 0x228;
const LGM_PS2_COPY_INTENSITY: u32 = 0x25c;
const LGM_PS2_FRAME_SRC: u32 = 0x290;
const LGM_PS2_FRAME_DST: u32 = 0x2c4;
const LGM_PS2_TEX0: u32 = 0x2f8;
const LGM_PS2_TEX1: u32 = 0x32c;
const LGM_PS2_TEX2: u32 = 0x360;
const LGM_PS2_TEX3: u32 = 0x394;
const LGM_PS2_JITTER_TEX: u32 = 0x3c8;
const LGM_PS2_SHOW_TEXTURES: u32 = 0x3fc;
const LGM_SIZE: u32 = 0x42c;
const LGM_CUTOFF: u32 = 0x460;
const LGM_COLOUR_POWER: u32 = 0x494;
const LGM_COLOUR_SCALE: u32 = 0x4c8;
const LGM_COLOUR_AMBIENT: u32 = 0x4fc;
const LGM_SIZE_POWER: u32 = 0x530;
const LGM_SIZE_SCALE: u32 = 0x564;
const LGM_SIZE_AMBIENT: u32 = 0x598;
const LGM_FADE_RATE: u32 = 0x5cc;
const LGM_CLEAR_ALPHA: u32 = 0x600;
const LGM_INTENSITY_R: u32 = 0x630;
const LGM_INTENSITY_G: u32 = 0x664;
const LGM_INTENSITY_B: u32 = 0x698;
const LGM_INTENSITY_A: u32 = 0x6cc;
const LGM_INTENSITY_SCALE: u32 = 0x700;

/// The saved float registers the constructor fills once and reuses as the
/// bounds of its items: f20 = 0.0, f21 = 2.0, f22 = 1.0, f23 = 64.0,
/// f24 = 4.0, f25 = 8.0.
const LGM_F_ZERO: usize = 20;
const LGM_F_TWO: usize = 21;
const LGM_F_ONE: usize = 22;
const LGM_F_SIXTY_FOUR: usize = 23;
const LGM_F_FOUR: usize = 24;
const LGM_F_EIGHT: usize = 25;

/// Where the generated body reads an item address from: a frame slot the
/// previous step stored, or a callee-saved register that holds it.
#[derive(Clone, Copy)]
enum LgmSrc {
    Slot(u32),
    Reg(usize),
}

/// `this + off` as the 64-bit value the generated code keeps in a register.
fn lgm_item(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `V0 = this + off; sw V0, sp + slot`: an item address kept in the frame.
fn lgm_stash(c: &mut Cpu, frame: Frame, slot: u32, off: u32) {
    c.g[V0][0] = lgm_item(c.g[S0][0], off);
    let addr = frame.at(c, slot);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// The item address a call reads from `src`.
fn lgm_value(c: &mut Cpu, frame: Frame, src: LgmSrc) -> u64 {
    match src {
        LgmSrc::Slot(off) => {
            let addr = frame.at(c, off);
            c.lw(addr)
        }
        LgmSrc::Reg(r) => c.g[r][0],
    }
}

/// `cIntMenuItem(item, index, &var, min, max, name)` (0x2cd8f8): the item
/// address in a0, the gp variable at `var`, min in a3, max in t0, the name
/// in t1. Returns to `ret_addr`.
fn lgm_int(
    c: &mut Cpu,
    item: u64,
    index: u32,
    var: u32,
    min: u64,
    max: u64,
    name: u32,
    ret_addr: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.g[A1][0] = index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A3][0] = min;
    c.g[T0][0] = max;
    c.g[T1][0] = sx(name);
    call_ptr(c, 0x002c_d8f8, ret_addr)
}

/// `cFloatMenuItem_ctor(item, index, &var, min, max, name)` (0x2cdce8): the
/// minimum is f20 (0.0); the maximum is the saved float register `max`, or
/// f13 as the caller already loaded it (`None`). The name goes in a3. t0 and
/// t1 keep their values, as in the generated body.
fn lgm_float(
    c: &mut Cpu,
    item: u64,
    index: u32,
    var: u32,
    name: u32,
    max: Option<usize>,
    ret_addr: u32,
) -> Flow {
    c.g[A0][0] = item;
    c.mov_s(12, LGM_F_ZERO);
    if let Some(max) = max {
        c.mov_s(13, max);
    }
    c.g[A3][0] = sx(name);
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(var));
    c.g[A1][0] = index as u64;
    call_ptr(c, 0x002c_dce8, ret_addr)
}

/// `MenuSysPage_InsertItem(this, item, -1)` (0x2cbcc8).
fn lgm_add(c: &mut Cpu, item: u64, ret_addr: u32) -> Flow {
    c.g[A1][0] = item;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_addr)
}

/// `cLightGlowMenu_cLightGlowMenu(this)` (0x249148): the constructor of the
/// light-glow debug menu. It builds the page (`MenuSysPage_ctor`), the
/// title (a null item) and the spacer, then the switches (`cIntMenuItem`)
/// and the scales (`cFloatMenuItem_ctor`), each bound to its gp variable.
/// It sets the vtable and inserts the 31 items into the page in order.
/// Returns `this` in v0.
///
/// The generated body passes each item's address to the constructor that
/// follows it through a frame slot (`sp + 4 * n`), and keeps the item
/// addresses that are used late in s1..s7/fp. The port writes the same
/// slots before the same calls, so an unwinding callee leaves the same
/// stack. Its straight-line sequence has no branches; the only exits are
/// the unwind returns from the calls.
pub fn light_glow_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 470;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0xf0);
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save(c, FP, 0x70);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 25, 0x128);
        frame.save_fpr(c, 24, 0x120);
        frame.save_fpr(c, 23, 0x118);
        frame.save_fpr(c, 22, 0x110);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);

        // The callee-saved item addresses used by the late calls.
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = lgm_item(c.g[S0][0], LGM_CLEAR_ALPHA);
        c.g[S3][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_G);
        c.g[S4][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_B);
        c.g[S6][0] = lgm_item(c.g[S0][0], LGM_SIZE_AMBIENT);
        c.g[S7][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_R);
        c.g[FP][0] = lgm_item(c.g[S0][0], LGM_SIZE_POWER);
        c.g[S1][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_A);
        c.g[S5][0] = lgm_item(c.g[S0][0], LGM_INTENSITY_SCALE);

        // The page: MenuSysPage_ctor(this), with the title item in slot 0.
        lgm_stash(c, frame, 0, LGM_TITLE);
        call_ptr(c, 0x002c_aa58, 0x0024_91bc)?;

        // Title: a null item named "Light Glow Menu"; then the vtable.
        c.mtc1(0, LGM_F_ZERO); // f20 = 0.0
        lgm_stash(c, frame, 4, LGM_SPACE);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x1dd8));
        c.g[V0][0] = 0x0048_0000;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, LGM_F_TWO); // f21 = 2.0
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x428)); // cLightGlowMenu_vtbl
        c.g[AT][0] = 0x4100_0000;
        c.mtc1(1, LGM_F_EIGHT); // f25 = 8.0
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(LGM_VTBL);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[AT][0] = 0x4280_0000;
        c.mtc1(1, LGM_F_SIXTY_FOUR); // f23 = 64.0
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, LGM_F_ONE); // f22 = 1.0
        let title = lgm_value(c, frame, LgmSrc::Slot(0));
        c.g[A0][0] = title;
        call_ptr(c, 0x002c_d2a0, 0x0024_9204)?; // cNullMenuItem(title, name)

        // Spacer: a space item of height 40.
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(1, LGM_F_FOUR); // f24 = 4.0
        let space = lgm_value(c, frame, LgmSrc::Slot(4));
        c.g[A0][0] = space;
        lgm_stash(c, frame, 8, LGM_ENABLE);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_9220)?; // cSpaceMenuItem(space, 40)

        // Enable (int 0..1).
        let item = lgm_value(c, frame, LgmSrc::Slot(8));
        lgm_stash(c, frame, 0xc, LGM_OVERRIDE_WP);
        lgm_stash(c, frame, 0x10, LGM_PS2_CAPTURE_SIZE);
        lgm_int(c, item, 0, 0x12d8, 0, 1, 0x004a_2d30, 0x0024_9250)?;

        // Override world painter (int 0..1).
        let item = lgm_value(c, frame, LgmSrc::Slot(0xc));
        lgm_stash(c, frame, 0x14, LGM_PS2_MIN_INTENSITY);
        lgm_int(c, item, 1, 0x12dc, 0, 1, 0x0047_e238, 0x0024_9278)?;

        // PS2 capture size (int 4..8).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x10));
        lgm_stash(c, frame, 0x18, LGM_PS2_MOD_INTENSITY);
        lgm_int(c, item, 2, 0x12e0, 4, 8, 0x0047_e250, 0x0024_92a0)?;

        // PS2 minimum intensity cutoff (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x14));
        lgm_stash(c, frame, 0x1c, LGM_PS2_COPY_INTENSITY);
        lgm_stash(c, frame, 0x20, LGM_PS2_FRAME_SRC);
        lgm_float(
            c,
            item,
            3,
            0x12e4,
            0x0047_e268,
            Some(LGM_F_TWO),
            0x0024_92d0,
        )?;

        // PS2 post-cutoff scale (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x18));
        lgm_stash(c, frame, 0x24, LGM_PS2_FRAME_DST);
        lgm_stash(c, frame, 0x28, LGM_PS2_TEX0);
        lgm_float(
            c,
            item,
            4,
            0x12e8,
            0x0047_e288,
            Some(LGM_F_TWO),
            0x0024_9300,
        )?;

        // PS2 copy intensity (0..8).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x1c));
        lgm_stash(c, frame, 0x2c, LGM_PS2_TEX1);
        lgm_stash(c, frame, 0x30, LGM_PS2_TEX2);
        lgm_float(
            c,
            item,
            5,
            0x12ec,
            0x0047_e2a0,
            Some(LGM_F_EIGHT),
            0x0024_9330,
        )?;

        // PS2 frame source and blend intensities (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x20));
        lgm_stash(c, frame, 0x34, LGM_PS2_TEX3);
        lgm_stash(c, frame, 0x38, LGM_PS2_JITTER_TEX);
        lgm_float(
            c,
            item,
            6,
            0x12f0,
            0x0047_e2b8,
            Some(LGM_F_TWO),
            0x0024_9360,
        )?;

        // PS2 frame blend intensity (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x24));
        lgm_stash(c, frame, 0x3c, LGM_PS2_SHOW_TEXTURES);
        lgm_float(
            c,
            item,
            7,
            0x12f4,
            0x0047_e2d8,
            Some(LGM_F_TWO),
            0x0024_9388,
        )?;

        // PS2 blend textures 0..3 (0..2).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x28));
        lgm_stash(c, frame, 0x40, LGM_SIZE);
        lgm_float(
            c,
            item,
            8,
            0x12f8,
            0x0047_e2f8,
            Some(LGM_F_TWO),
            0x0024_93b0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x2c));
        lgm_stash(c, frame, 0x44, LGM_CUTOFF);
        lgm_float(
            c,
            item,
            9,
            0x12fc,
            0x0047_e310,
            Some(LGM_F_TWO),
            0x0024_93d8,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x30));
        lgm_stash(c, frame, 0x48, LGM_COLOUR_POWER);
        lgm_stash(c, frame, 0x4c, LGM_COLOUR_SCALE);
        lgm_float(
            c,
            item,
            10,
            0x1300,
            0x0047_e328,
            Some(LGM_F_TWO),
            0x0024_9408,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x34));
        lgm_stash(c, frame, 0x50, LGM_COLOUR_AMBIENT);
        lgm_float(
            c,
            item,
            11,
            0x1304,
            0x0047_e340,
            Some(LGM_F_TWO),
            0x0024_9430,
        )?;

        // PS2 texture jitter (0..8).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x38));
        lgm_stash(c, frame, 0x54, LGM_SIZE_SCALE);
        lgm_float(
            c,
            item,
            12,
            0x1308,
            0x0047_e358,
            Some(LGM_F_EIGHT),
            0x0024_9458,
        )?;

        // PS2 show textures (int 0..1).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x3c));
        lgm_stash(c, frame, 0x58, LGM_FADE_RATE);
        lgm_int(c, item, 13, 0x130c, 0, 1, 0x0047_e370, 0x0024_9480)?;

        // Size, cutoff, colour and size scales (floats).
        let item = lgm_value(c, frame, LgmSrc::Slot(0x40));
        lgm_float(
            c,
            item,
            14,
            0x1310,
            0x004a_2d38,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_94a0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x44));
        lgm_float(
            c,
            item,
            15,
            0x1314,
            0x004a_2d40,
            Some(LGM_F_ONE),
            0x0024_94c0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x48));
        lgm_float(
            c,
            item,
            16,
            0x1318,
            0x0047_e388,
            Some(LGM_F_FOUR),
            0x0024_94e0,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x4c));
        lgm_float(
            c,
            item,
            17,
            0x131c,
            0x0047_e398,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_9500,
        )?;
        // Colour ambient (0..16): the maximum is loaded into f13 here.
        let item = lgm_value(c, frame, LgmSrc::Slot(0x50));
        c.g[AT][0] = 0x4180_0000;
        c.mtc1(1, 13); // f13 = 16.0
        lgm_float(c, item, 18, 0x1320, 0x0047_e3a8, None, 0x0024_9524)?;
        let item = c.g[FP][0];
        lgm_float(
            c,
            item,
            19,
            0x1324,
            0x0047_e3b8,
            Some(LGM_F_FOUR),
            0x0024_9544,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x54));
        lgm_float(
            c,
            item,
            20,
            0x1328,
            0x0047_e3c8,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_9564,
        )?;
        let item = c.g[S6][0];
        lgm_float(
            c,
            item,
            21,
            0x132c,
            0x0047_e3d8,
            Some(LGM_F_SIXTY_FOUR),
            0x0024_9584,
        )?;
        let item = lgm_value(c, frame, LgmSrc::Slot(0x58));
        lgm_float(
            c,
            item,
            22,
            0x1330,
            0x0047_e3e8,
            Some(LGM_F_ONE),
            0x0024_95a4,
        )?;

        // Clear (background) alpha (int 0..255).
        let item = c.g[S2][0];
        lgm_int(c, item, 23, 0x1334, 0, 0xff, 0x0047_e3f8, 0x0024_95c4)?;

        // Intensity red, green, blue, alpha and scale (floats 0..1).
        let item = c.g[S7][0];
        lgm_float(
            c,
            item,
            24,
            0x1338,
            0x0047_e410,
            Some(LGM_F_ONE),
            0x0024_95e4,
        )?;
        let item = c.g[S3][0];
        lgm_float(
            c,
            item,
            25,
            0x133c,
            0x0047_e420,
            Some(LGM_F_ONE),
            0x0024_9604,
        )?;
        let item = c.g[S4][0];
        lgm_float(
            c,
            item,
            26,
            0x1340,
            0x0047_e430,
            Some(LGM_F_ONE),
            0x0024_9624,
        )?;
        let item = c.g[S1][0];
        lgm_float(
            c,
            item,
            27,
            0x1344,
            0x0047_e440,
            Some(LGM_F_ONE),
            0x0024_9644,
        )?;
        let item = c.g[S5][0];
        lgm_float(
            c,
            item,
            28,
            0x1348,
            0x0047_e450,
            Some(LGM_F_ONE),
            0x0024_9664,
        )?;

        // Insert the 31 items into the page, in the order the constructor
        // made them: title, spacer, then the toggles and scales.
        let order = [
            LgmSrc::Slot(0x00),
            LgmSrc::Slot(0x04),
            LgmSrc::Slot(0x08),
            LgmSrc::Slot(0x0c),
            LgmSrc::Slot(0x10),
            LgmSrc::Slot(0x14),
            LgmSrc::Slot(0x18),
            LgmSrc::Slot(0x1c),
            LgmSrc::Slot(0x20),
            LgmSrc::Slot(0x24),
            LgmSrc::Slot(0x28),
            LgmSrc::Slot(0x2c),
            LgmSrc::Slot(0x30),
            LgmSrc::Slot(0x34),
            LgmSrc::Slot(0x38),
            LgmSrc::Slot(0x3c),
            LgmSrc::Slot(0x40),
            LgmSrc::Slot(0x44),
            LgmSrc::Slot(0x48),
            LgmSrc::Slot(0x4c),
            LgmSrc::Slot(0x50),
            LgmSrc::Reg(FP),
            LgmSrc::Slot(0x54),
            LgmSrc::Reg(S6),
            LgmSrc::Slot(0x58),
            LgmSrc::Reg(S2),
            LgmSrc::Reg(S7),
            LgmSrc::Reg(S3),
            LgmSrc::Reg(S4),
            LgmSrc::Reg(S1),
            LgmSrc::Reg(S5),
        ];
        for (i, src) in order.into_iter().enumerate() {
            let item = lgm_value(c, frame, src);
            lgm_add(c, item, 0x0024_9674 + 0x10 * i as u32)?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 25, 0x128);
        frame.restore_fpr(c, 24, 0x120);
        frame.restore_fpr(c, 23, 0x118);
        frame.restore_fpr(c, 22, 0x110);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA};
    use ssx3_ee as ee;

    #[test]
    fn light_glow_menu_ctor_matches() {
        check(
            ee::s06::f_00249148__generated,
            light_glow_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x0024_9148, 0);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0024_9148;
            },
        );
    }
}
