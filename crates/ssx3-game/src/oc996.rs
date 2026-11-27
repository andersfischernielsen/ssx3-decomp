//! Staging module oc996 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc996`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0021_f660,
    c_ov_state_hud1p_draw_race_time,
    "cOVStateHUD1P_DrawRaceTime",
)];

/// HUD element table entry size (`elems + index * 0x24`).
pub(crate) const RACE_TIME_ELEM_SIZE: u32 = 0x24;
/// Element index of the race-time panel (`flag == 0`) and of the small variant.
pub(crate) const RACE_TIME_ELEM: u32 = 0x42;
pub(crate) const RACE_TIME_ELEM_SMALL: u32 = 0x1b;
/// `self` fields: the two textures and the time font.
pub(crate) const RACE_TIME_TEX: u32 = 0x4b4;
pub(crate) const RACE_TIME_TEX_SMALL: u32 = 0x49c;
pub(crate) const RACE_TIME_FONT: u32 = 0x42c;
/// Mode byte (0 or 5: clock time, otherwise a points count), the per-rider
/// value table and the two `sprintf` formats.
pub(crate) const RACE_TIME_MODE: u32 = 0x0053_5c10;
pub(crate) const RACE_TIME_VALUES: u32 = 0x0053_6640;
pub(crate) const RACE_TIME_FMT_CLOCK: u32 = 0x0046_ed68;
pub(crate) const RACE_TIME_FMT_COUNT: u32 = 0x004a_2138;
/// Text colour (16 bytes) copied into the font.
pub(crate) const RACE_TIME_COLOUR: u32 = 0x004c_8808;

/// `void cOVStateHUD1P::DrawRaceTime(self, elems, rider, x, flag)`
/// (0x0021f660): draws the race-time sprite panel (`HUD_DrawSprite`, element
/// 0x42, or the small element 0x1b when `flag != 0`); with `flag == 0` it also
/// prints the rider's time (`h:mm:ss`, or a points count in other modes) in
/// the HUD font, centred on the panel with the shared colour.
/// Args: a0 `self`, a1 element table, a2 rider (index at +0), a3 `x`, t0 `flag`.
pub fn c_ov_state_hud1p_draw_race_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x150);
        let sp0 = sp(c);
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c); // world
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        c.g[S3][0] = c.g[A3][0]; // x
        frame.save(c, S6, 0xe0);
        c.g[S4][0] = c.g[A0][0]; // self
        frame.save(c, FP, 0xc0);
        c.g[S6][0] = c.g[T0][0]; // flag
        frame.save(c, S0, 0x140);
        c.g[FP][0] = c.g[A2][0]; // rider
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S5, 0xf0);
        frame.save(c, S7, 0xd0);
        frame.save_ra(c, 0xb0);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[V1][0] = Addr(c.g[FP][0] as u32).lw(c);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = RACE_TIME_ELEM as u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[S5][0] = Addr((c.g[S4][0] as u32).wrapping_add(RACE_TIME_TEX)).lw(c);
        let small = c.g[S6][0] != 0;
        c.g[S7][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c); // rider object
        if small {
            c.cycles += 2;
            c.g[S5][0] = Addr((c.g[S4][0] as u32).wrapping_add(RACE_TIME_TEX_SMALL)).lw(c);
            c.g[A0][0] = RACE_TIME_ELEM_SMALL as u64;
        }

        // The sprite: position / size / scale of the element, aligned, drawn.
        c.cycles += 60;
        c.g[S0][0] = RACE_TIME_ELEM_SIZE as u64;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.mult(0, V0, A0, S0);
        c.g[S2][0] = sx(sp0.wrapping_add(0x20)); // &a
        c.swc1(0, sp0.wrapping_add(0x4)); // scale = (1, 1)
        c.swc1(0, sp0);
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32)); // element
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_91f8, 0x0021_f6f8)?; // HUD_GetElemPos(e, &a)
        c.g[S1][0] = sx(sp0.wrapping_add(0x30)); // &size
        c.g[A2][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x001e_9220, 0x0021_f70c)?; // HUD_GetElemSize(e, &size, tex)
        c.g[A1][0] = sx(sp0.wrapping_add(0x40)); // &sc
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_9290, 0x0021_f718)?; // HUD_GetElemScale(e, &sc)
        c.lwc1(0, sp0.wrapping_add(0x30));
        c.g[T3][0] = c.g[S3][0]; // x
        c.lwc1(1, sp0.wrapping_add(0x34));
        c.g[S3][0] = sx(sp0.wrapping_add(0x10)); // &p
        c.lwc1(2, sp0.wrapping_add(0x40));
        c.g[A2][0] = c.g[S2][0];
        c.lwc1(3, sp0.wrapping_add(0x44));
        c.g[T0][0] = c.g[SP][0];
        c.mul_s(0, 0, 2); // size *= sc
        c.g[A3][0] = c.g[S1][0];
        c.mul_s(1, 1, 3);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.swc1(0, sp0.wrapping_add(0x30));
        c.swc1(1, sp0.wrapping_add(0x34));
        c.g[T2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x21)).lb(c);
        c.g[T1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x20)).lb(c);
        call_ptr(c, 0x001f_10f8, 0x0021_f760)?; // HUD_AlignElemPos(self, &p, &a, &size, &scale, ax, ay, x)
        c.g[S2][0] = sx(sp0.wrapping_add(0x50)); // &colour
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x001e_91a8, 0x0021_f770)?; // HUD_GetElemColor(e, &colour)
        c.g[V0][0] = Addr(gp.wrapping_sub(0x854)).lw(c); // renderer
        c.g[T0][0] = !0x3e0u64; // clears the layer bits
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x22)).lb(c);
        c.g[A1][0] = c.g[S5][0];
        c.g[T2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c); // draw state
        c.g[A2][0] = c.g[S3][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[A3][0] = c.g[S1][0];
        c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[V1][0] &= 0x3e0;
        c.g[T1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[V0][0] &= c.g[T0][0];
        c.g[V0][0] |= c.g[V1][0];
        c.g[T0][0] = c.g[SP][0];
        let state = (c.g[T2][0] as u32).wrapping_add(0x8);
        let word = c.g[V0][0];
        Addr(state).sw(c, word); // state flags with the element's layer
        call_ptr(c, 0x001f_1190, 0x0021_f7b4)?; // HUD_DrawSprite(self, tex, &p, &size, &scale, &colour)
        let small = c.g[S6][0] != 0;
        frame.restore(c, S0, 0x140);
        if !small {
            c.cycles += 19;
            call_ptr(c, 0x0014_dc80, 0x0021_f7c4)?; // cBE_getBE()
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_dd58, 0x0021_f7d0)?; // BELibrary_GetInterface(be, 0)
            c.g[A1][0] = Addr((c.g[S7][0] as u32).wrapping_add(0x6c0)).lw(c);
            c.g[V0][0] = 0x0053_0000;
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x5c10)).lb(c); // mode
            c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6c0)); // being
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lh(c);
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x3c)).lw(c);
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0021_f7f0)?; // being->getAIIndex()
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let mode_zero = c.g[S0][0] == 0;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c); // value
            let clock = if mode_zero {
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x5;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A2][0] = Addr((c.g[FP][0] as u32).wrapping_add(0x7c)).lw(c);
                    false
                } else {
                    true
                }
            };
            if clock {
                // value / 60 ticks to seconds -> h:mm:ss
                c.cycles += 7;
                c.mtc1(V1, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(1, gp.wrapping_sub(0x5344));
                c.g[V1][0] = 0xe10;
                c.g[V0][0] = 0xe10;
                c.cycles += 18;
                c.g[A3][0] = 0x3c;
                c.mul_s(0, 0, 1);
                c.g[V0][0] = 0x3c;
                c.g[S0][0] = sx(sp0.wrapping_add(0x60)); // buf
                c.cvt_w_s(1, 0);
                c.mfc1(A2, 1);
                c.g[A1][0] = 0x0047_0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1298));
                c.div(0, A2, V1); // t / 3600
                c.g[A2][0] = c.lo[0];
                c.g[V0][0] = c.hi[0];
                c.div(0, V0, A3); // rem / 60
                c.g[A3][0] = c.lo[0];
                c.g[T0][0] = c.hi[0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041_7828, 0x0021_f870)?; // sprintf(buf, fmt, h, m, s)
                c.g[A1][0] = c.g[S0][0];
            } else {
                // value + rider->points
                c.cycles += 7;
                c.g[S0][0] = sx(sp0.wrapping_add(0x60));
                c.g[A1][0] = 0x004a_0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                call_ptr(c, 0x0041_7828, 0x0021_f890)?; // sprintf(buf, fmt, n)
                c.g[A1][0] = c.g[S0][0];
            }

            // Centre the text on the panel (p += (size.x * 0.5, size.y)), shared
            // colour, draw, then restore the font scale.
            c.cycles += 59;
            c.lwc1(0, sp0.wrapping_add(0x30));
            c.g[V1][0] = 0x004d_0000;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 1);
            c.lwc1(2, sp0.wrapping_add(0x10));
            c.mul_s(0, 0, 1);
            c.lwc1(3, sp0.wrapping_add(0x14));
            c.lwc1(4, sp0.wrapping_add(0x40));
            c.lwc1(1, sp0.wrapping_add(0x44));
            c.lwc1(5, sp0.wrapping_add(0x34));
            c.add_s(2, 2, 0);
            c.lwc1(6, gp.wrapping_sub(0x5340));
            c.add_s(3, 3, 5);
            c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(RACE_TIME_FONT)).lw(c);
            c.mul_s(1, 1, 6);
            c.swc1(2, sp0.wrapping_add(0x10));
            c.mul_s(4, 4, 6);
            c.swc1(3, sp0.wrapping_add(0x14));
            c.swc1(1, sp0.wrapping_add(0x44));
            c.swc1(4, sp0.wrapping_add(0x40));
            let font = c.g[V0][0] as u32;
            c.lwc1(0, font.wrapping_add(0x30));
            c.lwc1(2, font.wrapping_add(0x34));
            c.mul_s(0, 0, 4);
            c.swc1(0, font.wrapping_add(0x38));
            c.lwc1(1, sp0.wrapping_add(0x44));
            c.mul_s(2, 2, 1);
            c.swc1(2, font.wrapping_add(0x3c));
            c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x77f8));
            let src = c.g[A3][0] as u32;
            c.ldl(2, src.wrapping_add(0x7));
            c.ldr(2, src);
            c.ldl(4, src.wrapping_add(0xf));
            c.ldr(4, src.wrapping_add(0x8));
            c.sdl(2, sp0.wrapping_add(0xa7));
            c.sdr(2, sp0.wrapping_add(0xa0));
            c.sdl(4, sp0.wrapping_add(0xaf));
            c.sdr(4, sp0.wrapping_add(0xa8));
            c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(RACE_TIME_FONT)).lw(c);
            c.ldl(3, sp0.wrapping_add(0xa7));
            c.ldr(3, sp0.wrapping_add(0xa0));
            c.ldl(4, sp0.wrapping_add(0xaf));
            c.ldr(4, sp0.wrapping_add(0xa8));
            let font = c.g[V0][0] as u32;
            c.sdl(3, font.wrapping_add(0x47));
            c.sdr(3, font.wrapping_add(0x40));
            c.sdl(4, font.wrapping_add(0x4f));
            c.sdr(4, font.wrapping_add(0x48));
            c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(RACE_TIME_FONT)).lw(c);
            c.lwc1(12, sp0.wrapping_add(0x10));
            c.lwc1(13, sp0.wrapping_add(0x14));
            call_ptr(c, 0x0039_2908, 0x0021_f958)?; // Font_DrawTextCentered(font, x, y, buf)
            c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(RACE_TIME_FONT)).lw(c);
            c.lwc1(1, sp0);
            let font = c.g[V0][0] as u32;
            c.lwc1(0, font.wrapping_add(0x30));
            c.lwc1(2, font.wrapping_add(0x34));
            c.mul_s(0, 0, 1);
            c.swc1(0, font.wrapping_add(0x38));
            c.lwc1(1, sp0.wrapping_add(0x4));
            c.mul_s(2, 2, 1);
            c.swc1(2, font.wrapping_add(0x3c));
            frame.restore(c, S0, 0x140);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use crate::game::fe::ovstatehud::tests::{
        put_cstr, rider_ind_elem, rider_ind_f, rider_ind_uf, GET_WORD,
    };

    /// `cOVStateHUD1P_DrawRaceTime`: element 0x42 (flag 0: sprite plus time
    /// text) or 0x1b (flag 1: sprite only); the texture present or missing;
    /// every mode byte (0 and 5: `h:mm:ss`, others: value + rider points;
    /// the clock value positive, negative and huge); the real element
    /// getters, `HUD_AlignElemPos`, `HUD_DrawSprite`, `Font_MeasureText`,
    /// `Font_DrawText`, `sprintf`, `cBE_getBE`/`GetInterface` and a real
    /// `getAIIndex` (reads a word of the being); renderer methods are fakes
    /// and some seeds unwind there.
    #[test]
    fn c_ov_state_hud1p_draw_race_time_matches() {
        const HUD: u32 = DATA + 0x2000;
        const RIDER: u32 = DATA + 0x3000;
        const WORLD: u32 = DATA + 0x3800;
        const ELEMS: u32 = DATA + 0x4000;
        const RIDER_OBJ: u32 = DATA + 0x5000;
        const BEING_VT: u32 = DATA + 0x5800;
        const BE_LIB: u32 = DATA + 0x6000;
        const RENDERER: u32 = DATA + 0x7000;
        const DRAW_STATE: u32 = DATA + 0x7800;
        const TEX: u32 = DATA + 0x7c00;
        const FONT_AT: u32 = DATA + 0x8000;
        const RENDERER_VT: u32 = DATA + 0x9000;
        const GLYPHS: u32 = DATA + 0xa000;
        const CODES: u32 = DATA + 0xa800;
        check(
            ee::s05::f_0021f660__generated,
            c_ov_state_hud1p_draw_race_time,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21_f660, 4);
                c.pc = 0x0021_f660;
                let gp = |off: i32| GAME_GP.wrapping_add(off as u32);
                let small = seed % 4 == 0;
                let index = r.next_u32() % 4;
                // The rider (index, points), world -> race -> slots -> object.
                c.bus.write32(RIDER, index);
                c.bus.write32(RIDER + 0x7c, r.next_u32() % 2000);
                c.bus.write32(gp(-0x848), WORLD);
                c.bus.write32(WORLD + 0x84, DATA + 0x3900);
                c.bus.write32(DATA + 0x3900 + 0xc, DATA + 0x3a00);
                c.bus.write32(DATA + 0x3a00 + 0x28 + 4 * index, RIDER_OBJ);
                // The being (+0x6c0): vtable slot 7 = a real word getter on
                // this + 0x20, which holds the AI index.
                let ai = r.next_u32() % 8;
                c.bus.write32(RIDER_OBJ + 0x6c0, BEING_VT);
                c.bus.write16(BEING_VT + 0x38, 0x20);
                c.bus.write32(BEING_VT + 0x3c, GET_WORD);
                c.bus.write32(RIDER_OBJ + 0x6e0, ai);
                // Mode byte and the per-rider value table.
                let mode = [0u8, 5, 1, 2, 0xff, 5][(seed / 4 % 6) as usize];
                c.bus.write8(RACE_TIME_MODE, mode);
                for k in 0..8 {
                    let v = match r.next_u32() % 6 {
                        0 => 0xffff_ff00 | (r.next_u32() & 0xff),   // negative
                        1 => 0x7fff_0000 | (r.next_u32() & 0xffff), // huge
                        2 => 216_000 + r.next_u32() % 40,           // around an hour
                        _ => r.next_u32() % 600_000,
                    };
                    c.bus.write32(RACE_TIME_VALUES + 4 * k, v);
                }
                put_cstr(c, RACE_TIME_FMT_CLOCK, b"%d:%02d:%02d");
                put_cstr(c, RACE_TIME_FMT_COUNT, b"%d pts");
                c.bus.write32(gp(-0x5344), rider_ind_f(1.0 / 60.0));
                c.bus
                    .write32(gp(-0x5340), rider_ind_f(rider_ind_uf(&mut r, 0.5, 2.5)));
                for k in 0..4 {
                    c.bus.write32(RACE_TIME_COLOUR + 4 * k, r.next_u32());
                }
                // BELibrary and its interface 0 (a fake method).
                c.bus.write32(gp(-0x1f04), DATA + 0x30_0000);
                c.bus.write32(gp(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, BE_LIB);
                c.bus.write32(BE_LIB + 4, BE_LIB + 0x100);
                c.bus.write16(BE_LIB + 0x100 + 8, 0);
                c.bus.write32(BE_LIB + 0x100 + 0xc, FAKE);
                // The renderer (draw state and two fake methods).
                c.bus.write32(gp(-0x854), RENDERER);
                c.bus.write32(gp(0x2a90), RENDERER);
                c.bus.write32(RENDERER + 0xe84, DRAW_STATE);
                c.bus.write32(DRAW_STATE + 8, r.next_u32());
                c.bus.write32(RENDERER + 0x10d8, RENDERER_VT);
                for (i, off) in [0x238u32, 0x280].into_iter().enumerate() {
                    c.bus
                        .write16(RENDERER_VT + off, (r.next_u32() as u16) & 0xfc);
                    c.bus.write32(RENDERER_VT + off + 4, FAKE + 0x10 * i as u32);
                }
                // The HUD: textures and the time font (glyph table, codes).
                let tex = if seed % 7 == 3 { 0 } else { TEX };
                c.bus.write32(HUD + RACE_TIME_TEX, tex);
                c.bus.write32(HUD + RACE_TIME_TEX_SMALL, tex);
                c.bus.write32(TEX, r.next_u32() % 64);
                for j in 1..8 {
                    c.bus
                        .write32(TEX + 4 * j, rider_ind_f(rider_ind_uf(&mut r, 0.0, 64.0)));
                }
                c.bus.write32(HUD + RACE_TIME_FONT, FONT_AT);
                fill(c, &mut r, FONT_AT, 0x20);
                for i in 0..8 {
                    c.bus.write32(
                        FONT_AT + 0x18 + 4 * i,
                        rider_ind_f(rider_ind_uf(&mut r, 0.5, 30.0)),
                    );
                }
                for i in 0..2 {
                    let v = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        rider_ind_f(rider_ind_uf(&mut r, 0.0, 3.0))
                    };
                    c.bus.write32(FONT_AT + 0x28 + 4 * i, v);
                }
                let n = 1 + r.next_u32() % 40;
                let mut code = 0;
                for i in 0..n {
                    code += 1 + r.next_u32() % 6;
                    c.bus.write16(CODES + 2 * i, code as u16);
                }
                fill(c, &mut r, GLYPHS, 0x400);
                c.bus.write32(FONT_AT, n);
                c.bus.write32(FONT_AT + 4, CODES);
                c.bus.write32(FONT_AT + 8, GLYPHS + 12 * (r.next_u32() % 8));
                c.bus.write32(FONT_AT + 0xc, r.next_u32() % 64);
                c.bus.write32(FONT_AT + 0x10, 64 + r.next_u32() % 128);
                // The two panel elements.
                rider_ind_elem(c, &mut r, ELEMS + RACE_TIME_ELEM_SIZE * RACE_TIME_ELEM);
                rider_ind_elem(
                    c,
                    &mut r,
                    ELEMS + RACE_TIME_ELEM_SIZE * RACE_TIME_ELEM_SMALL,
                );
                c.g[A0][0] = sx(HUD);
                c.g[A1][0] = sx(ELEMS);
                c.g[A2][0] = sx(RIDER);
                c.g[A3][0] = (r.next_u32() % 640) as u64;
                c.g[T0][0] = small as u64;
            },
        );
    }
}
