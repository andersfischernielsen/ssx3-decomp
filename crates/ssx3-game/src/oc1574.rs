//! Staging module oc1574 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1574`.

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
    (0x002d_3f78, menu_sys_message_page_dtor, "MenuSysMessagePage_dtor"),
    (0x002d_3fd8, c_ps2_console_config_v01, "cPS2ConsoleConfig_v01"),
    (0x002d_4008, menu_sys_page_sort_compare, "MenuSysPage_SortCompare"),
    (0x002d_4030, c_ps2_console_config_base_dtor, "cPS2ConsoleConfig_BaseDtor"),
    (0x002d_4060, ssx_stats_man_global_ctors, "SSXStatsMan_GlobalCtors"),
    (0x002d_48d0, visual_effects_global_ctors, "VisualEffects_GlobalCtors"),
    (0x002d_bd10, fx_loader_construct, "FXLoader_ctor"),
    (0x002d_c168, desc_key4_sort_compare, "desc_key4_sort_compare"),
];

/// `MenuSysMessagePage` destructor body (0x2d3f78): destroys the three item
/// members at +0x164, +0x14c and +0x130 (`MenuSysItem_dtor(item, 2)`), then
/// the base page through `MenuSysPage_dtor(this, flags)`.
pub fn menu_sys_message_page_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        for (off, ret_to) in [
            (0x164u32, 0x002d_3f9c),
            (0x14c, 0x002d_3fa8),
            (0x130, 0x002d_3fb4),
        ] {
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(off));
            c.g[A1][0] = 0x2;
            Callee {
                addr: 0x002c_a280,
                f: ee::s09::MenuSysItem_dtor,
            }
            .call(c, ret_to)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        Callee {
            addr: 0x002c_aa80,
            f: ee::s09::MenuSysPage_dtor,
        }
        .call(c, 0x002d_3fc0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2ConsoleConfig` deleting destructor (0x2d3fd8): the vtable 0x487480 at
/// +0x34, then `operator delete(this)` when bit 0 of the flags is set.
pub fn c_ps2_console_config_v01(c: &mut Cpu) {
    crate::game::visualfx::worldlightman::deleting_dtor(c, 0x34, 0x0048_7480, 0x002d_3ffc);
}

/// `cPS2ConsoleConfig` base destructor (0x2d4030): the same body as
/// [`c_ps2_console_config_v01`], returning from the delete to 0x2d4054.
pub fn c_ps2_console_config_base_dtor(c: &mut Cpu) {
    crate::game::visualfx::worldlightman::deleting_dtor(c, 0x34, 0x0048_7480, 0x002d_4054);
}

/// `MenuSysPage_SortCompare(a, b)` (0x2d4008): calls the comparator stored in
/// the global at `gp + 0x23cc` on the two items `*a` and `*b`.
pub fn menu_sys_page_sort_compare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x23cc)).lw(c);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
        call_ptr(c, target, 0x002d_4020)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXStatsMan` global constructor (0x2d4060): `SSXStatsMan_StaticInitDestroy(1, 0xffff)`.
pub fn ssx_stats_man_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x002d_2988,
            f: ee::s09::SSXStatsMan_StaticInitDestroy,
        }
        .call(c, 0x002d_4074)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VisualEffects` global constructor (0x2d48d0): `VisualEffects_StaticInit(1, 0xffff)`.
pub fn visual_effects_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x002d_40c0,
            f: ee::s09::VisualEffects_StaticInit,
        }
        .call(c, 0x002d_48e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Texture files the constructor loads (`FILE_load(name, 0x100)`).
const FXLOADER_FILE_LOAD: u32 = 0x003e_18c8;
/// `SHAPE_locatez(file, entry)`: the effect shape of an entry in a file.
const FXLOADER_SHAPE_LOCATEZ: u32 = 0x003b_3900;
/// `MEM_free(file)`.
const FXLOADER_MEM_FREE: u32 = 0x0025_23a8;

/// `cFXLoader::cFXLoader(this, particles_only)` (0x2dbd10): loads the particle
/// texture file (and the effects file unless `particles_only`), binds each of
/// the 81 effect shapes to a texture id in the texture manager, frees the
/// files and fills the ten effect texture slots. Returns `this`.
pub fn fx_loader_construct(c: &mut Cpu) {
    guest(c, fxloader_body);
}

fn fxloader_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0xc0);
    c.g[V0][0] = 0x1;
    frame.save(c, S4, 0x70);
    frame.save(c, S0, 0xb0);
    c.g[S4][0] = 0x2;
    frame.save(c, S1, 0xa0);
    if c.g[A1][0] != 0 {
        c.g[S4][0] = c.g[V0][0];
    }
    frame.save(c, S2, 0x90);
    c.g[S0][0] = 0;
    frame.save(c, S3, 0x80);
    frame.save(c, S5, 0x60);
    frame.save(c, S6, 0x50);
    frame.save(c, S7, 0x40);
    frame.save(c, FP, 0x30);
    frame.save_ra(c, 0x20);
    let this = c.g[A0][0];
    Addr(frame.at(c, 0x10)).sw(c, this);

    // Load the files: index 0 (particles) to sp+0, index 1 (effects) to sp+4.
    if c.g[S4][0] != 0 {
        c.cycles += 3;
        c.g[S3][0] = 0x1;
        c.g[S2][0] = 0x0048_0000;
        c.g[S1][0] = 0x0048_0000;
        loop {
            c.cycles += 2;
            // The arms share the `'b_2dbd94` tail (+1, next index) except the
            // plain increment of the last arm, which skips it.
            let mut shared_tail = true;
            if c.g[S0][0] == 0 {
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7918));
                c.cycles += 4;
                c.g[A1][0] = 0x100;
                call_ptr(c, FXLOADER_FILE_LOAD, 0x002d_bd80)?;
                let handle = c.g[V0][0];
                Addr(frame.at(c, 0)).sw(c, handle);
            } else {
                c.cycles += 2;
                let second = c.g[S0][0] == c.g[S3][0];
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7938));
                if second {
                    c.cycles += 3;
                    c.g[A1][0] = 0x100;
                    call_ptr(c, FXLOADER_FILE_LOAD, 0x002d_bd90)?;
                    let handle = c.g[V0][0];
                    Addr(frame.at(c, 4)).sw(c, handle);
                } else {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    shared_tail = false;
                }
            }
            if shared_tail {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_bd60);
            }
        }
    }

    // Bind the 81 effect shapes to texture ids in the texture manager.
    c.cycles += 9;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
    c.g[V0][0] = 0x0049_0000;
    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e50));
    c.g[S1][0] = 0;
    c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10b0));
    c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf50));
    c.g[S7][0] = 0xc;
    c.g[A1][0] = 0;
    loop {
        c.cycles += 4;
        c.g[S0][0] = 0;
        c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
        // Search the files for shape k (index i while i < n).
        loop {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            let none_left = c.g[V0][0] == 0;
            c.mult(0, 2, 17, 23);
            if none_left {
                break;
            }
            c.cycles += 8;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            call_ptr(c, FXLOADER_SHAPE_LOCATEZ, 0x002d_bdf8)?;
            c.g[A1][0] = c.g[V0][0];
            if c.g[A1][0] == 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_bdd8);
                }
                continue;
            }
            break;
        }
        // The shape's texture id (-1 when it was not found).
        c.cycles += 2;
        let not_found = c.g[A1][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !not_found {
            c.cycles += 14;
            c.mult(0, 6, 17, 23);
            c.g[T3][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2a90)).lw(c);
            c.g[V0][0] = 0x0049_0000;
            c.g[T0][0] = 0x1;
            c.g[V1][0] = Addr((c.g[T3][0] as u32).wrapping_add(0x10d8)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e48));
            c.g[T1][0] = u64::MAX;
            c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x170)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x174)).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[A3][0] = Addr(c.g[T2][0] as u32).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x002d_be44)?;
        }
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S1][0] = c.g[S3][0];
        let tex_id = c.g[V0][0];
        Addr(c.g[V1][0] as u32).sw(c, tex_id);
        c.g[V0][0] = ((c.g[S1][0] as i64) < 81) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = 0;
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002d_bdc8);
            }
            continue;
        }
        break;
    }

    // Free the files.
    c.cycles += 2;
    let with_effects = c.g[S4][0] != 0;
    c.g[A0][0] = c.g[FP][0];
    if with_effects {
        c.cycles += 3;
        c.g[S1][0] = c.g[S4][0];
        c.g[S0][0] = c.g[SP][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        loop {
            c.cycles += 5;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            call_ptr(c, FXLOADER_MEM_FREE, 0x002d_be7c)?;
            if c.g[S1][0] != 0 {
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002d_be70);
                }
                continue;
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[FP][0];
            break;
        }
    }

    // Copy the ten effect texture slots (effect map 71..80) into dst[0..9].
    c.cycles += 2;
    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x11c));
    c.g[A1][0] = 0x9;
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        let slot = c.g[V0][0];
        Addr(c.g[A0][0] as u32).sw(c, slot);
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x002d_be90);
            }
            continue;
        }
        break;
    }
    c.cycles += 13;
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    frame.restore(c, S0, 0xb0);
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore(c, S4, 0x70);
    frame.restore(c, S5, 0x60);
    frame.restore(c, S6, 0x50);
    frame.restore(c, S7, 0x40);
    frame.restore(c, FP, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// The comparator of the descending key sort (0x2dc168): compares the word at
/// +4 of `a0` and `a1`; `1` when `a < b`, `0` when equal, `-1` otherwise.
pub fn desc_key4_sort_compare(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let less = c.g[V0][0] != 0;
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if less {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0u64;
        }
    }
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, check_lohi, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words and the magic that opens them (unique fixture addresses).
    const OC1574_MAGIC: u32 = 0x1574_0001;
    const OC1574_ITEM_CTRL: u32 = DATA + 0x20_1000;
    const OC1574_PAGE_CTRL: u32 = DATA + 0x20_1004;
    const OC1574_FILE_CTRL: u32 = DATA + 0x20_1008;
    const OC1574_SHAPE_CTRL: u32 = DATA + 0x20_100c;
    const OC1574_FREE_CTRL: u32 = DATA + 0x20_1010;
    const OC1574_SHAPE_SALT: u32 = DATA + 0x20_1014;
    const OC1574_INIT_CTRL: u32 = DATA + 0x20_1018;
    const OC1574_VFX_CTRL: u32 = DATA + 0x20_101c;

    static OC1574_PREV_ITEM: AtomicUsize = AtomicUsize::new(0);
    static OC1574_PREV_PAGE: AtomicUsize = AtomicUsize::new(0);
    static OC1574_PREV_FILE: AtomicUsize = AtomicUsize::new(0);
    static OC1574_PREV_SHAPE: AtomicUsize = AtomicUsize::new(0);
    static OC1574_PREV_FREE: AtomicUsize = AtomicUsize::new(0);
    static OC1574_PREV_INIT: AtomicUsize = AtomicUsize::new(0);
    static OC1574_PREV_VFX: AtomicUsize = AtomicUsize::new(0);

    /// `MenuSysItem_dtor`: a logged call while its gate is on.
    fn oc1574_fake_item_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1574_ITEM_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_ITEM, ee::s09::f_002ca280__generated);
        }
        fakes::fake_call(c, 0x002c_a280);
    }

    /// `MenuSysPage_dtor`: a logged call while its gate is on.
    fn oc1574_fake_page_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1574_PAGE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_PAGE, ee::s09::f_002caa80__generated);
        }
        fakes::fake_call(c, 0x002c_aa80);
    }

    /// `FILE_load`: a logged call returning a handle while its gate is on.
    fn oc1574_fake_file(c: &mut Cpu) {
        if c.bus.read32(OC1574_FILE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_FILE, ee::s14::f_003e18c8__generated);
        }
        fakes::fake_call(c, FXLOADER_FILE_LOAD);
    }

    /// `SHAPE_locatez`: found or not found, by a hash of the entry and a salt
    /// per seed, so both arms of the search run.
    fn oc1574_fake_shape(c: &mut Cpu) {
        if c.bus.read32(OC1574_SHAPE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_SHAPE, ee::s13::f_003b3900__generated);
        }
        let entry = c.g[A1][0] as u32;
        let salt = c.bus.read32(OC1574_SHAPE_SALT) as u64;
        let found = fakes::mix(entry as u64 ^ salt) % 3 != 0;
        fakes::fake_call(c, FXLOADER_SHAPE_LOCATEZ);
        c.g[V0][0] = if found {
            sx(DATA + 0x20_2000 + (entry & 0xfff))
        } else {
            0
        };
    }

    /// `MEM_free`: a logged call while its gate is on.
    fn oc1574_fake_free(c: &mut Cpu) {
        if c.bus.read32(OC1574_FREE_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_FREE, ee::s06::f_002523a8__generated);
        }
        fakes::fake_call(c, FXLOADER_MEM_FREE);
    }

    /// `SSXStatsMan_StaticInitDestroy`, a logged call while its gate is on.
    fn oc1574_fake_init(c: &mut Cpu) {
        if c.bus.read32(OC1574_INIT_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_INIT, ee::s09::f_002d2988__generated);
        }
        fakes::fake_call(c, 0x002d_2988);
    }

    /// `VisualEffects_StaticInit`, a logged call while its gate is on.
    fn oc1574_fake_vfx(c: &mut Cpu) {
        if c.bus.read32(OC1574_VFX_CTRL) != OC1574_MAGIC {
            return fakes::fall_back(c, &OC1574_PREV_VFX, ee::s09::f_002d40c0__generated);
        }
        fakes::fake_call(c, 0x002d_40c0);
    }

    fn oc1574_hook_menu() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc1574_fake_item_dtor,
            &OC1574_PREV_ITEM,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            oc1574_fake_page_dtor,
            &OC1574_PREV_PAGE,
        );
    }

    fn oc1574_hook_fx() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E18C8),
            oc1574_fake_file,
            &OC1574_PREV_FILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B3900),
            oc1574_fake_shape,
            &OC1574_PREV_SHAPE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1574_fake_free,
            &OC1574_PREV_FREE,
        );
    }

    fn oc1574_hook_globals() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D2988),
            oc1574_fake_init,
            &OC1574_PREV_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D40C0),
            oc1574_fake_vfx,
            &OC1574_PREV_VFX,
        );
    }

    #[test]
    fn menu_sys_message_page_dtor_matches() {
        oc1574_hook_menu();
        check(
            ee::s09::f_002d3f78__generated,
            menu_sys_message_page_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x2d3f, 0);
                c.bus.write32(OC1574_ITEM_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_PAGE_CTRL, OC1574_MAGIC);
                c.g[A0][0] = (DATA + 0x2_0000 + 0x40 * (seed % 3)) as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x002d_3f78;
            },
        );
    }

    #[test]
    fn c_ps2_console_config_v01_matches() {
        crate::game::visualfx::worldlightman::tests::dtor_case(
            ee::s09::f_002d3fd8__generated,
            c_ps2_console_config_v01,
            0x2d3fd8,
        );
    }

    #[test]
    fn c_ps2_console_config_base_dtor_matches() {
        crate::game::visualfx::worldlightman::tests::dtor_case(
            ee::s09::f_002d4030__generated,
            c_ps2_console_config_base_dtor,
            0x2d4030,
        );
    }

    #[test]
    fn menu_sys_page_sort_compare_matches() {
        check(
            ee::s09::f_002d4008__generated,
            menu_sys_page_sort_compare,
            200,
            &|c, seed| {
                machine(c, seed, 0x2d40, 0);
                c.bus.write32(GAME_GP + 0x23cc, fakes::FAKE + 0x20);
                let (a, b) = (DATA + 0x100, DATA + 0x200);
                c.bus.write32(a, DATA + 0x300);
                c.bus.write32(b, DATA + 0x340);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.pc = 0x002d_4008;
            },
        );
    }

    #[test]
    fn ssx_stats_man_global_ctors_matches() {
        oc1574_hook_globals();
        check(
            ee::s09::f_002d4060__generated,
            ssx_stats_man_global_ctors,
            100,
            &|c, seed| {
                machine(c, seed, 0x2d40, 0);
                c.bus.write32(OC1574_INIT_CTRL, OC1574_MAGIC);
                c.pc = 0x002d_4060;
            },
        );
    }

    #[test]
    fn visual_effects_global_ctors_matches() {
        oc1574_hook_globals();
        check(
            ee::s09::f_002d48d0__generated,
            visual_effects_global_ctors,
            100,
            &|c, seed| {
                machine(c, seed, 0x2d48, 0);
                c.bus.write32(OC1574_VFX_CTRL, OC1574_MAGIC);
                c.pc = 0x002d_48d0;
            },
        );
    }

    /// The constructor's fixture: the texture manager pointer sits at gp+0x2a90
    /// (`D_BD10`), the manager holds the this-delta at +0x170 and the fake
    /// `AddTexture` at +0x174. Every seed picks `particles_only` (`a1`) and its
    /// own search outcomes.
    #[test]
    fn fx_loader_construct_matches() {
        oc1574_hook_fx();
        check_lohi(
            ee::s09::f_002dbd10__generated,
            fx_loader_construct,
            200,
            &|c, seed| {
                machine(c, seed, 0x2dbd, 0);
                c.bus.write32(OC1574_FILE_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_SHAPE_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_FREE_CTRL, OC1574_MAGIC);
                c.bus.write32(OC1574_SHAPE_SALT, seed.wrapping_mul(0x9e37_79b9));
                let (grman, mgr) = (DATA + 0x30_000, DATA + 0x32_000);
                c.bus.write32(GAME_GP + 0x2a90, grman);
                c.bus.write32(grman + 0x10d8, mgr);
                c.bus.write16(mgr + 0x170, ((seed % 4) * 0x10) as u16);
                c.bus.write32(mgr + 0x174, fakes::FAKE + 0x10);
                c.g[A0][0] = (DATA + 0x30_800) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002d_bd10;
            },
        );
    }

    #[test]
    fn desc_key4_sort_compare_matches() {
        check(
            ee::s09::f_002dc168__generated,
            desc_key4_sort_compare,
            200,
            &|c, seed| {
                machine(c, seed, 0x2dc1, 0);
                let (a, b) = (DATA + 0x100, DATA + 0x200);
                c.bus.write32(a + 4, seed % 3);
                c.bus.write32(b + 4, (seed / 3) % 3);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
                c.pc = 0x002d_c168;
            },
        );
    }
}
