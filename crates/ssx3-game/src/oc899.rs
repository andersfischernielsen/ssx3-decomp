//! Staging module oc899 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc899`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001c_2ee0, cfe_state_stats_update, "cFEStateStats_Update"),
    (0x001c_2d90, cfe_state_stats_ctor_2, "cFEStateStats_ctor_2"),
    (
        0x001c_2468,
        cfe_state_net_quick_messages_refresh_message,
        "cFEStateNetQuickMessages_RefreshMessage",
    ),
    (
        0x001c_2578,
        cfe_state_net_quick_messages_set_text_ellipsized,
        "cFEStateNetQuickMessages_SetTextEllipsized",
    ),
];

/// `cFEStateStats_Update(self)` (0x1c2ee0): the stats state's update is the
/// net FE state's update, tail-called with its own return address saved.
pub fn cfe_state_stats_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x001a_8770,
            f: ee::s03::f_001a8770,
        }
        .call(c, 0x001c_2ef0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateStats::cFEStateStats(this)` (0x1c2d90), the second constructor:
/// runs the net FE state's base constructor, then sets the stats vtable
/// (0x468278), zeroes the fields at +0x6e0, +0x6e4, +0x6e8 and the four menu
/// slots at +0x6d0..+0x6dc.
pub fn cfe_state_stats_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x001a_8500,
            f: ee::s03::f_001a8500,
        }
        .call(c, 0x001c_2dac)?;
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(0x6e0)).sw(c, 0);
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x7d88));
        Addr(this.wrapping_add(0x6e4)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        c.g[V0][0] = sx(this.wrapping_add(0x6dc));
        Addr(this.wrapping_add(0x6e8)).sw(c, 0);
        c.g[V1][0] = 0x3;
        loop {
            c.cycles += 7;
            let slot = c.g[V0][0] as u32;
            Addr(slot).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_2dd0);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetQuickMessages_RefreshMessage(self, i)` (0x1c2468): formats the
/// label of quick message `i` (`sprintf` of `i + 1`), builds its text from the
/// quick-key message (or, when the option has none, the stock text reached
/// through the world at gp-0x848), and sets the text widget of entry `i`
/// (`self + 0x6d4 + 4*i`) to the result through `SetTextEllipsized`.
pub fn cfe_state_net_quick_messages_refresh_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, 18, 0x90);
        frame.save(c, 20, 0x70);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0xb0);
        frame.save(c, 19, 0x80);
        frame.save(c, 17, 0xa0);
        frame.save_ra(c, 0x60);
        c.g[S4][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x001c_2490)?;
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001c_249c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(0x004a_1c08);
        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x0041_7828, 0x001c_24b4)?;
        c.g[A0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] = c.g[SP][0];
        c.g[S3][0] = c.g[A0][0];
        call_ptr(c, 0x0031_82d8, 0x001c_24c4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0014_f6a8, 0x001c_24d0)?;
        let have_quick_key = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[V0][0];
        if have_quick_key {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_8910, 0x001c_2534)?;
        } else {
            c.cycles += 21;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
            c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x0014_f678, 0x001c_24f8)?;
            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x001c_2508)?;
            c.g[S0][0] = sx(frame.at(c, 0x20));
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003d_cb20, 0x001c_2518)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_8910, 0x001c_2524)?;
        }
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
        c.cycles += 16;
        let text_arg = Addr(frame.at(c, 0x10)).lw(c);
        c.g[A2][0] = text_arg;
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S4][0];
        let widget = Addr((c.g[V0][0] as u32).wrapping_add(0x6d4)).lw(c);
        c.g[A1][0] = widget;
        Callee {
            addr: 0x001c_2578,
            f: ee::s03::f_001c2578,
        }
        .call(c, 0x001c_254c)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0031_8260, 0x001c_2558)?;
        frame.restore(c, 16, 0xb0);
        frame.restore(c, 17, 0xa0);
        frame.restore(c, 18, 0x90);
        frame.restore(c, 19, 0x80);
        frame.restore(c, 20, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The width test of the ellipsis loop: `f1 = [sp+0x10] - [sp+0x88]` against
/// `[sp+0x78]` (both loaded into f0/f2 as the original does). True when the
/// text is too wide for its box (the compare sets the FCC bit).
fn set_text_ellipsized_too_wide(c: &mut Cpu, frame: Frame) -> bool {
    let measured = Addr(frame.at(c, 0x10)).read_fpu(c);
    abi::set_fpr(c, 1, measured);
    let offset = Addr(frame.at(c, 0x88)).read_fpu(c);
    abi::set_fpr(c, 0, offset);
    let limit = Addr(frame.at(c, 0x78)).read_fpu(c);
    abi::set_fpr(c, 2, limit);
    let width = FpuF32(c.f[1]) - FpuF32(c.f[0]);
    c.f[1] = width.0;
    abi::c_lt_s(c, width, limit)
}

/// `cFEStateNetQuickMessages_SetTextEllipsized(self, text, str)` (0x1c2578):
/// copies `str` into the UI text `text`. The text's two scale floats (at
/// +0x50/+0x54) are copied to the frame and the text's font metrics are used
/// to measure `str`; while it is wider than the box, the last character is
/// dropped and "..." is appended.
pub fn cfe_state_net_quick_messages_set_text_ellipsized(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 70;
        let frame = Frame::open(c, 0x110);
        frame.save(c, 16, 0x100);
        frame.save(c, 17, 0xf0);
        frame.save(c, 18, 0xe0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 19, 0xd0);
        c.g[A1][0] = sx(frame.at(c, 0x10));
        frame.save(c, 20, 0xc0);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 21, 0xb0);
        frame.save(c, 22, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[S6][0] = 0;
        let text = c.g[S1][0] as u32;
        c.g[V1][0] = Addr(text.wrapping_add(0x8)).lw(c);
        // ldr/ldl at +0x50 then sdl/sdr to the frame: 8 bytes, then a word.
        let scales = Addr(text.wrapping_add(0x50)).read_u64_unaligned(c);
        c.g[V0][0] = scales;
        c.g[A0][0] = Addr(text.wrapping_add(0x58)).lw(c);
        Addr(frame.at(c, 0)).write_u64_unaligned(c, scales);
        let word = c.g[A0][0];
        Addr(frame.at(c, 8)).sw(c, word);
        let vt = c.g[V1][0] as u32;
        c.g[A0][0] = Addr(vt.wrapping_add(0xa0)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(0xa4)).lw(c);
        let slot = c.g[V0][0] as u32;
        c.g[A0][0] = sx(text.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, slot, 0x001c_25d8)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003a_04f0, 0x001c_25e0)?;
        let font = c.g[V0][0] as u32;
        let scale_x = Addr(frame.at(c, 0)).read_fpu(c);
        abi::set_fpr(c, 2, scale_x);
        c.g[A0][0] = c.g[S3][0];
        let scale_y = Addr(frame.at(c, 4)).read_fpu(c);
        abi::set_fpr(c, 3, scale_y);
        let glyph_w = Addr(font.wrapping_add(0x30)).read_fpu(c);
        abi::set_fpr(c, 0, glyph_w);
        let glyph_h = Addr(font.wrapping_add(0x34)).read_fpu(c);
        abi::set_fpr(c, 1, glyph_h);
        c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[2])).0;
        c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[3])).0;
        let out_w = FpuF32(c.f[0]);
        Addr(font.wrapping_add(0x38)).write_fpu(c, out_w);
        let out_h = FpuF32(c.f[1]);
        Addr(font.wrapping_add(0x3c)).write_fpu(c, out_h);
        call_ptr(c, 0x0041_6810, 0x001c_2608)?;
        c.g[S0][0] = sx(frame.at(c, 0x20));
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        call_ptr(c, 0x0041_66f4, 0x001c_261c)?;
        c.g[S4][0] = c.g[S0][0];
        c.g[V1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bd0));
        call_ptr(c, 0x003a_04f0, 0x001c_2630)?;
        let font2 = c.g[V0][0] as u32;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = sx(frame.at(c, 0x80));
        let x12 = Addr(font2.wrapping_add(0x38)).read_fpu(c);
        abi::set_fpr(c, 12, x12);
        c.g[A1][0] = c.g[S5][0];
        let y13 = Addr(font2.wrapping_add(0x3c)).read_fpu(c);
        abi::set_fpr(c, 13, y13);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001c_264c)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003a_04f0, 0x001c_2654)?;
        let font3 = c.g[V0][0] as u32;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        let x12b = Addr(font3.wrapping_add(0x38)).read_fpu(c);
        abi::set_fpr(c, 12, x12b);
        c.g[A2][0] = sx(frame.at(c, 0x70));
        let y13b = Addr(font3.wrapping_add(0x3c)).read_fpu(c);
        abi::set_fpr(c, 13, y13b);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_1fb0, 0x001c_2670)?;
        let mut too_wide = set_text_ellipsized_too_wide(c, frame);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        if too_wide {
            c.cycles += 2;
            c.g[S6][0] = 1;
            loop {
                c.cycles += 19;
                c.g[A0][0] = c.g[S1][0];
                let end = c.g[V0][0] as u32;
                Addr(end).sb(c, 0);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x003a_04f0, 0x001c_26a8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                let x12 = Addr((c.g[A0][0] as u32).wrapping_add(0x38)).read_fpu(c);
                abi::set_fpr(c, 12, x12);
                c.g[A2][0] = sx(frame.at(c, 0x70));
                let y13 = Addr((c.g[A0][0] as u32).wrapping_add(0x3c)).read_fpu(c);
                abi::set_fpr(c, 13, y13);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0039_1fb0, 0x001c_26c4)?;
                too_wide = set_text_ellipsized_too_wide(c, frame);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                if !too_wide {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001c_2698);
                }
            }
        }
        c.cycles += 2;
        let no_cut = c.g[S6][0] == 0;
        c.g[A1][0] = c.g[S5][0];
        if !no_cut {
            c.cycles += 7;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0041_62d0, 0x001c_26f4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x003a_0c50, 0x001c_2700)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a_0c50, 0x001c_2714)?;
        }
        frame.restore(c, 16, 0x100);
        c.cycles += 9;
        frame.restore(c, 17, 0xf0);
        frame.restore(c, 18, 0xe0);
        frame.restore(c, 19, 0xd0);
        frame.restore(c, 20, 0xc0);
        frame.restore(c, 21, 0xb0);
        frame.restore(c, 22, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the net-update fake answers only when this is set.
    pub(crate) const OC899_GATE: u32 = DATA + 0x6_e000;
    pub(crate) const OC899_MAGIC: u32 = 0x0899_c0de;
    static OC899_PREV_NET_UPDATE: AtomicUsize = AtomicUsize::new(0);

    fn oc899_fake_net_update(c: &mut Cpu) {
        if c.bus.read32(OC899_GATE) != OC899_MAGIC {
            return fakes::fall_back(c, &OC899_PREV_NET_UPDATE, ee::s03::f_001a8770__generated);
        }
        fakes::fake_call(c, 0x001a_8770);
    }

    /// The globals the net FE state's constructor and the quick-message
    /// formatting read (the BELibrary dummy and the world).
    pub(crate) fn oc899_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x6_8000);
        c.bus.write32(DATA + 0x6_8000 + 0x8c, DATA + 0x6_9000);
        c.bus.write32(DATA + 0x6_9000 + 0x4, DATA + 0x6_9100);
        c.bus.write16(DATA + 0x6_9100 + 0x20, 0x10);
        c.bus.write32(DATA + 0x6_9100 + 0x24, fakes::FAKE + 0x10);
    }

    #[test]
    fn cfe_state_stats_update_matches() {
        check(
            ee::s03::f_001c2ee0__generated,
            cfe_state_stats_update,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x2ee0, 1);
                fakes::hook(
                    addr_of_mut!(ee::s03::OVERRIDE_001A8770),
                    oc899_fake_net_update,
                    &OC899_PREV_NET_UPDATE,
                );
                c.bus.write32(OC899_GATE, OC899_MAGIC);
                c.pc = 0x001c_2ee0;
            },
        );
    }

    #[test]
    fn cfe_state_stats_ctor_2_matches() {
        check(
            ee::s03::f_001c2d90__generated,
            cfe_state_stats_ctor_2,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x2d90, 3);
                oc899_globals(c);
                let this = DATA + 0x1_0000;
                c.pc = 0x001c_2d90;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x6cc, DATA + 0x2_0000);
                c.bus.write32(DATA + 0x2_0000 + 0x4, DATA + 0x3_0000);
                c.bus.write32(this + 0x6c8 + 0x4, DATA + 0x3_0000);
                c.bus.write32(this + 0x6e0, seed);
            },
        );
    }

    #[test]
    fn refresh_message_matches() {
        check(
            ee::s03::f_001c2468__generated,
            cfe_state_net_quick_messages_refresh_message,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x2468, 3);
                oc899_globals(c);
                let this = DATA + 0x1_0000;
                c.pc = 0x001c_2468;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 4);
                for i in 0..4u32 {
                    c.bus
                        .write32(this + 0x6d4 + 4 * i, DATA + 0x2_0000 + 0x100 * i);
                }
                c.bus.write32(this + 0x6cc, DATA + 0x2_0000);
                c.bus.write32(DATA + 0x2_0000 + 0x4, DATA + 0x3_0000);
            },
        );
    }

    #[test]
    fn set_text_ellipsized_matches() {
        check(
            ee::s03::f_001c2578__generated,
            cfe_state_net_quick_messages_set_text_ellipsized,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x2578, 2);
                oc899_globals(c);
                let this = DATA + 0x1_0000;
                let text = DATA + 0x2_0000;
                let vt = DATA + 0x2_1000;
                c.pc = 0x001c_2578;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(text);
                c.g[A2][0] = sx(DATA + 0x3_0000);
                c.bus.write32(text + 0x8, vt);
                c.bus.write16(vt + 0xa0, (seed % 4) as u16 * 4);
                c.bus.write32(vt + 0xa4, fakes::FAKE + 0x20);
                c.bus
                    .write32(text + 0x50, ((seed % 7 + 1) as f32).to_bits());
                c.bus
                    .write32(text + 0x54, (((seed % 5) + 1) as f32).to_bits());
                c.bus.write32(text + 0x58, seed % 3);
                for (i, b) in b"Quick message text".iter().enumerate() {
                    c.bus.write8(DATA + 0x3_0000 + i as u32, *b);
                }
            },
        );
    }
}
