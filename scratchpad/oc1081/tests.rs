
#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (in the machine's own RAM) and the
    /// word the font fake returns.
    pub(crate) const OC1081_CTRL: u32 = DATA + 0xf_a000;
    pub(crate) const OC1081_FONT_RESULT: u32 = OC1081_CTRL + 4;
    pub(crate) const OC1081_MAGIC: u32 = 0x1081_c0de;

    pub(crate) const OC1081_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC1081_SCREEN: u32 = DATA + 0x2000;
    pub(crate) const OC1081_TRACK: u32 = DATA + 0x3000;
    pub(crate) const OC1081_THUMB: u32 = DATA + 0x3100;
    pub(crate) const OC1081_UP: u32 = DATA + 0x3200;
    pub(crate) const OC1081_DOWN: u32 = DATA + 0x3300;
    pub(crate) const OC1081_EXTRA: u32 = DATA + 0x3400;
    pub(crate) const OC1081_VT: u32 = DATA + 0x4000;
    pub(crate) const OC1081_FONT: u32 = DATA + 0x6000;
    pub(crate) const OC1081_GRMAN: u32 = DATA + 0x7000;
    pub(crate) const OC1081_STATE: u32 = DATA + 0x9000;
    pub(crate) const OC1081_SCROLLW: u32 = DATA + 0x9400;
    pub(crate) const OC1081_STRS: u32 = DATA + 0x9800;

    /// The fake callee: logs the call like `fakes::fake_call`, then gives the
    /// callees whose result steers the function a controlled result.
    fn oc1081_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let first = if addr == 0x0041_6810 { c.bus.read8(a0) } else { 0 };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            // strlen: empty or not by the first byte.
            0x0041_6810 => c.g[V0][0] = (first != 0) as u64,
            // UIScreen_LookupValue: "no frame" (0xffff) or a frame number.
            0x0039_c778 => {
                c.g[V0][0] = if a1 & 0x10 != 0 { 0xffff } else { (a1 & 0x7fff) as u64 }
            }
            // cUITextScroll_GetFont: the font, or none.
            0x003a_12d0 => c.g[V0][0] = c.bus.read32(OC1081_FONT_RESULT) as u64,
            _ => {}
        }
    }

    macro_rules! oc1081_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1081_CTRL) != OC1081_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1081_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1081_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1081_fakes!(
        (oc1081_fk_strlen, OC1081_PREV_STRLEN, ee::s15::OVERRIDE_00416810, ee::s15::f_00416810__generated, 0x0041_6810),
        (oc1081_fk_hash, OC1081_PREV_HASH, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (oc1081_fk_lookup, OC1081_PREV_LOOKUP, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039_c778),
        (oc1081_fk_thread, OC1081_PREV_THREAD, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc1081_fk_parts, OC1081_PREV_PARTS, ee::s13::OVERRIDE_0039DE68, ee::s13::f_0039de68__generated, 0x0039_de68),
        (oc1081_fk_font, OC1081_PREV_FONT, ee::s13::OVERRIDE_003A12D0, ee::s13::f_003a12d0__generated, 0x003a_12d0),
        (oc1081_fk_measure, OC1081_PREV_MEASURE, ee::s12::OVERRIDE_003921F0, ee::s12::f_003921f0__generated, 0x0039_21f0),
        (oc1081_fk_print, OC1081_PREV_PRINT, ee::s12::OVERRIDE_00391E30, ee::s12::f_00391e30__generated, 0x0039_1e30),
        (oc1081_fk_align, OC1081_PREV_ALIGN, ee::s13::OVERRIDE_0039FEB8, ee::s13::f_0039feb8__generated, 0x0039_feb8),
        (oc1081_fk_range, OC1081_PREV_RANGE, ee::s13::OVERRIDE_0039DA20, ee::s13::f_0039da20__generated, 0x0039_da20),
    );

    fn oc1081_gate(c: &mut Cpu) {
        oc1081_install_fakes();
        c.bus.write32(OC1081_CTRL, OC1081_MAGIC);
        c.bus.write32(OC1081_FONT_RESULT, 0);
    }

    /// The widget vtable: slots 0x48 (show), 0x80 (pre-render), 0xa0 and
    /// 0xa8 (get/set size) call fakes with a random `this` delta.
    fn oc1081_vtable(c: &mut Cpu, r: &mut crate::difftest::Rng) {
        for (i, off) in [0x48u32, 0x80, 0xa0, 0xa8].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(OC1081_VT + off, delta as u16);
            c.bus.write32(OC1081_VT + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    fn oc1081_widget(c: &mut Cpu, r: &mut crate::difftest::Rng, at: u32) {
        c.bus.write32(at + 8, OC1081_VT);
        for k in [0x44u32, 0x48, 0x4c] {
            c.bus.write32(at + k, r.f32(100.0));
        }
    }

    /// `cUIScroll_SetRange`: label frames on a moving position (string empty
    /// or not, screen or none, frame found or not), missing track/thumb,
    /// total zero, thumb resizing, arrows by position, the three hide modes,
    /// full and partial thumbs.
    #[test]
    fn cuiscroll_set_range_matches() {
        fakes::covered(
            ee::s13::f_0039da20__generated,
            cuiscroll_set_range,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x39da20, 10);
                oc1081_gate(c);
                oc1081_vtable(c, &mut r);
                let me = OC1081_SELF;
                for k in 0..0x40 {
                    c.bus.write32(me + 4 * k, r.next_u32());
                }
                // Labels: empty or a short name.
                c.bus.write8(me + CUISCROLL_UP_LABEL, if seed % 3 == 0 { 0 } else { b'u' });
                c.bus.write8(me + CUISCROLL_DOWN_LABEL, if seed % 5 < 2 { 0 } else { b'd' });
                c.bus.write32(me + CUISCROLL_SCREEN, if seed % 7 == 0 { 0 } else { OC1081_SCREEN });
                let old_pos = (r.next_u32() % 20) as i32;
                c.bus.write32(me + CUISCROLL_POS, old_pos as u32);
                let flags = if seed % 4 < 3 { 1 } else { 0 };
                c.bus.write32(me + CUISCROLL_FLAGS, flags | (r.next_u32() & !1));
                let mode = [0u32, 1, 2, 3, 0xffff_ffff, r.next_u32()][(seed / 3 % 6) as usize];
                c.bus.write32(me + CUISCROLL_MODE, mode);
                let widgets = [
                    (CUISCROLL_TRACK, OC1081_TRACK, seed % 13 == 1),
                    (CUISCROLL_THUMB, OC1081_THUMB, seed % 11 == 2),
                    (CUISCROLL_UP, OC1081_UP, seed % 5 == 3),
                    (CUISCROLL_DOWN, OC1081_DOWN, seed % 6 == 4),
                    (CUISCROLL_EXTRA, OC1081_EXTRA, seed % 3 == 1),
                ];
                for (off, at, none) in widgets {
                    c.bus.write32(me + off, if none { 0 } else { at });
                    oc1081_widget(c, &mut r, at);
                }
                // The sizes the getters would produce (the fakes leave them).
                let frame = c.g[SP][0] as u32 - 0xb0;
                let ty = r.f32(60.0);
                let ny = if seed % 2 == 0 { ty } else { r.f32(60.0) };
                c.bus.write32(frame + 0x20, r.f32(50.0));
                c.bus.write32(frame + 0x24, ty);
                c.bus.write32(frame + 0x30, r.f32(50.0));
                c.bus.write32(frame + 0x34, ny);
                // Arguments: pos, visible, total.
                let total = if seed % 9 == 5 { 0 } else { 1 + r.next_u32() % 30 };
                let pos = match seed % 4 {
                    0 => old_pos as u32 + 1 + r.next_u32() % 5,
                    1 => (old_pos as u32).saturating_sub(1 + r.next_u32() % 5),
                    2 => old_pos as u32,
                    _ => 0,
                };
                c.g[A0][0] = sx(me);
                c.g[A1][0] = pos as u64;
                c.g[A2][0] = (r.next_u32() % 12) as u64;
                c.g[A3][0] = total as u64;
                c.pc = 0x0039_da20;
            },
        );
    }

    /// `cUITextScroll_Render`: with and without the pre-render call, no font,
    /// shadow on or off, selection colour, rows from none to a negative
    /// (wrapped) byte, lines cut by the count, with and without the scroll
    /// bar.
    #[test]
    fn cuitextscroll_render_matches() {
        fakes::covered(
            ee::s13::f_003a1588__generated,
            cuitextscroll_render,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a1588, 12);
                oc1081_gate(c);
                oc1081_vtable(c, &mut r);
                let me = OC1081_SELF;
                for k in 0..0x40 {
                    c.bus.write32(me + 4 * k, r.next_u32());
                }
                c.bus.write32(me + 8, OC1081_VT);
                // Flags: bit 2 of +0x14 (skip the pre-render call), the layer byte.
                let f14 = (r.next_u32() & !(1 << 2)) | if seed % 3 == 0 { 1 << 2 } else { 0 };
                c.bus.write32(me + 0x14, f14);
                // +0x74: bit 0 selection colouring, bit 3 shadow.
                let f74 = (r.next_u32() & !9)
                    | if seed % 2 == 0 { 1 } else { 0 }
                    | if seed % 5 < 3 { 8 } else { 0 };
                c.bus.write32(me + 0x74, f74);
                c.bus.write32(me + 0x50, (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits());
                c.bus.write32(me + 0x54, (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits());
                c.bus.write32(me + 0x44, r.f32(200.0));
                c.bus.write32(me + 0x48, r.f32(200.0));
                c.bus.write32(me + 0xcc, r.f32(5.0));
                let height = match seed % 8 {
                    0 => 0.0f32,
                    1 => 3000.0,
                    2 => -50.0,
                    _ => 30.0 + (r.next_u32() % 250) as f32,
                };
                c.bus.write32(me + 0x64, height.to_bits());
                // Lines: top, count and current selection.
                let count = if seed % 10 == 4 { 0 } else { 1 + r.next_u32() % 20 };
                let top = r.next_u32() % 22;
                let cur = if seed % 4 == 1 { top + r.next_u32() % 3 } else { r.next_u32() % 22 };
                c.bus.write16(me + CUITEXTSCROLL_TOP, top as u16);
                c.bus.write16(me + CUITEXTSCROLL_COUNT, count as u16);
                c.bus.write16(me + 0xc4, cur as u16);
                c.bus.write32(me + 0xbc, OC1081_STRS);
                for k in 0..0x80 {
                    c.bus.write32(OC1081_STRS + 4 * k, DATA + 0x400 + 0x10 * (r.next_u32() % 40));
                }
                let scroll = if seed % 3 == 1 { 0 } else { OC1081_SCROLLW };
                c.bus.write32(me + CUITEXTSCROLL_SCROLL, scroll);
                // The font: none, or sizes in a plausible range.
                let font = if seed % 17 == 6 { 0 } else { OC1081_FONT };
                c.bus.write32(OC1081_FONT_RESULT, font);
                c.bus.write32(OC1081_FONT + 0x14, 6 + r.next_u32() % 20);
                c.bus.write32(OC1081_FONT + 0x30, r.f32(3.0));
                c.bus.write32(OC1081_FONT + 0x34, (0.5 + (r.next_u32() % 4) as f32 * 0.5).to_bits());
                for k in 0..0x30 {
                    c.bus.write32(OC1081_FONT + 0x38 + 4 * k, r.next_u32());
                }
                // The graphics manager and its draw-state word.
                c.bus.write32(crate::difftest::GAME_GP - 0x854, OC1081_GRMAN);
                c.bus.write32(OC1081_GRMAN + 0xe84, OC1081_STATE);
                c.bus.write32(OC1081_STATE + 8, r.next_u32());
                // Floats on the stack the measure fake leaves untouched.
                let frame = c.g[SP][0] as u32 - 0xd0;
                for k in 0..0x30 {
                    c.bus.write32(frame + 4 * k, r.f32(100.0));
                }
                c.g[A0][0] = sx(me);
                c.f[12] = r.f32(100.0);
                c.f[13] = r.f32(100.0);
                c.pc = 0x003a_1588;
            },
        );
    }
}
