//! Staging module oc407 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc407`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0011_c298, rider_load_step, "Rider_LoadStep")];

/// `cRider::loadRider` state machine: entry, end, and the loop heads the
/// function can be resumed at after an event unwind.
pub(crate) const RIDER_LOAD_STEP_ENTRY: u32 = 0x0011_c298;
pub(crate) const RIDER_LOAD_STEP_END: u32 = 0x0011_d390;

/// Rider fields (relative to the rider `this`, kept in `fp`).
pub(crate) const RLS_SKEL: u32 = 0x780;
pub(crate) const RLS_ANIM_PLAYER: u32 = 0x784;
pub(crate) const RLS_AIR_PREDICTOR: u32 = 0x788;
/// Load step: 0 start, 1 bolt lists queued, 2 polling the file loads, 3 (then
/// 5 and 4 in the same call) release the files, 6 build, 7 done.
pub(crate) const RLS_LOAD_STATE: u32 = 0x880;
/// The load list (0x2948 bytes: counts at +0x2940/+0x2944, 0x108-byte entries from +0x0).
pub(crate) const RLS_LOAD_LIST: u32 = 0x894;

const AIR_PREDICTOR_CTOR: Callee = Callee { addr: 0x0011_3138, f: ee::s00::f_00113138 };
const GET_EQUIPPED_BOLTS: Callee = Callee { addr: 0x0011_bbe8, f: ee::s00::f_0011bbe8 };
const ADD_BOLT_NAMES: Callee = Callee { addr: 0x0011_bd60, f: ee::s00::f_0011bd60 };
const ADD_BOLT_FILES: Callee = Callee { addr: 0x0011_be88, f: ee::s00::f_0011be88 };
const LOAD_BOLT_MODELS: Callee = Callee { addr: 0x0011_c138, f: ee::s00::f_0011c138 };
const RIDER_ANIM_SET_RIDER: Callee = Callee { addr: 0x0010_4d38, f: ee::s00::f_00104d38 };
const UPDATE_BOUNDS: Callee = Callee { addr: 0x0011_e150, f: ee::s00::f_0011e150 };
const FIND_NEARBY_LIGHTS: Callee = Callee { addr: 0x0012_2088, f: ee::s00::f_00122088 };
const RIDER_RESET: Callee = Callee { addr: 0x0011_d390, f: ee::s00::f_0011d390 };

/// `void Rider_LoadStep(cRider* this)` (0x11c298, `cRider::loadRider`): one
/// step of the rider's asynchronous load, switched on the load state at
/// +0x880 (jump table at 0x457c30): 0 allocates the air predictor and the
/// animation set, 1 queues the equipped bolts' names and files, 2 polls the
/// file loads and loads the bolt models of finished ones (at most 10 per
/// call), 3 registers the names with the model, then 5 compiles it and 4
/// frees the list, 6 builds the rider (anim player, joint indices, bone
/// matrices, lights, the 0x2b0-byte physics object), sets state 7 and ends in
/// `Rider_Reset`. State 7 and above return at once. Kept in the generated block structure:
/// it is resumable at its five loop heads.
pub fn rider_load_step(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0011c298 => {
                c.cycles += 20;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x220));
                c.sq(30, r32(c, SP).wrapping_add(0x170));
                c.sq(16, r32(c, SP).wrapping_add(0x1f0));
                c.g[FP][0] = c.g[A0][0];
                c.sq(17, r32(c, SP).wrapping_add(0x1e0));
                c.sq(18, r32(c, SP).wrapping_add(0x1d0));
                c.sq(19, r32(c, SP).wrapping_add(0x1c0));
                c.sq(20, r32(c, SP).wrapping_add(0x1b0));
                c.sq(21, r32(c, SP).wrapping_add(0x1a0));
                c.sq(22, r32(c, SP).wrapping_add(0x190));
                c.sq(23, r32(c, SP).wrapping_add(0x180));
                c.sd(r32(c, SP).wrapping_add(0x160), c.g[RA][0]);
                c.swc1(23, r32(c, SP).wrapping_add(0x218));
                c.swc1(22, r32(c, SP).wrapping_add(0x210));
                c.swc1(21, r32(c, SP).wrapping_add(0x208));
                c.swc1(20, r32(c, SP).wrapping_add(0x200));
                c.g[V1][0] = c.lw(r32(c, FP).wrapping_add(RLS_LOAD_STATE));
                c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x450000;
                if t { pc = 0x0011d350; continue; }
                pc = 0x0011c2e8; continue;
                }
                0x0011c2e8 => {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7c30));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
                }
                0x0011c300 => {
                c.cycles += 23;
                c.g[A1][0] = 0x450000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x7980));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = 0xb0;
                call_ptr(c, 0x00317d70, 0x0011c318)?;
                c.g[A0][0] = c.g[V0][0];
                AIR_PREDICTOR_CTOR.call(c, 0x0011c320)?;
                c.g[A1][0] = 0x450000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x7990));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = 0x160;
                c.sw(r32(c, FP).wrapping_add(RLS_AIR_PREDICTOR), c.g[V0][0]);
                call_ptr(c, 0x00317d70, 0x0011c33c)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0030d4b8, 0x0011c344)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(0x88c));
                c.g[A1][0] = c.g[V0][0];
                c.sw(r32(c, FP).wrapping_add(RLS_SKEL), c.g[V0][0]);
                call_ptr(c, 0x002f19f0, 0x0011c354)?;
                c.g[V0][0] = 0x1;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                pc = 0x0011c35c; continue;
                }
                0x0011c35c => {
                c.cycles += 27;
                c.g[A1][0] = c.g[SP][0];
                c.g[A0][0] = c.g[FP][0];
                GET_EQUIPPED_BOLTS.call(c, 0x0011c368)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = sx(r32(c, S0).wrapping_add(0x1));
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x0030d840, 0x0011c37c)?;
                c.g[A1][0] = 0x450000;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x79a0));
                c.g[A0][0] = 0x2948;
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x00317d70, 0x0011c394)?;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_LIST), c.g[V0][0]);
                c.g[A1][0] = c.g[SP][0];
                c.sw(r32(c, V0).wrapping_add(0x2940), 0u64);
                c.g[A0][0] = c.g[FP][0];
                c.sw(r32(c, V0).wrapping_add(0x2944), 0u64);
                c.g[A2][0] = c.g[S0][0];
                ADD_BOLT_NAMES.call(c, 0x0011c3b0)?;
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.g[FP][0];
                c.g[A1][0] = c.g[SP][0];
                ADD_BOLT_FILES.call(c, 0x0011c3c0)?;
                c.g[V0][0] = 0x2;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                pc = 0x0011c3c8; continue;
                }
                0x0011c3c8 => {
                c.cycles += 7;
                c.g[S4][0] = c.lw(r32(c, FP).wrapping_add(RLS_LOAD_LIST));
                c.g[S3][0] = 0u64;
                c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(0x2940));
                c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(0x2944));
                c.g[S5][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                let t = (c.g[S5][0] as i64) <= 0;
                c.g[S2][0] = 0u64;
                if t { pc = 0x0011c4bc; continue; }
                pc = 0x0011c3e4; continue;
                }
                0x0011c3e4 => {
                c.cycles += 3;
                c.g[S1][0] = c.g[S4][0];
                c.g[S7][0] = 0x1;
                c.g[S6][0] = 0xffffffffffffffff;
                pc = 0x0011c3f0; continue;
                }
                0x0011c3f0 => {
                c.cycles += 3;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x100));
                let t = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = 0u64;
                if t { pc = 0x0011c408; continue; }
                pc = 0x0011c3fc; continue;
                }
                0x0011c3fc => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x104));
                c.g[V1][0] = c.g[S7][0];
                if c.g[V0][0] == 0 { c.g[V1][0] = 0u64; }
                pc = 0x0011c408; continue;
                }
                0x0011c408 => {
                c.cycles += 2;
                
                if c.g[V1][0] != 0u64 { c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x1)); pc = 0x0011c4b0; continue; }
                pc = 0x0011c410; continue;
                }
                0x0011c410 => {
                c.cycles += 2;
                
                if (c.g[A0][0] as i64) < 0 { c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x1)); pc = 0x0011c4b0; continue; }
                pc = 0x0011c418; continue;
                }
                0x0011c418 => {
                c.cycles += 4;
                call_ptr(c, 0x003df980, 0x0011c420)?;
                
                if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x100)); pc = 0x0011c430; continue; }
                pc = 0x0011c428; continue;
                }
                0x0011c428 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t { pc = 0x0011c444; continue; }
                pc = 0x0011c430; continue;
                }
                0x0011c430 => {
                c.cycles += 5;
                c.g[A1][0] = sx(r32(c, S1).wrapping_add(0x104));
                c.g[A2][0] = sx(r32(c, SP).wrapping_add(0x140));
                call_ptr(c, 0x003df9d8, 0x0011c43c)?;
                c.sw(r32(c, S1).wrapping_add(0x100), c.g[S6][0]);
                c.g[V0][0] = 0x1;
                pc = 0x0011c444; continue;
                }
                0x0011c444 => {
                c.cycles += 2;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lw(r32(c, S4).wrapping_add(0x2940)); pc = 0x0011c454; continue; }
                pc = 0x0011c44c; continue;
                }
                0x0011c44c => {
                c.cycles += 2;
                let t = true;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x1));
                if t { pc = 0x0011c4ac; continue; }
                pc = 0x0011c454; continue;
                }
                0x0011c454 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                
                if t { pc = 0x0011c474; continue; }
                pc = 0x0011c460; continue;
                }
                0x0011c460 => {
                c.cycles += 5;
                c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x104));
                c.g[A0][0] = c.g[FP][0];
                LOAD_BOLT_MODELS.call(c, 0x0011c46c)?;
                let t = true;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x1));
                if t { pc = 0x0011c4b0; continue; }
                pc = 0x0011c474; continue;
                }
                0x0011c474 => {
                c.cycles += 4;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x104));
                call_ptr(c, 0x003b4818, 0x0011c47c)?;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A2][0] = 0x100;
                if t { pc = 0x0011c4ac; continue; }
                pc = 0x0011c484; continue;
                }
                0x0011c484 => {
                c.cycles += 10;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00252fa0, 0x0011c490)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x104));
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003b47f8, 0x0011c4a0)?;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x104));
                call_ptr(c, 0x002523a8, 0x0011c4a8)?;
                c.sw(r32(c, S1).wrapping_add(0x104), c.g[S0][0]);
                pc = 0x0011c4ac; continue;
                }
                0x0011c4ac => {
                c.cycles += 1;
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x1));
                pc = 0x0011c4b0; continue;
                }
                0x0011c4b0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x108));
                if t { if back_edge_unwinds(c, 0x0011c3f0) { return Err(Unwound); } pc = 0x0011c3f0; continue; }
                pc = 0x0011c4bc; continue;
                }
                0x0011c4bc => {
                c.cycles += 2;
                let t = (c.g[S5][0] as i64) <= 0;
                c.g[S1][0] = 0u64;
                if t { pc = 0x0011c52c; continue; }
                pc = 0x0011c4c4; continue;
                }
                0x0011c4c4 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[S4][0];
                if t { pc = 0x0011c52c; continue; }
                pc = 0x0011c4d0; continue;
                }
                0x0011c4d0 => {
                c.cycles += 2;
                c.g[S3][0] = 0x1;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x100));
                pc = 0x0011c4d8; continue;
                }
                0x0011c4d8 => {
                c.cycles += 2;
                let t = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = 0u64;
                if t { pc = 0x0011c4ec; continue; }
                pc = 0x0011c4e0; continue;
                }
                0x0011c4e0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x104));
                c.g[V1][0] = c.g[S3][0];
                if c.g[V0][0] == 0 { c.g[V1][0] = 0u64; }
                pc = 0x0011c4ec; continue;
                }
                0x0011c4ec => {
                c.cycles += 2;
                
                if c.g[V1][0] != 0u64 { c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1)); pc = 0x0011c514; continue; }
                pc = 0x0011c4f4; continue;
                }
                0x0011c4f4 => {
                c.cycles += 2;
                
                if (c.g[A0][0] as i64) >= 0 { c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1)); pc = 0x0011c514; continue; }
                pc = 0x0011c4fc; continue;
                }
                0x0011c4fc => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x100;
                call_ptr(c, 0x003df690, 0x0011c508)?;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x1));
                c.sw(r32(c, S0).wrapping_add(0x100), c.g[V0][0]);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x1));
                pc = 0x0011c514; continue;
                }
                0x0011c514 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x108));
                if t { pc = 0x0011c52c; continue; }
                pc = 0x0011c520; continue;
                }
                0x0011c520 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x100)); if back_edge_unwinds(c, 0x0011c4d8) { return Err(Unwound); } pc = 0x0011c4d8; continue; }
                pc = 0x0011c52c; continue;
                }
                0x0011c52c => {
                c.cycles += 2;
                let t = c.g[S2][0] != 0u64;
                c.lq(16, r32(c, SP).wrapping_add(0x1f0));
                if t { pc = 0x0011d354; continue; }
                pc = 0x0011c534; continue;
                }
                0x0011c534 => {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let t = true;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                if t { pc = 0x0011d354; continue; }
                pc = 0x0011c540; continue;
                }
                0x0011c540 => {
                c.cycles += 8;
                c.g[A1][0] = c.lw(r32(c, FP).wrapping_add(RLS_LOAD_LIST));
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x2940));
                c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(0x2944));
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[S2][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                if t { pc = 0x0011c590; continue; }
                pc = 0x0011c560; continue;
                }
                0x0011c560 => {
                c.cycles += 6;
                c.g[V0][0] = 0x108;
                c.mult(0, 2, 16, 2);
                c.g[S0][0] = sx(r32(c, A0).wrapping_sub(r32(c, S0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x104));
                c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                c.g[A1][0] = c.lw(r32(c, S1));
                pc = 0x0011c578; continue;
                }
                0x0011c578 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x108));
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x1));
                call_ptr(c, 0x0030db70, 0x0011c588)?;
                
                if c.g[S0][0] != 0u64 { c.g[A1][0] = c.lw(r32(c, S1)); if back_edge_unwinds(c, 0x0011c578) { return Err(Unwound); } pc = 0x0011c578; continue; }
                pc = 0x0011c590; continue;
                }
                0x0011c590 => {
                c.cycles += 2;
                c.g[V0][0] = 0x5;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                pc = 0x0011c598; continue;
                }
                0x0011c598 => {
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(0xaa8));
                call_ptr(c, 0x00375990, 0x0011c5a0)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0030dbd0, 0x0011c5ac)?;
                c.g[V0][0] = 0x4;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                pc = 0x0011c5b4; continue;
                }
                0x0011c5b4 => {
                c.cycles += 3;
                c.g[S2][0] = c.lw(r32(c, FP).wrapping_add(RLS_LOAD_LIST));
                let t = c.g[S2][0] == 0u64;
                c.g[V0][0] = 0x6;
                if t { pc = 0x0011c610; continue; }
                pc = 0x0011c5c0; continue;
                }
                0x0011c5c0 => {
                c.cycles += 7;
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0x2940));
                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(0x2944));
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x108;
                if t { pc = 0x0011c604; continue; }
                pc = 0x0011c5dc; continue;
                }
                0x0011c5dc => {
                c.cycles += 5;
                c.mult(0, 2, 16, 2);
                c.g[S0][0] = sx(r32(c, A0).wrapping_sub(r32(c, S0)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x104));
                c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
                c.g[A0][0] = c.lw(r32(c, S1));
                pc = 0x0011c5f0; continue;
                }
                0x0011c5f0 => {
                c.cycles += 5;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x108));
                call_ptr(c, 0x002523a8, 0x0011c5fc)?;
                
                if c.g[S0][0] != 0u64 { c.g[A0][0] = c.lw(r32(c, S1)); if back_edge_unwinds(c, 0x0011c5f0) { return Err(Unwound); } pc = 0x0011c5f0; continue; }
                pc = 0x0011c604; continue;
                }
                0x0011c604 => {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00317e50, 0x0011c60c)?;
                c.g[V0][0] = 0x6;
                pc = 0x0011c610; continue;
                }
                0x0011c610 => {
                c.cycles += 3;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_LIST), 0u64);
                let t = true;
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                if t { pc = 0x0011d350; continue; }
                pc = 0x0011c61c; continue;
                }
                0x0011c61c => {
                c.cycles += 169;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x5;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[S1][0] = 0x4a0000;
                call_ptr(c, 0x0030ea80, 0x0011c630)?;
                c.g[S2][0] = 0x450000;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x6;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[S5][0] = 0x450000;
                call_ptr(c, 0x0030ea80, 0x0011c648)?;
                c.g[S4][0] = 0x450000;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x8;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[S3][0] = 0x450000;
                call_ptr(c, 0x0030ea80, 0x0011c660)?;
                c.g[S6][0] = 0x450000;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x9;
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[S7][0] = 0x450000;
                call_ptr(c, 0x0030ea80, 0x0011c678)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0xffffffffffffffff;
                c.g[A1][0] = 0xb;
                call_ptr(c, 0x0030ea80, 0x0011c688)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0030eca0, 0x0011c694)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0030eca0, 0x0011c6a0)?;
                c.g[A1][0] = 0x450000;
                c.g[A3][0] = 0u64;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x79b0));
                c.g[A0][0] = 0x70;
                call_ptr(c, 0x00317d70, 0x0011c6b8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003118d0, 0x0011c6c4)?;
                c.g[V1][0] = 0x460000;
                c.sw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER), c.g[S0][0]);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x7ba8));
                c.g[A0][0] = c.g[S0][0];
                c.sw(r32(c, S0).wrapping_add(0x58), c.g[V1][0]);
                c.g[A1][0] = c.g[FP][0];
                c.sw(r32(c, S0).wrapping_add(0x60), 0u64);
                c.sw(r32(c, S0).wrapping_add(0x64), 0u64);
                RIDER_ANIM_SET_RIDER.call(c, 0x0011c6e8)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x79c0));
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00310c48, 0x0011c6fc)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x79d0));
                c.g[A1][0] = 0x1;
                c.sw(r32(c, FP).wrapping_add(0x8a0), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c714)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x4a0000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x1080));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8a4), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c72c)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = sx(r32(c, S1).wrapping_add(0x1088));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x89c), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c740)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x79e0));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8a8), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c758)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x79f0));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8ac), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c770)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = sx(r32(c, S2).wrapping_add(0x7a00));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8b0), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c784)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7a10));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8b4), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c79c)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = sx(r32(c, S5).wrapping_add(0x7a20));
                c.sw(r32(c, FP).wrapping_add(0x8b8), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c7b0)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = sx(r32(c, S4).wrapping_add(0x7a30));
                c.sw(r32(c, FP).wrapping_add(0x8bc), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c7c4)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = sx(r32(c, S3).wrapping_add(0x7a40));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8e0), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c7d8)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7a50));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x8e4), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c7f0)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = sx(r32(c, S6).wrapping_add(0x7a60));
                c.sw(r32(c, FP).wrapping_add(0x8e8), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c804)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = sx(r32(c, S7).wrapping_add(0x7a70));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x910), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c818)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7a80));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x914), c.g[V0][0]);
                call_ptr(c, 0x00310c48, 0x0011c830)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7a90));
                c.g[A1][0] = 0u64;
                c.sw(r32(c, FP).wrapping_add(0x918), c.g[V0][0]);
                call_ptr(c, 0x00310ce8, 0x0011c848)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7b38));
                c.g[A1][0] = 0u64;
                c.sd(r32(c, FP).wrapping_add(0x8c0), c.g[V0][0]);
                call_ptr(c, 0x00310ce8, 0x0011c860)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7bc8));
                c.g[A1][0] = 0u64;
                c.sd(r32(c, FP).wrapping_add(0x8c8), c.g[V0][0]);
                call_ptr(c, 0x00310ce8, 0x0011c878)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x4a0000;
                c.g[A1][0] = 0x7;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x1090));
                c.sd(r32(c, FP).wrapping_add(0x8d0), c.g[V0][0]);
                call_ptr(c, 0x00310ce8, 0x0011c890)?;
                c.g[A1][0] = c.ld(r32(c, FP).wrapping_add(0x8c0));
                c.g[V1][0] = c.ld(r32(c, FP).wrapping_add(0x8c8));
                c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(0x86c));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sd(r32(c, FP).wrapping_add(0x8c0), c.g[A1][0]);
                c.sd(r32(c, FP).wrapping_add(0x8c8), c.g[V1][0]);
                call_ptr(c, 0x0014a080, 0x0011c8b0)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x450000;
                if t { pc = 0x0011c928; continue; }
                pc = 0x0011c8c0; continue;
                }
                0x0011c8c0 => {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7c50));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
                }
                0x0011c8d8 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c8e0; continue;
                }
                0x0011c8e0 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x4;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c8e8; continue;
                }
                0x0011c8e8 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x20;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c8f0; continue;
                }
                0x0011c8f0 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x2;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c8f8; continue;
                }
                0x0011c8f8 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x8;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c900; continue;
                }
                0x0011c900 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x100;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c908; continue;
                }
                0x0011c908 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x200;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c910; continue;
                }
                0x0011c910 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x80;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c918; continue;
                }
                0x0011c918 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x10;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c920; continue;
                }
                0x0011c920 => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0x40;
                if t { pc = 0x0011c930; continue; }
                pc = 0x0011c928; continue;
                }
                0x0011c928 => {
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                pc = 0x0011c930; continue;
                }
                0x0011c930 => {
                c.cycles += 46;
                c.sw(r32(c, FP).wrapping_add(0x364), c.g[V0][0]);
                call_ptr(c, 0x0014dc80, 0x0011c938)?;
                c.g[S2][0] = 0x500000;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014dd58, 0x0011c948)?;
                c.g[S5][0] = 0x500000;
                c.g[A1][0] = c.lw(r32(c, FP).wrapping_add(0x86c));
                c.g[A0][0] = c.g[V0][0];
                c.g[S6][0] = sx(r32(c, FP).wrapping_add(0x940));
                c.g[S7][0] = sx(r32(c, FP).wrapping_add(0x970));
                call_ptr(c, 0x0014efa8, 0x0011c960)?;
                c.g[V0][0] = sx(r32(c, FP).wrapping_add(0x9a0));
                c.mov_s(23, 0);
                c.sw(r32(c, SP).wrapping_add(0x154), c.g[V0][0]);
                c.mov_s(13, 23);
                c.mov_s(12, 23);
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                call_ptr(c, 0x0030dba0, 0x0011c97c)?;
                c.g[V1][0] = sx(r32(c, FP).wrapping_add(0x400));
                c.sw(r32(c, SP).wrapping_add(0x158), c.g[V1][0]);
                c.g[T2][0] = c.lw(r32(c, FP).wrapping_add(0x89c));
                c.g[A3][0] = c.lw(r32(c, FP).wrapping_add(0x8b8));
                c.g[T0][0] = c.lw(r32(c, FP).wrapping_add(0x918));
                c.g[T1][0] = c.lw(r32(c, FP).wrapping_add(0x8e8));
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(0xaa8));
                c.g[A1][0] = c.lw(r32(c, FP).wrapping_add(0x8a8));
                c.g[A2][0] = c.lw(r32(c, FP).wrapping_add(0x8b0));
                call_ptr(c, 0x00374ce0, 0x0011c9a4)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                c.g[A1][0] = 0x5;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x003128e8, 0x0011c9bc)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00312598, 0x0011c9c8)?;
                c.g[V0][0] = sx(r32(c, SP).wrapping_add(0xa0));
                c.g[V1][0] = sx(r32(c, SP).wrapping_add(0xd0));
                c.sw(r32(c, SP).wrapping_add(0x15c), c.g[V0][0]);
                c.g[S3][0] = sx(r32(c, SP).wrapping_add(0x144));
                c.sw(r32(c, SP).wrapping_add(0x150), c.g[V1][0]);
                c.g[S4][0] = sx(r32(c, SP).wrapping_add(0x148));
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = 0x1;
                pc = 0x0011c9e8; continue;
                }
                0x0011c9e8 => {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[S0][0] = sx(r32(c, S2).wrapping_sub(0xed0));
                if t { if back_edge_unwinds(c, 0x0011c9e8) { return Err(Unwound); } pc = 0x0011c9e8; continue; }
                pc = 0x0011ca04; continue;
                }
                0x0011ca04 => {
                c.cycles += 351;
                c.g[V0][0] = 0x500000;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xdd0));
                c.g[A1][0] = c.g[SP][0];
                c.lq(6, r32(c, V0).wrapping_add(0x10));
                c.lq(3, r32(c, V0));
                c.g[S1][0] = sx(r32(c, S5).wrapping_sub(0xea0));
                c.sq(6, r32(c, SP).wrapping_add(0x10));
                c.sq(3, r32(c, SP));
                c.sq(3, r32(c, SP).wrapping_add(0x20));
                c.sq(6, r32(c, SP).wrapping_add(0x30));
                c.sw(r32(c, SP).wrapping_add(0x14c), 0u64);
                call_ptr(c, 0x00310200, 0x0011ca38)?;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 20);
                c.g[A3][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = c.lw(r32(c, FP).wrapping_add(0x8e8));
                c.g[V0][0] = c.lw(r32(c, A3).wrapping_add(0x2c));
                c.g[V1][0] = sx(r32(c, V1) << 5);
                c.g[A1][0] = c.lw(r32(c, FP).wrapping_add(0x918));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A2][0] = c.lw(r32(c, FP).wrapping_add(0x8a4));
                c.lq(4, r32(c, V1));
                c.g[A1][0] = sx(r32(c, A1) << 5);
                c.g[A2][0] = sx(r32(c, A2) << 5);
                c.sq(4, r32(c, SP).wrapping_add(0x40));
                c.lq(2, r32(c, V1).wrapping_add(0x10));
                c.sq(2, r32(c, SP).wrapping_add(0x50));
                c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x2c));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                c.lq(2, r32(c, A1));
                c.sq(2, r32(c, SP).wrapping_add(0x60));
                c.lq(3, r32(c, A1).wrapping_add(0x10));
                c.sq(3, r32(c, SP).wrapping_add(0x70));
                c.g[V0][0] = c.lw(r32(c, A3).wrapping_add(0x2c));
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, A2).wrapping_add(0x1c));
                c.swc1(0, r32(c, SP).wrapping_add(0x10c));
                c.lwc1(2, r32(c, A2).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0x100));
                c.neg_s(2, 2);
                c.lwc1(1, r32(c, A2).wrapping_add(0x14));
                c.swc1(1, r32(c, SP).wrapping_add(0x104));
                c.neg_s(1, 1);
                c.lwc1(0, r32(c, A2).wrapping_add(0x18));
                c.swc1(2, r32(c, SP).wrapping_add(0x100));
                c.neg_s(0, 0);
                c.swc1(1, r32(c, SP).wrapping_add(0x104));
                c.swc1(0, r32(c, SP).wrapping_add(0x108));
                c.lq(2, r32(c, SP).wrapping_add(0x100));
                c.sq(2, r32(c, SP).wrapping_add(0xe0));
                c.sq(2, r32(c, SP).wrapping_add(0xf0));
                c.lqc2(3, r32(c, A2));
                c.lqc2(4, r32(c, S2).wrapping_sub(0xed0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, SP).wrapping_add(0x130));
                c.lq(2, r32(c, SP).wrapping_add(0x130));
                c.sq(2, r32(c, SP).wrapping_add(0x120));
                c.lqc2(4, r32(c, SP).wrapping_add(0xe0));
                c.lqc2(5, r32(c, SP).wrapping_add(0x120));
                c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().opmula(4, 6);
                c.vu0().opmsub(7, 6, 4);
                c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                c.sqc2(8, r32(c, SP).wrapping_add(0x130));
                c.lq(2, r32(c, SP).wrapping_add(0x130));
                c.sq(2, r32(c, SP).wrapping_add(0x110));
                c.mfc1(3, 20);
                c.lqc2(4, r32(c, SP).wrapping_add(0x110));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, r32(c, SP).wrapping_add(0x130));
                c.lq(2, r32(c, SP).wrapping_add(0x130));
                c.sq(2, r32(c, SP).wrapping_add(0x100));
                c.lqc2(3, r32(c, SP).wrapping_add(0x100));
                c.lqc2(4, r32(c, S2).wrapping_sub(0xed0));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, SP).wrapping_add(0x130));
                c.lq(2, r32(c, SP).wrapping_add(0x130));
                c.lq(3, r32(c, SP).wrapping_add(0xe0));
                c.sq(2, r32(c, SP).wrapping_add(0xd0));
                c.sq(2, r32(c, SP).wrapping_add(0x80));
                c.sq(3, r32(c, SP).wrapping_add(0x90));
                c.sq(2, r32(c, SP).wrapping_add(0xf0));
                c.lqc2(4, r32(c, SP).wrapping_add(0x90));
                c.lqc2(5, r32(c, SP).wrapping_add(0x50));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, r32(c, SP).wrapping_add(0xf0));
                c.lq(2, r32(c, SP).wrapping_add(0xf0));
                c.sq(2, r32(c, SP).wrapping_add(0xd0));
                c.sq(2, r32(c, SP).wrapping_add(0xe0));
                c.lqc2(3, r32(c, SP).wrapping_add(0x40));
                c.lqc2(4, r32(c, S2).wrapping_sub(0xed0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, SP).wrapping_add(0x110));
                c.lq(2, r32(c, SP).wrapping_add(0x110));
                c.sq(2, r32(c, SP).wrapping_add(0x100));
                c.lqc2(4, r32(c, SP).wrapping_add(0x90));
                c.lqc2(5, r32(c, SP).wrapping_add(0x100));
                c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().opmula(4, 6);
                c.vu0().opmsub(7, 6, 4);
                c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                c.sqc2(8, r32(c, SP).wrapping_add(0x110));
                c.lq(2, r32(c, SP).wrapping_add(0x110));
                c.sq(2, r32(c, SP).wrapping_add(0xf0));
                c.lqc2(3, r32(c, SP).wrapping_add(0xf0));
                c.lqc2(4, r32(c, SP).wrapping_add(0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, SP).wrapping_add(0x110));
                c.lq(2, r32(c, SP).wrapping_add(0x110));
                c.lq(3, r32(c, SP).wrapping_add(0xd0));
                c.sq(2, r32(c, FP).wrapping_add(0x8f0));
                c.sq(3, r32(c, FP).wrapping_add(0x900));
                c.sq(2, r32(c, SP).wrapping_add(0xe0));
                c.sq(2, r32(c, SP).wrapping_add(0xc0));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                c.sq(3, r32(c, SP).wrapping_add(0xb0));
                c.lqc2(4, r32(c, SP).wrapping_add(0x90));
                c.lqc2(5, r32(c, SP).wrapping_add(0x70));
                c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                c.sqc2(8, r32(c, SP).wrapping_add(0xf0));
                c.lq(2, r32(c, SP).wrapping_add(0xf0));
                c.sq(2, r32(c, SP).wrapping_add(0xd0));
                c.sq(2, r32(c, SP).wrapping_add(0xe0));
                c.lqc2(3, r32(c, SP).wrapping_add(0x60));
                c.lqc2(4, r32(c, S2).wrapping_sub(0xed0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, SP).wrapping_add(0x110));
                c.lq(2, r32(c, SP).wrapping_add(0x110));
                c.sq(2, r32(c, SP).wrapping_add(0x100));
                c.lqc2(4, r32(c, SP).wrapping_add(0x90));
                c.lqc2(5, r32(c, SP).wrapping_add(0x100));
                c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().opmula(4, 6);
                c.vu0().opmsub(7, 6, 4);
                c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                c.sqc2(8, r32(c, SP).wrapping_add(0x110));
                c.lq(2, r32(c, SP).wrapping_add(0x110));
                c.sq(2, r32(c, SP).wrapping_add(0xf0));
                c.lqc2(3, r32(c, SP).wrapping_add(0xf0));
                c.lqc2(4, r32(c, SP).wrapping_add(0x80));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, r32(c, SP).wrapping_add(0x110));
                c.lq(2, r32(c, SP).wrapping_add(0x110));
                c.lq(3, r32(c, SP).wrapping_add(0xd0));
                c.sq(2, r32(c, FP).wrapping_add(0x920));
                c.sq(3, r32(c, FP).wrapping_add(0x930));
                c.sq(3, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                c.sq(2, r32(c, SP).wrapping_add(0xe0));
                c.sq(2, r32(c, SP).wrapping_add(0xc0));
                c.g[V0][0] = c.lw(r32(c, FP).wrapping_add(0x8a8));
                c.g[A2][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V0][0] = sx(r32(c, V0) << 5);
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(0x8ac));
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x2c));
                c.g[A0][0] = sx(r32(c, A0) << 5);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.lwc1(0, r32(c, V1).wrapping_add(0x1c));
                c.swc1(0, r32(c, SP).wrapping_add(0xbc));
                c.lwc1(2, r32(c, V1).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(2, 2);
                c.lwc1(1, r32(c, V1).wrapping_add(0x14));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.neg_s(1, 1);
                c.lwc1(0, r32(c, V1).wrapping_add(0x18));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(0, 0);
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.swc1(0, r32(c, SP).wrapping_add(0xb8));
                c.lq(2, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, FP).wrapping_add(0x950));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x2c));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.lwc1(0, r32(c, V1).wrapping_add(0x1c));
                c.swc1(0, r32(c, SP).wrapping_add(0xbc));
                c.lwc1(2, r32(c, V1).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.lwc1(1, r32(c, V1).wrapping_add(0x14));
                c.neg_s(2, 2);
                c.g[A1][0] = c.lw(r32(c, FP).wrapping_add(0x89c));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.neg_s(1, 1);
                c.g[A1][0] = sx(r32(c, A1) << 5);
                c.lwc1(0, r32(c, V1).wrapping_add(0x18));
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                c.neg_s(0, 0);
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.swc1(0, r32(c, SP).wrapping_add(0xb8));
                c.lq(2, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, FP).wrapping_add(0x980));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(0x2c));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
                c.lwc1(0, r32(c, V1).wrapping_add(0x1c));
                c.swc1(0, r32(c, SP).wrapping_add(0xbc));
                c.lwc1(2, r32(c, V1).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(2, 2);
                c.lwc1(1, r32(c, V1).wrapping_add(0x14));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.neg_s(1, 1);
                c.lwc1(0, r32(c, V1).wrapping_add(0x18));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(0, 0);
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.swc1(0, r32(c, SP).wrapping_add(0xb8));
                c.lq(2, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, FP).wrapping_add(0x9b0));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                call_ptr(c, 0x00311a50, 0x0011cde8)?;
                c.lq(2, r32(c, S0));
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                c.g[A1][0] = c.g[S4][0];
                c.lwc1(12, r32(c, GP).wrapping_sub(0x79b4));
                c.sq(2, r32(c, S0).wrapping_add(0x30));
                call_ptr(c, 0x0031be50, 0x0011ce04)?;
                c.lwc1(1, r32(c, S1).wrapping_add(0x8));
                c.g[A0][0] = 0x1;
                c.lwc1(0, r32(c, SP).wrapping_add(0x144));
                c.g[A2][0] = 0u64;
                c.lwc1(3, r32(c, S5).wrapping_sub(0xea0));
                c.mov_s(12, 20);
                c.lwc1(4, r32(c, S1).wrapping_add(0x4));
                c.mul_s(1, 0, 1);
                c.mul_s(3, 0, 3);
                c.lwc1(2, r32(c, SP).wrapping_add(0x148));
                c.mul_s(0, 0, 4);
                c.g[A1][0] = 0x5;
                c.swc1(2, r32(c, SP).wrapping_add(0xac));
                c.swc1(1, r32(c, SP).wrapping_add(0xa8));
                c.swc1(0, r32(c, SP).wrapping_add(0xa4));
                c.swc1(3, r32(c, SP).wrapping_add(0xa0));
                c.lq(2, r32(c, SP).wrapping_add(0xa0));
                c.sq(2, r32(c, S0).wrapping_add(0x40));
                c.g[V1][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                c.sw(r32(c, V1).wrapping_add(0x18), c.g[A0][0]);
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                call_ptr(c, 0x003128e8, 0x0011ce5c)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_ANIM_PLAYER));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00312598, 0x0011ce68)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x00310200, 0x0011ce74)?;
                c.g[T0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0xf;
                c.g[V0][0] = c.lw(r32(c, FP).wrapping_add(0x8a8));
                c.g[A2][0] = 0u64;
                c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(0x2c));
                c.g[A0][0] = c.g[T0][0];
                c.g[V0][0] = sx(r32(c, V0) << 5);
                c.g[A3][0] = c.lw(r32(c, FP).wrapping_add(0x8ac));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[T1][0] = c.lw(r32(c, FP).wrapping_add(0x89c));
                c.lwc1(0, r32(c, V1).wrapping_add(0x1c));
                c.g[A3][0] = sx(r32(c, A3) << 5);
                c.g[T1][0] = sx(r32(c, T1) << 5);
                c.swc1(0, r32(c, SP).wrapping_add(0xbc));
                c.lwc1(2, r32(c, V1).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(2, 2);
                c.lwc1(1, r32(c, V1).wrapping_add(0x14));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.neg_s(1, 1);
                c.lwc1(0, r32(c, V1).wrapping_add(0x18));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(0, 0);
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.swc1(0, r32(c, SP).wrapping_add(0xb8));
                c.lq(2, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, FP).wrapping_add(0x960));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(0x2c));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
                c.lwc1(0, r32(c, V1).wrapping_add(0x1c));
                c.swc1(0, r32(c, SP).wrapping_add(0xbc));
                c.lwc1(2, r32(c, V1).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(2, 2);
                c.lwc1(1, r32(c, V1).wrapping_add(0x14));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.neg_s(1, 1);
                c.lwc1(0, r32(c, V1).wrapping_add(0x18));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(0, 0);
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.swc1(0, r32(c, SP).wrapping_add(0xb8));
                c.lq(2, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, FP).wrapping_add(0x990));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                c.g[V1][0] = c.lw(r32(c, T0).wrapping_add(0x2c));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T1)));
                c.lwc1(0, r32(c, V1).wrapping_add(0x1c));
                c.swc1(0, r32(c, SP).wrapping_add(0xbc));
                c.lwc1(2, r32(c, V1).wrapping_add(0x10));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(2, 2);
                c.lwc1(1, r32(c, V1).wrapping_add(0x14));
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.neg_s(1, 1);
                c.lwc1(0, r32(c, V1).wrapping_add(0x18));
                c.swc1(2, r32(c, SP).wrapping_add(0xb0));
                c.neg_s(0, 0);
                c.swc1(1, r32(c, SP).wrapping_add(0xb4));
                c.swc1(0, r32(c, SP).wrapping_add(0xb8));
                c.lq(2, r32(c, SP).wrapping_add(0xb0));
                c.sq(2, r32(c, FP).wrapping_add(0x9c0));
                c.sq(2, r32(c, SP).wrapping_add(0xa0));
                call_ptr(c, 0x0030ebc0, 0x0011cf78)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011cf80; continue;
                }
                0x0011cf80 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x10;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011cf90)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011cf98; continue;
                }
                0x0011cf98 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x11;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011cfa8)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011cfb0; continue;
                }
                0x0011cfb0 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x13;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011cfc0)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011cfc8; continue;
                }
                0x0011cfc8 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x14;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011cfd8)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011cfe0; continue;
                }
                0x0011cfe0 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x15;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011cff0)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011cff8; continue;
                }
                0x0011cff8 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x16;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d008)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d010; continue;
                }
                0x0011d010 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x17;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d020)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d028; continue;
                }
                0x0011d028 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x18;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d038)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d040; continue;
                }
                0x0011d040 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x19;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d050)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d058; continue;
                }
                0x0011d058 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x1a;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d068)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d070; continue;
                }
                0x0011d070 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x1b;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d080)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d088; continue;
                }
                0x0011d088 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x1c;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d098)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0b8; continue; }
                pc = 0x0011d0a0; continue;
                }
                0x0011d0a0 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x1d;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d0b0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x0011d0bc; continue; }
                pc = 0x0011d0b8; continue;
                }
                0x0011d0b8 => {
                c.cycles += 1;
                c.sw(r32(c, SP).wrapping_add(0x14c), c.g[V0][0]);
                pc = 0x0011d0bc; continue;
                }
                0x0011d0bc => {
                c.cycles += 9;
                c.g[V1][0] = c.lw(r32(c, SP).wrapping_add(0x14c));
                c.g[S0][0] = 0u64;
                c.g[A1][0] = 0x1e;
                c.g[A2][0] = 0u64;
                c.sw(r32(c, S6), c.g[V1][0]);
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                call_ptr(c, 0x0030ebc0, 0x0011d0d8)?;
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = 0x1; pc = 0x0011d12c; continue; }
                pc = 0x0011d0e0; continue;
                }
                0x0011d0e0 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x1f;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d0f0)?;
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = 0x1; pc = 0x0011d12c; continue; }
                pc = 0x0011d0f8; continue;
                }
                0x0011d0f8 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x20;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d108)?;
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = 0x1; pc = 0x0011d12c; continue; }
                pc = 0x0011d110; continue;
                }
                0x0011d110 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x22;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d120)?;
                
                if c.g[V0][0] == 0u64 { c.sw(r32(c, S7), c.g[S0][0]); pc = 0x0011d130; continue; }
                pc = 0x0011d128; continue;
                }
                0x0011d128 => {
                c.cycles += 1;
                c.g[S0][0] = 0x1;
                pc = 0x0011d12c; continue;
                }
                0x0011d12c => {
                c.cycles += 1;
                c.sw(r32(c, S7), c.g[S0][0]);
                pc = 0x0011d130; continue;
                }
                0x0011d130 => {
                c.cycles += 7;
                c.g[A1][0] = 0x24;
                c.g[S0][0] = 0u64;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                call_ptr(c, 0x0030ebc0, 0x0011d144)?;
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = 0x1; pc = 0x0011d198; continue; }
                pc = 0x0011d14c; continue;
                }
                0x0011d14c => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x26;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d15c)?;
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = 0x1; pc = 0x0011d198; continue; }
                pc = 0x0011d164; continue;
                }
                0x0011d164 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x27;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d174)?;
                
                if c.g[V0][0] != 0u64 { c.g[S0][0] = 0x1; pc = 0x0011d198; continue; }
                pc = 0x0011d17c; continue;
                }
                0x0011d17c => {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A1][0] = 0x2a;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0030ebc0, 0x0011d18c)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw(r32(c, SP).wrapping_add(0x154));
                if t { pc = 0x0011d19c; continue; }
                pc = 0x0011d194; continue;
                }
                0x0011d194 => {
                c.cycles += 1;
                c.g[S0][0] = 0x1;
                pc = 0x0011d198; continue;
                }
                0x0011d198 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, SP).wrapping_add(0x154));
                pc = 0x0011d19c; continue;
                }
                0x0011d19c => {
                c.cycles += 109;
                c.g[A0][0] = c.g[FP][0];
                c.g[A1][0] = 0u64;
                c.g[AT][0] = 0x41c80000;
                c.mtc1(1, 21);
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 22);
                c.sw(r32(c, V0), c.g[S0][0]);
                UPDATE_BOUNDS.call(c, 0x0011d1bc)?;
                c.g[AT][0] = 0x41700000;
                c.mtc1(1, 20);
                c.g[A1][0] = c.lw(r32(c, SP).wrapping_add(0x158));
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(0x860));
                call_ptr(c, 0x00332db8, 0x0011d1d0)?;
                c.g[A0][0] = c.g[FP][0];
                FIND_NEARBY_LIGHTS.call(c, 0x0011d1d8)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7bf0));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d1ec)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = 0x4a0000;
                c.sw(r32(c, SP).wrapping_add(0xa0), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x1088));
                c.swc1(21, r32(c, SP).wrapping_add(0xd0));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d208)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xa4), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7c00));
                c.swc1(22, r32(c, SP).wrapping_add(0xd4));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d224)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xa8), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x7a00));
                c.swc1(20, r32(c, SP).wrapping_add(0xd8));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d240)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xac), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7c10));
                c.swc1(20, r32(c, SP).wrapping_add(0xdc));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d25c)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xb0), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x7a20));
                c.swc1(20, r32(c, SP).wrapping_add(0xe0));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d278)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xb4), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x7a60));
                c.swc1(20, r32(c, SP).wrapping_add(0xe4));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d294)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xb8), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x7a70));
                c.swc1(21, r32(c, SP).wrapping_add(0xe8));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d2b0)?;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[V1][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xbc), c.g[V0][0]);
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x7a30));
                c.swc1(22, r32(c, SP).wrapping_add(0xec));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d2cc)?;
                c.g[V1][0] = 0x450000;
                c.g[A0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.g[A2][0] = sx(r32(c, V1).wrapping_add(0x7a40));
                c.sw(r32(c, SP).wrapping_add(0xc0), c.g[V0][0]);
                c.swc1(21, r32(c, SP).wrapping_add(0xf0));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00310c48, 0x0011d2e8)?;
                c.g[A1][0] = 0x450000;
                c.sw(r32(c, SP).wrapping_add(0xc4), c.g[V0][0]);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x7c20));
                c.swc1(22, r32(c, SP).wrapping_add(0xf4));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A0][0] = 0x2b0;
                call_ptr(c, 0x00317d70, 0x0011d308)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00329910, 0x0011d310)?;
                c.sw(r32(c, FP).wrapping_add(0xaa0), c.g[V0][0]);
                c.g[A0][0] = c.g[V0][0];
                c.g[AT][0] = 0x42b80000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0xa;
                c.g[V0][0] = c.lw(r32(c, FP).wrapping_add(RLS_SKEL));
                c.mul_s(12, 23, 12);
                c.g[T1][0] = c.lw(r32(c, SP).wrapping_add(0xa0));
                c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0x2c));
                c.g[A3][0] = c.lw(r32(c, SP).wrapping_add(0x15c));
                c.g[T0][0] = c.lw(r32(c, SP).wrapping_add(0x150));
                call_ptr(c, 0x003299c8, 0x0011d340)?;
                c.g[V0][0] = 0x7;
                c.g[A0][0] = c.g[FP][0];
                c.sw(r32(c, FP).wrapping_add(RLS_LOAD_STATE), c.g[V0][0]);
                RIDER_RESET.call(c, 0x0011d350)?;
                pc = 0x0011d350; continue;
                }
                0x0011d350 => {
                c.cycles += 1;
                c.lq(16, r32(c, SP).wrapping_add(0x1f0));
                pc = 0x0011d354; continue;
                }
                0x0011d354 => {
                c.cycles += 15;
                c.lq(17, r32(c, SP).wrapping_add(0x1e0));
                c.lq(18, r32(c, SP).wrapping_add(0x1d0));
                c.lq(19, r32(c, SP).wrapping_add(0x1c0));
                c.lq(20, r32(c, SP).wrapping_add(0x1b0));
                c.lq(21, r32(c, SP).wrapping_add(0x1a0));
                c.lq(22, r32(c, SP).wrapping_add(0x190));
                c.lq(23, r32(c, SP).wrapping_add(0x180));
                c.lq(30, r32(c, SP).wrapping_add(0x170));
                c.g[RA][0] = c.ld(r32(c, SP).wrapping_add(0x160));
                c.lwc1(23, r32(c, SP).wrapping_add(0x218));
                c.lwc1(22, r32(c, SP).wrapping_add(0x210));
                c.lwc1(21, r32(c, SP).wrapping_add(0x208));
                c.lwc1(20, r32(c, SP).wrapping_add(0x200));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x220));
                c.pc = t;
                return Ok(());
                }
                _ => {
                    c.jump_out(pc, RIDER_LOAD_STEP_ENTRY, RIDER_LOAD_STEP_END);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, fall_back, hook, machine, prev_cell};
    use crate::difftest::{check, put_words, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    // Fixture layout (all inside DATA..DATA+0xfffff, fake allocations from
    // LS_ALLOC, the control words at DATA+0x200000).
    const LS_CTRL: u32 = DATA + 0x20_0000;
    const LS_MAGIC: u32 = 0x4c53_c298;
    const LS_RIDER: u32 = DATA + 0x1_0000;
    const LS_SKEL: u32 = DATA + 0x1_4000;
    const LS_BONES: u32 = DATA + 0x1_9000;
    const LS_PARTS: u32 = DATA + 0x1_c000;
    const LS_PIDX: u32 = DATA + 0x1_d000;
    const LS_LIST: u32 = DATA + 0x2_0000;
    const LS_METRIX: u32 = DATA + 0x3_0000;
    const LS_HANDLER: u32 = DATA + 0x3_1000;
    const LS_ANIM: u32 = DATA + 0x3_2000;
    const LS_ALLOC: u32 = DATA + 0x6_0000;
    /// Fake ids (index in the fake table below) whose result the fixture sets.
    const ID_CHARACTER_ID: u32 = 0;
    const ID_CHEAT_ID: u32 = 4;
    const ID_ANIM_FLAGS: u32 = 20;
    const ID_FILE_READY: u32 = 35;

    #[derive(Clone, Copy)]
    enum LsKind {
        /// The word at `LS_CTRL + 0x100 + 4 * id` (0 unless the fixture sets it).
        Fixed,
        /// A pointer to fresh zeroed memory (`LS_ALLOC`).
        Alloc,
        /// Bit `n` of that word for the n-th call (flag chains).
        Bits,
    }

    /// A gated fake callee: logs its arguments like `fakes::fake`, returns
    /// the result the fixture asked for, may unwind at the fixture's call
    /// index, and stops `Rider_Reset` (the last call) at its first call.
    fn ls_fake_body(c: &mut Cpu, id: u32, addr: u32, kind: LsKind) {
        let ra = c.g[RA][0] as u32;
        fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let word = c.bus.read32(LS_CTRL + 0x100 + 4 * id);
        let seen = c.bus.read32(LS_CTRL + 0x400 + 4 * id);
        c.bus.write32(LS_CTRL + 0x400 + 4 * id, seen + 1);
        c.g[V0][0] = match kind {
            LsKind::Fixed => word as u64,
            LsKind::Bits => ((word >> (seen & 31)) & 1) as u64,
            LsKind::Alloc => {
                let n = c.bus.read32(LS_CTRL + 4);
                c.bus.write32(LS_CTRL + 4, n + 1);
                (LS_ALLOC + 0x3000 * (n & 15)) as u64
            }
        };
        if (0x0011_d390..0x0011_e098).contains(&ra) {
            c.pc = 0xdead_0ff0;
        }
    }

    macro_rules! ls_fake {
        ($name:ident, $gen:path, $id:expr, $addr:expr, $kind:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(LS_CTRL) != LS_MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                ls_fake_body(c, $id, $addr, $kind);
            }
        };
    }

    ls_fake!(ls_fake_0014a080, ee::s01::f_0014a080__generated, 0, 0x0014a080, LsKind::Fixed);
    ls_fake!(ls_fake_0014dc80, ee::s01::f_0014dc80__generated, 1, 0x0014dc80, LsKind::Alloc);
    ls_fake!(ls_fake_0014dd58, ee::s01::f_0014dd58__generated, 2, 0x0014dd58, LsKind::Alloc);
    ls_fake!(ls_fake_0014efa8, ee::s01::f_0014efa8__generated, 3, 0x0014efa8, LsKind::Fixed);
    ls_fake!(ls_fake_0014a0b0, ee::s01::f_0014a0b0__generated, 4, 0x0014a0b0, LsKind::Fixed);
    ls_fake!(ls_fake_0014bdb8, ee::s01::f_0014bdb8__generated, 5, 0x0014bdb8, LsKind::Alloc);
    ls_fake!(ls_fake_0014d998, ee::s01::f_0014d998__generated, 6, 0x0014d998, LsKind::Alloc);
    ls_fake!(ls_fake_0014d988, ee::s01::f_0014d988__generated, 7, 0x0014d988, LsKind::Fixed);
    ls_fake!(ls_fake_0014ad50, ee::s01::f_0014ad50__generated, 8, 0x0014ad50, LsKind::Alloc);
    ls_fake!(ls_fake_0014ac30, ee::s01::f_0014ac30__generated, 9, 0x0014ac30, LsKind::Fixed);
    ls_fake!(ls_fake_0014ac50, ee::s01::f_0014ac50__generated, 10, 0x0014ac50, LsKind::Fixed);
    ls_fake!(ls_fake_00144bc0, ee::s01::f_00144bc0__generated, 11, 0x00144bc0, LsKind::Fixed);
    ls_fake!(ls_fake_00145c38, ee::s01::f_00145c38__generated, 12, 0x00145c38, LsKind::Fixed);
    ls_fake!(ls_fake_0014ef70, ee::s01::f_0014ef70__generated, 13, 0x0014ef70, LsKind::Fixed);
    ls_fake!(ls_fake_002523a8, ee::s06::f_002523a8__generated, 14, 0x002523a8, LsKind::Fixed);
    ls_fake!(ls_fake_002f19f0, ee::s09::f_002f19f0__generated, 15, 0x002f19f0, LsKind::Fixed);
    ls_fake!(ls_fake_0030d4b8, ee::s10::f_0030d4b8__generated, 16, 0x0030d4b8, LsKind::Alloc);
    ls_fake!(ls_fake_0030d840, ee::s10::f_0030d840__generated, 17, 0x0030d840, LsKind::Fixed);
    ls_fake!(ls_fake_0030dba0, ee::s10::f_0030dba0__generated, 18, 0x0030dba0, LsKind::Fixed);
    ls_fake!(ls_fake_0030ea80, ee::s10::f_0030ea80__generated, 19, 0x0030ea80, LsKind::Fixed);
    ls_fake!(ls_fake_0030ebc0, ee::s10::f_0030ebc0__generated, 20, 0x0030ebc0, LsKind::Bits);
    ls_fake!(ls_fake_0030eca0, ee::s10::f_0030eca0__generated, 21, 0x0030eca0, LsKind::Fixed);
    ls_fake!(ls_fake_00310200, ee::s10::f_00310200__generated, 22, 0x00310200, LsKind::Fixed);
    ls_fake!(ls_fake_003118d0, ee::s10::f_003118d0__generated, 23, 0x003118d0, LsKind::Fixed);
    ls_fake!(ls_fake_00311a50, ee::s10::f_00311a50__generated, 24, 0x00311a50, LsKind::Fixed);
    ls_fake!(ls_fake_00312598, ee::s10::f_00312598__generated, 25, 0x00312598, LsKind::Fixed);
    ls_fake!(ls_fake_003128e8, ee::s10::f_003128e8__generated, 26, 0x003128e8, LsKind::Fixed);
    ls_fake!(ls_fake_00317d70, ee::s10::f_00317d70__generated, 27, 0x00317d70, LsKind::Alloc);
    ls_fake!(ls_fake_00317e50, ee::s10::f_00317e50__generated, 28, 0x00317e50, LsKind::Fixed);
    ls_fake!(ls_fake_0031be50, ee::s10::f_0031be50__generated, 29, 0x0031be50, LsKind::Fixed);
    ls_fake!(ls_fake_00329910, ee::s10::f_00329910__generated, 30, 0x00329910, LsKind::Alloc);
    ls_fake!(ls_fake_003299c8, ee::s10::f_003299c8__generated, 31, 0x003299c8, LsKind::Fixed);
    ls_fake!(ls_fake_00332db8, ee::s11::f_00332db8__generated, 32, 0x00332db8, LsKind::Fixed);
    ls_fake!(ls_fake_00374ce0, ee::s12::f_00374ce0__generated, 33, 0x00374ce0, LsKind::Fixed);
    ls_fake!(ls_fake_00375990, ee::s12::f_00375990__generated, 34, 0x00375990, LsKind::Fixed);
    ls_fake!(ls_fake_003df980, ee::s14::f_003df980__generated, 35, 0x003df980, LsKind::Fixed);
    ls_fake!(ls_fake_00113138, ee::s00::f_00113138__generated, 36, 0x00113138, LsKind::Alloc);
    ls_fake!(ls_fake_00104d38, ee::s00::f_00104d38__generated, 37, 0x00104d38, LsKind::Fixed);
    ls_fake!(ls_fake_0011e150, ee::s00::f_0011e150__generated, 38, 0x0011e150, LsKind::Fixed);
    ls_fake!(ls_fake_00122088, ee::s00::f_00122088__generated, 39, 0x00122088, LsKind::Fixed);

    fn ls_install_fakes() {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 40] = [
            (addr_of_mut!(ee::s01::OVERRIDE_0014A080), ls_fake_0014a080),
            (addr_of_mut!(ee::s01::OVERRIDE_0014DC80), ls_fake_0014dc80),
            (addr_of_mut!(ee::s01::OVERRIDE_0014DD58), ls_fake_0014dd58),
            (addr_of_mut!(ee::s01::OVERRIDE_0014EFA8), ls_fake_0014efa8),
            (addr_of_mut!(ee::s01::OVERRIDE_0014A0B0), ls_fake_0014a0b0),
            (addr_of_mut!(ee::s01::OVERRIDE_0014BDB8), ls_fake_0014bdb8),
            (addr_of_mut!(ee::s01::OVERRIDE_0014D998), ls_fake_0014d998),
            (addr_of_mut!(ee::s01::OVERRIDE_0014D988), ls_fake_0014d988),
            (addr_of_mut!(ee::s01::OVERRIDE_0014AD50), ls_fake_0014ad50),
            (addr_of_mut!(ee::s01::OVERRIDE_0014AC30), ls_fake_0014ac30),
            (addr_of_mut!(ee::s01::OVERRIDE_0014AC50), ls_fake_0014ac50),
            (addr_of_mut!(ee::s01::OVERRIDE_00144BC0), ls_fake_00144bc0),
            (addr_of_mut!(ee::s01::OVERRIDE_00145C38), ls_fake_00145c38),
            (addr_of_mut!(ee::s01::OVERRIDE_0014EF70), ls_fake_0014ef70),
            (addr_of_mut!(ee::s06::OVERRIDE_002523A8), ls_fake_002523a8),
            (addr_of_mut!(ee::s09::OVERRIDE_002F19F0), ls_fake_002f19f0),
            (addr_of_mut!(ee::s10::OVERRIDE_0030D4B8), ls_fake_0030d4b8),
            (addr_of_mut!(ee::s10::OVERRIDE_0030D840), ls_fake_0030d840),
            (addr_of_mut!(ee::s10::OVERRIDE_0030DBA0), ls_fake_0030dba0),
            (addr_of_mut!(ee::s10::OVERRIDE_0030EA80), ls_fake_0030ea80),
            (addr_of_mut!(ee::s10::OVERRIDE_0030EBC0), ls_fake_0030ebc0),
            (addr_of_mut!(ee::s10::OVERRIDE_0030ECA0), ls_fake_0030eca0),
            (addr_of_mut!(ee::s10::OVERRIDE_00310200), ls_fake_00310200),
            (addr_of_mut!(ee::s10::OVERRIDE_003118D0), ls_fake_003118d0),
            (addr_of_mut!(ee::s10::OVERRIDE_00311A50), ls_fake_00311a50),
            (addr_of_mut!(ee::s10::OVERRIDE_00312598), ls_fake_00312598),
            (addr_of_mut!(ee::s10::OVERRIDE_003128E8), ls_fake_003128e8),
            (addr_of_mut!(ee::s10::OVERRIDE_00317D70), ls_fake_00317d70),
            (addr_of_mut!(ee::s10::OVERRIDE_00317E50), ls_fake_00317e50),
            (addr_of_mut!(ee::s10::OVERRIDE_0031BE50), ls_fake_0031be50),
            (addr_of_mut!(ee::s10::OVERRIDE_00329910), ls_fake_00329910),
            (addr_of_mut!(ee::s10::OVERRIDE_003299C8), ls_fake_003299c8),
            (addr_of_mut!(ee::s11::OVERRIDE_00332DB8), ls_fake_00332db8),
            (addr_of_mut!(ee::s12::OVERRIDE_00374CE0), ls_fake_00374ce0),
            (addr_of_mut!(ee::s12::OVERRIDE_00375990), ls_fake_00375990),
            (addr_of_mut!(ee::s14::OVERRIDE_003DF980), ls_fake_003df980),
            (addr_of_mut!(ee::s00::OVERRIDE_00113138), ls_fake_00113138),
            (addr_of_mut!(ee::s00::OVERRIDE_00104D38), ls_fake_00104d38),
            (addr_of_mut!(ee::s00::OVERRIDE_0011E150), ls_fake_0011e150),
            (addr_of_mut!(ee::s00::OVERRIDE_00122088), ls_fake_00122088),
        ];
        for (slot, fake) in slots {
            hook(slot, fake, prev_cell(fake));
        }
    }

    /// Common fixture: a rider in load state `state`, a model with a bone
    /// table, the jump tables of the original, the control words. Returns
    /// the rng.
    fn ls_setup(c: &mut Cpu, seed: u32, state: u32, calls: u32) -> crate::difftest::Rng {
        let mut r = machine(c, seed, 0x0011_c298 + state as u64, calls);
        c.pc = RIDER_LOAD_STEP_ENTRY;
        c.bus.write32(LS_CTRL, LS_MAGIC);
        // Jump tables at 0x457c30 (load state) and 0x457c50 (character id).
        put_words(
            c,
            0x0045_7c30,
            &[0x11c300, 0x11c35c, 0x11c3c8, 0x11c540, 0x11c5b4, 0x11c598, 0x11c61c, 0x11d350],
        );
        put_words(
            c,
            0x0045_7c50,
            &[
                0x11c8d8, 0x11c8e0, 0x11c8e8, 0x11c8f0, 0x11c8f8, 0x11c900, 0x11c908, 0x11c910,
                0x11c918, 0x11c920, 0x2820_7325,
            ],
        );
        let f = r.f32(10.0);
        c.bus.write32(GAME_GP - 0x79b4, f);
        c.g[A0][0] = sx(LS_RIDER);
        let w = |c: &mut Cpu, off: u32, v: u32| c.bus.write32(LS_RIDER + off, v);
        w(c, RLS_LOAD_STATE, state);
        w(c, RLS_SKEL, LS_SKEL);
        w(c, RLS_ANIM_PLAYER, LS_ANIM);
        w(c, RLS_LOAD_LIST, LS_LIST);
        w(c, 0x77c, LS_HANDLER);
        w(c, 0x790, LS_METRIX);
        for off in [0x860, 0x86c, 0x88c, 0xaa8, 0x364] {
            let v = r.next_u32() % 4;
            w(c, off, v);
        }
        // The model: bone matrices to read, no bones to find by name.
        c.bus.write32(LS_SKEL + 0x2c, LS_BONES);
        c.bus.write32(LS_SKEL + 0xc, LS_PARTS);
        c.bus.write32(LS_SKEL + 0x1c, LS_PIDX);
        for i in 0..0x800 {
            let v = r.f32(50.0);
            c.bus.write32(LS_BONES - 0x400 + 4 * i, v);
        }
        r
    }

    /// Entries of the load list: `n` of 0x108 bytes, the first `a` in the
    /// first group; `typed` fills +0x100 (file handle) and +0x104 (name).
    fn ls_list(c: &mut Cpu, r: &mut crate::difftest::Rng, a: u32, b: u32, mode: u32) {
        c.bus.write32(LS_LIST + 0x2940, a);
        c.bus.write32(LS_LIST + 0x2944, b);
        for i in 0..a + b {
            let e = LS_LIST + 0x108 * i;
            let (handle, name) = match mode {
                // Poll: a handle that is still loading, or one already
                // released with a name (never "ready" or "to start").
                2 if r.next_u32() % 2 == 0 => (1 + r.next_u32() % 50, r.next_u32() % 2 * LS_LIST),
                2 => (u32::MAX, LS_LIST + 0x1000),
                _ => (r.next_u32() % 3, LS_LIST + 0x1000 + 0x10 * i),
            };
            c.bus.write32(e + 0x100, handle);
            c.bus.write32(e + 0x104, name);
        }
    }

    /// Load states 0 (allocate the air predictor and the animation set), 1
    /// (queue the equipped bolts: none here), 7 (nothing left to do) and
    /// above (return at once).
    #[test]
    fn rider_load_step_early_states_match() {
        ls_install_fakes();
        check(ee::s00::f_0011c298__generated, rider_load_step, 120, &|c, seed| {
            let state = [0, 1, 7, 8, 9, 0x7fff_ffff, 0xffff_ffff, 1][seed as usize % 8];
            let mut r = ls_setup(c, seed, state, 14);
            let cheat = r.next_u32() & 1;
            c.bus.write32(LS_CTRL + 0x100 + 4 * ID_CHEAT_ID, cheat);
            let ch = r.next_u32() % 12;
            c.bus.write32(LS_CTRL + 0x100 + 4 * ID_CHARACTER_ID, ch);
        });
    }

    /// Load state 2: poll the list: no entries, entries still loading,
    /// entries without a file; the state moves to 3 once none is pending.
    #[test]
    fn rider_load_step_poll_matches() {
        ls_install_fakes();
        check(ee::s00::f_0011c298__generated, rider_load_step, 300, &|c, seed| {
            let mut r = ls_setup(c, seed, 2, 6);
            let n = if seed % 5 == 0 { 0 } else { r.next_u32() % 13 };
            let a = if n == 0 { 0 } else { r.next_u32() % (n + 1) };
            ls_list(c, &mut r, a, n - a, 2);
            // No file is ever ready (the release path calls the real file
            // system; see the report).
            c.bus.write32(LS_CTRL + 0x100 + 4 * ID_FILE_READY, 0);
        });
    }

    /// Load states 3, 5 and 4: register the model names (`+0x2944` of them),
    /// compile the model, free the list and its entries, state 6.
    #[test]
    fn rider_load_step_finish_loading_matches() {
        ls_install_fakes();
        check(ee::s00::f_0011c298__generated, rider_load_step, 300, &|c, seed| {
            let state = [3, 5, 4][seed as usize % 3];
            let mut r = ls_setup(c, seed, state, 12);
            let a = r.next_u32() % 3;
            let b = r.next_u32() % 4;
            ls_list(c, &mut r, a, b, 3);
            if state == 4 && seed % 4 == 0 {
                c.bus.write32(LS_RIDER + RLS_LOAD_LIST, 0);
            }
            c.bus.write32(LS_SKEL + 0x60, r.next_u32() % 4);
        });
    }

    /// Load state 6: build the rider: joint indices and bone matrices
    /// (VU0), the flag chains over the character, lights, the physics
    /// object; ends in `Rider_Reset`, which the fakes stop at its first call.
    #[test]
    fn rider_load_step_build_matches() {
        ls_install_fakes();
        check(ee::s00::f_0011c298__generated, rider_load_step, 300, &|c, seed| {
            let mut r = ls_setup(c, seed, 6, 70);
            let ch = r.next_u32() % 12;
            c.bus.write32(LS_CTRL + 0x100 + 4 * ID_CHARACTER_ID, ch);
            let flags = if seed % 2 == 0 { 0 } else { r.next_u32() & r.next_u32() & r.next_u32() };
            c.bus.write32(LS_CTRL + 0x100 + 4 * ID_ANIM_FLAGS, flags);
        });
    }
}
