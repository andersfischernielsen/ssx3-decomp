//! Staging module oc1498 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1498`.

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
        0x002b_b1d0,
        cworld_painter_man_alloc_files,
        "cWorldPainterMan_AllocFiles",
    ),
    (0x0019_5aa0, felocale_set_language, "FELocale_v03"),
    (
        0x002a_16b0,
        ssx_audio_system_rider_say_20bc,
        "SSXAudioSystem_RiderSay20BC",
    ),
    (
        0x002b_b348,
        cworld_painter_man_set_file_data,
        "cWorldPainterMan_SetFileData",
    ),
    (0x002f_4380, lensfx_init_flare_set, "LensFX_InitFlareSet"),
    (0x001a_2b00, fe_module_load_anim_data, "FEModule_LoadAnimData"),
    (0x002f_5180, world_light_man_ctor, "WorldLightMan_ctor"),
];

/// `cWorldPainterMan_AllocFiles(WPArr* self, int n)` (0x2bb1d0): destroys the
/// files of `self`'s current array (the count is the word in front of the
/// array; the block goes back to the allocator), then allocates an array of
/// `n` 12-byte files with a count word in front, stores `n` at `self+0` and
/// the array at `self+4`, and constructs the files.
pub fn cworld_painter_man_alloc_files(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let empty = c.g[A1][0] == 0u64;
        c.g[S3][0] = 0x480000;
        if !empty {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = 0xc;
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002b_b390,
                        f: ee::s08::f_002bb390,
                    }
                    .call(c, 0x002b_b22c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x002b_b220);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    break;
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x002b_b244)?;
        }
        c.cycles += 14;
        c.g[A0][0] = 0xc;
        c.sw(c.g[S1][0] as u32, c.g[S2][0]);
        c.mult(0, 4, 18, 4);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c68));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0u64;
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7e30, 0x002b_b26c)?;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[S2][0]);
        let none = c.g[S2][0] == 0u64;
        c.g[S0][0] = c.g[S3][0];
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0xffff_ffff_ffff_ffff;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                Callee {
                    addr: 0x002b_b370,
                    f: ee::s08::f_002bb370,
                }
                .call(c, 0x002b_b28c)?;
                let more = c.g[S1][0] != c.g[S2][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b280);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        c.sw(c.g[S4][0] as u32, c.g[S3][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan_SetFileData(WPArr* self, int idx, int arg)` (0x2bb348):
/// `WPFile_SetData(self->arr + idx * 12, idx, arg)`.
pub fn cworld_painter_man_set_file_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 2, 5, 2);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Callee {
            addr: 0x002b_b570,
            f: ee::s08::f_002bb570,
        }
        .call(c, 0x002b_b364)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cSunHalo::cSunHalo(sunHalo)` (0x2f4380), `LensFX_InitFlareSet`: runs the
/// flare constructor stub, spins its 16-turn counter loop, clears the record's
/// count word (+0x204) and the two halfword fields (+0x2c, +0x2e), then sets
/// the default flares and returns the record.
pub fn lensfx_init_flare_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002e_2860,
            f: ee::s09::f_002e2860,
        }
        .call(c, 0x002f_4394)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 0xf;
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002f_43a0);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x204), 0u64);
        c.sh((c.g[S0][0] as u32).wrapping_add(0x2e), 0u64);
        c.sh((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
        Callee {
            addr: 0x002f_43e0,
            f: ee::s09::f_002f43e0,
        }
        .call(c, 0x002f_43cc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The changed-language arm of `FELocale_v03` (`'b_195b5c`): when the locale's
/// flag bit 0 (byte +3) is clear, runs the language switch, loads the table of
/// the slot, then the FE tables (byte +0x58 == 1) or the game tables. Returns
/// true when the generated code takes its `cycles += 1` exit.
fn felocale_changed(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x3));
    c.g[V0][0] = c.g[V0][0] & 0x1;
    let flagged = c.g[V0][0] != 0u64;
    c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[S0][0]);
    if flagged {
        return Ok(true);
    }
    felocale_slot(c)?;
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x0019_59a8,
        f: ee::s02::f_001959a8,
    }
    .call(c, 0x0019_5b34)?;
    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x58));
    c.g[V0][0] = 0x1;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0019_5de0,
            f: ee::s02::f_00195de0,
        }
        .call(c, 0x0019_5b5c)?;
        return Ok(true);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x0019_5be0,
        f: ee::s02::f_00195be0,
    }
    .call(c, 0x0019_5b4c)?;
    Ok(false)
}

/// The language switch of `FELocale_v03`: unloads the tables, then leaves in
/// `a1` the slot for the language `s0`: 1 for language 1, 0 for 0, 2 and 3 for
/// themselves, -1 for the others (the cycle charges follow the generated
/// branches).
fn felocale_slot(c: &mut Cpu) -> Result<(), crate::call::Unwound> {
    c.cycles += 5;
    Callee {
        addr: 0x0019_5948,
        f: ee::s02::f_00195948,
    }
    .call(c, 0x0019_5ae0)?;
    c.g[V0][0] = 0x1;
    let is_one = c.g[S0][0] == c.g[V0][0];
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    if is_one {
        c.cycles += 2;
        c.g[A1][0] = 0x1;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
    let at_least_two = c.g[V0][0] == 0u64;
    c.g[V0][0] = 0x2;
    if !at_least_two {
        c.cycles += 2;
        if c.g[S0][0] == 0u64 {
            c.g[A1][0] = 0u64;
            return Ok(());
        }
        c.cycles += 2;
        return Ok(());
    }
    c.cycles += 2;
    let is_two = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = 0x3;
    if is_two {
        c.cycles += 1;
        c.g[A1][0] = 0x2;
        return Ok(());
    }
    c.cycles += 2;
    if c.g[S0][0] == c.g[V0][0] {
        c.g[A1][0] = 0x3;
        return Ok(());
    }
    c.cycles += 2;
    Ok(())
}

/// `FELocale_v03(FELocale* self, int lang)` (0x195aa0), `cFELocale_setLanguage`:
/// if `lang` (the low byte of a1) is not the current language (byte +1), stores
/// it and, unless the flag bit 0 of byte +3 is set, reloads the tables for it.
pub fn felocale_set_language(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x1));
        if c.g[V0][0] != c.g[S0][0] {
            let plus_one = felocale_changed(c)?;
            if plus_one {
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SSXAudioSystem_RiderSay20BC(mgr, id)` (0x2a16b0): when the speech volume is
/// on, the speech system's instance is ready, the race interface and the game
/// path are set, and the speech event context takes the character `id`, sends
/// the speaker's speech through the speech call table entry.
pub fn ssx_audio_system_rider_say_20bc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Callee {
            addr: 0x0029_f0a0,
            f: ee::s08::f_0029f0a0,
        }
        .call(c, 0x002a_16cc)?;
        let speech_on = c.g[V0][0] != 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
        // Every exit leaves through the same epilogue; the exits that go
        // through `'b_2a1764` also charge one cycle (`plus_one`).
        let plus_one = 'exit: {
            if !speech_on {
                break 'exit true;
            }
            c.cycles += 5;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x002b_0e28,
                f: ee::s08::f_002b0e28,
            }
            .call(c, 0x002a_16e0)?;
            if c.g[V0][0] == 0u64 {
                break 'exit false;
            }
            c.cycles += 9;
            call_ptr(c, 0x0014_dc80, 0x002a_16f0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014_dd58, 0x002a_16fc)?;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
            let has_path = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S0][0];
            if !has_path {
                break 'exit true;
            }
            c.cycles += 10;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x20bc;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = c.g[S1][0];
            c.g[T1][0] = 0u64;
            c.g[T2][0] = 0x1;
            c.mtc1(0, 12);
            Callee {
                addr: 0x002b_1458,
                f: ee::s08::f_002b1458,
            }
            .call(c, 0x002a_172c)?;
            let set = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S1][0];
            if !set {
                break 'exit true;
            }
            c.cycles += 12;
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x0029_f2b0,
                f: ee::s08::f_0029f2b0,
            }
            .call(c, 0x002a_173c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x1;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x20bc;
            call_ptr(c, 0x003d_8008, 0x002a_1750)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = 0x1;
            call_ptr(c, target, 0x002a_1764)?;
            true
        };
        if plus_one {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEModule_LoadAnimData` (0x1a2b00), `cRealFE_loadCharAnimations`: opens the
/// big FE animation file set, creates one `AdfDataSource` (0x18-byte block) per
/// entry of the anim table, stores it in the animation database and adds it to
/// the game anim map, resolves the map and closes the file set.
pub fn fe_module_load_anim_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = 0x460000;
        frame.save(c, S0, 0x40);
        c.g[A1][0] = 0u64;
        frame.save(c, S1, 0x30);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1330));
        frame.save(c, S2, 0x20);
        c.g[A2][0] = 0x64;
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[SP][0];
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x003d_ed50, 0x001a_2b30)?;
        c.g[S2][0] = 0x460000;
        c.g[S1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1730));
        c.g[A2][0] = 0u64;
        loop {
            c.cycles += 20;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = 0x18;
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1348));
            call_ptr(c, 0x0031_7d70, 0x001a_2b50)?;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A0][0] = c.g[V0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x0031_1250, 0x001a_2b60)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd08));
            c.g[A0][0] = c.g[S0][0] & 0xff;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            call_ptr(c, 0x0031_4f30, 0x001a_2b88)?;
            let more = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001a_2b40);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xd8c));
        call_ptr(c, 0x0031_50b0, 0x001a_2b98)?;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x003d_edc0, 0x001a_2ba4)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldLightMan::cWorldLightMan(cBXNash* pNash)` (0x2f5180), `WorldLightMan_ctor`:
/// marks the 8 overlap-list nodes (stride 0x40) and the 32 light-cache entries
/// (stride 0x180) as empty (-1), stores `pNash`, builds the light cache and
/// takes the lighting query from the world's painter manager.
pub fn world_light_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = 0x7;
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let more = c.g[A0][0] != c.g[A2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002f_51a8);
                }
                continue;
            }
            break;
        }
        c.cycles += 5;
        c.g[V1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x210));
        c.g[A0][0] = 0x1f;
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let more = c.g[A0][0] != c.g[A2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x180));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002f_51d8);
                }
                continue;
            }
            break;
        }
        c.cycles += 15;
        c.sw(c.g[S0][0] as u32, c.g[A1][0]);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002f_5298,
            f: ee::s09::f_002f5298,
        }
        .call(c, 0x002f_5200)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x24a8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x002c_0a58, 0x002f_5218)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word value: a test's fakes run only while their gate word holds it.
    const OC1498_MAGIC: u32 = 0x1498_c0de;

    /// Gate words of the tests of this module (one per test, in the fixture
    /// area above the call log).
    const OC1498_AF_GATE: u32 = DATA + 0x20_0000;
    const OC1498_SRF_GATE: u32 = DATA + 0x20_0100;
    const OC1498_LFX_GATE: u32 = DATA + 0x20_0200;
    const OC1498_WLM_GATE: u32 = DATA + 0x20_0300;
    const OC1498_FEL_GATE: u32 = DATA + 0x20_0400;
    const OC1498_RSY_GATE: u32 = DATA + 0x20_0500;
    const OC1498_ANM_GATE: u32 = DATA + 0x20_0600;

    /// A gated fake of one callee's override slot. With its gate word set, it
    /// logs the call to `target` (returning to `ra`) and returns the word at
    /// `gate + mail` in v0; with the gate off it runs the body the slot held
    /// before.
    macro_rules! oc1498_gated_fake {
        ($fake:ident, $prev:ident, $gate:expr, $mail:expr, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != OC1498_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32($gate + $mail) as u64;
            }
        };
    }

    // cWorldPainterMan_AllocFiles: callees WPFile_dtor, WPFile_ctor, operator new, free.
    oc1498_gated_fake!(
        oc1498_af_dtor,
        OC1498_AF_PREV_DTOR,
        OC1498_AF_GATE,
        4,
        ee::s08::f_002bb390__generated,
        0x002b_b390
    );
    oc1498_gated_fake!(
        oc1498_af_ctor,
        OC1498_AF_PREV_CTOR,
        OC1498_AF_GATE,
        4,
        ee::s08::f_002bb370__generated,
        0x002b_b370
    );
    oc1498_gated_fake!(
        oc1498_af_new,
        OC1498_AF_PREV_NEW,
        OC1498_AF_GATE,
        4,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );
    oc1498_gated_fake!(
        oc1498_af_free,
        OC1498_AF_PREV_FREE,
        OC1498_AF_GATE,
        4,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );

    fn oc1498_af_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB390),
            oc1498_af_dtor,
            &OC1498_AF_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB370),
            oc1498_af_ctor,
            &OC1498_AF_PREV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1498_af_new,
            &OC1498_AF_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1498_af_free,
            &OC1498_AF_PREV_FREE,
        );
    }

    /// `cWorldPainterMan_AllocFiles`: an empty and a filled old array, zero and
    /// several new files, with unwinds from the fakes.
    #[test]
    fn cworld_painter_man_alloc_files_matches() {
        check(
            ee::s08::f_002bb1d0__generated,
            cworld_painter_man_alloc_files,
            300,
            &|c, seed| {
                oc1498_af_hook();
                machine(c, seed, 0x2b_b1d0, 3);
                c.pc = 0x002b_b1d0;
                c.bus.write32(OC1498_AF_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_AF_GATE + 4, DATA + 0x4000);
                let this = DATA + 0x1000;
                let arr = if seed % 4 == 0 { 0 } else { DATA + 0x2010 };
                c.bus.write32(this + 4, arr);
                if arr != 0 {
                    c.bus.write32(arr - 0x10, seed % 4);
                }
                c.bus.write32(this, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 3);
            },
        );
    }

    // cWorldPainterMan_SetFileData: callee WPFile_SetData.
    oc1498_gated_fake!(
        oc1498_srf_setdata,
        OC1498_SRF_PREV,
        OC1498_SRF_GATE,
        4,
        ee::s08::f_002bb570__generated,
        0x002b_b570
    );

    fn oc1498_srf_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB570),
            oc1498_srf_setdata,
            &OC1498_SRF_PREV,
        );
    }

    /// `cWorldPainterMan_SetFileData` for file indices 0..4 and random args.
    #[test]
    fn cworld_painter_man_set_file_data_matches() {
        check(
            ee::s08::f_002bb348__generated,
            cworld_painter_man_set_file_data,
            300,
            &|c, seed| {
                oc1498_srf_hook();
                let mut r = machine(c, seed, 0x2b_b348, 2);
                c.pc = 0x002b_b348;
                c.bus.write32(OC1498_SRF_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_SRF_GATE + 4, DATA + 0x4000);
                let this = DATA + 0x1000;
                c.bus.write32(this + 4, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 5);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    // LensFX_InitFlareSet: callees the flare constructor stub and SetDefaultFlares.
    oc1498_gated_fake!(
        oc1498_lfx_ctor,
        OC1498_LFX_PREV_CTOR,
        OC1498_LFX_GATE,
        4,
        ee::s09::f_002e2860__generated,
        0x002e_2860
    );
    oc1498_gated_fake!(
        oc1498_lfx_defaults,
        OC1498_LFX_PREV_DEFAULTS,
        OC1498_LFX_GATE,
        4,
        ee::s09::f_002f43e0__generated,
        0x002f_43e0
    );

    fn oc1498_lfx_hook() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002E2860),
            oc1498_lfx_ctor,
            &OC1498_LFX_PREV_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F43E0),
            oc1498_lfx_defaults,
            &OC1498_LFX_PREV_DEFAULTS,
        );
    }

    /// `LensFX_InitFlareSet` on a random flare set, with unwinds from the fakes.
    #[test]
    fn lensfx_init_flare_set_matches() {
        check(
            ee::s09::f_002f4380__generated,
            lensfx_init_flare_set,
            300,
            &|c, seed| {
                oc1498_lfx_hook();
                machine(c, seed, 0x2f_4380, 2);
                c.pc = 0x002f_4380;
                c.bus.write32(OC1498_LFX_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_LFX_GATE + 4, DATA + 0x6000);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    // FELocale_v03: callees UnloadAllTables, LoadTable, LoadFETables, LoadGameTables.
    oc1498_gated_fake!(
        oc1498_fel_unload,
        OC1498_FEL_PREV_UNLOAD,
        OC1498_FEL_GATE,
        4,
        ee::s02::f_00195948__generated,
        0x0019_5948
    );
    oc1498_gated_fake!(
        oc1498_fel_table,
        OC1498_FEL_PREV_TABLE,
        OC1498_FEL_GATE,
        4,
        ee::s02::f_001959a8__generated,
        0x0019_59a8
    );
    oc1498_gated_fake!(
        oc1498_fel_fe,
        OC1498_FEL_PREV_FE,
        OC1498_FEL_GATE,
        4,
        ee::s02::f_00195be0__generated,
        0x0019_5be0
    );
    oc1498_gated_fake!(
        oc1498_fel_game,
        OC1498_FEL_PREV_GAME,
        OC1498_FEL_GATE,
        4,
        ee::s02::f_00195de0__generated,
        0x0019_5de0
    );

    fn oc1498_fel_hook() {
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195948),
            oc1498_fel_unload,
            &OC1498_FEL_PREV_UNLOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_001959A8),
            oc1498_fel_table,
            &OC1498_FEL_PREV_TABLE,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195BE0),
            oc1498_fel_fe,
            &OC1498_FEL_PREV_FE,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00195DE0),
            oc1498_fel_game,
            &OC1498_FEL_PREV_GAME,
        );
    }

    /// `FELocale_v03` for languages -3..=5 (every switch arm), the same and a
    /// different current language, the flag bit set and clear, both table kinds.
    #[test]
    fn felocale_set_language_matches() {
        check(
            ee::s02::f_00195aa0__generated,
            felocale_set_language,
            300,
            &|c, seed| {
                oc1498_fel_hook();
                let mut r = machine(c, seed, 0x19_5aa0, 2);
                c.pc = 0x0019_5aa0;
                c.bus.write32(OC1498_FEL_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_FEL_GATE + 4, DATA + 0x6000);
                let lang = (seed % 9) as i32 - 3;
                let this = DATA + 0x1000;
                let current = if seed % 2 == 0 {
                    lang as u8
                } else {
                    r.next_u32() as u8
                };
                c.bus.write8(this + 1, current);
                c.bus.write8(this + 3, (seed % 4 == 0) as u8);
                c.bus.write8(this + 0x58, (seed % 3 == 0) as u8);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(lang as u32);
            },
        );
    }

    // SSXAudioSystem_RiderSay20BC: the speech callees, and the table entry the
    // tail call goes through (0x4000 is outside the function table: a miss).
    oc1498_gated_fake!(
        oc1498_rsy_volume,
        OC1498_RSY_PREV_VOLUME,
        OC1498_RSY_GATE,
        4,
        ee::s08::f_0029f0a0__generated,
        0x0029_f0a0
    );
    oc1498_gated_fake!(
        oc1498_rsy_ready,
        OC1498_RSY_PREV_READY,
        OC1498_RSY_GATE,
        8,
        ee::s08::f_002b0e28__generated,
        0x002b_0e28
    );
    oc1498_gated_fake!(
        oc1498_rsy_be,
        OC1498_RSY_PREV_BE,
        OC1498_RSY_GATE,
        12,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );
    oc1498_gated_fake!(
        oc1498_rsy_interface,
        OC1498_RSY_PREV_INTERFACE,
        OC1498_RSY_GATE,
        16,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );
    oc1498_gated_fake!(
        oc1498_rsy_event,
        OC1498_RSY_PREV_EVENT,
        OC1498_RSY_GATE,
        20,
        ee::s08::f_002b1458__generated,
        0x002b_1458
    );
    oc1498_gated_fake!(
        oc1498_rsy_mask,
        OC1498_RSY_PREV_MASK,
        OC1498_RSY_GATE,
        24,
        ee::s08::f_0029f2b0__generated,
        0x0029_f2b0
    );
    oc1498_gated_fake!(
        oc1498_rsy_speech,
        OC1498_RSY_PREV_SPEECH,
        OC1498_RSY_GATE,
        28,
        ee::s14::f_003d8008__generated,
        0x003d_8008
    );

    fn oc1498_rsy_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F0A0),
            oc1498_rsy_volume,
            &OC1498_RSY_PREV_VOLUME,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B0E28),
            oc1498_rsy_ready,
            &OC1498_RSY_PREV_READY,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc1498_rsy_be,
            &OC1498_RSY_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1498_rsy_interface,
            &OC1498_RSY_PREV_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B1458),
            oc1498_rsy_event,
            &OC1498_RSY_PREV_EVENT,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_0029F2B0),
            oc1498_rsy_mask,
            &OC1498_RSY_PREV_MASK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003D8008),
            oc1498_rsy_speech,
            &OC1498_RSY_PREV_SPEECH,
        );
    }

    /// `SSXAudioSystem_RiderSay20BC` with each callee's result drawn per seed:
    /// speech volume off, instance not ready, no path, event context refused,
    /// and the full speech call.
    #[test]
    fn ssx_audio_system_rider_say_20bc_matches() {
        check(
            ee::s08::f_002a16b0__generated,
            ssx_audio_system_rider_say_20bc,
            300,
            &|c, seed| {
                oc1498_rsy_hook();
                machine(c, seed, 0x2a_16b0, 4);
                c.pc = 0x002a_16b0;
                c.bus.write32(OC1498_RSY_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_RSY_GATE + 4, (seed % 4 != 0) as u32);
                c.bus.write32(OC1498_RSY_GATE + 8, (seed % 3 != 0) as u32);
                c.bus.write32(OC1498_RSY_GATE + 12, DATA + 0x7000);
                c.bus.write32(OC1498_RSY_GATE + 16, DATA + 0x7100);
                c.bus.write32(OC1498_RSY_GATE + 20, (seed % 7 != 0) as u32);
                c.bus.write32(OC1498_RSY_GATE + 24, 0x1234 + seed);
                c.bus.write32(OC1498_RSY_GATE + 28, DATA + 0x7200);
                c.bus.write8(0x0053_5c11, (seed % 5 != 0) as u8);
                c.bus.write32(GAME_GP + 0x173c, fakes::FAKE);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(seed % 4);
            },
        );
    }

    // WorldLightMan_ctor: callees AllocOverlapList and the lighting query.
    oc1498_gated_fake!(
        oc1498_wlm_overlap,
        OC1498_WLM_PREV_OVERLAP,
        OC1498_WLM_GATE,
        4,
        ee::s09::f_002f5298__generated,
        0x002f_5298
    );
    oc1498_gated_fake!(
        oc1498_wlm_query,
        OC1498_WLM_PREV_QUERY,
        OC1498_WLM_GATE,
        4,
        ee::s08::f_002c0a58__generated,
        0x002c_0a58
    );

    fn oc1498_wlm_hook() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F5298),
            oc1498_wlm_overlap,
            &OC1498_WLM_PREV_OVERLAP,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C0A58),
            oc1498_wlm_query,
            &OC1498_WLM_PREV_QUERY,
        );
    }

    /// `WorldLightMan_ctor` on a random record, with the world, painter manager
    /// and query chain in RAM.
    #[test]
    fn world_light_man_ctor_matches() {
        check(
            ee::s09::f_002f5180__generated,
            world_light_man_ctor,
            300,
            &|c, seed| {
                oc1498_wlm_hook();
                let mut r = machine(c, seed, 0x2f_5180, 2);
                c.pc = 0x002f_5180;
                c.bus.write32(OC1498_WLM_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_WLM_GATE + 4, DATA + 0x6400);
                let world = DATA + 0x6000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, DATA + 0x6100);
                c.bus.write32(DATA + 0x6100 + 0x10, DATA + 0x6200);
                c.bus.write32(DATA + 0x6200 + 0x8, DATA + 0x6300);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    // FEModule_LoadAnimData: the file set, allocator, data source, anim map callees.
    oc1498_gated_fake!(
        oc1498_anm_addbig,
        OC1498_ANM_PREV_ADDBIG,
        OC1498_ANM_GATE,
        4,
        ee::s14::f_003ded50__generated,
        0x003d_ed50
    );
    oc1498_gated_fake!(
        oc1498_anm_alloc,
        OC1498_ANM_PREV_ALLOC,
        OC1498_ANM_GATE,
        4,
        ee::s10::f_00317d70__generated,
        0x0031_7d70
    );
    oc1498_gated_fake!(
        oc1498_anm_source,
        OC1498_ANM_PREV_SOURCE,
        OC1498_ANM_GATE,
        4,
        ee::s10::f_00311250__generated,
        0x0031_1250
    );
    oc1498_gated_fake!(
        oc1498_anm_add,
        OC1498_ANM_PREV_ADD,
        OC1498_ANM_GATE,
        4,
        ee::s10::f_00314f30__generated,
        0x0031_4f30
    );
    oc1498_gated_fake!(
        oc1498_anm_resolve,
        OC1498_ANM_PREV_RESOLVE,
        OC1498_ANM_GATE,
        4,
        ee::s10::f_003150b0__generated,
        0x0031_50b0
    );
    oc1498_gated_fake!(
        oc1498_anm_delbig,
        OC1498_ANM_PREV_DELBIG,
        OC1498_ANM_GATE,
        4,
        ee::s14::f_003dedc0__generated,
        0x003d_edc0
    );

    fn oc1498_anm_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DED50),
            oc1498_anm_addbig,
            &OC1498_ANM_PREV_ADDBIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc1498_anm_alloc,
            &OC1498_ANM_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00311250),
            oc1498_anm_source,
            &OC1498_ANM_PREV_SOURCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00314F30),
            oc1498_anm_add,
            &OC1498_ANM_PREV_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003150B0),
            oc1498_anm_resolve,
            &OC1498_ANM_PREV_RESOLVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEDC0),
            oc1498_anm_delbig,
            &OC1498_ANM_PREV_DELBIG,
        );
    }

    /// `FEModule_LoadAnimData` with the database and anim map pointers in RAM
    /// and unwinds from the fakes.
    #[test]
    fn fe_module_load_anim_data_matches() {
        check(
            ee::s03::f_001a2b00__generated,
            fe_module_load_anim_data,
            300,
            &|c, seed| {
                oc1498_anm_hook();
                machine(c, seed, 0x1a_2b00, 3);
                c.pc = 0x001a_2b00;
                c.bus.write32(OC1498_ANM_GATE, OC1498_MAGIC);
                c.bus.write32(OC1498_ANM_GATE + 4, DATA + 0x6000);
                c.bus.write32(GAME_GP + 0xd08, DATA + 0x6800);
                c.bus.write32(GAME_GP + 0xd8c, DATA + 0x6900);
                c.bus.write32(GAME_GP - 0x1730, DATA + 0x6a00);
            },
        );
    }
}
