//! Staging module oc877 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc877`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_0130,
        cfe_state_challenge_received_popup_update,
        "cFEStateChallengeReceivedPopup_Update",
    ),
    (
        0x001a_f930,
        net_fe_clear_player_stats,
        "NetFE_ClearPlayerStats",
    ),
    (
        0x001a_f568,
        cfe_state_challenge_popup_show_waiting_for_challenge,
        "cFEStateChallengePopup_ShowWaitingForChallenge",
    ),
    (
        0x001a_f208,
        cfe_state_challenge_popup_update_inform_arrows,
        "cFEStateChallengePopup_UpdateInformArrows",
    ),
];

/// The callee that hashes the name literal in a0 into v0 (0x317670).
const FE_CHALLENGE_LIT_HASH: u32 = 0x0031_7670;
/// `cUIScreen_getObjectByHashName(screen, hash)` (0x39d860).
const FE_CHALLENGE_FIND_OBJECT: u32 = 0x0039_d860;
/// Sets a text object from a literal string (0x3a0c50).
const FE_CHALLENGE_SET_LIT: u32 = 0x003a_0c50;
/// `cUIText_setUnicodeString(text, str)` (0x3a0e90).
const FE_CHALLENGE_SET_UNICODE: u32 = 0x003a_0e90;

/// `void cFEStateChallengeReceivedPopup::Update()` (0x1b0130): forward the
/// frame to the base state's update (`NetFEState_Update`, 0x1a8770).
pub fn cfe_state_challenge_received_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001b_0140)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One stat label of `NetFE_ClearPlayerStats`: hash the literal `name`, look
/// the object up on the screen in s0, and when it exists set its text to the
/// blank literal at 0x4a1390. `rets` are the three return addresses (hash,
/// lookup, set).
fn net_fe_clear_stat(c: &mut Cpu, name: u32, rets: [u32; 3]) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx(name);
    call_ptr(c, FE_CHALLENGE_LIT_HASH, rets[0])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, FE_CHALLENGE_FIND_OBJECT, rets[1])?;
    let found = c.g[V0][0] != 0;
    c.g[A1][0] = 0x004a_0000;
    if found {
        c.cycles += 3;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(0x004a_1390);
        call_ptr(c, FE_CHALLENGE_SET_LIT, rets[2])?;
    }
    Ok(())
}

/// `void NetFE_ClearPlayerStats(NetFE* screen)` (0x1af930): blank the six
/// player-stat labels of the screen: each literal is hashed, looked up on
/// the screen, and when present its text is cleared. The last label leaves
/// a1 at the hash when it is missing, as the generated body does.
pub fn net_fe_clear_player_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let null = c.g[S0][0] == 0;
        frame.save_ra(c, 0);
        if !null {
            net_fe_clear_stat(c, 0x0046_3108, [0x001a_f950, 0x001a_f95c, 0x001a_f970])?;
            net_fe_clear_stat(c, 0x0046_34e8, [0x001a_f97c, 0x001a_f988, 0x001a_f99c])?;
            net_fe_clear_stat(c, 0x0046_34f8, [0x001a_f9a8, 0x001a_f9b4, 0x001a_f9c8])?;
            net_fe_clear_stat(c, 0x0046_3508, [0x001a_f9d4, 0x001a_f9e0, 0x001a_f9f4])?;
            net_fe_clear_stat(c, 0x0046_3520, [0x001a_fa00, 0x001a_fa0c, 0x001a_fa20])?;
            c.cycles += 8;
            c.g[A0][0] = sx(0x0046_3538);
            call_ptr(c, FE_CHALLENGE_LIT_HASH, 0x001a_fa2c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, FE_CHALLENGE_FIND_OBJECT, 0x001a_fa38)?;
            c.g[A0][0] = c.g[V0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = sx(0x004a_1390);
                call_ptr(c, FE_CHALLENGE_SET_LIT, 0x001a_fa4c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateChallengePopup::ShowWaitingForChallenge(self)` (0x1af568):
/// mark the popup as waiting (word +0x6d0 = 1), look up the challenge
/// widgets on the screen at +0x40, and fill the status line with the
/// formatted challenge text. The settings are locked when the screen has
/// its main object.
pub fn cfe_state_challenge_popup_show_waiting_for_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x460);
        c.g[V0][0] = 1;
        frame.save(c, S3, 0x420);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x410);
        frame.save(c, S0, 0x450);
        frame.save(c, S1, 0x440);
        frame.save(c, S2, 0x430);
        frame.save_ra(c, 0x400);
        c.g[A0][0] = sx(0x0046_3468);
        let waiting = c.g[V0][0];
        Addr(r32(c, S3)).off(0x6d0).sw(c, waiting);
        call_ptr(c, FE_CHALLENGE_LIT_HASH, 0x001a_f59c)?;
        c.g[A0][0] = Addr(r32(c, S3)).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, FE_CHALLENGE_FIND_OBJECT, 0x001a_f5a8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        if c.g[S4][0] != 0 {
            c.cycles += 44;
            c.g[A0][0] = sx(0x0046_3480);
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
            c.g[S0][0] = Addr(r32(c, V1).wrapping_add(0x8c)).lw(c);
            c.g[S2][0] = Addr(r32(c, V0).wrapping_add(0x98)).lw(c);
            c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x20)).lh(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            call_ptr(c, FE_CHALLENGE_LIT_HASH, 0x001a_f5d8)?;
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x24)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            let slot = r32(c, V1);
            call_ptr(c, slot, 0x001a_f5e8)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x002c_26d0, 0x001a_f5f8)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, FE_CHALLENGE_SET_UNICODE, 0x001a_f604)?;
            c.g[A0][0] = sx(0x0046_31c8);
            call_ptr(c, FE_CHALLENGE_LIT_HASH, 0x001a_f610)?;
            c.g[A0][0] = Addr(r32(c, S3)).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, FE_CHALLENGE_FIND_OBJECT, 0x001a_f61c)?;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x40)).lh(c);
            c.g[A2][0] = Addr(r32(c, V1).wrapping_add(0x44)).lw(c);
            let slot = r32(c, A2);
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            call_ptr(c, slot, 0x001a_f634)?;
            c.g[A0][0] = sx(0x0046_30e0);
            call_ptr(c, FE_CHALLENGE_LIT_HASH, 0x001a_f640)?;
            c.g[A0][0] = Addr(r32(c, S3)).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, FE_CHALLENGE_FIND_OBJECT, 0x001a_f64c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 7;
            call_ptr(c, 0x0039_b960, 0x001a_f658)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x001a_ec60, 0x001a_f664)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x450);
        frame.restore(c, S1, 0x440);
        frame.restore(c, S2, 0x430);
        frame.restore(c, S3, 0x420);
        frame.restore(c, S4, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

/// One compare stage of `UpdateInformArrows`: `prefix` cycles, the literal
/// `lit` is hashed and compared with the state's name word (+0x38). On a
/// match, `equal` cycles, the `found` literal's object is looked up and kept
/// in s4, and `next` is left in a0 for the tail lookup. `rets` are the three
/// return addresses (hash, hash of `found`, lookup).
struct InformArrowStage {
    prefix: u64,
    lit: u32,
    rets: [u32; 3],
    equal: u64,
    found: u32,
    next: u32,
}

const INFORM_ARROW_STAGES: [InformArrowStage; 5] = [
    InformArrowStage {
        prefix: 8,
        lit: 0x0046_31f8,
        rets: [0x001a_f250, 0x001a_f264, 0x001a_f270],
        equal: 10,
        found: 0x0046_3300,
        next: 0x0046_3310,
    },
    InformArrowStage {
        prefix: 5,
        lit: 0x0046_3210,
        rets: [0x001a_f28c, 0x001a_f2a0, 0x001a_f2ac],
        equal: 10,
        found: 0x0046_3320,
        next: 0x0046_3330,
    },
    InformArrowStage {
        prefix: 5,
        lit: 0x0046_3238,
        rets: [0x001a_f2c8, 0x001a_f2dc, 0x001a_f2e8],
        equal: 10,
        found: 0x0046_3340,
        next: 0x0046_3350,
    },
    InformArrowStage {
        prefix: 5,
        lit: 0x0046_3248,
        rets: [0x001a_f304, 0x001a_f318, 0x001a_f324],
        equal: 10,
        found: 0x0046_3360,
        next: 0x0046_3370,
    },
    InformArrowStage {
        prefix: 5,
        lit: 0x0046_3260,
        rets: [0x001a_f340, 0x001a_f350, 0x001a_f35c],
        equal: 8,
        found: 0x0046_3380,
        next: 0x0046_3390,
    },
];

/// Run one stage. `Ok(Some(next))` when the name word matched (the child
/// for `found` is in s4 and a0 holds `next`), `Ok(None)` otherwise (a0 is
/// the blank base 0x460000).
fn inform_arrow_stage(c: &mut Cpu, st: &InformArrowStage) -> Result<Option<u32>, Unwound> {
    c.cycles += st.prefix;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    c.g[A0][0] = sx(st.lit);
    call_ptr(c, FE_CHALLENGE_LIT_HASH, st.rets[0])?;
    let same = c.g[S0][0] == c.g[V0][0];
    c.g[A0][0] = 0x0046_0000;
    if !same {
        return Ok(None);
    }
    c.cycles += st.equal;
    c.g[A0][0] = sx(st.found);
    call_ptr(c, FE_CHALLENGE_LIT_HASH, st.rets[1])?;
    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, FE_CHALLENGE_FIND_OBJECT, st.rets[2])?;
    c.g[S4][0] = c.g[V0][0];
    c.g[A0][0] = sx(st.next);
    Ok(Some(st.next))
}

/// The compare chain of `UpdateInformArrows` (blocks `'b_1af280` to the
/// `'b_1af368` tail): the first stage whose literal matches picks the child
/// for its next literal into s4, and then the lookup in `'b_1af368` picks s3.
fn inform_arrows_compares(c: &mut Cpu) -> Flow {
    c.g[S4][0] = 0;
    c.g[S3][0] = 0;
    let mut matched = None;
    for stage in INFORM_ARROW_STAGES {
        matched = inform_arrow_stage(c, &stage)?;
        if matched.is_some() {
            break;
        }
    }
    if matched.is_some() {
        c.cycles += 6;
        call_ptr(c, FE_CHALLENGE_LIT_HASH, 0x001a_f370)?;
        c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, FE_CHALLENGE_FIND_OBJECT, 0x001a_f37c)?;
        c.g[S3][0] = c.g[V0][0];
    }
    Ok(())
}

/// `obj->slot(a0 = obj + half(+0x48), a1 = s5)`: the notify call of the
/// found child in s4 (`obj_reg`) through the vtable at obj+8 (slot +0x4c).
fn inform_arrow_notify(c: &mut Cpu, obj_reg: usize, ret: u32) -> Flow {
    let obj = r32(c, obj_reg);
    c.g[V0][0] = Addr(obj.wrapping_add(0x8)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    let slot = r32(c, V1);
    c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
    call_ptr(c, slot, ret)
}

/// `void cFEStateChallengePopup::UpdateInformArrows(self, state, arg)`
/// (0x1af208): match the popup's state name against the arrow labels and
/// notify the matching child objects (in s4 and s3) with `arg`.
pub fn cfe_state_challenge_popup_update_inform_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S5, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        let state_null = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !state_null {
            inform_arrows_compares(c)?;
            c.cycles += 2;
            c.g[A1][0] = c.g[S5][0];
            if c.g[S4][0] != 0 {
                c.cycles += 5;
                inform_arrow_notify(c, S4, 0x001a_f39c)?;
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S5][0];
            if c.g[S3][0] != 0 {
                c.cycles += 5;
                inform_arrow_notify(c, S3, 0x001a_f3b8)?;
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA, GAME_GP};

    /// The value the literal-hash callee returns for `lit`, computed once per
    /// literal on a fresh machine (its result depends only on the literal).
    fn oc877_hash(lit: u32) -> u32 {
        use std::sync::Mutex;
        static CACHE: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
        if let Some(&(_, h)) = CACHE.lock().unwrap().iter().find(|(l, _)| *l == lit) {
            return h;
        }
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = sx(lit);
        c.g[RA][0] = sx(0x0012_3450);
        c.pc = FE_CHALLENGE_LIT_HASH;
        ssx3_ee_rt::funcs::call_indirect(&mut c, FE_CHALLENGE_LIT_HASH);
        let h = c.g[V0][0] as u32;
        CACHE.lock().unwrap().push((lit, h));
        h
    }

    /// A UI screen at `screen` whose object list holds one object per literal
    /// in `lits` (hash at +0x38, a vtable with the fake slot at +0x44 and
    /// +0x4c at +0x8). The list lives at fixed addresses under DATA+0x2000.
    fn oc877_screen(c: &mut Cpu, screen: u32, lits: &[u32]) {
        let count = DATA + 0x2200;
        let objs = DATA + 0x2300;
        let vt = DATA + 0x2800;
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(screen + 0x3c, objs);
        c.bus.write32(count, lits.len() as u32);
        c.bus.write32(vt + 0x44, FAKE);
        c.bus.write32(vt + 0x4c, FAKE);
        for (i, lit) in lits.iter().enumerate() {
            let o = DATA + 0x2400 + 0x40 * i as u32;
            c.bus.write32(objs + 4 * i as u32, o);
            c.bus.write32(o + 0x38, oc877_hash(*lit));
            c.bus.write32(o + 0x8, vt);
        }
    }

    #[test]
    fn cfe_state_challenge_received_popup_update_matches() {
        check(
            ee::s03::f_001b0130__generated,
            cfe_state_challenge_received_popup_update,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x001b_0130, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn net_fe_clear_player_stats_matches() {
        const SIX: [u32; 6] = [
            0x0046_3108,
            0x0046_34e8,
            0x0046_34f8,
            0x0046_3508,
            0x0046_3520,
            0x0046_3538,
        ];
        check(
            ee::s03::f_001af930__generated,
            net_fe_clear_player_stats,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x001a_f930, 0);
                let screen = DATA + 0x2000;
                oc877_screen(c, screen, &SIX[..(seed % 7) as usize]);
                c.g[A0][0] = sx(if seed % 11 == 0 { 0 } else { screen });
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_show_waiting_for_challenge_matches() {
        check(
            ee::s03::f_001af568__generated,
            cfe_state_challenge_popup_show_waiting_for_challenge,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x001a_f568, 4);
                let this = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let lits: &[u32] = if seed % 4 == 0 {
                    &[0x0046_31c8, 0x0046_30e0]
                } else {
                    &[0x0046_3468, 0x0046_31c8, 0x0046_30e0]
                };
                oc877_screen(c, screen, lits);
                c.bus.write32(this + 0x40, screen);
                let world = DATA + 0x3000;
                let s0 = DATA + 0x3800;
                let s1 = DATA + 0x3900;
                let v0 = DATA + 0x3400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x8c, s0);
                c.bus.write32(s0 + 0x4, s1);
                c.bus.write32(s1 + 0x24, FAKE);
                c.bus.write32(GAME_GP - 0xc8, v0);
                c.bus.write32(v0 + 0x98, DATA + 0x3a00);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_update_inform_arrows_matches() {
        const STAGE_LITS: [u32; 5] = [
            0x0046_31f8,
            0x0046_3210,
            0x0046_3238,
            0x0046_3248,
            0x0046_3260,
        ];
        const FOUND: [u32; 10] = [
            0x0046_3300,
            0x0046_3310,
            0x0046_3320,
            0x0046_3330,
            0x0046_3340,
            0x0046_3350,
            0x0046_3360,
            0x0046_3370,
            0x0046_3380,
            0x0046_3390,
        ];
        check(
            ee::s03::f_001af208__generated,
            cfe_state_challenge_popup_update_inform_arrows,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x001a_f208, 4);
                let this = DATA + 0x1000;
                let state = DATA + 0x1100;
                let screen = DATA + 0x2000;
                oc877_screen(c, screen, &FOUND);
                c.bus.write32(this + 0x40, screen);
                let name = if seed % 6 == 5 {
                    0x1234_5678
                } else {
                    oc877_hash(STAGE_LITS[(seed % 6) as usize])
                };
                c.bus.write32(state + 0x38, name);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(if seed % 11 == 0 { 0 } else { state });
                c.g[A2][0] = sx(DATA + 0x1200);
            },
        );
    }
}
