//! Staging module oc1097 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1097`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::call::{Callee as Oc1097Callee};
use crate::events::{deliver_event_here as oc1097_deliver, event_due as oc1097_due};

/// `SND_DspBufferSource_Init` (the generated function, through its slot).
const DSP_BUFFER_SOURCE_INIT: Callee = Callee {
    addr: 0x003c_f798,
    f: ee::s14::SND_DspBufferSource_Init,
};
const SND_EE_FX_DISABLE: Callee = Callee {
    addr: 0x003c_c848,
    f: ee::s14::SND_EeFxDisable,
};
const SND_EE_FX_BUILD: Callee = Callee {
    addr: 0x003c_c070,
    f: ee::s14::SND_EeFxBuild,
};

/// `void SND_EeFxProcess(int n)` (0x3cbc58): the sound mixer's effects bus
/// pass for `n` samples. Clears the 6-entry `done` flags; if the wet bus
/// pointer (sndmix+0x14) is set it copies the wet buffer (`n*4` bytes) into
/// the work buffer at 0x515d80 and clears it. Otherwise, unless the work
/// buffer's use counter (+0x1800) reaches 3000, it initialises the DSP
/// buffer source and, per output channel not yet done, runs the channel
/// object's process function and mixes `wet` into the channel buffers named
/// by the route table, marking them done; finally it clears the work buffer.
/// Written register for register (state in `c.g`, as the generated body).
pub fn snd_ee_fx_process(c: &mut Cpu) {
    guest(c, |c| {
'b_3cbe9c: {
'b_3cbcd8: {
c.cycles += 21;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
c.g[A1][0] = 0u64;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[A2][0] = 0x18;
c.g[S5][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
c.g[RA][0] = sx(0x003cbc98);
c.g[A0][0] = c.g[SP][0];
call_ptr(c, 0x00416210, 0x003cbc98)?;
c.g[V1][0] = 0x510000;
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5b40));
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
let t = c.g[A0][0] == 0u64;
c.g[S0][0] = 0x510000;
if t {
break 'b_3cbcd8;
} else {
c.cycles += 11;
c.g[S1][0] = sx((c.g[S5][0] as u32) << 2);
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5d80));
c.g[A2][0] = c.g[S1][0];
c.g[RA][0] = sx(0x003cbcc0);
c.g[A1][0] = c.g[S0][0];
call_ptr(c, 0x003c51e8, 0x003cbcc0)?;
c.g[A0][0] = c.g[S0][0];
c.g[A2][0] = c.g[S1][0];
c.g[RA][0] = sx(0x003cbcd0);
c.g[A1][0] = 0u64;
call_ptr(c, 0x00416210, 0x003cbcd0)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_3cbe9c;
}
}
'b_3cbe98: {
c.cycles += 10;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = 0x510000;
c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d80));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1800));
c.g[S6][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb4));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[V1][0] as i64) < 3000i64) as u64;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1800), c.g[V1][0]);
if t {
break 'b_3cbe98;
} else {
'b_3cbe84: {
c.cycles += 13;
c.g[V0][0] = 0x510000;
c.g[V1][0] = 0x510000;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5d58));
c.g[A3][0] = sx((c.g[S5][0] as u32) << 2);
c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5d40));
c.g[A1][0] = 0x4;
c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[A3][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[RA][0] = sx(0x003cbd2c);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, 0x003c72f8, 0x003cbd2c)?;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x510000;
if t {
break 'b_3cbe84;
} else {
c.cycles += 12;
c.g[A0][0] = 0x510000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7600));
c.g[A1][0] = c.g[S0][0];
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5d44));
c.g[S2][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[RA][0] = sx(0x003cbd58);
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
DSP_BUFFER_SOURCE_INIT.call(c, 0x003cbd58)?;
c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x5));
let t = c.g[V0][0] == 0u64;
c.g[S7][0] = c.g[S1][0];
if t {
break 'b_3cbe84;
} else {
c.cycles += 1;
c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
'l_3cbd68: loop {
'b_3cbe70: {
c.cycles += 4;
c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[V0][0] = c.lw((c.g[S4][0] as u32));
let t = c.g[V0][0] != 0u64;
c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
if t {
break 'b_3cbe70;
} else {
c.cycles += 6;
c.g[V1][0] = 0x510000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5d48));
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
let t = c.g[A0][0] == 0u64;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_3cbe70;
} else {
c.cycles += 32;
c.g[A3][0] = c.g[S6][0];
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[A1][0] = c.g[S5][0];
c.g[T0][0] = c.g[FP][0];
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x003cbda8);
c.g[S3][0] = sx((c.g[S7][0] as u32).wrapping_add(0xd0));
call_ptr(c, t, 0x003cbda8)?;
c.g[S1][0] = 0u64;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x1e0));
c.g[A2][0] = c.lw((c.g[V0][0] as u32));
c.g[A0][0] = c.g[S5][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 12);
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003cbdcc);
c.g[A1][0] = c.g[S6][0];
call_ptr(c, t, 0x003cbdcc)?;
c.g[V0][0] = 0x510000;
c.g[A3][0] = 0x450000;
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5d58));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e9c));
c.lo[0] = c.g[A3][0];
c.g[V0][0] = 0xd8;
c.g[A2][0] = c.lbu((c.g[S7][0] as u32).wrapping_add(0x5));
c.madd(0, 5, 5, 2);
c.g[V1][0] = 0x24;
c.g[A0][0] = 0x6;
c.mult(0, 3, 6, 3);
c.mult(0, 4, 18, 4);
c.g[V0][0] = 0x1;
c.sw((c.g[S4][0] as u32), c.g[V0][0]);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
let t = c.g[A2][0] == 0u64;
c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
if t {
break 'b_3cbe70;
} else {
c.cycles += 4;
c.g[S4][0] = c.g[S3][0];
c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0xd0));
c.g[S3][0] = 0x1;
'l_3cbe20: loop {
'b_3cbe5c: {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
if t {
break 'b_3cbe5c;
} else {
c.cycles += 12;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1e0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 12);
c.g[A2][0] = c.lw((c.g[V0][0] as u32));
c.g[A0][0] = c.g[S5][0];
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003cbe4c);
c.g[A1][0] = c.g[S6][0];
call_ptr(c, t, 0x003cbe4c)?;
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[V1][0] as u32), c.g[S3][0]);
break 'b_3cbe5c;
}
}
c.cycles += 5;
c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x5));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x003cbe20); } continue 'l_3cbe20;
} else {
break 'b_3cbe70;
}
}
}
}
}
}
c.cycles += 5;
c.g[V0][0] = c.lbu((c.g[S7][0] as u32).wrapping_add(0x5));
c.g[S2][0] = c.g[FP][0];
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
if t {
if event_due(c) { deliver_event_here(c, 0x003cbd68); } continue 'l_3cbd68;
} else {
break 'b_3cbe84;
}
}
}
}
}
c.cycles += 5;
c.g[V0][0] = 0x510000;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d80));
c.g[RA][0] = sx(0x003cbe98);
c.g[A1][0] = 0u64;
call_ptr(c, 0x00416210, 0x003cbe98)?;
break 'b_3cbe98;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_3cbe9c;
}
c.cycles += 11;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.pc = t;
Ok(())
    });
}

