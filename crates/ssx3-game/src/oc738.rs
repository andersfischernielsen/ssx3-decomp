//! Staging module oc738 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc738`.

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
    (
        0x0028_a298,
        snd_load_queue_start_next,
        "SndLoadQueue_StartNext",
    ),
    (
        0x0027_a860,
        script_engine_queue_finish_scripts,
        "ScriptEngine_QueueFinishScripts",
    ),
    (
        0x0023_6960,
        game_state_event_switch_load_update,
        "GameStateEventSwitchLoad_Update",
    ),
    (
        0x0022_0ad0,
        hud_draw_option_icon_text,
        "HUD_DrawOptionIconText",
    ),
    (
        0x0020_0b50,
        event_select_is_event_completed,
        "EventSelect_IsEventCompleted",
    ),
];

/// The gcc block copy of a by-value struct: 0x20 bytes per round from the
/// address in register `src` to the one in `dst` until `src` reaches the end
/// address in `lim`. Four 8-byte chunks per round go through `regs`, with
/// unaligned `ldl/ldr` + `sdl/sdr` pairs (19 cycles a round) when the source
/// is not 8-byte aligned, plain `ld`/`sd` (11 cycles) otherwise. `head`
/// is the loop head the back-edge event check reports. Leaves `src`/`dst`
/// advanced; the caller charges the exit cycles.
fn oc738_copy_blocks(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    lim: usize,
    aligned: bool,
    regs: [usize; 4],
    head: u32,
) {
    loop {
        let from = c.g[src][0] as u32;
        let to = c.g[dst][0] as u32;
        if aligned {
            c.cycles += 11;
            for (k, &r) in regs.iter().enumerate() {
                c.g[r][0] = c.ld(from.wrapping_add(8 * k as u32));
            }
            for (k, &r) in regs.iter().enumerate() {
                c.sd(to.wrapping_add(8 * k as u32), c.g[r][0]);
            }
        } else {
            c.cycles += 19;
            for (k, &r) in regs.iter().enumerate() {
                c.ldl(r, from.wrapping_add(8 * k as u32 + 7));
                c.ldr(r, from.wrapping_add(8 * k as u32));
            }
            for (k, &r) in regs.iter().enumerate() {
                c.sdl(r, to.wrapping_add(8 * k as u32 + 7));
                c.sdr(r, to.wrapping_add(8 * k as u32));
            }
        }
        c.g[src][0] = sx(from.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[lim][0];
        c.g[dst][0] = sx(to.wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `int EventSelect_IsEventCompleted(void* self, sPeakGoal goal, int peak)`
/// (0x200b50), the C++ `cUITemplate_MAP_isMapEventCompleted`. `goal` is a
/// 0x6c-byte struct passed by value (`location` at +0, `gameEvent` at +8).
/// Location 23 is never completed; event types 4 and 5 (the peak-1/2/3 mission
/// events) are completed when the player interface says the event for that
/// peak is not locked; every other event is completed when the character has a
/// track medal for it.
pub fn event_select_is_event_completed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0xb0);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[V0][0] = c.g[A1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        frame.save_ra(c, 0x70);
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
        if aligned {
            oc738_copy_blocks(c, V1, A0, V0, true, [A1, A2, A3, T0], 0x0020_0bd4);
        } else {
            oc738_copy_blocks(c, V1, A0, V0, false, [A1, A2, A3, T0], 0x0020_0b7c);
            c.cycles += 2;
        }

        // The trailing 12 bytes of the struct.
        c.cycles += 23;
        let src = c.g[V1][0] as u32;
        c.ldl(2, src.wrapping_add(0x7));
        c.ldr(2, src);
        c.g[A1][0] = Addr(src.wrapping_add(0x8)).lw(c);
        let dst = c.g[A0][0] as u32;
        c.sdl(2, dst.wrapping_add(0x7));
        c.sdr(2, dst);
        c.sw(dst.wrapping_add(0x8), c.g[A1][0]);
        call_ptr(c, 0x0014_dc80, 0x0020_0c1c)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014_dd58, 0x0020_0c28)?; // BELibrary_GetInterface(be, 1)
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0020_0c30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0020_0c3c)?; // BELibrary_GetInterface(be, 13)
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x0020_0c48)?; // BELibrary_GetCharacterID(0)
        c.g[A2][0] = c.g[V0][0];
        c.g[T0][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 0x17;
        let not_summit = c.g[T0][0] != c.g[V0][0];
        c.g[A3][0] = Addr(sp(c).wrapping_add(0x8)).lw(c);
        if !not_summit {
            c.cycles += 2;
            c.g[V0][0] = 0;
            oc738_event_completed_exit(c, frame);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = 0x4;
        let not_four = c.g[A3][0] != c.g[V0][0];
        c.g[V0][0] = 0x5;
        if !not_four {
            c.cycles += 7;
            c.g[V1][0] = c.g[S1][0] ^ 0x1;
            c.g[A0][0] = 0x7;
            c.g[S0][0] = 0x6;
            c.g[A1][0] = c.g[S1][0] ^ 0x2;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            c.g[V0][0] = 0x8;
        } else {
            c.cycles += 2;
            let is_five = c.g[A3][0] == c.g[V0][0];
            c.g[V1][0] = c.g[S1][0] ^ 0x1;
            if !is_five {
                // Any other event type: a track medal exists.
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0015_8910, 0x0020_0ca0)?; // getTrackMedal(rw, 0, cid, event, loc)
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                oc738_event_completed_exit(c, frame);
                return Ok(());
            }
            c.cycles += 5;
            c.g[A0][0] = 0xa;
            c.g[S0][0] = 0x9;
            c.g[A1][0] = c.g[S1][0] ^ 0x2;
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[A0][0];
            }
            c.g[V0][0] = 0xb;
        }

        // Event types 4 and 5: the event index for the peak is in S0.
        c.cycles += 9;
        if c.g[A1][0] == 0 {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x0020_0ccc)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_6a70, 0x0020_0ce0)?; // isEventLocked_b(np, 0, cid, idx)
        c.g[V0][0] ^= 0x1;
        oc738_event_completed_exit(c, frame);
        Ok(())
    });
}

/// Epilogue of `HUD_DrawOptionIconText`.
fn oc738_option_icon_text_exit(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x340);
    frame.restore(c, S1, 0x330);
    frame.restore(c, S2, 0x320);
    frame.restore(c, S3, 0x310);
    frame.restore(c, S4, 0x300);
    frame.restore(c, S5, 0x2f0);
    frame.restore_ra(c, 0x2e0);
    frame.restore_fpr(c, 20, 0x350);
    frame.close(c);
}

/// `void HUD_DrawOptionIconText(HudObj* self, HudData* data, float progress)`
/// (0x220ad0), the C++ `cOVStateHUD1P_renderNISSkipMessage`: draws the
/// "skip" message of a cutscene, fading it in and out with `progress`
/// (`f12`, 0..1). The element's position and scale come from the HUD
/// definition at `data + 0xad4`; while a script plays and the option
/// interface (a 0x288-byte struct copied to the stack) says so, the text
/// shrinks to 0.75 and moves up 60 units. The colour alpha follows a tent
/// of `progress`; the text goes through `HUD_DrawIconText` with the shadow
/// offset set to (2, 2) and the draw state's bit group set from the data.
pub fn hud_draw_option_icon_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x360);
        frame.save(c, S2, 0x320);
        frame.save(c, S0, 0x340);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x330);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xad4));
        frame.save_fpr(c, 20, 0x350);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x2f0);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S3, 0x310);
        c.mov_s(20, 12);
        frame.save(c, S4, 0x300);
        frame.save_ra(c, 0x2e0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001e_91f8, 0x0022_0b10)?; // HUD_GetElemPos(elem, &pos)
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[S5][0] = c.g[A1][0];
        call_ptr(c, 0x001e_9290, 0x0022_0b20)?; // HUD_GetElemScale(elem, &scale)
        call_ptr(c, 0x0014_dc80, 0x0022_0b28)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x0014_dd58, 0x0022_0b34)?; // BELibrary_GetInterface(be, 4)

        // A copy of the option interface (0x535610, 0x280 bytes) on the stack.
        c.g[V1][0] = 0x0053_0000;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
        c.g[V0][0] = c.g[A1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
        c.cycles += 5;
        c.g[A2][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x280));
        c.g[S0][0] = sx(sp(c).wrapping_add(0x2b0));
        c.g[S4][0] = sx(sp(c).wrapping_add(0x2d0));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb48));
        if aligned {
            oc738_copy_blocks(c, A1, A0, V0, true, [T2, V1, A3, T0], 0x0022_0bc4);
            c.cycles += 1;
        } else {
            oc738_copy_blocks(c, A1, A0, V0, false, [V1, A3, T0, T1], 0x0022_0b5c);
            c.cycles += 2;
        }
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);

        // The trailing 8 bytes, then the "script playing" adjustment.
        c.cycles += 7;
        let src = c.g[A1][0] as u32;
        let dst = c.g[A0][0] as u32;
        c.ldl(T1, src.wrapping_add(0x7));
        c.ldr(T1, src);
        c.sdl(T1, dst.wrapping_add(0x7));
        c.sdr(T1, dst);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x504)).lw(c);
        let not_playing = c.g[V0][0] == 0;
        c.g[V0][0] = Addr(sp(c).wrapping_add(0x20)).lw(c);
        if !not_playing {
            c.cycles += 5;
            c.g[V1][0] = 0x0030_0000;
            c.g[A0][0] = 0x0020_0000;
            c.g[V0][0] &= c.g[V1][0];
            let mode2 = c.g[V0][0] == c.g[A0][0];
            c.lwc1(0, sp(c).wrapping_add(0x14));
            if !mode2 {
                c.cycles += 9;
                c.lwc1(1, sp(c).wrapping_add(0x4));
                c.g[AT][0] = 0x3f40_0000;
                c.mtc1(1, 2);
                c.g[AT][0] = 0x4270_0000;
                c.mtc1(1, 3);
                c.mul_s(0, 0, 2);
                c.sub_s(1, 1, 3);
                c.swc1(0, sp(c).wrapping_add(0x14));
                c.swc1(1, sp(c).wrapping_add(0x4));
            }
        }

        // The colour, its alpha scaled by a tent of the progress.
        c.cycles += 9;
        c.g[A0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x001e_91a8, 0x0022_0c54)?; // HUD_GetElemColor(elem, &colour)
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 0);
        c.c_le_s(20, 0);
        let rising = c.fcc();
        c.lwc1(0, sp(c).wrapping_add(0x2b0));
        if rising {
            c.cycles += 4;
            c.add_s(1, 20, 20);
            c.mul_s(0, 0, 1);
            c.swc1(0, sp(c).wrapping_add(0x2b0));
        } else {
            c.cycles += 7;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.lwc1(1, sp(c).wrapping_add(0x2b0));
            c.sub_s(0, 0, 20);
            c.add_s(0, 0, 0);
            c.mul_s(1, 1, 0);
            c.swc1(1, sp(c).wrapping_add(0x2b0));
        }

        // Shadow offset (2, 2), the draw state's bit group, the draw call.
        c.cycles += 40;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 0);
        c.g[T1][0] = 0xffff_ffff_ffff_fc1f;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xb50)).lw(c);
        c.g[A0][0] = c.g[S3][0];
        c.swc1(0, sp(c).wrapping_add(0x2c4));
        c.g[A1][0] = c.g[S4][0];
        c.swc1(0, sp(c).wrapping_add(0x2c0));
        c.g[A2][0] = c.g[S5][0];
        c.ldl(8, sp(c).wrapping_add(0x2c7));
        c.ldr(8, sp(c).wrapping_add(0x2c0));
        let font = c.g[V0][0] as u32;
        c.sdl(8, font.wrapping_add(0x2f));
        c.sdr(8, font.wrapping_add(0x28));
        c.g[A3][0] = c.g[S0][0];
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x854)).lw(c);
        c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xaf6)).lb(c);
        c.g[T0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe84)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
        c.g[V0][0] = Addr((c.g[T0][0] as u32).wrapping_add(0x8)).lw(c);
        c.g[V1][0] &= 0x3e0;
        c.g[V0][0] &= c.g[T1][0];
        c.g[V0][0] |= c.g[V1][0];
        let state = c.g[T0][0] as u32;
        c.sw(state.wrapping_add(0x8), c.g[V0][0]);
        c.ldl(8, sp(c).wrapping_add(0x7));
        c.ldr(8, sp(c));
        c.sdl(8, sp(c).wrapping_add(0x2d7));
        c.sdr(8, sp(c).wrapping_add(0x2d0));
        c.g[T1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xaf5)).lb(c);
        c.g[T0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xaf4)).lb(c);
        call_ptr(c, 0x001e_95a0, 0x0022_0d10)?; // HUD_DrawIconText
        oc738_option_icon_text_exit(c, frame);
        Ok(())
    });
}

/// `void GameStateEventSwitchLoad_Update(GameState* self)` (0x236960), the C++
/// `cGFSectionLoadState_update`. While the section manager waits for the
/// transport (`mWaitTransport`, +0x1c8) it steps `self->state` (+0x10):
/// state 0 waits for the section to be loaded, then puts the world-view
/// stream focus (slot `idx`, the one the player camera does not use) at the
/// NIS end position (radius 45000) and moves to state 1; state 1 waits until
/// the focus is closer than 20000 and the transition manager, the NIS
/// transition and the script engine are idle, then ends the transport
/// sequence, activates the section (7) and moves to state 2.
pub fn game_state_event_switch_load_update(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
        frame.save(c, S4, 0x30);
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[S2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x78)).lw(c);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x1c8)).lw(c);
        let not_waiting = c.g[V0][0] == 0;
        c.g[S3][0] = 0x1;
        'exit: {
            'done: {
                if not_waiting {
                    break 'done;
                }
                'state1: {
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[S1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xb0)).lw(c);
                    let is_state1 = c.g[A1][0] == c.g[S3][0];
                    c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    if is_state1 {
                        break 'state1;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x70);
                        break 'exit;
                    }
                    c.cycles += 2;
                    if c.g[A1][0] != 0 {
                        frame.restore(c, S0, 0x70);
                        break 'exit;
                    }

                    // State 0: wait for the section to be loaded.
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1bc)).lw(c);
                    call_ptr(c, 0x0022_d278, 0x0023_69d4)?; // cSectionMan_isLoaded
                    let not_loaded = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    if not_loaded {
                        break 'done;
                    }
                    c.cycles += 19;
                    c.g[A1][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
                    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).lw(c);
                    call_ptr(c, 0x0012_3f38, 0x0023_69f4)?; // cRider_GetNISEndPosition
                    c.g[V0][0] = 0x50;
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.mult(0, 2, 16, 2);
                    c.lwc1(12, gp.wrapping_sub(0x5168));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let slot = c.g[V1][0] as u32;
                    c.sw(slot.wrapping_add(0x250), c.g[S3][0]);
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003a_9658, 0x0023_6a20)?; // cWorldView_setFocus
                    let state = c.g[S4][0] as u32;
                    c.sw(state.wrapping_add(0x10), c.g[S3][0]);
                    break 'done;
                }

                // State 1: wait for the focus to arrive and everything to idle.
                c.cycles += 12;
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = sx(sp(c).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_9770, 0x0023_6a40)?; // cWorldView_getTestFocus
                c.lwc1(1, sp(c).wrapping_add(0x10));
                c.lwc1(0, gp.wrapping_sub(0x5164));
                c.c_lt_s(1, 0);
                let close = c.fcc();
                frame.restore(c, S0, 0x70);
                if close {
                    break 'exit;
                }
                c.cycles += 13;
                c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                call_ptr(c, 0x0023_0698, 0x0023_6a68)?; // GameModule_GetTransitionMan
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x44)).lw(c);
                let is_busy = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, is_busy, 0x0023_6a84)?; // isBusy()
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x70);
                    break 'exit;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x44)).lw(c);
                let playing = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x70);
                if playing {
                    break 'exit;
                }
                c.cycles += 4;
                c.g[A0][0] = Addr(gp.wrapping_sub(0x84c)).lw(c);
                call_ptr(c, 0x0027_9298, 0x0023_6aa0)?; // IsActiveNISInTransition
                let nis = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x70);
                if nis {
                    break 'exit;
                }
                c.cycles += 9;
                c.g[A0][0] = Addr(gp.wrapping_sub(0x84c)).lw(c);
                call_ptr(c, 0x0027_a9f0, 0x0023_6ab0)?; // EndTransportSequence
                let man = c.g[S2][0] as u32;
                c.sw(man.wrapping_add(0x1c8), 0);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = 0x7;
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1bc)).lw(c);
                call_ptr(c, 0x0022_d088, 0x0023_6ac4)?; // cSectionMan_activate
                c.g[V0][0] = 0x2;
                let state = c.g[S4][0] as u32;
                c.sw(state.wrapping_add(0x10), c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_BeginPlayback(Replay* self, int frame, int instant, int auto)`
/// (0x26f8a0), the C++ `cReplay_startReplay`. Unless the replay is already
/// in a playback state (14 or 15): saves the resume snapshot when none is
/// loaded, enters state 7, clears the command and auto-replay words, stores
/// `instant` (+0x620), seeks to `frame`, steps one frame and sets up the
/// instant-replay or the replay cameras; a full replay that is not an auto
/// replay also tells the audio system the replay begins.
#[cfg(test)] // duplicate of oc729/oc730, kept for its test only
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        c.g[V0][0] ^= 0x1;
        let can_begin = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        // Where the epilogue chain is entered: 0 runs it all (`+1, lq s0`,
        // `+1, lq s1`, the rest), 1 skips the first step, 2 the first two.
        let mut entry = 0;
        'done: {
            if can_begin {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x608)).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?; // Replay_SaveResumeSnapshot
                    c.g[A0][0] = c.g[S0][0];
                }
                c.cycles += 13;
                c.g[A1][0] = 0x7;
                call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?; // Replay_SetState(self, 7)
                let this = c.g[S0][0] as u32;
                c.sw(this.wrapping_add(0x618), 0);
                c.g[A1][0] = c.g[S2][0];
                c.sw(this.wrapping_add(0x61c), 0);
                c.g[A0][0] = c.g[S0][0];
                c.sw(this.wrapping_add(0x620), c.g[S1][0]);
                call_ptr(c, 0x0027_02f8, 0x0026_f914)?; // Replay_SeekToSnapshot
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0027_03f0, 0x0026_f920)?; // Replay_StepFrame(self, 1)
                if c.g[S1][0] == 0 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_ee68, 0x0026_f940)?; // Replay_EnableReplayCameras
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_ef30, 0x0026_f930)?; // Replay_EnableInstantReplayCameras
                }
                c.cycles += 2;
                let instant = c.g[S1][0] != 0;
                frame.restore(c, S0, 0x40);
                if instant {
                    entry = 1;
                    break 'done;
                }
                c.cycles += 2;
                let auto = c.g[S3][0] != 0;
                frame.restore(c, S1, 0x30);
                if auto {
                    entry = 2;
                    break 'done;
                }
                c.cycles += 4;
                call_ptr(c, 0x0028_b180, 0x0026_f958)?; // SSXAudioSystem_GetInstance
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0028_f200, 0x0026_f960)?; // SSXAudioSystem_OnReplayBegin
            }
        }
        if entry == 0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        if entry <= 1 {
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// One `ScriptEngine_ListAddChoice(self = a0, 1 = a1, a2, a3, t0, -1, -1, -1)`
/// call (a0 and a1 are set by the caller), returning to `ret`.
fn oc738_list_add_choice(c: &mut Cpu, a2: u64, a3: u64, t0: u64, ret: u32) -> Flow {
    c.g[A2][0] = a2;
    c.g[A3][0] = a3;
    c.g[T0][0] = t0;
    c.g[T1][0] = u64::MAX;
    c.g[T2][0] = u64::MAX;
    c.g[T3][0] = u64::MAX;
    call_ptr(c, 0x0027_8e50, ret)
}

/// `void ScriptEngine_QueueFinishScripts(ScriptEngine* self, int online, int
/// intro)` (0x27a860), the C++ `cSSXScriptEngine_PlayTransportSequence`:
/// queues the finish-line NIS scripts as a list (an optional intro, the
/// finish script, and the world-mode dependent final script; offline also
/// sets the graphics manager's skip-page-flip flag), prepares and starts the
/// list, and sends the music event 20.
pub fn script_engine_queue_finish_scripts(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let online = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        // `world->+0x84->+0xc->+0x7c == 1` (the world mode).
        let world_mode_is_one = |c: &mut Cpu| -> bool {
            c.g[A1][0] = 0x1;
            let w = c.g[A0][0] as u32;
            c.g[V0][0] = Addr(w.wrapping_add(0x84)).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
            let one = c.g[A0][0] == c.g[A1][0];
            c.g[A0][0] = c.g[S0][0];
            one
        };
        'queued: {
            if online {
                c.cycles += 2;
                let intro = c.g[A2][0] != 0;
                c.g[A1][0] = 0x1;
                if intro {
                    c.cycles += 7;
                    oc738_list_add_choice(c, 0x12, 1, 1, 0x0027_a898)?;
                }
                c.cycles += 16;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                oc738_list_add_choice(c, 0x13, 0, 0, 0x0027_a8bc)?;
                c.g[A0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
                let one = world_mode_is_one(c);
                c.cycles += 4;
                c.g[A2][0] = if one { 0x14 } else { 0x15 };
                c.g[A3][0] = 0x8;
                c.g[T0][0] = 0;
                // The last finish script (A2, A3, T0 as set above).
                c.cycles += 6;
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = u64::MAX;
                c.g[T3][0] = u64::MAX;
                call_ptr(c, 0x0027_8e50, 0x0027_a984)?;
                c.g[A0][0] = c.g[S0][0];
                break 'queued;
            }
            c.cycles += 2;
            let intro = c.g[A2][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if intro {
                c.cycles += 8;
                c.g[A1][0] = 0x1;
                oc738_list_add_choice(c, 0xc, 1, 1, 0x0027_a920)?;
            }
            c.cycles += 18;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1;
            oc738_list_add_choice(c, 0xd, 0, 1, 0x0027_a944)?;
            c.g[V1][0] = Addr(gp.wrapping_sub(0x854)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
            let flags = c.g[V1][0] as u32;
            c.sw(flags.wrapping_add(0xf44), c.g[A1][0]);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x7c)).lw(c);
            let one = c.g[A0][0] == c.g[A1][0];
            c.g[A0][0] = c.g[S0][0];
            if one {
                c.cycles += 3;
                c.g[A2][0] = 0xe;
                c.g[A3][0] = 0x8;
                c.g[T0][0] = 1;
                c.cycles += 6;
                c.g[T1][0] = u64::MAX;
                c.g[T2][0] = u64::MAX;
                c.g[T3][0] = u64::MAX;
                call_ptr(c, 0x0027_8e50, 0x0027_a984)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 8;
                oc738_list_add_choice(c, 0xf, 8, 0, 0x0027_a9a8)?;
                c.g[A0][0] = c.g[S0][0];
            }
        }
        c.cycles += 17;
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0027_8f38, 0x0027_a9b4)?; // ScriptEngine_ListPrepare(self, 1)
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x0027_8f68, 0x0027_a9c8)?; // ScriptEngine_ListStart(self, 1, 0, 1)
        call_ptr(c, 0x0028_b180, 0x0027_a9d0)?; // SSXAudioSystem_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x14;
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0028_e8c0, 0x0027_a9e0)?; // SSXAudioSystem_HandleMusicEvent(audio, 20, 1)
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The tail element of a circular queue (`Array[Tail]`): the queue header is
/// at the address in `q` (`Array` at +4, `Tail` at +0xc), the element lands in
/// `dst`. The queue is known to be non-empty here (5 cycles).
fn oc738_queue_peek(c: &mut Cpu, q: usize, dst: usize) {
    c.cycles += 5;
    let base = c.g[q][0] as u32;
    c.g[V0][0] = Addr(base.wrapping_add(0xc)).lw(c);
    c.g[V1][0] = Addr(base.wrapping_add(0x4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[dst][0] = Addr(c.g[V0][0] as u32).lw(c);
}

/// Loads the head queue's file into the main buffer: the file name index
/// (`mFilenameIndex` queue at +0x19c, -1 when empty) picks a 64-byte name
/// slot at +0xc; `ASYNCFILE_loadat(name, buffer (+0x1cc), size (+0x1d0))`
/// and its handle goes to `self->mHandle` (+0). The state machine's kind 0
/// and kind 2 only differ in the return address.
fn oc738_load_at_buffer(c: &mut Cpu, ret: u32) -> Flow {
    let this = c.g[S0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x1a8)).lw(c);
    let not_empty = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = sx(this.wrapping_add(0x19c));
    if not_empty {
        oc738_queue_peek(c, A0, V0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
    }
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[V0][0] as u32) << 6);
    c.g[A1][0] = Addr(this.wrapping_add(0x1cc)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[A2][0] = Addr(this.wrapping_add(0x1d0)).lw(c);
    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, 0x003d_f748, ret)?; // ASYNCFILE_loadat
    c.sw(this, c.g[V0][0]);
    Ok(())
}

/// Kind 1: the buffer is allocated through the object's virtual methods
/// (vtable at +0x1d4: entry 3 allocates `(handle, -1, flags)`, entry 4 gives
/// the size of `handle`); with a buffer the file is read into it
/// (`ASYNCFILE_loadat`), else it is loaded by `ASYNCFILE_load(name, flags)`.
fn oc738_load_allocated(c: &mut Cpu) -> Flow {
    let this = c.g[S0][0] as u32;
    // The file name (index queue +0x19c, -1 when empty).
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x1a4)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1a8)).lw(c);
    let not_empty = c.g[V1][0] != c.g[V0][0];
    c.g[A0][0] = sx(this.wrapping_add(0x19c));
    if not_empty {
        oc738_queue_peek(c, A0, V0);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
    }
    // The allocation handle (user handle queue, s2) and the vtable entry 3.
    c.cycles += 10;
    c.g[V1][0] = Addr(this.wrapping_add(0x1d4)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = Addr(this.wrapping_add(0x194)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lh(c);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = Addr(this.wrapping_add(0x198)).lw(c);
    c.g[S4][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    let has_handle = c.g[A1][0] != c.g[V1][0];
    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
    if has_handle {
        oc738_queue_peek(c, S2, A1);
    } else {
        c.cycles += 2;
        c.g[A1][0] = u64::MAX;
    }
    // The flags (main memory flags queue +0x1bc, s1).
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x1c4)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1c8)).lw(c);
    let has_flags = c.g[V1][0] != c.g[V0][0];
    c.g[S1][0] = sx(this.wrapping_add(0x1bc));
    if has_flags {
        oc738_queue_peek(c, S1, A3);
    } else {
        c.cycles += 2;
        c.g[A3][0] = u64::MAX;
    }
    // buffer = vt[3].fn(this + delta, handle, -1, flags)
    c.cycles += 11;
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
    let alloc = c.g[V0][0] as u32;
    c.g[A2][0] = u64::MAX;
    call_ptr(c, alloc, 0x0028_a49c)?;
    c.g[S3][0] = c.g[V0][0];
    // size = vt[4].fn(this + delta, handle)
    c.g[A1][0] = Addr(this.wrapping_add(0x194)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1d4)).lw(c);
    c.g[A0][0] = Addr(this.wrapping_add(0x198)).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24));
    let has_handle = c.g[A1][0] != c.g[A0][0];
    c.g[A2][0] = sx(this.wrapping_add(c.g[V1][0] as u32));
    if has_handle {
        oc738_queue_peek(c, S2, A1);
    } else {
        c.cycles += 2;
        c.g[A1][0] = u64::MAX;
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
    let size_fn = c.g[V0][0] as u32;
    c.g[A0][0] = c.g[A2][0];
    call_ptr(c, size_fn, 0x0028_a4e4)?;
    let no_buffer = c.g[S3][0] == 0;
    c.g[A0][0] = c.g[S4][0];
    if !no_buffer {
        c.cycles += 5;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x003d_f748, 0x0028_a4f8)?; // ASYNCFILE_loadat(name, buffer, size)
        c.sw(this, c.g[V0][0]);
        return Ok(());
    }
    // No buffer: ASYNCFILE_load(name, flags).
    c.cycles += 4;
    c.g[V1][0] = Addr(this.wrapping_add(0x1c4)).lw(c);
    c.g[V0][0] = Addr(this.wrapping_add(0x1c8)).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
        c.cycles += 4;
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
    } else {
        c.cycles += 2;
        c.g[A1][0] = u64::MAX;
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S4][0];
    call_ptr(c, 0x003d_f690, 0x0028_a530)?; // ASYNCFILE_load(name, flags)
    c.sw(this, c.g[V0][0]);
    Ok(())
}

/// `void SndLoadQueue_StartNext(SndLoadQueue* self)` (0x28a298), the C++
/// `cAsyncSys_ASYNCSYS_StartLoad`: when no load is in flight (+4) and the
/// user-handle queue (+0x18c) is not empty, marks the system busy, notifies
/// through the vtable (+0x1d4, entry 2) with the next handle and starts the
/// asynchronous load for the kind at the tail of the kind queue (+0x1ac):
/// 0 and 2 load a named file into the main buffer, 1 allocates a buffer
/// through the vtable first. The file handle is stored at `self+0`.
pub fn snd_load_queue_start_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this.wrapping_add(0x4)).lw(c);
        'exit: {
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x50);
                break 'exit;
            }
            'done: {
                c.cycles += 4;
                c.g[V1][0] = Addr(this.wrapping_add(0x194)).lw(c);
                c.g[V0][0] = Addr(this.wrapping_add(0x198)).lw(c);
                let empty = c.g[V1][0] == c.g[V0][0];
                c.g[S2][0] = sx(this.wrapping_add(0x18c));
                if empty {
                    break 'done;
                }
                // Busy; notify(handle at the tail of the user-handle queue).
                c.cycles += 16;
                c.g[V1][0] = 0x1;
                c.g[A1][0] = Addr(this.wrapping_add(0x1d4)).lw(c);
                c.sw(this.wrapping_add(0x4), c.g[V1][0]);
                c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x10)).lh(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A2][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let notify = c.g[A2][0] as u32;
                c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, notify, 0x0028_a304)?;
                // The kind at the tail of the kind queue (-1 when empty).
                c.g[V1][0] = Addr(this.wrapping_add(0x1b4)).lw(c);
                c.g[V0][0] = Addr(this.wrapping_add(0x1b8)).lw(c);
                let has_kind = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx(this.wrapping_add(0x1ac));
                if has_kind {
                    oc738_queue_peek(c, A0, V1);
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = u64::MAX;
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let kind1 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                if kind1 {
                    oc738_load_allocated(c)?;
                    break 'done;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // Kind >= 2: only 2 loads.
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x50);
                        break 'exit;
                    }
                    c.g[V1][0] = Addr(this.wrapping_add(0x1a4)).lw(c);
                    oc738_load_at_buffer(c, 0x0028_a3ec)?;
                } else {
                    // Kind < 2: only 0 loads.
                    c.cycles += 2;
                    if c.g[V1][0] != 0 {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x50);
                        break 'exit;
                    }
                    c.g[V1][0] = Addr(this.wrapping_add(0x1a4)).lw(c);
                    oc738_load_at_buffer(c, 0x0028_a3a4)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Epilogue of `EventSelect_IsEventCompleted`.
fn oc738_event_completed_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore_ra(c, 0x70);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    const OC738_CTRL: u32 = DATA + 0x20_0000;
    const OC738_MAGIC: u32 = 0x4f43_3738;
    /// Control words `OC738_CTRL + 4, 8, 0xc`: non-zero makes the medal fake
    /// return -1, `isLoaded` and `IsActiveNISInTransition` return 0.
    const OC738_MEDAL_M1: u32 = OC738_CTRL + 4;
    const OC738_NOT_LOADED: u32 = OC738_CTRL + 8;
    const OC738_NIS_IDLE: u32 = OC738_CTRL + 0xc;

    /// A fake for a callee's override slot, active only on machines that set
    /// the control word (the suite runs its tests in parallel).
    macro_rules! oc738_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC738_CTRL) != OC738_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc738_fake!(
        oc738_fake_get_be,
        OC738_PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );
    oc738_fake!(
        oc738_fake_get_interface,
        OC738_PREV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );
    oc738_fake!(
        oc738_fake_get_char_id,
        OC738_PREV_GET_CHAR_ID,
        ee::s01::f_0014a080__generated,
        0x0014_a080
    );
    oc738_fake!(
        oc738_fake_event_locked,
        OC738_PREV_EVENT_LOCKED,
        ee::s01::f_00146a70__generated,
        0x0014_6a70
    );

    oc738_fake!(
        oc738_fake_elem_pos,
        OC738_PREV_ELEM_POS,
        ee::s04::f_001e91f8__generated,
        0x001e_91f8
    );
    oc738_fake!(
        oc738_fake_elem_scale,
        OC738_PREV_ELEM_SCALE,
        ee::s04::f_001e9290__generated,
        0x001e_9290
    );
    oc738_fake!(
        oc738_fake_elem_color,
        OC738_PREV_ELEM_COLOR,
        ee::s04::f_001e91a8__generated,
        0x001e_91a8
    );

    oc738_fake!(
        oc738_fake_replay_save_snapshot,
        OC738_PREV_REPLAY_SAVE_SNAPSHOT,
        ee::s07::f_0026f7f8__generated,
        0x0026_f7f8
    );
    oc738_fake!(
        oc738_fake_replay_set_state,
        OC738_PREV_REPLAY_SET_STATE,
        ee::s07::f_0026f4a0__generated,
        0x0026_f4a0
    );
    oc738_fake!(
        oc738_fake_replay_seek,
        OC738_PREV_REPLAY_SEEK,
        ee::s07::f_002702f8__generated,
        0x0027_02f8
    );
    oc738_fake!(
        oc738_fake_replay_step,
        OC738_PREV_REPLAY_STEP,
        ee::s07::f_002703f0__generated,
        0x0027_03f0
    );
    oc738_fake!(
        oc738_fake_replay_instant_cams,
        OC738_PREV_REPLAY_INSTANT_CAMS,
        ee::s07::f_0026ef30__generated,
        0x0026_ef30
    );
    oc738_fake!(
        oc738_fake_replay_cams,
        OC738_PREV_REPLAY_CAMS,
        ee::s07::f_0026ee68__generated,
        0x0026_ee68
    );
    oc738_fake!(
        oc738_fake_audio_instance,
        OC738_PREV_AUDIO_INSTANCE,
        ee::s07::f_0028b180__generated,
        0x0028_b180
    );
    oc738_fake!(
        oc738_fake_audio_replay_begin,
        OC738_PREV_AUDIO_REPLAY_BEGIN,
        ee::s07::f_0028f200__generated,
        0x0028_f200
    );

    oc738_fake!(
        oc738_fake_list_add_choice,
        OC738_PREV_LIST_ADD_CHOICE,
        ee::s07::f_00278e50__generated,
        0x0027_8e50
    );
    oc738_fake!(
        oc738_fake_list_prepare,
        OC738_PREV_LIST_PREPARE,
        ee::s07::f_00278f38__generated,
        0x0027_8f38
    );
    oc738_fake!(
        oc738_fake_list_start,
        OC738_PREV_LIST_START,
        ee::s07::f_00278f68__generated,
        0x0027_8f68
    );

    oc738_fake!(
        oc738_fake_async_load_at,
        OC738_PREV_ASYNC_LOAD_AT,
        ee::s14::f_003df748__generated,
        0x003d_f748
    );
    oc738_fake!(
        oc738_fake_async_load,
        OC738_PREV_ASYNC_LOAD,
        ee::s14::f_003df690__generated,
        0x003d_f690
    );

    /// A fake whose result becomes `$val` when the control word at
    /// `OC738_CTRL + $flag` is non-zero (a result the generic fake cannot
    /// produce: -1, or zero).
    macro_rules! oc738_fake_val {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, $flag:expr, $val:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC738_CTRL) != OC738_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                if c.pc == c.g[RA][0] as u32 && c.bus.read32(OC738_CTRL + $flag) != 0 {
                    c.g[V0][0] = $val;
                }
            }
        };
    }

    // `getTrackMedal` returns -1 (no medal).
    oc738_fake_val!(
        oc738_fake_medal,
        OC738_PREV_MEDAL,
        ee::s01::f_00158910__generated,
        0x0015_8910,
        4,
        u64::MAX
    );
    // `cSectionMan_isLoaded` returns 0.
    oc738_fake_val!(
        oc738_fake_is_loaded,
        OC738_PREV_IS_LOADED,
        ee::s05::f_0022d278__generated,
        0x0022_d278,
        8,
        0
    );
    // `IsActiveNISInTransition` returns 0.
    oc738_fake_val!(
        oc738_fake_nis_active,
        OC738_PREV_NIS_ACTIVE,
        ee::s07::f_00279298__generated,
        0x0027_9298,
        0xc,
        0
    );
    oc738_fake!(
        oc738_fake_nis_end_pos,
        OC738_PREV_NIS_END_POS,
        ee::s00::f_00123f38__generated,
        0x0012_3f38
    );
    oc738_fake!(
        oc738_fake_set_focus,
        OC738_PREV_SET_FOCUS,
        ee::s13::f_003a9658__generated,
        0x003a_9658
    );
    oc738_fake!(
        oc738_fake_get_test_focus,
        OC738_PREV_GET_TEST_FOCUS,
        ee::s13::f_003a9770__generated,
        0x003a_9770
    );
    oc738_fake!(
        oc738_fake_get_transition_man,
        OC738_PREV_GET_TRANSITION_MAN,
        ee::s06::f_00230698__generated,
        0x0023_0698
    );
    oc738_fake!(
        oc738_fake_end_transport,
        OC738_PREV_END_TRANSPORT,
        ee::s07::f_0027a9f0__generated,
        0x0027_a9f0
    );
    oc738_fake!(
        oc738_fake_section_activate,
        OC738_PREV_SECTION_ACTIVATE,
        ee::s05::f_0022d088__generated,
        0x0022_d088
    );

    fn oc738_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc738_fake_get_be,
            &OC738_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc738_fake_get_interface,
            &OC738_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014A080),
            oc738_fake_get_char_id,
            &OC738_PREV_GET_CHAR_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146A70),
            oc738_fake_event_locked,
            &OC738_PREV_EVENT_LOCKED,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E91F8),
            oc738_fake_elem_pos,
            &OC738_PREV_ELEM_POS,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E9290),
            oc738_fake_elem_scale,
            &OC738_PREV_ELEM_SCALE,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E91A8),
            oc738_fake_elem_color,
            &OC738_PREV_ELEM_COLOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D278),
            oc738_fake_is_loaded,
            &OC738_PREV_IS_LOADED,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00279298),
            oc738_fake_nis_active,
            &OC738_PREV_NIS_ACTIVE,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_00123F38),
            oc738_fake_nis_end_pos,
            &OC738_PREV_NIS_END_POS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9658),
            oc738_fake_set_focus,
            &OC738_PREV_SET_FOCUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9770),
            oc738_fake_get_test_focus,
            &OC738_PREV_GET_TEST_FOCUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230698),
            oc738_fake_get_transition_man,
            &OC738_PREV_GET_TRANSITION_MAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027A9F0),
            oc738_fake_end_transport,
            &OC738_PREV_END_TRANSPORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D088),
            oc738_fake_section_activate,
            &OC738_PREV_SECTION_ACTIVATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F7F8),
            oc738_fake_replay_save_snapshot,
            &OC738_PREV_REPLAY_SAVE_SNAPSHOT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            oc738_fake_replay_set_state,
            &OC738_PREV_REPLAY_SET_STATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002702F8),
            oc738_fake_replay_seek,
            &OC738_PREV_REPLAY_SEEK,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002703F0),
            oc738_fake_replay_step,
            &OC738_PREV_REPLAY_STEP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026EF30),
            oc738_fake_replay_instant_cams,
            &OC738_PREV_REPLAY_INSTANT_CAMS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026EE68),
            oc738_fake_replay_cams,
            &OC738_PREV_REPLAY_CAMS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc738_fake_audio_instance,
            &OC738_PREV_AUDIO_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028F200),
            oc738_fake_audio_replay_begin,
            &OC738_PREV_AUDIO_REPLAY_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278E50),
            oc738_fake_list_add_choice,
            &OC738_PREV_LIST_ADD_CHOICE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F38),
            oc738_fake_list_prepare,
            &OC738_PREV_LIST_PREPARE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F68),
            oc738_fake_list_start,
            &OC738_PREV_LIST_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF748),
            oc738_fake_async_load_at,
            &OC738_PREV_ASYNC_LOAD_AT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            oc738_fake_async_load,
            &OC738_PREV_ASYNC_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00158910),
            oc738_fake_medal,
            &OC738_PREV_MEDAL,
        );
    }

    /// A scrambled machine with the fakes on and the gp of the game.
    fn oc738_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC738_CTRL, OC738_MAGIC);
        r
    }

    /// An unaligned guest word store (byte stores).
    fn oc738_put32(c: &mut Cpu, a: u32, v: u32) {
        for i in 0..4 {
            c.bus.write8(a + i, (v >> (8 * i)) as u8);
        }
    }

    /// `sp` inside the frame of the 0x80-byte functions called with the
    /// machine's stack.
    const SP_AFTER_FRAME: u32 = crate::difftest::SP - 0x80;

    /// `EventSelect_IsEventCompleted`: the goal struct at an aligned or
    /// unaligned address; location 23, event types 4/5 (peaks 0..2) and
    /// other event types (medal present or -1).
    #[test]
    fn event_select_is_event_completed_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s05::f_00200b50__generated,
            event_select_is_event_completed,
            400,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x20_0b50, 6);
                let base = DATA + 0x1000;
                fill(c, &mut r, base, 0x20);
                let off = if seed % 3 == 0 { 0 } else { 1 + (seed / 3) % 7 };
                let location = if seed % 5 == 0 {
                    0x17
                } else {
                    r.next_u32() % 40
                };
                let event = [4u32, 5, 4, 5, 0, 1, 2, 3, 6][((seed / 5) % 9) as usize];
                oc738_put32(c, base + off, location);
                oc738_put32(c, base + off + 8, event);
                c.bus.write32(OC738_MEDAL_M1, ((seed / 45) % 2) as u32);
                c.g[A0][0] = sx(DATA + 0x2000);
                c.g[A1][0] = sx(base + off);
                c.g[A2][0] = (seed % 4) as u64;
                c.pc = 0x0020_0b50;
            },
        );
    }

    /// `HUD_DrawOptionIconText`: the option struct's mode word (the 0x300000
    /// bits equal 0x200000 or not), script playing or not, and progress on
    /// both sides of 0.5. The last callee (`HUD_DrawIconText`) is the real
    /// one and returns at once (the message object's text is empty). The
    /// option interface address is constant and 8-byte aligned, so the
    /// unaligned copy is not reachable from here.
    #[test]
    fn hud_draw_option_icon_text_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s05::f_00220ad0__generated,
            hud_draw_option_icon_text,
            300,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x22_0ad0, 5);
                let (obj, data, engine) = (DATA + 0x4000, DATA + 0x6000, DATA + 0x3000);
                let (grman, state, font) = (DATA + 0x7000, DATA + 0x8000, DATA + 0x9000);
                fill(c, &mut r, 0x0053_5610, 0xa2);
                let mode = [0u32, 0x0020_0000, 0x0010_0000, 0x0030_0000, r.next_u32()]
                    [(seed % 5) as usize];
                c.bus.write32(0x0053_5610, mode);
                c.bus.write32(GAME_GP.wrapping_sub(0x84c), engine);
                c.bus.write32(engine + 0x504, (seed / 5) % 2);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), grman);
                c.bus.write32(grman + 0xe84, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(obj + 0xb50, font);
                for i in 0..3 {
                    c.bus.write8(data + 0xaf4 + i, r.next_u32() as u8);
                }
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(data);
                c.f[12] = [
                    0x3e80_0000u32,
                    0x3f00_0000,
                    0x3f40_0000,
                    0x3f80_0000,
                    0,
                    r.edge_f32(),
                ][((seed / 10) % 6) as usize];
                c.pc = 0x0022_0ad0;
            },
        );
    }

    /// `GameStateEventSwitchLoad_Update`: not waiting; state 0 (section not
    /// loaded / loaded); state 1 (focus too far; transition manager busy, a
    /// transition playing, NIS active, or all idle); other states.
    #[test]
    fn game_state_event_switch_load_update_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s06::f_00236960__generated,
            game_state_event_switch_load_update,
            600,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x23_6960, 6);
                let gp = GAME_GP;
                let (holder, game, sm, vm, vm2) = (
                    DATA + 0x1000,
                    DATA + 0x1100,
                    DATA + 0x1200,
                    DATA + 0x1300,
                    DATA + 0x1400,
                );
                let (focus, views, this, tm, tm_vt) = (
                    DATA + 0x1500,
                    DATA + 0x1600,
                    DATA + 0x1700,
                    DATA + 0x1800,
                    DATA + 0x1900,
                );
                c.bus.write32(gp.wrapping_sub(0x848), holder);
                c.bus.write32(holder + 0x84, game);
                c.bus.write32(game + 0x78, sm);
                c.bus
                    .write32(sm + 0x1c8, [0u32, 1, 1, 1][(seed % 4) as usize]);
                c.bus.write32(game + 0x84, vm);
                c.bus.write32(vm + 4, vm2);
                c.bus
                    .write32(vm2 + 0xb0, [0u32, 1, 2][((seed / 4) % 3) as usize]);
                c.bus.write32(game + 0x10, focus);
                c.bus.write32(focus, views);
                c.bus.write32(game + 0xc, DATA + 0x1a00);
                c.bus.write32(DATA + 0x1a00 + 0x40, DATA + 0x1b00);
                // The player camera slot read for the NIS end position.
                c.bus.write32(DATA + 0x1b00 + 0x18, DATA + 0x1c00);
                c.bus.write32(sm + 0x1bc, r.next_u32());
                let state = [0u32, 1, 1, 1, 2, 3, u32::MAX][((seed / 12) % 7) as usize];
                c.bus.write32(this + 0x10, state);
                // 45000.0 / 20000.0 float constants.
                c.bus.write32(gp.wrapping_sub(0x5168), 0x472f_c800);
                c.bus.write32(gp.wrapping_sub(0x5164), 0x469c_4000);
                // The focus distance getTestFocus leaves at sp+0x10.
                let dist = if (seed / 3) % 2 == 0 {
                    0x461c_4000u32
                } else {
                    0x46ea_6000
                };
                c.bus.write32(SP_AFTER_FRAME + 0x10, dist);
                c.bus.write32(OC738_NOT_LOADED, (seed / 7) % 2);
                c.bus.write32(OC738_NIS_IDLE, (seed / 5) % 2);
                // Fake call 1 of state 1 is GetTransitionMan: a real object.
                c.bus.write32(fakes::LOG_RESULTS + 4, tm);
                c.bus.write32(tm + 0x1c, tm_vt);
                c.bus
                    .write16(tm_vt + 0x40, (r.next_u32() as i16 as i32 & !3) as u16);
                // isBusy: the zero leaf (idle) or a fake (busy).
                let busy = if (seed / 2) % 2 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE
                };
                c.bus.write32(tm_vt + 0x44, busy);
                c.bus.write32(tm + 0x44, (seed / 11) % 2);
                c.g[A0][0] = sx(this);
                c.pc = 0x0023_6960;
            },
        );
    }

    /// `Replay_BeginPlayback`: already playing (14, 15) or not, a loaded
    /// snapshot or not, instant or full replay, auto replay or not.
    #[test]
    fn replay_begin_playback_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s07::f_0026f8a0,
            replay_begin_playback,
            400,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x26_f8a0, 8);
                let this = DATA + 0x1000;
                let state = [14u32, 15, 0, 7, 13, 16, 3][(seed % 7) as usize];
                c.bus.write32(this, state);
                c.bus.write32(this + 0x608, (seed / 7) % 2);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = ((seed / 14) % 2) as u64 * (1 + r.next_u32() as u64 % 5);
                c.g[A3][0] = ((seed / 28) % 2) as u64;
                c.pc = 0x0026_f8a0;
            },
        );
    }

    /// The globals `ScriptEngine_QueueFinishScripts` reads: the world mode
    /// (`world->+0x84->+0xc->+0x7c`) and the graphics manager's flag block.
    fn oc738_queue_finish_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = oc738_machine(c, seed, salt, 6);
        let gp = GAME_GP;
        let (w, x, y, g) = (DATA + 0x1100, DATA + 0x1200, DATA + 0x1300, DATA + 0x1400);
        c.bus.write32(gp.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x84, x);
        c.bus.write32(x + 0xc, y);
        c.bus
            .write32(y + 0x7c, [1u32, 0, 2][((seed / 4) % 3) as usize]);
        c.bus.write32(gp.wrapping_sub(0x854), g);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = (seed % 2) as u64;
        c.g[A2][0] = ((seed / 2) % 2) as u64;
        c.pc = 0x0027_a860;
        r
    }

    /// `ScriptEngine_QueueFinishScripts`, online and offline, with and
    /// without the intro, world mode 1 or not. The final callee
    /// (`SSXAudioSystem_HandleMusicEvent`) has no override slot and its real
    /// body does not stay short on test data, so every seed unwinds at or
    /// before the `GetInstance` call before it (the full path is in the
    /// ignored test below).
    #[test]
    fn script_engine_queue_finish_scripts_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s07::f_0027a860__generated,
            script_engine_queue_finish_scripts,
            400,
            &|c, seed| {
                let mut r = oc738_queue_finish_fixture(c, seed, 0x27_a860);
                let intro = (seed / 2) % 2;
                c.bus
                    .write32(fakes::LOG_UNWIND_AT, r.next_u32() % (5 + intro));
            },
        );
    }

    /// The same with the real `HandleMusicEvent` at the end: it runs the real
    /// audio code on test data; run it with `--ignored` where that is
    /// bounded.
    #[test]
    #[ignore = "the real SSXAudioSystem_HandleMusicEvent has no bounded fixture"]
    fn script_engine_queue_finish_scripts_full_matches() {
        oc738_install_fakes();
        check(
            ee::s07::f_0027a860__generated,
            script_engine_queue_finish_scripts,
            100,
            &|c, seed| {
                let _ = oc738_queue_finish_fixture(c, seed, 0x27_a861);
            },
        );
    }

    /// A circular queue header at `q` (`Array` +4, `Head` +8, `Tail` +0xc)
    /// holding `values` from tail 1 on: empty when `values` is empty.
    fn oc738_queue(c: &mut Cpu, q: u32, array: u32, values: &[u32]) {
        c.bus.write32(q + 4, array);
        if values.is_empty() {
            c.bus.write32(q + 8, 2);
            c.bus.write32(q + 0xc, 2);
            return;
        }
        c.bus.write32(q + 8, 5);
        c.bus.write32(q + 0xc, 1);
        for (i, v) in values.iter().enumerate() {
            c.bus.write32(array + 4 * (1 + i as u32), *v);
        }
    }

    /// `SndLoadQueue_StartNext`: busy; empty user queue; every queue empty or
    /// not; kinds 0, 1 (buffer allocated or not), 2 and invalid ones.
    #[test]
    fn snd_load_queue_start_next_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s07::f_0028a298__generated,
            snd_load_queue_start_next,
            600,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x28_a298, 4);
                let this = DATA + 0x1000;
                let vt = DATA + 0x2000;
                c.bus.write32(this + 4, (seed % 7 == 0) as u32);
                let handle = r.next_u32();
                let one_user = [handle];
                let users: &[u32] = if seed % 5 == 0 { &[] } else { &one_user };
                oc738_queue(c, this + 0x18c, DATA + 0x3000, users);
                let kind = [0u32, 1, 2, 3, u32::MAX, 1, 0, 2, 1][((seed / 7) % 9) as usize];
                let one_kind = [kind];
                let kinds: &[u32] = if (seed / 63) % 8 == 0 { &[] } else { &one_kind };
                oc738_queue(c, this + 0x1ac, DATA + 0x3100, kinds);
                let one_name = [r.next_u32() % 8];
                let names: &[u32] = if (seed / 3) % 6 == 0 { &[] } else { &one_name };
                oc738_queue(c, this + 0x19c, DATA + 0x3200, names);
                let one_flag = [r.next_u32()];
                let flags: &[u32] = if (seed / 4) % 4 == 0 { &[] } else { &one_flag };
                oc738_queue(c, this + 0x1bc, DATA + 0x3300, flags);
                c.bus.write32(this + 0x1cc, r.next_u32());
                c.bus.write32(this + 0x1d0, r.next_u32());
                c.bus.write32(this + 0x1d4, vt);
                for off in [0x10u32, 0x18, 0x20] {
                    c.bus
                        .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
                }
                c.bus.write32(vt + 0x14, fakes::FAKE);
                // Allocation: a fake (non-zero buffer) or the zero leaf.
                let alloc = if (seed / 2) % 2 == 0 {
                    fakes::FAKE + 0x10
                } else {
                    0x0014_06c0
                };
                c.bus.write32(vt + 0x1c, alloc);
                c.bus.write32(vt + 0x24, fakes::FAKE + 0x20);
                c.g[A0][0] = sx(this);
                c.pc = 0x0028_a298;
            },
        );
    }
}
