//! `src/fe/ovtemplatepausemenu.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festaterewards::p as p_m2;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::ovstatepause::oc1023_at;
pub(crate) use crate::game::wscript::wscriptman::gp;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
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
    (
        0x001f_8df8,
        ovscreen_pause_freeride_play_icon_anim,
        "OVScreenPauseFreeride_PlayIconAnim",
    ),
    (
        0x001f_8168,
        ov_screen_pause_freeride_init,
        "OVScreenPauseFreeride_Init",
    ),
    (
        0x001f_88e8,
        ov_screen_pause_freeride_shutdown,
        "OVScreenPauseFreeride_Shutdown",
    ),
    (
        0x001f_8948,
        ov_screen_pause_freeride_on_widget_action,
        "OVScreenPauseFreeride_OnWidgetAction",
    ),
    (
        0x0022_14f8,
        ov_screen_pause_freeride_ctor,
        "OVScreenPauseFreeride_ctor",
    ),
    (
        0x0022_1558,
        ov_screen_pause_freeride_ctor_2,
        "OVScreenPauseFreeride_ctor_2",
    ),
    (
        0x0022_15c8,
        ov_screen_pause_freeride_dtor,
        "OVScreenPauseFreeride_dtor",
    ),
    (
        0x001f_8448,
        oc1023_pause_freeride_transition_in,
        "OVScreenPauseFreeride_TransitionIn",
    ),
    (
        0x001f_8728,
        oc1023_pause_freeride_update,
        "OVScreenPauseFreeride_Update",
    ),
    (
        0x001f_8720,
        ov_screen_pause_freeride_on_deactivate_001f8720,
        "OVScreenPauseFreeride_OnDeactivate",
    ),
    (
        0x001f_8df0,
        ov_screen_pause_freeride_handle_input_001f8df0,
        "OVScreenPauseFreeride_HandleInput",
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

// ---------------------------------------------------------------------------
// Pause menu.

/// `void OVScreenPauseFreeride_PlayIconAnim(self, idx, on)` (0x1f8df8): format
/// the icon animation's label, look its frame up on the screen and either
/// start it (remembering the playing frame) or retarget the running
/// animation.
pub fn ovscreen_pause_freeride_play_icon_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        let on = c.g[A2][0] != 0;
        c.g[S0][0] = c.g[A0][0];

        'outer: {
            if !on {
                c.cycles += 18;
                c.g[V0][0] = 0x440000;
                c.g[V1][0] = sx(r(c, A1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1c30));
                c.g[A1][0] = 0x4a0000;
                c.g[V1][0] = sx(p_m2(r(c, V1), r(c, V0)));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = Addr(r(c, V1)).lw(c);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(0x2490));
                call_ptr(c, 0x0041_7828, 0x001f_8e98)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x001f_8ea0)?;
                c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_8eac)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = 0xffff;
                if c.g[V1][0] == c.g[A0][0] {
                    frame.restore(c, 16, 0x50);
                    break 'outer;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S0)).off(0xc4).lw(c);
                c.sh(Addr(r(c, V0)).off(0xc).0, c.g[V1][0]);
                c.g[V1][0] = Addr(r(c, S0)).off(0xc4).lw(c);
                c.sh(Addr(r(c, V1)).off(0xe).0, c.g[A0][0]);
                c.sw(Addr(r(c, S0)).off(0xc4).0, 0);
                c.cycles += 1;
                frame.restore(c, 16, 0x50);
                break 'outer;
            }

            c.cycles += 18;
            c.g[V0][0] = 0x440000;
            c.g[V1][0] = sx(r(c, A1) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x1c30));
            c.g[A1][0] = 0x4a0000;
            c.g[V1][0] = sx(p_m2(r(c, V1), r(c, V0)));
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = Addr(r(c, V1)).lw(c);
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x2488));
            call_ptr(c, 0x0041_7828, 0x001f_8e30)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x0031_7670, 0x001f_8e38)?;
            c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001f_8e44)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            if c.g[A1][0] == c.g[V0][0] {
                frame.restore(c, 16, 0x50);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r(c, S0)).off(0xc4).lw(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x50);
                break 'outer;
            }
            c.cycles += 5;
            c.g[A0][0] = Addr(r(c, S0)).off(0xc0).lw(c);
            c.g[A2][0] = 1;
            call_ptr(c, 0x0039_c870, 0x001f_8e6c)?;
            c.sw(Addr(r(c, S0)).off(0xc4).0, c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x50);
        }

        c.cycles += 3;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPauseFreeride_Init(cOVScreenPauseFreeride* self)` (0x001f8168):
/// builds the freeride pause menu. Creates the screen from its hash name (and
/// plays its first frame), registers the PDA template, runs the overlay base
/// init, clears the option flag array, then for each of the eight menu slots
/// looks up the menu text object, asks the engine's sound/UI interface whether
/// the entry is enabled (and sets its text id or string), and hides or shows the
/// highlight icons. Finally sets the defaults (selected slot, flags), clears the
/// per-slot unlock table (depends on the BE state 0xb), plays the transition
/// and pauses the audio. Every register, cycle and call is kept as in the
/// generated body.
pub fn ov_screen_pause_freeride_init(c: &mut Cpu) {
    if c.pc != 0x001f8168 {
        return ov_screen_pause_freeride_init_resume(c);
    }
    guest(c, |c| {
        'b_1f81e4: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[A0][0] = 0x470000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x001f81a4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7e8));
            call_ptr(c, 0x00317670, 0x001f81a4)?;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7d0));
            c.g[RA][0] = sx(0x001f81b8);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001f81b8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f81cc);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001f81cc)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
            if t {
                break 'b_1f81e4;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001f81e4);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001f81e4)?;
                break 'b_1f81e4;
            }
        }
        c.cycles += 11;
        c.g[RA][0] = sx(0x001f81ec);
        c.g[S6][0] = 0u64;
        call_ptr(c, 0x0020d110, 0x001f81ec)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001f81f8);
        c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0xd8));
        call_ptr(c, 0x0020a380, 0x001f81f8)?;
        c.g[S7][0] = 0u64;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x001f820c);
        c.g[A2][0] = 0x2;
        call_ptr(c, 0x00416210, 0x001f820c)?;
        'l_1f8210: loop {
            'b_1f8308: {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001f8220);
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x00317670, 0x001f8220)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                c.g[RA][0] = sx(0x001f822c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001f822c)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0u64 {
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    break 'b_1f8308;
                } else {
                    'b_1f8304: {
                        'b_1f82d4: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                            c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                            if t {
                                break 'b_1f82d4;
                            } else {
                                'b_1f82c8: {
                                    'b_1f82ac: {
                                        c.cycles += 21;
                                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x9c));
                                        c.g[S3][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[S5][0] = c.g[A0][0];
                                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                                        c.g[V0][0] = 0x440000;
                                        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bc0));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                        c.g[RA][0] = sx(0x001f8270);
                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                        call_ptr(c, 0x00317670, 0x001f8270)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001f8294);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001f8294)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1f82ac;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001f82a4);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x003a1000, 0x001f82a4)?;
                                            let t = true;
                                            c.g[V0][0] =
                                                sx((c.g[S5][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                            if t {
                                                break 'b_1f82c8;
                                            } else {
                                                break 'b_1f82ac;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                    c.g[RA][0] = sx(0x001f82c4);
                                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                    call_ptr(c, 0x003a0c50, 0x001f82c4)?;
                                    c.g[V0][0] =
                                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                                    break 'b_1f82c8;
                                }
                                c.cycles += 3;
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                if t {
                                    break 'b_1f8304;
                                } else {
                                    break 'b_1f82d4;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f82ec);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f82ec)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f8304);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001f8304)?;
                        break 'b_1f8304;
                    }
                    c.cycles += 1;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    break 'b_1f8308;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 8i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001f8210);
                }
                continue 'l_1f8210;
            } else {
                c.cycles += 9;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5544));
                c.g[V0][0] = 0x1;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xcc));
                c.g[V1][0] = 0xd;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc8), 0u64);
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x34));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd0), 0u64);
                'l_1f8338: loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f8338);
                        }
                        continue 'l_1f8338;
                    } else {
                        'b_1f836c: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0xb;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
                            if c.g[V1][0] == c.g[A1][0] {
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xf0), 0u64);
                                break 'b_1f836c;
                            } else {
                                break 'b_1f836c;
                            }
                        }
                        'b_1f83e0: {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x7;
                            c.g[RA][0] = sx(0x001f837c);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                            call_ptr(c, 0x0014dd58, 0x001f837c)?;
                            c.g[V1][0] = 0x530000;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = 0x4a0000;
                            if t {
                                break 'b_1f83e0;
                            } else {
                                'b_1f83b0: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[RA][0] = sx(0x001f8398);
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    call_ptr(c, 0x00231ab8, 0x001f8398)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1f83b0;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x224));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[A0][0] = 0x4a0000;
                                        if t {
                                            break 'b_1f83e0;
                                        } else {
                                            break 'b_1f83b0;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[FP][0];
                                c.g[V1][0] = 0u64;
                                c.g[A1][0] = 0x1;
                                'l_1f83c0: loop {
                                    'b_1f83cc: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[A1][0];
                                        c.sw(c.g[A0][0] as u32, 0u64);
                                        if t {
                                            break 'b_1f83cc;
                                        } else {
                                            c.cycles += 1;
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0xdc),
                                                c.g[V1][0],
                                            );
                                            break 'b_1f83cc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 14i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001f83c0);
                                        }
                                        continue 'l_1f83c0;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1f83e0;
                                    }
                                }
                            }
                        }
                        'b_1f8404: {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001f83e8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
                            call_ptr(c, 0x00317670, 0x001f83e8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                            c.g[RA][0] = sx(0x001f83f4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f83f4)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_sub(0xc88));
                            if t {
                                break 'b_1f8404;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001f8404);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0039b960, 0x001f8404)?;
                                break 'b_1f8404;
                            }
                        }
                        c.cycles += 16;
                        c.g[RA][0] = sx(0x001f840c);
                        call_ptr(c, 0x0028b180, 0x001f840c)?;
                        c.g[RA][0] = sx(0x001f8414);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00289b70, 0x001f8414)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    });
}

/// Native resume of `ov_screen_pause_freeride_init` (converted from the `entries` body): any entry
/// other than 0x001f8168, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ov_screen_pause_freeride_init_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001f8168 => {
                    c.cycles += 27;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = 0x470000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x001f81a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7e8));
                    call_ptr(c, 0x00317670, 0x001f81a4)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7d0));
                    c.g[RA][0] = sx(0x001f81b8);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001f81b8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f81cc);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00397cc0, 0x001f81cc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                    if t {
                        pc = 0x001f81e4;
                        continue;
                    }
                    pc = 0x001f81d4;
                    continue;
                }
                0x001f81d4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001f81e4);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x001f81e4)?;
                    pc = 0x001f81e4;
                    continue;
                }
                0x001f81e4 => {
                    c.cycles += 11;
                    c.g[RA][0] = sx(0x001f81ec);
                    c.g[S6][0] = 0u64;
                    call_ptr(c, 0x0020d110, 0x001f81ec)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f81f8);
                    c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0xd8));
                    call_ptr(c, 0x0020a380, 0x001f81f8)?;
                    c.g[S7][0] = 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001f820c);
                    c.g[A2][0] = 0x2;
                    call_ptr(c, 0x00416210, 0x001f820c)?;
                    pc = 0x001f8210;
                    continue;
                }
                0x001f8210 => {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001f8220);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001f8220)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                    c.g[RA][0] = sx(0x001f822c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f822c)?;
                    c.g[S0][0] = c.g[V0][0];

                    if c.g[S0][0] == 0u64 {
                        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                        pc = 0x001f8308;
                        continue;
                    }
                    pc = 0x001f8238;
                    continue;
                }
                0x001f8238 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
                    if t {
                        pc = 0x001f82d4;
                        continue;
                    }
                    pc = 0x001f8248;
                    continue;
                }
                0x001f8248 => {
                    c.cycles += 21;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x9c));
                    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S5][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = 0x440000;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bc0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[RA][0] = sx(0x001f8270);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x001f8270)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001f8294);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8294)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x001f82ac;
                        continue;
                    }
                    pc = 0x001f829c;
                    continue;
                }
                0x001f829c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f82a4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001f82a4)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    if t {
                        pc = 0x001f82c8;
                        continue;
                    }
                    pc = 0x001f82ac;
                    continue;
                }
                0x001f82ac => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[RA][0] = sx(0x001f82c4);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x003a0c50, 0x001f82c4)?;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    pc = 0x001f82c8;
                    continue;
                }
                0x001f82c8 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    if t {
                        pc = 0x001f8304;
                        continue;
                    }
                    pc = 0x001f82d4;
                    continue;
                }
                0x001f82d4 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f82ec);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f82ec)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f8304);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8304)?;
                    pc = 0x001f8304;
                    continue;
                }
                0x001f8304 => {
                    c.cycles += 1;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                    pc = 0x001f8308;
                    continue;
                }
                0x001f8308 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S6][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001f8210) {
                            return Ok(());
                        }
                        pc = 0x001f8210;
                        continue;
                    }
                    pc = 0x001f8314;
                    continue;
                }
                0x001f8314 => {
                    c.cycles += 9;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5544));
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
                    c.g[A0][0] = 0x1;
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xcc));
                    c.g[V1][0] = 0xd;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xc8), 0u64);
                    c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x34));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xd0), 0u64);
                    pc = 0x001f8338;
                    continue;
                }
                0x001f8338 => {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001f8338) {
                            return Ok(());
                        }
                        pc = 0x001f8338;
                        continue;
                    }
                    pc = 0x001f8354;
                    continue;
                }
                0x001f8354 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0xb;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));

                    if c.g[V1][0] == c.g[A1][0] {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xf0), 0u64);
                        pc = 0x001f836c;
                        continue;
                    }
                    pc = 0x001f836c;
                    continue;
                }
                0x001f836c => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x7;
                    c.g[RA][0] = sx(0x001f837c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x001f837c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001f83e0;
                        continue;
                    }
                    pc = 0x001f838c;
                    continue;
                }
                0x001f838c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x001f8398);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    call_ptr(c, 0x00231ab8, 0x001f8398)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        pc = 0x001f83b0;
                        continue;
                    }
                    pc = 0x001f83a0;
                    continue;
                }
                0x001f83a0 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x224));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001f83e0;
                        continue;
                    }
                    pc = 0x001f83b0;
                    continue;
                }
                0x001f83b0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[V1][0] = 0u64;
                    c.g[A1][0] = 0x1;
                    pc = 0x001f83c0;
                    continue;
                }
                0x001f83c0 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[A1][0];
                    c.sw(c.g[A0][0] as u32, 0u64);
                    if t {
                        pc = 0x001f83cc;
                        continue;
                    }
                    pc = 0x001f83c8;
                    continue;
                }
                0x001f83c8 => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xdc), c.g[V1][0]);
                    pc = 0x001f83cc;
                    continue;
                }
                0x001f83cc => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 14i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001f83c0) {
                            return Ok(());
                        }
                        pc = 0x001f83c0;
                        continue;
                    }
                    pc = 0x001f83dc;
                    continue;
                }
                0x001f83dc => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x4a0000;
                    pc = 0x001f83e0;
                    continue;
                }
                0x001f83e0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001f83e8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
                    call_ptr(c, 0x00317670, 0x001f83e8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                    c.g[RA][0] = sx(0x001f83f4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f83f4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_sub(0xc88));
                    if t {
                        pc = 0x001f8404;
                        continue;
                    }
                    pc = 0x001f83fc;
                    continue;
                }
                0x001f83e4 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc0));
                    c.g[RA][0] = sx(0x001f83f4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f83f4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_sub(0xc88));
                    if t {
                        pc = 0x001f8404;
                        continue;
                    }
                    pc = 0x001f83fc;
                    continue;
                }
                0x001f83fc => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001f8404);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0039b960, 0x001f8404)?;
                    pc = 0x001f8404;
                    continue;
                }
                0x001f8404 => {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x001f840c);
                    call_ptr(c, 0x0028b180, 0x001f840c)?;
                    c.g[RA][0] = sx(0x001f8414);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00289b70, 0x001f8414)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001f8168, 0x001f8444);
                    return Ok(());
                }
            }
        }
    });
}

/// `void OVScreenPauseFreeride_OnWidgetAction(self, widget, event)` (0x001f8948):
/// the freeride pause menu's widget handler (`a2` = UI event 1..8, `a1` the
/// widget, whose `+0x18` is the menu slot). Select: plays the denied or accept
/// sound (by the unlock table at `self+0xd8`), remembers the slot and starts the
/// screen's transition-out, then by slot closes the pause (resume audio, pop
/// the state) or triggers overlay 0x17/7/0x18/0x1c/0x19/0x1a/0x1b/0x20/0x21/0x22
/// or opens the audio options popup. Other events: back, focus gain/loss
/// (icon animation) and the pause-state messages. Every register, cycle and call
/// is kept as in the generated body (the two jump tables as `match`).
pub fn ov_screen_pause_freeride_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001f8948 {
        return ov_screen_pause_freeride_on_widget_action_resume(c);
    }
    guest(c, |c| 'l_1f8948: loop {
        'b_1f8dd4: {
            'b_1f8dd0: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1f8dd0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_1f8dd0;
                    } else {
                        'b_1f8d54: {
                            'b_1f8c88: {
                                'b_1f8c70: {
                                    'b_1f8bfc: {
                                        'b_1f8bec: {
                                            'b_1f8bd4: {
                                                'b_1f8bc4: {
                                                    'b_1f8bb4: {
                                                        'b_1f8ba4: {
                                                            'b_1f8b94: {
                                                                'b_1f8b84: {
                                                                    'b_1f8b74: {
                                                                        'b_1f8b0c: {
                                                                            'b_1f8afc: {
                                                                                'b_1f8ae4: {
                                                                                    'b_1f8a6c: {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x780));
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            as u32;
                                                                                        match t {
0x001f8948 => {
if event_due(c) { deliver_event_here(c, 0x001f8948); } continue 'l_1f8948;
}
0x001f8994 => {
'l_1f8994: loop {
'b_1f89f4: {
c.cycles += 17;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
c.g[A0][0] = 0x4a0000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
c.g[RA][0] = sx(0x001f89b0);
if c.g[V0][0] != 0 { c.g[S2][0] = c.g[V0][0]; }
call_ptr(c, 0x00317670, 0x001f89b0)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
c.g[RA][0] = sx(0x001f89bc);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039d860, 0x001f89bc)?;
c.g[S3][0] = c.g[V0][0];
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
let t = c.g[V1][0] != 0u64;
c.g[V0][0] = c.lw(c.g[S2][0] as u32);
if t {
break 'b_1f89f4;
} else {
c.cycles += 7;
c.g[A1][0] = 0x4;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001f89ec);
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x001f89ec)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
if t {
break 'b_1f8dd4;
} else {
break 'b_1f89f4;
}
}
}
c.cycles += 24;
c.g[A1][0] = 0x6;
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001f8a08);
c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
call_ptr(c, t, 0x001f8a08)?;
c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x95));
c.g[A0][0] = 0x470000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7b0));
c.g[RA][0] = sx(0x001f8a1c);
c.sb((c.g[GP][0] as u32).wrapping_sub(0xc88), c.g[V0][0]);
call_ptr(c, 0x00317670, 0x001f8a1c)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
c.g[RA][0] = sx(0x001f8a28);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039c778, 0x001f8a28)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
c.g[A1][0] = c.g[V0][0] & 0xffff;
c.g[RA][0] = sx(0x001f8a38);
c.g[A2][0] = 0x1;
call_ptr(c, 0x0039c870, 0x001f8a38)?;
c.g[V1][0] = 0x1;
c.sw((c.g[S1][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = (c.g[A0][0] < 0xdu64) as u64;
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0x470000;
if t {
break 'b_1f8dd0;
} else {
c.cycles += 6;
c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x760));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
c.g[A0][0] = c.lw(c.g[V1][0] as u32);
let t = c.g[A0][0] as u32;
match t {
0x001f8948 => {
if event_due(c) { deliver_event_here(c, 0x001f8948); } continue 'l_1f8948;
}
0x001f8994 => {
if event_due(c) { deliver_event_here(c, 0x001f8994); } continue 'l_1f8994;
}
0x001f8a6c => {
break 'b_1f8a6c;
}
0x001f8ae4 => {
break 'b_1f8ae4;
}
0x001f8afc => {
break 'b_1f8afc;
}
0x001f8b0c => {
break 'b_1f8b0c;
}
0x001f8b74 => {
break 'b_1f8b74;
}
0x001f8b84 => {
break 'b_1f8b84;
}
0x001f8b94 => {
break 'b_1f8b94;
}
0x001f8ba4 => {
break 'b_1f8ba4;
}
0x001f8bb4 => {
break 'b_1f8bb4;
}
0x001f8bc4 => {
break 'b_1f8bc4;
}
0x001f8bd4 => {
break 'b_1f8bd4;
}
0x001f8bec => {
break 'b_1f8bec;
}
0x001f8bfc => {
break 'b_1f8bfc;
}
0x001f8c88 => {
break 'b_1f8c88;
}
0x001f8d54 => {
break 'b_1f8d54;
}
0x001f8dd0 => {
break 'b_1f8dd0;
}
_ => { c.jump_out(t, 0x001f8948, 0x001f8dec); return Ok(()); }
}
}
}
}
0x001f8a6c => {
break 'b_1f8a6c;
}
0x001f8ae4 => {
break 'b_1f8ae4;
}
0x001f8afc => {
break 'b_1f8afc;
}
0x001f8b0c => {
break 'b_1f8b0c;
}
0x001f8b74 => {
break 'b_1f8b74;
}
0x001f8b84 => {
break 'b_1f8b84;
}
0x001f8b94 => {
break 'b_1f8b94;
}
0x001f8ba4 => {
break 'b_1f8ba4;
}
0x001f8bb4 => {
break 'b_1f8bb4;
}
0x001f8bc4 => {
break 'b_1f8bc4;
}
0x001f8bd4 => {
break 'b_1f8bd4;
}
0x001f8bec => {
break 'b_1f8bec;
}
0x001f8bfc => {
break 'b_1f8bfc;
}
0x001f8c88 => {
break 'b_1f8c88;
}
0x001f8d54 => {
break 'b_1f8d54;
}
0x001f8dd0 => {
break 'b_1f8dd0;
}
_ => { c.jump_out(t, 0x001f8948, 0x001f8dec); return Ok(()); }
}
                                                                                    }
                                                                                    'b_1f8a90: {
                                                                                        c.cycles +=
                                                                                            3;
                                                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            == 0u64;

                                                                                        if t {
                                                                                            break 'b_1f8a90;
                                                                                        } else {
                                                                                            c.cycles += 6;
                                                                                            c.g[RA][0] = sx(0x001f8a80);
                                                                                            c.g[A1][0] = 0u64;
                                                                                            call_ptr(c, 0x00258790, 0x001f8a80)?;
                                                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                                                                            c.g[V0][0] = 0xffffffffffffffff;
                                                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                                                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                                                                                            break 'b_1f8a90;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 21;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001f8a98,
                                                                                    );
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0020d190,
                                                                                        0x001f8a98,
                                                                                    )?;
                                                                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                                                    c.g[A1][0] =
                                                                                        0x1;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001f8aa8,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0039f190,
                                                                                        0x001f8aa8,
                                                                                    )?;
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                                                                    c.g[A1][0] =
                                                                                        0x530000;
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                                                                                    c.g[A2][0] =
                                                                                        0x440000;
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                                                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[A0][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                                                                    c.g[V0][0] = c
                                                                                        .lw(
                                                                                        c.g[V1][0]
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V0][0] = sx(
                                                                                        (c.g[V0][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                                                                    c.g[V1][0] = c
                                                                                        .lw(
                                                                                        c.g[V0][0]
                                                                                            as u32,
                                                                                    );
                                                                                    let t = true;
                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                                                                    if t {
                                                                                        break 'b_1f8c70;
                                                                                    } else {
                                                                                        break 'b_1f8ae4;
                                                                                    }
                                                                                }
                                                                                c.cycles += 6;
                                                                                c.g[V0][0] = 0x1;
                                                                                c.g[A0][0] = 0x17;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001f8af4);
                                                                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9e8), c.g[V0][0]);
                                                                                call_ptr(
                                                                                    c, 0x0020ab50,
                                                                                    0x001f8af4,
                                                                                )?;
                                                                                let t = true;
                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                                                if t {
                                                                                    break 'b_1f8dd4;
                                                                                } else {
                                                                                    break 'b_1f8afc;
                                                                                }
                                                                            }
                                                                            c.cycles += 4;
                                                                            c.g[RA][0] =
                                                                                sx(0x001f8b04);
                                                                            c.g[A0][0] = 0x7;
                                                                            call_ptr(
                                                                                c, 0x0020ab50,
                                                                                0x001f8b04,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x40,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1f8dd4;
                                                                            } else {
                                                                                break 'b_1f8b0c;
                                                                            }
                                                                        }
                                                                        c.cycles += 26;
                                                                        c.g[V1][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[A1][0] = 0x470000;
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x7a0,
                                                                                ));
                                                                        c.g[A0][0] = 0x80;
                                                                        c.g[V0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[RA][0] = sx(0x001f8b30);
                                                                        c.g[S0][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x48));
                                                                        call_ptr(
                                                                            c, 0x00317d70,
                                                                            0x001f8b30,
                                                                        )?;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        c.g[RA][0] = sx(0x001f8b44);
                                                                        c.g[A2][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00195ff0,
                                                                            0x001f8b44,
                                                                        )?;
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        c.g[A3][0] =
                                                                            0xffffffffffffc0ff;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                        c.g[A1][0] = c.g[V0][0];
                                                                        c.g[V1][0] =
                                                                            c.g[V1][0] & c.g[A3][0];
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[V1][0] =
                                                                            c.g[V1][0] | 0x100;
                                                                        c.g[A3][0] = 0u64;
                                                                        c.g[RA][0] = sx(0x001f8b6c);
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(0x1c),
                                                                            c.g[V1][0],
                                                                        );
                                                                        call_ptr(
                                                                            c, 0x0039f320,
                                                                            0x001f8b6c,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x40),
                                                                        );
                                                                        if t {
                                                                            break 'b_1f8dd4;
                                                                        } else {
                                                                            break 'b_1f8b74;
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x001f8b7c);
                                                                    c.g[A0][0] = 0x18;
                                                                    call_ptr(
                                                                        c, 0x0020ab50, 0x001f8b7c,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x40),
                                                                    );
                                                                    if t {
                                                                        break 'b_1f8dd4;
                                                                    } else {
                                                                        break 'b_1f8b84;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x001f8b8c);
                                                                c.g[A0][0] = 0x1c;
                                                                call_ptr(
                                                                    c, 0x0020ab50, 0x001f8b8c,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                if t {
                                                                    break 'b_1f8dd4;
                                                                } else {
                                                                    break 'b_1f8b94;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x001f8b9c);
                                                            c.g[A0][0] = 0x19;
                                                            call_ptr(c, 0x0020ab50, 0x001f8b9c)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            if t {
                                                                break 'b_1f8dd4;
                                                            } else {
                                                                break 'b_1f8ba4;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001f8bac);
                                                        c.g[A0][0] = 0x1a;
                                                        call_ptr(c, 0x0020ab50, 0x001f8bac)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        );
                                                        if t {
                                                            break 'b_1f8dd4;
                                                        } else {
                                                            break 'b_1f8bb4;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x001f8bbc);
                                                    c.g[A0][0] = 0x1b;
                                                    call_ptr(c, 0x0020ab50, 0x001f8bbc)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_1f8dd4;
                                                    } else {
                                                        break 'b_1f8bc4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001f8bcc);
                                                c.g[A0][0] = 0x20;
                                                call_ptr(c, 0x0020ab50, 0x001f8bcc)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f8dd4;
                                                } else {
                                                    break 'b_1f8bd4;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[V0][0] = 0x2;
                                            c.g[A0][0] = 0x21;
                                            c.g[RA][0] = sx(0x001f8be4);
                                            c.sw(
                                                (c.g[GP][0] as u32).wrapping_sub(0xb5c),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x0020ab50, 0x001f8be4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                            if t {
                                                break 'b_1f8dd4;
                                            } else {
                                                break 'b_1f8bec;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001f8bf4);
                                        c.g[A0][0] = 0x22;
                                        call_ptr(c, 0x0020ab50, 0x001f8bf4)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        if t {
                                            break 'b_1f8dd4;
                                        } else {
                                            break 'b_1f8bfc;
                                        }
                                    }
                                    'b_1f8c20: {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                        let t = c.g[A0][0] == 0u64;

                                        if t {
                                            break 'b_1f8c20;
                                        } else {
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x001f8c10);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x00258790, 0x001f8c10)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                            c.g[V0][0] = 0xffffffffffffffff;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x64),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x68),
                                                c.g[V0][0],
                                            );
                                            break 'b_1f8c20;
                                        }
                                    }
                                    c.cycles += 20;
                                    c.g[RA][0] = sx(0x001f8c28);
                                    call_ptr(c, 0x0020d190, 0x001f8c28)?;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                    c.g[V0][0] = 0x530000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
                                    c.g[A3][0] = 0x440000;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x28f0));
                                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A2][0]);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[RA][0] = sx(0x001f8c70);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                    call_ptr(c, 0x0039f190, 0x001f8c70)?;
                                    break 'b_1f8c70;
                                }
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x001f8c78);
                                call_ptr(c, 0x0028b180, 0x001f8c78)?;
                                c.g[RA][0] = sx(0x001f8c80);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x00289bb8, 0x001f8c80)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f8dd4;
                                } else {
                                    break 'b_1f8c88;
                                }
                            }
                            'b_1f8d08: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = 0x4a0000;
                                if t {
                                    break 'b_1f8d08;
                                } else {
                                    'b_1f8cec: {
                                        'b_1f8ccc: {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A1][0] = 0x4d0000;
                                            if t {
                                                break 'b_1f8ccc;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x7508));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001f8cc4);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001f8cc4)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                                if t {
                                                    break 'b_1f8cec;
                                                } else {
                                                    break 'b_1f8ccc;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x4d0000;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7518));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001f8ce8);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001f8ce8)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                        break 'b_1f8cec;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1f8d08;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001f8d04);
                                        c.g[A2][0] = 0x1;
                                        Callee {
                                            addr: 0x001f8df8,
                                            f: ee::s04::OVScreenPauseFreeride_PlayIconAnim,
                                        }
                                        .call(c, 0x001f8d04)?;
                                        c.g[A0][0] = 0x4a0000;
                                        break 'b_1f8d08;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x001f8d10);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2480));
                            call_ptr(c, 0x00317670, 0x001f8d10)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
                            c.g[RA][0] = sx(0x001f8d1c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f8d1c)?;
                            c.g[S1][0] = c.g[V0][0];
                            let t = c.g[S1][0] == 0u64;
                            c.g[V0][0] = 0x440000;
                            if t {
                                break 'b_1f8dd0;
                            } else {
                                c.cycles += 11;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bf8));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x001f8d40);
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                call_ptr(c, 0x00317670, 0x001f8d40)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001f8d4c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a1000, 0x001f8d4c)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1f8dd4;
                                } else {
                                    break 'b_1f8d54;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            break 'b_1f8dd4;
                        } else {
                            'b_1f8db8: {
                                'b_1f8d98: {
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A1][0] = 0x4d0000;
                                    if t {
                                        break 'b_1f8d98;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74e8));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001f8d90);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001f8d90)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                        if t {
                                            break 'b_1f8db8;
                                        } else {
                                            break 'b_1f8d98;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x4d0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74f8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f8db4);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001f8db4)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                                break 'b_1f8db8;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                break 'b_1f8dd4;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001f8dd0);
                                c.g[A2][0] = 0u64;
                                Callee {
                                    addr: 0x001f8df8,
                                    f: ee::s04::OVScreenPauseFreeride_PlayIconAnim,
                                }
                                .call(c, 0x001f8dd0)?;
                                break 'b_1f8dd0;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1f8dd4;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// Native resume of `ov_screen_pause_freeride_on_widget_action` (converted from the `entries` body): any entry
/// other than 0x001f8948, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn ov_screen_pause_freeride_on_widget_action_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x001f8948 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[S0][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        pc = 0x001f8dd0;
                        continue;
                    }
                    pc = 0x001f896c;
                    continue;
                }
                0x001f896c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        pc = 0x001f8dd0;
                        continue;
                    }
                    pc = 0x001f897c;
                    continue;
                }
                0x001f897c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x780));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001f8994 => {
                    c.cycles += 17;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                    c.g[A0][0] = 0x4a0000;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2470));
                    c.g[RA][0] = sx(0x001f89b0);
                    if c.g[V0][0] != 0 {
                        c.g[S2][0] = c.g[V0][0];
                    }
                    call_ptr(c, 0x00317670, 0x001f89b0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
                    c.g[RA][0] = sx(0x001f89bc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f89bc)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    if t {
                        pc = 0x001f89f4;
                        continue;
                    }
                    pc = 0x001f89d8;
                    continue;
                }
                0x001f89d8 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0x4;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f89ec);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f89ec)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f89f4;
                    continue;
                }
                0x001f89f4 => {
                    c.cycles += 24;
                    c.g[A1][0] = 0x6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f8a08);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8a08)?;
                    c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x95));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7b0));
                    c.g[RA][0] = sx(0x001f8a1c);
                    c.sb((c.g[GP][0] as u32).wrapping_sub(0xc88), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001f8a1c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
                    c.g[RA][0] = sx(0x001f8a28);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001f8a28)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
                    c.g[A1][0] = c.g[V0][0] & 0xffff;
                    c.g[RA][0] = sx(0x001f8a38);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039c870, 0x001f8a38)?;
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = (c.g[A0][0] < 0xdu64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        pc = 0x001f8dd0;
                        continue;
                    }
                    pc = 0x001f8a54;
                    continue;
                }
                0x001f8a54 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x760));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001f8a6c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        pc = 0x001f8a90;
                        continue;
                    }
                    pc = 0x001f8a78;
                    continue;
                }
                0x001f8a78 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001f8a80);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00258790, 0x001f8a80)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                    pc = 0x001f8a90;
                    continue;
                }
                0x001f8a90 => {
                    c.cycles += 21;
                    c.g[RA][0] = sx(0x001f8a98);
                    call_ptr(c, 0x0020d190, 0x001f8a98)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x001f8aa8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039f190, 0x001f8aa8)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                    c.g[A1][0] = 0x530000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                    c.g[A2][0] = 0x440000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = true;
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                    if t {
                        pc = 0x001f8c70;
                        continue;
                    }
                    pc = 0x001f8ae4;
                    continue;
                }
                0x001f8ae4 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = 0x17;
                    c.g[RA][0] = sx(0x001f8af4);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9e8), c.g[V0][0]);
                    call_ptr(c, 0x0020ab50, 0x001f8af4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8afc;
                    continue;
                }
                0x001f8afc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8b04);
                    c.g[A0][0] = 0x7;
                    call_ptr(c, 0x0020ab50, 0x001f8b04)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8b0c;
                    continue;
                }
                0x001f8b0c => {
                    c.cycles += 26;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7a0));
                    c.g[A0][0] = 0x80;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x001f8b30);
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    call_ptr(c, 0x00317d70, 0x001f8b30)?;
                    c.g[A3][0] = 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001f8b44);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x00195ff0, 0x001f8b44)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = 0xffffffffffffc0ff;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                    c.g[A2][0] = 0u64;
                    c.g[V1][0] = c.g[V1][0] | 0x100;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x001f8b6c);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    call_ptr(c, 0x0039f320, 0x001f8b6c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8b74;
                    continue;
                }
                0x001f8b74 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8b7c);
                    c.g[A0][0] = 0x18;
                    call_ptr(c, 0x0020ab50, 0x001f8b7c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8b84;
                    continue;
                }
                0x001f8b84 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8b8c);
                    c.g[A0][0] = 0x1c;
                    call_ptr(c, 0x0020ab50, 0x001f8b8c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8b94;
                    continue;
                }
                0x001f8b94 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8b9c);
                    c.g[A0][0] = 0x19;
                    call_ptr(c, 0x0020ab50, 0x001f8b9c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8ba4;
                    continue;
                }
                0x001f8ba4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8bac);
                    c.g[A0][0] = 0x1a;
                    call_ptr(c, 0x0020ab50, 0x001f8bac)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8bb4;
                    continue;
                }
                0x001f8bb4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8bbc);
                    c.g[A0][0] = 0x1b;
                    call_ptr(c, 0x0020ab50, 0x001f8bbc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8bc4;
                    continue;
                }
                0x001f8bc4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8bcc);
                    c.g[A0][0] = 0x20;
                    call_ptr(c, 0x0020ab50, 0x001f8bcc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8bd4;
                    continue;
                }
                0x001f8bd4 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x2;
                    c.g[A0][0] = 0x21;
                    c.g[RA][0] = sx(0x001f8be4);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), c.g[V0][0]);
                    call_ptr(c, 0x0020ab50, 0x001f8be4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8bec;
                    continue;
                }
                0x001f8bec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001f8bf4);
                    c.g[A0][0] = 0x22;
                    call_ptr(c, 0x0020ab50, 0x001f8bf4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8bfc;
                    continue;
                }
                0x001f8bfc => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    let t = c.g[A0][0] == 0u64;

                    if t {
                        pc = 0x001f8c20;
                        continue;
                    }
                    pc = 0x001f8c08;
                    continue;
                }
                0x001f8c08 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001f8c10);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00258790, 0x001f8c10)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                    pc = 0x001f8c20;
                    continue;
                }
                0x001f8c20 => {
                    c.cycles += 20;
                    c.g[RA][0] = sx(0x001f8c28);
                    call_ptr(c, 0x0020d190, 0x001f8c28)?;
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
                    c.g[A3][0] = 0x440000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x28f0));
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A2][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[RA][0] = sx(0x001f8c70);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                    call_ptr(c, 0x0039f190, 0x001f8c70)?;
                    pc = 0x001f8c70;
                    continue;
                }
                0x001f8c70 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001f8c78);
                    call_ptr(c, 0x0028b180, 0x001f8c78)?;
                    c.g[RA][0] = sx(0x001f8c80);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00289bb8, 0x001f8c80)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8c88;
                    continue;
                }
                0x001f8c88 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001f8d08;
                        continue;
                    }
                    pc = 0x001f8c94;
                    continue;
                }
                0x001f8c94 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = 0x4d0000;
                    if t {
                        pc = 0x001f8ccc;
                        continue;
                    }
                    pc = 0x001f8cac;
                    continue;
                }
                0x001f8cac => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7508));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001f8cc4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8cc4)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                    if t {
                        pc = 0x001f8cec;
                        continue;
                    }
                    pc = 0x001f8ccc;
                    continue;
                }
                0x001f8ccc => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x4d0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7518));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001f8ce8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8ce8)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                    pc = 0x001f8cec;
                    continue;
                }
                0x001f8cec => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x001f8d08;
                        continue;
                    }
                    pc = 0x001f8cf4;
                    continue;
                }
                0x001f8cf4 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f8d04);
                    c.g[A2][0] = 0x1;
                    Callee {
                        addr: 0x001f8df8,
                        f: ee::s04::OVScreenPauseFreeride_PlayIconAnim,
                    }
                    .call(c, 0x001f8d04)?;
                    c.g[A0][0] = 0x4a0000;
                    pc = 0x001f8d08;
                    continue;
                }
                0x001f8d08 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001f8d10);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2480));
                    call_ptr(c, 0x00317670, 0x001f8d10)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc0));
                    c.g[RA][0] = sx(0x001f8d1c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f8d1c)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x001f8dd0;
                        continue;
                    }
                    pc = 0x001f8d28;
                    continue;
                }
                0x001f8d28 => {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1bf8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x001f8d40);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x00317670, 0x001f8d40)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f8d4c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001f8d4c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8d54;
                    continue;
                }
                0x001f8d54 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));

                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8d60;
                    continue;
                }
                0x001f8d60 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd8));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = 0x4d0000;
                    if t {
                        pc = 0x001f8d98;
                        continue;
                    }
                    pc = 0x001f8d78;
                    continue;
                }
                0x001f8d78 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74e8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001f8d90);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8d90)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                    if t {
                        pc = 0x001f8db8;
                        continue;
                    }
                    pc = 0x001f8d98;
                    continue;
                }
                0x001f8d98 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x4d0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74f8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001f8db4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001f8db4)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd0));
                    pc = 0x001f8db8;
                    continue;
                }
                0x001f8db8 => {
                    c.cycles += 2;

                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        pc = 0x001f8dd4;
                        continue;
                    }
                    pc = 0x001f8dc0;
                    continue;
                }
                0x001f8dc0 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f8dd0);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x001f8df8,
                        f: ee::s04::OVScreenPauseFreeride_PlayIconAnim,
                    }
                    .call(c, 0x001f8dd0)?;
                    pc = 0x001f8dd0;
                    continue;
                }
                0x001f8dd0 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x001f8dd4;
                    continue;
                }
                0x001f8dd4 => {
                    c.cycles += 6;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001f8948, 0x001f8dec);
                    return Ok(());
                }
            }
        }
    });
}

/// `void OVScreenPauseFreeride_Shutdown(OVScreenPauseFreeride* self)`
/// (0x1f88e8): clear the screen's +0xc4 word and run the overlay base shutdown
/// (`cOVStateOverlay_Shutdown`, 0x20a430).
pub fn ov_screen_pause_freeride_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), 0u64);
        call_ptr(c, 0x0020_a430, 0x001f_88fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIState_ctor` (0x39e2a0), the base constructor every `OVState` runs first.
pub(crate) const OV_STATE_UI_CTOR: u32 = 0x0039_e2a0;

/// `UIState_dtor` (0x39e390), run last by every `OVState` destructor.
pub(crate) const OV_STATE_UI_DTOR: u32 = 0x0039_e390;

/// `cOVState_vtbl`: the vtable every `OVState` starts with (stored at +0x8).
pub(crate) const OV_STATE_BASE_VTBL: u32 = 0x0047_4e08;

/// The game-image data page (`0x4a0000`) holding `str_sp_b[0]` at +0x2088.
pub(crate) const OV_STATE_DATA_PAGE: u32 = 0x004a_0000;

/// The field stores every `OVState` constructor makes after the base
/// `UIState` constructor has returned (`this` is `s0`): the overlay id at
/// +0x98, the transition, pause-menu and PDA flags (+0x48, +0x4c, +0x54)
/// cleared, the vtable at +0x8 (written twice: the base one, then `vptr`,
/// which is what the object finally has), the chat buffer's first halfword
/// at +0x58 and, for the pause screens (`pause_thread`), the thread at +0xc4.
/// Leaves `v0` = this, `v1` = `vptr`, `a1` = the data page, `a0` = the
/// halfword, as the generated body does.
pub(crate) fn ov_state_fill_fields(c: &mut Cpu, ov_id: u64, vptr: u32, pause_thread: bool) {
    let obj = c.g[S0][0] as u32;
    c.sw(obj.wrapping_add(0x98), ov_id);
    c.sw(obj.wrapping_add(0x48), 0);
    c.sw(obj.wrapping_add(0x8), OV_STATE_BASE_VTBL as u64);
    c.sw(obj.wrapping_add(0x4c), 0);
    c.sw(obj.wrapping_add(0x54), 0);
    c.g[V1][0] = vptr as u64;
    c.sw(obj.wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    c.g[A1][0] = OV_STATE_DATA_PAGE as u64;
    c.g[A0][0] = c.lhu(OV_STATE_DATA_PAGE.wrapping_add(0x2088));
    c.sh(obj.wrapping_add(0x58), c.g[A0][0]);
    if pause_thread {
        c.sw(obj.wrapping_add(0xc4), 0);
    }
}

/// The body shared by the three `OVState` destructors: set the base vtable
/// on `this` (a0, copied to v1), run `UIState_dtor` on it and return to `ret`.
pub(crate) fn ov_state_dtor_body(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = OV_STATE_BASE_VTBL as u64;
    let this = c.g[V1][0] as u32;
    c.sw(this.wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, OV_STATE_UI_DTOR, ret)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `OVScreenPauseFreeride_ctor` (0x2214f8): the free-ride pause overlay state,
/// overlay id 0, pause-menu template vtable.
pub fn ov_screen_pause_freeride_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1510)?;
        ov_state_fill_fields(c, 0, 0x0047_3170, false);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenPauseFreeride_ctor_2` (0x221558): as the constructor above, with
/// the overlay id from `a2` and the pause thread cleared.
pub fn ov_screen_pause_freeride_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_STATE_UI_CTOR, 0x0022_1574)?;
        let ov_id = c.g[S1][0];
        ov_state_fill_fields(c, ov_id, 0x0047_3170, true);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenPauseFreeride_dtor` (0x2215c8): the free-ride pause overlay's
/// destructor (base vtable, then `UIState_dtor`).
pub fn ov_screen_pause_freeride_dtor(c: &mut Cpu) {
    guest(c, |c| ov_state_dtor_body(c, 0x0022_15e4));
}

/// `bool OVScreenPauseFreeride_TransitionIn(self)` (0x1f8448): on the first
/// transition in (`self+0xd4` set, then cleared) shows thirteen-plus-one
/// objects named by `sprintf` of table strings (`0x441c30`), each through a
/// vtable call with 0, then the first eight entries of the index list
/// (`self+0x9c`, `self+0xbc` of them) through the table: each found object
/// gets a vtable call with 1 and its position set to `index * 40.0`.
/// Returns 0.
pub fn oc1023_pause_freeride_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x90);
        frame.save(c, S0, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 20, 0xc0);
        c.g[V0][0] = Addr(oc1023_at(c, S2, 0xd4)).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V0][0] = 0x440000;
        if !skip {
            c.cycles += 5;
            Addr(oc1023_at(c, S2, 0xd4)).sw(c, 0);
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c30));
            c.g[S4][0] = 0x470000;
            c.g[S0][0] = 0xd;
            'l_1f8490: loop {
                'b_1f84d4: {
                    c.cycles += 12;
                    c.g[A2][0] = Addr(oc1023_at(c, S1, 0)).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x7c0));
                    call_ptr(c, 0x0041_7828, 0x001f_84a0)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_7670, 0x001f_84a8)?;
                    c.g[A0][0] = Addr(oc1023_at(c, S2, 0xc0)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001f_84b4)?;
                    c.g[A2][0] = c.g[V0][0];
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if none {
                        break 'b_1f84d4;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
                    c.g[A0][0] = Addr(oc1023_at(c, V0, 0x48)).lh(c);
                    c.g[V1][0] = Addr(oc1023_at(c, V0, 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_84d4)?;
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                let more = (c.g[S0][0] as i64) >= 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_8490);
                    }
                    continue 'l_1f8490;
                }
                break;
            }
            c.cycles += 6;
            c.g[V0][0] = 0x440000;
            c.g[AT][0] = 0x4220_0000;
            c.mtc1(1, 20);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c30));
            c.g[S1][0] = 0;
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x9c));
            'l_1f84f8: loop {
                'b_1f8584: {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(oc1023_at(c, S2, 0xbc)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    if done {
                        break 'b_1f8584;
                    }
                    c.cycles += 14;
                    c.g[V0][0] = Addr(oc1023_at(c, S3, 0)).lw(c);
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x7c0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    call_ptr(c, 0x0041_7828, 0x001f_8520)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_7670, 0x001f_8528)?;
                    c.g[A0][0] = Addr(oc1023_at(c, S2, 0xc0)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001f_8534)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[A1][0] = 1;
                    if none {
                        break 'b_1f8584;
                    }
                    c.cycles += 17;
                    c.g[V0][0] = Addr(oc1023_at(c, S0, 0x8)).lw(c);
                    c.g[A0][0] = Addr(oc1023_at(c, V0, 0x48)).lh(c);
                    c.g[V1][0] = Addr(oc1023_at(c, V0, 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_8554)?;
                    c.mtc1(17, 0);
                    c.cvt_s_w(0, 0);
                    let at = frame.at(c, 0x40);
                    c.sw(at, 0);
                    let at = frame.at(c, 0x48);
                    c.sw(at, 0);
                    c.mul_s(0, 0, 20);
                    let at = frame.at(c, 0x44);
                    c.swc1(0, at);
                    let at = frame.at(c, 0x47);
                    c.ldl(2, at);
                    let at = frame.at(c, 0x40);
                    c.ldr(2, at);
                    c.g[V1][0] = Addr(frame.at(c, 0x48)).lw(c);
                    let at = oc1023_at(c, S0, 0x4b);
                    c.sdl(2, at);
                    let at = oc1023_at(c, S0, 0x44);
                    c.sdr(2, at);
                    let word = c.g[V1][0];
                    Addr(oc1023_at(c, S0, 0x4c)).sw(c, word);
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 8) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_84f8);
                    }
                    continue 'l_1f84f8;
                }
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xb0);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPauseFreeride_Update(self)` (0x1f8728): while the state
/// (`self+0x110`) is 1, 2 or 4 and the message center has unread messages,
/// pulses the blink phase (`self+0xc8` += `self+0xcc`, bouncing the step at
/// 0 and 1) and passes it to the message icon object (vtable `0x58`). Then
/// runs the common update (`0x20e900`); if the HUD object (`gp-0x204`) has
/// a pending request (`+0x68` set, `+0x64` zero) it clears it, plays the
/// UI sound, steps the screen counter (`gp-0x69c`) back one and loads the
/// next screen id from the tables.
pub fn oc1023_pause_freeride_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(oc1023_at(c, S0, 0x110)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let blink = c.g[V0][0] != 0;
        c.g[V0][0] = 4;
        let run_blink = if blink {
            true
        } else {
            c.cycles += 2;
            c.g[V1][0] == c.g[V0][0]
        };
        if run_blink {
            c.cycles += 4;
            call_ptr(c, 0x001e_3408, 0x001f_875c)?;
            if (c.g[V0][0] as i64) > 0 {
                'b_1f87ac: {
                    'b_1f879c: {
                        c.cycles += 9;
                        c.lwc1(0, oc1023_at(c, S0, 0xc8));
                        c.lwc1(1, oc1023_at(c, S0, 0xcc));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 2);
                        c.add_s(0, 0, 1);
                        c.c_le_s(2, 0);
                        let at_top = c.fcc();
                        c.swc1(0, oc1023_at(c, S0, 0xc8));
                        if at_top {
                            break 'b_1f879c;
                        }
                        c.cycles += 5;
                        c.mtc1(0, 2);
                        c.c_lt_s(0, 2);
                        if !c.fcc() {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_1f87ac;
                        }
                    }
                    c.cycles += 4;
                    c.neg_s(0, 1);
                    c.swc1(2, oc1023_at(c, S0, 0xc8));
                    c.swc1(0, oc1023_at(c, S0, 0xcc));
                    c.g[A0][0] = 0x4a0000;
                }
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2478));
                call_ptr(c, 0x0031_7670, 0x001f_87b4)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0xc0)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_87c0)?;
                c.g[A2][0] = c.g[V0][0];
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = c.g[SP][0];
                if !none {
                    c.cycles += 15;
                    c.lwc1(0, oc1023_at(c, S0, 0xc8));
                    let at = oc1023_at(c, A2, 0x23);
                    c.ldl(2, at);
                    let at = oc1023_at(c, A2, 0x1c);
                    c.ldr(2, at);
                    let at = oc1023_at(c, A2, 0x2b);
                    c.ldl(3, at);
                    let at = oc1023_at(c, A2, 0x24);
                    c.ldr(3, at);
                    let at = frame.at(c, 0x7);
                    c.sdl(2, at);
                    let at = frame.at(c, 0);
                    c.sdr(2, at);
                    let at = frame.at(c, 0xf);
                    c.sdl(3, at);
                    let at = frame.at(c, 0x8);
                    c.sdr(3, at);
                    let at = frame.at(c, 0);
                    c.swc1(0, at);
                    c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
                    c.g[A0][0] = Addr(oc1023_at(c, V0, 0x58)).lh(c);
                    c.g[V1][0] = Addr(oc1023_at(c, V0, 0x5c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_8808)?;
                }
            }
        }
        // 0x1f8808
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0020_e900, 0x001f_8810)?;
        c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x204))).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(oc1023_at(c, V1, 0x68)).lw(c);
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            let mut pending = false;
            if none {
                // 0x1f8834
                c.cycles += 1;
                pending = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(oc1023_at(c, V1, 0x64)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    pending = true;
                }
            }
            if pending {
                c.g[A0][0] = 1;
            }
            c.cycles += 2;
            let idle = c.g[A0][0] == 0;
            c.g[A0][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x204))).lw(c);
            if !idle {
                c.cycles += 37;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0025_8790, 0x001f_8848)?;
                c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x204))).lw(c);
                c.g[V0][0] = u64::MAX;
                c.g[A0][0] = 0x470000;
                let minus_one = c.g[V0][0];
                Addr(oc1023_at(c, V1, 0x64)).sw(c, minus_one);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7b0));
                Addr(oc1023_at(c, V1, 0x68)).sw(c, minus_one);
                call_ptr(c, 0x0031_7670, 0x001f_8864)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0xc0)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_8870)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0xc0)).lw(c);
                c.g[A2][0] = 1;
                c.g[A1][0] = c.g[V0][0] & 0xffff;
                call_ptr(c, 0x0039_c870, 0x001f_8880)?;
                c.g[V1][0] = 1;
                Addr(oc1023_at(c, S0, 0xd0)).sw(c, 1);
                call_ptr(c, 0x0020_d190, 0x001f_888c)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_889c)?;
                c.g[A0][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).lw(c);
                c.g[A1][0] = 0x530000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                c.g[A2][0] = 0x440000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                let counter = c.g[A0][0];
                Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).sw(c, counter);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let screen_id = c.g[V1][0];
                Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x6a0))).sw(c, screen_id);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPauseFreeride_OnDeactivate` (0x001f_8720): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_pause_freeride_on_deactivate_001f8720(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenPauseFreeride_HandleInput` (0x001f_8df0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_pause_freeride_handle_input_001f8df0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

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

    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_m2;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_m2;

    // ----------------------------------------------------------- pause menu

    #[test]
    fn ovscreen_pause_freeride_play_icon_anim_matches() {
        strict(
            ee::s04::f_001f8df8__generated,
            ovscreen_pause_freeride_play_icon_anim,
            90,
            &|c, seed| {
                let _ = base_m2(c, seed, 0x1f8d, 6);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 2) as u64;
                w_m2(c, DATA + 0xc0, DATA + 0x800);
                w_m2(c, DATA + 0xc4, if seed % 3 == 0 { 0 } else { DATA + 0x900 });
                w_m2(c, DATA + 0x800 + 0x38, DATA + 0xa00);
                w_m2(c, DATA + 0xa00, 3);
                w_m2(c, DATA + 0x900 + 0x8, 0);
            },
        );
    }

    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::game::bx::bxstringctor::tests::base_3;
    use crate::game::bx::bxstringctor::tests::slot_2;
    use crate::game::fe::fememcardcallbacks::tests::hash_screen;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's RAM): the
    /// fakes only act when it holds `OC982_MAGIC`, so other tests that reach
    /// the same override slots run the real callees.
    pub(crate) const OC982_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const OC982_MAGIC: u32 = 0x9820_c0de;

    /// The overlay object (`a0`).
    pub(crate) const OC982_SELF: u32 = DATA + 0x5_0000;

    /// The world (`gp - 0x848`) and what hangs off it.
    pub(crate) const OC982_WORLD: u32 = DATA + 0x6_0000;

    pub(crate) const OC982_GAME: u32 = DATA + 0x6_1000;

    pub(crate) const OC982_IFACE: u32 = DATA + 0x6_2000;

    pub(crate) const OC982_IFACE_VT: u32 = DATA + 0x6_3000;

    pub(crate) const OC982_MODEMAN: u32 = DATA + 0x6_4000;

    /// The callee fakes: a call logs everything the caller set up (so the
    /// register dance is compared) and returns a pointer, or 0 where the
    /// seed's zero mask says so.
    pub(crate) fn oc982_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let mask = c.bus.read64(OC982_CTRL + 8);
        if n < 64 && (mask >> n) & 1 == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc982_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC982_CTRL) != OC982_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc982_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc982_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc982_fakes!(
        (
            oc982_fake_00144be0,
            OC982_PREV_00144BE0,
            ee::s01::OVERRIDE_00144BE0,
            ee::s01::f_00144be0__generated,
            0x00144be0
        ),
        (
            oc982_fake_00144bf0,
            OC982_PREV_00144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x00144bf0
        ),
        (
            oc982_fake_00145750,
            OC982_PREV_00145750,
            ee::s01::OVERRIDE_00145750,
            ee::s01::f_00145750__generated,
            0x00145750
        ),
        (
            oc982_fake_00146a70,
            OC982_PREV_00146A70,
            ee::s01::OVERRIDE_00146A70,
            ee::s01::f_00146a70__generated,
            0x00146a70
        ),
        (
            oc982_fake_00146e98,
            OC982_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc982_fake_0014a080,
            OC982_PREV_0014A080,
            ee::s01::OVERRIDE_0014A080,
            ee::s01::f_0014a080__generated,
            0x0014a080
        ),
        (
            oc982_fake_0014dc80,
            OC982_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc982_fake_0014dd58,
            OC982_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc982_fake_0014ee58,
            OC982_PREV_0014EE58,
            ee::s01::OVERRIDE_0014EE58,
            ee::s01::f_0014ee58__generated,
            0x0014ee58
        ),
        (
            oc982_fake_0014eec8,
            OC982_PREV_0014EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014eec8
        ),
        (
            oc982_fake_00155130,
            OC982_PREV_00155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x00155130
        ),
        (
            oc982_fake_00155190,
            OC982_PREV_00155190,
            ee::s01::OVERRIDE_00155190,
            ee::s01::f_00155190__generated,
            0x00155190
        ),
        (
            oc982_fake_00155208,
            OC982_PREV_00155208,
            ee::s01::OVERRIDE_00155208,
            ee::s01::f_00155208__generated,
            0x00155208
        ),
        (
            oc982_fake_00157b08,
            OC982_PREV_00157B08,
            ee::s01::OVERRIDE_00157B08,
            ee::s01::f_00157b08__generated,
            0x00157b08
        ),
        (
            oc982_fake_00195ff0,
            OC982_PREV_00195FF0,
            ee::s02::OVERRIDE_00195FF0,
            ee::s02::f_00195ff0__generated,
            0x00195ff0
        ),
        (
            oc982_fake_001f8df8,
            OC982_PREV_001F8DF8,
            ee::s04::OVERRIDE_001F8DF8,
            ee::s04::f_001f8df8__generated,
            0x001f8df8
        ),
        (
            oc982_fake_001fd190,
            OC982_PREV_001FD190,
            ee::s04::OVERRIDE_001FD190,
            ee::s04::f_001fd190__generated,
            0x001fd190
        ),
        (
            oc982_fake_0020a1a8,
            OC982_PREV_0020A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020a1a8
        ),
        (
            oc982_fake_0020a380,
            OC982_PREV_0020A380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020a380
        ),
        (
            oc982_fake_0020ab50,
            OC982_PREV_0020AB50,
            ee::s05::OVERRIDE_0020AB50,
            ee::s05::f_0020ab50__generated,
            0x0020ab50
        ),
        (
            oc982_fake_0020d110,
            OC982_PREV_0020D110,
            ee::s05::OVERRIDE_0020D110,
            ee::s05::f_0020d110__generated,
            0x0020d110
        ),
        (
            oc982_fake_0020d190,
            OC982_PREV_0020D190,
            ee::s05::OVERRIDE_0020D190,
            ee::s05::f_0020d190__generated,
            0x0020d190
        ),
        (
            oc982_fake_00231ab8,
            OC982_PREV_00231AB8,
            ee::s06::OVERRIDE_00231AB8,
            ee::s06::f_00231ab8__generated,
            0x00231ab8
        ),
        (
            oc982_fake_00258790,
            OC982_PREV_00258790,
            ee::s06::OVERRIDE_00258790,
            ee::s06::f_00258790__generated,
            0x00258790
        ),
        (
            oc982_fake_00289b70,
            OC982_PREV_00289B70,
            ee::s07::OVERRIDE_00289B70,
            ee::s07::f_00289b70__generated,
            0x00289b70
        ),
        (
            oc982_fake_00289bb8,
            OC982_PREV_00289BB8,
            ee::s07::OVERRIDE_00289BB8,
            ee::s07::f_00289bb8__generated,
            0x00289bb8
        ),
        (
            oc982_fake_0028b180,
            OC982_PREV_0028B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc982_fake_00294f78,
            OC982_PREV_00294F78,
            ee::s08::OVERRIDE_00294F78,
            ee::s08::f_00294f78__generated,
            0x00294f78
        ),
        (
            oc982_fake_002c2508,
            OC982_PREV_002C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c2508
        ),
        (
            oc982_fake_002c2540,
            OC982_PREV_002C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c2540
        ),
        (
            oc982_fake_002c26d0,
            OC982_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc982_fake_00317670,
            OC982_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc982_fake_00317d70,
            OC982_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc982_fake_00397cc0,
            OC982_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc982_fake_0039b960,
            OC982_PREV_0039B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039b960
        ),
        (
            oc982_fake_0039c778,
            OC982_PREV_0039C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039c778
        ),
        (
            oc982_fake_0039c870,
            OC982_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc982_fake_0039d860,
            OC982_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc982_fake_0039e8b8,
            OC982_PREV_0039E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039e8b8
        ),
        (
            oc982_fake_0039e948,
            OC982_PREV_0039E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039e948
        ),
        (
            oc982_fake_0039f190,
            OC982_PREV_0039F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039f190
        ),
        (
            oc982_fake_0039f320,
            OC982_PREV_0039F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc982_fake_003a0c50,
            OC982_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc982_fake_003a0d00,
            OC982_PREV_003A0D00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a0d00
        ),
        (
            oc982_fake_003a0e90,
            OC982_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc982_fake_003a1000,
            OC982_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc982_fake_004139f8,
            OC982_PREV_004139F8,
            ee::s15::OVERRIDE_004139F8,
            ee::s15::f_004139f8__generated,
            0x004139f8
        ),
        (
            oc982_fake_00416210,
            OC982_PREV_00416210,
            ee::s15::OVERRIDE_00416210,
            ee::s15::f_00416210__generated,
            0x00416210
        ),
        (
            oc982_fake_00417828,
            OC982_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// A test machine with the game image (jump tables, `.lit4`), the callee
    /// fakes armed, and a per-seed zero mask for the fakes' results.
    pub(crate) fn oc982_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        oc982_install_fakes();
        c.bus.write32(OC982_CTRL, OC982_MAGIC);
        let noise = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        let mask = match seed % 5 {
            0 => 0,
            1 => u64::MAX,
            _ => noise(&mut r) & noise(&mut r),
        };
        c.bus.write64(OC982_CTRL + 8, mask);
        r
    }

    /// The world with the interface object whose vtable the front end calls
    /// through (`world+0x8c -> +4 -> +0x20 / +0x24`), the game (`+0x84`) and
    /// the game-mode manager (`+0xc0`).
    pub(crate) fn oc982_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC982_WORLD);
        c.bus.write32(OC982_WORLD + 0x84, OC982_GAME);
        c.bus.write32(OC982_WORLD + 0x8c, OC982_IFACE);
        c.bus.write32(OC982_IFACE + 4, OC982_IFACE_VT);
        c.bus
            .write16(OC982_IFACE_VT + 0x20, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(OC982_IFACE_VT + 0x24, fakes::FAKE);
        c.bus.write32(OC982_WORLD + 0xc0, OC982_MODEMAN);
        c.bus.write32(OC982_WORLD + 0x78, DATA + 0x6_5000);
    }

    /// The distinct cycle totals the generated body takes over the seeds
    /// (a path count: the fixtures must reach different arms).
    pub(crate) fn oc982_path_count(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    // -- 0x001f8168 OVScreenPauseFreeride_Init -------------------------------

    pub(crate) fn pause_freeride_init_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1f8168, 80);
        oc982_world(c, &mut r);
        let (engine, x, y) = (DATA + 0x5_1000, DATA + 0x5_2000, DATA + 0x5_3000);
        c.bus.write32(OC982_SELF + 0x10, engine);
        c.bus.write32(engine + 0x10, x);
        c.bus.write32(x + 4, y);
        c.bus.write16(y + 0x20, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(y + 0x24, fakes::FAKE);
        // Menu slot count and the slots' string indices.
        c.bus.write32(OC982_SELF + 0xbc, seed % 10);
        for i in 0..8 {
            c.bus.write32(OC982_SELF + 0x9c + 4 * i, r.next_u32() % 16);
        }
        let state = if seed % 3 == 0 { 0xb } else { 0 };
        c.bus.write32(OC982_GAME + 0x214, state);
        c.bus.write32(OC982_GAME + 0x224, (seed / 2) % 2);
        c.bus.write32(0x0053_4b30, (seed / 4) % 2);
        c.g[A0][0] = OC982_SELF as u64;
        c.pc = 0x001f_8168;
    }

    /// OVScreenPauseFreeride_Init: zero and non-zero screen/object results,
    /// the BE state 0xb or not, one to eight menu slots, unwinds inside calls.
    #[test]
    fn ov_screen_pause_freeride_init_matches() {
        check(
            ee::s04::f_001f8168__generated,
            ov_screen_pause_freeride_init,
            96,
            &pause_freeride_init_setup,
        );
        assert!(
            oc982_path_count(
                ee::s04::f_001f8168__generated,
                96,
                &pause_freeride_init_setup
            ) >= 4,
            "fixture reaches too few paths"
        );
    }

    // -- 0x001f8948 OVScreenPauseFreeride_OnWidgetAction ---------------------

    pub(crate) fn pause_freeride_action_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1f8948, 24);
        oc982_world(c, &mut r);
        let (engine, snd, snd_vt) = (DATA + 0x5_1000, DATA + 0x5_4000, DATA + 0x5_5000);
        let (wid, wid_obj, pause) = (DATA + 0x5_8000, DATA + 0x5_9000, DATA + 0x5_6000);
        c.bus.write32(OC982_SELF + 0x10, engine);
        // The engine's sound object, or the null sound in the image (gp + 0x2968).
        let sound = if seed % 2 == 0 { snd } else { 0 };
        c.bus.write32(engine + 0x14, sound);
        c.bus.write32(snd, snd_vt);
        c.bus.write32(GAME_GP + 0x2968, snd_vt);
        c.bus.write16(snd_vt + 0x10, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(snd_vt + 0x14, fakes::FAKE);
        c.bus.write32(OC982_SELF + 0xc0, DATA + 0x5_a000);
        c.bus.write32(OC982_SELF + 0xd0, seed % 2);
        // The unlock flag of every slot.
        for i in 0..16 {
            c.bus.write32(OC982_SELF + 0xd8 + 4 * i, r.next_u32() % 2);
        }
        // The widget: slot index and its object's vtable call.
        let idx = (seed / 3 + seed / 10) % 15;
        c.bus.write32(wid + 0x18, idx);
        c.bus.write32(wid + 0x8, wid_obj);
        c.bus
            .write16(wid_obj + 0x58, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(wid_obj + 0x5c, fakes::FAKE);
        c.bus
            .write16(wid_obj + 0x48, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(wid_obj + 0x4c, fakes::FAKE);
        c.bus
            .write16(wid_obj + 0x40, (r.next_u32() % 64 * 4) as u16);
        c.bus.write32(wid_obj + 0x44, fakes::FAKE);
        // The pause-state record and the pause stack counter.
        let record = if seed % 3 == 0 { 0 } else { pause };
        c.bus.write32(GAME_GP.wrapping_sub(0x204), record);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x69c), 1 + r.next_u32() % 4);
        c.g[A0][0] = OC982_SELF as u64;
        c.g[A1][0] = if seed % 11 == 0 { 0 } else { wid as u64 };
        c.g[A2][0] = (seed % 10) as u64;
        c.pc = 0x001f_8948;
    }

    /// OVScreenPauseFreeride_OnWidgetAction over every UI event (0..9), every
    /// menu slot (0..14), locked and unlocked entries, the engine's sound or
    /// the null sound, with and without the pause record; callees faked.
    #[test]
    fn ov_screen_pause_freeride_on_widget_action_matches() {
        check(
            ee::s04::f_001f8948__generated,
            ov_screen_pause_freeride_on_widget_action,
            220,
            &pause_freeride_action_setup,
        );
        assert!(
            oc982_path_count(
                ee::s04::f_001f8948__generated,
                220,
                &pause_freeride_action_setup
            ) >= 6,
            "fixture reaches too few paths"
        );
    }

    #[test]
    fn ov_screen_pause_freeride_shutdown_matches() {
        check(
            ee::s04::f_001f88e8__generated,
            ov_screen_pause_freeride_shutdown,
            40,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1f88);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x50, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x10, screen - 0x18);
                c.bus.write32(DATA + 0xc4, r.next_u32());
                hash_screen(c, screen, node, 0x004a_20d0);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot_2(c, DATA + 0x3000, 0xc0, 1);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    /// Random junk in every register and in the object (+0..0x100) at
    /// `DATA + 0x1000`, the object in `a0`. The constructors and destructors
    /// have no branches, so one path is the whole space; the junk makes sure
    /// every stored field is visibly written.
    pub(crate) fn ov_object_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0022_1400 ^ seed as u64);
        scramble(c, &mut r);
        let obj = DATA + 0x1000;
        for k in 0..0x40 {
            c.bus.write32(obj + 4 * k, r.next_u32());
        }
        c.g[A0][0] = sx(obj);
    }

    #[test]
    fn ov_screen_pause_freeride_ctor_matches() {
        check(
            ee::s05::f_002214f8__generated,
            ov_screen_pause_freeride_ctor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_pause_freeride_ctor_2_matches() {
        check(
            ee::s05::f_00221558__generated,
            ov_screen_pause_freeride_ctor_2,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_pause_freeride_dtor_matches() {
        check(
            ee::s05::f_002215c8__generated,
            ov_screen_pause_freeride_dtor,
            200,
            &|c, seed| ov_object_setup(c, seed),
        );
    }

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_force;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_force_all;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_machine;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_object;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc1023_screen_tables;

    pub(crate) use crate::game::fe::ovstatepause::tests::OC1023_OBJ;

    #[test]
    fn oc1023_pause_freeride_transition_in_matches() {
        fakes::covered(
            ee::s04::f_001f8448__generated,
            oc1023_pause_freeride_transition_in,
            48,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_8448, 40);
                let this = DATA + 0x100;
                // First transition (flag set) or a later one.
                c.bus.write32(this + 0xd4, (seed % 4 != 0) as u32);
                c.bus.write32(this + 0xc0, DATA + 0x300);
                c.bus.write32(this + 0xbc, seed % 10);
                for i in 0..8u32 {
                    c.bus.write32(this + 0x9c + 4 * i, (seed + 3 * i) % 14);
                }
                for j in 0..16u32 {
                    c.bus.write32(0x0044_1c30 + 4 * j, 0x004a_0000 + 0x10 * j);
                }
                oc1023_object(c, OC1023_OBJ);
                oc1023_force_all(c, &mut r, OC1023_OBJ);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_pause_freeride_update_matches() {
        fakes::covered(
            ee::s04::f_001f8728__generated,
            oc1023_pause_freeride_update,
            96,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_8728, 12);
                let this = DATA + 0x100;
                // States 1, 2 and 4 blink when mail is waiting; 0, 3, 5 do not.
                c.bus
                    .write32(this + 0x110, [1, 2, 4, 0, 3, 5][seed as usize % 6]);
                // Blink phase and step: stays, wraps above 1, wraps below 0.
                let phase = [0.2f32, 0.95, -0.3, 0.5][(seed / 6) as usize % 4];
                let step = [0.1f32, 0.2, -0.1, 0.05][(seed / 24) as usize % 4];
                c.bus.write32(this + 0xc8, phase.to_bits());
                c.bus.write32(this + 0xcc, step.to_bits());
                c.bus.write32(this + 0xc0, DATA + 0x300);
                c.bus.write32(this + 0x10, DATA + 0x380);
                oc1023_object(c, OC1023_OBJ);
                oc1023_force_all(c, &mut r, OC1023_OBJ);
                // Unread messages: some, none, or a negative count.
                oc1023_force(c, 0, [3, 0, 0xffff_ffff, 1][(seed / 3) as usize % 4]);
                // The HUD request object: absent, pending, or busy (both set).
                let hud = DATA + 0x3000;
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 7 == 0 { 0 } else { hud },
                );
                c.bus
                    .write32(hud + 0x68, [0, 0, 5, 5][(seed / 5) as usize % 4]);
                c.bus
                    .write32(hud + 0x64, [0, 7, 0, 7][(seed / 2) as usize % 4]);
                oc1023_screen_tables(c, 1 + seed % 3);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `OVScreenPauseFreeride_OnDeactivate` (0x001f_8720) against the generated body.
    #[test]
    fn ov_screen_pause_freeride_on_deactivate_001f8720_matches() {
        check(
            ee::s04::f_001f8720__generated,
            ov_screen_pause_freeride_on_deactivate_001f8720,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1f8720, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenPauseFreeride_HandleInput` (0x001f_8df0) against the generated body.
    #[test]
    fn ov_screen_pause_freeride_handle_input_001f8df0_matches() {
        check(
            ee::s04::f_001f8df0__generated,
            ov_screen_pause_freeride_handle_input_001f8df0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1f8df0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
