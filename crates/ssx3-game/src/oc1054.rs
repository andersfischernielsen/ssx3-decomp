//! Staging module oc1054 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1054`.

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
    0x002f_6b88,
    visual_effects_man_static_init_destroy,
    "VisualEffectsMan_StaticInitDestroy",
)];

/// `VisualEffectsMan_StaticInitDestroy(int init, int mode)` (0x002f6b88), the visual-effects
/// manager's static initialiser / destructor. Only for `mode == 0xffff`: `init != 0` fills the
/// static emitter and light parameter tables (about 600 cycles of float and vector stores from
/// the gp constants) and runs `mEmitter_ctor` over two arrays; `init == 0` runs
/// `VisualEffectsMan_EmitterHolderDtor` over the holder array backwards. Any other `mode` does
/// nothing. The store sequence is kept as the generated body schedules it (scratch registers
/// and cycles included).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn visual_effects_man_static_init_destroy(c: &mut Cpu) {
    if c.pc != 0x002f6b88 {
        return ee::s09::f_002f6b88__generated(c);
    }
    guest(c, |c| {
        'b_2f7a20: {
            'b_2f7a1c: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                c.g[V0][0] = 0xffff;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_2f7a1c;
                } else {
                    'b_2f79e8: {
                        c.cycles += 2;
                        let t = c.g[A0][0] == 0u64;
                        c.g[A3][0] = 0x4d0000;
                        if t {
                            break 'b_2f79e8;
                        } else {
                            c.cycles += 605;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3890));
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x57e0));
                            c.g[T3][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x388c));
                            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x5810));
                            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[S1][0] = 0x4d0000;
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 11);
                            c.g[T5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x57b0));
                            c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[A2][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3888));
                            c.g[A1][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3884));
                            c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x57d0));
                            c.swc1(11, (c.g[T5][0] as u32).wrapping_add(0xc));
                            c.g[T1][0] = 0x4d0000;
                            c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0x57c0));
                            c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x57f0));
                            c.swc1(4, (c.g[A2][0] as u32).wrapping_add(0x57d0));
                            c.g[T2][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3880));
                            c.g[T7][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5800));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T4][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x387c));
                            c.g[T6][0] = sx((c.g[T4][0] as u32).wrapping_add(0x5820));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3878));
                            c.g[T0][0] = 0x4d0000;
                            c.swc1(8, (c.g[A3][0] as u32).wrapping_add(0x57e0));
                            c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5830));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3874));
                            c.g[S3][0] = 0x4d0000;
                            c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5840));
                            c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3870));
                            c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5850));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x386c));
                            c.g[S7][0] = 0x4d0000;
                            c.swc1(5, (c.g[T1][0] as u32).wrapping_add(0x57f0));
                            c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x5860));
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3868));
                            c.g[T1][0] = 0x4d0000;
                            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x5870));
                            c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5880));
                            c.swc1(12, (c.g[T2][0] as u32).wrapping_add(0x5800));
                            c.g[S6][0] = 0x4d0000;
                            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3864));
                            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x58a0));
                            c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S5][0] = 0x4d0000;
                            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3860));
                            c.g[T2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x58b0));
                            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x385c));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(15, (c.g[T3][0] as u32).wrapping_add(0x5810));
                            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x3858));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(17, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x3854));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(13, (c.g[T4][0] as u32).wrapping_add(0x5820));
                            c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58c0));
                            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x3850));
                            c.g[A0][0] = 0x4d0000;
                            c.swc1(14, (c.g[T6][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x57c0));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0xc));
                            c.swc1(18, (c.g[T0][0] as u32).wrapping_add(0x5830));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x57b0));
                            c.g[T0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[T4][0] = sx((c.g[T0][0] as u32).wrapping_add(0x58d0));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = 0x4d0000;
                            c.g[T5][0] = 0x4d0000;
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x58e0));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[T5][0] = 0x4d0000;
                            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x384c));
                            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x57c0));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3848));
                            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5900));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[T0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5910));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5920));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(19, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5950));
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3844));
                            c.g[T9][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5940));
                            c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3840));
                            c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5960));
                            c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x383c));
                            c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3838));
                            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5980));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3834));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(3, (c.g[S3][0] as u32).wrapping_add(0x5840));
                            c.swc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5990));
                            c.swc1(5, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x5850));
                            c.g[T8][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59a0));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x5860));
                            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x59b0));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3830));
                            c.g[A1][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5890));
                            c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[FP][0] = 0x7ff;
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x58a0));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x58b0));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A2][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a10));
                            c.g[V0][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x382c));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5870));
                            c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5880));
                            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5a30));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5a40));
                            c.g[T1][0] = 0x4d0000;
                            c.g[A3][0] = c.g[T1][0];
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x5890));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5890));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3828));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[A3][0] = 0x4d0000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[V1][0] = 0x4d0000;
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
                            c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[T2][0] = 0x4d0000;
                            c.swc1(1, (c.g[S4][0] as u32).wrapping_add(0x5920));
                            c.g[T1][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5ad0));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3824));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x58c0));
                            c.g[T2][0] = 0xffffffffffff0000;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x58d0));
                            c.g[T2][0] = c.g[T2][0] | 0xff00;
                            c.swc1(11, (c.g[T4][0] as u32).wrapping_add(0xc));
                            c.g[V1][0] = c.g[S4][0];
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x58e0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x58f0));
                            c.swc1(11, (c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[A3][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x58f0));
                            c.swc1(11, (c.g[V1][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3820));
                            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5ae0));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5900));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5910));
                            c.g[A0][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x381c));
                            c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5af0));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3818));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3814));
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5b00));
                            c.swc1(5, (c.g[T5][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(6, (c.g[T0][0] as u32).wrapping_add(0x4));
                            c.swc1(7, (c.g[T0][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5930));
                            c.g[T0][0] = 0x4d0000;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5930));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3810));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x380c));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5940));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5930));
                            c.swc1(11, (c.g[T4][0] as u32).wrapping_add(0x4));
                            c.g[T3][0] = 0x4d0000;
                            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x8));
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x58f0));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[T4][0] = 0x4d0000;
                            c.swc1(11, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = 0x4d0000;
                            c.swc1(11, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58f0));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3808));
                            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[T3][0] = sx((c.g[T4][0] as u32).wrapping_add(0x5b60));
                            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = 0xc;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[T5][0] = 0xb;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(2, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3804));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3800));
                            c.swc1(3, (c.g[T9][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5950));
                            c.sw((c.g[T9][0] as u32).wrapping_add(0xc), 0u64);
                            c.swc1(4, (c.g[T6][0] as u32).wrapping_add(0x4));
                            c.swc1(5, (c.g[T6][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x37fc));
                            c.g[T6][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x37f8));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x5960));
                            c.swc1(6, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[T6][0] = 0x4d0000;
                            c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x37f4));
                            c.g[S4][0] = c.g[T7][0];
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5970));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5970));
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x37f0));
                            c.swc1(8, (c.g[S4][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x5980));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(9, (c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[T6][0] = 0xffffffffffffffff;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5990));
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0xc));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x59a0));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x37ec));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x37e8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5970));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x59b0));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0xc));
                            c.swc1(10, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.swc1(12, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x37e4));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x59c0));
                            c.g[T7][0] = c.g[S0][0];
                            c.swc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x59c0));
                            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a20));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x37e0));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x37dc));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x5a00));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x37d8));
                            c.g[S0][0] = c.g[T7][0];
                            c.swc1(3, (c.g[A1][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59d0));
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x37d4));
                            c.swc1(2, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x37d0));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x59d0));
                            c.swc1(5, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[T7][0] = c.g[S4][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x59e0));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x59e0));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = c.g[S0][0];
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x59f0));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x59f0));
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5a00));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x37cc));
                            c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S0][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x37c8));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5a10));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x37c4));
                            c.g[A1][0] = c.g[S4][0];
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5a20));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a20));
                            c.swc1(8, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x37c0));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5a30));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x59d0));
                            c.swc1(9, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x59e0));
                            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x37bc));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x5a40));
                            c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5a50));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x59f0));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a00));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a20));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5a60));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5aa0));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.g[A2][0] = 0x4d0000;
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a50));
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x37b8));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5ab0));
                            c.swc1(13, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.g[A1][0] = 0x4d0000;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x37b4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(1, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.g[S0][0] = c.g[T7][0];
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                            c.g[S4][0] = 0x4d0000;
                            c.g[A2][0] = c.g[A1][0];
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5a60));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5a70));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a70));
                            c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a80));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5a80));
                            c.g[V0][0] = c.g[S4][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5a90));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a90));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A2][0] = c.g[A1][0];
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x37b0));
                            c.g[T7][0] = 0x4d0000;
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5aa0));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5aa0));
                            c.swc1(3, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = c.g[T7][0];
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x37ac));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5ab0));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ab0));
                            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[S4][0];
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x37a8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ac0));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5ac0));
                            c.g[A1][0] = 0x4d0000;
                            c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5a50));
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x37a4));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x37a0));
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5a60));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5ad0));
                            c.g[S0][0] = 0x4d0000;
                            c.swc1(6, (c.g[T1][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a70));
                            c.swc1(7, (c.g[T1][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a80));
                            c.swc1(13, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5a90));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = 0x4d0000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5aa0));
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = 0x4d0000;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5ab0));
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[T7][0] = 0x4d0000;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5ac0));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                            c.g[S0][0] = 0xffffffffffffffff;
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x57c0));
                            c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = 0x4d0000;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2440), c.g[S0][0]);
                            c.g[A2][0] = 0x4d0000;
                            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x379c));
                            c.g[T1][0] = 0x4d0000;
                            c.lwc1(9, (c.g[S4][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.g[S0][0] & 0xff;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3798));
                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                            c.swc1(2, (c.g[A3][0] as u32).wrapping_add(0xc));
                            c.g[T2][0] = c.g[T1][0];
                            c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x5b10));
                            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3794));
                            c.g[T7][0] = 0x4d0000;
                            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3790));
                            c.g[S0][0] = c.g[T7][0];
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x5ae0));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b20));
                            c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = 0x4d0000;
                            c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = c.g[T0][0];
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b50));
                            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x378c));
                            c.g[V1][0] = 0x4d0000;
                            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3788));
                            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x5af0));
                            c.swc1(5, (c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = 0x1;
                            c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A3][0] = 0x4d0000;
                            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3784));
                            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3780));
                            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x5b00));
                            c.swc1(7, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0x2;
                            c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x5b10));
                            c.g[A0][0] = c.g[V1][0];
                            c.sw((c.g[T2][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b40));
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x5b20));
                            c.g[T1][0] = 0x4;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            c.g[T7][0] = 0x6;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2440), c.g[V0][0]);
                            c.g[V0][0] = c.g[S4][0];
                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x5b30));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b30));
                            c.g[S4][0] = 0x8;
                            c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x5b40));
                            c.swc1(11, (c.g[A0][0] as u32).wrapping_add(0x8));
                            c.swc1(9, (c.g[A0][0] as u32).wrapping_add(0xc));
                            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x5b50));
                            c.sw((c.g[T2][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[T0][0] = 0x3;
                            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[T2][0] = 0x5;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                            c.g[S0][0] = 0x7;
                            c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2458), c.g[A2][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2480), c.g[T7][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2478), c.g[T2][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2460), c.g[A3][0]);
                            c.g[V1][0] = c.g[T7][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2468), c.g[T0][0]);
                            c.g[A3][0] = c.g[A2][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2470), c.g[T1][0]);
                            c.g[A0][0] = c.g[T2][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2450), 0u64);
                            c.g[T7][0] = 0x4d0000;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2488), c.g[S0][0]);
                            c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x5b50));
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2490), c.g[S4][0]);
                            c.g[A0][0] = c.g[A0][0] | 0x500;
                            c.g[T0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2460));
                            c.g[V0][0] = c.g[S0][0] & 0xff;
                            c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2468));
                            c.g[V1][0] = c.g[V1][0] | 0x600;
                            c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2470));
                            c.g[V0][0] = c.g[V0][0] | 0x700;
                            c.g[T2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2450));
                            c.g[A3][0] = c.g[A3][0] | 0x100;
                            c.swc1(9, (c.g[T7][0] as u32).wrapping_add(0xc));
                            c.g[A2][0] = c.g[A2][0] | 0x400;
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                            c.g[T0][0] = c.g[T0][0] | 0x200;
                            c.g[T1][0] = c.g[T1][0] | 0x300;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2470), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2478), c.g[A0][0]);
                            c.g[A1][0] = c.g[S4][0] & 0xff;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2480), c.g[V1][0]);
                            c.g[A1][0] = c.g[A1][0] | 0x800;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2488), c.g[V0][0]);
                            c.g[S0][0] = 0x9;
                            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x5b60));
                            c.g[S4][0] = 0xa;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2450), c.g[T2][0]);
                            c.g[V0][0] = 0xd;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2458), c.g[A3][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2460), c.g[T0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2468), c.g[T1][0]);
                            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x8));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x244c), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2448), 0u64);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2490), c.g[A1][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x2498), c.g[S0][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24a0), c.g[S4][0]);
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24a8), c.g[T5][0]);
                            c.g[V1][0] = c.g[S0][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24b0), c.g[S1][0]);
                            c.g[A1][0] = c.g[S4][0] & 0xff;
                            c.sb((c.g[GP][0] as u32).wrapping_add(0x24b8), c.g[V0][0]);
                            c.g[A2][0] = c.g[T5][0] & 0xff;
                            c.g[A0][0] = c.g[S1][0] & 0xff;
                            c.g[V1][0] = c.g[V1][0] | 0x900;
                            c.g[V0][0] = c.g[V0][0] & 0xff;
                            c.g[A1][0] = c.g[A1][0] | 0xa00;
                            c.g[A2][0] = c.g[A2][0] | 0xb00;
                            c.g[A0][0] = c.g[A0][0] | 0xc00;
                            c.g[V0][0] = c.g[V0][0] | 0xd00;
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x2498), c.g[V1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24a0), c.g[A1][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24a8), c.g[A2][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24b0), c.g[A0][0]);
                            c.sw((c.g[GP][0] as u32).wrapping_add(0x24b8), c.g[V0][0]);
                            'l_2f7538: loop {
                                c.cycles += 7;
                                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x1));
                                let t = c.g[FP][0] != c.g[T6][0];
                                c.g[V0][0] = 0x3f;
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f7538);
                                    }
                                    continue 'l_2f7538;
                                } else {
                                    c.cycles += 1;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    'l_2f7558: loop {
                                        c.cycles += 7;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        let t = c.g[V0][0] != c.g[V1][0];
                                        c.g[S0][0] = 0x3f;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002f7558);
                                            }
                                            continue 'l_2f7558;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x4f0000;
                                            c.g[S1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
                                            c.g[S2][0] = 0xffffffffffffffff;
                                            'l_2f7580: loop {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                                                c.g[RA][0] = sx(0x002f758c);
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                call_ptr(c, 0x003714b8, 0x002f758c)?;
                                                let t = c.g[S0][0] != c.g[S2][0];
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x2f0));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002f7580);
                                                    }
                                                    continue 'l_2f7580;
                                                } else {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x377c),
                                                    );
                                                    c.g[V0][0] = 0x7;
                                                    c.g[V1][0] = 0xffffffffffffffff;
                                                    c.swc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_add(0x24bc),
                                                    );
                                                    'l_2f75a8: loop {
                                                        c.cycles += 7;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        let t = c.g[V0][0] != c.g[V1][0];
                                                        c.g[A0][0] = 0x500000;
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x002f75a8);
                                                            }
                                                            continue 'l_2f75a8;
                                                        } else {
                                                            c.cycles += 265;
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x5510));
                                                            c.lwc1(
                                                                4,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x3778),
                                                            );
                                                            c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x54f0));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x54d0));
                                                            c.g[AT][0] = 0x3f000000;
                                                            c.mtc1(1, 5);
                                                            c.g[A2][0] = 0x500000;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 0);
                                                            c.swc1(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[A1][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x54b0));
                                                            c.swc1(
                                                                5,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[T3][0] = 0x500000;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x3774),
                                                            );
                                                            c.g[T6][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_sub(0x5510),
                                                            );
                                                            c.g[T5][0] = 0x500000;
                                                            c.swc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T3][0] = sx((c.g[T5][0] as u32)
                                                                .wrapping_sub(0x53f0));
                                                            c.swc1(
                                                                0,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_sub(0x54f0),
                                                            );
                                                            c.g[T7][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[S0][0] = 0x500000;
                                                            c.g[S1][0] = 0x500000;
                                                            c.g[T5][0] = sx((c.g[T7][0] as u32)
                                                                .wrapping_sub(0x53d0));
                                                            c.swc1(
                                                                0,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_sub(0x54d0),
                                                            );
                                                            c.g[T6][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_sub(0x53b0));
                                                            c.g[AT][0] = 0x43a00000;
                                                            c.mtc1(1, 3);
                                                            c.g[S0][0] = 0x500000;
                                                            c.g[AT][0] = 0x43700000;
                                                            c.mtc1(1, 2);
                                                            c.g[T7][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[A3][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_sub(0x5490),
                                                            );
                                                            c.g[A2][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_sub(0x5490));
                                                            c.swc1(
                                                                0,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_sub(0x54b0),
                                                            );
                                                            c.g[T0][0] = 0x500000;
                                                            c.swc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24c8),
                                                            );
                                                            c.g[A3][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_sub(0x5470));
                                                            c.swc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24cc),
                                                            );
                                                            c.g[T1][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24c4),
                                                                0u64,
                                                            );
                                                            c.g[T0][0] = sx((c.g[T1][0] as u32)
                                                                .wrapping_sub(0x5450));
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x24c0),
                                                                0u64,
                                                            );
                                                            c.g[T2][0] = 0x500000;
                                                            c.swc1(
                                                                4,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T1][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_sub(0x5430));
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T4][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T2][0] = sx((c.g[T4][0] as u32)
                                                                .wrapping_sub(0x5410));
                                                            c.swc1(
                                                                0,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[S4][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T4][0] = sx((c.g[S4][0] as u32)
                                                                .wrapping_sub(0x5390));
                                                            c.swc1(0, (c.g[SP][0] as u32));
                                                            c.g[V0][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[S1][0] = 0x4d0000;
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T7][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x5870));
                                                            c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5500));
                                                            c.ldl(
                                                                20,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(20, (c.g[A1][0] as u32));
                                                            c.ldl(
                                                                3,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(20, (c.g[A0][0] as u32));
                                                            c.sdl(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[T7][0] as u32)
                                                                .wrapping_add(0x5880));
                                                            c.g[S4][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x54e0));
                                                            c.ldl(
                                                                16,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(16, (c.g[V0][0] as u32));
                                                            c.ldl(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                16,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(16, (c.g[S4][0] as u32));
                                                            c.sdl(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = 0x4d0000;
                                                            c.g[S0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x5890));
                                                            c.g[T7][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x54c0));
                                                            c.ldl(
                                                                4,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, (c.g[S0][0] as u32));
                                                            c.ldl(
                                                                5,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, (c.g[T7][0] as u32));
                                                            c.sdl(
                                                                5,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S1][0] = 0x4d0000;
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[T7][0] = 0x500000;
                                                            c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x57b0));
                                                            c.g[A0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x54a0));
                                                            c.ldl(
                                                                20,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(20, (c.g[A1][0] as u32));
                                                            c.ldl(
                                                                2,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                2,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(20, (c.g[A0][0] as u32));
                                                            c.sdl(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S0][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S1][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[S4][0] = 0x500000;
                                                            c.g[A2][0] = 0x500000;
                                                            c.g[V0][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_sub(0x5470),
                                                            );
                                                            c.g[V1][0] = 0x4d0000;
                                                            c.sw(
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_sub(0x5450),
                                                            );
                                                            c.sw(
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_sub(0x5430),
                                                            );
                                                            c.sw(
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[S0][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_sub(0x5410),
                                                            );
                                                            c.sw(
                                                                (c.g[T2][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_sub(0x53f0),
                                                            );
                                                            c.sw(
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T0][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x57c0));
                                                            c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5480));
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, (c.g[A3][0] as u32));
                                                            c.ldl(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, (c.g[A2][0] as u32));
                                                            c.sdl(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[T7][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x57d0));
                                                            c.g[T3][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x5460));
                                                            c.ldl(
                                                                9,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(9, (c.g[T7][0] as u32));
                                                            c.ldl(
                                                                10,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                10,
                                                                (c.g[T7][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                9,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(9, (c.g[T3][0] as u32));
                                                            c.sdl(
                                                                10,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                10,
                                                                (c.g[T3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A1][0] = 0x4d0000;
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x57e0));
                                                            c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5440));
                                                            c.ldl(
                                                                17,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(17, (c.g[A0][0] as u32));
                                                            c.ldl(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                20,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                17,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(17, (c.g[V1][0] as u32));
                                                            c.sdl(
                                                                20,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                20,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T2][0] = 0x4d0000;
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[T1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x57f0));
                                                            c.g[T0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x5420));
                                                            c.ldl(
                                                                6,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(6, (c.g[T1][0] as u32));
                                                            c.ldl(
                                                                7,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                7,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(6, (c.g[T0][0] as u32));
                                                            c.sdl(
                                                                7,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                7,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[S4][0] = 0x4d0000;
                                                            c.g[S1][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_add(0x5800));
                                                            c.g[S0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5400));
                                                            c.ldl(
                                                                11,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(11, (c.g[S1][0] as u32));
                                                            c.ldl(
                                                                15,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                15,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                11,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(11, (c.g[S0][0] as u32));
                                                            c.sdl(
                                                                15,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                15,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.g[A2][0] = sx((c.g[S4][0] as u32)
                                                                .wrapping_add(0x5810));
                                                            c.g[A1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x53e0));
                                                            c.ldl(
                                                                2,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(2, (c.g[A2][0] as u32));
                                                            c.ldl(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                2,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(2, (c.g[A1][0] as u32));
                                                            c.sdl(
                                                                4,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                4,
                                                                (c.g[A1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_sub(0x53d0),
                                                            );
                                                            c.g[T0][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[T1][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_sub(0x53b0),
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.sw(
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.g[T2][0] = 0x4d0000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T1][0] as u32)
                                                                    .wrapping_sub(0x5390),
                                                            );
                                                            c.g[V1][0] = 0x500000;
                                                            c.swc1(
                                                                0,
                                                                (c.g[T4][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[T7][0] = 0x4d0000;
                                                            c.sw(
                                                                (c.g[T4][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[T4][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.g[T6][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_add(0x5820));
                                                            c.g[T5][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x53c0));
                                                            c.ldl(
                                                                11,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(11, (c.g[T6][0] as u32));
                                                            c.ldl(
                                                                12,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                12,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                11,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(11, (c.g[T5][0] as u32));
                                                            c.sdl(
                                                                12,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                12,
                                                                (c.g[T5][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = sx((c.g[T7][0] as u32)
                                                                .wrapping_add(0x5830));
                                                            c.g[S4][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x53a0));
                                                            c.ldl(
                                                                16,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(16, (c.g[V0][0] as u32));
                                                            c.ldl(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                17,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                16,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(16, (c.g[S4][0] as u32));
                                                            c.sdl(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                17,
                                                                (c.g[S4][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = 0x4d0000;
                                                            c.g[A3][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x5840));
                                                            c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x5380));
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, (c.g[A3][0] as u32));
                                                            c.ldl(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                4,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, (c.g[A2][0] as u32));
                                                            c.sdl(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xa0),
                                                            );
                                                            if t {
                                                                break 'b_2f7a20;
                                                            } else {
                                                                break 'b_2f79e8;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 0x4f0000;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x500000;
                    if t {
                        break 'b_2f7a1c;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5c90));
                        let t = c.g[V1][0] == c.g[S0][0];
                        c.g[S1][0] = c.g[V1][0];
                        if t {
                            break 'b_2f7a1c;
                        } else {
                            c.cycles += 1;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2f0));
                            'l_2f7a08: loop {
                                c.cycles += 5;
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x002f7a14);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002d56b0,
                                    f: ee::s09::VisualEffectsMan_EmitterHolderDtor,
                                }
                                .call(c, 0x002f7a14)?;
                                let t = c.g[S0][0] != c.g[S1][0];
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2f0));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002f7a08);
                                    }
                                    continue 'l_2f7a08;
                                } else {
                                    break 'b_2f7a1c;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            break 'b_2f7a20;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, Rng};
    use std::ptr::addr_of_mut;

    /// Gate of the oc1054 fakes (other tests' machines fall back to the real callees).
    pub(crate) const OC1054_GATE: u32 = crate::difftest::DATA + 0xf_0000;
    pub(crate) const OC1054_MAGIC: u32 = 0x1054_0c0e;

    macro_rules! oc1054_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1054_GATE) != OC1054_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1054_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1054_fakes! {
        oc1054_fake_emitter_ctor, 0x0037_14b8, ee::s12::OVERRIDE_003714B8, ee::s12::f_003714b8__generated;
        oc1054_fake_holder_dtor, 0x002d_56b0, ee::s09::OVERRIDE_002D56B0, ee::s09::f_002d56b0__generated;
    }

    /// `a1 == 0xffff` (init when `a0 != 0`, destroy when `a0 == 0`) or another
    /// mode (early exit, every sixth seed); `calls`: the callee call index at
    /// which a fake may unwind.
    fn oc1054_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        oc1054_hook();
        let mut r = machine(c, seed, 0x1054, calls);
        elf_image(c);
        c.bus.write32(OC1054_GATE, OC1054_MAGIC);
        c.g[A0][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64 | 1
        };
        c.g[A1][0] = if seed % 6 == 5 {
            r.next_u32() as u64
        } else {
            0xffff
        };
        r
    }

    /// Early exit, destroy loop and init (with its two constructor loops),
    /// with fakes that unwind at random callee calls.
    #[test]
    fn visual_effects_man_static_init_destroy_matches() {
        check(
            ee::s09::f_002f6b88__generated,
            visual_effects_man_static_init_destroy,
            300,
            &|c, seed| {
                oc1054_setup(c, seed, 130);
            },
        );
    }

    /// The seeds reach the early exit, the destroy loop and the init path.
    #[test]
    fn visual_effects_man_static_init_destroy_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..12 {
            let mut c = crate::difftest::fresh();
            oc1054_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s09::f_002f6b88__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 3, "only {} distinct paths", seen.len());
    }

    /// Events delivered or unwinding at the loop back edges.
    #[test]
    fn visual_effects_man_static_init_destroy_events() {
        check_events(
            ee::s09::f_002f6b88__generated,
            visual_effects_man_static_init_destroy,
            120,
            &|c, seed| {
                let mut r = oc1054_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x002f_6b88)
            },
        );
    }
}
