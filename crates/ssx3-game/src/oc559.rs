//! Staging module oc559 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc559`.

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
    0x0031_50f8,
    anim_math_static_init_destroy,
    "AnimMath_StaticInitDestroy",
)];

/// Where a word of the static table lands: an absolute guest address, or an
/// offset from `gp`.
#[derive(Clone, Copy)]
pub(crate) enum InitDst {
    Abs(u32),
    Gp(i32),
}

/// What the word holds: a literal word read from `gp + k` (the generated
/// `lwc1`/`swc1` pair), a constant word, or a constant byte.
#[derive(Clone, Copy)]
pub(crate) enum InitSrc {
    Lit(i32),
    Word(u32),
    Byte(u8),
}

/// Cycles the static-table fill charges (the generated body's one `+= 471`).
pub(crate) const ANIM_INIT_CYCLES: u64 = 471;

/// The table fill, in the generated store order (226 stores: 224 table words
/// at `0x4fe420..`, and the invalid-anim byte and word at `gp + 0x2860`).
pub(crate) const ANIM_INIT_STORES: &[(InitDst, InitSrc)] = &[
    (InitDst::Gp(10336), InitSrc::Byte(0xff)),
    (InitDst::Abs(0x4fe430), InitSrc::Lit(-12832)),
    (InitDst::Abs(0x4fe440), InitSrc::Lit(-12828)),
    (InitDst::Abs(0x4fe450), InitSrc::Lit(-12824)),
    (InitDst::Abs(0x4fe45c), InitSrc::Lit(-12820)),
    (InitDst::Abs(0x4fe460), InitSrc::Lit(-12816)),
    (InitDst::Abs(0x4fe470), InitSrc::Lit(-12812)),
    (InitDst::Abs(0x4fe480), InitSrc::Lit(-12808)),
    (InitDst::Abs(0x4fe48c), InitSrc::Lit(-12804)),
    (InitDst::Abs(0x4fe490), InitSrc::Lit(-12800)),
    (InitDst::Gp(10336), InitSrc::Word(0xffffffff)),
    (InitDst::Abs(0x4fe420), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe42c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe43c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe44c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe454), InitSrc::Lit(-12796)),
    (InitDst::Abs(0x4fe458), InitSrc::Lit(-12792)),
    (InitDst::Abs(0x4fe464), InitSrc::Lit(-12788)),
    (InitDst::Abs(0x4fe468), InitSrc::Lit(-12784)),
    (InitDst::Abs(0x4fe46c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe47c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe484), InitSrc::Lit(-12780)),
    (InitDst::Abs(0x4fe488), InitSrc::Lit(-12776)),
    (InitDst::Abs(0x4fe494), InitSrc::Lit(-12772)),
    (InitDst::Abs(0x4fe424), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe428), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe434), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe438), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe444), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe448), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe474), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe478), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe498), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4a0), InitSrc::Lit(-12768)),
    (InitDst::Abs(0x4fe4b0), InitSrc::Lit(-12764)),
    (InitDst::Abs(0x4fe4bc), InitSrc::Lit(-12760)),
    (InitDst::Abs(0x4fe4ec), InitSrc::Lit(-12756)),
    (InitDst::Abs(0x4fe49c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4a4), InitSrc::Lit(-12752)),
    (InitDst::Abs(0x4fe4a8), InitSrc::Lit(-12748)),
    (InitDst::Abs(0x4fe4ac), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4b4), InitSrc::Lit(-12744)),
    (InitDst::Abs(0x4fe4b8), InitSrc::Lit(-12740)),
    (InitDst::Abs(0x4fe4c0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4cc), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4d0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4dc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4e0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4e8), InitSrc::Lit(-12736)),
    (InitDst::Abs(0x4fe4f0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4fc), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe500), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe50c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe510), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe51c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4c4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4c8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4d4), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4d8), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe4e4), InitSrc::Lit(-12756)),
    (InitDst::Abs(0x4fe4f4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe4f8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe504), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe508), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe514), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe518), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe520), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe56c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe57c), InitSrc::Lit(-12732)),
    (InitDst::Abs(0x4fe590), InitSrc::Lit(-12728)),
    (InitDst::Abs(0x4fe52c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe530), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe53c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe540), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe54c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe550), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe55c), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe560), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe570), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe574), InitSrc::Lit(-12724)),
    (InitDst::Abs(0x4fe578), InitSrc::Lit(-12720)),
    (InitDst::Abs(0x4fe580), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe588), InitSrc::Lit(-12716)),
    (InitDst::Abs(0x4fe58c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe594), InitSrc::Lit(-12712)),
    (InitDst::Abs(0x4fe598), InitSrc::Lit(-12708)),
    (InitDst::Abs(0x4fe59c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5a0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe524), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe528), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe534), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe538), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe544), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe548), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe554), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe558), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe564), InitSrc::Word(0x3f000000)),
    (InitDst::Abs(0x4fe568), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe584), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5a4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5a8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5dc), InitSrc::Lit(-12704)),
    (InitDst::Abs(0x4fe5fc), InitSrc::Lit(-12700)),
    (InitDst::Abs(0x4fe5ac), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5b0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5b8), InitSrc::Lit(-12692)),
    (InitDst::Abs(0x4fe5bc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5c0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5c4), InitSrc::Lit(-12688)),
    (InitDst::Abs(0x4fe5c8), InitSrc::Lit(-12684)),
    (InitDst::Abs(0x4fe5cc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5d0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5d4), InitSrc::Lit(-12680)),
    (InitDst::Abs(0x4fe5d8), InitSrc::Lit(-12696)),
    (InitDst::Abs(0x4fe5e0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5e8), InitSrc::Lit(-12676)),
    (InitDst::Abs(0x4fe5ec), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5f0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5f4), InitSrc::Lit(-12672)),
    (InitDst::Abs(0x4fe600), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe60c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe610), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe61c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe620), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe624), InitSrc::Lit(-12668)),
    (InitDst::Abs(0x4fe628), InitSrc::Lit(-12664)),
    (InitDst::Abs(0x4fe62c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe5b4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe5e4), InitSrc::Lit(-12660)),
    (InitDst::Abs(0x4fe5f8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe604), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe608), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe614), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe618), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe630), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe670), InitSrc::Lit(-12656)),
    (InitDst::Abs(0x4fe68c), InitSrc::Lit(-12652)),
    (InitDst::Abs(0x4fe69c), InitSrc::Lit(-12648)),
    (InitDst::Abs(0x4fe63c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe640), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe644), InitSrc::Lit(-12644)),
    (InitDst::Abs(0x4fe64c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe650), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe65c), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe660), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe66c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe67c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe680), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe684), InitSrc::Lit(-12640)),
    (InitDst::Abs(0x4fe688), InitSrc::Lit(-12636)),
    (InitDst::Abs(0x4fe690), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe694), InitSrc::Lit(-12632)),
    (InitDst::Abs(0x4fe6a0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6a8), InitSrc::Lit(-12628)),
    (InitDst::Abs(0x4fe6ac), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6b0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6b4), InitSrc::Lit(-12624)),
    (InitDst::Abs(0x4fe634), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe638), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe648), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe654), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe658), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe664), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe668), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe674), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe678), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe698), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6a4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6b8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6bc), InitSrc::Lit(-12620)),
    (InitDst::Abs(0x4fe6dc), InitSrc::Lit(-12660)),
    (InitDst::Abs(0x4fe71c), InitSrc::Lit(-12616)),
    (InitDst::Abs(0x4fe72c), InitSrc::Lit(-12612)),
    (InitDst::Abs(0x4fe6c0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6cc), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6d0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6e0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6ec), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6f0), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6fc), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe700), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe70c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe710), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe714), InitSrc::Lit(-12608)),
    (InitDst::Abs(0x4fe720), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe728), InitSrc::Lit(-12604)),
    (InitDst::Abs(0x4fe730), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe738), InitSrc::Lit(-12600)),
    (InitDst::Abs(0x4fe73c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6c4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6c8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6d4), InitSrc::Lit(-12660)),
    (InitDst::Abs(0x4fe6d8), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6e4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6e8), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe6f4), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe6f8), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe704), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe708), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe718), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe724), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe734), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe740), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe76c), InitSrc::Lit(-12596)),
    (InitDst::Abs(0x4fe77c), InitSrc::Lit(-12592)),
    (InitDst::Abs(0x4fe744), InitSrc::Lit(-12588)),
    (InitDst::Abs(0x4fe748), InitSrc::Lit(-12584)),
    (InitDst::Abs(0x4fe74c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe750), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe754), InitSrc::Lit(-12580)),
    (InitDst::Abs(0x4fe758), InitSrc::Lit(-12576)),
    (InitDst::Abs(0x4fe75c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe760), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe764), InitSrc::Lit(-12572)),
    (InitDst::Abs(0x4fe768), InitSrc::Lit(-12568)),
    (InitDst::Abs(0x4fe770), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe774), InitSrc::Lit(-12564)),
    (InitDst::Abs(0x4fe778), InitSrc::Lit(-12560)),
    (InitDst::Abs(0x4fe780), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe78c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe790), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe794), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe79c), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe784), InitSrc::Word(0x0)),
    (InitDst::Abs(0x4fe788), InitSrc::Word(0x3f800000)),
    (InitDst::Abs(0x4fe798), InitSrc::Word(0x0)),
];

/// GPRs the fill leaves set (the generated body's final values; the
/// callee-saved `s0..s7` and `fp` are restored by the epilogue).
pub(crate) const ANIM_INIT_GPRS: &[(usize, u64)] = &[
    (V0, 0x500000),
    (V1, 0x500000),
    (A0, 0x500000),
    (A1, 0x4fe760),
    (A2, 0x4fe770),
    (A3, 0x500000),
    (T0, 0x4fe750),
    (T1, 0x4fe5f0),
    (T2, 0x500000),
    (T3, 0x500000),
    (T4, 0x4fe640),
    (T5, 0x500000),
    (T6, 0x4fe720),
    (T7, 0x4fe710),
    (T8, 0x500000),
    (T9, 0x500000),
    (AT, 0x3f000000),
];

/// FPRs the fill leaves set: the last value moved into each (`mtc1` of the
/// constants 1.0 / 0.5, or the literal word of the last `lwc1`).
pub(crate) const ANIM_INIT_FPRS: &[(usize, InitSrc)] = &[
    (0, InitSrc::Word(0x3f800000)),
    (1, InitSrc::Lit(-12592)),
    (2, InitSrc::Lit(-12596)),
    (3, InitSrc::Lit(-12588)),
    (4, InitSrc::Lit(-12584)),
    (5, InitSrc::Lit(-12580)),
    (6, InitSrc::Lit(-12576)),
    (7, InitSrc::Lit(-12572)),
    (8, InitSrc::Lit(-12568)),
    (9, InitSrc::Lit(-12564)),
    (10, InitSrc::Lit(-12560)),
    (11, InitSrc::Lit(-12664)),
    (12, InitSrc::Lit(-12660)),
    (13, InitSrc::Lit(-12784)),
    (14, InitSrc::Lit(-12780)),
    (15, InitSrc::Lit(-12776)),
    (16, InitSrc::Lit(-12796)),
    (17, InitSrc::Lit(-12792)),
];

/// `void AnimMath_StaticInitDestroy(int prio, int init)` (0x3150f8, the
/// animation map's static setup). With `prio == 0xFFFF` and `init != 0` it sets
/// the invalid-anim byte and word at `gp + 0x2860` to -1 and fills the static
/// colour and ramp table at `0x4fe420..0x4fe7a0` (224 words), from literal
/// words in gp-relative data. Otherwise it does nothing. The
/// registers and cycles of the generated body are reproduced, including the
/// scratch values the fill leaves (`v0` is `0xffff` when `prio` is not
/// `0xFFFF`, `-1` when `init` is 0). The callee-saved registers are saved to
/// the frame and not changed.
pub fn anim_math_static_init_destroy(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x003150f8);
    c.cycles += 12;
    let frame_sp = (c.g[SP][0] as u32).wrapping_sub(0x90);
    c.g[SP][0] = sx(frame_sp);
    c.g[V0][0] = 0xffff;
    c.sq(16, frame_sp.wrapping_add(0x80));
    c.sq(17, frame_sp.wrapping_add(0x70));
    c.sq(18, frame_sp.wrapping_add(0x60));
    c.sq(19, frame_sp.wrapping_add(0x50));
    c.sq(20, frame_sp.wrapping_add(0x40));
    c.sq(21, frame_sp.wrapping_add(0x30));
    c.sq(22, frame_sp.wrapping_add(0x20));
    c.sq(23, frame_sp.wrapping_add(0x10));
    c.sq(30, frame_sp);
    if c.g[A1][0] == 0xffff {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if c.g[A0][0] != 0 {
            anim_init_fill_table(c);
        }
    }
    // Epilogue: the generated `lq` reloads of s0..s7 and fp read back the
    // values saved above, which the body never changed, so they are skipped.
    c.cycles += 11;
    c.g[SP][0] = sx(frame_sp.wrapping_add(0x90));
    ret(c);
}

/// The table fill of `anim_math_static_init_destroy` (the `init != 0` path).
fn anim_init_fill_table(c: &mut Cpu) {
    c.cycles += ANIM_INIT_CYCLES;
    let gp = c.g[GP][0] as u32;
    for &(dst, src) in ANIM_INIT_STORES {
        let at = match dst {
            InitDst::Abs(a) => a,
            InitDst::Gp(k) => gp.wrapping_add(k as u32),
        };
        match src {
            InitSrc::Lit(k) => {
                let w = Addr(gp.wrapping_add(k as u32)).read_u32(c);
                Addr(at).sw(c, w as u64);
            }
            InitSrc::Word(w) => Addr(at).sw(c, w as u64),
            InitSrc::Byte(b) => Addr(at).sb(c, b as u64),
        }
    }
    for &(r, v) in ANIM_INIT_GPRS {
        c.g[r][0] = v;
    }
    for &(n, src) in ANIM_INIT_FPRS {
        c.f[n] = match src {
            InitSrc::Lit(k) => Addr(gp.wrapping_add(k as u32)).read_u32(c),
            InitSrc::Word(w) => w,
            InitSrc::Byte(b) => b as u32,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, GAME_GP};

    /// Both paths of the prologue, the table fill and the `init == 0` return:
    /// seed % 4 == 0 gives `prio != 0xFFFF`, seed % 6 == 0 gives `init == 0`.
    #[test]
    fn anim_math_static_init_destroy_matches() {
        check(
            ssx3_ee::s10::f_003150f8__generated,
            anim_math_static_init_destroy,
            200,
            &|c, seed| {
                let mut r = Rng(0x3150_f8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003150f8;
                c.g[GP][0] = sx(GAME_GP);
                c.g[SP][0] = sx(crate::difftest::SP);
                c.g[RA][0] = sx(crate::difftest::RA);
                let prio = if seed % 4 == 0 { r.next_u32() } else { 0xffff };
                let init = if seed % 6 == 0 { 0 } else { r.next_u32() | 1 };
                c.g[A1][0] = prio as u64;
                c.g[A0][0] = init as u64;
                // The literal words the fill reads, gp-relative.
                let mut off = -0x3400i32;
                while off < 0x2900 {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.next_u32());
                    off += 4;
                }
            },
        );
    }
}
