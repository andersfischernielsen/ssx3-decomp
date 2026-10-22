//! `src/dirtysock/tags.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use callee::*;
use crate::abi;
use crate::call::Flow;
use crate::abi::r32;
use crate::call::sp;
use crate::events::deliver_event;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::festaterewards::r;
pub(crate) use crate::game::fe::feasyncfile::lbu as lbu_2;
pub(crate) use crate::game::fe::feasyncfile::a;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_bb60, tag_field_strnzcpy, "TagFieldStrnzcpy"),
    (0x003e_be40, tag_field_set_number, "TagFieldSetNumber"),
    (0x003e_bf28, tag_field_set_flags, "TagFieldSetFlags"),
    (0x003e_c0f8, tag_field_set_token, "TagFieldSetToken"),
    (0x003e_c300, tag_field_set_binary, "TagFieldSetBinary"),
    (0x003e_cb00, tag_field_merge, "TagFieldMerge"),
    (0x003e_cf30, tag_field_get_number, "TagFieldGetNumber"),
    (0x003e_cfb0, tag_field_get_flags, "TagFieldGetFlags"),
    (0x003e_cff8, tag_field_get_address, "TagFieldGetAddress"),
    (0x003e_d070, tag_field_get_token, "TagFieldGetToken"),
    (0x003e_d578, tag_field_decode_hex, "TagFieldDecodeHex"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2 as IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2 as GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW as TAG_FIELD_SET_RAW_2;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
}


// ---------------------------------------------------------------------------
// Dirtysock tag fields.

/// `int TagFieldStrnzcpy(char* dst, int len, const char* src)` (0x3ebb60):
/// copy at most `len-1` characters, NUL-terminate, return the number copied.
///
/// The generated body is a `pc`-dispatch loop (it re-enters at the store
/// stage 0x3ebb70 after a thread switch), so this port accepts the same
/// resume addresses.
pub fn tag_field_strnzcpy(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_bb60 => {
                c.cycles += 3;
                c.g[A3][0] = c.g[A1][0]; // left = len
                c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
                c.pc = 0x003e_bb84;
            }
            0x003e_bb6c => {
                c.cycles += 1;
                c.pc = 0x003e_bb70;
            }
            0x003e_bb84 => {
                c.cycles += 2;
                c.pc = if c.g[V0][0] != 0 {
                    0x003e_bb98
                } else {
                    0x003e_bb8c
                };
            }
            0x003e_bb8c => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lbu(c);
                if !more {
                    c.pc = 0x003e_bb98;
                    continue;
                }
                if back_edge_unwinds(c, 0x003e_bb70) {
                    return;
                }
                c.pc = 0x003e_bb70;
            }
            0x003e_bb70 => {
                c.cycles += 5;
                Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < 2) as u64;
                c.pc = 0x003e_bb84;
            }
            0x003e_bb98 => {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    Addr(c.g[A0][0] as u32).sb(c, 0);
                }
                c.pc = 0x003e_bba0;
            }
            0x003e_bba0 => {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                crate::abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_bb60, 0x003e_bba8);
                return;
            }
        }
    }
}


/// `int TagFieldGetFlags(const char* type, int defval)` (0x3ecfb0): OR together
/// the flag words `D_004965E0[c]` for every character of the tag type; the
/// default is returned for a null type.
pub fn tag_field_get_flags(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_cfb0 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[A1][0];
                c.pc = if c.g[A0][0] == 0 {
                    0x003e_cff0
                } else {
                    0x003e_cfb8
                };
            }
            0x003e_cfb8 => {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                c.g[V1][0] = 0x0049_0000;
                c.g[V1][0] = sx(0x0049_0000u32.wrapping_add(0x65e0)); // D_004965E0
                c.g[A1][0] = 0;
                c.pc = 0x003e_cfd8;
            }
            0x003e_cfcc => {
                c.cycles += 1;
                c.pc = 0x003e_cfd0;
            }
            0x003e_cfd0 => {
                c.cycles += 2;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                c.pc = 0x003e_cfd8;
            }
            0x003e_cfd8 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] |= c.g[V0][0];
                    if back_edge_unwinds(c, 0x003e_cfd0) {
                        return;
                    }
                    c.pc = 0x003e_cfd0;
                } else {
                    c.pc = 0x003e_cfec;
                }
            }
            0x003e_cfec => {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
                c.pc = 0x003e_cff0;
            }
            0x003e_cff0 => {
                c.cycles += 2;
                crate::abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_cfb0, 0x003e_cff8);
                return;
            }
        }
    }
}


/// `lb off(reg)` (zero-extended).
#[inline]
pub(crate) fn lbu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lbu(c)
}


/// `int TagFieldGetAddress(const char* tag, int defval)` (0x3ecff8): parse a
/// dotted IPv4 text address; digits shift in decimal, a '.' shifts the
/// accumulator left by eight, any other character ends the scan. Returns
/// `defval` for a null tag.
pub fn tag_field_get_address(c: &mut Cpu) {
    loop {
        match c.pc {
            0x003e_cff8 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[A0][0];
                c.g[V0][0] = c.g[A1][0];
                let null = c.g[A2][0] == 0;
                c.g[V1][0] = 0;
                c.pc = if null { 0x003e_d068 } else { 0x003e_d008 };
            }
            0x003e_d008 => {
                c.cycles += 5;
                c.g[A1][0] = 0xffff_ffff_ffff_0000;
                c.g[T0][0] = 0xa;
                c.g[A1][0] = c.g[A1][0] | 0xff00;
                c.g[A3][0] = 0x2e;
                c.pc = 0x003e_d024;
            }
            0x003e_d020 => {
                c.cycles += 1;
                c.g[A2][0] = sx(low(c, A2).wrapping_add(1));
                c.pc = 0x003e_d024;
            }
            0x003e_d024 => {
                c.cycles += 5;
                c.g[A0][0] = lbu_r(c, A2, 0);
                c.g[V0][0] = sx(low(c, A0).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let digit = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[V1][0] & 0xff;
                c.pc = if digit { 0x003e_d038 } else { 0x003e_d050 };
            }
            0x003e_d038 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.mult(0, V0, V0, T0); // lo = (accum & 0xff) * 10
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A0)));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                if back_edge_unwinds(c, 0x003e_d020) {
                    return;
                }
                c.pc = 0x003e_d020;
            }
            0x003e_d050 => {
                c.cycles += 4;
                c.g[V0][0] = sx(low(c, A0) << 24);
                c.g[V0][0] = sx(((low(c, V0) as i32) >> 24) as u32);
                let dot = c.g[V0][0] != c.g[A3][0];
                c.g[V0][0] = c.g[V1][0];
                c.pc = if dot { 0x003e_d068 } else { 0x003e_d060 };
            }
            0x003e_d060 => {
                c.cycles += 2;
                c.g[V1][0] = sx(low(c, V1) << 8);
                if back_edge_unwinds(c, 0x003e_d020) {
                    return;
                }
                c.pc = 0x003e_d020;
            }
            0x003e_d068 => {
                c.cycles += 2;
                abi::ret(c);
                return;
            }
            pc => {
                c.jump_out(pc, 0x003e_cff8, 0x003e_d070);
                return;
            }
        }
    }
}


/// `int TagFieldGetNumber(const char* tag, int defval)` (0x003ecf30): parse a
/// signed decimal number from a tag field.
pub fn tag_field_get_number(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    let empty = c.g[A2][0] == 0;
    c.g[V0][0] = c.g[A1][0];
    if !empty {
        c.cycles += 4;
        c.g[V1][0] = Addr(at(c, A2, 0)).lb(c);
        c.g[V0][0] = 0x2b;
        let not_plus = c.g[V1][0] != c.g[V0][0];
        c.g[T0][0] = 0x1;
        if not_plus {
            c.cycles += 3;
            c.g[V0][0] = 0x2d;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
            } else {
                c.cycles += 2;
                c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
                c.g[T0][0] = 0xffffffffffffffff;
                c.cycles += 1;
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
            }
        } else {
            c.cycles += 2;
            c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
            c.cycles += 1;
            c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
        }
        c.cycles += 4;
        c.g[V0][0] = sx(word(c, A1).wrapping_sub(0x30));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        let not_digit = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !not_digit {
            c.cycles += 1;
            c.g[A3][0] = 0xa;
            loop {
                c.cycles += 8;
                c.g[A0][0] = sx(word(c, A1) & 0xf);
                c.mult(0, V1, V1, A3);
                c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
                c.g[A1][0] = Addr(at(c, A2, 0)).lbu(c);
                c.g[V0][0] = sx(word(c, A1).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                let digit = c.g[V0][0] != 0;
                c.g[V1][0] = sx(word(c, V1).wrapping_add(word(c, A0)));
                if !digit {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_cf80);
                }
            }
        }
        c.cycles += 1;
        c.mult(0, V0, T0, V1);
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Tag fields.

/// `int TagFieldGetToken(const char* data, int defval)` (0x3ed070): read up to
/// four characters into a FourCC (starting from `0x20202020`), or `defval`
/// when the text is empty or the token overflows; a token ends at a control
/// character or a character >= 127.
pub fn tag_field_get_token(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A2][0] = c.g[A0][0];
    let null = c.g[A2][0] == 0;
    c.g[V0][0] = c.g[A1][0];
    if null {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    let mut data = r32(c, A2);
    c.g[V1][0] = Addr(data).lb(c);
    let ch = c.g[V1][0];
    c.g[T0][0] = ((ch as i64) < 33) as u64;
    let too_small = c.g[T0][0] != 0;
    c.g[A3][0] = Addr(data).lbu(c);
    if too_small {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V1][0] = ((ch as i64) < 127) as u64;
    let too_big = c.g[V1][0] == 0;
    c.g[A0][0] = 0x2020_0000;
    if too_big {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[A0][0] | 0x2020;
    c.cycles += 2;
    c.g[A1][0] = 0x20ff_0000;
    c.cycles += 2;
    c.g[A1][0] = c.g[A1][0] | 0xfffe;
    c.g[V0][0] = sx(r32(c, A3) << 24);
    loop {
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, A0) << 8);
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 24) as u32);
        c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
        c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let stop = c.g[V1][0] != 0;
        c.g[A2][0] = sx(data.wrapping_add(1));
        if stop {
            break;
        }
        c.cycles += 4;
        data = r32(c, A2);
        c.g[V1][0] = Addr(data).lb(c);
        let ch = c.g[V1][0];
        c.g[V0][0] = ((ch as i64) < 33) as u64;
        let too_small = c.g[V0][0] != 0;
        c.g[A3][0] = Addr(data).lbu(c);
        if too_small {
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((ch as i64) < 127) as u64;
        let too_big = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, A3) << 24);
        if too_big {
            if event_due(c) {
                deliver_event_here(c, 0x003e_d0b0);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}


/// `void TagFieldSetToken(TagField* self, u32 id, char* str, u32 token)`
/// (0x3ec0f8): copy the current tag into a stack buffer, append the token as
/// up to three characters (big-endian order) and set the field.
pub fn tag_field_set_token(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x160);
        let sp0 = spv(c);
        c.sq(17, at_2(sp0, 0x140));
        c.sq(18, at_2(sp0, 0x130));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, at_2(sp0, 0x150));
        c.g[S2][0] = c.g[A1][0];
        c.sd(at_2(sp0, 0x120), c.g[RA][0]);
        c.g[S0][0] = c.g[A3][0];
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_COPY.call(c, 0x003e_c120)?;
        c.g[V1][0] = c.g[V0][0];
        let token = c.g[S0][0];
        if token != 0 {
            c.cycles += 4;
            c.g[A0][0] = 0x20ff_ffff;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            loop {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
                if !skip {
                    c.cycles += 2;
                    c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = sx(at_2(c.g[V1][0] as u32, 1));
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32) << 8);
                let more = c.g[S0][0] != 0;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_c138);
                }
            }
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        c.sb(c.g[V1][0] as u32, 0);
        c.g[A2][0] = c.g[SP][0];
        callee::TAG_FIELD_SET_RAW.call(c, 0x003e_c168)?;
        c.lq(16, at_2(sp0, 0x150));
        c.lq(17, at_2(sp0, 0x140));
        c.lq(18, at_2(sp0, 0x130));
        c.g[RA][0] = c.ld(at_2(sp0, 0x120));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x160));
        c.pc = ra;
        Ok(())
    })
}


/// `void TagFieldSetFlags(Tag tag, int flags, void* format, int value)`
/// (0x3ebf28): copy `tag` to a stack buffer, append the value's base-32 digits
/// from the table at `0x4965c0` and set the flags field with the result.
pub fn tag_field_set_flags(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x150);
        c.g[S2][0] = c.g[A1][0];
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x120);
        c.g[S0][0] = c.g[A3][0];
        call_ptr(c, 0x003e_b198, 0x003e_bf50)?; // TagFieldCopyTag
        c.g[A1][0] = 0x0049_0000;
        c.g[V1][0] = c.g[V0][0];
        let no_value = c.g[S0][0] == 0;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x65c0));
        // `zero` means the common tail (A0 = S1) runs; `add_one` is its +1.
        let mut zero = no_value;
        let mut add_one = no_value;
        if !no_value {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
            if c.g[V0][0] == 0 {
                zero = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0] & 1;
                loop {
                    c.cycles += 2;
                    let bit_clear = c.g[V0][0] == 0;
                    c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 1) as u32);
                    if !bit_clear {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 2;
                    let end = c.g[S0][0] == 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                    if end {
                        zero = true;
                        add_one = true;
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lb(c);
                    let cont = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S0][0] & 1;
                    if !cont {
                        zero = true;
                        add_one = true;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_bf70);
                    }
                }
            }
        }
        if zero {
            if add_one {
                c.cycles += 1;
            }
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 10;
        c.g[A1][0] = c.g[S2][0];
        c.sb(c.g[V1][0] as u32, 0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003e_b1e0, 0x003e_bfac)?; // TagFieldSetRaw
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    });
}


/// `lbu` of `a + off`.
#[inline(always)]
pub(crate) fn lbu(c: &mut Cpu, a: u32, off: u32) -> u64 {
    Addr(p(a, off)).lbu(c)
}
 // D_004960C0: low nibble -> character

pub fn tag_field_set_binary(c: &mut Cpu) {
    guest(c, |c| {

        'b_3ec38c: {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1150));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1120));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1110));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1140));
        c.g[S3][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1130));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x1100), c.g[RA][0]);
        c.g[S1][0] = c.g[T0][0];
        c.g[S0][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x003ec330);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003eb198, 0x003ec330)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x24;
        c.sb((c.g[A0][0] as u32), c.g[V0][0]);
        let t = (c.g[S1][0] as i64) <= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if t {
        break 'b_3ec38c;
        } else {
        c.cycles += 5;
        c.g[V0][0] = 0x490000;
        c.g[V1][0] = 0x490000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5fc0));
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60c0));
        'l_3ec358: loop {
        c.cycles += 13;
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
        c.sb((c.g[A0][0] as u32), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
        c.sb((c.g[A0][0] as u32), c.g[V1][0]);
        let t = (c.g[S1][0] as i64) > 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x003ec358); }
        continue 'l_3ec358;
        } else {
        break 'b_3ec38c;
        }
        }
        }
        }
        c.cycles += 12;
        c.sb((c.g[A0][0] as u32), 0u64);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x003ec3a0);
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003eb1e0, 0x003ec3a0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1140));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1150));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Tag fields.

/// `int TagFieldMerge(void* record, void* len, char* data)` (0x3ecb00): merge
/// whitespace-separated `name=value` fields from `data` into the tag record,
/// returning the number of fields accepted.
pub fn tag_field_merge(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003e_cb00 => {
                c.cycles += 15;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x70));
                c.sq(S0, p(r32(c, SP), 0x60));
                c.sq(S1, p(r32(c, SP), 0x50));
                c.g[S0][0] = c.g[A2][0];
                c.sq(S2, p(r32(c, SP), 0x40));
                c.g[S1][0] = 0;
                c.sq(S3, p(r32(c, SP), 0x30));
                c.g[S2][0] = c.g[A1][0];
                c.sq(S4, p(r32(c, SP), 0x20));
                c.g[S3][0] = c.g[A0][0];
                c.sq(S5, p(r32(c, SP), 0x10));
                c.g[S4][0] = 0x3a;
                c.sd(r32(c, SP), c.g[RA][0]);
                c.g[S5][0] = 0x3d;
                pc = 0x003e_cb84;
            }
            0x003e_cb3c => {
                c.cycles += 1;
                pc = 0x003e_cb40;
            }
            0x003e_cb40 => {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                if call_ptr(c, 0x003e_b1e0, 0x003e_cb4c).is_err() {
                    return;
                }
                c.g[V1][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = ((0u64 as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[V1][0] = (c.g[V1][0] < 0x20u64) as u64;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                if c.g[V1][0] != 0 {
                    pc = 0x003e_cb84;
                } else {
                    pc = 0x003e_cb60;
                }
            }
            0x003e_cb60 => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                pc = 0x003e_cb68;
            }
            0x003e_cb68 => {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = (c.g[V0][0] < 0x20u64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    if back_edge_unwinds(c, 0x003e_cb68) {
                        return;
                    }
                    pc = 0x003e_cb68;
                } else {
                    pc = 0x003e_cb84;
                }
            }
            0x003e_cb84 => {
                c.cycles += 3;
                c.g[V1][0] = c.lbu(r32(c, S0));
                c.g[V0][0] = (c.g[V1][0] < 0x21u64) as u64;
                if c.g[V1][0] == 0 {
                    pc = 0x003e_cbb0;
                } else {
                    pc = 0x003e_cb90;
                }
            }
            0x003e_cb90 => {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    pc = 0x003e_cba0;
                } else {
                    pc = 0x003e_cb98;
                }
            }
            0x003e_cb98 => {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                if back_edge_unwinds(c, 0x003e_cb84) {
                    return;
                }
                pc = 0x003e_cb84;
            }
            0x003e_cba0 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[S1][0];
                if c.g[V1][0] == c.g[S5][0] {
                    pc = 0x003e_cbb4;
                } else {
                    pc = 0x003e_cba8;
                }
            }
            0x003e_cba8 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[V1][0] != c.g[S4][0] {
                    if back_edge_unwinds(c, 0x003e_cb40) {
                        return;
                    }
                    pc = 0x003e_cb40;
                } else {
                    pc = 0x003e_cbb0;
                }
            }
            0x003e_cbb0 => {
                c.cycles += 1;
                c.g[V0][0] = c.g[S1][0];
                pc = 0x003e_cbb4;
            }
            0x003e_cbb4 => {
                c.cycles += 9;
                c.lq(S0, p(r32(c, SP), 0x60));
                c.lq(S1, p(r32(c, SP), 0x50));
                c.lq(S2, p(r32(c, SP), 0x40));
                c.lq(S3, p(r32(c, SP), 0x30));
                c.lq(S4, p(r32(c, SP), 0x20));
                c.lq(S5, p(r32(c, SP), 0x10));
                c.g[RA][0] = c.ld(r32(c, SP));
                let ra = r32(c, RA);
                c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x70));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003e_cb00, 0x003e_cbd8);
                return;
            }
        }
    }
}


// ---------------------------------------------------------------------------
// Tag fields.

/// `void TagFieldSetNumber(TagField* record, char* out, ...)` (0x3ebe40):
/// format `value` in decimal into a temporary buffer and store it through the
/// tag field's append/terminate helpers. This one is a resume point list: the
/// two division/character loops can be re-entered after a host-thread unwind,
/// so every `pc` the generated dispatcher can continue at is accepted.
pub fn tag_field_set_number(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_be40 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x160));
                    c.sq(17, p(r(c, SP), 0x140));
                    c.sq(18, p(r(c, SP), 0x130));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, p(r(c, SP), 0x150));
                    c.g[S2][0] = c.g[A1][0];
                    c.sd(p(r(c, SP), 0x120), c.g[RA][0]);
                    c.g[S0][0] = c.g[A3][0];
                    c.g[A1][0] = c.g[SP][0];
                    callee::TAG_FIELD_COPY_TAG.call(c, 0x003e_be68)?;
                    let non_negative = (c.g[S0][0] as i64) >= 0;
                    c.g[A2][0] = c.g[V0][0];
                    pc = if non_negative {
                        0x003e_be80
                    } else {
                        0x003e_be70
                    };
                }
                0x003e_be70 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x2d;
                    c.g[S0][0] = sx(0u32.wrapping_sub(r(c, S0)));
                    c.sb(r(c, A2), c.g[V0][0]);
                    c.g[A2][0] = sx(p(r(c, A2), 0x1));
                    pc = 0x003e_be80;
                }
                0x003e_be80 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(p(r(c, A2), 0x1f));
                    let no_digits = (c.g[S0][0] as i64) <= 0;
                    c.sb(p(r(c, A2), 0x1f), 0);
                    pc = if no_digits { 0x003e_bec8 } else { 0x003e_be8c };
                }
                0x003e_be8c => {
                    c.cycles += 3;
                    c.g[A1][0] = 0xa;
                    c.g[A3][0] = 0;
                    c.div(0, 16, 5);
                    pc = 0x003e_be98;
                }
                0x003e_be98 => {
                    c.cycles += 2;
                    if c.g[A1][0] == c.g[A3][0] {
                        pc = 0x003e_bea0;
                    } else {
                        pc = 0x003e_bea0;
                    }
                }
                0x003e_bea0 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x1));
                    c.g[V0][0] = c.hi[0];
                    c.g[V1][0] = c.lo[0];
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x30));
                    c.g[S0][0] = c.g[V1][0];
                    c.sb(r(c, A0), c.g[V0][0]);
                    if c.g[A1][0] == c.g[A3][0] {
                        pc = 0x003e_bec0;
                    } else {
                        pc = 0x003e_bec0;
                    }
                }
                0x003e_bec0 => {
                    c.cycles += 2;
                    if (c.g[S0][0] as i64) > 0 {
                        c.div(0, 16, 5);
                        if event_due(c) && deliver_event(c, 0x003e_be98) {
                            return Ok(());
                        }
                        pc = 0x003e_be98;
                    } else {
                        pc = 0x003e_bec8;
                    }
                }
                0x003e_bec8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lbu(r(c, A0));
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(r(c, A0));
                        pc = 0x003e_bee4;
                    } else {
                        pc = 0x003e_bed4;
                    }
                }
                0x003e_bed4 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x1));
                    c.g[V0][0] = 0x30;
                    c.sb(r(c, A0), c.g[V0][0]);
                    pc = 0x003e_bee0;
                }
                0x003e_bee0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lbu(r(c, A0));
                    pc = 0x003e_bee4;
                }
                0x003e_bee4 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx(p(r(c, A0), 0x1));
                    c.sb(r(c, A2), c.g[V0][0]);
                    c.g[V0][0] = c.lbu(r(c, A0));
                    let more = c.g[V0][0] != 0;
                    c.g[A2][0] = sx(p(r(c, A2), 0x1));
                    if more {
                        if event_due(c) && deliver_event(c, 0x003e_bee0) {
                            return Ok(());
                        }
                        pc = 0x003e_bee0;
                    } else {
                        pc = 0x003e_befc;
                    }
                }
                0x003e_befc => {
                    c.cycles += 11;
                    c.sb(r(c, A2), 0);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[SP][0];
                    callee::TAG_FIELD_SET_RAW_2.call(c, 0x003e_bf10)?;
                    frame_lq_3(c, 0x150, 0x140, 0x130);
                    c.g[RA][0] = c.ld(p(r(c, SP), 0x120));
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x160));
                    c.pc = ra;
                    return Ok(());
                }
                other => {
                    c.jump_out(other, 0x003e_be40, 0x003e_bf28);
                    return Ok(());
                }
            }
        }
    })
}


/// `lq s0/s1/s2, off(sp)` in that order.
#[inline(always)]
pub(crate) fn frame_lq_3(c: &mut Cpu, a: u32, b: u32, d: u32) {
    c.lq(16, p(r(c, SP), a));
    c.lq(17, p(r(c, SP), b));
    c.lq(18, p(r(c, SP), d));
}


/// `TagFieldDecodeHex(data, buffer, len)` (0x3ed578): decode `$`-prefixed
/// hex pairs. With a null buffer, count the pairs; otherwise decode up to
/// `len` pairs, returning -1 when the data is not `$`-prefixed or the length
/// is not positive.
///
/// Dispatcher-shaped: an event at a loop back edge may unwind; the machine
/// re-enters at the saved `pc`, so every stage is accepted here.
pub fn tag_field_decode_hex(c: &mut Cpu) {
    let mut pc = c.pc;
    loop {
        match pc {
            0x003e_d578 => {
                c.cycles += 3;
                c.g[T2][0] = c.g[A2][0];
                let t = c.g[A0][0] == 0;
                c.g[T0][0] = c.g[A1][0];
                pc = if t { 0x003e_d594 } else { 0x003e_d584 };
            }
            0x003e_d584 => {
                c.cycles += 4;
                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                c.g[V0][0] = 0x24;
                let t = c.g[V1][0] == c.g[V0][0];
                pc = if t { 0x003e_d59c } else { 0x003e_d594 };
            }
            0x003e_d594 => {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                abi::ret(c);
                return;
            }
            0x003e_d59c => {
                c.cycles += 2;
                let t = c.g[T0][0] != 0;
                c.g[T1][0] = 0;
                pc = if t { 0x003e_d5dc } else { 0x003e_d5a4 };
            }
            0x003e_d5a4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(1));
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                pc = 0x003e_d5b8;
            }
            0x003e_d5b0 => {
                c.cycles += 2;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003e_d5b8;
            }
            0x003e_d5b8 => {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d5d4 } else { 0x003e_d5c4 };
            }
            0x003e_d5c4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] == 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d5b0;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d5b0;
                } else {
                    pc = 0x003e_d5d4;
                }
            }
            0x003e_d5d4 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return;
            }
            0x003e_d5dc => {
                c.cycles += 2;
                let t = (c.g[T2][0] as i64) <= 0;
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d594;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d594;
                } else {
                    pc = 0x003e_d5e4;
                }
            }
            0x003e_d5e4 => {
                c.cycles += 2;
                let t = (c.g[T2][0] as i64) <= 0;
                c.g[T1][0] = 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d5ec };
            }
            0x003e_d5ec => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d5fc };
            }
            0x003e_d5fc => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 0x0049_0000;
                pc = if t { 0x003e_d670 } else { 0x003e_d60c };
            }
            0x003e_d60c => {
                c.cycles += 5;
                c.g[V1][0] = 0x0049_0000;
                c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x61c0));
                c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x62c0));
                c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                pc = 0x003e_d620;
            }
            0x003e_d620 => {
                c.cycles += 12;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[A2][0] = ((c.g[T1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(2));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
                c.g[V0][0] |= c.g[V1][0];
                c.sb(c.g[T0][0] as u32, c.g[V0][0]);
                let t = c.g[A2][0] == 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                pc = if t { 0x003e_d670 } else { 0x003e_d650 };
            }
            0x003e_d650 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu(c.g[A3][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                let t = c.g[V0][0] != 0;
                pc = if t { 0x003e_d670 } else { 0x003e_d660 };
            }
            0x003e_d660 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[V0][0] < 0x30) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.lbu(c.g[A3][0] as u32);
                    if c.cycles >= c.next_event {
                        c.pc = 0x003e_d620;
                        if c.event() {
                            return;
                        }
                    }
                    pc = 0x003e_d620;
                } else {
                    pc = 0x003e_d670;
                }
            }
            0x003e_d670 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003e_d578, 0x003e_d678);
                return;
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check_lohi;
    use crate::difftest::Rng;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use crate::difftest::elf_image;
    use crate::difftest::scramble;
    use crate::difftest::put_vec;
    use crate::difftest::SP;
    use crate::difftest::fakes;
    use crate::difftest::seed_count;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::compare;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::w;
    pub(crate) use crate::game::bx::bxstring::tests::b;
    pub(crate) use crate::game::main::gamemode::tests::be_library;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_3;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_2;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_3;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_17 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2 as strict_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_5;


    // -----------------------------------------------------------------------
    // 0x3ebb60 TagFieldStrnzcpy.

    #[test]
    fn tag_field_strnzcpy_matches() {
        check(ee::s14::f_003ebb60__generated, tag_field_strnzcpy, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x3ebb_60, 0);
            c.pc = 0x003e_bb60;
            let src = DATA + 0x200;
            let dst = DATA + 0x400;
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = (seed % 6) as u64;
            c.g[A2][0] = sx(src);
            let n = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 3,
                _ => 5,
            };
            for i in 0..n {
                c.bus.write8(src + i, b'a' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(src + n, 0);
        });
    }


    // -----------------------------------------------------------------------
    // 0x3ecfb0 TagFieldGetFlags.

    #[test]
    fn tag_field_get_flags_matches() {
        check(ee::s14::f_003ecfb0__generated, tag_field_get_flags, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3ecf_b0, 0);
            c.pc = 0x003e_cfb0;
            let text = DATA + 0x200;
            c.g[A1][0] = r.next_u32() as u64;
            if seed % 4 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = sx(text);
                let n = 1 + seed % 6;
                for i in 0..n {
                    c.bus.write8(text + i, (r.next_u32() % 0x30) as u8);
                }
                c.bus.write8(text + n, 0);
                // A table of nonzero flag words for the tag characters used.
                for i in 0..0x30u32 {
                    let (ch, flags) = (i as u8, (i * 3) as u32);
                    c.bus.write32(0x0049_65e0 + 4 * ch as u32, flags);
                }
            }
        });
    }


    #[test]
    fn tag_field_get_address_matches() {
        check(ee::s14::f_003ecff8__generated, tag_field_get_address, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3ecff8, 0);
            be_library(c);
            c.pc = 0x003e_cff8;
            c.g[A1][0] = r.next_u32() as u64;
            let s = DATA + 0x200;
            if seed % 4 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = s as u64;
                let n = 1 + seed % 6;
                let chars = *b"0129.x ";
                for i in 0..n {
                    b(c, s + i, chars[((seed + i) % 7) as usize]);
                }
                b(c, s + n, 0);
            }
        });
    }

    pub(crate) fn put_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            wb(c, a + i as u32, *b);
        }
        wb(c, a + s.len() as u32, 0);
    }


    #[test]
    fn tag_field_get_number_matches() {
        check(ee::s14::f_003ecf30__generated, tag_field_get_number, 24, &|c, seed| {
            let _ = base(c, seed, 0x3ecf_30, 0);
            let tag = DATA + 0x1000;
            let text: &[u8] = match seed % 6 {
                0 => b"123",
                1 => b"-45",
                2 => b"+7",
                3 => b"abc",
                4 => b"",
                _ => b"0x10",
            };
            put_str(c, tag, text);
            c.g[A0][0] = tag as u64;
            c.g[A1][0] = 77;
        });
    }


    // ------------------------------------------------------------------
    // Tag fields.

    #[test]
    fn tag_field_get_token_matches() {
        check(ssx3_ee::s14::f_003ed070__generated, tag_field_get_token, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x3ed070, 0);
            let s = DATA + 0x200;
            if seed % 7 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = s as u64;
                // A run of printable characters, a control character and a
                // high byte, at varying offsets.
                let run = (r.next_u32() % 6) as usize;
                for i in 0..0x10 {
                    let v = if i < run {
                        b'a' + (r.next_u32() % 26) as u8
                    } else {
                        [0u8, 0x20, 0x7f, 0x80, 0xff][(r.next_u32() % 5) as usize]
                    };
                    b(c, s + i as u32, v);
                }
            }
            c.g[A1][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn tag_field_set_token_matches() {
        check(ee::s14::f_003ec0f8__generated, tag_field_set_token, 40, &|c, seed| {
            let _r = machine(c, seed, 0x3ec0f8, 2);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A3][0] = [0u64, 1, 0x414243, u32::MAX as u64][seed as usize % 4];
        });
    }


    #[test]
    fn tag_field_set_flags_matches() {
        check(ssx3_ee::s14::f_003ebf28__generated, tag_field_set_flags, 12, &|c, seed| {
            let _ = base_2(c, seed, 0x003e_bf28);
            heap(c);
            let tag = DATA + 0x100;
            clear(c, tag, 0x120);
            put_str_2(c, 0x0049_65c0, "0123456789abcdefghijklmnopqrstuv");
            c.g[A0][0] = sx(tag);
            c.g[A1][0] = (seed % 0x10000) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 0xffff_ffff,
                _ => (seed * 2654435761) as u64,
            };
        });
    }


    #[test]
    fn tag_field_set_binary_matches() {
        strict(ee::s14::f_003ec300__generated, tag_field_set_binary, 120, &|c, seed| {
            let mut r = Rng(0x003e_c300 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            let record = DATA;
            c.bus.write32(record, 0);
            let name = DATA + 0x100;
            let len = if seed % 3 == 0 { 0 } else { 1 + (r.next_u32() % 6) as usize };
            for i in 0..len {
                c.bus.write8(name + i as u32, b'A' + (r.next_u32() % 26) as u8);
            }
            c.bus.write8(name + len as u32, 0);
            c.bus.write8(name + len as u32 + 1, b'Q');
            for i in 0..16u32 {
                c.bus.write8(DATA + 0x200 + i, (r.next_u32() % 40 + 32) as u8);
            }
            let size: i32 = [0, 1, 3, -1][(seed / 2 % 4) as usize];
            c.g[A0][0] = sx(record);
            c.g[A1][0] = 0;
            c.g[A2][0] = if seed % 3 == 0 { 0 } else { sx(name) };
            c.g[A3][0] = sx(DATA + 0x200);
            c.g[T0][0] = sx(size as u32);
        });
    }


    #[test]
    fn tag_field_merge_matches() {
        check(ee::s14::f_003ecb00__generated, tag_field_merge, 16, &|c, seed| {
            let _ = base_3(c, seed, 0x3ecb00, 2);
            let rec = DATA;
            let data = DATA + 0x200;
            clear_2(c, rec, 0x200);
            clear_2(c, data, 0x200);
            c.pc = 0x003e_cb00;
            c.g[A0][0] = rec as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = data as u64;
            let text: &[u8] = match seed % 5 {
                0 => b"name=value other:2  \0",
                1 => b"\0",
                2 => b"   \0",
                3 => b"a=b\0",
                _ => b"key=1=2\0",
            };
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(data + i as u32, *b);
            }
        });
    }


    // ----------------------------------------------------------- tag fields

    #[test]
    fn tag_field_set_number_matches() {
        strict_2(ee::s14::f_003ebe40__generated, tag_field_set_number, 90, &|c, seed| {
            let _ = base_4(c, seed, 0x3ebe, 6);
            let value = match seed % 5 {
                0 => 0,
                1 => 7,
                2 => 12345,
                3 => 0x7fff_ffff,
                _ => (seed % 1000) + 1,
            };
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 8;
            c.g[A2][0] = (DATA + 0x900) as u64;
            c.g[A3][0] = if seed % 2 == 0 {
                value as u64
            } else {
                (-(value as i64)) as u64
            };
            for (i, b) in b"name\0".iter().enumerate() {
                c.bus.write8(DATA + 0x900 + i as u32, *b);
            }
            c.pc = 0x003e_be40;
        });
    }


    #[test]
    fn tag_field_decode_hex_matches() {
        strict_3(ssx3_ee::s14::f_003ed578__generated, tag_field_decode_hex, SEEDS_3, &|c, seed| {
            let _ = base_5(c, seed, 0xd578);
            let data = DATA + 0x1000;
            let buf = DATA + 0x2000;
            let len = DATA + 0x3000;
            let text: &[u8] = match seed % 6 {
                0 => b"\0",
                1 => b"XABCDEF\0",
                2 => b"$ABCDEF\0",
                3 => b"$ABCDEF\0",
                4 => b"$ABCDEF\0",
                _ => b"$AB\0",
            };
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(data + i as u32, *b);
            }
            c.bus.write16(len, (seed % 4) as u16);
            c.g[A0][0] = sx(if seed % 6 == 0 { 0 } else { data });
            c.g[A1][0] = sx(if seed % 6 == 2 { 0 } else { buf });
            c.g[A2][0] = sx(c.bus.read32(len));
            c.pc = 0x003e_d578;
        });
    }
}
