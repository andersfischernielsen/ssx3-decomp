//! Staging module oc1128 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1128`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_6070,
        fe_state_net_messenger_show_buddy_info,
        "cFEStateNetMessenger_ShowBuddyInfo",
    ),
    (
        0x003d_5a98,
        path_track_update_volume_fade,
        "PATH_TrackUpdateVolumeFade",
    ),
    (
        0x003d_5cc0,
        path_track_update_aux_send_fade,
        "PATH_TrackUpdateAuxSendFade",
    ),
];

/// Offset of the screen pointer in a `cFEStateNetMessenger`.
pub(crate) const SBI_SCREEN: u32 = 0x40;
/// Offset of the buddy-name vector (`mpStart`, `mpFinish`) in the messenger.
pub(crate) const SBI_NAMES_START: u32 = 0x6e8;
pub(crate) const SBI_NAMES_FINISH: u32 = 0x6ec;
/// Offset of the shown-name string in the messenger.
pub(crate) const SBI_SHOWN_NAME: u32 = 0x6e0;
/// `gp + 0x238`: the buddy session; `gp - 0xc8`: the lobby (reconnect target).
const SBI_GP_SESSION: u32 = 0x238;

/// `lw` of `gp + off` (sign-extended).
fn sbi_gp_lw(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `lw` of `gp - off`.
fn sbi_gp_lw_neg(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// The first half of the inlined `GetSession()` (create and reconnect the
/// buddy session when needed), starting at the block `s`: when the session
/// pointer in `v0`/`v1` was null (`need_create`) call `BuddyMan_Create`,
/// then `BuddyMan_IsConnected(session)` and, when that fails,
/// `BuddyMan_Reconnect(session)`. Leaves `v0 = session`.
fn sbi_session_connect(c: &mut Cpu, s: u32, need_create: bool) -> Flow {
    if need_create {
        c.cycles += 2;
        call_ptr(c, 0x0026_0f80, s + 0x8)?;
    }
    c.cycles += 4;
    c.g[A0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    call_ptr(c, 0x0026_1460, s + 0x10)?;
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
        call_ptr(c, 0x0026_13c8, s + 0x20)?;
        c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    }
    Ok(())
}

/// The second half of the inlined `GetSession()`: when the session is still
/// null, `LobbyMan_Disconnect(lobby)`. The session is reloaded into `dest`
/// (`v0` or `a0`: whatever the compiler used after the check).
fn sbi_session_check(c: &mut Cpu, s: u32, dest: usize) -> Flow {
    c.cycles += 2;
    let have = c.g[V0][0] != 0;
    c.g[dest][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    if !have {
        c.cycles += 3;
        c.g[A0][0] = sbi_gp_lw_neg(c, 0xc8);
        call_ptr(c, 0x0025_b800, s + 0x34)?;
        c.g[dest][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    }
    Ok(())
}

/// `GetSession()` in full (both halves), the session ends up in `dest`.
fn sbi_get_session(c: &mut Cpu, s: u32, need_create: bool, dest: usize) -> Flow {
    sbi_session_connect(c, s, need_create)?;
    sbi_session_check(c, s, dest)
}

/// `bool cFEStateNetMessenger::ShowBuddyInfo(int index)` (0x001a6070): fills
/// the buddy panel of the messenger screen: the name and status text objects
/// (`TextBuddyName`, `TextBuddyLocation`) are looked up by hash; with no
/// buddies it shows the "no buddies" / "updating" text; otherwise it resolves
/// the buddy at `index` through the session (`FindRosterEntryNoCase`,
/// `GetRosterEntry`), shows the name and either the presence
/// (`ShowBuddyPresence`) or the "offline" text. Returns whether a buddy was
/// shown. The back edge after a failed roster lookup checks events (resume
/// point 0x001a61e0).
pub fn fe_state_net_messenger_show_buddy_info(c: &mut Cpu) {
    if c.pc != 0x001a_6070 {
        return ee::s03::f_001a6070__generated(c);
    }
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
        let screen = Addr((c.g[S0][0] as u32).wrapping_add(SBI_SCREEN)).lw(c);
        c.g[V0][0] = screen;
        c.g[S1][0] = c.g[A1][0];
        // Which epilogue entry the body leaves through.
        let full_exit = if screen == 0 {
            // 0x1a61e0
            c.cycles += 2;
            c.g[V0][0] = 0;
            true
        } else {
            sbi_show_buddy_body(c)?
        };
        // The early exit restores s0 in a branch delay slot, without the cycle.
        if full_exit {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x50);
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

/// The body of `ShowBuddyInfo` after the screen check (0x1a609c..0x1a63a8).
/// `Ok(true)` leaves through the epilogue entry that restores `s0` (0x1a63a8),
/// `Ok(false)` through the early one (0x1a60dc: `s0` restored in the delay
/// slot of the `s2 == 0` branch).
fn sbi_show_buddy_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 16;
    // s2 = the name text object, s3 = the status text object (looked up by
    // the hash of their names on the screen).
    c.g[A0][0] = 0x0046_17a8;
    call_ptr(c, 0x0031_7670, 0x001a_60a8)?;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SBI_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001a_60b4)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = 0x0046_1558;
    call_ptr(c, 0x0031_7670, 0x001a_60c4)?;
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SBI_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001a_60d0)?;
    c.g[S3][0] = c.g[V0][0];
    let no_name = c.g[S3][0] == 0;
    c.g[V0][0] = 0;
    if no_name {
        return Ok(true);
    }
    // 0x1a60dc
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        return Ok(false);
    }
    // 0x1a60e4: the number of buddies, (finish - start) / 4.
    c.cycles += 6;
    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SBI_NAMES_FINISH)).lw(c);
    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SBI_NAMES_START)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
    let has_buddies = c.g[V0][0] != 0;
    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if !has_buddies {
        return sbi_no_buddies(c);
    }
    // 0x1a61d8: index in range?
    c.cycles += 2;
    let in_range = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    // 0x1a61e8: buddyName = mpStart[index]
    c.cycles += 4;
    c.g[V1][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let have_session = c.g[V1][0] != 0;
    c.g[S1][0] = Addr(c.g[V0][0] as u32).lw(c);
    sbi_get_session(c, 0x001a_61f8, !have_session, A0)?;
    // 0x1a6230: id = FindRosterEntryNoCase(session, buddyName)
    c.cycles += 6;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0026_24f8, 0x001a_6238)?;
    c.g[S4][0] = c.g[V0][0];
    c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    let have_session = c.g[V0][0] != 0;
    sbi_get_session(c, 0x001a_6248, !have_session, A0)?;
    // 0x1a6280: str = GetRosterEntry(session, id)
    c.cycles += 5;
    c.g[A1][0] = c.g[S4][0];
    call_ptr(c, 0x0026_25c8, 0x001a_6288)?;
    c.g[S1][0] = c.g[V0][0];
    let missing = c.g[S1][0] == 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(SBI_SHOWN_NAME));
    if missing {
        // Back edge to 0x1a61e0 (leave with 0).
        if back_edge_unwinds(c, 0x001a_61e0) {
            return Err(crate::call::Unwound);
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    // 0x1a6294: shown name = str; name->SetString(str); setVisible(1) x2.
    c.cycles += 20;
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0031_84e8, 0x001a_629c)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x003a_0c50, 0x001a_62a8)?;
    sbi_set_visible(c, S2, 0x001a_62c0)?;
    sbi_set_visible(c, S3, 0x001a_62d8)?;
    c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    let have_session = c.g[V0][0] != 0;
    sbi_get_session(c, 0x001a_62e4, !have_session, A0)?;
    // 0x1a631c: IsOnlineAt(session, id)
    c.cycles += 4;
    c.g[A1][0] = c.g[S4][0];
    call_ptr(c, 0x0026_3338, 0x001a_6324)?;
    let offline = c.g[V0][0] == 0;
    c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    if offline {
        // 0x1a638c: status->SetTextId(StrHash(offline text)); return true.
        c.cycles += 7;
        c.g[A0][0] = 0x0046_17e0;
        call_ptr(c, 0x0031_7670, 0x001a_6398)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001a_63a4)?;
        c.g[V0][0] = 1;
        return Ok(true);
    }
    // 0x1a632c
    c.cycles += 2;
    let have_session = c.g[V0][0] != 0;
    sbi_get_session(c, 0x001a_6334, !have_session, A0)?;
    // 0x1a636c: ShowBuddyPresence(this, str, IsSameProductAt(session, id))
    c.cycles += 8;
    c.g[A1][0] = c.g[S4][0];
    call_ptr(c, 0x0026_33f8, 0x001a_6374)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x001a_7390, 0x001a_6384)?;
    c.g[V0][0] = 1;
    Ok(true)
}

/// `obj->setVisible(1)`: the virtual call through the vtable slot at `+0x48`
/// (delta) / `+0x4c` (function) of the object's vtable at `obj + 8`.
fn sbi_set_visible(c: &mut Cpu, obj: usize, ret: u32) -> Flow {
    let vt = Addr((c.g[obj][0] as u32).wrapping_add(0x8)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// The "no buddies" arm (0x1a60fc): clears the name text, shows "no buddies"
/// (roster done) or "updating" (and reconnects) on the status text; returns
/// 0.
fn sbi_no_buddies(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 7;
    c.g[A1][0] = 0x004a_1390;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x003a_0c50, 0x001a_610c)?;
    c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    let have_session = c.g[V0][0] != 0;
    sbi_session_connect(c, 0x001a_6118, !have_session)?;
    // 0x1a613c / 0x1a6144 (session in v0)
    sbi_session_check(c, 0x001a_6118, V0)?;
    // 0x1a6150: roster done flag at session + 0xe8
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xe8)).lw(c);
    let updating = c.g[V1][0] == 0;
    c.g[A0][0] = 0x0046_0000;
    if !updating {
        // 0x1a615c: SetTextId(status, "no buddies")
        c.cycles += 8;
        c.g[A0][0] = 0x0046_17b8;
        call_ptr(c, 0x0031_7670, 0x001a_6168)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001a_6174)?;
        c.g[V0][0] = 0;
        return Ok(true);
    }
    // 0x1a617c: SetTextId(status, "updating"), then keep the session alive.
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17c8));
    call_ptr(c, 0x0031_7670, 0x001a_6184)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x003a_1000, 0x001a_6190)?;
    c.g[V0][0] = sbi_gp_lw(c, SBI_GP_SESSION);
    let have_session = c.g[V0][0] != 0;
    sbi_session_connect(c, 0x001a_619c, !have_session)?;
    // 0x1a61c0: a different tail than the other two inlined copies.
    c.cycles += 2;
    let have = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !have {
        c.cycles += 4;
        c.g[A0][0] = sbi_gp_lw_neg(c, 0xc8);
        call_ptr(c, 0x0025_b800, 0x001a_61d0)?;
        c.g[V0][0] = 0;
    }
    Ok(true)
}

/// `PATH_TrackUpdateVolumeFade(track, _, _, a3)` (0x003d5a98): advances the
/// volume fade of an audio track. Reads the fade clock of the global path
/// state (`0x44f420`: `+0x14` duration in ticks, `+0x18` curve in the low
/// 7 bits: 1 linear, 2 quadratic, 3 inverse) and the current time
/// (`0x44f438`); the fade runs from the volume byte at `track+0xc` to the
/// target at `track+0xe`, started at `track+8`. While running it sets the
/// platform volume (`PATH_PlatSetVolume(track->handle(+0x54), vol)`); at
/// the end it sets the final volume and resets the fade fields (`+0x34`,
/// `+0xc`, `+0xe`, `+0xd` = -1, `+8` = 0).
#[allow(unused_variables, unused_labels, unused_assignments, unreachable_code)]
pub fn path_track_update_volume_fade(c: &mut Cpu) {
'b_3d5b10: {
c.cycles += 29;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.g[A1][0] = 0x450000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A3][0] = c.g[A3][0] & 0xffff;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0xbe0));
c.g[A2][0] = 0xffffffffffffff80;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xe));
c.g[T0][0] = 0x1;
c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xc));
c.mtc1(2, 2);
c.cvt_s_w(2, 2);
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 3);
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x14));
c.mtc1(0, 1);
c.g[V0][0] = c.g[V0][0] & 0x7f;
c.sub_s(2, 2, 0);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
c.g[A3][0] = c.g[A3][0] & c.g[A2][0];
c.c_lt_s(2, 1);
let t = !c.fcc();
c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
if t {
break 'b_3d5b10;
} else {
c.cycles += 1;
c.g[T0][0] = 0u64;
break 'b_3d5b10;
}
}
'b_3d5b60: {
'b_3d5b44: {
c.cycles += 9;
c.g[V0][0] = 0x450000;
c.g[A0][0] = 0xffffffffffffff7f;
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbc8));
c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A3][0] = c.g[A3][0] & c.g[A0][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
if t {
break 'b_3d5b44;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
if t {
break 'b_3d5b60;
} else {
break 'b_3d5b44;
}
}
}
c.cycles += 7;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
break 'b_3d5b60;
}
'b_3d5b8c: {
'b_3d5b78: {
c.cycles += 2;
if (c.g[V0][0] as i64) < 0 {
c.g[V0][0] = c.g[V0][0] & 0x1;
break 'b_3d5b78;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;

if t {
break 'b_3d5b8c;
} else {
break 'b_3d5b78;
}
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[A3][0] as u32) >> 17);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
break 'b_3d5b8c;
}
'b_3d5cb0: {
'b_3d5cac: {
'b_3d5bd4: {
c.cycles += 7;
c.div_s(0, 1, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 1);
c.c_le_s(1, 0);
let t = !c.fcc();
c.g[V0][0] = c.g[A3][0] & 0x80;
if t {
break 'b_3d5bd4;
} else {
c.cycles += 11;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
c.g[RA][0] = sx(0x003d5bb4);
c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x34));
c.pc = 0x002af6c0; call_indirect(c, 0x002af6c0);
if c.pc != 0x003d5bb4 { return; }
c.g[V1][0] = 0xffffffffffffffff;
c.g[V0][0] = 0xffffffffffffffff;
c.sb((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0xd), c.g[V1][0]);
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
if t {
break 'b_3d5cac;
} else {
break 'b_3d5bd4;
}
}
}
'b_3d5bdc: {
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.sub_s(0, 1, 0);
break 'b_3d5bdc;
} else {
break 'b_3d5bdc;
}
}
'b_3d5c38: {
'b_3d5c34: {
'b_3d5c24: {
'b_3d5c18: {
c.cycles += 4;
c.g[V1][0] = c.g[A3][0] & 0x7f;
c.g[V0][0] = 0x2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x3u64) as u64;
if t {
break 'b_3d5c18;
} else {
'b_3d5c04: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_3d5c04;
} else {
c.cycles += 2;
if c.g[V1][0] == c.g[V0][0] {
c.sub_s(3, 1, 0);
break 'b_3d5c34;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xc));
if t {
break 'b_3d5c38;
} else {
break 'b_3d5c04;
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = c.g[V1][0] == c.g[V0][0];

if t {
break 'b_3d5c24;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xc));
if t {
break 'b_3d5c38;
} else {
break 'b_3d5c18;
}
}
}
}
c.cycles += 3;
c.mul_s(0, 0, 0);
let t = true;
c.sub_s(3, 1, 0);
if t {
break 'b_3d5c34;
} else {
break 'b_3d5c24;
}
}
c.cycles += 4;
c.div_s(1, 1, 0);
c.g[AT][0] = 0x41c80000;
c.mtc1(1, 0);
c.div_s(3, 1, 0);
break 'b_3d5c34;
}
c.cycles += 1;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xc));
break 'b_3d5c38;
}
'b_3d5c58: {
c.cycles += 6;
c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xe));
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_3d5c58;
} else {
c.cycles += 2;
c.mtc1(3, 1);
c.cvt_s_w(1, 1);
break 'b_3d5c58;
}
}
'b_3d5c6c: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(2, 0);
if c.fcc() {
c.neg_s(2, 2);
break 'b_3d5c6c;
} else {
break 'b_3d5c6c;
}
}
c.cycles += 4;
c.c_le_s(0, 3);
if !c.fcc() {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3d5cb0;
} else {
c.cycles += 6;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.c_le_s(3, 0);
if !c.fcc() {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3d5cb0;
} else {
c.cycles += 6;
c.mul_s(0, 2, 3);
c.add_s(3, 1, 0);
c.cvt_w_s(0, 3);
c.mfc1(5, 0);
c.g[RA][0] = sx(0x003d5cac);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
c.pc = 0x002af6c0; call_indirect(c, 0x002af6c0);
if c.pc != 0x003d5cac { return; }
break 'b_3d5cac;
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3d5cb0;
}
c.cycles += 3;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return;
}

/// `PATH_TrackUpdateAuxSendFade(track, _, _, a3)` (0x003d5cc0): the aux-send
/// twin of `PATH_TrackUpdateVolumeFade`: same fade clock (state `+0x1c`
/// duration, `+0x20` curve) and maths, on the bytes at `track+0x14`
/// (current), `+0x16` (target) and the start time at `+0x10`; it sets the
/// send level with `PATH_PlatSetAuxSend(handle(+0x54), 0, level)`
/// (0x002af4e0) and resets the fade fields at the end.
#[allow(unused_variables, unused_labels, unused_assignments, unreachable_code)]
pub fn path_track_update_aux_send_fade(c: &mut Cpu) {
'b_3d5d38: {
c.cycles += 29;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.g[A1][0] = 0x450000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A3][0] = c.g[A3][0] & 0xffff;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0xbe0));
c.g[A2][0] = 0xffffffffffffff80;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x16));
c.g[T0][0] = 0x1;
c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x14));
c.mtc1(2, 2);
c.cvt_s_w(2, 2);
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 3);
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x1c));
c.mtc1(0, 1);
c.g[V0][0] = c.g[V0][0] & 0x7f;
c.sub_s(2, 2, 0);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
c.g[A3][0] = c.g[A3][0] & c.g[A2][0];
c.c_lt_s(2, 1);
let t = !c.fcc();
c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
if t {
break 'b_3d5d38;
} else {
c.cycles += 1;
c.g[T0][0] = 0u64;
break 'b_3d5d38;
}
}
'b_3d5d88: {
'b_3d5d6c: {
c.cycles += 9;
c.g[V0][0] = 0x450000;
c.g[A0][0] = 0xffffffffffffff7f;
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0xbc8));
c.g[V1][0] = sx((c.g[T0][0] as u32) << 7);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[A3][0] = c.g[A3][0] & c.g[A0][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = (c.g[V0][0] as i64) < 0;
c.g[A3][0] = c.g[A3][0] | c.g[V1][0];
if t {
break 'b_3d5d6c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
if t {
break 'b_3d5d88;
} else {
break 'b_3d5d6c;
}
}
}
c.cycles += 7;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.g[V0][0] = sx((c.g[A3][0] as u32) >> 16);
break 'b_3d5d88;
}
'b_3d5db4: {
'b_3d5da0: {
c.cycles += 2;
if (c.g[V0][0] as i64) < 0 {
c.g[V0][0] = c.g[V0][0] & 0x1;
break 'b_3d5da0;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;

if t {
break 'b_3d5db4;
} else {
break 'b_3d5da0;
}
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[A3][0] as u32) >> 17);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
break 'b_3d5db4;
}
'b_3d5edc: {
'b_3d5ed8: {
'b_3d5df8: {
c.cycles += 7;
c.div_s(0, 1, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 1);
c.c_le_s(1, 0);
let t = !c.fcc();
c.g[A1][0] = 0u64;
if t {
break 'b_3d5df8;
} else {
c.cycles += 10;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
c.g[RA][0] = sx(0x003d5ddc);
c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x16));
c.pc = 0x002af4e0; call_indirect(c, 0x002af4e0);
if c.pc != 0x003d5ddc { return; }
c.g[V1][0] = 0xffffffffffffffff;
c.g[V0][0] = 0xffffffffffffffff;
c.sb((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
c.sb((c.g[S0][0] as u32).wrapping_add(0x15), c.g[V1][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
let t = true;
c.sb((c.g[S0][0] as u32).wrapping_add(0x16), c.g[V1][0]);
if t {
break 'b_3d5ed8;
} else {
break 'b_3d5df8;
}
}
}
'b_3d5e04: {
c.cycles += 3;
c.g[V0][0] = c.g[A3][0] & 0x80;
if c.g[V0][0] != 0u64 {
c.sub_s(0, 1, 0);
break 'b_3d5e04;
} else {
break 'b_3d5e04;
}
}
'b_3d5e60: {
'b_3d5e5c: {
'b_3d5e4c: {
'b_3d5e40: {
c.cycles += 4;
c.g[V1][0] = c.g[A3][0] & 0x7f;
c.g[V0][0] = 0x2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = (c.g[V1][0] < 0x3u64) as u64;
if t {
break 'b_3d5e40;
} else {
'b_3d5e2c: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_3d5e2c;
} else {
c.cycles += 2;
if c.g[V1][0] == c.g[V0][0] {
c.sub_s(3, 1, 0);
break 'b_3d5e5c;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x14));
if t {
break 'b_3d5e60;
} else {
break 'b_3d5e2c;
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = c.g[V1][0] == c.g[V0][0];

if t {
break 'b_3d5e4c;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x14));
if t {
break 'b_3d5e60;
} else {
break 'b_3d5e40;
}
}
}
}
c.cycles += 3;
c.mul_s(0, 0, 0);
let t = true;
c.sub_s(3, 1, 0);
if t {
break 'b_3d5e5c;
} else {
break 'b_3d5e4c;
}
}
c.cycles += 4;
c.div_s(1, 1, 0);
c.g[AT][0] = 0x41c80000;
c.mtc1(1, 0);
c.div_s(3, 1, 0);
break 'b_3d5e5c;
}
c.cycles += 1;
c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_3d5e60;
}
'b_3d5e80: {
c.cycles += 6;
c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x16));
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_3d5e80;
} else {
c.cycles += 2;
c.mtc1(3, 1);
c.cvt_s_w(1, 1);
break 'b_3d5e80;
}
}
'b_3d5e94: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(2, 0);
if c.fcc() {
c.neg_s(2, 2);
break 'b_3d5e94;
} else {
break 'b_3d5e94;
}
}
c.cycles += 4;
c.c_le_s(0, 3);
if !c.fcc() {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3d5edc;
} else {
c.cycles += 6;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.c_le_s(3, 0);
if !c.fcc() {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3d5edc;
} else {
c.cycles += 7;
c.mul_s(0, 2, 3);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
c.add_s(3, 1, 0);
c.cvt_w_s(0, 3);
c.mfc1(6, 0);
c.g[RA][0] = sx(0x003d5ed8);
c.g[A1][0] = 0u64;
c.pc = 0x002af4e0; call_indirect(c, 0x002af4e0);
if c.pc != 0x003d5ed8 { return; }
break 'b_3d5ed8;
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3d5edc;
}
c.cycles += 3;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the gated fakes (in this machine's RAM) and its magic.
    pub(crate) const OC1128_CTRL: u32 = DATA + 0x38_0000;
    pub(crate) const OC1128_MAGIC: u32 = 0x1128_c0de;
    /// Forced results of the faked callees (literal `v0`), one word per slot.
    pub(crate) const OC1128_MAIL: u32 = DATA + 0x38_0100;

    /// Mailbox slot of a faked callee.
    fn oc1128_slot(addr: u32) -> u32 {
        match addr {
            0x0039_d860 => 0,
            0x0026_24f8 => 2,
            0x0026_25c8 => 3,
            0x0026_3338 => 4,
            0x0026_33f8 => 5,
            0x0026_1460 => 6,
            _ => 15,
        }
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// what the fixture forced (`StrHash` returns its argument, so the two
    /// `FindObject` lookups are told apart by the string address).
    pub(crate) fn oc1128_fake_call(c: &mut Cpu, addr: u32) {
        let (a0, a1, ra) = (c.g[A0][0], c.g[A1][0] as u32, c.g[RA][0] as u32);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0031_7670 => c.g[V0][0] = a0,
            0x0039_d860 => {
                let slot = if a1 == 0x0046_17a8 { 0 } else { 1 };
                c.g[V0][0] = c.bus.read32(OC1128_MAIL + 4 * slot) as u64;
            }
            0x0026_24f8 | 0x0026_25c8 | 0x0026_3338 | 0x0026_33f8 | 0x0026_1460 => {
                c.g[V0][0] = c.bus.read32(OC1128_MAIL + 4 * oc1128_slot(addr)) as u64;
            }
            _ => {}
        }
    }

    macro_rules! oc1128_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1128_CTRL) != OC1128_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1128_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1128_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1128_fakes!(
        (oc1128_fk_317670, OC1128_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (oc1128_fk_39d860, OC1128_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc1128_fk_3a0c50, OC1128_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (oc1128_fk_3a1000, OC1128_PREV_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (oc1128_fk_2624f8, OC1128_PREV_2624F8, ee::s07::OVERRIDE_002624F8, ee::s07::f_002624f8__generated, 0x0026_24f8),
        (oc1128_fk_2625c8, OC1128_PREV_2625C8, ee::s07::OVERRIDE_002625C8, ee::s07::f_002625c8__generated, 0x0026_25c8),
        (oc1128_fk_3184e8, OC1128_PREV_3184E8, ee::s10::OVERRIDE_003184E8, ee::s10::f_003184e8__generated, 0x0031_84e8),
        (oc1128_fk_263338, OC1128_PREV_263338, ee::s07::OVERRIDE_00263338, ee::s07::f_00263338__generated, 0x0026_3338),
        (oc1128_fk_2633f8, OC1128_PREV_2633F8, ee::s07::OVERRIDE_002633F8, ee::s07::f_002633f8__generated, 0x0026_33f8),
        (oc1128_fk_1a7390, OC1128_PREV_1A7390, ee::s03::OVERRIDE_001A7390, ee::s03::f_001a7390__generated, 0x001a_7390),
        (oc1128_fk_260f80, OC1128_PREV_260F80, ee::s06::OVERRIDE_00260F80, ee::s06::f_00260f80__generated, 0x0026_0f80),
        (oc1128_fk_261460, OC1128_PREV_261460, ee::s06::OVERRIDE_00261460, ee::s06::f_00261460__generated, 0x0026_1460),
        (oc1128_fk_2613c8, OC1128_PREV_2613C8, ee::s06::OVERRIDE_002613C8, ee::s06::f_002613c8__generated, 0x0026_13c8),
        (oc1128_fk_25b800, OC1128_PREV_25B800, ee::s06::OVERRIDE_0025B800, ee::s06::f_0025b800__generated, 0x0025_b800),
        (oc1128_fk_2af6c0, OC1128_PREV_2AF6C0, ee::s08::OVERRIDE_002AF6C0, ee::s08::f_002af6c0__generated, 0x002a_f6c0),
        (oc1128_fk_2af4e0, OC1128_PREV_2AF4E0, ee::s08::OVERRIDE_002AF4E0, ee::s08::f_002af4e0__generated, 0x002a_f4e0),
    );

    /// Fixture of `ShowBuddyInfo`. `edge`: force the path that reaches the
    /// back edge (valid index, roster lookup fails).
    fn oc1128_buddy_fixture(c: &mut Cpu, seed: u32, edge: bool) -> Rng {
        oc1128_install_fakes();
        let mut r = machine(c, seed, 0x1a_6070, 20);
        if edge {
            // No unwinds: the event test needs the back edge reached.
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        }
        c.bus.write32(OC1128_CTRL, OC1128_MAGIC);
        let this = DATA + 0x1000;
        let (name_obj, status_obj, vt) = (DATA + 0x2000, DATA + 0x2100, DATA + 0x7000);
        let screen = if !edge && seed % 11 == 0 { 0 } else { DATA + 0x5000 };
        c.bus.write32(this + SBI_SCREEN, screen);
        // Buddy names: none on every fifth seed.
        let start = DATA + 0x6000;
        let count = if !edge && seed % 5 == 0 { 0 } else { 1 + r.next_u32() % 4 };
        c.bus.write32(this + SBI_NAMES_START, start);
        c.bus.write32(this + SBI_NAMES_FINISH, start + 4 * count);
        for i in 0..4 {
            c.bus.write32(start + 4 * i, r.next_u32());
        }
        let index = match seed % 7 {
            0 if !edge => count,
            1 if !edge => (r.next_u32() | 0x8000_0000) as u32,
            _ => r.next_u32() % count.max(1),
        };
        // The two text objects share a vtable whose slot 0x48 is a fake.
        c.bus.write16(vt + 0x48, (r.next_u32() & 0x3c) as u16);
        c.bus.write32(vt + 0x4c, fakes::FAKE);
        c.bus.write32(name_obj + 8, vt);
        c.bus.write32(status_obj + 8, vt);
        let nz = |on: bool, v: u32| if on { v } else { 0 };
        c.bus.write32(OC1128_MAIL, nz(edge || seed % 13 != 1, name_obj));
        c.bus.write32(OC1128_MAIL + 4, nz(edge || seed % 9 != 2, status_obj));
        c.bus.write32(OC1128_MAIL + 8, r.next_u32());
        c.bus.write32(OC1128_MAIL + 12, nz(!edge && seed % 6 != 3, DATA + 0xa000));
        c.bus.write32(OC1128_MAIL + 16, (seed % 2) as u32);
        c.bus.write32(OC1128_MAIL + 20, r.next_u32());
        c.bus.write32(OC1128_MAIL + 24, ((seed / 2) % 3 != 0) as u32);
        // The buddy session (`gp + 0x238`): missing on every third seed.
        let session = DATA + 0x9000;
        c.bus.write32(GAME_GP + SBI_GP_SESSION, nz(seed % 3 != 0, session));
        c.bus.write32(session + 0xe8, (seed % 4 / 2) as u32);
        c.bus
            .write32(GAME_GP.wrapping_sub(0xc8), DATA + 0xb000);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(index);
        c.pc = 0x001a_6070;
        r
    }

    /// `ShowBuddyInfo` against the generated body: the no-screen, no-buddy
    /// (both session states), out-of-range, lookup-failure, offline and
    /// online arms, with fake callees that unwind at random call indices.
    #[test]
    fn fe_state_net_messenger_show_buddy_info_matches() {
        fakes::covered(
            ee::s03::f_001a6070__generated,
            fe_state_net_messenger_show_buddy_info,
            400,
            &|c, seed| {
                oc1128_buddy_fixture(c, seed, false);
            },
        );
    }

    /// The back edge after a failed roster lookup, with real events (off,
    /// delivered, unwinding to the resume point 0x1a61e0).
    #[test]
    fn fe_state_net_messenger_show_buddy_info_events_match() {
        driven!(gen_buddy, ee::s03::f_001a6070__generated);
        driven!(hand_buddy, fe_state_net_messenger_show_buddy_info);
        check_events(gen_buddy, hand_buddy, 60, &|c, seed| {
            let mut r = oc1128_buddy_fixture(c, seed, true);
            // Every lookup succeeds up to the roster entry, which fails.
            c.bus.write32(OC1128_MAIL + 12, 0);
            c.bus.write32(GAME_GP + SBI_GP_SESSION, DATA + 0x9000);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, 0x001a_6070)
        });
    }

    /// Fixture of the two fade functions: the fade clock state, the track
    /// and the current time. `track_off` is 0 for the volume fade and 8 for
    /// the aux-send fade (their fields differ by 8), `state_off` the offset
    /// of the clock fields in the path state.
    fn oc1128_fade_fixture(c: &mut Cpu, seed: u32, salt: u64, aux: bool) {
        oc1128_install_fakes();
        let mut r = machine(c, seed, salt, 1);
        c.bus.write32(OC1128_CTRL, OC1128_MAGIC);
        let (track, state) = (DATA + 0x1000, DATA + 0x2000);
        let (cur, target, start, st_dur, st_mode) = if aux {
            (0x14, 0x16, 0x10, 0x1c, 0x20)
        } else {
            (0xc, 0xe, 0x8, 0x14, 0x18)
        };
        c.bus.write32(0x0044_f420, state);
        let dur = if seed % 17 == 0 { 0 } else { 1 + r.next_u32() % 200 };
        c.bus.write16(state + st_dur, dur as u16);
        // Curve 0..4 in the low 7 bits (1 linear, 2 quadratic, 3 inverse).
        let mode = (seed % 5) | (r.next_u32() & !0x7f);
        c.bus.write32(state + st_mode, mode);
        // Signed byte volumes; equal on every ninth seed.
        let a = r.next_u32() as u8;
        let b = if seed % 9 == 0 { a } else { r.next_u32() as u8 };
        c.bus.write8(track + cur, a);
        c.bus.write8(track + target, b);
        c.bus.write8(track + 0x34, r.next_u32() as u8);
        c.bus.write32(track + 0x54, r.next_u32());
        let begin = r.next_u32() % 100_000;
        c.bus.write32(track + start, begin);
        // Elapsed ticks: before the start, inside the fade, past its end.
        let elapsed = match seed % 4 {
            0 => (r.next_u32() % 8) as i32 - 8,
            1 => (r.next_u32() % (dur + dur / 2 + 1)) as i32,
            2 => dur as i32 + (r.next_u32() % 5) as i32,
            _ => (r.next_u32() % (dur + 1)) as i32,
        };
        c.bus
            .write32(0x0044_f438, begin.wrapping_add(elapsed as u32));
        c.g[A0][0] = sx(track);
        c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
    }

    /// `PATH_TrackUpdateVolumeFade`: every curve, the finished fade, a
    /// fade that has not started, unwinding `PlatSetVolume` calls.
    #[test]
    fn path_track_update_volume_fade_matches() {
        check(
            ee::s14::f_003d5a98__generated,
            path_track_update_volume_fade,
            500,
            &|c, seed| oc1128_fade_fixture(c, seed, 0x3d5a98, false),
        );
    }

    /// `PATH_TrackUpdateAuxSendFade`: the same cases on the aux-send fields.
    #[test]
    fn path_track_update_aux_send_fade_matches() {
        check(
            ee::s14::f_003d5cc0__generated,
            path_track_update_aux_send_fade,
            500,
            &|c, seed| oc1128_fade_fixture(c, seed, 0x3d5cc0, true),
        );
    }
}
