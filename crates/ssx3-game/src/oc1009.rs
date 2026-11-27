//! Staging module oc1009 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1009`.

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
    (0x0023_3438, fe_load_screen_load, "cFELoadScreen_Load"),
    (
        0x0023_3640,
        fe_load_screen_load_pictures,
        "cFELoadScreen_LoadPictures",
    ),
    (
        0x0024_7ab0,
        fe_load_state_setup_result_widget,
        "cFELoadState_SetupResultWidget",
    ),
    (0x0028_5210, ssx_audio_system_dtor, "SSXAudioSystem_dtor"),
];

/// The address `gp - neg` (the game's gp-relative globals).
#[inline]
fn oc1009_gp_minus(c: &Cpu, neg: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(neg)
}

/// `reg + off` as a 32-bit address.
#[inline]
fn oc1009_ea(reg: u64, off: u32) -> u32 {
    (reg as u32).wrapping_add(off)
}

/// `a0 = base - sub` as the `lui`/`addiu` pair leaves it (sign-extended).
#[inline]
fn oc1009_base_minus(base: u32, sub: u32) -> u64 {
    sx(base.wrapping_sub(sub))
}

// ---------------------------------------------------------------------------
// cFELoadScreen_Load (0x233438)
// ---------------------------------------------------------------------------

/// Offsets of the load-screen state object (`this`).
pub(crate) const FE_LOAD_SCREEN_UI_ENGINE: u32 = 0xc;
pub(crate) const FE_LOAD_SCREEN_STATE: u32 = 0x18;

/// `void cFELoadScreen_Load(cFELoadScreen* this)` (0x233438): builds the
/// loading screen. Calls the app object's vtable slot (`+0x10d8`), creates the
/// UI engine (`this+0xc`, 0x70 bytes, `cUIEngine_loadFile`), copies two config
/// words into it, gives it a 0x1c-byte helper object, then picks one of three
/// screens (0x27: plain, no saved picture state or no mode byte: picture type
/// 1, else picture type 2), loads its pictures (`LoadPictures`), adds the screen
/// to the engine's list (`engine+0x18`) and marks `this+0x18 = -1`.
pub fn fe_load_screen_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.lw(oc1009_gp_minus(c, 0x854));
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0x100;
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc1009_ea(c.g[A2][0], 0x10d8));
        c.g[A0][0] = c.lh(oc1009_ea(c.g[V0][0], 0x160));
        c.g[V1][0] = c.lw(oc1009_ea(c.g[V0][0], 0x164));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(oc1009_ea(c.g[A2][0], c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_346c)?;
        // engine = new cUIEngine (0x70 bytes)
        c.g[A1][0] = 0x0048_0000;
        c.g[A2][0] = 0x100;
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4968);
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0023_3484)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0039_7b08, 0x0023_348c)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4050);
        c.sw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE), c.g[V0][0]);
        call_ptr(c, 0x0039_7c40, 0x0023_34a0)?;
        // copy two config words of the game object into the engine
        c.g[T0][0] = c.lw(oc1009_gp_minus(c, 0x848));
        c.g[A1][0] = 0x0048_0000;
        c.g[T1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        c.g[A0][0] = 0x1c;
        c.g[V0][0] = c.lw(oc1009_ea(c.g[T0][0], 0x88));
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4040);
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.sw(oc1009_ea(c.g[T1][0], 0x8), c.g[V0][0]);
        c.g[V1][0] = c.lw(oc1009_ea(c.g[T0][0], 0x8c));
        c.g[V0][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        c.sw(oc1009_ea(c.g[V0][0], 0x10), c.g[V1][0]);
        // helper object (0x1c bytes) at engine+0xc
        call_ptr(c, 0x0031_7d70, 0x0023_34d4)?;
        c.g[V1][0] = 0x0047_0000;
        c.g[A0][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x2e30);
        c.sb(oc1009_ea(c.g[V0][0], 0x4), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0x8), c.g[V1][0]);
        c.sw(oc1009_ea(c.g[V0][0], 0x18), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0xc), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0x10), 0);
        c.sw(oc1009_ea(c.g[V0][0], 0x14), 0);
        c.sw(oc1009_ea(c.g[A0][0], 0xc), c.g[V0][0]);
        c.g[V0][0] = c.lw(oc1009_gp_minus(c, 0x848));
        c.g[S0][0] = c.lw(oc1009_ea(c.g[V0][0], 0xc0));
        call_ptr(c, 0x0014_dc80, 0x0023_3508)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_3514)?;
        c.g[V1][0] = c.lw(oc1009_gp_minus(c, 0x172c));
        c.g[V0][0] = 0x27;
        // Which screen: 0 = id 0x27 (plain), 1 = picture type 1, 2 = picture type 2.
        let kind;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw(oc1009_ea(c.g[S0][0], 0x9c));
            c.cycles += 2;
            let no_state = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0053_0000;
            if no_state {
                kind = 1;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x5c11));
                let has_mode = c.g[V1][0] != 0;
                c.g[A1][0] = 0x0048_0000;
                kind = if has_mode { 2 } else { 1 };
            }
        } else {
            kind = 0;
        }
        match kind {
            0 => {
                c.cycles += 15;
                c.g[A1][0] = 0x0048_0000;
                c.g[A0][0] = 0x48;
                c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4030);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_353c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_3550)?;
                c.g[V1][0] = 0x0048_0000;
                c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x3ac8);
                c.sw(oc1009_ea(c.g[S0][0], 0x8), c.g[V1][0]);
                c.cycles += 1;
            }
            1 => {
                c.cycles += 21;
                c.g[A1][0] = 0x0048_0000;
                c.g[A0][0] = 0x4c;
                c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4018);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_358c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_35a0)?;
                c.g[V1][0] = 0x0048_0000;
                c.sw(oc1009_ea(c.g[S0][0], 0x48), 0);
                c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x3928);
                c.g[A0][0] = c.g[S2][0];
                c.sw(oc1009_ea(c.g[S0][0], 0x8), c.g[V1][0]);
                c.g[A1][0] = 1;
                c.sw(oc1009_ea(c.g[S1][0], 0x48), c.g[S2][0]);
                call_ptr(c, 0x0023_3640, 0x0023_35c0)?;
            }
            _ => {
                c.cycles += 18;
                c.g[A0][0] = 0x4c;
                c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4000);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0023_35dc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_e2a0, 0x0023_35f0)?;
                c.g[V1][0] = 0x0048_0000;
                c.sw(oc1009_ea(c.g[S0][0], 0x48), 0);
                c.g[V1][0] = oc1009_base_minus(c.g[V1][0] as u32, 0x39f8);
                c.g[A0][0] = c.g[S2][0];
                c.sw(oc1009_ea(c.g[S0][0], 0x8), c.g[V1][0]);
                c.g[A1][0] = 2;
                c.sw(oc1009_ea(c.g[S1][0], 0x48), c.g[S2][0]);
                call_ptr(c, 0x0023_3640, 0x0023_3610)?;
                c.cycles += 1;
            }
        }
        // (screen kind 1 has no extra cycle: the generated block exits one step earlier)
        c.g[A0][0] = c.lw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_UI_ENGINE));
        // engine->screens.add(screen)
        c.cycles += 11;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(oc1009_ea(c.g[A0][0], 0x18));
        call_ptr(c, 0x0039_f400, 0x0023_3620)?;
        c.g[V0][0] = u64::MAX;
        frame.restore(c, S0, 0x30);
        c.sw(oc1009_ea(c.g[S2][0], FE_LOAD_SCREEN_STATE), c.g[V0][0]);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFELoadScreen_LoadPictures (0x233640)
// ---------------------------------------------------------------------------

/// `void cFELoadScreen_LoadPictures(cFELoadScreen* this, int type)` (0x233640):
/// opens the `0x47bd78` big file (stack `cBigFile`), and loads the loading
/// screen's rider picture(s). Type 1: one picture named from the rider's
/// character id (player 0 or 1 depending on the load-screen options at
/// `0x534b30`, else the local player's) into `this+0x10`; type 2 with two
/// players (`0x535c11 == 2`): one picture per player into `this+0x10` and
/// `this+0x14`. Closes the big file (`cBigFile::~cBigFile(2)`).
pub fn fe_load_screen_load_pictures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        frame.save(c, S2, 0xe0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xd0);
        frame.save(c, S7, 0x90);
        frame.save(c, S1, 0xf0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save(c, S6, 0xa0);
        frame.save_ra(c, 0x80);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3674)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_3680)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_3688)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_3694)?;
        // stack cBigFile at sp+0x70
        c.g[A0][0] = sx(oc1009_ea(c.g[SP][0], 0x70));
        c.g[A1][0] = 0x0048_0000;
        c.g[S7][0] = c.g[A0][0];
        c.g[A1][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4288);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_68b0, 0x0023_36ac)?;
        c.g[V1][0] = 1;
        let not_type1 = c.g[S0][0] != c.g[V1][0];
        c.g[V0][0] = 2;
        if !not_type1 {
            // type 1: one picture
            c.cycles += 10;
            c.g[V1][0] = c.lw(oc1009_gp_minus(c, 0x848));
            c.g[V0][0] = 0x0048_0000;
            c.g[S1][0] = oc1009_base_minus(c.g[V0][0] as u32, 0x4270);
            c.g[A1][0] = 7;
            c.g[A0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x78));
            call_ptr(c, 0x0014_dd58, 0x0023_36d0)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x4b30));
            let opt_off = c.g[V0][0] == 0;
            c.g[A0][0] = sx(oc1009_ea(c.g[V1][0], 0x4b30));
            if !opt_off {
                // options on: rider index 0 or 1 by the alternate flag at +8
                c.cycles += 3;
                c.g[V0][0] = c.lw(oc1009_ea(c.g[A0][0], 0x8));
                let alt_off = c.g[V0][0] == 0;
                c.g[S0][0] = 0x0044_0000;
                c.g[A0][0] = c.g[S3][0];
                if alt_off {
                    c.cycles += 3;
                    c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], 0x2928));
                    c.g[A1][0] = 1;
                } else {
                    c.cycles += 4;
                    c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], 0x2928));
                    c.g[A1][0] = 0;
                }
                c.cycles += 11;
                call_ptr(c, 0x0014_7398, 0x0023_3710)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = c.lw(oc1009_gp_minus(c, 0x5b8));
                c.g[V0][0] = sx(oc1009_ea(c.g[V0][0], c.g[S0][0] as u32));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x0023_372c)?;
                c.g[A1][0] = 0x0300_0000;
            } else {
                c.cycles += 16;
                c.g[S0][0] = 0x0044_0000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], 0x2928));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_6e98, 0x0023_3748)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7398, 0x0023_3754)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = c.lw(oc1009_gp_minus(c, 0x5b8));
                c.g[V0][0] = sx(oc1009_ea(c.g[V0][0], c.g[S0][0] as u32));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x0023_3770)?;
                c.g[A1][0] = 0x0300_0000;
            }
            // data = FileLoad(name, 0x3000100); this->tex = CreateTexture(data)
            c.cycles += 19;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] |= 0x100;
            call_ptr(c, 0x003e_2190, 0x0023_3780)?;
            c.g[T2][0] = c.lw(oc1009_gp_minus(c, 0x854));
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x14));
            c.g[A2][0] = 0x0048_0000;
            c.g[V1][0] = c.lw(oc1009_ea(c.g[T2][0], 0x10d8));
            c.g[A2][0] = oc1009_base_minus(c.g[A2][0] as u32, 0x4260);
            c.g[A1][0] = sx(oc1009_ea(c.g[S0][0], c.g[A1][0] as u32));
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x170));
            c.g[T0][0] = 1;
            c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x174));
            c.g[T1][0] = u64::MAX;
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(oc1009_ea(c.g[T2][0], c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_37b8)?;
            let no_data = c.g[S0][0] == 0;
            c.sw(oc1009_ea(c.g[S2][0], 0x10), c.g[V0][0]);
            if no_data {
                c.cycles += 1;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x0023_37c8)?;
            }
            c.g[A0][0] = c.g[S7][0];
        } else {
            c.cycles += 2;
            let not_type2 = c.g[S0][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S7][0];
            if !not_type2 {
                c.cycles += 4;
                c.g[V0][0] = 0x0053_0000;
                c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x5c11));
                let not_two_players = c.g[V1][0] != c.g[S0][0];
                c.g[V0][0] = 0x0044_0000;
                if !not_two_players {
                    c.cycles += 6;
                    c.g[S2][0] = sx(oc1009_ea(c.g[S2][0], 0x10));
                    c.g[S6][0] = sx(oc1009_ea(c.g[V0][0], 0x2928));
                    c.g[S1][0] = 0;
                    c.g[S5][0] = 0x0048_0000;
                    c.g[S4][0] = 0x0048_0000;
                    loop {
                        c.cycles += 29;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0014_7488, 0x0023_380c)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A3][0] = c.lw(oc1009_gp_minus(c, 0x5b4));
                        c.g[V0][0] = sx(oc1009_ea(c.g[V0][0], c.g[S6][0] as u32));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A1][0] = oc1009_base_minus(c.g[S5][0] as u32, 0x4270);
                        call_ptr(c, 0x0041_7828, 0x0023_3828)?;
                        c.g[A1][0] = 0x0300_0000;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] |= 0x100;
                        call_ptr(c, 0x003e_2190, 0x0023_3838)?;
                        c.g[T2][0] = c.lw(oc1009_gp_minus(c, 0x854));
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x14));
                        c.g[A2][0] = oc1009_base_minus(c.g[S4][0] as u32, 0x4260);
                        c.g[V1][0] = c.lw(oc1009_ea(c.g[T2][0], 0x10d8));
                        c.g[A3][0] = 0;
                        c.g[A1][0] = sx(oc1009_ea(c.g[S0][0], c.g[A1][0] as u32));
                        c.g[T0][0] = 1;
                        c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x170));
                        c.g[T1][0] = u64::MAX;
                        c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x174));
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(oc1009_ea(c.g[T2][0], c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0023_386c)?;
                        let no_data = c.g[S0][0] == 0;
                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                        if !no_data {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0031_7e98, 0x0023_387c)?;
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx(oc1009_ea(c.g[S1][0], 1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                        let again = c.g[V0][0] != 0;
                        c.g[S2][0] = sx(oc1009_ea(c.g[S2][0], 4));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_3800);
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S7][0];
                }
            }
        }
        // cBigFile::~cBigFile(a0 = stack file, 2)
        c.cycles += 13;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_68e8, 0x0023_3898)?;
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore(c, S6, 0xa0);
        frame.restore(c, S7, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFELoadState_SetupResultWidget (0x247ab0)
// ---------------------------------------------------------------------------

/// Offset of a widget's name hash.
pub(crate) const FE_LOAD_STATE_WIDGET_HASH: u32 = 0x38;

/// One link of the widget-name chain: charges `extra` cycles, loads the
/// widget's hash (`s4+0x38` into `s0`), computes `GetHashValue32` of the name
/// string at `0x480000 - sub` (0x317670, returning to `ret`). Returns whether
/// the widget is NOT this one (`s0 != v0`). With `set_a0` the delay slot's
/// `a0 = 0x480000` follows.
fn oc1009_srw_hash_miss(
    c: &mut Cpu,
    extra: u64,
    sub: u32,
    ret: u32,
    set_a0: bool,
) -> Result<bool, crate::call::Unwound> {
    c.cycles += extra;
    c.g[S0][0] = c.lw(oc1009_ea(c.g[S4][0], FE_LOAD_STATE_WIDGET_HASH));
    c.g[A0][0] = oc1009_base_minus(c.g[A0][0] as u32, sub);
    call_ptr(c, 0x0031_7670, ret)?;
    let miss = c.g[S0][0] != c.g[V0][0];
    if set_a0 {
        c.g[A0][0] = 0x0048_0000;
    }
    Ok(miss)
}

/// Epilogue of `SetupResultWidget` (`s0` is restored by the caller).
fn oc1009_srw_finish(c: &mut Cpu, frame: Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x100);
    frame.restore(c, S2, 0xf0);
    frame.restore(c, S3, 0xe0);
    frame.restore(c, S4, 0xd0);
    frame.restore(c, S5, 0xc0);
    frame.restore_ra(c, 0xb0);
    frame.close(c);
}

/// `UIText::SetText(widget, buf)` (0x3a0e90) with the text built in the stack
/// buffer, then the epilogue.
fn oc1009_srw_set_text(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003a_0e90, 0x0024_7d08)?;
    frame.restore(c, S0, 0x110);
    oc1009_srw_finish(c, frame);
    Ok(())
}

/// Appends the number in `a2` to the format in `a1` (0x2c26d0, wide-string
/// printf) into the stack buffer, then sets the widget text.
fn oc1009_srw_format_and_set(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    call_ptr(c, 0x002c_26d0, 0x0024_7cfc)?;
    c.g[A0][0] = c.g[S4][0];
    oc1009_srw_set_text(c, frame)
}

/// `void cFELoadState_SetupResultWidget(cFELoadState* this, cUIText* widget)`
/// (0x247ab0): fills the text of one widget of the result screen, picked by the
/// widget's name hash (`widget+0x38`): the rider's name (0x4138), a rider
/// string formatted with a statistic (0x4128), one of five numeric statistics
/// of the saved result record (`%d` format at `0x4a2c28`, fields `+0x2c`, `+0x28`,
/// `+0x18`, `+0x20`, `+0xc`), the race time as `mm:ss` (0x40a0, seconds `+0x48`
/// as float) and the score (0x4078, field `+0x88`).
pub fn fe_load_state_setup_result_widget(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S4, 0xd0);
        frame.save(c, S0, 0x110);
        frame.save(c, S1, 0x100);
        frame.save(c, S2, 0xf0);
        frame.save(c, S3, 0xe0);
        frame.save(c, S5, 0xc0);
        frame.save_ra(c, 0xb0);
        c.g[S4][0] = c.g[A1][0];
        // interfaces 1, 2, 3 and 8 of the BE
        call_ptr(c, 0x0014_dc80, 0x0024_7ad8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0024_7ae4)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0024_7aec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0014_dd58, 0x0024_7af8)?;
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0024_7b00)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0014_dd58, 0x0024_7b0c)?;
        c.g[S5][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0024_7b14)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, 0x0014_dd58, 0x0024_7b20)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_6e98, 0x0024_7b30)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0015_5270, 0x0024_7b3c)?;
        // s1 = the result record of the local player
        c.g[A0][0] = 0x0048_0000;
        c.g[S1][0] = c.g[V0][0];

        // arm 1 (0x4138): the first string of the rider
        if !oc1009_srw_hash_miss(c, 0, 0x4138, 0x0024_7b50, true)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e98, 0x0024_7b64)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x0024_7b70)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002c_2540, 0x0024_7b7c)?;
            c.g[A0][0] = c.g[S4][0];
            return oc1009_srw_set_text(c, frame);
        }

        // arm 2 (0x4128): a game-object string formatted with the rider's value
        if !oc1009_srw_hash_miss(c, 5, 0x4128, 0x0024_7b90, true)? {
            c.cycles += 26;
            c.g[V0][0] = c.lw(oc1009_gp_minus(c, 0x848));
            c.g[A0][0] = 0x0048_0000;
            c.g[S2][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x30));
            c.g[A0][0] = oc1009_base_minus(c.g[A0][0] as u32, 0x4380);
            c.g[S0][0] = c.lw(oc1009_ea(c.g[V0][0], 0x8c));
            c.g[S1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x4));
            c.g[V0][0] = c.lh(oc1009_ea(c.g[S1][0], 0x20));
            c.g[S0][0] = sx(oc1009_ea(c.g[S0][0], c.g[V0][0] as u32));
            call_ptr(c, 0x0031_7670, 0x0024_7bbc)?;
            c.g[V1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x24));
            c.g[A0][0] = c.g[S0][0];
            let method = c.g[V1][0] as u32;
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, method, 0x0024_7bcc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x0024_7bd8)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e98, 0x0024_7be4)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_8950, 0x0024_7bf0)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[V0][0];
            return oc1009_srw_format_and_set(c, frame);
        }

        // arms 3..7: `%d` (wide format at 0x4a2c28) of one field of the record:
        // (name offset, hash return, format return, extra cycles, field)
        const NUMERIC: [(u32, u32, u32, u64, u32); 5] = [
            (0x4118, 0x0024_7c0c, 0x0024_7c28, 7, 0x2c),
            (0x4100, 0x0024_7c3c, 0x0024_7c58, 7, 0x28),
            (0x40e8, 0x0024_7c6c, 0x0024_7c88, 7, 0x18),
            (0x40d0, 0x0024_7c9c, 0x0024_7cb8, 7, 0x20),
            (0x40b8, 0x0024_7ccc, 0x0024_7ce8, 6, 0x0c),
        ];
        for (sub, hash_ret, fmt_ret, extra, field) in NUMERIC {
            if !oc1009_srw_hash_miss(c, 5, sub, hash_ret, true)? {
                c.cycles += extra;
                c.g[S0][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x30));
                c.g[A1][0] = 0x004a_0000;
                c.g[A1][0] = sx(oc1009_ea(c.g[A1][0], 0x2c28));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_2540, fmt_ret)?;
                c.g[A2][0] = c.lw(oc1009_ea(c.g[S1][0], field));
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[SP][0];
                return oc1009_srw_format_and_set(c, frame);
            }
        }

        // arm 8 (0x40a0): the race time as "%02.0f:%02.0f" (minutes, seconds)
        if !oc1009_srw_hash_miss(c, 5, 0x40a0, 0x0024_7d1c, true)? {
            c.cycles += 4;
            c.lwc1(0, oc1009_ea(c.g[S1][0], 0x48));
            c.g[V1][0] = 0x3c;
            c.cycles += 33;
            c.g[A1][0] = 0x0048_0000;
            c.cvt_w_s(1, 0);
            c.mfc1(4, 1);
            c.g[A2][0] = oc1009_base_minus(c.g[A1][0] as u32, 0x4088);
            c.g[V0][0] = 0x3c;
            c.g[S3][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x90));
            c.g[S2][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x70));
            c.div(0, 4, 3);
            c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_sub(0x4088));
            c.g[A1][0] = c.lhu(oc1009_ea(c.g[A2][0], 0xc));
            c.g[V0][0] = c.lw(oc1009_ea(c.g[A2][0], 0x8));
            c.sd(oc1009_ea(c.g[SP][0] as u64, 0x70), c.g[A3][0]);
            c.sh(oc1009_ea(c.g[SP][0] as u64, 0x7c), c.g[A1][0]);
            c.g[A0][0] = c.lo[0];
            c.g[S1][0] = c.hi[0];
            c.mtc1(4, 12);
            c.cvt_s_w(12, 12);
            c.sw(oc1009_ea(c.g[SP][0] as u64, 0x78), c.g[V0][0]);
            call_ptr(c, 0x0041_3af8, 0x0024_7d80)?;
            c.mtc1(17, 12);
            c.cvt_s_w(12, 12);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0041_3af8, 0x0024_7d90)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_7828, 0x0024_7da4)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a_0c50, 0x0024_7db0)?;
            frame.restore(c, S0, 0x110);
            oc1009_srw_finish(c, frame);
            return Ok(());
        }

        // arm 9 (0x4078): the score
        let miss = oc1009_srw_hash_miss(c, 5, 0x4078, 0x0024_7dc4, false)?;
        frame.restore(c, S0, 0x110);
        if !miss {
            c.cycles += 13;
            c.g[S0][0] = sx(oc1009_ea(c.g[SP][0] as u64, 0x30));
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(oc1009_ea(c.g[A1][0], 0x2c28));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_2540, 0x0024_7de0)?;
            c.g[A2][0] = c.lw(oc1009_ea(c.g[S1][0], 0x88));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002c_26d0, 0x0024_7df0)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0e90, 0x0024_7dfc)?;
            frame.restore(c, S0, 0x110);
        }
        oc1009_srw_finish(c, frame);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_dtor (0x285210)
// ---------------------------------------------------------------------------

/// `void SSXAudioSystem::~SSXAudioSystem(int flags)` (0x285210): `this = a0`,
/// `flags = a1` (bit 0: delete the object, bit 1: destroy the virtual bases).
/// Resets the vtable words (music system `+0x5558`, speech system `+0x571c`,
/// object `+0xc`), points the sound-system bases' vtables back at the base
/// tables (`BankManager+0x1d8/+0x4`, `+0xab0`, `+0x1d4`; with `flags == 0` it
/// first builds adjusted stack copies of those tables, gcc 2.95's virtual-base
/// `this` fix-up), unloads the loading-screen bank, removes the user-chunk
/// handler, tears down the speech system (`+0x5560`), the big-file sync (`+0x5728`)
/// and the play history (`+0x5980`), runs the controller destructor and, for
/// `flags & 2`, the stream, bank-manager and sound-system destructors, and for
/// `flags & 1` frees the object.
pub fn ssx_audio_system_dtor(c: &mut Cpu) {
    guest(c, |c| {
        // vtable words, then (flags == 0) the stack copies of the base tables
        c.cycles += 36;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.g[V0][0] = 0x480000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V1][0] = 0x480000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A2][0] = 0x480000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x36d8));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x36b8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5558), c.g[V0][0]);
        c.g[A0][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3708));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x571c), c.g[V1][0]);
        c.g[A2][0] = 0x480000;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
        c.g[T8][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3648));
        c.g[S3][0] = 0x480000;
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[T9][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3728));
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T8][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
        let t = c.g[S2][0] != 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T9][0]);
        if !t {
            c.cycles += 81;
            c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x3708));
            c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sd((c.g[SP][0] as u32), c.g[A1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.g[A0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[T5][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x3648));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
            c.g[T4][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x8));
            c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[T3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[T2][0] as u32));
            c.g[T1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[T0][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c90));
            c.g[A3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
            c.g[A2][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x38));
            c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[T2][0] as u32));
            c.g[A3][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x30));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T6][0]);
            c.g[A2][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x8));
            c.g[T7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[T7][0] as u32));
            c.g[A0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c98));
            c.g[V0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
            c.g[T6][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x3728));
            c.g[T0][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[T5][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[T4][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x18));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[T3][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[T2][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x28));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[T1][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A2][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T6][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T1][0]);
            c.g[V0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lw((c.g[T7][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[S0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
        }
        let frame = Frame { size: 0x100 };
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0028_f730,
            f: ee::s07::f_0028f730,
        }
        .call(c, 0x0028_53ec)?;
        c.g[A0][0] = 0x0029_0000;
        c.g[A0][0] = oc1009_base_minus(c.g[A0][0] as u32, 0x69b0);
        call_ptr(c, 0x003b_5a60, 0x0028_53f8)?;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002b_0da0, 0x0028_5400)?;
        c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x5728));
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x5980));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x64;
            call_ptr(c, 0x003d_edc0, 0x0028_5414)?;
            c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x5980));
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0028_5428)?;
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        c.g[A1][0] = 0;
        call_ptr(c, 0x002a_b478, 0x0028_5434)?;
        c.g[V0][0] = c.g[S2][0] & 0x2;
        let no_bases = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        if !no_bases {
            c.cycles += 13;
            c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x118));
            c.g[A1][0] = 0;
            call_ptr(c, 0x002a_a648, 0x0028_544c)?;
            c.g[V0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x118));
            c.g[A1][0] = 0;
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            Callee {
                addr: 0x0028_bb10,
                f: ee::s07::f_0028bb10,
            }
            .call(c, 0x0028_545c)?;
            c.g[V1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x118));
            c.g[A1][0] = 0;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A0][0] = c.lw(oc1009_ea(c.g[V0][0], 0x1d8));
            call_ptr(c, 0x002a_dee8, 0x0028_5470)?;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xf0);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0028_5484)?;
            frame.restore(c, S0, 0xf0);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_OnRiderFinish (0x286ea0)
// ---------------------------------------------------------------------------

/// `void SSXAudioSystem::OnRiderFinish(Rider* rider)` (0x286ea0): `this = a0`,
/// `rider = a1`. Ignored unless the rider is the one the audio system tracks
/// for its player slot (`rider+0x870`). Reports the finish to the sound
/// controller (`gp+0x410`, mode 2, rider id unless a replay is playing),
/// updates the rider's, race and music state, switches the game music (monitor
/// message `0x24` when the finish is in the racing modes 2/3 and the player
/// is ahead, else message `0xa`), then, for a rider that has not finished
/// (`rider+0x480 == 0`), starts the finish speech (`0x2a3708`) or the
/// celebration (`0x2a34d0`) and finally the finish stinger (`0x2a45c0`) in
/// the plain race mode.
#[allow(dead_code)]
pub fn ssx_audio_system_on_rider_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x870));
        Callee {
            addr: 0x0028_5d98,
            f: ee::s07::f_00285d98,
        }
        .call(c, 0x0028_6ed0)?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x60);
            return oc1009_orf_finish(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0028_8ae0,
            f: ee::s07::f_00288ae0,
        }
        .call(c, 0x0028_6ee0)?;
        let replay = c.g[V0][0] != 0;
        c.g[A2][0] = 0;
        if !replay {
            c.cycles += 1;
            c.g[A2][0] = c.lw(oc1009_ea(c.g[S1][0], 0x870));
        }
        c.cycles += 29;
        c.g[A0][0] = c.lw(oc1009_ea(c.g[GP][0], 0x410));
        c.g[A1][0] = 2;
        c.g[S4][0] = 1;
        call_ptr(c, 0x002a_73a8, 0x0028_6efc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_6e20, 0x0028_6f08)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_7438, 0x0028_6f14)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_b3c0, 0x0028_6f20)?;
        c.g[V0][0] = c.lw(oc1009_gp_minus(c, 0x848));
        c.g[V1][0] = c.lw(oc1009_ea(c.g[V0][0], 0xc0));
        c.g[S3][0] = c.lw(c.g[V1][0] as u32);
        c.g[S2][0] = c.lw(oc1009_ea(c.g[V1][0], 0x70));
        call_ptr(c, 0x0014_dc80, 0x0028_6f34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6f40)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[S5][0] = c.g[V1][0];
        c.g[V1][0] = sx(oc1009_ea(c.g[V1][0], 0x5bc8));
        c.g[V0][0] = c.lb(oc1009_ea(c.g[V1][0], 0x48));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let not_race_mode = c.g[V0][0] == 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        if !not_race_mode {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                Callee {
                    addr: 0x0028_d960,
                    f: ee::s07::f_0028d960,
                }
                .call(c, 0x0028_6f70)?;
                let music_off = c.g[V0][0] == 0;
                c.g[A2][0] = sx(oc1009_ea(c.g[S0][0], 0x118));
                if !music_off {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x5558));
                    c.g[A1][0] = 0x24;
                    c.g[S4][0] = 0;
                    c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x20));
                    c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x24));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc1009_ea(c.g[A2][0], c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0028_6f94)?;
                    c.sw(oc1009_ea(c.g[S0][0], 0x627c), 0);
                }
            }
        }
        c.cycles += 2;
        if c.g[S4][0] != 0 {
            c.cycles += 4;
            Callee {
                addr: 0x0028_d960,
                f: ee::s07::f_0028d960,
            }
            .call(c, 0x0028_6fa8)?;
            let music_off = c.g[V0][0] == 0;
            c.g[A2][0] = sx(oc1009_ea(c.g[S0][0], 0x118));
            if !music_off {
                c.cycles += 6;
                c.g[V1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x5558));
                c.g[A1][0] = 0xa;
                c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x20));
                c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x24));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx(oc1009_ea(c.g[A2][0], c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_6fc8)?;
            }
        }
        // the rider's own finish sound
        c.cycles += 8;
        call_ptr(c, 0x0014_dc80, 0x0028_6fd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6fdc)?;
        c.g[V1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x480));
        let already = c.g[V1][0] != 0;
        c.g[V0][0] = sx(oc1009_ea(c.g[S5][0], 0x5bc8));
        if !already {
            // Which start: the finish speech (two code paths, `a` and `b`, that
            // differ in cycles and return address) or the celebration.
            #[derive(PartialEq)]
            enum Start {
                SpeechA,
                SpeechB,
                Celebrate,
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4078, 0x0028_6ff0)?;
            let tracked = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            let start;
            if tracked {
                start = Start::SpeechA;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx(oc1009_ea(c.g[S5][0], 0x5bc8));
                c.g[A0][0] = 2;
                c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x49));
                let not_mode2 = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 3) as u64;
                if !not_mode2 {
                    c.cycles += 6;
                    Callee {
                        addr: 0x0028_b1d8,
                        f: ee::s07::f_0028b1d8,
                    }
                    .call(c, 0x0028_7014)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0012_a250, 0x0028_701c)?;
                    let no_race = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    start = if no_race {
                        Start::Celebrate
                    } else {
                        Start::SpeechA
                    };
                } else {
                    c.cycles += 2;
                    let few_players = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    start = if few_players {
                        Start::Celebrate
                    } else {
                        Start::SpeechB
                    };
                }
            }
            match start {
                Start::SpeechA => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_3708, 0x0028_702c)?;
                }
                Start::Celebrate => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    c.g[A3][0] = u64::MAX;
                    call_ptr(c, 0x002a_34d0, 0x0028_7050)?;
                }
                Start::SpeechB => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_3708, 0x0028_7060)?;
                }
            }
            c.g[V0][0] = sx(oc1009_ea(c.g[S5][0], 0x5bc8));
        }
        c.cycles += 3;
        c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x49));
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x60);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_45c0, 0x0028_7078)?;
            frame.restore(c, S0, 0x60);
        }
        oc1009_orf_finish(c, frame)
    });
}

/// Epilogue of `OnRiderFinish` (`s0` is restored by the caller).
fn oc1009_orf_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies).
    const OC1009_CTRL: u32 = DATA + 0x20_0000;
    const OC1009_MAGIC: u32 = 0x1009_f00d;
    /// Per-fake call counters (`+ 4 * index`) and result queues (`+ 0x40 *
    /// index`, 16 words: 0 = the fake's own pointer, 1 = zero, else the value).
    const OC1009_CNT: u32 = DATA + 0x20_0800;
    const OC1009_Q: u32 = DATA + 0x20_1000;
    /// Fixture objects.
    const OC1009_APP: u32 = DATA + 0x21_1000;
    const OC1009_APP_VT: u32 = DATA + 0x21_2000;
    const OC1009_GOBJ: u32 = DATA + 0x21_3000;
    const OC1009_GM: u32 = DATA + 0x21_4000;
    const OC1009_THIS: u32 = DATA + 0x21_0000;

    /// Every faked callee: a gated fake in its override slot that logs the
    /// call (all registers) and answers from its result queue.
    macro_rules! oc1009_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            const OC1009_TARGETS: &[u32] = &[$($addr),*];
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1009_CTRL) != OC1009_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    oc1009_fake(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1009_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1009_fakes! {
        oc1009_fake_0014dc80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1009_fake_0014dd58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1009_fake_00317d70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc1009_fake_00397b08, 0x0039_7b08, ee::s13::OVERRIDE_00397B08, ee::s13::f_00397b08__generated;
        oc1009_fake_00397c40, 0x0039_7c40, ee::s13::OVERRIDE_00397C40, ee::s13::f_00397c40__generated;
        oc1009_fake_0039e2a0, 0x0039_e2a0, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
        oc1009_fake_0039f400, 0x0039_f400, ee::s13::OVERRIDE_0039F400, ee::s13::f_0039f400__generated;
        oc1009_fake_003168b0, 0x0031_68b0, ee::s10::OVERRIDE_003168B0, ee::s10::f_003168b0__generated;
        oc1009_fake_003168e8, 0x0031_68e8, ee::s10::OVERRIDE_003168E8, ee::s10::f_003168e8__generated;
        oc1009_fake_00147398, 0x0014_7398, ee::s01::OVERRIDE_00147398, ee::s01::f_00147398__generated;
        oc1009_fake_00417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc1009_fake_00146e98, 0x0014_6e98, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated;
        oc1009_fake_003e2190, 0x003e_2190, ee::s14::OVERRIDE_003E2190, ee::s14::f_003e2190__generated;
        oc1009_fake_00317e98, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc1009_fake_00147488, 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc1009_fake_00155270, 0x0015_5270, ee::s01::OVERRIDE_00155270, ee::s01::f_00155270__generated;
        oc1009_fake_00317670, 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc1009_fake_002c2540, 0x002c_2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated;
        oc1009_fake_0014ee58, 0x0014_ee58, ee::s01::OVERRIDE_0014EE58, ee::s01::f_0014ee58__generated;
        oc1009_fake_002c2508, 0x002c_2508, ee::s08::OVERRIDE_002C2508, ee::s08::f_002c2508__generated;
        oc1009_fake_00148950, 0x0014_8950, ee::s01::OVERRIDE_00148950, ee::s01::f_00148950__generated;
        oc1009_fake_002c26d0, 0x002c_26d0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated;
        oc1009_fake_003a0e90, 0x003a_0e90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated;
        oc1009_fake_00413af8, 0x0041_3af8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated;
        oc1009_fake_003a0c50, 0x003a_0c50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated;
        oc1009_fake_0028f730, 0x0028_f730, ee::s07::OVERRIDE_0028F730, ee::s07::f_0028f730__generated;
        oc1009_fake_003b5a60, 0x003b_5a60, ee::s13::OVERRIDE_003B5A60, ee::s13::f_003b5a60__generated;
        oc1009_fake_002b0da0, 0x002b_0da0, ee::s08::OVERRIDE_002B0DA0, ee::s08::f_002b0da0__generated;
        oc1009_fake_003dedc0, 0x003d_edc0, ee::s14::OVERRIDE_003DEDC0, ee::s14::f_003dedc0__generated;
        oc1009_fake_002ab478, 0x002a_b478, ee::s08::OVERRIDE_002AB478, ee::s08::f_002ab478__generated;
        oc1009_fake_002aa648, 0x002a_a648, ee::s08::OVERRIDE_002AA648, ee::s08::f_002aa648__generated;
        oc1009_fake_0028bb10, 0x0028_bb10, ee::s07::OVERRIDE_0028BB10, ee::s07::f_0028bb10__generated;
        oc1009_fake_002adee8, 0x002a_dee8, ee::s08::OVERRIDE_002ADEE8, ee::s08::f_002adee8__generated;
        oc1009_fake_00317e50, 0x0031_7e50, ee::s10::OVERRIDE_00317E50, ee::s10::f_00317e50__generated;
        oc1009_fake_00285d98, 0x0028_5d98, ee::s07::OVERRIDE_00285D98, ee::s07::f_00285d98__generated;
        oc1009_fake_00288ae0, 0x0028_8ae0, ee::s07::OVERRIDE_00288AE0, ee::s07::f_00288ae0__generated;
        oc1009_fake_002a73a8, 0x002a_73a8, ee::s08::OVERRIDE_002A73A8, ee::s08::f_002a73a8__generated;
        oc1009_fake_00296e20, 0x0029_6e20, ee::s08::OVERRIDE_00296E20, ee::s08::f_00296e20__generated;
        oc1009_fake_00297438, 0x0029_7438, ee::s08::OVERRIDE_00297438, ee::s08::f_00297438__generated;
        oc1009_fake_0029b3c0, 0x0029_b3c0, ee::s08::OVERRIDE_0029B3C0, ee::s08::f_0029b3c0__generated;
        oc1009_fake_0028d960, 0x0028_d960, ee::s07::OVERRIDE_0028D960, ee::s07::f_0028d960__generated;
        oc1009_fake_002a4078, 0x002a_4078, ee::s08::OVERRIDE_002A4078, ee::s08::f_002a4078__generated;
        oc1009_fake_0028b1d8, 0x0028_b1d8, ee::s07::OVERRIDE_0028B1D8, ee::s07::f_0028b1d8__generated;
        oc1009_fake_0012a250, 0x0012_a250, ee::s00::OVERRIDE_0012A250, ee::s00::f_0012a250__generated;
        oc1009_fake_002a3708, 0x002a_3708, ee::s08::OVERRIDE_002A3708, ee::s08::f_002a3708__generated;
        oc1009_fake_002a34d0, 0x002a_34d0, ee::s08::OVERRIDE_002A34D0, ee::s08::f_002a34d0__generated;
        oc1009_fake_002a45c0, 0x002a_45c0, ee::s08::OVERRIDE_002A45C0, ee::s08::f_002a45c0__generated;
        oc1009_fake_00233640, 0x0023_3640, ee::s06::OVERRIDE_00233640, ee::s06::f_00233640__generated;
    }

    /// The logged call, with its result taken from the target's queue.
    fn oc1009_fake(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        let idx = OC1009_TARGETS.iter().position(|t| *t == target).unwrap() as u32;
        let k = c.bus.read32(OC1009_CNT + 4 * idx);
        c.bus.write32(OC1009_CNT + 4 * idx, k + 1);
        if k < 16 {
            let w = c.bus.read32(OC1009_Q + 0x40 * idx + 4 * k);
            if w == 1 {
                c.g[V0][0] = 0;
            } else if w != 0 {
                c.g[V0][0] = sx(w);
            }
        }
    }

    /// The results of the successive calls to `target` (see `OC1009_Q`).
    fn oc1009_queue(c: &mut Cpu, target: u32, vals: &[u32]) {
        let idx = OC1009_TARGETS.iter().position(|t| *t == target).unwrap() as u32;
        for (k, v) in vals.iter().enumerate() {
            c.bus.write32(OC1009_Q + 0x40 * idx + 4 * k as u32, *v);
        }
    }

    /// Hooked fakes, scrambled registers, the gate on.
    fn oc1009_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1009_hook();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1009_CTRL, OC1009_MAGIC);
        r
    }

    /// The app object (`gp-0x854`) with its UI vtable: slots `+0x160` and
    /// `+0x170` point at fake method addresses outside the function table.
    fn oc1009_app_object(c: &mut Cpu, delta: u16) {
        c.bus.write32(GAME_GP - 0x854, OC1009_APP);
        c.bus.write32(OC1009_APP + 0x10d8, OC1009_APP_VT);
        c.bus.write16(OC1009_APP_VT + 0x160, delta);
        c.bus.write32(OC1009_APP_VT + 0x164, fakes::FAKE);
        c.bus.write16(OC1009_APP_VT + 0x170, delta);
        c.bus.write32(OC1009_APP_VT + 0x174, fakes::FAKE + 0x10);
    }

    /// `cFELoadScreen_Load`: the three screens (id 0x27; no saved state; two
    /// players) and unwinds at every call.
    #[test]
    fn fe_load_screen_load_matches() {
        check(
            ee::s06::f_00233438__generated,
            fe_load_screen_load,
            120,
            &|c, seed| {
                let mut r = oc1009_machine(c, seed, 0x0023_3438, 14);
                oc1009_app_object(c, (r.next_u32() % 0x40) as u16);
                c.bus.write32(GAME_GP - 0x848, OC1009_GOBJ);
                c.bus.write32(OC1009_GOBJ + 0x88, r.next_u32());
                c.bus.write32(OC1009_GOBJ + 0x8c, r.next_u32());
                c.bus.write32(OC1009_GOBJ + 0xc0, OC1009_GM);
                let kind = seed % 4;
                c.bus
                    .write32(GAME_GP - 0x172c, if kind == 0 { 0x27 } else { 0x26 });
                c.bus
                    .write32(OC1009_GM + 0x9c, if kind == 1 { 0 } else { 1 });
                c.bus.write8(0x0053_5c11, if kind == 3 { 2 } else { 0 });
                c.g[A0][0] = OC1009_THIS as u64;
            },
        );
    }

    /// The rider-name tables the picture loader indexes with the (small) ids
    /// its callees return, and the options record at `0x534b30`.
    fn oc1009_lp_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = oc1009_machine(c, seed, salt, 22);
        oc1009_app_object(c, (r.next_u32() % 0x40) as u16);
        c.bus.write32(GAME_GP - 0x848, OC1009_GOBJ);
        c.bus.write32(OC1009_GOBJ + 0x78, r.next_u32());
        c.bus.write32(GAME_GP - 0x5b8, r.next_u32());
        c.bus.write32(GAME_GP - 0x5b4, r.next_u32());
        // the callee ids are small (they index a table)
        let ids: Vec<u32> = (0..4).map(|_| 2 + r.next_u32() % 6).collect();
        oc1009_queue(c, 0x0014_7398, &ids);
        oc1009_queue(c, 0x0014_7488, &ids);
        oc1009_queue(c, 0x0014_6e98, &ids);
        // the picture data: present or missing, per call
        let missing = |bit: u32| if (seed >> bit) & 1 == 1 { 1 } else { 0 };
        oc1009_queue(c, 0x003e_2190, &[missing(3), missing(4)]);
        let (on, alt) = ((seed >> 5) & 1, (seed >> 6) & 1);
        c.bus.write32(0x0053_4b30, on);
        c.bus.write32(0x0053_4b38, alt);
        r
    }

    /// `cFELoadScreen_LoadPictures`: type 1 (options on/off, alternate rider
    /// on/off), type 2 with one and two players, another type; present and
    /// missing picture data.
    #[test]
    fn fe_load_screen_load_pictures_matches() {
        check(
            ee::s06::f_00233640__generated,
            fe_load_screen_load_pictures,
            320,
            &|c, seed| {
                let _r = oc1009_lp_setup(c, seed, 0x0023_3640);
                let ty = [1u64, 1, 2, 3][(seed % 4) as usize];
                c.g[A0][0] = OC1009_THIS as u64;
                c.g[A1][0] = ty;
                c.bus
                    .write8(0x0053_5c11, if (seed >> 7) & 1 == 1 { 2 } else { 0 });
            },
        );
    }

    /// The saved result record the widgets read.
    const OC1009_RECORD: u32 = DATA + 0x22_0000;
    const OC1009_NAME_OBJ: u32 = DATA + 0x22_1000;
    const OC1009_NAME_VT: u32 = DATA + 0x22_2000;

    /// `cFELoadState_SetupResultWidget`: every arm of the name chain (arm `seed % 10`;
    /// 9 = no arm matches), with varied statistics and race times, and unwinds.
    #[test]
    fn fe_load_state_setup_result_widget_matches() {
        check(
            ee::s06::f_00247ab0__generated,
            fe_load_state_setup_result_widget,
            200,
            &|c, seed| {
                let mut r = oc1009_machine(c, seed, 0x0024_7ab0, 26);
                let widget = DATA + 0x22_3000;
                c.g[A0][0] = (DATA + 0x21_0000) as u64;
                c.g[A1][0] = widget as u64;
                let hash = 0x1234_5000 | (r.next_u32() & 0xfff);
                c.bus.write32(widget + 0x38, hash);
                // GetHashValue32 results: misses until the arm, a hit at the arm
                let arm = seed % 10;
                let mut q = [0u32; 16];
                for (k, slot) in q.iter_mut().enumerate() {
                    *slot = if k as u32 == arm { hash } else { hash ^ 0x55 };
                }
                oc1009_queue(c, 0x0031_7670, &q);
                oc1009_queue(c, 0x0015_5270, &[OC1009_RECORD]);
                for off in [0x0c, 0x18, 0x20, 0x28, 0x2c, 0x88] {
                    c.bus.write32(OC1009_RECORD + off, r.next_u32());
                }
                let secs =
                    [0.0f32, 61.5, 125.25, 3599.9, 1e10, -7.0, 59.0][r.next_u32() as usize % 7];
                c.bus.write32(OC1009_RECORD + 0x48, secs.to_bits());
                // arm 2's object, vtable and its method
                c.bus.write32(GAME_GP - 0x848, OC1009_GOBJ);
                c.bus.write32(OC1009_GOBJ + 0x8c, OC1009_NAME_OBJ);
                c.bus.write32(OC1009_NAME_OBJ + 4, OC1009_NAME_VT);
                c.bus
                    .write16(OC1009_NAME_VT + 0x20, (r.next_u32() % 0x40) as u16);
                c.bus.write32(OC1009_NAME_VT + 0x24, fakes::FAKE);
            },
        );
    }

    /// `SSXAudioSystem_dtor`: flags 0..3 (vtable fix-up copies with flags 0,
    /// base destructors with bit 1, delete with bit 0), with and without the
    /// big-file sync and the play history.
    #[test]
    fn ssx_audio_system_dtor_matches() {
        check(
            ee::s07::f_00285210__generated,
            ssx_audio_system_dtor,
            160,
            &|c, seed| {
                let mut r = oc1009_machine(c, seed, 0x0028_5210, 12);
                let this = DATA + 0x23_0000;
                let (ptr, bm, ss) = (DATA + 0x23_1000, DATA + 0x23_2000, DATA + 0x23_3000);
                c.bus.write32(this + 0x118, ptr);
                c.bus.write32(ptr, bm);
                c.bus.write32(bm + 0x1d8, ss);
                c.bus.write32(
                    this + 0x5728,
                    if (seed >> 2) & 1 == 1 {
                        r.next_u32() | 1
                    } else {
                        0
                    },
                );
                c.bus.write32(
                    this + 0x5980,
                    if (seed >> 3) & 1 == 1 {
                        r.next_u32() | 1
                    } else {
                        0
                    },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }
}
