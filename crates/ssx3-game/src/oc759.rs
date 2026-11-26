//! Staging module oc759 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc759`.

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
    (
        0x002e_26b8,
        spray_fx_configure_for_rider,
        "cDynamicSprayFX_ConfigureForRider",
    ),
    (
        0x002f_6238,
        rider_base_ctor_cheat_code,
        "RiderBase_CtorCheatCode",
    ),
    (0x002e_3680, player_pulse_fx_ctor, "cPlayerPulseFX_ctor"),
    (0x0039_9768, uilistbox_ctor_w244, "UIListBox_ctor"),
    (0x003d_1ba8, path_remove_event, "PATH_RemoveEvent"),
    (
        0x0029_ced8,
        ssx_audio_system_play_multiplier_sfx,
        "SSXAudioSystem_PlayMultiplierSfx",
    ),
];

/// `base + off` (a 32-bit guest address).
#[inline(always)]
fn at759(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

// ---------------------------------------------------------------------------
// cDynamicSprayFX_ConfigureForRider (0x2e26b8)

/// Offsets used by `cDynamicSprayFX_ConfigureForRider` (`*self` is the rider).
pub(crate) const SPRAY_IS_PLAYER: u32 = 0x124;
pub(crate) const SPRAY_EMITTERS: u32 = 0x28;
pub(crate) const SPRAY_PARAMS: u32 = 0x2c;
pub(crate) const RIDER_VIEW_INDEX: u32 = 0x870;

/// `void cDynamicSprayFX_ConfigureForRider(cDynamicSprayFX* self)` (0x2e26b8):
/// `mIsPlayer = (rider->mViewIndex >= 0)`; sets the spray tuning floats (the
/// player's values or the AI's) and tells emitters 5 and 6 about their
/// parameter blocks through their vtable slot 3.
pub fn spray_fx_configure_for_rider(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = c.lw(at759(c.g[V1][0], RIDER_VIEW_INDEX));
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
        let off_player = c.g[V0][0] == 0;
        c.sw(at759(c.g[S0][0], SPRAY_IS_PLAYER), c.g[V0][0]);
        if !off_player {
            // The player's tuning: 55, 25, 170, 30, 20, 60.
            c.cycles += 18;
            c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
            c.g[AT][0] = 0x425c0000;
            c.mtc1(1, 0);
            c.g[AT][0] = 0x41c80000;
            c.mtc1(1, 1);
            c.swc1(0, at759(c.g[V0][0], 0x498));
            c.g[AT][0] = 0x432a0000;
            c.mtc1(1, 2);
            c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
            c.g[AT][0] = 0x41f00000;
            c.mtc1(1, 0);
            c.swc1(1, at759(c.g[V0][0], 0x4a0));
            c.g[AT][0] = 0x41a00000;
            c.mtc1(1, 3);
            c.g[AT][0] = 0x42700000;
            c.mtc1(1, 1);
            c.g[V1][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
        } else {
            // The AI's tuning: 24.75, 5, 76.5, 19.5, 8, 39.
            c.cycles += 17;
            c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
            c.g[AT][0] = 0x41c60000;
            c.mtc1(1, 0);
            c.g[AT][0] = 0x40a00000;
            c.mtc1(1, 1);
            c.swc1(0, at759(c.g[V0][0], 0x498));
            c.g[AT][0] = 0x42990000;
            c.mtc1(1, 2);
            c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
            c.g[AT][0] = 0x419c0000;
            c.mtc1(1, 0);
            c.swc1(1, at759(c.g[V0][0], 0x4a0));
            c.g[AT][0] = 0x41000000;
            c.mtc1(1, 3);
            c.g[V1][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
            c.g[AT][0] = 0x421c0000;
            c.mtc1(1, 1);
        }
        c.cycles += 31;
        c.swc1(2, at759(c.g[V1][0], 0x554));
        c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
        c.swc1(0, at759(c.g[V0][0], 0x580));
        c.g[V1][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
        c.swc1(3, at759(c.g[V1][0], 0x588));
        c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
        c.swc1(1, at759(c.g[V0][0], 0x63c));
        // Emitter 5: vtable slot 3 (adjusted `this`) with the param block at +0x488.
        c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_EMITTERS));
        c.g[A1][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
        c.g[A2][0] = c.lw(at759(c.g[V0][0], 0xc48));
        c.g[A3][0] = sx(at759(c.g[V0][0], 0xa50));
        c.lwc1(12, at759(c.g[V0][0], 0xaa8));
        c.g[A1][0] = sx(at759(c.g[A1][0], 0x488));
        c.g[A0][0] = c.lh(at759(c.g[A2][0], 0x18));
        c.g[V1][0] = c.lw(at759(c.g[A2][0], 0x1c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_27b0)?;
        // Emitter 6, param block at +0x570.
        c.g[V0][0] = c.lw(at759(c.g[S0][0], SPRAY_EMITTERS));
        c.g[A1][0] = c.lw(at759(c.g[S0][0], SPRAY_PARAMS));
        c.g[V1][0] = c.lw(at759(c.g[V0][0], 0xe58));
        c.g[A2][0] = sx(at759(c.g[V0][0], 0xc60));
        c.lwc1(12, at759(c.g[V0][0], 0xcb8));
        c.g[A1][0] = sx(at759(c.g[A1][0], 0x570));
        c.g[A0][0] = c.lh(at759(c.g[V1][0], 0x18));
        c.g[V0][0] = c.lw(at759(c.g[V1][0], 0x1c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002e_27d8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderBase_CtorCheatCode (0x2f6238) and cPlayerPulseFX_ctor (0x2e3680)

/// What differs between the two rider-wrapper constructors (the generated
/// bodies are otherwise identical, register for register).
struct RiderWrapperCtor {
    /// Offset of the embedded rider inside the wrapper (`self + off`).
    rider_off: u32,
    /// `0x490000 - vt_a` / `- vt_b`: the two vtable templates.
    vt_a: u32,
    vt_b: u32,
    /// Return address of the `Rider_ctor` call.
    ret_rider: u32,
    /// Loop head addresses (for the event checks).
    loop1: u32,
    loop2: u32,
    /// The cleanup call (`Clear`/`Reset`) and its return address.
    tail: u32,
    ret_tail: u32,
}

const CHEAT_CTOR: RiderWrapperCtor = RiderWrapperCtor {
    rider_off: 0x10,
    vt_a: 0x7920,
    vt_b: 0x78d8,
    ret_rider: 0x002f_6264,
    loop1: 0x002f_62b0,
    loop2: 0x002f_6314,
    tail: 0x002f_64f0,
    ret_tail: 0x002f_6370,
};

const PULSE_CTOR: RiderWrapperCtor = RiderWrapperCtor {
    rider_off: 0x40,
    vt_a: 0x7638,
    vt_b: 0x75f0,
    ret_rider: 0x002e_36ac,
    loop1: 0x002e_36f8,
    loop2: 0x002e_375c,
    tail: 0x002e_3930,
    ret_tail: 0x002e_37b8,
};

/// `T* ctor(T* self, int inChrg)` for a wrapper embedding a `cRider`: with
/// `inChrg` construct the rider at `self + rider_off`; point the rider's three
/// vtable fields (+0x6e8, +0x6d0, +0x6c0) at the templates; without `inChrg`
/// build adjusted copies of both templates on the stack (this-adjustment of
/// entry 1) and point +0x6e8 / +0x6c0 at the copies; then the type's reset.
fn rider_wrapper_ctor(c: &mut Cpu, k: &RiderWrapperCtor) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let in_charge = c.g[S1][0] != 0;
        frame.save_ra(c, 0x100);
        if in_charge {
            c.cycles += 4;
            c.g[V0][0] = sx(at759(c.g[S0][0], k.rider_off));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(k.ret_rider);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x0011_b698, k.ret_rider)?;
        }
        'vt: {
            c.cycles += 15;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 0x490000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(k.vt_a));
            c.g[A0][0] = 0x460000;
            c.sw(at759(c.g[V1][0], 0x6e8), c.g[A2][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
            c.g[V1][0] = 0x490000;
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[T2][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(k.vt_b));
            c.sw(at759(c.g[A1][0], 0x6d0), c.g[A0][0]);
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            let skip_copies = c.g[S1][0] != 0;
            c.sw(at759(c.g[V0][0], 0x6c0), c.g[V1][0]);
            if skip_copies {
                break 'vt;
            }
            // Copy template A (0x48 bytes) to the stack.
            c.cycles += 4;
            c.g[A1][0] = c.g[A2][0];
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx(at759(c.g[A1][0], 0x40));
            c.g[T0][0] = sx(at759(c.g[SP][0], 0x50));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.ld(c.g[A1][0] as u32);
                c.g[A2][0] = c.ld(at759(c.g[A1][0], 0x8));
                c.g[T1][0] = c.ld(at759(c.g[A1][0], 0x10));
                c.g[T3][0] = c.ld(at759(c.g[A1][0], 0x18));
                c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                c.sd(at759(c.g[V1][0], 0x8), c.g[A2][0]);
                c.sd(at759(c.g[V1][0], 0x10), c.g[T1][0]);
                c.sd(at759(c.g[V1][0], 0x18), c.g[T3][0]);
                c.g[A1][0] = sx(at759(c.g[A1][0], 0x20));
                let more = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx(at759(c.g[V1][0], 0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, k.loop1);
                }
            }
            c.cycles += 14;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(k.vt_a));
            c.g[A2][0] = c.ld(c.g[A1][0] as u32);
            c.sd(c.g[V1][0] as u32, c.g[A2][0]);
            c.sw(at759(c.g[V0][0], 0x6e8), c.g[SP][0]);
            c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(k.vt_b));
            c.g[V1][0] = c.lhu(at759(c.g[A0][0], 0x8));
            c.g[T1][0] = sx(at759(c.g[A2][0], 0xa0));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = c.g[T0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(k.rider_off));
            c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sh(at759(c.g[SP][0], 0x8), c.g[V1][0]);
            // Copy template B (0xb0 bytes) after it.
            loop {
                c.cycles += 11;
                c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                c.g[V1][0] = c.ld(at759(c.g[A2][0], 0x8));
                c.g[A0][0] = c.ld(at759(c.g[A2][0], 0x10));
                c.g[T3][0] = c.ld(at759(c.g[A2][0], 0x18));
                c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                c.sd(at759(c.g[A1][0], 0x8), c.g[V1][0]);
                c.sd(at759(c.g[A1][0], 0x10), c.g[A0][0]);
                c.sd(at759(c.g[A1][0], 0x18), c.g[T3][0]);
                c.g[A2][0] = sx(at759(c.g[A2][0], 0x20));
                let more = c.g[A2][0] != c.g[T1][0];
                c.g[A1][0] = sx(at759(c.g[A1][0], 0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, k.loop2);
                }
            }
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(k.vt_b));
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[V1][0] = c.lhu(at759(c.g[V0][0], 0x8));
            c.g[V0][0] = c.ld(c.g[A2][0] as u32);
            c.g[T1][0] = c.ld(at759(c.g[A2][0], 0x8));
            c.sd(c.g[A1][0] as u32, c.g[V0][0]);
            c.sd(at759(c.g[A1][0], 0x8), c.g[T1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sw(at759(c.g[A0][0], 0x6c0), c.g[T0][0]);
            c.sh(at759(c.g[SP][0], 0x58), c.g[V1][0]);
        }
        c.cycles += 8;
        c.g[RA][0] = sx(k.ret_tail);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, k.tail, k.ret_tail)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x110);
        frame.restore(c, S0, 0x120);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `RiderBase* RiderBase_CtorCheatCode(RiderBase* self, int inChrg)`
/// (0x2f6238): rider wrapper at `self + 0x10`, then `RiderBase_ClearCheatCode`.
pub fn rider_base_ctor_cheat_code(c: &mut Cpu) {
    rider_wrapper_ctor(c, &CHEAT_CTOR);
}

/// `cPlayerPulseFX* cPlayerPulseFX_ctor(self, int inChrg)` (0x2e3680): rider
/// wrapper at `self + 0x40`, then `cPlayerPulseFX_Reset`.
pub fn player_pulse_fx_ctor(c: &mut Cpu) {
    rider_wrapper_ctor(c, &PULSE_CTOR);
}

// ---------------------------------------------------------------------------
// UIListBox_ctor (0x399768)

/// `void UIListBox_ctor(UIListBox* self)` (0x399768): base `UIObject_ctor`,
/// vtable, attribute bits, 32 cleared 0x14-byte rows at +0x98, entry counters
/// cleared, the row area `memset` (0x280 bytes) and the static colour copied
/// from the global at 0x4c66c8 to +0x88.
pub fn uilistbox_ctor_w244(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0039_977c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_fb30, 0x0039_977c)?;
        c.g[V0][0] = c.lbu(at759(c.g[S0][0], 0x74));
        c.g[V1][0] = 0x490000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4798));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.sw(at759(c.g[S0][0], 0x8), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xc4;
        c.g[A0][0] = sx(at759(c.g[S0][0], 0x98));
        c.sb(at759(c.g[S0][0], 0x74), c.g[V0][0]);
        c.g[V0][0] = 0x1f;
        loop {
            c.cycles += 8;
            c.sw(c.g[A0][0] as u32, 0);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.sw(at759(c.g[A0][0], 0x4), 0);
            c.sw(at759(c.g[A0][0], 0x8), 0);
            c.sw(at759(c.g[A0][0], 0xc), 0);
            c.sb(at759(c.g[A0][0], 0x10), 0);
            let more = c.g[V0][0] != c.g[A1][0];
            c.g[A0][0] = sx(at759(c.g[A0][0], 0x14));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0039_97a8);
            }
        }
        c.cycles += 21;
        c.sb(at759(c.g[S0][0], 0x318), 0);
        c.g[A0][0] = sx(at759(c.g[S0][0], 0x98));
        c.sb(at759(c.g[S0][0], 0x319), 0);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0039_97e0);
        c.g[A2][0] = 0x280;
        call_ptr(c, 0x0041_6210, 0x0039_97e0)?;
        c.g[V1][0] = 0x4c0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66c8));
        c.ldl(4, at759(c.g[A3][0], 0x7));
        c.ldr(4, c.g[A3][0] as u32);
        c.ldl(5, at759(c.g[A3][0], 0xf));
        c.ldr(5, at759(c.g[A3][0], 0x8));
        c.sdl(4, at759(c.g[S0][0], 0x8f));
        c.sdr(4, at759(c.g[S0][0], 0x88));
        c.sdl(5, at759(c.g[S0][0], 0x97));
        c.sdr(5, at759(c.g[S0][0], 0x90));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_RemoveEvent (0x3d1ba8)

/// `void PATH_RemoveEvent(int idx)` (0x3d1ba8): remove the event in slot
/// `idx` of the path player (global at 0x44f420): if it lies in the event
/// heap (+0x11c .. +0x91c) close the gap with `memcpy` and shift the slot
/// pointers above it down by the event's size; otherwise call the removal
/// callback (global at 0x44f428). Then clear the slot, close the slot array
/// and decrement the event count (+0x91c).
pub fn path_remove_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x450000;
        frame.save(c, S0, 0x80);
        frame.save(c, S4, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        c.g[S4][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbe0));
        c.g[S6][0] = c.g[V0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
        frame.save(c, S5, 0x30);
        c.g[S3][0] = sx(at759(c.g[V1][0], 0x91c));
        frame.save_ra(c, 0x10);
        c.g[V1][0] = sx(at759(c.g[V1][0], 0x11c));
        frame.save(c, S1, 0x70);
        c.g[S1][0] = c.lw(at759(c.g[V0][0], 0xdc));
        c.g[V0][0] = c.lhu(at759(c.g[S1][0], 0xe));
        c.g[V1][0] = (c.g[S1][0] < c.g[V1][0]) as u64;
        // Dead copy of the event's 16-byte header to the stack.
        c.ldl(4, at759(c.g[S1][0], 0x7));
        c.ldr(4, c.g[S1][0] as u32);
        c.ldl(6, at759(c.g[S1][0], 0xf));
        c.ldr(6, at759(c.g[S1][0], 0x8));
        c.sdl(4, at759(c.g[SP][0], 0x7));
        c.sdr(4, c.g[SP][0] as u32);
        c.sdl(6, at759(c.g[SP][0], 0xf));
        c.g[V0][0] = c.g[V0][0] & 0x3f;
        c.sdr(6, at759(c.g[SP][0], 0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        let below_heap = c.g[V1][0] != 0;
        c.g[S2][0] = sx(at759(c.g[V0][0], 0x10));
        let mut in_heap = false;
        if !below_heap {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < c.g[S3][0]) as u64;
            let above_heap = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            in_heap = !above_heap;
        }
        if in_heap {
            c.cycles += 9;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            c.g[RA][0] = sx(0x003d_1c44);
            c.g[S5][0] = c.g[S4][0];
            call_ptr(c, 0x0041_605c, 0x003d_1c44)?;
            c.g[S4][0] = sx(at759(c.g[S0][0], 0x1));
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0xbe0));
            c.g[A1][0] = 0;
            c.g[A0][0] = sx(at759(c.g[V0][0], 0xdc));
            loop {
                c.cycles += 4;
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = (c.g[S1][0] < c.g[V1][0]) as u64;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = (c.g[V1][0] < c.g[S3][0]) as u64;
                if !skip {
                    c.cycles += 2;
                    let skip = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                    if !skip {
                        c.cycles += 1;
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    }
                }
                c.cycles += 4;
                c.g[A1][0] = sx(at759(c.g[A1][0], 0x1));
                c.g[V0][0] = (c.g[A1][0] < 0x11u64) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(at759(c.g[A0][0], 0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_1c58);
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0xbe0));
        } else {
            // Not in the heap: the removal callback, if any.
            c.cycles += 5;
            c.g[V0][0] = 0x450000;
            c.g[S4][0] = sx(at759(c.g[S0][0], 0x1));
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbd8));
            let no_callback = c.g[V0][0] == 0;
            c.g[S5][0] = sx((c.g[S0][0] as u32) << 2);
            if !no_callback {
                c.cycles += 2;
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003d_1ca8);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, target, 0x003d_1ca8)?;
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0xbe0));
        }
        // Clear the slot and close the slot array.
        c.cycles += 5;
        c.g[S0][0] = c.g[S4][0];
        c.g[V1][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
        let last = c.g[V1][0] == 0;
        c.sw(at759(c.g[V0][0], 0xdc), 0);
        if !last {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V0][0] = sx(at759(c.g[V0][0], 0xd8));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V0][0] = c.lw(at759(c.g[A0][0], 0x4));
                c.g[S0][0] = sx(at759(c.g[S0][0], 0x1));
                c.g[V1][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                let more = c.g[V1][0] != 0;
                c.g[A0][0] = sx(at759(c.g[A0][0], 0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_1cd0);
                }
            }
        }
        c.cycles += 14;
        c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0xbe0));
        frame.restore(c, S0, 0x80);
        c.g[V0][0] = c.lw(at759(c.g[V1][0], 0x91c));
        frame.restore(c, S1, 0x70);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        c.sw(at759(c.g[V1][0], 0x91c), c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayMultiplierSfx (0x29ced8)

/// `void SSXAudioSystem_PlayMultiplierSfx(self, int kind, Obj* obj, float x)`
/// (0x29ced8): for a local human (or no) `obj`, pick the sfx id for `kind`
/// (0..3 -> 0x70, 0x71, 0x75, 0x76; else 0x74), queue a voice into the audio
/// system's voice table (the 9-field record the inlined `AddVoice` fills),
/// resolve the channel gain pointer, push the sound, and for kind 2 say the
/// arcade line for multiplier x = 2, 3, 5 or 10.
pub fn ssx_audio_system_play_multiplier_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 20, 0x70);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.mov_s(20, 12);
        frame.save(c, S1, 0x50);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        'done: {
            if c.g[S3][0] != 0 {
                // A given object must be a local human.
                c.cycles += 3;
                c.g[V0][0] = c.lw(at759(c.g[S3][0], 0x874));
                c.g[V1][0] = 0;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(at759(c.g[S3][0], 0x87c));
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                }
                c.cycles += 2;
                let not_local = c.g[V1][0] == 0;
                frame.restore(c, S0, 0x60);
                if not_local {
                    break 'done;
                }
            }
            // The sfx id for `kind` in s5.
            c.cycles += 2;
            c.g[S5][0] = 0x74;
            if c.g[S4][0] == 0 {
                c.cycles += 2;
                c.g[S5][0] = 0x70;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let not1 = c.g[S4][0] != c.g[V0][0];
                c.g[V0][0] = 0x2;
                if !not1 {
                    c.cycles += 2;
                    c.g[S5][0] = 0x71;
                } else {
                    c.cycles += 2;
                    let not2 = c.g[S4][0] != c.g[V0][0];
                    c.g[V1][0] = 0x76;
                    if !not2 {
                        c.cycles += 2;
                        c.g[S5][0] = 0x75;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S4][0] ^ 0x3;
                        if c.g[V0][0] == 0 {
                            c.g[S5][0] = c.g[V1][0];
                        }
                    }
                }
            }
            // Queue the voice record, resolve the gain pointer, push the sound.
            c.cycles += 151;
            c.g[S0][0] = c.lw(c.g[S2][0] as u32);
            c.g[T1][0] = 0x18;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 9);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[V1][0] as u32);
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 9);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[RA][0] = sx(0x0029d03c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x00416210, 0x0029d03c)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A3][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[T0][0] = 0x5a;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[T1][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T2][0] = 0x7f;
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[T3][0] = 0x1;
            c.g[A1][0] = 0x9;
            c.g[A2][0] = 0u64;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[A3][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[T2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[T3][0]);
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S5][0]);
            c.g[RA][0] = sx(0x0029d198);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
            call_ptr(c, 0x00287968, 0x0029d198)?;
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.g[S2][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[RA][0] = sx(0x0029d1b8);
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x002906b8, 0x0029d1b8)?;
            c.g[V1][0] = 0x2;
            let not_kind2 = c.g[S4][0] != c.g[V1][0];
            frame.restore(c, S0, 0x60);
            if not_kind2 {
                break 'done;
            }
            // Kind 2 (the multiplier voice): say the arcade line for x.
            c.cycles += 6;
            c.g[AT][0] = 0x40000000; // 2.0
            c.mtc1(1, 0);
            c.c_eq_s(20, 0);
            let ne = !c.fcc();
            c.g[A0][0] = c.g[S2][0];
            if !ne {
                c.cycles += 5;
                c.g[A1][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0029_d1e8);
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x002a_3eb8, 0x0029_d1e8)?;
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            c.cycles += 6;
            c.g[AT][0] = 0x40400000; // 3.0
            c.mtc1(1, 0);
            c.c_eq_s(20, 0);
            let ne = !c.fcc();
            c.g[A1][0] = c.g[S3][0];
            if !ne {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0029_d210);
                c.g[A2][0] = 0x2;
                call_ptr(c, 0x002a_3eb8, 0x0029_d210)?;
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            c.cycles += 6;
            c.g[AT][0] = 0x40a00000; // 5.0
            c.mtc1(1, 0);
            c.c_eq_s(20, 0);
            let ne = !c.fcc();
            c.g[A0][0] = c.g[S2][0];
            if !ne {
                c.cycles += 5;
                c.g[A1][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0029_d23c);
                c.g[A2][0] = 0x4;
                call_ptr(c, 0x002a_3eb8, 0x0029_d23c)?;
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            c.cycles += 6;
            c.g[AT][0] = 0x41200000; // 10.0
            c.mtc1(1, 0);
            c.c_eq_s(20, 0);
            let ne = !c.fcc();
            c.g[A1][0] = c.g[S3][0];
            if !ne {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0029_d264);
                c.g[A2][0] = 0x8;
                call_ptr(c, 0x002a_3eb8, 0x0029_d264)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC759_CTRL`
    /// (in the machine's own RAM, so other tests' machines are unaffected).
    pub(crate) const OC759_CTRL: u32 = DATA + 0xf_e000;
    pub(crate) const OC759_MAGIC: u32 = 0x7590_44c3;

    /// A gated fake call: logs and clobbers like `fakes::fake`; `zero`
    /// callees then return 0 in `v0`.
    pub(crate) fn oc759_fake_body(c: &mut Cpu, addr: u32, zero: bool) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if zero && c.pc == ra {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc759_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path, $zero:literal;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC759_CTRL) != OC759_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc759_fake_body(c, $addr, $zero)
                }
            )*
            pub(crate) fn oc759_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc759_fakes! {
        oc759_fake_0011b698, OC759_PREV_0011B698, 0x0011_b698, ee::s00::OVERRIDE_0011B698, ee::s00::f_0011b698__generated, false;
        oc759_fake_00287968, OC759_PREV_00287968, 0x0028_7968, ee::s07::OVERRIDE_00287968, ee::s07::f_00287968__generated, false;
        oc759_fake_002906b8, OC759_PREV_002906B8, 0x0029_06b8, ee::s07::OVERRIDE_002906B8, ee::s07::f_002906b8__generated, false;
        oc759_fake_0029f160, OC759_PREV_0029F160, 0x0029_f160, ee::s08::OVERRIDE_0029F160, ee::s08::f_0029f160__generated, true;
        oc759_fake_002e3930, OC759_PREV_002E3930, 0x002e_3930, ee::s09::OVERRIDE_002E3930, ee::s09::f_002e3930__generated, false;
        oc759_fake_0039fb30, OC759_PREV_0039FB30, 0x0039_fb30, ee::s13::OVERRIDE_0039FB30, ee::s13::f_0039fb30__generated, false;
        oc759_fake_0041605c, OC759_PREV_0041605C, 0x0041_605c, ee::s15::OVERRIDE_0041605C, ee::s15::f_0041605c__generated, false;
        oc759_fake_00416210, OC759_PREV_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated, false;
    }

    /// A scrambled machine with the gated fakes on, `pc` at the entry, and an
    /// unwind at one of `calls` fake calls.
    pub(crate) fn oc759_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc759_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC759_CTRL, OC759_MAGIC);
        r
    }

    /// `cDynamicSprayFX_ConfigureForRider` against the generated body: the
    /// player (view index >= 0) and AI tunings, both emitter calls through
    /// fake vtable methods, unwinds in each.
    #[test]
    fn spray_fx_configure_for_rider_matches() {
        fakes::covered(
            ee::s09::f_002e26b8__generated,
            spray_fx_configure_for_rider,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2e26b8, 2);
                c.pc = 0x002e_26b8;
                let this = DATA + 0x2000;
                let rider = DATA + 0x8000;
                let params = DATA + 0x2400;
                let emitters = DATA + 0x4000;
                let t5 = DATA + 0x6000;
                let t6 = DATA + 0x6100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, rider);
                c.bus.write32(this + SPRAY_EMITTERS, emitters);
                c.bus.write32(this + SPRAY_PARAMS, params);
                let view = if seed % 2 == 0 {
                    r.next_u32() & 0x7fff_ffff
                } else {
                    r.next_u32() | 0x8000_0000
                };
                c.bus.write32(rider + RIDER_VIEW_INDEX, view);
                c.bus.write32(emitters + 0xc48, t5);
                c.bus.write32(emitters + 0xe58, t6);
                c.bus.write16(t5 + 0x18, r.next_u32() as u16);
                c.bus.write32(t5 + 0x1c, fakes::FAKE);
                c.bus.write16(t6 + 0x18, r.next_u32() as u16);
                c.bus.write32(t6 + 0x1c, fakes::FAKE + 0x10);
                c.bus.write32(emitters + 0xaa8, r.next_u32());
                c.bus.write32(emitters + 0xcb8, r.next_u32());
            },
        );
    }

    /// Fixture of the two rider-wrapper constructors: the wrapper at `DATA +
    /// 0x3000`, random vtable templates, `inChrg` alternating.
    fn rider_wrapper_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32, tmpl: [u32; 2]) {
        let mut r = oc759_machine(c, seed, salt, 2, entry);
        let this = DATA + 0x3000;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (seed % 2) as u64;
        // Without `inChrg` the rider pointer is already set.
        c.bus.write32(this, DATA + 0x3400);
        fill(c, &mut r, tmpl[0], 0x14);
        fill(c, &mut r, tmpl[1], 0x2c);
        // The wrapper's rider region (what the stores overwrite).
        fill(c, &mut r, DATA + 0x3400, 0x40);
    }

    /// `RiderBase_CtorCheatCode` against the generated body: with and
    /// without `inChrg` (rider ctor call; stack copies of both vtable
    /// templates), then `RiderBase_ClearCheatCode` (a real callee over a
    /// faked memset); unwinds in each call.
    #[test]
    fn rider_base_ctor_cheat_code_matches() {
        check(
            ee::s09::f_002f6238__generated,
            rider_base_ctor_cheat_code,
            80,
            &|c, seed| rider_wrapper_fixture(c, seed, 0x2f6238, 0x002f_6238, [0x4886e0, 0x488728]),
        );
    }

    /// `cPlayerPulseFX_ctor` against the generated body (same shape as
    /// `RiderBase_CtorCheatCode`, other templates, offset and reset call).
    #[test]
    fn player_pulse_fx_ctor_matches() {
        check(
            ee::s09::f_002e3680__generated,
            player_pulse_fx_ctor,
            80,
            &|c, seed| rider_wrapper_fixture(c, seed, 0x2e3680, 0x002e_3680, [0x4889c8, 0x488a10]),
        );
    }

    /// `UIListBox_ctor` against the generated body: base ctor and memset are
    /// fakes (unwinds in each); the row clear loop, flags and the colour
    /// copy are real.
    #[test]
    fn uilistbox_ctor_w244_matches() {
        check(
            ee::s13::f_00399768__generated,
            uilistbox_ctor_w244,
            80,
            &|c, seed| {
                let mut r = oc759_machine(c, seed, 0x399768, 2, 0x0039_9768);
                let this = DATA + 0x5000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x100);
                fill(c, &mut r, 0x004c_66c8, 4);
            },
        );
    }

    /// `PATH_RemoveEvent` against the generated body: slot pointers inside the
    /// event heap (memcpy fake, slot pointer fix-ups of all 17 slots, shift of
    /// the slot array), outside it (callback present or not), the last slot,
    /// unwinds in each call.
    #[test]
    fn path_remove_event_matches() {
        check(
            ee::s14::f_003d1ba8__generated,
            path_remove_event,
            200,
            &|c, seed| {
                let mut r = oc759_machine(c, seed, 0x3d1ba8, 3, 0x003d_1ba8);
                let player = DATA + 0x2_0000;
                c.bus.write32(0x0044_f420, player);
                let callback = if seed % 3 == 0 { 0 } else { fakes::FAKE };
                c.bus.write32(0x0044_f428, callback);
                fill(c, &mut r, player, 0x250);
                let heap = player + 0x11c;
                let end = player + 0x91c;
                for i in 0..17u32 {
                    let v = match r.next_u32() % 6 {
                        0 | 1 | 2 => heap + (r.next_u32() % 0x700 & !0xf),
                        3 => DATA + 0x3_0000,
                        4 => DATA + 0x1_0000,
                        _ => end,
                    };
                    c.bus.write32(player + 0xdc + 4 * i, v);
                }
                c.bus.write32(end, r.next_u32() % 40);
                let idx = if seed % 5 == 0 { 15 } else { r.next_u32() % 16 };
                c.g[A0][0] = idx as u64;
            },
        );
    }

    /// `SSXAudioSystem_PlayMultiplierSfx` against the generated body: with and
    /// without a (local human, other) object, every `kind`, the voice record
    /// and its parallel arrays, kind 2 with x = 2, 3, 5, 10 and other, and
    /// unwinds in each call (memset, gain pointer, push, arcade say).
    #[test]
    fn ssx_audio_system_play_multiplier_sfx_matches() {
        check(
            ee::s08::f_0029ced8__generated,
            ssx_audio_system_play_multiplier_sfx,
            300,
            &|c, seed| {
                let mut r = oc759_machine(c, seed, 0x29ced8, 4, 0x0029_ced8);
                let sys = DATA + 0x1_0000;
                let state = DATA + 0x1_2000; // *(sys): voice state, +0x1d8.. tables
                let other = DATA + 0x1_6000; // *(sys+0x118)
                let ostate = DATA + 0x1_8000; // *other
                fill(c, &mut r, state + 0x1d8, 0x20);
                c.bus.write32(sys, state);
                c.bus.write32(sys + 0x118, other);
                c.bus.write32(other, ostate);
                c.bus.write32(state + 0x1f4, r.next_u32() % 6);
                c.bus.write32(ostate + 0x1f4, r.next_u32() % 6);
                // Table pointers of the voice record (each in its own arena).
                let tables = [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ];
                for (k, off) in tables.iter().enumerate() {
                    c.bus
                        .write32(state + 0x1d8 + off, DATA + 0x3_0000 + 0x1000 * k as u32);
                }
                c.bus.write32(ostate + 0x214, DATA + 0x5_0000);
                c.bus.write32(sys + 4, DATA + 0x6_0000);
                c.bus.write32(sys + 8, DATA + 0x6_1000);
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = [0u64, 1, 2, 3, 4, 7][(seed % 6) as usize];
                // The object: none, local human, non-human, human but not local.
                let obj = DATA + 0x1_a000;
                match (seed / 6) % 4 {
                    0 => c.g[A2][0] = 0,
                    1 => {
                        c.g[A2][0] = sx(obj);
                        c.bus.write32(obj + 0x874, 1);
                        c.bus.write32(obj + 0x87c, 1);
                    }
                    2 => {
                        c.g[A2][0] = sx(obj);
                        c.bus.write32(obj + 0x874, 0);
                    }
                    _ => {
                        c.g[A2][0] = sx(obj);
                        c.bus.write32(obj + 0x874, 1);
                        c.bus.write32(obj + 0x87c, 0);
                    }
                }
                c.f[12] = [2.0f32, 3.0, 5.0, 10.0, 7.5][((seed / 24) % 5) as usize].to_bits();
            },
        );
    }
}
