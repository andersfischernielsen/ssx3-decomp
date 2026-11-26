//! Staging module oc904 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc904`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001a9cc0,
    net_fe_state_show_net_error_popup,
    "NetFEState_ShowNetErrorPopup",
)];

/// `NetFEState_ShowNetErrorPopup` (0x001a9cc0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_labels, unused_assignments, unused_variables)]
pub fn net_fe_state_show_net_error_popup(c: &mut Cpu) {
    if c.pc != 0x001a9cc0 {
        return ee::s03::f_001a9cc0__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001a9cc0;
        loop {
            match pc {
                0x001a9cc0 => {
                    c.cycles += 36;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A3][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S7][0] = 0u64;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x001a9d04);
                    c.g[FP][0] = 0u64;
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001a9d04)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S0][0] as u32), c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                    c.g[RA][0] = sx(0x001a9d20);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001a9d20)?;
                    c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xbc));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001a9d34);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x001c57c0,
                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                    }
                    .call(c, 0x001a9d34)?;
                    c.g[V0][0] = 0x102;
                    c.g[A0][0] = 0x6d610000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[A0][0] = c.g[A0][0] | 0x7574;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa41c;
                        continue;
                    }
                    pc = 0x001a9d50;
                    continue;
                }
                0x001a9d50 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x72610000;
                    if t {
                        pc = 0x001aa088;
                        continue;
                    }
                    pc = 0x001a9d58;
                    continue;
                }
                0x001a9d58 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x66750000;
                    c.g[V0][0] = c.g[V0][0] | 0x6c6c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa640;
                        continue;
                    }
                    pc = 0x001a9d68;
                    continue;
                }
                0x001a9d68 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x696e0000;
                    if t {
                        pc = 0x001a9ef8;
                        continue;
                    }
                    pc = 0x001a9d70;
                    continue;
                }
                0x001a9d70 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x63680000;
                    c.g[V0][0] = c.g[V0][0] | 0x6574;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa754;
                        continue;
                    }
                    pc = 0x001a9d80;
                    continue;
                }
                0x001a9d80 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x646e0000;
                    if t {
                        pc = 0x001a9e3c;
                        continue;
                    }
                    pc = 0x001a9d88;
                    continue;
                }
                0x001a9d88 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x626c0000;
                    c.g[V0][0] = c.g[V0][0] | 0x616b;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa6cc;
                        continue;
                    }
                    pc = 0x001a9d98;
                    continue;
                }
                0x001a9d98 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x626f0000;
                    if t {
                        pc = 0x001a9df0;
                        continue;
                    }
                    pc = 0x001a9da0;
                    continue;
                }
                0x001a9da0 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x61750000;
                    c.g[V0][0] = c.g[V0][0] | 0x6e72;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa6b4;
                        continue;
                    }
                    pc = 0x001a9db0;
                    continue;
                }
                0x001a9db0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x61750000;
                    if t {
                        pc = 0x001a9dd0;
                        continue;
                    }
                    pc = 0x001a9db8;
                    continue;
                }
                0x001a9db8 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x616a0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6f69;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa7a8;
                        continue;
                    }
                    pc = 0x001a9dc8;
                    continue;
                }
                0x001a9dc8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9dd0;
                    continue;
                }
                0x001a9dd0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7468;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x62610000;
                    if t {
                        pc = 0x001aa4f8;
                        continue;
                    }
                    pc = 0x001a9ddc;
                    continue;
                }
                0x001a9ddc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6463;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa6a4;
                        continue;
                    }
                    pc = 0x001a9de8;
                    continue;
                }
                0x001a9de8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9df0;
                    continue;
                }
                0x001a9df0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x726e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa6f0;
                        continue;
                    }
                    pc = 0x001a9dfc;
                    continue;
                }
                0x001a9dfc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x63640000;
                    if t {
                        pc = 0x001a9e1c;
                        continue;
                    }
                    pc = 0x001a9e04;
                    continue;
                }
                0x001a9e04 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x626f0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6775;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa674;
                        continue;
                    }
                    pc = 0x001a9e14;
                    continue;
                }
                0x001a9e14 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9e1c;
                    continue;
                }
                0x001a9e1c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6576;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x63680000;
                    if t {
                        pc = 0x001aa7f8;
                        continue;
                    }
                    pc = 0x001a9e28;
                    continue;
                }
                0x001a9e28 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa62c;
                        continue;
                    }
                    pc = 0x001a9e34;
                    continue;
                }
                0x001a9e34 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9e3c;
                    continue;
                }
                0x001a9e3c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7366;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa8b4;
                        continue;
                    }
                    pc = 0x001a9e48;
                    continue;
                }
                0x001a9e48 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x65630000;
                    if t {
                        pc = 0x001a9ea0;
                        continue;
                    }
                    pc = 0x001a9e50;
                    continue;
                }
                0x001a9e50 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x64650000;
                    c.g[V0][0] = c.g[V0][0] | 0x6670;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa410;
                        continue;
                    }
                    pc = 0x001a9e60;
                    continue;
                }
                0x001a9e60 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x64650000;
                    if t {
                        pc = 0x001a9e80;
                        continue;
                    }
                    pc = 0x001a9e68;
                    continue;
                }
                0x001a9e68 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x64620000;
                    c.g[V0][0] = c.g[V0][0] | 0x6572;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa3e0;
                        continue;
                    }
                    pc = 0x001a9e78;
                    continue;
                }
                0x001a9e78 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9e80;
                    continue;
                }
                0x001a9e80 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7465;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x64690000;
                    if t {
                        pc = 0x001aa494;
                        continue;
                    }
                    pc = 0x001a9e8c;
                    continue;
                }
                0x001a9e8c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7361;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa64c;
                        continue;
                    }
                    pc = 0x001a9e98;
                    continue;
                }
                0x001a9e98 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9ea0;
                    continue;
                }
                0x001a9ea0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa72c;
                        continue;
                    }
                    pc = 0x001a9eac;
                    continue;
                }
                0x001a9eac => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x656c0000;
                    if t {
                        pc = 0x001a9ed8;
                        continue;
                    }
                    pc = 0x001a9eb4;
                    continue;
                }
                0x001a9eb4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x646f0000;
                    c.g[V0][0] = c.g[V0][0] | 0x776e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x64750000;
                    if t {
                        pc = 0x001aa808;
                        continue;
                    }
                    pc = 0x001a9ec4;
                    continue;
                }
                0x001a9ec4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x706c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa400;
                        continue;
                    }
                    pc = 0x001a9ed0;
                    continue;
                }
                0x001a9ed0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9ed8;
                    continue;
                }
                0x001a9ed8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x656e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x66690000;
                    if t {
                        pc = 0x001aa788;
                        continue;
                    }
                    pc = 0x001a9ee4;
                    continue;
                }
                0x001a9ee4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6c74;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa534;
                        continue;
                    }
                    pc = 0x001a9ef0;
                    continue;
                }
                0x001a9ef0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9ef8;
                    continue;
                }
                0x001a9ef8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7670;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa3f4;
                        continue;
                    }
                    pc = 0x001a9f04;
                    continue;
                }
                0x001a9f04 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6b690000;
                    if t {
                        pc = 0x001a9fcc;
                        continue;
                    }
                    pc = 0x001a9f0c;
                    continue;
                }
                0x001a9f0c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x69670000;
                    c.g[V0][0] = c.g[V0][0] | 0x616d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa738;
                        continue;
                    }
                    pc = 0x001a9f1c;
                    continue;
                }
                0x001a9f1c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x696e0000;
                    if t {
                        pc = 0x001a9f74;
                        continue;
                    }
                    pc = 0x001a9f24;
                    continue;
                }
                0x001a9f24 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x67700000;
                    c.g[V0][0] = c.g[V0][0] | 0x6172;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa634;
                        continue;
                    }
                    pc = 0x001a9f34;
                    continue;
                }
                0x001a9f34 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x686f0000;
                    if t {
                        pc = 0x001a9f54;
                        continue;
                    }
                    pc = 0x001a9f3c;
                    continue;
                }
                0x001a9f3c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x67650000;
                    c.g[V0][0] = c.g[V0][0] | 0x6e64;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa6fc;
                        continue;
                    }
                    pc = 0x001a9f4c;
                    continue;
                }
                0x001a9f4c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9f54;
                    continue;
                }
                0x001a9f54 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7374;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x69610000;
                    if t {
                        pc = 0x001aa4ac;
                        continue;
                    }
                    pc = 0x001a9f60;
                    continue;
                }
                0x001a9f60 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6472;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa744;
                        continue;
                    }
                    pc = 0x001a9f6c;
                    continue;
                }
                0x001a9f6c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9f74;
                    continue;
                }
                0x001a9f74 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa77c;
                        continue;
                    }
                    pc = 0x001a9f80;
                    continue;
                }
                0x001a9f80 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x696e0000;
                    if t {
                        pc = 0x001a9fac;
                        continue;
                    }
                    pc = 0x001a9f88;
                    continue;
                }
                0x001a9f88 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x69670000;
                    c.g[V0][0] = c.g[V0][0] | 0x6e6f;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x696d0000;
                    if t {
                        pc = 0x001aa620;
                        continue;
                    }
                    pc = 0x001a9f98;
                    continue;
                }
                0x001a9f98 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7374;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa3c0;
                        continue;
                    }
                    pc = 0x001a9fa4;
                    continue;
                }
                0x001a9fa4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9fac;
                    continue;
                }
                0x001a9fac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x676d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x696e0000;
                    if t {
                        pc = 0x001aa4b8;
                        continue;
                    }
                    pc = 0x001a9fb8;
                    continue;
                }
                0x001a9fb8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7073;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa7f0;
                        continue;
                    }
                    pc = 0x001a9fc4;
                    continue;
                }
                0x001a9fc4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001a9fcc;
                    continue;
                }
                0x001a9fcc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x636b;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa4a0;
                        continue;
                    }
                    pc = 0x001a9fd8;
                    continue;
                }
                0x001a9fd8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6c6f0000;
                    if t {
                        pc = 0x001aa030;
                        continue;
                    }
                    pc = 0x001a9fe0;
                    continue;
                }
                0x001a9fe0 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x69700000;
                    c.g[V0][0] = c.g[V0][0] | 0x6572;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa3c8;
                        continue;
                    }
                    pc = 0x001a9ff0;
                    continue;
                }
                0x001a9ff0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x69740000;
                    if t {
                        pc = 0x001aa010;
                        continue;
                    }
                    pc = 0x001a9ff8;
                    continue;
                }
                0x001a9ff8 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x69700000;
                    c.g[V0][0] = c.g[V0][0] | 0x6173;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa774;
                        continue;
                    }
                    pc = 0x001aa008;
                    continue;
                }
                0x001aa008 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa010;
                    continue;
                }
                0x001aa010 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x726e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6a6f0000;
                    if t {
                        pc = 0x001aa794;
                        continue;
                    }
                    pc = 0x001aa01c;
                    continue;
                }
                0x001aa01c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x696e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa7a0;
                        continue;
                    }
                    pc = 0x001aa028;
                    continue;
                }
                0x001aa028 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa030;
                    continue;
                }
                0x001aa030 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x676e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa528;
                        continue;
                    }
                    pc = 0x001aa03c;
                    continue;
                }
                0x001aa03c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6d610000;
                    if t {
                        pc = 0x001aa068;
                        continue;
                    }
                    pc = 0x001aa044;
                    continue;
                }
                0x001aa044 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6b690000;
                    c.g[V0][0] = c.g[V0][0] | 0x6b68;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6c6f0000;
                    if t {
                        pc = 0x001aa660;
                        continue;
                    }
                    pc = 0x001aa054;
                    continue;
                }
                0x001aa054 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x636b;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x001aa53c;
                        continue;
                    }
                    pc = 0x001aa060;
                    continue;
                }
                0x001aa060 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa068;
                    continue;
                }
                0x001aa068 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x696c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6d610000;
                    if t {
                        pc = 0x001aa6d8;
                        continue;
                    }
                    pc = 0x001aa074;
                    continue;
                }
                0x001aa074 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6e79;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa724;
                        continue;
                    }
                    pc = 0x001aa080;
                    continue;
                }
                0x001aa080 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa088;
                    continue;
                }
                0x001aa088 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6e67;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa47c;
                        continue;
                    }
                    pc = 0x001aa094;
                    continue;
                }
                0x001aa094 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x74690000;
                    if t {
                        pc = 0x001aa230;
                        continue;
                    }
                    pc = 0x001aa09c;
                    continue;
                }
                0x001aa09c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6e700000;
                    c.g[V0][0] = c.g[V0][0] | 0x7269;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa4d0;
                        continue;
                    }
                    pc = 0x001aa0ac;
                    continue;
                }
                0x001aa0ac => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6f760000;
                    if t {
                        pc = 0x001aa174;
                        continue;
                    }
                    pc = 0x001aa0b4;
                    continue;
                }
                0x001aa0b4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6e650000;
                    c.g[V0][0] = c.g[V0][0] | 0x7730;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa504;
                        continue;
                    }
                    pc = 0x001aa0c4;
                    continue;
                }
                0x001aa0c4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6e6b0000;
                    if t {
                        pc = 0x001aa11c;
                        continue;
                    }
                    pc = 0x001aa0cc;
                    continue;
                }
                0x001aa0cc => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6d610000;
                    c.g[V0][0] = c.g[V0][0] | 0x7874;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa7d8;
                        continue;
                    }
                    pc = 0x001aa0dc;
                    continue;
                }
                0x001aa0dc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6d690000;
                    if t {
                        pc = 0x001aa0fc;
                        continue;
                    }
                    pc = 0x001aa0e4;
                    continue;
                }
                0x001aa0e4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6d610000;
                    c.g[V0][0] = c.g[V0][0] | 0x7870;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa408;
                        continue;
                    }
                    pc = 0x001aa0f4;
                    continue;
                }
                0x001aa0f4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa0fc;
                    continue;
                }
                0x001aa0fc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7373;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6e650000;
                    if t {
                        pc = 0x001aa3e8;
                        continue;
                    }
                    pc = 0x001aa108;
                    continue;
                }
                0x001aa108 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6d6c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa71c;
                        continue;
                    }
                    pc = 0x001aa114;
                    continue;
                }
                0x001aa114 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa11c;
                    continue;
                }
                0x001aa11c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x696b;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa6c0;
                        continue;
                    }
                    pc = 0x001aa128;
                    continue;
                }
                0x001aa128 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6e6f0000;
                    if t {
                        pc = 0x001aa154;
                        continue;
                    }
                    pc = 0x001aa130;
                    continue;
                }
                0x001aa130 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6e660000;
                    c.g[V0][0] = c.g[V0][0] | 0x6e64;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6e670000;
                    if t {
                        pc = 0x001aa7bc;
                        continue;
                    }
                    pc = 0x001aa140;
                    continue;
                }
                0x001aa140 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa4e4;
                        continue;
                    }
                    pc = 0x001aa14c;
                    continue;
                }
                0x001aa14c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa154;
                    continue;
                }
                0x001aa154 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x776e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6e700000;
                    if t {
                        pc = 0x001aa51c;
                        continue;
                    }
                    pc = 0x001aa160;
                    continue;
                }
                0x001aa160 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6c61;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa74c;
                        continue;
                    }
                    pc = 0x001aa16c;
                    continue;
                }
                0x001aa16c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa174;
                    continue;
                }
                0x001aa174 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6572;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa698;
                        continue;
                    }
                    pc = 0x001aa180;
                    continue;
                }
                0x001aa180 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x706d0000;
                    if t {
                        pc = 0x001aa1d8;
                        continue;
                    }
                    pc = 0x001aa188;
                    continue;
                }
                0x001aa188 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6e730000;
                    c.g[V0][0] = c.g[V0][0] | 0x7063;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa7b0;
                        continue;
                    }
                    pc = 0x001aa198;
                    continue;
                }
                0x001aa198 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6e730000;
                    if t {
                        pc = 0x001aa1b8;
                        continue;
                    }
                    pc = 0x001aa1a0;
                    continue;
                }
                0x001aa1a0 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6e720000;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa4dc;
                        continue;
                    }
                    pc = 0x001aa1b0;
                    continue;
                }
                0x001aa1b0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa1b8;
                    continue;
                }
                0x001aa1b8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7276;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6f6e0000;
                    if t {
                        pc = 0x001aa768;
                        continue;
                    }
                    pc = 0x001aa1c4;
                    continue;
                }
                0x001aa1c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6c6e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa6ac;
                        continue;
                    }
                    pc = 0x001aa1d0;
                    continue;
                }
                0x001aa1d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa1d8;
                    continue;
                }
                0x001aa1d8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa6e4;
                        continue;
                    }
                    pc = 0x001aa1e4;
                    continue;
                }
                0x001aa1e4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x70730000;
                    if t {
                        pc = 0x001aa210;
                        continue;
                    }
                    pc = 0x001aa1ec;
                    continue;
                }
                0x001aa1ec => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x70610000;
                    c.g[V0][0] = c.g[V0][0] | 0x7373;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x70610000;
                    if t {
                        pc = 0x001aa3d4;
                        continue;
                    }
                    pc = 0x001aa1fc;
                    continue;
                }
                0x001aa1fc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7574;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa440;
                        continue;
                    }
                    pc = 0x001aa208;
                    continue;
                }
                0x001aa208 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa210;
                    continue;
                }
                0x001aa210 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6574;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x71750000;
                    if t {
                        pc = 0x001aa510;
                        continue;
                    }
                    pc = 0x001aa21c;
                    continue;
                }
                0x001aa21c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6974;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa67c;
                        continue;
                    }
                    pc = 0x001aa228;
                    continue;
                }
                0x001aa228 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa230;
                    continue;
                }
                0x001aa230 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6d65;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa68c;
                        continue;
                    }
                    pc = 0x001aa23c;
                    continue;
                }
                0x001aa23c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x756c0000;
                    if t {
                        pc = 0x001aa304;
                        continue;
                    }
                    pc = 0x001aa244;
                    continue;
                }
                0x001aa244 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x73610000;
                    c.g[V0][0] = c.g[V0][0] | 0x6d65;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa614;
                        continue;
                    }
                    pc = 0x001aa254;
                    continue;
                }
                0x001aa254 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x73700000;
                    if t {
                        pc = 0x001aa2ac;
                        continue;
                    }
                    pc = 0x001aa25c;
                    continue;
                }
                0x001aa25c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x72610000;
                    c.g[V0][0] = c.g[V0][0] | 0x7574;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa710;
                        continue;
                    }
                    pc = 0x001aa26c;
                    continue;
                }
                0x001aa26c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x726f0000;
                    if t {
                        pc = 0x001aa28c;
                        continue;
                    }
                    pc = 0x001aa274;
                    continue;
                }
                0x001aa274 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x72610000;
                    c.g[V0][0] = c.g[V0][0] | 0x6e6b;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa66c;
                        continue;
                    }
                    pc = 0x001aa284;
                    continue;
                }
                0x001aa284 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa28c;
                    continue;
                }
                0x001aa28c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x72730000;
                    if t {
                        pc = 0x001aa4ec;
                        continue;
                    }
                    pc = 0x001aa298;
                    continue;
                }
                0x001aa298 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7276;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa600;
                        continue;
                    }
                    pc = 0x001aa2a4;
                    continue;
                }
                0x001aa2a4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa2ac;
                    continue;
                }
                0x001aa2ac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa704;
                        continue;
                    }
                    pc = 0x001aa2b8;
                    continue;
                }
                0x001aa2b8 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x73740000;
                    if t {
                        pc = 0x001aa2e4;
                        continue;
                    }
                    pc = 0x001aa2c0;
                    continue;
                }
                0x001aa2c0 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x73650000;
                    c.g[V0][0] = c.g[V0][0] | 0x6c66;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x736f0000;
                    if t {
                        pc = 0x001aa4c4;
                        continue;
                    }
                    pc = 0x001aa2d0;
                    continue;
                }
                0x001aa2d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa684;
                        continue;
                    }
                    pc = 0x001aa2dc;
                    continue;
                }
                0x001aa2dc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa2e4;
                    continue;
                }
                0x001aa2e4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7274;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x74650000;
                    if t {
                        pc = 0x001aa488;
                        continue;
                    }
                    pc = 0x001aa2f0;
                    continue;
                }
                0x001aa2f0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa7d0;
                        continue;
                    }
                    pc = 0x001aa2fc;
                    continue;
                }
                0x001aa2fc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa304;
                    continue;
                }
                0x001aa304 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7374;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa450;
                        continue;
                    }
                    pc = 0x001aa310;
                    continue;
                }
                0x001aa310 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x75750000;
                    if t {
                        pc = 0x001aa368;
                        continue;
                    }
                    pc = 0x001aa318;
                    continue;
                }
                0x001aa318 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x746f0000;
                    c.g[V0][0] = c.g[V0][0] | 0x7361;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa608;
                        continue;
                    }
                    pc = 0x001aa328;
                    continue;
                }
                0x001aa328 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x74730000;
                    if t {
                        pc = 0x001aa348;
                        continue;
                    }
                    pc = 0x001aa330;
                    continue;
                }
                0x001aa330 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x746f0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6f79;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa760;
                        continue;
                    }
                    pc = 0x001aa340;
                    continue;
                }
                0x001aa340 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa348;
                    continue;
                }
                0x001aa348 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7461;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x75670000;
                    if t {
                        pc = 0x001aa7e4;
                        continue;
                    }
                    pc = 0x001aa354;
                    continue;
                }
                0x001aa354 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x616d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa468;
                        continue;
                    }
                    pc = 0x001aa360;
                    continue;
                }
                0x001aa360 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa368;
                    continue;
                }
                0x001aa368 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7372;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if t {
                        pc = 0x001aa45c;
                        continue;
                    }
                    pc = 0x001aa374;
                    continue;
                }
                0x001aa374 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x776f0000;
                    if t {
                        pc = 0x001aa3a0;
                        continue;
                    }
                    pc = 0x001aa37c;
                    continue;
                }
                0x001aa37c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x756d0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6163;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x75720000;
                    if t {
                        pc = 0x001aa470;
                        continue;
                    }
                    pc = 0x001aa38c;
                    continue;
                }
                0x001aa38c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa448;
                        continue;
                    }
                    pc = 0x001aa398;
                    continue;
                }
                0x001aa398 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa3a0;
                    continue;
                }
                0x001aa3a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6c66;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x78690000;
                    if t {
                        pc = 0x001aa654;
                        continue;
                    }
                    pc = 0x001aa3ac;
                    continue;
                }
                0x001aa3ac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7374;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa7c8;
                        continue;
                    }
                    pc = 0x001aa3b8;
                    continue;
                }
                0x001aa3b8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001aa8d8;
                        continue;
                    }
                    pc = 0x001aa3c0;
                    continue;
                }
                0x001aa3c0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d68));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa3c8;
                    continue;
                }
                0x001aa3c8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d88));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa3d4;
                    continue;
                }
                0x001aa3d4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1da0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa3e0;
                    continue;
                }
                0x001aa3e0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1dc0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa3e8;
                    continue;
                }
                0x001aa3e8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1de0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa3f4;
                    continue;
                }
                0x001aa3f4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e00));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa400;
                    continue;
                }
                0x001aa400 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e20));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa408;
                    continue;
                }
                0x001aa408 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e38));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa410;
                    continue;
                }
                0x001aa410 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e50));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa41c;
                    continue;
                }
                0x001aa41c => {
                    c.cycles += 9;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001aa428);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e70));
                    call_ptr(c, 0x00317670, 0x001aa428)?;
                    c.g[A0][0] = 0x3;
                    c.g[V1][0] = 0x83d;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x150), c.g[V1][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc0), c.g[A0][0]);
                    if t {
                        pc = 0x001aa8f4;
                        continue;
                    }
                    pc = 0x001aa440;
                    continue;
                }
                0x001aa440 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e88));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa448;
                    continue;
                }
                0x001aa448 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ea0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa450;
                    continue;
                }
                0x001aa450 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1eb8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa45c;
                    continue;
                }
                0x001aa45c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ed8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa468;
                    continue;
                }
                0x001aa468 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ef0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa470;
                    continue;
                }
                0x001aa470 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f08));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa47c;
                    continue;
                }
                0x001aa47c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f28));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa488;
                    continue;
                }
                0x001aa488 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f40));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa494;
                    continue;
                }
                0x001aa494 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f60));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4a0;
                    continue;
                }
                0x001aa4a0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f80));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa4ac;
                    continue;
                }
                0x001aa4ac => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f98));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4b8;
                    continue;
                }
                0x001aa4b8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fb0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4c4;
                    continue;
                }
                0x001aa4c4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fc8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4d0;
                    continue;
                }
                0x001aa4d0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fd8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa4dc;
                    continue;
                }
                0x001aa4dc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ff8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4e4;
                    continue;
                }
                0x001aa4e4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2010));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4ec;
                    continue;
                }
                0x001aa4ec => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2028));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa4f8;
                    continue;
                }
                0x001aa4f8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2040));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa504;
                    continue;
                }
                0x001aa504 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2058));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa510;
                    continue;
                }
                0x001aa510 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2078));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa51c;
                    continue;
                }
                0x001aa51c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2090));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa528;
                    continue;
                }
                0x001aa528 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20a8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa534;
                    continue;
                }
                0x001aa534 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20c8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa53c;
                    continue;
                }
                0x001aa53c => {
                    c.cycles += 28;
                    c.g[V0][0] = 0x460000;
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20e0));
                    c.g[S5][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20f0));
                    c.g[S3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001aa568);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001aa568)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = c.g[V0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001aa578);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, t, 0x001aa578)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001aa584);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003dbca0, 0x001aa584)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001aa590);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00416810, 0x001aa590)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[RA][0] = sx(0x001aa59c);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00416810, 0x001aa59c)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6b8));
                    let t = c.g[A0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7));
                    if t {
                        pc = 0x001aa5b8;
                        continue;
                    }
                    pc = 0x001aa5ac;
                    continue;
                }
                0x001aa5ac => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001aa5b4);
                    call_ptr(c, 0x00317e98, 0x001aa5b4)?;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x6b8), 0u64);
                    pc = 0x001aa5b8;
                    continue;
                }
                0x001aa5b8 => {
                    c.cycles += 18;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2108));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001aa5d0);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x001aa5d0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x6b8), c.g[V0][0]);
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001aa5ec);
                    c.g[T0][0] = c.g[S5][0];
                    call_ptr(c, 0x002c27c0, 0x001aa5ec)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6b8));
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x001aa8f4;
                        continue;
                    }
                    pc = 0x001aa600;
                    continue;
                }
                0x001aa600 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2118));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa608;
                    continue;
                }
                0x001aa608 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2130));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa614;
                    continue;
                }
                0x001aa614 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2140));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa620;
                    continue;
                }
                0x001aa620 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2158));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa62c;
                    continue;
                }
                0x001aa62c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2170));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa634;
                    continue;
                }
                0x001aa634 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2190));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa640;
                    continue;
                }
                0x001aa640 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21a8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa64c;
                    continue;
                }
                0x001aa64c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21c0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa654;
                    continue;
                }
                0x001aa654 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21d8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa660;
                    continue;
                }
                0x001aa660 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21e8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa66c;
                    continue;
                }
                0x001aa66c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2200));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa674;
                    continue;
                }
                0x001aa674 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2218));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa67c;
                    continue;
                }
                0x001aa67c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2230));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa684;
                    continue;
                }
                0x001aa684 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2248));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa68c;
                    continue;
                }
                0x001aa68c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2268));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa698;
                    continue;
                }
                0x001aa698 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2280));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa6a4;
                    continue;
                }
                0x001aa6a4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2298));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa6ac;
                    continue;
                }
                0x001aa6ac => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x22b8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa6b4;
                    continue;
                }
                0x001aa6b4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x22d0));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa6c0;
                    continue;
                }
                0x001aa6c0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x22e8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa6cc;
                    continue;
                }
                0x001aa6cc => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2308));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa6d8;
                    continue;
                }
                0x001aa6d8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2320));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa6e4;
                    continue;
                }
                0x001aa6e4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2338));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa6f0;
                    continue;
                }
                0x001aa6f0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2358));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa6fc;
                    continue;
                }
                0x001aa6fc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2378));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa704;
                    continue;
                }
                0x001aa704 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2398));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa710;
                    continue;
                }
                0x001aa710 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23b0));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa71c;
                    continue;
                }
                0x001aa71c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23c8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa724;
                    continue;
                }
                0x001aa724 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa72c;
                    continue;
                }
                0x001aa72c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23f8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa738;
                    continue;
                }
                0x001aa738 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2418));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa744;
                    continue;
                }
                0x001aa744 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2430));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa74c;
                    continue;
                }
                0x001aa74c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2450));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa754;
                    continue;
                }
                0x001aa754 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2468));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa760;
                    continue;
                }
                0x001aa760 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2480));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa768;
                    continue;
                }
                0x001aa768 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2498));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa774;
                    continue;
                }
                0x001aa774 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24b0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa77c;
                    continue;
                }
                0x001aa77c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24b0));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa788;
                    continue;
                }
                0x001aa788 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24d0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa794;
                    continue;
                }
                0x001aa794 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24e8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7a0;
                    continue;
                }
                0x001aa7a0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2510));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7a8;
                    continue;
                }
                0x001aa7a8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2528));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7b0;
                    continue;
                }
                0x001aa7b0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21a8));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa7bc;
                    continue;
                }
                0x001aa7bc => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2548));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7c8;
                    continue;
                }
                0x001aa7c8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2560));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7d0;
                    continue;
                }
                0x001aa7d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2580));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7d8;
                    continue;
                }
                0x001aa7d8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2598));
                    if t {
                        pc = 0x001aa8c0;
                        continue;
                    }
                    pc = 0x001aa7e4;
                    continue;
                }
                0x001aa7e4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25b0));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7f0;
                    continue;
                }
                0x001aa7f0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25c8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa7f8;
                    continue;
                }
                0x001aa7f8 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[FP][0] = 0x1;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25e8));
                    if t {
                        pc = 0x001aa8e0;
                        continue;
                    }
                    pc = 0x001aa808;
                    continue;
                }
                0x001aa808 => {
                    c.cycles += 5;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[RA][0] = sx(0x001aa814);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317fe8, 0x001aa814)?;
                    let t = true;
                    c.g[S1][0] = 0x460000;
                    if t {
                        pc = 0x001aa860;
                        continue;
                    }
                    pc = 0x001aa81c;
                    continue;
                }
                0x001aa81c => {
                    c.cycles += 1;
                    pc = 0x001aa820;
                    continue;
                }
                0x001aa820 => {
                    c.cycles += 6;
                    c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = 0xa;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x001aa858;
                        continue;
                    }
                    pc = 0x001aa838;
                    continue;
                }
                0x001aa838 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001aa840);
                    c.g[A2][0] = 0xd;
                    call_ptr(c, 0x00318e68, 0x001aa840)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001aa850);
                    c.g[A2][0] = 0xd;
                    call_ptr(c, 0x00318e68, 0x001aa850)?;
                    let t = true;
                    c.g[A1][0] = 0xa;
                    if t {
                        pc = 0x001aa864;
                        continue;
                    }
                    pc = 0x001aa858;
                    continue;
                }
                0x001aa858 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001aa860);
                    c.g[A2][0] = 0x20;
                    call_ptr(c, 0x00318e68, 0x001aa860)?;
                    pc = 0x001aa860;
                    continue;
                }
                0x001aa860 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0xa;
                    pc = 0x001aa864;
                    continue;
                }
                0x001aa864 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001aa870);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00318c18, 0x001aa870)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001aa820) {
                            return Ok(());
                        }
                        pc = 0x001aa820;
                        continue;
                    }
                    pc = 0x001aa880;
                    continue;
                }
                0x001aa880 => {
                    c.cycles += 13;
                    c.g[V1][0] = 0x2;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2600));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[RA][0] = sx(0x001aa894);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001aa894)?;
                    c.g[V1][0] = 0x3;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001aa8ac);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x001aa8ac)?;
                    let t = true;

                    if t {
                        pc = 0x001aa8f4;
                        continue;
                    }
                    pc = 0x001aa8b4;
                    continue;
                }
                0x001aa8b4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    c.g[S7][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c40));
                    pc = 0x001aa8c0;
                    continue;
                }
                0x001aa8c0 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001aa8c8);
                    call_ptr(c, 0x00317670, 0x001aa8c8)?;
                    c.g[V1][0] = 0x3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    if t {
                        pc = 0x001aa8f4;
                        continue;
                    }
                    pc = 0x001aa8d8;
                    continue;
                }
                0x001aa8d8 => {
                    c.cycles += 2;
                    c.g[S7][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2618));
                    pc = 0x001aa8e0;
                    continue;
                }
                0x001aa8e0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x001aa8e8);
                    call_ptr(c, 0x00317670, 0x001aa8e8)?;
                    c.g[V1][0] = 0x3;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    pc = 0x001aa8f4;
                    continue;
                }
                0x001aa8f4 => {
                    c.cycles += 2;
                    let t = c.g[S7][0] != 0u64;

                    if t {
                        pc = 0x001aa90c;
                        continue;
                    }
                    pc = 0x001aa8fc;
                    continue;
                }
                0x001aa8fc => {
                    c.cycles += 2;
                    let t = c.g[FP][0] == 0u64;
                    c.g[V0][0] = 0x83d;
                    if t {
                        pc = 0x001aa980;
                        continue;
                    }
                    pc = 0x001aa904;
                    continue;
                }
                0x001aa904 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x94), c.g[V0][0]);
                    if t {
                        pc = 0x001aa91c;
                        continue;
                    }
                    pc = 0x001aa90c;
                    continue;
                }
                0x001aa90c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001aa914);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    call_ptr(c, 0x00255fa0, 0x001aa914)?;
                    c.g[V0][0] = 0x83e;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x94), c.g[V0][0]);
                    pc = 0x001aa91c;
                    continue;
                }
                0x001aa91c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x001aa930;
                        continue;
                    }
                    pc = 0x001aa928;
                    continue;
                }
                0x001aa928 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001aa930);
                    call_ptr(c, 0x00260f80, 0x001aa930)?;
                    pc = 0x001aa930;
                    continue;
                }
                0x001aa930 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001aa938);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x00261460, 0x001aa938)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    if t {
                        pc = 0x001aa94c;
                        continue;
                    }
                    pc = 0x001aa940;
                    continue;
                }
                0x001aa940 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001aa948);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x002613c8, 0x001aa948)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    pc = 0x001aa94c;
                    continue;
                }
                0x001aa94c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x001aa95c;
                        continue;
                    }
                    pc = 0x001aa954;
                    continue;
                }
                0x001aa954 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001aa95c);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025b800, 0x001aa95c)?;
                    pc = 0x001aa95c;
                    continue;
                }
                0x001aa95c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001aa964);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x00261408, 0x001aa964)?;
                    c.g[RA][0] = sx(0x001aa96c);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025b800, 0x001aa96c)?;
                    c.g[RA][0] = sx(0x001aa974);
                    call_ptr(c, 0x002591b8, 0x001aa974)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001aa980);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002636f0, 0x001aa980)?;
                    pc = 0x001aa980;
                    continue;
                }
                0x001aa980 => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
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
                }
                _ => {
                    c.jump_out(pc, 0x001a9cc0, 0x001aa9b0);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt};
    use crate::difftest::{check, fresh, run_generated, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the callee fakes below act only on machines that set it.
    pub(crate) const OC904_CTRL: u32 = DATA + 0x24_0000;
    pub(crate) const OC904_MAGIC: u32 = 0x4f43_3930;
    /// The net state (`+0x6b8`: the popup text buffer).
    pub(crate) const OC904_SELF: u32 = DATA + 0x24_1000;
    /// Where the popup pointer is stored.
    pub(crate) const OC904_POPUP_OUT: u32 = DATA + 0x24_2000;
    /// The error event (`+8` the 4-character code, `+0xc` / `+0x10` strings).
    pub(crate) const OC904_EVENT: u32 = DATA + 0x24_3000;
    /// Text the `down` case scans for newlines.
    pub(crate) const OC904_TEXT: u32 = DATA + 0x24_4000;
    /// The world (`+0x8c`: the locale object).
    pub(crate) const OC904_WORLD: u32 = DATA + 0x24_5000;
    pub(crate) const OC904_LOCALE: u32 = DATA + 0x24_5400;
    /// The online error codes the function knows (and unknown ones after).
    pub(crate) const OC904_CODES: &[u32] = &[
        0x696d7374, 0x69706572, 0x70617373, 0x64626572, 0x6d697373, 0x696e7670, 0x6475706c,
        0x6d617870, 0x64656670, 0x6d617574, 0x70617574, 0x75726f6d, 0x756c7374, 0x75757372,
        0x7567616d, 0x756d6163, 0x72616e67, 0x73747274, 0x64657465, 0x6b69636b, 0x686f7374,
        0x696e676d, 0x73656c66, 0x6e707269, 0x6e726f6d, 0x6e67616d, 0x726f6f6d, 0x61757468,
        0x6e657730, 0x70736574, 0x6e6f776e, 0x6c6f676e, 0x66696c74, 0x6c6f636b, 0x72737276,
        0x746f7361, 0x73616d65, 0x69676e6f, 0x6368616c, 0x67706172, 0x66756c6c, 0x64697361,
        0x776f6c66, 0x6b696b68, 0x72616e6b, 0x626f6775, 0x71756974, 0x736f6f6e, 0x74696d65,
        0x6f766572, 0x62616463, 0x6f6e6c6e, 0x61756e72, 0x6e6b696b, 0x626c616b, 0x6d61696c,
        0x706d616c, 0x626f726e, 0x67656e64, 0x7370616d, 0x72617574, 0x6e656d6c, 0x6d616e79,
        0x65636f6e, 0x6967616d, 0x69616472, 0x6e706c61, 0x63686574, 0x746f6f79, 0x6e737276,
        0x69706173, 0x696e616d, 0x656c656e, 0x6974726e, 0x6a6f696e, 0x616a6f69, 0x6e737063,
        0x6e666e64, 0x78697374, 0x7465616d, 0x6d617874, 0x74737461, 0x696e7073, 0x63646576,
        0x646f776e, 0x646e7366,
    ];

    /// `cBXString_find` fake: finds a few newlines, then -1.
    fn oc904_find(c: &mut Cpu) {
        let n = c.bus.read32(OC904_CTRL + 0x10);
        c.bus.write32(OC904_CTRL + 0x10, n + 1);
        let limit = c.bus.read32(OC904_CTRL + 0x14);
        c.g[V0][0] = if n < limit { (n * 3) as u64 } else { u64::MAX };
    }

    fn oc904_noop(_c: &mut Cpu) {}

    macro_rules! oc904_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $post:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC904_CTRL) != OC904_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        $post(c);
                    }
                }
            )*
            pub(crate) fn oc904_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc904_fakes!(
        (
            oc904_fake_001a9c50,
            OC904_PREV_001A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50,
            oc904_noop
        ),
        (
            oc904_fake_001c57c0,
            OC904_PREV_001C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0,
            oc904_noop
        ),
        (
            oc904_fake_0025b800,
            OC904_PREV_0025B800,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800,
            oc904_noop
        ),
        (
            oc904_fake_00260f80,
            OC904_PREV_00260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80,
            oc904_noop
        ),
        (
            oc904_fake_002613c8,
            OC904_PREV_002613C8,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8,
            oc904_noop
        ),
        (
            oc904_fake_00261408,
            OC904_PREV_00261408,
            ee::s06::OVERRIDE_00261408,
            ee::s06::f_00261408__generated,
            0x0026_1408,
            oc904_noop
        ),
        (
            oc904_fake_00261460,
            OC904_PREV_00261460,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460,
            oc904_noop
        ),
        (
            oc904_fake_002636f0,
            OC904_PREV_002636F0,
            ee::s07::OVERRIDE_002636F0,
            ee::s07::f_002636f0__generated,
            0x0026_36f0,
            oc904_noop
        ),
        (
            oc904_fake_002c27c0,
            OC904_PREV_002C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0,
            oc904_noop
        ),
        (
            oc904_fake_00317670,
            OC904_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            oc904_noop
        ),
        (
            oc904_fake_00317e30,
            OC904_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30,
            oc904_noop
        ),
        (
            oc904_fake_00317e98,
            OC904_PREV_00317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98,
            oc904_noop
        ),
        (
            oc904_fake_00317fe8,
            OC904_PREV_00317FE8,
            ee::s10::OVERRIDE_00317FE8,
            ee::s10::f_00317fe8__generated,
            0x0031_7fe8,
            oc904_noop
        ),
        (
            oc904_fake_00318260,
            OC904_PREV_00318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260,
            oc904_noop
        ),
        (
            oc904_fake_00318c18,
            OC904_PREV_00318C18,
            ee::s10::OVERRIDE_00318C18,
            ee::s10::f_00318c18__generated,
            0x0031_8c18,
            oc904_find
        ),
        (
            oc904_fake_00318e68,
            OC904_PREV_00318E68,
            ee::s10::OVERRIDE_00318E68,
            ee::s10::f_00318e68__generated,
            0x0031_8e68,
            oc904_noop
        ),
        (
            oc904_fake_003dbca0,
            OC904_PREV_003DBCA0,
            ee::s14::OVERRIDE_003DBCA0,
            ee::s14::f_003dbca0__generated,
            0x003d_bca0,
            oc904_noop
        ),
        (
            oc904_fake_00416810,
            OC904_PREV_00416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810,
            oc904_noop
        ),
    );

    /// `NetFEState_ShowNetErrorPopup(self, popup_out, owner, event)`: builds a
    /// one-button popup for an online error event; the code at `event+8`
    /// picks the message (about 100 codes through a compare tree). `lock`
    /// formats the account lock text, `down` shows the server text with its
    /// newlines replaced, and `cdev`, `dnsf`, `maut` and unknown codes reset
    /// the online session.
    fn oc904_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x1a_9cc0, 14);
        oc904_install_fakes();
        c.bus.write32(OC904_CTRL, OC904_MAGIC);
        c.bus.write32(OC904_CTRL + 0x14, seed % 4);
        let n = OC904_CODES.len() as u32;
        let k = seed % (n + 3);
        let code = if k < n {
            OC904_CODES[k as usize]
        } else {
            0x7a7a_7a00 + k
        };
        c.bus.write32(OC904_EVENT + 0x8, code);
        c.bus.write32(OC904_EVENT + 0xc, OC904_TEXT + 0x400);
        c.bus.write32(OC904_EVENT + 0x10, OC904_TEXT + 0x800);
        c.bus.write32(
            OC904_SELF + 0x6b8,
            if (seed / n) % 2 == 0 {
                0
            } else {
                OC904_TEXT + 0xc00
            },
        );
        for i in 0..0x20u32 {
            c.bus
                .write8(OC904_TEXT + i, if (i + seed) % 3 == 0 { 0xa } else { 0x61 });
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC904_WORLD);
        c.bus.write32(OC904_WORLD + 0x8c, OC904_LOCALE);
        let vt = fakes::vtable(c, &mut r, &[0x20]);
        c.bus.write32(OC904_LOCALE + 0x4, vt);
        // The `down` case keeps its string object on the stack: [sp] is the text.
        let sp = ssx3_ee_rt::cpu::reg::SP;
        let frame = (c.g[sp][0] as u32).wrapping_sub(0xb0);
        c.bus.write32(frame, OC904_TEXT);
        // Comm system (gp-0x238) and the buddy manager (gp+0x238).
        c.bus
            .write32(GAME_GP.wrapping_sub(0x238), OC904_SELF + 0x800);
        c.bus.write32(OC904_SELF + 0x800 + 0x1c, 0xffff_ffff);
        c.bus.write32(
            GAME_GP.wrapping_add(0x238),
            if seed % 5 == 0 { 0 } else { OC904_SELF + 0x900 },
        );
        c.pc = 0x001a_9cc0;
        c.g[A0][0] = OC904_SELF as u64;
        c.g[A1][0] = OC904_POPUP_OUT as u64;
        c.g[A3][0] = OC904_EVENT as u64;
    }

    #[test]
    fn net_fe_state_show_net_error_popup_matches() {
        let seeds = 330;
        check(
            ee::s03::f_001a9cc0__generated,
            net_fe_state_show_net_error_popup,
            seeds,
            &oc904_setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            oc904_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s03::f_001a9cc0__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 20, "only {} distinct paths", seen.len());
    }
}
