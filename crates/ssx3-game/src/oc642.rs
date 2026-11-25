//! Staging module oc642 (wave g22.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc642`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0022_7fc0, ssxapp_shutdown, "SSXApp_Shutdown")];

/// `operator delete` (0x317e50).
const SSXAPP_OPERATOR_DELETE: u32 = 0x0031_7e50;
/// `FILESYS_delbigsync(file, 100)` (0x3dedc0).
const SSXAPP_DELBIGSYNC: u32 = 0x003d_edc0;

/// `this->field` (`this` is `s2`, read after every call: callees keep it).
fn ssxapp_field(c: &Cpu, off: u32) -> u32 {
    (c.g[S2][0] as u32).wrapping_add(off)
}

/// The global at `gp - off`.
fn ssxapp_global(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `FILESYS_delbigsync(a0, 100)` after `cost` cycles, returning to `ret_addr`.
fn ssxapp_delbigsync(c: &mut Cpu, cost: u64, ret_addr: u32) -> Flow {
    c.cycles += cost;
    c.g[A1][0] = 0x64;
    call_ptr(c, SSXAPP_DELBIGSYNC, ret_addr)
}

/// The tail of a big-file block: `if (a0) delbigsync(a0, 100)`, then
/// `a0 = this->next` (loaded in both cases).
fn ssxapp_free_then_load(c: &mut Cpu, cost: u64, ret_addr: u32, next: u32) -> Flow {
    if c.g[A0][0] != 0 {
        ssxapp_delbigsync(c, cost, ret_addr)?;
    }
    let a = ssxapp_field(c, next);
    c.g[A0][0] = c.lw(a);
    Ok(())
}

/// A virtual call on `obj`: `v0` is the vtable (`lw` at `obj + vt_off`), `a0`
/// the this-adjustment (`lh` at slot), `v1` the function (`lw` at slot + 4);
/// calls `fn(obj + adjust)` returning to `ret_addr`.
fn ssxapp_vcall(c: &mut Cpu, obj: u32, vt_off: u32, slot: u32, ret_addr: u32) -> Flow {
    let table_at = obj.wrapping_add(vt_off);
    c.g[V0][0] = c.lw(table_at);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = c.lh(vt.wrapping_add(slot));
    c.g[V1][0] = c.lw(vt.wrapping_add(slot + 4));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_addr)
}

/// `void cSSXApp::purge(cSSXApp* self)` (`SSXApp_Shutdown`, 0x227fc0): tears
/// the app down. Frees the input maps, the font interface and the pad cache,
/// deinitialises the script engine and the graphics manager, frees the big
/// files (the texture array is cleared in a loop of 11), deinits the world
/// triggers and the audio, then the BE, the game-mode manager, the async
/// manager and the console config, and clears the global `SSXApp`.
pub fn ssxapp_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        c.g[S0][0] = sx(ssxapp_field(c, 0xb8));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x1;

        // for (i = 0; i < 2; i++) if (InputMaps[i]) operator_delete(InputMaps[i]);
        'l_227fe0: loop {
            'b_227ff4: {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                let t = c.g[A0][0] == 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_227ff4;
                }
                c.cycles += 2;
                call_ptr(c, SSXAPP_OPERATOR_DELETE, 0x0022_7ff4)?;
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break 'l_227fe0;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_7fe0);
            }
        }

        // Fonts: if (Fonts) { purgeFonts(Fonts); delete Fonts; } s0 = PadCache.
        c.cycles += 3;
        let a = ssxapp_field(c, 0x88);
        c.g[S0][0] = c.lw(a);
        if c.g[S0][0] == 0 {
            let a = ssxapp_global(c, 0x850);
            c.g[S0][0] = c.lw(a);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039_8438, 0x0022_8010)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, SSXAPP_OPERATOR_DELETE, 0x0022_8018)?;
            let a = ssxapp_global(c, 0x850);
            c.g[S0][0] = c.lw(a);
        }

        // Fonts = 0; PadCache = 0; cPadCache_purge(mgr); delete mgr.
        c.cycles += 9;
        let a = ssxapp_field(c, 0x88);
        c.sw(a, 0);
        c.g[A0][0] = c.g[S0][0];
        let a = ssxapp_global(c, 0x850);
        c.sw(a, 0);
        call_ptr(c, 0x0032_6a68, 0x0022_802c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, SSXAPP_OPERATOR_DELETE, 0x0022_8034)?;

        // if (ScriptEngine) { DeInitOnce(ScriptEngine); vt[1](engine, 3); ScriptEngine = 0; }
        let a = ssxapp_global(c, 0x84c);
        c.g[A0][0] = c.lw(a);
        let t = c.g[A0][0] == 0;
        let a = ssxapp_global(c, 0x854);
        c.g[A1][0] = c.lw(a);
        if !t {
            'b_228068: {
                c.cycles += 5;
                call_ptr(c, 0x0027_8308, 0x0022_8048)?;
                let a = ssxapp_global(c, 0x84c);
                c.g[A2][0] = c.lw(a);
                let t = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_228068;
                }
                c.cycles += 5;
                let engine = c.g[A2][0] as u32;
                ssxapp_vcall(c, engine, 0x2a8, 0x8, 0x0022_8068)?;
            }
            c.cycles += 2;
            let a = ssxapp_global(c, 0x84c);
            c.sw(a, 0);
            let a = ssxapp_global(c, 0x854);
            c.g[A1][0] = c.lw(a);
        }

        // GrMan->Purge(); if (GrMan) GrMan->dtor(3); GrMan = 0.
        c.cycles += 8;
        let grman = c.g[A1][0] as u32;
        ssxapp_vcall(c, grman, 0x10d8, 0x18, 0x0022_8084)?;
        let a = ssxapp_global(c, 0x854);
        c.g[A2][0] = c.lw(a);
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !t {
            c.cycles += 5;
            let obj = c.g[A2][0] as u32;
            ssxapp_vcall(c, obj, 0x10d8, 0x8, 0x0022_80a4)?;
        }

        // Script data, model and the texture array (11 entries from +0xec).
        c.cycles += 3;
        let a = ssxapp_field(c, 0xe4);
        c.g[A0][0] = c.lw(a);
        let a = ssxapp_global(c, 0x854);
        c.sw(a, 0);
        if c.g[A0][0] != 0 {
            ssxapp_delbigsync(c, 2, 0x0022_80b8)?;
        }
        c.cycles += 3;
        let a = ssxapp_field(c, 0xe8);
        c.g[A0][0] = c.lw(a);
        c.g[S0][0] = sx(ssxapp_field(c, 0xec));
        if c.g[A0][0] != 0 {
            ssxapp_delbigsync(c, 3, 0x0022_80cc)?;
            c.g[S0][0] = sx(ssxapp_field(c, 0xec));
        }
        c.cycles += 2;
        c.g[S1][0] = 0xa;
        'l_2280d8: loop {
            'b_2280f0: {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    break 'b_2280f0;
                }
                ssxapp_delbigsync(c, 3, 0x0022_80ec)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            }
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break 'l_2280d8;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0022_80d8);
            }
        }

        // Rewards big file.
        c.cycles += 3;
        let a = ssxapp_field(c, 0x118);
        c.g[A0][0] = c.lw(a);
        if c.g[A0][0] != 0 {
            ssxapp_delbigsync(c, 2, 0x0022_810c)?;
        }

        // WorldTriggerManager, audio; then the audio, music and speech big files and BE.
        c.cycles += 7;
        call_ptr(c, 0x002b_4b48, 0x0022_8114)?;
        call_ptr(c, 0x0028_4c28, 0x0022_811c)?;
        let a = ssxapp_field(c, 0xd4);
        c.g[A0][0] = c.lw(a);
        ssxapp_free_then_load(c, 3, 0x0022_8130, 0xd8)?;
        c.cycles += 2;
        ssxapp_free_then_load(c, 3, 0x0022_8144, 0xdc)?;
        c.cycles += 2;
        ssxapp_free_then_load(c, 3, 0x0022_8158, 0xe0)?;
        c.cycles += 2;
        ssxapp_free_then_load(c, 3, 0x0022_816c, 0x78)?;
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0014_dd98, 0x0022_8180)?;
        }

        // GameModeMan, then the async manager (dtor flags 3) and the console config.
        c.cycles += 5;
        let a = ssxapp_field(c, 0xc0);
        c.g[A0][0] = c.lw(a);
        call_ptr(c, 0x0023_80e8, 0x0022_8188)?;
        let a = ssxapp_field(c, 0x11c);
        c.g[A0][0] = c.lw(a);
        if c.g[A0][0] == 0 {
            let a = ssxapp_field(c, 0x9c);
            c.g[A2][0] = c.lw(a);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x001a_35e8, 0x0022_819c)?;
            let a = ssxapp_field(c, 0x9c);
            c.g[A2][0] = c.lw(a);
        }
        c.cycles += 2;
        let t = c.g[A2][0] == 0;
        c.g[A1][0] = 0x3;
        if !t {
            c.cycles += 5;
            let console = c.g[A2][0] as u32;
            ssxapp_vcall(c, console, 0x34, 0x8, 0x0022_81bc)?;
        }

        // Epilogue: SSXApp = 0; return.
        c.cycles += 7;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let a = ssxapp_global(c, 0x848);
        c.sw(a, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word at `OC642_CTRL`; the word after it holds the test's flags.
    const OC642_CTRL: u32 = DATA + 0x7_2000;
    const OC642_MAGIC: u32 = 0x0642_0c0d;
    /// Flags: clear ScriptEngine during its DeInit (bit 0), clear GrMan
    /// during its Purge (bit 1).
    const OC642_FLAG_SCRIPT: u32 = 1;
    const OC642_FLAG_GRMAN: u32 = 2;

    /// Vtables of the three objects, and the fake targets their slots hold.
    const OC642_VT_SCRIPT: u32 = DATA + 0x2_0000;
    const OC642_VT_GRMAN: u32 = DATA + 0x2_0100;
    const OC642_VT_CONSOLE: u32 = DATA + 0x2_0200;
    const OC642_SLOT_SCRIPT_DTOR: u32 = FAKE + 0x100;
    const OC642_SLOT_GRMAN_DTOR: u32 = FAKE + 0x110;
    const OC642_SLOT_GRMAN_PURGE: u32 = FAKE + 0x120;
    const OC642_SLOT_CONSOLE_DTOR: u32 = FAKE + 0x130;

    /// A gated fake for a callee with an override slot: logs the call with
    /// the usual fake clobbers, or runs the body in `PREV` with the gate off.
    macro_rules! oc642_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC642_CTRL) != OC642_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc642_gated!(
        oc642_fake_operator_delete,
        OC642_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    oc642_gated!(
        oc642_fake_font_purge,
        OC642_PREV_FONT,
        0x0039_8438,
        ee::s13::f_00398438__generated
    );
    oc642_gated!(
        oc642_fake_pad_purge,
        OC642_PREV_PAD,
        0x0032_6a68,
        ee::s10::f_00326a68__generated
    );
    oc642_gated!(
        oc642_fake_delbigsync,
        OC642_PREV_BIG,
        0x003d_edc0,
        ee::s14::f_003dedc0__generated
    );
    oc642_gated!(
        oc642_fake_world_deinit,
        OC642_PREV_WORLD,
        0x002b_4b48,
        ee::s08::f_002b4b48__generated
    );
    oc642_gated!(
        oc642_fake_audio_deinit,
        OC642_PREV_AUDIO,
        0x0028_4c28,
        ee::s07::f_00284c28__generated
    );
    oc642_gated!(
        oc642_fake_be_purge,
        OC642_PREV_BE,
        0x0014_dd98,
        ee::s01::f_0014dd98__generated
    );
    oc642_gated!(
        oc642_fake_gm_purge,
        OC642_PREV_GM,
        0x0023_80e8,
        ee::s06::f_002380e8__generated
    );
    oc642_gated!(
        oc642_fake_async_dtor,
        OC642_PREV_ASYNC,
        0x001a_35e8,
        ee::s03::f_001a35e8__generated
    );

    static OC642_PREV_SCRIPT: AtomicUsize = AtomicUsize::new(0);

    /// ScriptEngine's DeInit: with the script flag it clears the global first,
    /// as the engine's own teardown would (so the dtor branch is skipped).
    fn oc642_fake_script_deinit(c: &mut Cpu) {
        if c.bus.read32(OC642_CTRL) != OC642_MAGIC {
            return fakes::fall_back(c, &OC642_PREV_SCRIPT, ee::s07::f_00278308__generated);
        }
        if c.bus.read32(OC642_CTRL + 4) & OC642_FLAG_SCRIPT != 0 {
            c.bus.write32(GAME_GP.wrapping_sub(0x84c), 0);
        }
        fakes::fake_call(c, 0x0027_8308);
    }

    /// The machine's miss hook: the GrMan vtable's Purge slot clears the
    /// global first when flagged (so the GrMan dtor branch is skipped).
    fn oc642_miss(c: &mut Cpu, target: u32) {
        if target == OC642_SLOT_GRMAN_PURGE && c.bus.read32(OC642_CTRL + 4) & OC642_FLAG_GRMAN != 0
        {
            c.bus.write32(GAME_GP.wrapping_sub(0x854), 0);
        }
        fakes::fake_call(c, target);
    }

    fn oc642_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc642_fake_operator_delete,
            &OC642_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00398438),
            oc642_fake_font_purge,
            &OC642_PREV_FONT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326A68),
            oc642_fake_pad_purge,
            &OC642_PREV_PAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278308),
            oc642_fake_script_deinit,
            &OC642_PREV_SCRIPT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEDC0),
            oc642_fake_delbigsync,
            &OC642_PREV_BIG,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B4B48),
            oc642_fake_world_deinit,
            &OC642_PREV_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00284C28),
            oc642_fake_audio_deinit,
            &OC642_PREV_AUDIO,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD98),
            oc642_fake_be_purge,
            &OC642_PREV_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002380E8),
            oc642_fake_gm_purge,
            &OC642_PREV_GM,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A35E8),
            oc642_fake_async_dtor,
            &OC642_PREV_ASYNC,
        );
    }

    /// A pointer field: null on about a quarter of the seeds.
    fn oc642_opt(r: &mut Rng, v: u32) -> u32 {
        if r.next_u32() % 4 == 0 {
            0
        } else {
            v
        }
    }

    /// One vtable slot: a random 16-bit this-adjustment at `at`, the target at `at + 4`.
    fn oc642_slot(c: &mut Cpu, r: &mut Rng, at: u32, target: u32) {
        c.bus.write16(at, (r.next_u32() as u16) & 0xfffc);
        c.bus.write32(at + 4, target);
    }

    /// The app object at DATA (pointer fields null or not per seed), the
    /// globals at gp, and the three objects whose vtables hold fake slots.
    fn oc642_fixture(c: &mut Cpu, r: &mut Rng) {
        let app = DATA;
        fill(c, r, app, 0x60);
        let engine = DATA + 0x8000;
        let grman = DATA + 0x4000;
        let console = DATA + 0xa000;
        let big = |k: u32| DATA + 0x1_0000 + 0x40 * k;
        for (off, k) in [
            (0x78u32, 1u32),
            (0x88, 2),
            (0xb8, 3),
            (0xbc, 4),
            (0xc0, 5),
            (0xd4, 6),
            (0xd8, 7),
            (0xdc, 8),
            (0xe0, 9),
            (0xe4, 10),
            (0xe8, 11),
            (0x118, 12),
            (0x11c, 13),
        ] {
            let v = oc642_opt(r, big(k));
            c.bus.write32(app + off, v);
        }
        for i in 0..11u32 {
            let v = oc642_opt(r, big(16 + i));
            c.bus.write32(app + 0xec + 4 * i, v);
        }
        let v = oc642_opt(r, console);
        c.bus.write32(app + 0x9c, v);
        let v = oc642_opt(r, engine);
        c.bus.write32(GAME_GP.wrapping_sub(0x84c), v);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), app);
        c.bus.write32(GAME_GP.wrapping_sub(0x850), DATA + 0x6000);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), grman);

        c.bus.write32(engine + 0x2a8, OC642_VT_SCRIPT);
        c.bus.write32(grman + 0x10d8, OC642_VT_GRMAN);
        c.bus.write32(console + 0x34, OC642_VT_CONSOLE);
        oc642_slot(c, r, OC642_VT_SCRIPT + 8, OC642_SLOT_SCRIPT_DTOR);
        oc642_slot(c, r, OC642_VT_GRMAN + 8, OC642_SLOT_GRMAN_DTOR);
        oc642_slot(c, r, OC642_VT_GRMAN + 0x18, OC642_SLOT_GRMAN_PURGE);
        oc642_slot(c, r, OC642_VT_CONSOLE + 8, OC642_SLOT_CONSOLE_DTOR);
    }

    #[test]
    fn ssxapp_shutdown_matches() {
        oc642_install();
        fakes::covered(
            ee::s05::f_00227fc0__generated,
            ssxapp_shutdown,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_7fc0, 12);
                c.hooks.miss = oc642_miss;
                c.bus.write32(OC642_CTRL, OC642_MAGIC);
                let flags = r.next_u32() & 3;
                c.bus.write32(OC642_CTRL + 4, flags);
                c.g[A0][0] = DATA as u64;
                oc642_fixture(c, &mut r);
            },
        );
    }
}
