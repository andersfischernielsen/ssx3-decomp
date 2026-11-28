//! Staging module oc1575 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1575`.

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
    (0x002e_bd88, transition_effect_ctor_random, "TransitionEffect_ctorRandom"),
    (0x002f_0fe8, rider_base_dtor_attack_fx, "RiderBase_DtorAttackFX"),
    (0x002f_1768, c_attached_dynamic_emitter_ctor, "cAttachedDynamicEmitter_ctor"),
    (0x002f_17a8, c_attached_dynamic_emitter_v04, "cAttachedDynamicEmitter_v04"),
    (0x002f_2130, c_attached_dynamic_emitter_create, "cAttachedDynamicEmitter_Create"),
    (0x002f_3618, c_splash_manager_dtor, "cSplashManager_dtor"),
    (0x002f_5230, world_light_man_dtor, "WorldLightMan_dtor"),
];

/// `TransitionEffect_ctorRandom` (0x2ebd88, `this, const tARGB& rect, float
/// x (f12), float y (f13)`): copies the 16-byte rect at `a1` to `this+4`,
/// runs the base constructor `TransitionEffect_ctor` (0x2e4d70), then stores
/// the vtable at `this` and the two floats at `+0x14` and `+0x18`.
pub fn transition_effect_ctor_random(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 13);
        let src = c.g[A1][0] as u32;
        c.ldl(V0, src.wrapping_add(0x7));
        c.ldr(V0, src);
        c.ldl(V1, src.wrapping_add(0xf));
        c.ldr(V1, src.wrapping_add(0x8));
        let at = frame.at(c, 0);
        c.sdl(V0, at.wrapping_add(0x7));
        c.sdr(V0, at);
        c.sdl(V1, at.wrapping_add(0xf));
        c.mov_s(20, 12);
        c.sdr(V1, at.wrapping_add(0x8));
        Callee {
            addr: 0x002e_4d70,
            f: ee::s09::f_002e4d70,
        }
        .call(c, 0x002e_bdcc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x490000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7f80));
        frame.restore_ra(c, 0x10);
        let vtable = c.g[V1][0];
        Addr(this).sw(c, vtable);
        let at = frame.at(c, 0);
        c.ldl(V1, at.wrapping_add(0x7));
        c.ldr(V1, at);
        c.ldl(A0, at.wrapping_add(0xf));
        c.ldr(A0, at.wrapping_add(0x8));
        c.sdl(V1, this.wrapping_add(0xb));
        c.sdr(V1, this.wrapping_add(0x4));
        c.sdl(A0, this.wrapping_add(0x13));
        c.sdr(A0, this.wrapping_add(0xc));
        c.swc1(20, this.wrapping_add(0x14));
        c.swc1(21, this.wrapping_add(0x18));
        frame.restore(c, S0, 0x20);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Gate word for the wave's test fakes (in the machine's RAM).
#[cfg(test)]
const OC1575_CTRL: u32 = crate::difftest::DATA + 0x20_0000;
#[cfg(test)]
const OC1575_MAGIC: u32 = 0x1575_1575;

/// `RiderBase_DtorAttackFX(RiderBase* this, int flags)` (0x2f0fe8): the
/// attack-FX part of the rider destructor. With `flags == 0` it copies the
/// two static tables at 0x4887d8 and 0x488820 onto the stack, points the
/// object's `+0x6e8` and `+0x6c0` words at them and sets two shorts from the
/// object's `+0xb0` offset; then bit 1 of `flags` runs `Rider_dtor`
/// (0x11b978) and bit 0 deletes the object (0x317e50).
pub fn rider_base_dtor_attack_fx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x130);
        c.g[V0][0] = 0x490000;
        frame.save(c, S0, 0x120);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7828));
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = 0x460000;
        c.g[A3][0] = c.g[V0][0];
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = Addr(this).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
        let v = c.g[A2][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x6e8)).sw(c, v);
        c.g[V1][0] = 0x490000;
        c.g[A1][0] = Addr(this).lw(c);
        c.g[T2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x77e0));
        let v = c.g[A0][0];
        Addr((c.g[A1][0] as u32).wrapping_add(0x6d0)).sw(c, v);
        c.g[V0][0] = Addr(this).lw(c);
        let v = c.g[V1][0];
        Addr((c.g[V0][0] as u32).wrapping_add(0x6c0)).sw(c, v);

        // Copy block (only when flags == 0).
        'copy: {
            if c.g[S1][0] != 0 {
                break 'copy;
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[A2][0];
            c.g[V1][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
            c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            loop {
                // Copy 0x20 bytes of the first table per pass.
                c.cycles += 11;
                c.g[A0][0] = Addr(c.g[A1][0] as u32).ld(c);
                c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).ld(c);
                c.g[T1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).ld(c);
                c.g[T3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).ld(c);
                let dst = c.g[V1][0] as u32;
                let v = c.g[A0][0];
                Addr(dst).sd(c, v);
                let v = c.g[A2][0];
                Addr(dst.wrapping_add(0x8)).sd(c, v);
                let v = c.g[T1][0];
                Addr(dst.wrapping_add(0x10)).sd(c, v);
                let v = c.g[T3][0];
                Addr(dst.wrapping_add(0x18)).sd(c, v);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                let more = c.g[A1][0] != c.g[V0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002f_104c);
                    }
                    continue;
                }
                c.cycles += 14;
                c.g[V0][0] = Addr(this).lw(c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7828));
                c.g[A2][0] = Addr(c.g[A1][0] as u32).ld(c);
                let dst = c.g[V1][0] as u32;
                let v = c.g[A2][0];
                Addr(dst).sd(c, v);
                let v = c.g[SP][0];
                Addr((c.g[V0][0] as u32).wrapping_add(0x6e8)).sw(c, v);
                c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x77e0));
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x8)).lhu(c);
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = Addr(this).lw(c);
                c.g[A1][0] = c.g[T0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb0));
                c.g[A3][0] = sx(this.wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                let v = c.g[V1][0];
                Addr((c.g[SP][0] as u32).wrapping_add(0x8)).sh(c, v);
                // Copy the second table up to 0x4888c0.
                loop {
                    c.cycles += 11;
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
                    c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).ld(c);
                    c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x10)).ld(c);
                    c.g[T3][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).ld(c);
                    let dst = c.g[A1][0] as u32;
                    let v = c.g[V0][0];
                    Addr(dst).sd(c, v);
                    let v = c.g[V1][0];
                    Addr(dst.wrapping_add(0x8)).sd(c, v);
                    let v = c.g[A0][0];
                    Addr(dst.wrapping_add(0x10)).sd(c, v);
                    let v = c.g[T3][0];
                    Addr(dst.wrapping_add(0x18)).sd(c, v);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let more = c.g[A2][0] != c.g[T1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002f_10b0);
                        }
                        continue;
                    }
                    c.cycles += 10;
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).ld(c);
                    c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x8)).ld(c);
                    let dst = c.g[A1][0] as u32;
                    let v = c.g[V0][0];
                    Addr(dst).sd(c, v);
                    let v = c.g[V1][0];
                    Addr(dst.wrapping_add(0x8)).sd(c, v);
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x77e0));
                    c.g[A0][0] = Addr(this).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lhu(c);
                    let v = c.g[T0][0];
                    Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).sw(c, v);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    let v = c.g[V1][0];
                    Addr((c.g[SP][0] as u32).wrapping_add(0x58)).sh(c, v);
                    break 'copy;
                }
            }
        }

        // Flags: bit 1 runs Rider_dtor, bit 0 deletes the object.
        c.cycles += 3;
        let no_dtor = (c.g[S1][0] & 0x2) == 0;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if !no_dtor {
            c.cycles += 4;
            c.g[A0][0] = Addr(this).lw(c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0011_b978, 0x002f_111c)?;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x120);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002f_1130)?;
            frame.restore(c, S0, 0x120);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `cAttachedDynamicEmitter::cAttachedDynamicEmitter(this)` (0x2f1768): runs
/// the base constructor (0x370b60), then clears the emitter's counters and
/// stores its vtable at `+0x1f8`.
pub fn c_attached_dynamic_emitter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0037_0b60, 0x002f_177c)?;
        c.g[V1][0] = 0x490000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e18));
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(0x200)).sw(c, 0);
        let vtable = c.g[V1][0];
        Addr(this.wrapping_add(0x1f8)).sw(c, vtable);
        Addr(this.wrapping_add(0x204)).sw(c, 0);
        Addr(this.wrapping_add(0x208)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAttachedDynamicEmitter::v04(this)` (0x2f17a8): stores the vtable at
/// `+0x1f8` and calls the virtual-slot helper 0x370c08.
pub fn c_attached_dynamic_emitter_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7e18));
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x1f8)).sw(c, vtable);
        call_ptr(c, 0x0037_0c08, 0x002f_17c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cAttachedDynamicEmitter::Create(this, a1, a2)` (0x2f2130): finds the
/// first empty slot of the ten at `this+0x6c` (counting the used ones in
/// s1). In an empty slot it allocates a 0x210-byte emitter, constructs it
/// and wires it up (calls through its vtable); returns the slot count, or
/// -1 when all ten slots are full.
pub fn c_attached_dynamic_emitter_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x110;
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.mult(0, S3, A1, V0); // s3 = a1 * 0x110
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = 0x480000;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 31;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7cb0));
                c.g[A0][0] = 0x210;
                call_ptr(c, 0x0031_7d70, 0x002f_2180)?;
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x002f_1768,
                    f: ee::s09::f_002f1768__generated,
                }
                .call(c, 0x002f_2188)?;
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xac0)).lw(c);
                let slot = c.g[S0][0] as u32;
                let v = c.g[V0][0];
                Addr(slot).sw(c, v);
                let v = c.g[S2][0];
                Addr((c.g[V0][0] as u32).wrapping_add(0x200)).sw(c, v);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                c.g[V0][0] = Addr(slot).lw(c);
                Addr((c.g[V0][0] as u32).wrapping_add(0x204)).sw(c, 0);
                c.g[V1][0] = Addr(slot).lw(c);
                let v = c.g[A0][0];
                Addr((c.g[V1][0] as u32).wrapping_add(0x20c)).sw(c, v);
                c.g[A0][0] = Addr(slot).lw(c);
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1f8)).lw(c);
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x20c)).lw(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x58));
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
                let target = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, target, 0x002f_21c8)?;
                c.g[A1][0] = Addr(slot).lw(c);
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1f8)).lw(c);
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002f_21e0)?;
                c.g[V0][0] = c.g[S1][0];
                break;
            }
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002f_2160);
                }
                continue;
            }
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            break;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSplashManager::~cSplashManager(this)` (0x2f3618): sets the vtable at
/// `+0xc` and calls the base destructor (0x3546c8).
pub fn c_splash_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7dd0));
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, vtable);
        call_ptr(c, 0x0035_46c8, 0x002f_3634)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WorldLightMan::~WorldLightMan(this, flags)` (0x2f5230): when the
/// owner at `+4` is set, calls its virtual slot 3 (`+0xc` of its vtable);
/// then `FreeOverlapLists` (0x2f53b0), and bit 0 of `flags` deletes the
/// object through 0x317e50.
pub fn world_light_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(this.wrapping_add(0x4)).lw(c);
        let owner_null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !owner_null {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x4)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002f_5268)?;
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002f_53b0,
            f: ee::s09::f_002f53b0,
        }
        .call(c, 0x002f_5270)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002f_5284)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, fake, fall_back, machine_rt};
    use crate::difftest::{check, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1575_CTRL) != OC1575_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $target, ra);
            }
        };
    }

    gated_fake!(
        oc1575_fake_rider_dtor,
        OC1575_PREV_RIDER_DTOR,
        0x0011_b978,
        ee::s00::f_0011b978__generated
    );

    gated_fake!(
        oc1575_fake_delete,
        OC1575_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    gated_fake!(
        oc1575_fake_free_overlap,
        OC1575_PREV_FREE_OVERLAP,
        0x002f_53b0,
        ee::s09::f_002f53b0__generated
    );

    /// Install the gated fakes (once per process; gate off they fall back).
    fn oc1575_hook() {
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc1575_fake_rider_dtor,
            &OC1575_PREV_RIDER_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1575_fake_delete,
            &OC1575_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F53B0),
            oc1575_fake_free_overlap,
            &OC1575_PREV_FREE_OVERLAP,
        );
    }

    #[test]
    fn transition_effect_ctor_random_matches() {
        check(
            ee::s09::f_002ebd88__generated,
            transition_effect_ctor_random,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002e_bd88, 0);
                let this = DATA + 0x1000;
                let rect = DATA + 0x2000;
                fill(c, &mut r, rect, 4);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(rect);
                c.f[12] = r.next_u32();
                c.f[13] = r.next_u32();
            },
        );
    }

    #[test]
    fn rider_base_dtor_attack_fx_matches() {
        oc1575_hook();
        check(
            ee::s09::f_002f0fe8__generated,
            rider_base_dtor_attack_fx,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002f_0fe8, 0);
                c.bus.write32(OC1575_CTRL, OC1575_MAGIC);
                let this = DATA + 0x1000;
                let obj = DATA + 0x2000;
                c.bus.write32(this, obj);
                fill(c, &mut r, obj, 0x400);
                c.g[A0][0] = sx(this);
                // Bit 0 deletes, bit 1 runs Rider_dtor; 0 copies the tables.
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn c_attached_dynamic_emitter_ctor_matches() {
        check(
            ee::s09::f_002f1768__generated,
            c_attached_dynamic_emitter_ctor,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x002f_1768, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_attached_dynamic_emitter_v04_matches() {
        check(
            ee::s09::f_002f17a8__generated,
            c_attached_dynamic_emitter_v04,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x002f_17a8, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_attached_dynamic_emitter_create_matches() {
        check(
            ee::s09::f_002f2130__generated,
            c_attached_dynamic_emitter_create,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x002f_2130, 0);
                let this = DATA + 0x1000;
                // The first `seed % 11` slots are used: 0..=10 passes.
                for i in 0..10u32 {
                    let slot = if i < seed % 11 {
                        DATA + 0x5000 + 0x40 * i
                    } else {
                        0
                    };
                    c.bus.write32(this + 0x6c + 4 * i, slot);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 0xff);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn c_splash_manager_dtor_matches() {
        check(
            ee::s09::f_002f3618__generated,
            c_splash_manager_dtor,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x002f_3618, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn world_light_man_dtor_matches() {
        oc1575_hook();
        check(
            ee::s09::f_002f5230__generated,
            world_light_man_dtor,
            200,
            &|c, seed| {
                let _r = machine_rt(c, seed, 0x002f_5230, 0);
                c.bus.write32(OC1575_CTRL, OC1575_MAGIC);
                let this = DATA + 0x1000;
                let owner = DATA + 0x2000;
                let vtable = DATA + 0x3000;
                // Owner null on every third seed; otherwise a virtual call
                // to an address that is not a function (the miss fake).
                c.bus
                    .write32(this + 0x4, if seed % 3 == 0 { 0 } else { owner });
                c.bus.write32(owner + 0x4, vtable);
                c.bus.write16(vtable + 0x8, (seed % 5) as u16);
                c.bus.write32(vtable + 0xc, 0x00c0_ffe0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
