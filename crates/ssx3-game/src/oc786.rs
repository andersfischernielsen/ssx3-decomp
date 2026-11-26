//! Staging module oc786 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc786`.

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
        0x0020_1750,
        event_select_init_peak_locks,
        "EventSelect_InitPeakLocks",
    ),
    (0x0018_6a08, cfe_state_mem_card_ctor, "cFEStateMemCard_ctor"),
    (
        0x002e_ff98,
        visual_effects_man_ctor,
        "VisualEffectsMan_ctor",
    ),
    (
        0x0019_ed80,
        fe_module_rider_preview_setup_model,
        "FEModule_RiderPreviewSetupModel",
    ),
];

// 0x002f5298 WorldLightMan_AllocOverlapList: already ported (game/visualfx/worldlightman.rs), skipped.

/// `SSXAudioSystem_SelectRandomSong(sys)` (0x0028d488): pick a random
/// front-end song. `+0x508` (current song) = `(rand & 0x7fff) * numSongs /
/// 0x7fff` (`+0x504` is the song count); then step the music group
/// `(rand & 0x7fff) * 10 / 0x7fff + 1` times with `SndBankMan` calls; when the
/// new song equals the previous one (`+0x62a4`) step once more, and if it is
/// still equal ask the bank manager to stop (`SndBank(.., 0, 0)`).
///
/// Not installed: its real callees (`SSXAudioSystem_GetTrackMusicGroup` and
/// `0x2b42b8`) have no override slot to fake and run away on test data, so
/// there is no bounded differential test.
#[allow(dead_code)]
pub fn ssx_audio_system_select_random_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x7fff;
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
        c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x504));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x62a4));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_df60, 0x0028_d4c0)?;
        c.g[V0][0] &= 0x7fff;

        c.cycles += 14;
        c.mult(0, V0, V0, S1);
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
        c.divu(0, V0, S0);
        c.g[V0][0] = c.lo[0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x508), c.g[V0][0]);
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_df60, 0x0028_d4ec)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] &= 0x7fff;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff;

        c.cycles += 5;
        c.divu(0, V0, S0);
        c.g[V1][0] = c.lo[0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[S0][0] = c.g[V0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            loop {
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                Callee {
                    addr: 0x0028_d8a0,
                    f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
                }
                .call(c, 0x0028_d528)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = 1;
                call_ptr(c, 0x002b_42b8, 0x0028_d538)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_d520);
                }
            }
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x508));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        if c.g[S3][0] != c.g[V0][0] {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x0028_d8a0,
                f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
            }
            .call(c, 0x0028_d554)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002b_42b8, 0x0028_d564)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x508));
            if c.g[S3][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x40);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x002b_42b8, 0x0028_d580)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `EventSelect_InitPeakLocks(self)` (0x00201750): mark the three peaks of
/// the event-select map unlocked (`+4/+8/+0xc = 1`), then for peaks 1 and 2
/// query the back end (interfaces 0 and 1 of `cBE`): in two-player path mode
/// (byte `0x535c11 == 2`) a peak stays unlocked unless both players have it
/// locked, otherwise the single-player lock query decides; a locked peak
/// clears `self->+8 + 4*(peak-1)`.
///
/// The resume entries (`pc != entry`) stay with the generated `__entries`.
pub fn event_select_init_peak_locks(c: &mut Cpu) {
    if c.pc != 0x0020_1750 {
        return ee::s05::f_00201750__entries(c);
    }
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        frame.save(c, S4, 0x20);
        c.g[S3][0] = 1;
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S5][0] = 2;
        call_ptr(c, 0x0014_dc80, 0x0020_1784)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0020_1790)?;
        call_ptr(c, 0x0014_dc80, 0x0020_1798)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0020_17a4)?;
        c.g[V1][0] = 1;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x53_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
        c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V1][0]);
        loop {
            c.cycles += 5;
            c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            let single = c.g[V0][0] != c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            let mut check_lock = true;
            if single {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0014_6008, 0x0020_1828)?;
            } else {
                c.cycles += 10;
                call_ptr(c, 0x0014_7488, 0x0020_17dc)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x0014_5f90, 0x0020_17f0)?;
                c.g[A0][0] = c.g[S1][0];
                let first_free = c.g[V0][0] == 0;
                c.g[A1][0] = 1;
                if first_free {
                    check_lock = false;
                } else {
                    c.cycles += 9;
                    call_ptr(c, 0x0014_7488, 0x0020_1804)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0014_5f90, 0x0020_1818)?;
                }
            }
            if check_lock {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(c.g[S2][0] as u32, 0);
                }
            }
            c.cycles += 4;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 3) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0020_17c0);
                }
                continue;
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
            return Ok(());
        }
    })
}

/// `cFEStateMemCard::cFEStateMemCard(this)` (0x00186a08): run the cFEMemCard
/// base constructor (engine 2), set the vtable (0x46b570), register the
/// instance in `gp-0x1c38`, clear the profile file and stage globals, set the
/// popup and "missing file" defaults, and hook the nine callbacks (and four
/// flags) into the memory-card overlay manager.
pub fn cfe_state_mem_card_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 67;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 2;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_53b0, 0x0018_6a24)?;
        c.g[S1][0] = 1;
        c.g[V0][0] = 0x47_0000;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4a90));
        c.g[V1][0] = u64::MAX;
        let this = c.g[S0][0] as u32;
        let gp = c.g[GP][0] as u32;
        c.sw(this.wrapping_add(8), c.g[V0][0]);
        c.g[A1][0] = 0x1a;
        c.sw(gp.wrapping_sub(0x1c38), c.g[S0][0]);
        c.sw(gp.wrapping_sub(0x1c3c), 0);
        c.sw(gp.wrapping_sub(0x1c34), 0);
        c.sw(this.wrapping_add(0x204), c.g[S1][0]);
        c.sw(this.wrapping_add(0xc), c.g[S1][0]);
        c.sw(this.wrapping_add(0x23c), c.g[S1][0]);
        c.sw(this.wrapping_add(0x230), 0);
        c.sw(this.wrapping_add(0x234), 0);
        c.sw(this.wrapping_add(0x240), 0);
        c.sw(this.wrapping_add(0x244), 0);
        c.sw(this.wrapping_add(0x19c), c.g[A1][0]);
        c.sw(this.wrapping_add(0x238), c.g[V1][0]);
        c.sw(this.wrapping_add(0x22c), c.g[V1][0]);
        call_ptr(c, 0x0022_7f80, 0x0018_6a78)?;
        // v1 = the memory-card overlay manager (SSXApp->getMCManager()).
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x18_0000;
        let mc = c.g[V1][0] as u32;
        c.sw(mc.wrapping_add(0x120), c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(mc.wrapping_add(0x12c), c.g[S1][0]);
        c.g[A1][0] = 0x18_0000;
        c.sw(mc.wrapping_add(0x124), c.g[S1][0]);
        for r in [A2, A3, T0, T1, T2, T3, T4] {
            c.g[r][0] = 0x18_0000;
        }
        let callbacks: [(usize, u32, u32); 9] = [
            (A0, 0x6f90, 0xa4),
            (A1, 0x7148, 0x38),
            (A2, 0x7180, 0x94),
            (A3, 0x7230, 0xac),
            (T0, 0x71a8, 0x90),
            (T1, 0x6c98, 0x48),
            (T2, 0x6d60, 0x3c),
            (T3, 0x6f40, 0xb0),
            (T4, 0x7360, 0x5c),
        ];
        for (r, add, _) in callbacks {
            c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(add));
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        for (r, _, off) in callbacks {
            c.sw(mc.wrapping_add(off), c.g[r][0]);
        }
        c.sw(mc.wrapping_add(0x11c), 0);
        c.sw(mc.wrapping_add(0x128), 0);
        frame.close(c);
        Ok(())
    })
}

/// `cVisualEffectsMan::cVisualEffectsMan(this)` (0x002eff98): set the vtable
/// (0x487da0), clear its 17 fields, register it in `gp+0xa88`, set the two
/// texture/blend globals (`gp+0x14e4 = 0x37`, `gp+0x134c = 0x30`) and fill in
/// the render-state table (`gp+0x2a90`, 0x20 bytes per entry): entries 1..4
/// get texture ids 0x2e/0x2f/0x31/0x44 at `+0x4`, entries 5..19 are cleared.
///
/// The resume entries (`pc != entry`) stay with the generated `__entries`.
pub fn visual_effects_man_ctor(c: &mut Cpu) {
    if c.pc != 0x002e_ff98 {
        return ee::s09::f_002eff98__entries(c);
    }
    c.cycles += 66;
    c.g[V0][0] = 0x48_0000;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7da0));
    let this = c.g[A0][0] as u32;
    let gp = c.g[GP][0] as u32;
    c.sw(this, 0);
    c.sw(this.wrapping_add(0x44), c.g[V0][0]);
    c.g[T2][0] = 0x37;
    c.sw(this.wrapping_add(4), 0);
    c.g[T0][0] = 0x30;
    c.sw(this.wrapping_add(8), 0);
    c.g[T1][0] = 0x2e;
    c.sw(this.wrapping_add(0xc), 0);
    c.g[A3][0] = 0x31;
    c.sw(this.wrapping_add(0x10), 0);
    c.g[A2][0] = 0x2f;
    c.sw(this.wrapping_add(0x14), 0);
    c.g[A1][0] = 0x44;
    c.sw(this.wrapping_add(0x18), 0);
    c.g[V0][0] = c.g[A0][0];
    for off in (0x1c..=0x40).step_by(4) {
        c.sw(this.wrapping_add(off), 0);
    }
    c.sw(gp.wrapping_add(0x14e4), c.g[T2][0]);
    c.sw(gp.wrapping_add(0x134c), c.g[T0][0]);
    c.sw(gp.wrapping_add(0xa88), c.g[A0][0]);
    let rs = c.g[V1][0] as u32;
    c.sw(rs.wrapping_add(0x24), c.g[T1][0]);
    c.sw(rs.wrapping_add(0x44), c.g[A3][0]);
    c.sw(rs.wrapping_add(0x84), c.g[A2][0]);
    c.sw(rs.wrapping_add(0x64), c.g[A1][0]);
    // Entries 5..=19: `+4` and `+8` cleared (entry 8's `+8` is at +0x268
    // in the generated order; the last store, +0x264, sits in the delay slot
    // of the return and is done after `ra` is read: same effect).
    const CLEARS: [u32; 30] = [
        0xa4, 0xa8, 0xc4, 0xc8, 0xe4, 0xe8, 0x104, 0x108, 0x268, 0x124, 0x128, 0x144, 0x148, 0x164,
        0x168, 0x184, 0x188, 0x1a4, 0x1a8, 0x1c4, 0x1c8, 0x1e4, 0x1e8, 0x204, 0x208, 0x224, 0x228,
        0x244, 0x248, 0x264,
    ];
    for off in CLEARS {
        c.sw(rs.wrapping_add(off), 0);
    }
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

/// `FEModule_RiderPreviewSetupModel(self)` (0x0019ed80): flag the model as
/// compiled (`+0xcb4 = 1`), set up the texture files and refs, compile the
/// anim model (`+8`), allocate the 0x70-byte rider anim (`new` with the
/// "RiderAnim" name), give it the cRiderAnim vtable (0x458458, slot `+0x58`),
/// link it to the rider (`+0xc`), play animation 434, set it up and transform
/// the anim model with an identity pose copied to the stack, then find the
/// bones.
pub fn fe_module_rider_preview_setup_model(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x50);
        c.g[A1][0] = 0;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x40);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xcb4), c.g[V0][0]);
        call_ptr(c, 0x0036_9890, 0x0019_edac)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0036_9690, 0x0019_edbc)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        call_ptr(c, 0x0030_dbd0, 0x0019_edc8)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A3][0] = 0;
        c.g[A2][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb28));
        c.g[A0][0] = 0x70;
        call_ptr(c, 0x0031_7d70, 0x0019_ede0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_18d0, 0x0019_edec)?;
        c.g[V1][0] = 0x46_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7ba8));
        c.g[A0][0] = c.g[S0][0];
        let anim = c.g[S0][0] as u32;
        c.sw(anim.wrapping_add(0x58), c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(anim.wrapping_add(0x60), 0);
        c.sw(anim.wrapping_add(0x64), 0);
        call_ptr(c, 0x0010_4d50, 0x0019_ee10)?;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x1b2;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = 0;
        call_ptr(c, 0x0031_28e8, 0x0019_ee28)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0031_2598, 0x0019_ee34)?;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 1;
        c.g[V0][0] = 0x50_0000;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xdd0));
        c.g[A1][0] = c.g[SP][0];
        // The identity pose (two 0x20-byte halves) copied from 0x4ff230 to
        // the stack, twice.
        let pose = c.g[V0][0] as u32;
        c.lq(6, pose.wrapping_add(0x10));
        c.lq(3, pose);
        frame.save(c, 6, 0x10);
        frame.save(c, 3, 0);
        frame.save(c, 3, 0x20);
        frame.save(c, 6, 0x30);
        call_ptr(c, 0x0031_0200, 0x0019_ee6c)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0019_e680,
            f: ee::s03::FEModule_RiderPreviewFindBones,
        }
        .call(c, 0x0019_ee74)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};

    /// A game-like object area well above the fakes' pointer results.
    const OC786_OBJ: u32 = DATA + 0x8_0000;
    /// Per-machine mask word: bit n set forces the n-th `0x145f90`/`0x146008`
    /// call of `event_select_init_peak_locks` to return 0.
    const OC786_ZERO_MASK: u32 = DATA + 0x20_0000;
    const OC786_ZERO_COUNT: u32 = DATA + 0x20_0004;

    /// The word that arms oc786's callee fakes in a machine's RAM.
    const OC786_CTRL: u32 = DATA + 0x30_0000;
    const OC786_MAGIC: u32 = 0x0786_f4ce;

    /// Gated fake body: log like `fakes::fake_call`; the lock queries
    /// (0x145f90, 0x146008) may then return 0 per the machine's mask.
    fn oc786_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && (addr == 0x0014_5f90 || addr == 0x0014_6008) {
            let k = c.bus.read32(OC786_ZERO_COUNT);
            c.bus.write32(OC786_ZERO_COUNT, k + 1);
            if (c.bus.read32(OC786_ZERO_MASK) >> (k & 31)) & 1 != 0 {
                c.g[V0][0] = 0;
            }
        }
    }

    macro_rules! oc786_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: std::sync::atomic::AtomicUsize =
                    std::sync::atomic::AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC786_CTRL) != OC786_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc786_fake_call(c, $addr);
                }
            )*
            fn oc786_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc786_fakes!(
        (
            oc786_fk_14dc80,
            OC786_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc786_fk_14dd58,
            OC786_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc786_fk_147488,
            OC786_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc786_fk_145f90,
            OC786_PREV_145F90,
            ee::s01::OVERRIDE_00145F90,
            ee::s01::f_00145f90__generated,
            0x0014_5f90
        ),
        (
            oc786_fk_146008,
            OC786_PREV_146008,
            ee::s01::OVERRIDE_00146008,
            ee::s01::f_00146008__generated,
            0x0014_6008
        ),
        (
            oc786_fk_1d53b0,
            OC786_PREV_1D53B0,
            ee::s04::OVERRIDE_001D53B0,
            ee::s04::f_001d53b0__generated,
            0x001d_53b0
        ),
        (
            oc786_fk_227f80,
            OC786_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc786_fk_369890,
            OC786_PREV_369890,
            ee::s12::OVERRIDE_00369890,
            ee::s12::f_00369890__generated,
            0x0036_9890
        ),
        (
            oc786_fk_369690,
            OC786_PREV_369690,
            ee::s12::OVERRIDE_00369690,
            ee::s12::f_00369690__generated,
            0x0036_9690
        ),
        (
            oc786_fk_30dbd0,
            OC786_PREV_30DBD0,
            ee::s10::OVERRIDE_0030DBD0,
            ee::s10::f_0030dbd0__generated,
            0x0030_dbd0
        ),
        (
            oc786_fk_317d70,
            OC786_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc786_fk_3118d0,
            OC786_PREV_3118D0,
            ee::s10::OVERRIDE_003118D0,
            ee::s10::f_003118d0__generated,
            0x0031_18d0
        ),
        (
            oc786_fk_104d50,
            OC786_PREV_104D50,
            ee::s00::OVERRIDE_00104D50,
            ee::s00::f_00104d50__generated,
            0x0010_4d50
        ),
        (
            oc786_fk_3128e8,
            OC786_PREV_3128E8,
            ee::s10::OVERRIDE_003128E8,
            ee::s10::f_003128e8__generated,
            0x0031_28e8
        ),
        (
            oc786_fk_312598,
            OC786_PREV_312598,
            ee::s10::OVERRIDE_00312598,
            ee::s10::f_00312598__generated,
            0x0031_2598
        ),
        (
            oc786_fk_310200,
            OC786_PREV_310200,
            ee::s10::OVERRIDE_00310200,
            ee::s10::f_00310200__generated,
            0x0031_0200
        ),
        (
            oc786_fk_19e680,
            OC786_PREV_19E680,
            ee::s03::OVERRIDE_0019E680,
            ee::s03::f_0019e680__generated,
            0x0019_e680
        ),
    );

    /// Arm the gated fakes in this machine and install them once.
    fn oc786_arm(c: &mut Cpu) {
        oc786_install_fakes();
        c.bus.write32(OC786_CTRL, OC786_MAGIC);
    }

    /// `EventSelect_InitPeakLocks`: both game paths, all lock results.
    #[test]
    fn event_select_init_peak_locks_matches() {
        fakes::covered(
            ee::s05::f_00201750__generated,
            event_select_init_peak_locks,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x201750, 14);
                oc786_arm(c);
                c.pc = 0x0020_1750;
                c.g[A0][0] = sx(OC786_OBJ);
                c.bus.write8(0x0053_5c11, if seed % 2 == 0 { 2 } else { 1 });
                c.bus.write32(OC786_ZERO_MASK, r.next_u32());
                for i in 0..3 {
                    c.bus.write32(OC786_OBJ + 4 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `cFEStateMemCard_ctor`: the manager pointer is forced into the object area.
    #[test]
    fn cfe_state_mem_card_ctor_matches() {
        fakes::covered(
            ee::s02::f_00186a08__generated,
            cfe_state_mem_card_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186a08, 3);
                oc786_arm(c);
                c.g[A0][0] = sx(OC786_OBJ);
                c.bus.write32(fakes::LOG_RESULTS + 4, OC786_OBJ + 0x2000);
                c.bus.write32(OC786_OBJ + 0x19c, r.next_u32());
            },
        );
    }

    /// `VisualEffectsMan_ctor`: a leaf; the render-state table sits at the pointer in `gp+0x2a90`.
    #[test]
    fn visual_effects_man_ctor_matches() {
        check(
            ee::s09::f_002eff98__generated,
            visual_effects_man_ctor,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2eff98, 0);
                c.pc = 0x002e_ff98;
                c.g[A0][0] = sx(OC786_OBJ);
                c.bus
                    .write32(c.g[GP][0] as u32 + 0x2a90, OC786_OBJ + 0x1000);
                for i in 0..0x80 {
                    c.bus.write32(OC786_OBJ + 0x1000 + 4 * i, r.next_u32());
                    c.bus.write32(OC786_OBJ + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `FEModule_RiderPreviewSetupModel`: all callees faked (the rider anim comes from a fake `new`).
    #[test]
    fn fe_module_rider_preview_setup_model_matches() {
        fakes::covered(
            ee::s03::f_0019ed80__generated,
            fe_module_rider_preview_setup_model,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19ed80, 11);
                oc786_arm(c);
                c.g[A0][0] = sx(OC786_OBJ);
                c.bus.write32(OC786_OBJ + 0x8, OC786_OBJ + 0x1000);
                c.bus.write32(OC786_OBJ + 0x18, r.next_u32());
                // The fake `new` (call 3) returns the anim object.
                c.bus
                    .write32(fakes::LOG_RESULTS + 4 * 3, OC786_OBJ + 0x3000);
            },
        );
    }
}
