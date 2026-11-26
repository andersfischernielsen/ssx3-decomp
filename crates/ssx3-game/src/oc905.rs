//! Staging module oc905 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc905`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001b_27b0,
    c_fe_state_chat_room_show_player_info,
    "cFEStateChatRoom_ShowPlayerInfo",
)];

/// Stack frame of `cFEStateChatRoom_ShowPlayerInfo`.
const SHOW_PLAYER_INFO_FRAME: u32 = 0x4c0;
/// `cFEStateChatRoom::mpScreen`.
pub(crate) const SHOW_PLAYER_INFO_SCREEN: u32 = 0x40;
/// `cFEStateChatRoom::mszPlayerName`.
pub(crate) const SHOW_PLAYER_INFO_NAME: u32 = 0x70c;
/// Local `info` (`sInfo_1B27B0`) at `sp + 0x410`: ranking, wins, games, disconnects.
pub(crate) const SHOW_PLAYER_INFO_INFO: u32 = 0x410;
/// Local lobby flag at `sp + 0x430`.
pub(crate) const SHOW_PLAYER_INFO_FLAG: u32 = 0x430;

/// Head of one text row: string-table lookup (`0x317670`) of `0x460000 + off`
/// and `cUIScreen_getObjectByHashName(mpScreen, v0)` (`0x39d860`). Leaves
/// `s3` = the text object and `a0` = `a0_end`.
fn show_player_info_row_head(
    c: &mut Cpu,
    off: u32,
    ret_l: u32,
    ret_o: u32,
    a0_end: u64,
    head_cycles: u64,
) -> Flow {
    c.cycles += head_cycles;
    c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(off));
    call_ptr(c, 0x0031_7670, ret_l)?;
    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_o)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A0][0] = a0_end;
    Ok(())
}

/// The blank text of a row without a valid player:
/// `cUIText_setAsciiString(s3, str)` (`0x3a0c50`, string at `0x4a1408`).
fn show_player_info_row_blank(c: &mut Cpu, ret: u32, a0_end: u64) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = 0x004a_0000;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
    call_ptr(c, 0x003a_0c50, ret)?;
    c.g[A0][0] = a0_end;
    Ok(())
}

/// Label lookup of a valid row: `s2` = text buffer (`sp + 0x10`), `a0` =
/// `0x460000 + lit_off` looked up (`0x317670`) and run through the world's
/// localisation object's virtual (`vt + 0x24`); `v0` = the label.
fn show_player_info_row_label(c: &mut Cpu, lit_off: u32, ret_l: u32, ret_v: u32) -> Flow {
    c.g[A0][0] = 0x0046_0000;
    c.g[S2][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(lit_off));
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, 0x0031_7670, ret_l)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let t = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, t, ret_v)?;
    Ok(())
}

/// Label and `info` field of one plain stat row, and where its calls return.
struct ShowPlayerInfoRow {
    lookup_off: u32,
    rets: [u32; 2],
    valid_off: u32,
    vrets: [u32; 4],
    gp_off: u32,
    field: u32,
    a0_end: u64,
    blank_ret: u32,
    /// Cycles charged at the row head (the ranking row's 8 are folded into
    /// the 12 charged before the `0x25f288` call).
    head_cycles: u64,
}

/// One plain stat row: `sprintf2_v(buf, fmt, label, info.field)` (`0x2c27c0`)
/// and `cUIText_setUnicodeString(text, buf)` (`0x3a0e90`), or the blank text
/// when `idx` is not a valid player (`s5 == 0`). Skipped if the screen has no
/// such text object.
fn show_player_info_stat_row(c: &mut Cpu, r: &ShowPlayerInfoRow) -> Flow {
    show_player_info_row_head(
        c,
        r.lookup_off,
        r.rets[0],
        r.rets[1],
        r.a0_end,
        r.head_cycles,
    )?;
    if c.g[S3][0] != 0 {
        c.cycles += 2;
        let no_player = c.g[S5][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !no_player {
            c.cycles += 22;
            show_player_info_row_label(c, r.valid_off, r.vrets[0], r.vrets[1])?;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(r.gp_off));
            c.g[A0][0] = c.g[S2][0];
            c.g[A3][0] = c.lw(sp(c).wrapping_add(r.field));
            c.g[A2][0] = c.g[V0][0];
            call_ptr(c, 0x002c_27c0, r.vrets[2])?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x003a_0e90, r.vrets[3])?;
            c.g[A0][0] = r.a0_end;
        } else {
            show_player_info_row_blank(c, r.blank_ret, r.a0_end)?;
        }
    }
    Ok(())
}

/// `cUIObject::setVisible`-style virtual (`vt + 0x4c`, this adjusted by the
/// `short` at `vt + 0x48`) on `obj` with `a1 = flag`; `ret` is the return
/// address. `a1` is already set by the caller.
fn show_player_info_vcall(c: &mut Cpu, obj: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw(obj.wrapping_add(8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `void cFEStateChatRoom::ShowPlayerInfo(this, int idx)` (0x1b27b0): fills
/// the chat room's player card for lobby player `idx`: copies the player's
/// name into `this + 0x70c`, shows or hides the select icon and text from the
/// lobby flag, sets the name text, then the ranking, games played, win ratio
/// (`wins / games * 100`, through `fptodp`), disconnects and overall ranking
/// rows from `0x25f288`. Without a valid player (`idx` out of range) the
/// rows show the blank string.
pub fn c_fe_state_chat_room_show_player_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, SHOW_PLAYER_INFO_FRAME);
        frame.save(c, S4, 0x460);
        c.g[A2][0] = sx(sp(c).wrapping_add(SHOW_PLAYER_INFO_FLAG));
        frame.save(c, S2, 0x480);
        c.g[S4][0] = c.g[A0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x4a0);
        frame.save(c, S1, 0x490);
        frame.save(c, S3, 0x470);
        frame.save(c, S5, 0x450);
        frame.save_ra(c, 0x440);
        frame.save_fpr(c, 20, 0x4b0);
        let flag_addr = sp(c).wrapping_add(SHOW_PLAYER_INFO_FLAG);
        c.sw(flag_addr, 0);
        // playerName = cBXString(LobbyMan_GetPlayerName(lobby, idx, &flag))
        call_ptr(c, 0x0025_f1e0, 0x001b_27ec)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0031_82d8, 0x001b_27f8)?;
        let name_ptr = c.lw(sp(c));
        c.g[A1][0] = name_ptr;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_NAME));
        call_ptr(c, 0x0041_66f4, 0x001b_2804)?;
        c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_SCREEN));
        let no_screen = c.g[V1][0] == 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        if no_screen {
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x001b_281c)?;
            frame.restore(c, S0, 0x4a0);
        } else {
            show_player_info_body(c)?;
            c.cycles += 3;
            c.g[A1][0] = 2;
            call_ptr(c, 0x0031_8260, 0x001b_2ca8)?;
            frame.restore(c, S0, 0x4a0);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x490);
        frame.restore(c, S2, 0x480);
        frame.restore(c, S3, 0x470);
        frame.restore(c, S4, 0x460);
        frame.restore(c, S5, 0x450);
        frame.restore_ra(c, 0x440);
        frame.restore_fpr(c, 20, 0x4b0);
        frame.close(c);
        Ok(())
    })
}

/// The part of `ShowPlayerInfo` that runs when the screen exists (up to the
/// `cBXString` destructor call).
fn show_player_info_body(c: &mut Cpu) -> Flow {
    // valid = idx < LobbyMan_GetPlayerCount() (when the count is positive)
    c.cycles += 4;
    c.g[S5][0] = 0;
    call_ptr(c, 0x0025_f1b0, 0x001b_282c)?;
    if (c.g[V0][0] as i64) <= 0 {
        c.g[A0][0] = 0x0046_0000;
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        call_ptr(c, 0x0025_f1b0, 0x001b_283c)?;
        c.g[S5][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A0][0] = 0x0046_0000;
    }

    // title = getObjectByHashName(mpScreen, "TextPlayerName")
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3108));
    call_ptr(c, 0x0031_7670, 0x001b_284c)?;
    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_2858)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] == 0 {
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    } else {
        c.cycles += 15;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ad8));
        call_ptr(c, 0x0031_7670, 0x001b_2870)?;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_SCREEN));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001b_287c)?;
        c.g[A0][0] = 0x004a_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b60));
        call_ptr(c, 0x0031_7670, 0x001b_288c)?;
        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_SCREEN));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001b_2898)?;
        let no_text = c.g[S0][0] == 0;
        c.g[A2][0] = c.g[V0][0];
        if !no_text {
            c.cycles += 2;
            let no_icon = c.g[A2][0] == 0;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(SHOW_PLAYER_INFO_FLAG));
            if !no_icon {
                c.cycles += 2;
                let flag_zero = c.g[V0][0] == 0;
                c.g[A1][0] = 0;
                let icon = c.g[A2][0] as u32;
                if !flag_zero {
                    // flag set: hide the icon and the text
                    c.cycles += 13;
                    show_player_info_vcall(c, icon, 0x001b_28c4)?;
                    c.g[A1][0] = 0;
                    let text = c.g[S0][0] as u32;
                    show_player_info_vcall(c, text, 0x001b_28dc)?;
                } else {
                    // flag clear: show both
                    c.cycles += 12;
                    c.g[A1][0] = 1;
                    show_player_info_vcall(c, icon, 0x001b_28fc)?;
                    c.g[A1][0] = 1;
                    let text = c.g[S0][0] as u32;
                    show_player_info_vcall(c, text, 0x001b_2914)?;
                }
            }
        }
        // title text: the name, or the blank string
        c.cycles += 2;
        let no_player = c.g[S5][0] == 0;
        c.g[A1][0] = 0x004a_0000;
        let mut blank = no_player;
        if !no_player {
            c.cycles += 4;
            c.g[A1][0] = c.lw(sp(c));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(8));
            if (c.g[V0][0] as i64) <= 0 {
                c.g[A1][0] = 0x004a_0000;
                blank = true;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003a_0c50, 0x001b_2934)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            }
        }
        if blank {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
            call_ptr(c, 0x003a_0c50, 0x001b_2948)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        }
    }

    // info = LobbyMan_GetPlayerInfo(lobby, idx)
    c.cycles += 12;
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = sx(sp(c).wrapping_add(SHOW_PLAYER_INFO_INFO));
    call_ptr(c, 0x0025_f288, 0x001b_2958)?;
    c.g[A0][0] = 0x0046_0000;

    // TextRanking
    show_player_info_stat_row(
        c,
        &ShowPlayerInfoRow {
            lookup_off: 0x3ae8,
            rets: [0x001b_2964, 0x001b_2970],
            valid_off: 0x3af8,
            vrets: [0x001b_29a4, 0x001b_29b4, 0x001b_29c8, 0x001b_29d4],
            gp_off: 0x15c0,
            field: 0x410,
            a0_end: 0x0046_0000,
            blank_ret: 0x001b_29ec,
            head_cycles: 0,
        },
    )?;
    // TextGamesPlayed
    show_player_info_stat_row(
        c,
        &ShowPlayerInfoRow {
            lookup_off: 0x3b10,
            rets: [0x001b_29f8, 0x001b_2a04],
            valid_off: 0x3568,
            vrets: [0x001b_2a38, 0x001b_2a48, 0x001b_2a5c, 0x001b_2a68],
            gp_off: 0x15bc,
            field: 0x418,
            a0_end: 0x0046_0000,
            blank_ret: 0x001b_2a80,
            head_cycles: 8,
        },
    )?;

    // TextWinRatio: wins / games * 100.0
    c.cycles += 8;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b20));
    call_ptr(c, 0x0031_7670, 0x001b_2a8c)?;
    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(SHOW_PLAYER_INFO_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_2a98)?;
    c.g[S3][0] = c.g[V0][0];
    let no_text = c.g[S3][0] == 0;
    c.g[A0][0] = 0x0046_0000;
    if !no_text {
        c.cycles += 2;
        let no_player = c.g[S5][0] == 0;
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x414));
        if no_player {
            show_player_info_row_blank(c, 0x001b_2b74, 0x0046_0000)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.lw(sp(c).wrapping_add(0x418));
            c.mtc1(0, 20);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                // wins < games: pct = (float)wins / (float)games * 100.0f
                c.cycles += 9;
                c.mtc1(3, 0);
                c.cvt_s_w(0, 0);
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 2);
                c.mtc1(4, 1);
                c.cvt_s_w(1, 1);
                c.div_s(0, 0, 1);
                c.mul_s(20, 0, 2);
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            } else {
                c.cycles += 2;
                let no_wins = c.g[V1][0] == 0;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if !no_wins {
                    // wins >= games, wins != 0: pct = 100.0f
                    c.cycles += 2;
                    c.g[AT][0] = 0x42c8_0000;
                    c.mtc1(1, 20);
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                }
            }
            c.cycles += 27;
            show_player_info_row_label(c, 0x3580, 0x001b_2b18, 0x001b_2b28)?;
            c.mov_s(12, 20);
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0041_3af8, 0x001b_2b34)?;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x15b8));
            c.g[A0][0] = c.g[S2][0];
            c.g[T0][0] = c.lw(sp(c).wrapping_add(0x418));
            c.g[T1][0] = c.g[V0][0];
            c.g[A3][0] = c.lw(sp(c).wrapping_add(0x414));
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x002c_27c0, 0x001b_2b50)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x003a_0e90, 0x001b_2b5c)?;
            c.g[A0][0] = 0x0046_0000;
        }
    }

    // TextDisconnects
    show_player_info_stat_row(
        c,
        &ShowPlayerInfoRow {
            lookup_off: 0x3b30,
            rets: [0x001b_2b80, 0x001b_2b8c],
            valid_off: 0x35a8,
            vrets: [0x001b_2bc0, 0x001b_2bd0, 0x001b_2be4, 0x001b_2bf0],
            gp_off: 0x15b4,
            field: 0x41c,
            a0_end: 0x0046_0000,
            blank_ret: 0x001b_2c08,
            head_cycles: 8,
        },
    )?;
    // TextOverallRanking
    let a0_end = c.g[SP][0];
    show_player_info_stat_row(
        c,
        &ShowPlayerInfoRow {
            lookup_off: 0x3b40,
            rets: [0x001b_2c14, 0x001b_2c20],
            valid_off: 0x3b58,
            vrets: [0x001b_2c54, 0x001b_2c64, 0x001b_2c78, 0x001b_2c84],
            gp_off: 0x15b0,
            field: 0x410,
            a0_end,
            blank_ret: 0x001b_2c9c,
            head_cycles: 8,
        },
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, fake_call, LOG_COUNT};
    use crate::difftest::{DATA, GAME_GP};

    const OC905_SELF: u32 = DATA + 0x20_0000;
    const OC905_SCREEN: u32 = DATA + 0x20_7000;
    const OC905_WORLD: u32 = DATA + 0x20_2000;
    const OC905_LOC: u32 = DATA + 0x20_3000;
    const OC905_LOC_VT: u32 = DATA + 0x20_4000;
    const OC905_NAME: u32 = DATA + 0x20_5000;
    /// Test control words: +0/+4 mask of call indices that return 0, +8 the
    /// lobby flag, +0xc name length, +0x10.. the four `info` words.
    const OC905_CTRL: u32 = DATA + 0x20_6000;

    /// The machine's miss hook: the logging fake, plus what the real callees
    /// leave in memory (name string, lobby flag, `info`) and a per-call mask
    /// that makes a call return 0 (null text objects, empty lobby).
    fn oc905_miss(c: &mut Cpu, target: u32) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let n = c.bus.read32(LOG_COUNT);
        fake_call(c, target);
        match target {
            0x0031_82d8 => {
                let len = c.bus.read32(OC905_CTRL + 0xc);
                c.bus.write32(a0, OC905_NAME);
                c.bus.write32(OC905_NAME - 8, len);
            }
            0x0025_f1e0 => {
                let flag = c.bus.read32(OC905_CTRL + 8);
                c.bus.write32(a2, flag);
            }
            0x0025_f288 => {
                for i in 0..4 {
                    let v = c.bus.read32(OC905_CTRL + 0x10 + 4 * i);
                    c.bus.write32(a2 + 4 * i, v);
                }
            }
            _ => {}
        }
        if n < 64 {
            let mask = c.bus.read32(OC905_CTRL + 4 * (n / 32));
            if mask >> (n % 32) & 1 != 0 {
                c.g[V0][0] = 0;
            }
        }
    }

    /// `ShowPlayerInfo`: every branch (no screen, empty lobby, invalid index,
    /// null text objects, lobby flag, win ratio below/at/above 100%, blank
    /// rows) and unwinds in the calls.
    #[test]
    fn c_fe_state_chat_room_show_player_info_matches() {
        fakes::covered(
            ee::s03::f_001b27b0__generated,
            c_fe_state_chat_room_show_player_info,
            240,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1b27b0, 4);
                c.hooks.miss = oc905_miss;
                c.pc = 0x001b_27b0;
                c.g[A0][0] = sx(OC905_SELF);
                c.g[A1][0] = match seed % 4 {
                    0 => 0x0090_0000,
                    _ => (r.next_u32() % 8) as u64,
                };
                c.bus.write32(
                    OC905_SELF + SHOW_PLAYER_INFO_SCREEN,
                    if seed % 7 == 0 { 0 } else { OC905_SCREEN },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC905_WORLD);
                c.bus.write32(OC905_WORLD + 0x8c, OC905_LOC);
                c.bus.write32(OC905_LOC + 4, OC905_LOC_VT);
                c.bus.write16(OC905_LOC_VT + 0x20, 0);
                c.bus.write32(OC905_LOC_VT + 0x24, 0x0014_06c0);
                let (m0, m1) = if seed % 5 == 0 {
                    (0, 0)
                } else {
                    (
                        r.next_u32() & r.next_u32() & r.next_u32() & r.next_u32(),
                        r.next_u32() & r.next_u32() & r.next_u32() & r.next_u32(),
                    )
                };
                c.bus.write32(OC905_CTRL, m0);
                c.bus.write32(OC905_CTRL + 4, m1);
                c.bus.write32(OC905_CTRL + 8, (seed / 2) % 2);
                c.bus.write32(
                    OC905_CTRL + 0xc,
                    if seed % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 9
                    },
                );
                let wins = r.next_u32() % 6;
                let games = match seed % 3 {
                    0 => wins,
                    1 => wins + 1 + r.next_u32() % 5,
                    _ => r.next_u32() % 6,
                };
                c.bus.write32(OC905_CTRL + 0x10, r.next_u32() & 0xfff);
                c.bus.write32(OC905_CTRL + 0x14, wins);
                c.bus.write32(OC905_CTRL + 0x18, games);
                c.bus.write32(OC905_CTRL + 0x1c, r.next_u32());
            },
        );
    }
}
