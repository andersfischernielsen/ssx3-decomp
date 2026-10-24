//! `src/fe/ovtemplatepausemenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::lbu;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::wscript::wscriptman::gp;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_85c0,
        ov_screen_pause_freeride_on_activate,
        "OVScreenPauseFreeride_OnActivate",
    ),
    (
        0x001f_8908,
        ov_screen_pause_freeride_on_widget_notify,
        "OVScreenPauseFreeride_OnWidgetNotify",
    ),
    (
        0x001f_8ee0,
        ov_screen_pause_freeride_init_mode1,
        "OVScreenPauseFreeride_InitMode1",
    ),
    (
        0x001f_8f48,
        ov_screen_pause_freeride_init_mode2,
        "OVScreenPauseFreeride_InitMode2",
    ),
    (
        0x001f_8fb0,
        ov_screen_pause_freeride_init_mode3,
        "OVScreenPauseFreeride_InitMode3",
    ),
    (
        0x001f_9018,
        ov_screen_pause_freeride_init_mode4,
        "OVScreenPauseFreeride_InitMode4",
    ),
    (
        0x001f_9080,
        ov_screen_pause_freeride_init_mode5,
        "OVScreenPauseFreeride_InitMode5",
    ),
    (
        0x001f_90e8,
        ov_screen_pause_freeride_init_mode_6,
        "OVScreenPauseFreeride_InitMode6",
    ),
    (
        0x0028_9bb8,
        ssx_audio_system_resume_game_audio,
        "SSXAudioSystem_ResumeGameAudio",
    ),
];

/// `int OVScreenPauseFreeride_OnWidgetNotify(...)` (0x1F8908): `Resume`
/// (6) answers 0, `Restart`/`Quit` (8, 9) answer 0x100, anything else
/// 0x101.
pub fn ov_screen_pause_freeride_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if is_six {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_six = c.g[V0][0] != 0;
    c.g[V0][0] = (c.g[A2][0] < 0xa) as u64;
    if !below_six {
        c.cycles += 2;
        let at_least_ten = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[A2][0] < 8) as u64;
        if at_least_ten {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_IGNORED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let below_eight = c.g[V0][0] != 0;
        c.g[V0][0] = WIDGET_ACTIVATED;
        if below_eight {
            c.cycles += 2;
            c.g[V0][0] = WIDGET_IGNORED;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = WIDGET_IGNORED;
    abi::ret(c);
}

/// `void SSXAudioSystem_ResumeGameAudio(SSXAudioSystem* audio)` (0x289bb8):
/// resume the paused game-audio streams and clear their pause flags.
pub fn ssx_audio_system_resume_game_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let audio = c.g[A0][0];
        c.g[S0][0] = audio;
        c.g[V0][0] = Addr(at(c, S0, 0x5fb4)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[A0][0] = sx(at(c, S0, 0x118));
            call_ptr(c, 0x002b_3a98, 0x0028_9bdc)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0029_ce70, 0x0028_9be4)?;
            c.g[V0][0] = Addr(at(c, S0, 0x5828)).lw(c);
            let quiet = c.g[V0][0] == 0;
            Addr(at(c, S0, 0x5fb4)).sw(c, 0);
            if !quiet {
                c.cycles += 6;
                c.g[A0][0] = sx(at(c, S0, 0x5560));
                c.g[A1][0] = 0;
                call_ptr(c, 0x002b_11b0, 0x0028_9bfc)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_4550, 0x0028_9c04)?;
                Addr(at(c, S0, 0x5828)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory helpers.

/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

#[inline(always)]
pub(crate) fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v);
}

// ---------------------------------------------------------------------------
// Pause-menu template initialisers (five modes, same shape).

pub(crate) fn pause_freeride_init(
    c: &mut Cpu,
    slots: u64,
    table: u32,
    mode: u64,
    head: u32,
    ret: u32,
) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    let this = c.g[A0][0];
    c.g[A1][0] = this;
    frame.save_ra(c, 0);
    c.g[V0][0] = slots;
    c.g[V1][0] = 0x0048_0000;
    sw(c, p(this as u32, 0xbc), c.g[V0][0]);
    c.g[V1][0] = sx(table);
    c.g[A2][0] = 0;
    c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
    loop {
        c.cycles += 8;
        c.g[V0][0] = lw(c, c.g[V1][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        sw(c, c.g[A0][0] as u32, c.g[V0][0]);
        c.g[V0][0] = lw(c, p(this as u32, 0xbc));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 7;
            c.g[V0][0] = mode;
            c.g[A0][0] = this;
            sw(c, p(this as u32, 0x110), c.g[V0][0]);
            call_ptr(c, 0x001f_8168, ret)?;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `OVScreenPauseFreeride_InitMode1(OVScreenPauseFreeride* self)` (0x1f8ee0).
pub fn ov_screen_pause_freeride_init_mode1(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 6, 0x0047_8160, 1, 0x001f_8f08, 0x001f_8f38)
    })
}

/// `OVScreenPauseFreeride_InitMode2(OVScreenPauseFreeride* self)` (0x1f8f48).
pub fn ov_screen_pause_freeride_init_mode2(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 6, 0x0047_8178, 2, 0x001f_8f70, 0x001f_8fa0)
    })
}

/// `OVScreenPauseFreeride_InitMode3(OVScreenPauseFreeride* self)` (0x1f8fb0).
pub fn ov_screen_pause_freeride_init_mode3(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 7, 0x0047_8140, 3, 0x001f_8fd8, 0x001f_9008)
    })
}

/// `OVScreenPauseFreeride_InitMode4(OVScreenPauseFreeride* self)` (0x1f9018).
pub fn ov_screen_pause_freeride_init_mode4(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 5, 0x0047_8190, 4, 0x001f_9040, 0x001f_9070)
    })
}

/// `OVScreenPauseFreeride_InitMode5(OVScreenPauseFreeride* self)` (0x1f9080).
pub fn ov_screen_pause_freeride_init_mode5(c: &mut Cpu) {
    guest(c, |c| {
        pause_freeride_init(c, 5, 0x0047_81a8, 5, 0x001f_90a8, 0x001f_90d8)
    })
}

/// `void OVScreenPauseFreeride_InitMode6(OVScreenPauseFreeride* self)`
/// (0x1f90e8): pick the item table (two when a restart is possible), copy it
/// into `+0x9c`, set the page to 6 and run the screen init.
pub fn ov_screen_pause_freeride_init_mode_6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x0048_0000;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e40));
        sw_2(c, p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
        c.g[V0][0] = lw_2(c, gp(c, -0x848));
        c.g[A0][0] = lw_2(c, p(c.g[V0][0] as u32, 0x84));
        call_ptr(c, 0x0023_1ab8, 0x001f_9118)?; // GameModule_CanRestartFromPause
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xbc));
        } else {
            c.cycles += 4;
            c.g[V0][0] = 2;
            c.g[S1][0] = sx(gp(c, -0x888));
            sw_2(c, p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
            c.g[V0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xbc));
        }
        c.cycles += 2;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V1][0] = c.g[S1][0];
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            loop {
                c.cycles += 8;
                c.g[V0][0] = lw_2(c, c.g[V1][0] as u32);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                sw_2(c, c.g[A0][0] as u32, c.g[V0][0]);
                c.g[V0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xbc));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_9140);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 6;
        c.g[A0][0] = c.g[S0][0];
        sw_2(c, p(c.g[S0][0] as u32, 0x110), c.g[V0][0]);
        call_ptr(c, 0x001f_8168, 0x001f_9170)?; // OVScreenPauseFreeride_Init
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_001F85C0(void* self, void* a1)` (0x1f85c0): activate the pause
/// menu's freeride variant: for each of the eight entries at_2 `+0xd8`, pick the
/// text/colour pair (the "selected" or "normal" one), format the entry's
/// label and hand it to the screen's text setter; then let the screen finish
/// the activation.
pub fn ov_screen_pause_freeride_on_activate(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x001f85c0 => {
                c.cycles += 21;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0xc0));
                sq(c, 17, at_2(c, SP, 0xa0));
                c.g[S1][0] = c.g[A0][0];
                sq(c, 16, at_2(c, SP, 0xb0));
                sq(c, 18, at_2(c, SP, 0x90));
                c.g[S0][0] = sx(at_2(c, S1, 0x9c));
                sq(c, 19, at_2(c, SP, 0x80));
                c.g[S2][0] = 0u64;
                sq(c, 20, at_2(c, SP, 0x70));
                c.g[S3][0] = sx(at_2(c, S1, 0xd8));
                sq(c, 21, at_2(c, SP, 0x60));
                c.g[S4][0] = c.g[A1][0];
                sq(c, 22, at_2(c, SP, 0x50));
                c.g[S5][0] = 0x4d0000;
                sq(c, 23, at_2(c, SP, 0x40));
                c.g[S6][0] = 0x4d0000;
                sq(c, 30, at_2(c, SP, 0x30));
                c.g[S7][0] = 0x4d0000;
                sd(c, at_2(c, SP, 0x20), c.g[RA][0]);
                let t = true;
                c.g[FP][0] = 0x4d0000;
                if t {
                    pc = 0x001f86a0;
                    continue;
                }
                pc = 0x001f8614;
                continue;
            }
            0x001f8614 => {
                c.cycles += 1;
                pc = 0x001f8618;
                continue;
            }
            0x001f8618 => {
                c.cycles += 2;
                let t = c.g[A2][0] == 0u64;
                c.g[V0][0] = c.lbu(r32(c, GP).wrapping_sub(0xc88));
                if t {
                    pc = 0x001f8698;
                    continue;
                }
                pc = 0x001f8620;
                continue;
            }
            0x001f8620 => {
                c.cycles += 2;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[V0][0] = lw_3(c, r32(c, S0));
                if t {
                    pc = 0x001f8650;
                    continue;
                }
                pc = 0x001f8628;
                continue;
            }
            0x001f8628 => {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw_3(c, r32(c, V0));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = sx(r32(c, FP).wrapping_sub(0x7508));
                if t {
                    pc = 0x001f8644;
                    continue;
                }
                pc = 0x001f863c;
                continue;
            }
            0x001f863c => {
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = lw_3(c, at_2(c, A2, 0x8));
                if t {
                    pc = 0x001f8668;
                    continue;
                }
                pc = 0x001f8644;
                continue;
            }
            0x001f8644 => {
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, at_2(c, A2, 0x8));
                let t = true;
                c.g[A1][0] = sx(r32(c, S7).wrapping_sub(0x7518));
                if t {
                    pc = 0x001f8668;
                    continue;
                }
                pc = 0x001f8650;
                continue;
            }
            0x001f8650 => {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw_3(c, r32(c, V0));
                let t = c.g[V1][0] == 0u64;
                c.g[A1][0] = sx(r32(c, S6).wrapping_sub(0x74e8));
                if t {
                    pc = 0x001f8680;
                    continue;
                }
                pc = 0x001f8664;
                continue;
            }
            0x001f8664 => {
                c.cycles += 1;
                c.g[V0][0] = lw_3(c, at_2(c, A2, 0x8));
                pc = 0x001f8668;
                continue;
            }
            0x001f8668 => {
                c.cycles += 6;
                c.g[A0][0] = lh(c, at_2(c, V0, 0x58));
                c.g[V1][0] = lw_3(c, at_2(c, V0, 0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f8678);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x001f8678).is_err() {
                    return;
                }
                let t = true;
                c.g[S0][0] = sx(at_2(c, S0, 0x4));
                if t {
                    pc = 0x001f869c;
                    continue;
                }
                pc = 0x001f8680;
                continue;
            }
            0x001f8680 => {
                c.cycles += 6;
                c.g[V0][0] = lw_3(c, at_2(c, A2, 0x8));
                c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x74f8));
                c.g[A0][0] = lh(c, at_2(c, V0, 0x58));
                c.g[V1][0] = lw_3(c, at_2(c, V0, 0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f8698);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x001f8698).is_err() {
                    return;
                }
                pc = 0x001f8698;
                continue;
            }
            0x001f8698 => {
                c.cycles += 1;
                c.g[S0][0] = sx(at_2(c, S0, 0x4));
                pc = 0x001f869c;
                continue;
            }
            0x001f869c => {
                c.cycles += 1;
                c.g[S2][0] = sx(at_2(c, S2, 0x1));
                pc = 0x001f86a0;
                continue;
            }
            0x001f86a0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x4a0000;
                if t {
                    pc = 0x001f86e4;
                    continue;
                }
                pc = 0x001f86ac;
                continue;
            }
            0x001f86ac => {
                c.cycles += 14;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f86bc);
                c.g[A1][0] = sx(at_2(c, V0, 0x2138));
                if call_ptr(c, 0x00417828, 0x001f86bc).is_err() {
                    return;
                }
                c.g[RA][0] = sx(0x001f86c4);
                c.g[A0][0] = c.g[SP][0];
                if call_ptr(c, 0x00317670, 0x001f86c4).is_err() {
                    return;
                }
                c.g[A0][0] = lw_3(c, at_2(c, S1, 0xc0));
                c.g[RA][0] = sx(0x001f86d0);
                c.g[A1][0] = c.g[V0][0];
                if call_ptr(c, 0x0039d860, 0x001f86d0).is_err() {
                    return;
                }
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = lw_3(c, at_2(c, S1, 0xbc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;

                if t {
                    if back_edge_unwinds(c, 0x001f8618) {
                        return;
                    }
                    pc = 0x001f8618;
                    continue;
                }
                pc = 0x001f86e4;
                continue;
            }
            0x001f86e4 => {
                c.cycles += 15;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001f86f0);
                c.g[A1][0] = c.g[S4][0];
                if call_ptr(c, 0x0039e4c0, 0x001f86f0).is_err() {
                    return;
                }
                lq(c, 16, at_2(c, SP, 0xb0));
                lq(c, 17, at_2(c, SP, 0xa0));
                lq(c, 18, at_2(c, SP, 0x90));
                lq(c, 19, at_2(c, SP, 0x80));
                lq(c, 20, at_2(c, SP, 0x70));
                lq(c, 21, at_2(c, SP, 0x60));
                lq(c, 22, at_2(c, SP, 0x50));
                lq(c, 23, at_2(c, SP, 0x40));
                lq(c, 30, at_2(c, SP, 0x30));
                c.g[RA][0] = ld(c, at_2(c, SP, 0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at_2(c, SP, 0xc0));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x001f85c0, 0x001f8720);
                return;
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn ov_screen_pause_freeride_on_widget_notify_matches() {
        check(
            ee::s04::f_001f8908__generated,
            ov_screen_pause_freeride_on_widget_notify,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x001f_8908, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_resume_game_audio_matches() {
        check(
            ssx3_ee::s07::f_00289bb8__generated,
            ssx_audio_system_resume_game_audio,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x289bb8, 0);
                w_2(c, OBJ + 0x5fb4, if seed % 3 == 0 { 0 } else { 1 });
                w_2(c, OBJ + 0x5828, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    /// `D_0048DCD4`: the immutable empty `cBXString` buffer.
    pub(crate) const EMPTY: u32 = 0x0048_dcd4;

    /// `D_004A3E90`: the global empty-`cBXString` pointer (`gp+0xda0`).
    pub(crate) const EMPTY_PTR: u32 = 0x004a_3e90;

    /// Scrambled registers, fake hooks, a game-like `gp`, the BELibrary dummy
    /// and the static empty string the string code compares against.
    pub(crate) fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        c.bus.write32(EMPTY_PTR, EMPTY);
        c.bus.write32(EMPTY - 0xc, 0xffff_ffff);
        c.bus.write32(EMPTY - 8, 0);
        c.bus.write32(EMPTY, 0);
        r
    }

    // -------------------------------------------------------- pause-menu modes

    /// The pause screen at `DATA` with a dummy UI engine and a null race
    /// manager. `cOVStateOverlay_Init` looks the engine up at `self+0x10` and
    /// hands `UIStateMgr_FindState(engine+0x18)` a *properly empty* `UIList`
    /// (sentinel at `+4`, `+8`), so the state walk returns at once instead of
    /// chasing the screen objects the null engine builds at address 0.
    pub(crate) fn pause_screen(c: &mut Cpu, seed: u32) {
        let engine = DATA + 0x8000;
        let world = DATA + 0x9000;
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x10, engine);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, 0);
        for off in [0x18u32, 0x34] {
            let list = engine + off;
            c.bus.write32(list + 4, list);
            c.bus.write32(list + 8, list);
        }
        c.bus.write32(engine + 0x10, 0);
        c.bus.write32(DATA + 0xbc, seed % 3);
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode1_matches() {
        check(
            ssx3_ee::s04::f_001f8ee0__generated,
            ov_screen_pause_freeride_init_mode1,
            8,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f8ee0);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode2_matches() {
        check(
            ssx3_ee::s04::f_001f8f48__generated,
            ov_screen_pause_freeride_init_mode2,
            8,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f8f48);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode3_matches() {
        check(
            ssx3_ee::s04::f_001f8fb0__generated,
            ov_screen_pause_freeride_init_mode3,
            8,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f8fb0);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode4_matches() {
        check(
            ssx3_ee::s04::f_001f9018__generated,
            ov_screen_pause_freeride_init_mode4,
            8,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f9018);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode5_matches() {
        check(
            ssx3_ee::s04::f_001f9080__generated,
            ov_screen_pause_freeride_init_mode5,
            8,
            &|c, seed| {
                let _r = setup(c, seed, 0x1f9080);
                pause_screen(c, seed);
            },
        );
    }

    #[test]
    fn ov_screen_pause_freeride_init_mode_6_matches() {
        check(
            ssx3_ee::s04::f_001f90e8__generated,
            ov_screen_pause_freeride_init_mode_6,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x001f_90e8);
                elf_image(c);
                heap(c);
                pool(c);
                let screen = DATA + 0x100;
                let engine = DATA + 0x2000;
                clear(c, screen, 0x300);
                clear(c, engine, 0x400);
                w(c, screen + 0x10, engine);
                w(c, screen + 0xbc, (seed % 4) as u32);
                c.g[A0][0] = sx(screen);
            },
        );
    }

    pub(crate) const PAUSE_MENU: u32 = DATA + 0x1_0000;

    pub(crate) const PAUSE_ENTRY: u32 = DATA + 0x1_1000;

    pub(crate) const PAUSE_TEXT: u32 = DATA + 0x1_2000;

    #[test]
    fn ov_screen_pause_freeride_on_activate_matches() {
        check_all(
            ee::s04::f_001f85c0__generated,
            ov_screen_pause_freeride_on_activate,
            20,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1f85_c0, 16);
                c.pc = 0x001f_85c0;
                c.g[A0][0] = sx(PAUSE_MENU);
                c.g[A1][0] = sx(DATA + 0x1_3000);
                c.bus.write32(PAUSE_MENU + 0xbc, (seed % 4) as u32);
                c.bus.write8(GAME_GP - 0xc88, (seed % 2) as u8);
                for i in 0..8u32 {
                    let e = if seed % 2 == 0 {
                        0
                    } else {
                        PAUSE_ENTRY + 0x40 * i
                    };
                    c.bus.write32(PAUSE_MENU + 0xd8 + 4 * i, e);
                    if e != 0 {
                        c.bus.write32(e + 0x8, 0);
                    }
                }
                c.bus.write32(PAUSE_MENU + 0xc0, PAUSE_TEXT);
                let _ = &mut r;
            },
        );
    }
}
