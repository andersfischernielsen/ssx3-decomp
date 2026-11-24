//! Staging module oc487 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc487`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0015_b028, be_static_init_destroy, "BE_StaticInitDestroy")];

/// The address `off(rs)` from the register's current value (the generated
/// code's `(c.g[rs][0] as u32).wrapping_add(off)`).
#[inline(always)]
fn bsi_ea(c: &Cpu, rs: usize, off: i32) -> u32 {
    (c.g[rs][0] as u32).wrapping_add(off as u32)
}

/// `void BE_StaticInitDestroy(int param_1, uint param_2)` (0x15b028), the
/// static constructor and destructor of the back end (`BE_GlobalCtors` passes
/// `1, 0xffff`, `BE_GlobalDtors` passes `0, 0xffff`). Only `param_2 == 0xffff`
/// does anything: `param_1 != 0` writes the initial values of the statics
/// (the rewards and option tables, the character profile slots, the name
/// string and the bolt DB), `param_1 == 0` destroys the profile, rewards,
/// bolt and name-string singletons.
pub fn be_static_init_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        'b_15bacc: {
            c.g[V0][0] = 0xffff;
            frame.save(c, 16, 0x90);
            frame.save(c, 17, 0x80);
            frame.save(c, 18, 0x70);
            frame.save(c, 19, 0x60);
            frame.save(c, 20, 0x50);
            frame.save(c, 21, 0x40);
            frame.save(c, 22, 0x30);
            frame.save(c, 23, 0x20);
            frame.save(c, 30, 0x10);
            let t = c.g[A1][0] != c.g[V0][0];
            frame.save_ra(c, 0);
            if t {
                break 'b_15bacc;
            } else {
                'b_15ba2c: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[T5][0] = 0x4a0000;
                    if t {
                        break 'b_15ba2c;
                    } else {
                        // param_1 != 0: reset the statics to their initial values.
                        c.cycles += 511;
                        c.lwc1(9, bsi_ea(c, GP, -0x6de8));
                        c.lwc1(10, bsi_ea(c, GP, -0x6de4));
                        c.g[T6][0] = 0x4a0000;
                        c.lwc1(11, bsi_ea(c, GP, -0x6de0));
                        c.g[T7][0] = 0x4a0000;
                        c.lwc1(7, bsi_ea(c, GP, -0x6ddc));
                        c.g[V0][0] = sx(bsi_ea(c, T7, 0x6400));
                        c.swc1(9, bsi_ea(c, T5, 0x63e0));
                        c.g[S1][0] = 0x4a0000;
                        c.swc1(10, bsi_ea(c, T6, 0x63f0));
                        c.g[S2][0] = 0x4a0000;
                        c.swc1(11, bsi_ea(c, T7, 0x6400));
                        c.g[S3][0] = 0x4a0000;
                        c.lwc1(8, bsi_ea(c, GP, -0x6dd8));
                        c.g[A0][0] = sx(bsi_ea(c, S3, 0x6430));
                        c.lwc1(4, bsi_ea(c, GP, -0x6dd4));
                        c.g[S5][0] = 0x4a0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6dd0));
                        c.g[S4][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, V0, 0xc));
                        c.g[T3][0] = 0x4a0000;
                        c.lwc1(2, bsi_ea(c, GP, -0x6dcc));
                        c.g[A1][0] = sx(bsi_ea(c, S5, 0x63d0));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[A3][0] = sx(bsi_ea(c, T5, 0x63e0));
                        c.swc1(8, bsi_ea(c, S1, 0x6410));
                        c.g[T1][0] = sx(bsi_ea(c, T6, 0x63f0));
                        c.swc1(4, bsi_ea(c, S2, 0x6420));
                        c.g[V1][0] = sx(bsi_ea(c, S1, 0x6410));
                        c.swc1(5, bsi_ea(c, S3, 0x6430));
                        c.g[T0][0] = sx(bsi_ea(c, S2, 0x6420));
                        c.lwc1(3, bsi_ea(c, GP, -0x6dc8));
                        c.g[A2][0] = sx(bsi_ea(c, S4, 0x6440));
                        c.lwc1(1, bsi_ea(c, GP, -0x6dc4));
                        c.g[S1][0] = 0x4a0000;
                        c.swc1(2, bsi_ea(c, A0, 0xc));
                        c.g[S0][0] = sx(bsi_ea(c, S1, 0x64a0));
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 6);
                        c.g[S2][0] = 0x4a0000;
                        c.swc1(3, bsi_ea(c, S4, 0x6440));
                        c.g[S1][0] = sx(bsi_ea(c, S2, 0x64b0));
                        c.swc1(1, bsi_ea(c, T3, 0x6450));
                        c.g[S3][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S5, 0x63d0));
                        c.g[S2][0] = sx(bsi_ea(c, S3, 0x64c0));
                        c.swc1(6, bsi_ea(c, A1, 0xc));
                        c.g[S6][0] = 0x4a0000;
                        c.sw(bsi_ea(c, A3, 0xc), 0u64);
                        c.g[T4][0] = sx(bsi_ea(c, T3, 0x6450));
                        c.lwc1(17, bsi_ea(c, GP, -0x6dc0));
                        c.g[S3][0] = sx(bsi_ea(c, S6, 0x64d0));
                        c.lwc1(18, bsi_ea(c, GP, -0x6dbc));
                        c.g[FP][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T1, 0xc), 0u64);
                        c.g[T2][0] = sx(bsi_ea(c, FP, 0x6460));
                        c.lwc1(13, bsi_ea(c, GP, -0x6db8));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(14, bsi_ea(c, GP, -0x6db4));
                        c.g[T7][0] = sx(bsi_ea(c, S7, 0x6490));
                        c.swc1(17, bsi_ea(c, V0, 0x4));
                        c.g[T9][0] = 0x4a0000;
                        c.swc1(18, bsi_ea(c, V0, 0x8));
                        c.g[T5][0] = sx(bsi_ea(c, T9, 0x6470));
                        c.swc1(13, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(14, bsi_ea(c, V1, 0x8));
                        c.g[S5][0] = sx(bsi_ea(c, V0, 0x64e0));
                        c.sw(bsi_ea(c, V1, 0xc), 0u64);
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(15, bsi_ea(c, GP, -0x6db0));
                        c.g[S6][0] = sx(bsi_ea(c, V0, 0x64f0));
                        c.lwc1(16, bsi_ea(c, GP, -0x6dac));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T0, 0xc), 0u64);
                        c.g[V0][0] = sx(bsi_ea(c, V1, 0x6520));
                        c.lwc1(12, bsi_ea(c, GP, -0x6da8));
                        c.g[T8][0] = 0x4a0000;
                        c.swc1(15, bsi_ea(c, A0, 0x4));
                        c.g[T6][0] = sx(bsi_ea(c, T8, 0x6480));
                        c.swc1(16, bsi_ea(c, A0, 0x8));
                        c.swc1(12, bsi_ea(c, A2, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A2, 0xc));
                        c.g[S4][0] = sx(bsi_ea(c, A0, 0x6530));
                        c.swc1(0, bsi_ea(c, A1, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A1, 0x8));
                        c.g[V1][0] = sx(bsi_ea(c, A0, 0x6540));
                        c.swc1(0, bsi_ea(c, A3, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A3, 0x8));
                        c.g[T3][0] = sx(bsi_ea(c, A0, 0x6550));
                        c.swc1(0, bsi_ea(c, T1, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T1, 0x8));
                        c.g[A1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T0, 0x4));
                        c.g[A1][0] = sx(bsi_ea(c, A1, 0x1278));
                        c.swc1(0, bsi_ea(c, T0, 0x8));
                        c.lwc1(19, bsi_ea(c, GP, -0x6da4));
                        c.swc1(0, bsi_ea(c, A2, 0x8));
                        c.lwc1(2, bsi_ea(c, GP, -0x6da0));
                        c.g[A2][0] = sx(bsi_ea(c, A0, 0x6570));
                        c.swc1(19, bsi_ea(c, T4, 0x4));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(1, bsi_ea(c, GP, -0x6d9c));
                        c.g[A3][0] = sx(bsi_ea(c, A0, 0x6580));
                        c.swc1(2, bsi_ea(c, FP, 0x6460));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(8, bsi_ea(c, GP, -0x6d98));
                        c.g[T0][0] = sx(bsi_ea(c, A0, 0x65d0));
                        c.swc1(1, bsi_ea(c, T2, 0xc));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(3, bsi_ea(c, GP, -0x6d94));
                        c.g[T1][0] = sx(bsi_ea(c, A0, 0x6630));
                        c.swc1(8, bsi_ea(c, T7, 0xc));
                        c.g[A0][0] = 0x4a0000;
                        c.lwc1(4, bsi_ea(c, GP, -0x6d90));
                        c.g[FP][0] = 0xffffffffffff0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6d8c));
                        c.g[FP][0] = c.g[FP][0] | 0xff00;
                        c.swc1(3, bsi_ea(c, T4, 0x8));
                        c.sw(bsi_ea(c, T4, 0xc), 0u64);
                        c.swc1(4, bsi_ea(c, T2, 0x4));
                        c.g[T4][0] = sx(bsi_ea(c, A0, 0x6650));
                        c.swc1(5, bsi_ea(c, T2, 0x8));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T9, 0x6470));
                        c.swc1(0, bsi_ea(c, T5, 0xc));
                        c.swc1(0, bsi_ea(c, T8, 0x6480));
                        c.sw(bsi_ea(c, T6, 0xc), 0u64);
                        c.lwc1(7, bsi_ea(c, GP, -0x6d88));
                        c.swc1(0, bsi_ea(c, S7, 0x6490));
                        c.swc1(7, bsi_ea(c, T7, 0x8));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64a0));
                        c.swc1(0, bsi_ea(c, S0, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64b0));
                        c.swc1(0, bsi_ea(c, S1, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, A0, 0x64c0));
                        c.g[T2][0] = sx(bsi_ea(c, S7, 0x6660));
                        c.sw(bsi_ea(c, S2, 0xc), 0u64);
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64d0));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T5, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T5, 0x8));
                        c.sw(bsi_ea(c, T6, 0x4), 0u64);
                        c.sw(bsi_ea(c, T6, 0x8), 0u64);
                        c.swc1(8, bsi_ea(c, T7, 0x4));
                        c.g[T6][0] = sx(bsi_ea(c, S7, 0x66f0));
                        c.swc1(0, bsi_ea(c, S0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S0, 0x8));
                        c.swc1(0, bsi_ea(c, S1, 0x4));
                        c.swc1(0, bsi_ea(c, S1, 0x8));
                        c.sw(bsi_ea(c, S2, 0x4), 0u64);
                        c.g[S1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S2, 0x8), 0u64);
                        c.g[S0][0] = sx(bsi_ea(c, S1, 0x66e0));
                        c.g[S2][0] = sx(bsi_ea(c, A0, 0x66d0));
                        c.sw(bsi_ea(c, S3, 0x4), 0u64);
                        c.sw(bsi_ea(c, S3, 0x8), 0u64);
                        c.g[A0][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S3, 0xc), 0u64);
                        c.g[A0][0] = sx(bsi_ea(c, A0, 0x6510));
                        c.lwc1(2, bsi_ea(c, GP, -0x6d84));
                        c.g[S1][0] = 0x4a0000;
                        c.swc1(6, bsi_ea(c, A0, 0xc));
                        c.g[S3][0] = 0x4a0000;
                        c.lwc1(1, bsi_ea(c, GP, -0x6d80));
                        c.g[T5][0] = sx(bsi_ea(c, S3, 0x6700));
                        c.swc1(2, bsi_ea(c, V0, 0xc));
                        c.g[A0][0] = 0x4a0000;
                        c.swc1(1, bsi_ea(c, S1, 0x6540));
                        c.g[T7][0] = sx(bsi_ea(c, A0, 0x6710));
                        c.swc1(0, bsi_ea(c, S7, 0x64e0));
                        c.g[S3][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S5, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x64f0));
                        c.g[S1][0] = sx(bsi_ea(c, S3, 0x6720));
                        c.g[A0][0] = 0x4a0000;
                        c.g[S7][0] = 0x4a0000;
                        c.g[S3][0] = sx(bsi_ea(c, A0, 0x6740));
                        c.swc1(6, bsi_ea(c, S6, 0xc));
                        c.g[A0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S7, 0x6500));
                        c.g[A0][0] = sx(bsi_ea(c, A0, 0x6500));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(6, bsi_ea(c, A0, 0xc));
                        c.swc1(0, bsi_ea(c, S7, 0x6510));
                        c.g[A0][0] = sx(bsi_ea(c, GP, 0x1bc8));
                        c.lwc1(8, bsi_ea(c, GP, -0x6d7c));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(7, bsi_ea(c, GP, -0x6d78));
                        c.swc1(0, bsi_ea(c, S7, 0x6520));
                        c.swc1(8, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, V0, 0x8));
                        c.lwc1(5, bsi_ea(c, GP, -0x6d74));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x6530));
                        c.lwc1(4, bsi_ea(c, GP, -0x6d70));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(3, bsi_ea(c, GP, -0x6d6c));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6500));
                        c.swc1(5, bsi_ea(c, S4, 0x8));
                        c.sw(bsi_ea(c, S4, 0xc), 0u64);
                        c.swc1(4, bsi_ea(c, V1, 0x4));
                        c.swc1(3, bsi_ea(c, V1, 0x8));
                        c.sw(bsi_ea(c, V1, 0xc), 0u64);
                        c.g[V1][0] = 0x4a0000;
                        c.lwc1(2, bsi_ea(c, GP, -0x6d68));
                        c.swc1(0, bsi_ea(c, V1, 0x6550));
                        c.swc1(0, bsi_ea(c, T3, 0xc));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x6560));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6510));
                        c.swc1(0, bsi_ea(c, S5, 0x4));
                        c.swc1(0, bsi_ea(c, S5, 0x8));
                        c.swc1(6, bsi_ea(c, S6, 0x4));
                        c.swc1(0, bsi_ea(c, S6, 0x8));
                        c.swc1(6, bsi_ea(c, V0, 0x4));
                        c.g[S6][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6560));
                        c.swc1(6, bsi_ea(c, V1, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x65a0));
                        c.swc1(0, bsi_ea(c, S4, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T3, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T3, 0x8));
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.lwc1(1, bsi_ea(c, GP, -0x6d64));
                        c.swc1(2, bsi_ea(c, A3, 0xc));
                        c.lwc1(3, bsi_ea(c, GP, -0x6d60));
                        c.swc1(1, bsi_ea(c, S7, 0xc));
                        c.swc1(3, bsi_ea(c, S6, 0x8));
                        c.sw(bsi_ea(c, S6, 0xc), 0u64);
                        c.lwc1(4, bsi_ea(c, GP, -0x6d5c));
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6d58));
                        c.g[S7][0] = c.g[S6][0];
                        c.swc1(0, bsi_ea(c, V0, 0x6570));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x6590));
                        c.swc1(4, bsi_ea(c, A2, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(5, bsi_ea(c, A2, 0x8));
                        c.sw(bsi_ea(c, A2, 0xc), 0u64);
                        c.lwc1(6, bsi_ea(c, GP, -0x6d54));
                        c.lwc1(7, bsi_ea(c, GP, -0x6d50));
                        c.swc1(0, bsi_ea(c, V1, 0x6580));
                        c.swc1(6, bsi_ea(c, A3, 0x4));
                        c.g[V1][0] = c.g[V0][0];
                        c.swc1(7, bsi_ea(c, A3, 0x8));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x65a0));
                        c.lwc1(8, bsi_ea(c, GP, -0x6d4c));
                        c.swc1(0, bsi_ea(c, S6, 0x6590));
                        c.swc1(8, bsi_ea(c, S7, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0xc));
                        c.lwc1(9, bsi_ea(c, GP, -0x6d48));
                        c.g[S7][0] = c.g[S6][0];
                        c.swc1(0, bsi_ea(c, V0, 0x65a0));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x65b0));
                        c.swc1(9, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S6, 0x65b0));
                        c.g[V1][0] = c.g[V0][0];
                        c.swc1(0, bsi_ea(c, S7, 0xc));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x65c0));
                        c.swc1(0, bsi_ea(c, V0, 0x65c0));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(11, bsi_ea(c, GP, -0x6d44));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(10, bsi_ea(c, GP, -0x6d40));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6590));
                        c.swc1(0, bsi_ea(c, S6, 0x65d0));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(10, bsi_ea(c, T0, 0x4));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x65a0));
                        c.swc1(11, bsi_ea(c, T0, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T0, 0xc), 0u64);
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x65b0));
                        c.lwc1(12, bsi_ea(c, GP, -0x6d3c));
                        c.swc1(0, bsi_ea(c, S7, 0x65e0));
                        c.swc1(12, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x65c0));
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S6, 0x8));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x65e0));
                        c.swc1(0, bsi_ea(c, S7, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x4));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6640));
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(4, bsi_ea(c, GP, -0x6d38));
                        c.swc1(0, bsi_ea(c, V0, 0xc));
                        c.lwc1(3, bsi_ea(c, GP, -0x6d34));
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(4, bsi_ea(c, V1, 0x6620));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x65f0));
                        c.lwc1(2, bsi_ea(c, GP, -0x6d30));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(3, bsi_ea(c, T1, 0xc));
                        c.lwc1(1, bsi_ea(c, GP, -0x6d2c));
                        c.swc1(2, bsi_ea(c, S6, 0xc));
                        c.swc1(1, bsi_ea(c, T2, 0xc));
                        c.g[S6][0] = c.g[V1][0];
                        c.lwc1(5, bsi_ea(c, GP, -0x6d28));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6600));
                        c.swc1(0, bsi_ea(c, S7, 0x65f0));
                        c.swc1(5, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.sw(bsi_ea(c, V0, 0xc), 0u64);
                        c.swc1(0, bsi_ea(c, V1, 0x6600));
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S6, 0xc));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6610));
                        c.swc1(0, bsi_ea(c, S7, 0x6610));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, V0, 0xc), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6620));
                        c.sw(bsi_ea(c, V1, 0xc), 0u64);
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(6, bsi_ea(c, GP, -0x6d24));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(7, bsi_ea(c, GP, -0x6d20));
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S6, 0x6630));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6640));
                        c.swc1(6, bsi_ea(c, T1, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, T1, 0x8));
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(8, bsi_ea(c, GP, -0x6d1c));
                        c.swc1(0, bsi_ea(c, S7, 0x6640));
                        c.swc1(8, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.lwc1(9, bsi_ea(c, GP, -0x6d18));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x6650));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x65f0));
                        c.swc1(9, bsi_ea(c, T4, 0x8));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, T4, 0xc), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6600));
                        c.lwc1(10, bsi_ea(c, GP, -0x6d14));
                        c.swc1(0, bsi_ea(c, S6, 0x6660));
                        c.swc1(10, bsi_ea(c, T2, 0x4));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x6670));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6610));
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x4));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x6620));
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6640));
                        c.sw(bsi_ea(c, S6, 0x8), 0u64);
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S7, 0x4), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6670));
                        c.sw(bsi_ea(c, S7, 0x8), 0u64);
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6680));
                        c.swc1(0, bsi_ea(c, T4, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, T2, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x66c0));
                        c.swc1(0, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(2, bsi_ea(c, GP, -0x6d10));
                        c.swc1(12, bsi_ea(c, S6, 0xc));
                        c.lwc1(1, bsi_ea(c, GP, -0x6d0c));
                        c.swc1(2, bsi_ea(c, S7, 0xc));
                        c.swc1(1, bsi_ea(c, S2, 0xc));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0xc));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V0, 0x6680));
                        c.g[S6][0] = c.g[V1][0];
                        c.swc1(0, bsi_ea(c, V1, 0x6690));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6690));
                        c.g[V0][0] = c.g[S7][0];
                        c.sw(bsi_ea(c, S6, 0xc), 0u64);
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S7, 0x66a0));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x66a0));
                        c.sw(bsi_ea(c, V0, 0xc), 0u64);
                        c.g[S6][0] = c.g[V1][0];
                        c.swc1(0, bsi_ea(c, V1, 0x66b0));
                        c.g[S7][0] = 0x4a0000;
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x66b0));
                        c.lwc1(3, bsi_ea(c, GP, -0x6d08));
                        c.sw(bsi_ea(c, S6, 0xc), 0u64);
                        c.g[V0][0] = c.g[S7][0];
                        c.swc1(0, bsi_ea(c, S7, 0x66c0));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x66c0));
                        c.swc1(3, bsi_ea(c, V0, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.lwc1(4, bsi_ea(c, GP, -0x6d04));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, V1, 0x66d0));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(4, bsi_ea(c, S2, 0x8));
                        c.g[V0][0] = 0x4a0000;
                        c.lwc1(5, bsi_ea(c, GP, -0x6d00));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6670));
                        c.swc1(0, bsi_ea(c, S6, 0x66e0));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(5, bsi_ea(c, S0, 0x8));
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x6680));
                        c.sw(bsi_ea(c, S0, 0xc), 0u64);
                        c.g[S6][0] = 0x4a0000;
                        c.lwc1(6, bsi_ea(c, GP, -0x6cfc));
                        c.g[S6][0] = sx(bsi_ea(c, S6, 0x6690));
                        c.lwc1(7, bsi_ea(c, GP, -0x6cf8));
                        c.swc1(0, bsi_ea(c, S7, 0x66f0));
                        c.swc1(6, bsi_ea(c, T6, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.swc1(7, bsi_ea(c, T6, 0x8));
                        c.g[S7][0] = sx(bsi_ea(c, S7, 0x66a0));
                        c.swc1(0, bsi_ea(c, V0, 0x8));
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.g[V0][0] = 0x4a0000;
                        c.swc1(12, bsi_ea(c, V1, 0x4));
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x66b0));
                        c.swc1(0, bsi_ea(c, S6, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S6, 0x8), 0u64);
                        c.g[V1][0] = sx(bsi_ea(c, V1, 0x66c0));
                        c.swc1(0, bsi_ea(c, S7, 0x4));
                        c.g[S6][0] = 0x4a0000;
                        c.sw(bsi_ea(c, S7, 0x8), 0u64);
                        c.swc1(0, bsi_ea(c, V0, 0x4));
                        c.g[S7][0] = 0x4a0000;
                        c.sw(bsi_ea(c, V0, 0x8), 0u64);
                        c.swc1(0, bsi_ea(c, V1, 0x8));
                        c.swc1(0, bsi_ea(c, S2, 0x4));
                        c.g[V1][0] = 0x4a0000;
                        c.swc1(0, bsi_ea(c, S0, 0x4));
                        c.g[S2][0] = 0xffffffffffffffff;
                        c.sw(bsi_ea(c, T6, 0xc), 0u64);
                        c.lwc1(2, bsi_ea(c, GP, -0x6cf4));
                        c.sb(bsi_ea(c, GP, 0x1bc0), c.g[S2][0]);
                        c.lwc1(1, bsi_ea(c, GP, -0x6cf0));
                        c.swc1(2, bsi_ea(c, T7, 0xc));
                        c.g[V0][0] = c.g[S2][0] & 0xff;
                        c.swc1(1, bsi_ea(c, S1, 0xc));
                        c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                        c.lwc1(4, bsi_ea(c, GP, -0x6cec));
                        c.lwc1(3, bsi_ea(c, GP, -0x6ce8));
                        c.swc1(0, bsi_ea(c, S6, 0x6700));
                        c.swc1(4, bsi_ea(c, T5, 0x4));
                        c.g[S6][0] = 0x4a0000;
                        c.swc1(3, bsi_ea(c, T5, 0x8));
                        c.sw(bsi_ea(c, T5, 0xc), 0u64);
                        c.lwc1(8, bsi_ea(c, GP, -0x6ce4));
                        c.lwc1(5, bsi_ea(c, GP, -0x6ce0));
                        c.swc1(0, bsi_ea(c, S7, 0x6710));
                        c.swc1(8, bsi_ea(c, T7, 0x4));
                        c.swc1(5, bsi_ea(c, T7, 0x8));
                        c.lwc1(6, bsi_ea(c, GP, -0x6cdc));
                        c.lwc1(7, bsi_ea(c, GP, -0x6cd8));
                        c.swc1(0, bsi_ea(c, V1, 0x6720));
                        c.swc1(6, bsi_ea(c, S1, 0x4));
                        c.swc1(7, bsi_ea(c, S1, 0x8));
                        c.g[S1][0] = 0x4a0000;
                        c.g[S2][0] = c.g[S1][0];
                        c.swc1(0, bsi_ea(c, S1, 0x6730));
                        c.g[S2][0] = sx(bsi_ea(c, S2, 0x6730));
                        c.sw(bsi_ea(c, S2, 0xc), 0u64);
                        c.swc1(0, bsi_ea(c, S6, 0x6740));
                        c.swc1(0, bsi_ea(c, S3, 0x4));
                        c.sw(bsi_ea(c, S3, 0xc), 0u64);
                        c.sw(bsi_ea(c, GP, 0x1bc0), c.g[V0][0]);
                        c.swc1(0, bsi_ea(c, S2, 0x8));
                        c.sw(bsi_ea(c, S2, 0x4), 0u64);
                        c.sw(bsi_ea(c, S3, 0x8), 0u64);
                        call_ptr(c, 0x003182d8, 0x0015b83c)?;
                        c.g[A0][0] = 0x4a0000;
                        c.g[A0][0] = sx(bsi_ea(c, A0, 0x6750));
                        call_ptr(c, 0x0014c620, 0x0015b848)?;
                        c.g[V0][0] = 0x4a0000;
                        c.g[T2][0] = 0xffffffffffffffff;
                        c.g[T0][0] = 0xffffffffffffffff;
                        c.g[T3][0] = 0x9b50;
                        c.g[V1][0] = 0x2;
                        c.g[V0][0] = sx(bsi_ea(c, V0, 0x6ca8));
                        // Character profiles: fill the slots with (0xffff, 0) halfword pairs.
                        'l_15b860: loop {
                            c.cycles += 4;
                            c.g[T1][0] = sx(bsi_ea(c, V1, -0x1));
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V1][0] = 0x9;
                            'l_15b870: loop {
                                c.cycles += 4;
                                c.g[A2][0] = sx(bsi_ea(c, V1, -0x1));
                                c.g[A3][0] = sx(bsi_ea(c, A1, 0xf88));
                                c.g[V1][0] = sx(bsi_ea(c, A1, 0x290));
                                c.g[A0][0] = 0x20c;
                                'l_15b880: loop {
                                    c.cycles += 7;
                                    c.sh(bsi_ea(c, V1, 0), c.g[T2][0]);
                                    c.g[A0][0] = sx(bsi_ea(c, A0, -0x1));
                                    c.sh(bsi_ea(c, V1, 0x2), 0u64);
                                    let t = c.g[A0][0] != c.g[T0][0];
                                    c.g[V1][0] = sx(bsi_ea(c, V1, 0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0015b880);
                                        }
                                        continue 'l_15b880;
                                    } else {
                                        c.cycles += 4;
                                        c.sw(bsi_ea(c, A1, 0x288), 0u64);
                                        c.g[V1][0] = c.g[A2][0];
                                        let t = c.g[V1][0] != c.g[T0][0];
                                        c.g[A1][0] = c.g[A3][0];
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0015b870);
                                            }
                                            continue 'l_15b870;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V1][0] = c.g[T1][0];
                                            let t = c.g[V1][0] != c.g[T0][0];
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[T3][0] as u32)));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0015b860);
                                                }
                                                continue 'l_15b860;
                                            } else {
                                                c.cycles += 84;
                                                c.g[A0][0] = 0x4c0000;
                                                c.g[A0][0] = sx(bsi_ea(c, A0, 0x3e98));
                                                call_ptr(c, 0x0015a778, 0x0015b8c4)?;
                                                c.g[A0][0] = 0x4c0000;
                                                c.g[A0][0] = sx(bsi_ea(c, A0, 0x3ef0));
                                                call_ptr(c, 0x0015a488, 0x0015b8d0)?;
                                                c.g[S3][0] = 0xffffffffffffffff;
                                                call_ptr(c, 0x0015c228, 0x0015b8d8)?;
                                                c.g[V0][0] = 0x4c0000;
                                                c.g[V1][0] = 0x2;
                                                c.g[A1][0] = 0x4;
                                                c.g[A2][0] = 0x5;
                                                c.g[S4][0] = sx(bsi_ea(c, V0, 0x4090));
                                                c.sb(bsi_ea(c, GP, 0x1bf0), c.g[V1][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c00), c.g[A1][0]);
                                                c.g[V0][0] = 0x1;
                                                c.sb(bsi_ea(c, GP, 0x1c08), c.g[A2][0]);
                                                c.g[A0][0] = 0x3;
                                                c.g[T1][0] = 0x7;
                                                c.g[T2][0] = 0x8;
                                                c.g[T3][0] = 0x9;
                                                c.g[T0][0] = 0xa;
                                                c.sb(bsi_ea(c, GP, 0x1be8), c.g[V0][0]);
                                                c.g[A3][0] = 0x6;
                                                c.sb(bsi_ea(c, GP, 0x1bf8), c.g[A0][0]);
                                                c.g[T7][0] = 0xb;
                                                c.sb(bsi_ea(c, GP, 0x1c18), c.g[T1][0]);
                                                c.g[T6][0] = 0xc;
                                                c.sb(bsi_ea(c, GP, 0x1c20), c.g[T2][0]);
                                                c.g[T1][0] = c.g[V0][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1c28), c.g[T3][0]);
                                                c.g[T2][0] = c.g[A1][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1c30), c.g[T0][0]);
                                                c.g[T3][0] = c.g[V1][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1c10), c.g[A3][0]);
                                                c.g[T0][0] = c.g[A2][0] & 0xff;
                                                c.sb(bsi_ea(c, GP, 0x1be0), 0u64);
                                                c.g[T4][0] = c.g[A0][0] & 0xff;
                                                c.g[A2][0] = c.lbu(bsi_ea(c, GP, 0x1c18));
                                                c.g[A3][0] = c.g[A3][0] & 0xff;
                                                c.g[A1][0] = c.lbu(bsi_ea(c, GP, 0x1c20));
                                                c.g[T1][0] = c.g[T1][0] | 0x100;
                                                c.g[V1][0] = c.lbu(bsi_ea(c, GP, 0x1c28));
                                                c.g[T3][0] = c.g[T3][0] | 0x200;
                                                c.g[A0][0] = c.lbu(bsi_ea(c, GP, 0x1be0));
                                                c.g[T4][0] = c.g[T4][0] | 0x300;
                                                c.g[V0][0] = c.lbu(bsi_ea(c, GP, 0x1c30));
                                                c.g[V1][0] = c.g[V1][0] | 0x900;
                                                c.g[T2][0] = c.g[T2][0] | 0x400;
                                                c.g[T0][0] = c.g[T0][0] | 0x500;
                                                c.g[A3][0] = c.g[A3][0] | 0x600;
                                                c.g[A2][0] = c.g[A2][0] | 0x700;
                                                c.g[A1][0] = c.g[A1][0] | 0x800;
                                                c.sw(bsi_ea(c, GP, 0x1be0), c.g[A0][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c28), c.g[V1][0]);
                                                c.g[V0][0] = c.g[V0][0] | 0xa00;
                                                c.sw(bsi_ea(c, GP, 0x1be8), c.g[T1][0]);
                                                c.g[T5][0] = 0xd;
                                                c.sw(bsi_ea(c, GP, 0x1bf0), c.g[T3][0]);
                                                c.sw(bsi_ea(c, GP, 0x1bf8), c.g[T4][0]);
                                                c.g[S2][0] = 0xffffffffffffffff;
                                                c.sw(bsi_ea(c, GP, 0x1c00), c.g[T2][0]);
                                                c.g[S0][0] = sx(bsi_ea(c, S4, 0x290));
                                                c.sw(bsi_ea(c, GP, 0x1c08), c.g[T0][0]);
                                                c.g[S1][0] = 0x20c;
                                                c.sw(bsi_ea(c, GP, 0x1c10), c.g[A3][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c18), c.g[A2][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c20), c.g[A1][0]);
                                                c.sw(bsi_ea(c, GP, 0x1bdc), 0u64);
                                                c.sw(bsi_ea(c, GP, 0x1bd8), 0u64);
                                                c.sw(bsi_ea(c, GP, 0x1c30), c.g[V0][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c38), c.g[T7][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c40), c.g[T6][0]);
                                                c.sb(bsi_ea(c, GP, 0x1c48), c.g[T5][0]);
                                                c.g[V0][0] = c.g[T7][0] & 0xff;
                                                c.g[V1][0] = c.g[T6][0] & 0xff;
                                                c.g[V0][0] = c.g[V0][0] | 0xb00;
                                                c.g[A0][0] = c.g[T5][0] & 0xff;
                                                c.g[V1][0] = c.g[V1][0] | 0xc00;
                                                c.sw(bsi_ea(c, GP, 0x1c38), c.g[V0][0]);
                                                c.g[A0][0] = c.g[A0][0] | 0xd00;
                                                c.sw(bsi_ea(c, GP, 0x1c40), c.g[V1][0]);
                                                c.sw(bsi_ea(c, GP, 0x1c48), c.g[A0][0]);
                                                'l_15ba08: loop {
                                                    c.cycles += 7;
                                                    c.sh(bsi_ea(c, S0, 0), c.g[S3][0]);
                                                    c.g[S1][0] = sx(bsi_ea(c, S1, -0x1));
                                                    c.sh(bsi_ea(c, S0, 0x2), 0u64);
                                                    let t = c.g[S1][0] != c.g[S2][0];
                                                    c.g[S0][0] = sx(bsi_ea(c, S0, 0x4));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0015ba08);
                                                        }
                                                        continue 'l_15ba08;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.sw(bsi_ea(c, S4, 0x288), 0u64);
                                                        break 'b_15bacc;
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
                // param_1 == 0: destroy the back end's singletons and the bolt DB.
                'b_15bab4: {
                    c.cycles += 14;
                    c.g[A0][0] = 0x4c0000;
                    c.g[A1][0] = 0x2;
                    c.g[S6][0] = sx(bsi_ea(c, GP, 0x1bc8));
                    c.g[A0][0] = sx(bsi_ea(c, A0, 0x4090));
                    call_ptr(c, 0x00151368, 0x0015ba40)?;
                    c.g[S5][0] = 0x4a0000;
                    c.g[A0][0] = 0x4c0000;
                    c.g[A1][0] = 0x2;
                    c.g[A0][0] = sx(bsi_ea(c, A0, 0x3e98));
                    call_ptr(c, 0x0015a7d0, 0x0015ba54)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[V1][0] = sx(bsi_ea(c, V0, 0x6ca8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x4c0000;
                    if t {
                        break 'b_15bab4;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = sx(bsi_ea(c, V0, 0x3e98));
                        let t = c.g[V1][0] == c.g[S1][0];
                        c.g[S4][0] = c.g[V1][0];
                        if t {
                            break 'b_15bab4;
                        } else {
                            c.cycles += 4;
                            c.g[S2][0] = 0xffffffffffff0000;
                            c.g[S2][0] = c.g[S2][0] | 0x64b0;
                            c.g[S3][0] = 0x9b50;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            'l_15ba80: loop {
                                'b_15baac: {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == 0u64;

                                    if t {
                                        break 'b_15baac;
                                    } else {
                                        c.cycles += 3;
                                        c.g[S0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[S3][0] as u32)));
                                        let t = c.g[S1][0] == c.g[S0][0];

                                        if t {
                                            break 'b_15baac;
                                        } else {
                                            c.cycles += 1;
                                            c.g[S0][0] = sx(bsi_ea(c, S0, -0xf88));
                                            'l_15ba98: loop {
                                                c.cycles += 5;
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x00151368, 0x0015baa4)?;
                                                if c.g[S1][0] != c.g[S0][0] {
                                                    c.g[S0][0] = sx(bsi_ea(c, S0, -0xf88));
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0015ba98);
                                                    }
                                                    continue 'l_15ba98;
                                                } else {
                                                    break 'b_15baac;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] != c.g[S4][0];
                                c.g[S1][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0015ba80);
                                    }
                                    continue 'l_15ba80;
                                } else {
                                    break 'b_15bab4;
                                }
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = sx(bsi_ea(c, S5, 0x6750));
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014c658, 0x0015bac0)?;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x00318260, 0x0015bacc)?;
                break 'b_15bacc;
            }
        }
        c.cycles += 12;
        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    /// The init path (`param_1 != 0`, `param_2 == 0xffff`) and the early exit
    /// (`param_2 != 0xffff`). The destroy path (`param_1 == 0`) is not
    /// compared: its outer loop steps the profile pointer down by 0x9b50 from
    /// 0x4ba348 and only stops at 0x4a6ca8, which it never hits on zeroed RAM.
    #[test]
    fn be_static_init_destroy_matches() {
        check(
            ssx3_ee::s01::f_0015b028__generated,
            be_static_init_destroy,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x15b028, 0);
                if seed % 4 == 3 {
                    c.g[A0][0] = (seed % 2) as u64;
                    c.g[A1][0] = 0x1234 + seed as u64;
                } else {
                    c.g[A0][0] = 1 + (seed % 3) as u64;
                    c.g[A1][0] = 0xffff;
                }
            },
        );
    }
}
