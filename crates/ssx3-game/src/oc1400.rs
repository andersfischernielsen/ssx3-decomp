//! Staging module oc1400 (wave av-1): hand-written AV subtitle setter
//! replacements, each with its differential test against the generated body.
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc1400`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_03a8, av_subtitle_set_x, "AV_SubtitleSetX"),
    (0x003b_03b0, av_subtitle_set_y, "AV_SubtitleSetY"),
    (0x003b_03b8, av_subtitle_set_width, "AV_SubtitleSetWidth"),
    (0x003b_03c0, av_subtitle_set_height, "AV_SubtitleSetHeight"),
    (
        0x003b_03c8,
        av_subtitle_set_string_id,
        "AV_SubtitleSetStringId",
    ),
    (0x003b_03d0, av_subtitle_set_frame, "AV_SubtitleSetFrame"),
    (
        0x003b_03d8,
        av_subtitle_set_justify_x,
        "AV_SubtitleSetJustifyX",
    ),
    (
        0x003b_03e0,
        av_subtitle_set_justify_y,
        "AV_SubtitleSetJustifyY",
    ),
    (0x003b_03e8, av_subtitle_set_shape, "AV_SubtitleSetShape"),
    (0x003b_03f0, av_subtitle_set_color0, "AV_SubtitleSetColor0"),
    (0x003b_03f8, av_subtitle_set_color1, "AV_SubtitleSetColor1"),
    (0x003b_0400, av_subtitle_set_color2, "AV_SubtitleSetColor2"),
    (0x003b_0408, av_subtitle_set_color3, "AV_SubtitleSetColor3"),
];

/// `void AV_SubtitleSetX(AVSubtitle* self, int x)` (0x3b03a8): store the x
/// field at +0x0.
pub fn av_subtitle_set_x(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetY(AVSubtitle* self, int y)` (0x3b03b0): store the y
/// field at +0x4.
pub fn av_subtitle_set_y(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetWidth(AVSubtitle* self, int w)` (0x3b03b8): store the
/// width field at +0x8.
pub fn av_subtitle_set_width(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x8).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetHeight(AVSubtitle* self, int h)` (0x3b03c0): store the
/// height field at +0xc.
pub fn av_subtitle_set_height(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0xc).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetStringId(AVSubtitle* self, int id)` (0x3b03c8): store
/// the string id field at +0x10.
pub fn av_subtitle_set_string_id(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x10).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetFrame(AVSubtitle* self, int frame)` (0x3b03d0): store
/// the frame field at +0x14.
pub fn av_subtitle_set_frame(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x14).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetJustifyX(AVSubtitle* self, int jx)` (0x3b03d8): store
/// the x-justify field at +0x18.
pub fn av_subtitle_set_justify_x(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x18).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetJustifyY(AVSubtitle* self, int jy)` (0x3b03e0): store
/// the y-justify field at +0x1c.
pub fn av_subtitle_set_justify_y(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x1c).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetShape(AVSubtitle* self, int shape)` (0x3b03e8): store
/// the shape field at +0x20.
pub fn av_subtitle_set_shape(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x20).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor0(AVSubtitle* self, int color)` (0x3b03f0):
/// store the first color byte at +0x24.
pub fn av_subtitle_set_color0(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x24).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor1(AVSubtitle* self, int color)` (0x3b03f8):
/// store the second color byte at +0x25.
pub fn av_subtitle_set_color1(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x25).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor2(AVSubtitle* self, int color)` (0x3b0400):
/// store the third color byte at +0x26.
pub fn av_subtitle_set_color2(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x26).sb(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void AV_SubtitleSetColor3(AVSubtitle* self, int color)` (0x3b0408):
/// store the fourth color byte at +0x27.
pub fn av_subtitle_set_color3(c: &mut Cpu) {
    c.cycles += 2;
    let self_ = Addr(abi::arg(c, 0));
    self_.off(0x27).sb(c, c.g[A1][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_paths, require_paths, scramble, Rng, DATA};

    /// Base of the scratch subtitle structs. Word-aligned with a nonzero low
    /// byte, so a wrong base or offset changes RAM; every word starts as
    /// garbage the stored value must overwrite exactly once.
    const OC1400_SUB: u32 = DATA + 0x1410;

    fn oc1400_setup(c: &mut Cpu, seed: u32, slot: u32, value: u32) {
        let mut r = Rng(0x0c40 ^ (seed as u64) ^ ((slot as u64) << 32));
        scramble(c, &mut r);
        for i in 0..16 {
            c.bus.write32(OC1400_SUB + slot * 0x40 + i * 4, 0xdead_beef);
        }
        c.g[A0][0] = (OC1400_SUB + slot * 0x40) as u64;
        c.g[A1][0] = (value ^ r.next_u32()) as u64;
    }

    #[test]
    fn av_subtitle_set_x_matches() {
        check(
            ssx3_ee::s13::f_003b03a8__generated,
            av_subtitle_set_x,
            100,
            &|c, seed| oc1400_setup(c, seed, 0, 0x1111_1111),
        );
    }

    #[test]
    fn av_subtitle_set_y_matches() {
        check(
            ssx3_ee::s13::f_003b03b0__generated,
            av_subtitle_set_y,
            100,
            &|c, seed| oc1400_setup(c, seed, 1, 0x2222_2222),
        );
    }

    #[test]
    fn av_subtitle_set_width_matches() {
        check(
            ssx3_ee::s13::f_003b03b8__generated,
            av_subtitle_set_width,
            100,
            &|c, seed| oc1400_setup(c, seed, 2, 0x3333_3333),
        );
    }

    #[test]
    fn av_subtitle_set_height_matches() {
        check(
            ssx3_ee::s13::f_003b03c0__generated,
            av_subtitle_set_height,
            100,
            &|c, seed| oc1400_setup(c, seed, 3, 0x4444_4444),
        );
    }

    #[test]
    fn av_subtitle_set_string_id_matches() {
        check(
            ssx3_ee::s13::f_003b03c8__generated,
            av_subtitle_set_string_id,
            100,
            &|c, seed| oc1400_setup(c, seed, 4, 0x5555_5555),
        );
    }

    #[test]
    fn av_subtitle_set_frame_matches() {
        check(
            ssx3_ee::s13::f_003b03d0__generated,
            av_subtitle_set_frame,
            100,
            &|c, seed| oc1400_setup(c, seed, 5, 0x6666_6666),
        );
    }

    #[test]
    fn av_subtitle_set_justify_x_matches() {
        check(
            ssx3_ee::s13::f_003b03d8__generated,
            av_subtitle_set_justify_x,
            100,
            &|c, seed| oc1400_setup(c, seed, 6, 0x7777_7777),
        );
    }

    #[test]
    fn av_subtitle_set_justify_y_matches() {
        check(
            ssx3_ee::s13::f_003b03e0__generated,
            av_subtitle_set_justify_y,
            100,
            &|c, seed| oc1400_setup(c, seed, 7, 0x1111_8888),
        );
    }

    #[test]
    fn av_subtitle_set_shape_matches() {
        check(
            ssx3_ee::s13::f_003b03e8__generated,
            av_subtitle_set_shape,
            100,
            &|c, seed| oc1400_setup(c, seed, 8, 0x2222_9999),
        );
    }

    #[test]
    fn av_subtitle_set_color0_matches() {
        check(
            ssx3_ee::s13::f_003b03f0__generated,
            av_subtitle_set_color0,
            100,
            &|c, seed| oc1400_setup(c, seed, 9, 0xaa),
        );
    }

    #[test]
    fn av_subtitle_set_color1_matches() {
        check(
            ssx3_ee::s13::f_003b03f8__generated,
            av_subtitle_set_color1,
            100,
            &|c, seed| oc1400_setup(c, seed, 10, 0xbb),
        );
    }

    #[test]
    fn av_subtitle_set_color2_matches() {
        check(
            ssx3_ee::s13::f_003b0400__generated,
            av_subtitle_set_color2,
            100,
            &|c, seed| oc1400_setup(c, seed, 11, 0xcc),
        );
    }

    #[test]
    fn av_subtitle_set_color3_matches() {
        check(
            ssx3_ee::s13::f_003b0408__generated,
            av_subtitle_set_color3,
            100,
            &|c, seed| oc1400_setup(c, seed, 12, 0xdd),
        );
    }
}
