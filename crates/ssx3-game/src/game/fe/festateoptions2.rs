//! `src/fe/festateoptions2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
#[cfg(test)]
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_6b08,
        fe_state_audio_options_on_widget_notify,
        "cFEStateAudioOptions_OnWidgetNotify",
    ),
    (
        0x0019_6e80,
        cfestaterequestline_shutdown,
        "cFEStateRequestLine_Shutdown",
    ),
    (
        0x0019_6f00,
        cfe_state_request_line_on_activate,
        "cFEStateRequestLine_OnActivate",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;

    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;

    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;

    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;

    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
}

/// `void cFEStateRequestLine_OnActivate(cFEStateRequestLine* s)` (0x196f00):
/// run the base activation, reset the request list, refresh the list and the
/// now-playing line, then update the button and help texts. 20 cycles plus
/// the callees.
pub fn cfe_state_request_line_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::FE_STATE_ON_ACTIVATE.call(c, 0x0019_6f14)?;
        c.sw(p(self_, 0x150), 0);
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_REFRESH_LIST.call(c, 0x0019_6f20)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_NOW_PLAYING.call(c, 0x0019_6f28)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::FE_STATE_UPDATE_BUTTONS_TEXT, 0x0019_6f34)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::REQUEST_LINE_UPDATE_HELP_TEXT.call(c, 0x0019_6f40)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateAudioOptions_OnWidgetNotify(cFEStateAudioOptions* self,
/// void* widget, int id, int kind)` (0x196B08): the volume sliders (kind
/// 2..4) answer 0x10 when their channel is idle, `Back`/`Ok` (8, 9) answer 0.
pub fn fe_state_audio_options_on_widget_notify(c: &mut Cpu) {
    /// The `0x101` result of a rejected widget.
    fn rejected(c: &mut Cpu) {
        c.cycles += 2;
        c.g[V0][0] = WIDGET_IGNORED;
        abi::ret(c);
    }
    /// The `0x10` result of an accepted slider.
    fn accepted(c: &mut Cpu) {
        c.cycles += 2;
        c.g[V0][0] = 0x10;
        abi::ret(c);
    }
    /// The tail both non-slider outcomes share (`a2 - 8 < 2` is `Back`/`Ok`).
    fn not_a_slider(c: &mut Cpu) {
        c.cycles += 3;
        c.g[V1][0] = (c.g[V1][0] < 2) as u64;
        let back_or_ok = c.g[V1][0] != 0;
        c.g[V0][0] = 0;
        if back_or_ok {
            c.cycles += 2;
            abi::ret(c);
        } else {
            c.cycles += 1;
            c.g[V0][0] = WIDGET_IGNORED;
            c.cycles += 2;
            abi::ret(c);
        }
    }

    c.cycles += 3;
    c.g[V0][0] = 6;
    if c.g[A2][0] != c.g[V0][0] {
        c.g[V1][0] = sx(ea(c.g[A2][0], 0xffff_fff8));
        return not_a_slider(c);
    }
    c.g[V1][0] = sx(ea(c.g[A2][0], 0xffff_fff8));
    c.cycles += 4;
    c.g[V0][0] = sx(ea(c.g[A3][0], 0xffff_fffe));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    let not_a_channel = c.g[V0][0] == 0;
    c.g[V0][0] = 2;
    if not_a_channel {
        return not_a_slider(c);
    }
    // Channel 2: the first slider is idle when +0x7C is clear.
    c.cycles += 2;
    let is_channel_two = c.g[A3][0] == c.g[V0][0];
    c.g[V0][0] = 3;
    if is_channel_two {
        c.cycles += 3;
        c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x7c));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 3;
        if idle {
            return accepted(c);
        }
    }
    // Channel 3.
    c.cycles += 2;
    let is_channel_three = c.g[A3][0] == c.g[V0][0];
    c.g[V0][0] = 4;
    if is_channel_three {
        c.cycles += 3;
        c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x7c));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 4;
        if idle {
            return accepted(c);
        }
    }
    // Channel 4 (both sliders idle).
    c.cycles += 2;
    let is_channel_four = c.g[A3][0] == c.g[V0][0];
    c.g[V0][0] = WIDGET_IGNORED;
    if is_channel_four {
        c.cycles += 3;
        c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x74));
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = WIDGET_IGNORED;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = c.lw(ea(c.g[A0][0], 0x7c));
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = WIDGET_IGNORED;
            if !busy {
                return accepted(c);
            }
        }
    }
    rejected(c);
}

/// `void cFEStateRequestLine_Shutdown(self)` (0x196e80): the request-line
/// twin of the audio-options shutdown (vtable slot at `+0xc0`).
pub fn cfestaterequestline_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = lw_at(c, gw(c, -0x848));
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, p(c.g[V1][0] as u32, 0x84));
        let no_race = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        if !no_race {
            c.cycles += 11;
            c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x10));
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, callee::STR_HASH, 0x0019_6eb4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_STATE_MGR_FIND_STATE.addr, 0x0019_6ec0)?;
            c.g[A3][0] = c.g[V0][0];
            let none = c.g[A3][0] == 0;
            c.g[A1][0] = 4;
            if !none {
                c.cycles += 6;
                c.g[V1][0] = lw_at(c, p(c.g[A3][0] as u32, 8));
                c.g[A2][0] = 0;
                let func = vslot(c, c.g[V1][0], 0xc0, V0);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x0019_6ee4)?;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::UI_STATE_SHUTDOWN.addr, 0x0019_6eec)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;

    #[test]
    fn cfe_state_request_line_on_activate_matches() {
        check(
            ee::s02::f_00196f00__generated,
            cfe_state_request_line_on_activate,
            24,
            &|c, seed| {
                let _r = base(c, seed, 0x196f);
                let s = DATA;
                c.bus.write32(s + 0x150, 0xdead);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn fe_state_audio_options_on_widget_notify_matches() {
        check(
            ssx3_ee::s02::f_00196b08__generated,
            fe_state_audio_options_on_widget_notify,
            100,
            &|c, seed| {
                machine(c, seed, 0x0019_6b08, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 6,
                    1 => 8 + (seed % 2) as u64,
                    _ => 5,
                };
                c.g[A3][0] = (seed % 6) as u64;
                w(c, DATA + 0x74, if seed % 2 == 0 { 0 } else { 1 });
                w(c, DATA + 0x7c, if seed % 4 == 0 { 0 } else { 1 });
            },
        );
    }
}
