//! Staging module oc972 (wave wave258): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc972`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001c8ef0, measure_desc_box, "cFEPopupConfirm_MeasureDescBox"),
    (0x001c93b0, get_desc_line_wrap_size, "cFEPopupConfirm_GetDescLineWrapSize"),
    (0x001c94b0, get_message_line_wrap_size_w, "cFEPopupConfirm_GetMessageLineWrapSizeW"),
    (0x001c9648, get_message_line_wrap_size, "cFEPopupConfirm_GetMessageLineWrapSize"),
    (0x001c97e0, set_wrapped_text, "cFEPopupConfirm_SetWrappedText"),
    (0x001c9938, word_wrap_w, "cFEPopupConfirm_WordWrapW"),
    (0x001cb500, keyboard_move_left, "cKeyboardPopup_MoveLeft"),
    (0x001cb650, keyboard_move_right, "cKeyboardPopup_MoveRight"),
];

/// `void cFEPopupConfirm::MeasureDescBox(cFEPopupConfirm* self, int* item, float* size, int which)`
/// (0x001c8ef0): measure the text of a popup description item (`item[0]`
/// kind: 1 wide string, 2 narrow string, 3 string-table id, 0 nothing) into
/// `size` (x, y) with `GetTextBoxSize[W]`, then add the extra line height
/// (`f12`) to `size.y`. `which != 0` measures the box at `self+0x74`,
/// else the one at `self+0x78`; a missing box leaves `size` untouched.
pub fn measure_desc_box(c: &mut Cpu) {
    guest(c, |c| {
'b_1c8f2c: {
'b_1c8f28: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S2][0] = c.g[A0][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S3][0] = c.g[A2][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.mov_s(20, 12);
let t = c.g[A3][0] == 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
if t {
break 'b_1c8f28;
} else {
c.cycles += 2;
let t = true;
c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
if t {
break 'b_1c8f2c;
} else {
break 'b_1c8f28;
}
}
}
c.cycles += 1;
c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x78));
break 'b_1c8f2c;
}
'b_1c901c: {
'b_1c9018: {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_1c9018;
} else {
'b_1c9010: {
'b_1c8fa8: {
'b_1c8f8c: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32));
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
if t {
break 'b_1c8f8c;
} else {
'b_1c8f58: {
c.cycles += 2;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = 0x2;
break 'b_1c8f58;
} else {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_1c901c;
} else {
c.cycles += 2;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c9010;
} else {
break 'b_1c8f58;
}
}
}
}
'b_1c8f70: {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x3;
if t {
break 'b_1c8f70;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_1c8fa8;
} else {
c.cycles += 2;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c9010;
} else {
break 'b_1c8f70;
}
}
}
}
c.cycles += 7;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001c8f84);
c.g[A3][0] = c.g[S3][0];
Callee { addr: 0x001c9210, f: ee::s03::cFEPopupConfirm_GetTextBoxSize }.call(c, 0x001c8f84)?;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c9010;
} else {
break 'b_1c8f8c;
}
}
}
c.cycles += 7;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001c8fa0);
c.g[A3][0] = c.g[S3][0];
Callee { addr: 0x001c9038, f: ee::s03::cFEPopupConfirm_GetTextBoxSizeW }.call(c, 0x001c8fa0)?;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c9010;
} else {
break 'b_1c8fa8;
}
}
'b_1c8ffc: {
c.cycles += 9;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001c8fc4);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001c8fc4)?;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = c.g[S2][0];
if t {
break 'b_1c8ffc;
} else {
c.cycles += 12;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0x460000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
c.g[RA][0] = sx(0x001c8fe0);
c.g[A0][0] = c.g[SP][0];
call_ptr(c, 0x00417828, 0x001c8fe0)?;
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = c.g[S1][0];
c.g[A3][0] = c.g[S3][0];
c.g[RA][0] = sx(0x001c8ff4);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x001c9210, f: ee::s03::cFEPopupConfirm_GetTextBoxSize }.call(c, 0x001c8ff4)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_1c901c;
} else {
break 'b_1c8ffc;
}
}
}
c.cycles += 5;
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.g[V0][0];
c.g[RA][0] = sx(0x001c900c);
c.g[A3][0] = c.g[S3][0];
Callee { addr: 0x001c9038, f: ee::s03::cFEPopupConfirm_GetTextBoxSizeW }.call(c, 0x001c900c)?;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
break 'b_1c9010;
}
c.cycles += 2;
c.add_s(0, 0, 20);
c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
break 'b_1c9018;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
break 'b_1c901c;
}
c.cycles += 7;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
    });
}

/// `void cFEPopupConfirm::GetDescLineWrapSize(cFEPopupConfirm* self, int* msg, float* out)`
/// (0x001c93b0): width where the first line of a description message wraps
/// in the text box at `self+0x74`; dispatches on `msg[0]` (1 wide, 2 narrow,
/// 3 string-table id with a `sprintf` fallback) to `GetMessageLineWrapSize[W]`.
pub fn get_desc_line_wrap_size(c: &mut Cpu) {
    guest(c, |c| {
'b_1c9498: {
'b_1c9494: {
c.cycles += 10;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S1][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
let t = c.g[A1][0] == 0u64;
c.g[S2][0] = c.g[A2][0];
if t {
break 'b_1c9494;
} else {
'b_1c946c: {
'b_1c945c: {
'b_1c9430: {
'b_1c9424: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = 0x1;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
if t {
break 'b_1c9424;
} else {
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
break 'b_1c9498;
} else {
'b_1c940c: {
c.cycles += 3;
c.g[V0][0] = 0x2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x3;
if t {
break 'b_1c940c;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_1c9430;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_1c9498;
} else {
break 'b_1c940c;
}
}
}
}
c.cycles += 6;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001c941c);
c.g[A3][0] = c.g[S2][0];
Callee { addr: 0x001c9648, f: ee::s03::cFEPopupConfirm_GetMessageLineWrapSize }.call(c, 0x001c941c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_1c9498;
} else {
break 'b_1c9424;
}
}
}
}
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
let t = true;
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_1c945c;
} else {
break 'b_1c9430;
}
}
c.cycles += 9;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001c944c);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001c944c)?;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S1][0];
if t {
break 'b_1c946c;
} else {
c.cycles += 2;
c.g[A2][0] = c.g[V0][0];
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x74));
break 'b_1c945c;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x001c9464);
c.g[A3][0] = c.g[S2][0];
Callee { addr: 0x001c94b0, f: ee::s03::cFEPopupConfirm_GetMessageLineWrapSizeW }.call(c, 0x001c9464)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
if t {
break 'b_1c9498;
} else {
break 'b_1c946c;
}
}
c.cycles += 10;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0x460000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
c.g[RA][0] = sx(0x001c9480);
c.g[A0][0] = c.g[SP][0];
call_ptr(c, 0x00417828, 0x001c9480)?;
c.g[A0][0] = c.g[S1][0];
c.g[A3][0] = c.g[S2][0];
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x74));
c.g[RA][0] = sx(0x001c9494);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x001c9648, f: ee::s03::cFEPopupConfirm_GetMessageLineWrapSize }.call(c, 0x001c9494)?;
break 'b_1c9494;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
break 'b_1c9498;
}
c.cycles += 5;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return Ok(());
    });
}

/// `void cFEPopupConfirm::GetMessageLineWrapSizeW(self, text, ushort* str, float* out)`
/// (0x001c94b0): width at which the wide string `str` overflows the width of
/// the text box `text` (summing glyph widths times the box scale), or its total
/// width when it fits; a double `D_004A1C38` char resets the run.
pub fn get_message_line_wrap_size_w(c: &mut Cpu) {
    guest(c, |c| {
'b_1c961c: {
c.cycles += 22;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S0][0] = c.g[A1][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[A0][0] = c.g[S0][0];
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S6][0] = c.g[A3][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S7][0] = c.g[A2][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.g[V0][0] | 0x80;
c.g[RA][0] = sx(0x001c94fc);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
call_ptr(c, 0x003a04f0, 0x001c94fc)?;
c.g[S5][0] = c.g[V0][0];
if c.g[S5][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
break 'b_1c961c;
} else {
'b_1c9600: {
c.cycles += 31;
c.g[A0][0] = c.g[S7][0];
c.g[RA][0] = sx(0x001c9514);
c.g[S1][0] = 0u64;
call_ptr(c, 0x003dbca0, 0x001c9514)?;
c.mtc1(0, 20);
c.g[S3][0] = c.g[V0][0];
c.g[A1][0] = 0x460000;
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
c.g[A2][0] = 0x100;
c.g[A3][0] = 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60e8));
c.g[RA][0] = sx(0x001c953c);
c.g[S4][0] = 0u64;
call_ptr(c, 0x00317e30, 0x001c953c)?;
c.g[S2][0] = c.g[V0][0];
c.g[A1][0] = c.g[S7][0];
c.g[RA][0] = sx(0x001c954c);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x003dbe20, 0x001c954c)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001c9564);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001c9564)?;
c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x57));
c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x50));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
let t = (c.g[S3][0] as i64) <= 0;

if t {
break 'b_1c9600;
} else {
c.cycles += 3;
c.g[S0][0] = c.g[S2][0];
c.g[V1][0] = c.lhu((c.g[S2][0] as u32));
'l_1c9590: loop {
'b_1c95f4: {
'b_1c95b8: {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[GP][0] as u32).wrapping_sub(0x14b8));
if c.g[V1][0] != c.g[V0][0] {
c.g[A1][0] = c.lhu((c.g[S0][0] as u32));
break 'b_1c95b8;
} else {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
if c.g[V0][0] != c.g[V1][0] {
c.g[A1][0] = c.lhu((c.g[S0][0] as u32));
break 'b_1c95b8;
} else {
c.cycles += 4;
c.mtc1(0, 20);
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
let t = true;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
if t {
break 'b_1c95f4;
} else {
break 'b_1c95b8;
}
}
}
}
'b_1c95f0: {
c.cycles += 10;
c.g[RA][0] = sx(0x001c95c0);
c.g[A0][0] = c.g[S5][0];
call_ptr(c, 0x00392c60, 0x001c95c0)?;
c.lwc1(1, (c.g[SP][0] as u32));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.mul_s(0, 0, 1);
c.add_s(20, 20, 0);
c.c_lt_s(2, 20);
if !c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
break 'b_1c95f0;
} else {
c.cycles += 4;
c.sub_s(0, 20, 0);
c.g[S4][0] = 0x1;
let t = true;
c.swc1(0, (c.g[S6][0] as u32));
if t {
break 'b_1c9600;
} else {
break 'b_1c95f0;
}
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
break 'b_1c95f4;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lhu((c.g[S0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x001c9590); } continue 'l_1c9590;
} else {
break 'b_1c9600;
}
}
}
}
'b_1c9608: {
c.cycles += 2;
if c.g[S4][0] == 0u64 {
c.swc1(20, (c.g[S6][0] as u32));
break 'b_1c9608;
} else {
break 'b_1c9608;
}
}
c.cycles += 2;
let t = c.g[S2][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
if t {
break 'b_1c961c;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x001c9618);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00317e98, 0x001c9618)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
break 'b_1c961c;
}
}
}
c.cycles += 11;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
    });
}

/// `void cFEPopupConfirm::GetMessageLineWrapSize(self, text, char* str, float* out)`
/// (0x001c9648): narrow-string twin of `GetMessageLineWrapSizeW` (a double backslash
/// and a carriage return reset the run).
pub fn get_message_line_wrap_size(c: &mut Cpu) {
    guest(c, |c| {
'b_1c97b8: {
c.cycles += 21;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S0][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A0][0] = c.g[S0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.g[A2][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S4][0] = c.g[A3][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.g[V0][0] | 0x80;
c.g[RA][0] = sx(0x001c9690);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
call_ptr(c, 0x003a04f0, 0x001c9690)?;
c.g[S2][0] = c.g[V0][0];
if c.g[S2][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1c97b8;
} else {
'b_1c979c: {
c.cycles += 29;
c.mtc1(0, 20);
c.g[RA][0] = sx(0x001c96a8);
c.g[A0][0] = c.g[S3][0];
call_ptr(c, 0x00416810, 0x001c96a8)?;
c.g[S5][0] = 0u64;
c.g[A1][0] = 0x460000;
c.g[A2][0] = 0x100;
c.g[A3][0] = 0u64;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[RA][0] = sx(0x001c96c4);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60f8));
call_ptr(c, 0x00317e30, 0x001c96c4)?;
c.g[S1][0] = c.g[V0][0];
c.g[A1][0] = c.g[S3][0];
c.g[RA][0] = sx(0x001c96d4);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x004166f4, 0x001c96d4)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001c96ec);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001c96ec)?;
c.g[V1][0] = c.lb((c.g[S1][0] as u32));
c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x57));
c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x50));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
let t = c.g[V1][0] == 0u64;

if t {
break 'b_1c979c;
} else {
c.cycles += 6;
c.g[S6][0] = 0x5c;
c.g[S3][0] = 0xd;
c.g[S0][0] = c.g[S1][0];
c.g[V0][0] = c.lb((c.g[S1][0] as u32));
c.g[A0][0] = c.g[S0][0];
'l_1c9728: loop {
'b_1c9790: {
'b_1c9744: {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S6][0];
c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x1));
if t {
break 'b_1c9744;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];

if t {
break 'b_1c9744;
} else {
c.cycles += 3;
c.mtc1(0, 20);
let t = true;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
if t {
break 'b_1c9790;
} else {
break 'b_1c9744;
}
}
}
}
'b_1c9758: {
c.cycles += 2;
if c.g[V0][0] != c.g[S3][0] {
c.g[A1][0] = c.lb((c.g[A0][0] as u32));
break 'b_1c9758;
} else {
c.cycles += 3;
c.mtc1(0, 20);
let t = true;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if t {
break 'b_1c9790;
} else {
break 'b_1c9758;
}
}
}
c.cycles += 10;
c.g[RA][0] = sx(0x001c9760);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00392ba8, 0x001c9760)?;
c.lwc1(1, (c.g[SP][0] as u32));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.mul_s(0, 0, 1);
c.add_s(20, 20, 0);
c.c_lt_s(2, 20);
if !c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_1c9790;
} else {
c.cycles += 4;
c.sub_s(0, 20, 0);
c.g[S5][0] = 0x1;
let t = true;
c.swc1(0, (c.g[S4][0] as u32));
if t {
break 'b_1c979c;
} else {
break 'b_1c9790;
}
}
}
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
if event_due(c) { deliver_event_here(c, 0x001c9728); } continue 'l_1c9728;
} else {
break 'b_1c979c;
}
}
}
}
'b_1c97a4: {
c.cycles += 2;
if c.g[S5][0] == 0u64 {
c.swc1(20, (c.g[S4][0] as u32));
break 'b_1c97a4;
} else {
break 'b_1c97a4;
}
}
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1c97b8;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x001c97b4);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x00317e98, 0x001c97b4)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1c97b8;
}
}
}
c.cycles += 10;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return Ok(());
    });
}

/// `void cFEPopupConfirm::SetWrappedText(self, int* info, float* out, int which, float extra)`
/// (0x001c97e0): set the text of the box at `self+0x74` (`which != 0`) or
/// `self+0x78` from an item (`info[0]` 0 desc, 1 wide, 2 narrow, 3 table id):
/// word wrap / measure it, and add `extra` to `out[1]` unless the narrow
/// fallback or the desc case returned early.
pub fn set_wrapped_text(c: &mut Cpu) {
    guest(c, |c| {
'b_1c981c: {
'b_1c9818: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S2][0] = c.g[A1][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S3][0] = c.g[A2][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.mov_s(20, 12);
let t = c.g[A3][0] == 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
if t {
break 'b_1c9818;
} else {
c.cycles += 2;
let t = true;
c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
if t {
break 'b_1c981c;
} else {
break 'b_1c9818;
}
}
}
c.cycles += 1;
c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
break 'b_1c981c;
}
'b_1c9918: {
'b_1c9914: {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_1c9914;
} else {
'b_1c990c: {
'b_1c98f8: {
'b_1c98c8: {
'b_1c98b8: {
'b_1c988c: {
'b_1c987c: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
if t {
break 'b_1c987c;
} else {
'b_1c9848: {
c.cycles += 2;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = 0x2;
break 'b_1c9848;
} else {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_1c98f8;
} else {
c.cycles += 2;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c990c;
} else {
break 'b_1c9848;
}
}
}
}
'b_1c9860: {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x3;
if t {
break 'b_1c9860;
} else {
c.cycles += 2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_1c988c;
} else {
c.cycles += 2;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c990c;
} else {
break 'b_1c9860;
}
}
}
}
c.cycles += 7;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001c9874);
c.g[A3][0] = c.g[S3][0];
Callee { addr: 0x001c9b28, f: ee::s03::cFEPopupConfirm_SetTextMeasured }.call(c, 0x001c9874)?;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c990c;
} else {
break 'b_1c987c;
}
}
}
c.cycles += 4;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S0][0];
let t = true;
c.g[A1][0] = c.g[S1][0];
if t {
break 'b_1c98b8;
} else {
break 'b_1c988c;
}
}
c.cycles += 9;
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001c98a8);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001c98a8)?;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_1c98c8;
} else {
c.cycles += 2;
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.g[V0][0];
break 'b_1c98b8;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x001c98c0);
c.g[A3][0] = c.g[S3][0];
Callee { addr: 0x001c9938, f: ee::s03::cFEPopupConfirm_WordWrapW }.call(c, 0x001c98c0)?;
let t = true;
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
if t {
break 'b_1c990c;
} else {
break 'b_1c98c8;
}
}
c.cycles += 12;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0x460000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a8));
c.g[RA][0] = sx(0x001c98dc);
c.g[A0][0] = c.g[SP][0];
call_ptr(c, 0x00417828, 0x001c98dc)?;
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = c.g[S1][0];
c.g[A3][0] = c.g[S3][0];
c.g[RA][0] = sx(0x001c98f0);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x001c9b28, f: ee::s03::cFEPopupConfirm_SetTextMeasured }.call(c, 0x001c98f0)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_1c9918;
} else {
break 'b_1c98f8;
}
}
c.cycles += 5;
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001c9904);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x001c8050, f: ee::s03::cFEPopupConfirm_SetTextFromDesc }.call(c, 0x001c9904)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_1c9918;
} else {
break 'b_1c990c;
}
}
c.cycles += 2;
c.add_s(0, 0, 20);
c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
break 'b_1c9914;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
break 'b_1c9918;
}
c.cycles += 7;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
    });
}

/// `void cFEPopupConfirm::WordWrapW(self, text, ushort* str, float* out)`
/// (0x001c9938): copy `str` to a heap buffer, break it at the last space
/// (turned into `\r`) whenever the line outgrows the box width, measure the
/// result with `GetTextBoxSizeW`, set it on the text with `UIText_SetWideString`
/// and free the buffer.
pub fn word_wrap_w(c: &mut Cpu) {
    guest(c, |c| {
'b_1c9af8: {
c.cycles += 24;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S0][0] = c.g[A2][0];
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.g[A1][0];
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[FP][0] = c.g[A3][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.g[S3][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.g[V0][0] | 0x80;
c.g[RA][0] = sx(0x001c998c);
c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[V0][0]);
call_ptr(c, 0x003a04f0, 0x001c998c)?;
c.g[S7][0] = c.g[V0][0];
if c.g[S7][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_1c9af8;
} else {
'b_1c9ac8: {
c.cycles += 31;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x001c99a4);
c.g[S6][0] = 0xffffffffffffffff;
call_ptr(c, 0x003dbca0, 0x001c99a4)?;
c.mtc1(0, 20);
c.g[S5][0] = c.g[V0][0];
c.g[A1][0] = 0x460000;
c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
c.g[A2][0] = 0x100;
c.g[A3][0] = 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60e8));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[RA][0] = sx(0x001c99cc);
c.g[S1][0] = 0u64;
call_ptr(c, 0x00317e30, 0x001c99cc)?;
c.g[S2][0] = c.g[V0][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x001c99dc);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x003dbe20, 0x001c99dc)?;
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001c99f4);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001c99f4)?;
c.ldl(2, (c.g[S3][0] as u32).wrapping_add(0x57));
c.ldr(2, (c.g[S3][0] as u32).wrapping_add(0x50));
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
let t = (c.g[S5][0] as i64) <= 0;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_1c9ac8;
} else {
c.cycles += 1;
c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
'l_1c9a18: loop {
'b_1c9ab8: {
'b_1c9a50: {
'b_1c9a4c: {
c.cycles += 6;
c.g[V1][0] = c.lb((c.g[GP][0] as u32).wrapping_sub(0x14b8));
c.g[A1][0] = c.g[V0][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[A0][0] = c.lhu((c.g[V0][0] as u32));
let t = c.g[A0][0] != c.g[V1][0];
c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
if t {
break 'b_1c9a4c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
let t = c.g[V0][0] != c.g[A0][0];
c.g[S4][0] = 0xd;
if t {
break 'b_1c9a50;
} else {
c.cycles += 4;
c.g[S6][0] = 0xffffffffffffffff;
c.mtc1(0, 20);
let t = true;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
if t {
break 'b_1c9ab8;
} else {
break 'b_1c9a4c;
}
}
}
}
c.cycles += 1;
c.g[S4][0] = 0xd;
break 'b_1c9a50;
}
'b_1c9ab4: {
'b_1c9a68: {
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
if c.g[V0][0] != c.g[S4][0] {
c.g[A1][0] = c.lhu((c.g[S0][0] as u32));
break 'b_1c9a68;
} else {
c.cycles += 3;
c.mtc1(0, 20);
let t = true;
c.g[S6][0] = 0xffffffffffffffff;
if t {
break 'b_1c9ab4;
} else {
break 'b_1c9a68;
}
}
}
c.cycles += 12;
c.g[RA][0] = sx(0x001c9a70);
c.g[A0][0] = c.g[S7][0];
call_ptr(c, 0x00392c60, 0x001c9a70)?;
c.lwc1(1, (c.g[SP][0] as u32));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.mul_s(0, 0, 1);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
c.g[V0][0] = c.g[V0][0] ^ 0x20;
c.add_s(20, 20, 0);
c.c_lt_s(2, 20);
let t = !c.fcc();
if c.g[V0][0] == 0 { c.g[S6][0] = c.g[S1][0]; }
if t {
break 'b_1c9ab4;
} else {
c.cycles += 2;
let t = (c.g[S6][0] as i64) < 0;
c.g[V0][0] = sx((c.g[S6][0] as u32) << 1);
if t {
break 'b_1c9ab4;
} else {
c.cycles += 5;
c.g[S1][0] = c.g[S6][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.mtc1(0, 20);
c.sh((c.g[V0][0] as u32), c.g[S4][0]);
c.g[S6][0] = 0xffffffffffffffff;
break 'b_1c9ab4;
}
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
break 'b_1c9ab8;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S5][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
if t {
if event_due(c) { deliver_event_here(c, 0x001c9a18); } continue 'l_1c9a18;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_1c9ac8;
}
}
}
}
c.cycles += 9;
c.g[A1][0] = c.g[S3][0];
c.g[A3][0] = c.g[FP][0];
c.g[RA][0] = sx(0x001c9ad8);
c.g[A2][0] = c.g[S2][0];
Callee { addr: 0x001c9038, f: ee::s03::cFEPopupConfirm_GetTextBoxSizeW }.call(c, 0x001c9ad8)?;
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x001c9ae4);
c.g[A1][0] = c.g[S2][0];
call_ptr(c, 0x003a0e90, 0x001c9ae4)?;
let t = c.g[S2][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
if t {
break 'b_1c9af8;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x001c9af4);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00317e98, 0x001c9af4)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_1c9af8;
}
}
}
c.cycles += 12;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.pc = t; return Ok(());
    });
}

/// `void cKeyboardPopup::MoveLeft(cKeyboardPopup* self, int a1)`
/// (0x001cb500): move the on-screen keyboard cursor one key left (wrapping
/// inside its row of 14, skipping `0x2a` holes of `D_00441390`); a disabled
/// key recurses to keep moving, `a1` records the previous key at `+0x434`.
pub fn keyboard_move_left(c: &mut Cpu) {
    guest(c, |c| {
'b_1cb530: {
'b_1cb52c: {
c.cycles += 9;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_1cb52c;
} else {
c.cycles += 2;
let t = true;
c.g[S1][0] = c.g[V0][0] ^ 0x1;
if t {
break 'b_1cb530;
} else {
break 'b_1cb52c;
}
}
}
c.cycles += 1;
c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
break 'b_1cb530;
}
'b_1cb54c: {
c.cycles += 5;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf));
c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
if t {
break 'b_1cb54c;
} else {
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
c.g[S1][0] = c.g[V0][0] ^ 0x1;
break 'b_1cb54c;
}
}
'b_1cb638: {
'b_1cb570: {
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x430));
if t {
break 'b_1cb570;
} else {
c.cycles += 5;
c.g[V1][0] = 0x2;
c.sw((c.g[S0][0] as u32).wrapping_add(0x434), c.g[A2][0]);
c.g[V0][0] = 0x3;
let t = c.g[A0][0] != c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x430), c.g[V1][0]);
if t {
break 'b_1cb570;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[A3][0]);
if t {
break 'b_1cb638;
} else {
break 'b_1cb570;
}
}
}
}
'b_1cb5c0: {
'b_1cb5bc: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
if c.g[V1][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb5bc;
} else {
c.cycles += 3;
c.g[V0][0] = 0xe;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb5bc;
} else {
c.cycles += 3;
c.g[V0][0] = 0x1c;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb5bc;
} else {
c.cycles += 3;
c.g[V0][0] = 0x2a;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb5bc;
} else {
c.cycles += 3;
c.g[V0][0] = 0x38;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb5bc;
} else {
c.cycles += 3;
c.g[V0][0] = 0x46;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
if t {
break 'b_1cb5c0;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb5bc;
}
}
}
}
}
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd));
break 'b_1cb5c0;
}
'b_1cb60c: {
c.cycles += 9;
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.g[V0][0] = 0x440000;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1390));
c.g[A0][0] = 0x2a;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = c.lb((c.g[V0][0] as u32));
if c.g[V1][0] != c.g[A0][0] {
c.g[A0][0] = c.g[S0][0];
break 'b_1cb60c;
} else {
c.cycles += 1;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
'l_1cb5e8: loop {
c.cycles += 7;
c.g[A1][0] = c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = c.lb((c.g[V1][0] as u32));
let t = c.g[V0][0] == c.g[A0][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x001cb5e8); } continue 'l_1cb5e8;
} else {
c.cycles += 2;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[A0][0] = c.g[S0][0];
break 'b_1cb60c;
}
}
}
}
'b_1cb634: {
c.cycles += 4;
c.g[RA][0] = sx(0x001cb614);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cb614)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_1cb634;
} else {
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x001cb628);
c.g[A1][0] = 0u64;
Callee { addr: 0x001cb500, f: ee::s04::cKeyboardPopup_MoveLeft }.call(c, 0x001cb628)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = (c.g[S1][0] < 0x1u64) as u64;
if t {
break 'b_1cb638;
} else {
break 'b_1cb634;
}
}
}
c.cycles += 1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x144), c.g[V0][0]);
break 'b_1cb638;
}
c.cycles += 5;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t; return Ok(());
    });
}

/// `void cKeyboardPopup::MoveRight(cKeyboardPopup* self, int a1)`
/// (0x001cb650): mirror of `MoveLeft`: one key right, wrapping at the row end.
pub fn keyboard_move_right(c: &mut Cpu) {
    guest(c, |c| {
'b_1cb684: {
'b_1cb680: {
c.cycles += 10;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_1cb680;
} else {
c.cycles += 2;
let t = true;
c.g[S2][0] = c.g[V0][0] ^ 0x1;
if t {
break 'b_1cb684;
} else {
break 'b_1cb680;
}
}
}
c.cycles += 1;
c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
break 'b_1cb684;
}
'b_1cb6a0: {
c.cycles += 5;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf));
c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
if t {
break 'b_1cb6a0;
} else {
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
c.g[S2][0] = c.g[V0][0] ^ 0x1;
break 'b_1cb6a0;
}
}
'b_1cb7d8: {
'b_1cb70c: {
'b_1cb708: {
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x430));
if t {
break 'b_1cb708;
} else {
c.cycles += 5;
c.g[V1][0] = 0x3;
c.sw((c.g[S0][0] as u32).wrapping_add(0x434), c.g[A2][0]);
c.g[V0][0] = 0x2;
let t = c.g[A0][0] != c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x430), c.g[V1][0]);
if t {
break 'b_1cb708;
} else {
'b_1cb6ec: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x144));
if c.g[V0][0] == c.g[A0][0] {
c.g[A0][0] = c.g[S0][0];
break 'b_1cb6ec;
} else {
'b_1cb6e0: {
c.cycles += 2;
let t = c.g[S2][0] == 0u64;

if t {
break 'b_1cb6e0;
} else {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_1cb6ec;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
if t {
break 'b_1cb70c;
} else {
break 'b_1cb6e0;
}
}
}
}
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb70c;
} else {
c.cycles += 1;
c.g[A0][0] = c.g[S0][0];
break 'b_1cb6ec;
}
}
}
c.cycles += 5;
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x001cb6f8);
c.g[A2][0] = 0u64;
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cb6f8)?;
if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb70c;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[S1][0]);
if t {
break 'b_1cb7d8;
} else {
break 'b_1cb708;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb70c;
}
'b_1cb75c: {
'b_1cb758: {
c.cycles += 3;
c.g[V0][0] = 0xd;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb758;
} else {
c.cycles += 3;
c.g[V0][0] = 0x1b;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb758;
} else {
c.cycles += 3;
c.g[V0][0] = 0x29;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb758;
} else {
c.cycles += 3;
c.g[V0][0] = 0x37;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb758;
} else {
c.cycles += 3;
c.g[V0][0] = 0x45;
if c.g[V1][0] == c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb758;
} else {
c.cycles += 3;
c.g[V0][0] = 0x53;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
if t {
break 'b_1cb75c;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb758;
}
}
}
}
}
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd));
break 'b_1cb75c;
}
'b_1cb7ac: {
c.cycles += 9;
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.g[V0][0] = 0x440000;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1390));
c.g[A0][0] = 0x2a;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = c.lb((c.g[V0][0] as u32));
if c.g[V1][0] != c.g[A0][0] {
c.g[A0][0] = c.g[S0][0];
break 'b_1cb7ac;
} else {
c.cycles += 2;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
'l_1cb788: loop {
c.cycles += 7;
c.g[A1][0] = c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = c.lb((c.g[V1][0] as u32));
let t = c.g[V0][0] == c.g[A0][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x001cb788); } continue 'l_1cb788;
} else {
c.cycles += 2;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[A0][0] = c.g[S0][0];
break 'b_1cb7ac;
}
}
}
}
'b_1cb7d4: {
c.cycles += 4;
c.g[RA][0] = sx(0x001cb7b4);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cb7b4)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_1cb7d4;
} else {
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x001cb7c8);
c.g[A1][0] = 0u64;
Callee { addr: 0x001cb650, f: ee::s04::cKeyboardPopup_MoveRight }.call(c, 0x001cb7c8)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = (c.g[S2][0] < 0x1u64) as u64;
if t {
break 'b_1cb7d8;
} else {
break 'b_1cb7d4;
}
}
}
c.cycles += 1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x144), c.g[V0][0]);
break 'b_1cb7d8;
}
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine_rt, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    // Fixture layout (clear of the test allocator's DATA+0x40000..0x80000 arena
    // and of the fakes' call log at DATA+0x100000).
    pub(crate) const OC972_SELF: u32 = DATA + 0x1_0000;
    pub(crate) const OC972_TEXT_A: u32 = DATA + 0x1_1000;
    pub(crate) const OC972_TEXT_B: u32 = DATA + 0x1_2000;
    pub(crate) const OC972_VT: u32 = DATA + 0x1_3000;
    pub(crate) const OC972_F1: u32 = DATA + 0x1_3400;
    pub(crate) const OC972_F2: u32 = DATA + 0x1_3500;
    pub(crate) const OC972_F3: u32 = DATA + 0x1_3600;
    pub(crate) const OC972_F4: u32 = DATA + 0x1_3700;
    pub(crate) const OC972_FONT: u32 = DATA + 0x1_4000;
    pub(crate) const OC972_GLYPHS: u32 = DATA + 0x1_4100;
    pub(crate) const OC972_ITEM: u32 = DATA + 0x1_5000;
    pub(crate) const OC972_STRW: u32 = DATA + 0x1_5100;
    pub(crate) const OC972_STRN: u32 = DATA + 0x1_5300;
    pub(crate) const OC972_OUT: u32 = DATA + 0x1_5400;
    pub(crate) const OC972_WORLD: u32 = DATA + 0x1_5500;
    pub(crate) const OC972_OBJ: u32 = DATA + 0x1_5600;
    pub(crate) const OC972_TVT: u32 = DATA + 0x1_5700;

    /// Machine with the allocator runtime, the string-table object chain
    /// (`gp-0x848 -> +0x8c -> +4 = vtable, slot +0x20/+0x24`) and the
    /// `sprintf` format `D_004660A8`.
    fn oc972_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        let r = machine_rt(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(GAME_GP - 0x848, OC972_WORLD);
        c.bus.write32(OC972_WORLD + 0x8c, OC972_OBJ);
        c.bus.write32(OC972_OBJ + 4, OC972_TVT);
        c.bus.write16(OC972_TVT + 0x20, 0);
        for (i, b) in b"id%d\0".iter().enumerate() {
            c.bus.write8(0x0046_60a8 + i as u32, *b);
        }
        // D_004A1C38: the escape character of the wide-string wrappers.
        c.bus.write8(GAME_GP - 0x14b8, 0x5c);
        r
    }

    /// A text box (`cUIText`) whose font chain, glyph widths and box-size
    /// method (a fake in the vtable at +0xa0/+0xa4) `UIText_GetFont`,
    /// `Font_GetCharWidth[W]` and the callers read. `font_ok = false` makes
    /// `UIText_GetFont` return null (the early exit).
    fn oc972_text(c: &mut Cpu, r: &mut Rng, text: u32, seed: u32, font_ok: bool) {
        let idx = seed % 4;
        c.bus.write32(text + 0x14, r.next_u32());
        c.bus.write32(text + 0x08, OC972_VT);
        c.bus.write16(OC972_VT + 0xa0, 0);
        c.bus.write32(OC972_VT + 0xa4, FAKE + 0x10);
        c.bus.write32(text + 0x5c, OC972_F1);
        c.bus.write32(OC972_F1 + 0xd0, OC972_F2);
        c.bus
            .write32(OC972_F2 + 0x10, if font_ok { OC972_F3 } else { 0 });
        c.bus.write32(OC972_F3 + 8, OC972_F4);
        c.bus.write32(text + 0x74, idx);
        for k in 0..4 {
            c.bus.write32(OC972_F4 + 12 * k + 0xc, OC972_FONT);
        }
        for k in 0..3 {
            let scale = 0.5 + (r.next_u32() % 100) as f32 / 100.0;
            c.bus.write32(text + 0x50 + 4 * k, scale.to_bits());
        }
        // Direct-indexed glyph table: min code 0, max 0x7f, width byte at +8.
        c.bus.write32(OC972_FONT, 0);
        c.bus.write32(OC972_FONT + 4, OC972_GLYPHS);
        c.bus.write32(OC972_FONT + 0xc, 0);
        c.bus.write32(OC972_FONT + 0x10, 0x7f);
        for ch in 0..0x80u32 {
            c.bus
                .write8(OC972_GLYPHS + 12 * ch + 8, (2 + ch % 5) as u8);
        }
    }

    /// Wide string at `at`: spaces, letters, `\r` and doubled escapes.
    fn oc972_wide(c: &mut Cpu, r: &mut Rng, at: u32) {
        let len = r.next_u32() % 16;
        let mut i = 0;
        while i < len {
            let ch = match r.next_u32() % 8 {
                0 | 1 => 0x20,
                2 => 0x0d,
                3 => {
                    c.bus.write16(at + 2 * i, 0x5c);
                    i += 1;
                    0x5c
                }
                _ => 0x41 + (r.next_u32() % 26) as u16,
            };
            c.bus.write16(at + 2 * i, ch);
            i += 1;
        }
        c.bus.write16(at + 2 * i, 0);
    }

    /// Narrow string at `at`: letters, spaces, `\r` and doubled backslashes.
    fn oc972_narrow(c: &mut Cpu, r: &mut Rng, at: u32) {
        let len = r.next_u32() % 16;
        let mut i = 0;
        while i < len {
            let ch = match r.next_u32() % 8 {
                0 | 1 => b' ',
                2 => 0x0d,
                3 => {
                    c.bus.write8(at + i, b'\\');
                    i += 1;
                    b'\\'
                }
                _ => b'A' + (r.next_u32() % 26) as u8,
            };
            c.bus.write8(at + i, ch);
            i += 1;
        }
        c.bus.write8(at + i, 0);
    }

    /// A message item `{kind, payload}` at `OC972_ITEM` by `sel`: 0 nothing,
    /// 1 wide string, 2 narrow string, 3 table id answered with a string (the
    /// fake returns `OC972_STRW`), 4 table id answered with 0 (the real leaf
    /// 0x1406c0: the `sprintf` fallback), 5 an unknown kind.
    fn oc972_item(c: &mut Cpu, r: &mut Rng, sel: u32) {
        oc972_wide(c, r, OC972_STRW);
        oc972_narrow(c, r, OC972_STRN);
        let (kind, payload) = match sel % 6 {
            0 => (0, 0),
            1 => (1, OC972_STRW),
            2 => (2, OC972_STRN),
            3 | 4 => (3, r.next_u32() % 1000),
            _ => (7, 0),
        };
        c.bus.write32(OC972_ITEM, kind);
        c.bus.write32(OC972_ITEM + 4, payload);
        let method = if sel % 6 == 4 { 0x0014_06c0 } else { FAKE + 0x20 };
        c.bus.write32(OC972_TVT + 0x24, method);
        if sel % 6 == 3 {
            c.bus.write32(LOG_RESULTS, OC972_STRW);
        }
    }

    /// The popup: boxes at `+0x74`/`+0x78` (each missing now and then).
    fn oc972_popup(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let a = if seed % 11 == 0 { 0 } else { OC972_TEXT_A };
        let b = if seed % 13 == 0 { 0 } else { OC972_TEXT_B };
        c.bus.write32(OC972_SELF + 0x74, a);
        c.bus.write32(OC972_SELF + 0x78, b);
        oc972_text(c, r, OC972_TEXT_A, seed, seed % 7 != 0);
        oc972_text(c, r, OC972_TEXT_B, seed / 3, seed % 5 != 0);
        c.bus.write32(OC972_OUT, r.f32(10.0));
        c.bus.write32(OC972_OUT + 4, r.f32(10.0));
    }

    /// Box width the (fake) size method leaves on the stack at `sp + 0x10`
    /// of a frame of `frame` bytes opened at the function's entry.
    fn oc972_box_width(c: &mut Cpu, r: &mut Rng, frame: u32) {
        let w = (8 + r.next_u32() % 90) as f32;
        let at = (c.g[SP][0] as u32).wrapping_sub(frame) + 0x10;
        c.bus.write32(at, w.to_bits());
    }

    /// `MeasureDescBox`: every item kind, both boxes, a missing box, a null
    /// font, the string-table fallback, and unwinds in the callees.
    #[test]
    fn measure_desc_box_matches() {
        check(
            ee::s03::f_001c8ef0__generated,
            measure_desc_box,
            240,
            &|c, seed| {
                let mut r = oc972_base(c, seed, 0x1c_8ef0, 6, 0x001c_8ef0);
                oc972_popup(c, &mut r, seed);
                oc972_item(c, &mut r, seed);
                c.g[A0][0] = sx(OC972_SELF);
                c.g[A1][0] = sx(OC972_ITEM);
                c.g[A2][0] = sx(OC972_OUT);
                c.g[A3][0] = ((seed / 6) % 3) as u64;
                c.f[12] = r.f32(8.0);
            },
        );
    }

    /// `GetDescLineWrapSize`: the same item kinds against the box at `+0x74`.
    #[test]
    fn get_desc_line_wrap_size_matches() {
        check(
            ee::s03::f_001c93b0__generated,
            get_desc_line_wrap_size,
            240,
            &|c, seed| {
                let mut r = oc972_base(c, seed, 0x1c_93b0, 6, 0x001c_93b0);
                oc972_popup(c, &mut r, seed);
                oc972_item(c, &mut r, seed);
                c.g[A0][0] = sx(OC972_SELF);
                c.g[A1][0] = sx(OC972_ITEM);
                c.g[A2][0] = sx(OC972_OUT);
            },
        );
    }

    /// `GetMessageLineWrapSizeW`: wide string against box widths that it does
    /// and does not overflow, with a null font now and then.
    #[test]
    fn get_message_line_wrap_size_w_matches() {
        check(
            ee::s03::f_001c94b0__generated,
            get_message_line_wrap_size_w,
            240,
            &|c, seed| {
                let mut r = oc972_base(c, seed, 0x1c_94b0, 6, 0x001c_94b0);
                oc972_text(c, &mut r, OC972_TEXT_A, seed, seed % 9 != 0);
                oc972_wide(c, &mut r, OC972_STRW);
                oc972_box_width(c, &mut r, 0xc0);
                c.bus.write32(OC972_OUT, r.f32(10.0));
                c.g[A0][0] = sx(OC972_SELF);
                c.g[A1][0] = sx(OC972_TEXT_A);
                c.g[A2][0] = sx(OC972_STRW);
                c.g[A3][0] = sx(OC972_OUT);
            },
        );
    }

    /// `GetMessageLineWrapSize`: the narrow twin.
    #[test]
    fn get_message_line_wrap_size_matches() {
        check(
            ee::s03::f_001c9648__generated,
            get_message_line_wrap_size,
            240,
            &|c, seed| {
                let mut r = oc972_base(c, seed, 0x1c_9648, 6, 0x001c_9648);
                oc972_text(c, &mut r, OC972_TEXT_A, seed, seed % 9 != 0);
                oc972_narrow(c, &mut r, OC972_STRN);
                oc972_box_width(c, &mut r, 0xb0);
                c.bus.write32(OC972_OUT, r.f32(10.0));
                c.g[A0][0] = sx(OC972_SELF);
                c.g[A1][0] = sx(OC972_TEXT_A);
                c.g[A2][0] = sx(OC972_STRN);
                c.g[A3][0] = sx(OC972_OUT);
            },
        );
    }

    /// `SetWrappedText`: the four item kinds into either box, then `out.y +=
    /// extra` (not after the table-id fallback or the desc case).
    #[test]
    fn set_wrapped_text_matches() {
        check(
            ee::s03::f_001c97e0__generated,
            set_wrapped_text,
            240,
            &|c, seed| {
                let mut r = oc972_base(c, seed, 0x1c_97e0, 8, 0x001c_97e0);
                oc972_popup(c, &mut r, seed);
                oc972_item(c, &mut r, seed);
                c.g[A0][0] = sx(OC972_SELF);
                c.g[A1][0] = sx(OC972_ITEM);
                c.g[A2][0] = sx(OC972_OUT);
                c.g[A3][0] = ((seed / 6) % 3) as u64;
                c.f[12] = r.f32(8.0);
            },
        );
    }

    /// `WordWrapW`: wide strings with spaces wrapped at the box width (break
    /// at the last space), carriage returns and doubled escapes.
    #[test]
    fn word_wrap_w_matches() {
        check(
            ee::s03::f_001c9938__generated,
            word_wrap_w,
            240,
            &|c, seed| {
                let mut r = oc972_base(c, seed, 0x1c_9938, 6, 0x001c_9938);
                oc972_text(c, &mut r, OC972_TEXT_A, seed, seed % 9 != 0);
                oc972_wide(c, &mut r, OC972_STRW);
                oc972_box_width(c, &mut r, 0xe0);
                c.bus.write32(OC972_OUT, r.f32(10.0));
                c.bus.write32(OC972_OUT + 4, r.f32(10.0));
                c.g[A0][0] = sx(OC972_SELF);
                c.g[A1][0] = sx(OC972_TEXT_A);
                c.g[A2][0] = sx(OC972_STRW);
                c.g[A3][0] = sx(OC972_OUT);
            },
        );
    }

    /// The keyboard: self fields `+0x68`/`+0x6c` (shift flags), `+0x140` key,
    /// `+0x144` state, `+0x148..` enabled flags, `+0x430`/`+0x434` history, and
    /// the key-hole table `D_00441390` (`0x2a` entries are skipped). Every
    /// row keeps an enabled key so the recursion ends.
    fn oc972_keyboard(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = oc972_base(c, seed, salt, 0, salt as u32);
        let me = OC972_SELF;
        c.bus.write32(me + 0x68, (seed % 2) as u32);
        c.bus.write32(me + 0x6c, ((seed / 2) % 2) as u32);
        let wrap = [0u32, 13, 14, 27, 28, 41, 42, 55, 56, 69, 70, 83];
        let key = if seed % 3 == 0 {
            wrap[(seed as usize / 3) % wrap.len()]
        } else {
            r.next_u32() % 84
        };
        c.bus.write32(me + 0x140, key);
        c.bus.write32(me + 0x144, r.next_u32() % 3);
        c.bus.write32(me + 0x430, (seed / 4) % 4);
        c.bus.write32(me + 0x434, r.next_u32() % 84);
        for i in 0..0x100 {
            c.bus.write8(0x0044_1390 + i, 0);
        }
        for _ in 0..(seed % 4) {
            let hole = r.next_u32() % 84;
            if !wrap.contains(&hole) {
                c.bus.write8(0x0044_1390 + hole, 0x2a);
            }
        }
        for i in 0..0x100u32 {
            let on = (r.next_u32() % 3 != 0) as u32;
            c.bus.write32(me + 0x148 + 4 * i, on);
        }
        for row in 0..6u32 {
            c.bus.write32(me + 0x148 + 4 * (row * 14 + 13), 1);
            c.bus.write32(me + 0x148 + 4 * (row * 14 + 6), 1);
        }
        c.g[A0][0] = sx(me);
        c.g[A1][0] = ((seed / 5) % 2) as u64;
    }

    /// `MoveLeft`: all rows and wraps, the history flag `a1`, a disabled key
    /// (the recursive call), holes in the key table.
    #[test]
    fn keyboard_move_left_matches() {
        check(
            ee::s04::f_001cb500__generated,
            keyboard_move_left,
            240,
            &|c, seed| oc972_keyboard(c, seed, 0x1c_b500),
        );
    }

    /// `MoveRight`: the mirror, with the `st == 2` re-check of the old key.
    #[test]
    fn keyboard_move_right_matches() {
        check(
            ee::s04::f_001cb650__generated,
            keyboard_move_right,
            240,
            &|c, seed| oc972_keyboard(c, seed, 0x1c_b650),
        );
    }
}
