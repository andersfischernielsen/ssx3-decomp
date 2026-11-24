//! Staging module oc498 (wave wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc498`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{guest, sp, Callee, Flow, Frame, Unwound};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0017_5a20,
        camera_trigger_man_static_init_destroy,
        "CameraTriggerMan_StaticInitDestroy",
    ),
];

/// `CameraTriggerMan_ctor` (0x16c800): the manager's constructor, returns to
/// the static-init code at 0x176228.
const CTM_CTOR: Callee = Callee {
    addr: 0x0016_c800,
    f: ee::s02::f_0016c800,
};

/// `CameraTriggerMan_dtor` (0x16c888): the manager's destructor, returns to
/// the static-init code at 0x176240.
const CTM_DTOR: Callee = Callee {
    addr: 0x0016_c888,
    f: ee::s02::f_0016c888,
};

/// Effective address `rs + off` as the generated code forms it
/// (`(c.g[rs][0] as u32).wrapping_add(off)`).
#[inline(always)]
pub(crate) fn cts_ea(c: &Cpu, rs: usize, off: i32) -> u32 {
    (c.g[rs][0] as u32).wrapping_add(off as u32)
}

/// `void CameraTriggerMan_StaticInitDestroy(int init, uint prio)` (0x175a20),
/// the static constructor and destructor of the camera trigger manager. Only
/// `prio == 0xffff` does anything: `init != 0` writes the initial values of
/// the trigger-manager globals (the float tables at 0x4c54b0..0x4c5820, read
/// from the constant pool at gp-0x6304..gp-0x61f4) and then constructs the
/// manager at `gCamTriggerMan`; `init == 0` destroys it. Other priorities
/// return at once.
pub fn camera_trigger_man_static_init_destroy(c: &mut Cpu) {
    guest(c, |c: &mut Cpu| -> Flow {
        'b_176244: {
        'b_176240: {
        c.cycles += 13;
        c.g[SP][0] = sx(cts_ea(c, SP, -0xa0));
        c.g[V0][0] = 0xffff;
        c.sq(16, cts_ea(c, SP, 0x90));
        c.sq(17, cts_ea(c, SP, 0x80));
        c.sq(18, cts_ea(c, SP, 0x70));
        c.sq(19, cts_ea(c, SP, 0x60));
        c.sq(20, cts_ea(c, SP, 0x50));
        c.sq(21, cts_ea(c, SP, 0x40));
        c.sq(22, cts_ea(c, SP, 0x30));
        c.sq(23, cts_ea(c, SP, 0x20));
        c.sq(30, cts_ea(c, SP, 0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sd(cts_ea(c, SP, 0), c.g[RA][0]);
        if t {
        break 'b_176240;
        } else {
        'b_176230: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[T5][0] = 0x4c0000;
        if t {
        break 'b_176230;
        } else {
        c.cycles += 501;
        c.lwc1(9, cts_ea(c, GP, -0x6304));
        c.lwc1(10, cts_ea(c, GP, -0x6300));
        c.g[T6][0] = 0x4c0000;
        c.lwc1(11, cts_ea(c, GP, -0x62fc));
        c.g[T7][0] = 0x4c0000;
        c.lwc1(7, cts_ea(c, GP, -0x62f8));
        c.g[V0][0] = sx(cts_ea(c, T7, 0x54e0));
        c.swc1(9, cts_ea(c, T5, 0x54c0));
        c.g[S1][0] = 0x4c0000;
        c.swc1(10, cts_ea(c, T6, 0x54d0));
        c.g[S2][0] = 0x4c0000;
        c.swc1(11, cts_ea(c, T7, 0x54e0));
        c.g[S3][0] = 0x4c0000;
        c.lwc1(8, cts_ea(c, GP, -0x62f4));
        c.g[A0][0] = sx(cts_ea(c, S3, 0x5510));
        c.lwc1(4, cts_ea(c, GP, -0x62f0));
        c.g[S5][0] = 0x4c0000;
        c.lwc1(5, cts_ea(c, GP, -0x62ec));
        c.g[S4][0] = 0x4c0000;
        c.swc1(7, cts_ea(c, V0, 0xc));
        c.g[T3][0] = 0x4c0000;
        c.lwc1(2, cts_ea(c, GP, -0x62e8));
        c.g[A1][0] = sx(cts_ea(c, S5, 0x54b0));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[A3][0] = sx(cts_ea(c, T5, 0x54c0));
        c.swc1(8, cts_ea(c, S1, 0x54f0));
        c.g[T1][0] = sx(cts_ea(c, T6, 0x54d0));
        c.swc1(4, cts_ea(c, S2, 0x5500));
        c.g[T0][0] = sx(cts_ea(c, S2, 0x5500));
        c.swc1(5, cts_ea(c, S3, 0x5510));
        c.g[V1][0] = sx(cts_ea(c, S1, 0x54f0));
        c.lwc1(3, cts_ea(c, GP, -0x62e4));
        c.g[S2][0] = 0x4c0000;
        c.lwc1(1, cts_ea(c, GP, -0x62e0));
        c.g[S1][0] = sx(cts_ea(c, S2, 0x5590));
        c.swc1(2, cts_ea(c, A0, 0xc));
        c.g[S3][0] = 0x4c0000;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 6);
        c.g[S2][0] = sx(cts_ea(c, S3, 0x55a0));
        c.swc1(3, cts_ea(c, S4, 0x5520));
        c.g[FP][0] = 0x4c0000;
        c.swc1(1, cts_ea(c, T3, 0x5530));
        c.g[A2][0] = sx(cts_ea(c, S4, 0x5520));
        c.swc1(0, cts_ea(c, S5, 0x54b0));
        c.g[S3][0] = sx(cts_ea(c, FP, 0x55b0));
        c.swc1(6, cts_ea(c, A1, 0xc));
        c.g[FP][0] = 0x4c0000;
        c.sw(cts_ea(c, A3, 0xc), 0u64);
        c.g[T4][0] = sx(cts_ea(c, T3, 0x5530));
        c.lwc1(17, cts_ea(c, GP, -0x62dc));
        c.g[T5][0] = 0x4c0000;
        c.lwc1(18, cts_ea(c, GP, -0x62d8));
        c.g[T2][0] = sx(cts_ea(c, T5, 0x5540));
        c.sw(cts_ea(c, T1, 0xc), 0u64);
        c.g[S7][0] = 0x4c0000;
        c.lwc1(13, cts_ea(c, GP, -0x62d4));
        c.g[T7][0] = sx(cts_ea(c, S7, 0x5570));
        c.lwc1(14, cts_ea(c, GP, -0x62d0));
        c.g[T9][0] = 0x4c0000;
        c.swc1(17, cts_ea(c, V0, 0x4));
        c.g[T5][0] = sx(cts_ea(c, T9, 0x5550));
        c.swc1(18, cts_ea(c, V0, 0x8));
        c.g[T8][0] = 0x4c0000;
        c.swc1(13, cts_ea(c, V1, 0x4));
        c.g[V0][0] = 0x4c0000;
        c.swc1(14, cts_ea(c, V1, 0x8));
        c.g[S5][0] = sx(cts_ea(c, V0, 0x55c0));
        c.sw(cts_ea(c, V1, 0xc), 0u64);
        c.g[T6][0] = sx(cts_ea(c, T8, 0x5560));
        c.lwc1(15, cts_ea(c, GP, -0x62cc));
        c.g[V1][0] = 0x4c0000;
        c.lwc1(16, cts_ea(c, GP, -0x62c8));
        c.g[V0][0] = sx(cts_ea(c, V1, 0x5600));
        c.sw(cts_ea(c, T0, 0xc), 0u64);
        c.g[V1][0] = sx(cts_ea(c, FP, 0x5620));
        c.lwc1(12, cts_ea(c, GP, -0x62c4));
        c.g[FP][0] = 0x4c0000;
        c.swc1(15, cts_ea(c, A0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(16, cts_ea(c, A0, 0x8));
        c.g[S0][0] = sx(cts_ea(c, S6, 0x5580));
        c.swc1(12, cts_ea(c, A2, 0x4));
        c.g[A0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, A2, 0xc));
        c.g[S4][0] = sx(cts_ea(c, A0, 0x5610));
        c.swc1(0, cts_ea(c, A1, 0x4));
        c.g[A0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, A1, 0x8));
        c.g[T3][0] = sx(cts_ea(c, A0, 0x5630));
        c.swc1(0, cts_ea(c, A3, 0x4));
        c.g[A1][0] = sx(cts_ea(c, FP, 0x5650));
        c.swc1(0, cts_ea(c, A3, 0x8));
        c.g[FP][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T1, 0x4));
        c.g[A0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T1, 0x8));
        c.swc1(0, cts_ea(c, T0, 0x4));
        c.g[T1][0] = sx(cts_ea(c, FP, 0x5680));
        c.swc1(0, cts_ea(c, T0, 0x8));
        c.g[FP][0] = 0x4c0000;
        c.lwc1(19, cts_ea(c, GP, -0x62c0));
        c.swc1(0, cts_ea(c, A2, 0x8));
        c.lwc1(2, cts_ea(c, GP, -0x62bc));
        c.g[A2][0] = sx(cts_ea(c, A0, 0x5660));
        c.swc1(19, cts_ea(c, T4, 0x4));
        c.g[A0][0] = 0x4c0000;
        c.lwc1(1, cts_ea(c, GP, -0x62b8));
        c.g[A3][0] = sx(cts_ea(c, A0, 0x56b0));
        c.swc1(2, cts_ea(c, FP, 0x5540));
        c.g[A0][0] = 0x4c0000;
        c.lwc1(8, cts_ea(c, GP, -0x62b4));
        c.g[FP][0] = 0x4c0000;
        c.swc1(1, cts_ea(c, T2, 0xc));
        c.g[T0][0] = sx(cts_ea(c, A0, 0x5710));
        c.lwc1(3, cts_ea(c, GP, -0x62b0));
        c.g[A0][0] = 0x4c0000;
        c.swc1(8, cts_ea(c, T7, 0xc));
        c.lwc1(4, cts_ea(c, GP, -0x62ac));
        c.lwc1(5, cts_ea(c, GP, -0x62a8));
        c.swc1(3, cts_ea(c, T4, 0x8));
        c.sw(cts_ea(c, T4, 0xc), 0u64);
        c.swc1(4, cts_ea(c, T2, 0x4));
        c.g[T4][0] = sx(cts_ea(c, FP, 0x5730));
        c.swc1(5, cts_ea(c, T2, 0x8));
        c.g[FP][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T9, 0x5550));
        c.g[T2][0] = sx(cts_ea(c, A0, 0x5740));
        c.swc1(0, cts_ea(c, T5, 0xc));
        c.g[A0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T8, 0x5560));
        c.sw(cts_ea(c, T6, 0xc), 0u64);
        c.lwc1(7, cts_ea(c, GP, -0x62a4));
        c.swc1(0, cts_ea(c, S7, 0x5570));
        c.swc1(7, cts_ea(c, T7, 0x8));
        c.swc1(0, cts_ea(c, S6, 0x5580));
        c.swc1(0, cts_ea(c, S0, 0xc));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x5590));
        c.swc1(0, cts_ea(c, S1, 0xc));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x55a0));
        c.sw(cts_ea(c, S2, 0xc), 0u64);
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, FP, 0x55b0));
        c.swc1(0, cts_ea(c, T5, 0x4));
        c.g[FP][0] = 0xffffffffffff0000;
        c.swc1(0, cts_ea(c, T5, 0x8));
        c.g[FP][0] = c.g[FP][0] | 0xff00;
        c.sw(cts_ea(c, T6, 0x4), 0u64);
        c.g[T5][0] = 0x4c0000;
        c.sw(cts_ea(c, T6, 0x8), 0u64);
        c.swc1(8, cts_ea(c, T7, 0x4));
        c.g[T6][0] = sx(cts_ea(c, S6, 0x57d0));
        c.swc1(0, cts_ea(c, S0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S0, 0x8));
        c.swc1(0, cts_ea(c, S1, 0x4));
        c.g[S0][0] = sx(cts_ea(c, T5, 0x57c0));
        c.swc1(0, cts_ea(c, S1, 0x8));
        c.g[T5][0] = 0x4c0000;
        c.sw(cts_ea(c, S2, 0x4), 0u64);
        c.sw(cts_ea(c, S2, 0x8), 0u64);
        c.g[S2][0] = sx(cts_ea(c, A0, 0x57b0));
        c.sw(cts_ea(c, S3, 0x4), 0u64);
        c.sw(cts_ea(c, S3, 0x8), 0u64);
        c.g[A0][0] = 0x4c0000;
        c.sw(cts_ea(c, S3, 0xc), 0u64);
        c.g[A0][0] = sx(cts_ea(c, A0, 0x55f0));
        c.lwc1(2, cts_ea(c, GP, -0x62a0));
        c.g[S3][0] = 0x4c0000;
        c.swc1(6, cts_ea(c, A0, 0xc));
        c.lwc1(1, cts_ea(c, GP, -0x629c));
        c.g[A0][0] = 0x4c0000;
        c.swc1(2, cts_ea(c, V0, 0xc));
        c.g[T7][0] = sx(cts_ea(c, A0, 0x57f0));
        c.swc1(1, cts_ea(c, T5, 0x5620));
        c.swc1(0, cts_ea(c, S6, 0x55c0));
        c.g[T5][0] = sx(cts_ea(c, S3, 0x57e0));
        c.swc1(0, cts_ea(c, S5, 0xc));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x55d0));
        c.g[S3][0] = 0x4c0000;
        c.g[A0][0] = c.g[S6][0];
        c.g[S1][0] = sx(cts_ea(c, S3, 0x5800));
        c.g[A0][0] = sx(cts_ea(c, A0, 0x55d0));
        c.g[S6][0] = 0x4c0000;
        c.swc1(6, cts_ea(c, A0, 0xc));
        c.g[S3][0] = sx(cts_ea(c, S6, 0x5820));
        c.g[A0][0] = 0x4c0000;
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, A0, 0x55e0));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x55e0));
        c.swc1(6, cts_ea(c, S6, 0xc));
        c.g[A0][0] = 0x4c0000;
        c.g[S6][0] = 0x4c0000;
        c.lwc1(8, cts_ea(c, GP, -0x6298));
        c.swc1(0, cts_ea(c, S6, 0x55f0));
        c.g[A0][0] = sx(cts_ea(c, A0, 0x5830));
        c.lwc1(7, cts_ea(c, GP, -0x6294));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x5600));
        c.swc1(8, cts_ea(c, V0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(7, cts_ea(c, V0, 0x8));
        c.lwc1(5, cts_ea(c, GP, -0x6290));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0x5610));
        c.lwc1(4, cts_ea(c, GP, -0x628c));
        c.g[V0][0] = 0x4c0000;
        c.lwc1(3, cts_ea(c, GP, -0x6288));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x55d0));
        c.swc1(5, cts_ea(c, S4, 0x8));
        c.sw(cts_ea(c, S4, 0xc), 0u64);
        c.swc1(4, cts_ea(c, V1, 0x4));
        c.swc1(3, cts_ea(c, V1, 0x8));
        c.sw(cts_ea(c, V1, 0xc), 0u64);
        c.g[V1][0] = 0x4c0000;
        c.lwc1(2, cts_ea(c, GP, -0x6284));
        c.swc1(0, cts_ea(c, V1, 0x5630));
        c.swc1(0, cts_ea(c, T3, 0xc));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x5640));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x55e0));
        c.swc1(0, cts_ea(c, S5, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S5, 0x8));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x55f0));
        c.swc1(6, cts_ea(c, V0, 0x4));
        c.swc1(0, cts_ea(c, V0, 0x8));
        c.swc1(6, cts_ea(c, V1, 0x4));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x8));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5640));
        c.swc1(6, cts_ea(c, S6, 0x4));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x8));
        c.swc1(0, cts_ea(c, S4, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T3, 0x4));
        c.swc1(0, cts_ea(c, T3, 0x8));
        c.swc1(0, cts_ea(c, V0, 0x4));
        c.lwc1(1, cts_ea(c, GP, -0x6280));
        c.swc1(2, cts_ea(c, A2, 0xc));
        c.lwc1(3, cts_ea(c, GP, -0x627c));
        c.swc1(1, cts_ea(c, T1, 0xc));
        c.swc1(3, cts_ea(c, V0, 0x8));
        c.sw(cts_ea(c, V0, 0xc), 0u64);
        c.lwc1(4, cts_ea(c, GP, -0x6278));
        c.g[V0][0] = 0x4c0000;
        c.lwc1(5, cts_ea(c, GP, -0x6274));
        c.swc1(0, cts_ea(c, V1, 0x5650));
        c.swc1(4, cts_ea(c, A1, 0x4));
        c.g[V1][0] = c.g[V0][0];
        c.swc1(5, cts_ea(c, A1, 0x8));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5670));
        c.sw(cts_ea(c, A1, 0xc), 0u64);
        c.lwc1(6, cts_ea(c, GP, -0x6270));
        c.lwc1(7, cts_ea(c, GP, -0x626c));
        c.swc1(0, cts_ea(c, S6, 0x5660));
        c.swc1(6, cts_ea(c, A2, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(7, cts_ea(c, A2, 0x8));
        c.lwc1(8, cts_ea(c, GP, -0x6268));
        c.swc1(0, cts_ea(c, V0, 0x5670));
        c.swc1(8, cts_ea(c, V1, 0x8));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0xc));
        c.lwc1(9, cts_ea(c, GP, -0x6264));
        c.g[V1][0] = c.g[V0][0];
        c.swc1(0, cts_ea(c, S6, 0x5680));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5690));
        c.swc1(9, cts_ea(c, T1, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0x5690));
        c.swc1(0, cts_ea(c, V1, 0xc));
        c.g[V0][0] = c.g[S6][0];
        c.swc1(0, cts_ea(c, S6, 0x56a0));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x56a0));
        c.swc1(0, cts_ea(c, V0, 0xc));
        c.g[V1][0] = 0x4c0000;
        c.lwc1(10, cts_ea(c, GP, -0x6260));
        c.g[S6][0] = 0x4c0000;
        c.lwc1(11, cts_ea(c, GP, -0x625c));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x56b0));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5670));
        c.swc1(10, cts_ea(c, A3, 0x4));
        c.g[V1][0] = 0x4c0000;
        c.swc1(11, cts_ea(c, A3, 0x8));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5690));
        c.sw(cts_ea(c, A3, 0xc), 0u64);
        c.lwc1(12, cts_ea(c, GP, -0x6258));
        c.swc1(0, cts_ea(c, S6, 0x56c0));
        c.swc1(12, cts_ea(c, V0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T1, 0x8));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x56a0));
        c.swc1(0, cts_ea(c, V1, 0x4));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x8));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x56c0));
        c.swc1(0, cts_ea(c, S6, 0x4));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x8));
        c.swc1(0, cts_ea(c, V0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0x8));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x5720));
        c.lwc1(4, cts_ea(c, GP, -0x6254));
        c.swc1(0, cts_ea(c, V0, 0xc));
        c.lwc1(3, cts_ea(c, GP, -0x6250));
        c.g[V0][0] = 0x4c0000;
        c.swc1(4, cts_ea(c, V1, 0x5700));
        c.lwc1(2, cts_ea(c, GP, -0x624c));
        c.g[V1][0] = c.g[V0][0];
        c.swc1(3, cts_ea(c, T0, 0xc));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x56d0));
        c.lwc1(1, cts_ea(c, GP, -0x6248));
        c.swc1(2, cts_ea(c, S6, 0xc));
        c.swc1(1, cts_ea(c, T2, 0xc));
        c.g[S6][0] = 0x4c0000;
        c.lwc1(5, cts_ea(c, GP, -0x6244));
        c.swc1(0, cts_ea(c, V0, 0x56d0));
        c.swc1(5, cts_ea(c, V1, 0x4));
        c.g[V0][0] = c.g[S6][0];
        c.sw(cts_ea(c, V1, 0xc), 0u64);
        c.g[V0][0] = sx(cts_ea(c, V0, 0x56e0));
        c.swc1(0, cts_ea(c, S6, 0x56e0));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0xc));
        c.g[S6][0] = c.g[V1][0];
        c.swc1(0, cts_ea(c, V1, 0x56f0));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x56f0));
        c.g[V0][0] = 0x4c0000;
        c.sw(cts_ea(c, S6, 0xc), 0u64);
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5700));
        c.lwc1(6, cts_ea(c, GP, -0x6240));
        c.sw(cts_ea(c, V0, 0xc), 0u64);
        c.g[V1][0] = 0x4c0000;
        c.lwc1(7, cts_ea(c, GP, -0x623c));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x5710));
        c.g[V0][0] = c.g[S6][0];
        c.swc1(6, cts_ea(c, T0, 0x4));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5720));
        c.swc1(7, cts_ea(c, T0, 0x8));
        c.g[V1][0] = 0x4c0000;
        c.lwc1(8, cts_ea(c, GP, -0x6238));
        c.swc1(0, cts_ea(c, S6, 0x5720));
        c.swc1(8, cts_ea(c, V0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.lwc1(9, cts_ea(c, GP, -0x6234));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x5730));
        c.swc1(9, cts_ea(c, T4, 0x8));
        c.g[V1][0] = 0x4c0000;
        c.sw(cts_ea(c, T4, 0xc), 0u64);
        c.g[V1][0] = sx(cts_ea(c, V1, 0x56d0));
        c.lwc1(10, cts_ea(c, GP, -0x6230));
        c.swc1(0, cts_ea(c, S6, 0x5740));
        c.swc1(10, cts_ea(c, T2, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0x5750));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x56e0));
        c.swc1(0, cts_ea(c, V1, 0x8));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x8));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x56f0));
        c.swc1(0, cts_ea(c, S6, 0x4));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0x4));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5700));
        c.sw(cts_ea(c, V0, 0x8), 0u64);
        c.g[S6][0] = 0x4c0000;
        c.sw(cts_ea(c, V1, 0x4), 0u64);
        c.g[S6][0] = sx(cts_ea(c, S6, 0x5720));
        c.sw(cts_ea(c, V1, 0x8), 0u64);
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x8));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5750));
        c.swc1(0, cts_ea(c, T4, 0x4));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, T2, 0x8));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5760));
        c.swc1(0, cts_ea(c, V0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.lwc1(2, cts_ea(c, GP, -0x622c));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x57a0));
        c.swc1(12, cts_ea(c, V1, 0xc));
        c.lwc1(1, cts_ea(c, GP, -0x6228));
        c.g[V1][0] = 0x4c0000;
        c.swc1(2, cts_ea(c, S6, 0xc));
        c.swc1(1, cts_ea(c, S2, 0xc));
        c.g[S6][0] = c.g[V1][0];
        c.swc1(0, cts_ea(c, V0, 0xc));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x5770));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x5770));
        c.swc1(0, cts_ea(c, V0, 0x5760));
        c.g[V0][0] = 0x4c0000;
        c.sw(cts_ea(c, S6, 0xc), 0u64);
        c.g[V1][0] = c.g[V0][0];
        c.swc1(0, cts_ea(c, V0, 0x5780));
        c.g[S6][0] = 0x4c0000;
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5780));
        c.sw(cts_ea(c, V1, 0xc), 0u64);
        c.g[V0][0] = c.g[S6][0];
        c.swc1(0, cts_ea(c, S6, 0x5790));
        c.g[V1][0] = 0x4c0000;
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5790));
        c.lwc1(3, cts_ea(c, GP, -0x6224));
        c.sw(cts_ea(c, V0, 0xc), 0u64);
        c.g[S6][0] = c.g[V1][0];
        c.swc1(0, cts_ea(c, V1, 0x57a0));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x57a0));
        c.swc1(3, cts_ea(c, S6, 0x4));
        c.g[V0][0] = 0x4c0000;
        c.lwc1(4, cts_ea(c, GP, -0x6220));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V0, 0x57b0));
        c.g[S6][0] = 0x4c0000;
        c.swc1(4, cts_ea(c, S2, 0x8));
        c.g[V0][0] = 0x4c0000;
        c.lwc1(5, cts_ea(c, GP, -0x621c));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5750));
        c.swc1(0, cts_ea(c, V1, 0x57c0));
        c.swc1(5, cts_ea(c, S0, 0x8));
        c.g[V1][0] = 0x4c0000;
        c.sw(cts_ea(c, S0, 0xc), 0u64);
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5760));
        c.lwc1(6, cts_ea(c, GP, -0x6218));
        c.lwc1(7, cts_ea(c, GP, -0x6214));
        c.swc1(0, cts_ea(c, S6, 0x57d0));
        c.swc1(6, cts_ea(c, T6, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(7, cts_ea(c, T6, 0x8));
        c.g[S6][0] = sx(cts_ea(c, S6, 0x5770));
        c.swc1(0, cts_ea(c, V0, 0x8));
        c.swc1(12, cts_ea(c, V1, 0x4));
        c.g[V0][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, V1, 0x8));
        c.g[V0][0] = sx(cts_ea(c, V0, 0x5780));
        c.sw(cts_ea(c, S6, 0x8), 0u64);
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S6, 0x4));
        c.g[V1][0] = sx(cts_ea(c, V1, 0x5790));
        c.swc1(0, cts_ea(c, V0, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.sw(cts_ea(c, V0, 0x8), 0u64);
        c.g[S6][0] = sx(cts_ea(c, S6, 0x57a0));
        c.swc1(0, cts_ea(c, V1, 0x4));
        c.g[V0][0] = 0xffffffffffffffff;
        c.sw(cts_ea(c, V1, 0x8), 0u64);
        c.swc1(0, cts_ea(c, S6, 0x8));
        c.g[V1][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S2, 0x4));
        c.g[S6][0] = 0x4c0000;
        c.swc1(0, cts_ea(c, S0, 0x4));
        c.g[S2][0] = 0x4c0000;
        c.sw(cts_ea(c, T6, 0xc), 0u64);
        c.lwc1(2, cts_ea(c, GP, -0x6210));
        c.sb(cts_ea(c, GP, 0x1d00), c.g[V0][0]);
        c.lwc1(1, cts_ea(c, GP, -0x620c));
        c.swc1(2, cts_ea(c, T7, 0xc));
        c.g[V0][0] = c.g[V0][0] & 0xff;
        c.swc1(1, cts_ea(c, S1, 0xc));
        c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
        c.lwc1(4, cts_ea(c, GP, -0x6208));
        c.g[FP][0] = c.g[S6][0];
        c.lwc1(3, cts_ea(c, GP, -0x6204));
        c.g[FP][0] = sx(cts_ea(c, FP, 0x5810));
        c.swc1(0, cts_ea(c, V1, 0x57e0));
        c.swc1(4, cts_ea(c, T5, 0x4));
        c.g[V1][0] = 0x4c0000;
        c.swc1(3, cts_ea(c, T5, 0x8));
        c.sw(cts_ea(c, T5, 0xc), 0u64);
        c.lwc1(8, cts_ea(c, GP, -0x6200));
        c.g[T5][0] = 0x4c0000;
        c.lwc1(5, cts_ea(c, GP, -0x61fc));
        c.swc1(0, cts_ea(c, T5, 0x57f0));
        c.swc1(8, cts_ea(c, T7, 0x4));
        c.swc1(5, cts_ea(c, T7, 0x8));
        c.lwc1(6, cts_ea(c, GP, -0x61f8));
        c.lwc1(7, cts_ea(c, GP, -0x61f4));
        c.swc1(0, cts_ea(c, S2, 0x5800));
        c.swc1(6, cts_ea(c, S1, 0x4));
        c.swc1(7, cts_ea(c, S1, 0x8));
        c.swc1(0, cts_ea(c, S6, 0x5810));
        c.sw(cts_ea(c, FP, 0xc), 0u64);
        c.swc1(0, cts_ea(c, V1, 0x5820));
        c.swc1(0, cts_ea(c, S3, 0x4));
        c.sw(cts_ea(c, S3, 0xc), 0u64);
        c.sw(cts_ea(c, GP, 0x1d00), c.g[V0][0]);
        c.sw(cts_ea(c, FP, 0x4), 0u64);
        c.swc1(0, cts_ea(c, FP, 0x8));
        c.sw(cts_ea(c, S3, 0x8), 0u64);
        CTM_CTOR.call(c, 0x00176228)?;
        let t = true;
        c.lq(16, cts_ea(c, SP, 0x90));
        if t {
        break 'b_176244;
        } else {
        break 'b_176230;
        }
        }
        }
        c.cycles += 4;
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = 0x2;
        c.g[A0][0] = sx(cts_ea(c, A0, 0x5830));
        CTM_DTOR.call(c, 0x00176240)?;
        break 'b_176240;
        }
        }
        c.cycles += 1;
        c.lq(16, cts_ea(c, SP, 0x90));
        break 'b_176244;
        }
        c.cycles += 11;
        c.lq(17, cts_ea(c, SP, 0x80));
        c.lq(18, cts_ea(c, SP, 0x70));
        c.lq(19, cts_ea(c, SP, 0x60));
        c.lq(20, cts_ea(c, SP, 0x50));
        c.lq(21, cts_ea(c, SP, 0x40));
        c.lq(22, cts_ea(c, SP, 0x30));
        c.lq(23, cts_ea(c, SP, 0x20));
        c.lq(30, cts_ea(c, SP, 0x10));
        c.g[RA][0] = c.ld(cts_ea(c, SP, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(cts_ea(c, SP, 0xa0));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the constructor and destructor fakes.
    const OC498_CTRL: u32 = DATA + 0x200000;
    const OC498_MAGIC: u32 = 0x498c_0de5;

    static OC498_PREV_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC498_PREV_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// The constructor, gated: with the gate on it only returns to `ra`.
    fn oc498_fake_ctor(c: &mut Cpu) {
        if c.bus.read32(OC498_CTRL) != OC498_MAGIC {
            return fakes::fall_back(c, &OC498_PREV_CTOR, ee::s02::f_0016c800__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// The destructor, gated: with the gate on it only returns to `ra`.
    fn oc498_fake_dtor(c: &mut Cpu) {
        if c.bus.read32(OC498_CTRL) != OC498_MAGIC {
            return fakes::fall_back(c, &OC498_PREV_DTOR, ee::s02::f_0016c888__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn oc498_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C800),
            oc498_fake_ctor,
            &OC498_PREV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C888),
            oc498_fake_dtor,
            &OC498_PREV_DTOR,
        );
    }

    #[test]
    fn camera_trigger_man_static_init_destroy_matches() {
        oc498_install_fakes();
        check(
            ee::s02::f_00175a20__generated,
            camera_trigger_man_static_init_destroy,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x498, 0);
                c.bus.write32(OC498_CTRL, OC498_MAGIC);
                c.pc = 0x0017_5a20;
                // init: 0 (destroy), or a non-zero value (initialise).
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 };
                // prio: 0xffff for most seeds, anything else returns at once.
                c.g[A1][0] = if seed % 4 == 0 {
                    (r.next_u32() as u64) & 0xfffe
                } else {
                    0xffff
                };
            },
        );
    }
}
