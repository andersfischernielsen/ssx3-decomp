//! `src/fe/messagecenter.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::call_ptr;
use crate::abi::r32;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::sound::soundsys::gp_4 as gp;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw;
pub(crate) use crate::game::fe::feasyncfile::a;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_79d8, cfestate_request_line_on_child_event, "cFEStateRequestLine_OnChildEvent"),
    (0x0019_7ad8, cfe_state_request_line_sort_song_list, "cFEStateRequestLine_SortSongList"),
    (0x0019_7bc8, cfestaterequestline_update_song_names, "cFEStateRequestLine_UpdateSongNames"),
    (0x0019_8118, cfestaterequestline_update_song_count, "cFEStateRequestLine_UpdateSongCount"),
    (0x0019_85b0, c_fe_state_request_line_update_money, "cFEStateRequestLine_UpdateMoney"),
    (0x0019_85f0, cfestaterequestline_update_slot_icons, "cFEStateRequestLine_UpdateSlotIcons"),
    (0x0019_88d8, cfe_request_line_buy_song, "cFEStateRequestLine_BuySong"),
    (0x0019_8988, cfestate_request_line_open_buy_popup, "cFEStateRequestLine_OpenBuyPopup"),
    (0x0019_8da0, cfe_state_char_equip_compare_items, "cFEStateCharEquip_CompareItems"),
];

// ---------------------------------------------------------------------------
// Callees (the generated crate's readable aliases, which dispatch through
// their override slot exactly as the generated `jal`/`call_indirect` does).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
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
                c.sw((c.g[V1][0] as u32), c.g[A1][0]);
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
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x160));
        loop {
            c.cycles += 6;
            c.g[V0][0] = c.g[V0][0] >> (c.g[V1][0] & 63);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] << 32;
            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.sw((c.g[A1][0] as u32), c.g[V1][0]);
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
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
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
    const REFRESH_LIST: Callee =
        Callee { addr: 0x0019_7b88, f: ee::s02::cFEStateRequestLine_RefreshList };
    const UPDATE_HELP: Callee =
        Callee { addr: 0x0019_81f8, f: ee::s02::cFEStateRequestLine_UpdateHelpText };
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
        let mut stage = 0u32; // 0 epilogue, 1 event 0x15, 2 event 0x16
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
    const GET_SONG_TITLE: Callee =
        Callee { addr: 0x0019_8a88, f: ee::s03::cFEStateRequestLine_GetSongTitle };
    const GET_SONG_PRICE: Callee =
        Callee { addr: 0x0019_8ae8, f: ee::s03::cFEStateRequestLine_GetSongPrice };
    const SET_TRACK: u32 = 0x001c_a980;
    const SET_CREDIT: u32 = 0x001c_a900;
    const PUSH_POPUP: u32 = 0x0039_f290;

    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x60);
        let this = abi::arg(c, 0);
        let song = abi::arg(c, 1);
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL;
    use ssx3_ee as ee;
    use crate::difftest::put_vec;
    use crate::difftest::SP;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::compare;
    pub(crate) use crate::game::fe::festatecharequip2::tests::money_text_test;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_2;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_3;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_2;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_4;


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
        check(ssx3_ee::s03::f_00198da0__generated, cfe_state_char_equip_compare_items, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x198da0, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 4);
            word(c, DATA, DATA + 0x100);
            word(c, DATA + 4, DATA + 0x200);
            c.bus.write16(DATA + 0x10a, (r.next_u32() & 0xffff) as u16);
            c.bus.write16(DATA + 0x20a, (r.next_u32() & 0xffff) as u16);
        });
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
                w(c, child + 0xc, if seed % 2 == 0 { 7 } else { r.next_u32() % 16 });
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
}
