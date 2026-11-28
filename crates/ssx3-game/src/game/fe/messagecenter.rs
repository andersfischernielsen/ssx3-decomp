//! `src/fe/messagecenter.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_79d8,
        cfestate_request_line_on_child_event,
        "cFEStateRequestLine_OnChildEvent",
    ),
    (
        0x0019_7ad8,
        cfe_state_request_line_sort_song_list,
        "cFEStateRequestLine_SortSongList",
    ),
    (
        0x0019_7bc8,
        cfestaterequestline_update_song_names,
        "cFEStateRequestLine_UpdateSongNames",
    ),
    (
        0x0019_8118,
        cfestaterequestline_update_song_count,
        "cFEStateRequestLine_UpdateSongCount",
    ),
    (
        0x0019_85b0,
        c_fe_state_request_line_update_money,
        "cFEStateRequestLine_UpdateMoney",
    ),
    (
        0x0019_85f0,
        cfestaterequestline_update_slot_icons,
        "cFEStateRequestLine_UpdateSlotIcons",
    ),
    (
        0x0019_88d8,
        cfe_request_line_buy_song,
        "cFEStateRequestLine_BuySong",
    ),
    (
        0x0019_8988,
        cfestate_request_line_open_buy_popup,
        "cFEStateRequestLine_OpenBuyPopup",
    ),
    (
        0x0019_8da0,
        cfe_state_char_equip_compare_items,
        "cFEStateCharEquip_CompareItems",
    ),
    (
        0x0019_7500,
        cfe_state_request_line_on_widget_action,
        "cFEStateRequestLine_OnWidgetAction",
    ),
    (
        0x0019_77d0,
        cfe_state_request_line_on_widget_notify,
        "cFEStateRequestLine_OnWidgetNotify",
    ),
    (
        0x0019_7ca0,
        cfe_state_request_line_update_song_icons,
        "cFEStateRequestLine_UpdateSongIcons",
    ),
    (
        0x0019_7e70,
        cfe_state_request_line_update_now_playing,
        "cFEStateRequestLine_UpdateNowPlaying",
    ),
    (
        0x0019_81f8,
        cfe_state_request_line_update_help_text,
        "cFEStateRequestLine_UpdateHelpText",
    ),
    (
        0x0019_8340,
        cfe_state_request_line_update_buttons,
        "cFEStateRequestLine_UpdateButtons",
    ),
    (
        0x0019_86d0,
        cfestate_request_line_show_song_info,
        "cFEStateRequestLine_ShowSongInfo",
    ),
    (
        0x0019_8a88,
        cfestate_request_line_get_song_title,
        "cFEStateRequestLine_GetSongTitle",
    ),
    (
        0x0019_8ab8,
        cfestate_request_line_get_song_artist,
        "cFEStateRequestLine_GetSongArtist",
    ),
    (
        0x0019_8ae8,
        cfestate_request_line_get_song_price,
        "cFEStateRequestLine_GetSongPrice",
    ),
    (0x0019_8af0, oc855_fe_format_money, "FE_FormatMoney"),
    (
        0x0019_6f98,
        cfe_state_request_line_on_widget_created,
        "cFEStateRequestLine_OnWidgetCreated",
    ),
];

// ---------------------------------------------------------------------------
// Callees (the generated crate's readable aliases, which dispatch through
// their override slot exactly as the generated `jal`/`call_indirect` does).

pub(crate) mod callee {

    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;

    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
}

// ---------------------------------------------------------------------------
// Money / cash text hooks: format the value and hand the string to a text.

/// `void cFEStateRequestLine_UpdateMoney(State* s)` (0x1985b0): set the
/// money text (`+0x1e0`) to the formatted value at `+0x168`.
pub fn c_fe_state_request_line_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1e0);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw(c, this, 0x168);
            callee::FE_FORMAT_MONEY.call(c, 0x0019_85d4)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw(c, this, 0x1e0);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING.call(c, 0x0019_85e0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateCharEquip_CompareItems(Item** a, Item** b)` (0x00198da0):
/// the list order is the signed short sort key at +0xa of each item.
pub fn cfe_state_char_equip_compare_items(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
    c.g[V1][0] = Addr(abi::arg(c, 1)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xa)).lh(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// `cFEStateRequestLine_SortSongList` (0x197ad8): stable-partition the song
// list by the selection mask into `list`: first the selected indices, then
// the unselected ones.
pub fn cfe_state_request_line_sort_song_list(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
    c.g[A1][0] = 0u64;
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0u64;
    if !empty {
        c.cycles += 2;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
        c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
        loop {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] >> (c.g[A1][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            }
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
            c.cycles += 4;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
                break;
            }
            c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
            if event_due(c) {
                deliver_event_here(c, 0x0019_7af0);
            }
        }
    }
    c.cycles += 2;
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = 0u64;
    if !empty {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4c));
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
        loop {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] >> (c.g[V1][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            }
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14c));
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
            if event_due(c) {
                deliver_event_here(c, 0x0019_7b48);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cFEStateRequestLine_BuySong(self, int songIndex)` (0x1988d8): mark
/// the song bought (only once at least six songs are owned) and refresh the
/// store's owned/bought bitmaps.
pub fn cfe_request_line_buy_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = 0;
        c.g[A1][0] = Addr(p(this as u32, 0x160)).ld(c);
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8900);
            }
        }
        c.cycles += 27;
        c.g[S0][0] = ((c.g[S0][0] as i64) < 6) as u64;
        c.g[S0][0] = c.g[S0][0] ^ 0x1;
        call_ptr(c, 0x0014_dc80, 0x0019_8928)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0019_8934)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lb(c);
        c.g[T0][0] = c.g[S0][0];
        c.g[A2][0] = Addr(p(c.g[S1][0] as u32, 0x48)).lw(c);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_8558, 0x0019_894c)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x160)).ld(c);
        c.g[V1][0] = 0x1;
        c.g[A0][0] = Addr(p(c.g[S1][0] as u32, 0x158)).ld(c);
        c.g[V1][0] = c.g[V1][0] << (c.g[S2][0] & 63);
        c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
        c.sw(p(c.g[S1][0] as u32, 0x168), c.g[V0][0]);
        c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
        c.sd(p(c.g[S1][0] as u32, 0x160), c.g[A1][0]);
        c.sd(p(c.g[S1][0] as u32, 0x158), c.g[A0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front end: request line.

/// `void cFEStateRequestLine_UpdateSongNames(self)` (0x197bc8): fill the
/// eight visible text rows from the message list (or hide the empty ones).
pub fn cfestaterequestline_update_song_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = sx(p(r32(c, S2), 0x174));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S0, 0x50);
        c.g[S4][0] = sx(p(r32(c, S2), 0x4c));
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S2), 0x150)).lw(c);
        loop {
            c.cycles += 8;
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr(p(r32(c, S2), 0x14c)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, S3)));
            c.g[V1][0] = sx(r32(c, V0) << 2);
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let in_range = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S4), r32(c, V1)));
            if in_range {
                c.cycles += 15;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let show = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, show, 0x0019_7c34)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0019_8a88, 0x0019_7c40)?;
                c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x0019_7c4c)?;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            } else {
                c.cycles += 8;
                c.g[A2][0] = Addr(r32(c, S1)).lw(c);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                let show = r32(c, V1);
                c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                call_ptr(c, show, 0x0019_7c70)?;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p(r32(c, S2), 0x150)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0019_7bf8);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Request line: song count / slot icons.

/// `void cFEStateRequestLine_UpdateSongCount(self)` (0x198118): format the
/// number of set mask bits into the song-count label.
pub fn cfestaterequestline_update_song_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x460000;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5d8));
        c.g[S0][0] = Addr(p(r32(c, V0), 0x8c)).lw(c);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x20)).lh(c);
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V0)));
        call_ptr(c, 0x0031_7670, 0x0019_8150)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x24)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        let f = r32(c, V1);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, f, 0x0019_8160)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003d_bca0, 0x0019_816c)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(5));
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = sx(r32(c, V0) << 1);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x688));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0019_8188)?;
        c.g[A1][0] = c.ld(p(r32(c, S2), 0x158));
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[A0][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (r32(c, A0) & 63);
            c.g[V0][0] = c.g[V0][0] & 1;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[A2][0] = sx(p(r32(c, V0), r32(c, A2)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8198);
            }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002c_26d0, 0x0019_81c0)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x1e4)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_0e90, 0x0019_81cc)?;
        if c.g[S0][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e98, 0x0019_81dc)?;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_UpdateSlotIcons(self)` (0x1985f0): show the
/// icons for the slots the bit mask selects, hide the rest.
pub fn cfestaterequestline_update_slot_icons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[A0][0] = 0;
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.ld(p(r32(c, S2), 0x160));
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (r32(c, A0) & 63);
            c.g[V0][0] = c.g[V0][0] & 1;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[S3][0] = sx(p(r32(c, V0), r32(c, S3)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8620);
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0;
        c.g[S4][0] = 6;
        c.g[S0][0] = sx(p(r32(c, S2), 0x1bc));
        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(r32(c, S0)).lw(c);
            let no_icon = c.g[A2][0] == 0;
            c.g[V0][0] = sx(r32(c, S4).wrapping_sub(r32(c, S1)));
            if no_icon {
                c.cycles += 1;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(p(r32(c, S2), 0x16c)).lw(c);
                c.g[A1][0] = 0;
                let disabled = c.g[V1][0] == 0;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if disabled {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                    c.cycles += 4;
                    c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                    c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                    let f = r32(c, V1);
                    c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                    call_ptr(c, f, 0x0019_869c)?;
                    c.cycles += 1;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    let few = c.g[V0][0] == 0;
                    c.g[V0][0] = Addr(p(r32(c, A2), 8)).lw(c);
                    if few {
                        c.cycles += 4;
                        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                        let f = r32(c, V1);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, f, 0x0019_869c)?;
                        c.cycles += 1;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = 1;
                        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
                        let f = r32(c, V1);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, f, 0x0019_8680)?;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_8648);
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRequestLine_OnChildEvent(self, arg, event)` (0x1979d8): the
/// request-line screen's child events: event 0x15 clears the "selected"
/// flag, event 0x16 sets it and refreshes the list when the child is the
/// current song.
pub fn cfestate_request_line_on_child_event(c: &mut Cpu) {
    const REFRESH_LIST: Callee = Callee {
        addr: 0x0019_7b88,
        f: ee::s02::cFEStateRequestLine_RefreshList,
    };
    const UPDATE_HELP: Callee = Callee {
        addr: 0x0019_81f8,
        f: ee::s02::cFEStateRequestLine_UpdateHelpText,
    };
    const GET_ROW_VALUE: u32 = 0x0019_88d8;
    const SET_ROW_SELECTED: u32 = 0x0019_86d0;
    const UPDATE_ARROWS: u32 = 0x0019_85b0;
    const UPDATE_LIST: u32 = 0x0019_85f0;
    const SET_ROW_STATE: u32 = 0x0019_8340;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x15;
        frame.save(c, 17, 0x10);
        frame.save(c, 16, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let is_15 = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let stage: u32; // 1 event 0x15, 2 event 0x16
        if is_15 {
            stage = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x16;
            let is_16 = c.g[A2][0] == c.g[V0][0];
            c.lq(16, frame.at(c, 0x20));
            if is_16 {
                stage = 2;
            } else {
                c.cycles += 2;
                c.lq(17, frame.at(c, 0x10));
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        if stage == 1 {
            c.cycles += 4;
            c.g[V1][0] = lw_2(c, p(c.g[A1][0] as u32, 0xc));
            c.g[V0][0] = 7;
            let current = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = 0xffff_ffff_ffff_fff7;
            if !current {
                c.cycles += 4;
                c.g[V0][0] = lw_2(c, p(this, 0x1c));
                c.g[V0][0] &= c.g[V1][0];
                sw(c, p(this, 0x1c), c.g[V0][0]);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        } else {
            // event 0x16: set bit 3 of the "selected" mask, then refresh.
            c.cycles += 5;
            c.g[V1][0] = lw_2(c, p(this, 0x1c));
            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] | 8;
                sw(c, p(this, 0x1c), c.g[V0][0]);
            }
            c.g[V1][0] = lw_2(c, p(c.g[A1][0] as u32, 0xc));
            c.cycles += 3;
            c.g[V0][0] = 7;
            let current = c.g[V1][0] != c.g[V0][0];
            c.lq(16, frame.at(c, 0x20));
            if current {
                c.cycles += 1;
                c.lq(17, frame.at(c, 0x10));
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = lw_2(c, p(c.g[A1][0] as u32, 0x6c));
            let no_child = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if no_child {
                c.cycles += 1;
                c.lq(17, frame.at(c, 0x10));
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 25;
            c.g[A2][0] = lw_2(c, p(this, 0x170));
            c.g[V0][0] = lw_2(c, p(this, 0x150));
            c.g[V1][0] = lw_2(c, p(c.g[A2][0] as u32, 0xa0));
            c.g[A1][0] = lw_2(c, p(c.g[V1][0] as u32, 0x18));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = lw_2(c, p(c.g[V0][0] as u32, 0x4c));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, GET_ROW_VALUE, 0x0019_7a88)?;
            c.g[A0][0] = c.g[S1][0];
            REFRESH_LIST.call(c, 0x0019_7a90)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, SET_ROW_SELECTED, 0x0019_7a9c)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, UPDATE_ARROWS, 0x0019_7aa4)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, UPDATE_LIST, 0x0019_7aac)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SET_ROW_STATE, 0x0019_7ab8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            UPDATE_HELP.call(c, 0x0019_7ac4)?;
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 1;
        c.lq(17, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRequestLine_OpenBuyPopup(self, song)` (0x198988): allocate and
/// construct the buy popup for `song`, count the song's credits, fill the
/// title/price line, and push the popup on the UI state stack.
pub fn cfestate_request_line_open_buy_popup(c: &mut Cpu) {
    const MEM_ALLOC: u32 = 0x0031_7d70;
    const POPUP_CTOR: u32 = 0x001c_aba8;
    const GET_SONG_TITLE: Callee = Callee {
        addr: 0x0019_8a88,
        f: ee::s03::cFEStateRequestLine_GetSongTitle,
    };
    const GET_SONG_PRICE: Callee = Callee {
        addr: 0x0019_8ae8,
        f: ee::s03::cFEStateRequestLine_GetSongPrice,
    };
    const SET_TRACK: u32 = 0x001c_a980;
    const SET_CREDIT: u32 = 0x001c_a900;
    const PUSH_POPUP: u32 = 0x0039_f290;

    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x60);
        let this = abi::arg(c, 0);
        let _song = abi::arg(c, 1);
        c.g[A2][0] = 0;
        frame.save(c, 20, 0x10);
        c.g[A3][0] = 0;
        frame.save(c, 16, 0x50);
        c.g[S4][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, 18, 0x30);
        c.g[A0][0] = 0x70;
        frame.save(c, 19, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x790));
        frame.save(c, 17, 0x40);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, MEM_ALLOC, 0x0019_89c8)?;
        c.g[A1][0] = lw_2(c, p(this, 0x10));
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.lb(p(this, 0x44));
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, POPUP_CTOR, 0x0019_89dc)?;
        c.g[A1][0] = c.ld(p(this, 0x160));
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
            c.g[V0][0] &= 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V1][0] = ((c.g[A0][0] as i64) < 64) as u64;
            let more = c.g[V1][0] != 0;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_89e8);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        GET_SONG_TITLE.call(c, 0x0019_8a10)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
        let few = c.g[V0][0] != 0;
        c.g[A2][0] = 6;
        if few {
            c.cycles += 5;
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S2][0] as u32));
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
            call_ptr(c, SET_CREDIT, 0x0019_8a58)?;
            c.g[A0][0] = lw_2(c, p(this, 0x10));
        } else {
            c.cycles += 10;
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S0][0];
            GET_SONG_PRICE.call(c, 0x0019_8a2c)?;
            c.g[A3][0] = lw_2(c, p(this, 0x168));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x48));
            call_ptr(c, SET_TRACK, 0x0019_8a40)?;
            c.g[A0][0] = lw_2(c, p(this, 0x10));
        }
        c.cycles += 11;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, PUSH_POPUP, 0x0019_8a68)?;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRequestLine_OnWidgetAction(self, widget, event)` (0x197500):
/// the request-line screen's widget actions (event 6: a jump through the
/// widget's vtable, scrolling the song list, buying/playing songs).
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_request_line_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1977c4: {
            'b_1977c0: {
                'b_1977bc: {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[A1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1977bc;
                    } else {
                        'b_197780: {
                            'b_19768c: {
                                'b_197550: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                    if t {
                                        break 'b_197550;
                                    } else {
                                        'b_19753c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_19753c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_197780;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_1977c4;
                                                    } else {
                                                        break 'b_19753c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_19768c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            if t {
                                                break 'b_1977c4;
                                            } else {
                                                break 'b_197550;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14c));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                if t {
                                    break 'b_1977bc;
                                } else {
                                    'b_19767c: {
                                        'b_197678: {
                                            'b_1975fc: {
                                                c.cycles += 9;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S1][0] as u32).wrapping_add(0x160));
                                                c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                                c.g[V0][0] = c.g[V0][0] >> (c.g[S0][0] & 63);
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                c.g[V0][0] = c.g[V0][0] << 32;
                                                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] =
                                                        c.ld(
                                                            (c.g[S1][0] as u32).wrapping_add(0x158)
                                                        );
                                                    break 'b_1975fc;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x16c)
                                                        );
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_197678;
                                                    } else {
                                                        'b_1975e8: {
                                                            c.cycles += 6;
                                                            c.g[RA][0] = sx(0x001975a4);
                                                            c.g[A1][0] = c.g[S0][0];
                                                            call_ptr(c, 0x00198ae8, 0x001975a4)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x168));
                                                            c.g[V1][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[V1][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_1975e8;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .ld((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x160));
                                                                c.g[A0][0] = 0u64;
                                                                'l_1975c0: loop {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = c.g[A2][0]
                                                                        >> (c.g[A0][0] & 63);
                                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.g[V1][0] = ((c.g[A0][0]
                                                                        as i64)
                                                                        < 64i64)
                                                                        as u64;
                                                                    let t = c.g[V1][0] != 0u64;
                                                                    c.g[A1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A1][0] as u32,
                                                                        ));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x001975c0,
                                                                            );
                                                                        }
                                                                        continue 'l_1975c0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = ((c.g[A1][0]
                                                                            as i64)
                                                                            < 6i64)
                                                                            as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        if t {
                                                                            break 'b_19767c;
                                                                        } else {
                                                                            break 'b_1975e8;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x001975f4);
                                                        c.g[A1][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00198988, 0x001975f4)?;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_19767c;
                                                        } else {
                                                            break 'b_1975fc;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_197638: {
                                                c.cycles += 5;
                                                c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                                                c.g[V0][0] = c.g[V0][0] << 32;
                                                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = c.g[V1][0] >> (c.g[S0][0] & 63);
                                                if t {
                                                    break 'b_197638;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[V0][0] = 0x1;
                                                    c.g[V0][0] = c.g[V0][0] << (c.g[S0][0] & 63);
                                                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                    c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                                                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                                    c.g[V0][0] = c.g[V0][0] << 32;
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_19767c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] =
                                                            c.g[V1][0] >> (c.g[S0][0] & 63);
                                                        break 'b_197638;
                                                    }
                                                }
                                            }
                                            'b_197664: {
                                                'b_19765c: {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[V0][0] = c.g[V0][0] << 32;
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_19765c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] =
                                                            c.g[V0][0] << (c.g[S0][0] & 63);
                                                        c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                        let t = true;
                                                        c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                                                        if t {
                                                            break 'b_197664;
                                                        } else {
                                                            break 'b_19765c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] = c.g[V0][0] << (c.g[S0][0] & 63);
                                                c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
                                                break 'b_197664;
                                            }
                                            c.cycles += 5;
                                            c.sd(
                                                (c.g[S1][0] as u32).wrapping_add(0x158),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x00197670);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x00197ca0, 0x00197670)?;
                                            c.g[RA][0] = sx(0x00197678);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x00198118, 0x00197678)?;
                                            break 'b_197678;
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_19767c;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00197684);
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x00198340, 0x00197684)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1977c0;
                                    } else {
                                        break 'b_19768c;
                                    }
                                }
                            }
                            'b_197714: {
                                c.cycles += 24;
                                c.g[RA][0] = sx(0x00197694);
                                call_ptr(c, 0x0014dc80, 0x00197694)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001976a0);
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x001976a0)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001976b8);
                                c.g[A3][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x158));
                                call_ptr(c, 0x00158848, 0x001976b8)?;
                                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[RA][0] = sx(0x001976cc);
                                c.g[A3][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x160));
                                call_ptr(c, 0x00158820, 0x001976cc)?;
                                c.g[RA][0] = sx(0x001976d4);
                                call_ptr(c, 0x0028b180, 0x001976d4)?;
                                c.g[A1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x158));
                                c.g[RA][0] = sx(0x001976e0);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0028c2d0, 0x001976e0)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_197714;
                                } else {
                                    c.cycles += 10;
                                    c.g[RA][0] = sx(0x001976f4);
                                    call_ptr(c, 0x0028b180, 0x001976f4)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5558));
                                    c.g[A1][0] = 0xa;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[RA][0] = sx(0x00197710);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00197710)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    break 'b_197714;
                                }
                            }
                            'b_197734: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_197734;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x0019772c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x0019772c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1977c0;
                                    } else {
                                        break 'b_197734;
                                    }
                                }
                            }
                            c.cycles += 13;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x80;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x650));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x0019774c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0019774c)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                            c.g[RA][0] = sx(0x00197760);
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
                            call_ptr(c, 0x00195ff0, 0x00197760)?;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1977c0;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00197778);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x00197778)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1977c0;
                                } else {
                                    break 'b_197780;
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.g[RA][0] = sx(0x001977a4);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x001986d0, 0x001977a4)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001977b0);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x001981f8, 0x001977b0)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001977bc);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00198340, 0x001977bc)?;
                        break 'b_1977bc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1977c0;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1977c4;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `int cFEStateRequestLine_OnWidgetNotify(self, void*, int msg, int k)` (0x1977d0): the jukebox request list widget messages: 6 asks whether row `k` may be toggled (0x10 refuse, 0x101 ok), 8 plays/queues the song, 4 scrolls the list and refreshes it, 9 returns 0.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_1979c8: {
            'b_1979c4: {
                'b_1979c0: {
                    'b_1979b4: {
                        'b_197998: {
                            'b_197960: {
                                'b_197958: {
                                    'b_197910: {
                                        'b_197908: {
                                            'b_197824: {
                                                c.cycles += 7;
                                                c.g[SP][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                                                c.g[V0][0] = 0x6;
                                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.g[S0][0] = c.g[A0][0];
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                                                if t {
                                                    break 'b_197824;
                                                } else {
                                                    'b_197808: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x4;
                                                        if t {
                                                            break 'b_197808;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] == c.g[V0][0];
                                                            c.g[V0][0] = 0x101;
                                                            if t {
                                                                break 'b_1979b4;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_1979c8;
                                                                } else {
                                                                    break 'b_197808;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x8;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0x9;
                                                    if t {
                                                        break 'b_197910;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0x101;
                                                        if t {
                                                            break 'b_197908;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_1979c8;
                                                            } else {
                                                                break 'b_197824;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_19788c: {
                                                c.cycles += 15;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                                                c.g[A0][0] = 0x1;
                                                c.g[V1][0] =
                                                    c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A3][0] as u32)));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                c.g[A0][0] = c.g[A0][0] << (c.g[A1][0] & 63);
                                                c.g[A0][0] = !(0u64 | c.g[A0][0]);
                                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
                                                c.g[V1][0] = c.g[V1][0] << 32;
                                                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                                                if t {
                                                    break 'b_19788c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x10;
                                                    if t {
                                                        break 'b_1979c4;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[V1][0] = c
                                                            .ld((c.g[S0][0] as u32)
                                                                .wrapping_add(0x160));
                                                        c.g[V1][0] =
                                                            c.g[V1][0] >> (c.g[A1][0] & 63);
                                                        c.g[V1][0] = c.g[V1][0] & 0x1;
                                                        c.g[V1][0] = c.g[V1][0] << 32;
                                                        c.g[V1][0] =
                                                            ((c.g[V1][0] as i64) >> 32) as u64;
                                                        let t = c.g[V1][0] == 0u64;
                                                        c.g[V0][0] = 0x101;
                                                        if t {
                                                            break 'b_1979c4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = 0x10;
                                                            if t {
                                                                break 'b_1979c4;
                                                            } else {
                                                                break 'b_19788c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_1978b8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1978b8;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        c.ld(
                                                            (c.g[S0][0] as u32).wrapping_add(0x160)
                                                        );
                                                    c.g[V0][0] = c.g[V0][0] >> (c.g[A1][0] & 63);
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[V0][0] = c.g[V0][0] << 32;
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x101;
                                                    if t {
                                                        break 'b_197958;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_1979c8;
                                                        } else {
                                                            break 'b_1978b8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x001978c0);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x00198ae8, 0x001978c0)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                                            c.g[V1][0] =
                                                ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A2][0] = 0u64;
                                            if t {
                                                break 'b_1979c0;
                                            } else {
                                                c.cycles += 2;
                                                c.g[A0][0] =
                                                    c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                                c.g[A1][0] = 0u64;
                                                'l_1978d8: loop {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = c.g[A0][0] >> (c.g[A1][0] & 63);
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    c.g[V1][0] =
                                                        ((c.g[A1][0] as i64) < 64i64) as u64;
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A2][0] as u32)));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001978d8);
                                                        }
                                                        continue 'l_1978d8;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            ((c.g[A2][0] as i64) < 6i64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x101;
                                                        if t {
                                                            break 'b_197958;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_1979c8;
                                                            } else {
                                                                break 'b_197908;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1979c4;
                                        } else {
                                            break 'b_197910;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14c));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1979c0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        if t {
                                            break 'b_197998;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                            c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] << 32;
                                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                            let t = c.g[V0][0] != 0u64;

                                            if t {
                                                break 'b_197960;
                                            } else {
                                                break 'b_197958;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x10;
                                if t {
                                    break 'b_1979c4;
                                } else {
                                    break 'b_197960;
                                }
                            }
                            c.cycles += 14;
                            c.g[RA][0] = sx(0x00197968);
                            call_ptr(c, 0x0028b180, 0x00197968)?;
                            c.g[RA][0] = sx(0x00197970);
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x508), c.g[S1][0]);
                            call_ptr(c, 0x0028b180, 0x00197970)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0xffffffffffffffff;
                            c.g[T0][0] = 0x1;
                            c.g[RA][0] = sx(0x00197988);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0028cf98, 0x00197988)?;
                            c.g[RA][0] = sx(0x00197990);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x00197e70,
                                f: ee::s02::cFEStateRequestLine_UpdateNowPlaying,
                            }
                            .call(c, 0x00197990)?;
                            let t = true;
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_1979c4;
                            } else {
                                break 'b_197998;
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001979a0);
                        call_ptr(c, 0x0028b180, 0x001979a0)?;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                        c.g[RA][0] = sx(0x001979ac);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x002b4978, 0x001979ac)?;
                        let t = true;
                        c.g[V0][0] = 0x101;
                        if t {
                            break 'b_1979c4;
                        } else {
                            break 'b_1979b4;
                        }
                    }
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A3][0]);
                    c.g[RA][0] = sx(0x001979c0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00197b88,
                        f: ee::s02::cFEStateRequestLine_RefreshList,
                    }
                    .call(c, 0x001979c0)?;
                    break 'b_1979c0;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x101;
                break 'b_1979c4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1979c8;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRequestLine_UpdateSongIcons(self)` (0x197ca0): hide the eight check-mark icons, then show and set the icon (check box, empty box or dollar sign) of every visible song row.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_song_icons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        'l_197cd8: loop {
            'b_197d84: {
                c.cycles += 13;
                c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00197cf4);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00197cf4)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                if t {
                    break 'b_197d84;
                } else {
                    'b_197d7c: {
                        'b_197d58: {
                            c.cycles += 16;
                            c.g[A3][0] = c.lw((c.g[S2][0] as u32));
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A1][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[S1][0] = c.lw((c.g[V0][0] as u32));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x00197d30);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00197d30)?;
                            c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                            c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                break 'b_197d58;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x204));
                                if t {
                                    break 'b_197d7c;
                                } else {
                                    break 'b_197d58;
                                }
                            }
                        }
                        'b_197d78: {
                            c.cycles += 6;
                            c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                            if t {
                                break 'b_197d78;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x208));
                                if t {
                                    break 'b_197d7c;
                                } else {
                                    break 'b_197d78;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20c));
                        break 'b_197d7c;
                    }
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[S4][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                    break 'b_197d84;
                }
            }
            c.cycles += 4;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00197cd8);
                }
                continue 'l_197cd8;
            } else {
                c.cycles += 9;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateRequestLine_UpdateNowPlaying(self)` (0x197e70): format the "now playing" title and artist lines from the music system, hiding the lines when the text is empty.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_now_playing(c: &mut Cpu) {
    guest(c, |c| {
        'b_198100: {
            'b_1980fc: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                let t = c.g[V0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1980fc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_198100;
                    } else {
                        'b_1980c4: {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x00197eb0);
                            call_ptr(c, 0x0028b180, 0x00197eb0)?;
                            c.g[RA][0] = sx(0x00197eb8);
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x508));
                            call_ptr(c, 0x0028b180, 0x00197eb8)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V1][0] == 0u64 {
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                break 'b_1980c4;
                            } else {
                                'b_197fd8: {
                                    'b_197fbc: {
                                        c.cycles += 15;
                                        c.g[RA][0] = sx(0x00197ed0);
                                        call_ptr(c, 0x0028b180, 0x00197ed0)?;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                        c.g[RA][0] = sx(0x00197edc);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x002b40f0, 0x00197edc)?;
                                        c.g[RA][0] = sx(0x00197ee4);
                                        c.g[S3][0] = c.g[V0][0];
                                        call_ptr(c, 0x0028b180, 0x00197ee4)?;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                        c.g[RA][0] = sx(0x00197ef0);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x002b4120, 0x00197ef0)?;
                                        c.g[S4][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00197efc);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x00416810, 0x00197efc)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_197fbc;
                                        } else {
                                            c.cycles += 42;
                                            c.g[A2][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00197f1c);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00197f1c)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x668));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x00197f3c);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x00197f3c)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A1][0] = c.g[V0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00197f4c);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, t, 0x00197f4c)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00197f58);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x003dbca0, 0x00197f58)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00197f64);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x00416810, 0x00197f64)?;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A1][0] = 0x460000;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x470));
                                            c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                                            c.g[A2][0] = 0x100;
                                            c.g[RA][0] = sx(0x00197f84);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317e30, 0x00197f84)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x00197f98);
                                            c.g[A2][0] = c.g[S3][0];
                                            call_ptr(c, 0x002c26d0, 0x00197f98)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                            c.g[RA][0] = sx(0x00197fa4);
                                            c.g[A1][0] = c.g[S0][0];
                                            call_ptr(c, 0x003a0e90, 0x00197fa4)?;
                                            let t = c.g[S0][0] == 0u64;

                                            if t {
                                                break 'b_197fd8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00197fb4);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x00317e98, 0x00197fb4)?;
                                                let t = true;

                                                if t {
                                                    break 'b_197fd8;
                                                } else {
                                                    break 'b_197fbc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00197fd8);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x00197fd8)?;
                                    break 'b_197fd8;
                                }
                                'b_1980a0: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00197fe0);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x00416810, 0x00197fe0)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1980a0;
                                    } else {
                                        c.cycles += 42;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00198000);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x00198000)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x678));
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x00198020);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        call_ptr(c, 0x00317670, 0x00198020)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A1][0] = c.g[V0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00198030);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, t, 0x00198030)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x0019803c);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x003dbca0, 0x0019803c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00198048);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x00416810, 0x00198048)?;
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A1][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                                        c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x00198068);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317e30, 0x00198068)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0019807c);
                                        c.g[A2][0] = c.g[S4][0];
                                        call_ptr(c, 0x002c26d0, 0x0019807c)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                                        c.g[RA][0] = sx(0x00198088);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a0e90, 0x00198088)?;
                                        if c.g[S0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            break 'b_1980fc;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x00198098);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x00317e98, 0x00198098)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1980fc;
                                            } else {
                                                break 'b_1980a0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 9;
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                                c.g[A1][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001980bc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001980bc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1980fc;
                                } else {
                                    break 'b_1980c4;
                                }
                            }
                        }
                        c.cycles += 14;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001980dc);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001980dc)?;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001980f8);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001980f8)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1980fc;
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_198100;
        }
        c.cycles += 6;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRequestLine_UpdateHelpText(self, int idx)` (0x1981f8): choose the help string for song `idx` (owned or not, in the store or not).
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_help_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_198334: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_198334;
            } else {
                'b_19831c: {
                    'b_198300: {
                        'b_198268: {
                            c.cycles += 7;
                            c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                            c.g[V0][0] = c.g[V0][0] >> (c.g[A1][0] & 63);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_198268;
                            } else {
                                'b_198260: {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                                    c.g[V1][0] = c.g[V1][0] << (c.g[A1][0] & 63);
                                    c.g[V1][0] = !(0u64 | c.g[V1][0]);
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                    c.g[V0][0] = c.g[V0][0] << 32;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_198260;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b0));
                                        if t {
                                            break 'b_198300;
                                        } else {
                                            break 'b_198260;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                                if t {
                                    break 'b_198300;
                                } else {
                                    break 'b_198268;
                                }
                            }
                        }
                        'b_1982d4: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_1982d4;
                            } else {
                                'b_1982cc: {
                                    'b_1982c0: {
                                        c.cycles += 6;
                                        c.g[RA][0] = sx(0x0019827c);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00198ae8, 0x0019827c)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                                        c.g[V1][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[A1][0] = 0u64;
                                        if t {
                                            break 'b_1982c0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A2][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                            c.g[A0][0] = 0u64;
                                            'l_198298: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00198298);
                                                    }
                                                    continue 'l_198298;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 6i64) as u64;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1982cc;
                                                    } else {
                                                        break 'b_1982c0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e0));
                                    if t {
                                        break 'b_198300;
                                    } else {
                                        break 'b_1982cc;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f8));
                                if t {
                                    break 'b_198300;
                                } else {
                                    break 'b_1982d4;
                                }
                            }
                        }
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x001982dc);
                        call_ptr(c, 0x0014dc80, 0x001982dc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001982e8);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001982e8)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_19831c;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x710));
                            break 'b_198300;
                        }
                    }
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00198308);
                    call_ptr(c, 0x00317670, 0x00198308)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                    c.g[RA][0] = sx(0x00198314);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00198314)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_198334;
                    } else {
                        break 'b_19831c;
                    }
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x00198324);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x720));
                call_ptr(c, 0x00317670, 0x00198324)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                c.g[RA][0] = sx(0x00198330);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x00198330)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_198334;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRequestLine_UpdateButtons(self, int idx)` (0x198340): set the add/remove/buy button caption for song `idx` and the visibility of the button icons.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_buttons(c: &mut Cpu) {
    guest(c, |c| {
        'b_1984f8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            let t = c.g[A0][0] == 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_1984f8;
            } else {
                'b_1984bc: {
                    'b_19844c: {
                        c.cycles += 25;
                        c.g[A1][0] = 0x4a0000;
                        c.g[S2][0] = 0u64;
                        c.g[RA][0] = sx(0x00198374);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                        call_ptr(c, 0x003a0c50, 0x00198374)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00198390);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00198390)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001983ac);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001983ac)?;
                        c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                        c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[V0][0] = c.g[V0][0] << 32;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'b_19844c;
                        } else {
                            'b_198430: {
                                'b_198424: {
                                    'b_198410: {
                                        c.cycles += 10;
                                        c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                                        c.g[V1][0] = c.g[V1][0] << (c.g[S1][0] & 63);
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V1][0] = !(0u64 | c.g[V1][0]);
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                        c.g[V0][0] = c.g[V0][0] << 32;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.g[A0][0] >> (c.g[S1][0] & 63);
                                        if t {
                                            break 'b_198410;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] << 32;
                                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_198424;
                                            } else {
                                                c.cycles += 3;
                                                c.g[S2][0] = 0x1;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x738));
                                                if t {
                                                    break 'b_198430;
                                                } else {
                                                    break 'b_198410;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    c.g[V0][0] = c.g[V0][0] << 32;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1984bc;
                                    } else {
                                        break 'b_198424;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x460000;
                                c.g[S2][0] = 0x1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x748));
                                break 'b_198430;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x00198438);
                            call_ptr(c, 0x00317670, 0x00198438)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                            c.g[RA][0] = sx(0x00198444);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00198444)?;
                            let t = true;

                            if t {
                                break 'b_1984bc;
                            } else {
                                break 'b_19844c;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_1984bc;
                    } else {
                        'b_1984a0: {
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x00198460);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00198ae8,
                                f: ee::s03::cFEStateRequestLine_GetSongPrice,
                            }
                            .call(c, 0x00198460)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1984a0;
                            } else {
                                c.cycles += 2;
                                c.g[A2][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                c.g[A0][0] = 0u64;
                                'l_198478: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[A1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00198478);
                                        }
                                        continue 'l_198478;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[A1][0] as i64) < 6i64) as u64;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_1984bc;
                                        } else {
                                            break 'b_1984a0;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A0][0] = 0x460000;
                        c.g[S2][0] = 0x1;
                        c.g[RA][0] = sx(0x001984b0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x758));
                        call_ptr(c, 0x00317670, 0x001984b0)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[RA][0] = sx(0x001984bc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001984bc)?;
                        break 'b_1984bc;
                    }
                }
                c.cycles += 2;
                let t = c.g[S2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1984f8;
                } else {
                    c.cycles += 13;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001984dc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001984dc)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001984f8);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001984f8)?;
                    break 'b_1984f8;
                }
            }
        }
        'b_198598: {
            'b_198560: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                let t = c.g[V1][0] == 0u64;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1fc));
                if t {
                    break 'b_198560;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                    c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_198560;
                    } else {
                        c.cycles += 15;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0019853c);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019853c)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00198558);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00198558)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_198598;
                        } else {
                            break 'b_198560;
                        }
                    }
                }
            }
            c.cycles += 14;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00198578);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00198578)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00198594);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00198594)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_198598;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateRequestLine::ShowSongInfo(this, idx)` (0x1986d0): show the
/// highlighted song's "title - artist" line in the info text widget
/// (`this+0x1d4`) and, when song `idx` is not owned yet, its price in the
/// price widget (`this+0x1d8`); owned songs clear the price and hide the
/// widget at `this+0x1dc`.
pub fn cfestate_request_line_show_song_info(c: &mut Cpu) {
    /// Widget pointers of the request line.
    const INFO_TEXT: u32 = 0x1d4;
    const PRICE_TEXT: u32 = 0x1d8;
    const PRICE_BOX: u32 = 0x1dc;
    const OWNED_MASK: u32 = 0x160;
    const SHOWN_FLAG: u32 = 0x16c;
    const GET_ARTIST: Callee = Callee {
        addr: 0x0019_8ab8,
        f: ee::s03::cFEStateRequestLine_GetSongArtist,
    };
    const GET_PRICE: Callee = Callee {
        addr: 0x0019_8ae8,
        f: ee::s03::cFEStateRequestLine_GetSongPrice,
    };
    const FORMAT_MONEY: Callee = Callee {
        addr: 0x0019_8af0,
        f: ee::s03::FE_FormatMoney,
    };

    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        let this = c.g[A0][0];
        c.g[S3][0] = this;
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(INFO_TEXT)).lw(c);
        c.g[S4][0] = c.g[A1][0];
        'text_done: {
            if c.g[V0][0] != 0 {
                c.cycles += 47;
                let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = world;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x770));
                c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
                c.g[S1][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x0019_871c)?;
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x24)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let method = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, method, 0x0019_872c)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                GET_ARTIST.call(c, 0x0019_873c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x003d_bca0, 0x0019_8748)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0041_6810, 0x0019_8754)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x460000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A3][0] = 0;
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x688));
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x0031_7e30, 0x0019_8774)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x002c_26d0, 0x0019_8788)?;
                let widget = Addr(r32(c, S3).wrapping_add(INFO_TEXT)).lw(c);
                c.g[A2][0] = widget;
                c.g[A1][0] = 1;
                let vt = Addr((widget as u32).wrapping_add(8)).lw(c);
                c.g[V0][0] = vt;
                c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
                let method = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0019_87a4)?;
                c.g[A0][0] = Addr(r32(c, S3).wrapping_add(INFO_TEXT)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0e90, 0x0019_87b0)?;
                if c.g[S0][0] == 0 {
                    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
                    break 'text_done;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x0019_87c0)?;
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
        }
        // `a0` = the price widget.
        'price_done: {
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, 16, 0x50);
                break 'price_done;
            }
            'hide_box: {
                'price: {
                    'show_box: {
                        c.cycles += 7;
                        c.g[A2][0] = Addr(r32(c, S3).wrapping_add(OWNED_MASK)).ld(c);
                        c.g[V0][0] = c.g[A2][0] >> (c.g[S4][0] & 63);
                        c.g[V0][0] &= 1;
                        c.g[V0][0] <<= 32;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        c.g[A1][0] = 0x4a0000;
                        if c.g[V0][0] != 0 {
                            // Song owned: blank the price.
                            c.cycles += 5;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                            call_ptr(c, 0x003a_0c50, 0x0019_87f0)?;
                            c.g[A2][0] = Addr(r32(c, S3).wrapping_add(PRICE_BOX)).lw(c);
                            c.g[A1][0] = 0;
                            break 'show_box;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = 0;
                        // Population count of the 64-bit owned mask.
                        loop {
                            c.cycles += 7;
                            c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                            c.g[V0][0] &= 1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                            c.g[V1][0] = ((c.g[A0][0] as i64) < 64) as u64;
                            let more = c.g[V1][0] != 0;
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019_8808);
                                }
                                continue;
                            }
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
                        let few = c.g[V0][0] != 0;
                        c.g[A0][0] = 0x460000;
                        if !few {
                            break 'price;
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x780));
                        call_ptr(c, 0x0031_7670, 0x0019_8838)?;
                        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a_1000, 0x0019_8844)?;
                        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SHOWN_FLAG)).lw(c);
                        c.g[A1][0] = 1;
                        if c.g[V0][0] == 0 {
                            break 'hide_box;
                        }
                        c.cycles += 1;
                        c.g[A2][0] = Addr(r32(c, S3).wrapping_add(PRICE_BOX)).lw(c);
                        break 'show_box;
                    }
                    // Show/hide the box widget through its vtable.
                    c.cycles += 7;
                    let widget = c.g[A2][0] as u32;
                    let vt = Addr(widget.wrapping_add(8)).lw(c);
                    c.g[V0][0] = vt;
                    c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
                    c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_8868)?;
                    frame.restore(c, 16, 0x50);
                    break 'price_done;
                }
                // Fewer than six songs owned: price line.
                c.cycles += 11;
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[S3][0];
                GET_PRICE.call(c, 0x0019_887c)?;
                c.g[A0][0] = c.g[V0][0];
                FORMAT_MONEY.call(c, 0x0019_8884)?;
                c.g[A0][0] = Addr(r32(c, S3).wrapping_add(PRICE_TEXT)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x0019_8890)?;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SHOWN_FLAG)).lw(c);
                c.g[A1][0] = 1;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    let widget = Addr(r32(c, S3).wrapping_add(PRICE_BOX)).lw(c);
                    c.g[A2][0] = widget;
                    let vt = Addr((widget as u32).wrapping_add(8)).lw(c);
                    c.g[V0][0] = vt;
                    c.g[A0][0] = Addr((vt as u32).wrapping_add(0x48)).lh(c);
                    c.g[V1][0] = Addr((vt as u32).wrapping_add(0x4c)).lw(c);
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0019_88b4)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `GetSongTitle`/`GetSongArtist`: `MusicPlayer_GetInstance`
/// (0x28b180) and then the music player's text getter `getter` on
/// `player + 0x118` with the song index.
pub(crate) fn oc855_song_text(c: &mut Cpu, getter: u32, ret_player: u32, ret_getter: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0028_b180, ret_player)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, getter, ret_getter)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `const char* cFEStateRequestLine::GetSongTitle(this, song)` (0x198a88):
/// the music player's title of song `song`.
pub fn cfestate_request_line_get_song_title(c: &mut Cpu) {
    oc855_song_text(c, 0x002b_40f0, 0x0019_8a9c, 0x0019_8aa8);
}

/// `const char* cFEStateRequestLine::GetSongArtist(this, song)` (0x198ab8):
/// the music player's artist of song `song`.
pub fn cfestate_request_line_get_song_artist(c: &mut Cpu) {
    oc855_song_text(c, 0x002b_4120, 0x0019_8acc, 0x0019_8ad8);
}

/// `int cFEStateRequestLine::GetSongPrice(this, song)` (0x198ae8): every
/// song costs 5000.
pub fn cfestate_request_line_get_song_price(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 5000;
    ret(c);
}

/// `char* FE_FormatMoney(int amount)` (0x198af0): format `amount` with the
/// locale's currency settings into the shared buffer at 0x4c6310 and return
/// it: the optional sign and `$` prefix, the digits grouped in threes with
/// the locale's separator, and the trailing `$` for locales that put the
/// symbol last.
pub fn oc855_fe_format_money(c: &mut Cpu) {
    /// Size of the locale record (at 0x535610) copied to the stack, less the
    /// last 8 bytes.
    const LOCALE_SIZE: u32 = 0x280;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x2f0);
        frame.save(c, 17, 0x2d0);
        frame.save(c, 18, 0x2c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x2b0);
        c.g[S2][0] = 0x20;
        frame.save(c, 16, 0x2e0);
        frame.save(c, 20, 0x2a0);
        frame.save_ra(c, 0x290);
        c.g[S3][0] = 1;
        call_ptr(c, 0x0014_dc80, 0x0019_8b1c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x0019_8b28)?;

        // Copy the locale record to the stack (0x20 bytes per round). The
        // record's address is a constant that is 8-byte aligned, so the
        // generated body's unaligned (`ldl`/`ldr`) copy loop is never taken
        // and is not reproduced here.
        c.g[V1][0] = 0x530000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
        c.g[V0][0] = c.g[V1][0] & 7;
        c.g[A0][0] = c.g[SP][0];
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(LOCALE_SIZE));
        c.g[S4][0] = 0x4c0000;
        c.g[A1][0] = 0x4a0000;
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            let dst = c.g[A0][0] as u32;
            for (i, reg) in [A2, A3, T0, T1].into_iter().enumerate() {
                c.g[reg][0] = Addr(src.wrapping_add(8 * i as u32)).ld(c);
            }
            for (i, reg) in [A2, A3, T0, T1].into_iter().enumerate() {
                let v = c.g[reg][0];
                Addr(dst.wrapping_add(8 * i as u32)).sd(c, v);
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_8ba8);
                }
                continue;
            }
            break;
        }
        // The last 8 bytes.
        c.cycles += 10;
        let v = Addr(c.g[V1][0] as u32).read_u64_unaligned(c);
        c.g[V0][0] = v;
        Addr(c.g[A0][0] as u32).write_u64_unaligned(c, v);
        c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
        c.g[V1][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
        c.g[A0][0] = c.g[V0][0] & 7;
        // Currency format 0..3: where the symbol goes and which separator.
        let known = 'format: {
            if c.g[A0][0] == c.g[V1][0] {
                c.g[S3][0] = 0;
                break 'format true;
            }
            c.cycles += 2;
            c.g[V0][0] = 2;
            if c.g[A0][0] == 0 {
                c.cycles += 2;
                c.g[S2][0] = 0x2c;
                break 'format true;
            }
            c.cycles += 2;
            let is2 = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is2 {
                c.cycles += 2;
                c.g[S2][0] = 0x2e;
                break 'format true;
            }
            c.cycles += 2;
            let is3 = c.g[A0][0] == c.g[V0][0];
            c.g[S0][0] = 0;
            if is3 {
                c.cycles += 1;
                c.g[S3][0] = 0;
                break 'format true;
            }
            c.cycles += 2;
            false
        };
        if known {
            c.cycles += 1;
            c.g[S0][0] = 0;
        }
        // Leading "$ " when the symbol goes first.
        c.cycles += 2;
        c.g[V1][0] = 0x24;
        if c.g[S3][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
            c.g[A0][0] = 0x20;
            c.sb((c.g[S4][0] as u32).wrapping_add(0x6310), c.g[V1][0]);
            c.sb((c.g[V0][0] as u32).wrapping_add(1), c.g[A0][0]);
            c.g[S0][0] = 2;
        }
        // Sign.
        c.cycles += 2;
        if (c.g[S1][0] as i64) >= 0 {
            c.g[A0][0] = c.g[SP][0];
        } else {
            c.cycles += 10;
            c.g[V0][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = 0x2d;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sb(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[S1][0] = sx(0u32.wrapping_sub(c.g[S1][0] as u32));
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[A0][0] = c.g[SP][0];
        }
        // Decimal digits of |amount| onto the stack buffer, then copy them
        // to the output with a separator before every group of three.
        c.cycles += 9;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x0019_8c8c)?;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0041_6810, 0x0019_8c94)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        let empty = c.g[A3][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 9;
            c.g[V0][0] = 0x4c0000;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6310));
            c.g[T1][0] = 3;
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T3][0] = 0;
            c.g[T0][0] = 0x0100_0000;
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.g[T2][0] = 0x0100_0000;
            loop {
                'digit_done: {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 24);
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                    c.g[S0][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                    let last = c.g[V0][0] == 0;
                    c.sb(c.g[A1][0] as u32, c.g[A0][0]);
                    'group: {
                        if !last {
                            c.cycles += 3;
                            c.div(0, 2, 9);
                            c.cycles += 3;
                            c.g[V0][0] = c.hi[0];
                            let mid_group = c.g[V0][0] != 0;
                            c.g[V0][0] = c.g[T0][0];
                            if mid_group {
                                break 'digit_done;
                            }
                            // A group boundary: insert the separator.
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                            c.sb(c.g[A1][0] as u32, c.g[S2][0]);
                            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            break 'group;
                        }
                        break 'group;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[T0][0];
                }
                c.cycles += 4;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                let more = c.g[A0][0] != c.g[A3][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_8cc8);
                    }
                    continue;
                }
                break;
            }
        }
        // Trailing " $" when the symbol goes last.
        c.cycles += 2;
        let symbol_first = c.g[S3][0] != 0;
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
        if !symbol_first {
            c.cycles += 16;
            c.g[V1][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.g[A0][0] = 0x20;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.sb(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[V0][0] = c.g[S0][0];
            c.g[V1][0] = 0x24;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sb(c.g[A1][0] as u32, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6310));
        }
        c.cycles += 10;
        frame.restore(c, 17, 0x2d0);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, 18, 0x2c0);
        frame.restore(c, 16, 0x2e0);
        frame.restore(c, 19, 0x2b0);
        frame.restore(c, 20, 0x2a0);
        frame.restore_ra(c, 0x290);
        c.sb(c.g[V1][0] as u32, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRequestLine_OnWidgetCreated(self, widget)` (0x00196f98): the
/// song-request line screen's widget-created hook. Hashes the widget's name
/// (`widget+0x38`) against the names it knows (current song/artist text,
/// help text, arrows, song list, cost/you-have lines, buttons, the 8+8 slot
/// widgets and the 6 option widgets), stores the widget in the matching
/// `self` field and hides or shows it through its virtual `setVisible`
/// (slot `+0x48`/`+0x4c`); the song-count line formats the locale string
/// with the bit count of `self+0x158`.
///
/// The normal entry, converted from the generated body (a `match pc` body:
/// every register, cycle and call kept). Any other `pc` (a resume after an
/// unwound callee or event) runs the generated body.
pub fn cfe_state_request_line_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x00196f98 {
        return cfe_state_request_line_on_widget_created_resume(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00196f98;
        loop {
            match pc {
                0x00196f98 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A0][0] = 0x460000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x588));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x00196fdc);
                    c.g[S4][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x00196fdc)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x00196ff0;
                        continue;
                    }
                    pc = 0x00196fe4;
                    continue;
                }
                0x00196fe4 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x4a0000;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[S2][0]);
                    if t {
                        pc = 0x00197008;
                        continue;
                    }
                    pc = 0x00196ff0;
                    continue;
                }
                0x00196ff0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00196ff8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x598));
                    call_ptr(c, 0x00317670, 0x00196ff8)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x0019701c;
                        continue;
                    }
                    pc = 0x00197000;
                    continue;
                }
                0x00197000 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S2][0]);
                    pc = 0x00197008;
                    continue;
                }
                0x00197008 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00197014);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x00197014)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x0019701c;
                    continue;
                }
                0x0019701c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197024);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    call_ptr(c, 0x00317670, 0x00197024)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197034;
                        continue;
                    }
                    pc = 0x0019702c;
                    continue;
                }
                0x0019702c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e8), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x00197034;
                    continue;
                }
                0x00197034 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019703c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18b8));
                    call_ptr(c, 0x00317670, 0x0019703c)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x0019704c;
                        continue;
                    }
                    pc = 0x00197044;
                    continue;
                }
                0x00197044 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1ec), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x0019704c;
                    continue;
                }
                0x0019704c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197054);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a8));
                    call_ptr(c, 0x00317670, 0x00197054)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00197064;
                        continue;
                    }
                    pc = 0x0019705c;
                    continue;
                }
                0x0019705c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1f0), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x00197064;
                    continue;
                }
                0x00197064 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019706c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b8));
                    call_ptr(c, 0x00317670, 0x0019706c)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00197080;
                        continue;
                    }
                    pc = 0x00197074;
                    continue;
                }
                0x00197074 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197080;
                    continue;
                }
                0x00197080 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197088);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c8));
                    call_ptr(c, 0x00317670, 0x00197088)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197148;
                        continue;
                    }
                    pc = 0x00197090;
                    continue;
                }
                0x00197090 => {
                    c.cycles += 26;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d8));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001970b0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001970b0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = c.g[V0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001970c0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, t, 0x001970c0)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001970cc);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003dbca0, 0x001970cc)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x001970e8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x001970e8)?;
                    c.g[A1][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x158));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = 0u64;
                    pc = 0x001970f8;
                    continue;
                }
                0x001970f8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001970f8) {
                            return Ok(());
                        }
                        pc = 0x001970f8;
                        continue;
                    }
                    pc = 0x00197114;
                    continue;
                }
                0x00197114 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00197120);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c26d0, 0x00197120)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e4), c.g[S2][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00197130);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x00197130)?;

                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197138;
                    continue;
                }
                0x00197138 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197140);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x00197140)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197148;
                    continue;
                }
                0x00197148 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197150);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c0));
                    call_ptr(c, 0x00317670, 0x00197150)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197164;
                        continue;
                    }
                    pc = 0x00197158;
                    continue;
                }
                0x00197158 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197164;
                    continue;
                }
                0x00197164 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019716c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c8));
                    call_ptr(c, 0x00317670, 0x0019716c)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197180;
                        continue;
                    }
                    pc = 0x00197174;
                    continue;
                }
                0x00197174 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1d4), c.g[S2][0]);
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197354;
                        continue;
                    }
                    pc = 0x00197180;
                    continue;
                }
                0x00197180 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197188);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18d0));
                    call_ptr(c, 0x00317670, 0x00197188)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001971bc;
                        continue;
                    }
                    pc = 0x00197190;
                    continue;
                }
                0x00197190 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x0019719c;
                    continue;
                }
                0x0019719c => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001971b4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001971b4)?;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x001971bc;
                    continue;
                }
                0x001971bc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001971c4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f0));
                    call_ptr(c, 0x00317670, 0x001971c4)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001971f8;
                        continue;
                    }
                    pc = 0x001971cc;
                    continue;
                }
                0x001971cc => {
                    c.cycles += 8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1d8), c.g[S2][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    c.g[RA][0] = sx(0x001971e0);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003a0c50, 0x001971e0)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x001971ec;
                    continue;
                }
                0x001971ec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x001971f8;
                    continue;
                }
                0x001971f8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197200);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
                    call_ptr(c, 0x00317670, 0x00197200)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00197220;
                        continue;
                    }
                    pc = 0x00197208;
                    continue;
                }
                0x00197208 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197214;
                    continue;
                }
                0x00197214 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197220;
                    continue;
                }
                0x00197220 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197228);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x610));
                    call_ptr(c, 0x00317670, 0x00197228)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x00197260;
                        continue;
                    }
                    pc = 0x00197230;
                    continue;
                }
                0x00197230 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x168));
                    c.g[RA][0] = sx(0x0019723c);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e0), c.g[S2][0]);
                    call_ptr(c, 0x00198af0, 0x0019723c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1e0));
                    c.g[RA][0] = sx(0x00197248);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x00197248)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197254;
                    continue;
                }
                0x00197254 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197260;
                    continue;
                }
                0x00197260 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197268);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x620));
                    call_ptr(c, 0x00317670, 0x00197268)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001972bc;
                        continue;
                    }
                    pc = 0x00197270;
                    continue;
                }
                0x00197270 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x001972b0;
                        continue;
                    }
                    pc = 0x0019727c;
                    continue;
                }
                0x0019727c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = 0u64;
                    pc = 0x00197288;
                    continue;
                }
                0x00197288 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A0][0] >> (c.g[A1][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x00197288) {
                            return Ok(());
                        }
                        pc = 0x00197288;
                        continue;
                    }
                    pc = 0x001972a4;
                    continue;
                }
                0x001972a4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x001972b0;
                    continue;
                }
                0x001972b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x001972bc;
                    continue;
                }
                0x001972bc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001972c4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
                    call_ptr(c, 0x00317670, 0x001972c4)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001972d4;
                        continue;
                    }
                    pc = 0x001972cc;
                    continue;
                }
                0x001972cc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x170), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x001972d4;
                    continue;
                }
                0x001972d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001972dc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x630));
                    call_ptr(c, 0x00317670, 0x001972dc)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001972f0;
                        continue;
                    }
                    pc = 0x001972e4;
                    continue;
                }
                0x001972e4 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1f4), c.g[S2][0]);
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197354;
                        continue;
                    }
                    pc = 0x001972f0;
                    continue;
                }
                0x001972f0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001972f8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x640));
                    call_ptr(c, 0x00317670, 0x001972f8)?;
                    let t = c.g[S4][0] != c.g[V0][0];

                    if t {
                        pc = 0x00197308;
                        continue;
                    }
                    pc = 0x00197300;
                    continue;
                }
                0x00197300 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1fc), c.g[S2][0]);
                    if t {
                        pc = 0x00197350;
                        continue;
                    }
                    pc = 0x00197308;
                    continue;
                }
                0x00197308 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197310);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1850));
                    call_ptr(c, 0x00317670, 0x00197310)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0019733c;
                        continue;
                    }
                    pc = 0x00197318;
                    continue;
                }
                0x00197318 => {
                    c.cycles += 9;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1f8), c.g[S2][0]);
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197334);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00197334)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x0019733c;
                    continue;
                }
                0x0019733c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197344);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x184c));
                    call_ptr(c, 0x00317670, 0x00197344)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V1][0] = 0x440000;
                    if t {
                        pc = 0x00197398;
                        continue;
                    }
                    pc = 0x0019734c;
                    continue;
                }
                0x0019734c => {
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x200), c.g[S2][0]);
                    pc = 0x00197350;
                    continue;
                }
                0x00197350 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x1;
                    pc = 0x00197354;
                    continue;
                }
                0x00197354 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x00197358;
                    continue;
                }
                0x00197358 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197368);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00197368)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197370;
                    continue;
                }
                0x00197370 => {
                    c.cycles += 10;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x194), c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197390);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00197390)?;
                    let t = true;

                    if t {
                        pc = 0x00197404;
                        continue;
                    }
                    pc = 0x00197398;
                    continue;
                }
                0x00197398 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11c0));
                    c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x11a0));
                    c.g[S1][0] = 0u64;
                    let t = true;
                    c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                    if t {
                        pc = 0x001973c4;
                        continue;
                    }
                    pc = 0x001973b0;
                    continue;
                }
                0x001973b0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001973b8);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x00317670, 0x001973b8)?;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00197370) {
                            return Ok(());
                        }
                        pc = 0x00197370;
                        continue;
                    }
                    pc = 0x001973c0;
                    continue;
                }
                0x001973c0 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x001973c4;
                    continue;
                }
                0x001973c4 => {
                    c.cycles += 3;
                    c.g[S6][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    let t = c.g[S6][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00197414;
                        continue;
                    }
                    pc = 0x001973d0;
                    continue;
                }
                0x001973d0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(c.g[S5][0] as u32);
                    c.g[RA][0] = sx(0x001973dc);
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                    call_ptr(c, 0x00317670, 0x001973dc)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x001973b0) {
                            return Ok(());
                        }
                        pc = 0x001973b0;
                        continue;
                    }
                    pc = 0x001973e4;
                    continue;
                }
                0x001973e4 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.sw(c.g[V1][0] as u32, c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197404);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00197404)?;
                    pc = 0x00197404;
                    continue;
                }
                0x00197404 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00197414;
                        continue;
                    }
                    pc = 0x0019740c;
                    continue;
                }
                0x0019740c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x00197414;
                    continue;
                }
                0x00197414 => {
                    c.cycles += 5;
                    c.g[S1][0] = 0u64;
                    c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1200));
                    c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1bc));
                    c.g[S7][0] = 0x6;
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                    pc = 0x00197428;
                    continue;
                }
                0x00197428 => {
                    c.cycles += 6;
                    c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    c.g[RA][0] = sx(0x00197438);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x00317670, 0x00197438)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[S1][0] = c.g[S5][0];
                        pc = 0x001974c4;
                        continue;
                    }
                    pc = 0x00197440;
                    continue;
                }
                0x00197440 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V1][0] == 0u64;
                    c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    if t {
                        pc = 0x001974a8;
                        continue;
                    }
                    pc = 0x00197454;
                    continue;
                }
                0x00197454 => {
                    c.cycles += 5;
                    c.g[A2][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x160));
                    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = 0u64;
                    pc = 0x00197468;
                    continue;
                }
                0x00197468 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if t {
                        if back_edge_unwinds(c, 0x00197468) {
                            return Ok(());
                        }
                        pc = 0x00197468;
                        continue;
                    }
                    pc = 0x00197484;
                    continue;
                }
                0x00197484 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x001974a8;
                        continue;
                    }
                    pc = 0x00197490;
                    continue;
                }
                0x00197490 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001974a0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001974a0)?;
                    let t = true;
                    c.g[S1][0] = c.g[S5][0];
                    if t {
                        pc = 0x001974c4;
                        continue;
                    }
                    pc = 0x001974a8;
                    continue;
                }
                0x001974a8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001974c0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001974c0)?;
                    c.g[S1][0] = c.g[S5][0];
                    pc = 0x001974c4;
                    continue;
                }
                0x001974c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x00197428) {
                            return Ok(());
                        }
                        pc = 0x00197428;
                        continue;
                    }
                    pc = 0x001974d0;
                    continue;
                }
                0x001974d0 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x001974d4;
                    continue;
                }
                0x001974d4 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00196f98, 0x00197500);
                    return Ok(());
                }
            }
        }
    });
}

/// Native resume of `cfe_state_request_line_on_widget_created` (converted from the `generated` body): any entry
/// other than 0x00196f98, after an unwound callee or event.
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn cfe_state_request_line_on_widget_created_resume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x00196f98 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A0][0] = 0x460000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x588));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x00196fdc);
                    c.g[S4][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x00196fdc)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x00196ff0;
                        continue;
                    }
                    pc = 0x00196fe4;
                    continue;
                }
                0x00196fe4 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x4a0000;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[S2][0]);
                    if t {
                        pc = 0x00197008;
                        continue;
                    }
                    pc = 0x00196ff0;
                    continue;
                }
                0x00196ff0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00196ff8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x598));
                    call_ptr(c, 0x00317670, 0x00196ff8)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x0019701c;
                        continue;
                    }
                    pc = 0x00197000;
                    continue;
                }
                0x00197000 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S2][0]);
                    pc = 0x00197008;
                    continue;
                }
                0x00197008 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00197014);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x00197014)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x0019701c;
                    continue;
                }
                0x0019701c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197024);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    call_ptr(c, 0x00317670, 0x00197024)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197034;
                        continue;
                    }
                    pc = 0x0019702c;
                    continue;
                }
                0x0019702c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e8), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x00197034;
                    continue;
                }
                0x00197034 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019703c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18b8));
                    call_ptr(c, 0x00317670, 0x0019703c)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x0019704c;
                        continue;
                    }
                    pc = 0x00197044;
                    continue;
                }
                0x00197044 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1ec), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x0019704c;
                    continue;
                }
                0x0019704c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197054);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a8));
                    call_ptr(c, 0x00317670, 0x00197054)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00197064;
                        continue;
                    }
                    pc = 0x0019705c;
                    continue;
                }
                0x0019705c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1f0), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x00197064;
                    continue;
                }
                0x00197064 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019706c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b8));
                    call_ptr(c, 0x00317670, 0x0019706c)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00197080;
                        continue;
                    }
                    pc = 0x00197074;
                    continue;
                }
                0x00197074 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197080;
                    continue;
                }
                0x00197080 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197088);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c8));
                    call_ptr(c, 0x00317670, 0x00197088)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197148;
                        continue;
                    }
                    pc = 0x00197090;
                    continue;
                }
                0x00197090 => {
                    c.cycles += 26;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d8));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001970b0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001970b0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A1][0] = c.g[V0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001970c0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, t, 0x001970c0)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001970cc);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003dbca0, 0x001970cc)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x001970e8);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x001970e8)?;
                    c.g[A1][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x158));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = 0u64;
                    pc = 0x001970f8;
                    continue;
                }
                0x001970f8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x001970f8) {
                            return Ok(());
                        }
                        pc = 0x001970f8;
                        continue;
                    }
                    pc = 0x00197114;
                    continue;
                }
                0x00197114 => {
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00197120);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c26d0, 0x00197120)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e4), c.g[S2][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00197130);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x00197130)?;

                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197138;
                    continue;
                }
                0x00197138 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197140);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x00197140)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197148;
                    continue;
                }
                0x00197148 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197150);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c0));
                    call_ptr(c, 0x00317670, 0x00197150)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197164;
                        continue;
                    }
                    pc = 0x00197158;
                    continue;
                }
                0x00197158 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197164;
                    continue;
                }
                0x00197164 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019716c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c8));
                    call_ptr(c, 0x00317670, 0x0019716c)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        pc = 0x00197180;
                        continue;
                    }
                    pc = 0x00197174;
                    continue;
                }
                0x00197174 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1d4), c.g[S2][0]);
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197354;
                        continue;
                    }
                    pc = 0x00197180;
                    continue;
                }
                0x00197180 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197188);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18d0));
                    call_ptr(c, 0x00317670, 0x00197188)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001971bc;
                        continue;
                    }
                    pc = 0x00197190;
                    continue;
                }
                0x00197190 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x0019719c;
                    continue;
                }
                0x0019719c => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001971b4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001971b4)?;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x001971bc;
                    continue;
                }
                0x001971bc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001971c4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f0));
                    call_ptr(c, 0x00317670, 0x001971c4)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001971f8;
                        continue;
                    }
                    pc = 0x001971cc;
                    continue;
                }
                0x001971cc => {
                    c.cycles += 8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1d8), c.g[S2][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    c.g[RA][0] = sx(0x001971e0);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x003a0c50, 0x001971e0)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x001971ec;
                    continue;
                }
                0x001971ec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x001971f8;
                    continue;
                }
                0x001971f8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197200);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
                    call_ptr(c, 0x00317670, 0x00197200)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x00197220;
                        continue;
                    }
                    pc = 0x00197208;
                    continue;
                }
                0x00197208 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197214;
                    continue;
                }
                0x00197214 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197220;
                    continue;
                }
                0x00197220 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197228);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x610));
                    call_ptr(c, 0x00317670, 0x00197228)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x00197260;
                        continue;
                    }
                    pc = 0x00197230;
                    continue;
                }
                0x00197230 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x168));
                    c.g[RA][0] = sx(0x0019723c);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1e0), c.g[S2][0]);
                    call_ptr(c, 0x00198af0, 0x0019723c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1e0));
                    c.g[RA][0] = sx(0x00197248);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x00197248)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197254;
                    continue;
                }
                0x00197254 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x00197260;
                    continue;
                }
                0x00197260 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197268);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x620));
                    call_ptr(c, 0x00317670, 0x00197268)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001972bc;
                        continue;
                    }
                    pc = 0x00197270;
                    continue;
                }
                0x00197270 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0u64;
                    if t {
                        pc = 0x001972b0;
                        continue;
                    }
                    pc = 0x0019727c;
                    continue;
                }
                0x0019727c => {
                    c.cycles += 3;
                    c.g[A0][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x160));
                    c.g[A1][0] = 0u64;
                    pc = 0x00197288;
                    continue;
                }
                0x00197288 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A0][0] >> (c.g[A1][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A1][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x00197288) {
                            return Ok(());
                        }
                        pc = 0x00197288;
                        continue;
                    }
                    pc = 0x001972a4;
                    continue;
                }
                0x001972a4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x001972b0;
                    continue;
                }
                0x001972b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197358;
                        continue;
                    }
                    pc = 0x001972bc;
                    continue;
                }
                0x001972bc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001972c4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
                    call_ptr(c, 0x00317670, 0x001972c4)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001972d4;
                        continue;
                    }
                    pc = 0x001972cc;
                    continue;
                }
                0x001972cc => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x170), c.g[S2][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x001972d4;
                    continue;
                }
                0x001972d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001972dc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x630));
                    call_ptr(c, 0x00317670, 0x001972dc)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001972f0;
                        continue;
                    }
                    pc = 0x001972e4;
                    continue;
                }
                0x001972e4 => {
                    c.cycles += 3;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1f4), c.g[S2][0]);
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00197354;
                        continue;
                    }
                    pc = 0x001972f0;
                    continue;
                }
                0x001972f0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001972f8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x640));
                    call_ptr(c, 0x00317670, 0x001972f8)?;
                    let t = c.g[S4][0] != c.g[V0][0];

                    if t {
                        pc = 0x00197308;
                        continue;
                    }
                    pc = 0x00197300;
                    continue;
                }
                0x00197300 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1fc), c.g[S2][0]);
                    if t {
                        pc = 0x00197350;
                        continue;
                    }
                    pc = 0x00197308;
                    continue;
                }
                0x00197308 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197310);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1850));
                    call_ptr(c, 0x00317670, 0x00197310)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x0019733c;
                        continue;
                    }
                    pc = 0x00197318;
                    continue;
                }
                0x00197318 => {
                    c.cycles += 9;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1f8), c.g[S2][0]);
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197334);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00197334)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x0019733c;
                    continue;
                }
                0x0019733c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00197344);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x184c));
                    call_ptr(c, 0x00317670, 0x00197344)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V1][0] = 0x440000;
                    if t {
                        pc = 0x00197398;
                        continue;
                    }
                    pc = 0x0019734c;
                    continue;
                }
                0x0019734c => {
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x200), c.g[S2][0]);
                    pc = 0x00197350;
                    continue;
                }
                0x00197350 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x1;
                    pc = 0x00197354;
                    continue;
                }
                0x00197354 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x00197358;
                    continue;
                }
                0x00197358 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197368);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00197368)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    if t {
                        pc = 0x001974d4;
                        continue;
                    }
                    pc = 0x00197370;
                    continue;
                }
                0x00197370 => {
                    c.cycles += 10;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x194), c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197390);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00197390)?;
                    let t = true;

                    if t {
                        pc = 0x00197404;
                        continue;
                    }
                    pc = 0x00197398;
                    continue;
                }
                0x00197398 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11c0));
                    c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x11a0));
                    c.g[S1][0] = 0u64;
                    let t = true;
                    c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                    if t {
                        pc = 0x001973c4;
                        continue;
                    }
                    pc = 0x001973b0;
                    continue;
                }
                0x001973b0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001973b8);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001973b8)?;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00197370) {
                            return Ok(());
                        }
                        pc = 0x00197370;
                        continue;
                    }
                    pc = 0x001973c0;
                    continue;
                }
                0x001973c0 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    pc = 0x001973c4;
                    continue;
                }
                0x001973c4 => {
                    c.cycles += 3;
                    c.g[S6][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                    let t = c.g[S6][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00197414;
                        continue;
                    }
                    pc = 0x001973d0;
                    continue;
                }
                0x001973d0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x001973dc);
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                    call_ptr(c, 0x00317670, 0x001973dc)?;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x001973b0) {
                            return Ok(());
                        }
                        pc = 0x001973b0;
                        continue;
                    }
                    pc = 0x001973e4;
                    continue;
                }
                0x001973e4 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32), c.g[S2][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00197404);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00197404)?;
                    pc = 0x00197404;
                    continue;
                }
                0x00197404 => {
                    c.cycles += 2;
                    let t = c.g[S6][0] == 0u64;
                    c.g[V0][0] = 0x440000;
                    if t {
                        pc = 0x00197414;
                        continue;
                    }
                    pc = 0x0019740c;
                    continue;
                }
                0x0019740c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                    if t {
                        pc = 0x001974d0;
                        continue;
                    }
                    pc = 0x00197414;
                    continue;
                }
                0x00197414 => {
                    c.cycles += 5;
                    c.g[S1][0] = 0u64;
                    c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1200));
                    c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1bc));
                    c.g[S7][0] = 0x6;
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                    pc = 0x00197428;
                    continue;
                }
                0x00197428 => {
                    c.cycles += 6;
                    c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                    c.g[RA][0] = sx(0x00197438);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x00197438)?;

                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[S1][0] = c.g[S5][0];
                        pc = 0x001974c4;
                        continue;
                    }
                    pc = 0x00197440;
                    continue;
                }
                0x00197440 => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V1][0] == 0u64;
                    c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub((c.g[S1][0] as u32)));
                    if t {
                        pc = 0x001974a8;
                        continue;
                    }
                    pc = 0x00197454;
                    continue;
                }
                0x00197454 => {
                    c.cycles += 5;
                    c.g[A2][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x160));
                    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = 0u64;
                    pc = 0x00197468;
                    continue;
                }
                0x00197468 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                    let t = c.g[V1][0] != 0u64;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x00197468) {
                            return Ok(());
                        }
                        pc = 0x00197468;
                        continue;
                    }
                    pc = 0x00197484;
                    continue;
                }
                0x00197484 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x001974a8;
                        continue;
                    }
                    pc = 0x00197490;
                    continue;
                }
                0x00197490 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001974a0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001974a0)?;
                    let t = true;
                    c.g[S1][0] = c.g[S5][0];
                    if t {
                        pc = 0x001974c4;
                        continue;
                    }
                    pc = 0x001974a8;
                    continue;
                }
                0x001974a8 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001974c0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001974c0)?;
                    c.g[S1][0] = c.g[S5][0];
                    pc = 0x001974c4;
                    continue;
                }
                0x001974c4 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x00197428) {
                            return Ok(());
                        }
                        pc = 0x00197428;
                        continue;
                    }
                    pc = 0x001974d0;
                    continue;
                }
                0x001974d0 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    pc = 0x001974d4;
                    continue;
                }
                0x001974d4 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00196f98, 0x00197500);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;

    use crate::difftest::fill;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::money_text_test;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;

    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn c_fe_state_request_line_update_money_matches() {
        money_text_test(
            ssx3_ee::s03::f_001985b0__generated,
            c_fe_state_request_line_update_money,
            0x1e0,
            0x168,
            0x1a,
        );
    }

    #[test]
    fn cfe_state_char_equip_compare_items_matches() {
        check(
            ssx3_ee::s03::f_00198da0__generated,
            cfe_state_char_equip_compare_items,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x198da0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 4);
                word(c, DATA, DATA + 0x100);
                word(c, DATA + 4, DATA + 0x200);
                c.bus.write16(DATA + 0x10a, (r.next_u32() & 0xffff) as u16);
                c.bus.write16(DATA + 0x20a, (r.next_u32() & 0xffff) as u16);
            },
        );
    }

    #[test]
    fn cfe_state_request_line_sort_song_list_matches() {
        check(
            ee::s02::f_00197ad8__generated,
            cfe_state_request_line_sort_song_list,
            120,
            &|c, seed| {
                let mut r = base(c, seed, 0x197a);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x14c, (seed % 4) as u32);
                c.bus
                    .write64(DATA + 0x160, if seed % 2 == 0 { 0x5 } else { 0xa });
                fill(c, &mut r, DATA + 0x4c, 8);
            },
        );
    }

    #[test]
    fn cfe_request_line_buy_song_matches() {
        check(
            ee::s03::f_001988d8__generated,
            cfe_request_line_buy_song,
            8,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0019_88d8, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 64) as u64;
                let mask = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffffu64
                } else {
                    0x0101_0101_0101_0101u64 << (seed % 8)
                };
                c.bus.write64(OBJ + 0x160, mask);
                c.bus.write64(OBJ + 0x158, seed as u64);
                c.bus.write8(OBJ + 0x44, (seed % 4) as u8);
                w(c, OBJ + 0x48, (seed % 9) as u32);
            },
        );
    }

    #[test]
    fn cfestaterequestline_update_song_count_matches() {
        check(
            ee::s02::f_00198118__generated,
            cfestaterequestline_update_song_count,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x198118, 3);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                let db = DATA + 0x400;
                clear(c, db, 0x100);
                w(c, WORLD + 0x8c, db);
                w(c, db + 4, DATA + 0x500);
                w(c, DATA + 0x500 + 0x20, 0);
                let vt = vtable(c, &mut r, &[0x20]);
                w(c, DATA + 0x500 + 8, vt);
                let _ = vt;
                w(c, this + 0x158, (seed % 4) as u32);
                w(c, this + 0x1e4, 0);
            },
        );
    }

    #[test]
    fn cfestaterequestline_update_slot_icons_matches() {
        check(
            ee::s03::f_001985f0__generated,
            cfestaterequestline_update_slot_icons,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1985f0, 3);
                let this = DATA;
                clear(c, this, 0x800);
                c.g[A0][0] = this as u64;
                w(c, this + 0x160, (seed % 8) as u32);
                w(c, this + 0x16c, (seed % 2) as u32);
                let vt = vtable(c, &mut r, &[0x48]);
                for i in 0..6u32 {
                    let icon = DATA + 0x200 + 0x80 * i;
                    clear(c, icon, 0x40);
                    if (seed as u32 + i) % 3 != 0 {
                        w(c, this + 0x1bc + 4 * i, icon);
                        w(c, icon + 8, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn cfestaterequestline_update_song_names_matches() {
        check(
            ee::s02::f_00197bc8__generated,
            cfestaterequestline_update_song_names,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x197bc8, 3);
                let this = DATA;
                clear(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                w(c, this + 0x14c, 0);
                w(c, this + 0x150, 0);
                let vt = vtable(c, &mut r, &[0x48]);
                for i in 0..8u32 {
                    let text = DATA + 0x200 + 0x80 * i;
                    clear(c, text, 0x40);
                    w(c, text + 8, vt);
                    w(c, this + 0x174 + 4 * i, text);
                }
            },
        );
    }

    #[test]
    fn cfestate_request_line_on_child_event_matches() {
        strict(
            ssx3_ee::s02::f_001979d8__generated,
            cfestate_request_line_on_child_event,
            SEEDS_2,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x79d8);
                let this = DATA + 0x1000;
                let child = DATA + 0x2000;
                w(c, this + 0x1c, r.next_u32());
                w(c, this + 0x170, DATA + 0x3000);
                w(c, this + 0x150, 0);
                w(
                    c,
                    child + 0xc,
                    if seed % 2 == 0 { 7 } else { r.next_u32() % 16 },
                );
                w(c, child + 0x6c, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(child);
                c.g[A2][0] = sx(match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => r.next_u32() % 32,
                });
            },
        );
    }

    #[test]
    fn cfestate_request_line_open_buy_popup_matches() {
        strict(
            ssx3_ee::s03::f_00198988__generated,
            cfestate_request_line_open_buy_popup,
            SEEDS_2,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x8988);
                let this = DATA + 0x1000;
                let song = DATA + 0x2000;
                let state = DATA + 0x5000;
                w(c, this + 0x10, state);
                w(c, this + 0x44, r.next_u32() & 0x7f);
                w(c, this + 0x160, r.next_u32());
                w(c, this + 0x164, r.next_u32());
                w(c, this + 0x168, r.next_u32() % 5000);
                w(c, song, r.next_u32());
                w(c, GAME_GP.wrapping_sub(0xc8), DATA + 0x6000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(song);
            },
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::Rng;

    pub(crate) use crate::game::fe::festateaudiooptions::tests::oc853_setup;
    pub(crate) use crate::game::fe::festateaudiooptions::tests::oc853_w;

    pub(crate) use crate::game::fe::festateaudiooptions::tests::OC853_POOL;
    pub(crate) use crate::game::fe::festateaudiooptions::tests::OC853_SELF;
    pub(crate) use crate::game::fe::festateaudiooptions::tests::OC853_WIDGET;
    pub(crate) use crate::game::fe::festateaudiooptions::tests::OC853_WORLD;

    use std::sync::atomic::AtomicUsize;

    /// Song-list fields the request-line action handler reads: visible range,
    /// song ids at `+0x4c`, owned/play-list masks, money, in-store flag.
    #[test]
    fn cfe_state_request_line_on_widget_action_matches() {
        check(
            ee::s02::f_00197500__generated,
            cfe_state_request_line_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_7500, 12);
                c.pc = 0x0019_7500;
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(OC853_WIDGET) };
                c.g[A2][0] = [5u64, 6, 6, 5, 0, 6, 5, 9, 6, 5][(seed % 10) as usize];
                oc853_w(c, OC853_WIDGET + 0x18, r.next_u32() % 6);
                oc853_w(c, OC853_SELF + 0x14c, 0);
                oc853_w(c, OC853_SELF + 0x150, r.next_u32() % 5);
                for i in 0..16u32 {
                    oc853_w(c, OC853_SELF + 0x4c + 4 * i, r.next_u32() % 64);
                }
                c.bus.write64(
                    OC853_SELF + 0x158,
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                );
                c.bus.write64(
                    OC853_SELF + 0x160,
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                );
                oc853_w(c, OC853_SELF + 0x16c, r.next_u32() % 2);
                oc853_w(c, OC853_SELF + 0x168, r.next_u32() % 400);
                c.bus.write8(OC853_SELF + 0x44, (r.next_u32() % 4) as u8);
                oc853_w(c, OC853_SELF + 0x10, OC853_POOL + 0x100);
                // Events 1 and 5 (with a song in range) reach the unported song-list
                // helpers, which run away on scattered data (`DLHeap` at 0x31ef40):
                // only the exits of event 5 (no song in range) and event 6 are tested.
                // The new-state path (world flag 0) reaches the unported
                // `cFEStateAudioOptions_ctor`; keep the flag set.
                oc853_w(c, OC853_WORLD + 0x84, OC853_POOL);
                oc853_w(c, OC853_WORLD + 0x5558, OC853_POOL + 0x500);
            },
        );
    }

    // The request line object (`cFEStateRequestLine`) and what it points at.
    pub(crate) const OC854_SELF: u32 = DATA + 0x400;

    pub(crate) const OC854_WORLD: u32 = DATA + 0x1800;

    pub(crate) const OC854_LOCALE: u32 = DATA + 0x1900;

    pub(crate) const OC854_LOCALE_VT: u32 = DATA + 0x1a00;

    pub(crate) const OC854_WIDGET_VT: u32 = DATA + 0x1b00;

    pub(crate) const OC854_BE: u32 = DATA + 0x1c00;

    pub(crate) const OC854_WIDGETS: u32 = DATA + 0x2000;

    pub(crate) const OC854_AUDIO: u32 = DATA + 0x3000;

    pub(crate) const OC854_NAME: u32 = DATA + 0x3400;

    pub(crate) const OC854_ARTIST: u32 = DATA + 0x3500;

    // Fakes are active only while this word of the machine's RAM holds the magic.
    pub(crate) const OC854_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC854_MAGIC: u32 = 0x4f43_3835;

    // Offsets in the request line.
    pub(crate) const SONG_ORDER: u32 = 0x4c;

    pub(crate) const NUM_SONGS: u32 = 0x14c;

    pub(crate) const FIRST_VISIBLE: u32 = 0x150;

    pub(crate) const OWNED_OR_SEL: u32 = 0x158;

    pub(crate) const PLAYLIST: u32 = 0x160;

    pub(crate) const MONEY: u32 = 0x168;

    pub(crate) const IN_STORE: u32 = 0x16c;

    pub(crate) const CHECK_MARKS: u32 = 0x194;

    pub(crate) const TITLE: u32 = 0x1b4;

    pub(crate) const ARTIST: u32 = 0x1b8;

    pub(crate) const HELP_TEXT: u32 = 0x1e8;

    pub(crate) const ARROW_UP: u32 = 0x1ec;

    pub(crate) const ARROW_DOWN: u32 = 0x1f0;

    pub(crate) const BTN_TEXT: u32 = 0x1f4;

    pub(crate) const BTN_BACK: u32 = 0x1f8;

    pub(crate) const BTN_A: u32 = 0x1fc;

    pub(crate) const BTN_B: u32 = 0x200;

    pub(crate) const IMG_CHECK: u32 = 0x204;

    pub(crate) const IMG_EMPTY: u32 = 0x208;

    pub(crate) const IMG_DOLLAR: u32 = 0x20c;

    pub(crate) fn w_2(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Mailbox word of a faked callee: a nonzero value is returned in `v0`.
    pub(crate) fn mail(addr: u32) -> u32 {
        OC854_CTRL + 0x100 + (addr & 0xffff)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// mailbox value (if set) unless it unwound.
    pub(crate) fn oc854_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let m = c.bus.read32(mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && m != 0 {
            c.g[V0][0] = m as u64;
        }
    }

    macro_rules! oc854_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC854_CTRL) != OC854_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc854_fake_call(c, $addr);
                }
            )*
            fn oc854_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc854_fakes!(
        (
            oc854_fake_28b180,
            OC854_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc854_fake_28cf98,
            OC854_PREV_28CF98,
            ee::s07::OVERRIDE_0028CF98,
            ee::s07::f_0028cf98__generated,
            0x0028_cf98
        ),
        (
            oc854_fake_2b4978,
            OC854_PREV_2B4978,
            ee::s08::OVERRIDE_002B4978,
            ee::s08::f_002b4978__generated,
            0x002b_4978
        ),
        (
            oc854_fake_2b40f0,
            OC854_PREV_2B40F0,
            ee::s08::OVERRIDE_002B40F0,
            ee::s08::f_002b40f0__generated,
            0x002b_40f0
        ),
        (
            oc854_fake_2b4120,
            OC854_PREV_2B4120,
            ee::s08::OVERRIDE_002B4120,
            ee::s08::f_002b4120__generated,
            0x002b_4120
        ),
        (
            oc854_fake_317670,
            OC854_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc854_fake_3dbca0,
            OC854_PREV_3DBCA0,
            ee::s14::OVERRIDE_003DBCA0,
            ee::s14::f_003dbca0__generated,
            0x003d_bca0
        ),
        (
            oc854_fake_317e30,
            OC854_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc854_fake_2c26d0,
            OC854_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc854_fake_3a0e90,
            OC854_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc854_fake_317e98,
            OC854_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc854_fake_3a0c50,
            OC854_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc854_fake_3a1000,
            OC854_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc854_fake_14dc80,
            OC854_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc854_fake_14dd58,
            OC854_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc854_fake_197b88,
            OC854_PREV_197B88,
            ee::s02::OVERRIDE_00197B88,
            ee::s02::f_00197b88__generated,
            0x0019_7b88
        ),
    );

    /// A mask of song bits: `mode` picks none, only `idx`, `idx` and a
    /// neighbour, a dense or a sparse random set, or everything but `idx`.
    pub(crate) fn oc854_mask(r: &mut Rng, mode: u32, idx: u32) -> u64 {
        let bit = 1u64 << (idx & 63);
        let dense = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        match mode % 7 {
            0 => 0,
            1 => bit,
            2 => bit | (1u64 << ((idx + 1 + r.next_u32() % 5) & 63)),
            3 => dense,
            4 => (1u64 << (r.next_u32() & 63)) | (1u64 << (r.next_u32() & 63)),
            5 => dense & !bit,
            _ => dense | bit,
        }
    }

    /// Machine state shared by the request line tests: fakes on, the world with
    /// the locale database, the widget vtable, the music system, the names.
    pub(crate) fn oc854_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        oc854_install_fakes();
        w_2(c, OC854_CTRL, OC854_MAGIC);
        w_2(c, GAME_GP - 0x848, OC854_WORLD);
        let game = r.next_u32() % 2;
        w_2(c, OC854_WORLD + 0x84, game);
        w_2(c, OC854_WORLD + 0x8c, OC854_LOCALE);
        w_2(c, OC854_LOCALE + 4, OC854_LOCALE_VT);
        c.bus.write16(OC854_LOCALE_VT + 0x20, 0);
        w_2(c, OC854_LOCALE_VT + 0x24, FAKE);
        c.bus.write16(OC854_WIDGET_VT + 0x48, 0);
        w_2(c, OC854_WIDGET_VT + 0x4c, FAKE);
        w_2(c, mail(0x0028_b180), OC854_AUDIO);
        w_2(c, mail(0x002b_40f0), OC854_NAME);
        w_2(c, mail(0x002b_4120), OC854_ARTIST);
        w_2(c, mail(0x0014_dc80), OC854_BE);
        let cur = r.next_u32() % 6;
        w_2(c, OC854_AUDIO + 0x508, cur);
        w_2(c, OC854_AUDIO + 0x504, r.next_u32() % 6);
        for a in [OC854_NAME, OC854_ARTIST] {
            let text = if r.next_u32() % 3 == 0 {
                0
            } else {
                0x4142_4300
            };
            w_2(c, a, text);
        }
        // cBENewRaceInterface_newRace.cGamePath
        c.bus.write8(0x0053_5c11, (r.next_u32() % 2) as u8);
        r
    }

    /// The request line at `OC854_SELF`: counts, the song order, the song masks
    /// and every widget (`null_ok` ones are sometimes missing).
    pub(crate) fn oc854_self(
        c: &mut Cpu,
        r: &mut Rng,
        idx_of: &dyn Fn(&mut Cpu, u32, u32) -> u32,
        dense: bool,
    ) -> u32 {
        let s = OC854_SELF;
        let num = r.next_u32() % 21;
        let first = r.next_u32() % 13;
        w_2(c, s + NUM_SONGS, num);
        w_2(c, s + FIRST_VISIBLE, first);
        for i in 0..40 {
            let v = r.next_u32() % 64;
            w_2(c, s + SONG_ORDER + 4 * i, v);
        }
        let idx = idx_of(c, first, r.next_u32() % 8);
        let (m1, m2) = if dense {
            (3, 3)
        } else {
            (r.next_u32(), r.next_u32())
        };
        let owned = oc854_mask(r, m1, idx);
        let playlist = oc854_mask(r, m2, idx);
        c.bus.write64(s + OWNED_OR_SEL, owned);
        c.bus.write64(s + PLAYLIST, playlist);
        let money = [0, 0x1387, 0x1388, 0x2000, r.next_u32() % 0x3000][(r.next_u32() % 5) as usize];
        w_2(c, s + MONEY, money);
        w_2(c, s + IN_STORE, r.next_u32() % 2);
        let mut next = 0;
        let mut put = |c: &mut Cpu, off: u32, nullable: bool, r: &mut Rng| {
            let widget = OC854_WIDGETS + 0x40 * next;
            next += 1;
            w_2(c, widget + 8, OC854_WIDGET_VT);
            let p = if nullable && r.next_u32() % 4 == 0 {
                0
            } else {
                widget
            };
            w_2(c, s + off, p);
        };
        for i in 0..8 {
            put(c, CHECK_MARKS + 4 * i, false, r);
        }
        for off in [TITLE, ARTIST, HELP_TEXT, BTN_TEXT] {
            put(c, off, true, r);
        }
        for off in [
            ARROW_UP, ARROW_DOWN, BTN_BACK, BTN_A, BTN_B, IMG_CHECK, IMG_EMPTY, IMG_DOLLAR,
        ] {
            put(c, off, false, r);
        }
        idx
    }

    /// The song of row `k` of the visible list (`mSongOrder[first + k]`).
    pub(crate) fn oc854_row_song(c: &mut Cpu, first: u32, k: u32) -> u32 {
        let a = OC854_SELF + SONG_ORDER + 4 * (first + k);
        c.bus.read32(a)
    }

    #[test]
    fn cfe_state_request_line_update_song_icons_matches() {
        check(
            ee::s02::f_00197ca0__generated,
            cfe_state_request_line_update_song_icons,
            120,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x197ca0, 20);
                oc854_self(c, &mut r, &|_, _, _| 0, true);
                c.g[A0][0] = OC854_SELF as u64;
                c.pc = 0x0019_7ca0;
            },
        );
    }

    #[test]
    fn cfe_request_line_update_scroll_arrows_matches() {
        check(
            ee::s02::f_00197db8__generated,
            crate::game::fe::festateaudiooptions::cfe_request_line_update_scroll_arrows,
            120,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x197db8, 4);
                oc854_self(c, &mut r, &|_, _, _| 0, true);
                c.g[A0][0] = OC854_SELF as u64;
                c.pc = 0x0019_7db8;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_update_now_playing_matches() {
        check(
            ee::s02::f_00197e70__generated,
            cfe_state_request_line_update_now_playing,
            240,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x197e70, 24);
                oc854_self(c, &mut r, &|_, _, _| 0, true);
                c.g[A0][0] = OC854_SELF as u64;
                c.pc = 0x0019_7e70;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_update_help_text_matches() {
        check(
            ee::s02::f_001981f8__generated,
            cfe_state_request_line_update_help_text,
            300,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x1981f8, 6);
                let idx = oc854_self(c, &mut r, &|_, _, _| 0, false);
                let idx = (idx + r.next_u32() % 64) % 64;
                // the masks were built for idx 0: rebuild them for the real one
                let owned = oc854_mask(&mut r, seed, idx);
                let playlist = oc854_mask(&mut r, seed / 7, idx);
                c.bus.write64(OC854_SELF + OWNED_OR_SEL, owned);
                c.bus.write64(OC854_SELF + PLAYLIST, playlist);
                c.g[A0][0] = OC854_SELF as u64;
                c.g[A1][0] = idx as u64;
                c.pc = 0x0019_81f8;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_update_buttons_matches() {
        check(
            ee::s03::f_00198340__generated,
            cfe_state_request_line_update_buttons,
            300,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x198340, 12);
                let idx = oc854_self(c, &mut r, &|_, _, _| 0, false);
                let idx = (idx + r.next_u32() % 64) % 64;
                let owned = oc854_mask(&mut r, seed, idx);
                let playlist = oc854_mask(&mut r, seed / 7, idx);
                c.bus.write64(OC854_SELF + OWNED_OR_SEL, owned);
                c.bus.write64(OC854_SELF + PLAYLIST, playlist);
                c.g[A0][0] = OC854_SELF as u64;
                c.g[A1][0] = idx as u64;
                c.pc = 0x0019_8340;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_on_widget_notify_matches() {
        check(
            ee::s02::f_001977d0__generated,
            cfe_state_request_line_on_widget_notify,
            400,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x1977d0, 10);
                let k = r.next_u32() % 8;
                let idx = oc854_self(c, &mut r, &oc854_row_song, false);
                // `idx_of` above used a random row; rebuild masks for the row `k`
                let first = c.bus.read32(OC854_SELF + FIRST_VISIBLE);
                let song = oc854_row_song(c, first, k);
                let _ = idx;
                let owned = oc854_mask(&mut r, seed, song);
                let playlist = oc854_mask(&mut r, seed / 7, song);
                c.bus.write64(OC854_SELF + OWNED_OR_SEL, owned);
                c.bus.write64(OC854_SELF + PLAYLIST, playlist);
                let msg = [6u64, 8, 9, 4, 5, 7, 6, 8][(r.next_u32() % 8) as usize];
                c.g[A0][0] = OC854_SELF as u64;
                c.g[A2][0] = msg;
                c.g[A3][0] = k as u64;
                c.pc = 0x0019_77d0;
            },
        );
    }

    /// Gate word of the fakes: they only act in machines that hold the magic.
    pub(crate) const OC855_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC855_MAGIC: u32 = 0x0855_f4e5;

    /// Mailbox words (forced results) of the fakes, one per callee address.
    pub(crate) const OC855_MAIL: u32 = DATA + 0x30_0000;

    pub(crate) fn oc855_mail(addr: u32) -> u32 {
        OC855_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc855_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc855_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc855_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc855_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's pointer).
    pub(crate) fn oc855_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc855_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc855_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC855_CTRL) != OC855_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc855_fake_call(c, $addr);
                }
            )*
            fn oc855_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc855_fakes!(
        (
            oc855_fake_28b180,
            OC855_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc855_fake_2b40f0,
            OC855_PREV_2B40F0,
            ee::s08::OVERRIDE_002B40F0,
            ee::s08::f_002b40f0__generated,
            0x002b_40f0
        ),
        (
            oc855_fake_2b4120,
            OC855_PREV_2B4120,
            ee::s08::OVERRIDE_002B4120,
            ee::s08::f_002b4120__generated,
            0x002b_4120
        ),
        (
            oc855_fake_317670,
            OC855_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc855_fake_3dbca0,
            OC855_PREV_3DBCA0,
            ee::s14::OVERRIDE_003DBCA0,
            ee::s14::f_003dbca0__generated,
            0x003d_bca0
        ),
        (
            oc855_fake_416810,
            OC855_PREV_416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc855_fake_317e30,
            OC855_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc855_fake_2c26d0,
            OC855_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc855_fake_3a0e90,
            OC855_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc855_fake_317e98,
            OC855_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc855_fake_3a0c50,
            OC855_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc855_fake_3a1000,
            OC855_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc855_fake_14dc80,
            OC855_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc855_fake_14dd58,
            OC855_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc855_fake_417828,
            OC855_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc855_fake_39e2a0,
            OC855_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc855_fake_1a1cd0,
            OC855_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0
        ),
        (
            oc855_fake_147488,
            OC855_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc855_fake_3e6448,
            OC855_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc855_fake_398380,
            OC855_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc855_fake_1a0508,
            OC855_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc855_fake_150928,
            OC855_PREV_150928,
            ee::s01::OVERRIDE_00150928,
            ee::s01::f_00150928__generated,
            0x0015_0928
        ),
        (
            oc855_fake_14aea8,
            OC855_PREV_14AEA8,
            ee::s01::OVERRIDE_0014AEA8,
            ee::s01::f_0014aea8__generated,
            0x0014_aea8
        ),
        (
            oc855_fake_14b478,
            OC855_PREV_14B478,
            ee::s01::OVERRIDE_0014B478,
            ee::s01::f_0014b478__generated,
            0x0014_b478
        ),
        (
            oc855_fake_14a130,
            OC855_PREV_14A130,
            ee::s01::OVERRIDE_0014A130,
            ee::s01::f_0014a130__generated,
            0x0014_a130
        ),
        (
            oc855_fake_146d98,
            OC855_PREV_146D98,
            ee::s01::OVERRIDE_00146D98,
            ee::s01::f_00146d98__generated,
            0x0014_6d98
        ),
        (
            oc855_fake_14b700,
            OC855_PREV_14B700,
            ee::s01::OVERRIDE_0014B700,
            ee::s01::f_0014b700__generated,
            0x0014_b700
        ),
        (
            oc855_fake_19da10,
            OC855_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc855_fake_14bdb8,
            OC855_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc855_fake_14ad28,
            OC855_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc855_fake_14d7e8,
            OC855_PREV_14D7E8,
            ee::s01::OVERRIDE_0014D7E8,
            ee::s01::f_0014d7e8__generated,
            0x0014_d7e8
        ),
        (
            oc855_fake_418ef8,
            OC855_PREV_418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
    );

    /// A machine with the fakes on, `calls` fake calls to unwind within.
    pub(crate) fn oc855_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc855_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC855_CTRL, OC855_MAGIC);
        r
    }

    pub(crate) fn oc855_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    #[test]
    fn cfestate_request_line_get_song_title_matches() {
        check(
            ssx3_ee::s03::f_00198a88__generated,
            cfestate_request_line_get_song_title,
            24,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198a88, 2);
                c.pc = 0x0019_8a88;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn cfestate_request_line_get_song_artist_matches() {
        check(
            ssx3_ee::s03::f_00198ab8__generated,
            cfestate_request_line_get_song_artist,
            24,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198ab8, 2);
                c.pc = 0x0019_8ab8;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn cfestate_request_line_get_song_price_matches() {
        check(
            ssx3_ee::s03::f_00198ae8__generated,
            cfestate_request_line_get_song_price,
            8,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198ae8, 0);
                c.pc = 0x0019_8ae8;
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn oc855_fe_format_money_matches() {
        check_lohi(
            ssx3_ee::s03::f_00198af0__generated,
            oc855_fe_format_money,
            96,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x198af0, 4);
                c.pc = 0x0019_8af0;
                // The locale record: random bytes, the currency format in
                // bits 22..24 of its first word.
                let fmt = seed % 8;
                for i in 0..0xa2 {
                    oc855_w(c, 0x0053_5610 + 4 * i, r.next_u32());
                }
                let flags = r.next_u32() & !(7 << 22) | fmt << 22;
                oc855_w(c, 0x0053_5610, flags);
                // The digit count `strlen` reports (0 = empty string).
                match seed / 8 % 6 {
                    0 => oc855_force_zero(c, 0x0041_6810),
                    1 => oc855_force(c, 0x0041_6810, 1),
                    2 => oc855_force(c, 0x0041_6810, 3),
                    3 => oc855_force(c, 0x0041_6810, 4),
                    4 => oc855_force(c, 0x0041_6810, 7),
                    _ => oc855_force(c, 0x0041_6810, 10),
                }
                let amount = match seed % 3 {
                    0 => r.next_u32() % 100_000,
                    1 => 0u32.wrapping_sub(r.next_u32() % 100_000),
                    _ => r.next_u32(),
                };
                c.g[A0][0] = if seed % 5 == 0 {
                    (r.next_u32() as u64) << 32 | amount as u64
                } else {
                    sx(amount)
                };
            },
        );
    }

    #[test]
    fn cfestate_request_line_show_song_info_matches() {
        check(
            ssx3_ee::s03::f_001986d0__generated,
            cfestate_request_line_show_song_info,
            128,
            &|c, seed| {
                let mut r = oc855_base(c, seed, 0x1986d0, 14);
                c.pc = 0x0019_86d0;
                let this = DATA + 0x1000;
                let info = DATA + 0x2000;
                let price = DATA + 0x2400;
                let boxw = DATA + 0x2800;
                let vt = DATA + 0x3000;
                let world = DATA + 0x4000;
                let obj = DATA + 0x4400;
                let sub = DATA + 0x4800;
                let idx = r.next_u32() % 64;
                oc855_w(c, this + 0x1d4, if seed % 7 == 0 { 0 } else { info });
                oc855_w(c, this + 0x1d8, if seed % 5 == 1 { 0 } else { price });
                oc855_w(c, this + 0x1dc, boxw);
                oc855_w(c, this + 0x16c, (seed / 3 % 2) * 5);
                // The owned-songs mask: song `idx` owned, few, or many owned.
                let rand64 = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                let mask: u64 = match seed % 4 {
                    0 => 1 << idx,
                    1 => 0,
                    2 => !(1u64 << idx),
                    _ => rand64 & !(1u64 << idx),
                };
                c.bus.write64(this + 0x160, mask);
                for w in [info, price, boxw] {
                    oc855_w(c, w + 8, vt);
                }
                c.bus
                    .write16(vt + 0x48, (r.next_u32() as i16 as i32 & !3) as u16);
                oc855_w(c, vt + 0x4c, FAKE);
                oc855_w(c, GAME_GP.wrapping_sub(0x848), world);
                oc855_w(c, world + 0x8c, obj);
                oc855_w(c, obj + 4, sub);
                c.bus
                    .write16(sub + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
                oc855_w(c, sub + 0x24, FAKE + 0x10);
                // The money locale (format and digits) behind FE_FormatMoney.
                for i in 0..0xa2 {
                    oc855_w(c, 0x0053_5610 + 4 * i, r.next_u32());
                }
                let flags = r.next_u32() & !(7 << 22) | (seed / 4 % 4) << 22;
                oc855_w(c, 0x0053_5610, flags);
                // Fake call order: 0 text height, 1 vtable, 2-3 artist, 4
                // length, 5 strlen, 6 alloc, 7 format, 8 vtable, 9 set text.
                if seed % 6 == 2 {
                    oc855_force_zero(c, 0x0031_7e30);
                } else {
                    oc855_force(c, 0x0031_7e30, 0x00a0_0000 + 0x40 * (seed % 16));
                }
                oc855_force(c, 0x0041_6810, 5 + seed % 3);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(idx);
            },
        );
    }

    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    pub(crate) const OC1124_ENTRY: u32 = 0x0019_6f98;

    /// Gate word (and magic) of this test's callee fakes; `+4`: 1 = the
    /// allocator fake returns null.
    pub(crate) const OC1124_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1124_MAGIC: u32 = 0x1124_c0de;

    pub(crate) const OC1124_THIS: u32 = DATA + 0x1000;

    pub(crate) const OC1124_WIDGET: u32 = DATA + 0x2000;

    pub(crate) const OC1124_WVT: u32 = DATA + 0x3000;

    pub(crate) const OC1124_WORLD: u32 = DATA + 0x4000;

    pub(crate) const OC1124_LOC: u32 = DATA + 0x5000;

    pub(crate) const OC1124_LVT: u32 = DATA + 0x6000;

    /// The hash fake returns its argument, so a widget whose name hash is the
    /// string address matches that entry of the chain.
    pub(crate) const OC1124_HASHES: [u32; 19] = [
        0x0046_0588,
        0x0046_0598,
        0x0046_0488,
        0x004a_18b8,
        0x0046_05a8,
        0x0046_05b8,
        0x0046_05c8,
        0x004a_18c0,
        0x004a_18c8,
        0x004a_18d0,
        0x0046_05f0,
        0x0046_0600,
        0x0046_0610,
        0x0046_0620,
        0x0045_e1b0,
        0x0046_0630,
        0x0046_0640,
        0x004a_1900,
        0x004a_1910,
    ];

    /// Targets: the 19 fixed names, 8 slot-A, 8 slot-B and 6 option names,
    /// and one name that matches nothing.
    pub(crate) const OC1124_NT: u32 = 19 + 8 + 8 + 6 + 1;

    /// The fake callee: logs like `fakes::fake_call`; the hash fake echoes
    /// its argument and the allocator fake can return null.
    pub(crate) fn oc1124_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let null_alloc = c.bus.read32(OC1124_CTRL + 4) == 1;
        fakes::fake_call(c, addr);
        if c.pc == ra {
            if addr == 0x0031_7670 {
                c.g[V0][0] = a0;
            } else if addr == 0x0031_7e30 && null_alloc {
                c.g[V0][0] = 0;
            }
        }
    }

    macro_rules! oc1124_fakes {
        ($($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1124_CTRL) != OC1124_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1124_fake_call(c, $addr);
                }
            )*
            fn oc1124_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1124_fakes! {
        oc1124_fake_317670, OC1124_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670;
        oc1124_fake_3dbca0, OC1124_PREV_3DBCA0, ee::s14::OVERRIDE_003DBCA0, ee::s14::f_003dbca0__generated, 0x003d_bca0;
        oc1124_fake_2c26d0, OC1124_PREV_2C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0;
        oc1124_fake_317e30, OC1124_PREV_317E30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated, 0x0031_7e30;
        oc1124_fake_317e98, OC1124_PREV_317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98;
        oc1124_fake_3a0e90, OC1124_PREV_3A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90;
        oc1124_fake_3a0c50, OC1124_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50;
        oc1124_fake_198af0, OC1124_PREV_198AF0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated, 0x0019_8af0;
    }

    /// Scene for seed `seed`: the widget's name hash is entry `seed % NT` of
    /// the chain; the variant (`seed / NT`) sets the cost-line flag, the bit
    /// counts of the done/selected masks and a null allocation.
    pub(crate) fn oc1124_scene(c: &mut Cpu, seed: u32) {
        let target = seed % OC1124_NT;
        let v = seed / OC1124_NT;
        c.bus.write32(OC1124_CTRL, OC1124_MAGIC);
        c.bus.write32(OC1124_CTRL + 4, (v % 3 == 2) as u32);
        // Name tables of the slot-A, slot-B and option loops.
        for i in 0..8u32 {
            c.bus.write32(0x0044_11a0 + 4 * i, 0x0050_0000 + 0x10 * i);
            c.bus.write32(0x0044_11c0 + 4 * i, 0x0050_0100 + 0x10 * i);
        }
        for i in 0..6u32 {
            c.bus.write32(0x0044_1200 + 4 * i, 0x0050_0200 + 0x10 * i);
        }
        c.bus.write32(GAME_GP - 0x1850, 0x004a_1900);
        c.bus.write32(GAME_GP - 0x184c, 0x004a_1910);
        let hash = match target {
            0..=18 => OC1124_HASHES[target as usize],
            19..=26 => 0x0050_0000 + 0x10 * (target - 19),
            27..=34 => 0x0050_0100 + 0x10 * (target - 27),
            35..=40 => 0x0050_0200 + 0x10 * (target - 35),
            _ => 0x1234_5670,
        };
        c.bus.write32(OC1124_WIDGET + 0x38, hash);
        c.bus.write32(OC1124_WIDGET + 8, OC1124_WVT);
        c.bus.write16(OC1124_WVT + 0x48, (4 * (v % 4)) as u16);
        c.bus.write32(OC1124_WVT + 0x4c, FAKE);
        // The locale object behind the world's `+0x8c` (virtual slot +0x20/+0x24).
        c.bus.write32(GAME_GP - 0x848, OC1124_WORLD);
        c.bus.write32(OC1124_WORLD + 0x8c, OC1124_LOC);
        c.bus.write32(OC1124_LOC + 4, OC1124_LVT);
        c.bus.write16(OC1124_LVT + 0x20, (4 * (v % 3)) as u16);
        c.bus.write32(OC1124_LVT + 0x24, FAKE + 0x10);
        // The line: done mask, selection mask, cost flag, price pointer.
        let k = (v * 5) % 9;
        c.bus.write64(
            OC1124_THIS + 0x158,
            0x9e37_79b9_7f4a_7c15u64.rotate_left(seed) | 1,
        );
        c.bus.write64(OC1124_THIS + 0x160, (1u64 << k) - 1);
        c.bus.write32(OC1124_THIS + 0x16c, (v >> 1) & 1);
        c.bus.write32(OC1124_THIS + 0x168, DATA + 0x7000);
        c.g[A0][0] = sx(OC1124_THIS);
        c.g[A1][0] = sx(OC1124_WIDGET);
        c.pc = OC1124_ENTRY;
    }

    /// `cFEStateRequestLine_OnWidgetCreated`: every name of the hash chain
    /// (and a miss) with 8 variants of the flag/bit-count masks; some seeds
    /// unwind in a callee (the resume runs the generated body); every call
    /// site is reached by some seed. Then events due at the bit-count loops.
    #[test]
    fn cfe_state_request_line_on_widget_created_matches() {
        oc1124_install();
        fakes::check_reaching(
            ee::s02::f_00196f98__generated,
            cfe_state_request_line_on_widget_created,
            OC1124_NT * 8,
            &[
                0x0019_7014,
                0x0019_70c0,
                0x0019_70e8,
                0x0019_7130,
                0x0019_7140,
                0x0019_71e0,
                0x0019_723c,
                0x0019_7248,
                0x0019_7334,
                0x0019_7368,
                0x0019_7390,
                0x0019_7404,
                0x0019_74a0,
                0x0019_74c0,
            ],
            &|c, seed| {
                machine(c, seed, 0x196f98, 40);
                oc1124_scene(c, seed);
            },
        );
        driven!(oc1124_gen_driven, ee::s02::f_00196f98__generated);
        driven!(oc1124_hand_driven, cfe_state_request_line_on_widget_created);
        check_events(oc1124_gen_driven, oc1124_hand_driven, 96, &|c, seed| {
            // Names that reach a bit-count loop (song list, free songs,
            // options) with the cost flag set where the loop needs it.
            let name = [6, 13, 35, 36, 37, 38][(seed % 6) as usize];
            let variant = [2, 3, 6, 7][((seed / 6) % 4) as usize];
            let mut r = machine(c, seed, 0x196f99, 0);
            oc1124_scene(c, name + OC1124_NT * variant);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, OC1124_ENTRY)
        });
    }
}
