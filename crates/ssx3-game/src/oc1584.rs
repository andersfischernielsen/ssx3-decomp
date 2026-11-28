//! Staging module oc1584 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1584`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `SND_BankIsInUse` `int SNDbankinuse(int bank)` (0x003b65d0): -8 when `SND_BankValidate` fails;
/// otherwise 1 when a sound handle of that bank is not done (its
/// `SND_IsHandleDone` is 0), else 0. Walks the bank table under the lock.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_bank_is_in_use(c: &mut Cpu) {
    guest(c, |c| {
'b_3b6654: {
c.cycles += 11;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S1][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003b65f4);
c.g[S3][0] = 0u64;
Callee { addr: 0x003b64d8, f: ee::s13::SND_BankValidate }.call(c, 0x003b65f4)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0xfffffffffffffff8;
if t {
break 'b_3b6654;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003b6604);
c.g[S0][0] = 0u64;
Callee { addr: 0x003b58a0, f: ee::s13::SND_Lock }.call(c, 0x003b6604)?;
c.g[S2][0] = 0x510000;
'l_3b6608: loop {
'b_3b6648: {
c.cycles += 5;
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x5718));
c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x17e));
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x8c;
if t {
break 'b_3b6648;
} else {
c.cycles += 6;
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f0));
c.mult(0, 2, 16, 2);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x12));
if c.g[V1][0] != c.g[S1][0] {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if event_due(c) { deliver_event_here(c, 0x003b6608); } continue 'l_3b6608;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003b663c);
c.g[A0][0] = c.lw((c.g[A0][0] as u32));
Callee { addr: 0x003b8ab8, f: ee::s13::SND_IsHandleDone }.call(c, 0x003b663c)?;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x003b6608); } continue 'l_3b6608;
} else {
c.cycles += 1;
c.g[S3][0] = 0x1;
break 'b_3b6648;
}
}
}
}
c.cycles += 3;
c.g[RA][0] = sx(0x003b6650);
Callee { addr: 0x003b58d8, f: ee::s13::SND_Unlock }.call(c, 0x003b6650)?;
c.g[V0][0] = c.g[S3][0];
break 'b_3b6654;
}
}
}
c.cycles += 7;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return Ok(());
    });
}
/// `SND_GetSoundInfo` `SND_GetSoundInfo(bank, a1, info, track, t0)` (0x003b6670): -8 when the bank
/// is invalid or has no patch; else under the lock parses the stream header
/// into the info, frees the track buffers (count at `info + 2`) and returns 0.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_get_sound_info(c: &mut Cpu) {
    guest(c, |c| {
'b_3b675c: {
'b_3b66e4: {
'b_3b66dc: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S1][0] = c.g[A1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[A2][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S5][0] = c.g[T0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003b66ac);
c.g[S6][0] = c.g[A3][0];
Callee { addr: 0x003b64d8, f: ee::s13::SND_BankValidate }.call(c, 0x003b66ac)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = 0x510000;
if t {
break 'b_3b66dc;
} else {
c.cycles += 10;
c.g[S3][0] = sx((c.g[S0][0] as u32) << 4);
c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
c.g[A1][0] = c.g[S1][0];
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1f4));
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[RA][0] = sx(0x003b66d0);
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
Callee { addr: 0x003c6540, f: ee::s13::SND_BankGetPatch }.call(c, 0x003b66d0)?;
c.g[S0][0] = c.g[V0][0];
let t = c.g[S0][0] != 0u64;

if t {
break 'b_3b66e4;
} else {
break 'b_3b66dc;
}
}
}
c.cycles += 2;
let t = true;
c.g[V0][0] = 0xfffffffffffffff8;
if t {
break 'b_3b675c;
} else {
break 'b_3b66e4;
}
}
'b_3b6750: {
c.cycles += 14;
c.g[RA][0] = sx(0x003b66ec);
c.g[S1][0] = 0u64;
Callee { addr: 0x003b58a0, f: ee::s13::SND_Lock }.call(c, 0x003b66ec)?;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1f4));
c.g[A1][0] = c.g[S0][0];
c.g[T0][0] = c.g[S5][0];
c.g[A2][0] = c.g[S2][0];
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A3][0] = c.g[S6][0];
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
c.g[RA][0] = sx(0x003b6710);
c.g[T1][0] = 0u64;
Callee { addr: 0x003bb820, f: ee::s13::SND_ParseStreamHeader }.call(c, 0x003b6710)?;
c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x2));
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x18));
if t {
break 'b_3b6750;
} else {
c.cycles += 1;
'l_3b6720: loop {
'b_3b6740: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32));
if c.g[A0][0] == 0u64 {
c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x2));
break 'b_3b6740;
} else {
c.cycles += 5;
c.g[RA][0] = sx(0x003b6734);
Callee { addr: 0x003b5e98, f: ee::s13::SND_MemFree }.call(c, 0x003b6734)?;
c.sw((c.g[S0][0] as u32), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x2));
break 'b_3b6740;
}
}
c.cycles += 4;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003b6720); } continue 'l_3b6720;
} else {
break 'b_3b6750;
}
}
}
}
c.cycles += 3;
c.g[RA][0] = sx(0x003b6758);
Callee { addr: 0x003b58d8, f: ee::s13::SND_Unlock }.call(c, 0x003b6758)?;
c.g[V0][0] = 0u64;
break 'b_3b675c;
}
c.cycles += 10;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return Ok(());
    });
}
/// `SNDSTRM_Stop` `int SNDSTRM_Stop(stream)` (0x003b78f8): under the lock, -8 when the stream
/// is missing; else stops its slot (status at +4 not negative), runs the close
/// callback unless the flag at +0x11 is set, moves the nodes of the list at
/// +0x11c to the one at +0x128, frees the buffers (count at +0x1a), clears four
/// fields of the stream,
/// unlocks and returns 0.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_strm_stop(c: &mut Cpu) {
    guest(c, |c| {
'b_3b7a44: {
'b_3b794c: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003b7928);
c.g[S0][0] = c.g[A0][0];
Callee { addr: 0x003b58a0, f: ee::s13::SND_Lock }.call(c, 0x003b7928)?;
c.g[RA][0] = sx(0x003b7930);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x003b6a38, f: ee::s13::SNDSTRM_GetStream }.call(c, 0x003b7930)?;
c.g[S0][0] = c.g[V0][0];
let t = c.g[S0][0] != 0u64;

if t {
break 'b_3b794c;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003b7944);
Callee { addr: 0x003b58d8, f: ee::s13::SND_Unlock }.call(c, 0x003b7944)?;
let t = true;
c.g[V0][0] = 0xfffffffffffffff8;
if t {
break 'b_3b7a44;
} else {
break 'b_3b794c;
}
}
}
'b_3b7968: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = 0xffffffffffffffff;
if t {
break 'b_3b7968;
} else {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[RA][0] = sx(0x003b7964);
Callee { addr: 0x003c6ce8, f: ee::s13::SNDSTRM_StopSlot }.call(c, 0x003b7964)?;
c.g[V0][0] = 0xffffffffffffffff;
break 'b_3b7968;
}
}
'b_3b7984: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x11));
if c.g[V1][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
break 'b_3b7984;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003b7980);
c.g[A0][0] = c.lw((c.g[S0][0] as u32));
call_ptr(c, 0x003e1110, 0x003b7980)?;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
break 'b_3b7984;
}
}
'b_3b79d0: {
c.cycles += 6;
c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11c));
c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
let t = c.g[V0][0] == 0u64;
c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
if t {
break 'b_3b79d0;
} else {
c.cycles += 3;
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
let t = c.g[V0][0] == 0u64;
c.g[S2][0] = 0u64;
if t {
break 'b_3b79d0;
} else {
c.cycles += 2;
c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
'l_3b79b0: loop {
c.cycles += 7;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[RA][0] = sx(0x003b79bc);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
Callee { addr: 0x003b5e98, f: ee::s13::SND_MemFree }.call(c, 0x003b79bc)?;
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
if event_due(c) { deliver_event_here(c, 0x003b79b0); } continue 'l_3b79b0;
} else {
c.cycles += 1;
break 'b_3b79d0;
}
}
}
}
}
'l_3b79d0: loop {
'b_3b79f0: {
c.cycles += 4;
c.g[RA][0] = sx(0x003b79d8);
c.g[A0][0] = c.g[S3][0];
Callee { addr: 0x003c62e8, f: ee::s13::SND_ListPopFront }.call(c, 0x003b79d8)?;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.g[V0][0];
if t {
break 'b_3b79f0;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x003b79e8);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x128));
Callee { addr: 0x003c6288, f: ee::s13::SND_ListPushFront }.call(c, 0x003b79e8)?;
let t = true;

if t {
if event_due(c) { deliver_event_here(c, 0x003b79d0); } continue 'l_3b79d0;
} else {
break 'b_3b79f0;
}
}
}
c.cycles += 21;
c.sb((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
c.g[A0][0] = c.g[S4][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x134), 0u64);
c.g[A1][0] = 0u64;
c.g[RA][0] = sx(0x003b7a08);
c.g[A2][0] = 0x4;
call_ptr(c, 0x00416210, 0x003b7a08)?;
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = 0u64;
c.g[RA][0] = sx(0x003b7a18);
c.g[A2][0] = 0x4;
call_ptr(c, 0x00416210, 0x003b7a18)?;
c.g[A0][0] = c.g[S6][0];
c.g[A1][0] = 0u64;
c.g[RA][0] = sx(0x003b7a28);
c.g[A2][0] = 0x68;
call_ptr(c, 0x00416210, 0x003b7a28)?;
c.g[A0][0] = c.g[S7][0];
c.g[A1][0] = 0u64;
c.g[RA][0] = sx(0x003b7a38);
c.g[A2][0] = 0x68;
call_ptr(c, 0x00416210, 0x003b7a38)?;
c.g[RA][0] = sx(0x003b7a40);
Callee { addr: 0x003b58d8, f: ee::s13::SND_Unlock }.call(c, 0x003b7a40)?;
c.g[V0][0] = 0u64;
break 'b_3b7a44;
}
}
c.cycles += 11;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.pc = t; return Ok(());
    });
}
/// `SNDSTRM_SetAuxSend` `int SNDSTRM_SetAuxSend(stream, a1, level)` (0x003b8050): -8 when the stream
/// is missing; else stores the level at stream+0xf1 and sets the aux send of
/// the stream's channel (`SND_SetAuxSend`).
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_strm_set_aux_send(c: &mut Cpu) {
    guest(c, |c| {
'b_3b808c: {
c.cycles += 9;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S0][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003b806c);
c.g[S1][0] = c.g[A1][0];
Callee { addr: 0x003b6a38, f: ee::s13::SNDSTRM_GetStream }.call(c, 0x003b806c)?;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = 0xfffffffffffffff8;
break 'b_3b808c;
} else {
c.cycles += 6;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003b8088);
c.sb((c.g[V0][0] as u32).wrapping_add(0xf1), c.g[S0][0]);
Callee { addr: 0x003b88c8, f: ee::s13::SND_SetAuxSend }.call(c, 0x003b8088)?;
c.g[V0][0] = 0u64;
break 'b_3b808c;
}
}
c.cycles += 5;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return Ok(());
    });
}
/// `SND_SetAuxSend` `SND_SetAuxSend(handle, a1, level)` (0x003b88c8): sets the aux send of every
/// channel of the handle (-1 when the handle is invalid). Resumable: the whole
/// `match pc` is kept, so an unwound callee or event resumes at its head.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_set_aux_send(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003b88c8 => {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003b88f0);
                c.g[S2][0] = c.g[A2][0];
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b88f0)?;
                c.g[S1][0] = c.g[V0][0];
                let t = (c.g[S1][0] as i64) < 0; 
                c.g[V1][0] = 0xffffffffffffffff;
                if t { pc = 0x003b8950; continue; }
                pc = 0x003b88fc; continue;
            }
            0x003b88fc => {
                c.cycles += 5;
                c.g[V0][0] = 0x510000;
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                let t = true; 
                c.g[S3][0] = 0x8c;
                if t { pc = 0x003b893c; continue; }
                pc = 0x003b8910; continue;
            }
            0x003b8910 => {
                c.cycles += 11;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x1f0));
                c.mult(0, 2, 2, 19);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x61), c.g[S2][0]);
                c.g[RA][0] = sx(0x003b8930);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                Callee { addr: 0x003ba740, f: ee::s13::SND_ChannelCalcSend }.call(c, 0x003b8930)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                c.g[RA][0] = sx(0x003b893c);
                c.g[A1][0] = c.g[S0][0];
                Callee { addr: 0x003c27e0, f: ee::s13::SND_ChannelApplySend }.call(c, 0x003b893c)?;
                pc = 0x003b893c; continue;
            }
            0x003b893c => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003b8948);
                c.g[A1][0] = c.g[SP][0];
                Callee { addr: 0x003baa00, f: ee::s13::SND_NextHandleChannel }.call(c, 0x003b8948)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                if t { if back_edge_unwinds(c, 0x003b8910) { return Ok(()); } pc = 0x003b8910; continue; }
                pc = 0x003b8950; continue;
            }
            0x003b8950 => {
                c.cycles += 9;
                c.g[V0][0] = c.g[S1][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b88c8, 0x003b8974); return Ok(()); }
        }
    }
    });
}
/// `SND_GetTimeRemaining` `SND_GetTimeRemaining(handle)` (0x003b8ad8): the smallest remaining time over
/// the handle's channels, starting from -7 (also the result for an invalid
/// handle). Resumable: the whole `match pc` is kept.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003b8ad8 => {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003b8b00);
                c.g[S1][0] = 0xfffffffffffffff9;
                Callee { addr: 0x003ba6e8, f: ee::s13::SND_HandleToChannel }.call(c, 0x003b8b00)?;
                c.g[S2][0] = c.g[V0][0];
                let t = (c.g[S2][0] as i64) < 0; 
                c.g[V1][0] = 0xffffffffffffffff;
                if t { pc = 0x003b8bd8; continue; }
                pc = 0x003b8b0c; continue;
            }
            0x003b8b0c => {
                c.cycles += 6;
                c.g[V0][0] = 0x510000;
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5718));
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                c.g[S4][0] = 0x8c;
                let t = true; 
                c.g[S3][0] = 0x64;
                if t { pc = 0x003b8bc4; continue; }
                pc = 0x003b8b24; continue;
            }
            0x003b8b24 => {
                c.cycles += 1;
                pc = 0x003b8b28; continue;
            }
            0x003b8b28 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x1f0));
                c.mult(0, 2, 5, 20);
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                let t = (c.g[V1][0] as i64) > 0; 
                c.g[V1][0] = 0xfffffffffffffff9;
                if t { pc = 0x003b8b4c; continue; }
                pc = 0x003b8b40; continue;
            }
            0x003b8b40 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003b8b48);
                c.g[A0][0] = c.g[A1][0];
                Callee { addr: 0x003c4318, f: ee::s13::SND_ChannelGetPosition }.call(c, 0x003b8b48)?;
                c.g[V1][0] = c.g[V0][0];
                pc = 0x003b8b4c; continue;
            }
            0x003b8b4c => {
                c.cycles += 2;
                
                if (c.g[V1][0] as i64) < 0 { c.g[A0][0] = c.g[S2][0]; pc = 0x003b8bc8; continue; }
                pc = 0x003b8b54; continue;
            }
            0x003b8b54 => {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.mult(0, 2, 2, 19);
                
                if c.g[A0][0] == 0u64 {  pc = 0x003b8b6c; continue; }
                pc = 0x003b8b6c; continue;
            }
            0x003b8b6c => {
                c.cycles += 6;
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8a));
                c.g[V1][0] = (c.g[V1][0] < 0x11u64) as u64;
                c.div(0, 2, 4);
                c.g[V0][0] = c.lo[0];
                let t = c.g[V1][0] != 0u64; 
                c.g[A0][0] = c.g[V0][0];
                if t { pc = 0x003b8bb4; continue; }
                pc = 0x003b8b84; continue;
            }
            0x003b8b84 => {
                c.cycles += 5;
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8a));
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 12);
                c.div(0, 2, 3);
                
                if c.g[V1][0] == 0u64 {  pc = 0x003b8b98; continue; }
                pc = 0x003b8b98; continue;
            }
            0x003b8b98 => {
                c.cycles += 7;
                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6a));
                c.g[V0][0] = c.lo[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 12);
                c.div(0, 2, 4);
                c.g[V0][0] = c.lo[0];
                let t = true; 
                c.g[A0][0] = c.g[V0][0];
                if t { pc = 0x003b8bbc; continue; }
                pc = 0x003b8bb4; continue;
            }
            0x003b8bb4 => {
                c.cycles += 2;
                c.g[A0][0] = 0x7fff0000;
                c.g[A0][0] = c.g[A0][0] | 0xffff;
                pc = 0x003b8bbc; continue;
            }
            0x003b8bbc => {
                c.cycles += 2;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] != 0 { c.g[S1][0] = c.g[A0][0]; }
                pc = 0x003b8bc4; continue;
            }
            0x003b8bc4 => {
                c.cycles += 1;
                c.g[A0][0] = c.g[S2][0];
                pc = 0x003b8bc8; continue;
            }
            0x003b8bc8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b8bd0);
                c.g[A1][0] = c.g[SP][0];
                Callee { addr: 0x003baa00, f: ee::s13::SND_NextHandleChannel }.call(c, 0x003b8bd0)?;
                
                if c.g[V0][0] != 0u64 { c.g[A1][0] = c.lw((c.g[SP][0] as u32)); if back_edge_unwinds(c, 0x003b8b28) { return Ok(()); } pc = 0x003b8b28; continue; }
                pc = 0x003b8bd8; continue;
            }
            0x003b8bd8 => {
                c.cycles += 10;
                c.g[V0][0] = c.g[S1][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b8ad8, 0x003b8c00); return Ok(()); }
        }
    }
    });
}
/// `SNDMIDI_Shutdown` `SNDMIDI_Shutdown()` (0x003b9880): under the lock, stops every MIDI track,
/// removes the tick callback and clears the MIDI state word; returns 0.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_midi_shutdown(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003b9890);
Callee { addr: 0x003b58a0, f: ee::s13::SND_Lock }.call(c, 0x003b9890)?;
c.g[RA][0] = sx(0x003b9898);
Callee { addr: 0x003b98c0, f: ee::s13::SNDMIDI_StopAll }.call(c, 0x003b9898)?;
c.g[A0][0] = 0x3c0000;
c.g[RA][0] = sx(0x003b98a4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6948));
Callee { addr: 0x003b5948, f: ee::s13::SND_RemoveTickCallback }.call(c, 0x003b98a4)?;
c.g[V0][0] = 0x510000;
c.g[RA][0] = sx(0x003b98b0);
c.sw((c.g[V0][0] as u32).wrapping_sub(0x5530), 0u64);
Callee { addr: 0x003b58d8, f: ee::s13::SND_Unlock }.call(c, 0x003b98b0)?;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.g[V0][0] = 0u64;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.pc = t; return Ok(());
    });
}
/// `SNDMIDI_Play` `SNDMIDI_Play(song, data, a2, a3)` (0x003b9a00): skips a 4-byte or 8-byte
/// header in `data`, then walks the track records: active ones are counted (16
/// end the walk); the first free record is filled from the song and its value returned.
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code, unused_mut)]
pub fn snd_midi_play(c: &mut Cpu) {
    guest(c, |c| {
'b_3b9a44: {
'b_3b9a2c: {
c.cycles += 9;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
c.g[T5][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = 0x444d;
c.sq(18, (c.g[SP][0] as u32));
c.g[V1][0] = c.lh((c.g[A1][0] as u32));
let t = c.g[V1][0] != c.g[V0][0];
c.g[T1][0] = 0xfffffffffffffff7;
if t {
break 'b_3b9a2c;
} else {
c.cycles += 2;
let t = true;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
if t {
break 'b_3b9a44;
} else {
break 'b_3b9a2c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V1][0] = 0x78440000;
c.g[V1][0] = c.g[V1][0] | 0x494d;
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
if c.g[V0][0] == 0 { c.g[A1][0] = c.g[A0][0]; }
break 'b_3b9a44;
}
'b_3b9a60: {
c.cycles += 6;
c.g[V1][0] = 0x510000;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5ee0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
let t = (c.g[V0][0] as i64) >= 0;
c.sw((c.g[V1][0] as u32).wrapping_add(0x7c0), c.g[V0][0]);
if t {
break 'b_3b9a60;
} else {
c.cycles += 1;
c.sw((c.g[V1][0] as u32).wrapping_add(0x7c0), 0u64);
break 'b_3b9a60;
}
}
c.cycles += 8;
c.g[T8][0] = c.g[V1][0];
c.g[T4][0] = 0u64;
c.g[A0][0] = 0x64;
c.g[S0][0] = 0xff;
c.g[S1][0] = 0x1;
c.g[S2][0] = 0x3;
c.g[T0][0] = c.g[T8][0];
'l_3b9a80: loop {
'b_3b9b4c: {
'b_3b9b40: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
if (c.g[V0][0] as i64) >= 0 {
c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
break 'b_3b9b40;
} else {
c.cycles += 33;
c.sw((c.g[T0][0] as u32).wrapping_add(0x4), 0u64);
c.g[T2][0] = 0x10000;
c.sw((c.g[T0][0] as u32), 0u64);
c.g[T1][0] = 0xffffffffff010000;
c.g[T9][0] = 0xfffffffffffffc00;
c.g[T2][0] = c.g[T2][0] | 0xfc00;
c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x10));
c.g[T1][0] = c.g[T1][0] | 0xffff;
c.g[T7][0] = 0x800000;
c.g[T6][0] = 0xffffffffffffffff;
c.mult(0, 3, 3, 4);
c.g[T3][0] = 0xf;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x30));
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 12) as u32);
c.sh((c.g[T0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x7c0));
c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
c.sh((c.g[T0][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
c.sw((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.sh((c.g[T0][0] as u32).wrapping_add(0x2e), c.g[A3][0]);
c.sb((c.g[T0][0] as u32).wrapping_add(0x26), c.g[S0][0]);
c.sb((c.g[T0][0] as u32).wrapping_add(0x27), c.g[S1][0]);
c.sw((c.g[T0][0] as u32).wrapping_add(0x18), 0u64);
c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
c.g[V0][0] = c.lb((c.g[T5][0] as u32));
c.sb((c.g[T0][0] as u32).wrapping_add(0x28), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
c.sw((c.g[T0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
c.g[V1][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
c.sh((c.g[T0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0xc));
c.sh((c.g[T0][0] as u32).wrapping_add(0x22), c.g[V0][0]);
'l_3b9b10: loop {
c.cycles += 10;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.g[V0][0] & c.g[T9][0];
c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
c.g[V0][0] = c.g[V0][0] | c.g[T7][0];
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.sb((c.g[A0][0] as u32).wrapping_add(0x3), c.g[T6][0]);
let t = (c.g[T3][0] as i64) >= 0;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003b9b10); } continue 'l_3b9b10;
} else {
c.cycles += 2;
let t = true;
c.g[T1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
if t {
break 'b_3b9b4c;
} else {
break 'b_3b9b40;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T4][0] as i64) < 16i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x70));
if t {
if event_due(c) { deliver_event_here(c, 0x003b9a80); } continue 'l_3b9a80;
} else {
break 'b_3b9b4c;
}
}
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.g[T1][0];
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lq(18, (c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.pc = t; return Ok(());
}
    });
}
/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_65d0, snd_bank_is_in_use, "SND_BankIsInUse"),
    (0x003b_6670, snd_get_sound_info, "SND_GetSoundInfo"),
    (0x003b_78f8, snd_strm_stop, "SNDSTRM_Stop"),
    (0x003b_8050, snd_strm_set_aux_send, "SNDSTRM_SetAuxSend"),
    (0x003b_88c8, snd_set_aux_send, "SND_SetAuxSend"),
    (0x003b_8ad8, snd_get_time_remaining, "SND_GetTimeRemaining"),
    (0x003b_9880, snd_midi_shutdown, "SNDMIDI_Shutdown"),
    (0x003b_9a00, snd_midi_play, "SNDMIDI_Play"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;

    /// This module's region of test RAM: the gate word, the forced-result
    /// mailboxes (8 bytes each: magic, value), the two list countdowns, the
    /// list nodes and the fixture records. Gated fakes act only on machines
    /// whose gate word holds `OC1584_MAGIC`.
    const OC1584_BASE: u32 = DATA + 0x6f_0000;
    const OC1584_GATE: u32 = OC1584_BASE;
    const OC1584_MAGIC: u32 = 0x1584_c0de;
    const OC1584_MBOX: u32 = OC1584_BASE + 0x100;
    /// Nodes `SND_ListPopFront` hands out while this countdown lasts.
    const OC1584_POPS: u32 = OC1584_BASE + 0x300;
    /// Channels `SND_NextHandleChannel` reports while this countdown lasts.
    const OC1584_NEXTS: u32 = OC1584_BASE + 0x304;
    const OC1584_NODES: u32 = OC1584_BASE + 0x1000;
    const OC1584_STREAM: u32 = OC1584_BASE + 0x2000;
    const OC1584_RECORDS: u32 = OC1584_BASE + 0x4000;
    const OC1584_BANKS: u32 = OC1584_BASE + 0x5000;
    const OC1584_SONG: u32 = OC1584_BASE + 0x8000;
    const OC1584_TRACK: u32 = OC1584_BASE + 0x9000;

    /// The sound globals (`0x510000 - 0x5718`) and the MIDI track table
    /// (`0x510000 - 0x5ee0`, the `gp`-free absolute addresses the bodies use).
    const SNDG: u32 = 0x0050_a8e8;
    const MIDI_TRACKS: u32 = 0x0050_a120;

    /// A gated stand-in for the callee at `$addr`: it logs the call through
    /// `fakes::fake` (every register, an unwind at the chosen call index)
    /// and, when the test forced a result in mailbox `$k`, returns it. With
    /// the gate off it runs the body the slot held before.
    macro_rules! oc1584_gated_fake {
        ($name:ident, $k:expr, $addr:expr, $generated:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1584_GATE) != OC1584_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                let mbox = OC1584_MBOX + 8 * $k;
                if c.pc == ra && c.bus.read32(mbox) == OC1584_MAGIC {
                    c.g[V0][0] = sx(c.bus.read32(mbox + 4));
                }
            }
        };
    }

    oc1584_gated_fake!(
        oc1584_fake_bank_validate,
        0,
        0x003b_64d8,
        ee::s13::f_003b64d8__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_lock,
        1,
        0x003b_58a0,
        ee::s13::f_003b58a0__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_unlock,
        2,
        0x003b_58d8,
        ee::s13::f_003b58d8__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_handle_done,
        3,
        0x003b_8ab8,
        ee::s13::f_003b8ab8__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_bank_patch,
        4,
        0x003c_6540,
        ee::s13::f_003c6540__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_parse_header,
        5,
        0x003b_b820,
        ee::s13::f_003bb820__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_mem_free,
        6,
        0x003b_5e98,
        ee::s13::f_003b5e98__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_get_stream,
        7,
        0x003b_6a38,
        ee::s13::f_003b6a38__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_stop_slot,
        8,
        0x003c_6ce8,
        ee::s13::f_003c6ce8__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_set_aux_send,
        9,
        0x003b_88c8,
        ee::s13::f_003b88c8__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_handle_to_channel,
        10,
        0x003b_a6e8,
        ee::s13::f_003ba6e8__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_calc_send,
        11,
        0x003b_a740,
        ee::s13::f_003ba740__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_apply_send,
        12,
        0x003c_27e0,
        ee::s13::f_003c27e0__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_channel_position,
        13,
        0x003c_4318,
        ee::s13::f_003c4318__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_stop_all,
        14,
        0x003b_98c0,
        ee::s13::f_003b98c0__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_remove_tick,
        15,
        0x003b_5948,
        ee::s13::f_003b5948__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_list_push,
        16,
        0x003c_6288,
        ee::s13::f_003c6288__generated
    );
    oc1584_gated_fake!(
        oc1584_fake_stream_callback,
        17,
        0x003e_1110,
        ee::s14::f_003e1110__generated
    );

    /// `SND_ListPopFront` (0x3c62e8): a node while the `OC1584_POPS`
    /// countdown lasts, then null (so the drain loops end).
    pub(crate) fn oc1584_fake_list_pop(c: &mut Cpu) {
        if c.bus.read32(OC1584_GATE) != OC1584_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1584_fake_list_pop),
                ee::s13::f_003c62e8__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003c_62e8, ra);
        if c.pc == ra {
            let n = c.bus.read32(OC1584_POPS);
            c.g[V0][0] = 0;
            if n > 0 {
                c.bus.write32(OC1584_POPS, n - 1);
                c.g[V0][0] = sx(OC1584_NODES + 0x40 * n);
            }
        }
    }

    /// `SND_NextHandleChannel` (0x3baa00): reports channel `n` through the
    /// out pointer in `a1` (and returns 1) while the `OC1584_NEXTS`
    /// countdown lasts, then 0.
    pub(crate) fn oc1584_fake_next_channel(c: &mut Cpu) {
        if c.bus.read32(OC1584_GATE) != OC1584_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1584_fake_next_channel),
                ee::s13::f_003baa00__generated,
            );
        }
        let out = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003b_aa00, ra);
        if c.pc == ra {
            let n = c.bus.read32(OC1584_NEXTS);
            c.g[V0][0] = 0;
            if n > 0 {
                c.bus.write32(OC1584_NEXTS, n - 1);
                c.bus.write32(out, n);
                c.g[V0][0] = 1;
            }
        }
    }

    /// Hooks every gated fake into its override slot (once per process).
    fn oc1584_install() {
        let hooks: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 20] = [
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B64D8),
                oc1584_fake_bank_validate,
            ),
            (addr_of_mut!(ee::s13::OVERRIDE_003B58A0), oc1584_fake_lock),
            (addr_of_mut!(ee::s13::OVERRIDE_003B58D8), oc1584_fake_unlock),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B8AB8),
                oc1584_fake_handle_done,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6540),
                oc1584_fake_bank_patch,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003BB820),
                oc1584_fake_parse_header,
            ),
            (addr_of_mut!(ee::s13::OVERRIDE_003B5E98), oc1584_fake_mem_free),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B6A38),
                oc1584_fake_get_stream,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6CE8),
                oc1584_fake_stop_slot,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B88C8),
                oc1584_fake_set_aux_send,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003BA6E8),
                oc1584_fake_handle_to_channel,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003BA740),
                oc1584_fake_calc_send,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C27E0),
                oc1584_fake_apply_send,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C4318),
                oc1584_fake_channel_position,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B98C0),
                oc1584_fake_stop_all,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003B5948),
                oc1584_fake_remove_tick,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C6288),
                oc1584_fake_list_push,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003C62E8),
                oc1584_fake_list_pop,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003BAA00),
                oc1584_fake_next_channel,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E1110),
                oc1584_fake_stream_callback,
            ),
        ];
        for (slot, fake) in hooks {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// A fake-callee machine with this module's gate on; `calls` bounds the
    /// call index at which a fake may unwind. The countdowns start at 0.
    fn oc1584_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc1584_install();
        machine(c, seed, salt, calls);
        c.bus.write32(OC1584_GATE, OC1584_MAGIC);
        c.bus.write32(OC1584_POPS, 0);
        c.bus.write32(OC1584_NEXTS, 0);
    }

    /// Forces the result of the gated fake `k` (all 32 bits).
    fn oc1584_force(c: &mut Cpu, k: u32, value: u32) {
        c.bus.write32(OC1584_MBOX + 8 * k, OC1584_MAGIC);
        c.bus.write32(OC1584_MBOX + 8 * k + 4, value);
    }

    /// `SNDbankinuse`: both bank-table shapes, the validate failure, the
    /// handle done and not-done cases, and the no-entry case.
    #[test]
    fn snd_bank_is_in_use_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x65d0, 4);
            oc1584_force(c, 0, if seed % 7 == 0 { 1 } else { 0 });
            oc1584_force(c, 3, if seed % 3 == 0 { 0 } else { 1 });
            c.g[A0][0] = (seed % 5) as u64;
            c.bus.write16(SNDG + 0x17e, (seed % 4) as u16);
            c.bus.write32(SNDG + 0x1f0, OC1584_RECORDS);
            for i in 0..4u32 {
                let rec = OC1584_RECORDS + 0x8c * i;
                c.bus.write32(rec, OC1584_STREAM + 0x100 * i);
                c.bus.write16(rec + 0x12, ((i + seed) % 5) as u16);
            }
        };
        covered(
            ee::s13::f_003b65d0__generated,
            snd_bank_is_in_use,
            200,
            &setup,
        );
    }

    /// `SND_GetSoundInfo`: the validate failure, the missing patch, the lock
    /// with 0..2 track buffers to free, and the header parse.
    #[test]
    fn snd_get_sound_info_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x6670, 6);
            oc1584_force(c, 0, if seed % 6 == 0 { 0xffff_ffff } else { 0 });
            oc1584_force(c, 4, if seed % 4 == 0 { 0 } else { DATA + 0x5000 });
            let bank = seed % 3;
            c.g[A0][0] = bank as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.g[A2][0] = OC1584_STREAM as u64;
            c.g[A3][0] = OC1584_RECORDS as u64;
            c.g[T0][0] = (seed % 9) as u64;
            c.bus.write8(OC1584_STREAM + 2, (seed % 3) as u8);
            c.bus.write32(SNDG + 0x1f4, OC1584_BANKS);
            for b in 0..3u32 {
                c.bus.write32(OC1584_BANKS + 16 * b, OC1584_SONG);
            }
            for i in 0..4u32 {
                c.bus.write32(OC1584_RECORDS + 0x18 + 4 * i, OC1584_SONG + 0x100 * i);
            }
        };
        covered(
            ee::s13::f_003b6670__generated,
            snd_get_sound_info,
            200,
            &setup,
        );
    }

    /// `SNDSTRM_Stop`: no stream, a status with and without the slot stop,
    /// the close callback with and without its flag, the buffer frees and
    /// the list drain.
    #[test]
    fn snd_strm_stop_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x78f8, 8);
            oc1584_force(c, 7, if seed % 5 == 0 { 0 } else { OC1584_STREAM });
            let s = OC1584_STREAM;
            c.bus.write32(s + 4, if seed % 2 == 0 { 0xffff_ffff } else { 3 });
            c.bus.write32(s + 8, OC1584_SONG);
            c.bus.write32(s, OC1584_SONG + 0x200);
            c.bus.write8(s + 0x11, (seed % 3 == 0) as u8);
            c.bus.write8(s + 0x1a, (seed % 4) as u8);
            for i in 0..4u32 {
                c.bus.write32(s + 0x9c + 4 * i, OC1584_SONG + 0x40 * i);
            }
            c.bus.write32(OC1584_POPS, seed % 4);
        };
        covered(ee::s13::f_003b78f8__generated, snd_strm_stop, 200, &setup);
    }

    /// `SNDSTRM_SetAuxSend`: a missing stream (-8) and the send on the
    /// stream's channel.
    #[test]
    fn snd_strm_set_aux_send_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x8050, 3);
            oc1584_force(c, 7, if seed % 4 == 0 { 0 } else { OC1584_STREAM });
            c.g[A0][0] = (seed % 3) as u64;
            c.g[A1][0] = (seed * 7 % 256) as u64;
            c.g[A2][0] = (seed % 256) as u64;
            c.bus.write32(OC1584_STREAM + 4, OC1584_SONG);
        };
        covered(
            ee::s13::f_003b8050__generated,
            snd_strm_set_aux_send,
            200,
            &setup,
        );
    }

    /// `SND_SetAuxSend`: an invalid handle, and the channel loop with 0..3
    /// channels (the resumable `match pc` body, entered at its own address).
    #[test]
    fn snd_set_aux_send_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x88c8, 6);
            c.pc = 0x003b_88c8;
            oc1584_force(c, 10, if seed % 5 == 0 { 0xffff_ffff } else { 7 });
            c.g[A0][0] = (seed % 4) as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (seed % 128) as u64;
            c.bus.write32(SNDG + 0x1f0, OC1584_RECORDS);
            c.bus.write32(OC1584_NEXTS, seed % 4);
        };
        covered(
            ee::s13::f_003b88c8__generated,
            snd_set_aux_send,
            200,
            &setup,
        );
    }

    /// `SND_GetTimeRemaining`: an invalid handle, and the channel loop with
    /// positions above and below zero and the divisions by the fields.
    #[test]
    fn snd_get_time_remaining_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x8ad8, 6);
            c.pc = 0x003b_8ad8;
            oc1584_force(c, 10, if seed % 5 == 0 { 0xffff_ffff } else { 2 });
            oc1584_force(c, 13, if seed % 2 == 0 { 0xffff_fff0 } else { 0x40 });
            c.bus.write32(OC1584_NEXTS, seed % 4);
            c.bus.write32(SNDG + 0x1f0, OC1584_RECORDS);
            for i in 0..4u32 {
                let rec = OC1584_RECORDS + 0x8c * i;
                let live = (i + seed) % 3 != 0;
                c.bus.write32(rec + 0x18, if live { 0x100 } else { 0xffff_fff0 });
                c.bus.write32(rec + 0x14, 0x1000 * (i + 1));
                c.bus.write16(rec + 0x20, (seed + i) as u16);
                let range: u16 = if (seed + i) % 4 == 0 { 0 } else { (0x20 + i * 7) as u16 };
                c.bus.write16(rec + 0x8a, range);
                c.bus.write16(rec + 0x6a, (seed * 3 + i) as u16);
            }
        };
        covered(
            ee::s13::f_003b8ad8__generated,
            snd_get_time_remaining,
            200,
            &setup,
        );
    }

    /// `SNDMIDI_Shutdown`: stops the tracks and removes the tick callback
    /// under the lock.
    #[test]
    fn snd_midi_shutdown_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            oc1584_machine(c, seed, 0x9880, 4);
        };
        covered(
            ee::s13::f_003b9880__generated,
            snd_midi_shutdown,
            120,
            &setup,
        );
    }

    /// `SNDMIDI_Play`: no callees. Both header shapes (the 8-byte magic and
    /// the 4-byte tag, or none), a free track (filled and returned at once),
    /// all tracks active (the walk stops at 16), and the counter.
    #[test]
    fn snd_midi_play_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 0x9a00, 0);
            for i in 0..40u32 {
                let free = (i + seed) % 3 == 0 && seed % 5 != 0;
                c.bus.write32(
                    MIDI_TRACKS + 0x70 * i + 0x10,
                    if free { 0xffff_ffff } else { 0 },
                );
            }
            c.bus.write32(MIDI_TRACKS + 0x7c0, if seed % 4 == 0 { 0xffff_ffe0 } else { seed * 16 });
            let song = OC1584_SONG;
            c.bus.write8(song, seed as u8);
            c.bus.write16(song + 0x8, (seed * 3) as u16);
            c.bus.write16(song + 0xc, (seed % 16) as u16);
            c.bus.write16(song + 0x10, (seed * 13) as u16);
            match seed % 4 {
                0 => c.bus.write16(OC1584_TRACK, 0x444d),
                1 => c.bus.write32(OC1584_TRACK, 0x7844_494d),
                _ => c.bus.write32(OC1584_TRACK, seed),
            }
            c.g[A0][0] = song as u64;
            c.g[A1][0] = OC1584_TRACK as u64;
            c.g[A2][0] = (seed * 3) as u64;
            c.g[A3][0] = (seed % 128) as u64;
        };
        check(
            ee::s13::f_003b9a00__generated,
            snd_midi_play,
            200,
            &setup,
        );
    }
}
