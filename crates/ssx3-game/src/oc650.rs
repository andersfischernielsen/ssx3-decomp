//! Staging module oc650 (wave g30.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc650`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0024_52d8,
    fe_state_progressive_on_widget_action,
    "cFEStateProgressive_OnWidgetAction",
)];

/// `StrHash(const char*)` (0x317670).
const OC650_STR_HASH: u32 = 0x0031_7670;
/// `EA_MemAllocNamed(size, name, ..)` (0x317d70).
const OC650_MEM_ALLOC: u32 = 0x0031_7d70;
/// `UIState_ctor(state, engine)` (0x39e2a0).
const OC650_UISTATE_CTOR: u32 = 0x0039_e2a0;
/// `UIStateMgr_SwitchState(stack, state)` (0x39f400).
const OC650_SWITCH_STATE: u32 = 0x0039_f400;

/// Pushes a UI state built from the engine's `mStateStack` (the dossier's
/// `newUIObj` + `cUIStateStack_push`): allocates 0x50 bytes, constructs the
/// state with `self->mpEngine`, stores the vtable at +8 and `flag` at +0x48,
/// then pushes it. `rets` are the generated body's return addresses for the
/// allocation, the constructor and the push. Returns the state in s0.
fn oc650_push_state(c: &mut Cpu, flag: u64, rets: [u32; 3]) -> Flow {
    c.g[A0][0] = 0x50;
    c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x4640));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, OC650_MEM_ALLOC, rets[0])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0x10).lw(c);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, OC650_UISTATE_CTOR, rets[1])?;
    c.g[A0][0] = Addr(c.g[S2][0] as u32).off(0x10).lw(c);
    c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x25a8));
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.sb((c.g[S0][0] as u32).wrapping_add(0x48), flag);
    call_ptr(c, OC650_SWITCH_STATE, rets[2])?;
    Ok(())
}

/// The type-5 branch of `cFEStateProgressive_OnWidgetAction`: `StrHash` of
/// the message's `+0x38` word against the string "yes" (at 0x4a2ac8). A
/// match pushes the yes state and plays frame 1 on `cGraphicsMan`; otherwise
/// the no state is pushed (flag 1, and s1 is left at 1, as the generated
/// body leaves it).
fn oc650_widget_type5(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = 0x004a_0000;
    c.g[S0][0] = Addr(c.g[A1][0] as u32).off(0x38).lw(c);
    c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2ac8));
    call_ptr(c, OC650_STR_HASH, 0x0024_5310)?;
    let is_yes = c.g[S0][0] == c.g[V0][0];
    c.g[A1][0] = 0x0048_0000;
    if is_yes {
        c.cycles += 26;
        oc650_push_state(c, 0, [0x0024_532c, 0x0024_533c, 0x0024_535c])?;
        // cGraphicsMan_TheGraphicsMan: vtable slot 8 (+0x40), frame 1.
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_5378)?;
    } else {
        c.cycles += 18;
        c.g[S1][0] = 1;
        oc650_push_state(c, 1, [0x0024_5398, 0x0024_53a8, 0x0024_53c8])?;
        c.cycles += 1;
    }
    Ok(())
}

/// `void cFEStateProgressive_OnWidgetAction(self a0, msg a1, type a2)`
/// (0x2452d8): ignores a null message, and handles only type 5 (see
/// [`oc650_widget_type5`]). Saves s0-s2 and ra in a 0x40-byte frame.
pub fn fe_state_progressive_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        if c.g[A1][0] == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            if c.g[A2][0] == c.g[V0][0] {
                oc650_widget_type5(c)?;
            }
        }
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        c.cycles += 5;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, elf_image, DATA, GAME_GP, RA};

    /// Each seed: a message whose `+0x38` word is or is not the hash of
    /// "yes", a null or non-null message, type 5 or not, and a graphics
    /// manager whose slot 8 is a fake method.
    #[test]
    fn fe_state_progressive_on_widget_action_matches() {
        check(
            ee::s06::f_002452d8__generated,
            fe_state_progressive_on_widget_action,
            300,
            &|c, seed| {
                let _r = machine(c, seed, 0x0024_52d8, 1);
                elf_image(c);
                let this = DATA + 0x1_0000;
                let engine = DATA + 0x1_0100;
                let msg = DATA + 0x1_0200;
                let gfx = DATA + 0x1_0300;
                let vt = DATA + 0x1_0400;
                // The hash of "yes" as the real StrHash computes it.
                let str_hash = ssx3_ee_rt::funcs::lookup(OC650_STR_HASH).expect("StrHash");
                c.g[A0][0] = 0x004a_2ac8;
                c.g[ssx3_ee_rt::cpu::reg::RA as usize][0] = sx(RA);
                c.pc = OC650_STR_HASH;
                str_hash(c);
                let yes = c.g[V0][0] as u32;
                c.bus.write32(msg + 0x38, if seed % 3 == 0 { yes ^ 1 } else { yes });
                c.bus.write32(this + 0x10, engine);
                c.bus.write32(GAME_GP + 0x2a90, gfx);
                c.bus.write32(gfx + 0x10d8, vt);
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, FAKE);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { sx(msg) };
                c.g[A2][0] = if seed % 5 == 0 { 4 } else { 5 };
                c.g[ssx3_ee_rt::cpu::reg::RA as usize][0] = sx(RA);
                c.pc = 0x0024_52d8;
            },
        );
    }
}
