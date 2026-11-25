//! Staging module oc627 (wave g07.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc627`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0014_9a88,
    be_library_reset_player_profile,
    "BELibrary_ResetPlayerProfile",
)];

/// `cBEProfileLib_init` (0x1567b8): resets one rider's profile characters.
const PROFILE_LIB_INIT: Callee = Callee {
    addr: 0x0015_67b8,
    f: ee::s01::f_001567b8,
};
/// `BENewPlayerInt_GetInstance` (0x145970).
const PLAYER_INT_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_5970,
    f: ee::s01::f_00145970,
};
/// `BECharInt_GetInstance` (0x147f78).
const CHAR_INT_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_7f78,
    f: ee::s01::f_00147f78,
};
/// `BEBAGTInt_GetInstance` (0x14f900).
const BAGT_INT_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_f900,
    f: ee::s01::f_0014f900,
};

/// The tail shared by the three interfaces: get the singleton, then call its
/// vtable method `{int16 delta; fn}` (at +0x10 / +0x14) on `this + delta`.
fn reset_profile_interface(c: &mut Cpu, get: Callee, get_ret: u32, method_ret: u32) -> Flow {
    get.call(c, get_ret)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let method = c.g[A1][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, method_ret)?;
    Ok(())
}

/// `void BELibrary_ResetPlayerProfile(BELibrary *self, int idx)` (0x149a88):
/// clears the rider slot `cBELibrary_playerDB[idx]` (0x1c bytes), marks it as
/// dual-shock character 4, formats its user name from the locale's label
/// (`sprintf(entry, label, idx + 1)`), clears that rider's profile characters,
/// then resets the player, character and BAGT interfaces' per-rider caches.
/// Frame 0x90: saves ra and s0-s3.
pub fn be_library_reset_player_profile(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 76;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 19, 0x50);
        frame.save(c, 17, 0x70);
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        frame.save_ra(c, 0x40);

        // idx = (signed char)a1; entry = 0x535b20 + idx * 0x1c.
        let idx = (c.g[A1][0] as u8 as i8) as i32;
        c.g[S3][0] = sx(idx as u32);
        c.g[S1][0] = 0x1c;
        c.mult(0, 17, 19, 17);
        c.g[V0][0] = 0x0053_5b20;
        c.g[A2][0] = 0x1c;
        c.g[A1][0] = 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let entry = c.g[S1][0] as u32;

        // memset(entry, 0, 0x1c).
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6210, 0x0014_9acc)?;

        // entry->flags |= 2; entry->character = 4.
        c.g[V1][0] = c.lw(entry.wrapping_add(0x10));
        c.g[V0][0] = 4;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = 0x0046_0000;
        c.g[V1][0] |= 2;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d98));
        c.sw(entry.wrapping_add(0x10), c.g[V1][0]);
        c.sb(entry.wrapping_add(0x11), c.g[V0][0]);

        // o = world->locale; vt = o->vtable; label method on this = o + vt->delta.
        c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
        c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
        c.g[V0][0] = c.lh((c.g[S2][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, 0x0031_7670, 0x0014_9b00)?;

        // label = o->vt->slot(this, hash); FormatTagged(buf = sp, label);
        // sprintf(entry, buf, idx + 1).
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.g[S0][0];
        let label_method = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, label_method, 0x0014_9b10)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002c_2580, 0x0014_9b1c)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x0014_9b2c)?;

        // cBEProfileLib_init(cBELibrary_profileLib[idx], 1); the table is at
        // 0x4a6ca8 with 0x9b50-byte entries.
        c.g[V1][0] = 0x9b50;
        c.g[A0][0] = 0x004a_0000;
        c.mult(0, 19, 19, 3);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        PROFILE_LIB_INIT.call(c, 0x0014_9b48)?;

        reset_profile_interface(c, PLAYER_INT_GET_INSTANCE, 0x0014_9b50, 0x0014_9b64)?;
        reset_profile_interface(c, CHAR_INT_GET_INSTANCE, 0x0014_9b6c, 0x0014_9b80)?;
        reset_profile_interface(c, BAGT_INT_GET_INSTANCE, 0x0014_9b88, 0x0014_9b9c)?;

        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fall_back, hook, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for the singleton getter fakes (a magic word in the machine's RAM).
    const OC627_CTRL: u32 = DATA + 0x3f00;
    const OC627_MAGIC: u32 = 0x6267_0627;
    /// The three singletons and their shared vtable (`{delta 0, method FAKE}`
    /// at +0x10 / +0x14).
    const OC627_PLAYER_OBJ: u32 = DATA + 0x3900;
    const OC627_CHAR_OBJ: u32 = DATA + 0x3940;
    const OC627_BAGT_OBJ: u32 = DATA + 0x3980;
    const OC627_VTABLE: u32 = DATA + 0x39c0;

    static OC627_PREV_PLAYER_GET: AtomicUsize = AtomicUsize::new(0);
    static OC627_PREV_CHAR_GET: AtomicUsize = AtomicUsize::new(0);
    static OC627_PREV_BAGT_GET: AtomicUsize = AtomicUsize::new(0);

    /// A getter that returns `obj` while the gate is set, else the body it replaced.
    fn oc627_getter(c: &mut Cpu, prev: &AtomicUsize, generated: fn(&mut Cpu), obj: u32) {
        if c.bus.read32(OC627_CTRL) != OC627_MAGIC {
            return fall_back(c, prev, generated);
        }
        c.g[V0][0] = obj as u64;
        c.pc = c.g[RA][0] as u32;
    }

    fn oc627_player_get(c: &mut Cpu) {
        oc627_getter(
            c,
            &OC627_PREV_PLAYER_GET,
            ee::s01::f_00145970__generated,
            OC627_PLAYER_OBJ,
        )
    }

    fn oc627_char_get(c: &mut Cpu) {
        oc627_getter(
            c,
            &OC627_PREV_CHAR_GET,
            ee::s01::f_00147f78__generated,
            OC627_CHAR_OBJ,
        )
    }

    fn oc627_bagt_get(c: &mut Cpu) {
        oc627_getter(
            c,
            &OC627_PREV_BAGT_GET,
            ee::s01::f_0014f900__generated,
            OC627_BAGT_OBJ,
        )
    }

    #[test]
    fn be_library_reset_player_profile_matches() {
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145970),
            oc627_player_get,
            &OC627_PREV_PLAYER_GET,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147F78),
            oc627_char_get,
            &OC627_PREV_CHAR_GET,
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014F900),
            oc627_bagt_get,
            &OC627_PREV_BAGT_GET,
        );
        covered(
            ee::s01::f_00149a88__generated,
            be_library_reset_player_profile,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0014_9a88, 3);
                let world = DATA + 0x3000;
                let locale = DATA + 0x3100;
                let vt = DATA + 0x3200;
                let text = DATA + 0x2000;
                // World at gp-0x848; world->+0x8c is the locale, locale->+4 its vtable.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, locale);
                c.bus.write32(locale + 4, vt);
                let delta = (r.next_u32() as i16) & !3;
                c.bus.write16(vt + 0x20, delta as u16);
                c.bus.write32(vt + 0x24, FAKE);
                // The label method (the first fake call) returns a format string.
                for (i, b) in b"Rider %d\0".iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                c.bus.write32(LOG_RESULTS, text);
                // The singletons (gated fakes) and their shared vtable.
                c.bus.write32(OC627_CTRL, OC627_MAGIC);
                c.bus.write32(OC627_VTABLE + 0x10, 0);
                c.bus.write32(OC627_VTABLE + 0x14, FAKE);
                for obj in [OC627_PLAYER_OBJ, OC627_CHAR_OBJ, OC627_BAGT_OBJ] {
                    c.bus.write32(obj + 0xc, OC627_VTABLE);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }
}
