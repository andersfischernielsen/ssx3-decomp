//! Staging module oc731 (wave wave241): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc731`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f_e4a8,
        wscript_add_tex_flip_modifier,
        "WScriptFn_AddTexFlipModifier",
    ),
    (
        0x002f_e668,
        wscript_add_boost_modifier,
        "WScriptFn_AddBoostModifier",
    ),
];

/// `gp` of the game (the world and script-manager globals are gp-relative).
fn wsm_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// The `memcpy` the modifier builders copy their template with.
const WSM_MEMCPY: u32 = 0x003e_6574;

/// `isMoveNode`'s return and the node-add call of one `WScriptFn_Add*Modifier`:
/// the two builders share one shape and differ in these constants.
struct ModSite {
    /// Frame size (`addiu sp, sp, -size`).
    frame: u32,
    /// Stack offsets of the saved `s0`..`s3` (`sq`) and of `ra` (`sd`).
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    ra: u32,
    /// `gp` offset of the one-time init flag.
    flag_gp: u32,
    /// The static instance template (`0x500000 - off`, the C++ `D_E4A8`).
    tpl: u32,
    /// Template size in bytes (the stack buffer's size).
    copy_len: u64,
    /// Return address of the template copy.
    copy_ret: u32,
    /// The parameter type table (`D_00446268` and friends).
    types: u32,
    /// Cycles charged before the parameter loop.
    fill_pre: u64,
    /// Event point of the parameter loop's back edge.
    fill_head: u32,
    /// Return address of the `isMoveNode` call.
    is_move_ret: u32,
    /// The node-add function and its return address.
    move_fn: u32,
    move_ret: u32,
}

/// Constants of `WScriptFn_AddTexFlipModifier` (0x2fe4a8): a 24-byte record
/// template at `0x4fb818`, the flag at `gp+0x2530`.
const TEX_SITE: ModSite = ModSite {
    frame: 0x80,
    s0: 0x70,
    s1: 0x60,
    s2: 0x50,
    s3: 0x40,
    ra: 0x30,
    flag_gp: 0x2530,
    tpl: 0x004f_b818,
    copy_len: 0x18,
    copy_ret: 0x002f_e534,
    types: 0x0044_6268,
    fill_pre: 5,
    fill_head: 0x002f_e550,
    is_move_ret: 0x002f_e614,
    move_fn: 0x0035_5bf0,
    move_ret: 0x002f_e624,
};

/// `WScriptFn_AddBoostModifier` (0x2fe668): the same shape with a 36-byte
/// template at `0x4fb830`.
const BOOST_SITE: ModSite = ModSite {
    frame: 0x90,
    s0: 0x80,
    s1: 0x70,
    s2: 0x60,
    s3: 0x50,
    ra: 0x40,
    flag_gp: 0x2534,
    tpl: 0x004f_b830,
    copy_len: 0x24,
    copy_ret: 0x002f_e710,
    types: 0x0044_6280,
    fill_pre: 4,
    fill_head: 0x002f_e728,
    is_move_ret: 0x002f_e7ec,
    move_fn: 0x0035_5c50,
    move_ret: 0x002f_e7fc,
};

/// The one-time init of the tex-flip template (flag clear): the instance
/// record's id word, its type/value bytes and the 2.0f scale, then the flag.
fn wsm_tex_init(c: &mut Cpu, s: &ModSite) {
    c.cycles += 15;
    c.g[V1][0] = sx(s.tpl);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x14)).lbu(c);
    c.g[V0][0] |= 0xffff;
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = 1;
    let v0 = c.g[V0][0];
    Addr(s.tpl).sw(c, v0);
    let a1 = c.g[A1][0];
    Addr(s.tpl + 0x14).sb(c, a1);
    c.swc1(0, s.tpl + 0x10);
    let a0 = c.g[A0][0];
    Addr(wsm_gp(c).wrapping_add(s.flag_gp)).sw(c, a0);
    Addr(s.tpl + 4).sw(c, 0);
    Addr(s.tpl + 8).sw(c, 0);
    Addr(s.tpl + 0xc).sw(c, 0);
    let a1 = c.g[A1][0];
    Addr(sp(c)).sb(c, a1);
}

/// The one-time init of the boost template (flag clear): the id word, the
/// float fields (0.1f, 20.0f, 0.0f, 1.0f from the table in gp), then the flag.
fn wsm_boost_init(c: &mut Cpu, s: &ModSite) {
    c.cycles += 22;
    c.g[V0][0] = sx(s.tpl);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lbu(c);
    c.g[V1][0] |= 0xffff;
    c.mtc1(0, 0);
    c.g[A0][0] = 1;
    c.lwc1(1, wsm_gp(c).wrapping_sub(0x3380));
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    let v1 = c.g[V1][0];
    Addr(s.tpl).sw(c, v1);
    let a1 = c.g[A1][0];
    Addr(s.tpl + 0x20).sb(c, a1);
    c.swc1(1, s.tpl + 0xc);
    c.swc1(2, s.tpl + 0x10);
    c.swc1(0, s.tpl + 0x18);
    c.swc1(3, s.tpl + 0x1c);
    let a0 = c.g[A0][0];
    Addr(wsm_gp(c).wrapping_add(s.flag_gp)).sw(c, a0);
    Addr(s.tpl + 4).sw(c, a0);
    c.swc1(0, s.tpl + 8);
    c.swc1(0, s.tpl + 0x14);
    let a1 = c.g[A1][0];
    Addr(sp(c)).sb(c, a1);
}

/// The parameter loop: each `(idx, val, type)` entry at `A2` (count `S0`)
/// stores its value into `buf[idx]`, as a float when the type table asks for
/// one and the entry's type does not match.
fn wsm_fill(c: &mut Cpu, s: &ModSite) {
    if (c.g[S0][0] as i64) <= 0 {
        return;
    }
    c.cycles += s.fill_pre;
    c.g[V0][0] = 0x0044_0000;
    c.g[A1][0] = c.g[S0][0];
    c.g[T0][0] = sx(s.types);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, s.fill_head);
        }
    }
}

/// The instance for the id in `buf[0]` (`S1`): the script manager's for -1,
/// else the section entry's record (`0` when there is none). Leaves it in V0.
fn wsm_resolve(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    c.g[A1][0] = Addr(wsm_gp(c).wrapping_add(0x16c8)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(wsm_gp(c).wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// The node add: when the instance (V0) has an active node that answers
/// `isMoveNode` non-zero, the node-add function runs on it with `buf` (`S1`).
fn wsm_move_node(c: &mut Cpu, s: &ModSite) -> Flow {
    c.cycles += 3;
    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[V1][0] = 0x004d_0000;
    if c.g[S0][0] == 0 {
        return Ok(());
    }
    c.cycles += 7;
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    let is_move = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, is_move, s.is_move_ret)?;
    c.g[A0][0] = c.g[S0][0];
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, s.move_fn, s.move_ret)?;
    }
    c.cycles += 1;
    c.g[V1][0] = 0x004d_0000;
    Ok(())
}

/// Epilogue: copies the 12-byte `luno` nil value to the hidden return slot
/// (`S3`), leaves that pointer in V0, restores the saved registers and returns.
fn wsm_epilogue(c: &mut Cpu, s: &ModSite, frame: Frame) {
    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S3][0] as u32;
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
    frame.restore(c, S0, s.s0);
    frame.restore(c, S1, s.s1);
    frame.restore(c, S2, s.s2);
    frame.restore(c, S3, s.s3);
    frame.restore_ra(c, s.ra);
    frame.close(c);
}

/// The shared body of the two `WScriptFn_Add*Modifier`: builds the instance
/// record from the parameter list in a stack buffer (copied from the static
/// template, with the one-time `init` when the flag is clear) and adds it to
/// the instance's node list.
fn wsm_add_modifier(c: &mut Cpu, s: &ModSite, init: fn(&mut Cpu, &ModSite)) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, s.frame);
    c.g[V0][0] = Addr(wsm_gp(c).wrapping_add(s.flag_gp)).lw(c);
    frame.save(c, S2, s.s2);
    frame.save(c, S0, s.s0);
    frame.save(c, S3, s.s3);
    frame.save(c, S1, s.s1);
    frame.save_ra(c, s.ra);
    c.g[S2][0] = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    c.g[S3][0] = c.g[A0][0];
    let flag_clear = c.g[V0][0] == 0;
    c.g[A2][0] = 0x0050_0000;
    if flag_clear {
        init(c, s);
    }

    // The id word of the buffer, then the template copy.
    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx(s.tpl);
    c.g[A2][0] = s.copy_len;
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, WSM_MEMCPY, s.copy_ret)?;

    c.g[A2][0] = c.g[S2][0];
    wsm_fill(c, s);
    wsm_resolve(c);
    wsm_move_node(c, s)?;
    wsm_epilogue(c, s, frame);
    Ok(())
}

/// `WScriptFn_AddTexFlipModifier` (0x2fe4a8), the C++ `AddTexFlipFunc`:
/// adds a tex-flip modifier built from the script's parameter list to the
/// instance named by the first parameter (or the script manager's, for -1).
pub fn wscript_add_tex_flip_modifier(c: &mut Cpu) {
    guest(c, |c| wsm_add_modifier(c, &TEX_SITE, wsm_tex_init));
}

/// `WScriptFn_AddBoostModifier` (0x2fe668), the C++ `AddBoostFunc`: the same
/// as the tex-flip builder with a boost record (speed, time and scale floats).
pub fn wscript_add_boost_modifier(c: &mut Cpu) {
    guest(c, |c| wsm_add_modifier(c, &BOOST_SITE, wsm_boost_init));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The machine state of the two modifier builders: `seed % 4` parameters
    /// (entry 0 is instance id 0 on `seed % 3 == 0`, else the id stays -1), the
    /// type table, the init flag, and the world -> section -> instance chain.
    /// The instance's active node is a vtable whose `isMoveNode` is the leaf
    /// at `0x1406c0` (returns 0), so the node-add call is not reached here.
    fn wsm_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, s: &ModSite) {
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(DATA + 0x7000);
        let n = seed % 4;
        c.g[A1][0] = n as u64;
        c.g[A2][0] = sx(DATA + 0x1000);
        for i in 0..n {
            let e = DATA + 0x1000 + 16 * i;
            let idx = if i == 0 && seed % 3 == 0 {
                0
            } else {
                1 + r.next_u32() % 2
            };
            let val = if idx == 0 { 0 } else { r.next_u32() };
            c.bus.write32(e, idx);
            c.bus.write32(e + 4, val);
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
        for i in 0..3 {
            c.bus.write32(s.types + 4 * i, r.next_u32() % 3);
        }
        c.bus.write32(GAME_GP + s.flag_gp, seed % 2);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        let section = if (seed / 3) % 4 == 2 {
            0
        } else {
            DATA + 0x6000
        };
        c.bus.write32(DATA + 0x5000, section);
        c.bus.write32(DATA + 0x601c, DATA + 0x6100);
        let entry = if (seed / 5) % 5 == 4 {
            0
        } else {
            ((DATA + 0x8000) >> 2) << 8
        };
        c.bus.write32(DATA + 0x6100, entry);
        c.bus
            .write32(DATA + 0x800c, if seed % 2 == 0 { DATA + 0x9000 } else { 0 });
        c.bus.write32(DATA + 0x900c, DATA + 0x9100);
        c.bus.write32(DATA + 0x9184, 0x0014_06c0);
        c.bus.write32(GAME_GP + 0xce8, DATA + 0xa000);
        c.bus.write32(DATA + 0xa290, DATA + 0xb000);
    }

    #[test]
    fn wscript_add_tex_flip_modifier_matches() {
        check(
            ee::s10::f_002fe4a8__generated,
            wscript_add_tex_flip_modifier,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fe4_a8 ^ seed as u64);
                scramble(c, &mut r);
                wsm_fixture(c, &mut r, seed, &TEX_SITE);
            },
        );
    }

    #[test]
    fn wscript_add_boost_modifier_matches() {
        check(
            ee::s10::f_002fe668__generated,
            wscript_add_boost_modifier,
            300,
            &|c, seed| {
                let mut r = Rng(0x2fe6_68 ^ seed as u64);
                scramble(c, &mut r);
                wsm_fixture(c, &mut r, seed, &BOOST_SITE);
            },
        );
    }
}
