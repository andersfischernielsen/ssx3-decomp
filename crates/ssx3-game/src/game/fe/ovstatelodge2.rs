//! `src/fe/ovstatelodge2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_IGNORED;
pub(crate) use crate::game::fe::festatebonusmaterial::WIDGET_ACTIVATED;
pub(crate) use crate::game::fe::festatebonusmaterial::widget_range_notify;
pub(crate) use crate::game::fe::feasyncfile::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001f_3790, cfe_state_lodge_init, "cFEStateLodge_Init"),
    (0x001f_3cc8, fe_state_lodge_on_widget_notify, "cFEStateLodge_OnWidgetNotify"),
    (0x001f_3ff8, c_fe_state_lodge_rider_detail_ctor, "cFEStateLodgeRiderDetail_ctor"),
    (0x001f_4050, fe_state_lodge_rider_detail_init, "cFEStateLodgeRiderDetail_Init"),
    (0x001f_4380, c_fe_state_lodge_rider_detail_on_render, "cFEStateLodgeRiderDetail_OnRender"),
    (0x001f_43d8, fe_state_lodge_rider_detail_on_widget_notify, "cFEStateLodgeRiderDetail_OnWidgetNotify"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
}


/// `void* cFEStateLodgeRiderDetail_ctor(self)` (0x1f3ff8): run the UI state
/// ctor, install the class and copy the pad mask byte.
pub fn c_fe_state_lodge_rider_detail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_400c);
        call_ptr(c, callee::UI_STATE_CTOR, 0x001f_400c)?;
        c.g[V0][0] = 0x28;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x39d8));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_4034);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_4034)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateLodgeRiderDetail_OnRender(self)` (0x1f4380): when the detail
/// screen is not open, find its widget by name and reveal it.
pub fn c_fe_state_lodge_rider_detail_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x48).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = 0x47_0000;
            c.g[RA][0] = sx(0x001f_43a8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1090));
            call_ptr(c, 0x0031_7670, 0x001f_43a8)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x40).lw(c);
            c.g[RA][0] = sx(0x001f_43b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_43b4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001f_43c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF, 0x001f_43c4)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateLodge_OnWidgetNotify(...)` (0x1F3CC8): ids 7..9 answer 0x100.
pub fn fe_state_lodge_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 7);
}


/// `int cFEStateLodgeRiderDetail_OnWidgetNotify(...)` (0x1F43D8): ids 8..9
/// answer 0x100.
pub fn fe_state_lodge_rider_detail_on_widget_notify(c: &mut Cpu) {
    widget_range_notify(c, 8);
}


/// `void cFEStateLodge_Init(self)` (0x1f3790): create the lodge screen, play
/// its first frame, select the lodge bank and start its music.
pub fn cfe_state_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2318));
        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_37b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_37c8)?;
        let missing = c.g[V0][0] == 0;
        c.sw(p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_37e0)?;
        }
        c.cycles += 24;
        call_ptr(c, 0x0014_dc80, 0x001f_37e8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001f_37f4)?;
        c.g[A1][0] = Addr(p(c.g[S1][0] as u32, 0x44)).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_74c8, 0x001f_3800)?;
        c.sw(p(c.g[S1][0] as u32, 0x48), c.g[V0][0]);
        c.g[V0][0] = 0x440000;
        c.sw(p(c.g[V0][0] as u32, 0x1b14), 0);
        call_ptr(c, 0x0028_b180, 0x001f_3810)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_f140, 0x001f_381c)?;
        c.g[V0][0] = 0x12;
        c.sw(p(c.g[S1][0] as u32, 0x54), 0);
        c.sw(p(c.g[S1][0] as u32, 0x50), c.g[V0][0]);
        c.sw(p(c.g[S1][0] as u32, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// FE state: lodge rider detail.

/// `void cFEStateLodgeRiderDetail_Init(cFEStateLodgeRiderDetail* s)` (0x1f4050):
/// create the "rider detail" screen from its name hash, play its first frame,
/// then install the screen's rider name and the current peak's description.
pub fn fe_state_lodge_rider_detail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        'outer: {
            frame.save(c, S1, 0x10);
            let this = r(c, A0);
            c.g[S1][0] = sx(this);
            frame.save_ra(c, 0);
            frame.save(c, S0, 0x20);
            c.g[A0][0] = sx(0x0046_ef20);
            c.g[RA][0] = sx(0x001f_4074);
            c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_4074)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_4088);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_4088)?;
            let none = c.g[V0][0] == 0;
            Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
            if none {
                break 'outer;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_40a0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_40a0)?;
            'mid: {
                c.cycles += 20;
                c.g[V0][0] = 1;
                c.g[RA][0] = sx(0x001f_40ac);
                Addr(p(r(c, S1), 0x48)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x0014_dc80, 0x001f_40ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40b8);
                c.g[A1][0] = 1;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40b8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[RA][0] = sx(0x001f_40c4);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_7488, 0x001f_40c4)?;
                c.g[RA][0] = sx(0x001f_40cc);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dc80, 0x001f_40cc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40d8);
                c.g[A1][0] = 0xd;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x001f_40d8)?;
                c.g[A1][0] = Addr(p(r(c, S1), 0x44)).lb(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f_40e8);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x0015_77a0, 0x001f_40e8)?;
                if c.g[V0][0] == 0 {
                    Addr(p(r(c, S1), 0x48)).sw(c, 0);
                    break 'mid;
                }
                break 'mid;
            }
            break 'outer;
        }
        c.cycles += 5;
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
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee as ee;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::camera::trigger::cameratriggerlist::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;


    #[test]
    fn c_fe_state_lodge_rider_detail_ctor_matches() {
        check(
            ee::s04::f_001f3ff8__generated,
            c_fe_state_lodge_rider_detail_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f3ff8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                let module = DATA + 0x2000;
                clear(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn c_fe_state_lodge_rider_detail_on_render_matches() {
        check(
            ee::s04::f_001f4380__generated,
            c_fe_state_lodge_rider_detail_on_render,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f4380, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let screen = DATA + 0x2000;
                clear(c, screen, 0x200);
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(this + 0x40, screen);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn fe_state_lodge_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001f3cc8__generated, fe_state_lodge_on_widget_notify, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x001f_3cc8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }


    #[test]
    fn fe_state_lodge_rider_detail_on_widget_notify_matches() {
        check(ssx3_ee::s04::f_001f43d8__generated, fe_state_lodge_rider_detail_on_widget_notify, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x001f_43d8, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A2][0] = (seed % 12) as u64;
        });
    }


    /// A screen-owning state: engine at +0x10, a screen created or not.
    pub(crate) fn screen_init(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = base(c, seed, salt, 2);
        c.g[A0][0] = OBJ as u64;
        w(c, OBJ + 0x10, OBJ2);
        w(c, OBJ + 0x40, 0);
        w(c, OBJ + 0x44, (seed % 4) as u32);
        w(c, OBJ2 + 0x10, OBJ3);
    }


        #[test]
    fn cfe_state_lodge_init_matches() {
        check(ee::s04::f_001f3790__generated, cfe_state_lodge_init, 6, &|c, seed| {
            screen_init(c, seed, 0x001f_3790)
        });
    }


    #[test]
    fn fe_state_lodge_rider_detail_init_matches() {
        check(ee::s04::f_001f4050__generated, fe_state_lodge_rider_detail_init, 40, &|c, seed| {
            alloc_base(c, seed, 0x1f40, 5);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            wb(c, DATA + 0x44, (seed % 4) as u8);
        });
    }
}
