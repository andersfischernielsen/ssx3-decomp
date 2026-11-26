//! `src/fe/festateaudiooptions.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::fe::feasyncfile::set_widget_enabled;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::sound::bankmonitor::p;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_7db8,
        cfe_request_line_update_scroll_arrows,
        "cFEStateRequestLine_UpdateScrollArrows",
    ),
    (
        0x0019_7b88,
        fe_state_request_line_refresh_list,
        "cFEStateRequestLine_RefreshList",
    ),
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
    (
        0x0019_6540,
        cfe_state_audio_options_on_widget_created,
        "cFEStateAudioOptions_OnWidgetCreated",
    ),
    (
        0x0019_66e0,
        cfe_state_audio_options_on_widget_action,
        "cFEStateAudioOptions_OnWidgetAction",
    ),
    (
        0x0019_6b90,
        cfe_state_request_line_ctor,
        "cFEStateRequestLine_ctor",
    ),
    (
        0x0019_6db0,
        cfe_state_request_line_init,
        "cFEStateRequestLine_Init",
    ),
];

// ---------------------------------------------------------------------------
// Callees (the generated name of each address, so a later hand-written
// replacement of a callee is reached through its override slot too).

pub(crate) mod callee {

    pub(crate) use crate::game::dirtysock::tagsunk::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
}

/// `void cFEStateRequestLine_RefreshList(cFEStateRequestLine* self)`
/// (0x197B88): refresh the song names, icons, scroll arrows and the song
/// count of the request-line screen.
pub fn fe_state_request_line_refresh_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::REQUEST_LINE_UPDATE_SONG_NAMES.call(c, 0x0019_7b9c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SONG_ICONS.call(c, 0x0019_7ba4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS.call(c, 0x0019_7bac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REQUEST_LINE_UPDATE_SONG_COUNT.call(c, 0x0019_7bb4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_UpdateScrollArrows(self)` (0x197db8): the same
/// two-arrow update for the jukebox request list (widgets at +0x1ec/+0x1f0).
pub fn cfe_request_line_update_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let count = Addr(this as u32).off(0x150).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x1ec).lw(c);
        if count == 0 {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x0019_7dec)?;
            c.cycles += 5;
        } else {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x0019_7e0c)?;
            c.cycles += 5;
        }
        c.g[V0][0] = Addr(this as u32).off(0x150).lw(c);
        c.g[A2][0] = Addr(this as u32).off(0x1f0).lw(c);
        c.g[V1][0] = Addr(this as u32).off(0x14c).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            set_widget_enabled(c, 1, 0x0019_7e5c)?;
        } else {
            c.cycles += 8;
            set_widget_enabled(c, 0, 0x0019_7e3c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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

/// `void cFEStateAudioOptions_OnWidgetCreated(self, widget)` (0x196540): the
/// audio options screen caches the widgets it finds by hash name (`widget+0x38`):
/// the four check icons (`+0x4c/+0x50/+0x54/+0x58`), the radio buttons
/// (`+0x64/+0x68/+0x6c`) and the three menu entries, and sets each cached
/// icon's text to the checked or the empty box according to the radio mode
/// (`+0x78`). A widget matching none of the names is ignored.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_audio_options_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1966c8: {
            'b_1966c4: {
                'b_196640: {
                    'b_196630: {
                        'b_19662c: {
                            'b_19658c: {
                                c.cycles += 14;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[S1][0] = c.g[A0][0];
                                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[S0][0] = c.g[A1][0];
                                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                c.g[A0][0] = 0x4a0000;
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1830));
                                c.g[RA][0] = sx(0x00196570);
                                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                                call_ptr(c, 0x00317670, 0x00196570)?;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[A0][0] = 0x4a0000;
                                if t {
                                    break 'b_19658c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    let t = c.g[V0][0] != 0u64;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_19658c;
                                        }
                                    }
                                }
                            }
                            'b_1965b4: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00196594);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1838));
                                call_ptr(c, 0x00317670, 0x00196594)?;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[V0][0] = 0x4a0000;
                                if t {
                                    break 'b_1965b4;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_1965b4;
                                        }
                                    }
                                }
                            }
                            'b_1965dc: {
                                c.cycles += 5;
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1840));
                                c.g[RA][0] = sx(0x001965c0);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x00317670, 0x001965c0)?;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_1965dc;
                                } else {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_1965dc;
                                        }
                                    }
                                }
                            }
                            'b_196604: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001965e4);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x00317670, 0x001965e4)?;
                                if c.g[S2][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x4a0000;
                                    break 'b_196604;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_196604;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0019660c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1848));
                            call_ptr(c, 0x00317670, 0x0019660c)?;
                            if c.g[S2][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_196640;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[S0][0]);
                                if t {
                                    break 'b_19662c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                    if t {
                                        break 'b_196630;
                                    } else {
                                        break 'b_19662c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                        break 'b_196630;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                    if t {
                        break 'b_1966c4;
                    } else {
                        break 'b_196640;
                    }
                }
                'b_196658: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00196648);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x420));
                    call_ptr(c, 0x00317670, 0x00196648)?;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        break 'b_196658;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1966c4;
                        } else {
                            break 'b_196658;
                        }
                    }
                }
                'b_1966c0: {
                    'b_196670: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196660);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x430));
                        call_ptr(c, 0x00317670, 0x00196660)?;
                        if c.g[S2][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_196670;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1966c0;
                            } else {
                                break 'b_196670;
                            }
                        }
                    }
                    'b_19668c: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196678);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x440));
                        call_ptr(c, 0x00317670, 0x00196678)?;
                        if c.g[S2][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_19668c;
                        } else {
                            c.cycles += 3;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x68), c.g[S0][0]);
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1966c0;
                            } else {
                                break 'b_19668c;
                            }
                        }
                    }
                    'b_1966a8: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196694);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x450));
                        call_ptr(c, 0x00317670, 0x00196694)?;
                        if c.g[S2][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_1966a8;
                        } else {
                            c.cycles += 3;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x64), c.g[S0][0]);
                            let t = true;
                            c.g[V0][0] = 0x4;
                            if t {
                                break 'b_1966c0;
                            } else {
                                break 'b_1966a8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001966b0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x460));
                    call_ptr(c, 0x00317670, 0x001966b0)?;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1966c8;
                    } else {
                        c.cycles += 2;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
                        c.g[V0][0] = 0x3;
                        break 'b_1966c0;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_1966c4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1966c8;
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
    })
}

/// `void cFEStateAudioOptions_OnWidgetAction(self, widget, event)` (0x1966e0):
/// the audio options screen's widget actions. Event 5 and 6 (a jump table at
/// `0x460560`) select the radio mode of the activated entry (`widget+0x18`, the
/// radio mode at `self+0x78`), re-draw the checked/empty boxes and notify the
/// screen's widgets through their vtables; event 1 opens a sub-screen. A resume
/// (`pc != entry`) goes to the generated twin.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_audio_options_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001966e0 {
        ee::s02::f_001966e0__entries(c);
        return;
    }
    guest(c, |c| 'l_1966e0: loop {
        'b_196ae8: {
            'b_196ae4: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S2][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_196ae4;
                } else {
                    'b_19692c: {
                        'b_19691c: {
                            'b_19688c: {
                                'b_196740: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                    if t {
                                        break 'b_196740;
                                    } else {
                                        'b_19672c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_19672c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_19692c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_196ae8;
                                                    } else {
                                                        break 'b_19672c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];

                                        if t {
                                            break 'b_19688c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_196ae8;
                                            } else {
                                                break 'b_196740;
                                            }
                                        }
                                    }
                                }
                                'b_1967f4: {
                                    'b_196770: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_196770;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_196770;
                                            } else {
                                                'b_196764: {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_196764;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x4;
                                                        if t {
                                                            break 'b_1967f4;
                                                        } else {
                                                            break 'b_196764;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                                                if c.g[V0][0] == 0u64 {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    break 'b_196ae8;
                                                } else {
                                                    break 'b_196770;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 33;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                                    c.g[A2][0] = 0xffffffffffffffff;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[A2][0]);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A2][0]);
                                    c.g[RA][0] = sx(0x001967b8);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[A1][0]);
                                    call_ptr(c, 0x0014dc80, 0x001967b8)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001967c4);
                                    c.g[A1][0] = 0xd;
                                    call_ptr(c, 0x0014dd58, 0x001967c4)?;
                                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                                    c.g[RA][0] = sx(0x001967d8);
                                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                    call_ptr(c, 0x001587f8, 0x001967d8)?;
                                    c.g[RA][0] = sx(0x001967e0);
                                    call_ptr(c, 0x0028b180, 0x001967e0)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                    c.g[RA][0] = sx(0x001967ec);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0028bf78, 0x001967ec)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_196ae8;
                                    } else {
                                        break 'b_1967f4;
                                    }
                                }
                                c.cycles += 2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_196ae8;
                                } else {
                                    'b_196814: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = 0x460000;
                                        if t {
                                            break 'b_196814;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                                            if c.g[V0][0] == 0u64 {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                break 'b_196ae8;
                                            } else {
                                                break 'b_196814;
                                            }
                                        }
                                    }
                                    'b_196880: {
                                        c.cycles += 15;
                                        c.g[A0][0] = 0x210;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x00196828);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x00196828)?;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                        c.g[RA][0] = sx(0x0019683c);
                                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                        call_ptr(c, 0x00196b90, 0x0019683c)?;
                                        c.g[T0][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V1][0] = 0xffffffffffffc0ff;
                                        if t {
                                            break 'b_196880;
                                        } else {
                                            c.cycles += 12;
                                            c.g[V0][0] =
                                                c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                            c.g[A1][0] = c.g[T0][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                            c.g[A3][0] = 0u64;
                                            c.g[V0][0] = c.g[V0][0] | 0x100;
                                            c.sw(
                                                (c.g[T0][0] as u32).wrapping_add(0x1c),
                                                c.g[V0][0],
                                            );
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[RA][0] = sx(0x00196878);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f320, 0x00196878)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_196ae8;
                                            } else {
                                                break 'b_196880;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    let t = true;
                                    c.g[A1][0] = c.g[T0][0];
                                    if t {
                                        break 'b_19691c;
                                    } else {
                                        break 'b_19688c;
                                    }
                                }
                            }
                            'b_1968ec: {
                                c.cycles += 19;
                                c.g[RA][0] = sx(0x00196894);
                                call_ptr(c, 0x0014dc80, 0x00196894)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001968a0);
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x001968a0)?;
                                c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                                c.g[RA][0] = sx(0x001968b4);
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, 0x001587f8, 0x001968b4)?;
                                c.g[RA][0] = sx(0x001968bc);
                                call_ptr(c, 0x0028b180, 0x001968bc)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                c.g[RA][0] = sx(0x001968c8);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0028bf78, 0x001968c8)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_1968ec;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001968e4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x001968e4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_196ae8;
                                    } else {
                                        break 'b_1968ec;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00196910);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196910)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_196ae4;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                break 'b_19691c;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196924);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f400, 0x00196924)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_196ae8;
                        } else {
                            break 'b_19692c;
                        }
                    }
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x00196934);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    call_ptr(c, 0x00317670, 0x00196934)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00196940);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00196940)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[S4][0] = 0u64;
                    if t {
                        break 'b_196ae4;
                    } else {
                        'b_196a88: {
                            'b_196a84: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                c.g[S2][0] = 0u64;
                                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[S3][0] = 0u64;
                                if t {
                                    break 'b_196a84;
                                } else {
                                    'b_196a70: {
                                        'b_196a54: {
                                            'b_196a4c: {
                                                'b_196a1c: {
                                                    'b_1969d8: {
                                                        'b_196994: {
                                                            'b_196988: {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = 0x460000;
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x560));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x001966e0 => {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x001966e0,
                                                                            );
                                                                        }
                                                                        continue 'l_1966e0;
                                                                    }
                                                                    0x0019697c => {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x498,
                                                                                ));
                                                                        if t {
                                                                            break 'b_196a54;
                                                                        } else {
                                                                            break 'b_196988;
                                                                        }
                                                                    }
                                                                    0x00196988 => {
                                                                        break 'b_196988;
                                                                    }
                                                                    0x00196994 => {
                                                                        break 'b_196994;
                                                                    }
                                                                    0x001969d8 => {
                                                                        break 'b_1969d8;
                                                                    }
                                                                    0x00196a1c => {
                                                                        break 'b_196a1c;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x001966e0,
                                                                            0x00196b04,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4b0));
                                                            if t {
                                                                break 'b_196a54;
                                                            } else {
                                                                break 'b_196994;
                                                            }
                                                        }
                                                        'b_1969d0: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x7c));
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_1969d0;
                                                            } else {
                                                                'b_1969bc: {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x84));
                                                                    let t = c.g[V1][0] == 0u64;
                                                                    c.g[S3][0] =
                                                                        (0u64 < c.g[V1][0]) as u64;
                                                                    if t {
                                                                        break 'b_1969bc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x4c0,
                                                                                ));
                                                                        if t {
                                                                            break 'b_196a54;
                                                                        } else {
                                                                            break 'b_1969bc;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x74));
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[A0][0] = 0x460000;
                                                                if t {
                                                                    break 'b_196a4c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4e0));
                                                                    if t {
                                                                        break 'b_196a54;
                                                                    } else {
                                                                        break 'b_1969d0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x510));
                                                        if t {
                                                            break 'b_196a54;
                                                        } else {
                                                            break 'b_1969d8;
                                                        }
                                                    }
                                                    'b_196a14: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x7c));
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A0][0] = 0x460000;
                                                        if t {
                                                            break 'b_196a14;
                                                        } else {
                                                            'b_196a00: {
                                                                c.cycles += 4;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                let t = c.g[V1][0] == 0u64;
                                                                c.g[S4][0] =
                                                                    (0u64 < c.g[V1][0]) as u64;
                                                                if t {
                                                                    break 'b_196a00;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x460000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4c0));
                                                                    if t {
                                                                        break 'b_196a54;
                                                                    } else {
                                                                        break 'b_196a00;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x74));
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_196a4c;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4e0));
                                                                if t {
                                                                    break 'b_196a54;
                                                                } else {
                                                                    break 'b_196a14;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x528));
                                                    if t {
                                                        break 'b_196a54;
                                                    } else {
                                                        break 'b_196a1c;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_196a70;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_196a70;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        let t = c.g[V1][0] == 0u64;
                                                        c.g[S2][0] = (0u64 < c.g[V1][0]) as u64;
                                                        if t {
                                                            break 'b_196a4c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4c0));
                                                            if t {
                                                                break 'b_196a54;
                                                            } else {
                                                                break 'b_196a4c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x4f8));
                                            break 'b_196a54;
                                        }
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00196a5c);
                                        call_ptr(c, 0x00317670, 0x00196a5c)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x00196a68);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x00196a68)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_196a88;
                                        } else {
                                            break 'b_196a70;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[RA][0] = sx(0x00196a78);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x540));
                                    call_ptr(c, 0x00317670, 0x00196a78)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00196a84);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x00196a84)?;
                                    break 'b_196a84;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            break 'b_196a88;
                        }
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        let t = c.g[V1][0] == 0u64;
                        c.g[A1][0] = c.g[S2][0];
                        if t {
                            break 'b_196ae4;
                        } else {
                            c.cycles += 20;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196aac);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196aac)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                            c.g[A1][0] = c.g[S3][0];
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196ac8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196ac8)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.g[A1][0] = c.g[S4][0];
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196ae4);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196ae4)?;
                            break 'b_196ae4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_196ae8;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateRequestLine *cFEStateRequestLine_ctor(self, int a1, int idx, int inStore)`
/// (0x196b90): the UIState base constructor, then the request-line state's
/// fields: player index (`+0x44`), the player's control mask (`+0x15`), the
/// 0x100-byte buffer cleared, the song lists and counters zeroed, and the
/// three widgets found by hash name (`+0x204/+0x208/+0x20c`).
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_request_line_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_196bf0: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00196bb8);
            c.g[S2][0] = c.g[A3][0];
            call_ptr(c, 0x0039e2a0, 0x00196bb8)?;
            c.g[V0][0] = 0x11;
            c.g[V1][0] = 0x470000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c88));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sb((c.g[S1][0] as u32).wrapping_add(0x44), c.g[S0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
                break 'b_196bf0;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00196be8);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a1cd0, 0x00196be8)?;
                c.sb((c.g[S1][0] as u32).wrapping_add(0x15), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
                break 'b_196bf0;
            }
        }
        c.cycles += 10;
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00196bfc);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x003e6448, 0x00196bfc)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14c), 0u64);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1bc));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x150), 0u64);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x194));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x170), 0u64);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x174));
        c.g[A0][0] = 0x7;
        'l_196c18: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            let t = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00196c18);
                }
                continue 'l_196c18;
            } else {
                c.cycles += 5;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1b4), 0u64);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1b8), 0u64);
                c.g[V1][0] = 0x5;
                'l_196c48: loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00196c48);
                        }
                        continue 'l_196c48;
                    } else {
                        c.cycles += 82;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x16c), c.g[S2][0]);
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25b8));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1dc), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d8), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e4), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e8), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1ec), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1f0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1f4), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1f8), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x200), 0u64);
                        c.sd((c.g[S1][0] as u32).wrapping_add(0x158), 0u64);
                        c.g[RA][0] = sx(0x00196cac);
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        call_ptr(c, 0x00317670, 0x00196cac)?;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[RA][0] = sx(0x00196cb8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00398380, 0x00196cb8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25c8));
                        c.g[RA][0] = sx(0x00196ccc);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x204), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00196ccc)?;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[RA][0] = sx(0x00196cd8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00398380, 0x00196cd8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25a8));
                        c.g[RA][0] = sx(0x00196cec);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x208), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00196cec)?;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[RA][0] = sx(0x00196cf8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00398380, 0x00196cf8)?;
                        c.g[RA][0] = sx(0x00196d00);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x20c), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x00196d00)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d0c);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x00196d0c)?;
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[RA][0] = sx(0x00196d18);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00147488, 0x00196d18)?;
                        c.g[RA][0] = sx(0x00196d20);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x00196d20)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d2c);
                        c.g[A1][0] = 0xb;
                        call_ptr(c, 0x0014dd58, 0x00196d2c)?;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d3c);
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        call_ptr(c, 0x00150928, 0x00196d3c)?;
                        c.g[RA][0] = sx(0x00196d44);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x168), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x00196d44)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d50);
                        c.g[A1][0] = 0xd;
                        call_ptr(c, 0x0014dd58, 0x00196d50)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x00196d64);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00158728, 0x00196d64)?;
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x00196d78);
                        c.sd((c.g[S1][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                        call_ptr(c, 0x00158750, 0x00196d78)?;
                        c.g[RA][0] = sx(0x00196d80);
                        c.sd((c.g[S1][0] as u32).wrapping_add(0x158), c.g[V0][0]);
                        call_ptr(c, 0x0028b180, 0x00196d80)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00196d90);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
                        call_ptr(c, 0x00197ad8, 0x00196d90)?;
                        c.g[V0][0] = c.g[S1][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `cFEStateRequestLine_Init` (0x196db0): the request-line state's screen
/// setup: the economy, reward and audio interfaces are queried for the
/// player's money, owned song list and play list, then the play list is
/// generated and the list/now-playing/help text refreshed.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_request_line_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_196e28: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = 0x4a0000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
            let t = c.g[V0][0] == 0u64;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18b0));
            if t {
                break 'b_196e28;
            } else {
                c.cycles += 13;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x460000;
                c.g[A0][0] = 0x4a0000;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x578));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[RA][0] = sx(0x00196df8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
                call_ptr(c, 0x00317670, 0x00196df8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00196e04);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039f9d8, 0x00196e04)?;
                c.g[A3][0] = c.g[V0][0];
                let t = c.g[A3][0] == 0u64;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_196e28;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00196e28);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00196e28)?;
                    break 'b_196e28;
                }
            }
        }
        'b_196e5c: {
            c.cycles += 10;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x00196e34);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00196e34)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00196e48);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00196e48)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_196e5c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00196e5c);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00196e5c)?;
                break 'b_196e5c;
            }
        }
        c.cycles += 9;
        c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x14c));
        c.g[RA][0] = sx(0x00196e68);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
        call_ptr(c, 0x0039b760, 0x00196e68)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `int cFEStateRequestLine_TransitionIn(self, int first_time)` (0x196f50):
/// on the first transition in, refreshes the scroll arrows, the button text
/// and the help text; returns 0.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
#[cfg(test)] // not installed: its test runs away in unbounded real callees
#[allow(dead_code)]
pub fn cfe_state_request_line_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_196f84: {
            c.cycles += 5;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            let t = c.g[A1][0] == 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_196f84;
            } else {
                c.cycles += 8;
                c.g[RA][0] = sx(0x00196f6c);
                call_ptr(c, 0x00197db8, 0x00196f6c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00196f78);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00198340, 0x00196f78)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00196f84);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001981f8, 0x00196f84)?;
                break 'b_196f84;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;

    #[test]
    fn fe_state_request_line_refresh_list_matches() {
        check(
            ssx3_ee::s02::f_00197b88__generated,
            fe_state_request_line_refresh_list,
            60,
            &|c, seed| {
                machine(c, seed, 0x197b_88, 4);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    pub(crate) use crate::game::be::beprepoststate::tests::belib as belib_m2;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;

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
                belib_m2(c);
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

    use crate::difftest::fakes;
    use crate::difftest::Rng;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;
    use std::sync::OnceLock;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const OC853_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC853_MAGIC: u32 = 0x4f43_3835;

    /// The state object (`self`).
    pub(crate) const OC853_SELF: u32 = DATA + 0x21_0000;

    /// The widget handed to the action/created handlers.
    pub(crate) const OC853_WIDGET: u32 = DATA + 0x22_0000;

    /// A scattered pool the object pointers point into (0x4000 bytes).
    pub(crate) const OC853_POOL: u32 = DATA + 0x23_0000;

    /// The world object at `gp-0x848`.
    pub(crate) const OC853_WORLD: u32 = DATA + 0x24_0000;

    /// The forced-result word of the fake for `addr` (1 = return 0).
    pub(crate) fn oc853_mail(addr: u32) -> u32 {
        OC853_CTRL + 0x100 + (addr & 0xffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns 0 if
    /// its mailbox asks for it.
    pub(crate) fn oc853_fake_call(c: &mut Cpu, addr: u32) {
        let mail = c.bus.read32(oc853_mail(addr));
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc853_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC853_CTRL) != OC853_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc853_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc853_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc853_fakes!(
        (
            oc853_fake_14dc80,
            OC853_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc853_fake_14dd58,
            OC853_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc853_fake_1587f8,
            OC853_PREV_1587F8,
            ee::s01::OVERRIDE_001587F8,
            ee::s01::f_001587f8__generated,
            0x001587f8
        ),
        (
            oc853_fake_147488,
            OC853_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x00147488
        ),
        (
            oc853_fake_150928,
            OC853_PREV_150928,
            ee::s01::OVERRIDE_00150928,
            ee::s01::f_00150928__generated,
            0x00150928
        ),
        (
            oc853_fake_158728,
            OC853_PREV_158728,
            ee::s01::OVERRIDE_00158728,
            ee::s01::f_00158728__generated,
            0x00158728
        ),
        (
            oc853_fake_158750,
            OC853_PREV_158750,
            ee::s01::OVERRIDE_00158750,
            ee::s01::f_00158750__generated,
            0x00158750
        ),
        (
            oc853_fake_158848,
            OC853_PREV_158848,
            ee::s01::OVERRIDE_00158848,
            ee::s01::f_00158848__generated,
            0x00158848
        ),
        (
            oc853_fake_158820,
            OC853_PREV_158820,
            ee::s01::OVERRIDE_00158820,
            ee::s01::f_00158820__generated,
            0x00158820
        ),
        (
            oc853_fake_28b180,
            OC853_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc853_fake_28bf78,
            OC853_PREV_28BF78,
            ee::s07::OVERRIDE_0028BF78,
            ee::s07::f_0028bf78__generated,
            0x0028bf78
        ),
        (
            oc853_fake_28c2d0,
            OC853_PREV_28C2D0,
            ee::s07::OVERRIDE_0028C2D0,
            ee::s07::f_0028c2d0__generated,
            0x0028c2d0
        ),
        (
            oc853_fake_317d70,
            OC853_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc853_fake_39f320,
            OC853_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc853_fake_39f190,
            OC853_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039f190
        ),
        (
            oc853_fake_39f400,
            OC853_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039f400
        ),
        (
            oc853_fake_39d860,
            OC853_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc853_fake_3a1000,
            OC853_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc853_fake_39e2a0,
            OC853_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039e2a0
        ),
        (
            oc853_fake_398380,
            OC853_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x00398380
        ),
        (
            oc853_fake_39f9d8,
            OC853_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039f9d8
        ),
        (
            oc853_fake_397cc0,
            OC853_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc853_fake_39c870,
            OC853_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc853_fake_39b760,
            OC853_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039b760
        ),
        (
            oc853_fake_1a1cd0,
            OC853_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a1cd0
        ),
        (
            oc853_fake_198988,
            OC853_PREV_198988,
            ee::s03::OVERRIDE_00198988,
            ee::s03::f_00198988__generated,
            0x00198988
        ),
        (
            oc853_fake_3e6448,
            OC853_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e6448
        ),
        (
            oc853_fake_197ad8,
            OC853_PREV_197AD8,
            ee::s02::OVERRIDE_00197AD8,
            ee::s02::f_00197ad8__generated,
            0x00197ad8
        ),
        (
            oc853_fake_198118,
            OC853_PREV_198118,
            ee::s02::OVERRIDE_00198118,
            ee::s02::f_00198118__generated,
            0x00198118
        ),
    );

    /// Words that are 0, a small 16-aligned count/address, or a pointer into the
    /// pool (so real callees reading the object find shallow data and virtual
    /// calls land in the fakes).
    pub(crate) fn oc853_scatter(c: &mut Cpu, r: &mut Rng, base: u32, words: u32) {
        for i in 0..words {
            let v = match r.next_u32() % 5 {
                0 | 1 => 0,
                2 => (r.next_u32() % 8) * 16,
                _ => OC853_POOL + (r.next_u32() & 0x3ff0),
            };
            c.bus.write32(base + 4 * i, v);
        }
    }

    pub(crate) fn oc853_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The machine for one seed: fakes hooked and gated on, the pool, `self`
    /// and the widget scattered, the world at `gp-0x848`.
    pub(crate) fn oc853_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc853_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        oc853_w(c, OC853_CTRL, OC853_MAGIC);
        oc853_scatter(c, &mut r, OC853_POOL, 0x1000);
        oc853_scatter(c, &mut r, OC853_SELF, 0x100);
        oc853_scatter(c, &mut r, OC853_WIDGET, 0x40);
        oc853_w(c, GAME_GP.wrapping_sub(0x848), OC853_WORLD);
        c.g[A0][0] = sx(OC853_SELF);
        c.g[A1][0] = sx(OC853_WIDGET);
        r
    }

    /// `StrHash` of the strings `cFEStateAudioOptions_OnWidgetCreated` compares
    /// the widget's hash name with, in the order it tests them.
    pub(crate) fn oc853_hashes() -> &'static [u32; 9] {
        static H: OnceLock<[u32; 9]> = OnceLock::new();
        H.get_or_init(|| {
            let ids = [
                0x004a_1830u32,
                0x004a_1838,
                0x004a_1840,
                0x004a_1848,
                0x0046_0420,
                0x0046_0430,
                0x0046_0440,
                0x0046_0450,
                0x0046_0460,
            ];
            ids.map(|a| {
                let mut c = crate::difftest::fresh();
                c.g[A0][0] = a as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0031_7670;
                ee::s10::f_00317670(&mut c);
                c.g[V0][0] as u32
            })
        })
    }

    /// Whether the widget matches each id string in turn, a non-matching hash
    /// and the zero hash; the radio mode (`self+0x78`) takes 0..=4.
    #[test]
    fn cfe_state_audio_options_on_widget_created_matches() {
        check(
            ee::s02::f_00196540__generated,
            cfe_state_audio_options_on_widget_created,
            220,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_6540, 0);
                c.pc = 0x0019_6540;
                let pick = (seed % 11) as usize;
                let hash = match pick {
                    0..=8 => oc853_hashes()[pick],
                    9 => r.next_u32() | 1,
                    _ => 0,
                };
                oc853_w(c, OC853_WIDGET + 0x38, hash);
                oc853_w(c, OC853_SELF + 0x78, (seed / 11) % 5);
            },
        );
    }

    /// The chain `cFEStateAudioOptions_OnWidgetAction` follows for its virtual
    /// call: `self+0x10 -> obj -> vtable (+4) -> +0x28 (this delta), +0x2c`.
    /// The slot holds a real leaf returning 0, a fake slot or null.
    pub(crate) fn oc853_chain(c: &mut Cpu, seed: u32) {
        let mgr = OC853_POOL + 0x100;
        let obj = OC853_POOL + 0x200;
        let vt = OC853_POOL + 0x300;
        oc853_w(c, OC853_SELF + 0x10, mgr);
        oc853_w(c, mgr, obj);
        oc853_w(c, obj + 4, vt);
        c.bus.write16(vt + 0x28, 0);
        let target = [0x0014_06c0u32, fakes::FAKE, 0, 0x10][((seed / 7) % 4) as usize];
        oc853_w(c, vt + 0x2c, target);
    }

    /// Events 5 and 6 (the jump table by `widget+0x18`, 0..=7) and the
    /// others, a null widget, the world flag at `+0x84`, the sub-state fields.
    #[test]
    fn cfe_state_audio_options_on_widget_action_matches() {
        check(
            ee::s02::f_001966e0__generated,
            cfe_state_audio_options_on_widget_action,
            400,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_66e0, 14);
                c.pc = 0x0019_66e0;
                oc853_chain(c, seed);
                c.g[A1][0] = if seed % 9 == 0 { 0 } else { sx(OC853_WIDGET) };
                c.g[A2][0] = [5u64, 6, 1, 0, 2, 9, 5, 6, 1, 3][(seed % 10) as usize];
                oc853_w(c, OC853_WIDGET + 0x18, r.next_u32() % 8);
                oc853_w(c, OC853_SELF + 0x40, OC853_POOL + 0x400);
                oc853_w(c, OC853_SELF + 0x70, r.next_u32() % 4);
                oc853_w(c, OC853_SELF + 0x74, r.next_u32() % 2);
                oc853_w(c, OC853_SELF + 0x78, r.next_u32() % 5);
                oc853_w(c, OC853_SELF + 0x7c, [0u32, 1, 3][(seed % 3) as usize]);
                oc853_w(
                    c,
                    OC853_WORLD + 0x84,
                    if seed % 4 == 0 { 0 } else { OC853_POOL },
                );
                oc853_w(c, oc853_mail(0x0039_d860), (seed % 5 == 0) as u32);
            },
        );
    }

    /// The constructor: with and without a `**(self+0x10)` (the control-mask
    /// lookup), negative player indices, in-store flag.
    #[test]
    fn cfe_state_request_line_ctor_matches() {
        check(
            ee::s02::f_00196b90__generated,
            cfe_state_request_line_ctor,
            200,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_6b90, 16);
                c.pc = 0x0019_6b90;
                c.g[A2][0] =
                    (r.next_u32() % 5) as u64 | if seed % 7 == 0 { 0xffff_ff00 } else { 0 };
                c.g[A3][0] = (seed % 2) as u64;
                let mgr = OC853_POOL + 0x100;
                oc853_w(c, OC853_SELF + 0x10, mgr);
                oc853_w(c, mgr, if seed % 3 == 0 { 0 } else { OC853_POOL + 0x200 });
            },
        );
    }

    /// The setup: the world flag (`+0x84`) off/on, the screen's lookups
    /// returning nothing or a found object.
    #[test]
    fn cfe_state_request_line_init_matches() {
        check(
            ee::s02::f_00196db0__generated,
            cfe_state_request_line_init,
            200,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_6db0, 5);
                c.pc = 0x0019_6db0;
                oc853_w(
                    c,
                    OC853_WORLD + 0x84,
                    if seed % 4 == 0 { 0 } else { OC853_POOL },
                );
                oc853_w(c, OC853_SELF + 0x10, OC853_POOL + 0x100);
                c.bus.write8(OC853_SELF + 0x14c, r.next_u32() as u8);
                oc853_w(c, oc853_mail(0x0039_f9d8), (seed % 3 == 0) as u32);
                oc853_w(c, oc853_mail(0x0039_7cc0), (seed % 2 == 0) as u32);
            },
        );
    }
}
