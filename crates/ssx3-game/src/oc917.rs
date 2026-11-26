//! Staging module oc917 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc917`.

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
        0x001a_7f78,
        cfe_state_read_message_popup_load_messages,
        "cFEStateReadMessagePopup_LoadMessages",
    ),
    (
        0x001a_ad50,
        net_fe_set_quality_icons,
        "NetFE_SetQualityIcons",
    ),
    (
        0x001a_cef8,
        cfe_use_existing_account_v31,
        "cFEUseExistingAccount_v31",
    ),
    (
        0x001a_ec60,
        cfe_state_challenge_popup_set_settings_locked,
        "cFEStateChallengePopup_SetSettingsLocked",
    ),
    (
        0x001a_f728,
        cfe_state_challenge_popup_fill_challenge_text,
        "cFEStateChallengePopup_FillChallengeText",
    ),
];

/// `reg + off` of a register holding a pointer.
#[inline]
fn oc917_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp + off` (`off` may be negative).
#[inline]
fn oc917_gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// Same-shard callees (they have override slots, so the generated wrappers
/// dispatch through them).
const OC917_ADD_MESSAGE: Callee = Callee {
    addr: 0x001a_7dd8,
    f: ee::s03::f_001a7dd8,
};
const OC917_GET_POPUP: Callee = Callee {
    addr: 0x001a_9c50,
    f: ee::s03::f_001a9c50,
};
const OC917_SHOW_POPUP: Callee = Callee {
    addr: 0x001a_9bc0,
    f: ee::s03::f_001a9bc0,
};
const OC917_CFG_OPTION_TEXT: Callee = Callee {
    addr: 0x001c_57c0,
    f: ee::s03::f_001c57c0,
};
const OC917_HANDLE_NET_EVENT: Callee = Callee {
    addr: 0x001a_8e40,
    f: ee::s03::f_001a8e40,
};

/// A virtual call through the object in `obj`: `v0 = obj->vtbl` (`+8`),
/// `a0 = obj + vtbl[slot].delta` (`lh`), `v1 = vtbl[slot].fn` (`+4`), call
/// `v1` returning to `ra`. The other arguments are set by the caller.
fn oc917_vslot(c: &mut Cpu, obj: usize, slot: u32, ra: u32) -> Flow {
    c.g[V0][0] = c.lw(oc917_at(c, obj, 8));
    c.g[A0][0] = c.lh(oc917_at(c, V0, slot));
    c.g[V1][0] = c.lw(oc917_at(c, V0, slot + 4));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(oc917_at(c, obj, 0).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ra)
}

/// The inlined "make sure the lobby session exists" sequence of
/// `LoadMessages`: with the session pointer `lw(gp+0x238)` already in `v0`,
/// create it (0x260f80) when null, then ask 0x261460 whether it is ready (if
/// not, 0x2613c8), and finally call the lobby manager (`gp-0xc8`) through
/// 0x25b800 when the session is still null. `first` is the cycle cost of the
/// leading null test, `ra` the four return addresses, `reload_a0` whether the
/// compiler had `a0 = lw(gp+0x238)` in the last branch delay slot.
fn oc917_session(c: &mut Cpu, first: u64, ra: [u32; 4], reload_a0: bool) -> Flow {
    c.cycles += first;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, ra[0])?;
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(oc917_gp(c, 0x238));
    call_ptr(c, 0x0026_1460, ra[1])?;
    let ready = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
    if !ready {
        c.cycles += 3;
        c.g[A0][0] = c.lw(oc917_gp(c, 0x238));
        call_ptr(c, 0x0026_13c8, ra[2])?;
        c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
    }
    c.cycles += 2;
    let have_session = c.g[V0][0] != 0;
    if reload_a0 {
        c.g[A0][0] = c.lw(oc917_gp(c, 0x238));
    }
    if !have_session {
        c.cycles += if reload_a0 { 3 } else { 2 };
        c.g[A0][0] = c.lw(oc917_gp(c, -0xc8));
        call_ptr(c, 0x0025_b800, ra[3])?;
        if reload_a0 {
            c.g[A0][0] = c.lw(oc917_gp(c, 0x238));
        }
    }
    Ok(())
}

/// `void cFEStateReadMessagePopup::LoadMessages()` (0x1a7f78): copy the
/// lobby session's private messages from the sender in `self->mszSender`
/// into the popup (`AddMessage` of a new 0x10-byte `cPrivateMessage` that
/// copies three strings), then delete those messages from the session,
/// scanning backwards. The session accessor is inlined five times.
///
/// An event unwind inside either loop leaves `pc` at the loop head; the
/// re-entry (a `pc` that is not the entry) runs the generated body.
pub fn cfe_state_read_message_popup_load_messages(c: &mut Cpu) {
    if c.pc != 0x001a_7f78 {
        return ee::s03::f_001a7f78__generated(c);
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x0047_0000;
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 0x0046_0000;
        frame.save(c, S0, 0x60);
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2740));
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);

        // Forward scan: add the messages of this sender to the popup.
        loop {
            c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
            oc917_session(
                c,
                3,
                [0x001a_807c, 0x001a_8084, 0x001a_8094, 0x001a_80a8],
                false,
            )?;
            c.cycles += 5;
            c.g[A0][0] = c.lw(oc917_gp(c, 0x238));
            call_ptr(c, 0x0026_2938, 0x001a_80b0)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
            if !more {
                break;
            }
            if event_due(c) {
                c.pc = 0x001a_7fb0;
                if c.event() {
                    return Ok(());
                }
            }
            oc917_session(
                c,
                2,
                [0x001a_7fc0, 0x001a_7fc8, 0x001a_7fd8, 0x001a_7fec],
                true,
            )?;
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x0026_3630, 0x001a_7ff8)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0 {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                continue;
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(oc917_at(c, S1, 0));
            c.g[A1][0] = c.lw(oc917_at(c, S3, 0x6d0));
            call_ptr(c, 0x0041_aa88, 0x001a_8010)?;
            if c.g[V0][0] != 0 {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                continue;
            }
            // A match: a new cPrivateMessage (0x10 bytes) copying the item.
            c.cycles += 20;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1a08));
            call_ptr(c, 0x0031_7d70, 0x001a_802c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.sw(oc917_at(c, S0, 0xc), c.g[S5][0]);
            call_ptr(c, 0x0031_7fe8, 0x001a_8040)?;
            c.g[A0][0] = sx(oc917_at(c, S0, 4));
            c.g[A1][0] = sx(oc917_at(c, S1, 4));
            call_ptr(c, 0x0031_7fe8, 0x001a_804c)?;
            c.g[A1][0] = sx(oc917_at(c, S1, 8));
            c.g[A0][0] = sx(oc917_at(c, S0, 8));
            call_ptr(c, 0x0031_7fe8, 0x001a_8058)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S3][0];
            OC917_ADD_MESSAGE.call(c, 0x001a_8064)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        }

        // Backward scan: remove those messages from the session.
        oc917_session(
            c,
            2,
            [0x001a_80cc, 0x001a_80d4, 0x001a_80e4, 0x001a_80f8],
            false,
        )?;
        c.cycles += 5;
        c.g[A0][0] = c.lw(oc917_gp(c, 0x238));
        call_ptr(c, 0x0026_2938, 0x001a_8100)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if (c.g[S0][0] as i64) < 0 {
            frame.restore(c, S0, 0x60);
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
            loop {
                oc917_session(
                    c,
                    2,
                    [0x001a_8120, 0x001a_8128, 0x001a_8138, 0x001a_814c],
                    true,
                )?;
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0026_3630, 0x001a_8158)?;
                'next: {
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                        break 'next;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(oc917_at(c, V0, 0));
                    c.g[A1][0] = c.lw(oc917_at(c, S3, 0x6d0));
                    call_ptr(c, 0x0041_aa88, 0x001a_816c)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                        break 'next;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
                    oc917_session(
                        c,
                        0,
                        [0x001a_8188, 0x001a_8190, 0x001a_81a0, 0x001a_81b4],
                        true,
                    )?;
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_35a8, 0x001a_81c0)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                }
                c.cycles += 2;
                let again = (c.g[S0][0] as i64) >= 0;
                c.g[V0][0] = c.lw(oc917_gp(c, 0x238));
                if !again {
                    break;
                }
                if event_due(c) {
                    c.pc = 0x001a_8110;
                    if c.event() {
                        return Ok(());
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Load the three colour floats of one quality tier (`gp` pool entries) and
/// the bar count, as `NetFE_SetQualityIcons` does for each ping range.
fn oc917_quality_tier(c: &mut Cpu, cycles: u64, bars: u64, pool: i32) {
    c.cycles += cycles;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.lwc1(0, oc917_gp(c, pool));
    c.lwc1(1, oc917_gp(c, pool + 4));
    c.lwc1(2, oc917_gp(c, pool + 8));
    c.g[S7][0] = bars;
}

/// `void NetFE_SetQualityIcons(void* screen, int slot, int ping)` (0x1aad50):
/// look up the five signal-bar objects `"quality%d%c"` (slot + 1, 'a'..'e')
/// on the screen, pick a colour and a bar count from the ping range
/// (under 200 ms: five bars; 200s: 4; 300s: 3; 400s: 2; 500 and up: 1), and
/// make that many bars visible with the colour, hiding the rest.
pub fn net_fe_set_quality_icons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0xf0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        frame.save(c, S6, 0x90);
        frame.save(c, S7, 0x80);
        frame.save(c, FP, 0x70);
        let no_screen = c.g[S3][0] == 0;
        frame.save_ra(c, 0x60);
        if !no_screen {
            c.cycles += 18;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.lwc1(1, oc917_gp(c, -0x5948));
            c.g[S7][0] = 5;
            c.lwc1(2, oc917_gp(c, -0x5944));
            c.g[S0][0] = 0;
            c.lwc1(3, oc917_gp(c, -0x5940));
            c.g[FP][0] = 0x0046_0000;
            let base = sp(c);
            c.swc1(0, base);
            c.g[S4][0] = sx(base.wrapping_add(0x30));
            c.swc1(1, base.wrapping_add(4));
            c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xc9));
            c.swc1(2, base.wrapping_add(8));
            c.g[S1][0] = sx(base.wrapping_add(0x10));
            c.swc1(3, base.wrapping_add(0xc));
            c.g[A1][0] = sx(oc917_at(c, FP, 0x2678));
            // The five bar objects into objs[0..5] (at sp+0x30).
            loop {
                c.cycles += 12;
                c.g[A2][0] = c.g[S5][0];
                c.g[A3][0] = sx(oc917_at(c, S0, 0x61));
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0041_7828, 0x001a_ade0)?;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0031_7670, 0x001a_ade8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0039_d860, 0x001a_adf4)?;
                c.g[V1][0] = c.g[V0][0];
                let missing = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                if !missing {
                    c.cycles += 2;
                    c.g[V0][0] = sx(oc917_at(c, S4, 0).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                }
                c.cycles += 5;
                c.g[V0][0] = sx(oc917_at(c, S0, 1));
                c.g[S0][0] = c.g[V0][0] & 0xff;
                c.g[V1][0] = (c.g[S0][0] < 5) as u64;
                let more = c.g[V1][0] != 0;
                c.g[A1][0] = sx(oc917_at(c, FP, 0x2678));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_add0);
                }
            }
            // The ping range picks the colour and the bar count.
            c.cycles += 3;
            c.g[V0][0] = (c.g[S6][0] < 0x63) as u64;
            let tier1 = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x12d));
            let mut copy_colour = true;
            'tiers: {
                if tier1 {
                    oc917_quality_tier(c, 7, 4, -0x593c);
                    break 'tiers;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                let tier2 = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x191));
                if tier2 {
                    oc917_quality_tier(c, 7, 3, -0x5930);
                    break 'tiers;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                let tier3 = c.g[V0][0] != 0;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 501) as u64;
                if tier3 {
                    oc917_quality_tier(c, 7, 2, -0x5924);
                    break 'tiers;
                }
                c.cycles += 2;
                let below_501 = c.g[V0][0] != 0;
                c.g[S1][0] = c.g[S4][0];
                if below_501 {
                    // The initial colour and five bars stay.
                    copy_colour = false;
                    break 'tiers;
                }
                oc917_quality_tier(c, 6, 1, -0x5918);
            }
            if copy_colour {
                // The tier's colour (f3, f0, f1, f2) into the 16 bytes at sp.
                c.cycles += 13;
                let base = sp(c);
                c.swc1(3, base.wrapping_add(0x50));
                c.swc1(0, base.wrapping_add(0x54));
                c.swc1(1, base.wrapping_add(0x58));
                c.swc1(2, base.wrapping_add(0x5c));
                c.ldl(2, base.wrapping_add(0x57));
                c.ldr(2, base.wrapping_add(0x50));
                c.ldl(3, base.wrapping_add(0x5f));
                c.ldr(3, base.wrapping_add(0x58));
                c.sdl(2, base.wrapping_add(7));
                c.sdr(2, base);
                c.sdl(3, base.wrapping_add(0xf));
                c.sdr(3, base.wrapping_add(8));
                c.g[S1][0] = c.g[S4][0];
            }
            // Show the first `bars` bars with the colour, hide the others.
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S7][0] as i64)) as u64;
                let hide = c.g[V0][0] == 0;
                c.g[S0][0] = c.lw(oc917_at(c, S1, 0));
                if !hide {
                    c.cycles += 14;
                    c.g[A1][0] = 1;
                    oc917_vslot(c, S0, 0x48, 0x001a_af14)?;
                    c.g[A1][0] = c.g[SP][0];
                    oc917_vslot(c, S0, 0x58, 0x001a_af2c)?;
                    c.g[S2][0] = sx(oc917_at(c, S2, 1));
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = 0;
                    oc917_vslot(c, S0, 0x48, 0x001a_af4c)?;
                    c.g[S2][0] = sx(oc917_at(c, S2, 1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 5) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(oc917_at(c, S1, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001a_aef0);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xf0);
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore(c, S5, 0xa0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// The tail every popup case of `cFEUseExistingAccount_v31` shares: store
/// the answer id in `popup->0x150`, show the popup, return 0.
fn oc917_show_popup_tail(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    c.sw(oc917_at(c, S0, 0x150), c.g[V0][0]);
    c.g[A1][0] = c.g[S0][0];
    OC917_SHOW_POPUP.call(c, 0x001a_d074)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// A one-button info popup (cases 207 and 210 of the account events): type
/// 1, message from the hashed string, an OK button, and the misc id.
/// `ras` are the return addresses of the get-popup, two hash and the option
/// text calls; `msg` is the string offset below 0x460000.
fn oc917_info_popup(c: &mut Cpu, cycles: u64, msg: u32, ras: [u32; 4]) -> Flow {
    c.cycles += cycles;
    c.g[A0][0] = c.g[S2][0];
    OC917_GET_POPUP.call(c, ras[0])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x0046_0000;
    c.g[V0][0] = 1;
    c.g[A0][0] = sx(oc917_at(c, A0, msg));
    c.sw(oc917_at(c, S0, 0xbc), c.g[V0][0]);
    c.g[S1][0] = sx(oc917_at(c, S0, 0xbc));
    call_ptr(c, 0x0031_7670, ras[1])?;
    c.g[V1][0] = 3;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = sx(oc917_at(c, A0, 0x1618));
    c.sw(oc917_at(c, S0, 0xc0), c.g[V1][0]);
    c.sw(oc917_at(c, S0, 0xc4), c.g[V0][0]);
    call_ptr(c, 0x0031_7670, ras[2])?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    OC917_CFG_OPTION_TEXT.call(c, ras[3])
}

/// How `cFEUseExistingAccount_v31` leaves: `Zero` through the shared
/// `return 0` tail (one extra cycle), `Base` after the base handler's call,
/// `Early` straight to the epilogue (the request switch found nothing).
enum Oc917Exit {
    Zero,
    Base,
    Early,
}

/// `int cFEUseExistingAccount::OnNetEvent(self, int id)` (0x1acef8): handle
/// the account events: 201 asks the lobby manager by the pending request
/// (2, 3 or 4); 207 and 210 show an OK popup (mail account / password
/// message); 212 shows a Yes/No "save account" popup; any other id goes to
/// the base state's handler (its result is returned). Everything else
/// returns 0.
pub fn cfe_use_existing_account_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xcf;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        let is_207 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let id = c.g[A1][0];
        let exit: Oc917Exit;
        if is_207 {
            oc917_info_popup(
                c,
                21,
                0x2d98,
                [0x001a_cfc8, 0x001a_cfe4, 0x001a_cffc, 0x001a_d00c],
            )?;
            c.g[V0][0] = 0x811;
            oc917_show_popup_tail(c)?;
            exit = Oc917Exit::Zero;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((id as i64) < 208) as u64;
            let ge_208 = c.g[V0][0] == 0;
            c.g[V0][0] = 0xc9;
            if !ge_208 {
                c.cycles += 2;
                if id == c.g[V0][0] {
                    c.g[V1][0] = c.lw(oc917_at(c, S2, 0x6d8));
                    exit = oc917_request(c)?;
                } else {
                    c.cycles += 2;
                    exit = Oc917Exit::Base;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0xd2;
                let is_210 = id == c.g[V0][0];
                c.g[V0][0] = 0xd4;
                if is_210 {
                    oc917_info_popup(
                        c,
                        20,
                        0x2db0,
                        [0x001a_d01c, 0x001a_d038, 0x001a_d050, 0x001a_d060],
                    )?;
                    c.g[V0][0] = 0x813;
                    oc917_show_popup_tail(c)?;
                    exit = Oc917Exit::Zero;
                } else {
                    c.cycles += 2;
                    if id == c.g[V0][0] {
                        oc917_save_account_popup(c)?;
                        exit = Oc917Exit::Zero;
                    } else {
                        c.cycles += 2;
                        exit = Oc917Exit::Base;
                    }
                }
            }
        }
        match exit {
            Oc917Exit::Base => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                OC917_HANDLE_NET_EVENT.call(c, 0x001a_d110)?;
                frame.restore(c, S0, 0x30);
            }
            Oc917Exit::Early => {
                frame.restore(c, S0, 0x30);
            }
            Oc917Exit::Zero => {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Event 201 of `cFEUseExistingAccount_v31`: switch on the pending request
/// in `v1` (`self->0x6d8`): 2 sends the user name request (0x25c3f8), 3 the
/// email request (0x25c1e0), 4 the cancel (0x25c610) to the lobby manager;
/// other values do nothing.
fn oc917_request(c: &mut Cpu) -> Result<Oc917Exit, crate::call::Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let is_3 = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < 4) as u64;
    if is_3 {
        c.cycles += 5;
        c.g[A1][0] = c.lw(oc917_at(c, S2, 0x6dc));
        c.g[A0][0] = c.lw(oc917_gp(c, -0xc8));
        call_ptr(c, 0x0025_c1e0, 0x001a_cfa8)?;
        c.g[V0][0] = 0;
        return Ok(Oc917Exit::Zero);
    }
    c.cycles += 2;
    let ge_4 = c.g[V0][0] == 0;
    c.g[V0][0] = 2;
    if !ge_4 {
        c.cycles += 2;
        let is_2 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0;
        if is_2 {
            c.cycles += 5;
            c.g[A0][0] = c.lw(oc917_gp(c, -0xc8));
            c.g[A1][0] = c.lw(oc917_at(c, A0, 0x48));
            call_ptr(c, 0x0025_c3f8, 0x001a_cf94)?;
            c.g[V0][0] = 0;
            return Ok(Oc917Exit::Zero);
        }
        c.cycles += 2;
        return Ok(Oc917Exit::Early);
    }
    c.cycles += 3;
    c.g[V0][0] = 4;
    let is_4 = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 0;
    if is_4 {
        c.cycles += 4;
        c.g[A0][0] = c.lw(oc917_gp(c, -0xc8));
        call_ptr(c, 0x0025_c610, 0x001a_cfb8)?;
        c.g[V0][0] = 0;
        return Ok(Oc917Exit::Zero);
    }
    c.cycles += 2;
    Ok(Oc917Exit::Early)
}

/// Event 212: the Yes/No "save account" popup.
fn oc917_save_account_popup(c: &mut Cpu) -> Flow {
    c.cycles += 35;
    c.g[A0][0] = c.g[S2][0];
    OC917_GET_POPUP.call(c, 0x001a_d084)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x0046_0000;
    c.g[V0][0] = 2;
    c.g[A0][0] = sx(oc917_at(c, A0, 0).wrapping_sub(0x2338));
    c.sw(oc917_at(c, S0, 0xbc), c.g[V0][0]);
    c.g[S1][0] = sx(oc917_at(c, S0, 0xbc));
    call_ptr(c, 0x0031_7670, 0x001a_d0a0)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S1][0];
    OC917_CFG_OPTION_TEXT.call(c, 0x001a_d0b0)?;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = sx(oc917_at(c, A0, 0).wrapping_sub(0x328));
    call_ptr(c, 0x0031_7670, 0x001a_d0bc)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    OC917_CFG_OPTION_TEXT.call(c, 0x001a_d0cc)?;
    c.g[V0][0] = 0x81e;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = sx(oc917_at(c, A0, 0x2d80));
    c.sw(oc917_at(c, S0, 0x150), c.g[V0][0]);
    call_ptr(c, 0x0031_7670, 0x001a_d0e0)?;
    c.g[V1][0] = 3;
    c.sw(oc917_at(c, S0, 0x168), 0);
    c.sw(oc917_at(c, S0, 0x164), 0);
    c.g[A0][0] = c.g[S2][0];
    c.sw(oc917_at(c, S0, 0xc4), c.g[V0][0]);
    c.g[A1][0] = c.g[S0][0];
    c.sw(oc917_at(c, S0, 0xc0), c.g[V1][0]);
    OC917_SHOW_POPUP.call(c, 0x001a_d100)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// `opts[opt] == 0 || locked` (`locked` is `s3`): the `a3` flag of a
/// `setEnabled` call of the settings lists. `opt` is the option word already
/// read; charges the cycles of the three-way branch.
fn oc917_option_flag(c: &mut Cpu, opt: u64) -> bool {
    let locked = c.g[S3][0];
    c.cycles += 2;
    if opt == 0 {
        return true;
    }
    if locked == 0 {
        return false;
    }
    c.cycles += 2;
    true
}

/// One list pair of `SetSettingsLocked` whose widget is switched before the
/// list (`ras`: hash, lookup, widget static, list static, list vcall, widget
/// vcall): look the widget up by the string at `0x460000 + str_off`; if
/// found, set widget and list (`self + lb_off`) static to `on` (`s2`), and
/// when `on` is 0 re-enable both unless the option word `opts[opt]` is 0 or
/// the game is ranked (`s3`).
fn oc917_pair_block(c: &mut Cpu, str_off: u32, lb_off: u32, opt: u32, ras: [u32; 6]) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx(oc917_at(c, A0, str_off));
    call_ptr(c, 0x0031_7670, ras[0])?;
    c.g[A0][0] = c.lw(oc917_at(c, S1, 0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ras[1])?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0;
    c.g[A1][0] = c.g[S2][0];
    let mut skip_inc = false;
    if !missing {
        c.cycles += 14;
        oc917_vslot(c, S0, 0x40, ras[2])?;
        c.g[A2][0] = c.lw(oc917_at(c, S1, lb_off));
        c.g[A1][0] = c.g[S2][0];
        oc917_vslot(c, A2, 0x40, ras[3])?;
        let is_on = c.g[S2][0] != 0;
        c.g[A0][0] = 0x0046_0000;
        if is_on {
            skip_inc = true;
        } else {
            // The list first.
            c.cycles += 9;
            c.g[A1][0] = c.lw(oc917_at(c, S1, lb_off));
            c.g[A3][0] = 0;
            c.g[V0][0] = c.lw(oc917_at(c, S1, 0x6d8));
            c.g[V1][0] = c.lw(oc917_at(c, A1, 8));
            c.g[A0][0] = c.lw(oc917_at(c, V0, opt));
            c.g[V0][0] = c.lh(oc917_at(c, V1, 0x40));
            c.g[A2][0] = sx(oc917_at(c, V1, 0x44));
            let opt_word = c.g[A0][0];
            c.g[A1][0] = sx(oc917_at(c, A1, 0).wrapping_add(c.g[V0][0] as u32));
            if oc917_option_flag(c, opt_word) {
                c.g[A3][0] = 1;
            }
            c.g[V0][0] = c.lw(oc917_at(c, A2, 0));
            c.cycles += 11;
            c.g[A0][0] = c.g[A1][0];
            let target = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[A3][0];
            call_ptr(c, target, ras[4])?;
            // Then the widget.
            c.g[A0][0] = c.lw(oc917_at(c, S0, 8));
            c.g[A3][0] = 0;
            c.g[A1][0] = c.lw(oc917_at(c, S1, 0x6d8));
            c.g[V0][0] = c.lh(oc917_at(c, A0, 0x40));
            c.g[A2][0] = sx(oc917_at(c, A0, 0x44));
            c.g[V1][0] = c.lw(oc917_at(c, A1, opt));
            let opt_word = c.g[V1][0];
            c.g[A0][0] = sx(oc917_at(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
            if oc917_option_flag(c, opt_word) {
                c.g[A3][0] = 1;
            }
            c.g[V0][0] = c.lw(oc917_at(c, A2, 0));
            c.cycles += 2;
            let target = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[A3][0];
            call_ptr(c, target, ras[5])?;
        }
    }
    if !skip_inc {
        c.cycles += 1;
        c.g[A0][0] = 0x0046_0000;
    }
    Ok(())
}

/// `void cFEStateChallengePopup::SetSettingsLocked(self, int on)`
/// (0x1aec60): with the screen present, grey out (`on`) or re-enable the
/// game settings widgets: ranked game, number of CPU riders, power-ups,
/// multipliers, point icons, and the event text, together with the matching
/// lists. Re-enabling honours the per-option "fixed" words and the network
/// interface's ranked flag.
pub fn cfe_state_challenge_popup_set_settings_locked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(oc917_at(c, S1, 0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !no_screen {
            // Ranked game: widget only.
            c.cycles += 15;
            c.g[V0][0] = c.lw(oc917_gp(c, -0x848));
            c.g[A1][0] = 7;
            c.g[A0][0] = c.lw(oc917_at(c, V0, 0x78));
            call_ptr(c, 0x0014_dd58, 0x001a_ec98)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = 0x0046_0000;
            c.g[S3][0] = c.lw(oc917_at(c, V1, 0x4b34));
            c.g[A0][0] = sx(oc917_at(c, A0, 0x32a8));
            call_ptr(c, 0x0031_7670, 0x001a_ecac)?;
            c.g[A0][0] = c.lw(oc917_at(c, S1, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_ecb8)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A1][0] = c.g[S2][0];
            if !missing {
                c.cycles += 5;
                oc917_vslot(c, S0, 0x40, 0x001a_ecd8)?;
            }

            // Number of CPU riders: the list is switched before the widget.
            c.cycles += 9;
            c.g[A0][0] = 0x0046_0000;
            c.g[A0][0] = sx(oc917_at(c, A0, 0x32b8));
            call_ptr(c, 0x0031_7670, 0x001a_ece4)?;
            c.g[A0][0] = c.lw(oc917_at(c, S1, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_ecf0)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A1][0] = c.g[S2][0];
            let mut skip_inc = false;
            if !missing {
                c.cycles += 14;
                c.g[A2][0] = c.lw(oc917_at(c, S1, 0x6e0));
                oc917_vslot(c, A2, 0x40, 0x001a_ed14)?;
                c.g[A1][0] = c.g[S2][0];
                oc917_vslot(c, S0, 0x40, 0x001a_ed2c)?;
                if c.g[S2][0] != 0 {
                    c.g[A0][0] = 0x0046_0000;
                    skip_inc = true;
                } else {
                    // The widget first.
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(oc917_at(c, S0, 8));
                    c.g[A3][0] = 0;
                    c.g[A1][0] = c.lw(oc917_at(c, S1, 0x6d8));
                    c.g[V0][0] = c.lh(oc917_at(c, A0, 0x40));
                    c.g[A2][0] = sx(oc917_at(c, A0, 0x44));
                    c.g[V1][0] = c.lw(oc917_at(c, A1, 0));
                    let opt_word = c.g[V1][0];
                    c.g[A0][0] = sx(oc917_at(c, S0, 0).wrapping_add(c.g[V0][0] as u32));
                    if oc917_option_flag(c, opt_word) {
                        c.g[A3][0] = 1;
                    }
                    c.g[V0][0] = c.lw(oc917_at(c, A2, 0));
                    c.cycles += 11;
                    let target = c.g[V0][0] as u32;
                    c.g[A1][0] = c.g[A3][0];
                    call_ptr(c, target, 0x001a_ed6c)?;
                    // Then the list.
                    c.g[A1][0] = c.lw(oc917_at(c, S1, 0x6e0));
                    c.g[A3][0] = 0;
                    c.g[V0][0] = c.lw(oc917_at(c, S1, 0x6d8));
                    c.g[V1][0] = c.lw(oc917_at(c, A1, 8));
                    c.g[A0][0] = c.lw(oc917_at(c, V0, 0));
                    c.g[V0][0] = c.lh(oc917_at(c, V1, 0x40));
                    c.g[A2][0] = sx(oc917_at(c, V1, 0x44));
                    let opt_word = c.g[A0][0];
                    c.g[A1][0] = sx(oc917_at(c, A1, 0).wrapping_add(c.g[V0][0] as u32));
                    let force = oc917_option_flag(c, opt_word);
                    c.g[A0][0] = c.g[A1][0];
                    if force {
                        c.g[A3][0] = 1;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(oc917_at(c, A2, 0));
                    let target = c.g[V0][0] as u32;
                    c.g[A1][0] = c.g[A3][0];
                    call_ptr(c, target, 0x001a_edac)?;
                }
            }
            if !skip_inc {
                c.cycles += 1;
                c.g[A0][0] = 0x0046_0000;
            }

            // Power-ups, multipliers, point icons: widget first, then list.
            oc917_pair_block(
                c,
                0x32d0,
                0x6e4,
                4,
                [
                    0x001a_edb8,
                    0x001a_edc4,
                    0x001a_ede4,
                    0x001a_ee00,
                    0x001a_ee48,
                    0x001a_ee80,
                ],
            )?;
            oc917_pair_block(
                c,
                0x32e0,
                0x6e8,
                0xc,
                [
                    0x001a_ee8c,
                    0x001a_ee98,
                    0x001a_eeb8,
                    0x001a_eed4,
                    0x001a_ef1c,
                    0x001a_ef54,
                ],
            )?;
            oc917_pair_block(
                c,
                0x32f0,
                0x6ec,
                8,
                [
                    0x001a_ef60,
                    0x001a_ef6c,
                    0x001a_ef8c,
                    0x001a_efa8,
                    0x001a_eff0,
                    0x001a_f028,
                ],
            )?;

            // The event text: the widget only.
            c.cycles += 8;
            c.g[A0][0] = sx(oc917_at(c, A0, 0x3130));
            call_ptr(c, 0x0031_7670, 0x001a_f034)?;
            c.g[A0][0] = c.lw(oc917_at(c, S1, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_f040)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A1][0] = c.g[S2][0];
            if !missing {
                c.cycles += 5;
                oc917_vslot(c, S0, 0x40, 0x001a_f060)?;
            }
            // The ranked-game list.
            c.cycles += 7;
            c.g[A2][0] = c.lw(oc917_at(c, S1, 0x6dc));
            c.g[A1][0] = c.g[S2][0];
            oc917_vslot(c, A2, 0x40, 0x001a_f07c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Set the text of the object in `s1` to a string: `setAsciiString`
/// (0x3a0c50) with `a1 = 0x4a0000 + off`, the shared "look-up and set"
/// step of `FillChallengeText`.
fn oc917_set_ascii_const(c: &mut Cpu, off: u32, ra: u32) -> Flow {
    c.g[A1][0] = 0x004a_0000;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = sx(oc917_at(c, A1, off));
    call_ptr(c, 0x003a_0c50, ra)
}

/// `void cFEStateChallengePopup::FillChallengeText(self, iface, screen)`
/// (0x1af728): with a screen, fill its "TextPeak", "TextEvent" and
/// "TextCourse" labels: from the new-race interface `iface` (the peak name
/// by location peak 0..2, the event name, the location name) or with the
/// blank string when there is none. When an event is set (and the debug
/// byte at 0x535c12 is non-zero) the CPU-rider count is reset first.
pub fn cfe_state_challenge_popup_fill_challenge_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        let no_screen = c.g[S3][0] == 0;
        frame.save_ra(c, 0);
        if no_screen {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        } else {
            // "TextPeak".
            c.cycles += 8;
            c.g[A0][0] = 0x0046_0000;
            c.g[A0][0] = sx(oc917_at(c, A0, 0x34b8));
            call_ptr(c, 0x0031_7670, 0x001a_f760)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0039_d860, 0x001a_f76c)?;
            let have_iface = c.g[S2][0] != 0;
            c.g[S1][0] = c.g[V0][0];
            if !have_iface {
                c.cycles += 6;
                oc917_set_ascii_const(c, 0x1390, 0x001a_f784)?;
                c.g[A0][0] = 0x0046_0000;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0014_4c98, 0x001a_f794)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                let peak_1 = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
                if peak_1 {
                    c.cycles += 6;
                    oc917_set_ascii_const(c, 0x1500, 0x001a_f7f0)?;
                    c.g[A0][0] = 0x0046_0000;
                } else {
                    c.cycles += 2;
                    let peak_ge_2 = c.g[V0][0] == 0;
                    c.g[V0][0] = 2;
                    if !peak_ge_2 {
                        c.cycles += 2;
                        let peak_0 = c.g[V1][0] == 0;
                        c.g[A1][0] = 0x004a_0000;
                        if peak_0 {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx(oc917_at(c, A1, 0x14f8));
                            call_ptr(c, 0x003a_0c50, 0x001a_f7d8)?;
                        } else {
                            c.cycles += 2;
                        }
                        c.g[A0][0] = 0x0046_0000;
                    } else {
                        c.cycles += 2;
                        let peak_2 = c.g[V1][0] == c.g[V0][0];
                        c.g[A1][0] = 0x004a_0000;
                        if peak_2 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = sx(oc917_at(c, A1, 0x1508));
                            call_ptr(c, 0x003a_0c50, 0x001a_f804)?;
                            c.g[A0][0] = 0x0046_0000;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = 0x0046_0000;
                        }
                    }
                }
            }

            // "TextEvent".
            c.cycles += 7;
            c.g[A0][0] = sx(oc917_at(c, A0, 0x34c8));
            call_ptr(c, 0x0031_7670, 0x001a_f810)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0039_d860, 0x001a_f81c)?;
            let have_iface = c.g[S2][0] != 0;
            c.g[S1][0] = c.g[V0][0];
            if !have_iface {
                c.cycles += 6;
                oc917_set_ascii_const(c, 0x1390, 0x001a_f834)?;
                c.g[A0][0] = 0x0046_0000;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x0053_0000;
                c.g[V1][0] = c.lb(oc917_at(c, V0, 0x5c12));
                if c.g[V1][0] != 0 {
                    // An event is set: reset the AI count and its list.
                    c.cycles += 24;
                    call_ptr(c, 0x0014_dc80, 0x001a_f854)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x001a_f860)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0014_4d98, 0x001a_f870)?;
                    c.g[V1][0] = c.lw(oc917_at(c, S0, 0xc));
                    c.g[A0][0] = c.lh(oc917_at(c, V1, 8));
                    c.g[V0][0] = c.lw(oc917_at(c, V1, 0xc));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc917_at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001a_f884)?;
                    c.g[A0][0] = c.lw(oc917_at(c, S4, 0x6e0));
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0039_a7a8, 0x001a_f890)?;
                    c.g[A2][0] = c.lw(oc917_at(c, S4, 0x6e0));
                    c.g[A1][0] = 1;
                    oc917_vslot(c, A2, 0x40, 0x001a_f8ac)?;
                }
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0014_5340, 0x001a_f8b4)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001a_f8c0)?;
                c.g[A0][0] = 0x0046_0000;
            }

            // "TextCourse".
            c.cycles += 7;
            c.g[A0][0] = sx(oc917_at(c, A0, 0x34d8));
            call_ptr(c, 0x0031_7670, 0x001a_f8cc)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_f8d8)?;
            let have_iface = c.g[S2][0] != 0;
            c.g[S1][0] = c.g[V0][0];
            if !have_iface {
                c.cycles += 6;
                oc917_set_ascii_const(c, 0x1390, 0x001a_f8f0)?;
                frame.restore(c, S0, 0x50);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0014_4bf0, 0x001a_f900)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001a_f90c)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (state of the machine's own RAM, so
    /// other tests in the binary keep the real callees), the result mask
    /// and the list size.
    pub(crate) const OC917_CTRL: u32 = DATA + 0x5000;
    pub(crate) const OC917_MAGIC: u32 = 0x0917_0917;
    /// Eight objects 0x40 apart sharing one vtable; the "be interface".
    pub(crate) const OC917_OBJ: u32 = DATA + 0x2000;
    pub(crate) const OC917_VT: u32 = DATA + 0x3000;
    pub(crate) const OC917_VT2: u32 = DATA + 0x3400;
    pub(crate) const OC917_BE: u32 = DATA + 0x3800;
    pub(crate) const OC917_OPTS: u32 = DATA + 0x4000;
    pub(crate) const OC917_ITEMS: u32 = DATA + 0x4800;
    pub(crate) const OC917_NAME: u32 = DATA + 0x4c00;
    pub(crate) const OC917_SESSION: u32 = DATA + 0x5800;
    pub(crate) const OC917_LOBBY: u32 = DATA + 0x6800;
    pub(crate) const OC917_WORLD: u32 = DATA + 0x7000;
    pub(crate) const OC917_POPUP: u32 = DATA + 0x8000;
    pub(crate) const OC917_SCREEN: u32 = DATA + 0x9000;
    pub(crate) const OC917_SELF: u32 = DATA + 0x1_0000;

    /// The miss hook: log the call as the fakes do, then give the callees
    /// whose result steers the code results from the fixture (list count,
    /// items, lookups, booleans), picked by call number and the mask.
    pub(crate) fn oc917_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let a1 = c.g[A1][0] as u32;
        fakes::fake_call(c, target);
        if c.pc != ra {
            return;
        }
        let mask = c.bus.read32(OC917_CTRL + 8);
        let bit = ((mask >> (n % 32)) & 1) as u64;
        match target {
            0x0026_2938 => c.g[V0][0] = c.bus.read32(OC917_CTRL + 4) as u64,
            0x0026_3630 => {
                c.g[V0][0] = if bit != 0 {
                    (OC917_ITEMS + 0x40 * (a1 & 3)) as u64
                } else {
                    0
                }
            }
            0x0041_aa88 => c.g[V0][0] = bit ^ 1,
            0x0026_1460 | 0x0026_13c8 => c.g[V0][0] = bit,
            0x0039_d860 => {
                c.g[V0][0] = if bit != 0 {
                    (OC917_OBJ + 0x40 * (n % 8)) as u64
                } else {
                    0
                }
            }
            0x0014_4c98 => c.g[V0][0] = (((mask >> (n % 29)) & 7) % 5) as u64,
            0x0014_dd58 => c.g[V0][0] = OC917_BE as u64,
            _ => {}
        }
    }

    /// A fake for a same-shard callee with an override slot: logs the call
    /// and returns to `ra`; `GetPopup` also returns the popup object. With
    /// the gate off it runs the body the slot held.
    macro_rules! oc917_gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path, $result:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC917_CTRL) != OC917_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake_call(c, $target);
                if c.pc == ra {
                    if let Some(v) = $result {
                        c.g[V0][0] = v;
                    }
                }
            }
        };
    }

    oc917_gated_fake!(
        oc917_fake_add_message,
        OC917_PREV_ADD,
        0x001a_7dd8,
        ee::s03::f_001a7dd8__generated,
        None::<u64>
    );
    oc917_gated_fake!(
        oc917_fake_get_popup,
        OC917_PREV_GET,
        0x001a_9c50,
        ee::s03::f_001a9c50__generated,
        Some(OC917_POPUP as u64)
    );
    oc917_gated_fake!(
        oc917_fake_show_popup,
        OC917_PREV_SHOW,
        0x001a_9bc0,
        ee::s03::f_001a9bc0__generated,
        None::<u64>
    );
    oc917_gated_fake!(
        oc917_fake_cfg_option,
        OC917_PREV_CFG,
        0x001c_57c0,
        ee::s03::f_001c57c0__generated,
        None::<u64>
    );
    oc917_gated_fake!(
        oc917_fake_handle_event,
        OC917_PREV_HANDLE,
        0x001a_8e40,
        ee::s03::f_001a8e40__generated,
        None::<u64>
    );

    /// Hook the callee slots once per process (each fake is gated on
    /// [`OC917_CTRL`]).
    pub(crate) fn oc917_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A7DD8),
            oc917_fake_add_message,
            &OC917_PREV_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9C50),
            oc917_fake_get_popup,
            &OC917_PREV_GET,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9BC0),
            oc917_fake_show_popup,
            &OC917_PREV_SHOW,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C57C0),
            oc917_fake_cfg_option,
            &OC917_PREV_CFG,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A8E40),
            oc917_fake_handle_event,
            &OC917_PREV_HANDLE,
        );
    }

    /// The shared fixture: fakes and miss hook, the object vtables, the
    /// session, the lobby manager and the world the code reads through
    /// `gp`.
    pub(crate) fn oc917_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc917_install_fakes();
        let mut r = fakes::machine_rt(c, seed, salt, calls);
        c.hooks.miss = oc917_miss;
        c.bus.write32(OC917_CTRL, OC917_MAGIC);
        c.bus.write32(OC917_CTRL + 4, seed % 4);
        c.bus.write32(OC917_CTRL + 8, r.next_u32());
        for k in 0..8 {
            c.bus.write32(OC917_OBJ + 0x40 * k + 8, OC917_VT);
        }
        for (off, f) in [(0x40, 0x10), (0x48, 0x20), (0x58, 0x30)] {
            c.bus.write16(OC917_VT + off, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(OC917_VT + off + 4, fakes::FAKE + f);
        }
        c.bus.write32(OC917_BE + 0xc, OC917_VT2);
        c.bus.write16(OC917_VT2 + 8, 0);
        c.bus.write32(OC917_VT2 + 0xc, fakes::FAKE + 0x40);
        c.bus.write32(
            GAME_GP + 0x238,
            if seed % 7 == 0 { 0 } else { OC917_SESSION },
        );
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), OC917_LOBBY);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC917_WORLD);
        r
    }

    fn oc917_load_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let r = oc917_base(c, seed, 0x917_1, calls);
        c.bus.write32(OC917_SELF + 0x6d0, OC917_NAME);
        c.pc = 0x001a_7f78;
        c.g[A0][0] = sx(OC917_SELF);
        r
    }

    /// `LoadMessages`: no, one, two and three session messages, null and
    /// live sessions, matching and other senders, the unwinds of every call.
    #[test]
    fn cfe_state_read_message_popup_load_messages_matches() {
        covered(
            ee::s03::f_001a7f78__generated,
            cfe_state_read_message_popup_load_messages,
            400,
            &|c, seed| {
                oc917_load_setup(c, seed, 24);
            },
        );
    }

    driven!(oc917_gen_load, ee::s03::f_001a7f78__generated);
    driven!(oc917_hand_load, cfe_state_read_message_popup_load_messages);

    /// The same loops with events delivered (and unwinding) at their back
    /// edges: the unwound run re-enters through the generated body.
    #[test]
    fn cfe_state_read_message_popup_load_messages_events() {
        check_events(oc917_gen_load, oc917_hand_load, 150, &|c, seed| {
            let mut r = oc917_load_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x001a_7f78)
        });
    }

    fn oc917_quality_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        const PINGS: [i32; 18] = [
            0, 150, 200, 201, 250, 299, 300, 301, 350, 399, 400, 401, 450, 499, 500, 501, 900, -5,
        ];
        let r = oc917_base(c, seed, 0x917_2, calls);
        c.pc = 0x001a_ad50;
        c.g[A0][0] = if seed % 9 == 0 { 0 } else { sx(OC917_SCREEN) };
        c.g[A1][0] = sx(seed % 6);
        c.g[A2][0] = sx(PINGS[seed as usize % PINGS.len()] as u32);
        // The bar objects the lookups may not overwrite.
        let frame = (c.g[SP][0] as u32).wrapping_sub(0x100);
        for i in 0..5 {
            c.bus
                .write32(frame + 0x30 + 4 * i, OC917_OBJ + 0x40 * (i % 8));
        }
        r
    }

    /// `SetQualityIcons`: every ping range edge (200, 300, 400, 500 and
    /// the open ends), a null screen, lookups that find or miss the bars,
    /// and the unwinds of the calls.
    #[test]
    fn net_fe_set_quality_icons_matches() {
        covered(
            ee::s03::f_001aad50__generated,
            net_fe_set_quality_icons,
            360,
            &|c, seed| {
                oc917_quality_setup(c, seed, 30);
            },
        );
    }

    driven!(oc917_gen_quality, ee::s03::f_001aad50__generated);
    driven!(oc917_hand_quality, net_fe_set_quality_icons);

    /// The lookup loop and the show/hide loop with events at their back
    /// edges.
    #[test]
    fn net_fe_set_quality_icons_events() {
        check_events(oc917_gen_quality, oc917_hand_quality, 150, &|c, seed| {
            let mut r = oc917_quality_setup(c, seed, 0);
            schedule(c, &mut r, seed, 0x001a_ad50)
        });
    }

    /// `cFEUseExistingAccount_v31`: every event id (201 with requests 2, 3,
    /// 4, none and out of range; 207; 210; 212; the base handler) and the
    /// unwinds of the popup calls.
    #[test]
    fn cfe_use_existing_account_v31_matches() {
        const IDS: [u32; 12] = [
            0xc9, 0xcf, 0xd2, 0xd4, 0, 0x7f, 0xca, 0xce, 0xd0, 0xd3, 0xd5, 0x1000,
        ];
        const REQS: [u32; 7] = [0, 1, 2, 3, 4, 5, 0xffff_ffff];
        covered(
            ee::s03::f_001acef8__generated,
            cfe_use_existing_account_v31,
            336,
            &|c, seed| {
                let _r = oc917_base(c, seed, 0x917_3, 12);
                c.pc = 0x001a_cef8;
                c.g[A0][0] = sx(OC917_SELF);
                c.g[A1][0] = IDS[seed as usize % IDS.len()] as u64;
                c.bus.write32(
                    OC917_SELF + 0x6d8,
                    REQS[(seed as usize / IDS.len()) % REQS.len()],
                );
                c.bus.write32(OC917_SELF + 0x6dc, 0x0123_4567);
                c.bus.write32(OC917_LOBBY + 0x48, 0x89ab_cdef);
            },
        );
    }

    /// `SetSettingsLocked`: a missing screen, on and off, the ranked flag,
    /// the per-option words, widgets found or missing, and the unwinds.
    #[test]
    fn cfe_state_challenge_popup_set_settings_locked_matches() {
        covered(
            ee::s03::f_001aec60__generated,
            cfe_state_challenge_popup_set_settings_locked,
            600,
            &|c, seed| {
                let mut r = oc917_base(c, seed, 0x917_4, 60);
                c.pc = 0x001a_ec60;
                c.g[A0][0] = sx(OC917_SELF);
                c.g[A1][0] = sx(seed % 2);
                c.bus.write32(
                    OC917_SELF + 0x40,
                    if seed % 11 == 0 { 0 } else { OC917_SCREEN },
                );
                c.bus.write32(OC917_SELF + 0x6d8, OC917_OPTS);
                for i in 0..4 {
                    let w = r.next_u32();
                    c.bus
                        .write32(OC917_OPTS + 4 * i, if w & 1 == 0 { 0 } else { w });
                }
                for (i, off) in [0x6dc, 0x6e0, 0x6e4, 0x6e8, 0x6ec].into_iter().enumerate() {
                    c.bus
                        .write32(OC917_SELF + off, OC917_OBJ + 0x40 * (i as u32 + 1));
                }
                c.bus.write32(0x0053_4b34, (seed / 2) % 2);
            },
        );
    }

    /// `FillChallengeText`: no screen, no interface, every location peak
    /// (0, 1, 2 and out of range), the event-set branch with the AI reset,
    /// and the unwinds.
    #[test]
    fn cfe_state_challenge_popup_fill_challenge_text_matches() {
        covered(
            ee::s03::f_001af728__generated,
            cfe_state_challenge_popup_fill_challenge_text,
            480,
            &|c, seed| {
                let _r = oc917_base(c, seed, 0x917_5, 24);
                c.pc = 0x001a_f728;
                c.g[A0][0] = sx(OC917_SELF);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(OC917_BE) };
                c.g[A2][0] = if seed % 7 == 0 { 0 } else { sx(OC917_SCREEN) };
                c.bus.write32(OC917_SELF + 0x6e0, OC917_OBJ);
                c.bus.write8(0x0053_5c12, ((seed / 3) % 2) as u8);
            },
        );
    }
}
