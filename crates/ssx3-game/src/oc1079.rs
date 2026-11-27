//! Staging module oc1079 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1079`.

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
    0x003a_3f48,
    oc1079_ui_engine_static_init_destroy,
    "UIEngine_StaticInitDestroy",
)];

/// Where a static-init store takes its word from.
#[derive(Clone, Copy)]
pub(crate) enum Oc1079Src {
    /// The float literal at `gp - offset` (a `.lit4` pool entry).
    Lit(u32),
    /// A literal bit pattern (0.0 or a constant the compiler built in a register).
    Bits(u32),
}

/// The sColor/t2Vector words stored into the UI colour table
/// (0x5083d0..0x508750), in the generated body's store order.
pub(crate) const OC1079_STORES: &[(u32, Oc1079Src)] = &[
    (0x005083e0, Oc1079Src::Lit(0x24d8)),
    (0x005083f0, Oc1079Src::Lit(0x24d4)),
    (0x00508400, Oc1079Src::Lit(0x24d0)),
    (0x0050840c, Oc1079Src::Lit(0x24cc)),
    (0x00508410, Oc1079Src::Lit(0x24c8)),
    (0x00508420, Oc1079Src::Lit(0x24c4)),
    (0x00508430, Oc1079Src::Lit(0x24c0)),
    (0x0050843c, Oc1079Src::Lit(0x24bc)),
    (0x00508440, Oc1079Src::Lit(0x24b8)),
    (0x00508450, Oc1079Src::Lit(0x24b4)),
    (0x005083d0, Oc1079Src::Bits(0x3f800000)),
    (0x005083dc, Oc1079Src::Bits(0x3f000000)),
    (0x005083ec, Oc1079Src::Bits(0x0)),
    (0x005083fc, Oc1079Src::Bits(0x0)),
    (0x00508404, Oc1079Src::Lit(0x24b0)),
    (0x00508408, Oc1079Src::Lit(0x24ac)),
    (0x00508414, Oc1079Src::Lit(0x24a8)),
    (0x00508418, Oc1079Src::Lit(0x24a4)),
    (0x0050841c, Oc1079Src::Bits(0x0)),
    (0x0050842c, Oc1079Src::Bits(0x0)),
    (0x00508434, Oc1079Src::Lit(0x24a0)),
    (0x00508438, Oc1079Src::Lit(0x249c)),
    (0x00508444, Oc1079Src::Lit(0x2498)),
    (0x0050844c, Oc1079Src::Bits(0x3f800000)),
    (0x005083d4, Oc1079Src::Bits(0x3f800000)),
    (0x005083d8, Oc1079Src::Bits(0x3f800000)),
    (0x005083e4, Oc1079Src::Bits(0x3f800000)),
    (0x005083e8, Oc1079Src::Bits(0x3f800000)),
    (0x005083f4, Oc1079Src::Bits(0x3f800000)),
    (0x005083f8, Oc1079Src::Bits(0x3f800000)),
    (0x00508424, Oc1079Src::Bits(0x3f800000)),
    (0x00508428, Oc1079Src::Bits(0x3f800000)),
    (0x00508448, Oc1079Src::Bits(0x3f800000)),
    (0x00508454, Oc1079Src::Lit(0x2494)),
    (0x00508460, Oc1079Src::Lit(0x2490)),
    (0x0050846c, Oc1079Src::Lit(0x248c)),
    (0x0050849c, Oc1079Src::Lit(0x2488)),
    (0x00508458, Oc1079Src::Lit(0x2484)),
    (0x0050845c, Oc1079Src::Bits(0x0)),
    (0x00508464, Oc1079Src::Lit(0x2480)),
    (0x00508468, Oc1079Src::Lit(0x247c)),
    (0x00508470, Oc1079Src::Bits(0x3f800000)),
    (0x0050847c, Oc1079Src::Bits(0x3f800000)),
    (0x00508480, Oc1079Src::Bits(0x3f800000)),
    (0x0050848c, Oc1079Src::Bits(0x0)),
    (0x00508490, Oc1079Src::Bits(0x3f800000)),
    (0x00508498, Oc1079Src::Lit(0x2478)),
    (0x005084a0, Oc1079Src::Bits(0x3f800000)),
    (0x005084ac, Oc1079Src::Bits(0x3f800000)),
    (0x005084b0, Oc1079Src::Bits(0x3f800000)),
    (0x005084bc, Oc1079Src::Bits(0x3f800000)),
    (0x005084c0, Oc1079Src::Bits(0x3f800000)),
    (0x005084cc, Oc1079Src::Bits(0x0)),
    (0x005084d0, Oc1079Src::Bits(0x3f800000)),
    (0x00508474, Oc1079Src::Bits(0x3f800000)),
    (0x00508478, Oc1079Src::Bits(0x3f800000)),
    (0x00508484, Oc1079Src::Bits(0x0)),
    (0x00508488, Oc1079Src::Bits(0x0)),
    (0x00508494, Oc1079Src::Lit(0x2488)),
    (0x005084a4, Oc1079Src::Bits(0x3f800000)),
    (0x005084a8, Oc1079Src::Bits(0x3f800000)),
    (0x005084b4, Oc1079Src::Bits(0x3f800000)),
    (0x005084b8, Oc1079Src::Bits(0x3f800000)),
    (0x005084c4, Oc1079Src::Bits(0x0)),
    (0x005084c8, Oc1079Src::Bits(0x0)),
    (0x005084d4, Oc1079Src::Bits(0x0)),
    (0x005084d8, Oc1079Src::Bits(0x0)),
    (0x005084dc, Oc1079Src::Bits(0x0)),
    (0x0050851c, Oc1079Src::Bits(0x3f000000)),
    (0x0050852c, Oc1079Src::Lit(0x2474)),
    (0x00508540, Oc1079Src::Lit(0x2470)),
    (0x005084e0, Oc1079Src::Bits(0x3f800000)),
    (0x005084ec, Oc1079Src::Bits(0x3f800000)),
    (0x005084f0, Oc1079Src::Bits(0x3f800000)),
    (0x005084fc, Oc1079Src::Bits(0x3f000000)),
    (0x00508500, Oc1079Src::Bits(0x3f800000)),
    (0x0050850c, Oc1079Src::Bits(0x3f000000)),
    (0x00508510, Oc1079Src::Bits(0x3f800000)),
    (0x00508520, Oc1079Src::Bits(0x3f800000)),
    (0x00508524, Oc1079Src::Lit(0x246c)),
    (0x00508528, Oc1079Src::Lit(0x2468)),
    (0x00508530, Oc1079Src::Bits(0x3f800000)),
    (0x00508538, Oc1079Src::Lit(0x2464)),
    (0x0050853c, Oc1079Src::Bits(0x0)),
    (0x00508544, Oc1079Src::Lit(0x2460)),
    (0x00508548, Oc1079Src::Lit(0x245c)),
    (0x0050854c, Oc1079Src::Bits(0x0)),
    (0x00508550, Oc1079Src::Bits(0x3f800000)),
    (0x0050855c, Oc1079Src::Bits(0x3f800000)),
    (0x00508560, Oc1079Src::Bits(0x3f800000)),
    (0x005084e4, Oc1079Src::Bits(0x3f800000)),
    (0x005084e8, Oc1079Src::Bits(0x3f800000)),
    (0x005084f4, Oc1079Src::Bits(0x3f000000)),
    (0x005084f8, Oc1079Src::Bits(0x3f800000)),
    (0x00508504, Oc1079Src::Bits(0x3f000000)),
    (0x00508508, Oc1079Src::Bits(0x3f800000)),
    (0x00508514, Oc1079Src::Bits(0x3f000000)),
    (0x00508518, Oc1079Src::Bits(0x3f800000)),
    (0x00508534, Oc1079Src::Bits(0x3f800000)),
    (0x00508554, Oc1079Src::Bits(0x3f800000)),
    (0x00508558, Oc1079Src::Bits(0x3f800000)),
    (0x00508564, Oc1079Src::Bits(0x3f800000)),
    (0x0050858c, Oc1079Src::Lit(0x2458)),
    (0x005085ac, Oc1079Src::Lit(0x2454)),
    (0x00508568, Oc1079Src::Lit(0x2450)),
    (0x0050856c, Oc1079Src::Bits(0x0)),
    (0x00508570, Oc1079Src::Bits(0x3f800000)),
    (0x00508574, Oc1079Src::Lit(0x244c)),
    (0x00508578, Oc1079Src::Lit(0x2448)),
    (0x0050857c, Oc1079Src::Bits(0x0)),
    (0x00508580, Oc1079Src::Bits(0x3f800000)),
    (0x00508584, Oc1079Src::Lit(0x2444)),
    (0x00508588, Oc1079Src::Lit(0x2440)),
    (0x00508590, Oc1079Src::Bits(0x3f800000)),
    (0x00508598, Oc1079Src::Lit(0x243c)),
    (0x0050859c, Oc1079Src::Bits(0x3f800000)),
    (0x005085a0, Oc1079Src::Bits(0x3f800000)),
    (0x005085a4, Oc1079Src::Lit(0x2438)),
    (0x005085b0, Oc1079Src::Bits(0x3f800000)),
    (0x005085bc, Oc1079Src::Bits(0x3f800000)),
    (0x005085c0, Oc1079Src::Bits(0x3f800000)),
    (0x005085cc, Oc1079Src::Bits(0x3f800000)),
    (0x005085d0, Oc1079Src::Bits(0x3f800000)),
    (0x005085d4, Oc1079Src::Lit(0x2434)),
    (0x005085d8, Oc1079Src::Lit(0x2430)),
    (0x005085dc, Oc1079Src::Bits(0x0)),
    (0x005085e0, Oc1079Src::Bits(0x3f800000)),
    (0x00508594, Oc1079Src::Lit(0x242c)),
    (0x005085a8, Oc1079Src::Bits(0x3f800000)),
    (0x005085b4, Oc1079Src::Bits(0x3f800000)),
    (0x005085b8, Oc1079Src::Bits(0x3f800000)),
    (0x005085c4, Oc1079Src::Bits(0x3f800000)),
    (0x005085c8, Oc1079Src::Bits(0x3f800000)),
    (0x005085e4, Oc1079Src::Bits(0x3f800000)),
    (0x005085e8, Oc1079Src::Bits(0x3f800000)),
    (0x005085ec, Oc1079Src::Bits(0x3f800000)),
    (0x00508620, Oc1079Src::Lit(0x2428)),
    (0x0050863c, Oc1079Src::Lit(0x2424)),
    (0x0050864c, Oc1079Src::Lit(0x2420)),
    (0x0050866c, Oc1079Src::Lit(0x241c)),
    (0x005085f0, Oc1079Src::Bits(0x3f800000)),
    (0x005085f4, Oc1079Src::Lit(0x2418)),
    (0x005085fc, Oc1079Src::Bits(0x0)),
    (0x00508600, Oc1079Src::Bits(0x3f800000)),
    (0x0050860c, Oc1079Src::Bits(0x3f800000)),
    (0x00508610, Oc1079Src::Bits(0x3f800000)),
    (0x0050861c, Oc1079Src::Bits(0x0)),
    (0x0050862c, Oc1079Src::Bits(0x0)),
    (0x00508630, Oc1079Src::Bits(0x3f800000)),
    (0x00508634, Oc1079Src::Lit(0x2414)),
    (0x00508638, Oc1079Src::Lit(0x2410)),
    (0x00508640, Oc1079Src::Bits(0x3f800000)),
    (0x00508644, Oc1079Src::Lit(0x240c)),
    (0x00508650, Oc1079Src::Bits(0x3f800000)),
    (0x00508658, Oc1079Src::Lit(0x2408)),
    (0x0050865c, Oc1079Src::Bits(0x0)),
    (0x00508660, Oc1079Src::Bits(0x3f800000)),
    (0x00508664, Oc1079Src::Lit(0x2404)),
    (0x00508670, Oc1079Src::Bits(0x3f800000)),
    (0x005085f8, Oc1079Src::Bits(0x3f800000)),
    (0x00508604, Oc1079Src::Bits(0x3f800000)),
    (0x00508608, Oc1079Src::Bits(0x3f800000)),
    (0x00508614, Oc1079Src::Bits(0x3f800000)),
    (0x00508618, Oc1079Src::Bits(0x0)),
    (0x00508624, Oc1079Src::Bits(0x0)),
    (0x00508628, Oc1079Src::Bits(0x0)),
    (0x00508648, Oc1079Src::Bits(0x3f800000)),
    (0x00508654, Oc1079Src::Bits(0x3f800000)),
    (0x00508668, Oc1079Src::Bits(0x3f800000)),
    (0x00508674, Oc1079Src::Bits(0x3f800000)),
    (0x0050868c, Oc1079Src::Lit(0x242c)),
    (0x005086cc, Oc1079Src::Lit(0x2400)),
    (0x005086dc, Oc1079Src::Lit(0x23fc)),
    (0x0050867c, Oc1079Src::Bits(0x3f800000)),
    (0x00508680, Oc1079Src::Bits(0x3f800000)),
    (0x00508690, Oc1079Src::Bits(0x3f800000)),
    (0x0050869c, Oc1079Src::Bits(0x0)),
    (0x005086a0, Oc1079Src::Bits(0x3f800000)),
    (0x005086ac, Oc1079Src::Bits(0x0)),
    (0x005086b0, Oc1079Src::Bits(0x3f800000)),
    (0x005086bc, Oc1079Src::Bits(0x0)),
    (0x005086c0, Oc1079Src::Bits(0x3f800000)),
    (0x005086c4, Oc1079Src::Lit(0x23f8)),
    (0x005086d0, Oc1079Src::Bits(0x3f800000)),
    (0x005086d8, Oc1079Src::Lit(0x23f4)),
    (0x005086e0, Oc1079Src::Bits(0x3f800000)),
    (0x005086e8, Oc1079Src::Lit(0x23f0)),
    (0x005086ec, Oc1079Src::Bits(0x0)),
    (0x005086f0, Oc1079Src::Bits(0x3f800000)),
    (0x005086f4, Oc1079Src::Lit(0x23ec)),
    (0x005086f8, Oc1079Src::Lit(0x23e8)),
    (0x00508678, Oc1079Src::Bits(0x3f800000)),
    (0x00508688, Oc1079Src::Bits(0x3f800000)),
    (0x00508684, Oc1079Src::Lit(0x242c)),
    (0x00508694, Oc1079Src::Bits(0x3f800000)),
    (0x00508698, Oc1079Src::Bits(0x0)),
    (0x005086a4, Oc1079Src::Bits(0x3f800000)),
    (0x005086a8, Oc1079Src::Bits(0x0)),
    (0x005086b4, Oc1079Src::Bits(0x3f800000)),
    (0x005086b8, Oc1079Src::Bits(0x0)),
    (0x005086c8, Oc1079Src::Bits(0x3f800000)),
    (0x005086d4, Oc1079Src::Bits(0x3f800000)),
    (0x005086e4, Oc1079Src::Bits(0x3f800000)),
    (0x005086fc, Oc1079Src::Bits(0x0)),
    (0x0050871c, Oc1079Src::Lit(0x23e4)),
    (0x0050872c, Oc1079Src::Lit(0x23e0)),
    (0x00508700, Oc1079Src::Bits(0x3f800000)),
    (0x00508704, Oc1079Src::Lit(0x23dc)),
    (0x00508708, Oc1079Src::Lit(0x23d8)),
    (0x0050870c, Oc1079Src::Bits(0x0)),
    (0x00508710, Oc1079Src::Bits(0x3f800000)),
    (0x00508714, Oc1079Src::Lit(0x23d4)),
    (0x00508718, Oc1079Src::Lit(0x23d0)),
    (0x00508720, Oc1079Src::Bits(0x3f800000)),
    (0x00508724, Oc1079Src::Lit(0x23cc)),
    (0x00508728, Oc1079Src::Lit(0x23c8)),
    (0x00508730, Oc1079Src::Bits(0x3f800000)),
    (0x0050873c, Oc1079Src::Bits(0x0)),
    (0x00508740, Oc1079Src::Bits(0x3f800000)),
    (0x00508744, Oc1079Src::Bits(0x3f800000)),
    (0x0050874c, Oc1079Src::Bits(0x0)),
    (0x00508738, Oc1079Src::Bits(0x3f800000)),
    (0x00508734, Oc1079Src::Bits(0x0)),
    (0x00508748, Oc1079Src::Bits(0x0)),
];

/// `void UIEngine_StaticInitDestroy(int initialize, int priority)`
/// (0x3a3f48): the UI engine's static constructor/destructor. With priority
/// 0xffff and `initialize != 0` it fills the UI colour table
/// (`D_005083D0..D_00508740`, sColor RGBA floats; the pool literals sit at
/// `gp - 0x24d8..`), runs `UISound_BaseCtor(&gUISoundNull)` (`gp + 0x2968`)
/// and sets the `t2Vector` at `gp + 0x2970` to (2.0, 2.0). With
/// `initialize == 0` it only stores the base sound vtable (0x46dd60) into
/// `gUISoundNull`. Any other priority does nothing. Scratch registers and
/// `f0..f19` end as the compiled body leaves them.
pub fn oc1079_ui_engine_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0xffff;
        for (reg, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.save(c, reg, off);
        }
        let priority_ok = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if priority_ok {
            c.cycles += 2;
            let initialize = c.g[A0][0] != 0;
            c.g[T5][0] = 0x0051_0000;
            let gp = c.g[GP][0] as u32;
            if initialize {
                c.cycles += 493;
                for &(addr, s) in OC1079_STORES {
                    let word = match s {
                        Oc1079Src::Lit(off) => c.bus.read32(gp.wrapping_sub(off)),
                        Oc1079Src::Bits(b) => b,
                    };
                    c.bus.write32(addr, word);
                }
                c.g[T5][0] = 0x005086e0;
                c.g[T6][0] = 0x005086d0;
                c.g[T7][0] = 0x00508700;
                c.g[V0][0] = 0x00510000;
                c.g[S1][0] = 0x00508710;
                c.g[S2][0] = 0x00510000;
                c.g[S3][0] = 0x00508740;
                c.g[S5][0] = 0x005084e0;
                c.g[S4][0] = 0x00508530;
                c.g[T3][0] = 0x00508550;
                c.g[A1][0] = 0x00508570;
                c.g[A3][0] = 0x005085d0;
                c.g[T1][0] = 0x005085a0;
                c.g[V1][0] = 0x00510000;
                c.g[T0][0] = 0x00508630;
                c.g[A2][0] = 0x00508580;
                c.g[T4][0] = 0x00508650;
                c.g[T9][0] = 0x00510000;
                c.g[T2][0] = 0x00508660;
                c.g[S6][0] = 0x00510000;
                c.g[FP][0] = 0x00510000;
                c.g[T8][0] = 0x00510000;
                c.g[S7][0] = 0x00510000;
                c.g[S0][0] = 0x00508730;
                c.g[A0][0] = sx(gp.wrapping_add(0x2968));
                c.f[0] = 0x3f800000;
                c.lwc1(1, gp.wrapping_sub(0x23e0));
                c.lwc1(2, gp.wrapping_sub(0x23e4));
                c.lwc1(3, gp.wrapping_sub(0x23dc));
                c.lwc1(4, gp.wrapping_sub(0x23d8));
                c.lwc1(5, gp.wrapping_sub(0x23d4));
                c.lwc1(6, gp.wrapping_sub(0x23d0));
                c.lwc1(7, gp.wrapping_sub(0x23cc));
                c.lwc1(8, gp.wrapping_sub(0x23c8));
                c.lwc1(9, gp.wrapping_sub(0x2408));
                c.lwc1(10, gp.wrapping_sub(0x2404));
                c.lwc1(11, gp.wrapping_sub(0x2430));
                c.lwc1(12, gp.wrapping_sub(0x242c));
                c.lwc1(13, gp.wrapping_sub(0x24a8));
                c.lwc1(14, gp.wrapping_sub(0x24a4));
                c.lwc1(15, gp.wrapping_sub(0x24a0));
                c.lwc1(16, gp.wrapping_sub(0x249c));
                c.lwc1(17, gp.wrapping_sub(0x24b0));
                c.lwc1(18, gp.wrapping_sub(0x24ac));
                c.lwc1(19, gp.wrapping_sub(0x2494));
                Callee {
                    addr: 0x0039_e288,
                    f: ee::s13::f_0039e288,
                }
                .call(c, 0x003a_4724)?;
                c.g[AT][0] = 0x4000_0000;
                c.f[0] = 0x4000_0000;
                c.swc1(0, gp.wrapping_add(0x2974));
                c.swc1(0, gp.wrapping_add(0x2970));
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x0047_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22a0));
                let v = c.g[V0][0];
                c.sw(gp.wrapping_add(0x2968), v);
            }
        }
        c.cycles += 12;
        for (reg, off) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as DT_RA};

    /// `UIEngine_StaticInitDestroy` against the generated body: the init
    /// path (colour table, `UISound_BaseCtor` real), the destroy path
    /// (vtable store) and the wrong-priority exit.
    #[test]
    fn oc1079_ui_engine_static_init_destroy_matches() {
        check(
            ee::s13::f_003a3f48__generated,
            oc1079_ui_engine_static_init_destroy,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1079, 0);
                let lit = GAME_GP.wrapping_sub(0x2500);
                for i in 0..0x80u32 {
                    c.bus.write32(lit + 4 * i, r.next_u32());
                }
                c.g[A0][0] = if seed % 3 == 1 {
                    0
                } else {
                    1 + (seed as u64 % 5)
                };
                c.g[A1][0] = if seed % 5 == 4 { 0xfffe } else { 0xffff };
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }
}
