//! Staging module oc354 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc354`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0022_d8d8, section_man_update, "SectionMan_Update")];

// ---------------------------------------------------------------------------
// SectionMan_Update (0x0022d8d8)

/// `cSectionMan` fields `SectionMan_Update` uses.
pub(crate) mod section_man_update_layout {
    /// A sky swap is pending (`SectionMan_ApplySky` runs while set).
    pub const SKY_PENDING: u32 = 0x1a8;
    /// The owning game module (`+0x10` -> `*` -> the world view).
    pub const GAME: u32 = 0x1b0;
    /// The event-switch state machine (0 idle .. 4 waiting for the overlay).
    pub const SWITCH: u32 = 0x1b4;
    pub const SWITCH_AUX: u32 = 0x1b8;
    /// Streaming enabled (picks the stream rate when nothing changed).
    pub const STREAM_ON: u32 = 0x1c0;
    /// The stream-rate class (index into the rate table).
    pub const RATE: u32 = 0x1c4;
    /// Sections settled: 1 when nothing forces the slow stream rate.
    pub const READY: u32 = 0x1d0;
    /// The world view: active and visible flags.
    pub const WV_ACTIVE: u32 = 0x250;
    pub const WV_VISIBLE: u32 = 0x2a0;
    /// The world view's stream manager.
    pub const WV_STREAM_MAN: u32 = 0x300;

    /// The section event table (16 bytes per event: `+4` world section,
    /// `+8` state, `+0xc` sky index).
    pub const EVENTS: u32 = 0x0044_2168;
    /// Per-sky request priorities (floats).
    pub const SKY_PRIORITY: u32 = 0x0044_c208;
    /// Stream rates per rate class.
    pub const RATES: u32 = 0x0044_2150;
    /// Path prefixes ("data/worlds/_", "data/worlds/") and the "%s%s" format.
    pub const PREFIX_MULTI: u32 = 0x0047_b470;
    pub const PREFIX_SINGLE: u32 = 0x0047_b480;
    pub const PATH_FORMAT: u32 = 0x004a_2a18;
    /// The event-state `switch` jump table (states 0..=8) and its targets.
    pub const STATE_TABLE: u32 = 0x0047_b490;
    pub const STATE_CASES: [u32; 9] = [
        0x0022_db64,
        0x0022_db00,
        0x0022_db2c,
        0x0022_dae4,
        0x0022_daf0,
        0x0022_d9e8,
        0x0022_da2c,
        0x0022_d9b8,
        0x0022_da78,
    ];
    /// The switch-state jump table (states 0..=4) and its targets.
    pub const SWITCH_TABLE: u32 = 0x0047_b4c0;
    pub const SWITCH_CASES: [u32; 5] = [
        0x0022_ddd8,
        0x0022_dd80,
        0x0022_ddd8,
        0x0022_dd94,
        0x0022_ddac,
    ];
}

const SU_APPLY_SKY: Callee = Callee {
    addr: 0x0022_de98,
    f: ee::s05::SectionMan_ApplySky,
};
const SU_NEXT_QUEUED: Callee = Callee {
    addr: 0x0022_e348,
    f: ee::s05::SectionMan_NextQueued,
};
const SU_DEQUEUE: Callee = Callee {
    addr: 0x0022_e360,
    f: ee::s05::SectionMan_DequeueSection,
};
const SU_REQUEST_SKY: Callee = Callee {
    addr: 0x0022_de58,
    f: ee::s05::SectionMan_RequestSky,
};
const SU_BEGIN_SWITCH: Callee = Callee {
    addr: 0x0022_df50,
    f: ee::s05::SectionMan_BeginEventSwitch,
};
const SU_OVERLAY_DONE: Callee = Callee {
    addr: 0x0022_dfd0,
    f: ee::s05::SectionMan_IsEventSwitchOverlayDone,
};
const SU_FINISH_SWITCH: Callee = Callee {
    addr: 0x0022_dff0,
    f: ee::s05::SectionMan_FinishEventSwitch,
};

/// `cBE_getBE`, `BELibrary_GetInterface`, the world cache streamer queries
/// and the game module hooks (called through the function table).
const SU_GET_BE: u32 = 0x0014_dc80;
const SU_GET_INTERFACE: u32 = 0x0014_dd58;
const SU_SECTION_UNLOADED: u32 = 0x003a_9890;
const SU_RELEASE_SECTION: u32 = 0x003a_9958;
const SU_SECTION_RESIDENT: u32 = 0x003a_9858;
const SU_ACTIVATE_SECTION: u32 = 0x003a_99d8;
const SU_SECTION_COUNT: u32 = 0x003a_9800;
const SU_REQUEST_SECTION: u32 = 0x003a_98c8;
const SU_ON_SECTION_UNLOAD: u32 = 0x0023_0360;
const SU_INVALIDATE_FOCUS: u32 = 0x0023_0338;
const SU_SECTION_FIELD10: u32 = 0x0014_4d50;
const SU_SECTION_ENTRY: u32 = 0x0014_4d38;
const SU_SPRINTF: u32 = 0x0041_7828;
const SU_SET_RATE: u32 = 0x003a_7058;

/// The register dance before every world-cache call: `v0 = self->game`,
/// `a1 = s0` (the world section), `v1 = game->+0x10`, `a0 = *v1 + 0x10`.
fn section_man_update_cache_args(c: &mut Cpu) {
    use section_man_update_layout::GAME;
    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(GAME)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
}

/// `this` (s2, re-read after every call).
fn section_man_update_this(c: &Cpu) -> Addr {
    Addr(r32(c, S2))
}

/// `void cSectionMan::update(cSectionMan* this)` (0x22d8d8): the per-frame
/// world-section streaming state machine.
///
/// Applies a pending sky swap, then walks the queued section events: waits
/// for unloads (7) and releases (5, then `GameModule_OnSectionUnload`),
/// promotes loaded sections (6 -> 1, 8 -> 2 and activates them), collects
/// pending loads (3, 4), and retires finished events (0, 1, 2: requesting
/// their sky, dequeuing the node). When nothing is busy it requests the
/// pending loads from the streamer (with the event's sky priority and its
/// path built by `sprintf`). Then it advances the event-switch state
/// (`+0x1b4`), picks the stream-rate class (`+0x1c4`) and sets the stream
/// manager's rate from the rate table.
pub fn section_man_update(c: &mut Cpu) {
    use section_man_update_layout::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x1b0);
        frame.save(c, S2, 0x180);
        frame.save(c, S0, 0x1a0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x190);
        frame.save(c, S3, 0x170);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save(c, S6, 0x140);
        frame.save(c, S7, 0x130);
        frame.save(c, FP, 0x120);
        frame.save_ra(c, 0x110);
        c.g[V0][0] = section_man_update_this(c).off(SKY_PENDING).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            SU_APPLY_SKY.call(c, 0x0022_d91c)?;
        }
        c.g[A0][0] = c.g[S2][0];

        // The first queued event.
        c.cycles += 5;
        c.g[A1][0] = 0;
        SU_NEXT_QUEUED.call(c, 0x0022_d928)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[S6][0] = 0;
        if c.g[S4][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = section_man_update_this(c).off(SWITCH).lw(c);
            frame.restore(c, S0, 0x1a0);
            if c.g[V0][0] == 0 {
                // Nothing queued and no event switch: done.
                section_man_update_epilogue(c, frame);
                return Ok(());
            }
        }

        c.cycles += 10;
        c.g[S7][0] = 0; // busy
        call_ptr(c, SU_GET_BE, 0x0022_d948)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, SU_GET_INTERFACE, 0x0022_d954)?;
        c.g[FP][0] = 0; // changed
        let iface_slot = Addr(frame.at(c, 0x100));
        let force_off = Addr(frame.at(c, 0x104));
        let pending = Addr(frame.at(c, 0x108));
        let prefix_slot = Addr(frame.at(c, 0x10c));
        let iface = c.g[V0][0];
        iface_slot.sw(c, iface);
        force_off.sw(c, 0);
        pending.sw(c, 0);

        if c.g[S4][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = 0x0044_0000;
            loop {
                // Loop head 0x22d970: s4 = node, s6 = previous node.
                c.cycles += 11;
                c.g[S1][0] = Addr(r32(c, S4)).lw(c); // event index
                c.g[S3][0] = c.g[V0][0];
                c.g[S5][0] = 0; // remove
                c.g[V1][0] = sx(r32(c, S1) << 4);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x2168));
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = c.g[A0][0];
                let event = r32(c, A0);
                c.g[A0][0] = Addr(event.wrapping_add(8)).lw(c);
                c.g[V1][0] = (c.g[A0][0] < 9) as u64;
                c.g[S0][0] = Addr(event.wrapping_add(4)).lw(c); // world section
                if c.g[A0][0] < 9 {
                    c.cycles += 7;
                    let state = c.g[A0][0] as u32;
                    c.g[V0][0] = sx(STATE_TABLE);
                    c.g[V1][0] = sx((state << 2).wrapping_add(STATE_TABLE));
                    c.g[A0][0] = sx(STATE_CASES[state as usize]);
                    match state {
                        7 => {
                            // Unloading: done once the streamer dropped it.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_SECTION_UNLOADED, 0x0022_d9d0)?;
                            let unloaded = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                            if unloaded {
                                c.cycles += 4;
                                c.g[V1][0] = sx(r32(c, S1) << 4);
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                Addr(r32(c, V0).wrapping_add(8)).sw(c, 0);
                            } else {
                                c.cycles += 2;
                                c.g[S7][0] = 1;
                            }
                        }
                        5 => {
                            // Releasing: on success the state becomes 7 and
                            // the game module hears of the unload.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_RELEASE_SECTION, 0x0022_da00)?;
                            let released = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                            if released {
                                c.cycles += 7;
                                c.g[V1][0] = sx(r32(c, S1) << 4);
                                c.g[A0][0] = 7;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                Addr(r32(c, V0).wrapping_add(8)).sw(c, 7);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                                call_ptr(c, SU_ON_SECTION_UNLOAD, 0x0022_da24)?;
                            }
                            c.cycles += 2;
                            c.g[S7][0] = 1;
                        }
                        6 => {
                            // Loading: once resident the state becomes 1.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_SECTION_RESIDENT, 0x0022_da44)?;
                            let resident = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                            if resident {
                                c.cycles += 9;
                                c.g[V1][0] = sx(r32(c, S1) << 4);
                                c.g[A0][0] = 1;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                                Addr(r32(c, V0).wrapping_add(8)).sw(c, 1);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                                call_ptr(c, SU_INVALIDATE_FOCUS, 0x0022_da68)?;
                            } else {
                                c.cycles += 2;
                                c.g[FP][0] = 1;
                            }
                        }
                        8 => {
                            // Loading for an event switch: once resident the
                            // state becomes 2 and the section is activated.
                            c.cycles += 8;
                            section_man_update_cache_args(c);
                            call_ptr(c, SU_SECTION_RESIDENT, 0x0022_da90)?;
                            let resident = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r32(c, S1) << 4);
                            if resident {
                                c.cycles += 15;
                                c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x2168));
                                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                c.g[V1][0] = 2;
                                Addr(r32(c, A0).wrapping_add(8)).sw(c, 2);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                                call_ptr(c, SU_INVALIDATE_FOCUS, 0x0022_dab4)?;
                                c.g[V1][0] = section_man_update_this(c).off(GAME).lw(c);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lw(c);
                                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                                call_ptr(c, SU_ACTIVATE_SECTION, 0x0022_dacc)?;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 1;
                                c.g[FP][0] = 1;
                                force_off.sw(c, 1);
                            }
                        }
                        3 => {
                            // Waiting to be requested.
                            c.cycles += 3;
                            c.g[V1][0] = 1;
                            pending.sw(c, 1);
                        }
                        4 => {
                            // Waiting to be requested (event switch).
                            c.cycles += 4;
                            c.g[V0][0] = 1;
                            pending.sw(c, 1);
                            force_off.sw(c, 1);
                        }
                        1 => {
                            // Active: request its sky, retire the event.
                            c.cycles += 6;
                            c.g[A0][0] = iface_slot.lw(c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, SU_SECTION_FIELD10, 0x0022_db0c)?;
                            c.g[V1][0] = 4;
                            c.g[S5][0] = 1;
                            if c.g[V0][0] == 4 {
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S2][0];
                                SU_REQUEST_SKY.call(c, 0x0022_db24)?;
                                c.g[S5][0] = 1;
                            }
                        }
                        2 => {
                            // Active after a switch: request and apply its
                            // sky, retire the event.
                            c.cycles += 6;
                            c.g[A0][0] = iface_slot.lw(c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, SU_SECTION_FIELD10, 0x0022_db38)?;
                            c.g[V1][0] = 4;
                            if c.g[V0][0] == 4 {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S2][0];
                                SU_REQUEST_SKY.call(c, 0x0022_db50)?;
                            }
                            c.g[V0][0] = section_man_update_this(c).off(SKY_PENDING).lw(c);
                            c.cycles += 2;
                            c.g[S5][0] = 1;
                            if c.g[V0][0] != 0 {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S2][0];
                                SU_APPLY_SKY.call(c, 0x0022_db64)?;
                                c.cycles += 1;
                                c.g[S5][0] = 1;
                            }
                        }
                        _ => {
                            // 0: finished, retire the event.
                            c.cycles += 1;
                            c.g[S5][0] = 1;
                        }
                    }
                }

                // 0x22db68: dequeue a retired node (the walk continues from
                // the previous one).
                c.cycles += 2;
                c.g[A2][0] = c.g[S4][0];
                if c.g[S5][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[S4][0] = c.g[S6][0];
                    SU_DEQUEUE.call(c, 0x0022_db80)?;
                }
                c.cycles += 7;
                c.g[S6][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S6][0];
                SU_NEXT_QUEUED.call(c, 0x0022_db90)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[V0][0] = 0x0044_0000;
                if c.g[S4][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_d970);
                }
            }
        }

        // 0x22db9c: request the pending loads once nothing is busy.
        c.cycles += 2;
        c.g[V1][0] = pending.lw(c);
        let mut request = c.g[S7][0] == 0;
        if request {
            c.cycles += 2;
            request = c.g[V1][0] != 0;
        }
        if request {
            c.cycles += 12;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            SU_NEXT_QUEUED.call(c, 0x0022_dbb8)?;
            c.g[V1][0] = section_man_update_this(c).off(GAME).lw(c);
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            call_ptr(c, SU_SECTION_COUNT, 0x0022_dbd0)?;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(PREFIX_SINGLE);
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx(PREFIX_MULTI);
            }
            c.cycles += 2;
            let prefix = c.g[V0][0];
            prefix_slot.sw(c, prefix);
            if c.g[S4][0] != 0 {
                c.cycles += 7;
                c.g[V0][0] = 0x0044_0000;
                c.g[V1][0] = 0x0045_0000;
                c.g[S7][0] = sx(EVENTS);
                c.g[S6][0] = sx(SKY_PRIORITY);
                c.g[S5][0] = sx(EVENTS + 0xc);
                c.g[A1][0] = Addr(r32(c, S4)).lw(c);
                loop {
                    // Loop head 0x22dc10: s4 = node, a1 = its event index.
                    c.cycles += 19;
                    c.g[V1][0] = sx(EVENTS + 4);
                    c.g[A0][0] = iface_slot.lw(c);
                    c.g[S0][0] = sx(r32(c, A1) << 4);
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                    c.g[S3][0] = Addr(r32(c, V0)).lw(c); // world section
                    call_ptr(c, SU_SECTION_ENTRY, 0x0022_dc2c)?;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A2][0] = prefix_slot.lw(c);
                    c.g[V0][0] = 0x004a_0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx(PATH_FORMAT);
                    call_ptr(c, SU_SPRINTF, 0x0022_dc44)?;
                    c.g[V1][0] = sx(r32(c, S7).wrapping_add(8));
                    c.g[V0][0] = 3;
                    c.g[S1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = Addr(r32(c, S1)).lw(c); // state
                    c.g[V0][0] = 4;
                    let state = c.g[V1][0];
                    let mut next_a1_set = false;
                    if state == 3 || state == 4 {
                        if state == 4 {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S4][0];
                        }
                        let ret = if state == 3 { 0x0022_dc88 } else { 0x0022_dcd0 };
                        c.cycles += 13;
                        c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                        c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c); // sky index
                        c.g[A1][0] = c.g[S3][0];
                        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
                        c.lwc1(12, r32(c, V0));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                        call_ptr(c, SU_REQUEST_SECTION, ret)?;
                        let requested = c.g[V0][0] != 0;
                        if state == 3 {
                            c.g[V0][0] = 6;
                            if requested {
                                c.cycles += 3;
                                c.g[FP][0] = 1;
                                Addr(r32(c, S1)).sw(c, 6);
                            }
                        } else {
                            c.g[V0][0] = 8;
                            if requested {
                                c.cycles += 4;
                                c.g[V1][0] = 1;
                                Addr(r32(c, S1)).sw(c, 8);
                                c.g[FP][0] = 1;
                                force_off.sw(c, 1);
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S4][0];
                        next_a1_set = true;
                    }
                    if !next_a1_set {
                        // Neither 3 nor 4 (0x22dc9c: the delay slot sets a1).
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S4][0];
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    SU_NEXT_QUEUED.call(c, 0x0022_dcf4)?;
                    c.g[S4][0] = c.g[V0][0];
                    if c.g[S4][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = Addr(r32(c, S4)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_dc10);
                    }
                }
            }
        }

        // 0x22dd00: the stream-rate class.
        c.cycles += 2;
        c.g[V0][0] = 2;
        if c.g[FP][0] != 0 {
            c.cycles += 2;
            section_man_update_this(c).off(RATE).sw(c, 2);
        } else {
            c.cycles += 3;
            c.g[V0][0] = section_man_update_this(c).off(STREAM_ON).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 3;
                section_man_update_this(c).off(RATE).sw(c, 3);
            } else {
                c.g[V0][0] = section_man_update_this(c).off(GAME).lw(c);
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x10)).lw(c);
                c.g[V1][0] = Addr(r32(c, V1)).lw(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WV_ACTIVE)).lw(c);
                if c.g[V0][0] == 0 {
                    section_man_update_this(c).off(RATE).sw(c, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(WV_VISIBLE)).lw(c);
                    let visible = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if visible {
                        c.cycles += 2;
                        section_man_update_this(c).off(RATE).sw(c, 1);
                    } else {
                        c.cycles += 1;
                        section_man_update_this(c).off(RATE).sw(c, 0);
                    }
                }
            }
        }

        // 0x22dd54: the event-switch state machine (s0 = hold).
        c.cycles += 4;
        c.g[V1][0] = section_man_update_this(c).off(SWITCH).lw(c);
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        c.g[S0][0] = 0;
        if c.g[V1][0] < 5 {
            c.cycles += 7;
            let sw = c.g[V1][0] as u32;
            c.g[V0][0] = sx(SWITCH_TABLE);
            c.g[V1][0] = sx((sw << 2).wrapping_add(SWITCH_TABLE));
            c.g[A0][0] = sx(SWITCH_CASES[sw as usize]);
            match sw {
                1 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    SU_BEGIN_SWITCH.call(c, 0x0022_dd88)?;
                    c.g[V0][0] = 2;
                    section_man_update_this(c).off(SWITCH).sw(c, 2);
                }
                3 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S0][0] = 1;
                    SU_BEGIN_SWITCH.call(c, 0x0022_dda0)?;
                    c.g[V0][0] = 4;
                    section_man_update_this(c).off(SWITCH).sw(c, 4);
                }
                4 => {
                    c.cycles += 3;
                    c.g[V0][0] = force_off.lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = 1;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        SU_OVERLAY_DONE.call(c, 0x0022_ddc0)?;
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = 1;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            SU_FINISH_SWITCH.call(c, 0x0022_ddd0)?;
                            section_man_update_this(c).off(SWITCH).sw(c, 0);
                            section_man_update_this(c).off(SWITCH_AUX).sw(c, 0);
                        }
                    }
                }
                _ => {}
            }
        }

        // 0x22ddd8: force the slow rate while switching or loading.
        c.cycles += 3;
        c.g[V1][0] = force_off.lw(c);
        c.g[V0][0] = 4;
        let mut hold = c.g[V1][0] != 0;
        if !hold {
            c.cycles += 2;
            hold = c.g[S0][0] != 0;
        }
        if hold {
            c.cycles += 3;
            section_man_update_this(c).off(READY).sw(c, 0);
            section_man_update_this(c).off(RATE).sw(c, 4);
        } else {
            c.g[V0][0] = 1;
            c.cycles += 1;
            section_man_update_this(c).off(READY).sw(c, 1);
        }

        // 0x22ddfc: cStreamMan_SetRate(view + 0x300, rates[class]).
        c.cycles += 12;
        c.g[A0][0] = section_man_update_this(c).off(GAME).lw(c);
        c.g[V1][0] = 0x0044_0000;
        c.g[V0][0] = section_man_update_this(c).off(RATE).lw(c);
        c.g[V1][0] = sx(RATES);
        c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(WV_STREAM_MAN));
        call_ptr(c, SU_SET_RATE, 0x0022_de28)?;
        frame.restore(c, S0, 0x1a0);
        section_man_update_epilogue(c, frame);
        Ok(())
    })
}

/// 0x22de2c: restore s1..s7, fp and ra (s0 is already restored), return.
fn section_man_update_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0x190);
    frame.restore(c, S2, 0x180);
    frame.restore(c, S3, 0x170);
    frame.restore(c, S4, 0x160);
    frame.restore(c, S5, 0x150);
    frame.restore(c, S6, 0x140);
    frame.restore(c, S7, 0x130);
    frame.restore(c, FP, 0x120);
    frame.restore_ra(c, 0x110);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{covered, fake_call, fall_back, hook, machine};
    use crate::difftest::{elf_image, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (other tests run the real bodies).
    const SECTION_UPDATE_CTRL: u32 = DATA + 0xf_f000;
    const SECTION_UPDATE_MAGIC: u32 = 0x5ec7_d8d8;

    macro_rules! section_update_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(SECTION_UPDATE_CTRL) != SECTION_UPDATE_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                fake_call(c, $addr);
            }
        };
    }

    section_update_gated!(
        section_update_fake_on_unload,
        SECTION_UPDATE_PREV_ON_UNLOAD,
        0x0023_0360,
        ee::s06::f_00230360__generated
    );
    section_update_gated!(
        section_update_fake_activate,
        SECTION_UPDATE_PREV_ACTIVATE,
        0x003a_99d8,
        ee::s13::f_003a99d8__generated
    );
    section_update_gated!(
        section_update_fake_request_sky,
        SECTION_UPDATE_PREV_REQUEST_SKY,
        0x0022_de58,
        ee::s05::f_0022de58__generated
    );
    section_update_gated!(
        section_update_fake_begin_switch,
        SECTION_UPDATE_PREV_BEGIN_SWITCH,
        0x0022_df50,
        ee::s05::f_0022df50__generated
    );
    section_update_gated!(
        section_update_fake_finish_switch,
        SECTION_UPDATE_PREV_FINISH_SWITCH,
        0x0022_dff0,
        ee::s05::f_0022dff0__generated
    );
    section_update_gated!(
        section_update_fake_sprintf,
        SECTION_UPDATE_PREV_SPRINTF,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    fn section_update_install_fakes() {
        use std::ptr::addr_of_mut;
        // SAFETY: override slots are plain fn pointers; `hook` serialises.
        {
            hook(
                addr_of_mut!(ee::s06::OVERRIDE_00230360),
                section_update_fake_on_unload,
                &SECTION_UPDATE_PREV_ON_UNLOAD,
            );
            hook(
                addr_of_mut!(ee::s13::OVERRIDE_003A99D8),
                section_update_fake_activate,
                &SECTION_UPDATE_PREV_ACTIVATE,
            );
            hook(
                addr_of_mut!(ee::s05::OVERRIDE_0022DE58),
                section_update_fake_request_sky,
                &SECTION_UPDATE_PREV_REQUEST_SKY,
            );
            hook(
                addr_of_mut!(ee::s05::OVERRIDE_0022DF50),
                section_update_fake_begin_switch,
                &SECTION_UPDATE_PREV_BEGIN_SWITCH,
            );
            hook(
                addr_of_mut!(ee::s05::OVERRIDE_0022DFF0),
                section_update_fake_finish_switch,
                &SECTION_UPDATE_PREV_FINISH_SWITCH,
            );
            hook(
                addr_of_mut!(ee::s15::OVERRIDE_00417828),
                section_update_fake_sprintf,
                &SECTION_UPDATE_PREV_SPRINTF,
            );
        }
    }

    /// SectionMan_Update on a queue of 0..6 section events in random states
    /// (0..=8 and out of range) over a real world cache (streamer states,
    /// counts and the `.rodata` jump tables from the ELF image), the real
    /// BE getters, list helpers, sky apply, overlay check and stream rate;
    /// fakes (that may unwind) for the module hooks, the activation, the
    /// sky request, the event-switch begin/finish and sprintf. Covers the
    /// empty-queue exit, busy/pending/changed/force-off combinations, every
    /// switch state and every rate class.
    #[test]
    fn section_man_update_matches() {
        use section_man_update_layout::*;
        section_update_install_fakes();
        covered(
            ee::s05::f_0022d8d8__generated,
            section_man_update,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0022_d8d8, 8);
                c.bus.write32(SECTION_UPDATE_CTRL, SECTION_UPDATE_MAGIC);
                c.pc = 0x0022_d8d8;
                let gp = GAME_GP;

                let this = DATA;
                let game = DATA + 0x2000;
                let holder = DATA + 0x2400;
                let view = DATA + 0x3000;
                let cache = view + 0x10;
                let streamer = DATA + 0x5000;
                c.g[A0][0] = sx(this);

                // BE: the library instance and interface 0 (a fake method).
                c.bus.write32(gp.wrapping_sub(0x1f04), DATA + 0x6000);
                c.bus.write32(gp.wrapping_sub(0x1efc), 0);
                let be_obj = DATA + 0x6100;
                let be_vt = DATA + 0x6200;
                c.bus.write32(0x0053_08a8, be_obj);
                c.bus.write32(be_obj + 4, be_vt);
                c.bus.write16(be_vt + 8, 0);
                c.bus.write32(be_vt + 0xc, crate::difftest::fakes::FAKE);
                // cStreamMan_SetRate's global.
                c.bus.write32(gp.wrapping_add(0x2a74), DATA + 0x6400);
                c.bus.write32(DATA + 0x6410, 1 + r.next_u32() % 0x8000);

                // The manager.
                let sky_pending = (r.next_u32() % 3 == 0) as u32;
                c.bus.write32(this + 0x1a4, 0);
                c.bus.write32(this + SKY_PENDING, sky_pending);
                c.bus.write32(this + 0x1ac, r.next_u32() % 50);
                c.bus.write32(this + GAME, game);
                let switch = match seed % 8 {
                    0..=4 => seed % 8,
                    5 => 4,
                    6 => 0,
                    _ => 5 + r.next_u32() % 3,
                };
                c.bus.write32(this + SWITCH, switch);
                c.bus.write32(this + SWITCH_AUX, r.next_u32());
                c.bus
                    .write32(this + STREAM_ON, (r.next_u32() % 3 != 0) as u32);
                c.bus.write32(this + RATE, r.next_u32());
                c.bus.write32(this + READY, r.next_u32());
                c.bus.write32(this + 0x198, DATA + 0x1800);

                // Game module, world view and its streamer.
                c.bus.write32(
                    game + 0xc,
                    if r.next_u32() & 1 == 0 {
                        0
                    } else {
                        DATA + 0x2100
                    },
                );
                c.bus.write32(
                    DATA + 0x2100 + 0xa4,
                    if r.next_u32() & 1 == 0 {
                        0
                    } else {
                        DATA + 0x2200
                    },
                );
                c.bus.write32(game + 0x10, holder);
                c.bus.write32(game + 0x204, 0);
                c.bus.write32(game + 0x208, r.next_u32() & 1);
                c.bus.write32(holder, view);
                c.bus.write32(view + WV_ACTIVE, r.next_u32() & 1);
                c.bus.write32(view + WV_VISIBLE, r.next_u32() & 1);
                let sections = 1 + r.next_u32() % 12;
                c.bus
                    .write32(cache + 4, if r.next_u32() % 8 == 0 { 0 } else { streamer });
                c.bus.write32(streamer + 8, sections);
                for s in 0..16 {
                    c.bus.write32(cache + 0x14 + 8 * s, r.next_u32() % 8);
                }

                // The queue: distinct events in random states.
                let n = match seed % 5 {
                    0 => 0,
                    _ => 1 + r.next_u32() % 6,
                };
                let mut used = Vec::new();
                let mut prev = 0u32;
                for i in 0..n {
                    let mut idx = r.next_u32() % 50;
                    while used.contains(&idx) {
                        idx = (idx + 1) % 50;
                    }
                    used.push(idx);
                    let node = DATA + 0x1000 + 8 * i;
                    c.bus.write32(node, idx);
                    c.bus.write32(node + 4, 0);
                    if prev == 0 {
                        c.bus.write32(this + 0x19c, node);
                    } else {
                        c.bus.write32(prev + 4, node);
                    }
                    prev = node;
                    let event = EVENTS + 16 * idx;
                    c.bus.write32(event + 4, r.next_u32() % (sections + 3));
                    let state = match r.next_u32() % 12 {
                        9 => 9 + r.next_u32() % 4,
                        10 => 3,
                        11 => 4,
                        s => s,
                    };
                    c.bus.write32(event + 8, state);
                    c.bus.write32(event + 0xc, r.next_u32() % 8);
                    let field10 = if r.next_u32() & 1 == 0 {
                        4
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(0x0043_e264 + 0x18 * idx, field10);
                }
                if n == 0 {
                    c.bus.write32(this + 0x19c, 0);
                }
                c.bus.write32(this + 0x1a0, n);
            },
        );
    }
}
